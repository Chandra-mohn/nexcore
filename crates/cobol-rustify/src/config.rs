//! Configuration for rustify operations.

use std::path::PathBuf;

use crate::rules::Tier;

/// DSL emission strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmitMode {
    /// Read from original COBOL AST directly (full fidelity, requires direct-emit feature).
    /// Named "legacy" because it reads the legacy COBOL source -- the source of truth.
    Legacy,
    /// Read from Phase 1 transpiled Rust/Java via syn.
    /// Named "transpiled" because it reads the transpiler's output.
    #[default]
    Transpiled,
    /// Run both paths, diff output, report discrepancies. Returns transpiled as production.
    Compare,
}

/// Per-emitter override: which emitters should use the legacy (COBOL AST) path.
#[derive(Debug, Clone, Default)]
pub struct EmitterOverrides {
    /// Emitter names that should use the legacy (COBOL AST) path (e.g., "schema", "transform").
    pub legacy: Vec<String>,
}

impl EmitterOverrides {
    /// Check whether a given emitter should use the legacy (COBOL AST) path.
    pub fn is_legacy(&self, emitter_name: &str) -> bool {
        self.legacy.iter().any(|e| e == emitter_name)
    }

    /// True if no per-emitter overrides are set.
    pub fn is_empty(&self) -> bool {
        self.legacy.is_empty()
    }
}

/// Configuration for a rustify run.
#[derive(Debug, Clone)]
pub struct RustifyConfig {
    /// Source directory (transpiled Rust workspace).
    pub source_dir: PathBuf,
    /// Output directory (if applying transforms).
    pub output_dir: Option<PathBuf>,
    /// Tier to apply.
    pub tier: Tier,
    /// Report format.
    pub report_format: ReportFormat,
    /// Show per-transform details.
    pub verbose: bool,
    /// File include glob pattern.
    pub include: Option<String>,
    /// File exclude glob pattern.
    pub exclude: Option<String>,
    /// Only apply these rule IDs (empty = all for tier).
    pub only_rules: Vec<String>,
    /// Skip these rule IDs.
    pub skip_rules: Vec<String>,
    /// Overwrite output even if patches detected.
    pub force: bool,
    /// Keep user modifications when re-running.
    pub preserve_patches: bool,
    /// Number of parallel jobs.
    pub jobs: usize,
    /// Emit Nexflow DSL files (Tier 5) after applying transforms.
    pub emit_dsl: bool,
    /// DSL emission strategy (legacy, direct, or compare).
    pub emit_mode: EmitMode,
    /// Per-emitter overrides for direct path.
    pub emitter_overrides: EmitterOverrides,
    /// Root directory of original COBOL sources (required for direct/compare emit modes).
    /// Combined with paths from cobol2rust-manifest.toml to locate .cbl files.
    pub cobol_source_dir: Option<PathBuf>,
}

/// Report output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Text,
    Json,
    Ndjson,
}

impl Default for RustifyConfig {
    fn default() -> Self {
        Self {
            source_dir: PathBuf::new(),
            output_dir: None,
            tier: Tier::Tier1,
            report_format: ReportFormat::Text,
            verbose: false,
            include: None,
            exclude: None,
            only_rules: Vec::new(),
            skip_rules: Vec::new(),
            force: false,
            preserve_patches: false,
            jobs: 1,
            emit_dsl: false,
            emit_mode: EmitMode::Transpiled,
            emitter_overrides: EmitterOverrides::default(),
            cobol_source_dir: None,
        }
    }
}
