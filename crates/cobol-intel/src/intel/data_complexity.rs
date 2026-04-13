use std::collections::HashMap;

use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeId, NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::{EnrichStats, IntelligencePass};

/// Table name substrings indicating parameter/config tables.
const PARAM_TABLE_PATTERNS: &[&str] = &[
    "PARM", "CONFIG", "CONTROL", "SETUP", "OPTION", "SETTING", "PARAMETER",
];

/// Table name substrings indicating rule/logic tables.
const RULE_TABLE_PATTERNS: &[&str] = &[
    "RULE", "LOGIC", "FORMULA", "CALC", "DECISION", "CRITERIA", "CONDITION",
];

/// Field name prefixes indicating parameter sensitivity.
const PARAM_FIELD_PREFIXES: &[&str] = &[
    "WS-PARM",
    "WS-CONFIG",
    "WS-OPTION",
    "WS-PARAM",
    "WS-SETTING",
    "WS-CONTROL",
    "WS-SETUP",
];

/// Field name prefixes indicating rule engine fields.
const RULE_FIELD_PREFIXES: &[&str] = &[
    "WS-RULE",
    "WS-FORMULA",
    "WS-CALC",
    "WS-DECISION",
    "WS-CRITERIA",
];

/// File name substrings indicating parameter/config VSAM files.
const PARAM_FILE_PATTERNS: &[&str] = &[
    "PARM", "CONFIG", "CONTROL", "SETUP", "OPTION", "SETTING", "PARAMETER",
];

/// File name substrings indicating rule/logic VSAM files.
const RULE_FILE_PATTERNS: &[&str] = &[
    "RULE", "LOGIC", "FORMULA", "CALC", "DECISION", "CRITERIA", "CONDITION",
];

/// External dependency classification for a program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DepClass {
    SelfContained,
    ParameterDriven,
    RuleDriven,
    Hybrid,
}

impl DepClass {
    fn as_str(self) -> &'static str {
        match self {
            Self::SelfContained => "self-contained",
            Self::ParameterDriven => "parameter-driven",
            Self::RuleDriven => "rule-driven",
            Self::Hybrid => "hybrid",
        }
    }
}

/// Layer 10: Data Complexity Impact.
///
/// Detects hidden complexity from external data dependencies:
/// - Parameter/config tables (DB2 + VSAM)
/// - Rule/logic tables (code-as-data patterns)
/// - Monster copybooks (high field count, REDEFINES, OCCURS)
///
/// Computes per-copybook:
/// - `copybook_complexity`: composite score (unbounded, 50+ = monster)
/// - `is_monster`: true if complexity > 50
///
/// Computes per-program:
/// - `parameter_table_count`: detected param/config tables accessed
/// - `parameter_sensitivity_score`: normalized 0-10
/// - `rule_table_count`: detected rule/logic tables accessed
/// - `rule_engine_score`: normalized based on rule signals
/// - `has_dynamic_perform`: true if dynamic PERFORM detected
/// - `inherited_copybook_complexity`: sum from COPY'd copybooks
/// - `external_dependency_class`: self-contained / parameter-driven / rule-driven / hybrid
/// - `data_complexity`: composite 0-10 score
/// - `data_sensitivity`: list of detected signal descriptions
#[derive(Debug)]
pub struct DataComplexityPass;

impl IntelligencePass for DataComplexityPass {
    fn name(&self) -> &'static str {
        "data_complexity"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();

        // Phase 1: Enrich copybook nodes with complexity scores
        enrich_copybooks(graph, &mut stats);

        // Phase 2: Enrich program nodes with data complexity
        enrich_programs(graph, &mut stats);

        Ok(stats)
    }
}

