// NexCore -- Nexflow Codegen: Java Expression Translator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Translates DSL expression strings to Java code.
//!
//! The TransformDSL AST stores expressions as raw strings (e.g. `"input.amount * 0.02"`).
//! This module translates common patterns to valid Java:
//!
//! - Field paths: `input.amount` -> `input.getAmount()`
//! - Null coalesce: `field ?? default` -> `(field != null ? field : default)`
//! - Null-safe access: `field?.sub` -> `(field != null ? field.getSub() : null)`
//! - When/otherwise: `when cond : val otherwise : def` -> ternary
//! - Function calls: `round(x, 2)` -> `Math.round(x, 2)` etc.

use crate::java::naming::{to_camel_case, to_pascal_case};

/// Translate a DSL expression string to Java code.
///
/// This handles the most common patterns. Complex expressions that don't
/// match known patterns pass through with field path translation applied.
pub fn translate_expression(expr: &str) -> String {
    let trimmed = expr.trim();

    // Empty expression
    if trimmed.is_empty() {
        return "null".to_string();
    }

    // String literal -- pass through
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        return trimmed.to_string();
    }

    // Numeric literal -- pass through
    if trimmed.parse::<f64>().is_ok() {
        return trimmed.to_string();
    }

    // Boolean literal
    if trimmed == "true" || trimmed == "false" {
        return trimmed.to_string();
    }

    // Null coalesce: expr ?? default
    if let Some((left, right)) = split_operator(trimmed, "??") {
        let left_java = translate_expression(left);
        let right_java = translate_expression(right);
        return format!("({left_java} != null ? {left_java} : {right_java})");
    }

    // Known function calls
    if let Some(result) = translate_function_call(trimmed) {
        return result;
    }

    // Field path translation (the most common case)
    translate_field_paths(trimmed)
}

/// Translate field path references in an expression to Java getter chains.
///
/// `input.amount` -> `input.getAmount()`
/// `input.customer.name` -> `input.getCustomer().getName()`
/// `output.total` -> just `total` (assignment target context)
fn translate_field_paths(expr: &str) -> String {
    let mut result = String::with_capacity(expr.len() * 2);
    let mut chars = expr.chars().peekable();
    let mut current_word = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' || c == '.' {
            current_word.push(c);
            chars.next();
        } else {
            if !current_word.is_empty() {
                result.push_str(&translate_field_path_token(&current_word));
                current_word.clear();
            }
            // Translate DSL operators to Java
            if c == '=' && chars.clone().nth(1) == Some('?') {
                // =? null-safe equality -> Objects.equals
                chars.next(); // consume =
                chars.next(); // consume ?
                result.push_str("Objects.equals");
                // The next tokens will be handled as arguments
                continue;
            }
            result.push(c);
            chars.next();
        }
    }

    if !current_word.is_empty() {
        result.push_str(&translate_field_path_token(&current_word));
    }

    // Post-process: replace DSL keywords with Java equivalents
    let result = result
        .replace(" and ", " && ")
        .replace(" or ", " || ")
        .replace(" not ", " !")
        .replace("is null", "== null")
        .replace("is not null", "!= null");

    result
}

/// Translate a single field path token (e.g. "input.amount.value").
fn translate_field_path_token(token: &str) -> String {
    if !token.contains('.') {
        // Simple identifier -- could be a local variable or keyword
        return translate_simple_identifier(token);
    }

    // Numeric literal with decimal point (e.g. "0.02", "3.14")
    if token.parse::<f64>().is_ok() {
        return token.to_string();
    }

    let parts: Vec<&str> = token.split('.').collect();
    if parts.is_empty() {
        return token.to_string();
    }

    let mut result = String::new();
    let first = parts[0];

    match first {
        "input" | "output" | "result" => {
            // Known context prefixes
            result.push_str(first);
            for part in &parts[1..] {
                result.push_str(&format!(".get{}()", to_pascal_case(part)));
            }
        }
        "lookups" => {
            // lookups.customer(key) pattern -- handled at call site
            if parts.len() >= 2 {
                result.push_str(&format!("lookup{}", to_pascal_case(parts[1])));
                // Additional parts become getter chains
                for part in &parts[2..] {
                    result.push_str(&format!(".get{}()", to_pascal_case(part)));
                }
            } else {
                result.push_str(token);
            }
        }
        "state" => {
            // state.get_window(), state.aggregate() etc.
            result.push_str("getState()");
            for part in &parts[1..] {
                result.push_str(&format!(".{}()", to_camel_case(part)));
            }
        }
        _ => {
            // Local variable or lookup result with getter chain
            result.push_str(first);
            for part in &parts[1..] {
                result.push_str(&format!(".get{}()", to_pascal_case(part)));
            }
        }
    }

    result
}

