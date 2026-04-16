//! JCL -> .proc DSL generator (Route 2).
//!
//! Converts a `ProcessGraph` into ProcDSL-valid text that follows
//! the grammar defined in `grammar/nexflow/ProcDSL.g4`.
//!
//! Mapping:
//! - JOB -> `process job_name` with `mode batch`
//! - EXEC PGM step -> `transform` step with DD-derived connectors
//! - EXEC PROC step -> `call` statement (sub-process reference)
//! - DD with DSN + DISP=SHR/OLD -> `receive` (input)
//! - DD with DSN + DISP=NEW/MOD -> `emit` (output)
//! - SYSOUT DD -> `emit` to log
//! - IF/ELSE/ENDIF -> `if ... then ... else ... endif`
//! - SORT step -> `order_by` hint in transform comment
//! - IEFBR14 -> comment (allocation-only, no processing)

use std::fmt::Write;

use crate::graph::{
    DdSummary, EdgeKind, NodeKind, ProcessEdge, ProcessGraph, ProcessNode, SubGraph,
};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Generate ProcDSL text from a JCL process graph.
pub fn generate_proc_dsl(graph: &ProcessGraph) -> String {
    let mut out = String::new();
    let proc_name = sanitize_id(&graph.job_name);

    // Header comments
    let _ = writeln!(out, "// Generated from JCL: {}", graph.job_name);
    if let Some(ref class) = graph.class {
        let _ = writeln!(out, "// Job class: {class}");
    }
    let _ = writeln!(out, "// Steps: {}", graph.step_count);
    out.push('\n');

    // Process definition
    let _ = writeln!(out, "process {proc_name}");
    let _ = writeln!(out, "    mode batch");
    out.push('\n');

    // Emit body from graph nodes
    emit_node_sequence(&mut out, &graph.nodes, &graph.edges, 1);

    out.push('\n');
    let _ = writeln!(out, "end");
    out
}

// ---------------------------------------------------------------------------
// Node sequence emission
// ---------------------------------------------------------------------------

fn emit_node_sequence(out: &mut String, nodes: &[ProcessNode], edges: &[ProcessEdge], depth: usize) {
    // Collect IDs of nodes that belong to conditional branches
    // so we don't emit them twice (once in the branch, once at top level)
    let mut branch_node_ids = std::collections::HashSet::new();
    for node in nodes {
        if node.kind == NodeKind::Condition {
            collect_all_branch_ids(&node.id, edges, nodes, &mut branch_node_ids);
        }
    }

    for node in nodes {
        // Skip nodes that are inside a conditional branch (handled by emit_condition)
        if branch_node_ids.contains(node.id.as_str()) {
            continue;
        }

        match node.kind {
            NodeKind::Condition => {
                emit_condition(out, node, nodes, edges, depth);
            }
            NodeKind::Join => {
                // Join nodes are handled by emit_condition (endif)
            }
            NodeKind::AllocOnly => {
                emit_comment(out, depth, &format!(
                    "{} -- allocation only (IEFBR14), no processing",
                    node.id
                ));
            }
            NodeKind::ProcCall => {
                emit_proc_call(out, node, depth);
            }
            NodeKind::Utility => {
                emit_utility_step(out, node, depth);
            }
            NodeKind::Db2 => {
                emit_db2_step(out, node, depth);
            }
            NodeKind::Step => {
                emit_program_step(out, node, depth);
            }
        }
    }
}

