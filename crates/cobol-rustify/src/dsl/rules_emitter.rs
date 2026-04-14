//! Rules emitter: generates `.rules` files from paragraph functions.
//!
//! Analyzes Rust function bodies via `syn` to detect decision patterns:
//! - `match` expressions (from COBOL EVALUATE) -> `decision_table`
//! - `if/else if` chains (from COBOL IF/ELSE IF) -> procedural `rule`
//!
//! Functions without decision logic are skipped (handled by TransformEmitter).

use std::collections::HashMap;

use super::cobol_attrs::{
    extract_annotated_fields, extract_annotated_fns, extract_program_name, AnnotatedField,
    AnnotatedFn, FieldCobolAttr,
};
use super::{DslEmitter, DslFile, DslLayer, EmitterContext};

/// Emitter that produces `.rules` files from paragraph functions with decision logic.
#[derive(Debug)]
pub struct RulesEmitter;

impl DslEmitter for RulesEmitter {
    fn id(&self) -> &'static str {
        "rules"
    }

    fn layer(&self) -> DslLayer {
        DslLayer::Rules
    }

    fn emit(&self, ctx: &EmitterContext<'_>) -> Vec<DslFile> {
        let fns = extract_annotated_fns(ctx.syn_file);
        let fields = extract_annotated_fields(ctx.syn_file);
        let program = extract_program_name(ctx.syn_file)
            .unwrap_or_else(|| ctx.program_name.clone());

        let field_map = build_field_map(&fields);

        // Analyze each function for decision patterns (may produce multiple rules per function)
        let mut candidates = Vec::new();
        for f in &fns {
            candidates.extend(analyze_function(f, ctx.syn_file, &field_map));
        }

        if candidates.is_empty() {
            return vec![];
        }

        // Group by section
        let sections = group_by_section(&candidates);

        let mut dsl_files = Vec::new();
        for (section_name, section_rules) in &sections {
            let (content, notes, confidence) =
                generate_rules_file(section_name, section_rules, &program);
            let source_fns: Vec<String> = section_rules
                .iter()
                .map(|r| r.cobol_name.clone())
                .collect();

            dsl_files.push(DslFile {
                path: format!(
                    "rules/{}.rules",
                    sanitize_identifier(section_name)
                ),
                content,
                confidence,
                notes,
                source_fields: source_fns,
            });
        }

        dsl_files
    }
}

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

/// Extracted branch: (condition_string, Vec<(field, value)>)
type ExtractedBranch = (String, Vec<(String, String)>);

/// Classification of a paragraph's decision logic.
#[derive(Debug, Clone)]
pub enum RuleShape {
    /// `match` expression with multiple arms -> decision_table
    DecisionTable {
        /// Number of match arms detected
        arm_count: usize,
        /// Scrutinee expression (what is being matched)
        scrutinee: String,
        /// Extracted conditions and actions from each WHEN branch
        branches: Vec<ExtractedBranch>,
    },
    /// `if/else if` chain that looks tabular -> decision_table
    #[allow(dead_code)]
    TabularIfChain {
        /// Number of branches
        branch_count: usize,
        /// Whether the chain has a final else (wildcard row)
        has_else: bool,
        /// Extracted conditions and actions from each branch
        branches: Vec<ExtractedBranch>,
    },
    /// `if/else if` chain with complex logic -> procedural rule
    #[allow(dead_code)]
    ProceduralRule {
        /// Number of branches (if + else if + else)
        branch_count: usize,
        /// Whether the chain has a final else
        has_else: bool,
        /// Extracted conditions and actions from each branch
        branches: Vec<ExtractedBranch>,
    },
}