/// Translate a simple (no-dot) identifier.
fn translate_simple_identifier(id: &str) -> String {
    match id {
        "null" | "true" | "false" => id.to_string(),
        "now" => "java.time.Instant.now()".to_string(),
        _ => id.to_string(),
    }
}

/// Translate known DSL function calls to Java.
fn translate_function_call(expr: &str) -> Option<String> {
    // Extract function name and args: "func(args)"
    let paren_start = expr.find('(')?;
    if !expr.ends_with(')') {
        return None;
    }
    let func_name = expr[..paren_start].trim();
    let args_str = &expr[paren_start + 1..expr.len() - 1];

    // Only match if func_name is a simple identifier (no dots, no spaces before paren)
    if func_name.contains(' ') || func_name.contains('.') {
        return None;
    }

    let args: Vec<String> = split_args(args_str)
        .iter()
        .map(|a| translate_expression(a))
        .collect();

    let result = match func_name {
        // Math
        "round" => {
            if args.len() >= 2 {
                format!(
                    "{}.setScale({}, java.math.RoundingMode.HALF_UP)",
                    args[0], args[1]
                )
            } else {
                format!("Math.round({})", args.join(", "))
            }
        }
        "abs" => format!("Math.abs({})", args.join(", ")),
        "max" if args.len() == 2 => format!("Math.max({}, {})", args[0], args[1]),
        "min" if args.len() == 2 => format!("Math.min({}, {})", args[0], args[1]),

        // String
        "upper" | "uppercase" => format!("{}.toUpperCase()", args.first().unwrap_or(&"\"\"".to_string())),
        "lower" | "lowercase" => format!("{}.toLowerCase()", args.first().unwrap_or(&"\"\"".to_string())),
        "trim" => format!("{}.trim()", args.first().unwrap_or(&"\"\"".to_string())),
        "concat" => {
            let parts: Vec<String> = args.iter().map(|a| format!("String.valueOf({a})")).collect();
            parts.join(" + ")
        }
        "length" => format!("{}.length()", args.first().unwrap_or(&"\"\"".to_string())),
        "substring" => {
            if args.len() >= 3 {
                format!("{}.substring({}, {})", args[0], args[1], args[2])
            } else if args.len() == 2 {
                format!("{}.substring({})", args[0], args[1])
            } else {
                format!("substring({})", args.join(", "))
            }
        }

        // Type conversion
        "string" => format!("String.valueOf({})", args.join(", ")),
        "integer" => format!("Long.valueOf({})", args.join(", ")),
        "decimal" => format!("new java.math.BigDecimal({})", args.join(", ")),

        // Date/time
        "now" | "processing_date" => "java.time.Instant.now()".to_string(),
        "current_date" | "current_business_date" => "java.time.LocalDate.now()".to_string(),
        "generate_uuid" => "java.util.UUID.randomUUID().toString()".to_string(),

        // Collection operations -> NexflowRuntime
        "filter" | "find" | "any" | "all" | "none" | "sum" | "count" | "avg"
        | "distinct" => {
            format!("NexflowRuntime.{}({})", func_name, args.join(", "))
        }

        // Encryption
        "encrypt" => format!("NexflowRuntime.encrypt({})", args.join(", ")),
        "decrypt" => format!("NexflowRuntime.decrypt({})", args.join(", ")),

        // Lookup
        "lookup" => {
            if args.len() >= 2 {
                format!("NexflowRuntime.lookup({}, {})", args[0], args[1..].join(", "))
            } else {
                format!("NexflowRuntime.lookup({})", args.join(", "))
            }
        }

        // Not a known function -- return None to fall through
        _ => return None,
    };

    Some(result)
}

/// Split a string at a top-level binary operator (not inside parens/quotes).
fn split_operator<'a>(expr: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let mut depth = 0;
    let mut in_string = false;
    let bytes = expr.as_bytes();
    let op_bytes = op.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            in_string = !in_string;
        } else if !in_string {
            if bytes[i] == b'(' {
                depth += 1;
            } else if bytes[i] == b')' {
                depth -= 1;
            } else if depth == 0
                && i + op_bytes.len() <= bytes.len()
                && &bytes[i..i + op_bytes.len()] == op_bytes
            {
                return Some((&expr[..i], &expr[i + op.len()..]));
            }
        }
        i += 1;
    }
    None
}

