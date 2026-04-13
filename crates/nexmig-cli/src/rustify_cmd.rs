//! `cobol2rust rustify` subcommand.
//!
//! Analyzes transpiler-generated Rust and proposes/applies idiomatic
//! transformations through a tiered promotion pipeline.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;
use miette::{IntoDiagnostic, Result};

use crate::Cli;

/// Rustify: transform transpiled Rust into idiomatic Rust.
#[derive(Debug, Args)]
pub struct RustifyArgs {
    /// Source directory (transpiled Rust workspace or previous stage output).
    pub source_dir: Option<PathBuf>,

    /// Tier to apply: 1 (cosmetic), 2 (type promotion), 3 (structural assessment).
    #[arg(long, default_value = "1")]
    pub tier: u8,

    /// Output directory for transformed Rust (omit for report-only mode).
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Print analysis report (default when no --output).
    #[arg(long)]
    pub report: bool,

    /// Report format: text, json, ndjson.
    #[arg(long, default_value = "text")]
    pub format: String,

    /// File include glob pattern (e.g., "acct*").
    #[arg(long)]
    pub include: Option<String>,

    /// File exclude glob pattern.
    #[arg(long)]
    pub exclude: Option<String>,

    /// Only apply these rule IDs (comma-separated).
    #[arg(long, value_delimiter = ',')]
    pub only: Vec<String>,

    /// Skip these rule IDs (comma-separated).
    #[arg(long, value_delimiter = ',')]
    pub skip: Vec<String>,

    /// List all available rules and exit.
    #[arg(long)]
    pub rules: bool,

    /// Overwrite output even if user patches detected.
    #[arg(long)]
    pub force: bool,

    /// Keep user modifications when re-running on existing output.
    #[arg(long)]
    pub preserve_patches: bool,

    /// Show per-transform details in report.
    #[arg(long)]
    pub verbose_report: bool,

    /// Parallel file processing jobs (default: num_cpus).
    #[arg(short = 'j', long)]
    pub jobs: Option<usize>,

    /// Emit Nexflow DSL files (.schema, .xform, .rules, .proc) under dsl/.
    #[arg(long)]
    pub emit_dsl: bool,

    /// DSL emission strategy: legacy (default), direct, or compare.
    #[arg(long, default_value = "legacy")]
    pub emit_mode: String,

    /// Comma-separated emitters to use direct path for (schema, transform, rules, process).
    /// Overrides --emit-mode for the named emitters.
    #[arg(long, value_delimiter = ',')]
    pub direct_emitters: Vec<String>,
}

