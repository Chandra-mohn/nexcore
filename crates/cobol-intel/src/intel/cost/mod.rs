pub mod config;
pub mod expr;

use serde_json::{json, Value as JsonValue};

use crate::error::IntelResult;
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeId, NodeKind, PropValue};
use crate::graph::CodeGraph;

use self::config::CostConfig;
use self::expr::Bindings;

use super::{EnrichStats, IntelligencePass};

/// Layer 11: Migration Cost Estimation.
///
/// Computes per-program:
/// - Multipliers: code, copybook, parameter, rule_engine, total
/// - Manual effort: analysis, rewrite, data_mapping, qa_testing, integration,
///   regression, cobol_sme (days)
/// - Nex effort: review, qa_testing, data_mapping, integration (days)
/// - Cost: manual_cost, nex_cost, savings, savings_pct
///
/// Aggregation (via estimate-cost verb): process, wave, portfolio.
#[derive(Debug)]
pub struct CostEstimatePass {
    config: CostConfig,
}

impl CostEstimatePass {
    pub fn new() -> Self {
        Self {
            config: CostConfig::default(),
        }
    }

    pub fn with_config(config: CostConfig) -> Self {
        Self { config }
    }
}

impl Default for CostEstimatePass {
    fn default() -> Self {
        Self::new()
    }
}

impl IntelligencePass for CostEstimatePass {
    fn name(&self) -> &'static str {
        "cost_estimate"
    }

    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats> {
        let mut stats = EnrichStats::default();
        let program_ids = graph.all_of_kind(NodeKind::Program);
        let config_bindings = self.config.config_bindings();

        for prog_id in &program_ids {
            let estimate = compute_program_estimate(graph, *prog_id, &self.config, &config_bindings);

            if let Some(node) = graph.node_mut(*prog_id) {
                // Multipliers
                node.properties.set("code_multiplier", PropValue::from(estimate.code_multiplier));
                node.properties.set("copybook_multiplier", PropValue::from(estimate.copybook_multiplier));
                node.properties.set("parameter_multiplier", PropValue::from(estimate.parameter_multiplier));
                node.properties.set("rule_engine_multiplier", PropValue::from(estimate.rule_engine_multiplier));
                node.properties.set("total_multiplier", PropValue::from(estimate.total_multiplier));

                // Manual effort (days)
                node.properties.set("manual_days", PropValue::from(estimate.manual_total));
                node.properties.set("nex_days", PropValue::from(estimate.nex_total));

                // Cost
                node.properties.set("manual_cost", PropValue::from(estimate.manual_cost));
                node.properties.set("nex_cost", PropValue::from(estimate.nex_cost));
                node.properties.set("savings", PropValue::from(estimate.savings));
                node.properties.set("savings_pct", PropValue::from(estimate.savings_pct));

                stats.nodes_enriched += 1;
                stats.properties_added += 11;
            }
        }

        Ok(stats)
    }
}

/// Per-program cost estimate result.
#[derive(Debug, Clone)]
pub struct ProgramEstimate {
    pub name: String,
    pub code_multiplier: f64,
    pub copybook_multiplier: f64,
    pub parameter_multiplier: f64,
    pub rule_engine_multiplier: f64,
    pub total_multiplier: f64,
    // Manual breakdown (days)
    pub manual_analysis: f64,
    pub manual_rewrite: f64,
    pub manual_data_mapping: f64,
    pub manual_qa_testing: f64,
    pub manual_integration: f64,
    pub manual_regression: f64,
    pub manual_cobol_sme: f64,
    pub manual_total: f64,
    // Nex breakdown (days)
    pub nex_automated: f64,
    pub nex_review: f64,
    pub nex_qa_testing: f64,
    pub nex_data_mapping: f64,
    pub nex_integration: f64,
    pub nex_total: f64,
    // Cost
    pub manual_cost: f64,
    pub nex_cost: f64,
    pub savings: f64,
    pub savings_pct: f64,
}

