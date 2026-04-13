//! Tier 3 rules: Structural Assessment.
//!
//! Assessment-only rules that produce plan.md. No auto-apply.
//! Rules: dispatcher-analysis, ws-decomposition, status-to-result,
//! io-modernize.

pub mod dispatcher_analysis;
pub mod io_modernize;
pub mod status_to_result;
pub mod ws_decomposition;

use super::RustifyRule;

pub fn tier3_rules() -> Vec<Box<dyn RustifyRule>> {
    vec![
        Box::new(dispatcher_analysis::DispatcherAnalysisRule),
        Box::new(ws_decomposition::WsDecompositionRule),
        Box::new(status_to_result::StatusToResultRule),
        Box::new(io_modernize::IoModernizeRule),
    ]
}
