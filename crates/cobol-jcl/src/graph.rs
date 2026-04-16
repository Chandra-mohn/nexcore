//! JCL process graph -- converts a parsed JCL job into a visual DAG model.
//!
//! The output is a `ProcessGraph` that serializes to JSON for rendering
//! in NexStudio (Svelte + @xyflow/svelte + ELK layout).
//!
//! Node types:
//! - Step: an EXEC PGM= or EXEC proc step
//! - Condition: an IF test (diamond node, true/false branches)
//! - Join: where IF/ELSE branches reconverge at ENDIF
//!
//! Edge types:
//! - Sequential: step A -> step B (normal flow)
//! - ConditionalTrue: condition -> step (when test passes)
//! - ConditionalFalse: condition -> step (ELSE branch)

use serde::Serialize;

use crate::ast::{
    CondOperator, CondParam, DdKind, DdStatement, ExecType, IfCondition, IfTest, JclJob, JclStep,
    JobBodyItem,
};
use crate::resolve::ProcResolver;

// ---------------------------------------------------------------------------
// Graph model
// ---------------------------------------------------------------------------

/// A process graph for a JCL job.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessGraph {
    /// Job name.
    pub job_name: String,
    /// Job class (A-Z, 0-9).
    pub class: Option<String>,
    /// Total step count.
    pub step_count: usize,
    /// Graph nodes.
    pub nodes: Vec<ProcessNode>,
    /// Graph edges.
    pub edges: Vec<ProcessEdge>,
}

/// A node in the process graph.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessNode {
    /// Unique node ID (step name or generated).
    pub id: String,
    /// Display label.
    pub label: String,
    /// Node kind (determines shape/color in UI).
    pub kind: NodeKind,
    /// Program name (for step nodes).
    pub program: Option<String>,
    /// Procedure name (for proc-call steps).
    pub proc_name: Option<String>,
    /// Utility sub-type (sort, idcams, db2, noop).
    pub utility_type: Option<String>,
    /// PARM= value passed to program.
    pub parm: Option<String>,
    /// COND= description (for step-level conditions).
    pub cond: Option<String>,
    /// Condition expression (for condition nodes).
    pub condition_expr: Option<String>,
    /// Input DD summary.
    pub inputs: Vec<DdSummary>,
    /// Output DD summary.
    pub outputs: Vec<DdSummary>,
    /// System/utility DD names (SYSOUT, SORTWK, etc.).
    pub system_dds: Vec<String>,
    /// Child sub-graph (resolved proc steps). None for leaf nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<SubGraph>,
    /// Whether the sub-graph is collapsed in the UI (default: true).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub collapsed: bool,
    /// Number of steps inside a proc (shown on collapsed badge).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_step_count: Option<usize>,
}

/// A sub-graph representing a resolved procedure's internal steps.
#[derive(Debug, Clone, Serialize)]
pub struct SubGraph {
    /// Proc name.
    pub proc_name: String,
    /// Symbolic parameters with defaults.
    pub symbols: Vec<(String, Option<String>)>,
    /// Nodes inside the proc.
    pub nodes: Vec<ProcessNode>,
    /// Edges inside the proc.
    pub edges: Vec<ProcessEdge>,
}

/// Node kind -- determines visual appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    /// Normal program execution step.
    Step,
    /// Utility step (SORT, IDCAMS, etc.).
    Utility,
    /// Allocation-only step (IEFBR14).
    AllocOnly,
    /// DB2 utility step.
    Db2,
    /// Procedure call step.
    ProcCall,
    /// IF condition (diamond node).
    Condition,
    /// ENDIF join point (where branches reconverge).
    Join,
}

/// Summary of a DD statement for display.
#[derive(Debug, Clone, Serialize)]
pub struct DdSummary {
    /// DD name (e.g., SORTIN, INPUT01).
    pub dd_name: String,
    /// Dataset name.
    pub dsn: Option<String>,
    /// DISP status (NEW, OLD, SHR, MOD).
    pub disp: Option<String>,
    /// Record format.
    pub recfm: Option<String>,
    /// Logical record length.
    pub lrecl: Option<u32>,
    /// Is inline data (DD * / DD DATA).
    pub inline: bool,
}

/// An edge in the process graph.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessEdge {
    /// Source node ID.
    pub from: String,
    /// Target node ID.
    pub to: String,
    /// Edge kind.
    pub kind: EdgeKind,
    /// Label (for conditional edges).
    pub label: Option<String>,
}