/// Compute cost estimate for a single program.
fn compute_program_estimate(
    graph: &CodeGraph,
    prog_id: NodeId,
    config: &CostConfig,
    config_bindings: &Bindings,
) -> ProgramEstimate {
    let node = graph.node(prog_id);
    let name = node.map(|n| n.name.clone()).unwrap_or_default();

    // Check for manual override
    if let Some(ovr) = config.overrides.programs.get(&name) {
        if let Some(manual_days) = ovr.manual_days_override {
            return make_override_estimate(&name, manual_days, config);
        }
    }

    // Extract properties from graph
    let get_f64 = |key: &str| -> f64 {
        node.and_then(|n| n.properties.get_f64(key)).unwrap_or(0.0)
    };
    let get_u64 = |key: &str| -> u64 {
        node.and_then(|n| n.properties.get_u64(key)).unwrap_or(0)
    };
    let get_bool = |key: &str| -> bool {
        node.and_then(|n| n.properties.get_bool(key)).unwrap_or(false)
    };

    let complexity = get_f64("complexity");
    let loc = get_u64("field_count") as f64 * 10.0 + get_u64("paragraph_count") as f64 * 20.0;
    // Use a minimum LOC of 50 for stub programs
    let loc = loc.max(50.0);
    let interface_count = get_u64("interface_count") as f64;
    let inherited_cb = get_f64("inherited_copybook_complexity");
    let param_table_count = get_u64("parameter_table_count") as f64;
    let rule_table_count = get_u64("rule_table_count") as f64;
    let has_dynamic_perform = get_bool("has_dynamic_perform");

    // Count process boundary crossings: number of distinct processes that call this program
    let callers = graph.neighbors_incoming(prog_id, EdgeKind::Calls);
    let mut caller_processes = std::collections::HashSet::new();
    for caller_id in &callers {
        if let Some(caller_node) = graph.node(*caller_id) {
            if let Some(proc_name) = caller_node.properties.get_str("process") {
                caller_processes.insert(proc_name.to_owned());
            }
        }
    }
    let process_boundary_crossings = caller_processes.len().max(1) as f64;

    // Compute multipliers (unbounded)
    let code_multiplier = 1.0 + (complexity / 5.0);
    let copybook_multiplier = (1.0 + inherited_cb / 20.0).max(1.0);
    let parameter_multiplier = if param_table_count > 0.0 {
        (1.0 + param_table_count * 0.3).max(1.0)
    } else {
        1.0
    };
    let rule_engine_multiplier = if rule_table_count > 0.0 || has_dynamic_perform {
        let base = 1.0 + rule_table_count * 2.0;
        let dynamic = if has_dynamic_perform { 3.0 } else { 0.0 };
        (base + dynamic).max(1.0)
    } else {
        1.0
    };
    let total_multiplier = code_multiplier
        * copybook_multiplier
        * parameter_multiplier
        * rule_engine_multiplier;

    // Build per-program bindings
    let mut bindings = config_bindings.clone();
    bindings.insert("loc".to_owned(), loc);
    bindings.insert("complexity".to_owned(), complexity);
    bindings.insert("code_multiplier".to_owned(), code_multiplier);
    bindings.insert("copybook_multiplier".to_owned(), copybook_multiplier);
    bindings.insert("parameter_multiplier".to_owned(), parameter_multiplier);
    bindings.insert("rule_engine_multiplier".to_owned(), rule_engine_multiplier);
    bindings.insert("total_multiplier".to_owned(), total_multiplier);
    bindings.insert("interface_count".to_owned(), interface_count);
    bindings.insert("field_count".to_owned(), get_u64("field_count") as f64);
    bindings.insert("paragraph_count".to_owned(), get_u64("paragraph_count") as f64);
    bindings.insert("call_count".to_owned(), get_u64("call_count") as f64);
    bindings.insert("data_complexity".to_owned(), get_f64("data_complexity"));
    bindings.insert("process_boundary_crossings".to_owned(), process_boundary_crossings);

    // Evaluate manual formulas
    let manual_analysis = eval_or_zero(&config.formulas.manual.analysis, &bindings);
    let manual_rewrite = eval_or_zero(&config.formulas.manual.rewrite, &bindings);
    let manual_data_mapping = eval_or_zero(&config.formulas.manual.data_mapping, &bindings);

    // qa_testing references manual.rewrite
    bindings.insert("manual.rewrite".to_owned(), manual_rewrite);
    let manual_qa_testing = eval_or_zero(&config.formulas.manual.qa_testing, &bindings);

    let manual_integration = eval_or_zero(&config.formulas.manual.integration, &bindings);
    let manual_regression = eval_or_zero(&config.formulas.manual.regression, &bindings);

    // cobol_sme references manual.total (sum of above, before sme)
    let manual_subtotal = manual_analysis + manual_rewrite + manual_data_mapping
        + manual_qa_testing + manual_integration + manual_regression;
    bindings.insert("manual.total".to_owned(), manual_subtotal);
    let manual_cobol_sme = eval_or_zero(&config.formulas.manual.cobol_sme, &bindings);
    let manual_total = manual_subtotal + manual_cobol_sme;

    // Make manual breakdown available to nex formulas
    bindings.insert("manual.analysis".to_owned(), manual_analysis);
    bindings.insert("manual.data_mapping".to_owned(), manual_data_mapping);
    bindings.insert("manual.qa_testing".to_owned(), manual_qa_testing);
    bindings.insert("manual.integration".to_owned(), manual_integration);
    bindings.insert("manual.regression".to_owned(), manual_regression);

    // Evaluate nex formulas
    let nex_automated = eval_or_zero(&config.formulas.nex.automated, &bindings);
    let nex_review = eval_or_zero(&config.formulas.nex.review, &bindings);
    let nex_qa_testing = eval_or_zero(&config.formulas.nex.qa_testing, &bindings);
    let nex_data_mapping = eval_or_zero(&config.formulas.nex.data_mapping, &bindings);
    let nex_integration = eval_or_zero(&config.formulas.nex.integration, &bindings);
    let nex_total = nex_automated + nex_review + nex_qa_testing + nex_data_mapping + nex_integration;

    // Cost
    let manual_rate = config.blended_manual_rate();
    let nex_rate = config.blended_nex_rate();
    let manual_cost = round2(manual_total * manual_rate);
    let nex_cost = round2(nex_total * nex_rate);
    let savings = round2(manual_cost - nex_cost);
    let savings_pct = if manual_cost > 0.0 {
        round1(savings / manual_cost * 100.0)
    } else {
        0.0
    };

    ProgramEstimate {
        name,
        code_multiplier: round2(code_multiplier),
        copybook_multiplier: round2(copybook_multiplier),
        parameter_multiplier: round2(parameter_multiplier),
        rule_engine_multiplier: round2(rule_engine_multiplier),
        total_multiplier: round2(total_multiplier),
        manual_analysis: round1(manual_analysis),
        manual_rewrite: round1(manual_rewrite),
        manual_data_mapping: round1(manual_data_mapping),
        manual_qa_testing: round1(manual_qa_testing),
        manual_integration: round1(manual_integration),
        manual_regression: round1(manual_regression),
        manual_cobol_sme: round1(manual_cobol_sme),
        manual_total: round1(manual_total),
        nex_automated: round1(nex_automated),
        nex_review: round1(nex_review),
        nex_qa_testing: round1(nex_qa_testing),
        nex_data_mapping: round1(nex_data_mapping),
        nex_integration: round1(nex_integration),
        nex_total: round1(nex_total),
        manual_cost,
        nex_cost,
        savings,
        savings_pct,
    }
}

