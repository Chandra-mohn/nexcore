// NexCore -- Nexflow Codegen: Procedural Rule Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates `*Rule.java` classes from procedural rule definitions.
//!
//! Procedural rules contain imperative logic: if-then-else chains,
//! variable assignments, action calls, and early returns.

use std::fmt::Write;

use nexflow_parser::ast::rules::{BlockItem, ProceduralRuleDef};

use crate::java::naming::to_pascal_case;

const DEFAULT_PACKAGE: &str = "com.nexflow.rules";

/// Generate a procedural rule Java class.
///
/// Returns `(filename, content)`.
pub fn generate_procedural_rule(rule: &ProceduralRuleDef) -> (String, String) {
    let class_name = format!("{}Rule", to_pascal_case(&rule.name));
    let filename = format!("{class_name}.java");

    let mut out = String::with_capacity(4096);

    // Package + imports
    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "import org.slf4j.Logger;").unwrap();
    writeln!(out, "import org.slf4j.LoggerFactory;").unwrap();
    writeln!(out, "import java.util.*;").unwrap();
    writeln!(out).unwrap();

    // Class
    writeln!(out, "/**").unwrap();
    if let Some(desc) = &rule.description {
        writeln!(out, " * {desc}").unwrap();
    } else {
        writeln!(out, " * Procedural rule: {}", rule.name).unwrap();
    }
    writeln!(
        out,
        " * Generated from {}.rules -- DO NOT EDIT.",
        rule.name
    )
    .unwrap();
    writeln!(out, " */").unwrap();
    writeln!(out, "public class {class_name} {{").unwrap();
    writeln!(
        out,
        "    private static final Logger LOG = LoggerFactory.getLogger({class_name}.class);"
    )
    .unwrap();
    writeln!(out).unwrap();

    // apply() method
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Apply the rule to the given input.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public Map<String, Object> apply(Map<String, Object> input) {{"
    )
    .unwrap();
    writeln!(
        out,
        "        Map<String, Object> result = new HashMap<>(input);"
    )
    .unwrap();
    writeln!(out).unwrap();

    // Generate body
    for item in &rule.body {
        write_block_item(&mut out, item, 2);
    }

    writeln!(out).unwrap();
    writeln!(out, "        return result;").unwrap();
    writeln!(out, "    }}").unwrap();

    // Close class
    writeln!(out, "}}").unwrap();

    (filename, out)
}

/// Write a single block item at the given indentation level.
fn write_block_item(out: &mut String, item: &BlockItem, indent: usize) {
    let pad = "    ".repeat(indent);

    match item {
        BlockItem::IfThenElse {
            condition,
            then_block,
            elseif_blocks,
            else_block,
        } => {
            let java_cond = translate_rule_expr(condition);
            writeln!(out, "{pad}if ({java_cond}) {{").unwrap();
            for sub in then_block {
                write_block_item(out, sub, indent + 1);
            }

            for (elseif_cond, elseif_body) in elseif_blocks {
                let java_cond = translate_rule_expr(elseif_cond);
                writeln!(out, "{pad}}} else if ({java_cond}) {{").unwrap();
                for sub in elseif_body {
                    write_block_item(out, sub, indent + 1);
                }
            }

            if !else_block.is_empty() {
                writeln!(out, "{pad}}} else {{").unwrap();
                for sub in else_block {
                    write_block_item(out, sub, indent + 1);
                }
            }

            writeln!(out, "{pad}}}").unwrap();
        }
        BlockItem::Set { name, expression } => {
            let java_expr = translate_rule_expr(expression);
            writeln!(
                out,
                "{pad}result.put(\"{name}\", {java_expr});"
            )
            .unwrap();
        }
        BlockItem::Let { name, expression } => {
            let java_expr = translate_rule_expr(expression);
            writeln!(out, "{pad}var {name} = {java_expr};").unwrap();
        }
        BlockItem::ActionCall { name, args } => {
            let args_str = args
                .iter()
                .map(|a| translate_rule_expr(a))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(out, "{pad}{name}({args_str});").unwrap();
        }
        BlockItem::Return => {
            writeln!(out, "{pad}return result;").unwrap();
        }
    }
}

