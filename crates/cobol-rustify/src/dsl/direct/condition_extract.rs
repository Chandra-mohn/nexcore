//! Condition extraction: converts COBOL AST conditions to DSL strings.
//!
//! Converts `Condition`, `Operand`, and `EvaluateStatement` from the COBOL AST
//! into the intermediate types used by the legacy rules generation functions.

use cobol_transpiler::ast::{
    ArithExpr, ArithOp, ComparisonOp, Condition, EvaluateStatement, EvaluateSubject, Literal,
    Operand, Statement,
};

use crate::dsl::rules_emitter::RuleShape;

/// Convert a COBOL `Condition` to a DSL condition string.
pub fn condition_to_string(cond: &Condition) -> String {
    match cond {
        Condition::Comparison { left, op, right } => {
            let l = operand_to_string(left);
            let r = operand_to_string(right);
            let op_str = comparison_op_to_string(op);
            format!("{l} {op_str} {r}")
        }
        Condition::And(a, b) => {
            format!("{} and {}", condition_to_string(a), condition_to_string(b))
        }
        Condition::Or(a, b) => {
            format!("{} or {}", condition_to_string(a), condition_to_string(b))
        }
        Condition::Not(inner) => {
            format!("not ({})", condition_to_string(inner))
        }
        Condition::ConditionName(dr) => {
            dr.name.to_lowercase().replace('-', "_")
        }
        Condition::ClassTest { field, class } => {
            let f = field.name.to_lowercase().replace('-', "_");
            let cls = format!("{class:?}").to_lowercase();
            format!("{f} is {cls}")
        }
        Condition::SignTest { field, sign } => {
            let f = field.name.to_lowercase().replace('-', "_");
            let s = format!("{sign:?}").to_lowercase();
            format!("{f} is {s}")
        }
    }
}

/// Convert an `Operand` to a DSL expression string.
pub fn operand_to_string(op: &Operand) -> String {
    match op {
        Operand::DataRef(dr) => dr.name.to_lowercase().replace('-', "_"),
        Operand::Literal(lit) => literal_to_string(lit),
        Operand::Function(fc) => format!("{}()", fc.name.to_lowercase()),
    }
}

fn literal_to_string(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(s) => s.clone(),
        Literal::Alphanumeric(s) => format!("\"{s}\""),
        Literal::Figurative(fc) => format!("{fc:?}").to_lowercase(),
    }
}

fn comparison_op_to_string(op: &ComparisonOp) -> &'static str {
    match op {
        ComparisonOp::Equal => "==",
        ComparisonOp::NotEqual => "!=",
        ComparisonOp::LessThan => "<",
        ComparisonOp::GreaterThan => ">",
        ComparisonOp::LessOrEqual => "<=",
        ComparisonOp::GreaterOrEqual => ">=",
    }
}

/// Convert an `ArithExpr` to a DSL expression string.
pub fn arith_expr_to_string(expr: &ArithExpr) -> String {
    match expr {
        ArithExpr::Operand(op) => operand_to_string(op),
        ArithExpr::Negate(inner) => format!("-({})", arith_expr_to_string(inner)),
        ArithExpr::BinaryOp { left, op, right } => {
            let l = arith_expr_to_string(left);
            let r = arith_expr_to_string(right);
            let op_str = arith_op_to_string(op);
            format!("{l} {op_str} {r}")
        }
        ArithExpr::Paren(inner) => format!("({})", arith_expr_to_string(inner)),
    }
}

fn arith_op_to_string(op: &ArithOp) -> &'static str {
    match op {
        ArithOp::Add => "+",
        ArithOp::Subtract => "-",
        ArithOp::Multiply => "*",
        ArithOp::Divide => "/",
        ArithOp::Power => "**",
    }
}

/// A mapping extracted from a COBOL statement: target field = expression.
#[derive(Debug, Clone)]
pub struct ExtractedMapping {
    /// Target field name (snake_case)
    pub target: String,
    /// Source expression as DSL string
    pub expr: String,
}

/// Extract all field mappings from a list of COBOL statements.
///
/// Walks COMPUTE, MOVE, ADD, SUBTRACT, MULTIPLY, DIVIDE and converts
/// each to (target, expression) pairs suitable for transform mappings.
pub fn extract_mappings(stmts: &[Statement]) -> Vec<ExtractedMapping> {
    let mut mappings = Vec::new();
    for stmt in stmts {
        extract_mappings_from_stmt(stmt, &mut mappings);
    }
    mappings
}

