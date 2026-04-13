use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// Layer 6: Business Logic Intelligence.
///
/// Computes per-program:
/// - `rule_count`: number of Rule nodes connected to this program's paragraphs
/// - `business_density`: rules per paragraph (higher = more business logic)
/// - `has_rules`: whether any business rules were extracted
///
/// Computes per-paragraph:
/// - `rule_count`: number of Rule nodes this paragraph produces
/// - `has_rules`: whether this paragraph contains business rules
///
/// Note: Rule nodes are created by the DSL emitter during rustification.
/// If no Rule nodes exist in the graph, this pass still runs but produces zeros.
#[derive(Debug)]
pub struct BusinessPass;

impl IntelligencePass for BusinessPass {
    fn name(&self) -> &'static str {
        "business"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        // Build a map: paragraph -> rule count
        // Rules are connected to paragraphs via properties or edges.
        // For now, we use a simple heuristic: Rule nodes have a `paragraph` property
        // that names the paragraph they came from. We also check for direct edges.
        let rule_ids = graph.all_of_kind(NodeKind::Rule);

        // Map paragraph name+program -> rule count
        let mut para_rule_counts: std::collections::HashMap<
            (String, String),
            usize,
        > = std::collections::HashMap::new();

        for rule_id in &rule_ids {
            if let Some(rule_node) = graph.node(*rule_id) {
                let para_name = rule_node
                    .properties
                    .get_str("paragraph")
                    .unwrap_or("")
                    .to_owned();
                let prog_name = rule_node.program.clone().unwrap_or_default();
                if !para_name.is_empty() {
                    *para_rule_counts
                        .entry((para_name, prog_name))
                        .or_default() += 1;
                }
            }
        }

        // Enrich paragraphs
        let para_ids = graph.all_of_kind(NodeKind::Paragraph);
        for para_id in &para_ids {
            if let Some(node) = graph.node(*para_id) {
                let para_name = node.name.clone();
                let prog_name = node.program.clone().unwrap_or_default();
                let count = para_rule_counts
                    .get(&(para_name, prog_name))
                    .copied()
                    .unwrap_or(0);

                if let Some(node_mut) = graph.node_mut(*para_id) {
                    node_mut
                        .properties
                        .set("rule_count", PropValue::from(count as u64));
                    node_mut
                        .properties
                        .set("has_rules", PropValue::from(count > 0));
                    stats.nodes_enriched += 1;
                    stats.properties_added += 2;
                }
            }
        }

        // Enrich programs
        let program_ids = graph.all_of_kind(NodeKind::Program);
        for prog_id in &program_ids {
            let contained = graph.neighbors_outgoing(*prog_id, EdgeKind::Contains);
            let para_count = contained
                .iter()
                .filter(|id| {
                    graph
                        .node(**id)
                        .is_some_and(|n| n.kind == NodeKind::Paragraph)
                })
                .count();

            // Sum rule counts from paragraphs
            let prog_name = graph.node(*prog_id).map(|n| n.name.clone()).unwrap_or_default();
            let total_rules: usize = graph
                .nodes_in_program(&prog_name)
                .iter()
                .filter_map(|id| {
                    let node = graph.node(*id)?;
                    if node.kind == NodeKind::Paragraph {
                        node.properties.get_u64("rule_count").map(|n| n as usize)
                    } else {
                        None
                    }
                })
                .sum();

            let business_density = if para_count > 0 {
                (total_rules as f64 / para_count as f64 * 10.0).round() / 10.0
            } else {
                0.0
            };

            if let Some(node) = graph.node_mut(*prog_id) {
                node.properties
                    .set("rule_count", PropValue::from(total_rules as u64));
                node.properties
                    .set("business_density", PropValue::from(business_density));
                node.properties
                    .set("has_rules", PropValue::from(total_rules > 0));
                stats.nodes_enriched += 1;
                stats.properties_added += 3;
            }
        }

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    fn make_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        let prog = g.add_node(Node::new(NodeKind::Program, "CLRG0100"));
        let p1 = g.add_node(Node::new(NodeKind::Paragraph, "1000-MAIN").with_program("CLRG0100"));
        let p2 = g.add_node(
            Node::new(NodeKind::Paragraph, "2000-VALIDATE").with_program("CLRG0100"),
        );
        let p3 = g.add_node(
            Node::new(NodeKind::Paragraph, "3000-UPDATE").with_program("CLRG0100"),
        );

