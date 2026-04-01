// NexCore -- Nexflow Codegen: Java Decision Engine Generator (L4)
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Java decision engine classes from RulesDSL ASTs.
//!
//! **L4 Generator** -- depends on L2 (.schema -> .avsc) for type references.
//!
//! For each `DecisionTableDef`, produces:
//! - `*Table.java` -- evaluator with hit policy, row matching, execute adapter
//! - `*Output.java` (optional) -- output record if multiple return fields
//!
//! For each `ProceduralRuleDef`, produces:
//! - `*Rule.java` -- imperative rule with if-then-else logic

pub mod condition;
pub mod decision_table;
pub mod procedural;

use std::collections::HashMap;

use nexflow_parser::ast::rules::RulesProgram;

use crate::GeneratedProject;

/// Configuration for Java rules generation.
#[derive(Debug, Clone)]
pub struct RulesGenConfig {
    /// Java package name (default: `com.nexflow.rules`).
    pub package: String,
    /// Output directory prefix (default: `src/main/java/com/nexflow/rules`).
    pub java_dir: String,
}

impl Default for RulesGenConfig {
    fn default() -> Self {
        Self {
            package: "com.nexflow.rules".to_string(),
            java_dir: "src/main/java/com/nexflow/rules".to_string(),
        }
    }
}

/// Generate all Java decision engine classes for a rules program.
///
/// Returns a `GeneratedProject` with file paths mapped to content.
pub fn generate_java_rules(
    program: &RulesProgram,
    config: &RulesGenConfig,
) -> Result<GeneratedProject, String> {
    let mut files = HashMap::new();

    // Decision tables
    for table in &program.decision_tables {
        let generated = decision_table::generate_decision_table(table);
        for (filename, content) in generated {
            files.insert(format!("{}/{filename}", config.java_dir), content);
        }
    }

    // Procedural rules
    for rule in &program.procedural_rules {
        let (filename, content) = procedural::generate_procedural_rule(rule);
        files.insert(format!("{}/{filename}", config.java_dir), content);
    }

    Ok(GeneratedProject { files })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::rules::*;

    fn make_program() -> RulesProgram {
        RulesProgram {
            imports: Vec::new(),
            services: Vec::new(),
            actions: Vec::new(),
            decision_tables: vec![DecisionTableDef {
                name: "credit_scoring".to_string(),
                hit_policy: Some(HitPolicy::FirstMatch),
                description: Some("Credit tier".to_string()),
                version: None,
                inputs: vec![
                    InputParam {
                        name: "income".to_string(),
                        param_type: "number".to_string(),
                    },
                    InputParam {
                        name: "history".to_string(),
                        param_type: "text".to_string(),
                    },
                ],
                columns: vec![
                    "income".to_string(),
                    "history".to_string(),
                    "tier".to_string(),
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
                returns: vec![ReturnParam {
                    name: "tier".to_string(),
                    param_type: "text".to_string(),
                }],
                execute: None,
                post_calculate: Vec::new(),
                aggregate: Vec::new(),
            }],
            procedural_rules: vec![ProceduralRuleDef {
                name: "transfer_check".to_string(),
                description: Some("Check transfer".to_string()),
                body: vec![BlockItem::Set {
                    name: "valid".to_string(),
                    expression: "true".to_string(),
                }],
            }],
        }
    }

    #[test]
    fn test_generate_java_rules_produces_all_files() {
        let program = make_program();
        let config = RulesGenConfig::default();
        let project = generate_java_rules(&program, &config).unwrap();

        let dir = &config.java_dir;

        // Decision table (single return -- no output record)
        assert!(
            project
                .files
                .contains_key(&format!("{dir}/CreditScoringTable.java")),
            "Missing CreditScoringTable"
        );

        // Procedural rule
        assert!(
            project
                .files
                .contains_key(&format!("{dir}/TransferCheckRule.java")),
            "Missing TransferCheckRule"
        );
    }

    #[test]
    fn test_decision_table_content() {
        let program = make_program();
        let config = RulesGenConfig::default();
        let project = generate_java_rules(&program, &config).unwrap();

        let content = &project.files[&format!(
            "{}/CreditScoringTable.java",
            config.java_dir
        )];

        assert!(content.contains("public class CreditScoringTable"));
        assert!(content.contains("Optional<String> evaluate("));
        assert!(content.contains("matchRow0("));
        assert!(content.contains("matchRow1("));
    }

    #[test]
    fn test_procedural_rule_content() {
        let program = make_program();
        let config = RulesGenConfig::default();
        let project = generate_java_rules(&program, &config).unwrap();

        let content = &project.files[&format!(
            "{}/TransferCheckRule.java",
            config.java_dir
        )];

        assert!(content.contains("public class TransferCheckRule"));
        assert!(content.contains("result.put(\"valid\", true)"));
    }

    #[test]
    fn test_multi_return_generates_output_record() {
        let mut program = make_program();
        // Add second return param to trigger output record generation
        program.decision_tables[0].returns.push(ReturnParam {
            name: "limit".to_string(),
            param_type: "number".to_string(),
        });
        program.decision_tables[0]
            .columns
            .push("limit".to_string());
        // Add limit cell to each row
        for row in &mut program.decision_tables[0].rows {
            row.cells
                .push(CellContent::Action(ActionExpr::Assign("5000".to_string())));
        }

        let config = RulesGenConfig::default();
        let project = generate_java_rules(&program, &config).unwrap();

        assert!(project
            .files
            .contains_key(&format!(
                "{}/CreditScoringOutput.java",
                config.java_dir
            )));
    }
}