fn extract_mappings_from_stmt(stmt: &Statement, out: &mut Vec<ExtractedMapping>) {
    match stmt {
        Statement::Compute(comp) => {
            let expr = arith_expr_to_string(&comp.expression);
            for target in &comp.targets {
                out.push(ExtractedMapping {
                    target: target.field.name.to_lowercase().replace('-', "_"),
                    expr: expr.clone(),
                });
            }
        }
        Statement::Move(mv) => {
            let expr = operand_to_string(&mv.source);
            for dest in &mv.destinations {
                out.push(ExtractedMapping {
                    target: dest.name.to_lowercase().replace('-', "_"),
                    expr: expr.clone(),
                });
            }
        }
        Statement::Add(add) => {
            // ADD a b TO c -> c = c + a + b
            let addends: Vec<String> = add.operands.iter().map(operand_to_string).collect();
            let addend_sum = addends.join(" + ");
            for target in &add.to {
                let tgt = target.field.name.to_lowercase().replace('-', "_");
                out.push(ExtractedMapping {
                    target: tgt.clone(),
                    expr: format!("{tgt} + {addend_sum}"),
                });
            }
            // ADD a b GIVING c -> c = a + b
            if !add.giving.is_empty() {
                for target in &add.giving {
                    out.push(ExtractedMapping {
                        target: target.field.name.to_lowercase().replace('-', "_"),
                        expr: addend_sum.clone(),
                    });
                }
            }
        }
        Statement::Subtract(sub) => {
            // SUBTRACT a b FROM c -> c = c - a - b
            let subtrahends: Vec<String> = sub.operands.iter().map(operand_to_string).collect();
            let sub_chain = subtrahends.join(" - ");
            for target in &sub.from {
                let tgt = target.field.name.to_lowercase().replace('-', "_");
                out.push(ExtractedMapping {
                    target: tgt.clone(),
                    expr: format!("{tgt} - {sub_chain}"),
                });
            }
            // SUBTRACT a FROM b GIVING c -> c = b - a
            if !sub.giving.is_empty() && !sub.from.is_empty() {
                let from_name = sub.from[0].field.name.to_lowercase().replace('-', "_");
                for target in &sub.giving {
                    out.push(ExtractedMapping {
                        target: target.field.name.to_lowercase().replace('-', "_"),
                        expr: format!("{from_name} - {sub_chain}"),
                    });
                }
            }
        }
        Statement::Multiply(mul) => {
            let operand = operand_to_string(&mul.operand);
            // MULTIPLY a BY b -> b = b * a
            for target in &mul.by {
                let tgt = target.field.name.to_lowercase().replace('-', "_");
                out.push(ExtractedMapping {
                    target: tgt.clone(),
                    expr: format!("{tgt} * {operand}"),
                });
            }
            // MULTIPLY a BY b GIVING c -> c = a * b
            if !mul.giving.is_empty() && !mul.by.is_empty() {
                let by_name = mul.by[0].field.name.to_lowercase().replace('-', "_");
                for target in &mul.giving {
                    out.push(ExtractedMapping {
                        target: target.field.name.to_lowercase().replace('-', "_"),
                        expr: format!("{operand} * {by_name}"),
                    });
                }
            }
        }
        Statement::Divide(div) => {
            let operand = operand_to_string(&div.operand);
            match div.direction {
                cobol_transpiler::ast::DivideDirection::Into => {
                    // DIVIDE a INTO b -> b = b / a
                    for target in &div.into {
                        let tgt = target.field.name.to_lowercase().replace('-', "_");
                        out.push(ExtractedMapping {
                            target: tgt.clone(),
                            expr: format!("{tgt} / {operand}"),
                        });
                    }
                }
                cobol_transpiler::ast::DivideDirection::By => {
                    // DIVIDE a BY b GIVING c -> c = a / b
                    if let Some(by_op) = &div.by_operand {
                        let by_str = operand_to_string(by_op);
                        for target in &div.giving {
                            out.push(ExtractedMapping {
                                target: target.field.name.to_lowercase().replace('-', "_"),
                                expr: format!("{operand} / {by_str}"),
                            });
                        }
                    }
                }
            }
        }
        Statement::Initialize(init) => {
            for target in &init.targets {
                out.push(ExtractedMapping {
                    target: target.name.to_lowercase().replace('-', "_"),
                    expr: "0".to_string(),
                });
            }
        }
        // Recurse into control flow
        Statement::If(if_stmt) => {
            extract_mappings_from_stmts(&if_stmt.then_body, out);
            extract_mappings_from_stmts(&if_stmt.else_body, out);
        }
        Statement::Perform(perf) => {
            extract_mappings_from_stmts(&perf.body, out);
        }
        _ => {}
    }
}

fn extract_mappings_from_stmts(stmts: &[Statement], out: &mut Vec<ExtractedMapping>) {
    for stmt in stmts {
        extract_mappings_from_stmt(stmt, out);
    }
}

/// Extract a `RuleShape` from a COBOL EVALUATE statement.
pub fn evaluate_to_rule_shape(eval: &EvaluateStatement) -> RuleShape {
    let scrutinee = eval
        .subjects
        .iter()
        .map(|s| match s {
            EvaluateSubject::Expr(op) => operand_to_string(op),
            EvaluateSubject::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        })
        .collect::<Vec<_>>()
        .join(", ");

    let arm_count = eval.when_branches.len()
        + if eval.when_other.is_empty() { 0 } else { 1 };

    RuleShape::DecisionTable {
        arm_count,
        scrutinee,
    }
}

