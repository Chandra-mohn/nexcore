//! Direct process emitter: generates `.proc` files from COBOL AST.
//!
//! Reads the COBOL AST's `ProcedureDivision` directly to build the call graph
//! instead of re-parsing Phase 1 Rust `#[cobol(...)]` attributes via `syn`.
//! Reuses `generate_process_file()` from the legacy emitter.

use std::collections::{HashMap, HashSet};

use cobol_transpiler::ast::{Paragraph, ProcedureDivision};

use super::cobol_extract::analyze_statement;
use super::{DirectDslEmitter, DirectEmitterContext};
use crate::dsl::process_emitter::{
    generate_process_file, sanitize_identifier, CallGraph, CallNode, NodeRole,
};
use crate::dsl::{DslFile, DslLayer};

/// Direct process emitter: reads COBOL AST ProcedureDivision.
#[derive(Debug)]
pub struct DirectProcessEmitter;

impl DirectDslEmitter for DirectProcessEmitter {
    fn id(&self) -> &'static str {
        "process"
    }

    fn layer(&self) -> DslLayer {
        DslLayer::Process
    }

    fn emit(&self, ctx: &DirectEmitterContext<'_>) -> Vec<DslFile> {
        let proc_div = match &ctx.cobol_program.procedure_division {
            Some(pd) => pd,
            None => return vec![],
        };

        let graph = build_call_graph_from_ast(proc_div);

        if graph.nodes.is_empty() {
            return vec![];
        }

        let has_steps = graph.entry_point.is_some() || !graph.sections.is_empty();
        if !has_steps {
            return vec![];
        }

        let (content, notes, confidence) = generate_process_file(&graph, &ctx.program_name);

        vec![DslFile {
            path: format!(
                "process/{}.proc",
                sanitize_identifier(&ctx.program_name.to_lowercase())
            ),
            content,
            confidence,
            notes,
            source_fields: graph.nodes.iter().map(|n| n.cobol_name.clone()).collect(),
        }]
    }
}

/// Build a `CallGraph` from the COBOL AST's ProcedureDivision.
fn build_call_graph_from_ast(proc_div: &ProcedureDivision) -> CallGraph {
    let mut nodes = Vec::new();
    let mut entry_point = None;
    let mut sections_seen = Vec::new();
    let mut section_members: HashMap<String, Vec<String>> = HashMap::new();

    // Collect all paragraph names that are PERFORM targets
    let all_performed = collect_all_perform_targets(proc_div);

    // Process sections and their paragraphs
    for section in &proc_div.sections {
        let sec_name = sanitize_identifier(&section.name.to_lowercase());
        if !sections_seen.contains(&sec_name) {
            sections_seen.push(sec_name.clone());
        }

        for para in &section.paragraphs {
            if let Some(node) = build_node_from_paragraph(para, Some(&section.name)) {
                section_members
                    .entry(sec_name.clone())
                    .or_default()
                    .push(node.nexflow_name.clone());
                nodes.push(node);
            }
        }
    }

    // Process standalone paragraphs (not in any section)
    for para in &proc_div.paragraphs {
        if let Some(node) = build_node_from_paragraph(para, None) {
            nodes.push(node);
        }
    }

    // Determine entry point: first node not performed by anyone that has performs
    for node in &nodes {
        if !all_performed.contains(&node.cobol_name) && !node.performs.is_empty() {
            entry_point = Some(node.nexflow_name.clone());
            break;
        }
    }

    // Fallback: first node with performs
    if entry_point.is_none() {
        for node in &nodes {
            if !node.performs.is_empty() {
                entry_point = Some(node.nexflow_name.clone());
                break;
            }
        }
    }

    CallGraph {
        nodes,
        entry_point,
        sections: sections_seen,
        section_members,
    }
}