/// Collect all node IDs that belong to conditional branches (both THEN and ELSE).
fn collect_all_branch_ids<'a>(
    cond_id: &str,
    edges: &[ProcessEdge],
    all_nodes: &'a [ProcessNode],
    ids: &mut std::collections::HashSet<&'a str>,
) {
    for branch_kind in [EdgeKind::ConditionalTrue, EdgeKind::ConditionalFalse] {
        let branch_nodes = collect_branch_nodes(cond_id, edges, branch_kind, all_nodes);
        for node in branch_nodes {
            ids.insert(&node.id);
        }
    }
    // Also mark the join node
    for edge in edges {
        if edge.from == cond_id {
            // Find the join that both branches connect to
            for node in all_nodes {
                if node.kind == NodeKind::Join {
                    ids.insert(&node.id);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Step emission
// ---------------------------------------------------------------------------

fn emit_program_step(out: &mut String, node: &ProcessNode, depth: usize) {
    let _step_id = sanitize_id(&node.id);
    let program = node.program.as_deref().unwrap_or("unknown");

    emit_comment(out, depth, &format!("Step {} -- PGM={}", node.id, program));

    // Receive blocks for input DDs
    for dd in &node.inputs {
        emit_receive(out, dd, depth);
    }

    // Transform
    let input_name = first_input_name(&node.inputs);
    let output_name = first_output_name(&node.outputs);
    let _ = writeln!(
        out,
        "{}transform {} using {} into {}",
        indent(depth), input_name, sanitize_id(program), output_name
    );

    // PARM as comment
    if let Some(ref parm) = node.parm {
        emit_comment(out, depth + 1, &format!("PARM={parm}"));
    }

    // Emit blocks for output DDs
    for dd in &node.outputs {
        emit_emit(out, dd, depth);
    }

    out.push('\n');
}

fn emit_utility_step(out: &mut String, node: &ProcessNode, depth: usize) {
    let util_type = node.utility_type.as_deref().unwrap_or("utility");
    let program = node.program.as_deref().unwrap_or("unknown");

    emit_comment(out, depth, &format!(
        "Step {} -- PGM={} [{}]", node.id, program, util_type
    ));

    // Receive inputs
    for dd in &node.inputs {
        emit_receive(out, dd, depth);
    }

    // Utility-specific transform
    match util_type {
        "sort" => {
            let input_name = first_input_name(&node.inputs);
            let output_name = first_output_name(&node.outputs);
            let _ = writeln!(
                out,
                "{}// SORT: order records by sort control cards",
                indent(depth)
            );
            let _ = writeln!(
                out,
                "{}transform {} using {} into {}",
                indent(depth), input_name, sanitize_id(program), output_name
            );
        }
        "idcams" => {
            let _ = writeln!(
                out,
                "{}// IDCAMS: VSAM utility operation",
                indent(depth)
            );
        }
        "ieb" => {
            let input_name = first_input_name(&node.inputs);
            let output_name = first_output_name(&node.outputs);
            let _ = writeln!(
                out,
                "{}transform {} using {} into {}",
                indent(depth), input_name, sanitize_id(program), output_name
            );
        }
        _ => {
            let input_name = first_input_name(&node.inputs);
            let output_name = first_output_name(&node.outputs);
            let _ = writeln!(
                out,
                "{}transform {} using {} into {}",
                indent(depth), input_name, sanitize_id(program), output_name
            );
        }
    }

    // Emit outputs
    for dd in &node.outputs {
        emit_emit(out, dd, depth);
    }

    out.push('\n');
}

fn emit_db2_step(out: &mut String, node: &ProcessNode, depth: usize) {
    let program = node.program.as_deref().unwrap_or("unknown");

    emit_comment(out, depth, &format!(
        "Step {} -- PGM={} [DB2]", node.id, program
    ));

    let _ = writeln!(
        out,
        "{}// DB2 SQL execution via {}",
        indent(depth), program
    );

    // SYSTSIN typically contains DSN RUN commands
    for dd in &node.inputs {
        if dd.dd_name == "SYSTSIN" {
            let _ = writeln!(
                out,
                "{}// SQL control from SYSTSIN (inline or dataset)",
                indent(depth)
            );
        }
    }

    // Receive/emit for data DDs
    for dd in &node.inputs {
        if dd.dd_name != "SYSTSIN" {
            emit_receive(out, dd, depth);
        }
    }
    for dd in &node.outputs {
        emit_emit(out, dd, depth);
    }

    out.push('\n');
}

fn emit_proc_call(out: &mut String, node: &ProcessNode, depth: usize) {
    let proc_name = node.proc_name.as_deref().unwrap_or("unknown");
    let proc_id = sanitize_id(proc_name);

    emit_comment(out, depth, &format!(
        "Step {} -- EXEC PROC {}", node.id, proc_name
    ));

    if let Some(ref sub) = node.children {
        // Resolved proc: emit as inline sub-process
        emit_comment(out, depth, &format!(
            "Procedure {} ({} steps)", proc_name, sub.nodes.len()
        ));
        emit_sub_graph(out, sub, depth);
    } else {
        // Unresolved proc: emit call reference
        let _ = writeln!(
            out,
            "{}call {}",
            indent(depth), proc_id
        );
    }

    // PARM
    if let Some(ref parm) = node.parm {
        emit_comment(out, depth + 1, &format!("PARM={parm}"));
    }

    out.push('\n');
}

fn emit_sub_graph(out: &mut String, sub: &SubGraph, depth: usize) {
    // Symbolic parameters as comments
    if !sub.symbols.is_empty() {
        let params: Vec<String> = sub.symbols.iter().map(|(k, v)| {
            match v {
                Some(default) => format!("{k}={default}"),
                None => k.clone(),
            }
        }).collect();
        emit_comment(out, depth, &format!("Symbols: {}", params.join(", ")));
    }

    emit_node_sequence(out, &sub.nodes, &sub.edges, depth);
}

// ---------------------------------------------------------------------------
// Conditional emission
// ---------------------------------------------------------------------------

fn emit_condition(
    out: &mut String,
    cond_node: &ProcessNode,
    all_nodes: &[ProcessNode],
    edges: &[ProcessEdge],
    depth: usize,
) {
    let expr = cond_node.condition_expr.as_deref().unwrap_or("true");
    let _ = writeln!(out, "{}if {} then", indent(depth), expr);

    // Find nodes in the THEN branch (conditional_true edges from this condition)
    let then_targets = collect_branch_nodes(&cond_node.id, edges, EdgeKind::ConditionalTrue, all_nodes);
    for node in &then_targets {
        emit_single_node(out, node, depth + 1);
    }

    // Find nodes in the ELSE branch
    let else_targets = collect_branch_nodes(&cond_node.id, edges, EdgeKind::ConditionalFalse, all_nodes);
    if !else_targets.is_empty() {
        // Check if else branch goes directly to join (empty else)
        let is_empty_else = else_targets.len() == 1 && else_targets[0].kind == NodeKind::Join;
        if !is_empty_else {
            let _ = writeln!(out, "{}else", indent(depth));
            for node in &else_targets {
                if node.kind != NodeKind::Join {
                    emit_single_node(out, node, depth + 1);
                }
            }
        }
    }

    let _ = writeln!(out, "{}endif", indent(depth));
    out.push('\n');
}

/// Collect the immediate branch nodes following a condition edge.
fn collect_branch_nodes<'a>(
    cond_id: &str,
    edges: &[ProcessEdge],
    branch_kind: EdgeKind,
    all_nodes: &'a [ProcessNode],
) -> Vec<&'a ProcessNode> {
    let mut result = Vec::new();

    // Find the first node in this branch
    let first_edge = edges.iter().find(|e| e.from == cond_id && e.kind == branch_kind);
    let Some(first) = first_edge else { return result };

    // Walk sequential edges from the first node until we hit a join
    let mut current_id = &first.to;
    loop {
        let Some(node) = all_nodes.iter().find(|n| n.id == *current_id) else { break };
        if node.kind == NodeKind::Join {
            break;
        }
        result.push(node);

        // Follow sequential edge
        let next = edges.iter().find(|e| e.from == *current_id && e.kind == EdgeKind::Sequential);
        match next {
            Some(e) => current_id = &e.to,
            None => break,
        }
    }

    result
}

fn emit_single_node(out: &mut String, node: &ProcessNode, depth: usize) {
    match node.kind {
        NodeKind::Step => emit_program_step(out, node, depth),
        NodeKind::Utility => emit_utility_step(out, node, depth),
        NodeKind::Db2 => emit_db2_step(out, node, depth),
        NodeKind::ProcCall => emit_proc_call(out, node, depth),
        NodeKind::AllocOnly => {
            emit_comment(out, depth, &format!("{} -- allocation only", node.id));
        }
        NodeKind::Condition => emit_condition(out, node, &[], &[], depth),
        NodeKind::Join => {}
    }
}

// ---------------------------------------------------------------------------
// Receive / Emit blocks
// ---------------------------------------------------------------------------

fn emit_receive(out: &mut String, dd: &DdSummary, depth: usize) {
    let dd_id = sanitize_id(&dd.dd_name);

    if dd.inline {
        let _ = writeln!(
            out,
            "{}// {} -- inline control cards",
            indent(depth), dd.dd_name
        );
        return;
    }

    let _ = write!(out, "{}receive {} from ", indent(depth), dd_id);

    if let Some(ref dsn) = dd.dsn {
        let connector = infer_connector(dd);
        let _ = writeln!(out, "{connector} \"{dsn}\"");
    } else {
        let _ = writeln!(out, "file \"{}\"", dd.dd_name);
    }

    // DCB info as schema hint
    if dd.recfm.is_some() || dd.lrecl.is_some() {
        let recfm = dd.recfm.as_deref().unwrap_or("?");
        let lrecl = dd.lrecl.map_or("?".to_string(), |l| l.to_string());
        emit_comment(out, depth + 1, &format!("DCB: RECFM={recfm}, LRECL={lrecl}"));
    }
}

fn emit_emit(out: &mut String, dd: &DdSummary, depth: usize) {
    let dd_id = sanitize_id(&dd.dd_name);

    // SYSOUT -> emit to log
    if dd.disp.as_deref().map_or(false, |d| d.starts_with("SYSOUT")) {
        let _ = writeln!(out, "{}emit {} to log", indent(depth), dd_id);
        return;
    }

    let _ = write!(out, "{}emit {} to ", indent(depth), dd_id);

    if let Some(ref dsn) = dd.dsn {
        let connector = infer_connector(dd);
        let _ = writeln!(out, "{connector} \"{dsn}\"");
    } else {
        let _ = writeln!(out, "file \"{}\"", dd.dd_name);
    }
}

/// Infer a ProcDSL connector type from DD characteristics.
fn infer_connector(dd: &DdSummary) -> &'static str {
    // Check if DSN looks like a VSAM dataset
    if let Some(ref dsn) = dd.dsn {
        let upper = dsn.to_uppercase();
        if upper.contains("VSAM") || upper.contains(".KSDS") || upper.contains(".ESDS") {
            return "jdbc"; // VSAM -> database connector
        }
    }

    // Check RECFM for CSV hint
    if let Some(ref recfm) = dd.recfm {
        if recfm.contains('V') {
            return "file"; // Variable-length -> generic file
        }
    }

    "file"
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn indent(level: usize) -> String {
    "    ".repeat(level)
}

fn emit_comment(out: &mut String, depth: usize, text: &str) {
    let _ = writeln!(out, "{}// {text}", indent(depth));
}

fn sanitize_id(name: &str) -> String {
    let lower = name.to_lowercase();
    let sanitized: String = lower.chars().map(|c| {
        if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }
    }).collect();

    if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{sanitized}")
    } else if sanitized.is_empty() {
        "unnamed".to_string()
    } else {
        sanitized
    }
}

fn first_input_name(inputs: &[DdSummary]) -> String {
    inputs.first()
        .map(|d| sanitize_id(&d.dd_name))
        .unwrap_or_else(|| "input_stream".to_string())
}

fn first_output_name(outputs: &[DdSummary]) -> String {
    outputs.first()
        .map(|d| sanitize_id(&d.dd_name))
        .unwrap_or_else(|| "output_stream".to_string())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::JclSource;
    use crate::graph::build_process_graph;
    use crate::parser::parse_jcl;

    #[test]
    fn simple_job_proc_dsl() {
        let jcl = r#"
//MYJOB   JOB ,'TEST',CLASS=A
//STEP1   EXEC PGM=SORT
//SORTIN  DD DSN=IN.FILE,DISP=SHR
//SORTOUT DD DSN=OUT.FILE,DISP=(NEW,CATLG),DCB=(RECFM=FB,LRECL=80)
//SYSIN   DD *
//SYSOUT  DD SYSOUT=*
//STEP2   EXEC PGM=MYPROG
//INPUT   DD DSN=OUT.FILE,DISP=SHR
//OUTPUT  DD DSN=RESULT.FILE,DISP=(NEW,CATLG)
//SYSOUT  DD SYSOUT=*
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source { JclSource::Job(j) => j, _ => panic!("expected Job") };
        let graph = build_process_graph(&job);
        let dsl = generate_proc_dsl(&graph);

        assert!(dsl.contains("process myjob"), "missing process name:\n{dsl}");
        assert!(dsl.contains("mode batch"), "missing mode:\n{dsl}");
        assert!(dsl.contains("receive sortin from file \"IN.FILE\""), "missing receive:\n{dsl}");
        assert!(dsl.contains("emit sortout to file \"OUT.FILE\""), "missing emit:\n{dsl}");
        assert!(dsl.contains("emit sysout to log"), "missing sysout emit:\n{dsl}");
        assert!(dsl.contains("transform sortin using sort into sortout"), "missing sort transform:\n{dsl}");
        assert!(dsl.contains("transform input using myprog into output"), "missing myprog transform:\n{dsl}");
        assert!(dsl.contains("end"), "missing end:\n{dsl}");

        eprintln!("Generated .proc DSL:\n{dsl}");
    }

    #[test]
    fn conditional_job_proc_dsl() {
        let jcl = r#"
//CONDJOB JOB ,'TEST',CLASS=A
//STEP1   EXEC PGM=PROG1
//SYSOUT  DD SYSOUT=*
// IF STEP1.RC LE 4 THEN
//STEP2   EXEC PGM=PROG2
//SYSOUT  DD SYSOUT=*
// ELSE
//STEP3   EXEC PGM=ERRHNDLR
//SYSOUT  DD SYSOUT=A
// ENDIF
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source { JclSource::Job(j) => j, _ => panic!("expected Job") };
        let graph = build_process_graph(&job);
        let dsl = generate_proc_dsl(&graph);

        assert!(dsl.contains("if"), "missing if:\n{dsl}");
        assert!(dsl.contains("STEP1.RC <= 4"), "missing condition:\n{dsl}");
        assert!(dsl.contains("else"), "missing else:\n{dsl}");
        assert!(dsl.contains("endif"), "missing endif:\n{dsl}");
        assert!(dsl.contains("PROG2"), "missing PROG2 in then branch:\n{dsl}");
        assert!(dsl.contains("ERRHNDLR"), "missing ERRHNDLR in else branch:\n{dsl}");

        eprintln!("Generated .proc DSL:\n{dsl}");
    }

    #[test]
    fn alloc_only_step() {
        let jcl = r#"
//ALLOCJOB JOB ,'TEST'
//STEP1    EXEC PGM=IEFBR14
//NEWDS    DD DSN=MY.NEW.FILE,DISP=(NEW,CATLG),SPACE=(CYL,(1,1))
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source { JclSource::Job(j) => j, _ => panic!("expected Job") };
        let graph = build_process_graph(&job);
        let dsl = generate_proc_dsl(&graph);

        assert!(dsl.contains("allocation only"), "missing alloc-only comment:\n{dsl}");
        assert!(!dsl.contains("transform"), "should not have transform for IEFBR14:\n{dsl}");

        eprintln!("Generated .proc DSL:\n{dsl}");
    }

    #[test]
    fn proc_call_step() {
        let jcl = r#"
//PROCJOB JOB ,'TEST'
//STEP1   EXEC MYPROC,PARM='HELLO'
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source { JclSource::Job(j) => j, _ => panic!("expected Job") };
        let graph = build_process_graph(&job);
        let dsl = generate_proc_dsl(&graph);

        assert!(dsl.contains("call myproc"), "missing call:\n{dsl}");
        assert!(dsl.contains("PARM='HELLO'"), "missing PARM:\n{dsl}");

        eprintln!("Generated .proc DSL:\n{dsl}");
    }
}
