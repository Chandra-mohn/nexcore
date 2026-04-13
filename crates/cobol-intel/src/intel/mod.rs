pub mod business;
pub mod control_flow;
pub mod cost;
pub mod data_complexity;
pub mod data_flow;
pub mod dependency;
pub mod external;
pub mod inter_program;
pub mod patterns;
pub mod process_discovery;
pub mod structural;

use crate::error::IntelResult;
use crate::graph::CodeGraph;

/// An intelligence pass enriches graph nodes with computed properties.
pub trait IntelligencePass {
    fn name(&self) -> &'static str;
    fn enrich(&self, graph: &mut CodeGraph) -> IntelResult<EnrichStats>;
}

/// Stats from running an intelligence pass.
#[derive(Debug, Default)]
pub struct EnrichStats {
    pub nodes_enriched: usize,
    pub properties_added: usize,
}

/// Run all intelligence passes in order.
/// Layers 1-11, each building on properties set by previous layers.
pub fn run_all(graph: &mut CodeGraph) -> IntelResult<Vec<(&'static str, EnrichStats)>> {
    run_all_with_config(graph, None)
}

/// Run all intelligence passes with optional cost configuration.
pub fn run_all_with_config(
    graph: &mut CodeGraph,
    cost_config: Option<cost::config::CostConfig>,
) -> IntelResult<Vec<(&'static str, EnrichStats)>> {
    let cost_pass = match cost_config {
        Some(cfg) => cost::CostEstimatePass::with_config(cfg),
        None => cost::CostEstimatePass::new(),
    };

    let passes: Vec<Box<dyn IntelligencePass>> = vec![
        Box::new(structural::StructuralPass),       // L1: counts, complexity, classification
        Box::new(control_flow::ControlFlowPass),     // L2: entry points, dead code, call depth
        Box::new(data_flow::DataFlowPass),           // L3: field access, data coupling
        Box::new(inter_program::InterProgramPass),   // L4: copybook coupling, clusters
        Box::new(external::ExternalPass),            // L5: file/table inventory, interfaces
        Box::new(business::BusinessPass),            // L6: rule counts, business density
        Box::new(dependency::DependencyPass),        // L7: impact radius, risk, migration waves
        Box::new(patterns::PatternPass),             // L8: fingerprints, pattern/era classification
        Box::new(process_discovery::ProcessDiscoveryPass), // L9: business process discovery
        Box::new(data_complexity::DataComplexityPass),       // L10: data complexity impact
        Box::new(cost_pass),                                 // L11: migration cost estimation
    ];

    let mut results = Vec::new();
    for pass in &passes {
        let stats = pass.enrich(graph)?;
        results.push((pass.name(), stats));
    }
    Ok(results)
}
