pub mod builder;
pub mod edge;
pub mod index;
pub mod node;
pub mod storage;

use petgraph::stable_graph::StableDiGraph;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use serde::{Deserialize, Serialize};

use crate::error::{IntelError, IntelResult};
use edge::{Edge, EdgeKind};
use index::GraphIndex;
use node::{Node, NodeId, NodeKind};

/// Portable serialization format. petgraph types don't implement serde,
/// so we flatten to vecs of nodes and edges for storage.
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphSnapshot {
    pub nodes: Vec<Node>,
    pub edges: Vec<(usize, usize, Edge)>, // (source_idx, target_idx, edge)
}

/// The code intelligence graph. Nodes are COBOL entities (programs, paragraphs,
/// fields, copybooks, files, tables, rules). Edges are relationships between them.
#[derive(Debug, Clone)]
pub struct CodeGraph {
    graph: StableDiGraph<Node, Edge>,
    index: GraphIndex,
}

impl CodeGraph {
    pub fn new() -> Self {
        Self {
            graph: StableDiGraph::new(),
            index: GraphIndex::default(),
        }
    }

    /// Serialize to a portable snapshot.
    pub fn to_snapshot(&self) -> GraphSnapshot {
        let node_indices: Vec<NodeId> = self.graph.node_indices().collect();
        let idx_map: std::collections::HashMap<NodeId, usize> = node_indices
            .iter()
            .enumerate()
            .map(|(i, id)| (*id, i))
            .collect();

        let nodes: Vec<Node> = node_indices
            .iter()
            .map(|id| self.graph.node_weight(*id).unwrap().clone())
            .collect();

        let edges: Vec<(usize, usize, Edge)> = self
            .graph
            .edge_indices()
            .filter_map(|eid| {
                let (src, tgt) = self.graph.edge_endpoints(eid)?;
                let edge = self.graph.edge_weight(eid)?.clone();
                Some((idx_map[&src], idx_map[&tgt], edge))
            })
            .collect();

        GraphSnapshot { nodes, edges }
    }

    /// Rebuild from a portable snapshot.
    pub fn from_snapshot(snap: GraphSnapshot) -> Self {
        let mut g = Self::new();
        let mut id_map: Vec<NodeId> = Vec::with_capacity(snap.nodes.len());

        for node in snap.nodes {
            let id = g.add_node(node);
            id_map.push(id);
        }

        for (src_idx, tgt_idx, edge) in snap.edges {
            g.add_edge(id_map[src_idx], id_map[tgt_idx], edge);
        }

        // Rebuild copybook/file index from edges
        for id in g.graph.node_indices() {
            let node = g.graph.node_weight(id).unwrap();
            if node.kind == NodeKind::Program {
                let prog_name = node.name.clone();
                let copybooks: Vec<NodeId> = g.neighbors_outgoing(id, EdgeKind::Uses);
                for cb_id in copybooks {
                    if let Some(cb_node) = g.graph.node_weight(cb_id) {
                        g.index.add_copybook_usage(&cb_node.name, id);
                    }
                }
                let files: Vec<NodeId> = g.neighbors_outgoing(id, EdgeKind::Accesses);
                for f_id in files {
                    if let Some(f_node) = g.graph.node_weight(f_id) {
                        if f_node.kind == NodeKind::File {
                            g.index.add_file_access(&f_node.name, id);
                        }
                    }
                }
                let _ = prog_name;
            }
        }

        g
    }

    /// Add a node to the graph. Returns its `NodeId`.
    pub fn add_node(&mut self, node: Node) -> NodeId {
        let kind = node.kind;
        let name = node.name.clone();
        let program = node.program.clone();
        let id = self.graph.add_node(node);
        self.index.insert(id, kind, &name, program.as_deref());
        id
    }

    /// Add a directed edge between two nodes.
    pub fn add_edge(&mut self, source: NodeId, target: NodeId, edge: Edge) {
        self.graph.add_edge(source, target, edge);
    }

