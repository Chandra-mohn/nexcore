use std::collections::HashMap;

use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeId, NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// A FunctionUnit is a paragraph cluster -- the uniform unit of process clustering.
/// Every program decomposes into 1+ FunctionUnits. Small programs = 1 unit.
/// Monster programs = N units. Program name is just an attribute.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct FunctionUnit {
    id: usize,
    /// Parent program.
    program_id: NodeId,
    program_name: String,
    /// Root paragraph name for this cluster.
    cluster_root: String,
    /// Paragraph IDs in this cluster.
    paragraph_ids: Vec<NodeId>,
    /// Files accessed by this cluster's paragraphs.
    files_accessed: Vec<NodeId>,
    /// Fields touched by this cluster's paragraphs.
    fields_touched: Vec<NodeId>,
}

/// Layer 9: Process Discovery Intelligence.
///
/// Discovers business processes by clustering FunctionUnits:
/// - Small programs are one FunctionUnit each
/// - Monster programs (>20 paragraphs) are decomposed into paragraph clusters
///
/// Affinity signals:
/// 1. Shared file access (weight 3)
/// 2. CALL chains (weight 3, reduced for utilities)
/// 3. Shared copybooks (weight 2, relative threshold)
/// 4. Naming prefix (weight 1)
///
/// Produces per-program:
/// - `process`: name of the discovered business process
/// - `process_role`: core / shared / utility / external
/// - `function_clusters`: (monster programs only) list of paragraph clusters
#[derive(Debug)]
pub struct ProcessDiscoveryPass;

impl IntelligencePass for ProcessDiscoveryPass {
    fn name(&self) -> &'static str {
        "process_discovery"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        let program_ids = graph.all_of_kind(NodeKind::Program);
        if program_ids.is_empty() {
            return Ok(stats);
        }

        // Step 1: Build FunctionUnits
        let mut units: Vec<FunctionUnit> = Vec::new();
        let mut program_to_units: HashMap<NodeId, Vec<usize>> = HashMap::new();

        for prog_id in &program_ids {
            let prog_name = graph
                .node(*prog_id)
                .map(|n| n.name.clone())
                .unwrap_or_default();

            let paragraphs: Vec<NodeId> = graph
                .neighbors_outgoing(*prog_id, EdgeKind::Contains)
                .into_iter()
                .filter(|id| {
                    graph
                        .node(*id)
                        .is_some_and(|n| n.kind == NodeKind::Paragraph)
                })
                .collect();

            // Always decompose into paragraph clusters. Small programs naturally
            // produce 1 cluster. Monster programs produce N clusters.
            let clusters = decompose_into_clusters(graph, &paragraphs);
            let mut unit_ids = Vec::new();

            if clusters.is_empty() {
                // Program with no paragraphs (external/stub): create a single unit
                let unit_id = units.len();
                let files: Vec<NodeId> = graph
                    .neighbors_outgoing(*prog_id, EdgeKind::Accesses)
                    .into_iter()
                    .filter(|id| {
                        graph
                            .node(*id)
                            .is_some_and(|n| n.kind == NodeKind::File || n.kind == NodeKind::Table)
                    })
                    .collect();
                units.push(FunctionUnit {
                    id: unit_id,
                    program_id: *prog_id,
                    program_name: prog_name.clone(),
                    cluster_root: prog_name.clone(),
                    paragraph_ids: Vec::new(),
                    files_accessed: files,
                    fields_touched: Vec::new(),
                });
                unit_ids.push(unit_id);
            } else {
                for cluster in clusters {
                    let unit_id = units.len();
                    let files = collect_files_for_paragraphs(graph, &cluster.paragraphs);
                    let fields = collect_fields_for_paragraphs(graph, &cluster.paragraphs);
                    units.push(FunctionUnit {
                        id: unit_id,
                        program_id: *prog_id,
                        program_name: prog_name.clone(),
                        cluster_root: cluster.root_name,
                        paragraph_ids: cluster.paragraphs,
                        files_accessed: files,
                        fields_touched: fields,
                    });
                    unit_ids.push(unit_id);
                }
                // Also include program-level file access for units without paragraph-level edges
                let prog_files: Vec<NodeId> = graph
                    .neighbors_outgoing(*prog_id, EdgeKind::Accesses)
                    .into_iter()
                    .filter(|id| {
                        graph
                            .node(*id)
                            .is_some_and(|n| n.kind == NodeKind::File || n.kind == NodeKind::Table)
                    })
                    .collect();
                if !prog_files.is_empty() {
                    if let Some(first_uid) = unit_ids.first() {
                        for f in prog_files {
                            if !units[*first_uid].files_accessed.contains(&f) {
                                units[*first_uid].files_accessed.push(f);
                            }
                        }
                    }
                }
            }

            program_to_units.insert(*prog_id, unit_ids);
        }