/// A function that has been analyzed and classified as containing rule logic.
#[derive(Debug)]
pub struct RuleCandidate {
    /// COBOL paragraph name
    pub cobol_name: String,
    /// Nexflow-valid rule name
    pub nexflow_name: String,
    /// Section this paragraph belongs to
    pub section: Option<String>,
    /// Detected rule shape
    pub shape: RuleShape,
    /// Fields this paragraph reads
    pub reads: Vec<String>,
    /// Fields this paragraph writes
    pub writes: Vec<String>,
    /// Other paragraphs this performs (for context, not emitted as rules)
    #[allow(dead_code)]
    pub performs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Field map
// ---------------------------------------------------------------------------

fn build_field_map(fields: &[AnnotatedField]) -> HashMap<String, (String, FieldCobolAttr)> {
    let mut map = HashMap::new();
    for f in fields {
        if let Some(attr) = &f.cobol_attr {
            let cobol_name = rust_name_to_cobol(&f.rust_name);
            map.insert(cobol_name, (f.rust_name.clone(), attr.clone()));
        }
    }
    map
}

// ---------------------------------------------------------------------------
// Function analysis -- detect decision patterns in Rust bodies
// ---------------------------------------------------------------------------

/// Analyze a function for decision logic patterns.
/// Returns all decision patterns found in the function body.
/// A paragraph with 5 independent if/else chains produces 5 rule candidates.
fn analyze_function(
    f: &AnnotatedFn,
    syn_file: &syn::File,
    _field_map: &HashMap<String, (String, FieldCobolAttr)>,
) -> Vec<RuleCandidate> {
    let attr = match f.cobol_attr.as_ref() {
        Some(a) => a,
        None => return vec![],
    };

    // Skip boilerplate
    if is_boilerplate(&f.name) {
        return vec![];
    }

    // Skip pure orchestrators (only performs, no reads/writes) -- these are process-level
    if attr.reads.is_empty() && attr.writes.is_empty() && !attr.performs.is_empty() {
        return vec![];
    }

    // Find the function in the syn file and extract ALL decision patterns
    let shapes = find_all_decisions(syn_file, &f.name);

    let base_name = sanitize_identifier(&f.name);
    let cobol_name = rust_name_to_cobol(&f.name);

    if shapes.len() <= 1 {
        // Single decision or none -- emit with the paragraph name
        shapes
            .into_iter()
            .map(|shape| RuleCandidate {
                cobol_name: cobol_name.clone(),
                nexflow_name: base_name.clone(),
                section: attr.section.clone(),
                shape,
                reads: attr.reads.clone(),
                writes: attr.writes.clone(),
                performs: attr.performs.clone(),
            })
            .collect()
    } else {
        // Multiple decisions -- each gets a numbered suffix
        shapes
            .into_iter()
            .enumerate()
            .map(|(i, shape)| RuleCandidate {
                cobol_name: cobol_name.clone(),
                nexflow_name: format!("{}_{}", base_name, i + 1),
                section: attr.section.clone(),
                shape,
                reads: attr.reads.clone(),
                writes: attr.writes.clone(),
                performs: attr.performs.clone(),
            })
            .collect()
    }
}

/// Find a function by name and extract ALL decision patterns from its body.
fn find_all_decisions(syn_file: &syn::File, fn_name: &str) -> Vec<RuleShape> {
    for item in &syn_file.items {
        if let syn::Item::Fn(func) = item {
            if func.sig.ident == fn_name {
                return collect_all_decisions(&func.block);
            }
        }
    }
    vec![]
}

/// Collect all independent decision patterns from a block.
fn collect_all_decisions(block: &syn::Block) -> Vec<RuleShape> {
    let mut shapes = Vec::new();

    for stmt in &block.stmts {
        let shape = match stmt {
            syn::Stmt::Expr(expr, _) => analyze_expr(expr),
            syn::Stmt::Local(syn::Local {
                init: Some(syn::LocalInit { expr, .. }),
                ..
            }) => analyze_expr(expr),
            _ => None,
        };

        if let Some(s) = shape {
            shapes.push(s);
        }
    }

    shapes
}

/// Analyze an expression for decision patterns.
fn analyze_expr(expr: &syn::Expr) -> Option<RuleShape> {
    match expr {
        syn::Expr::Match(m) => {
            let arm_count = m.arms.len();
            if arm_count >= 2 {
                let scrutinee = quote::quote!(#(m.expr)).to_string();
                // Truncate long scrutinee for readability
                let scrutinee = if scrutinee.len() > 60 {
                    format!("{}...", &scrutinee[..57])
                } else {
                    scrutinee
                };
                Some(RuleShape::DecisionTable {
                    arm_count,
                    scrutinee,
                    branches: vec![],
                })
            } else {
                None
            }
        }
        syn::Expr::If(if_expr) => {
            let (branch_count, has_else) = count_if_branches(if_expr);
            if branch_count >= 2 {
                // Extract conditions and actions from all branches
                let branches = super::expr_extract::extract_if_chain(if_expr);

                // Check if the chain is tabular (simple comparisons on same var)
                if is_tabular_if_chain(if_expr) {
                    Some(RuleShape::TabularIfChain {
                        branch_count,
                        has_else,
                        branches,
                    })
                } else {
                    Some(RuleShape::ProceduralRule {
                        branch_count,
                        has_else,
                        branches,
                    })
                }
            } else {
                // Single if -- might contain nested decisions
                analyze_if_body(if_expr)
            }
        }
        syn::Expr::Block(b) => collect_all_decisions(&b.block).into_iter().next(),
        _ => None,
    }
}

/// Count branches in an if/else if chain.
fn count_if_branches(if_expr: &syn::ExprIf) -> (usize, bool) {
    let mut count = 1; // The initial `if`
    let mut has_else = false;
    let mut current = if_expr.else_branch.as_ref();

    while let Some((_, else_expr)) = current {
        match else_expr.as_ref() {
            syn::Expr::If(nested_if) => {
                count += 1;
                current = nested_if.else_branch.as_ref();
            }
            syn::Expr::Block(_) => {
                has_else = true;
                break;
            }
            _ => break,
        }
    }

    (count, has_else)
}

/// Analyze the body of an if expression for nested decision patterns.
fn analyze_if_body(if_expr: &syn::ExprIf) -> Option<RuleShape> {
    // Check the then-branch for nested decisions
    if let Some(shape) = collect_all_decisions(&if_expr.then_branch).into_iter().next() {
        return Some(shape);
    }
    // Check the else-branch
    if let Some((_, else_expr)) = &if_expr.else_branch {
        return analyze_expr(else_expr);
    }
    None
}

/// Check if an if/else chain is "tabular" -- each branch is a simple comparison
/// suitable for conversion to a decision_table row.
///
/// A tabular chain has the pattern:
///   if <expr> <op> <literal> { ... }
///   else if <expr> <op> <literal> { ... }
///   else { ... }
///
/// We detect this by checking that the top-level condition is a binary comparison.
fn is_tabular_if_chain(if_expr: &syn::ExprIf) -> bool {
    // Check the first condition is a binary comparison
    if !is_simple_comparison(&if_expr.cond) {
        return false;
    }

    // Check all else-if branches too
    let mut current = if_expr.else_branch.as_ref();
    while let Some((_, else_expr)) = current {
        match else_expr.as_ref() {
            syn::Expr::If(nested_if) => {
                if !is_simple_comparison(&nested_if.cond) {
                    return false;
                }
                current = nested_if.else_branch.as_ref();
            }
            syn::Expr::Block(_) => break, // Final else is always OK
            _ => return false,
        }
    }

    true
}

/// Check if an expression is a simple or compound comparison suitable for a table row.
/// Accepts: `a > b`, `a == b`, `a >= 1 && a <= 3` (range), `a == "X" || a == "Y"` (set).
fn is_simple_comparison(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Binary(bin) => matches!(
            bin.op,
            syn::BinOp::Gt(_) | syn::BinOp::Lt(_) | syn::BinOp::Ge(_) | syn::BinOp::Le(_)
            | syn::BinOp::Eq(_) | syn::BinOp::Ne(_)
        ) || matches!(
            bin.op,
            syn::BinOp::And(_) | syn::BinOp::Or(_)
        ) && is_simple_comparison(&bin.left) && is_simple_comparison(&bin.right),
        syn::Expr::Paren(p) => is_simple_comparison(&p.expr),
        syn::Expr::Unary(syn::ExprUnary { op: syn::UnOp::Not(_), expr, .. }) => {
            is_simple_comparison(expr)
        }
        _ => false,
    }
}

/// Given two shapes, pick the more dominant one (decision_table > tabular > procedural).
#[allow(dead_code)]
fn pick_dominant_shape(existing: Option<RuleShape>, new: RuleShape) -> RuleShape {
    fn branch_count(shape: &RuleShape) -> usize {
        match shape {
            RuleShape::DecisionTable { arm_count, .. } => *arm_count,
            RuleShape::TabularIfChain { branch_count, .. } => *branch_count,
            RuleShape::ProceduralRule { branch_count, .. } => *branch_count,
        }
    }
    fn priority(shape: &RuleShape) -> u8 {
        match shape {
            RuleShape::DecisionTable { .. } => 3,
            RuleShape::TabularIfChain { .. } => 2,
            RuleShape::ProceduralRule { .. } => 1,
        }
    }

    match existing {
        None => new,
        Some(existing) => {
            let ep = priority(&existing);
            let np = priority(&new);
            if np > ep {
                new
            } else if ep > np {
                existing
            } else {
                // Same priority -- pick the one with more branches
                if branch_count(&new) > branch_count(&existing) { new } else { existing }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Grouping
// ---------------------------------------------------------------------------

pub fn group_by_section(rules: &[RuleCandidate]) -> Vec<(String, Vec<&RuleCandidate>)> {
    let mut map: HashMap<String, Vec<&RuleCandidate>> = HashMap::new();

    for r in rules {
        let key = r
            .section
            .as_deref()
            .map_or_else(|| "program_misc".to_string(), sanitize_identifier);
        map.entry(key).or_default().push(r);
    }

    let mut result: Vec<(String, Vec<&RuleCandidate>)> = map.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

// ---------------------------------------------------------------------------
// DSL generation
// ---------------------------------------------------------------------------

pub fn generate_rules_file(
    section_name: &str,
    rules: &[&RuleCandidate],
    program: &str,
) -> (String, Vec<String>, f64) {
    use super::dsl_ast::*;

    let mut items = Vec::new();
    let mut notes = Vec::new();

    for r in rules {
        match &r.shape {
            RuleShape::DecisionTable { arm_count, scrutinee, branches } => {
                let (dt, note) = build_match_decision_table(r, *arm_count, scrutinee, branches);
                items.push(RuleItem::DecisionTable(dt));
                notes.push(note);
            }
            RuleShape::TabularIfChain { branches, .. } => {
                let (dt, note) = build_tabular_decision_table(r, branches);
                items.push(RuleItem::DecisionTable(dt));
                notes.push(note);
            }
            RuleShape::ProceduralRule { branches, .. } => {
                let (pr, note) = build_procedural_rule(r, branches);
                items.push(RuleItem::ProceduralRule(pr));
                notes.push(note);
            }
        }
    }

    let file = RulesFile {
        comments: vec![
            Comment(format!("Generated by cobol2rust Nexflow emitter")),
            Comment(format!("Source: {program} section {section_name}")),
        ],
        imports: vec![ImportPath::schema(program)],
        items,
    };

    // Confidence is 1.0 -- output is grammar-valid by construction
    (file.to_text(), notes, 1.0)
}

fn build_given_params(r: &RuleCandidate) -> Vec<dsl_ast::GivenParam> {
    use super::dsl_ast::*;
    if r.reads.is_empty() {
        vec![GivenParam {
            name: Ident::new("input_value"),
            param_type: RulesParamType::Number,
        }]
    } else {
        r.reads.iter().map(|read| GivenParam {
            name: Ident::new(&cobol_to_snake(read)),
            param_type: cobol_name_to_typed_param(read),
        }).collect()
    }
}

fn build_return_params(r: &RuleCandidate) -> Vec<dsl_ast::ReturnParam> {
    use super::dsl_ast::*;
    if r.writes.is_empty() {
        vec![ReturnParam { name: Ident::new("result"), param_type: RulesParamType::Text }]
    } else {
        r.writes.iter().map(|w| ReturnParam {
            name: Ident::new(&cobol_to_snake(w)),
            param_type: cobol_name_to_typed_param(w),
        }).collect()
    }
}

/// Build a decision_table from a match expression (EVALUATE).
fn build_match_decision_table(
    r: &RuleCandidate,
    arm_count: usize,
    scrutinee: &str,
    branches: &[ExtractedBranch],
) -> (dsl_ast::DecisionTable, String) {
    use super::dsl_ast::*;

    // If we have extracted branches, use them (same logic as tabular)
    if !branches.is_empty() {
        let mut action_col_names: Vec<String> = Vec::new();
        for (_, actions) in branches {
            for (field, _) in actions {
                if !action_col_names.contains(field) {
                    action_col_names.push(field.clone());
                }
            }
        }
        if action_col_names.is_empty() {
            action_col_names.push("result".to_string());
        }

        let action_idents: Vec<Ident> = action_col_names.iter().map(|n| Ident::new(n)).collect();

        let rows: Vec<DecideRow> = branches.iter().map(|(condition, actions)| {
            let action_values: Vec<String> = action_col_names.iter().map(|col| {
                actions.iter()
                    .find(|(f, _)| f == col)
                    .map(|(_, v)| v.clone())
                    .unwrap_or_else(|| "*".to_string())
            }).collect();
            DecideRow {
                condition: condition.clone(),
                actions: action_values,
            }
        }).collect();

        let dt = DecisionTable {
            name: Ident::new(&r.nexflow_name),
            comment: Some(format!("COBOL paragraph: {} (EVALUATE {}, {} arms)", r.cobol_name, scrutinee, arm_count)),
            hit_policy: HitPolicy::FirstMatch,
            given: build_given_params(r),
            decide: DecideMatrix {
                condition_col: scrutinee.to_string(),
                action_cols: action_idents,
                rows,
            },
            return_params: build_return_params(r),
        };

        let note = format!("{}: EVALUATE decision table with {} extracted rows", r.cobol_name, branches.len());
        return (dt, note);
    }

    // Fallback: no extracted branches (should not happen with direct emitter)
    let action_col_names: Vec<Ident> = if r.writes.is_empty() {
        vec![Ident::new("result")]
    } else {
        r.writes.iter().map(|w| Ident::new(&cobol_to_snake(w))).collect()
    };

    let row_count = arm_count.min(10);
    let rows: Vec<DecideRow> = (0..row_count).map(|i| DecideRow {
        condition: "*".to_string(),
        actions: action_col_names.iter().map(|_| format!("\"row_{i}\"")).collect(),
    }).collect();

    let dt = DecisionTable {
        name: Ident::new(&r.nexflow_name),
        comment: Some(format!("COBOL paragraph: {} (EVALUATE, {} arms, scrutinee: {})", r.cobol_name, arm_count, scrutinee)),
        hit_policy: HitPolicy::FirstMatch,
        given: build_given_params(r),
        decide: DecideMatrix {
            condition_col: "condition".to_string(),
            action_cols: action_col_names,
            rows,
        },
        return_params: build_return_params(r),
    };

    let note = format!("{}: EVALUATE decision table with {} placeholder rows (extraction failed)", r.cobol_name, row_count);
    (dt, note)
}

/// Build a decision_table from extracted if/else chain branches.
fn build_tabular_decision_table(
    r: &RuleCandidate,
    branches: &[ExtractedBranch],
) -> (dsl_ast::DecisionTable, String) {
    use super::dsl_ast::*;

    // Collect action column names from branches
    let mut action_col_names: Vec<String> = Vec::new();
    for (_, actions) in branches {
        for (field, _) in actions {
            if !action_col_names.contains(field) {
                action_col_names.push(field.clone());
            }
        }
    }
    if action_col_names.is_empty() {
        action_col_names.push("result".to_string());
    }

    let action_idents: Vec<Ident> = action_col_names.iter().map(|n| Ident::new(n)).collect();

    let rows: Vec<DecideRow> = branches.iter().map(|(condition, actions)| {
        let action_values: Vec<String> = action_col_names.iter().map(|col| {
            actions.iter()
                .find(|(f, _)| f == col)
                .map(|(_, v)| v.clone())
                .unwrap_or_else(|| "*".to_string())
        }).collect();
        DecideRow {
            condition: condition.clone(),
            actions: action_values,
        }
    }).collect();

    let dt = DecisionTable {
        name: Ident::new(&r.nexflow_name),
        comment: Some(format!("COBOL paragraph: {} (tabular IF/ELSE -> decision_table)", r.cobol_name)),
        hit_policy: HitPolicy::FirstMatch,
        given: build_given_params(r),
        decide: DecideMatrix {
            condition_col: "condition".to_string(),
            action_cols: action_idents,
            rows,
        },
        return_params: build_return_params(r),
    };

    let note = format!("{}: tabular decision table with {} extracted rows", r.cobol_name, branches.len());
    (dt, note)
}

/// Build a procedural rule from extracted if/else branches.
fn build_procedural_rule(
    r: &RuleCandidate,
    branches: &[ExtractedBranch],
) -> (dsl_ast::ProceduralRule, String) {
    use super::dsl_ast::*;

    let mut body = Vec::new();

    if !branches.is_empty() {
        let first_cond = &branches[0].0;
        let first_actions = branches_to_stmts(&branches[0].1);

        let mut elseif_blocks = Vec::new();
        let mut else_block = None;

        for (cond, actions) in &branches[1..] {
            let stmts = branches_to_stmts(actions);
            if cond == "otherwise" {
                else_block = Some(stmts);
            } else {
                elseif_blocks.push((Expr::Raw(cond.clone()), stmts));
            }
        }

        body.push(RuleStmt::If(IfStmt {
            condition: Expr::Raw(first_cond.clone()),
            then_block: first_actions,
            elseif_blocks,
            else_block,
        }));
    }

    body.push(RuleStmt::Return);

    let pr = ProceduralRule {
        name: Ident::new(&r.nexflow_name),
        comment: Some(format!("COBOL paragraph: {} (IF/ELSE pattern)", r.cobol_name)),
        description: Some(infer_rule_description(&r.nexflow_name)),
        body,
    };

    let note = format!("{}: procedural rule with {} extracted branches", r.cobol_name, branches.len());
    (pr, note)
}

/// Convert extracted (field, value) pairs to RuleStmt::Set statements.
fn branches_to_stmts(actions: &[(String, String)]) -> Vec<dsl_ast::RuleStmt> {
    use super::dsl_ast::*;
    actions.iter().map(|(field, value)| {
        RuleStmt::Set(Ident::new(field), Expr::Raw(value.clone()))
    }).collect()
}

use super::dsl_ast;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn is_boilerplate(name: &str) -> bool {
    matches!(
        name,
        "run" | "main" | "new" | "stop_run" | "goback"
    )
}

fn cobol_to_snake(name: &str) -> String {
    name.to_lowercase().replace('-', "_")
}

fn rust_name_to_cobol(name: &str) -> String {
    name.to_uppercase().replace('_', "-")
}

pub fn sanitize_identifier(name: &str) -> String {
    let s = name.to_lowercase().replace(['-', ' '], "_");
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{s}")
    } else if s.is_empty() {
        "field".to_string()
    } else {
        s
    }
}

/// Map COBOL field names to typed RulesDSL param types using naming heuristics.
fn cobol_name_to_typed_param(cobol_name: &str) -> dsl_ast::RulesParamType {
    use dsl_ast::RulesParamType;
    let lower = cobol_name.to_lowercase();
    if lower.contains("flag") || lower.contains("switch") || lower.contains("ind")
        || lower.contains("active") || lower.contains("valid")
    {
        RulesParamType::Boolean
    } else if lower.contains("name") || lower.contains("desc") || lower.contains("text")
        || lower.contains("addr") || lower.contains("msg") || lower.contains("code")
        || lower.contains("type") || lower.contains("status") || lower.contains("reason")
    {
        RulesParamType::Text
    } else if lower.contains("date") || lower.ends_with("-dt") || lower.ends_with("-dte") {
        RulesParamType::Date
    } else if lower.contains("amt") || lower.contains("amount") || lower.contains("balance")
        || lower.contains("rate") || lower.contains("price") || lower.contains("total")
        || lower.contains("limit") || lower.contains("income")
    {
        RulesParamType::Money
    } else if lower.contains("count") || lower.contains("num") || lower.contains("qty")
        || lower.contains("score") || lower.contains("idx")
    {
        RulesParamType::Integer
    } else if lower.contains("pct") || lower.contains("percent") || lower.contains("ratio") {
        RulesParamType::Percentage
    } else {
        RulesParamType::Number
    }
}

/// Infer a human-readable description from a rule name.
fn infer_rule_description(name: &str) -> String {
    let clean = name.replace('_', " ");
    // Capitalize first letter
    let mut chars = clean.chars();
    match chars.next() {
        None => "Business rule".to_string(),
        Some(first) => {
            format!(
                "{}{}",
                first.to_uppercase(),
                chars.as_str()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_boilerplate() {
        assert!(is_boilerplate("run"));
        assert!(is_boilerplate("main"));
        assert!(is_boilerplate("stop_run"));
        assert!(!is_boilerplate("validate_input"));
    }

    #[test]
    fn rules_type_heuristics() {
        use super::dsl_ast::RulesParamType;
        assert!(matches!(cobol_name_to_typed_param("WS-ERR-FLAG"), RulesParamType::Boolean));
        assert!(matches!(cobol_name_to_typed_param("WS-ACCT-TYPE"), RulesParamType::Text));
        assert!(matches!(cobol_name_to_typed_param("WS-OPEN-DATE"), RulesParamType::Date));
        assert!(matches!(cobol_name_to_typed_param("WS-BALANCE"), RulesParamType::Money));
        assert!(matches!(cobol_name_to_typed_param("WS-SCORE"), RulesParamType::Integer));
        assert!(matches!(cobol_name_to_typed_param("WS-DTI-RATIO"), RulesParamType::Percentage));
        assert!(matches!(cobol_name_to_typed_param("WS-RESULT"), RulesParamType::Number));
    }

    #[test]
    fn infer_description_from_name() {
        assert_eq!(
            infer_rule_description("validate_income"),
            "Validate income"
        );
        assert_eq!(
            infer_rule_description("check_exposure"),
            "Check exposure"
        );
    }

    #[test]
    fn sanitize_names() {
        assert_eq!(sanitize_identifier("WS-ACCT-TYPE"), "ws_acct_type");
        assert_eq!(sanitize_identifier("123bad"), "_123bad");
        assert_eq!(sanitize_identifier(""), "field");
    }

    #[test]
    fn detect_match_as_decision_table() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "X(1)")]
                pub ws_acct_type: PicX,
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_rate: PackedDecimal,
            }

            #[cobol(section = "RATE-SECTION", reads = "WS-ACCT-TYPE", writes = "WS-RATE")]
            fn determine_rate(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
                match ws.ws_acct_type.to_string().as_str() {
                    "S" => { ws.ws_rate.pack(dec!(3)); }
                    "C" => { ws.ws_rate.pack(dec!(5)); }
                    "P" => { ws.ws_rate.pack(dec!(2)); }
                    _ => { ws.ws_rate.pack(dec!(4)); }
                }
            }
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = RulesEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert!(!files.is_empty(), "Should produce rules file for match pattern");

        let content = &files[0].content;
        assert!(
            content.contains("decision_table"),
            "Match arms should produce decision_table: {content}"
        );
        assert!(
            content.contains("hit_policy first_match"),
            "Should have hit_policy: {content}"
        );
        assert!(
            content.contains("given:"),
            "Should have given block: {content}"
        );
        assert!(
            content.contains("decide:"),
            "Should have decide block: {content}"
        );
        assert!(
            content.contains("return:"),
            "Should have return block: {content}"
        );
        assert!(content.contains("end"), "Should be terminated with end");
    }

    #[test]
    fn detect_tabular_if_chain_as_decision_table() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_score: PackedDecimal,
                #[cobol(level = 1, pic = "X(10)")]
                pub ws_decision: PicX,
            }

            #[cobol(section = "DECIDE-SECTION", reads = "WS-SCORE", writes = "WS-DECISION")]
            fn check_score(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
                if ws.ws_score.to_decimal() > dec!(750) {
                    move_alphanumeric_literal(b"APPROVED", &mut ws.ws_decision, &ctx.config);
                } else if ws.ws_score.to_decimal() > dec!(650) {
                    move_alphanumeric_literal(b"REVIEW", &mut ws.ws_decision, &ctx.config);
                } else {
                    move_alphanumeric_literal(b"DECLINED", &mut ws.ws_decision, &ctx.config);
                }
            }
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = RulesEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert!(!files.is_empty(), "Should produce rules file for tabular if/else");

        let content = &files[0].content;
        // Tabular if/else chains should now produce decision_table (not procedural rule)
        assert!(
            content.contains("decision_table check_score"),
            "Tabular if/else should produce decision_table: {content}"
        );
        assert!(
            content.contains("hit_policy first_match"),
            "Should have hit_policy: {content}"
        );
        assert!(
            content.contains("given:"),
            "Should have given block: {content}"
        );
        assert!(
            content.contains("decide:"),
            "Should have decide block: {content}"
        );
        assert!(
            content.contains("return:"),
            "Should have return block: {content}"
        );
    }

    #[test]
    fn detect_complex_if_as_procedural_rule() {
        // Complex conditions (not simple comparisons) should stay as procedural rule
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "X(1)")]
                pub ws_flag: PicX,
                #[cobol(level = 1, pic = "X(10)")]
                pub ws_result: PicX,
            }

