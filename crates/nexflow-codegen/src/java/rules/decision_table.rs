// NexCore -- Nexflow Codegen: Decision Table Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates `*Table.java` decision table evaluator classes.
//!
//! Each decision table produces:
//! - `evaluate()` method implementing the hit policy
//! - `matchRowN()` methods for each row's condition matching
//! - `getRowNResult()` methods for each row's result extraction
//! - `execute()` adapter for Map-based Flink pipeline integration
//! - Optional `*Output` record if multiple return fields

use std::fmt::Write;

use nexflow_parser::ast::rules::{
    ActionExpr, CellContent, DecisionTableDef, HitPolicy, TableRow,
};

use crate::java::naming::{to_camel_case, to_pascal_case};

use super::condition::{rules_type_to_java, translate_condition};

const DEFAULT_PACKAGE: &str = "com.nexflow.rules";

/// Generate a decision table Java class.
///
/// Returns a list of `(filename, content)` pairs (table + optional output record).
pub fn generate_decision_table(table: &DecisionTableDef) -> Vec<(String, String)> {
    let mut files = Vec::new();

    let class_name = format!("{}Table", to_pascal_case(&table.name));
    let filename = format!("{class_name}.java");

    let has_multi_return = table.returns.len() > 1;
    let output_type = if has_multi_return {
        format!("{}Output", to_pascal_case(&table.name))
    } else if table.returns.len() == 1 {
        rules_type_to_java(&table.returns[0].param_type)
    } else {
        "Object".to_string()
    };

    // Generate output record if needed
    if has_multi_return {
        let (rec_file, rec_content) = generate_output_record(table);
        files.push((rec_file, rec_content));
    }

    let mut out = String::with_capacity(16384);

    // Package + imports
    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();
    write_imports(&mut out, table);
    writeln!(out).unwrap();

    // Class javadoc
    writeln!(out, "/**").unwrap();
    if let Some(desc) = &table.description {
        writeln!(out, " * {desc}").unwrap();
    } else {
        writeln!(out, " * Decision table: {}", table.name).unwrap();
    }
    writeln!(
        out,
        " * Generated from {}.rules -- DO NOT EDIT.",
        table.name
    )
    .unwrap();
    if let Some(hp) = &table.hit_policy {
        writeln!(out, " * Hit policy: {:?}", hp).unwrap();
    }
    writeln!(out, " */").unwrap();
    writeln!(out, "public class {class_name} {{").unwrap();
    writeln!(
        out,
        "    private static final Logger LOG = LoggerFactory.getLogger({class_name}.class);"
    )
    .unwrap();
    writeln!(out).unwrap();

    // Input record (inner class)
    write_input_record(&mut out, table);

    // evaluate() method
    write_evaluate_method(&mut out, table, &output_type);

    // Row methods
    for (i, row) in table.rows.iter().enumerate() {
        write_match_row(&mut out, i, row, table);
        write_result_row(&mut out, i, row, table, &output_type);
    }

    // execute() adapter
    write_execute_method(&mut out, table, &output_type);

    // Close class
    writeln!(out, "}}").unwrap();

    files.push((filename, out));
    files
}

/// Write the inner Input record class.
fn write_input_record(out: &mut String, table: &DecisionTableDef) {
    let input_name = format!("{}TableInput", to_pascal_case(&table.name));

    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Typed input for this decision table.").unwrap();
    writeln!(out, "     */").unwrap();
    write!(out, "    public record {input_name}(").unwrap();
    for (i, p) in table.inputs.iter().enumerate() {
        let java_type = rules_type_to_java(&p.param_type);
        let java_name = to_camel_case(&p.name);
        if i > 0 {
            write!(out, ", ").unwrap();
        }
        write!(out, "{java_type} {java_name}").unwrap();
    }
    writeln!(out, ") {{}}").unwrap();
    writeln!(out).unwrap();
}

