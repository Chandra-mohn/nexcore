//! Direct rules emitter: generates `.rules` files from COBOL AST.
//!
//! Reads the COBOL AST's IF/EVALUATE statements directly to detect decision
//! patterns, instead of re-parsing Phase 1 Rust via `syn`.
//! Reuses `generate_rules_file()` and `group_by_section()` from the legacy emitter.

use std::collections::HashSet;

use cobol_transpiler::ast::{Paragraph, ProcedureDivision, Statement};

use super::cobol_extract::analyze_statement;
use super::condition_extract::{
    evaluate_to_rule_shape, extract_if_branches, is_tabular_condition,
};
use super::{DirectDslEmitter, DirectEmitterContext};
use crate::dsl::rules_emitter::{
    generate_rules_file, group_by_section, sanitize_identifier, RuleCandidate, RuleShape,
};
use crate::dsl::{DslFile, DslLayer};

/// Direct rules emitter: reads COBOL AST for IF/EVALUATE decision patterns.
#[derive(Debug)]
pub struct DirectRulesEmitter;

impl DirectDslEmitter for DirectRulesEmitter {
    fn id(&self) -> &'static str {
        "rules"
    }

    fn layer(&self) -> DslLayer {
        DslLayer::Rules
    }

    fn emit(&self, ctx: &DirectEmitterContext<'_>) -> Vec<DslFile> {
        let proc_div = match &ctx.cobol_program.procedure_division {
            Some(pd) => pd,
            None => return vec![],
        };

        let candidates = extract_rules_from_ast(proc_div);
        if candidates.is_empty() {
            return vec![];
        }

        let sections = group_by_section(&candidates);

        let mut dsl_files = Vec::new();
        for (section_name, section_rules) in &sections {
            let (content, notes, confidence) =
                generate_rules_file(section_name, section_rules, &ctx.program_name);
            let source_fns: Vec<String> = section_rules
                .iter()
                .map(|r| r.cobol_name.clone())
                .collect();

            dsl_files.push(DslFile {
                path: format!("rules/{}.rules", sanitize_identifier(section_name)),
                content,
                confidence,
                notes,
                source_fields: source_fns,
            });
        }

        dsl_files
    }
}

/// Extract rule candidates from all paragraphs in the procedure division.
fn extract_rules_from_ast(proc_div: &ProcedureDivision) -> Vec<RuleCandidate> {
    let mut candidates = Vec::new();

    for section in &proc_div.sections {
        for para in &section.paragraphs {
            candidates.extend(analyze_paragraph_for_rules(para, Some(&section.name)));
        }
    }

    for para in &proc_div.paragraphs {
        candidates.extend(analyze_paragraph_for_rules(para, None));
    }

    candidates
}

/// Analyze a paragraph for decision patterns (IF chains, EVALUATE).
fn analyze_paragraph_for_rules(
    para: &Paragraph,
    section: Option<&str>,
) -> Vec<RuleCandidate> {
    let nexflow_name = sanitize_identifier(&para.name.to_lowercase());

    if is_boilerplate(&nexflow_name) {
        return vec![];
    }

    // First get reads/writes/performs for context
    let mut performs = Vec::new();
    let mut reads = HashSet::new();
    let mut writes = HashSet::new();

    for sentence in &para.sentences {
        for stmt in &sentence.statements {
            analyze_statement(stmt, &mut performs, &mut reads, &mut writes);
        }
    }

    // Skip pure orchestrators (only performs)
    if reads.is_empty() && writes.is_empty() && !performs.is_empty() {
        return vec![];
    }

    // Find all decision patterns in the paragraph
    let mut shapes = Vec::new();
    for sentence in &para.sentences {
        for stmt in &sentence.statements {
            collect_decision_shapes(stmt, &mut shapes);
        }
    }

    if shapes.is_empty() {
        return vec![];
    }

    let reads_vec: Vec<String> = sorted_vec(reads);
    let writes_vec: Vec<String> = sorted_vec(writes);

    if shapes.len() <= 1 {
        shapes
            .into_iter()
            .map(|shape| RuleCandidate {
                cobol_name: para.name.clone(),
                nexflow_name: nexflow_name.clone(),
                section: section.map(String::from),
                shape,
                reads: reads_vec.clone(),
                writes: writes_vec.clone(),
                performs: performs.clone(),
            })
            .collect()
    } else {
        shapes
            .into_iter()
            .enumerate()
            .map(|(i, shape)| RuleCandidate {
                cobol_name: para.name.clone(),
                nexflow_name: format!("{}_{}", nexflow_name, i + 1),
                section: section.map(String::from),
                shape,
                reads: reads_vec.clone(),
                writes: writes_vec.clone(),
                performs: performs.clone(),
            })
            .collect()
    }
}

