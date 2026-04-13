//! Tier 4: Structural transformation rules.
//!
//! These rules perform workspace-level structural transforms consuming the
//! target architecture config (`.cobol2rust-target.toml`) and Tier 3
//! assessment results. Unlike Tiers 1-3 (per-file), Tier 4 operates across
//! the entire workspace: creating, modifying, and removing files.

pub mod structural;

mod dispatch;
mod domain;
mod error_type;
mod io_backend;

use structural::StructuralRule;

/// Return all Tier 4 structural rules in execution order.
///
/// Order matters: each rule's output feeds the next.
/// 1. Dispatch elimination (restructures control flow)
/// 2. Domain modeling (restructures data)
/// 3. Error types (adds error hierarchy)
/// 4. I/O backend (modernizes file access)
pub fn tier4_rules() -> Vec<Box<dyn StructuralRule>> {
    vec![
        Box::new(dispatch::DispatchEliminationRule),
        Box::new(domain::DomainModelRule),
        Box::new(error_type::ErrorTypeRule),
        Box::new(io_backend::IoBackendRule),
    ]
}