    /// Get a node by its ID.
    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.graph.node_weight(id)
    }

    /// Get a mutable reference to a node by its ID.
    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.graph.node_weight_mut(id)
    }

    /// Get an edge between two nodes (first match).
    pub fn edge_between(&self, source: NodeId, target: NodeId) -> Option<&Edge> {
        self.graph
            .edges_connecting(source, target)
            .next()
            .map(|e| e.weight())
    }

    /// Look up nodes by kind and name.
    pub fn lookup(&self, kind: NodeKind, name: &str) -> Vec<&Node> {
        self.index
            .lookup(kind, name)
            .iter()
            .filter_map(|id| self.graph.node_weight(*id))
            .collect()
    }

    /// Look up a single node by kind and name.
    pub fn lookup_one(&self, kind: NodeKind, name: &str) -> IntelResult<NodeId> {
        self.index.lookup_one(kind, name).ok_or_else(|| IntelError::NodeNotFound {
            kind: kind.to_string(),
            name: name.to_owned(),
        })
    }

    /// Get all neighbors along outgoing edges of a given kind.
    pub fn neighbors_outgoing(&self, id: NodeId, edge_kind: EdgeKind) -> Vec<NodeId> {
        self.graph
            .edges_directed(id, Direction::Outgoing)
            .filter(|e| e.weight().kind == edge_kind)
            .map(|e| e.target())
            .collect()
    }

    /// Get all neighbors along incoming edges of a given kind.
    pub fn neighbors_incoming(&self, id: NodeId, edge_kind: EdgeKind) -> Vec<NodeId> {
        self.graph
            .edges_directed(id, Direction::Incoming)
            .filter(|e| e.weight().kind == edge_kind)
            .map(|e| e.source())
            .collect()
    }

    /// Traverse: given a node and a NexQuery verb, return matching neighbors.
    pub fn traverse(&self, id: NodeId, verb: &str) -> IntelResult<Vec<NodeId>> {
        let (edge_kind, is_forward) =
            EdgeKind::from_verb(verb).ok_or_else(|| IntelError::QueryError {
                reason: format!("unknown traversal verb: '{verb}'"),
            })?;

        if is_forward {
            Ok(self.neighbors_outgoing(id, edge_kind))
        } else {
            Ok(self.neighbors_incoming(id, edge_kind))
        }
    }

    /// All nodes of a given kind.
    pub fn all_of_kind(&self, kind: NodeKind) -> Vec<NodeId> {
        self.graph
            .node_indices()
            .filter(|id| {
                self.graph
                    .node_weight(*id)
                    .is_some_and(|n| n.kind == kind)
            })
            .collect()
    }

    /// Nodes belonging to a specific program.
    pub fn nodes_in_program(&self, program_name: &str) -> &[NodeId] {
        self.index.nodes_in_program(program_name)
    }

    /// Record that a program uses a copybook (for index).
    pub fn index_copybook_usage(&mut self, copybook_name: &str, program_id: NodeId) {
        self.index.add_copybook_usage(copybook_name, program_id);
    }

    /// Record that a program accesses a file (for index).
    pub fn index_file_access(&mut self, file_name: &str, program_id: NodeId) {
        self.index.add_file_access(file_name, program_id);
    }

    /// Total node count.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Total edge count.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Access the underlying petgraph (for advanced traversals).
    pub fn raw_graph(&self) -> &StableDiGraph<Node, Edge> {
        &self.graph
    }

    /// Access the index.
    pub fn index(&self) -> &GraphIndex {
        &self.index
    }
}