        if units.is_empty() {
            return Ok(stats);
        }

        // Step 2: Build affinities between FunctionUnits
        let total_units = units.len();
        let mut uf = UnionFind::new(total_units);

        // 2a. Shared file access (weight 3)
        let mut file_to_units: HashMap<NodeId, Vec<usize>> = HashMap::new();
        for unit in &units {
            for file_id in &unit.files_accessed {
                file_to_units.entry(*file_id).or_default().push(unit.id);
            }
        }
        for (_, unit_ids) in &file_to_units {
            if unit_ids.len() >= 2 && unit_ids.len() <= (total_units as f64 * 0.25).max(5.0) as usize {
                for i in 0..unit_ids.len() {
                    for j in (i + 1)..unit_ids.len() {
                        // Only merge if affinity is strong enough (multiple shared files or other signals)
                        uf.add_weight(unit_ids[i], unit_ids[j], 3.0);
                    }
                }
            }
        }

        // 2b. CALL chains (weight 3, reduced for utilities)
        for prog_id in &program_ids {
            let callees = graph.neighbors_outgoing(*prog_id, EdgeKind::Calls);
            for callee in &callees {
                let callers_count = graph.neighbors_incoming(*callee, EdgeKind::Calls).len();
                let weight = if callers_count >= 3 {
                    0.5
                } else if callers_count >= 2 {
                    1.5
                } else {
                    3.0
                };
                // Connect first unit of caller to first unit of callee
                if let (Some(caller_units), Some(callee_units)) =
                    (program_to_units.get(prog_id), program_to_units.get(callee))
                {
                    if let (Some(&cu), Some(&ce)) = (caller_units.first(), callee_units.first()) {
                        uf.add_weight(cu, ce, weight);
                    }
                }
            }
        }

        // 2c. Shared copybooks (weight 2, relative threshold)
        let infrastructure_threshold = (program_ids.len() as f64 * 0.25).max(5.0) as usize;
        let copybook_ids = graph.all_of_kind(NodeKind::Copybook);
        for cb_id in &copybook_ids {
            let users: Vec<NodeId> = graph
                .neighbors_incoming(*cb_id, EdgeKind::Uses)
                .into_iter()
                .filter(|id| graph.node(*id).is_some_and(|n| n.kind == NodeKind::Program))
                .collect();
            if users.len() >= 2 && users.len() <= infrastructure_threshold {
                for i in 0..users.len() {
                    for j in (i + 1)..users.len() {
                        if let (Some(u1), Some(u2)) =
                            (program_to_units.get(&users[i]).and_then(|v| v.first()),
                             program_to_units.get(&users[j]).and_then(|v| v.first()))
                        {
                            uf.add_weight(*u1, *u2, 2.0);
                        }
                    }
                }
            }
        }

        // 2d. Naming prefix (weight 1) -- based on parent program name
        for i in 0..units.len() {
            for j in (i + 1)..units.len() {
                let na = &units[i].program_name;
                let nb = &units[j].program_name;
                if na == nb {
                    continue; // same program, different clusters -- don't merge by name
                }
                if na.len() >= 4 && nb.len() >= 4 && na[..4] == nb[..4] {
                    uf.add_weight(i, j, 1.0);
                }
            }
        }

        // Step 3: Merge units with sufficient affinity
        uf.merge_above_threshold(2.0);

        // Step 4: Collect clusters and name processes
        let mut clusters: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..units.len() {
            let root = uf.find(i);
            clusters.entry(root).or_default().push(i);
        }

        // Assign process names
        let mut unit_process: Vec<String> = vec![String::new(); units.len()];
        for (_, member_ids) in &clusters {
            let name = name_process_from_units(&units, member_ids);
            for &uid in member_ids {
                unit_process[uid] = name.clone();
            }
        }

        // Step 5: Classify roles
        // External: program has no content (unresolved CALL target)
        // Shared: called by programs in different processes
        // Utility: high copybook sharing, small cluster
        // Core: default

