use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// Layer 3: Data Flow Intelligence.
///
/// Computes per-field:
/// - `read_count`: number of paragraphs that read this field
/// - `write_count`: number of paragraphs that write this field
/// - `access_mode`: read-only / write-only / read-write
/// - `field_role`: input / output / accumulator / temp / unused
///
/// Computes per-paragraph:
/// - `reads_count`: number of fields this paragraph reads
/// - `writes_count`: number of fields this paragraph writes
///
/// Computes per-program:
/// - `data_coupling`: ratio of shared fields across paragraphs (0.0-1.0)
///   High coupling = many paragraphs touching the same fields
#[derive(Debug)]
pub struct DataFlowPass;

impl IntelligencePass for DataFlowPass {
    fn name(&self) -> &'static str {
        "data_flow"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        // Enrich fields
        let field_ids = graph.all_of_kind(NodeKind::Field);
        for field_id in &field_ids {
            let readers = graph.neighbors_incoming(*field_id, EdgeKind::Reads);
            let writers = graph.neighbors_incoming(*field_id, EdgeKind::Writes);
            let read_count = readers.len();
            let write_count = writers.len();

            let access_mode = match (read_count > 0, write_count > 0) {
                (true, true) => "read-write",
                (true, false) => "read-only",
                (false, true) => "write-only",
                (false, false) => "unused",
            };

            let field_role = classify_field_role(read_count, write_count);

            if let Some(node) = graph.node_mut(*field_id) {
                node.properties
                    .set("read_count", PropValue::from(read_count as u64));
                node.properties
                    .set("write_count", PropValue::from(write_count as u64));
                node.properties
                    .set("access_mode", PropValue::from(access_mode));
                node.properties
                    .set("field_role", PropValue::from(field_role));
                stats.nodes_enriched += 1;
                stats.properties_added += 4;
            }
        }

        // Enrich paragraphs with field access counts
        let para_ids = graph.all_of_kind(NodeKind::Paragraph);
        for para_id in &para_ids {
            let reads = graph.neighbors_outgoing(*para_id, EdgeKind::Reads);
            let writes = graph.neighbors_outgoing(*para_id, EdgeKind::Writes);

            if let Some(node) = graph.node_mut(*para_id) {
                node.properties
                    .set("reads_count", PropValue::from(reads.len() as u64));
                node.properties
                    .set("writes_count", PropValue::from(writes.len() as u64));
                stats.nodes_enriched += 1;
                stats.properties_added += 2;
            }
        }

        // Enrich programs with data coupling score
        let program_ids = graph.all_of_kind(NodeKind::Program);
        for prog_id in &program_ids {
            let coupling = compute_data_coupling(graph, *prog_id);
            if let Some(node) = graph.node_mut(*prog_id) {
                node.properties
                    .set("data_coupling", PropValue::from(coupling));
                stats.nodes_enriched += 1;
                stats.properties_added += 1;
            }
        }

        Ok(stats)
    }
}

/// Classify a field's role based on read/write patterns.
fn classify_field_role(read_count: usize, write_count: usize) -> &'static str {
    match (read_count, write_count) {
        (0, 0) => "unused",
        (_, 0) => "input",        // only read, never written -> external input
        (0, _) => "output",       // only written, never read -> output
        (r, w) if w > r => "accumulator", // written more than read -> accumulator/counter
        _ => "temp",              // read and written roughly equally -> working variable
    }
}