/// Write the evaluate() method based on hit policy.
fn write_evaluate_method(out: &mut String, table: &DecisionTableDef, output_type: &str) {
    let input_name = format!("{}TableInput", to_pascal_case(&table.name));
    let hit_policy = table.hit_policy.as_ref().unwrap_or(&HitPolicy::FirstMatch);

    match hit_policy {
        HitPolicy::FirstMatch => {
            writeln!(out, "    /**").unwrap();
            writeln!(
                out,
                "     * Evaluate the decision table (first match)."
            )
            .unwrap();
            writeln!(out, "     */").unwrap();
            writeln!(
                out,
                "    public Optional<{output_type}> evaluate({input_name} input) {{"
            )
            .unwrap();
            for i in 0..table.rows.len() {
                writeln!(out, "        if (matchRow{i}(input)) {{").unwrap();
                writeln!(
                    out,
                    "            return Optional.of(getRow{i}Result(input));"
                )
                .unwrap();
                writeln!(out, "        }}").unwrap();
            }
            writeln!(out, "        return Optional.empty();").unwrap();
            writeln!(out, "    }}").unwrap();
        }
        HitPolicy::MultiHit | HitPolicy::CollectAll => {
            writeln!(out, "    /**").unwrap();
            writeln!(
                out,
                "     * Evaluate the decision table (multi-hit, returns all matches)."
            )
            .unwrap();
            writeln!(out, "     */").unwrap();
            writeln!(
                out,
                "    public List<{output_type}> evaluate({input_name} input) {{"
            )
            .unwrap();
            writeln!(
                out,
                "        List<{output_type}> results = new ArrayList<>();"
            )
            .unwrap();
            for i in 0..table.rows.len() {
                writeln!(out, "        if (matchRow{i}(input)) {{").unwrap();
                writeln!(
                    out,
                    "            results.add(getRow{i}Result(input));"
                )
                .unwrap();
                writeln!(out, "        }}").unwrap();
            }
            writeln!(out, "        return results;").unwrap();
            writeln!(out, "    }}").unwrap();
        }
        HitPolicy::SingleHit => {
            writeln!(out, "    /**").unwrap();
            writeln!(
                out,
                "     * Evaluate the decision table (single hit, expects exactly one match)."
            )
            .unwrap();
            writeln!(out, "     */").unwrap();
            writeln!(
                out,
                "    public {output_type} evaluate({input_name} input) {{"
            )
            .unwrap();
            writeln!(
                out,
                "        List<{output_type}> results = new ArrayList<>();"
            )
            .unwrap();
            for i in 0..table.rows.len() {
                writeln!(out, "        if (matchRow{i}(input)) {{").unwrap();
                writeln!(
                    out,
                    "            results.add(getRow{i}Result(input));"
                )
                .unwrap();
                writeln!(out, "        }}").unwrap();
            }
            writeln!(out, "        if (results.size() != 1) {{").unwrap();
            writeln!(
                out,
                "            throw new IllegalStateException(\"Expected exactly 1 match, got \" + results.size());"
            )
            .unwrap();
            writeln!(out, "        }}").unwrap();
            writeln!(out, "        return results.get(0);").unwrap();
            writeln!(out, "    }}").unwrap();
        }
    }
    writeln!(out).unwrap();
}

/// Write a matchRowN() method.
fn write_match_row(out: &mut String, idx: usize, row: &TableRow, table: &DecisionTableDef) {
    let input_name = format!("{}TableInput", to_pascal_case(&table.name));

    writeln!(
        out,
        "    private boolean matchRow{idx}({input_name} input) {{"
    )
    .unwrap();

    // Build conditions from the input columns
    let input_cols = &table.columns[..table.inputs.len().min(table.columns.len())];
    let mut conditions = Vec::new();

    for (col_idx, col_name) in input_cols.iter().enumerate() {
        if col_idx >= row.cells.len() {
            break;
        }
        let cell = &row.cells[col_idx];
        let param_type = table
            .inputs
            .get(col_idx)
            .map(|p| p.param_type.as_str())
            .unwrap_or("text");
        let accessor = format!("input.{}()", to_camel_case(col_name));

        match cell {
            CellContent::Wildcard => {
                conditions.push("true".to_string());
            }
            CellContent::Condition(cond) => {
                conditions.push(translate_condition(cond, &accessor, param_type));
            }
            CellContent::Action(_) => {
                // Actions in input columns -- treat as wildcard
                conditions.push("true".to_string());
            }
        }
    }

    if conditions.is_empty() || conditions.iter().all(|c| c == "true") {
        writeln!(out, "        return true;").unwrap();
    } else {
        // Filter out trivial "true" conditions
        let non_trivial: Vec<&String> = conditions.iter().filter(|c| *c != "true").collect();
        if non_trivial.is_empty() {
            writeln!(out, "        return true;").unwrap();
        } else {
            let combined = non_trivial
                .iter()
                .map(|c| format!("            {c}"))
                .collect::<Vec<_>>()
                .join(" &&\n");
            writeln!(out, "        return\n{combined};").unwrap();
        }
    }

    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();
}

