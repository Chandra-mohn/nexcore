use std::collections::{HashMap, HashSet};

use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// Layer 4: Inter-Program Intelligence.
///
/// Computes per-program:
/// - `copybook_coupling`: list of programs sharing at least one copybook
/// - `shared_copybook_count`: number of copybooks shared with other programs
/// - `naming_prefix`: first 4 chars of program name (cluster hint)
/// - `cluster`: programs with same naming prefix form a cluster
///
/// Computes per-copybook:
/// - `user_count`: number of programs using this copybook
/// - `is_shared`: true if used by more than one program
#[derive(Debug)]
pub struct InterProgramPass;

impl IntelligencePass for InterProgramPass {
    fn name(&self) -> &'static str {
        "inter_program"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        // --- Copybook enrichment ---
        let copybook_ids = graph.all_of_kind(NodeKind::Copybook);
        // Build copybook -> users map
        let mut copybook_users: HashMap<petgraph::graph::NodeIndex, Vec<petgraph::graph::NodeIndex>> =
            HashMap::new();

        for cb_id in &copybook_ids {
            let users = graph.neighbors_incoming(*cb_id, EdgeKind::Uses);
            let user_count = users.len();
            let is_shared = user_count > 1;

            copybook_users.insert(*cb_id, users);

            if let Some(node) = graph.node_mut(*cb_id) {
                node.properties
                    .set("user_count", PropValue::from(user_count as u64));
                node.properties
                    .set("is_shared", PropValue::from(is_shared));
                stats.nodes_enriched += 1;
                stats.properties_added += 2;
            }
        }

        // --- Program enrichment ---
        let program_ids = graph.all_of_kind(NodeKind::Program);

        // Build program -> copybooks map
        let mut program_copybooks: HashMap<petgraph::graph::NodeIndex, Vec<petgraph::graph::NodeIndex>> =
            HashMap::new();
        for prog_id in &program_ids {
            let cbs = graph.neighbors_outgoing(*prog_id, EdgeKind::Uses);
            program_copybooks.insert(*prog_id, cbs);
        }

        // For each program, find other programs sharing copybooks
        let mut coupled_programs: HashMap<petgraph::graph::NodeIndex, Vec<String>> = HashMap::new();
        let mut shared_counts: HashMap<petgraph::graph::NodeIndex, usize> = HashMap::new();

        for prog_id in &program_ids {
            let my_cbs = program_copybooks.get(prog_id).cloned().unwrap_or_default();
            let mut partners: HashSet<String> = HashSet::new();
            let mut shared_count = 0usize;

            for cb_id in &my_cbs {
                if let Some(users) = copybook_users.get(cb_id) {
                    if users.len() > 1 {
                        shared_count += 1;
                        for user_id in users {
                            if user_id != prog_id {
                                if let Some(user_node) = graph.node(*user_id) {
                                    partners.insert(user_node.name.clone());
                                }
                            }
                        }
                    }
                }
            }

            let mut partner_list: Vec<String> = partners.into_iter().collect();
            partner_list.sort();
            coupled_programs.insert(*prog_id, partner_list);
            shared_counts.insert(*prog_id, shared_count);
        }

        // Apply properties
        for prog_id in &program_ids {
            let prog_name = graph.node(*prog_id).map(|n| n.name.clone()).unwrap_or_default();

            // Naming prefix (first 4 chars, uppercase)
            let prefix = if prog_name.len() >= 4 {
                prog_name[..4].to_uppercase()
            } else {
                prog_name.to_uppercase()
            };

            let partners = coupled_programs.remove(prog_id).unwrap_or_default();
            let shared_count = shared_counts.get(prog_id).copied().unwrap_or(0);

            if let Some(node) = graph.node_mut(*prog_id) {
                node.properties
                    .set("naming_prefix", PropValue::from(prefix.clone()));
                node.properties
                    .set("shared_copybook_count", PropValue::from(shared_count as u64));
                if !partners.is_empty() {
                    node.properties
                        .set("copybook_coupling", PropValue::from(partners));
                }
                stats.nodes_enriched += 1;
                stats.properties_added += 3;
            }
        }

        // Compute clusters by naming prefix
        let mut clusters: HashMap<String, Vec<String>> = HashMap::new();
        for prog_id in &program_ids {
            if let Some(node) = graph.node(*prog_id) {
                let prefix = node
                    .properties
                    .get_str("naming_prefix")
                    .unwrap_or("")
                    .to_owned();
                clusters
                    .entry(prefix)
                    .or_default()
                    .push(node.name.clone());
            }
        }