/// Compute data coupling for a program: how much do paragraphs share fields?
/// Returns 0.0 (no sharing) to 1.0 (all paragraphs touch same fields).
fn compute_data_coupling(graph: &CodeGraph, prog_id: petgraph::graph::NodeIndex) -> f64 {
    let contained = graph.neighbors_outgoing(prog_id, EdgeKind::Contains);
    let paragraphs: Vec<_> = contained
        .iter()
        .filter(|id| {
            graph
                .node(**id)
                .is_some_and(|n| n.kind == NodeKind::Paragraph)
        })
        .copied()
        .collect();

    if paragraphs.len() <= 1 {
        return 0.0; // single paragraph can't have coupling
    }

    // Collect all fields touched by each paragraph
    let mut all_fields: std::collections::HashSet<petgraph::graph::NodeIndex> =
        std::collections::HashSet::new();
    let mut total_accesses = 0usize;

    for para_id in &paragraphs {
        let reads = graph.neighbors_outgoing(*para_id, EdgeKind::Reads);
        let writes = graph.neighbors_outgoing(*para_id, EdgeKind::Writes);
        for f in reads.iter().chain(writes.iter()) {
            all_fields.insert(*f);
            total_accesses += 1;
        }
    }

    if all_fields.is_empty() {
        return 0.0;
    }

    // Coupling = total_accesses / (paragraphs * unique_fields)
    // If every paragraph touches every field, this approaches 1.0
    // If each paragraph touches unique fields, this approaches 1/paragraphs
    let max_accesses = paragraphs.len() * all_fields.len();
    (total_accesses as f64 / max_accesses as f64 * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    fn make_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        let prog = g.add_node(Node::new(NodeKind::Program, "PROG1"));
        let p1 = g.add_node(Node::new(NodeKind::Paragraph, "1000-INIT").with_program("PROG1"));
        let p2 =
            g.add_node(Node::new(NodeKind::Paragraph, "2000-PROCESS").with_program("PROG1"));

        // Fields
        let f_input = g.add_node(Node::new(NodeKind::Field, "WS-INPUT").with_program("PROG1"));
        let f_output = g.add_node(Node::new(NodeKind::Field, "WS-OUTPUT").with_program("PROG1"));
        let f_counter =
            g.add_node(Node::new(NodeKind::Field, "WS-COUNTER").with_program("PROG1"));
        let f_temp = g.add_node(Node::new(NodeKind::Field, "WS-TEMP").with_program("PROG1"));
        let _f_unused = g.add_node(Node::new(NodeKind::Field, "WS-UNUSED").with_program("PROG1"));

        g.add_edge(prog, p1, Edge::new(EdgeKind::Contains));
        g.add_edge(prog, p2, Edge::new(EdgeKind::Contains));

        // f_input: read by both paragraphs, never written -> input
        g.add_edge(p1, f_input, Edge::new(EdgeKind::Reads));
        g.add_edge(p2, f_input, Edge::new(EdgeKind::Reads));

        // f_output: written by p2, never read -> output
        g.add_edge(p2, f_output, Edge::new(EdgeKind::Writes));

        // f_counter: written 2x, read 1x -> accumulator
        g.add_edge(p1, f_counter, Edge::new(EdgeKind::Writes));
        g.add_edge(p2, f_counter, Edge::new(EdgeKind::Writes));
        g.add_edge(p2, f_counter, Edge::new(EdgeKind::Reads));

        // f_temp: read and written equally -> temp
        g.add_edge(p1, f_temp, Edge::new(EdgeKind::Reads));
        g.add_edge(p1, f_temp, Edge::new(EdgeKind::Writes));

        // f_unused: no edges -> unused

        g
    }

    #[test]
    fn field_access_counts() {
        let mut g = make_graph();
        DataFlowPass.enrich(&mut g).unwrap();

        let f = g.lookup_one(NodeKind::Field, "WS-INPUT").unwrap();
        let node = g.node(f).unwrap();
        assert_eq!(node.properties.get_u64("read_count"), Some(2));
        assert_eq!(node.properties.get_u64("write_count"), Some(0));
        assert_eq!(node.properties.get_str("access_mode"), Some("read-only"));
    }

    #[test]
    fn field_role_classification() {
        let mut g = make_graph();
        DataFlowPass.enrich(&mut g).unwrap();

        let check = |name: &str, expected_role: &str| {
            let id = g.lookup_one(NodeKind::Field, name).unwrap();
            let role = g.node(id).unwrap().properties.get_str("field_role").unwrap();
            assert_eq!(role, expected_role, "field {name} expected role={expected_role}, got={role}");
        };

        check("WS-INPUT", "input");
        check("WS-OUTPUT", "output");
        check("WS-COUNTER", "accumulator");
        check("WS-TEMP", "temp");
        check("WS-UNUSED", "unused");
    }

    #[test]
    fn paragraph_field_access_counts() {
        let mut g = make_graph();
        DataFlowPass.enrich(&mut g).unwrap();

        let p1 = g.lookup_one(NodeKind::Paragraph, "1000-INIT").unwrap();
        let node = g.node(p1).unwrap();
        // p1 reads: WS-INPUT, WS-TEMP = 2
        assert_eq!(node.properties.get_u64("reads_count"), Some(2));
        // p1 writes: WS-COUNTER, WS-TEMP = 2
        assert_eq!(node.properties.get_u64("writes_count"), Some(2));
    }

    #[test]
    fn data_coupling_score() {
        let mut g = make_graph();
        DataFlowPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "PROG1").unwrap();
        let coupling = g.node(prog).unwrap().properties.get_f64("data_coupling").unwrap();
        // 2 paragraphs, 4 unique accessed fields, multiple accesses
        // coupling should be > 0 (fields are shared)
        assert!(coupling > 0.0, "coupling should be > 0, got {coupling}");
        assert!(coupling <= 1.0, "coupling should be <= 1, got {coupling}");
    }

    #[test]
    fn data_coupling_single_paragraph() {
        let mut g = CodeGraph::new();
        let prog = g.add_node(Node::new(NodeKind::Program, "SIMPLE"));
        let p1 = g.add_node(Node::new(NodeKind::Paragraph, "MAIN").with_program("SIMPLE"));
        let f1 = g.add_node(Node::new(NodeKind::Field, "F1").with_program("SIMPLE"));
        g.add_edge(prog, p1, Edge::new(EdgeKind::Contains));
        g.add_edge(p1, f1, Edge::new(EdgeKind::Reads));

        DataFlowPass.enrich(&mut g).unwrap();

        let prog_id = g.lookup_one(NodeKind::Program, "SIMPLE").unwrap();
        assert_eq!(
            g.node(prog_id).unwrap().properties.get_f64("data_coupling"),
            Some(0.0)
        );
    }
}