/// Compute and set copybook_complexity on each Copybook node.
///
/// Uses metadata stored by `GraphBuilder::add_copybook_metadata()` if available.
/// Properties used: `cb_field_count`, `cb_redefines_depth`, `cb_occurs_count`,
/// `cb_record_length`.
fn enrich_copybooks(graph: &mut CodeGraph, stats: &mut EnrichStats) {
    let copybook_ids = graph.all_of_kind(NodeKind::Copybook);

    for cb_id in copybook_ids {
        let (field_count, redefines_depth, occurs_count, record_length) = {
            let node = match graph.node(cb_id) {
                Some(n) => n,
                None => continue,
            };
            (
                node.properties.get_u64("cb_field_count").unwrap_or(0),
                node.properties.get_u64("cb_redefines_depth").unwrap_or(0),
                node.properties.get_u64("cb_occurs_count").unwrap_or(0),
                node.properties.get_u64("cb_record_length").unwrap_or(0),
            )
        };

        // Only compute if metadata is available (field_count > 0)
        if field_count == 0 {
            // No metadata -- estimate from programs that use this copybook.
            // Count fields in programs using this copybook and average.
            let users = graph.neighbors_incoming(cb_id, EdgeKind::Uses);
            if users.is_empty() {
                continue;
            }

            // Use program field stats as a rough proxy
            let mut total_fields = 0u64;
            let mut total_redefines = 0u64;
            let mut total_arrays = 0u64;
            let mut program_count = 0u64;

            for prog_id in &users {
                let prog_name = graph
                    .node(*prog_id)
                    .map(|n| n.name.clone())
                    .unwrap_or_default();
                let fields = graph.nodes_in_program(&prog_name);
                let mut fcount = 0u64;
                let mut rcount = 0u64;
                let mut acount = 0u64;
                for fid in fields {
                    if let Some(fnode) = graph.node(*fid) {
                        if fnode.kind == NodeKind::Field {
                            fcount += 1;
                            if fnode.properties.get_bool("redefines").unwrap_or(false) {
                                rcount += 1;
                            }
                            if fnode.properties.get_bool("is_array").unwrap_or(false) {
                                acount += 1;
                            }
                        }
                    }
                }
                total_fields += fcount;
                total_redefines += rcount;
                total_arrays += acount;
                program_count += 1;
            }

            if program_count > 0 {
                // Distribute evenly across copybooks used by these programs
                // (rough approximation)
                let avg_fields = total_fields / program_count;
                let avg_redefines = total_redefines / program_count;
                let avg_arrays = total_arrays / program_count;

                let complexity = compute_copybook_complexity(
                    avg_fields,
                    avg_redefines,
                    avg_arrays,
                    0, // no record length info
                );

                if let Some(node) = graph.node_mut(cb_id) {
                    node.properties
                        .set("copybook_complexity", PropValue::from(complexity));
                    node.properties
                        .set("is_monster", PropValue::from(complexity > 50.0));
                    stats.nodes_enriched += 1;
                    stats.properties_added += 2;
                }
            }
            continue;
        }

        let complexity = compute_copybook_complexity(
            field_count,
            redefines_depth,
            occurs_count,
            record_length,
        );

        if let Some(node) = graph.node_mut(cb_id) {
            node.properties
                .set("copybook_complexity", PropValue::from(complexity));
            node.properties
                .set("is_monster", PropValue::from(complexity > 50.0));
            stats.nodes_enriched += 1;
            stats.properties_added += 2;
        }
    }
}

/// Copybook complexity formula (unbounded):
/// `(field_count / 100) * (1 + redefines_depth * 0.5) * (1 + occurs * 0.2) * (1 + record_length / 4096)`
fn compute_copybook_complexity(
    field_count: u64,
    redefines_depth: u64,
    occurs_count: u64,
    record_length: u64,
) -> f64 {
    let base = field_count as f64 / 100.0;
    let redefines_mult = 1.0 + redefines_depth as f64 * 0.5;
    let occurs_mult = 1.0 + occurs_count as f64 * 0.2;
    let record_mult = if record_length > 0 {
        1.0 + record_length as f64 / 4096.0
    } else {
        1.0
    };

    let raw = base * redefines_mult * occurs_mult * record_mult;
    // Round to 1 decimal
    (raw * 10.0).round() / 10.0
}