        g.add_edge(prog, p1, Edge::new(EdgeKind::Contains));
        g.add_edge(prog, p2, Edge::new(EdgeKind::Contains));
        g.add_edge(prog, p3, Edge::new(EdgeKind::Contains));

        // Rules: 2 rules from 2000-VALIDATE, 1 from 3000-UPDATE, 0 from 1000-MAIN
        g.add_node(
            Node::new(NodeKind::Rule, "RULE-ACCT-ACTIVE")
                .with_program("CLRG0100")
                .with_property("paragraph", PropValue::from("2000-VALIDATE"))
                .with_property("type", PropValue::from("validation")),
        );
        g.add_node(
            Node::new(NodeKind::Rule, "RULE-BAL-POSITIVE")
                .with_program("CLRG0100")
                .with_property("paragraph", PropValue::from("2000-VALIDATE"))
                .with_property("type", PropValue::from("validation")),
        );
        g.add_node(
            Node::new(NodeKind::Rule, "RULE-UPDATE-BAL")
                .with_program("CLRG0100")
                .with_property("paragraph", PropValue::from("3000-UPDATE"))
                .with_property("type", PropValue::from("calculation")),
        );

        g
    }

    #[test]
    fn paragraph_rule_counts() {
        let mut g = make_graph();
        BusinessPass.enrich(&mut g).unwrap();

        let p1 = g.lookup_one(NodeKind::Paragraph, "1000-MAIN").unwrap();
        assert_eq!(g.node(p1).unwrap().properties.get_u64("rule_count"), Some(0));
        assert_eq!(g.node(p1).unwrap().properties.get_bool("has_rules"), Some(false));

        let p2 = g.lookup_one(NodeKind::Paragraph, "2000-VALIDATE").unwrap();
        assert_eq!(g.node(p2).unwrap().properties.get_u64("rule_count"), Some(2));
        assert_eq!(g.node(p2).unwrap().properties.get_bool("has_rules"), Some(true));

        let p3 = g.lookup_one(NodeKind::Paragraph, "3000-UPDATE").unwrap();
        assert_eq!(g.node(p3).unwrap().properties.get_u64("rule_count"), Some(1));
    }

    #[test]
    fn program_rule_totals() {
        let mut g = make_graph();
        BusinessPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let node = g.node(prog).unwrap();
        assert_eq!(node.properties.get_u64("rule_count"), Some(3));
        assert_eq!(node.properties.get_bool("has_rules"), Some(true));
    }

    #[test]
    fn business_density() {
        let mut g = make_graph();
        BusinessPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let density = g.node(prog).unwrap().properties.get_f64("business_density").unwrap();
        // 3 rules / 3 paragraphs = 1.0
        assert_eq!(density, 1.0);
    }

    #[test]
    fn program_no_rules() {
        let mut g = CodeGraph::new();
        let _prog = g.add_node(Node::new(NodeKind::Program, "EMPTY"));
        let p1 = g.add_node(Node::new(NodeKind::Paragraph, "MAIN").with_program("EMPTY"));
        g.add_edge(_prog, p1, Edge::new(EdgeKind::Contains));

        BusinessPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "EMPTY").unwrap();
        assert_eq!(g.node(prog).unwrap().properties.get_u64("rule_count"), Some(0));
        assert_eq!(g.node(prog).unwrap().properties.get_bool("has_rules"), Some(false));
        assert_eq!(
            g.node(prog).unwrap().properties.get_f64("business_density"),
            Some(0.0)
        );
    }
}