/// Edge kind -- determines visual style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    /// Normal sequential flow.
    Sequential,
    /// True branch from a condition.
    ConditionalTrue,
    /// False branch from a condition (ELSE).
    ConditionalFalse,
}

// ---------------------------------------------------------------------------
// Graph builder
// ---------------------------------------------------------------------------

/// Build a process graph from a parsed JCL job (no proc resolution).
pub fn build_process_graph(job: &JclJob) -> ProcessGraph {
    build_process_graph_impl(job, None)
}

/// Build a process graph with proc resolution.
///
/// When a step is `EXEC proc_name`, the resolver finds and parses
/// the proc file, embedding its steps as a sub-graph on the node.
/// Nesting is recursive up to the resolver's max depth.
pub fn build_process_graph_with_resolver(
    job: &JclJob,
    resolver: &mut ProcResolver,
) -> ProcessGraph {
    build_process_graph_impl(job, Some(resolver))
}

fn build_process_graph_impl(job: &JclJob, resolver: Option<&mut ProcResolver>) -> ProcessGraph {
    let mut builder = GraphBuilder::new(&job.name, resolver);

    for item in &job.body {
        builder.process_item(item);
    }

    builder.finalize();

    ProcessGraph {
        job_name: job.name.clone(),
        class: job.params.class.clone(),
        step_count: job.steps().len(),
        nodes: builder.nodes,
        edges: builder.edges,
    }
}

struct GraphBuilder<'a> {
    nodes: Vec<ProcessNode>,
    edges: Vec<ProcessEdge>,
    /// ID of the last emitted step/join node (for sequential edges).
    last_node_id: Option<String>,
    /// Counter for generating unique IDs.
    counter: usize,
    /// Stack of condition context for IF/ELSE/ENDIF nesting.
    cond_stack: Vec<CondContext>,
    /// Optional proc resolver for expanding EXEC proc steps.
    resolver: Option<&'a mut ProcResolver>,
    /// Current nesting depth (to prevent infinite recursion).
    depth: usize,
}

/// Tracks state during IF/ELSE/ENDIF processing.
struct CondContext {
    /// ID of the condition (diamond) node.
    condition_id: String,
    /// ID of the join node (ENDIF).
    join_id: String,
    /// Whether we are in the ELSE branch.
    in_else: bool,
    /// Last node ID before the IF (for connecting the true branch start).
    _pre_if_node: Option<String>,
    /// Last node in the true branch (connects to join).
    last_true_node: Option<String>,
    /// Last node in the false branch (connects to join).
    last_false_node: Option<String>,
}

