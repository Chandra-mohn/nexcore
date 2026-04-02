// NexCore -- Nexflow Codegen: Rust Rules Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates pure Rust functions from RulesDSL ASTs.
//!
//! - Decision tables -> `pub fn evaluate_*() -> Option<Output>` with if/else chains
//! - Procedural rules -> `pub fn apply_*() -> HashMap<String, Value>`
//!
//! No framework dependencies. Uses serde_json::Value for dynamic results
//! and typed structs where input/output schemas are known.

use std::fmt::Write;

use nexflow_parser::ast::rules::{
    ActionExpr, BlockItem, CellContent, ConditionExpr, DecisionTableDef, HitPolicy,
    ProceduralRuleDef, RulesProgram,
};

use crate::naming::snake_to_pascal;

/// Generate a complete Rust module from a `RulesProgram`.
///
/// Returns `(filename, content)` -- typically `("rules.rs", ...)`.
pub fn generate_rust_rules(program: &RulesProgram) -> (String, String) {
    let mut out = String::with_capacity(8192);

    writeln!(out, "//! Generated rules from Nexflow RulesDSL.").unwrap();
    writeln!(out, "//! DO NOT EDIT -- regenerate from .rules files.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#![allow(unused_variables, dead_code, clippy::needless_return)]").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use std::collections::HashMap;").unwrap();
    writeln!(out).unwrap();

    for table in &program.decision_tables {
        generate_decision_table(&mut out, table);
    }

    for rule in &program.procedural_rules {
        generate_procedural_rule(&mut out, rule);
    }

    ("rules.rs".to_string(), out)
}

/// Generate a decision table evaluator function.
fn generate_decision_table(out: &mut String, table: &DecisionTableDef) {
    let fn_name = format!("evaluate_{}", table.name);
    let hit_policy = table.hit_policy.as_ref().unwrap_or(&HitPolicy::FirstMatch);

    // Output type
    let has_multi_return = table.returns.len() > 1;
    let output_type = if has_multi_return {
        let struct_name = format!("{}Output", snake_to_pascal(&table.name));
        // Generate output struct
        writeln!(out, "#[derive(Debug, Clone)]").unwrap();
        writeln!(out, "pub struct {struct_name} {{").unwrap();
        for r in &table.returns {
            let rust_type = rules_type_to_rust(&r.param_type);
            writeln!(out, "    pub {}: {rust_type},", r.name).unwrap();
        }
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        struct_name
    } else if table.returns.len() == 1 {
        rules_type_to_rust(&table.returns[0].param_type)
    } else {
        "()".to_string()
    };

    // Input struct
    let input_struct = format!("{}Input", snake_to_pascal(&table.name));
    writeln!(out, "#[derive(Debug, Clone)]").unwrap();
    writeln!(out, "pub struct {input_struct} {{").unwrap();
    for p in &table.inputs {
        let rust_type = rules_type_to_rust(&p.param_type);
        writeln!(out, "    pub {}: {rust_type},", p.name).unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // Doc comment
    if let Some(desc) = &table.description {
        writeln!(out, "/// {desc}").unwrap();
    }
    writeln!(out, "///").unwrap();
    writeln!(out, "/// Hit policy: {hit_policy:?}").unwrap();

    // Function signature based on hit policy
    let return_type = match hit_policy {
        HitPolicy::FirstMatch => format!("Option<{output_type}>"),
        HitPolicy::MultiHit | HitPolicy::CollectAll => format!("Vec<{output_type}>"),
        HitPolicy::SingleHit => output_type.clone(),
    };

    writeln!(out, "pub fn {fn_name}(input: &{input_struct}) -> {return_type} {{").unwrap();

    let input_col_count = table.inputs.len().min(table.columns.len());

    match hit_policy {
        HitPolicy::FirstMatch => {
            for (row_idx, row) in table.rows.iter().enumerate() {
                let conditions = build_row_conditions(row, table, input_col_count);
                let result = build_row_result(row, table, input_col_count, &output_type, has_multi_return);

                if conditions.is_empty() || conditions.iter().all(|c| c == "true") {
                    writeln!(out, "    // Row {row_idx}: catch-all").unwrap();
                    writeln!(out, "    return Some({result});").unwrap();
                } else {
                    let cond_str = conditions
                        .iter()
                        .filter(|c| *c != "true")
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(" && ");
                    writeln!(out, "    // Row {row_idx}").unwrap();
                    writeln!(out, "    if {cond_str} {{").unwrap();
                    writeln!(out, "        return Some({result});").unwrap();
                    writeln!(out, "    }}").unwrap();
                }
            }
            writeln!(out, "    None").unwrap();
        }
        HitPolicy::MultiHit | HitPolicy::CollectAll => {
            writeln!(out, "    let mut results = Vec::new();").unwrap();
            for (row_idx, row) in table.rows.iter().enumerate() {
                let conditions = build_row_conditions(row, table, input_col_count);
                let result = build_row_result(row, table, input_col_count, &output_type, has_multi_return);

                if conditions.is_empty() || conditions.iter().all(|c| c == "true") {
                    writeln!(out, "    results.push({result}); // Row {row_idx}: catch-all").unwrap();
                } else {
                    let cond_str = conditions
                        .iter()
                        .filter(|c| *c != "true")
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(" && ");
                    writeln!(out, "    if {cond_str} {{ results.push({result}); }} // Row {row_idx}").unwrap();
                }
            }
            writeln!(out, "    results").unwrap();
        }
        HitPolicy::SingleHit => {
            writeln!(out, "    let mut results = Vec::new();").unwrap();
            for (row_idx, row) in table.rows.iter().enumerate() {
                let conditions = build_row_conditions(row, table, input_col_count);
                let result = build_row_result(row, table, input_col_count, &output_type, has_multi_return);
                let cond_str = conditions
                    .iter()
                    .filter(|c| *c != "true")
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" && ");
                if cond_str.is_empty() {
                    writeln!(out, "    results.push({result}); // Row {row_idx}").unwrap();
                } else {
                    writeln!(out, "    if {cond_str} {{ results.push({result}); }} // Row {row_idx}").unwrap();
                }
            }
            writeln!(out, "    assert_eq!(results.len(), 1, \"Expected exactly 1 match, got {{}}\", results.len());").unwrap();
            writeln!(out, "    results.into_iter().next().unwrap()").unwrap();
        }
    }

    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();
}

