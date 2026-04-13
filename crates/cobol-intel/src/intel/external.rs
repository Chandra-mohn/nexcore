use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// Layer 5: External Interface Intelligence.
///
/// Computes per-program:
/// - `file_inventory`: list of file names accessed
/// - `table_inventory`: list of SQL table names accessed
/// - `interface_count`: total external interfaces (files + tables)
/// - `interface_complexity`: weighted score based on interface diversity
/// - `has_sql`: whether the program uses EXEC SQL
/// - `has_file_io`: whether the program uses file I/O
///
/// Computes per-file:
/// - `accessor_count`: number of programs accessing this file
///
/// Computes per-table:
/// - `accessor_count`: number of programs accessing this table
/// - `sql_operations`: list of SQL operation types (from edge properties)
#[derive(Debug)]
pub struct ExternalPass;

impl IntelligencePass for ExternalPass {
    fn name(&self) -> &'static str {
        "external"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        // Enrich files
        let file_ids = graph.all_of_kind(NodeKind::File);
        for file_id in &file_ids {
            let accessors = graph.neighbors_incoming(*file_id, EdgeKind::Accesses);
            if let Some(node) = graph.node_mut(*file_id) {
                node.properties
                    .set("accessor_count", PropValue::from(accessors.len() as u64));
                stats.nodes_enriched += 1;
                stats.properties_added += 1;
            }
        }

        // Enrich tables
        let table_ids = graph.all_of_kind(NodeKind::Table);
        for table_id in &table_ids {
            let accessors = graph.neighbors_incoming(*table_id, EdgeKind::Accesses);

            // Collect SQL operation types from edge properties
            let mut operations: Vec<String> = Vec::new();
            for accessor_id in &accessors {
                if let Some(edge) = graph.edge_between(*accessor_id, *table_id) {
                    if let Some(sql_type) = edge.properties.get_str("sql_type") {
                        if !operations.contains(&sql_type.to_owned()) {
                            operations.push(sql_type.to_owned());
                        }
                    }
                }
            }

            if let Some(node) = graph.node_mut(*table_id) {
                node.properties
                    .set("accessor_count", PropValue::from(accessors.len() as u64));
                if !operations.is_empty() {
                    node.properties
                        .set("sql_operations", PropValue::from(operations));
                }
                stats.nodes_enriched += 1;
                stats.properties_added += 2;
            }
        }

