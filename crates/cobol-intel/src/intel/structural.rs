use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// Layer 1: Structural Intelligence.
///
/// Computes per-program:
/// - `paragraph_count`: number of paragraphs
/// - `field_count`: number of fields
/// - `copybook_count`: number of copybooks used
/// - `call_count`: number of programs called
/// - `file_count`: number of files accessed
/// - `complexity`: composite complexity score
/// - `program_class`: batch / online / subprogram / utility / unknown
#[derive(Debug)]
pub struct StructuralPass;

impl IntelligencePass for StructuralPass {
    fn name(&self) -> &'static str {
        "structural"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        let program_ids = graph.all_of_kind(NodeKind::Program);

        for prog_id in program_ids {
            let paragraphs = graph.neighbors_outgoing(prog_id, EdgeKind::Contains);
            let para_count = paragraphs
                .iter()
                .filter(|id| {
                    graph
                        .node(**id)
                        .is_some_and(|n| n.kind == NodeKind::Paragraph)
                })
                .count();

            let calls = graph.neighbors_outgoing(prog_id, EdgeKind::Calls);
            let call_count = calls.len();

            let copybooks = graph.neighbors_outgoing(prog_id, EdgeKind::Uses);
            let copybook_count = copybooks.len();

            let files_and_tables = graph.neighbors_outgoing(prog_id, EdgeKind::Accesses);
            let file_count = files_and_tables
                .iter()
                .filter(|id| {
                    graph
                        .node(**id)
                        .is_some_and(|n| n.kind == NodeKind::File)
                })
                .count();

            // Count fields belonging to this program (via program property)
            let prog_name = graph.node(prog_id).map(|n| n.name.clone()).unwrap_or_default();
            let field_count = graph
                .nodes_in_program(&prog_name)
                .iter()
                .filter(|id| {
                    graph
                        .node(**id)
                        .is_some_and(|n| n.kind == NodeKind::Field)
                })
                .count();

            // Classify program type
            let has_callers = !graph.neighbors_incoming(prog_id, EdgeKind::Calls).is_empty();
            let existing_type = graph
                .node(prog_id)
                .and_then(|n| n.properties.get_str("type"))
                .map(str::to_owned);

            let program_class = classify_program(
                existing_type.as_deref(),
                has_callers,
                call_count,
                para_count,
                file_count,
            );

            // Compute complexity: weighted composite score
            let complexity = compute_complexity(para_count, call_count, field_count, file_count);

            // Apply properties to the node
            if let Some(node) = graph.node_mut(prog_id) {
                node.properties
                    .set("paragraph_count", PropValue::from(para_count as u64));
                node.properties
                    .set("field_count", PropValue::from(field_count as u64));
                node.properties
                    .set("copybook_count", PropValue::from(copybook_count as u64));
                node.properties
                    .set("call_count", PropValue::from(call_count as u64));
                node.properties
                    .set("file_count", PropValue::from(file_count as u64));
                node.properties
                    .set("complexity", PropValue::from(complexity));
                node.properties
                    .set("program_class", PropValue::from(program_class));

                stats.nodes_enriched += 1;
                stats.properties_added += 7;
            }
        }

        Ok(stats)
    }
}

fn classify_program(
    existing_type: Option<&str>,
    has_callers: bool,
    call_count: usize,
    para_count: usize,
    file_count: usize,
) -> &'static str {
    // If the builder already set a type, respect it
    if let Some(t) = existing_type {
        if t == "batch" || t == "online" || t == "subprogram" {
            return match t {
                "batch" => "batch",
                "online" => "online",
                "subprogram" => "subprogram",
                _ => "unknown",
            };
        }
    }

    // Heuristic classification
    if has_callers && call_count == 0 && para_count <= 5 {
        "utility"
    } else if has_callers {
        "subprogram"
    } else if file_count > 0 {
        "batch"
    } else {
        "unknown"
    }
}