impl<'a> GraphBuilder<'a> {
    fn new(job_name: &str, resolver: Option<&'a mut ProcResolver>) -> Self {
        let _ = job_name;
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            last_node_id: None,
            counter: 0,
            cond_stack: Vec::new(),
            resolver,
            depth: 0,
        }
    }

    fn next_id(&mut self, prefix: &str) -> String {
        self.counter += 1;
        format!("{prefix}_{}", self.counter)
    }

    fn process_item(&mut self, item: &JobBodyItem) {
        match item {
            JobBodyItem::Step(step) => self.add_step(step),
            JobBodyItem::If(if_stmt) => self.open_condition(if_stmt),
            JobBodyItem::Else => self.switch_to_else(),
            JobBodyItem::EndIf => self.close_condition(),
            JobBodyItem::Comment(_) | JobBodyItem::Include(_) | JobBodyItem::Set { .. } => {}
        }
    }

    fn add_step(&mut self, step: &JclStep) {
        let id = step.name.clone().unwrap_or_else(|| self.next_id("step"));
        let mut node = build_step_node(&id, step);

        // Resolve proc sub-graph if this is a proc call and we have a resolver
        if node.kind == NodeKind::ProcCall {
            if let Some(proc_name) = step.proc_name() {
                let max_depth = self.resolver.as_ref().map_or(0, |r| r.max_depth());
                if self.depth < max_depth {
                    // Temporarily take resolver out of self to avoid borrow conflict
                    if let Some(mut resolver) = self.resolver.take() {
                        let resolved = resolver.resolve(proc_name).cloned();
                        if let Some(proc) = resolved {
                            let sub = build_sub_graph(&proc, &mut resolver, self.depth + 1);
                            node.child_step_count = Some(sub.nodes.iter().filter(|n| {
                                !matches!(n.kind, NodeKind::Condition | NodeKind::Join)
                            }).count());
                            node.collapsed = true;
                            node.children = Some(sub);
                        }
                        // Put resolver back
                        self.resolver = Some(resolver);
                    }
                }
            }
        }

        // Connect from previous node
        if let Some(ref from) = self.last_node_id {
            let edge_kind = if let Some(ctx) = self.cond_stack.last() {
                if ctx.in_else {
                    // First step in ELSE branch: edge from condition node
                    if ctx.last_false_node.is_none() {
                        self.edges.push(ProcessEdge {
                            from: ctx.condition_id.clone(),
                            to: id.clone(),
                            kind: EdgeKind::ConditionalFalse,
                            label: Some("else".to_string()),
                        });
                        // Don't also add a sequential edge
                        self.nodes.push(node);
                        self.last_node_id = Some(id.clone());
                        if let Some(ctx) = self.cond_stack.last_mut() {
                            ctx.last_false_node = Some(id);
                        }
                        return;
                    }
                    EdgeKind::Sequential
                } else if ctx.last_true_node.is_none() {
                    // First step in TRUE branch: edge from condition node
                    self.edges.push(ProcessEdge {
                        from: ctx.condition_id.clone(),
                        to: id.clone(),
                        kind: EdgeKind::ConditionalTrue,
                        label: Some("then".to_string()),
                    });
                    self.nodes.push(node);
                    self.last_node_id = Some(id.clone());
                    if let Some(ctx) = self.cond_stack.last_mut() {
                        ctx.last_true_node = Some(id);
                    }
                    return;
                } else {
                    EdgeKind::Sequential
                }
            } else {
                EdgeKind::Sequential
            };

            self.edges.push(ProcessEdge {
                from: from.clone(),
                to: id.clone(),
                kind: edge_kind,
                label: None,
            });
        }

        self.nodes.push(node);
        self.last_node_id = Some(id.clone());

        // Track last node in current branch
        if let Some(ctx) = self.cond_stack.last_mut() {
            if ctx.in_else {
                ctx.last_false_node = Some(id);
            } else {
                ctx.last_true_node = Some(id);
            }
        }
    }

    fn open_condition(&mut self, if_stmt: &crate::ast::IfStatement) {
        let cond_id = self.next_id("cond");
        let join_id = self.next_id("join");

        let expr = format_condition(&if_stmt.condition);

        let cond_node = ProcessNode {
            id: cond_id.clone(),
            label: expr.clone(),
            kind: NodeKind::Condition,
            program: None,
            proc_name: None,
            utility_type: None,
            parm: None,
            cond: None,
            condition_expr: Some(expr),
            inputs: Vec::new(),
            outputs: Vec::new(),
            system_dds: Vec::new(),
            children: None,
            collapsed: false,
            child_step_count: None,
        };

        // Connect from previous node
        if let Some(ref from) = self.last_node_id {
            self.edges.push(ProcessEdge {
                from: from.clone(),
                to: cond_id.clone(),
                kind: EdgeKind::Sequential,
                label: None,
            });
        }

        self.nodes.push(cond_node);

        let pre_if = self.last_node_id.clone();
        self.last_node_id = Some(cond_id.clone());

        self.cond_stack.push(CondContext {
            condition_id: cond_id,
            join_id,
            in_else: false,
            _pre_if_node: pre_if,
            last_true_node: None,
            last_false_node: None,
        });
    }

    fn switch_to_else(&mut self) {
        if let Some(ctx) = self.cond_stack.last_mut() {
            ctx.in_else = true;
            // Reset last_node_id to condition node so ELSE branch starts from there
            self.last_node_id = Some(ctx.condition_id.clone());
        }
    }

    fn close_condition(&mut self) {
        let Some(ctx) = self.cond_stack.pop() else {
            return;
        };

        let join_node = ProcessNode {
            id: ctx.join_id.clone(),
            label: "endif".to_string(),
            kind: NodeKind::Join,
            program: None,
            proc_name: None,
            utility_type: None,
            parm: None,
            cond: None,
            condition_expr: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            system_dds: Vec::new(),
            children: None,
            collapsed: false,
            child_step_count: None,
        };
        self.nodes.push(join_node);

        // Connect true branch end -> join
        if let Some(ref true_end) = ctx.last_true_node {
            self.edges.push(ProcessEdge {
                from: true_end.clone(),
                to: ctx.join_id.clone(),
                kind: EdgeKind::Sequential,
                label: None,
            });
        } else {
            // Empty true branch: condition -> join directly
            self.edges.push(ProcessEdge {
                from: ctx.condition_id.clone(),
                to: ctx.join_id.clone(),
                kind: EdgeKind::ConditionalTrue,
                label: Some("then (empty)".to_string()),
            });
        }

        // Connect false branch end -> join
        if let Some(ref false_end) = ctx.last_false_node {
            self.edges.push(ProcessEdge {
                from: false_end.clone(),
                to: ctx.join_id.clone(),
                kind: EdgeKind::Sequential,
                label: None,
            });
        } else {
            // No ELSE branch: condition -> join directly for false path
            self.edges.push(ProcessEdge {
                from: ctx.condition_id.clone(),
                to: ctx.join_id.clone(),
                kind: EdgeKind::ConditionalFalse,
                label: Some("else (skip)".to_string()),
            });
        }

        self.last_node_id = Some(ctx.join_id);
    }

    fn finalize(&mut self) {
        // Close any unclosed conditions
        while !self.cond_stack.is_empty() {
            self.close_condition();
        }
    }
}

