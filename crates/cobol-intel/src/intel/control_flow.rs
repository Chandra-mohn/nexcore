use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// Layer 2: Control Flow Intelligence.
///
/// Computes per-paragraph:
/// - `is_entry_point`: true if no incoming PERFORM edges (first paragraph executed)
/// - `is_dead`: true if unreachable (no incoming PERFORMs and not an entry point)
/// - `perform_count`: number of outgoing PERFORM edges
/// - `performed_by_count`: number of incoming PERFORM edges
///
/// Computes per-program:
/// - `entry_points`: list of entry point paragraph names
/// - `dead_paragraph_count`: count of unreachable paragraphs
/// - `max_call_depth`: maximum depth of the CALL chain from this program
/// - `is_entry_program`: true if no incoming CALL edges (top-level)
#[derive(Debug)]
pub struct ControlFlowPass;

impl IntelligencePass for ControlFlowPass {
    fn name(&self) -> &'static str {
        "control_flow"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        // Enrich paragraphs
        let para_ids = graph.all_of_kind(NodeKind::Paragraph);
        for para_id in &para_ids {
            let performs_out = graph.neighbors_outgoing(*para_id, EdgeKind::Performs);
            let performed_by = graph.neighbors_incoming(*para_id, EdgeKind::Performs);

            let is_entry = performed_by.is_empty();
            let perform_count = performs_out.len();
            let performed_by_count = performed_by.len();

            if let Some(node) = graph.node_mut(*para_id) {
                node.properties
                    .set("is_entry_point", PropValue::from(is_entry));
                node.properties
                    .set("perform_count", PropValue::from(perform_count as u64));
                node.properties
                    .set("performed_by_count", PropValue::from(performed_by_count as u64));
                stats.nodes_enriched += 1;
                stats.properties_added += 3;
            }
        }

        // Enrich programs
        let program_ids = graph.all_of_kind(NodeKind::Program);
        for prog_id in &program_ids {
            let callers = graph.neighbors_incoming(*prog_id, EdgeKind::Calls);
            let is_entry_program = callers.is_empty();

            // Find entry point paragraphs within this program
            let contained = graph.neighbors_outgoing(*prog_id, EdgeKind::Contains);
            let mut entry_points = Vec::new();
            let mut dead_count = 0u64;

            for child_id in &contained {
                if let Some(child) = graph.node(*child_id) {
                    if child.kind == NodeKind::Paragraph {
                        let performed_by =
                            graph.neighbors_incoming(*child_id, EdgeKind::Performs);
                        if performed_by.is_empty() {
                            entry_points.push(child.name.clone());
                        }
                    }
                }
            }

            // Dead paragraphs: a paragraph is "dead" if:
            // 1. No incoming PERFORM edges (nobody calls it)
            // 2. It also doesn't PERFORM anything else (not the entry paragraph)
            // The entry paragraph typically has outgoing PERFORMs but no incoming ones.
            // A truly dead paragraph has neither incoming nor outgoing PERFORMs.
            let para_ids_in_program: Vec<_> = contained
                .iter()
                .filter(|id| {
                    graph
                        .node(**id)
                        .is_some_and(|n| n.kind == NodeKind::Paragraph)
                })
                .copied()
                .collect();

            let mut dead_para_ids = Vec::new();
            for child_id in &para_ids_in_program {
                let performed_by =
                    graph.neighbors_incoming(*child_id, EdgeKind::Performs);
                let performs_others =
                    graph.neighbors_outgoing(*child_id, EdgeKind::Performs);

                // Dead = no one calls it AND it doesn't call anyone
                // (an entry paragraph calls others but isn't called itself)
                if performed_by.is_empty() && performs_others.is_empty() {
                    dead_count += 1;
                    dead_para_ids.push(*child_id);
                }
            }

            // Mark dead paragraphs in a separate pass
            for dead_id in &dead_para_ids {
                if let Some(para_node) = graph.node_mut(*dead_id) {
                    para_node
                        .properties
                        .set("is_dead", PropValue::from(true));
                }
            }

            // Compute max call depth via BFS
            let max_depth = compute_call_depth(graph, *prog_id);

            if let Some(node) = graph.node_mut(*prog_id) {
                node.properties
                    .set("is_entry_program", PropValue::from(is_entry_program));
                node.properties
                    .set("entry_points", PropValue::from(entry_points));
                node.properties
                    .set("dead_paragraph_count", PropValue::from(dead_count));
                node.properties
                    .set("max_call_depth", PropValue::from(max_depth));
                stats.nodes_enriched += 1;
                stats.properties_added += 4;
            }
        }

        Ok(stats)
    }
}

