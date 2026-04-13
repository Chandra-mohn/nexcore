//! Shared COBOL processing pipeline -- single source of truth for
//! copybook resolution, COPY expansion, and parsing.
//!
//! All CLI commands that need to expand COPY statements or parse COBOL
//! should use these functions instead of creating their own resolver/expander.

#![allow(dead_code)] // Functions/fields used as commands are migrated.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use miette::{Context, Result};

use cobol_transpiler::ast::CobolProgram;
use cobol_transpiler::diagnostics::TranspileDiagnostic;
use cobol_transpiler::parser::copy_expand::CopyExpander;
use cobol_transpiler::parser::copybook::FileSystemResolver;
use cobol_transpiler::parser::error_collector::TokenError;
use cobol_transpiler::parser::preprocess::detect_source_format;
use cobol_transpiler::parser::SourceFormat;

use crate::workspace;
use crate::Cli;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Resolved configuration for the COBOL processing pipeline.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// All copybook search directories (explicit + auto-discovered).
    pub copy_paths: Vec<PathBuf>,
    /// Maximum nesting depth for COPY expansion.
    pub max_copy_depth: usize,
    /// Library name -> directory mapping for `COPY name OF library`.
    pub library_map: HashMap<String, PathBuf>,
    /// Source format override. When `Some`, skips auto-detection and uses this format.
    /// Set from the `-f fixed/free` CLI flag. `None` means auto-detect.
    pub source_format: Option<SourceFormat>,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            copy_paths: Vec::new(),
            max_copy_depth: 10,
            library_map: HashMap::new(),
            source_format: None,
        }
    }
}

/// Build a `PipelineConfig` from CLI arguments.
///
/// Takes the explicit `-C` paths and auto-discovers copybook subdirectories
/// recursively within each path.
pub fn build_config(cli: &Cli) -> PipelineConfig {
    let mut paths = cli.copybook_paths.clone();
    for dir in &cli.copybook_paths {
        for d in workspace::discover_copybook_dirs(dir) {
            if !paths.contains(&d) {
                paths.push(d);
            }
        }
    }
    let source_format = match cli.source_format {
        crate::SourceFormatArg::Fixed => Some(SourceFormat::Fixed),
        crate::SourceFormatArg::Free => Some(SourceFormat::Free),
        crate::SourceFormatArg::Auto => None,
    };
    PipelineConfig {
        copy_paths: paths,
        max_copy_depth: 10,
        library_map: HashMap::new(),
        source_format,
    }
}

/// Build a `PipelineConfig` from explicit paths (for audit and other commands
/// that resolve copybook directories independently).
pub fn build_config_from_paths(copy_paths: Vec<PathBuf>) -> PipelineConfig {
    PipelineConfig {
        copy_paths,
        max_copy_depth: 10,
        library_map: HashMap::new(),
        source_format: None,
    }
}

// ---------------------------------------------------------------------------
// Result types
// ---------------------------------------------------------------------------

/// Result of COPY expansion.
#[derive(Debug)]
pub struct ExpandedSource {
    /// The expanded source code.
    pub source: String,
    /// Warnings from expansion (e.g., missing copybooks that were skipped).
    pub warnings: Vec<String>,
}

/// Result of parsing an expanded COBOL source.
#[derive(Debug)]
pub struct ParsedSource {
    /// The parsed AST.
    pub program: CobolProgram,
    /// Diagnostics from statement extraction.
    pub diagnostics: Vec<TranspileDiagnostic>,
    /// Token recognition errors from the ANTLR lexer/parser.
    pub token_errors: Vec<TokenError>,
    /// The expanded source that was parsed.
    pub expanded_source: String,
}

// ---------------------------------------------------------------------------
// Pipeline functions
// ---------------------------------------------------------------------------

/// Create a `FileSystemResolver` from the pipeline config.
fn make_resolver(config: &PipelineConfig) -> FileSystemResolver {
    let resolver = FileSystemResolver::new(config.copy_paths.clone());
    if config.library_map.is_empty() {
        resolver
    } else {
        resolver.with_library_map(config.library_map.clone())
    }
}