/// Execute the rustify subcommand.
pub fn run(cli: &Cli, args: &RustifyArgs) -> Result<ExitCode> {
    // Handle --rules: list all rules and exit
    if args.rules {
        let rules = cobol_rustify::list_rules();
        if rules.is_empty() {
            println!("No rules registered yet (rules will be added in upcoming sessions).");
            println!();
            println!("Planned rules:");
            println!();
            println!("  Tier 1: Cosmetic Cleanup");
            println!("    const-extract      Repeated .parse::<Decimal>() -> named constants");
            println!("    zero-literal       \"0\".parse::<Decimal>() -> Decimal::ZERO");
            println!("    dead-field         Remove unused WorkingStorage fields");
            println!("    allow-cleanup      Remove unnecessary #[allow] attributes");
            println!("    unused-import      Remove unused `use` statements");
            println!("    display-simplify   Simplify format!/display patterns");
            println!();
            println!("  Tier 2: Type Promotion");
            println!("    pic-to-string      PicX -> String (safety-gated)");
            println!("    packed-to-native   PackedDecimal -> Decimal (safety-gated)");
            println!("    localize-vars      WorkingStorage fields -> local variables");
            println!("    bool-extract       Level-88 conditions -> bool");
            println!("    enum-extract       Level-88 groups -> enum");
            println!();
            println!("  Tier 3: Structural Assessment");
            println!("    dispatcher-analysis  Paragraph dispatch analysis");
            println!("    ws-decomposition     WorkingStorage decomposition plan");
            println!("    status-to-result     Status codes -> Result/enum mapping");
            println!("    io-modernize         File I/O pattern modernization plan");
        } else {
            println!("Available rustify rules:");
            println!();
            let mut current_tier = None;
            for rule in &rules {
                if current_tier != Some(rule.tier) {
                    if current_tier.is_some() {
                        println!();
                    }
                    println!("  {}", rule.tier);
                    current_tier = Some(rule.tier);
                }
                println!("    {:<20} {}", rule.id, rule.description);
            }
        }
        return Ok(ExitCode::SUCCESS);
    }

    // Require source_dir for all other operations
    let source_dir = match &args.source_dir {
        Some(d) => d.clone(),
        None => {
            eprintln!("error: <SOURCE_DIR> is required (use --rules to list rules)");
            return Ok(ExitCode::from(1));
        }
    };

    let tier = match args.tier {
        1 => cobol_rustify::rules::Tier::Tier1,
        2 => cobol_rustify::rules::Tier::Tier2,
        3 => cobol_rustify::rules::Tier::Tier3,
        n => {
            eprintln!("error: invalid tier {n} (must be 1, 2, or 3)");
            return Ok(ExitCode::from(1));
        }
    };

    let report_format = match args.format.as_str() {
        "text" => cobol_rustify::config::ReportFormat::Text,
        "json" => cobol_rustify::config::ReportFormat::Json,
        "ndjson" => cobol_rustify::config::ReportFormat::Ndjson,
        f => {
            eprintln!("error: invalid format '{f}' (must be text, json, or ndjson)");
            return Ok(ExitCode::from(1));
        }
    };

    let verbose = args.verbose_report || cli.verbose > 0;

    let emit_mode = match args.emit_mode.as_str() {
        "legacy" => cobol_rustify::config::EmitMode::Legacy,
        "direct" => cobol_rustify::config::EmitMode::Direct,
        "compare" => cobol_rustify::config::EmitMode::Compare,
        m => {
            eprintln!("error: invalid emit-mode '{m}' (must be legacy, direct, or compare)");
            return Ok(ExitCode::from(1));
        }
    };

    let emitter_overrides = cobol_rustify::config::EmitterOverrides {
        direct: args.direct_emitters.clone(),
    };

    let config = cobol_rustify::config::RustifyConfig {
        source_dir,
        output_dir: args.output.clone(),
        tier,
        report_format,
        verbose,
        include: args.include.clone(),
        exclude: args.exclude.clone(),
        only_rules: args.only.clone(),
        skip_rules: args.skip.clone(),
        force: args.force,
        preserve_patches: args.preserve_patches,
        jobs: args.jobs.unwrap_or_else(num_cpus::get),
        emit_dsl: args.emit_dsl,
        emit_mode,
        emitter_overrides,
    };

    // If --output provided, run apply mode; otherwise report mode
    if args.output.is_some() {
        let apply_report = cobol_rustify::apply_workspace(&config).into_diagnostic()?;

        if !apply_report.files_preserved.is_empty() {
            eprintln!(
                "note: {} file(s) preserved (user patches kept)",
                apply_report.files_preserved.len()
            );
        }

        let show_report = args.report;
        if show_report {
            let mut stdout = std::io::stdout();
            apply_report
                .analysis
                .write_report(&mut stdout, report_format, verbose)
                .into_diagnostic()?;
        }

        eprintln!(
            "Applied to {}: {} files copied, {} files transformed, manifest written.",
            args.output.as_ref().unwrap().display(),
            apply_report.files_copied,
            apply_report.files_transformed,
        );
    } else if args.emit_dsl {
        // DSL-only mode: emit DSL without applying tier transforms
        let dsl_reports = cobol_rustify::emit_dsl_for_workspace(&config.source_dir)
            .into_diagnostic()?;

        let total_files: usize = dsl_reports.iter().map(|r| r.total_files).sum();
        let programs = dsl_reports.len();

        for report in &dsl_reports {
            eprintln!(
                "  {} -> {} DSL files (avg confidence: {:.2})",
                report.program_name, report.total_files, report.avg_confidence
            );
        }

        eprintln!(
            "Emitted {} DSL files for {} program(s) under {}/dsl/",
            total_files, programs, config.source_dir.display()
        );
    } else {
        // Report-only mode
        let report = cobol_rustify::analyze_workspace(&config).into_diagnostic()?;

        let mut stdout = std::io::stdout();
        report
            .write_report(&mut stdout, report_format, verbose)
            .into_diagnostic()?;
    }

    Ok(ExitCode::SUCCESS)
}
