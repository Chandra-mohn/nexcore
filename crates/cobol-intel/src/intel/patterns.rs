use crate::error::IntelResult;
use crate::graph::node::{NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// Layer 8: Pattern & Idiom Intelligence.
///
/// Computes per-program:
/// - `fingerprint`: structural fingerprint string for similarity matching
///   Format: "P{para_count}C{call_count}F{field_count}I{interface_count}"
///   Programs with identical fingerprints are structurally similar.
/// - `pattern_class`: batch-read-process-write / batch-report / online-cics /
///   db-crud / utility / unknown
/// - `era`: modern (has SQL) / structured (>5 paragraphs, no SQL) /
///   simple (<= 5 paragraphs)
/// - `similar_programs`: list of program names with the same fingerprint
#[derive(Debug)]
pub struct PatternPass;

impl IntelligencePass for PatternPass {
    fn name(&self) -> &'static str {
        "patterns"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        let program_ids = graph.all_of_kind(NodeKind::Program);

        // Step 1: Compute fingerprints for all programs
        let mut fingerprints: Vec<(petgraph::graph::NodeIndex, String, String)> = Vec::new();

        for prog_id in &program_ids {
            if let Some(node) = graph.node(*prog_id) {
                let para_count = node.properties.get_u64("paragraph_count").unwrap_or(0);
                let call_count = node.properties.get_u64("call_count").unwrap_or(0);
                let field_count = node.properties.get_u64("field_count").unwrap_or(0);
                let interface_count = node.properties.get_u64("interface_count").unwrap_or(0);

                // Bucket into ranges for fuzzy matching
                let fp = format!(
                    "P{}C{}F{}I{}",
                    bucket(para_count),
                    bucket(call_count),
                    bucket(field_count),
                    bucket(interface_count),
                );

                fingerprints.push((*prog_id, node.name.clone(), fp));
            }
        }

        // Step 2: Find similar programs (same fingerprint)
        let mut similarity_groups: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        for (_, name, fp) in &fingerprints {
            similarity_groups
                .entry(fp.clone())
                .or_default()
                .push(name.clone());
        }

        // Step 3: Apply properties
        for (prog_id, prog_name, fp) in &fingerprints {
            let pattern_class = classify_pattern(graph, *prog_id);
            let era = classify_era(graph, *prog_id);

            let similar: Vec<String> = similarity_groups
                .get(fp)
                .map(|group| {
                    group
                        .iter()
                        .filter(|n| n.as_str() != prog_name.as_str())
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();

            if let Some(node) = graph.node_mut(*prog_id) {
                node.properties
                    .set("fingerprint", PropValue::from(fp.clone()));
                node.properties
                    .set("pattern_class", PropValue::from(pattern_class));
                node.properties
                    .set("era", PropValue::from(era));
                if !similar.is_empty() {
                    node.properties
                        .set("similar_programs", PropValue::from(similar));
                }
                stats.nodes_enriched += 1;
                stats.properties_added += 4;
            }
        }

        Ok(stats)
    }
}

/// Bucket a count into a range label for fingerprinting.
/// This allows fuzzy matching: programs with 12 and 14 paragraphs
/// are both "M" (medium) and thus similar.
fn bucket(count: u64) -> &'static str {
    match count {
        0 => "0",
        1..=3 => "S",
        4..=10 => "M",
        11..=30 => "L",
        _ => "X",
    }
}

fn classify_pattern(graph: &CodeGraph, prog_id: petgraph::graph::NodeIndex) -> &'static str {
    let node = match graph.node(prog_id) {
        Some(n) => n,
        None => return "unknown",
    };

    let has_sql = node.properties.get_bool("has_sql").unwrap_or(false);
    let has_file_io = node.properties.get_bool("has_file_io").unwrap_or(false);
    let has_rules = node.properties.get_bool("has_rules").unwrap_or(false);
    let call_count = node.properties.get_u64("call_count").unwrap_or(0);
    let para_count = node.properties.get_u64("paragraph_count").unwrap_or(0);

    if has_sql && has_file_io {
        "batch-db-io"
    } else if has_sql && !has_file_io {
        "db-crud"
    } else if has_file_io && has_rules {
        "batch-read-process-write"
    } else if has_file_io && !has_rules {
        "batch-io"
    } else if call_count == 0 && para_count <= 3 {
        "utility"
    } else {
        "unknown"
    }
}

