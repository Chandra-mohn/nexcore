//! Process emitter: generates `.proc` files from program call graph.
//!
//! Builds the orchestration layer (L1) from `#[cobol(...)]` attributes:
//! - Call graph from `performs` attributes -> process step ordering
//! - File I/O from `reads`/`writes` referencing file fields -> receive/emit blocks
//! - Section structure -> logical grouping of steps
//! - References transforms (E2) and rules (E3) as `transform using` / `evaluate using`
//!
//! Produces one `.proc` file per COBOL program.

use std::collections::{HashMap, HashSet};

use super::cobol_attrs::{
    extract_annotated_fns, extract_program_name, AnnotatedFn,
};
use super::dsl_ast::*;
use super::{DslEmitter, DslFile, DslLayer, EmitterContext};

/// Emitter that produces `.proc` files from program call graphs.
#[derive(Debug)]
pub struct ProcessEmitter;

impl DslEmitter for ProcessEmitter {
    fn id(&self) -> &'static str {
        "process"
    }

    fn layer(&self) -> DslLayer {
        DslLayer::Process
    }

    fn emit(&self, ctx: &EmitterContext<'_>) -> Vec<DslFile> {
        let fns = extract_annotated_fns(ctx.syn_file);
        let program = extract_program_name(ctx.syn_file)
            .unwrap_or_else(|| ctx.program_name.clone());

        // Build the call graph
        let graph = build_call_graph(&fns);

        if graph.nodes.is_empty() {
            return vec![];
        }

        // Don't emit empty process files (single-paragraph programs with no call graph)
        let has_steps = graph.entry_point.is_some() || !graph.sections.is_empty();
        if !has_steps {
            return vec![];
        }

        // Generate a single .proc file for the program
        let (content, notes, confidence) = generate_process_file(&graph, &program);

        vec![DslFile {
            path: format!("process/{}.proc", sanitize_identifier(&program.to_lowercase())),
            content,
            confidence,
            notes,
            source_fields: graph.nodes.iter().map(|n| n.cobol_name.clone()).collect(),
        }]
    }
}

// ---------------------------------------------------------------------------
// Call graph types
// ---------------------------------------------------------------------------

/// A node in the call graph (a paragraph or section).
#[derive(Debug, Clone)]
pub struct CallNode {
    /// COBOL paragraph name
    pub cobol_name: String,
    /// Nexflow-valid name
    pub nexflow_name: String,
    /// Section this paragraph belongs to
    #[allow(dead_code)]
    pub section: Option<String>,
    /// Paragraphs this node PERFORMs (edges)
    pub performs: Vec<String>,
    /// Fields this paragraph reads
    pub reads: Vec<String>,
    /// Fields this paragraph writes
    pub writes: Vec<String>,
    /// What role this node plays
    pub role: NodeRole,
}

/// Classification of a node's role in the process.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeRole {
    /// Entry point (run function or main dispatcher)
    EntryPoint,
    /// Section dispatcher (performs sub-paragraphs, no data access)
    SectionDispatcher,
    /// Processing step (has reads and/or writes)
    ProcessingStep,
    /// Leaf paragraph (no performs, does actual work)
    Leaf,
}

/// The complete call graph for a program.
#[derive(Debug)]
pub struct CallGraph {
    /// All nodes
    pub nodes: Vec<CallNode>,
    /// Entry point node name (nexflow name)
    pub entry_point: Option<String>,
    /// Sections in order of first encounter
    pub sections: Vec<String>,
    /// Which sections contain which paragraphs (section nexflow name -> para nexflow names)
    pub section_members: HashMap<String, Vec<String>>,
    /// File I/O info from COBOL FD entries (connector type + config).
    /// Populated by direct emitter when DataDivision is available.
    pub file_io: Vec<FileIoEntry>,
}