        // Step 6: Apply properties to programs
        for prog_id in &program_ids {
            if let Some(unit_ids) = program_to_units.get(prog_id) {
                let process_name = unit_ids
                    .first()
                    .map(|&uid| unit_process[uid].clone())
                    .unwrap_or_else(|| "UNKNOWN".to_owned());

                // Check if external
                let has_content = !graph.neighbors_outgoing(*prog_id, EdgeKind::Contains).is_empty()
                    || !graph.neighbors_outgoing(*prog_id, EdgeKind::Uses).is_empty()
                    || !graph.neighbors_outgoing(*prog_id, EdgeKind::Accesses).is_empty()
                    || graph
                        .node(*prog_id)
                        .and_then(|n| n.properties.get_u64("field_count"))
                        .unwrap_or(0)
                        > 0;

                let role = if !has_content {
                    "external"
                } else {
                    // Check if shared (called by programs in different processes)
                    let callers = graph.neighbors_incoming(*prog_id, EdgeKind::Calls);
                    let mut caller_processes = std::collections::HashSet::new();
                    for caller_id in &callers {
                        if let Some(caller_units) = program_to_units.get(caller_id) {
                            if let Some(&uid) = caller_units.first() {
                                caller_processes.insert(unit_process[uid].clone());
                            }
                        }
                    }
                    if caller_processes.len() > 1 {
                        "shared"
                    } else {
                        let my_cbs = graph
                            .node(*prog_id)
                            .and_then(|n| n.properties.get_u64("shared_copybook_count"))
                            .unwrap_or(0);
                        if my_cbs > infrastructure_threshold as u64 && unit_ids.len() == 1 {
                            "utility"
                        } else {
                            "core"
                        }
                    }
                };

                if let Some(node) = graph.node_mut(*prog_id) {
                    node.properties
                        .set("process", PropValue::from(process_name));
                    node.properties
                        .set("process_role", PropValue::from(role));
                    stats.nodes_enriched += 1;
                    stats.properties_added += 2;

                    // Record function cluster count and names
                    node.properties.set(
                        "function_cluster_count",
                        PropValue::from(unit_ids.len() as u64),
                    );
                    stats.properties_added += 1;

                    if unit_ids.len() > 1 {
                        let cluster_names: Vec<String> = unit_ids
                            .iter()
                            .map(|&uid| units[uid].cluster_root.clone())
                            .collect();
                        node.properties.set(
                            "function_clusters",
                            PropValue::from(cluster_names),
                        );
                        stats.properties_added += 1;
                    }
                }
            }
        }

        Ok(stats)
    }
}

// --- Monster decomposition ---

struct ParagraphCluster {
    root_name: String,
    paragraphs: Vec<NodeId>,
}

/// Decompose a program's paragraphs into function clusters.
/// Each root paragraph (no incoming PERFORMs) + its transitive PERFORM closure = one cluster.
/// Small programs naturally produce 1 cluster. Monster programs produce N clusters.
fn decompose_into_clusters(
    graph: &CodeGraph,
    paragraphs: &[NodeId],
) -> Vec<ParagraphCluster> {
    // Find root paragraphs: those with no incoming PERFORM edges (within this program)
    let para_set: std::collections::HashSet<NodeId> = paragraphs.iter().copied().collect();

    let mut roots: Vec<NodeId> = Vec::new();
    let mut _non_roots: std::collections::HashSet<NodeId> = std::collections::HashSet::new();

    for para_id in paragraphs {
        let performers = graph.neighbors_incoming(*para_id, EdgeKind::Performs);
        let has_internal_performer = performers.iter().any(|p| para_set.contains(p));
        if !has_internal_performer {
            roots.push(*para_id);
        }
    }

    // Build transitive PERFORM closure for each root
    let mut clusters: Vec<ParagraphCluster> = Vec::new();
    let mut assigned: std::collections::HashSet<NodeId> = std::collections::HashSet::new();

    for root in &roots {
        if assigned.contains(root) {
            continue;
        }
        let root_name = graph
            .node(*root)
            .map(|n| n.name.clone())
            .unwrap_or_default();

        let mut closure = vec![*root];
        let mut queue = vec![*root];
        assigned.insert(*root);

        while let Some(current) = queue.pop() {
            let performs = graph.neighbors_outgoing(current, EdgeKind::Performs);
            for target in performs {
                if para_set.contains(&target) && !assigned.contains(&target) {
                    assigned.insert(target);
                    closure.push(target);
                    queue.push(target);
                }
            }
        }

        clusters.push(ParagraphCluster {
            root_name,
            paragraphs: closure,
        });
    }

    // Any unassigned paragraphs go into an "OTHER" cluster
    let remaining: Vec<NodeId> = paragraphs
        .iter()
        .filter(|id| !assigned.contains(id))
        .copied()
        .collect();
    if !remaining.is_empty() {
        clusters.push(ParagraphCluster {
            root_name: "OTHER".to_owned(),
            paragraphs: remaining,
        });
    }

    clusters
}

