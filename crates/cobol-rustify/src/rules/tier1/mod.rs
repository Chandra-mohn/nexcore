//! Tier 1 rules: Cosmetic Cleanup.
//!
//! Fully automatic, zero semantic risk. Can be applied without review.
//! Rules: const-extract, zero-literal, dead-field, allow-cleanup,
//! unused-import, display-simplify.

pub mod allow_cleanup;
pub mod const_extract;
pub mod dead_field;
pub mod display_simplify;
pub mod unused_import;
pub mod zero_literal;

use super::RustifyRule;

/// Create all Tier 1 rule instances.
pub fn tier1_rules() -> Vec<Box<dyn RustifyRule>> {
    vec![
        Box::new(zero_literal::ZeroLiteralRule),
        Box::new(const_extract::ConstExtractRule),
        Box::new(dead_field::DeadFieldRule),
        Box::new(unused_import::UnusedImportRule),
        Box::new(allow_cleanup::AllowCleanupRule),
        Box::new(display_simplify::DisplaySimplifyRule),
    ]
}
