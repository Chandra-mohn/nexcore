//! Code generation module.
//!
//! Generates Rust and Java source code from the typed COBOL AST.

pub mod attributes;
pub mod data_gen;
pub mod field_analysis;
pub mod java;
pub mod proc_gen;
pub mod rust_writer;
pub mod screen_gen;