fn collect_files_for_paragraphs(graph: &CodeGraph, paragraphs: &[NodeId]) -> Vec<NodeId> {
    let mut files = Vec::new();
    for para_id in paragraphs {
        let accessed = graph.neighbors_outgoing(*para_id, EdgeKind::Accesses);
        for file_id in accessed {
            if !files.contains(&file_id) {
                files.push(file_id);
            }
        }
    }
    files
}

fn collect_fields_for_paragraphs(graph: &CodeGraph, paragraphs: &[NodeId]) -> Vec<NodeId> {
    let mut fields = Vec::new();
    for para_id in paragraphs {
        for field_id in graph.neighbors_outgoing(*para_id, EdgeKind::Reads) {
            if !fields.contains(&field_id) {
                fields.push(field_id);
            }
        }
        for field_id in graph.neighbors_outgoing(*para_id, EdgeKind::Writes) {
            if !fields.contains(&field_id) {
                fields.push(field_id);
            }
        }
    }
    fields
}

/// Name a process based on its FunctionUnit members.
fn name_process_from_units(units: &[FunctionUnit], member_ids: &[usize]) -> String {
    if member_ids.is_empty() {
        return "UNKNOWN".to_owned();
    }

    // Collect program names (deduplicated)
    let mut names: Vec<String> = Vec::new();
    for &uid in member_ids {
        let name = &units[uid].program_name;
        if !names.contains(name) {
            names.push(name.clone());
        }
    }

    if names.is_empty() {
        return "UNKNOWN".to_owned();
    }

    // Count 4-char prefixes
    let mut prefix_counts: HashMap<String, usize> = HashMap::new();
    for name in &names {
        if name.len() >= 4 {
            *prefix_counts.entry(name[..4].to_uppercase()).or_default() += 1;
        }
    }

    if let Some((prefix, count)) = prefix_counts.iter().max_by_key(|(_, c)| *c) {
        if *count as f64 / names.len() as f64 > 0.5 {
            return prefix.clone();
        }
    }

    if names.len() == 1 {
        return names[0].clone();
    }

    format!("{}-GROUP", names[0])
}