/// Build condition expressions for a row.
fn build_row_conditions(
    row: &nexflow_parser::ast::rules::TableRow,
    table: &DecisionTableDef,
    input_col_count: usize,
) -> Vec<String> {
    let mut conditions = Vec::new();

    for col_idx in 0..input_col_count {
        if col_idx >= row.cells.len() {
            break;
        }
        let cell = &row.cells[col_idx];
        let field_name = &table.columns[col_idx];
        let param_type = table
            .inputs
            .get(col_idx)
            .map(|p| p.param_type.as_str())
            .unwrap_or("text");
        let accessor = format!("input.{field_name}");

        match cell {
            CellContent::Wildcard => conditions.push("true".to_string()),
            CellContent::Condition(cond) => {
                conditions.push(translate_rust_condition(cond, &accessor, param_type));
            }
            CellContent::Action(_) => conditions.push("true".to_string()),
        }
    }

    conditions
}

/// Build result expression for a row.
fn build_row_result(
    row: &nexflow_parser::ast::rules::TableRow,
    table: &DecisionTableDef,
    input_col_count: usize,
    output_type: &str,
    has_multi_return: bool,
) -> String {
    let output_cells: Vec<(&str, &CellContent)> = table.columns[input_col_count..]
        .iter()
        .enumerate()
        .filter_map(|(i, col)| {
            row.cells.get(input_col_count + i).map(|cell| (col.as_str(), cell))
        })
        .collect();

    if has_multi_return {
        let mut fields = Vec::new();
        for (col_name, cell) in &output_cells {
            let ret_type = table
                .returns
                .iter()
                .find(|r| r.name == *col_name)
                .map(|r| r.param_type.as_str())
                .unwrap_or("text");
            let value = translate_cell_value(cell, ret_type);
            fields.push(format!("{col_name}: {value}"));
        }
        format!("{output_type} {{ {} }}", fields.join(", "))
    } else if let Some((_, cell)) = output_cells.first() {
        let ret_type = table
            .returns
            .first()
            .map(|r| r.param_type.as_str())
            .unwrap_or("text");
        translate_cell_value(cell, ret_type)
    } else {
        "()".to_string()
    }
}

