//! Cost estimation configuration: day rates, productivity factors, Nex reduction
//! factors, formula definitions, and per-program overrides.
//!
//! Serialized to/from `nexintel/cost_config.json`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Complete cost estimation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostConfig {
    pub day_rates: DayRates,
    pub productivity: Productivity,
    pub nex_reduction: NexReduction,
    pub formulas: FormulaSet,
    #[serde(default)]
    pub overrides: CostOverrides,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayRates {
    pub target_engineer: f64,
    pub cobol_sme: f64,
    pub qa_engineer: f64,
    pub data_engineer: f64,
    pub integration_engineer: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Productivity {
    pub manual_loc_per_day: f64,
    pub review_loc_per_day: f64,
    pub qa_ratio: f64,
    pub regression_loc_per_day: f64,
    pub cobol_sme_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexReduction {
    pub qa_reduction: f64,
    pub data_mapping_reduction: f64,
    pub integration_reduction: f64,
    pub sme_reduction: f64,
}

/// Formula definitions for manual and Nex-assisted migration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaSet {
    pub manual: ManualFormulas,
    pub nex: NexFormulas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualFormulas {
    pub analysis: String,
    pub rewrite: String,
    pub data_mapping: String,
    pub qa_testing: String,
    pub integration: String,
    pub regression: String,
    pub cobol_sme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexFormulas {
    pub automated: String,
    pub review: String,
    pub qa_testing: String,
    pub data_mapping: String,
    pub integration: String,
}

/// Per-program overrides.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CostOverrides {
    #[serde(default)]
    pub programs: HashMap<String, ProgramOverride>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_days_override: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Default for CostConfig {
    fn default() -> Self {
        Self {
            day_rates: DayRates {
                target_engineer: 1200.0,
                cobol_sme: 800.0,
                qa_engineer: 900.0,
                data_engineer: 1100.0,
                integration_engineer: 1200.0,
                currency: "USD".to_owned(),
            },
            productivity: Productivity {
                manual_loc_per_day: 100.0,
                review_loc_per_day: 2000.0,
                qa_ratio: 1.5,
                regression_loc_per_day: 500.0,
                cobol_sme_percentage: 0.15,
            },
            nex_reduction: NexReduction {
                qa_reduction: 0.6,
                data_mapping_reduction: 0.5,
                integration_reduction: 0.7,
                sme_reduction: 0.33,
            },
            formulas: FormulaSet {
                manual: ManualFormulas {
                    analysis: "3.0 * code_multiplier".to_owned(),
                    rewrite: "loc / productivity.manual_loc_per_day * code_multiplier".to_owned(),
                    data_mapping: "interface_count * 2.0".to_owned(),
                    qa_testing: "manual.rewrite * productivity.qa_ratio".to_owned(),
                    integration: "process_boundary_crossings * 3.0".to_owned(),
                    regression: "loc / productivity.regression_loc_per_day".to_owned(),
                    cobol_sme: "manual.total * productivity.cobol_sme_percentage".to_owned(),
                },
                nex: NexFormulas {
                    automated: "0".to_owned(),
                    review: "loc / productivity.review_loc_per_day * code_multiplier".to_owned(),
                    qa_testing: "manual.qa_testing * (1 - nex_reduction.qa_reduction)".to_owned(),
                    data_mapping: "manual.data_mapping * (1 - nex_reduction.data_mapping_reduction)".to_owned(),
                    integration: "manual.integration * (1 - nex_reduction.integration_reduction)".to_owned(),
                },
            },
            overrides: CostOverrides::default(),
        }
    }
}

impl CostConfig {
    /// Build the set of config-level bindings (productivity, nex_reduction).
    /// These are constant across all programs.
    pub fn config_bindings(&self) -> super::expr::Bindings {
        let mut b = super::expr::Bindings::new();
        b.insert("productivity.manual_loc_per_day".to_owned(), self.productivity.manual_loc_per_day);
        b.insert("productivity.review_loc_per_day".to_owned(), self.productivity.review_loc_per_day);
        b.insert("productivity.qa_ratio".to_owned(), self.productivity.qa_ratio);
        b.insert("productivity.regression_loc_per_day".to_owned(), self.productivity.regression_loc_per_day);
        b.insert("productivity.cobol_sme_percentage".to_owned(), self.productivity.cobol_sme_percentage);
        b.insert("nex_reduction.qa_reduction".to_owned(), self.nex_reduction.qa_reduction);
        b.insert("nex_reduction.data_mapping_reduction".to_owned(), self.nex_reduction.data_mapping_reduction);
        b.insert("nex_reduction.integration_reduction".to_owned(), self.nex_reduction.integration_reduction);
        b.insert("nex_reduction.sme_reduction".to_owned(), self.nex_reduction.sme_reduction);
        b
    }

    /// Weighted average day rate (for converting days to cost).
    /// Uses target_engineer as the primary rate for code work.
    pub fn blended_manual_rate(&self) -> f64 {
        // Weighted: 40% target eng, 15% COBOL SME, 25% QA, 10% data, 10% integration
        self.day_rates.target_engineer * 0.40
            + self.day_rates.cobol_sme * 0.15
            + self.day_rates.qa_engineer * 0.25
            + self.day_rates.data_engineer * 0.10
            + self.day_rates.integration_engineer * 0.10
    }

    /// Blended rate for Nex-assisted (less COBOL SME time).
    pub fn blended_nex_rate(&self) -> f64 {
        self.day_rates.target_engineer * 0.45
            + self.day_rates.cobol_sme * 0.05
            + self.day_rates.qa_engineer * 0.30
            + self.day_rates.data_engineer * 0.10
            + self.day_rates.integration_engineer * 0.10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_serde_roundtrip() {
        let config = CostConfig::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: CostConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.day_rates.target_engineer, 1200.0);
        assert_eq!(parsed.day_rates.currency, "USD");
        assert_eq!(parsed.productivity.qa_ratio, 1.5);
        assert_eq!(parsed.nex_reduction.qa_reduction, 0.6);
        assert_eq!(parsed.formulas.manual.analysis, "3.0 * code_multiplier");
    }

    #[test]
    fn config_bindings() {
        let config = CostConfig::default();
        let bindings = config.config_bindings();
        assert_eq!(bindings["productivity.qa_ratio"], 1.5);
        assert_eq!(bindings["nex_reduction.qa_reduction"], 0.6);
    }

    #[test]
    fn blended_rates() {
        let config = CostConfig::default();
        let manual = config.blended_manual_rate();
        let nex = config.blended_nex_rate();
        assert!(manual > 0.0);
        assert!(nex > 0.0);
        // Nex rate slightly different due to less SME time
        assert!((manual - nex).abs() < 200.0);
    }

    #[test]
    fn config_with_overrides() {
        let mut config = CostConfig::default();
        config.overrides.programs.insert(
            "CLRG0100".to_owned(),
            ProgramOverride {
                manual_days_override: Some(45.0),
                note: Some("Known monster".to_owned()),
            },
        );
        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: CostConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed.overrides.programs["CLRG0100"].manual_days_override,
            Some(45.0)
        );
    }
}