/// A file I/O entry derived from COBOL FILE SECTION FD.
#[derive(Debug, Clone)]
pub struct FileIoEntry {
    pub file_name: String,
    pub direction: IoDirection,
    pub connector: crate::dsl::dsl_ast::ConnectorSpec,
    pub record_schema: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoDirection {
    Input,
    Output,
    InputOutput,
}

// ---------------------------------------------------------------------------
// Graph construction
// ---------------------------------------------------------------------------

fn build_call_graph(fns: &[AnnotatedFn]) -> CallGraph {
    let mut nodes = Vec::new();
    let mut entry_point = None;
    let mut sections_seen = Vec::new();
    let mut section_members: HashMap<String, Vec<String>> = HashMap::new();

    // Collect all function names that are performed by others
    let all_performed: HashSet<String> = fns
        .iter()
        .filter_map(|f| f.cobol_attr.as_ref())
        .flat_map(|a| a.performs.iter().cloned())
        .collect();

    for f in fns {
        let attr = match &f.cobol_attr {
            Some(a) => a,
            None => continue,
        };

        // Determine role
        let role = if f.name == "run" {
            NodeRole::EntryPoint
        } else if !attr.performs.is_empty() && attr.reads.is_empty() && attr.writes.is_empty() {
            NodeRole::SectionDispatcher
        } else if attr.performs.is_empty() && (attr.reads.is_empty() && attr.writes.is_empty()) {
            // No data flow and no performs -- likely a no-op or boilerplate
            continue;
        } else if attr.performs.is_empty() {
            NodeRole::Leaf
        } else {
            NodeRole::ProcessingStep
        };

        // Skip boilerplate except run
        if is_boilerplate(&f.name) && f.name != "run" {
            continue;
        }

        let nexflow_name = sanitize_identifier(&f.name);

        if role == NodeRole::EntryPoint {
            entry_point = Some(nexflow_name.clone());
        }

        // Track sections
        if let Some(section) = &attr.section {
            let sec_name = sanitize_identifier(&section.to_lowercase());
            if !sections_seen.contains(&sec_name) {
                sections_seen.push(sec_name.clone());
            }
            section_members
                .entry(sec_name)
                .or_default()
                .push(nexflow_name.clone());
        }

        nodes.push(CallNode {
            cobol_name: rust_name_to_cobol(&f.name),
            nexflow_name,
            section: attr.section.clone(),
            performs: attr.performs.clone(),
            reads: attr.reads.clone(),
            writes: attr.writes.clone(),
            role,
        });
    }

    // If no explicit run, find the node that isn't performed by anyone
    if entry_point.is_none() {
        for node in &nodes {
            let cobol_name = &node.cobol_name;
            if !all_performed.contains(cobol_name) && node.role != NodeRole::Leaf {
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
        file_io: vec![],
    }
}

// ---------------------------------------------------------------------------
// Process generation
// ---------------------------------------------------------------------------

pub fn generate_process_file(
    graph: &CallGraph,
    program: &str,
) -> (String, Vec<String>, f64) {
    let mut notes = Vec::new();

    let proc_name = sanitize_identifier(&program.to_lowercase());

    // Imports -- reference transforms and rules from E2/E3
    let mut imports = vec![ImportPath::schema(program)];
    for section in &graph.sections {
        imports.push(ImportPath::transform(section));
        imports.push(ImportPath::rules(section));
    }

    // Detect I/O fields for receive/emit
    let io_info = detect_io(graph);

    // Build process body statements
    let mut body = Vec::new();

    // Receive blocks from FD entries or heuristic I/O detection
    let input_fds: Vec<&FileIoEntry> = graph.file_io.iter()
        .filter(|f| f.direction == IoDirection::Input || f.direction == IoDirection::InputOutput)
        .collect();

    if !input_fds.is_empty() {
        body.push(ProcessStmt::Comment("Input data sources (from COBOL FILE SECTION)".to_string()));
        for fd in &input_fds {
            let name = sanitize_identifier(&fd.file_name.to_lowercase());
            body.push(ProcessStmt::Receive(ReceiveBlock {
                name: Ident::new(&name),
                schema: fd.record_schema.as_ref().map(|s| Ident::new(s)),
                connector: Some(fd.connector.clone()),
            }));
        }
    } else if !io_info.input_fields.is_empty() {
        body.push(ProcessStmt::Comment("Input data sources (from COBOL field name heuristic)".to_string()));
        body.push(ProcessStmt::Receive(ReceiveBlock {
            name: Ident::new("input_records"),
            schema: Some(Ident::new(&format!("{proc_name}_input"))),
            connector: None,
        }));
        notes.push("Input receive block needs data source configuration".to_string());
    }

    // Process steps -- walk the call graph from entry point
    if let Some(entry) = &graph.entry_point {
        body.push(ProcessStmt::Comment("Processing flow (from COBOL paragraph call graph)".to_string()));
        collect_steps(&mut body, graph, entry, &mut notes, &mut HashSet::new());
    } else {
        // No clear entry point -- list sections in order
        body.push(ProcessStmt::Comment("Processing sections (no clear entry point detected)".to_string()));
        for section in &graph.sections {
            body.push(ProcessStmt::Comment(format!("Section: {section}")));
            if let Some(members) = graph.section_members.get(section) {
                for member in members {
                    body.push(ProcessStmt::TransformUsing {
                        input: Ident::new("input_records"),
                        using: Ident::new(member),
                        output: Ident::new("output_records"),
                    });
                }
            }
        }
        notes.push("No entry point detected -- sections listed in declaration order".to_string());
    }

    // Emit blocks from FD entries or heuristic I/O detection
    let output_fds: Vec<&FileIoEntry> = graph.file_io.iter()
        .filter(|f| f.direction == IoDirection::Output || f.direction == IoDirection::InputOutput)
        .collect();

    if !output_fds.is_empty() {
        body.push(ProcessStmt::Comment("Output data sinks (from COBOL FILE SECTION)".to_string()));
        for fd in &output_fds {
            let name = sanitize_identifier(&fd.file_name.to_lowercase());
            body.push(ProcessStmt::Emit(EmitBlock {
                target: Ident::new(&name),
                schema: fd.record_schema.as_ref().map(|s| Ident::new(s)),
                connector: Some(fd.connector.clone()),
            }));
        }
    } else if !io_info.output_fields.is_empty() {
        body.push(ProcessStmt::Comment("Output data sinks (from COBOL field name heuristic)".to_string()));
        body.push(ProcessStmt::Emit(EmitBlock {
            target: Ident::new("output_records"),
            schema: Some(Ident::new(&format!("{proc_name}_output"))),
            connector: None,
        }));
        notes.push("Output emit block needs data sink configuration".to_string());
    }

    let file = ProcessFile {
        comments: vec![
            Comment(format!("Generated by cobol2rust Nexflow emitter")),
            Comment(format!("Source: {program}")),
        ],
        imports,
        processes: vec![ProcessDef {
            name: Ident::new(&proc_name),
            mode: Some(ProcessMode::Batch),
            execution: None,
            body,
        }],
    };

    // Confidence is 1.0 -- output is grammar-valid by construction
    (file.to_text(), notes, 1.0)
}

/// Check if a set of performed sections are independent (no shared read/write fields).
/// Two sections are independent if no field is read by one and written by the other.
fn are_sections_independent(graph: &CallGraph, section_names: &[String]) -> bool {
    if section_names.len() < 2 {
        return false;
    }

    // Collect reads and writes for each section (including its children)
    let mut section_fields: Vec<(HashSet<String>, HashSet<String>)> = Vec::new();

    for sec_name in section_names {
        let mut reads = HashSet::new();
        let mut writes = HashSet::new();
        collect_section_fields(graph, sec_name, &mut reads, &mut writes, &mut HashSet::new());
        section_fields.push((reads, writes));
    }

    // Check pairwise independence
    for i in 0..section_fields.len() {
        for j in (i + 1)..section_fields.len() {
            let (reads_i, writes_i) = &section_fields[i];
            let (reads_j, writes_j) = &section_fields[j];

            // Conflict if one reads what the other writes
            if !reads_i.is_disjoint(writes_j) || !reads_j.is_disjoint(writes_i) {
                return false;
            }
            // Conflict if both write the same field
            if !writes_i.is_disjoint(writes_j) {
                return false;
            }
        }
    }

    true
}

/// Recursively collect all reads and writes for a section and its children.
fn collect_section_fields(
    graph: &CallGraph,
    node_name: &str,
    reads: &mut HashSet<String>,
    writes: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) {
    if !visited.insert(node_name.to_string()) {
        return;
    }
    if let Some(node) = graph.nodes.iter().find(|n| n.nexflow_name == node_name) {
        reads.extend(node.reads.iter().cloned());
        writes.extend(node.writes.iter().cloned());
        for perf in &node.performs {
            let perf_name = sanitize_identifier(&perf.to_lowercase().replace('-', "_"));
            collect_section_fields(graph, &perf_name, reads, writes, visited);
        }
    }
}

/// Walk the call graph and collect process statements into a Vec.
fn collect_steps(
    body: &mut Vec<ProcessStmt>,
    graph: &CallGraph,
    node_name: &str,
    notes: &mut Vec<String>,
    visited: &mut HashSet<String>,
) {
    // Prevent infinite recursion from cycles
    if !visited.insert(node_name.to_string()) {
        body.push(ProcessStmt::Comment(format!("cycle detected: {node_name} already visited")));
        notes.push(format!("Cycle detected at {node_name} -- may indicate PERFORM UNTIL loop"));
        return;
    }

    let node = match graph.nodes.iter().find(|n| n.nexflow_name == node_name) {
        Some(n) => n,
        None => {
            // Referenced but not found -- emit as external reference
            body.push(ProcessStmt::TransformUsing {
                input: Ident::new("input_records"),
                using: Ident::new(node_name),
                output: Ident::new("output_records"),
            });
            return;
        }
    };

    match node.role {
        NodeRole::EntryPoint => {
            // Entry point: walk its performs
            body.push(ProcessStmt::Comment(format!("Entry: {}", node.cobol_name)));

            // Check if performed sections are independent (no shared fields)
            let perf_names: Vec<String> = node.performs.iter()
                .map(|p| sanitize_identifier(&p.to_lowercase().replace('-', "_")))
                .collect();

            if perf_names.len() >= 2 && are_sections_independent(graph, &perf_names) {
                // Independent sections -> parallel fan-out
                let mut branches = Vec::new();
                for perf_name in &perf_names {
                    let mut branch_body = Vec::new();
                    collect_steps(&mut branch_body, graph, perf_name, notes, visited);
                    branches.push((Ident::new(perf_name), branch_body));
                }
                body.push(ProcessStmt::Parallel(ParallelBlock { branches }));
            } else {
                // Sequential execution
                for perf_name in &perf_names {
                    collect_steps(body, graph, perf_name, notes, visited);
                }
            }
        }
        NodeRole::SectionDispatcher => {
            // Section dispatcher: walk its performs in order
            body.push(ProcessStmt::Comment(format!("Section: {}", node.cobol_name)));
            for perf in &node.performs {
                let perf_name = sanitize_identifier(&perf.to_lowercase().replace('-', "_"));
                collect_steps(body, graph, &perf_name, notes, visited);
            }
        }
        NodeRole::ProcessingStep => {
            // Has both performs and data access
            let reads_str = if node.reads.is_empty() {
                "none".to_string()
            } else {
                node.reads.join(",")
            };
            let writes_str = if node.writes.is_empty() {
                "none".to_string()
            } else {
                node.writes.join(",")
            };
            body.push(ProcessStmt::Comment(format!(
                "Step: {} (reads: {}, writes: {})",
                node.cobol_name, reads_str, writes_str,
            )));

            // Emit as transform + check for parallel fan-out
            body.push(ProcessStmt::TransformUsing {
                input: Ident::new("input_records"),
                using: Ident::new(&node.nexflow_name),
                output: Ident::new("output_records"),
            });

            let perf_names: Vec<String> = node.performs.iter()
                .map(|p| sanitize_identifier(&p.to_lowercase().replace('-', "_")))
                .collect();

            if perf_names.len() >= 2 && are_sections_independent(graph, &perf_names) {
                let mut branches = Vec::new();
                for pn in &perf_names {
                    let mut branch_body = Vec::new();
                    collect_steps(&mut branch_body, graph, pn, notes, visited);
                    branches.push((Ident::new(pn), branch_body));
                }
                body.push(ProcessStmt::Parallel(ParallelBlock { branches }));
            } else {
                for pn in &perf_names {
                    collect_steps(body, graph, pn, notes, visited);
                }
            }
        }
        NodeRole::Leaf => {
            // Leaf paragraph: actual work happens here
            let has_decision = node.reads.len() > 1 || node.writes.len() > 1;

            if has_decision {
                // Likely a rule evaluation (multiple inputs -> multiple outputs)
                body.push(ProcessStmt::Comment(format!(
                    "Evaluate: {} (decision logic)", node.cobol_name
                )));
                body.push(ProcessStmt::EvaluateUsing(Ident::new(&node.nexflow_name)));
            } else {
                // Simple transform
                body.push(ProcessStmt::Comment(format!("Transform: {}", node.cobol_name)));
                body.push(ProcessStmt::TransformUsing {
                    input: Ident::new("input_records"),
                    using: Ident::new(&node.nexflow_name),
                    output: Ident::new("output_records"),
                });
            }
        }
    }
}

// ---------------------------------------------------------------------------
// I/O detection
// ---------------------------------------------------------------------------

/// Detected I/O patterns from field naming conventions.
struct IoInfo {
    /// Fields that look like file inputs (names with FILE, INPUT, READ)
    input_fields: Vec<String>,
    /// Fields that look like file outputs (names with OUTPUT, WRITE, PRINT, REPORT)
    output_fields: Vec<String>,
}

fn detect_io(graph: &CallGraph) -> IoInfo {
    let mut all_reads = HashSet::new();
    let mut all_writes = HashSet::new();

    for node in &graph.nodes {
        for r in &node.reads {
            all_reads.insert(r.clone());
        }
        for w in &node.writes {
            all_writes.insert(w.clone());
        }
    }

    let input_fields: Vec<String> = all_reads
        .iter()
        .filter(|f| is_io_field(f))
        .cloned()
        .collect();

    let output_fields: Vec<String> = all_writes
        .iter()
        .filter(|f| is_io_field(f))
        .cloned()
        .collect();

    IoInfo {
        input_fields,
        output_fields,
    }
}

/// Check if a field name suggests file I/O.
fn is_io_field(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("file")
        || lower.contains("input")
        || lower.contains("output")
        || lower.contains("print")
        || lower.contains("report")
        || lower.contains("record")
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn is_boilerplate(name: &str) -> bool {
    matches!(name, "main" | "new" | "stop_run" | "goback")
}

fn rust_name_to_cobol(name: &str) -> String {
    name.to_uppercase().replace('_', "-")
}

pub fn sanitize_identifier(name: &str) -> String {
    let s = name.to_lowercase().replace(['-', ' '], "_");
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{s}")
    } else if s.is_empty() {
        "process".to_string()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::EmitterContext;

    #[test]
    fn skip_empty_program() {
        let code = r#"
            pub struct WorkingStorage {
                pub x: i32,
            }
            fn helper() {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = ProcessEmitter;
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
        assert!(files.is_empty(), "No annotated functions -> no process file");
    }

    #[test]
    fn simple_linear_process() {
        let code = r#"
            #[cobol(program = "SIMPLE")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
            }

            #[cobol(performs = "INIT-PARA,CALC-PARA,FINISH-PARA")]
            fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "MAIN-SECTION", writes = "WS-COUNT")]
            fn init_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "MAIN-SECTION", reads = "WS-COUNT", writes = "WS-COUNT")]
            fn calc_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "MAIN-SECTION", reads = "WS-COUNT")]
            fn finish_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = ProcessEmitter;
        let ctx = EmitterContext {
            program_name: "SIMPLE".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert_eq!(files.len(), 1, "Should produce one .proc file");
        assert_eq!(files[0].path, "process/simple.proc");

        let content = &files[0].content;
        assert!(content.contains("process simple"), "Should have process name: {content}");
        assert!(content.contains("end"), "Should be terminated");
        assert!(content.contains("import"), "Should import schemas");
        // Steps should reference the paragraphs
        assert!(content.contains("init_para"), "Should reference init_para: {content}");
        assert!(content.contains("calc_para"), "Should reference calc_para: {content}");
        assert!(content.contains("finish_para"), "Should reference finish_para: {content}");
    }

    #[test]
    fn section_dispatcher_expands() {
        let code = r#"
            #[cobol(program = "SECTIONED")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
            }

            #[cobol(performs = "PROCESSING-SECTION")]
            fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "PROCESSING-SECTION", performs = "STEP-A,STEP-B")]
            fn processing_section(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "PROCESSING-SECTION", writes = "WS-COUNT")]
            fn step_a(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "PROCESSING-SECTION", reads = "WS-COUNT", writes = "WS-COUNT")]
            fn step_b(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = ProcessEmitter;
        let ctx = EmitterContext {
            program_name: "SECTIONED".to_string(),
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
        // Section dispatcher should expand into its children
        assert!(content.contains("step_a"), "Should expand section to step_a: {content}");
        assert!(content.contains("step_b"), "Should expand section to step_b: {content}");
        assert!(content.contains("Section"), "Should have section comment: {content}");
    }

    #[test]
    fn io_fields_generate_receive_emit() {
        let code = r#"
            #[cobol(program = "IOBATCH")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "X(80)")]
                pub ws_input_record: PicX,
                #[cobol(level = 1, pic = "X(132)")]
                pub ws_output_record: PicX,
            }

            #[cobol(performs = "PROCESS-PARA")]
            fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "MAIN-SECTION", reads = "WS-INPUT-RECORD", writes = "WS-OUTPUT-RECORD")]
            fn process_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = ProcessEmitter;
        let ctx = EmitterContext {
            program_name: "IOBATCH".to_string(),
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
        // I/O fields with "input"/"output" in name should trigger receive/emit
        assert!(
            content.contains("receive"),
            "Input field should trigger receive block: {content}"
        );
        assert!(
            content.contains("emit to"),
            "Output field should trigger emit block: {content}"
        );
    }

    #[test]
    fn cycle_detection() {
        let code = r#"
            #[cobol(program = "CYCLE")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
            }

            #[cobol(performs = "PARA-A")]
            fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "MAIN-SECTION", performs = "PARA-B", reads = "WS-COUNT", writes = "WS-COUNT")]
            fn para_a(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "MAIN-SECTION", performs = "PARA-A", reads = "WS-COUNT")]
            fn para_b(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = ProcessEmitter;
        let ctx = EmitterContext {
            program_name: "CYCLE".to_string(),
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
        assert!(
            content.contains("cycle detected"),
            "Should detect and comment on cycle: {content}"
        );
        // Should still produce valid output, not infinite loop
        assert!(content.contains("end"), "Should still terminate properly");
    }

    #[test]
    fn multi_input_leaf_uses_evaluate() {
        let code = r#"
            #[cobol(program = "DECIDE")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(3)")]
                pub ws_score: PackedDecimal,
                #[cobol(level = 1, pic = "X(1)")]
                pub ws_type: PicX,
                #[cobol(level = 1, pic = "X(10)")]
                pub ws_decision: PicX,
                #[cobol(level = 1, pic = "X(5)")]
                pub ws_code: PicX,
            }

            #[cobol(performs = "DECIDE-PARA")]
            fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "DECISION-SECTION", reads = "WS-SCORE,WS-TYPE", writes = "WS-DECISION,WS-CODE")]
            fn decide_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = ProcessEmitter;
        let ctx = EmitterContext {
            program_name: "DECIDE".to_string(),
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
        // Multi-input/multi-output leaf -> evaluate (decision logic)
        assert!(
            content.contains("evaluate using"),
            "Multi-input leaf should use evaluate: {content}"
        );
    }

    #[test]
    fn process_file_path_format() {
        let code = r#"
            #[cobol(program = "CARD-PROC")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
            }

            #[cobol(performs = "INIT-PARA")]
            fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "MAIN-SECTION", writes = "WS-COUNT")]
            fn init_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = ProcessEmitter;
        let ctx = EmitterContext {
            program_name: "CARD-PROC".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "process/card_proc.proc");
    }

    #[test]
    fn imports_reference_sections() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
            }

            #[cobol(performs = "INIT-PARA")]
            fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "PROCESSING-SECTION", writes = "WS-COUNT")]
            fn init_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = ProcessEmitter;
        let ctx = EmitterContext {
            program_name: "TESTPROG".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        let content = &files[0].content;

        assert!(
            content.contains("import ../transform/processing_section.xform"),
            "Should import section transform: {content}"
        );
        assert!(
            content.contains("import ../rules/processing_section.rules"),
            "Should import section rules: {content}"
        );
        assert!(
            content.contains("import ../schema/TESTPROG.schema"),
            "Should import schema: {content}"
        );
    }
}