// ---------------------------------------------------------------------------
// Node construction
// ---------------------------------------------------------------------------

fn build_step_node(id: &str, step: &JclStep) -> ProcessNode {
    let (kind, utility_type) = classify_step(step);
    let program = step.program().map(str::to_string);
    let proc_name = step.proc_name().map(str::to_string);

    let label = match &step.exec {
        ExecType::Pgm(pgm) => pgm.clone(),
        ExecType::Proc { name, .. } => format!("PROC {name}"),
    };

    let cond = step.params.cond.as_ref().map(format_cond_param);
    let parm = step.params.parm.clone();

    let (inputs, outputs, system_dds) = summarize_dds(&step.dd_statements);

    ProcessNode {
        id: id.to_string(),
        label,
        kind,
        program,
        proc_name,
        utility_type,
        parm,
        cond,
        condition_expr: None,
        inputs,
        outputs,
        system_dds,
        children: None,
        collapsed: false,
        child_step_count: None,
    }
}

/// Build a sub-graph from a resolved proc, recursively resolving nested procs.
fn build_sub_graph(
    proc: &crate::ast::JclProc,
    resolver: &mut ProcResolver,
    depth: usize,
) -> SubGraph {
    let mut builder = GraphBuilder::new(
        proc.name.as_deref().unwrap_or("PROC"),
        Some(resolver),
    );
    builder.depth = depth;

    for item in &proc.body {
        builder.process_item(item);
    }
    builder.finalize();

    let symbols: Vec<(String, Option<String>)> = proc
        .symbols
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    SubGraph {
        proc_name: proc.name.clone().unwrap_or_default(),
        symbols,
        nodes: builder.nodes,
        edges: builder.edges,
    }
}

/// Classify a step by its program name.
fn classify_step(step: &JclStep) -> (NodeKind, Option<String>) {
    if step.is_noop() {
        return (NodeKind::AllocOnly, Some("noop".to_string()));
    }
    if step.is_sort() {
        return (NodeKind::Utility, Some("sort".to_string()));
    }
    if step.is_idcams() {
        return (NodeKind::Utility, Some("idcams".to_string()));
    }
    if step.is_db2() {
        return (NodeKind::Db2, Some("db2".to_string()));
    }

    // Check for other known utilities
    if let Some(pgm) = step.program() {
        let upper = pgm.to_uppercase();
        match upper.as_str() {
            "IEBGENER" | "IEBCOPY" | "IEBUPDTE" => {
                return (NodeKind::Utility, Some("ieb".to_string()));
            }
            "ADRDSSU" => {
                return (NodeKind::Utility, Some("dfdss".to_string()));
            }
            "IDCAMS" => {
                return (NodeKind::Utility, Some("idcams".to_string()));
            }
            "FTP" => {
                return (NodeKind::Utility, Some("ftp".to_string()));
            }
            _ => {}
        }
    }

    if step.proc_name().is_some() {
        return (NodeKind::ProcCall, None);
    }

    (NodeKind::Step, None)
}