/// Build a CallNode from a COBOL paragraph by analyzing its statements.
fn build_node_from_paragraph(para: &Paragraph, section: Option<&str>) -> Option<CallNode> {
    let cobol_name = para.name.clone();
    let nexflow_name = sanitize_identifier(&para.name.to_lowercase());

    if is_boilerplate(&nexflow_name) {
        return None;
    }

    let mut performs = Vec::new();
    let mut reads = HashSet::new();
    let mut writes = HashSet::new();

    for sentence in &para.sentences {
        for stmt in &sentence.statements {
            analyze_statement(stmt, &mut performs, &mut reads, &mut writes);
        }
    }

    let has_performs = !performs.is_empty();
    let has_reads = !reads.is_empty();
    let has_writes = !writes.is_empty();

    let role = if !has_performs && !has_reads && !has_writes {
        return None;
    } else if has_performs && !has_reads && !has_writes {
        NodeRole::SectionDispatcher
    } else if !has_performs {
        NodeRole::Leaf
    } else {
        NodeRole::ProcessingStep
    };

    Some(CallNode {
        cobol_name,
        nexflow_name,
        section: section.map(String::from),
        performs,
        reads: sorted_vec(reads),
        writes: sorted_vec(writes),
        role,
    })
}

/// Collect all paragraph names that are PERFORM targets.
fn collect_all_perform_targets(proc_div: &ProcedureDivision) -> HashSet<String> {
    let mut targets = HashSet::new();
    let all_paras = proc_div
        .sections
        .iter()
        .flat_map(|s| &s.paragraphs)
        .chain(&proc_div.paragraphs);

    for para in all_paras {
        for sentence in &para.sentences {
            for stmt in &sentence.statements {
                collect_perform_targets(stmt, &mut targets);
            }
        }
    }
    targets
}

fn collect_perform_targets(stmt: &cobol_transpiler::ast::Statement, targets: &mut HashSet<String>) {
    match stmt {
        cobol_transpiler::ast::Statement::Perform(perf) => {
            if let Some(target) = &perf.target {
                targets.insert(target.name.clone());
            }
            for inner in &perf.body {
                collect_perform_targets(inner, targets);
            }
        }
        cobol_transpiler::ast::Statement::If(if_stmt) => {
            for s in &if_stmt.then_body {
                collect_perform_targets(s, targets);
            }
            for s in &if_stmt.else_body {
                collect_perform_targets(s, targets);
            }
        }
        cobol_transpiler::ast::Statement::Evaluate(eval) => {
            for branch in &eval.when_branches {
                for s in &branch.body {
                    collect_perform_targets(s, targets);
                }
            }
        }
        _ => {}
    }
}

fn is_boilerplate(name: &str) -> bool {
    matches!(name, "main" | "new" | "stop_run" | "goback")
}

fn sorted_vec(set: HashSet<String>) -> Vec<String> {
    let mut v: Vec<String> = set.into_iter().collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::*;

    fn make_perform(target: &str) -> Statement {
        Statement::Perform(PerformStatement {
            target: Some(PerformTarget {
                name: target.to_string(),
            }),
            thru: None,
            loop_type: PerformLoopType::Once,
            body: vec![],
        })
    }

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
    fn direct_process_simple_linear() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "MAIN-SECTION".to_string(),
                paragraphs: vec![
                    make_paragraph("MAIN-PARA", vec![
                        make_perform("INIT-PARA"),
                        make_perform("CALC-PARA"),
                    ]),
                    make_paragraph("INIT-PARA", vec![
                        make_move("WS-ZERO", "WS-COUNT"),
                    ]),
                    make_paragraph("CALC-PARA", vec![
                        make_move("WS-COUNT", "WS-RESULT"),
                    ]),
                ],
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

        let files = DirectProcessEmitter.emit(&ctx);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "process/testprog.proc");

        let content = &files[0].content;
        assert!(content.contains("process testprog"), "Content: {content}");
        assert!(content.contains("end"));
        assert!(content.contains("init_para"), "Content: {content}");
        assert!(content.contains("calc_para"), "Content: {content}");
    }

    #[test]
    fn direct_process_empty_proc_div() {
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

        let files = DirectProcessEmitter.emit(&ctx);
        assert!(files.is_empty());
    }

    #[test]
    fn direct_process_standalone_paragraphs() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![],
            paragraphs: vec![
                make_paragraph("MAIN-PARA", vec![
                    make_perform("WORKER-PARA"),
                ]),
                make_paragraph("WORKER-PARA", vec![
                    make_move("WS-IN", "WS-OUT"),
                ]),
            ],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "STANDALONE".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectProcessEmitter.emit(&ctx);
        assert_eq!(files.len(), 1);
        let content = &files[0].content;
        assert!(content.contains("worker_para"), "Content: {content}");
    }
}