            #[cobol(section = "CHECK-SECTION", reads = "WS-FLAG", writes = "WS-RESULT")]
            fn check_flag(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
                if ws.ws_flag.to_string() == "Y" && ws.ws_result.to_string() != "DONE" {
                    move_alphanumeric_literal(b"OK", &mut ws.ws_result, &ctx.config);
                } else if ws.ws_flag.to_string() == "N" || ws.ws_result.to_string() == "ERR" {
                    move_alphanumeric_literal(b"FAIL", &mut ws.ws_result, &ctx.config);
                }
            }
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = RulesEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert!(!files.is_empty(), "Should produce rules file");

        let content = &files[0].content;
        // Compound conditions (&&, ||) of simple comparisons are now tabular -> decision_table
        assert!(
            content.contains("decision_table check_flag") || content.contains("rule check_flag:"),
            "Should produce decision_table or rule: {content}"
        );
    }

    #[test]
    fn skip_no_decision_logic() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
            }

            #[cobol(section = "INIT-SECTION", writes = "WS-COUNT")]
            fn init_count(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
                ws.ws_count.pack(dec!(0));
            }
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = RulesEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert!(
            files.is_empty(),
            "Simple assignment should not produce rules: got {:?}",
            files.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
    }

    #[test]
    fn skip_orchestrators() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
            }

            #[cobol(section = "MAIN-SECTION", performs = "INIT-PARA,CALC-PARA")]
            fn main_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
                // orchestrator -- only calls others
            }
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = RulesEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert!(files.is_empty(), "Orchestrators should not produce rules");
    }

    #[test]
    fn multi_section_produces_multiple_files() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_score: PackedDecimal,
                #[cobol(level = 1, pic = "X(10)")]
                pub ws_decision: PicX,
                #[cobol(level = 1, pic = "X(1)")]
                pub ws_flag: PicX,
            }

            #[cobol(section = "SECTION-A", reads = "WS-SCORE", writes = "WS-DECISION")]
            fn check_score(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
                if ws.ws_score.to_decimal() > dec!(750) {
                    move_alphanumeric_literal(b"OK", &mut ws.ws_decision, &ctx.config);
                } else if ws.ws_score.to_decimal() > dec!(500) {
                    move_alphanumeric_literal(b"MAYBE", &mut ws.ws_decision, &ctx.config);
                } else {
                    move_alphanumeric_literal(b"NO", &mut ws.ws_decision, &ctx.config);
                }
            }

            #[cobol(section = "SECTION-B", reads = "WS-FLAG", writes = "WS-DECISION")]
            fn check_flag(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
                match ws.ws_flag.to_string().as_str() {
                    "Y" => { move_alphanumeric_literal(b"YES", &mut ws.ws_decision, &ctx.config); }
                    "N" => { move_alphanumeric_literal(b"NO", &mut ws.ws_decision, &ctx.config); }
                    _ => { move_alphanumeric_literal(b"UNK", &mut ws.ws_decision, &ctx.config); }
                }
            }
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = RulesEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert_eq!(
            files.len(),
            2,
            "Two sections should produce two .rules files, got: {:?}",
            files.iter().map(|f| &f.path).collect::<Vec<_>>()
        );

        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.contains(&"rules/section_a.rules"));
        assert!(paths.contains(&"rules/section_b.rules"));

        // Section A should have decision_table (tabular if/else)
        let section_a = files.iter().find(|f| f.path.contains("section_a")).unwrap();
        assert!(section_a.content.contains("decision_table check_score"));

        // Section B should have decision table
        let section_b = files.iter().find(|f| f.path.contains("section_b")).unwrap();
        assert!(section_b.content.contains("decision_table check_flag"));
    }

    #[test]
    fn decision_table_respects_reads_writes() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "X(1)")]
                pub ws_acct_type: PicX,
                #[cobol(level = 1, pic = "9(3)")]
                pub ws_acct_score: PackedDecimal,
                #[cobol(level = 1, pic = "9(5)V99")]
                pub ws_rate: PackedDecimal,
                #[cobol(level = 1, pic = "X(10)")]
                pub ws_category: PicX,
            }

            #[cobol(section = "RATE-SECTION", reads = "WS-ACCT-TYPE,WS-ACCT-SCORE", writes = "WS-RATE,WS-CATEGORY")]
            fn determine_rate(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
                match ws.ws_acct_type.to_string().as_str() {
                    "S" => { ws.ws_rate.pack(dec!(3)); }
                    "C" => { ws.ws_rate.pack(dec!(5)); }
                    _ => { ws.ws_rate.pack(dec!(4)); }
                }
            }
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = RulesEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert_eq!(files.len(), 1);

        let content = &files[0].content;
        // Given block should list reads
        assert!(content.contains("ws_acct_type"), "Should have acct_type in given: {content}");
        assert!(content.contains("ws_acct_score"), "Should have acct_score in given: {content}");
        // Return block should list writes
        assert!(content.contains("ws_rate"), "Should have rate in return: {content}");
        assert!(content.contains("ws_category"), "Should have category in return: {content}");
        // Type heuristics
        assert!(content.contains("text"), "acct_type should be text type: {content}");
        assert!(content.contains("money"), "rate should be money type: {content}");
    }

    #[test]
    fn confidence_degrades_appropriately() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "X(10)")]
                pub ws_result: PicX,
            }

            #[cobol(section = "TEST-SECTION", writes = "WS-RESULT")]
            fn test_rule(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
                if true {
                    move_alphanumeric_literal(b"A", &mut ws.ws_result, &ctx.config);
                } else if true {
                    move_alphanumeric_literal(b"B", &mut ws.ws_result, &ctx.config);
                }
            }
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = RulesEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert_eq!(files.len(), 1);
        // Typed AST: confidence is 1.0 (grammar-valid by construction)
        assert!(
            (files[0].confidence - 1.0).abs() < f64::EPSILON,
            "Typed AST confidence should be 1.0, got {}",
            files[0].confidence
        );
    }
}
