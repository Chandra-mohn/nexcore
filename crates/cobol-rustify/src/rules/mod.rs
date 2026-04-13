//! Rule framework for rustification transforms.
//!
//! Each rule implements the `RustifyRule` trait and is registered
//! in the `RuleRegistry`. Rules are organized by tier.

pub mod tier1;
pub mod tier2;
pub mod tier3;
pub mod tier4;
pub mod transform;

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::hints::FileHints;
use transform::Transform;

/// Tier classification for rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Tier {
    /// Cosmetic cleanup (fully automatic, zero risk).
    Tier1 = 1,
    /// Type promotion (safety-gated, review.md for blocked items).
    Tier2 = 2,
    /// Structural assessment (plan.md, no auto-apply).
    Tier3 = 3,
    /// Structural transformation (config-driven, workspace-level).
    Tier4 = 4,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tier1 => write!(f, "Tier 1: Cosmetic Cleanup"),
            Self::Tier2 => write!(f, "Tier 2: Type Promotion"),
            Self::Tier3 => write!(f, "Tier 3: Structural Assessment"),
            Self::Tier4 => write!(f, "Tier 4: Structural Transformation"),
        }
    }
}

/// Context provided to rules during analysis.
#[allow(missing_debug_implementations)]
pub struct AnalysisContext<'a> {
    /// Parsed Rust source (syn AST).
    pub source: &'a syn::File,
    /// Raw source text.
    pub source_text: &'a str,
    /// Path to the file being analyzed.
    pub file_path: &'a Path,
    /// COBOL semantic hints (if available).
    pub hints: Option<&'a FileHints>,
}

/// A rustification rule that can analyze and propose transforms.
pub trait RustifyRule: Send + Sync {
    /// Unique rule identifier (e.g., "const-extract").
    fn id(&self) -> &'static str;

    /// Human-readable description.
    fn description(&self) -> &'static str;

    /// Which tier this rule belongs to.
    fn tier(&self) -> Tier;

    /// Analyze a file and return proposed transforms.
    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform>;
}

/// Information about a registered rule.
#[derive(Debug, Clone)]
pub struct RuleInfo {
    pub id: &'static str,
    pub description: &'static str,
    pub tier: Tier,
}

/// Registry of all available rustification rules.
#[allow(missing_debug_implementations)]
pub struct RuleRegistry {
    rules: Vec<Box<dyn RustifyRule>>,
}

impl RuleRegistry {
    /// Create a new registry with all built-in rules.
    pub fn new() -> Self {
        let mut rules: Vec<Box<dyn RustifyRule>> = Vec::new();
        rules.extend(tier1::tier1_rules());
        rules.extend(tier2::tier2_rules());
        rules.extend(tier3::tier3_rules());
        Self { rules }
    }

    /// Get all rules for a given tier.
    pub fn rules_for_tier(&self, tier: Tier) -> Vec<&dyn RustifyRule> {
        self.rules
            .iter()
            .filter(|r| r.tier() <= tier)
            .map(std::convert::AsRef::as_ref)
            .collect()
    }

    /// Get a rule by its ID.
    pub fn rule_by_id(&self, id: &str) -> Option<&dyn RustifyRule> {
        self.rules.iter().find(|r| r.id() == id).map(std::convert::AsRef::as_ref)
    }

    /// List all registered rules.
    pub fn list_rules(&self) -> Vec<RuleInfo> {
        self.rules
            .iter()
            .map(|r| RuleInfo {
                id: r.id(),
                description: r.description(),
                tier: r.tier(),
            })
            .collect()
    }

    /// Get rules filtered by config (only/skip lists).
    pub fn filtered_rules(
        &self,
        tier: Tier,
        only: &[String],
        skip: &[String],
    ) -> Vec<&dyn RustifyRule> {
        self.rules
            .iter()
            .filter(|r| r.tier() <= tier)
            .filter(|r| {
                if only.is_empty() {
                    true
                } else {
                    only.iter().any(|o| o == r.id())
                }
            })
            .filter(|r| !skip.iter().any(|s| s == r.id()))
            .map(std::convert::AsRef::as_ref)
            .collect()
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_rules() {
        let reg = RuleRegistry::new();
        assert!(!reg.rules_for_tier(Tier::Tier1).is_empty());
        assert!(!reg.list_rules().is_empty());
        assert!(reg.rule_by_id("zero-literal").is_some());
        assert!(reg.rule_by_id("const-extract").is_some());
        assert!(reg.rule_by_id("nonexistent").is_none());
    }

    #[test]
    fn tier_display() {
        assert_eq!(format!("{}", Tier::Tier1), "Tier 1: Cosmetic Cleanup");
        assert_eq!(format!("{}", Tier::Tier2), "Tier 2: Type Promotion");
        assert_eq!(format!("{}", Tier::Tier3), "Tier 3: Structural Assessment");
        assert_eq!(format!("{}", Tier::Tier4), "Tier 4: Structural Transformation");
    }

    #[test]
    fn tier_ordering() {
        assert!(Tier::Tier1 < Tier::Tier2);
        assert!(Tier::Tier2 < Tier::Tier3);
        assert!(Tier::Tier3 < Tier::Tier4);
    }
}