/// Enrich programs with parameter/rule detection, classification, and composite score.
fn enrich_programs(graph: &mut CodeGraph, stats: &mut EnrichStats) {
    let program_ids = graph.all_of_kind(NodeKind::Program);

    // Pre-collect table and file classifications
    let table_classes = classify_tables(graph);
    let file_classes = classify_files(graph);

    // Pre-collect copybook complexity scores
    let copybook_complexities = collect_copybook_complexities(graph);

    for prog_id in &program_ids {
        let mut signals: Vec<String> = Vec::new();

        // 1. Parameter table detection (DB2)
        let accessed = graph.neighbors_outgoing(*prog_id, EdgeKind::Accesses);
        let mut param_table_count = 0u64;
        let mut rule_table_count = 0u64;

        for target_id in &accessed {
            if let Some(target) = graph.node(*target_id) {
                if target.kind == NodeKind::Table {
                    if let Some(class) = table_classes.get(target_id) {
                        match *class {
                            TableClass::Parameter => {
                                param_table_count += 1;
                                signals.push(format!("param-table: {}", target.name));
                            }
                            TableClass::Rule => {
                                rule_table_count += 1;
                                signals.push(format!("rule-table: {}", target.name));
                            }
                            TableClass::Normal => {}
                        }
                    }
                } else if target.kind == NodeKind::File {
                    if let Some(class) = file_classes.get(target_id) {
                        match *class {
                            FileClass::Parameter => {
                                param_table_count += 1;
                                signals.push(format!("param-file: {}", target.name));
                            }
                            FileClass::Rule => {
                                rule_table_count += 1;
                                signals.push(format!("rule-file: {}", target.name));
                            }
                            FileClass::Normal => {}
                        }
                    }
                }
            }
        }

        // 2. Parameter/rule field name detection
        let prog_name = graph
            .node(*prog_id)
            .map(|n| n.name.clone())
            .unwrap_or_default();
        let field_ids = graph.nodes_in_program(&prog_name);
        let mut param_field_count = 0u64;
        let mut rule_field_count = 0u64;
        let mut redefines_count = 0u64;
        let mut array_count = 0u64;

        for fid in field_ids {
            if let Some(fnode) = graph.node(*fid) {
                if fnode.kind != NodeKind::Field {
                    continue;
                }
                let name_upper = fnode.name.to_uppercase();
                if PARAM_FIELD_PREFIXES
                    .iter()
                    .any(|p| name_upper.starts_with(p))
                {
                    param_field_count += 1;
                }
                if RULE_FIELD_PREFIXES
                    .iter()
                    .any(|p| name_upper.starts_with(p))
                {
                    rule_field_count += 1;
                }
                if fnode.properties.get_bool("redefines").unwrap_or(false) {
                    redefines_count += 1;
                }
                if fnode.properties.get_bool("is_array").unwrap_or(false) {
                    array_count += 1;
                }
            }
        }

        // 3. Dynamic PERFORM detection (from properties set by builder)
        let has_dynamic_perform = graph
            .node(*prog_id)
            .and_then(|n| n.properties.get_bool("has_dynamic_perform"))
            .unwrap_or(false);
        if has_dynamic_perform {
            signals.push("dynamic-perform".to_owned());
        }

        // 4. Large EVALUATE detection (from properties set by builder)
        let evaluate_branch_max = graph
            .node(*prog_id)
            .and_then(|n| n.properties.get_u64("evaluate_branch_max"))
            .unwrap_or(0);
        if evaluate_branch_max > 10 {
            signals.push(format!("large-evaluate: {evaluate_branch_max} branches"));
        }

        // 5. Copybook complexity propagation
        let copybooks_used = graph.neighbors_outgoing(*prog_id, EdgeKind::Uses);
        let mut inherited_complexity = 0.0f64;
        for cb_id in &copybooks_used {
            if let Some(&complexity) = copybook_complexities.get(cb_id) {
                inherited_complexity += complexity;
                if complexity > 50.0 {
                    let cb_name = graph
                        .node(*cb_id)
                        .map(|n| n.name.clone())
                        .unwrap_or_default();
                    signals.push(format!("monster-copybook: {cb_name}"));
                }
            }
        }

        // 6. Compute scores
        let parameter_sensitivity_score = compute_param_score(
            param_table_count,
            param_field_count,
        );

        let rule_engine_score = compute_rule_score(
            rule_table_count,
            rule_field_count,
            has_dynamic_perform,
            evaluate_branch_max,
        );

        // 7. Classification
        let dep_class = classify_dependency(parameter_sensitivity_score, rule_engine_score);

        // 8. Composite data_complexity (0-10)
        let data_complexity = compute_data_complexity(
            inherited_complexity,
            parameter_sensitivity_score,
            rule_engine_score,
            redefines_count,
            array_count,
        );

        // 9. Apply properties
        if let Some(node) = graph.node_mut(*prog_id) {
            node.properties
                .set("parameter_table_count", PropValue::from(param_table_count));
            node.properties.set(
                "parameter_sensitivity_score",
                PropValue::from(parameter_sensitivity_score),
            );
            node.properties
                .set("rule_table_count", PropValue::from(rule_table_count));
            node.properties
                .set("rule_engine_score", PropValue::from(rule_engine_score));
            node.properties
                .set("has_dynamic_perform", PropValue::from(has_dynamic_perform));
            node.properties.set(
                "inherited_copybook_complexity",
                PropValue::from(inherited_complexity),
            );
            node.properties.set(
                "external_dependency_class",
                PropValue::from(dep_class.as_str()),
            );
            node.properties
                .set("data_complexity", PropValue::from(data_complexity));

            if !signals.is_empty() {
                node.properties
                    .set("data_sensitivity", PropValue::from(signals));
            }

            stats.nodes_enriched += 1;
            stats.properties_added += 8;
            if graph
                .node(*prog_id)
                .and_then(|n| n.properties.get("data_sensitivity"))
                .is_some()
            {
                stats.properties_added += 1;
            }
        }
    }
}

