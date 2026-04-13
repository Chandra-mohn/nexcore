use std::collections::HashSet;

use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// Layer 7: Dependency & Impact Intelligence.
///
/// Computes per-program:
/// - `impact_radius`: number of programs transitively affected if this program changes
///   (reverse call graph + copybook sharing propagation)
/// - `risk_score`: composite risk = complexity * coupling_factor * interface_weight (0-10)
/// - `migration_wave`: topological wave for migration sequencing
///   (wave 0 = leaf programs with no dependencies, wave 1 = depends only on wave 0, etc.)
/// - `dependency_count`: number of programs this one transitively depends on
#[derive(Debug)]
pub struct DependencyPass;

impl IntelligencePass for DependencyPass {
    fn name(&self) -> &'static str {
        "dependency"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        let program_ids = graph.all_of_kind(NodeKind::Program);

        // Precompute transitive dependency counts and impact radii
        let mut dep_counts: Vec<(petgraph::graph::NodeIndex, usize)> = Vec::new();
        let mut impact_radii: Vec<(petgraph::graph::NodeIndex, usize)> = Vec::new();

        for prog_id in &program_ids {
            // Forward: transitive dependencies (programs I call, transitively)
            let deps = transitive_neighbors(graph, *prog_id, EdgeKind::Calls, true);
            dep_counts.push((*prog_id, deps.len()));

            // Reverse: impact radius (programs that call me, transitively)
            let mut impacted = transitive_neighbors(graph, *prog_id, EdgeKind::Calls, false);

            // Also add programs sharing copybooks with me
            let my_copybooks = graph.neighbors_outgoing(*prog_id, EdgeKind::Uses);
            for cb_id in &my_copybooks {
                let cb_users = graph.neighbors_incoming(*cb_id, EdgeKind::Uses);
                for user_id in &cb_users {
                    if user_id != prog_id {
                        impacted.insert(*user_id);
                    }
                }
            }

            impact_radii.push((*prog_id, impacted.len()));
        }

        // Compute migration waves via topological layering
        let waves = compute_migration_waves(graph, &program_ids);

        // Compute risk scores and apply all properties
        for prog_id in &program_ids {
            let dep_count = dep_counts
                .iter()
                .find(|(id, _)| id == prog_id)
                .map_or(0, |(_, c)| *c);

            let impact = impact_radii
                .iter()
                .find(|(id, _)| id == prog_id)
                .map_or(0, |(_, c)| *c);

            let wave = waves
                .iter()
                .find(|(id, _)| id == prog_id)
                .map_or(0, |(_, w)| *w);

            let risk = compute_risk_score(graph, *prog_id, impact);

            if let Some(node) = graph.node_mut(*prog_id) {
                node.properties
                    .set("impact_radius", PropValue::from(impact as u64));
                node.properties
                    .set("dependency_count", PropValue::from(dep_count as u64));
                node.properties
                    .set("risk_score", PropValue::from(risk));
                node.properties
                    .set("migration_wave", PropValue::from(wave));
                stats.nodes_enriched += 1;
                stats.properties_added += 4;
            }
        }

        Ok(stats)
    }
}

/// BFS to find all transitively connected nodes along a given edge kind.
fn transitive_neighbors(
    graph: &CodeGraph,
    start: petgraph::graph::NodeIndex,
    edge_kind: EdgeKind,
    forward: bool,
) -> HashSet<petgraph::graph::NodeIndex> {
    let mut visited = HashSet::new();
    let mut queue = vec![start];

    while let Some(current) = queue.pop() {
        let neighbors = if forward {
            graph.neighbors_outgoing(current, edge_kind)
        } else {
            graph.neighbors_incoming(current, edge_kind)
        };
        for n in neighbors {
            if visited.insert(n) {
                queue.push(n);
            }
        }
    }

    visited
}

