use std::collections::HashMap;

use super::node::{NodeId, NodeKind};

/// Secondary indexes for fast node lookup by name, kind, and parent program.
#[derive(Debug, Clone, Default)]
pub struct GraphIndex {
    /// (kind, name) -> NodeId. For programs, name alone is unique.
    /// For paragraphs/fields, the name may repeat across programs,
    /// so we store the first match and use `by_program` for scoped lookups.
    by_kind_name: HashMap<(NodeKind, String), Vec<NodeId>>,

    /// program_name -> list of NodeIds belonging to that program
    /// (paragraphs, fields, rules).
    by_program: HashMap<String, Vec<NodeId>>,

    /// copybook_name -> list of program NodeIds that use it.
    by_copybook: HashMap<String, Vec<NodeId>>,

    /// file_name -> list of program NodeIds that access it.
    by_file: HashMap<String, Vec<NodeId>>,
}

impl GraphIndex {
    pub fn insert(&mut self, id: NodeId, kind: NodeKind, name: &str, program: Option<&str>) {
        self.by_kind_name
            .entry((kind, name.to_owned()))
            .or_default()
            .push(id);

        if let Some(prog) = program {
            self.by_program.entry(prog.to_owned()).or_default().push(id);
        }
    }

    pub fn add_copybook_usage(&mut self, copybook_name: &str, program_id: NodeId) {
        self.by_copybook
            .entry(copybook_name.to_owned())
            .or_default()
            .push(program_id);
    }

    pub fn add_file_access(&mut self, file_name: &str, program_id: NodeId) {
        self.by_file
            .entry(file_name.to_owned())
            .or_default()
            .push(program_id);
    }

    /// Look up nodes by kind and name. Returns all matches
    /// (e.g., paragraphs named "1000-MAIN" in different programs).
    pub fn lookup(&self, kind: NodeKind, name: &str) -> &[NodeId] {
        self.by_kind_name
            .get(&(kind, name.to_owned()))
            .map_or(&[], Vec::as_slice)
    }

    /// Look up a single node by kind and name. Returns the first match.
    pub fn lookup_one(&self, kind: NodeKind, name: &str) -> Option<NodeId> {
        self.lookup(kind, name).first().copied()
    }

    /// All nodes belonging to a program (paragraphs, fields, rules).
    pub fn nodes_in_program(&self, program_name: &str) -> &[NodeId] {
        self.by_program
            .get(program_name)
            .map_or(&[], Vec::as_slice)
    }

    /// Programs that use a given copybook.
    pub fn programs_using_copybook(&self, copybook_name: &str) -> &[NodeId] {
        self.by_copybook
            .get(copybook_name)
            .map_or(&[], Vec::as_slice)
    }

    /// Programs that access a given file.
    pub fn programs_accessing_file(&self, file_name: &str) -> &[NodeId] {
        self.by_file
            .get(file_name)
            .map_or(&[], Vec::as_slice)
    }

    /// Total number of indexed entries (for stats).
    pub fn entry_count(&self) -> usize {
        self.by_kind_name.values().map(Vec::len).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::NodeIndex;

    fn nid(n: u32) -> NodeId {
        NodeIndex::new(n as usize)
    }

    #[test]
    fn lookup_by_kind_name() {
        let mut idx = GraphIndex::default();
        idx.insert(nid(0), NodeKind::Program, "CLRG0100", None);
        idx.insert(nid(1), NodeKind::Program, "VALUTIL", None);

        assert_eq!(idx.lookup(NodeKind::Program, "CLRG0100"), &[nid(0)]);
        assert_eq!(idx.lookup(NodeKind::Program, "VALUTIL"), &[nid(1)]);
        assert_eq!(idx.lookup(NodeKind::Program, "NOPE"), &[]);
    }

    #[test]
    fn lookup_one() {
        let mut idx = GraphIndex::default();
        idx.insert(nid(5), NodeKind::Copybook, "CPYCLRG", None);

        assert_eq!(idx.lookup_one(NodeKind::Copybook, "CPYCLRG"), Some(nid(5)));
        assert_eq!(idx.lookup_one(NodeKind::Copybook, "NOPE"), None);
    }

    #[test]
    fn duplicate_paragraph_names_across_programs() {
        let mut idx = GraphIndex::default();
        idx.insert(nid(10), NodeKind::Paragraph, "1000-MAIN", Some("PROG-A"));
        idx.insert(nid(20), NodeKind::Paragraph, "1000-MAIN", Some("PROG-B"));

        let matches = idx.lookup(NodeKind::Paragraph, "1000-MAIN");
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&nid(10)));
        assert!(matches.contains(&nid(20)));
    }

    #[test]
    fn nodes_in_program() {
        let mut idx = GraphIndex::default();
        idx.insert(nid(1), NodeKind::Paragraph, "1000-MAIN", Some("CLRG0100"));
        idx.insert(nid(2), NodeKind::Paragraph, "2000-PROCESS", Some("CLRG0100"));
        idx.insert(nid(3), NodeKind::Field, "WS-ACCT-NBR", Some("CLRG0100"));

        let nodes = idx.nodes_in_program("CLRG0100");
        assert_eq!(nodes.len(), 3);
    }

    #[test]
    fn copybook_usage() {
        let mut idx = GraphIndex::default();
        idx.add_copybook_usage("CPYCLRG", nid(0));
        idx.add_copybook_usage("CPYCLRG", nid(1));

        assert_eq!(idx.programs_using_copybook("CPYCLRG").len(), 2);
        assert_eq!(idx.programs_using_copybook("NOPE").len(), 0);
    }

    #[test]
    fn file_access() {
        let mut idx = GraphIndex::default();
        idx.add_file_access("ACCTMAST", nid(0));

        assert_eq!(idx.programs_accessing_file("ACCTMAST"), &[nid(0)]);
    }
}