/// Extract branches from an IF/ELSE IF chain in the COBOL AST.
///
/// Returns `(condition_string, actions)` pairs. Actions are `(field, value)` pairs
/// extracted from MOVE statements in each branch.
pub fn extract_if_branches(
    cond: &Condition,
    then_body: &[cobol_transpiler::ast::Statement],
    else_body: &[cobol_transpiler::ast::Statement],
) -> Vec<(String, Vec<(String, String)>)> {
    let mut branches = Vec::new();

    // First branch: the IF condition
    let cond_str = condition_to_string(cond);
    let actions = extract_move_actions(then_body);
    branches.push((cond_str, actions));

    // Walk else-if chain
    let mut remaining = else_body;
    while !remaining.is_empty() {
        if remaining.len() == 1 {
            if let cobol_transpiler::ast::Statement::If(nested) = &remaining[0] {
                let cond_str = condition_to_string(&nested.condition);
                let actions = extract_move_actions(&nested.then_body);
                branches.push((cond_str, actions));
                remaining = &nested.else_body;
                continue;
            }
        }
        // Final else
        let actions = extract_move_actions(remaining);
        branches.push(("otherwise".to_string(), actions));
        break;
    }

    branches
}

/// Extract MOVE target/value pairs from a list of statements.
fn extract_move_actions(stmts: &[cobol_transpiler::ast::Statement]) -> Vec<(String, String)> {
    let mut actions = Vec::new();
    for stmt in stmts {
        if let cobol_transpiler::ast::Statement::Move(mv) = stmt {
            let value = operand_to_string(&mv.source);
            for dest in &mv.destinations {
                let field = dest.name.to_lowercase().replace('-', "_");
                actions.push((field, value.clone()));
            }
        }
    }
    actions
}

/// Determine if an IF chain is "tabular" -- simple comparisons suitable for decision_table.
pub fn is_tabular_condition(cond: &Condition) -> bool {
    matches!(
        cond,
        Condition::Comparison { .. }
            | Condition::ConditionName(_)
    ) || match cond {
        Condition::And(a, b) | Condition::Or(a, b) => {
            is_tabular_condition(a) && is_tabular_condition(b)
        }
        Condition::Not(inner) => is_tabular_condition(inner),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::DataReference;

    fn make_data_ref(name: &str) -> Operand {
        Operand::DataRef(DataReference {
            name: name.to_string(),
            qualifiers: vec![],
            subscripts: vec![],
            ref_mod: None,
        })
    }

    #[test]
    fn simple_comparison() {
        let cond = Condition::Comparison {
            left: make_data_ref("WS-COUNT"),
            op: ComparisonOp::GreaterThan,
            right: Operand::Literal(Literal::Numeric("100".to_string())),
        };
        assert_eq!(condition_to_string(&cond), "ws_count > 100");
    }

    #[test]
    fn and_condition() {
        let cond = Condition::And(
            Box::new(Condition::Comparison {
                left: make_data_ref("WS-A"),
                op: ComparisonOp::Equal,
                right: Operand::Literal(Literal::Alphanumeric("Y".to_string())),
            }),
            Box::new(Condition::Comparison {
                left: make_data_ref("WS-B"),
                op: ComparisonOp::LessThan,
                right: Operand::Literal(Literal::Numeric("50".to_string())),
            }),
        );
        assert_eq!(
            condition_to_string(&cond),
            "ws_a == \"Y\" and ws_b < 50"
        );
    }

    #[test]
    fn condition_name() {
        let cond = Condition::ConditionName(DataReference {
            name: "WS-ACTIVE".to_string(),
            qualifiers: vec![],
            subscripts: vec![],
            ref_mod: None,
        });
        assert_eq!(condition_to_string(&cond), "ws_active");
    }

    #[test]
    fn operand_formats() {
        assert_eq!(operand_to_string(&make_data_ref("WS-TOTAL")), "ws_total");
        assert_eq!(
            operand_to_string(&Operand::Literal(Literal::Numeric("42".to_string()))),
            "42"
        );
        assert_eq!(
            operand_to_string(&Operand::Literal(Literal::Alphanumeric("YES".to_string()))),
            "\"YES\""
        );
    }

    #[test]
    fn tabular_detection() {
        let simple = Condition::Comparison {
            left: make_data_ref("X"),
            op: ComparisonOp::Equal,
            right: Operand::Literal(Literal::Numeric("1".to_string())),
        };
        assert!(is_tabular_condition(&simple));

        let compound = Condition::And(
            Box::new(simple.clone()),
            Box::new(Condition::Comparison {
                left: make_data_ref("Y"),
                op: ComparisonOp::GreaterThan,
                right: Operand::Literal(Literal::Numeric("0".to_string())),
            }),
        );
        assert!(is_tabular_condition(&compound));
    }
}