/// Parameter sensitivity score (0-10).
///
/// Based on param table count + param field count.
fn compute_param_score(param_table_count: u64, param_field_count: u64) -> f64 {
    if param_table_count == 0 && param_field_count == 0 {
        return 0.0;
    }
    let table_score = (param_table_count as f64 * 2.0).min(6.0);
    let field_score = (param_field_count as f64 * 0.5).min(4.0);
    let raw = table_score + field_score;
    (raw.min(10.0) * 10.0).round() / 10.0
}

/// Rule engine score (0-10).
///
/// Based on rule table count + rule fields + dynamic PERFORM + large EVALUATE.
fn compute_rule_score(
    rule_table_count: u64,
    rule_field_count: u64,
    has_dynamic_perform: bool,
    evaluate_branch_max: u64,
) -> f64 {
    if rule_table_count == 0 && rule_field_count == 0 && !has_dynamic_perform {
        return 0.0;
    }
    let table_score = (rule_table_count as f64 * 2.5).min(5.0);
    let field_score = (rule_field_count as f64 * 0.5).min(2.0);
    let dynamic_score = if has_dynamic_perform { 2.0 } else { 0.0 };
    let evaluate_score = if evaluate_branch_max > 10 {
        1.0
    } else {
        0.0
    };
    let raw = table_score + field_score + dynamic_score + evaluate_score;
    (raw.min(10.0) * 10.0).round() / 10.0
}

/// Classify external dependency type.
fn classify_dependency(param_score: f64, rule_score: f64) -> DepClass {
    let has_param = param_score > 0.0;
    let has_rule = rule_score > 0.0;
    match (has_param, has_rule) {
        (true, true) => DepClass::Hybrid,
        (false, true) => DepClass::RuleDriven,
        (true, false) => DepClass::ParameterDriven,
        (false, false) => DepClass::SelfContained,
    }
}

/// Composite data complexity score (0.0 - 10.0).
fn compute_data_complexity(
    inherited_copybook_complexity: f64,
    parameter_sensitivity_score: f64,
    rule_engine_score: f64,
    redefines_count: u64,
    array_count: u64,
) -> f64 {
    // Copybook complexity: normalize -- 40 points of inherited = 2.5 score points
    let cb_component = (inherited_copybook_complexity / 40.0).min(2.5);

    // Parameter sensitivity: contributes up to 2.5 points
    let param_component = parameter_sensitivity_score * 0.25;

    // Rule engine: contributes up to 2.5 points
    let rule_component = rule_engine_score * 0.25;

    // Data structure complexity: REDEFINES and arrays add up to 2.5 points
    let redefines_score = (redefines_count as f64 * 0.2).min(1.5);
    let array_score = (array_count as f64 * 0.1).min(1.0);
    let struct_component = redefines_score + array_score;

    let raw = cb_component + param_component + rule_component + struct_component;
    (raw.min(10.0) * 10.0).round() / 10.0
}

// --- Table / File classification helpers ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TableClass {
    Parameter,
    Rule,
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileClass {
    Parameter,
    Rule,
    Normal,
}

fn classify_tables(graph: &CodeGraph) -> HashMap<NodeId, TableClass> {
    let table_ids = graph.all_of_kind(NodeKind::Table);
    let mut result = HashMap::new();
    for tid in table_ids {
        let name = graph.node(tid).map(|n| n.name.as_str()).unwrap_or("");
        let upper = name.to_uppercase();
        if RULE_TABLE_PATTERNS.iter().any(|p| upper.contains(p)) {
            result.insert(tid, TableClass::Rule);
        } else if PARAM_TABLE_PATTERNS.iter().any(|p| upper.contains(p)) {
            result.insert(tid, TableClass::Parameter);
        } else {
            result.insert(tid, TableClass::Normal);
        }
    }
    result
}