/// Basic translation of rule expressions to Java.
fn translate_rule_expr(expr: &str) -> String {
    let result = expr.trim().to_string();

    // Boolean operators
    let result = result
        .replace(" and ", " && ")
        .replace(" or ", " || ")
        .replace(" not ", " !");

    // Field access: bare identifiers become map lookups in procedural context
    // But we keep it simple for now -- the expression is passed through
    // with boolean operators translated.

    // String literals -- check if it looks like a quoted string
    if result.starts_with('"') && result.ends_with('"') {
        return result;
    }

    // Numeric
    if result.parse::<f64>().is_ok() {
        return result;
    }

    // Boolean
    if result == "true" || result == "false" {
        return result;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_transfer_rule() -> ProceduralRuleDef {
        ProceduralRuleDef {
            name: "transfer_validation".to_string(),
            description: Some("Validate fund transfer request".to_string()),
            body: vec![BlockItem::IfThenElse {
                condition: "amount > 0 and amount <= account_balance".to_string(),
                then_block: vec![
                    BlockItem::IfThenElse {
                        condition: "amount > 10000".to_string(),
                        then_block: vec![BlockItem::Set {
                            name: "requires_approval".to_string(),
                            expression: "true".to_string(),
                        }],
                        elseif_blocks: Vec::new(),
                        else_block: vec![BlockItem::Set {
                            name: "requires_approval".to_string(),
                            expression: "false".to_string(),
                        }],
                    },
                    BlockItem::Set {
                        name: "valid".to_string(),
                        expression: "true".to_string(),
                    },
                ],
                elseif_blocks: vec![(
                    "amount <= 0".to_string(),
                    vec![
                        BlockItem::Set {
                            name: "valid".to_string(),
                            expression: "false".to_string(),
                        },
                        BlockItem::Set {
                            name: "error_message".to_string(),
                            expression: "\"Amount must be positive\"".to_string(),
                        },
                    ],
                )],
                else_block: vec![
                    BlockItem::Set {
                        name: "valid".to_string(),
                        expression: "false".to_string(),
                    },
                    BlockItem::Set {
                        name: "error_message".to_string(),
                        expression: "\"Insufficient funds\"".to_string(),
                    },
                ],
            }],
        }
    }

    #[test]
    fn test_procedural_rule_filename() {
        let (filename, _) = generate_procedural_rule(&make_transfer_rule());
        assert_eq!(filename, "TransferValidationRule.java");
    }

    #[test]
    fn test_procedural_rule_structure() {
        let (_, content) = generate_procedural_rule(&make_transfer_rule());

        assert!(content.contains("public class TransferValidationRule {"));
        assert!(content.contains("public Map<String, Object> apply(Map<String, Object> input)"));
        assert!(content.contains("Map<String, Object> result = new HashMap<>(input)"));
        assert!(content.contains("return result;"));
    }

    #[test]
    fn test_if_then_else() {
        let (_, content) = generate_procedural_rule(&make_transfer_rule());

        // Main if
        assert!(content.contains("if (amount > 0 && amount <= account_balance)"));
        // Nested if
        assert!(content.contains("if (amount > 10000)"));
        // Set
        assert!(content.contains("result.put(\"requires_approval\", true)"));
        assert!(content.contains("result.put(\"valid\", true)"));
        // Else if
        assert!(content.contains("} else if (amount <= 0)"));
        // Else
        assert!(content.contains("} else {"));
        assert!(content.contains("result.put(\"error_message\", \"Insufficient funds\")"));
    }

    #[test]
    fn test_let_binding() {
        let rule = ProceduralRuleDef {
            name: "calc_rule".to_string(),
            description: None,
            body: vec![
                BlockItem::Let {
                    name: "total".to_string(),
                    expression: "amount * 1.1".to_string(),
                },
                BlockItem::Set {
                    name: "calculated_total".to_string(),
                    expression: "total".to_string(),
                },
            ],
        };

        let (_, content) = generate_procedural_rule(&rule);
        assert!(content.contains("var total = amount * 1.1;"));
        assert!(content.contains("result.put(\"calculated_total\", total)"));
    }

    #[test]
    fn test_action_call() {
        let rule = ProceduralRuleDef {
            name: "audit_rule".to_string(),
            description: None,
            body: vec![BlockItem::ActionCall {
                name: "log_decision".to_string(),
                args: vec!["\"approved\"".to_string(), "amount".to_string()],
            }],
        };

        let (_, content) = generate_procedural_rule(&rule);
        assert!(content.contains("log_decision(\"approved\", amount)"));
    }

    #[test]
    fn test_early_return() {
        let rule = ProceduralRuleDef {
            name: "guard_rule".to_string(),
            description: None,
            body: vec![BlockItem::IfThenElse {
                condition: "input == null".to_string(),
                then_block: vec![BlockItem::Return],
                elseif_blocks: Vec::new(),
                else_block: Vec::new(),
            }],
        };

        let (_, content) = generate_procedural_rule(&rule);
        assert!(content.contains("return result;"));
    }
}