/// Composite complexity score (0.0 - 10.0 scale).
///
/// Weights:
/// - paragraphs: proxy for procedural complexity
/// - calls: proxy for coupling
/// - fields: proxy for data complexity
/// - files: proxy for I/O complexity
fn compute_complexity(
    para_count: usize,
    call_count: usize,
    field_count: usize,
    file_count: usize,
) -> f64 {
    let para_score = (para_count as f64 / 20.0).min(3.0); // max 3 points
    let call_score = (call_count as f64 / 5.0).min(2.0); // max 2 points
    let field_score = (field_count as f64 / 50.0).min(3.0); // max 3 points
    let file_score = (file_count as f64 / 3.0).min(2.0); // max 2 points

    let raw = para_score + call_score + field_score + file_score;
    (raw * 10.0).round() / 10.0 // round to 1 decimal
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    fn make_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        // CLRG0100: 3 paragraphs, calls 2 programs, uses 2 copybooks, 5 fields
        let clrg = g.add_node(Node::new(NodeKind::Program, "CLRG0100"));
        let valutil = g.add_node(Node::new(NodeKind::Program, "VALUTIL"));
        let balupd = g.add_node(Node::new(NodeKind::Program, "BALUPD"));
        let _deadprog = g.add_node(Node::new(NodeKind::Program, "DEADPROG"));

        let p1 = g.add_node(Node::new(NodeKind::Paragraph, "1000-MAIN").with_program("CLRG0100"));
        let p2 = g.add_node(
            Node::new(NodeKind::Paragraph, "2000-VALIDATE").with_program("CLRG0100"),
        );
        let p3 = g.add_node(
            Node::new(NodeKind::Paragraph, "3000-UPDATE").with_program("CLRG0100"),
        );

        for i in 0..5 {
            g.add_node(
                Node::new(NodeKind::Field, &format!("WS-FIELD-{i}")).with_program("CLRG0100"),
            );
        }

        let cb1 = g.add_node(Node::new(NodeKind::Copybook, "CPYCLRG"));
        let cb2 = g.add_node(Node::new(NodeKind::Copybook, "CPYACCT"));
        let f1 = g.add_node(Node::new(NodeKind::File, "ACCTMAST"));

        g.add_edge(clrg, valutil, Edge::new(EdgeKind::Calls));
        g.add_edge(clrg, balupd, Edge::new(EdgeKind::Calls));
        g.add_edge(clrg, p1, Edge::new(EdgeKind::Contains));
        g.add_edge(clrg, p2, Edge::new(EdgeKind::Contains));
        g.add_edge(clrg, p3, Edge::new(EdgeKind::Contains));
        g.add_edge(clrg, cb1, Edge::new(EdgeKind::Uses));
        g.add_edge(clrg, cb2, Edge::new(EdgeKind::Uses));
        g.add_edge(clrg, f1, Edge::new(EdgeKind::Accesses));
        g.add_edge(p1, p2, Edge::new(EdgeKind::Performs));
        g.add_edge(p1, p3, Edge::new(EdgeKind::Performs));

        g
    }

    #[test]
    fn structural_enrichment() {
        let mut g = make_graph();
        let stats = StructuralPass.enrich(&mut g).unwrap();

        assert_eq!(stats.nodes_enriched, 4); // 4 programs

        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let node = g.node(clrg).unwrap();
        assert_eq!(node.properties.get_u64("paragraph_count"), Some(3));
        assert_eq!(node.properties.get_u64("field_count"), Some(5));
        assert_eq!(node.properties.get_u64("copybook_count"), Some(2));
        assert_eq!(node.properties.get_u64("call_count"), Some(2));
        assert_eq!(node.properties.get_u64("file_count"), Some(1));
    }

    #[test]
    fn complexity_scoring() {
        let mut g = make_graph();
        StructuralPass.enrich(&mut g).unwrap();

        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let complexity = g.node(clrg).unwrap().properties.get_f64("complexity").unwrap();
        // 3 paras/20*10=1.5, 2 calls/5*10=4.0->2.0max, 5 fields/50*10=1.0, 1 file/3*10=3.3->2.0max
        // = 0.15 + 0.4 + 0.1 + 0.33 = 0.98... rounded
        assert!(complexity > 0.0, "complexity should be > 0, got {complexity}");
        assert!(complexity <= 10.0, "complexity should be <= 10, got {complexity}");
    }

    #[test]
    fn program_classification() {
        let mut g = make_graph();
        StructuralPass.enrich(&mut g).unwrap();

        // CLRG0100: not called by anyone, has files -> batch
        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        assert_eq!(
            g.node(clrg).unwrap().properties.get_str("program_class"),
            Some("batch")
        );

        // VALUTIL: called by CLRG0100, calls nothing, 0 paragraphs -> utility
        let valutil = g.lookup_one(NodeKind::Program, "VALUTIL").unwrap();
        assert_eq!(
            g.node(valutil).unwrap().properties.get_str("program_class"),
            Some("utility")
        );

        // DEADPROG: no callers, no files -> unknown
        let dead = g.lookup_one(NodeKind::Program, "DEADPROG").unwrap();
        assert_eq!(
            g.node(dead).unwrap().properties.get_str("program_class"),
            Some("unknown")
        );
    }

    #[test]
    fn compute_complexity_fn() {
        // Empty program
        assert_eq!(compute_complexity(0, 0, 0, 0), 0.0);

        // Max complexity (all dimensions saturated)
        let max = compute_complexity(100, 50, 500, 30);
        assert_eq!(max, 10.0);

        // Moderate
        let mid = compute_complexity(10, 3, 25, 1);
        assert!(mid > 0.0 && mid < 10.0);
    }
}