/// BFS call depth from a program node. Returns the longest chain length.
fn compute_call_depth(graph: &CodeGraph, start: petgraph::graph::NodeIndex) -> u64 {
    let mut max_depth = 0u64;
    let mut queue: Vec<(petgraph::graph::NodeIndex, u64)> = vec![(start, 0)];
    let mut visited = vec![start];

    while let Some((current, depth)) = queue.pop() {
        if depth > max_depth {
            max_depth = depth;
        }
        let callees = graph.neighbors_outgoing(current, EdgeKind::Calls);
        for callee in callees {
            if !visited.contains(&callee) {
                visited.push(callee);
                queue.push((callee, depth + 1));
            }
        }
    }

    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    fn make_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        // CLRG0100 calls VALUTIL calls LOGUTIL (depth 2)
        let clrg = g.add_node(Node::new(NodeKind::Program, "CLRG0100"));
        let valutil = g.add_node(Node::new(NodeKind::Program, "VALUTIL"));
        let logutil = g.add_node(Node::new(NodeKind::Program, "LOGUTIL"));

        // Paragraphs in CLRG0100
        let p1 = g.add_node(Node::new(NodeKind::Paragraph, "1000-MAIN").with_program("CLRG0100"));
        let p2 = g.add_node(
            Node::new(NodeKind::Paragraph, "2000-VALIDATE").with_program("CLRG0100"),
        );
        let p3 = g.add_node(
            Node::new(NodeKind::Paragraph, "3000-DEAD").with_program("CLRG0100"),
        );

        // Edges
        g.add_edge(clrg, valutil, Edge::new(EdgeKind::Calls));
        g.add_edge(valutil, logutil, Edge::new(EdgeKind::Calls));
        g.add_edge(clrg, p1, Edge::new(EdgeKind::Contains));
        g.add_edge(clrg, p2, Edge::new(EdgeKind::Contains));
        g.add_edge(clrg, p3, Edge::new(EdgeKind::Contains));
        // 1000-MAIN performs 2000-VALIDATE, but NOT 3000-DEAD
        g.add_edge(p1, p2, Edge::new(EdgeKind::Performs));

        g
    }

    #[test]
    fn paragraph_entry_points() {
        let mut g = make_graph();
        ControlFlowPass.enrich(&mut g).unwrap();

        let p1 = g.lookup_one(NodeKind::Paragraph, "1000-MAIN").unwrap();
        assert_eq!(
            g.node(p1).unwrap().properties.get_bool("is_entry_point"),
            Some(true)
        );

        let p2 = g
            .lookup_one(NodeKind::Paragraph, "2000-VALIDATE")
            .unwrap();
        assert_eq!(
            g.node(p2).unwrap().properties.get_bool("is_entry_point"),
            Some(false)
        );
    }

    #[test]
    fn paragraph_perform_counts() {
        let mut g = make_graph();
        ControlFlowPass.enrich(&mut g).unwrap();

        let p1 = g.lookup_one(NodeKind::Paragraph, "1000-MAIN").unwrap();
        assert_eq!(
            g.node(p1).unwrap().properties.get_u64("perform_count"),
            Some(1) // performs 2000-VALIDATE
        );
        assert_eq!(
            g.node(p1)
                .unwrap()
                .properties
                .get_u64("performed_by_count"),
            Some(0)
        );
    }

    #[test]
    fn dead_paragraph_detection() {
        let mut g = make_graph();
        ControlFlowPass.enrich(&mut g).unwrap();

        // 3000-DEAD: no incoming performs, doesn't perform anything -> dead
        let p3 = g.lookup_one(NodeKind::Paragraph, "3000-DEAD").unwrap();
        assert_eq!(
            g.node(p3).unwrap().properties.get_bool("is_dead"),
            Some(true)
        );

        // Program-level dead count
        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        assert_eq!(
            g.node(clrg)
                .unwrap()
                .properties
                .get_u64("dead_paragraph_count"),
            Some(1)
        );
    }

    #[test]
    fn entry_program_detection() {
        let mut g = make_graph();
        ControlFlowPass.enrich(&mut g).unwrap();

        // CLRG0100: not called by anyone -> entry program
        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        assert_eq!(
            g.node(clrg)
                .unwrap()
                .properties
                .get_bool("is_entry_program"),
            Some(true)
        );

        // VALUTIL: called by CLRG0100 -> not entry
        let valutil = g.lookup_one(NodeKind::Program, "VALUTIL").unwrap();
        assert_eq!(
            g.node(valutil)
                .unwrap()
                .properties
                .get_bool("is_entry_program"),
            Some(false)
        );
    }

    #[test]
    fn call_depth() {
        let mut g = make_graph();
        ControlFlowPass.enrich(&mut g).unwrap();

        // CLRG0100 -> VALUTIL -> LOGUTIL = depth 2
        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        assert_eq!(
            g.node(clrg)
                .unwrap()
                .properties
                .get_u64("max_call_depth"),
            Some(2)
        );

        // VALUTIL -> LOGUTIL = depth 1
        let valutil = g.lookup_one(NodeKind::Program, "VALUTIL").unwrap();
        assert_eq!(
            g.node(valutil)
                .unwrap()
                .properties
                .get_u64("max_call_depth"),
            Some(1)
        );

        // LOGUTIL -> nothing = depth 0
        let logutil = g.lookup_one(NodeKind::Program, "LOGUTIL").unwrap();
        assert_eq!(
            g.node(logutil)
                .unwrap()
                .properties
                .get_u64("max_call_depth"),
            Some(0)
        );
    }

    #[test]
    fn run_all_passes() {
        let mut g = make_graph();
        let results = crate::intel::run_all(&mut g).unwrap();
        assert_eq!(results.len(), 11);
        assert_eq!(results[0].0, "structural");
        assert_eq!(results[1].0, "control_flow");
        assert_eq!(results[2].0, "data_flow");
        assert_eq!(results[3].0, "inter_program");
        assert_eq!(results[4].0, "external");
        assert_eq!(results[5].0, "business");
        assert_eq!(results[6].0, "dependency");
        assert_eq!(results[7].0, "patterns");
        assert_eq!(results[8].0, "process_discovery");
        assert_eq!(results[9].0, "data_complexity");
        assert_eq!(results[10].0, "cost_estimate");
        assert!(results[0].1.nodes_enriched > 0);
        assert!(results[1].1.nodes_enriched > 0);
    }
}
