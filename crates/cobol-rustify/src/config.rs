//! Configuration for rustify operations.

use std::path::PathBuf;

use crate::rules::Tier;

/// DSL emission strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmitMode {
    /// Read from Phase 1 Rust via syn (current behavior).
    #[default]
    Legacy,
    /// Read from COBOL AST directly (new path, requires direct-emit feature).
    Direct,
    /// Run both paths, diff output, report discrepancies. Returns legacy as production.
    Compare,
}

/// Per-emitter override: which emitters should use the direct path.
#[derive(Debug, Clone, Default)]
pub struct EmitterOverrides {
    /// Emitter names that should use the direct path (e.g., "schema", "transform").
    pub direct: Vec<String>,
}

impl EmitterOverrides {
    /// Check whether a given emitter should use the direct path.
    pub fn is_direct(&self, emitter_name: &str) -> bool {
        self.direct.iter().any(|e| e == emitter_name)
    }

    /// True if no per-emitter overrides are set.
    pub fn is_empty(&self) -> bool {
        self.direct.is_empty()
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
            emit_mode: EmitMode::Legacy,
            emitter_overrides: EmitterOverrides::default(),
        }
    }
}