/// Build an estimate from a manual override (human-estimated days).
fn make_override_estimate(name: &str, manual_days: f64, config: &CostConfig) -> ProgramEstimate {
    let manual_cost = round2(manual_days * config.blended_manual_rate());
    // Estimate Nex at 30% of manual for overridden programs
    let nex_days = round1(manual_days * 0.3);
    let nex_cost = round2(nex_days * config.blended_nex_rate());
    let savings = round2(manual_cost - nex_cost);
    let savings_pct = if manual_cost > 0.0 {
        round1(savings / manual_cost * 100.0)
    } else {
        0.0
    };

    ProgramEstimate {
        name: name.to_owned(),
        code_multiplier: 1.0,
        copybook_multiplier: 1.0,
        parameter_multiplier: 1.0,
        rule_engine_multiplier: 1.0,
        total_multiplier: 1.0,
        manual_analysis: 0.0,
        manual_rewrite: 0.0,
        manual_data_mapping: 0.0,
        manual_qa_testing: 0.0,
        manual_integration: 0.0,
        manual_regression: 0.0,
        manual_cobol_sme: 0.0,
        manual_total: round1(manual_days),
        nex_automated: 0.0,
        nex_review: 0.0,
        nex_qa_testing: 0.0,
        nex_data_mapping: 0.0,
        nex_integration: 0.0,
        nex_total: nex_days,
        manual_cost,
        nex_cost,
        savings,
        savings_pct,
    }
}