/// Write a getRowNResult() method.
fn write_result_row(
    out: &mut String,
    idx: usize,
    row: &TableRow,
    table: &DecisionTableDef,
    output_type: &str,
) {
    let input_name = format!("{}TableInput", to_pascal_case(&table.name));

    writeln!(
        out,
        "    private {output_type} getRow{idx}Result({input_name} input) {{"
    )
    .unwrap();

    // Output columns start after input columns
    let output_start = table.inputs.len().min(table.columns.len());
    let output_cells: Vec<(&str, &CellContent)> = table.columns[output_start..]
        .iter()
        .enumerate()
        .filter_map(|(i, col)| {
            row.cells
                .get(output_start + i)
                .map(|cell| (col.as_str(), cell))
        })
        .collect();

    if table.returns.len() <= 1 {
        // Single return -- return the value directly
        if let Some((_, cell)) = output_cells.first() {
            let value = translate_action_value(cell, &table.returns.first().map(|r| r.param_type.as_str()).unwrap_or("text"));
            writeln!(out, "        return {value};").unwrap();
        } else {
            writeln!(out, "        return null;").unwrap();
        }
    } else {
        // Multiple returns -- use builder
        writeln!(
            out,
            "        return {output_type}.builder()"
        )
        .unwrap();
        for (col_name, cell) in &output_cells {
            let ret_type = table
                .returns
                .iter()
                .find(|r| r.name == *col_name)
                .map(|r| r.param_type.as_str())
                .unwrap_or("text");
            let setter = to_camel_case(col_name);
            let value = translate_action_value(cell, ret_type);
            writeln!(out, "            .{setter}({value})").unwrap();
        }
        writeln!(out, "            .build();").unwrap();
    }

    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();
}

/// Translate a cell's action/value to Java.
fn translate_action_value(cell: &CellContent, return_type: &str) -> String {
    match cell {
        CellContent::Wildcard => "null".to_string(),
        CellContent::Condition(cond) => {
            // Conditions in output columns are treated as literal values
            match cond {
                nexflow_parser::ast::rules::ConditionExpr::ExactMatch(v) => {
                    format_literal(v, return_type)
                }
                _ => "null".to_string(),
            }
        }
        CellContent::Action(action) => match action {
            ActionExpr::Assign(v) => format_literal(v, return_type),
            ActionExpr::Calculate(expr) => expr.clone(),
            ActionExpr::Lookup {
                table,
                args,
                default,
            } => {
                let args_str = args.join(", ");
                if let Some(def) = default {
                    format!("lookup(\"{table}\", {args_str}, {def})")
                } else {
                    format!("lookup(\"{table}\", {args_str})")
                }
            }
            ActionExpr::Call { function, args } => {
                let args_str = args.join(", ");
                format!("{function}({args_str})")
            }
            ActionExpr::Emit { target } => {
                format!("emit(\"{target}\")")
            }
        },
    }
}

/// Format a literal value for the given return type.
fn format_literal(value: &str, return_type: &str) -> String {
    match return_type {
        "text" | "string" | "String" => format!("\"{value}\""),
        "money" | "decimal" | "percentage" => {
            format!("new java.math.BigDecimal(\"{value}\")")
        }
        "number" | "integer" => {
            if value.contains('.') {
                format!("new java.math.BigDecimal(\"{value}\")")
            } else {
                format!("{value}L")
            }
        }
        "boolean" => value.to_string(),
        _ => format!("\"{value}\""),
    }
}

