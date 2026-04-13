//! Direct DSL emission: reads COBOL AST directly, bypassing Phase 1 Rust.
//!
//! This module provides a parallel emission path that reads the COBOL AST
//! from the transpiler instead of re-parsing Phase 1 Rust via `syn`.
//! Both paths produce identical `Vec<DslFile>` output.

pub mod cobol_extract;
pub mod condition_extract;
pub mod process_emitter;
pub mod rules_emitter;
pub mod schema_emitter;
pub mod schema_fields;
pub mod schema_keys;
pub mod schema_occurs;
pub mod schema_pattern;
pub mod schema_redefines;
pub mod schema_sql;
pub mod transform_emitter;

use std::path::PathBuf;

use cobol_transpiler::ast::CobolProgram;

use super::DslFile;
use crate::hints::FileHints;
use crate::rules::transform::Transform;
use crate::target_config::TargetConfig;

/// Context for direct DSL emitters (reads COBOL AST, not syn).
pub struct DirectEmitterContext<'a> {
    /// The parsed COBOL program AST.
    pub cobol_program: &'a CobolProgram,
    /// COBOL program name (from PROGRAM-ID).
    pub program_name: String,
    /// Optional COBOL semantic hints.
    pub hints: Option<&'a FileHints>,
    /// Tier 3 assessment results.
    pub assessments: &'a [Transform],
    /// Optional target architecture config.
    pub target: Option<&'a TargetConfig>,
    /// Source file path.
    pub source_path: PathBuf,
}

impl std::fmt::Debug for DirectEmitterContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectEmitterContext")
            .field("program_name", &self.program_name)
            .field("source_path", &self.source_path)
            .finish_non_exhaustive()
    }
}

/// Trait for direct DSL emitters (mirror of `DslEmitter` but reads COBOL AST).
pub trait DirectDslEmitter: Send + Sync {
    /// Unique identifier for this emitter.
    fn id(&self) -> &'static str;

    /// Which DSL layer this emitter targets.
    fn layer(&self) -> super::DslLayer;

    /// Generate DSL files from the COBOL AST context.
    fn emit(&self, ctx: &DirectEmitterContext<'_>) -> Vec<DslFile>;
}

/// Run all available direct emitters and collect results.
pub fn emit_all_dsl_direct(ctx: &DirectEmitterContext<'_>) -> Vec<DslFile> {
    let emitters: Vec<Box<dyn DirectDslEmitter>> = vec![
        Box::new(schema_emitter::DirectSchemaEmitter),
        Box::new(transform_emitter::DirectTransformEmitter),
        Box::new(rules_emitter::DirectRulesEmitter),
        Box::new(process_emitter::DirectProcessEmitter),
    ];

    let mut all_files = Vec::new();
    for emitter in &emitters {
        all_files.extend(emitter.emit(ctx));
    }
    all_files
}
