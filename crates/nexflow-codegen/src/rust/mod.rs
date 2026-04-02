// NexCore -- Nexflow Codegen: Rust Target Generators
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Rust-native code generation from Nexflow DSL ASTs.
//!
//! Generates pure Rust functions -- no framework dependencies:
//! - `.xform` -> `pub fn transform_name(input) -> Output` (typed functions)
//! - `.rules` -> `pub fn evaluate_table(input) -> Option<Output>` (decision tables)
//! - `.rules` -> `pub fn apply_rule(input) -> HashMap` (procedural rules)

pub mod expression;
pub mod rules;
pub mod xform;