fn classify_era(graph: &CodeGraph, prog_id: petgraph::graph::NodeIndex) -> &'static str {
    let node = match graph.node(prog_id) {
        Some(n) => n,
        None => return "unknown",
    };

    let has_sql = node.properties.get_bool("has_sql").unwrap_or(false);
    let para_count = node.properties.get_u64("paragraph_count").unwrap_or(0);

    if has_sql {
        "modern"
    } else if para_count > 5 {
        "structured"
    } else {
        "simple"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::node::Node;

    fn make_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        // Two similar batch programs (same shape)
        g.add_node(
            Node::new(NodeKind::Program, "CLRG0100")
                .with_property("paragraph_count", PropValue::from(8u64))
                .with_property("call_count", PropValue::from(2u64))
                .with_property("field_count", PropValue::from(15u64))
                .with_property("interface_count", PropValue::from(2u64))
                .with_property("has_file_io", PropValue::from(true))
                .with_property("has_sql", PropValue::from(false))
                .with_property("has_rules", PropValue::from(true)),
        );
        g.add_node(
            Node::new(NodeKind::Program, "CLRG0200")
                .with_property("paragraph_count", PropValue::from(7u64))
                .with_property("call_count", PropValue::from(2u64))
                .with_property("field_count", PropValue::from(12u64))
                .with_property("interface_count", PropValue::from(2u64))
                .with_property("has_file_io", PropValue::from(true))
                .with_property("has_sql", PropValue::from(false))
                .with_property("has_rules", PropValue::from(true)),
        );

        // A SQL program (different shape)
        g.add_node(
            Node::new(NodeKind::Program, "DBPROG")
                .with_property("paragraph_count", PropValue::from(3u64))
                .with_property("call_count", PropValue::from(0u64))
                .with_property("field_count", PropValue::from(5u64))
                .with_property("interface_count", PropValue::from(2u64))
                .with_property("has_file_io", PropValue::from(false))
                .with_property("has_sql", PropValue::from(true))
                .with_property("has_rules", PropValue::from(false)),
        );

        // A tiny utility
        g.add_node(
            Node::new(NodeKind::Program, "UTIL01")
                .with_property("paragraph_count", PropValue::from(1u64))
                .with_property("call_count", PropValue::from(0u64))
                .with_property("field_count", PropValue::from(2u64))
                .with_property("interface_count", PropValue::from(0u64))
                .with_property("has_file_io", PropValue::from(false))
                .with_property("has_sql", PropValue::from(false))
                .with_property("has_rules", PropValue::from(false)),
        );

        g
    }

    #[test]
    fn fingerprint_generation() {
        let mut g = make_graph();
        PatternPass.enrich(&mut g).unwrap();

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let fp1 = g.node(clrg1).unwrap().properties.get_str("fingerprint").unwrap().to_owned();

        let clrg2 = g.lookup_one(NodeKind::Program, "CLRG0200").unwrap();
        let fp2 = g.node(clrg2).unwrap().properties.get_str("fingerprint").unwrap().to_owned();

        // Same shape -> same fingerprint
        assert_eq!(fp1, fp2);

        // Different shape -> different fingerprint
        let db = g.lookup_one(NodeKind::Program, "DBPROG").unwrap();
        let fp_db = g.node(db).unwrap().properties.get_str("fingerprint").unwrap();
        assert_ne!(fp1.as_str(), fp_db);
    }

    #[test]
    fn similar_programs_detected() {
        let mut g = make_graph();
        PatternPass.enrich(&mut g).unwrap();

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        if let Some(PropValue::List(similar)) =
            g.node(clrg1).unwrap().properties.get("similar_programs")
        {
            assert_eq!(similar, &["CLRG0200".to_owned()]);
        } else {
            panic!("expected similar_programs list");
        }

        // DBPROG has no similar programs
        let db = g.lookup_one(NodeKind::Program, "DBPROG").unwrap();
        assert_eq!(g.node(db).unwrap().properties.get("similar_programs"), None);
    }

    #[test]
    fn pattern_classification() {
        let mut g = make_graph();
        PatternPass.enrich(&mut g).unwrap();

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        assert_eq!(
            g.node(clrg1).unwrap().properties.get_str("pattern_class"),
            Some("batch-read-process-write")
        );

        let db = g.lookup_one(NodeKind::Program, "DBPROG").unwrap();
        assert_eq!(
            g.node(db).unwrap().properties.get_str("pattern_class"),
            Some("db-crud")
        );

        let util = g.lookup_one(NodeKind::Program, "UTIL01").unwrap();
        assert_eq!(
            g.node(util).unwrap().properties.get_str("pattern_class"),
            Some("utility")
        );
    }

    #[test]
    fn era_classification() {
        let mut g = make_graph();
        PatternPass.enrich(&mut g).unwrap();

        let clrg1 = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        assert_eq!(
            g.node(clrg1).unwrap().properties.get_str("era"),
            Some("structured") // >5 paragraphs, no SQL
        );

        let db = g.lookup_one(NodeKind::Program, "DBPROG").unwrap();
        assert_eq!(
            g.node(db).unwrap().properties.get_str("era"),
            Some("modern") // has SQL
        );

        let util = g.lookup_one(NodeKind::Program, "UTIL01").unwrap();
        assert_eq!(
            g.node(util).unwrap().properties.get_str("era"),
            Some("simple") // <= 5 paragraphs, no SQL
        );
    }

    #[test]
    fn bucket_function() {
        assert_eq!(bucket(0), "0");
        assert_eq!(bucket(1), "S");
        assert_eq!(bucket(3), "S");
        assert_eq!(bucket(4), "M");
        assert_eq!(bucket(10), "M");
        assert_eq!(bucket(11), "L");
        assert_eq!(bucket(30), "L");
        assert_eq!(bucket(31), "X");
        assert_eq!(bucket(100), "X");
    }
}