/// Assign migration waves via iterative peeling.
/// Wave 0: programs with no outgoing CALL edges (leaf nodes).
/// Wave N: programs whose callees are all in wave < N.
fn compute_migration_waves(
    graph: &CodeGraph,
    program_ids: &[petgraph::graph::NodeIndex],
) -> Vec<(petgraph::graph::NodeIndex, u64)> {
    let mut waves: Vec<(petgraph::graph::NodeIndex, u64)> = Vec::new();
    let mut assigned: HashSet<petgraph::graph::NodeIndex> = HashSet::new();
    let mut current_wave = 0u64;
    let total = program_ids.len();

    loop {
        let mut wave_members = Vec::new();

        for prog_id in program_ids {
            if assigned.contains(prog_id) {
                continue;
            }

            let callees = graph.neighbors_outgoing(*prog_id, EdgeKind::Calls);
            let all_deps_assigned = callees.iter().all(|c| assigned.contains(c));

            if all_deps_assigned {
                wave_members.push(*prog_id);
            }
        }

        if wave_members.is_empty() {
            // Remaining programs have circular dependencies -- assign to current wave
            for prog_id in program_ids {
                if !assigned.contains(prog_id) {
                    waves.push((*prog_id, current_wave));
                    assigned.insert(*prog_id);
                }
            }
            break;
        }

        for member in &wave_members {
            waves.push((*member, current_wave));
            assigned.insert(*member);
        }

        if assigned.len() >= total {
            break;
        }

        current_wave += 1;
    }

    waves
}