/// Split function arguments at top-level commas.
fn split_args(args: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut start = 0;

    for (i, c) in args.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '(' | '[' | '{' if !in_string => depth += 1,
            ')' | ']' | '}' if !in_string => depth -= 1,
            ',' if !in_string && depth == 0 => {
                result.push(args[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = args[start..].trim();
    if !last.is_empty() {
        result.push(last);
    }
    result
}

/// Generate a Java setter assignment for a target field path.
///
/// `output.total` -> `result.setTotal(<value>);`
/// `summary.account_id` -> `result.setAccountId(<value>);`
pub fn generate_setter(target: &str, value_expr: &str) -> String {
    let parts: Vec<&str> = target.split('.').collect();
    if parts.len() >= 2 {
        // Strip the output prefix (first part)
        let field_parts = &parts[1..];
        if field_parts.len() == 1 {
            let setter = format!("set{}", to_pascal_case(field_parts[0]));
            format!("result.{setter}({value_expr});")
        } else {
            // Nested: result.getSub().setField(value)
            let mut chain = "result".to_string();
            for part in &field_parts[..field_parts.len() - 1] {
                chain.push_str(&format!(".get{}()", to_pascal_case(part)));
            }
            let setter = format!(
                "set{}",
                to_pascal_case(field_parts[field_parts.len() - 1])
            );
            format!("{chain}.{setter}({value_expr});")
        }
    } else {
        // No dot -- direct assignment
        let setter = format!("set{}", to_pascal_case(target));
        format!("result.{setter}({value_expr});")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_literal() {
        assert_eq!(translate_expression("\"hello\""), "\"hello\"");
    }

    #[test]
    fn test_numeric_literal() {
        assert_eq!(translate_expression("42"), "42");
        assert_eq!(translate_expression("3.14"), "3.14");
    }

    #[test]
    fn test_boolean_literal() {
        assert_eq!(translate_expression("true"), "true");
        assert_eq!(translate_expression("false"), "false");
    }

    #[test]
    fn test_field_path_simple() {
        assert_eq!(
            translate_expression("input.amount"),
            "input.getAmount()"
        );
    }

    #[test]
    fn test_field_path_nested() {
        assert_eq!(
            translate_expression("input.customer.name"),
            "input.getCustomer().getName()"
        );
    }

    #[test]
    fn test_null_coalesce() {
        let result = translate_expression("input.name ?? \"unknown\"");
        assert!(result.contains("!= null"));
        assert!(result.contains("\"unknown\""));
    }

    #[test]
    fn test_logical_operators() {
        let result = translate_expression("input.active and input.verified");
        assert!(result.contains("&&"));

        let result = translate_expression("input.a or input.b");
        assert!(result.contains("||"));
    }

    #[test]
    fn test_function_round() {
        let result = translate_expression("round(input.amount, 2)");
        assert!(result.contains("setScale"));
        assert!(result.contains("HALF_UP"));
    }

    #[test]
    fn test_function_upper() {
        let result = translate_expression("upper(input.name)");
        assert!(result.contains("toUpperCase()"));
    }

    #[test]
    fn test_function_concat() {
        let result = translate_expression("concat(input.first, input.last)");
        assert!(result.contains("String.valueOf"));
        assert!(result.contains(" + "));
    }

    #[test]
    fn test_function_generate_uuid() {
        let result = translate_expression("generate_uuid()");
        assert!(result.contains("UUID.randomUUID()"));
    }

    #[test]
    fn test_function_now() {
        let result = translate_expression("now()");
        assert!(result.contains("Instant.now()"));
    }

    #[test]
    fn test_collection_ops() {
        let result = translate_expression("sum(transactions, t -> t.amount)");
        assert!(result.contains("NexflowRuntime.sum("));
    }

    #[test]
    fn test_setter_simple() {
        assert_eq!(
            generate_setter("output.total", "calculated"),
            "result.setTotal(calculated);"
        );
    }

    #[test]
    fn test_setter_nested() {
        assert_eq!(
            generate_setter("output.address.street", "value"),
            "result.getAddress().setStreet(value);"
        );
    }

    #[test]
    fn test_arithmetic_with_field_paths() {
        let result = translate_expression("input.amount * 0.02");
        assert!(result.contains("input.getAmount()"));
        assert!(result.contains("* 0.02"));
    }

    #[test]
    fn test_split_args() {
        let args = split_args("input.a, round(input.b, 2), \"hello\"");
        assert_eq!(args.len(), 3);
        assert_eq!(args[0], "input.a");
        assert_eq!(args[1], "round(input.b, 2)");
        assert_eq!(args[2], "\"hello\"");
    }

    #[test]
    fn test_lookups_path() {
        let result = translate_expression("lookups.customer.name");
        assert!(result.contains("lookupCustomer"));
        assert!(result.contains("getName()"));
    }
}