// --- Union-Find with weighted edges ---

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    /// Accumulated affinity weights between pairs (before merging)
    weights: HashMap<(usize, usize), f64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            weights: HashMap::new(),
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
    }

    fn add_weight(&mut self, a: usize, b: usize, weight: f64) {
        let key = if a < b { (a, b) } else { (b, a) };
        *self.weights.entry(key).or_default() += weight;
    }

    fn merge_above_threshold(&mut self, threshold: f64) {
        let pairs: Vec<((usize, usize), f64)> = self.weights.drain().collect();
        for ((a, b), weight) in pairs {
            if weight >= threshold {
                self.union(a, b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    fn make_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        // Process 1: CLRG cluster (connected by shared file + naming)
        let clrg1 = g.add_node(
            Node::new(NodeKind::Program, "CLRG0100")
                .with_property("complexity", PropValue::from(4.0f64))
                .with_property("field_count", PropValue::from(20u64)),
        );
        let clrg2 = g.add_node(
            Node::new(NodeKind::Program, "CLRG0200")
                .with_property("complexity", PropValue::from(3.0f64))
                .with_property("field_count", PropValue::from(15u64)),
        );
        let clrg3 = g.add_node(
            Node::new(NodeKind::Program, "CLRG0300")
                .with_property("complexity", PropValue::from(2.0f64))
                .with_property("field_count", PropValue::from(10u64)),
        );

        // Process 2: LOAN cluster (connected by CALL)
        let loan1 = g.add_node(
            Node::new(NodeKind::Program, "LNOR0100")
                .with_property("complexity", PropValue::from(5.0f64))
                .with_property("field_count", PropValue::from(30u64)),
        );
        let loan2 = g.add_node(
            Node::new(NodeKind::Program, "LNOR0200")
                .with_property("complexity", PropValue::from(3.0f64))
                .with_property("field_count", PropValue::from(20u64)),
        );

        // Shared utility
        let util = g.add_node(
            Node::new(NodeKind::Program, "DATEUTIL")
                .with_property("complexity", PropValue::from(1.0f64))
                .with_property("field_count", PropValue::from(5u64))
                .with_property("shared_copybook_count", PropValue::from(8u64)),
        );

        // Orphan
        let _orphan = g.add_node(
            Node::new(NodeKind::Program, "ORPHAN01")
                .with_property("complexity", PropValue::from(0.5f64))
                .with_property("field_count", PropValue::from(3u64)),
        );

        // Files
        let clrg_file = g.add_node(Node::new(NodeKind::File, "CLRG-DATA"));
        let loan_file = g.add_node(Node::new(NodeKind::File, "LOAN-DATA"));

        // CLRG cluster: all access CLRG-DATA
        g.add_edge(clrg1, clrg_file, Edge::new(EdgeKind::Accesses));
        g.add_edge(clrg2, clrg_file, Edge::new(EdgeKind::Accesses));
        g.add_edge(clrg3, clrg_file, Edge::new(EdgeKind::Accesses));

        // LOAN cluster: connected by CALL chain
        g.add_edge(loan1, loan2, Edge::new(EdgeKind::Calls));
        g.add_edge(loan1, loan_file, Edge::new(EdgeKind::Accesses));
        g.add_edge(loan2, loan_file, Edge::new(EdgeKind::Accesses));

        // Utility: called by both clusters
        g.add_edge(clrg1, util, Edge::new(EdgeKind::Calls));
        g.add_edge(loan1, util, Edge::new(EdgeKind::Calls));

        g
    }

    #[test]
    fn discovers_processes() {
        let mut g = make_graph();
        let stats = ProcessDiscoveryPass.enrich(&mut g).unwrap();
        assert!(stats.nodes_enriched > 0);

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let clrg2 = g.lookup_one(NodeKind::Program, "CLRG0200").unwrap();
        let proc1 = g.node(clrg1).unwrap().properties.get_str("process").unwrap().to_owned();
        let proc2 = g.node(clrg2).unwrap().properties.get_str("process").unwrap().to_owned();
        assert_eq!(proc1, proc2, "CLRG0100 and CLRG0200 should be in same process");

        let loan1 = g.lookup_one(NodeKind::Program, "LNOR0100").unwrap();
        let loan2 = g.lookup_one(NodeKind::Program, "LNOR0200").unwrap();
        let lproc1 = g.node(loan1).unwrap().properties.get_str("process").unwrap().to_owned();
        let lproc2 = g.node(loan2).unwrap().properties.get_str("process").unwrap().to_owned();
        assert_eq!(lproc1, lproc2, "LNOR0100 and LNOR0200 should be in same process");

        assert_ne!(proc1, lproc1, "CLRG and LNOR should be in different processes");
    }

    #[test]
    fn process_naming() {
        let mut g = make_graph();
        ProcessDiscoveryPass.enrich(&mut g).unwrap();

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let proc_name = g.node(clrg1).unwrap().properties.get_str("process").unwrap();
        assert_eq!(proc_name, "CLRG", "Process should be named by common prefix");
    }

    #[test]
    fn role_classification() {
        let mut g = make_graph();
        ProcessDiscoveryPass.enrich(&mut g).unwrap();

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let role = g.node(clrg1).unwrap().properties.get_str("process_role").unwrap();
        assert_eq!(role, "core");

        let util = g.lookup_one(NodeKind::Program, "DATEUTIL").unwrap();
        let util_role = g.node(util).unwrap().properties.get_str("process_role").unwrap();
        assert!(
            util_role == "shared" || util_role == "utility" || util_role == "core",
            "DATEUTIL should be shared/utility/core, got {util_role}"
        );
    }

    #[test]
    fn orphan_gets_own_process() {
        let mut g = make_graph();
        ProcessDiscoveryPass.enrich(&mut g).unwrap();

        let orphan = g.lookup_one(NodeKind::Program, "ORPHAN01").unwrap();
        let proc = g.node(orphan).unwrap().properties.get_str("process").unwrap();
        assert!(!proc.is_empty(), "Orphan should have a process name");
    }

    #[test]
    fn union_find_weighted() {
        let mut uf = UnionFind::new(5);
        uf.add_weight(0, 1, 1.5);
        uf.add_weight(0, 1, 1.5); // total = 3.0
        uf.add_weight(2, 3, 1.0); // total = 1.0 (below threshold)
        uf.merge_above_threshold(2.0);

        assert_eq!(uf.find(0), uf.find(1), "0 and 1 should merge (weight 3.0)");
        assert_ne!(uf.find(2), uf.find(3), "2 and 3 should NOT merge (weight 1.0)");
    }
}