        // Set cluster property on programs (only for clusters with 2+ members)
        for prog_id in &program_ids {
            if let Some(node) = graph.node(*prog_id) {
                let prefix = node
                    .properties
                    .get_str("naming_prefix")
                    .unwrap_or("")
                    .to_owned();
                if let Some(members) = clusters.get(&prefix) {
                    if members.len() > 1 {
                        let cluster_name = prefix.clone();
                        if let Some(node_mut) = graph.node_mut(*prog_id) {
                            node_mut
                                .properties
                                .set("cluster", PropValue::from(cluster_name));
                            stats.properties_added += 1;
                        }
                    }
                }
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

        // CLRG0100 and CLRG0200 share CPYACCT and CPYCLRG
        // LNOR0100 uses CPYACCT (but not CPYCLRG)
        // ORPHAN has no copybooks
        let clrg1 = g.add_node(Node::new(NodeKind::Program, "CLRG0100"));
        let clrg2 = g.add_node(Node::new(NodeKind::Program, "CLRG0200"));
        let lnor = g.add_node(Node::new(NodeKind::Program, "LNOR0100"));
        let _orphan = g.add_node(Node::new(NodeKind::Program, "ORPHAN"));

        let cpyclrg = g.add_node(Node::new(NodeKind::Copybook, "CPYCLRG"));
        let cpyacct = g.add_node(Node::new(NodeKind::Copybook, "CPYACCT"));
        let _cpylone = g.add_node(Node::new(NodeKind::Copybook, "CPYLONE"));

        g.add_edge(clrg1, cpyclrg, Edge::new(EdgeKind::Uses));
        g.add_edge(clrg1, cpyacct, Edge::new(EdgeKind::Uses));
        g.add_edge(clrg2, cpyclrg, Edge::new(EdgeKind::Uses));
        g.add_edge(clrg2, cpyacct, Edge::new(EdgeKind::Uses));
        g.add_edge(lnor, cpyacct, Edge::new(EdgeKind::Uses));
        // CPYLONE used by nobody

        g
    }

    #[test]
    fn copybook_user_count() {
        let mut g = make_graph();
        InterProgramPass.enrich(&mut g).unwrap();

        let cpyacct = g.lookup_one(NodeKind::Copybook, "CPYACCT").unwrap();
        assert_eq!(
            g.node(cpyacct).unwrap().properties.get_u64("user_count"),
            Some(3)
        );
        assert_eq!(
            g.node(cpyacct).unwrap().properties.get_bool("is_shared"),
            Some(true)
        );

        let cpylone = g.lookup_one(NodeKind::Copybook, "CPYLONE").unwrap();
        assert_eq!(
            g.node(cpylone).unwrap().properties.get_u64("user_count"),
            Some(0)
        );
        assert_eq!(
            g.node(cpylone).unwrap().properties.get_bool("is_shared"),
            Some(false)
        );
    }

    #[test]
    fn shared_copybook_count() {
        let mut g = make_graph();
        InterProgramPass.enrich(&mut g).unwrap();

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        // CLRG0100 uses CPYCLRG (shared with CLRG0200) and CPYACCT (shared with CLRG0200, LNOR0100)
        assert_eq!(
            g.node(clrg1)
                .unwrap()
                .properties
                .get_u64("shared_copybook_count"),
            Some(2)
        );
    }

    #[test]
    fn copybook_coupling_partners() {
        let mut g = make_graph();
        InterProgramPass.enrich(&mut g).unwrap();

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let node = g.node(clrg1).unwrap();
        // Should be coupled with CLRG0200 and LNOR0100
        if let Some(PropValue::List(partners)) = node.properties.get("copybook_coupling") {
            assert!(partners.contains(&"CLRG0200".to_owned()));
            assert!(partners.contains(&"LNOR0100".to_owned()));
            assert_eq!(partners.len(), 2);
        } else {
            panic!("expected copybook_coupling list");
        }
    }

    #[test]
    fn naming_prefix() {
        let mut g = make_graph();
        InterProgramPass.enrich(&mut g).unwrap();

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        assert_eq!(
            g.node(clrg1).unwrap().properties.get_str("naming_prefix"),
            Some("CLRG")
        );

        let lnor = g.lookup_one(NodeKind::Program, "LNOR0100").unwrap();
        assert_eq!(
            g.node(lnor).unwrap().properties.get_str("naming_prefix"),
            Some("LNOR")
        );
    }

    #[test]
    fn cluster_assignment() {
        let mut g = make_graph();
        InterProgramPass.enrich(&mut g).unwrap();

        // CLRG0100 and CLRG0200 share prefix "CLRG" -> cluster
        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        assert_eq!(
            g.node(clrg1).unwrap().properties.get_str("cluster"),
            Some("CLRG")
        );

        let clrg2 = g.lookup_one(NodeKind::Program, "CLRG0200").unwrap();
        assert_eq!(
            g.node(clrg2).unwrap().properties.get_str("cluster"),
            Some("CLRG")
        );

        // LNOR0100 is alone -> no cluster
        let lnor = g.lookup_one(NodeKind::Program, "LNOR0100").unwrap();
        assert_eq!(
            g.node(lnor).unwrap().properties.get_str("cluster"),
            None
        );

        // ORPHAN is alone -> no cluster
        let orphan = g.lookup_one(NodeKind::Program, "ORPHAN").unwrap();
        assert_eq!(
            g.node(orphan).unwrap().properties.get_str("cluster"),
            None
        );
    }

    #[test]
    fn orphan_program_no_coupling() {
        let mut g = make_graph();
        InterProgramPass.enrich(&mut g).unwrap();

        let orphan = g.lookup_one(NodeKind::Program, "ORPHAN").unwrap();
        assert_eq!(
            g.node(orphan)
                .unwrap()
                .properties
                .get_u64("shared_copybook_count"),
            Some(0)
        );
        assert_eq!(
            g.node(orphan).unwrap().properties.get("copybook_coupling"),
            None
        );
    }
}