impl Default for CodeGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use node::PropValue;

    fn make_test_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        let clrg = g.add_node(
            Node::new(NodeKind::Program, "CLRG0100")
                .with_property("type", PropValue::from("batch"))
                .with_property("loc", PropValue::from(1847u64)),
        );
        let valutil = g.add_node(
            Node::new(NodeKind::Program, "VALUTIL")
                .with_property("type", PropValue::from("subprogram")),
        );
        let main_para = g.add_node(
            Node::new(NodeKind::Paragraph, "1000-MAIN").with_program("CLRG0100"),
        );
        let validate = g.add_node(
            Node::new(NodeKind::Paragraph, "2000-VALIDATE").with_program("CLRG0100"),
        );
        let acct_nbr = g.add_node(
            Node::new(NodeKind::Field, "WS-ACCT-NBR").with_program("CLRG0100"),
        );
        let cpyclrg = g.add_node(Node::new(NodeKind::Copybook, "CPYCLRG"));

        g.add_edge(clrg, valutil, Edge::new(EdgeKind::Calls));
        g.add_edge(clrg, main_para, Edge::new(EdgeKind::Contains));
        g.add_edge(clrg, validate, Edge::new(EdgeKind::Contains));
        g.add_edge(main_para, validate, Edge::new(EdgeKind::Performs));
        g.add_edge(validate, acct_nbr, Edge::new(EdgeKind::Reads));
        g.add_edge(clrg, cpyclrg, Edge::new(EdgeKind::Uses));

        g.index_copybook_usage("CPYCLRG", clrg);

        g
    }

    #[test]
    fn graph_counts() {
        let g = make_test_graph();
        assert_eq!(g.node_count(), 6);
        assert_eq!(g.edge_count(), 6);
    }

    #[test]
    fn lookup_by_kind_name() {
        let g = make_test_graph();
        let nodes = g.lookup(NodeKind::Program, "CLRG0100");
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "CLRG0100");
    }

    #[test]
    fn lookup_one_success() {
        let g = make_test_graph();
        let id = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        assert_eq!(g.node(id).unwrap().name, "CLRG0100");
    }

    #[test]
    fn lookup_one_not_found() {
        let g = make_test_graph();
        let err = g.lookup_one(NodeKind::Program, "NOPE").unwrap_err();
        assert!(err.to_string().contains("NOPE"));
    }

    #[test]
    fn traverse_forward_calling() {
        let g = make_test_graph();
        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let called = g.traverse(clrg, "calling").unwrap();
        assert_eq!(called.len(), 1);
        assert_eq!(g.node(called[0]).unwrap().name, "VALUTIL");
    }

    #[test]
    fn traverse_reverse_called_by() {
        let g = make_test_graph();
        let valutil = g.lookup_one(NodeKind::Program, "VALUTIL").unwrap();
        let callers = g.traverse(valutil, "called-by").unwrap();
        assert_eq!(callers.len(), 1);
        assert_eq!(g.node(callers[0]).unwrap().name, "CLRG0100");
    }

    #[test]
    fn traverse_containing() {
        let g = make_test_graph();
        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let paras = g.traverse(clrg, "containing").unwrap();
        assert_eq!(paras.len(), 2);
    }

    #[test]
    fn traverse_within() {
        let g = make_test_graph();
        let main_para = g.index().lookup_one(NodeKind::Paragraph, "1000-MAIN").unwrap();
        let programs = g.traverse(main_para, "within").unwrap();
        assert_eq!(programs.len(), 1);
        assert_eq!(g.node(programs[0]).unwrap().name, "CLRG0100");
    }

    #[test]
    fn traverse_performing() {
        let g = make_test_graph();
        let main_para = g.index().lookup_one(NodeKind::Paragraph, "1000-MAIN").unwrap();
        let performed = g.traverse(main_para, "performing").unwrap();
        assert_eq!(performed.len(), 1);
        assert_eq!(g.node(performed[0]).unwrap().name, "2000-VALIDATE");
    }

    #[test]
    fn traverse_reading() {
        let g = make_test_graph();
        let validate = g.index().lookup_one(NodeKind::Paragraph, "2000-VALIDATE").unwrap();
        let fields = g.traverse(validate, "reading").unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(g.node(fields[0]).unwrap().name, "WS-ACCT-NBR");
    }

    #[test]
    fn traverse_unknown_verb() {
        let g = make_test_graph();
        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let err = g.traverse(clrg, "foobar").unwrap_err();
        assert!(err.to_string().contains("foobar"));
    }

    #[test]
    fn all_of_kind() {
        let g = make_test_graph();
        assert_eq!(g.all_of_kind(NodeKind::Program).len(), 2);
        assert_eq!(g.all_of_kind(NodeKind::Paragraph).len(), 2);
    }

    #[test]
    fn multi_hop_traversal() {
        let g = make_test_graph();
        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let paras = g.traverse(clrg, "containing").unwrap();
        let mut all_fields = Vec::new();
        for para in &paras {
            all_fields.extend(g.traverse(*para, "reading").unwrap());
        }
        assert_eq!(all_fields.len(), 1);
        assert_eq!(g.node(all_fields[0]).unwrap().name, "WS-ACCT-NBR");
    }

    #[test]
    fn snapshot_roundtrip() {
        let g = make_test_graph();
        let snap = g.to_snapshot();

        assert_eq!(snap.nodes.len(), 6);
        assert_eq!(snap.edges.len(), 6);

        let g2 = CodeGraph::from_snapshot(snap);
        assert_eq!(g2.node_count(), 6);
        assert_eq!(g2.edge_count(), 6);

        // Verify traversals still work
        let clrg = g2.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let called = g2.traverse(clrg, "calling").unwrap();
        assert_eq!(called.len(), 1);
        assert_eq!(g2.node(called[0]).unwrap().name, "VALUTIL");
    }
}