/// Write the execute() adapter method for Map-based pipeline integration.
fn write_execute_method(out: &mut String, table: &DecisionTableDef, output_type: &str) {
    let input_name = format!("{}TableInput", to_pascal_case(&table.name));

    writeln!(out, "    /**").unwrap();
    writeln!(
        out,
        "     * Execute adapter for Map-based pipeline integration."
    )
    .unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public Map<String, Object> execute(Map<String, Object> inputMap) {{"
    )
    .unwrap();

    // Extract typed input from map
    writeln!(
        out,
        "        {input_name} input = new {input_name}("
    )
    .unwrap();
    for (i, p) in table.inputs.iter().enumerate() {
        let key = &p.name;
        let java_type = rules_type_to_java(&p.param_type);
        let cast = map_extract_cast(&java_type);
        let comma = if i + 1 < table.inputs.len() { "," } else { "" };
        writeln!(
            out,
            "            {cast}inputMap.get(\"{key}\"){comma}"
        )
        .unwrap();
    }
    writeln!(out, "        );").unwrap();
    writeln!(out).unwrap();

    // Call evaluate and convert result to map
    let hit_policy = table.hit_policy.as_ref().unwrap_or(&HitPolicy::FirstMatch);
    match hit_policy {
        HitPolicy::FirstMatch => {
            writeln!(
                out,
                "        Optional<{output_type}> result = evaluate(input);"
            )
            .unwrap();
            writeln!(out, "        Map<String, Object> output = new HashMap<>();").unwrap();
            writeln!(out, "        result.ifPresent(r -> {{").unwrap();
            for r in &table.returns {
                let key = &r.name;
                let accessor = format!("r.{}()", to_camel_case(key));
                writeln!(
                    out,
                    "            output.put(\"{key}\", {accessor});"
                )
                .unwrap();
            }
            writeln!(out, "        }});").unwrap();
            writeln!(out, "        return output;").unwrap();
        }
        _ => {
            writeln!(out, "        var results = evaluate(input);").unwrap();
            writeln!(out, "        Map<String, Object> output = new HashMap<>();").unwrap();
            writeln!(
                out,
                "        output.put(\"results\", results);"
            )
            .unwrap();
            writeln!(out, "        return output;").unwrap();
        }
    }
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();
}

/// Generate an output record class for multi-return decision tables.
fn generate_output_record(table: &DecisionTableDef) -> (String, String) {
    let output_name = format!("{}Output", to_pascal_case(&table.name));
    let filename = format!("{output_name}.java");

    let mut out = String::with_capacity(2048);

    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "import java.io.Serializable;").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/**").unwrap();
    writeln!(
        out,
        " * Output record for decision table: {}.",
        table.name
    )
    .unwrap();
    writeln!(out, " * Generated -- DO NOT EDIT.").unwrap();
    writeln!(out, " */").unwrap();

    // Record declaration
    write!(out, "public record {output_name}(").unwrap();
    for (i, r) in table.returns.iter().enumerate() {
        let java_type = rules_type_to_java(&r.param_type);
        let java_name = to_camel_case(&r.name);
        if i > 0 {
            write!(out, ", ").unwrap();
        }
        write!(out, "{java_type} {java_name}").unwrap();
    }
    writeln!(out, ") implements Serializable {{").unwrap();
    writeln!(out).unwrap();

    // Builder
    writeln!(out, "    public static Builder builder() {{").unwrap();
    writeln!(out, "        return new Builder();").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "    public static class Builder {{").unwrap();
    for r in &table.returns {
        let java_type = rules_type_to_java(&r.param_type);
        let java_name = to_camel_case(&r.name);
        writeln!(out, "        private {java_type} {java_name};").unwrap();
    }
    writeln!(out).unwrap();
    for r in &table.returns {
        let java_type = rules_type_to_java(&r.param_type);
        let java_name = to_camel_case(&r.name);
        writeln!(
            out,
            "        public Builder {java_name}({java_type} {java_name}) {{"
        )
        .unwrap();
        writeln!(out, "            this.{java_name} = {java_name};").unwrap();
        writeln!(out, "            return this;").unwrap();
        writeln!(out, "        }}").unwrap();
        writeln!(out).unwrap();
    }
    writeln!(out, "        public {output_name} build() {{").unwrap();
    let args: Vec<String> = table
        .returns
        .iter()
        .map(|r| to_camel_case(&r.name))
        .collect();
    writeln!(
        out,
        "            return new {output_name}({});",
        args.join(", ")
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(out, "}}").unwrap();

    (filename, out)
}