fn eval_or_zero(formula: &str, bindings: &Bindings) -> f64 {
    expr::eval(formula, bindings).unwrap_or(0.0)
}

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

// --- Portfolio aggregation (used by estimate-cost verb) ---

/// Aggregate cost estimates across programs, grouped by scope.
pub fn aggregate_estimates(
    graph: &CodeGraph,
    scope: &str,      // "all", "process", "wave"
    scope_value: Option<&str>, // process name or wave number
    config: &CostConfig,
) -> JsonValue {
    let program_ids = graph.all_of_kind(NodeKind::Program);

    // Filter programs by scope
    let filtered: Vec<NodeId> = program_ids
        .into_iter()
        .filter(|prog_id| {
            let node = match graph.node(*prog_id) {
                Some(n) => n,
                None => return false,
            };
            match scope {
                "all" => true,
                "process" => {
                    if let Some(target) = scope_value {
                        node.properties.get_str("process") == Some(target)
                    } else {
                        true
                    }
                }
                "wave" => {
                    if let Some(target) = scope_value {
                        if let Ok(wave_num) = target.parse::<u64>() {
                            node.properties.get_u64("migration_wave") == Some(wave_num)
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                }
                _ => true,
            }
        })
        .collect();

    let total_programs = filtered.len();
    let mut total_loc = 0u64;
    let mut manual_total_days = 0.0f64;
    let mut nex_total_days = 0.0f64;
    let mut manual_total_cost = 0.0f64;
    let mut nex_total_cost = 0.0f64;
    let mut risk_factors: Vec<String> = Vec::new();

    // Collect per-program data
    let mut program_details: Vec<JsonValue> = Vec::new();

    for prog_id in &filtered {
        if let Some(node) = graph.node(*prog_id) {
            // Skip placeholder programs (unresolved CALL targets)
            if node.name == "UNKNOWN" || node.name.contains(" IS ") {
                continue;
            }

            let loc_est = node.properties.get_u64("field_count").unwrap_or(0) * 10
                + node.properties.get_u64("paragraph_count").unwrap_or(0) * 20;
            total_loc += loc_est.max(50);

            let m_days = node.properties.get_f64("manual_days").unwrap_or(0.0);
            let n_days = node.properties.get_f64("nex_days").unwrap_or(0.0);
            let m_cost = node.properties.get_f64("manual_cost").unwrap_or(0.0);
            let n_cost = node.properties.get_f64("nex_cost").unwrap_or(0.0);

            manual_total_days += m_days;
            nex_total_days += n_days;
            manual_total_cost += m_cost;
            nex_total_cost += n_cost;

            // Collect risk signals
            let dep_class = node.properties.get_str("external_dependency_class").unwrap_or("self-contained");
            if dep_class == "rule-driven" || dep_class == "hybrid" {
                let mult = node.properties.get_f64("total_multiplier").unwrap_or(1.0);
                risk_factors.push(format!(
                    "{} classified as {} ({:.1}x multiplier)",
                    node.name, dep_class.to_uppercase(), mult
                ));
            }
            if dep_class == "parameter-driven" {
                let param_count = node.properties.get_u64("parameter_table_count").unwrap_or(0);
                if param_count >= 3 {
                    risk_factors.push(format!(
                        "{}: {} parameter tables detected",
                        node.name, param_count
                    ));
                }
            }

            program_details.push(json!({
                "name": node.name,
                "manual_days": m_days,
                "nex_days": n_days,
                "manual_cost": m_cost,
                "nex_cost": n_cost,
                "savings_pct": node.properties.get_f64("savings_pct").unwrap_or(0.0),
                "total_multiplier": node.properties.get_f64("total_multiplier").unwrap_or(1.0),
                "dep_class": dep_class,
                "complexity": node.properties.get_f64("complexity").unwrap_or(0.0),
                "process": node.properties.get_str("process").unwrap_or(""),
            }));
        }
    }

    // Sort programs by manual_cost descending
    program_details.sort_by(|a, b| {
        let ca = a["manual_cost"].as_f64().unwrap_or(0.0);
        let cb = b["manual_cost"].as_f64().unwrap_or(0.0);
        cb.partial_cmp(&ca).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Check for monster copybooks
    let copybook_ids = graph.all_of_kind(NodeKind::Copybook);
    for cb_id in &copybook_ids {
        if let Some(cb_node) = graph.node(*cb_id) {
            if cb_node.properties.get_bool("is_monster").unwrap_or(false) {
                let users = graph.neighbors_incoming(*cb_id, EdgeKind::Uses).len();
                let complexity = cb_node.properties.get_f64("copybook_complexity").unwrap_or(0.0);
                risk_factors.push(format!(
                    "Monster copybook {} (complexity {:.0}) used by {} programs",
                    cb_node.name, complexity, users
                ));
            }
        }
    }

    let savings_total = round2(manual_total_cost - nex_total_cost);
    let savings_pct = if manual_total_cost > 0.0 {
        round1(savings_total / manual_total_cost * 100.0)
    } else {
        0.0
    };

    // Duration estimates (assuming team sizes)
    let manual_team_size = (total_programs as f64 / 5.0).ceil().max(1.0) as u64;
    let nex_team_size = manual_team_size; // same team, faster
    let manual_months = if manual_team_size > 0 {
        (manual_total_days / (manual_team_size as f64 * 20.0)).ceil() as u64
    } else {
        0
    };
    let nex_months = if nex_team_size > 0 {
        (nex_total_days / (nex_team_size as f64 * 20.0)).ceil() as u64
    } else {
        0
    };

    let actual_program_count = program_details.len();

    json!({
        "result_type": "cost_estimate",
        "scope": scope,
        "scope_value": scope_value,
        "total_programs": actual_program_count,
        "total_loc": total_loc,
        "currency": config.day_rates.currency,
        "manual": {
            "total_days": round1(manual_total_days),
            "total_cost": round2(manual_total_cost),
            "duration_months": manual_months,
            "team_size": manual_team_size,
        },
        "nex": {
            "total_days": round1(nex_total_days),
            "total_cost": round2(nex_total_cost),
            "duration_months": nex_months,
            "team_size": nex_team_size,
        },
        "savings": {
            "days": round1(manual_total_days - nex_total_days),
            "cost": savings_total,
            "percent": savings_pct,
            "time_reduction_months": manual_months.saturating_sub(nex_months),
        },
        "risk_factors": risk_factors,
        "programs": program_details,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    fn make_cost_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        // Program with moderate complexity
        let prog1 = g.add_node(
            Node::new(NodeKind::Program, "CLRG0100")
                .with_property("complexity", PropValue::from(4.0f64))
                .with_property("field_count", PropValue::from(50u64))
                .with_property("paragraph_count", PropValue::from(10u64))
                .with_property("interface_count", PropValue::from(3u64))
                .with_property("call_count", PropValue::from(2u64))
                .with_property("data_complexity", PropValue::from(2.5f64))
                .with_property("inherited_copybook_complexity", PropValue::from(10.0f64))
                .with_property("parameter_table_count", PropValue::from(0u64))
                .with_property("rule_table_count", PropValue::from(0u64))
                .with_property("process", PropValue::from("CLRG"))
                .with_property("migration_wave", PropValue::from(0u64)),
        );

        // Parameter-driven program
        let _prog2 = g.add_node(
            Node::new(NodeKind::Program, "CFGR0100")
                .with_property("complexity", PropValue::from(6.0f64))
                .with_property("field_count", PropValue::from(80u64))
                .with_property("paragraph_count", PropValue::from(15u64))
                .with_property("interface_count", PropValue::from(5u64))
                .with_property("call_count", PropValue::from(1u64))
                .with_property("data_complexity", PropValue::from(5.0f64))
                .with_property("inherited_copybook_complexity", PropValue::from(40.0f64))
                .with_property("parameter_table_count", PropValue::from(3u64))
                .with_property("rule_table_count", PropValue::from(0u64))
                .with_property("external_dependency_class", PropValue::from("parameter-driven"))
                .with_property("process", PropValue::from("CFGR"))
                .with_property("migration_wave", PropValue::from(1u64)),
        );

        // Simple utility
        let _prog3 = g.add_node(
            Node::new(NodeKind::Program, "DATEUTIL")
                .with_property("complexity", PropValue::from(1.0f64))
                .with_property("field_count", PropValue::from(5u64))
                .with_property("paragraph_count", PropValue::from(2u64))
                .with_property("interface_count", PropValue::from(0u64))
                .with_property("call_count", PropValue::from(0u64))
                .with_property("data_complexity", PropValue::from(0.0f64))
                .with_property("parameter_table_count", PropValue::from(0u64))
                .with_property("rule_table_count", PropValue::from(0u64))
                .with_property("process", PropValue::from("CLRG"))
                .with_property("migration_wave", PropValue::from(0u64)),
        );

        // CLRG0100 calls DATEUTIL
        let dateutil_id = g.lookup_one(NodeKind::Program, "DATEUTIL").unwrap();
        g.add_edge(prog1, dateutil_id, Edge::new(EdgeKind::Calls));

        g
    }

    #[test]
    fn enrichment_sets_multipliers() {
        let mut g = make_cost_graph();
        let pass = CostEstimatePass::new();
        let stats = pass.enrich(&mut g).unwrap();
        assert_eq!(stats.nodes_enriched, 3);

        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let node = g.node(clrg).unwrap();

        let code_mult = node.properties.get_f64("code_multiplier").unwrap();
        assert!(code_mult > 1.0, "code_multiplier should be > 1.0, got {code_mult}");

        let total_mult = node.properties.get_f64("total_multiplier").unwrap();
        assert!(total_mult >= code_mult, "total should be >= code mult");
    }

    #[test]
    fn parameter_driven_higher_multiplier() {
        let mut g = make_cost_graph();
        let pass = CostEstimatePass::new();
        pass.enrich(&mut g).unwrap();

        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let cfgr = g.lookup_one(NodeKind::Program, "CFGR0100").unwrap();

        let clrg_mult = g.node(clrg).unwrap().properties.get_f64("total_multiplier").unwrap();
        let cfgr_mult = g.node(cfgr).unwrap().properties.get_f64("total_multiplier").unwrap();

        assert!(
            cfgr_mult > clrg_mult,
            "parameter-driven should have higher multiplier: cfgr={cfgr_mult} vs clrg={clrg_mult}"
        );
    }

    #[test]
    fn manual_more_than_nex() {
        let mut g = make_cost_graph();
        let pass = CostEstimatePass::new();
        pass.enrich(&mut g).unwrap();

        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let node = g.node(clrg).unwrap();

        let manual = node.properties.get_f64("manual_cost").unwrap();
        let nex = node.properties.get_f64("nex_cost").unwrap();
        let savings = node.properties.get_f64("savings").unwrap();
        let pct = node.properties.get_f64("savings_pct").unwrap();

        assert!(manual > 0.0, "manual_cost should be > 0");
        assert!(nex > 0.0, "nex_cost should be > 0");
        assert!(manual > nex, "manual should exceed nex: {manual} vs {nex}");
        assert!(savings > 0.0, "savings should be > 0");
        assert!(pct > 0.0 && pct < 100.0, "savings_pct should be 0-100, got {pct}");
    }

    #[test]
    fn utility_lowest_cost() {
        let mut g = make_cost_graph();
        let pass = CostEstimatePass::new();
        pass.enrich(&mut g).unwrap();

        let dateutil = g.lookup_one(NodeKind::Program, "DATEUTIL").unwrap();
        let clrg = g.lookup_one(NodeKind::Program, "CLRG0100").unwrap();

        let util_cost = g.node(dateutil).unwrap().properties.get_f64("manual_cost").unwrap();
        let clrg_cost = g.node(clrg).unwrap().properties.get_f64("manual_cost").unwrap();

        assert!(
            util_cost < clrg_cost,
            "utility should cost less: {util_cost} vs {clrg_cost}"
        );
    }

    #[test]
    fn aggregate_all() {
        let mut g = make_cost_graph();
        let pass = CostEstimatePass::new();
        pass.enrich(&mut g).unwrap();

        let config = CostConfig::default();
        let result = aggregate_estimates(&g, "all", None, &config);

        assert_eq!(result["total_programs"], 3);
        assert!(result["manual"]["total_cost"].as_f64().unwrap() > 0.0);
        assert!(result["nex"]["total_cost"].as_f64().unwrap() > 0.0);
        assert!(result["savings"]["percent"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn aggregate_by_process() {
        let mut g = make_cost_graph();
        let pass = CostEstimatePass::new();
        pass.enrich(&mut g).unwrap();

        let config = CostConfig::default();
        let clrg = aggregate_estimates(&g, "process", Some("CLRG"), &config);
        let cfgr = aggregate_estimates(&g, "process", Some("CFGR"), &config);

        assert_eq!(clrg["total_programs"], 2); // CLRG0100 + DATEUTIL
        assert_eq!(cfgr["total_programs"], 1); // CFGR0100
    }

    #[test]
    fn aggregate_by_wave() {
        let mut g = make_cost_graph();
        let pass = CostEstimatePass::new();
        pass.enrich(&mut g).unwrap();

        let config = CostConfig::default();
        let wave0 = aggregate_estimates(&g, "wave", Some("0"), &config);
        let wave1 = aggregate_estimates(&g, "wave", Some("1"), &config);

        assert_eq!(wave0["total_programs"], 2); // CLRG0100 + DATEUTIL
        assert_eq!(wave1["total_programs"], 1); // CFGR0100
    }

    #[test]
    fn manual_override() {
        let mut g = CodeGraph::new();
        g.add_node(
            Node::new(NodeKind::Program, "MONSTER")
                .with_property("complexity", PropValue::from(8.0f64))
                .with_property("field_count", PropValue::from(200u64))
                .with_property("paragraph_count", PropValue::from(50u64)),
        );

        let mut config = CostConfig::default();
        config.overrides.programs.insert(
            "MONSTER".to_owned(),
            config::ProgramOverride {
                manual_days_override: Some(90.0),
                note: Some("Manually estimated monster".to_owned()),
            },
        );

        let pass = CostEstimatePass::with_config(config);
        pass.enrich(&mut g).unwrap();

        let monster = g.lookup_one(NodeKind::Program, "MONSTER").unwrap();
        let node = g.node(monster).unwrap();
        assert_eq!(node.properties.get_f64("manual_days"), Some(90.0));
        assert!(node.properties.get_f64("manual_cost").unwrap() > 0.0);
    }

    #[test]
    fn empty_graph() {
        let mut g = CodeGraph::new();
        let pass = CostEstimatePass::new();
        let stats = pass.enrich(&mut g).unwrap();
        assert_eq!(stats.nodes_enriched, 0);
    }

    #[test]
    fn savings_pct_reasonable() {
        let mut g = make_cost_graph();
        let pass = CostEstimatePass::new();
        pass.enrich(&mut g).unwrap();

        for prog_id in g.all_of_kind(NodeKind::Program) {
            let node = g.node(prog_id).unwrap();
            let pct = node.properties.get_f64("savings_pct").unwrap_or(0.0);
            assert!(
                pct >= 0.0 && pct <= 100.0,
                "savings_pct for {} should be 0-100, got {pct}",
                node.name
            );
        }
    }
}