/// Create a `CopyExpander` from the pipeline config.
fn make_expander(config: &PipelineConfig) -> CopyExpander {
    CopyExpander::new(Box::new(make_resolver(config)), config.max_copy_depth)
}

/// Expand COPY statements in a source string.
///
/// On failure, prints a warning to stderr and returns the original source.
pub fn expand_source(config: &PipelineConfig, source: &str) -> ExpandedSource {
    let expander = make_expander(config);
    match expander.expand(source) {
        Ok(expanded) => ExpandedSource {
            source: expanded,
            warnings: Vec::new(),
        },
        Err(e) => {
            let msg = format!("COPY expansion failed: {e}");
            eprintln!("  Warning: {msg}");
            ExpandedSource {
                source: source.to_string(),
                warnings: vec![msg],
            }
        }
    }
}

/// Expand COPY statements, propagating errors instead of falling back.
pub fn expand_source_strict(config: &PipelineConfig, source: &str) -> Result<String> {
    let expander = make_expander(config);
    expander
        .expand(source)
        .map_err(|e| miette::miette!("{e}"))
}

/// Read a COBOL file and expand COPY statements.
pub fn read_and_expand(config: &PipelineConfig, path: &Path) -> Result<ExpandedSource> {
    let source = crate::cobol_read::read_cobol_source(path)
        .wrap_err_with(|| format!("failed to read {}", path.display()))?;
    Ok(expand_source(config, &source))
}

/// Read a COBOL file, expand COPY statements, and parse to AST.
///
/// Detects the source format on the original file BEFORE COPY expansion,
/// then uses that format for preprocessing. This prevents copybook content
/// with non-standard columns 1-6 from fooling the format detector.
pub fn read_expand_parse(config: &PipelineConfig, path: &Path) -> Result<ParsedSource> {
    let source = crate::cobol_read::read_cobol_source(path)
        .wrap_err_with(|| format!("failed to read {}", path.display()))?;
    // Use explicit format if provided (-f fixed), otherwise detect on ORIGINAL source.
    let format = config.source_format.unwrap_or_else(|| detect_source_format(&source));
    let expanded = expand_source(config, &source);
    let (program, diagnostics, token_errors) =
        cobol_transpiler::parser::parse_cobol_with_format(&expanded.source, format)
            .wrap_err("COBOL parse failed")?;
    Ok(ParsedSource {
        program,
        diagnostics,
        token_errors,
        expanded_source: expanded.source,
    })
}

/// Expand and parse an already-read source string.
///
/// Detects the source format on the original source BEFORE COPY expansion.
pub fn expand_and_parse(config: &PipelineConfig, source: &str) -> Result<ParsedSource> {
    // Use explicit format if provided (-f fixed), otherwise detect on ORIGINAL source.
    let format = config.source_format.unwrap_or_else(|| detect_source_format(source));
    let expanded = expand_source(config, source);
    let (program, diagnostics, token_errors) =
        cobol_transpiler::parser::parse_cobol_with_format(&expanded.source, format)
            .wrap_err("COBOL parse failed")?;
    Ok(ParsedSource {
        program,
        diagnostics,
        token_errors,
        expanded_source: expanded.source,
    })
}

/// Collect COPY dependencies without full expansion.
///
/// Returns `(found, missing)` sets of uppercase copybook names.
pub fn collect_dependencies(
    config: &PipelineConfig,
    source: &str,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let expander = make_expander(config);
    expander.collect_dependencies(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = PipelineConfig::default();
        assert_eq!(config.max_copy_depth, 10);
        assert!(config.copy_paths.is_empty());
        assert!(config.library_map.is_empty());
    }

    #[test]
    fn expand_source_no_copies() {
        let config = PipelineConfig::default();
        let source = "IDENTIFICATION DIVISION.\nPROGRAM-ID. TEST.\n";
        let result = expand_source(&config, source);
        assert_eq!(result.source, source);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn collect_dependencies_empty() {
        let config = PipelineConfig::default();
        let source = "IDENTIFICATION DIVISION.\nPROGRAM-ID. TEST.\n";
        let (found, missing) = collect_dependencies(&config, source);
        assert!(found.is_empty());
        assert!(missing.is_empty());
    }
}