/// Translate a condition to a Rust boolean expression.
fn translate_rust_condition(cond: &ConditionExpr, accessor: &str, param_type: &str) -> String {
    match cond {
        ConditionExpr::ExactMatch(value) => {
            let cleaned = value.trim_matches('"');
            if is_string_type(param_type) {
                format!("{accessor} == \"{cleaned}\"")
            } else if cleaned == "true" || cleaned == "false" {
                format!("{accessor} == {cleaned}")
            } else {
                format!("{accessor} == {cleaned}")
            }
        }
        ConditionExpr::Range { from, to } => {
            format!("({accessor} >= {from} && {accessor} <= {to})")
        }
        ConditionExpr::InSet { values, negated } => {
            let items: String = values
                .iter()
                .map(|v| {
                    let cleaned = v.trim_matches('"');
                    if is_string_type(param_type) {
                        format!("\"{cleaned}\"")
                    } else {
                        cleaned.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let contains = format!("[{items}].contains(&{accessor})");
            if *negated {
                format!("!{contains}")
            } else {
                contains
            }
        }
        ConditionExpr::Pattern { kind, pattern } => match kind.as_str() {
            "starts_with" => format!("{accessor}.starts_with(\"{pattern}\")"),
            "ends_with" => format!("{accessor}.ends_with(\"{pattern}\")"),
            "contains" => format!("{accessor}.contains(\"{pattern}\")"),
            _ => format!("regex::Regex::new(\"{pattern}\").unwrap().is_match(&{accessor})"),
        },
        ConditionExpr::NullCheck { is_null } => {
            if *is_null {
                format!("{accessor}.is_none()")
            } else {
                format!("{accessor}.is_some()")
            }
        }
        ConditionExpr::Comparison { operator, value } => {
            let cleaned = value.trim_matches('"');
            if is_string_type(param_type) {
                match operator.as_str() {
                    "==" | "=" => format!("{accessor} == \"{cleaned}\""),
                    "!=" => format!("{accessor} != \"{cleaned}\""),
                    _ => format!("{accessor} {operator} \"{cleaned}\""),
                }
            } else {
                format!("{accessor} {operator} {cleaned}")
            }
        }
        ConditionExpr::Expression(expr) => {
            let translated = expr
                .replace(" and ", " && ")
                .replace(" or ", " || ")
                .replace(" not ", " !");
            format!("({translated})")
        }
        ConditionExpr::MarkerState { marker, state } => {
            format!("markers.get(\"{marker}\") == Some(&\"{state}\".to_string())")
        }
    }
}

/// Translate a cell value to a Rust expression.
fn translate_cell_value(cell: &CellContent, return_type: &str) -> String {
    match cell {
        CellContent::Wildcard => "Default::default()".to_string(),
        CellContent::Condition(cond) => match cond {
            ConditionExpr::ExactMatch(v) => format_rust_literal(v, return_type),
            _ => "Default::default()".to_string(),
        },
        CellContent::Action(action) => match action {
            ActionExpr::Assign(v) => format_rust_literal(v, return_type),
            ActionExpr::Calculate(expr) => expr.clone(),
            ActionExpr::Call { function, args } => {
                format!("{}({})", function, args.join(", "))
            }
            ActionExpr::Lookup { table, args, .. } => {
                format!("lookup(\"{table}\", &[{}])", args.join(", "))
            }
            ActionExpr::Emit { target } => {
                format!("emit(\"{target}\")")
            }
        },
    }
}

fn format_rust_literal(value: &str, return_type: &str) -> String {
    // Strip surrounding quotes if the AST stored them
    let cleaned = value.trim_matches('"');
    match return_type {
        "text" | "string" => format!("\"{cleaned}\".to_string()"),
        "number" | "integer" => {
            if cleaned.contains('.') {
                cleaned.to_string()
            } else {
                format!("{cleaned}_i64")
            }
        }
        "money" | "decimal" | "percentage" => {
            format!("dec!({cleaned})")
        }
        "boolean" => cleaned.to_string(),
        _ => format!("\"{cleaned}\".to_string()"),
    }
}

/// Generate a procedural rule function.
fn generate_procedural_rule(out: &mut String, rule: &ProceduralRuleDef) {
    let fn_name = format!("apply_{}", rule.name);

    if let Some(desc) = &rule.description {
        writeln!(out, "/// {desc}").unwrap();
    }

    writeln!(
        out,
        "pub fn {fn_name}(input: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {{"
    )
    .unwrap();
    writeln!(out, "    let mut result = input.clone();").unwrap();
    writeln!(out).unwrap();

    for item in &rule.body {
        write_rust_block_item(out, item, 1);
    }

    writeln!(out).unwrap();
    writeln!(out, "    result").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();
}

fn write_rust_block_item(out: &mut String, item: &BlockItem, indent: usize) {
    let pad = "    ".repeat(indent);

    match item {
        BlockItem::IfThenElse {
            condition,
            then_block,
            elseif_blocks,
            else_block,
        } => {
            let rust_cond = condition
                .replace(" and ", " && ")
                .replace(" or ", " || ")
                .replace(" not ", " !");
            writeln!(out, "{pad}if {rust_cond} {{").unwrap();
            for sub in then_block {
                write_rust_block_item(out, sub, indent + 1);
            }
            for (elseif_cond, elseif_body) in elseif_blocks {
                let rust_cond = elseif_cond
                    .replace(" and ", " && ")
                    .replace(" or ", " || ");
                writeln!(out, "{pad}}} else if {rust_cond} {{").unwrap();
                for sub in elseif_body {
                    write_rust_block_item(out, sub, indent + 1);
                }
            }
            if !else_block.is_empty() {
                writeln!(out, "{pad}}} else {{").unwrap();
                for sub in else_block {
                    write_rust_block_item(out, sub, indent + 1);
                }
            }
            writeln!(out, "{pad}}}").unwrap();
        }
        BlockItem::Set { name, expression } => {
            let rust_expr = super::expression::translate_rust_expr(expression);
            writeln!(
                out,
                "{pad}result.insert(\"{name}\".to_string(), serde_json::json!({rust_expr}));"
            )
            .unwrap();
        }
        BlockItem::Let { name, expression } => {
            let rust_expr = super::expression::translate_rust_expr(expression);
            writeln!(out, "{pad}let {name} = {rust_expr};").unwrap();
        }
        BlockItem::ActionCall { name, args } => {
            let args_str = args
                .iter()
                .map(|a| super::expression::translate_rust_expr(a))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(out, "{pad}{name}({args_str});").unwrap();
        }
        BlockItem::Return => {
            writeln!(out, "{pad}return result;").unwrap();
        }
    }
}

fn rules_type_to_rust(type_name: &str) -> String {
    match type_name {
        "text" | "string" => "String".to_string(),
        "number" | "integer" => "i64".to_string(),
        "money" | "decimal" | "percentage" => "Decimal".to_string(),
        "boolean" => "bool".to_string(),
        "date" | "bizdate" => "chrono::NaiveDate".to_string(),
        "timestamp" => "chrono::DateTime<chrono::Utc>".to_string(),
        other => snake_to_pascal(other),
    }
}

fn is_string_type(t: &str) -> bool {
    matches!(t, "text" | "string" | "String")
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::rules::*;

    fn make_credit_program() -> RulesProgram {
        RulesProgram {
            imports: Vec::new(),
            services: Vec::new(),
            actions: Vec::new(),
            decision_tables: vec![DecisionTableDef {
                name: "credit_scoring".to_string(),
                hit_policy: Some(HitPolicy::FirstMatch),
                description: Some("Determine credit tier".to_string()),
                version: None,
                inputs: vec![
                    InputParam { name: "income".to_string(), param_type: "number".to_string() },
                    InputParam { name: "history".to_string(), param_type: "text".to_string() },
                ],
                columns: vec![
                    "income".to_string(), "history".to_string(), "tier".to_string(),
                ],
                rows: vec![
                    TableRow {
                        priority: None,
                        cells: vec![
                            CellContent::Condition(ConditionExpr::Comparison {
                                operator: ">=".to_string(),
                                value: "100000".to_string(),
                            }),
                            CellContent::Condition(ConditionExpr::ExactMatch("excellent".to_string())),
                            CellContent::Action(ActionExpr::Assign("gold".to_string())),
                        ],
                    },
                    TableRow {
                        priority: None,
                        cells: vec![
                            CellContent::Wildcard,
                            CellContent::Wildcard,
                            CellContent::Action(ActionExpr::Assign("review".to_string())),
                        ],
                    },
                ],
                returns: vec![ReturnParam { name: "tier".to_string(), param_type: "text".to_string() }],
                execute: None,
                post_calculate: Vec::new(),
                aggregate: Vec::new(),
            }],
            procedural_rules: vec![ProceduralRuleDef {
                name: "transfer_check".to_string(),
                description: Some("Validate transfer".to_string()),
                body: vec![BlockItem::IfThenElse {
                    condition: "amount > 0 and amount <= balance".to_string(),
                    then_block: vec![BlockItem::Set {
                        name: "valid".to_string(),
                        expression: "true".to_string(),
                    }],
                    elseif_blocks: Vec::new(),
                    else_block: vec![BlockItem::Set {
                        name: "valid".to_string(),
                        expression: "false".to_string(),
                    }],
                }],
            }],
        }
    }

    #[test]
    fn test_generates_module() {
        let (filename, content) = generate_rust_rules(&make_credit_program());
        assert_eq!(filename, "rules.rs");
        assert!(content.contains("//! Generated rules"));
    }

    #[test]
    fn test_decision_table_input_struct() {
        let (_, content) = generate_rust_rules(&make_credit_program());
        assert!(content.contains("pub struct CreditScoringInput {"));
        assert!(content.contains("pub income: i64,"));
        assert!(content.contains("pub history: String,"));
    }

    #[test]
    fn test_decision_table_first_match() {
        let (_, content) = generate_rust_rules(&make_credit_program());
        assert!(content.contains("pub fn evaluate_credit_scoring(input: &CreditScoringInput) -> Option<String>"));
        assert!(content.contains("input.income >= 100000"));
        assert!(content.contains("input.history == \"excellent\""));
        assert!(content.contains("return Some(\"gold\".to_string())"));
        assert!(content.contains("return Some(\"review\".to_string())"));
        assert!(content.contains("None"));
    }

    #[test]
    fn test_procedural_rule() {
        let (_, content) = generate_rust_rules(&make_credit_program());
        assert!(content.contains("pub fn apply_transfer_check("));
        assert!(content.contains("if amount > 0 && amount <= balance {"));
        assert!(content.contains("result.insert(\"valid\".to_string()"));
    }

    #[test]
    fn test_condition_in_set() {
        let cond = ConditionExpr::InSet {
            values: vec!["A".to_string(), "B".to_string()],
            negated: false,
        };
        let result = translate_rust_condition(&cond, "input.status", "text");
        assert!(result.contains("[\"A\", \"B\"].contains(&input.status)"));
    }

    #[test]
    fn test_condition_range() {
        let cond = ConditionExpr::Range {
            from: "100".to_string(),
            to: "999".to_string(),
        };
        let result = translate_rust_condition(&cond, "input.score", "number");
        assert!(result.contains("input.score >= 100"));
        assert!(result.contains("input.score <= 999"));
    }
}