fn write_imports(out: &mut String, table: &DecisionTableDef) {
    let hit_policy = table.hit_policy.as_ref().unwrap_or(&HitPolicy::FirstMatch);

    writeln!(out, "import org.slf4j.Logger;").unwrap();
    writeln!(out, "import org.slf4j.LoggerFactory;").unwrap();
    writeln!(out, "import java.util.*;").unwrap();

    match hit_policy {
        HitPolicy::FirstMatch => {
            writeln!(out, "import java.util.Optional;").unwrap();
        }
        _ => {}
    }
}

fn map_extract_cast(java_type: &str) -> String {
    match java_type {
        "String" => "(String) ".to_string(),
        "Long" => "((Number) inputMap.get(\"_\") != null ? ((Number) ".to_string()
            .replace("inputMap.get(\"_\")", ""),
        "java.math.BigDecimal" => {
            "new java.math.BigDecimal(String.valueOf(".to_string()
        }
        "Boolean" => "(Boolean) ".to_string(),
        _ => format!("({java_type}) "),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::rules::*;

    fn make_credit_table() -> DecisionTableDef {
        DecisionTableDef {
            name: "credit_scoring".to_string(),
            hit_policy: Some(HitPolicy::FirstMatch),
            description: Some("Determine credit tier".to_string()),
            version: Some("1.0.0".to_string()),
            inputs: vec![
                InputParam {
                    name: "income".to_string(),
                    param_type: "number".to_string(),
                },
                InputParam {
                    name: "credit_history".to_string(),
                    param_type: "text".to_string(),
                },
            ],
            columns: vec![
                "income".to_string(),
                "credit_history".to_string(),
                "tier".to_string(),
                "limit".to_string(),
            ],
            rows: vec![
                TableRow {
                    priority: None,
                    cells: vec![
                        CellContent::Condition(ConditionExpr::Comparison {
                            operator: ">=".to_string(),
                            value: "100000".to_string(),
                        }),
                        CellContent::Condition(ConditionExpr::ExactMatch(
                            "excellent".to_string(),
                        )),
                        CellContent::Action(ActionExpr::Assign("gold".to_string())),
                        CellContent::Action(ActionExpr::Assign("50000".to_string())),
                    ],
                },
                TableRow {
                    priority: None,
                    cells: vec![
                        CellContent::Wildcard,
                        CellContent::Condition(ConditionExpr::ExactMatch("poor".to_string())),
                        CellContent::Action(ActionExpr::Assign("denied".to_string())),
                        CellContent::Action(ActionExpr::Assign("0".to_string())),
                    ],
                },
                TableRow {
                    priority: None,
                    cells: vec![
                        CellContent::Wildcard,
                        CellContent::Wildcard,
                        CellContent::Action(ActionExpr::Assign("review".to_string())),
                        CellContent::Action(ActionExpr::Assign("5000".to_string())),
                    ],
                },
            ],
            returns: vec![
                ReturnParam {
                    name: "tier".to_string(),
                    param_type: "text".to_string(),
                },
                ReturnParam {
                    name: "limit".to_string(),
                    param_type: "number".to_string(),
                },
            ],
            execute: None,
            post_calculate: Vec::new(),
            aggregate: Vec::new(),
        }
    }

    #[test]
    fn test_decision_table_produces_two_files() {
        let files = generate_decision_table(&make_credit_table());
        // Should have: Output record + Table class
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|(f, _)| f == "CreditScoringOutput.java"));
        assert!(files.iter().any(|(f, _)| f == "CreditScoringTable.java"));
    }

    #[test]
    fn test_table_class_structure() {
        let files = generate_decision_table(&make_credit_table());
        let (_, content) = files.iter().find(|(f, _)| f.contains("Table.java")).unwrap();

        assert!(content.contains("public class CreditScoringTable {"));
        assert!(content.contains("public record CreditScoringTableInput("));
        assert!(content.contains("Long income, String creditHistory"));
    }

    #[test]
    fn test_first_match_evaluate() {
        let files = generate_decision_table(&make_credit_table());
        let (_, content) = files.iter().find(|(f, _)| f.contains("Table.java")).unwrap();

        assert!(content.contains("public Optional<CreditScoringOutput> evaluate("));
        assert!(content.contains("if (matchRow0(input))"));
        assert!(content.contains("return Optional.of(getRow0Result(input))"));
        assert!(content.contains("return Optional.empty()"));
    }

    #[test]
    fn test_row_matching() {
        let files = generate_decision_table(&make_credit_table());
        let (_, content) = files.iter().find(|(f, _)| f.contains("Table.java")).unwrap();

        // Row 0: income >= 100000 AND credit_history = "excellent"
        assert!(content.contains("matchRow0(CreditScoringTableInput input)"));
        assert!(content.contains(">= 100000L"));
        assert!(content.contains("\"excellent\".equals(input.creditHistory())"));

        // Row 1: wildcard AND credit_history = "poor"
        assert!(content.contains("matchRow1("));
        assert!(content.contains("\"poor\".equals(input.creditHistory())"));

        // Row 2: all wildcards
        assert!(content.contains("matchRow2("));
    }

    #[test]
    fn test_row_results_multi_return() {
        let files = generate_decision_table(&make_credit_table());
        let (_, content) = files.iter().find(|(f, _)| f.contains("Table.java")).unwrap();

        assert!(content.contains("getRow0Result("));
        assert!(content.contains("CreditScoringOutput.builder()"));
        assert!(content.contains(".tier(\"gold\")"));
        assert!(content.contains(".limit(50000L)"));
    }

    #[test]
    fn test_output_record() {
        let files = generate_decision_table(&make_credit_table());
        let (_, content) = files
            .iter()
            .find(|(f, _)| f.contains("Output.java"))
            .unwrap();

        assert!(content.contains("public record CreditScoringOutput(String tier, Long limit)"));
        assert!(content.contains("implements Serializable"));
        assert!(content.contains("public static Builder builder()"));
        assert!(content.contains("public Builder tier(String tier)"));
        assert!(content.contains("public Builder limit(Long limit)"));
    }

    #[test]
    fn test_execute_adapter() {
        let files = generate_decision_table(&make_credit_table());
        let (_, content) = files.iter().find(|(f, _)| f.contains("Table.java")).unwrap();

        assert!(content.contains("public Map<String, Object> execute(Map<String, Object> inputMap)"));
        assert!(content.contains("Optional<CreditScoringOutput> result = evaluate(input)"));
    }

    #[test]
    fn test_multi_hit_policy() {
        let mut table = make_credit_table();
        table.hit_policy = Some(HitPolicy::MultiHit);

        let files = generate_decision_table(&table);
        let (_, content) = files.iter().find(|(f, _)| f.contains("Table.java")).unwrap();

        assert!(content.contains("public List<CreditScoringOutput> evaluate("));
        assert!(content.contains("List<CreditScoringOutput> results = new ArrayList<>()"));
    }

    #[test]
    fn test_single_return() {
        let mut table = make_credit_table();
        table.returns = vec![ReturnParam {
            name: "tier".to_string(),
            param_type: "text".to_string(),
        }];
        // Adjust columns
        table.columns = vec![
            "income".to_string(),
            "credit_history".to_string(),
            "tier".to_string(),
        ];
        // Adjust rows to only have 3 cells
        for row in &mut table.rows {
            row.cells.truncate(3);
        }

        let files = generate_decision_table(&table);
        // Single return -- no output record file
        assert_eq!(files.len(), 1);

        let (_, content) = &files[0];
        assert!(content.contains("Optional<String> evaluate("));
        assert!(content.contains("return \"gold\""));
    }
}