/// Recursively collect decision shapes from statements.
fn collect_decision_shapes(stmt: &Statement, shapes: &mut Vec<RuleShape>) {
    match stmt {
        Statement::Evaluate(eval) => {
            shapes.push(evaluate_to_rule_shape(eval));
        }
        Statement::If(if_stmt) => {
            // Count branches in the if/else-if chain
            let branch_count = count_if_branches(&if_stmt.condition, &if_stmt.else_body);

            if branch_count >= 2 {
                let branches = extract_if_branches(
                    &if_stmt.condition,
                    &if_stmt.then_body,
                    &if_stmt.else_body,
                );
                let has_else = has_final_else(&if_stmt.else_body);

                if is_tabular_condition(&if_stmt.condition) {
                    shapes.push(RuleShape::TabularIfChain {
                        branch_count,
                        has_else,
                        branches,
                    });
                } else {
                    shapes.push(RuleShape::ProceduralRule {
                        branch_count,
                        has_else,
                        branches,
                    });
                }
            } else {
                // Single if -- check body for nested decisions
                for s in &if_stmt.then_body {
                    collect_decision_shapes(s, shapes);
                }
                for s in &if_stmt.else_body {
                    collect_decision_shapes(s, shapes);
                }
            }
        }
        Statement::Perform(perf) => {
            for s in &perf.body {
                collect_decision_shapes(s, shapes);
            }
        }
        _ => {}
    }
}

/// Count branches in an IF/ELSE-IF chain.
fn count_if_branches(
    _cond: &cobol_transpiler::ast::Condition,
    else_body: &[Statement],
) -> usize {
    let mut count = 1; // The initial IF
    let mut remaining = else_body;

    loop {
        if remaining.is_empty() {
            break;
        }
        if remaining.len() == 1 {
            if let Statement::If(nested) = &remaining[0] {
                count += 1;
                remaining = &nested.else_body;
                continue;
            }
        }
        // Final else counts as a branch
        count += 1;
        break;
    }

    count
}

/// Check if an else body is a final else (not another else-if).
fn has_final_else(else_body: &[Statement]) -> bool {
    if else_body.is_empty() {
        return false;
    }
    if else_body.len() == 1 {
        if let Statement::If(_) = &else_body[0] {
            return false;
        }
    }
    true
}

fn is_boilerplate(name: &str) -> bool {
    matches!(name, "main" | "new" | "stop_run" | "goback")
}