fn classify_files(graph: &CodeGraph) -> HashMap<NodeId, FileClass> {
    let file_ids = graph.all_of_kind(NodeKind::File);
    let mut result = HashMap::new();
    for fid in file_ids {
        let name = graph.node(fid).map(|n| n.name.as_str()).unwrap_or("");
        let upper = name.to_uppercase();
        if RULE_FILE_PATTERNS.iter().any(|p| upper.contains(p)) {
            result.insert(fid, FileClass::Rule);
        } else if PARAM_FILE_PATTERNS.iter().any(|p| upper.contains(p)) {
            result.insert(fid, FileClass::Parameter);
        } else {
            result.insert(fid, FileClass::Normal);
        }
    }
    result
}

fn collect_copybook_complexities(graph: &CodeGraph) -> HashMap<NodeId, f64> {
    let cb_ids = graph.all_of_kind(NodeKind::Copybook);
    let mut result = HashMap::new();
    for cb_id in cb_ids {
        if let Some(complexity) = graph.node(cb_id).and_then(|n| n.properties.get_f64("copybook_complexity")) {
            result.insert(cb_id, complexity);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    // --- Helper: build a graph with parameter-driven program ---
    fn make_param_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        let prog = g.add_node(Node::new(NodeKind::Program, "PARAM-PROG"));
        let config_tbl = g.add_node(Node::new(NodeKind::Table, "SYS_CONFIG_TABLE"));
        let setup_tbl = g.add_node(Node::new(NodeKind::Table, "APP_SETUP"));
        let normal_tbl = g.add_node(Node::new(NodeKind::Table, "ACCT_MASTER"));

        g.add_edge(prog, config_tbl, Edge::new(EdgeKind::Accesses));
        g.add_edge(prog, setup_tbl, Edge::new(EdgeKind::Accesses));
        g.add_edge(prog, normal_tbl, Edge::new(EdgeKind::Accesses));

        // Add param fields
        g.add_node(
            Node::new(NodeKind::Field, "WS-CONFIG-FLAG").with_program("PARAM-PROG"),
        );
        g.add_node(
            Node::new(NodeKind::Field, "WS-PARM-CODE").with_program("PARAM-PROG"),
        );
        g.add_node(
            Node::new(NodeKind::Field, "WS-ACCT-NUM").with_program("PARAM-PROG"),
        );

        g
    }

    // --- Helper: build a graph with rule-driven program ---
    fn make_rule_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        let prog = g.add_node(
            Node::new(NodeKind::Program, "RULE-ENGINE")
                .with_property("has_dynamic_perform", PropValue::from(true))
                .with_property("evaluate_branch_max", PropValue::from(15u64)),
        );
        let rule_tbl = g.add_node(Node::new(NodeKind::Table, "BUSINESS_RULES"));
        let decision_tbl = g.add_node(Node::new(NodeKind::Table, "DECISION_MATRIX"));
        let normal_tbl = g.add_node(Node::new(NodeKind::Table, "TXN_LOG"));

        g.add_edge(prog, rule_tbl, Edge::new(EdgeKind::Accesses));
        g.add_edge(prog, decision_tbl, Edge::new(EdgeKind::Accesses));
        g.add_edge(prog, normal_tbl, Edge::new(EdgeKind::Accesses));

        // Rule fields
        g.add_node(
            Node::new(NodeKind::Field, "WS-RULE-CODE").with_program("RULE-ENGINE"),
        );
        g.add_node(
            Node::new(NodeKind::Field, "WS-FORMULA-ID").with_program("RULE-ENGINE"),
        );

        g
    }

    // --- Helper: build a graph with hybrid program ---
    fn make_hybrid_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        let prog = g.add_node(
            Node::new(NodeKind::Program, "HYBRID-PROG")
                .with_property("has_dynamic_perform", PropValue::from(true)),
        );
        let config_tbl = g.add_node(Node::new(NodeKind::Table, "APP_CONFIG"));
        let rule_tbl = g.add_node(Node::new(NodeKind::Table, "CALC_RULES"));

        g.add_edge(prog, config_tbl, Edge::new(EdgeKind::Accesses));
        g.add_edge(prog, rule_tbl, Edge::new(EdgeKind::Accesses));

        g.add_node(
            Node::new(NodeKind::Field, "WS-PARM-TYPE").with_program("HYBRID-PROG"),
        );
        g.add_node(
            Node::new(NodeKind::Field, "WS-RULE-FLAG").with_program("HYBRID-PROG"),
        );

        g
    }

    // --- Helper: graph with copybook metadata ---
    fn make_copybook_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        let prog = g.add_node(Node::new(NodeKind::Program, "BIG-PROG"));

        // Monster copybook with metadata
        let monster_cb = g.add_node(
            Node::new(NodeKind::Copybook, "CPYMONSTER")
                .with_property("cb_field_count", PropValue::from(8000u64))
                .with_property("cb_redefines_depth", PropValue::from(8u64))
                .with_property("cb_occurs_count", PropValue::from(5u64))
                .with_property("cb_record_length", PropValue::from(32768u64)),
        );

        // Normal copybook with metadata
        let normal_cb = g.add_node(
            Node::new(NodeKind::Copybook, "CPYNORMAL")
                .with_property("cb_field_count", PropValue::from(20u64))
                .with_property("cb_redefines_depth", PropValue::from(0u64))
                .with_property("cb_occurs_count", PropValue::from(1u64))
                .with_property("cb_record_length", PropValue::from(256u64)),
        );

        g.add_edge(prog, monster_cb, Edge::new(EdgeKind::Uses));
        g.add_edge(prog, normal_cb, Edge::new(EdgeKind::Uses));

        g
    }

    // --- Helper: self-contained program ---
    fn make_self_contained_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        let _prog = g.add_node(Node::new(NodeKind::Program, "SIMPLE-PROG"));
        let normal_tbl = g.add_node(Node::new(NodeKind::Table, "ACCT_MASTER"));
        let prog = g.lookup_one(NodeKind::Program, "SIMPLE-PROG").unwrap();
        g.add_edge(prog, normal_tbl, Edge::new(EdgeKind::Accesses));

        g.add_node(
            Node::new(NodeKind::Field, "WS-ACCT-NUM").with_program("SIMPLE-PROG"),
        );
        g.add_node(
            Node::new(NodeKind::Field, "WS-BALANCE").with_program("SIMPLE-PROG"),
        );

        g
    }

    // --- Tests ---

    #[test]
    fn parameter_driven_detection() {
        let mut g = make_param_graph();
        let s = DataComplexityPass.enrich(&mut g).unwrap();
        assert!(s.nodes_enriched > 0);

        let prog = g.lookup_one(NodeKind::Program, "PARAM-PROG").unwrap();
        let node = g.node(prog).unwrap();

        assert_eq!(node.properties.get_u64("parameter_table_count"), Some(2));
        assert_eq!(node.properties.get_u64("rule_table_count"), Some(0));
        assert_eq!(
            node.properties.get_str("external_dependency_class"),
            Some("parameter-driven")
        );

        let param_score = node
            .properties
            .get_f64("parameter_sensitivity_score")
            .unwrap();
        assert!(param_score > 0.0, "param score should be > 0, got {param_score}");
    }

    #[test]
    fn rule_driven_detection() {
        let mut g = make_rule_graph();
        DataComplexityPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "RULE-ENGINE").unwrap();
        let node = g.node(prog).unwrap();

        assert_eq!(node.properties.get_u64("rule_table_count"), Some(2));
        assert_eq!(node.properties.get_bool("has_dynamic_perform"), Some(true));
        assert_eq!(
            node.properties.get_str("external_dependency_class"),
            Some("rule-driven")
        );

        let rule_score = node.properties.get_f64("rule_engine_score").unwrap();
        assert!(rule_score > 0.0, "rule score should be > 0, got {rule_score}");
    }

    #[test]
    fn hybrid_classification() {
        let mut g = make_hybrid_graph();
        DataComplexityPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "HYBRID-PROG").unwrap();
        let node = g.node(prog).unwrap();

        assert_eq!(
            node.properties.get_str("external_dependency_class"),
            Some("hybrid")
        );
        assert!(
            node.properties.get_u64("parameter_table_count").unwrap() > 0,
            "should have param tables"
        );
        assert!(
            node.properties.get_u64("rule_table_count").unwrap() > 0,
            "should have rule tables"
        );
    }

    #[test]
    fn self_contained_classification() {
        let mut g = make_self_contained_graph();
        DataComplexityPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "SIMPLE-PROG").unwrap();
        let node = g.node(prog).unwrap();

        assert_eq!(
            node.properties.get_str("external_dependency_class"),
            Some("self-contained")
        );
        assert_eq!(node.properties.get_u64("parameter_table_count"), Some(0));
        assert_eq!(node.properties.get_u64("rule_table_count"), Some(0));
        assert_eq!(
            node.properties.get_f64("parameter_sensitivity_score"),
            Some(0.0)
        );
        assert_eq!(node.properties.get_f64("rule_engine_score"), Some(0.0));
    }

    #[test]
    fn copybook_complexity_with_metadata() {
        let mut g = make_copybook_graph();
        DataComplexityPass.enrich(&mut g).unwrap();

        // Monster copybook
        let monster = g.lookup_one(NodeKind::Copybook, "CPYMONSTER").unwrap();
        let node = g.node(monster).unwrap();
        let complexity = node.properties.get_f64("copybook_complexity").unwrap();
        assert!(
            complexity > 50.0,
            "monster copybook complexity should be > 50, got {complexity}"
        );
        assert_eq!(node.properties.get_bool("is_monster"), Some(true));

        // Normal copybook
        let normal = g.lookup_one(NodeKind::Copybook, "CPYNORMAL").unwrap();
        let nnode = g.node(normal).unwrap();
        let ncomplexity = nnode.properties.get_f64("copybook_complexity").unwrap();
        assert!(
            ncomplexity < 50.0,
            "normal copybook complexity should be < 50, got {ncomplexity}"
        );
        assert_eq!(nnode.properties.get_bool("is_monster"), Some(false));
    }

    #[test]
    fn inherited_copybook_complexity_propagation() {
        let mut g = make_copybook_graph();
        DataComplexityPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "BIG-PROG").unwrap();
        let node = g.node(prog).unwrap();
        let inherited = node
            .properties
            .get_f64("inherited_copybook_complexity")
            .unwrap();
        assert!(
            inherited > 0.0,
            "inherited complexity should be > 0, got {inherited}"
        );
    }

    #[test]
    fn data_sensitivity_signals() {
        let mut g = make_param_graph();
        DataComplexityPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "PARAM-PROG").unwrap();
        let node = g.node(prog).unwrap();

        if let Some(PropValue::List(signals)) = node.properties.get("data_sensitivity") {
            assert!(
                signals.iter().any(|s| s.contains("param-table")),
                "should have param-table signal, got: {signals:?}"
            );
        } else {
            panic!("expected data_sensitivity list");
        }
    }

    #[test]
    fn data_complexity_composite_score() {
        let mut g = make_rule_graph();
        DataComplexityPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "RULE-ENGINE").unwrap();
        let node = g.node(prog).unwrap();
        let dc = node.properties.get_f64("data_complexity").unwrap();
        assert!(dc >= 0.0, "data_complexity should be >= 0, got {dc}");
        assert!(dc <= 10.0, "data_complexity should be <= 10, got {dc}");
        assert!(
            dc > 0.0,
            "rule-driven program should have non-zero data_complexity"
        );
    }

    #[test]
    fn empty_graph() {
        let mut g = CodeGraph::new();
        let s = DataComplexityPass.enrich(&mut g).unwrap();
        assert_eq!(s.nodes_enriched, 0);
    }

    #[test]
    fn compute_copybook_complexity_formula() {
        // 8000 fields, 8 REDEFINES, 5 OCCURS, 32K record
        let c = compute_copybook_complexity(8000, 8, 5, 32768);
        // base=80, redefines_mult=5.0, occurs_mult=2.0, record_mult=9.0
        // = 80 * 5.0 * 2.0 * 9.0 = 7200
        assert!(c > 100.0, "monster complexity should be > 100, got {c}");

        // 20 fields, no redefines, 1 occurs, 256 bytes
        let n = compute_copybook_complexity(20, 0, 1, 256);
        assert!(n < 50.0, "normal complexity should be < 50, got {n}");

        // Zero fields
        let z = compute_copybook_complexity(0, 0, 0, 0);
        assert_eq!(z, 0.0);
    }

    #[test]
    fn param_score_computation() {
        assert_eq!(compute_param_score(0, 0), 0.0);

        let s1 = compute_param_score(2, 4);
        assert!(s1 > 0.0 && s1 <= 10.0, "param score out of range: {s1}");

        let smax = compute_param_score(10, 20);
        assert_eq!(smax, 10.0, "max param score should cap at 10");
    }

    #[test]
    fn rule_score_computation() {
        assert_eq!(compute_rule_score(0, 0, false, 0), 0.0);

        let s1 = compute_rule_score(2, 3, true, 15);
        assert!(s1 > 0.0 && s1 <= 10.0, "rule score out of range: {s1}");

        let smax = compute_rule_score(5, 10, true, 20);
        assert_eq!(smax, 10.0, "max rule score should cap at 10");
    }

    #[test]
    fn dependency_classification_logic() {
        assert_eq!(classify_dependency(0.0, 0.0), DepClass::SelfContained);
        assert_eq!(classify_dependency(3.0, 0.0), DepClass::ParameterDriven);
        assert_eq!(classify_dependency(0.0, 5.0), DepClass::RuleDriven);
        assert_eq!(classify_dependency(3.0, 5.0), DepClass::Hybrid);
    }

    #[test]
    fn data_complexity_score_bounds() {
        // Zero everything
        assert_eq!(compute_data_complexity(0.0, 0.0, 0.0, 0, 0), 0.0);

        // Max everything
        let max = compute_data_complexity(200.0, 10.0, 10.0, 50, 50);
        assert_eq!(max, 10.0);

        // Moderate
        let mid = compute_data_complexity(40.0, 5.0, 3.0, 5, 3);
        assert!(mid > 0.0 && mid < 10.0, "mid complexity out of range: {mid}");
    }

    #[test]
    fn vsam_param_file_detection() {
        let mut g = CodeGraph::new();

        let prog = g.add_node(Node::new(NodeKind::Program, "FILE-PROG"));
        let param_file = g.add_node(Node::new(NodeKind::File, "PARM-CONTROL-FILE"));
        let normal_file = g.add_node(Node::new(NodeKind::File, "ACCT-MASTER"));

        g.add_edge(prog, param_file, Edge::new(EdgeKind::Accesses));
        g.add_edge(prog, normal_file, Edge::new(EdgeKind::Accesses));

        DataComplexityPass.enrich(&mut g).unwrap();

        let prog_id = g.lookup_one(NodeKind::Program, "FILE-PROG").unwrap();
        let node = g.node(prog_id).unwrap();

        // PARM-CONTROL-FILE matches both PARM and CONTROL patterns -> counted once as param
        assert!(
            node.properties.get_u64("parameter_table_count").unwrap() >= 1,
            "should detect VSAM param file"
        );
        assert_eq!(
            node.properties.get_str("external_dependency_class"),
            Some("parameter-driven")
        );
    }

    #[test]
    fn vsam_rule_file_detection() {
        let mut g = CodeGraph::new();

        let prog = g.add_node(Node::new(NodeKind::Program, "VRULE-PROG"));
        let rule_file = g.add_node(Node::new(NodeKind::File, "BUSINESS-RULES-FILE"));

        g.add_edge(prog, rule_file, Edge::new(EdgeKind::Accesses));

        DataComplexityPass.enrich(&mut g).unwrap();

        let prog_id = g.lookup_one(NodeKind::Program, "VRULE-PROG").unwrap();
        let node = g.node(prog_id).unwrap();

        assert!(
            node.properties.get_u64("rule_table_count").unwrap() >= 1,
            "should detect VSAM rule file"
        );
        assert_eq!(
            node.properties.get_str("external_dependency_class"),
            Some("rule-driven")
        );
    }

    #[test]
    fn monster_copybook_signal_in_sensitivity() {
        let mut g = make_copybook_graph();
        DataComplexityPass.enrich(&mut g).unwrap();

        let prog = g.lookup_one(NodeKind::Program, "BIG-PROG").unwrap();
        let node = g.node(prog).unwrap();

        if let Some(PropValue::List(signals)) = node.properties.get("data_sensitivity") {
            assert!(
                signals.iter().any(|s| s.contains("monster-copybook")),
                "should have monster-copybook signal, got: {signals:?}"
            );
        } else {
            panic!("expected data_sensitivity list with monster-copybook signal");
        }
    }

    #[test]
    fn run_all_includes_data_complexity() {
        use crate::intel::run_all;

        let mut g = make_param_graph();
        let results = run_all(&mut g).unwrap();
        let names: Vec<&str> = results.iter().map(|(n, _)| *n).collect();
        assert!(
            names.contains(&"data_complexity"),
            "run_all should include data_complexity pass"
        );
    }
}