/// Risk score: composite of complexity, coupling, and interface weight.
fn compute_risk_score(
    graph: &CodeGraph,
    prog_id: petgraph::graph::NodeIndex,
    impact_radius: usize,
) -> f64 {
    let node = match graph.node(prog_id) {
        Some(n) => n,
        None => return 0.0,
    };

    let complexity = node.properties.get_f64("complexity").unwrap_or(0.0);
    let interface_count = node.properties.get_u64("interface_count").unwrap_or(0) as f64;
    let shared_cbs = node
        .properties
        .get_u64("shared_copybook_count")
        .unwrap_or(0) as f64;

    // Normalize components to 0-1 range
    let complexity_norm = (complexity / 10.0).min(1.0);
    let interface_norm = (interface_count / 5.0).min(1.0);
    let coupling_norm = (shared_cbs / 5.0).min(1.0);
    let impact_norm = (impact_radius as f64 / 10.0).min(1.0);

    // Weighted combination
    let raw = complexity_norm * 0.3
        + interface_norm * 0.2
        + coupling_norm * 0.2
        + impact_norm * 0.3;

    (raw * 100.0).round() / 10.0 // 0.0 - 10.0 scale
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    fn make_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        // Call chain: MAIN -> MID -> LEAF
        // MAIN also calls UTIL
        // MAIN and MID share copybook CPYSHARE
        let main = g.add_node(
            Node::new(NodeKind::Program, "MAIN")
                .with_property("complexity", PropValue::from(5.0f64))
                .with_property("interface_count", PropValue::from(3u64))
                .with_property("shared_copybook_count", PropValue::from(1u64)),
        );
        let mid = g.add_node(
            Node::new(NodeKind::Program, "MID")
                .with_property("complexity", PropValue::from(3.0f64))
                .with_property("interface_count", PropValue::from(1u64))
                .with_property("shared_copybook_count", PropValue::from(1u64)),
        );
        let leaf = g.add_node(
            Node::new(NodeKind::Program, "LEAF")
                .with_property("complexity", PropValue::from(1.0f64))
                .with_property("interface_count", PropValue::from(0u64))
                .with_property("shared_copybook_count", PropValue::from(0u64)),
        );
        let util = g.add_node(
            Node::new(NodeKind::Program, "UTIL")
                .with_property("complexity", PropValue::from(2.0f64))
                .with_property("interface_count", PropValue::from(0u64))
                .with_property("shared_copybook_count", PropValue::from(0u64)),
        );

        let cpyshare = g.add_node(Node::new(NodeKind::Copybook, "CPYSHARE"));

        g.add_edge(main, mid, Edge::new(EdgeKind::Calls));
        g.add_edge(main, util, Edge::new(EdgeKind::Calls));
        g.add_edge(mid, leaf, Edge::new(EdgeKind::Calls));
        g.add_edge(main, cpyshare, Edge::new(EdgeKind::Uses));
        g.add_edge(mid, cpyshare, Edge::new(EdgeKind::Uses));

        g
    }

    #[test]
    fn dependency_count() {
        let mut g = make_graph();
        DependencyPass.enrich(&mut g).unwrap();

        // MAIN -> MID, UTIL, LEAF (transitive) = 3
        let main = g.lookup_one(NodeKind::Program, "MAIN").unwrap();
        assert_eq!(
            g.node(main).unwrap().properties.get_u64("dependency_count"),
            Some(3)
        );

        // MID -> LEAF = 1
        let mid = g.lookup_one(NodeKind::Program, "MID").unwrap();
        assert_eq!(
            g.node(mid).unwrap().properties.get_u64("dependency_count"),
            Some(1)
        );

        // LEAF -> nothing = 0
        let leaf = g.lookup_one(NodeKind::Program, "LEAF").unwrap();
        assert_eq!(
            g.node(leaf).unwrap().properties.get_u64("dependency_count"),
            Some(0)
        );
    }

    #[test]
    fn impact_radius() {
        let mut g = make_graph();
        DependencyPass.enrich(&mut g).unwrap();

        // LEAF: called by MID, transitively by MAIN = 2 callers
        let leaf = g.lookup_one(NodeKind::Program, "LEAF").unwrap();
        assert_eq!(
            g.node(leaf).unwrap().properties.get_u64("impact_radius"),
            Some(2)
        );

        // MID: called by MAIN + shares CPYSHARE with MAIN = MAIN already counted = 1
        let mid = g.lookup_one(NodeKind::Program, "MID").unwrap();
        assert_eq!(
            g.node(mid).unwrap().properties.get_u64("impact_radius"),
            Some(1)
        );

        // MAIN: nobody calls it, but shares CPYSHARE with MID = 1
        let main = g.lookup_one(NodeKind::Program, "MAIN").unwrap();
        assert_eq!(
            g.node(main).unwrap().properties.get_u64("impact_radius"),
            Some(1) // MID shares copybook
        );
    }

    #[test]
    fn migration_waves() {
        let mut g = make_graph();
        DependencyPass.enrich(&mut g).unwrap();

        // LEAF and UTIL: no deps -> wave 0
        let leaf = g.lookup_one(NodeKind::Program, "LEAF").unwrap();
        assert_eq!(
            g.node(leaf).unwrap().properties.get_u64("migration_wave"),
            Some(0)
        );
        let util = g.lookup_one(NodeKind::Program, "UTIL").unwrap();
        assert_eq!(
            g.node(util).unwrap().properties.get_u64("migration_wave"),
            Some(0)
        );

        // MID: depends on LEAF (wave 0) -> wave 1
        let mid = g.lookup_one(NodeKind::Program, "MID").unwrap();
        assert_eq!(
            g.node(mid).unwrap().properties.get_u64("migration_wave"),
            Some(1)
        );

        // MAIN: depends on MID (wave 1) and UTIL (wave 0) -> wave 2
        let main = g.lookup_one(NodeKind::Program, "MAIN").unwrap();
        assert_eq!(
            g.node(main).unwrap().properties.get_u64("migration_wave"),
            Some(2)
        );
    }

    #[test]
    fn risk_scoring() {
        let mut g = make_graph();
        DependencyPass.enrich(&mut g).unwrap();

        let main = g.lookup_one(NodeKind::Program, "MAIN").unwrap();
        let risk = g.node(main).unwrap().properties.get_f64("risk_score").unwrap();
        assert!(risk > 0.0, "risk should be > 0, got {risk}");
        assert!(risk <= 10.0, "risk should be <= 10, got {risk}");

        // LEAF should have lower risk than MAIN
        let leaf = g.lookup_one(NodeKind::Program, "LEAF").unwrap();
        let leaf_risk = g.node(leaf).unwrap().properties.get_f64("risk_score").unwrap();
        assert!(leaf_risk < risk, "LEAF risk ({leaf_risk}) should be < MAIN risk ({risk})");
    }

    #[test]
    fn circular_dependency_waves() {
        let mut g = CodeGraph::new();
        let a = g.add_node(Node::new(NodeKind::Program, "A"));
        let b = g.add_node(Node::new(NodeKind::Program, "B"));
        g.add_edge(a, b, Edge::new(EdgeKind::Calls));
        g.add_edge(b, a, Edge::new(EdgeKind::Calls));

        DependencyPass.enrich(&mut g).unwrap();

        // Circular deps: both assigned to same wave
        let a_wave = g
            .node(g.lookup_one(NodeKind::Program, "A").unwrap())
            .unwrap()
            .properties
            .get_u64("migration_wave")
            .unwrap();
        let b_wave = g
            .node(g.lookup_one(NodeKind::Program, "B").unwrap())
            .unwrap()
            .properties
            .get_u64("migration_wave")
            .unwrap();
        assert_eq!(a_wave, b_wave);
    }
}