fn sorted_vec(set: std::collections::HashSet<String>) -> Vec<String> {
    let mut v: Vec<String> = set.into_iter().collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::*;

    fn make_move(source: &str, dest: &str) -> Statement {
        Statement::Move(MoveStatement {
            corresponding: false,
            source: Operand::DataRef(DataReference {
                name: source.to_string(),
                qualifiers: vec![],
                subscripts: vec![],
                ref_mod: None,
            }),
            destinations: vec![DataReference {
                name: dest.to_string(),
                qualifiers: vec![],
                subscripts: vec![],
                ref_mod: None,
            }],
        })
    }

    fn make_if(left: &str, op: ComparisonOp, right_val: &str, then_stmts: Vec<Statement>, else_stmts: Vec<Statement>) -> Statement {
        Statement::If(IfStatement {
            condition: Condition::Comparison {
                left: Operand::DataRef(DataReference {
                    name: left.to_string(),
                    qualifiers: vec![],
                    subscripts: vec![],
                    ref_mod: None,
                }),
                op,
                right: Operand::Literal(Literal::Alphanumeric(right_val.to_string())),
            },
            then_body: then_stmts,
            else_body: else_stmts,
        })
    }

    fn make_evaluate(subject: &str, branches: Vec<(&str, Vec<Statement>)>) -> Statement {
        let when_branches: Vec<WhenBranch> = branches
            .into_iter()
            .map(|(val, body)| WhenBranch {
                values: vec![WhenValue::Value(Operand::Literal(
                    Literal::Alphanumeric(val.to_string()),
                ))],
                body,
            })
            .collect();

        Statement::Evaluate(EvaluateStatement {
            subjects: vec![EvaluateSubject::Expr(Operand::DataRef(DataReference {
                name: subject.to_string(),
                qualifiers: vec![],
                subscripts: vec![],
                ref_mod: None,
            }))],
            when_branches,
            when_other: vec![],
        })
    }

    fn make_paragraph(name: &str, stmts: Vec<Statement>) -> Paragraph {
        Paragraph {
            name: name.to_string(),
            sentences: vec![Sentence { statements: stmts }],
        }
    }

    fn make_program_with_proc(proc_div: ProcedureDivision) -> CobolProgram {
        CobolProgram {
            program_id: "TESTPROG".to_string(),
            data_division: None,
            procedure_division: Some(proc_div),
            source_path: None,
            exec_sql_statements: vec![],
        }
    }

    #[test]
    fn direct_rules_evaluate_produces_decision_table() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "RATE-SECTION".to_string(),
                paragraphs: vec![make_paragraph(
                    "DETERMINE-RATE",
                    vec![make_evaluate("WS-ACCT-TYPE", vec![
                        ("S", vec![make_move("3", "WS-RATE")]),
                        ("C", vec![make_move("5", "WS-RATE")]),
                        ("P", vec![make_move("2", "WS-RATE")]),
                    ])],
                )],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectRulesEmitter.emit(&ctx);
        assert!(!files.is_empty(), "Should produce rules file");

        let content = &files[0].content;
        assert!(content.contains("decision_table"), "Content: {content}");
        assert!(content.contains("hit_policy first_match"), "Content: {content}");
        assert!(content.contains("end"));
    }

    #[test]
    fn direct_rules_if_chain_produces_rule() {
        let if_stmt = make_if(
            "WS-SCORE",
            ComparisonOp::GreaterThan,
            "90",
            vec![make_move("\"A\"", "WS-GRADE")],
            vec![make_if(
                "WS-SCORE",
                ComparisonOp::GreaterThan,
                "80",
                vec![make_move("\"B\"", "WS-GRADE")],
                vec![make_move("\"C\"", "WS-GRADE")],
            )],
        );

        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "GRADE-SECTION".to_string(),
                paragraphs: vec![make_paragraph("ASSIGN-GRADE", vec![if_stmt])],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectRulesEmitter.emit(&ctx);
        assert!(!files.is_empty(), "Should produce rules file for if chain");

        let content = &files[0].content;
        assert!(content.contains("end"));
    }

    #[test]
    fn direct_rules_empty_proc_div() {
        let program = CobolProgram {
            program_id: "EMPTY".to_string(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: vec![],
        };
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "EMPTY".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectRulesEmitter.emit(&ctx);
        assert!(files.is_empty());
    }

    #[test]
    fn direct_rules_no_decisions_skipped() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "MAIN-SECTION".to_string(),
                paragraphs: vec![make_paragraph(
                    "SIMPLE-MOVE",
                    vec![make_move("WS-A", "WS-B")],
                )],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectRulesEmitter.emit(&ctx);
        assert!(files.is_empty(), "No decision logic -> no rules file");
    }
}
