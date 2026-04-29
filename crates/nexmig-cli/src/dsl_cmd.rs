//! `cobol2rust dsl` -- emit NexFlow DSL from COBOL sources.
//!
//! Target-agnostic DSL emission. Works with the original COBOL AST
//! (legacy mode) or from transpiled Rust/Java output (transpiled mode).

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;
use miette::{IntoDiagnostic, Result};

use crate::Cli;

/// Emit NexFlow DSL files (.schema, .xform, .rules, .proc) from a workspace.
#[derive(Debug, Args)]
pub struct DslArgs {
    /// Source directory (transpiled workspace or COBOL source root).
    pub source_dir: PathBuf,

    /// DSL emission strategy: transpiled (default), legacy, or compare.
    /// - transpiled: reads Phase 1 transpiled Rust/Java via syn
    /// - legacy: reads original COBOL AST directly (requires --cobol-source)
    /// - compare: runs both paths and diffs output
    #[arg(long, default_value = "transpiled")]
    pub emit_mode: String,

    /// Root directory of original COBOL sources (required for --emit-mode legacy/compare).
    /// Paths in cobol2rust-manifest.toml are resolved relative to this directory.
    #[arg(long)]
    pub cobol_source: Option<PathBuf>,

    /// Comma-separated emitters to use legacy (COBOL AST) path for (schema, transform, rules, process).
    /// Overrides --emit-mode for the named emitters.
    #[arg(long, value_delimiter = ',')]
    pub legacy_emitters: Vec<String>,

    /// File include glob pattern.
    #[arg(long)]
    pub include: Option<String>,

    /// File exclude glob pattern.
    #[arg(long)]
    pub exclude: Option<String>,

    /// Parallel file processing jobs (default: num_cpus).
    #[arg(short = 'j', long)]
    pub jobs: Option<usize>,
}

/// Execute the dsl subcommand.
pub fn run(cli: &Cli, args: &DslArgs) -> Result<ExitCode> {
    let emit_mode = match args.emit_mode.as_str() {
        "transpiled" => cobol_rustify::config::EmitMode::Transpiled,
        "legacy" => cobol_rustify::config::EmitMode::Legacy,
        "compare" => cobol_rustify::config::EmitMode::Compare,
        m => {
            eprintln!("error: invalid emit-mode '{m}' (must be transpiled, legacy, or compare)");
            return Ok(ExitCode::from(1));
        }
    };

    let emitter_overrides = cobol_rustify::config::EmitterOverrides {
        legacy: args.legacy_emitters.clone(),
    };

    let config = cobol_rustify::config::RustifyConfig {
        source_dir: args.source_dir.clone(),
        output_dir: None,
        tier: cobol_rustify::rules::Tier::Tier1,
        report_format: cobol_rustify::config::ReportFormat::Text,
        verbose: cli.verbose > 0,
        include: args.include.clone(),
        exclude: args.exclude.clone(),
        only_rules: Vec::new(),
        skip_rules: Vec::new(),
        force: false,
        preserve_patches: false,
        jobs: args.jobs.unwrap_or_else(num_cpus::get),
        emit_dsl: true,
        emit_mode,
        emitter_overrides,
        cobol_source_dir: args.cobol_source.clone(),
    };

    let dsl_reports = cobol_rustify::emit_dsl_for_workspace(&config, &config.source_dir)
        .into_diagnostic()?;

    let total_files: usize = dsl_reports.iter().map(|r| r.total_files).sum();
    let programs = dsl_reports.len();

    for report in &dsl_reports {
        if !cli.quiet {
            eprintln!(
                "  {} -> {} DSL files (avg confidence: {:.2})",
                report.program_name, report.total_files, report.avg_confidence
            );
        }
    }

    if !cli.quiet {
        eprintln!(
            "Emitted {} DSL files for {} program(s) under {}/dsl/",
            total_files, programs, config.source_dir.display()
        );
    }

    Ok(ExitCode::SUCCESS)
}