        // Enrich programs
        let program_ids = graph.all_of_kind(NodeKind::Program);
        for prog_id in &program_ids {
            let accessed = graph.neighbors_outgoing(*prog_id, EdgeKind::Accesses);

            let mut file_names = Vec::new();
            let mut table_names = Vec::new();

            for target_id in &accessed {
                if let Some(target) = graph.node(*target_id) {
                    match target.kind {
                        NodeKind::File => file_names.push(target.name.clone()),
                        NodeKind::Table => table_names.push(target.name.clone()),
                        _ => {}
                    }
                }
            }

            let has_file_io = !file_names.is_empty();
            let has_sql = !table_names.is_empty();
            let interface_count = file_names.len() + table_names.len();

            // Interface complexity: files are 1 point each, tables are 2 points each
            // (SQL is more complex than sequential file I/O)
            let interface_complexity =
                (file_names.len() as f64 + table_names.len() as f64 * 2.0).min(10.0);

            if let Some(node) = graph.node_mut(*prog_id) {
                if !file_names.is_empty() {
                    node.properties
                        .set("file_inventory", PropValue::from(file_names));
                }
                if !table_names.is_empty() {
                    node.properties
                        .set("table_inventory", PropValue::from(table_names));
                }
                node.properties
                    .set("interface_count", PropValue::from(interface_count as u64));
                node.properties
                    .set("interface_complexity", PropValue::from(interface_complexity));
                node.properties
                    .set("has_file_io", PropValue::from(has_file_io));
                node.properties
                    .set("has_sql", PropValue::from(has_sql));
                stats.nodes_enriched += 1;
                stats.properties_added += 6;
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

        let prog1 = g.add_node(Node::new(NodeKind::Program, "PROG1"));
        let prog2 = g.add_node(Node::new(NodeKind::Program, "PROG2"));
        let prog3 = g.add_node(Node::new(NodeKind::Program, "NODB"));

        let file1 = g.add_node(
            Node::new(NodeKind::File, "ACCTMAST")
                .with_property("organization", PropValue::from("Indexed")),
        );
        let file2 = g.add_node(Node::new(NodeKind::File, "TXNFILE"));

        let table1 = g.add_node(Node::new(NodeKind::Table, "ACCT_TABLE"));
        let table2 = g.add_node(Node::new(NodeKind::Table, "TXN_LOG"));

        // PROG1: accesses 2 files + 1 table
        g.add_edge(prog1, file1, Edge::new(EdgeKind::Accesses));
        g.add_edge(prog1, file2, Edge::new(EdgeKind::Accesses));
        g.add_edge(
            prog1,
            table1,
            Edge::new(EdgeKind::Accesses)
                .with_property("sql_type", PropValue::from("SelectInto")),
        );

        // PROG2: accesses 1 file + 2 tables
        g.add_edge(prog2, file1, Edge::new(EdgeKind::Accesses));
        g.add_edge(
            prog2,
            table1,
            Edge::new(EdgeKind::Accesses)
                .with_property("sql_type", PropValue::from("Update")),
        );
        g.add_edge(
            prog2,
            table2,
            Edge::new(EdgeKind::Accesses)
                .with_property("sql_type", PropValue::from("Insert")),
        );

        // NODB: no external interfaces
        let _ = prog3;

        g
    }

    #[test]
    fn program_interface_inventory() {
        let mut g = make_graph();
        ExternalPass.enrich(&mut g).unwrap();

        let prog1 = g.lookup_one(NodeKind::Program, "PROG1").unwrap();
        let node = g.node(prog1).unwrap();

        assert_eq!(node.properties.get_bool("has_file_io"), Some(true));
        assert_eq!(node.properties.get_bool("has_sql"), Some(true));
        assert_eq!(node.properties.get_u64("interface_count"), Some(3));
    }

    #[test]
    fn program_no_interfaces() {
        let mut g = make_graph();
        ExternalPass.enrich(&mut g).unwrap();

        let nodb = g.lookup_one(NodeKind::Program, "NODB").unwrap();
        let node = g.node(nodb).unwrap();

        assert_eq!(node.properties.get_bool("has_file_io"), Some(false));
        assert_eq!(node.properties.get_bool("has_sql"), Some(false));
        assert_eq!(node.properties.get_u64("interface_count"), Some(0));
    }

    #[test]
    fn interface_complexity_scoring() {
        let mut g = make_graph();
        ExternalPass.enrich(&mut g).unwrap();

        // PROG1: 2 files + 1 table = 2 + 2 = 4.0
        let prog1 = g.lookup_one(NodeKind::Program, "PROG1").unwrap();
        assert_eq!(
            g.node(prog1)
                .unwrap()
                .properties
                .get_f64("interface_complexity"),
            Some(4.0)
        );

        // PROG2: 1 file + 2 tables = 1 + 4 = 5.0
        let prog2 = g.lookup_one(NodeKind::Program, "PROG2").unwrap();
        assert_eq!(
            g.node(prog2)
                .unwrap()
                .properties
                .get_f64("interface_complexity"),
            Some(5.0)
        );
    }

    #[test]
    fn file_accessor_count() {
        let mut g = make_graph();
        ExternalPass.enrich(&mut g).unwrap();

        let file1 = g.lookup_one(NodeKind::File, "ACCTMAST").unwrap();
        assert_eq!(
            g.node(file1).unwrap().properties.get_u64("accessor_count"),
            Some(2) // PROG1 and PROG2
        );
    }

    #[test]
    fn table_sql_operations() {
        let mut g = make_graph();
        ExternalPass.enrich(&mut g).unwrap();

        let table1 = g.lookup_one(NodeKind::Table, "ACCT_TABLE").unwrap();
        let node = g.node(table1).unwrap();
        assert_eq!(node.properties.get_u64("accessor_count"), Some(2));

        if let Some(PropValue::List(ops)) = node.properties.get("sql_operations") {
            assert!(ops.contains(&"SelectInto".to_owned()));
            assert!(ops.contains(&"Update".to_owned()));
        } else {
            panic!("expected sql_operations list");
        }
    }
}
