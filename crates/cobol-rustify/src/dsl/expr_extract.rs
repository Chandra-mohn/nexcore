//! Expression extraction from transpiler-generated Rust AST.
//!
//! Recognizes the regular patterns emitted by cobol-transpiler and converts
//! them to RulesDSL/TransformDSL-valid expressions.
//!
//! Transpiler condition patterns:
//!   ws.FIELD.to_decimal() OP "N".parse::<Decimal>().unwrap()  -> field OP N
//!   ws.FIELD.as_bytes() OP "S".as_bytes()                     -> field OP "S"
//!   cond1 && cond2                                            -> cond1 and cond2
//!   cond1 || cond2                                            -> cond1 or cond2
//!   !(cond)                                                   -> not cond
//!
//! Transpiler action patterns:
//!   move_alphanumeric_literal(b"TEXT", &mut ws.FIELD, ...)    -> set field = "TEXT"
//!   move_numeric_literal("N".parse::<Decimal>().unwrap(), ...) -> set field = N

/// Extract a RulesDSL-valid boolean expression from a Rust condition expression.
pub fn extract_condition(expr: &syn::Expr) -> String {
    match expr {
        // Binary: a > b, a == b, a && b, a || b
        syn::Expr::Binary(bin) => {
            let op = match bin.op {
                syn::BinOp::Gt(_) => ">",
                syn::BinOp::Lt(_) => "<",
                syn::BinOp::Ge(_) => ">=",
                syn::BinOp::Le(_) => "<=",
                syn::BinOp::Eq(_) => "==",
                syn::BinOp::Ne(_) => "!=",
                syn::BinOp::And(_) => "and",
                syn::BinOp::Or(_) => "or",
                _ => return format!("/* unsupported op: {} */", quote::quote!(#expr)),
            };
            let left = extract_condition(&bin.left);
            let right = extract_condition(&bin.right);
            format!("{left} {op} {right}")
        }

        // Unary not: !(expr)
        syn::Expr::Unary(syn::ExprUnary { op: syn::UnOp::Not(_), expr, .. }) => {
            let inner = extract_condition(expr);
            format!("not ({inner})")
        }

        // Parenthesized: (expr)
        syn::Expr::Paren(p) => {
            let inner = extract_condition(&p.expr);
            format!("({inner})")
        }

        // Method call: ws.field.to_decimal(), ws.field.as_bytes(), ws.field.is_alphabetic()
        syn::Expr::MethodCall(mc) => {
            let method = mc.method.to_string();
            match method.as_str() {
                "to_decimal" | "as_bytes" | "display_bytes" => {
                    // Extract field name from receiver: ws.field.method()
                    extract_field_ref(&mc.receiver)
                }
                "is_alphabetic" | "is_numeric" | "is_positive" | "is_negative" | "is_zero" => {
                    let field = extract_field_ref(&mc.receiver);
                    format!("{field} is {}", method.strip_prefix("is_").unwrap_or(&method))
                }
                "unwrap" => {
                    // "25".parse::<Decimal>().unwrap() -> extract the literal
                    extract_parse_literal(&mc.receiver)
                }
                _ => extract_field_ref(expr),
            }
        }

        // Literal string/number
        syn::Expr::Lit(lit) => extract_literal(&lit.lit),

        // Reference: &mut ws.field -> extract field
        syn::Expr::Reference(r) => extract_condition(&r.expr),

        // Field access: ws.field
        syn::Expr::Field(_) => extract_field_ref(expr),

        // Call expression: might be "25".parse::<Decimal>().unwrap() chain
        syn::Expr::Call(call) => {
            // Try to extract as a function call pattern
            let func_str = quote::quote!(#call).to_string();
            if func_str.contains("parse") {
                extract_parse_chain_from_call(call)
            } else {
                format!("/* call: {} */", abbreviate(&func_str))
            }
        }

        _ => {
            let s = quote::quote!(#expr).to_string();
            format!("/* expr: {} */", abbreviate(&s))
        }
    }
}

/// Extract a (field_name, value) pair from a statement that is an action.
/// Returns None if the statement is not a recognized action pattern.
pub fn extract_action(stmt: &syn::Stmt) -> Option<(String, String)> {
    match stmt {
        syn::Stmt::Expr(expr, _) => extract_action_from_expr(expr),
        _ => None,
    }
}

/// Extract action from an expression statement.
pub fn extract_action_from_expr(expr: &syn::Expr) -> Option<(String, String)> {
    match expr {
        syn::Expr::Call(call) => {
            let func_name = call_func_name(call)?;
            match func_name.as_str() {
                "move_alphanumeric_literal" => {
                    // move_alphanumeric_literal(b"TEXT", &mut ws.field, &ctx.config)
                    if call.args.len() >= 2 {
                        let value = extract_byte_literal(&call.args[0])?;
                        let field = extract_mut_field_name(&call.args[1])?;
                        Some((field, format!("\"{value}\"")))
                    } else {
                        None
                    }
                }
                "move_numeric_literal" => {
                    // move_numeric_literal("99".parse::<Decimal>().unwrap(), &mut ws.field, ...)
                    if call.args.len() >= 2 {
                        let value = extract_condition(&call.args[0]);
                        let field = extract_mut_field_name(&call.args[1])?;
                        Some((field, value))
                    } else {
                        None
                    }
                }
                "cobol_add" | "cobol_add_giving" => {
                    extract_arithmetic_verb("+", &call.args)
                }
                "cobol_subtract" | "cobol_subtract_giving" => {
                    extract_arithmetic_verb("-", &call.args)
                }
                "cobol_multiply" | "cobol_multiply_giving" => {
                    extract_arithmetic_verb("*", &call.args)
                }
                "cobol_divide_into" | "cobol_divide_giving" => {
                    extract_arithmetic_verb("/", &call.args)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Extract all actions from a block (if-body or else-body).
pub fn extract_actions_from_block(block: &syn::Block) -> Vec<(String, String)> {
    block
        .stmts
        .iter()
        .filter_map(extract_action)
        .collect()
}

/// Extract the condition and actions from each branch of an if/else chain.
/// Returns Vec of (condition_string, Vec<(field, value)>).
pub fn extract_if_chain(if_expr: &syn::ExprIf) -> Vec<(String, Vec<(String, String)>)> {
    let mut branches = Vec::new();

    // First branch
    let cond = extract_condition(&if_expr.cond);
    let actions = extract_actions_from_block(&if_expr.then_branch);
    branches.push((cond, actions));

    // Walk else-if chain
    let mut current = if_expr.else_branch.as_ref();
    while let Some((_, else_expr)) = current {
        match else_expr.as_ref() {
            syn::Expr::If(nested) => {
                let cond = extract_condition(&nested.cond);
                let actions = extract_actions_from_block(&nested.then_branch);
                branches.push((cond, actions));
                current = nested.else_branch.as_ref();
            }
            syn::Expr::Block(block) => {
                let actions = extract_actions_from_block(&block.block);
                branches.push(("otherwise".to_string(), actions));
                break;
            }
            _ => break,
        }
    }

    branches
}

// ---------------------------------------------------------------------------
// Purity detection (legacy path)
// ---------------------------------------------------------------------------

/// Check whether a syn::Block contains calls that indicate side effects.
/// Looks for transpiler-generated functions like `display`, `cobol_read`,
/// `cobol_write`, `open_file`, `close_file`, `exec_sql`, `cobol_call`, etc.
pub fn block_has_side_effects(block: &syn::Block) -> bool {
    block.stmts.iter().any(|stmt| syn_stmt_has_side_effects(stmt))
}

fn syn_stmt_has_side_effects(stmt: &syn::Stmt) -> bool {
    match stmt {
        syn::Stmt::Expr(expr, _) => syn_expr_has_side_effects(expr),
        syn::Stmt::Local(local) => local
            .init
            .as_ref()
            .is_some_and(|init| syn_expr_has_side_effects(&init.expr)),
        _ => false,
    }
}

fn syn_expr_has_side_effects(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Call(call) => {
            if let Some(name) = call_func_name(call) {
                matches!(
                    name.as_str(),
                    "display"
                        | "accept_input"
                        | "open_file"
                        | "close_file"
                        | "cobol_read"
                        | "cobol_write"
                        | "cobol_rewrite"
                        | "cobol_delete"
                        | "cobol_start"
                        | "exec_sql"
                        | "execute_sql"
                        | "cobol_call"
                        | "call_program"
                        | "cobol_sort"
                        | "cobol_merge"
                )
            } else {
                false
            }
        }
        syn::Expr::If(if_expr) => {
            block_has_side_effects(&if_expr.then_branch)
                || if_expr
                    .else_branch
                    .as_ref()
                    .is_some_and(|(_, e)| syn_expr_has_side_effects(e))
        }
        syn::Expr::Block(b) => block_has_side_effects(&b.block),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Extract a field name from ws.field or ws.field.method()
fn extract_field_ref(expr: &syn::Expr) -> String {
    match expr {
        syn::Expr::Field(f) => {
            if let syn::Member::Named(ident) = &f.member {
                let name = ident.to_string();
                // If receiver is also a field access (ws.field), just return the field name
                if let syn::Expr::Field(_) = f.base.as_ref() {
                    // ws.ws_field -> ws_field
                    name
                } else {
                    name
                }
            } else {
                quote::quote!(#expr).to_string()
            }
        }
        syn::Expr::MethodCall(mc) => {
            let method = mc.method.to_string();
            if matches!(method.as_str(), "to_decimal" | "as_bytes" | "display_bytes") {
                extract_field_ref(&mc.receiver)
            } else {
                quote::quote!(#expr).to_string()
            }
        }
        syn::Expr::Reference(r) => extract_field_ref(&r.expr),
        _ => {
            let s = quote::quote!(#expr).to_string();
            abbreviate(&s)
        }
    }
}

/// Extract a literal value from a syn::Lit.
fn extract_literal(lit: &syn::Lit) -> String {
    match lit {
        syn::Lit::Str(s) => format!("\"{}\"", s.value()),
        syn::Lit::ByteStr(b) => {
            let bytes = b.value();
            let text = String::from_utf8_lossy(&bytes);
            format!("\"{text}\"")
        }
        syn::Lit::Int(i) => i.base10_digits().to_string(),
        syn::Lit::Float(f) => f.base10_digits().to_string(),
        syn::Lit::Bool(b) => if b.value { "true" } else { "false" }.to_string(),
        _ => format!("/* lit */"),
    }
}

/// Extract numeric literal from "25".parse::<Decimal>().unwrap() chain.
/// The receiver of .unwrap() is "25".parse::<Decimal>()
fn extract_parse_literal(receiver: &syn::Expr) -> String {
    match receiver {
        syn::Expr::MethodCall(mc) if mc.method == "parse" => {
            // Receiver is the string literal "25"
            // Extract the raw string and return as bare number if numeric
            match mc.receiver.as_ref() {
                syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) => {
                    let val = s.value();
                    if val.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-') {
                        val
                    } else {
                        format!("\"{val}\"")
                    }
                }
                _ => extract_condition(&mc.receiver),
            }
        }
        syn::Expr::Call(call) => {
            extract_parse_chain_from_call(call)
        }
        _ => {
            let s = quote::quote!(#receiver).to_string();
            if let Some(num) = extract_quoted_number(&s) {
                num
            } else {
                abbreviate(&s)
            }
        }
    }
}

/// Try to extract the number from a parse chain call expression.
fn extract_parse_chain_from_call(call: &syn::ExprCall) -> String {
    let s = quote::quote!(#call).to_string();
    if let Some(num) = extract_quoted_number(&s) {
        num
    } else {
        format!("/* call: {} */", abbreviate(&s))
    }
}

/// Extract a value from a string like `"25" . parse :: < Decimal > () . unwrap ()`.
/// Returns bare number for numeric strings, quoted string for non-numeric.
fn extract_quoted_number(s: &str) -> Option<String> {
    // Look for "VALUE" pattern
    let start = s.find('"')?;
    let rest = &s[start + 1..];
    let end = rest.find('"')?;
    let inner = &rest[..end];
    // If it looks numeric, return bare (no quotes)
    if inner.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-') {
        Some(inner.to_string())
    } else {
        // Non-numeric: return quoted
        Some(format!("\"{inner}\""))
    }
}

/// Extract byte string literal: b"TEXT" -> "TEXT"
fn extract_byte_literal(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::ByteStr(b), .. }) => {
            let bytes = b.value();
            Some(String::from_utf8_lossy(&bytes).to_string())
        }
        _ => None,
    }
}

/// Extract field name from &mut ws.field
fn extract_mut_field_name(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Reference(r) => extract_mut_field_name(&r.expr),
        syn::Expr::Field(f) => {
            if let syn::Member::Named(ident) = &f.member {
                Some(ident.to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Extract a (field, expression) pair from cobol_add/subtract/multiply/divide patterns.
/// Pattern: cobol_add(&ws.src, &mut ws.dest, ...) -> (dest, "dest + src")
/// The operator is applied as `dest OP src` (in-place mutation semantics).
fn extract_arithmetic_verb(
    op: &str,
    args: &syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>,
) -> Option<(String, String)> {
    if args.len() < 2 {
        return None;
    }
    let src = extract_mut_field_name(&args[0])?;
    let dest = extract_mut_field_name(&args[1])?;
    Some((dest.clone(), format!("{dest} {op} {src}")))
}

/// Get the function name from a call expression.
fn call_func_name(call: &syn::ExprCall) -> Option<String> {
    match call.func.as_ref() {
        syn::Expr::Path(p) => {
            p.path.segments.last().map(|s| s.ident.to_string())
        }
        _ => None,
    }
}

/// Abbreviate a long string for comments.
fn abbreviate(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.len() > 50 {
        format!("{}...", &trimmed[..47])
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expr(code: &str) -> syn::Expr {
        syn::parse_str::<syn::Expr>(code).unwrap()
    }

    #[test]
    fn numeric_comparison() {
        let expr = parse_expr(r#"ws.ws_a.to_decimal() > "25".parse::<Decimal>().unwrap()"#);
        let result = extract_condition(&expr);
        assert_eq!(result, "ws_a > 25", "Got: {result}");
    }

    #[test]
    fn string_comparison() {
        let expr = parse_expr(r#"ws.ws_grade.as_bytes() == "A".as_bytes()"#);
        let result = extract_condition(&expr);
        assert_eq!(result, "ws_grade == \"A\"", "Got: {result}");
    }

    #[test]
    fn compound_and() {
        let expr = parse_expr(
            r#"ws.ws_a.to_decimal() > "25".parse::<Decimal>().unwrap() && ws.ws_b.to_decimal() > "50".parse::<Decimal>().unwrap()"#,
        );
        let result = extract_condition(&expr);
        assert_eq!(result, "ws_a > 25 and ws_b > 50", "Got: {result}");
    }

    #[test]
    fn compound_or() {
        let expr = parse_expr(
            r#"ws.ws_a.to_decimal() > "100".parse::<Decimal>().unwrap() || ws.ws_b.to_decimal() > "50".parse::<Decimal>().unwrap()"#,
        );
        let result = extract_condition(&expr);
        assert_eq!(result, "ws_a > 100 or ws_b > 50", "Got: {result}");
    }

    #[test]
    fn negation() {
        let expr = parse_expr(
            r#"!(ws.ws_a.to_decimal() > "100".parse::<Decimal>().unwrap())"#,
        );
        let result = extract_condition(&expr);
        assert!(result.contains("not"), "Should contain not: {result}");
        assert!(result.contains("ws_a"), "Should reference ws_a: {result}");
        assert!(result.contains("100"), "Should have 100: {result}");
    }

    #[test]
    fn field_to_field_comparison() {
        let expr = parse_expr(r#"ws.ws_a.as_bytes() > ws.ws_b.as_bytes()"#);
        let result = extract_condition(&expr);
        assert_eq!(result, "ws_a > ws_b", "Got: {result}");
    }

    #[test]
    fn range_condition() {
        let expr = parse_expr(
            r#"ws.ws_month.to_decimal() >= "1".parse::<Decimal>().unwrap() && ws.ws_month.to_decimal() <= "3".parse::<Decimal>().unwrap()"#,
        );
        let result = extract_condition(&expr);
        assert_eq!(result, "ws_month >= 1 and ws_month <= 3", "Got: {result}");
    }

    #[test]
    fn extract_alphanumeric_action() {
        let code = r#"move_alphanumeric_literal(b"HIGH", &mut ws.ws_result, &ctx.config)"#;
        let expr = syn::parse_str::<syn::Expr>(code).unwrap();
        let result = extract_action_from_expr(&expr);
        assert_eq!(
            result,
            Some(("ws_result".to_string(), "\"HIGH\"".to_string())),
            "Got: {result:?}"
        );
    }

    #[test]
    fn extract_numeric_action() {
        let code = r#"move_numeric_literal("99".parse::<Decimal>().unwrap(), &mut ws.ws_code, &ctx.config)"#;
        let expr = syn::parse_str::<syn::Expr>(code).unwrap();
        let result = extract_action_from_expr(&expr);
        assert!(result.is_some(), "Should extract numeric action");
        let (field, value) = result.unwrap();
        assert_eq!(field, "ws_code");
        assert!(value.contains("99"), "Value should contain 99: {value}");
    }

    #[test]
    fn extract_if_chain_branches() {
        let code = r#"
            if ws.ws_a.to_decimal() > "75".parse::<Decimal>().unwrap() {
                move_alphanumeric_literal(b"HIGH", &mut ws.ws_result, &ctx.config);
            } else if ws.ws_a.to_decimal() > "50".parse::<Decimal>().unwrap() {
                move_alphanumeric_literal(b"MEDIUM", &mut ws.ws_result, &ctx.config);
            } else if ws.ws_a.to_decimal() > "25".parse::<Decimal>().unwrap() {
                move_alphanumeric_literal(b"LOW", &mut ws.ws_result, &ctx.config);
            } else {
                move_alphanumeric_literal(b"VERY LOW", &mut ws.ws_result, &ctx.config);
            }
        "#;
        let expr = syn::parse_str::<syn::Expr>(code).unwrap();
        let if_expr = match &expr {
            syn::Expr::If(i) => i,
            other => panic!("Expected Expr::If, got {:?}", std::mem::discriminant(other)),
        };

        let branches = extract_if_chain(if_expr);
        assert_eq!(branches.len(), 4, "Should have 4 branches (3 if + 1 else)");

        // First branch: ws_a > 75 -> set ws_result = "HIGH"
        assert_eq!(branches[0].0, "ws_a > 75");
        assert_eq!(branches[0].1.len(), 1);
        assert_eq!(branches[0].1[0], ("ws_result".to_string(), "\"HIGH\"".to_string()));

        // Second branch: ws_a > 50 -> set ws_result = "MEDIUM"
        assert_eq!(branches[1].0, "ws_a > 50");
        assert_eq!(branches[1].1[0].1, "\"MEDIUM\"");

        // Third branch: ws_a > 25 -> set ws_result = "LOW"
        assert_eq!(branches[2].0, "ws_a > 25");
        assert_eq!(branches[2].1[0].1, "\"LOW\"");

        // Fourth (else): otherwise -> set ws_result = "VERY LOW"
        assert_eq!(branches[3].0, "otherwise");
        assert_eq!(branches[3].1[0].1, "\"VERY LOW\"");
    }

    #[test]
    fn extract_grade_chain() {
        let code = r#"
            if ws.ws_grade.as_bytes() == "A".as_bytes() {
                move_alphanumeric_literal(b"EXCELLENT", &mut ws.ws_result, &ctx.config);
            } else if ws.ws_grade.as_bytes() == "B".as_bytes() {
                move_alphanumeric_literal(b"GOOD", &mut ws.ws_result, &ctx.config);
            } else if ws.ws_grade.as_bytes() == "C".as_bytes() {
                move_alphanumeric_literal(b"AVERAGE", &mut ws.ws_result, &ctx.config);
            } else {
                move_alphanumeric_literal(b"UNKNOWN GRADE", &mut ws.ws_result, &ctx.config);
            }
        "#;
        let expr = syn::parse_str::<syn::Expr>(code).unwrap();
        let if_expr = match &expr {
            syn::Expr::If(i) => i,
            other => panic!("Expected Expr::If, got {:?}", std::mem::discriminant(other)),
        };

        let branches = extract_if_chain(if_expr);
        assert_eq!(branches.len(), 4);
        assert!(branches[0].0.contains("ws_grade") && branches[0].0.contains("\"A\""));
        assert!(branches[1].0.contains("ws_grade") && branches[1].0.contains("\"B\""));
        assert!(branches[2].0.contains("ws_grade") && branches[2].0.contains("\"C\""));
        assert_eq!(branches[3].0, "otherwise");
    }

    // -----------------------------------------------------------------------
    // Arithmetic verb extraction tests
    // -----------------------------------------------------------------------

    #[test]
    fn extract_cobol_add_action() {
        let code = r#"cobol_add(&ws.ws_amount, &mut ws.ws_total, None, &ctx.config)"#;
        let expr = syn::parse_str::<syn::Expr>(code).unwrap();
        let result = extract_action_from_expr(&expr);
        assert_eq!(
            result,
            Some(("ws_total".to_string(), "ws_total + ws_amount".to_string()))
        );
    }

    #[test]
    fn extract_cobol_subtract_action() {
        let code =
            r#"cobol_subtract(&ws.ws_deduction, &mut ws.ws_balance, None, &ctx.config)"#;
        let expr = syn::parse_str::<syn::Expr>(code).unwrap();
        let result = extract_action_from_expr(&expr);
        assert_eq!(
            result,
            Some(("ws_balance".to_string(), "ws_balance - ws_deduction".to_string()))
        );
    }

    #[test]
    fn extract_cobol_multiply_action() {
        let code = r#"cobol_multiply(&ws.ws_rate, &mut ws.ws_amount, None, &ctx.config)"#;
        let expr = syn::parse_str::<syn::Expr>(code).unwrap();
        let result = extract_action_from_expr(&expr);
        assert_eq!(
            result,
            Some(("ws_amount".to_string(), "ws_amount * ws_rate".to_string()))
        );
    }

    #[test]
    fn extract_cobol_divide_action() {
        let code =
            r#"cobol_divide_into(&ws.ws_divisor, &mut ws.ws_result, None, &ctx.config)"#;
        let expr = syn::parse_str::<syn::Expr>(code).unwrap();
        let result = extract_action_from_expr(&expr);
        assert_eq!(
            result,
            Some(("ws_result".to_string(), "ws_result / ws_divisor".to_string()))
        );
    }

    // -----------------------------------------------------------------------
    // Legacy purity detection tests
    // -----------------------------------------------------------------------

    #[test]
    fn syn_pure_block() {
        let code = r#"{ cobol_add(&ws.ws_a, &mut ws.ws_b, None, &ctx.config); }"#;
        let block = syn::parse_str::<syn::Block>(code).unwrap();
        assert!(!block_has_side_effects(&block));
    }

    #[test]
    fn syn_impure_display() {
        let code = r#"{ display(&ws.ws_msg, &ctx.config); }"#;
        let block = syn::parse_str::<syn::Block>(code).unwrap();
        assert!(block_has_side_effects(&block));
    }

    #[test]
    fn syn_impure_exec_sql() {
        let code = r#"{ exec_sql(&mut ws, &ctx); }"#;
        let block = syn::parse_str::<syn::Block>(code).unwrap();
        assert!(block_has_side_effects(&block));
    }
}