/// Partition DD statements into inputs, outputs, and system DDs.
fn summarize_dds(dds: &[DdStatement]) -> (Vec<DdSummary>, Vec<DdSummary>, Vec<String>) {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut system = Vec::new();

    for dd in dds {
        if dd.is_system_dd() {
            // Some system DDs are meaningful inputs/outputs
            let upper = dd.name.to_uppercase();
            match upper.as_str() {
                "SYSIN" | "SORTIN" => {
                    inputs.push(build_dd_summary(dd));
                }
                "SYSPRINT" | "SYSOUT" | "SORTOUT" => {
                    outputs.push(build_dd_summary(dd));
                }
                _ => {
                    system.push(dd.name.clone());
                }
            }
            continue;
        }

        // Non-system DDs: classify by DISP
        match &dd.kind {
            DdKind::Dataset(ds) => {
                if let Some(ref disp) = ds.disp {
                    match disp.status {
                        crate::ast::DispStatus::New | crate::ast::DispStatus::Mod => {
                            outputs.push(build_dd_summary(dd));
                        }
                        crate::ast::DispStatus::Old | crate::ast::DispStatus::Shr => {
                            inputs.push(build_dd_summary(dd));
                        }
                    }
                } else {
                    // No DISP -- assume input
                    inputs.push(build_dd_summary(dd));
                }
            }
            DdKind::Sysout(_) => {
                outputs.push(build_dd_summary(dd));
            }
            DdKind::Dummy | DdKind::Dynam => {
                system.push(dd.name.clone());
            }
            DdKind::InlineData { .. } | DdKind::DdRef(_) | DdKind::Path(_) => {
                inputs.push(build_dd_summary(dd));
            }
        }
    }

    (inputs, outputs, system)
}

fn build_dd_summary(dd: &DdStatement) -> DdSummary {
    match &dd.kind {
        DdKind::Dataset(ds) => DdSummary {
            dd_name: dd.name.clone(),
            dsn: ds.dsn.clone(),
            disp: ds.disp.as_ref().map(|d| d.status.to_string()),
            recfm: ds.dcb.recfm.clone(),
            lrecl: ds.dcb.lrecl,
            inline: false,
        },
        DdKind::Sysout(sys) => DdSummary {
            dd_name: dd.name.clone(),
            dsn: None,
            disp: Some(format!("SYSOUT={}", sys.class)),
            recfm: None,
            lrecl: None,
            inline: false,
        },
        DdKind::InlineData { .. } => DdSummary {
            dd_name: dd.name.clone(),
            dsn: None,
            disp: None,
            recfm: None,
            lrecl: None,
            inline: true,
        },
        DdKind::Path(p) => DdSummary {
            dd_name: dd.name.clone(),
            dsn: Some(p.path.clone()),
            disp: None,
            recfm: None,
            lrecl: None,
            inline: false,
        },
        DdKind::DdRef(name) => DdSummary {
            dd_name: dd.name.clone(),
            dsn: Some(format!("DDNAME={name}")),
            disp: None,
            recfm: None,
            lrecl: None,
            inline: false,
        },
        DdKind::Dummy | DdKind::Dynam => DdSummary {
            dd_name: dd.name.clone(),
            dsn: None,
            disp: Some("DUMMY".to_string()),
            recfm: None,
            lrecl: None,
            inline: false,
        },
    }
}

// ---------------------------------------------------------------------------
// Formatting helpers
// ---------------------------------------------------------------------------

fn format_condition(cond: &IfCondition) -> String {
    match cond {
        IfCondition::Test(test) => format_if_test(test),
        IfCondition::Not(inner) => format!("NOT ({})", format_condition(inner)),
        IfCondition::And(left, right) => {
            format!("{} AND {}", format_condition(left), format_condition(right))
        }
        IfCondition::Or(left, right) => {
            format!("{} OR {}", format_condition(left), format_condition(right))
        }
    }
}

fn format_if_test(test: &IfTest) -> String {
    match test {
        IfTest::Rc { step, operator, value } => {
            let prefix = step.as_deref().map_or("RC".to_string(), |s| format!("{s}.RC"));
            format!("{prefix} {} {value}", op_symbol(operator))
        }
        IfTest::Abend { step, value } => {
            let prefix = step.as_deref().map_or("ABEND".to_string(), |s| format!("{s}.ABEND"));
            format!("{prefix} = {value}")
        }
        IfTest::AbendCc { step, operator, value } => {
            let prefix =
                step.as_deref().map_or("ABENDCC".to_string(), |s| format!("{s}.ABENDCC"));
            format!("{prefix} {} {value}", op_symbol(operator))
        }
        IfTest::Run { step, value } => {
            let prefix = step.as_deref().map_or("RUN".to_string(), |s| format!("{s}.RUN"));
            format!("{prefix} = {value}")
        }
    }
}

