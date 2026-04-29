//! Nexflow DSL generation from Phase 1 transpiled Rust.
//!
//! Emitters read `#[cobol(...)]` attributes and Rust AST via `syn`,
//! combined with Tier 3 assessments, to produce grammar-valid DSL
//! that the Nexflow compiler accepts.

pub mod cobol_attrs;
#[cfg(feature = "direct-emit")]
pub mod direct;
pub mod dsl_ast;
pub mod expr_extract;
#[cfg(test)]
mod grammar_validation_test;
#[cfg(test)]
mod integration_test;
pub mod process_emitter;
pub mod rules_emitter;
pub mod schema_emitter;
pub mod transform_emitter;
pub mod type_mapping;
pub mod writer;

use std::fmt;
use std::path::PathBuf;

use crate::hints::FileHints;
use crate::rules::transform::Transform;
use crate::target_config::TargetConfig;

/// Which Nexflow DSL layer an emitter targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DslLayer {
    /// L2 - SchemaDSL (.schema files)
    Schema,
    /// L3 - TransformDSL (.xform files)
    Transform,
    /// L4 - RulesDSL (.rules files)
    Rules,
    /// L1 - ProcDSL (.proc files)
    Process,
}

impl DslLayer {
    pub fn extension(&self) -> &'static str {
        match self {
            DslLayer::Schema => "schema",
            DslLayer::Transform => "xform",
            DslLayer::Rules => "rules",
            DslLayer::Process => "proc",
        }
    }
}

/// A generated DSL file.
#[derive(Debug, Clone)]
pub struct DslFile {
    /// Relative path, e.g. "schema/account_info.schema"
    pub path: String,
    /// Grammar-valid DSL text
    pub content: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Human-readable review notes for SME
    pub notes: Vec<String>,
    /// COBOL fields that contributed to this file
    pub source_fields: Vec<String>,
}

/// Context passed to all DSL emitters.
pub struct EmitterContext<'a> {
    /// COBOL program name (from `#[cobol(program = "...")]`)
    pub program_name: String,
    /// Parsed Rust AST
    pub syn_file: &'a syn::File,
    /// Raw source text
    pub source_text: &'a str,
    /// Optional COBOL semantic hints
    pub hints: Option<&'a FileHints>,
    /// Tier 3 assessment results
    pub assessments: &'a [Transform],
    /// Optional target architecture config
    pub target: Option<&'a TargetConfig>,
    /// Source file path
    pub source_path: PathBuf,
}

impl fmt::Debug for EmitterContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EmitterContext")
            .field("program_name", &self.program_name)
            .field("source_path", &self.source_path)
            .finish_non_exhaustive()
    }
}

/// Trait for all Nexflow DSL emitters.
pub trait DslEmitter: Send + Sync {
    /// Unique identifier for this emitter.
    fn id(&self) -> &'static str;

    /// Which DSL layer this emitter targets.
    fn layer(&self) -> DslLayer;

    /// Generate DSL files from the given context.
    fn emit(&self, ctx: &EmitterContext<'_>) -> Vec<DslFile>;
}