fn op_symbol(op: &CondOperator) -> &'static str {
    match op {
        CondOperator::Gt => ">",
        CondOperator::Ge => ">=",
        CondOperator::Eq => "=",
        CondOperator::Lt => "<",
        CondOperator::Le => "<=",
        CondOperator::Ne => "!=",
    }
}

fn format_cond_param(cond: &CondParam) -> String {
    match cond {
        CondParam::Even => "EVEN".to_string(),
        CondParam::Only => "ONLY".to_string(),
        CondParam::Tests(tests) => {
            let parts: Vec<String> = tests
                .iter()
                .map(|t| {
                    let step_part = t.step.as_deref().map_or(String::new(), |s| format!(",{s}"));
                    format!("({},{}{step_part})", t.code, t.operator)
                })
                .collect();
            parts.join(",")
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::JclSource;
    use crate::parser::parse_jcl;

    #[test]
    fn linear_job_graph() {
        let jcl = r#"
//MYJOB   JOB ,'TEST',CLASS=A,MSGCLASS=X
//STEP1   EXEC PGM=IEFBR14
//NEWDS   DD DSN=MY.NEW.FILE,DISP=(NEW,CATLG),SPACE=(CYL,(1,1)),UNIT=SYSDA
//STEP2   EXEC PGM=SORT
//SORTIN  DD DSN=MY.INPUT,DISP=SHR
//SORTOUT DD DSN=MY.SORTED,DISP=(NEW,CATLG),SPACE=(CYL,(5,2))
//SYSIN   DD *
//SYSOUT  DD SYSOUT=*
//STEP3   EXEC PGM=MYPROG
//INPUT   DD DSN=MY.SORTED,DISP=SHR
//OUTPUT  DD DSN=MY.RESULT,DISP=(NEW,CATLG),SPACE=(CYL,(10,5))
//SYSOUT  DD SYSOUT=*
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };
        let graph = build_process_graph(&job);

        assert_eq!(graph.job_name, "MYJOB");
        assert_eq!(graph.class.as_deref(), Some("A"));
        assert_eq!(graph.step_count, 3);

        // 3 step nodes
        assert_eq!(graph.nodes.len(), 3);

        // Node kinds
        assert_eq!(graph.nodes[0].kind, NodeKind::AllocOnly);
        assert_eq!(graph.nodes[0].utility_type.as_deref(), Some("noop"));
        assert_eq!(graph.nodes[1].kind, NodeKind::Utility);
        assert_eq!(graph.nodes[1].utility_type.as_deref(), Some("sort"));
        assert_eq!(graph.nodes[2].kind, NodeKind::Step);

        // Edges: linear chain
        assert_eq!(graph.edges.len(), 2);
        assert_eq!(graph.edges[0].from, "STEP1");
        assert_eq!(graph.edges[0].to, "STEP2");
        assert_eq!(graph.edges[0].kind, EdgeKind::Sequential);
        assert_eq!(graph.edges[1].from, "STEP2");
        assert_eq!(graph.edges[1].to, "STEP3");

        // DD summaries for SORT step
        let sort_node = &graph.nodes[1];
        assert!(!sort_node.inputs.is_empty(), "SORT should have inputs");
        assert!(!sort_node.outputs.is_empty(), "SORT should have outputs");
        assert!(
            sort_node.inputs.iter().any(|d| d.dd_name == "SORTIN"),
            "missing SORTIN"
        );
        assert!(
            sort_node.outputs.iter().any(|d| d.dd_name == "SORTOUT"),
            "missing SORTOUT"
        );

        // DD summaries for MYPROG step
        let prog_node = &graph.nodes[2];
        assert!(
            prog_node.inputs.iter().any(|d| d.dd_name == "INPUT"),
            "missing INPUT"
        );
        assert!(
            prog_node.outputs.iter().any(|d| d.dd_name == "OUTPUT"),
            "missing OUTPUT"
        );
    }

    #[test]
    fn conditional_job_graph() {
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
//STEP4   EXEC PGM=CLEANUP
//SYSOUT  DD SYSOUT=*
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };
        let graph = build_process_graph(&job);

        assert_eq!(graph.step_count, 4);

        // Nodes: STEP1, condition, STEP2, STEP3, join, STEP4
        assert_eq!(graph.nodes.len(), 6);

        let cond_node = graph.nodes.iter().find(|n| n.kind == NodeKind::Condition).unwrap();
        assert!(cond_node.condition_expr.is_some());
        assert!(
            cond_node
                .condition_expr
                .as_ref()
                .unwrap()
                .contains("STEP1.RC"),
            "condition should reference STEP1.RC: {:?}",
            cond_node.condition_expr
        );

        let join_node = graph.nodes.iter().find(|n| n.kind == NodeKind::Join).unwrap();
        assert_eq!(join_node.label, "endif");

        // Check edge types
        let cond_true_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::ConditionalTrue)
            .collect();
        let cond_false_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::ConditionalFalse)
            .collect();

        assert_eq!(cond_true_edges.len(), 1, "should have one true edge");
        assert_eq!(cond_false_edges.len(), 1, "should have one false edge");

        // True edge -> STEP2
        assert_eq!(cond_true_edges[0].to, "STEP2");
        // False edge -> STEP3
        assert_eq!(cond_false_edges[0].to, "STEP3");

        // Both branches should connect to join, then join -> STEP4
        let join_id = &join_node.id;
        assert!(
            graph.edges.iter().any(|e| e.to == *join_id && e.from == "STEP2"),
            "STEP2 should connect to join"
        );
        assert!(
            graph.edges.iter().any(|e| e.to == *join_id && e.from == "STEP3"),
            "STEP3 should connect to join"
        );
        assert!(
            graph.edges.iter().any(|e| e.from == *join_id && e.to == "STEP4"),
            "join should connect to STEP4"
        );
    }

    #[test]
    fn dd_classification() {
        let jcl = r#"
//DDJOB   JOB ,'TEST'
//STEP1   EXEC PGM=MYPROG
//INPUT1  DD DSN=PROD.MASTER,DISP=SHR
//INPUT2  DD DSN=PROD.TRANS,DISP=OLD
//OUTPUT1 DD DSN=PROD.RESULT,DISP=(NEW,CATLG),DCB=(RECFM=FB,LRECL=100)
//REPORT  DD SYSOUT=A
//SYSIN   DD *
//NULLDD  DD DUMMY
//SYSOUT  DD SYSOUT=*
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };
        let graph = build_process_graph(&job);

        let node = &graph.nodes[0];

        // Inputs: INPUT1 (SHR), INPUT2 (OLD), SYSIN (inline)
        let input_names: Vec<&str> = node.inputs.iter().map(|d| d.dd_name.as_str()).collect();
        assert!(input_names.contains(&"INPUT1"), "missing INPUT1: {input_names:?}");
        assert!(input_names.contains(&"INPUT2"), "missing INPUT2: {input_names:?}");
        assert!(input_names.contains(&"SYSIN"), "missing SYSIN: {input_names:?}");

        // Outputs: OUTPUT1 (NEW), REPORT (SYSOUT), SYSOUT (SYSOUT)
        let output_names: Vec<&str> = node.outputs.iter().map(|d| d.dd_name.as_str()).collect();
        assert!(output_names.contains(&"OUTPUT1"), "missing OUTPUT1: {output_names:?}");
        assert!(output_names.contains(&"REPORT"), "missing REPORT: {output_names:?}");
        assert!(output_names.contains(&"SYSOUT"), "missing SYSOUT: {output_names:?}");

        // DCB on OUTPUT1
        let out1 = node.outputs.iter().find(|d| d.dd_name == "OUTPUT1").unwrap();
        assert_eq!(out1.recfm.as_deref(), Some("FB"));
        assert_eq!(out1.lrecl, Some(100));
        assert_eq!(out1.disp.as_deref(), Some("NEW"));

        // System: NULLDD
        assert!(node.system_dds.contains(&"NULLDD".to_string()), "missing NULLDD in system DDs");
    }

    #[test]
    fn proc_call_node() {
        let jcl = r#"
//PROCJOB JOB ,'TEST'
//STEP1   EXEC MYPROC,PARM='HELLO'
//STEP1.INPUT DD DSN=MY.DATA,DISP=SHR
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };
        let graph = build_process_graph(&job);

        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].kind, NodeKind::ProcCall);
        assert_eq!(graph.nodes[0].proc_name.as_deref(), Some("MYPROC"));
        assert_eq!(graph.nodes[0].label, "PROC MYPROC");
    }

    #[test]
    fn json_serialization() {
        let jcl = r#"
//MYJOB   JOB ,'TEST',CLASS=B
//STEP1   EXEC PGM=SORT
//SORTIN  DD DSN=IN.FILE,DISP=SHR
//SORTOUT DD DSN=OUT.FILE,DISP=(NEW,CATLG)
//SYSIN   DD *
//SYSOUT  DD SYSOUT=*
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };
        let graph = build_process_graph(&job);
        let json = serde_json::to_string_pretty(&graph).unwrap();

        // Verify key fields are present in JSON
        assert!(json.contains("\"job_name\": \"MYJOB\""), "missing job_name");
        assert!(json.contains("\"class\": \"B\""), "missing class");
        assert!(json.contains("\"kind\": \"utility\""), "missing utility kind");
        assert!(json.contains("\"utility_type\": \"sort\""), "missing sort type");
        assert!(json.contains("\"dd_name\": \"SORTIN\""), "missing SORTIN");
        assert!(json.contains("\"dd_name\": \"SORTOUT\""), "missing SORTOUT");
    }

    #[test]
    fn proc_resolution_sub_graph() {
        let dir = tempfile::tempdir().unwrap();

        // Write a proc file
        std::fs::write(dir.path().join("MYPROC.jcl"), r#"
//MYPROC  PROC
//PS1     EXEC PGM=SORT
//SORTIN  DD DSN=IN.FILE,DISP=SHR
//SORTOUT DD DSN=OUT.FILE,DISP=(NEW,CATLG)
//SYSIN   DD *
//SYSOUT  DD SYSOUT=*
//PS2     EXEC PGM=UPDATE
//INPUT   DD DSN=OUT.FILE,DISP=SHR
//OUTPUT  DD DSN=FINAL.FILE,DISP=(NEW,CATLG)
// PEND
"#).unwrap();

        let jcl = r#"
//TESTJOB JOB ,'TEST',CLASS=A
//STEP1   EXEC PGM=INIT
//SYSOUT  DD SYSOUT=*
//STEP2   EXEC MYPROC
//STEP3   EXEC PGM=DONE
//SYSOUT  DD SYSOUT=*
//
"#;
        let source = parse_jcl(jcl).unwrap();
        let job = match source {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };

        let mut resolver = ProcResolver::new(vec![dir.path().to_path_buf()]);
        let graph = build_process_graph_with_resolver(&job, &mut resolver);

        assert_eq!(graph.step_count, 3);
        assert_eq!(graph.nodes.len(), 3);

        // STEP2 should be a ProcCall with children
        let step2 = graph.nodes.iter().find(|n| n.id == "STEP2").unwrap();
        assert_eq!(step2.kind, NodeKind::ProcCall);
        assert!(step2.collapsed, "proc should be collapsed by default");
        assert_eq!(step2.child_step_count, Some(2), "proc has 2 steps");

        let sub = step2.children.as_ref().expect("should have sub-graph");
        assert_eq!(sub.proc_name, "MYPROC");
        assert_eq!(sub.nodes.len(), 2);
        assert_eq!(sub.edges.len(), 1); // PS1 -> PS2

        // First sub-node is SORT
        assert_eq!(sub.nodes[0].id, "PS1");
        assert_eq!(sub.nodes[0].kind, NodeKind::Utility);
        assert!(!sub.nodes[0].inputs.is_empty());

        // Second sub-node is UPDATE
        assert_eq!(sub.nodes[1].id, "PS2");
        assert_eq!(sub.nodes[1].kind, NodeKind::Step);

        // STEP1 and STEP3 should NOT have children
        let step1 = graph.nodes.iter().find(|n| n.id == "STEP1").unwrap();
        assert!(step1.children.is_none());
        let step3 = graph.nodes.iter().find(|n| n.id == "STEP3").unwrap();
        assert!(step3.children.is_none());

        // JSON should include nested structure
        let json = serde_json::to_string_pretty(&graph).unwrap();
        assert!(json.contains("\"children\""), "missing children in JSON");
        assert!(json.contains("\"proc_name\": \"MYPROC\""), "missing proc_name");
    }
}
