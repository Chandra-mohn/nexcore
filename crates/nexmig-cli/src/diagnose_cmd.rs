//! `cobol2rust diagnose` -- comprehensive diagnostics for COBOL files.
//!
//! Two modes:
//! - **Report mode**: Query an existing audit JSON (`diagnose report <audit.json>`)
//! - **File mode**: Quick single-file diagnostics (`diagnose tokens <file>`, etc.)

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Subcommand, ValueEnum};
use miette::{Context, IntoDiagnostic, Result};
use serde::Serialize;

use crate::audit_cmd::AuditReport;
use crate::cobol_pipeline;
use crate::Cli;

// ---------------------------------------------------------------------------
// CLI arguments
// ---------------------------------------------------------------------------

#[derive(Debug, Args)]
pub struct DiagnoseArgs {
    #[command(subcommand)]
    pub command: DiagnoseCommand,
}

#[derive(Debug, Subcommand)]
pub enum DiagnoseCommand {
    /// Query an existing audit JSON report.
    Report(ReportArgs),
    /// Token recognition error breakdown for a COBOL file.
    Tokens(TokensArgs),
    /// COBOL verb inventory (supported vs unsupported).
    Verbs(VerbsArgs),
    /// COPY dependencies for a COBOL file.
    Copybooks(CopybooksArgs),
    /// Scan directory for non-ASCII encoding issues.
    Encoding(EncodingArgs),
    /// Find and explain skeleton transpile outputs.
    Skeleton(SkeletonArgs),
    /// Rank files by issue count from an audit report.
    Hotspots(HotspotsArgs),
    /// Detect source format (fixed/free) with evidence.
    Format(FormatArgs),
    /// Detect COBOL dialect (IBM, Microfocus, ANSI).
    Dialect(DialectArgs),
    /// Preview or apply character sanitization.
    Sanitize(SanitizeArgs),
    /// Compare two audit reports.
    Compare(CompareArgs),
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

// -- Subcommand args --

#[derive(Debug, Args)]
pub struct ReportArgs {
    /// Path to audit JSON file.
    pub audit_json: PathBuf,
    /// Named query to run (omit for summary).
    #[arg(long)]
    pub query: Option<String>,
    /// Filter by program name (partial match, for relevant queries).
    #[arg(long)]
    pub program: Option<String>,
    /// Filter by copybook name (for relevant queries).
    #[arg(long)]
    pub copybook: Option<String>,
    /// Number of results to show for ranked queries.
    #[arg(long, default_value = "20")]
    pub top: usize,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct TokensArgs {
    /// COBOL source file.
    pub file: PathBuf,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct VerbsArgs {
    /// COBOL source file.
    pub file: PathBuf,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct CopybooksArgs {
    /// COBOL source file.
    pub file: PathBuf,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct EncodingArgs {
    /// Directory to scan.
    pub dir: PathBuf,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct SkeletonArgs {
    /// Directory containing transpiled .rs files.
    pub dir: PathBuf,
    /// Line count threshold for skeleton detection.
    #[arg(long, default_value = "80")]
    pub threshold: usize,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct HotspotsArgs {
    /// Path to audit JSON file.
    pub audit_json: PathBuf,
    /// Number of results.
    #[arg(long, default_value = "20")]
    pub top: usize,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct FormatArgs {
    /// COBOL source file.
    pub file: PathBuf,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct DialectArgs {
    /// COBOL source file.
    pub file: PathBuf,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct SanitizeArgs {
    /// Directory to scan/clean.
    pub dir: PathBuf,
    /// Actually apply changes (default is dry-run).
    #[arg(long)]
    pub apply: bool,
    /// File extensions to process.
    #[arg(long, default_value = "cbl,cob,cpy,copy,cpylib")]
    pub extensions: String,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct CompareArgs {
    /// First audit JSON (baseline).
    pub baseline: PathBuf,
    /// Second audit JSON (current).
    pub current: PathBuf,
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

pub fn run(cli: &Cli, args: &DiagnoseArgs) -> Result<ExitCode> {
    match &args.command {
        DiagnoseCommand::Report(a) => run_report(a),
        DiagnoseCommand::Tokens(a) => run_tokens(cli, a),
        DiagnoseCommand::Verbs(a) => run_verbs(cli, a),
        DiagnoseCommand::Copybooks(a) => run_copybooks(cli, a),
        DiagnoseCommand::Encoding(a) => run_encoding(a),
        DiagnoseCommand::Skeleton(a) => run_skeleton(a),
        DiagnoseCommand::Hotspots(a) => run_hotspots(a),
        DiagnoseCommand::Format(a) => run_format(a),
        DiagnoseCommand::Dialect(a) => run_dialect(a),
        DiagnoseCommand::Sanitize(a) => run_sanitize(a),
        DiagnoseCommand::Compare(a) => run_compare(a),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn load_audit(path: &PathBuf) -> Result<AuditReport> {
    let content = std::fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&content)
        .into_diagnostic()
        .wrap_err("failed to parse audit JSON")
}

fn print_json<T: Serialize>(value: &T) -> Result<()> {
    crate::util::print_json(value)
}

fn read_and_expand(cli: &Cli, file: &PathBuf) -> Result<String> {
    let config = cobol_pipeline::build_config(cli);
    let expanded = cobol_pipeline::read_and_expand(&config, file)?;
    Ok(expanded.source)
}

// ---------------------------------------------------------------------------
// diagnose report
// ---------------------------------------------------------------------------

fn run_report(args: &ReportArgs) -> Result<ExitCode> {
    let report = load_audit(&args.audit_json)?;

    let query = args.query.as_deref().unwrap_or("summary");
    match query {
        "summary" => report_summary(&report, &args.format),
        "missing" | "missing-copybooks" => report_missing(&report, &args.format),
        "missing-for-program" => report_missing_for_program(&report, args),
        "programs-for-copybook" => report_programs_for_copybook(&report, args),
        "blocked" => report_blocked(&report, &args.format),
        "encoding" => report_encoding(&report, &args.format),
        "unused" => report_unused(&report, &args.format),
        "largest" => report_largest(&report, args.top, &args.format),
        "warnings" => report_warnings(&report, args.top, &args.format),
        "most-referenced" => report_most_referenced(&report, args.top, &args.format),
        "token-errors" => report_token_errors(&report, args.top, &args.format),
        "verbs" => report_verbs(&report, &args.format),
        other => {
            eprintln!("Unknown query: {other}");
            eprintln!("Available: summary, missing, missing-for-program, programs-for-copybook,");
            eprintln!("  blocked, encoding, unused, largest, warnings, most-referenced,");
            eprintln!("  token-errors, verbs");
            return Ok(ExitCode::from(1));
        }
    }?;

    Ok(ExitCode::SUCCESS)
}

fn report_summary(report: &AuditReport, fmt: &OutputFormat) -> Result<()> {
    if matches!(fmt, OutputFormat::Json) {
        return print_json(&report.summary);
    }
    let s = &report.summary;
    let r = &s.readiness;
    eprintln!("Source files:     {}", s.total_source_files);
    eprintln!("Copybook files:   {}", s.total_copybook_files);
    eprintln!("Total lines:      {}", s.total_lines);
    eprintln!();
    eprintln!("Scores:");
    eprintln!("  Encoding:       {:.1}  [{}]", r.encoding.score, r.encoding.status);
    eprintln!("  Dependencies:   {:.1}  [{}]", r.dependencies.score, r.dependencies.status);
    eprintln!("  Parsing:        {:.1}  [{}]", r.parsing.score, r.parsing.status);
    eprintln!("  Coverage:       {:.1}  [{}]", r.coverage.score, r.coverage.status);
    eprintln!("  Overall:        {:.1}", r.overall);
    eprintln!("  Verdict:        {}", s.verdict.to_uppercase());
    eprintln!();
    let d = &report.phases.dependencies;
    eprintln!("Dependencies:     {} referenced, {} found, {} missing",
        d.unique_copybooks_referenced, d.copybooks_found, d.copybooks_missing.len());
    let v = &report.phases.validation;
    eprintln!("Validation:       {}/{} parsed OK, {} blocked, {} errors",
        v.files_parsed_ok, report.summary.total_source_files, v.files_blocked, v.files_with_errors);
    if let Some(ref c) = report.phases.coverage {
        eprintln!("Coverage:         {:.1}% average ({} files)", c.average_coverage_pct, c.files_analyzed);
    }
    Ok(())
}

fn report_missing(report: &AuditReport, fmt: &OutputFormat) -> Result<()> {
    let mut missing: Vec<_> = report.phases.dependencies.copybooks_missing.iter().collect();
    missing.sort_by(|a, b| b.referenced_by.len().cmp(&a.referenced_by.len()));
    if matches!(fmt, OutputFormat::Json) {
        return print_json(&missing);
    }
    for m in &missing {
        eprintln!("{:>4}  {}", m.referenced_by.len(), m.name);
    }
    eprintln!("\nTotal missing: {}", missing.len());
    Ok(())
}

fn report_missing_for_program(report: &AuditReport, args: &ReportArgs) -> Result<()> {
    let pattern = args.program.as_deref().unwrap_or("");
    if pattern.is_empty() {
        eprintln!("Use --program <name> to filter");
        return Ok(());
    }
    let pat_lower = pattern.to_lowercase();
    let matches: Vec<_> = report.phases.dependencies.copybooks_missing.iter()
        .filter(|m| m.referenced_by.iter().any(|r| r.to_lowercase().contains(&pat_lower)))
        .collect();
    if matches!(args.format, OutputFormat::Json) {
        return print_json(&matches);
    }
    for m in &matches {
        eprintln!("{}", m.name);
    }
    eprintln!("\nMissing copybooks for *{}*: {}", pattern, matches.len());
    Ok(())
}

fn report_programs_for_copybook(report: &AuditReport, args: &ReportArgs) -> Result<()> {
    let cb = args.copybook.as_deref().unwrap_or("");
    if cb.is_empty() {
        eprintln!("Use --copybook <name> to filter");
        return Ok(());
    }
    let cb_upper = cb.to_uppercase();
    let found = report.phases.dependencies.copybooks_missing.iter()
        .find(|m| m.name.to_uppercase() == cb_upper);
    if matches!(args.format, OutputFormat::Json) {
        return print_json(&found.map(|f| &f.referenced_by));
    }
    match found {
        Some(m) => {
            for f in &m.referenced_by {
                eprintln!("{f}");
            }
            eprintln!("\n{} files reference {}", m.referenced_by.len(), m.name);
        }
        None => eprintln!("Copybook {} not found in missing list", cb),
    }
    Ok(())
}

fn report_blocked(report: &AuditReport, fmt: &OutputFormat) -> Result<()> {
    let blocked: Vec<_> = report.phases.validation.file_results.iter()
        .filter(|r| r.status == "blocked")
        .collect();
    if matches!(fmt, OutputFormat::Json) {
        return print_json(&blocked);
    }
    for r in &blocked {
        let missing: Vec<_> = r.errors.iter()
            .filter_map(|e| e.message.strip_prefix("missing copybook: "))
            .collect();
        eprintln!("{}  {}", r.path, missing.join(", "));
    }
    eprintln!("\nTotal blocked: {}", blocked.len());
    Ok(())
}

fn report_encoding(report: &AuditReport, fmt: &OutputFormat) -> Result<()> {
    let mut issues: Vec<_> = report.phases.discovery.encoding_issues.iter().collect();
    issues.sort_by(|a, b| b.non_ascii_count.cmp(&a.non_ascii_count));
    if matches!(fmt, OutputFormat::Json) {
        return print_json(&issues);
    }
    for ei in &issues {
        eprintln!("{:>6}  {:30}  {}", ei.non_ascii_count, ei.file_encoding, ei.path);
    }
    eprintln!("\nTotal files with encoding issues: {}", issues.len());
    Ok(())
}

fn report_unused(report: &AuditReport, fmt: &OutputFormat) -> Result<()> {
    let unused = &report.phases.dependencies.copybooks_unused;
    if matches!(fmt, OutputFormat::Json) {
        return print_json(unused);
    }
    for name in unused {
        eprintln!("{name}");
    }
    eprintln!("\nTotal unused: {}", unused.len());
    Ok(())
}

fn report_largest(report: &AuditReport, top: usize, fmt: &OutputFormat) -> Result<()> {
    let mut files: Vec<_> = report.phases.validation.file_results.iter().collect();
    files.sort_by(|a, b| b.line_count.cmp(&a.line_count));
    files.truncate(top);
    if matches!(fmt, OutputFormat::Json) {
        return print_json(&files);
    }
    for r in &files {
        eprintln!("{:>8}  {:8}  {}", r.line_count, r.status, r.path);
    }
    Ok(())
}

fn report_warnings(report: &AuditReport, top: usize, fmt: &OutputFormat) -> Result<()> {
    let mut files: Vec<(&str, usize)> = report.phases.validation.file_results.iter()
        .filter(|r| !r.warnings.is_empty())
        .map(|r| (r.path.as_str(), r.warnings.len()))
        .collect();
    files.sort_by(|a, b| b.1.cmp(&a.1));
    files.truncate(top);
    if matches!(fmt, OutputFormat::Json) {
        return print_json(&files);
    }
    for (path, count) in &files {
        eprintln!("{count:>6}  {path}");
    }
    let total: usize = report.phases.validation.file_results.iter()
        .map(|r| r.warnings.len()).sum();
    let file_count = report.phases.validation.file_results.iter()
        .filter(|r| !r.warnings.is_empty()).count();
    eprintln!("\nTotal warnings: {total} across {file_count} files");
    Ok(())
}

fn report_most_referenced(report: &AuditReport, top: usize, fmt: &OutputFormat) -> Result<()> {
    let mut missing: Vec<_> = report.phases.dependencies.copybooks_missing.iter().collect();
    missing.sort_by(|a, b| b.referenced_by.len().cmp(&a.referenced_by.len()));
    missing.truncate(top);
    if matches!(fmt, OutputFormat::Json) {
        return print_json(&missing);
    }
    for m in &missing {
        eprintln!("{:>4}  {}", m.referenced_by.len(), m.name);
    }
    Ok(())
}

fn report_token_errors(report: &AuditReport, top: usize, fmt: &OutputFormat) -> Result<()> {
    // Aggregate token errors across all files.
    let mut global: BTreeMap<String, usize> = BTreeMap::new();
    let mut total = 0usize;
    let mut files_with_errors = 0usize;
    for r in &report.phases.validation.file_results {
        if let Some(ref te) = r.token_errors {
            if te.total_count > 0 {
                files_with_errors += 1;
                total += te.total_count;
                for (ch, count) in &te.by_character {
                    *global.entry(ch.clone()).or_insert(0) += count;
                }
            }
        }
    }
    let mut sorted: Vec<_> = global.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.truncate(top);

    if matches!(fmt, OutputFormat::Json) {
        return print_json(&sorted);
    }
    for (ch, count) in &sorted {
        eprintln!("{count:>8}  {ch}");
    }
    eprintln!("\nTotal token errors: {total} across {files_with_errors} files");
    Ok(())
}

fn report_verbs(report: &AuditReport, fmt: &OutputFormat) -> Result<()> {
    // Aggregate verb usage across all files.
    let mut supported: BTreeMap<String, usize> = BTreeMap::new();
    let mut unsupported: BTreeMap<String, usize> = BTreeMap::new();
    for r in &report.phases.validation.file_results {
        if let Some(ref vi) = r.verb_inventory {
            for v in &vi.supported {
                *supported.entry(v.clone()).or_insert(0) += 1;
            }
            for v in &vi.unsupported {
                *unsupported.entry(v.clone()).or_insert(0) += 1;
            }
        }
    }

    if matches!(fmt, OutputFormat::Json) {
        #[derive(Serialize)]
        struct VerbSummary {
            supported: BTreeMap<String, usize>,
            unsupported: BTreeMap<String, usize>,
        }
        return print_json(&VerbSummary { supported, unsupported });
    }

    if !supported.is_empty() {
        eprintln!("Supported verbs ({}):", supported.len());
        let mut s: Vec<_> = supported.into_iter().collect();
        s.sort_by(|a, b| b.1.cmp(&a.1));
        for (v, count) in &s {
            eprintln!("  {count:>6}  {v}");
        }
    }
    if !unsupported.is_empty() {
        eprintln!("\nUnsupported verbs ({}):", unsupported.len());
        let mut u: Vec<_> = unsupported.into_iter().collect();
        u.sort_by(|a, b| b.1.cmp(&a.1));
        for (v, count) in &u {
            eprintln!("  {count:>6}  {v}");
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// diagnose tokens
// ---------------------------------------------------------------------------

fn run_tokens(cli: &Cli, args: &TokensArgs) -> Result<ExitCode> {
    let config = cobol_pipeline::build_config(cli);
    let parsed = cobol_pipeline::read_expand_parse(&config, &args.file)?;
    let token_errors = parsed.token_errors;

    let mut by_char: BTreeMap<String, usize> = BTreeMap::new();
    for te in &token_errors {
        if !te.offending_text.is_empty() {
            *by_char.entry(te.offending_text.clone()).or_insert(0) += 1;
        }
    }
    let mut sorted: Vec<_> = by_char.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    if matches!(args.format, OutputFormat::Json) {
        #[derive(Serialize)]
        struct TokenReport {
            file: String,
            total: usize,
            by_character: Vec<(String, usize)>,
        }
        return print_json(&TokenReport {
            file: args.file.display().to_string(),
            total: token_errors.len(),
            by_character: sorted,
        }).map(|()| ExitCode::SUCCESS);
    }

    for (ch, count) in &sorted {
        eprintln!("{count:>8}  {ch}");
    }
    eprintln!("\nTotal token errors: {}", token_errors.len());
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// diagnose verbs
// ---------------------------------------------------------------------------

fn run_verbs(cli: &Cli, args: &VerbsArgs) -> Result<ExitCode> {
    let expanded = read_and_expand(cli, &args.file)?;
    let analysis = crate::analyze::analyze_source(&expanded, true);

    if matches!(args.format, OutputFormat::Json) {
        #[derive(Serialize)]
        struct VerbReport {
            file: String,
            supported: Vec<String>,
            unsupported: Vec<String>,
            coverage_pct: f64,
        }
        let coverage_pct = analysis.coverage.as_ref().map_or(0.0, |c| c.coverage_pct);
        return print_json(&VerbReport {
            file: args.file.display().to_string(),
            supported: analysis.verbs_used.clone(),
            unsupported: analysis.verbs_unsupported.clone(),
            coverage_pct,
        }).map(|()| ExitCode::SUCCESS);
    }

    if !analysis.verbs_used.is_empty() {
        eprintln!("Supported verbs ({}):", analysis.verbs_used.len());
        for v in &analysis.verbs_used {
            eprintln!("  {v}");
        }
    }
    if !analysis.verbs_unsupported.is_empty() {
        eprintln!("\nUnsupported verbs ({}):", analysis.verbs_unsupported.len());
        for v in &analysis.verbs_unsupported {
            eprintln!("  {v}");
        }
    }
    if let Some(ref c) = analysis.coverage {
        eprintln!("\nStatement coverage: {:.1}% ({}/{})",
            c.coverage_pct, c.mapped_statements, c.total_statements);
    }
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// diagnose copybooks
// ---------------------------------------------------------------------------

fn run_copybooks(cli: &Cli, args: &CopybooksArgs) -> Result<ExitCode> {
    let source = crate::cobol_read::read_cobol_source(&args.file)
        .wrap_err_with(|| format!("failed to read {}", args.file.display()))?;
    let config = cobol_pipeline::build_config(cli);
    let (found, missing) = cobol_pipeline::collect_dependencies(&config, &source);

    let found: Vec<String> = found.into_iter().collect();
    let missing: Vec<String> = missing.into_iter().collect();

    if matches!(args.format, OutputFormat::Json) {
        #[derive(Serialize)]
        struct CopybookReport {
            file: String,
            found: Vec<String>,
            missing: Vec<String>,
        }
        return print_json(&CopybookReport {
            file: args.file.display().to_string(),
            found,
            missing,
        }).map(|()| ExitCode::SUCCESS);
    }

    if !found.is_empty() {
        eprintln!("Found ({}):", found.len());
        for name in &found {
            eprintln!("  {name}");
        }
    }
    if !missing.is_empty() {
        eprintln!("\nMissing ({}):", missing.len());
        for name in &missing {
            eprintln!("  {name}");
        }
    }
    eprintln!("\nTotal: {} found, {} missing", found.len(), missing.len());
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// diagnose encoding
// ---------------------------------------------------------------------------

fn run_encoding(args: &EncodingArgs) -> Result<ExitCode> {
    use crate::scan::discover;

    let files = discover::discover_files(&args.dir, &[], &[])
        .wrap_err("failed to discover files")?;

    #[derive(Serialize)]
    struct EncodingEntry {
        path: String,
        encoding: String,
        non_ascii_count: usize,
    }

    let mut entries = Vec::new();
    let mut read_errors: u32 = 0;
    for f in &files {
        let bytes = match std::fs::read(&f.absolute_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("  [WARN] Cannot read {}: {e}", f.absolute_path);
                read_errors += 1;
                continue;
            }
        };
        let non_ascii_count = bytes.iter().filter(|&&b| b > 127).count();
        if non_ascii_count > 0 {
            let encoding = crate::audit_cmd::classify_encoding_pub(&bytes);
            entries.push(EncodingEntry {
                path: f.relative_path.clone(),
                encoding,
                non_ascii_count,
            });
        }
    }
    entries.sort_by(|a, b| b.non_ascii_count.cmp(&a.non_ascii_count));

    if matches!(args.format, OutputFormat::Json) {
        return print_json(&entries).map(|()| ExitCode::SUCCESS);
    }

    for e in &entries {
        eprintln!("{:>6}  {:30}  {}", e.non_ascii_count, e.encoding, e.path);
    }
    if read_errors > 0 {
        eprintln!("  ({read_errors} file(s) could not be read)");
    }
    eprintln!("\nTotal files with non-ASCII bytes: {}", entries.len());
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// diagnose skeleton
// ---------------------------------------------------------------------------

fn run_skeleton(args: &SkeletonArgs) -> Result<ExitCode> {
    let mut skeletons = Vec::new();

    fn walk_rs(dir: &std::path::Path, threshold: usize, results: &mut Vec<(String, usize)>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_rs(&path, threshold, results);
            } else if path.extension().is_some_and(|e| e == "rs") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let lines = content.lines().count();
                    if lines <= threshold {
                        results.push((path.display().to_string(), lines));
                    }
                }
            }
        }
    }

    walk_rs(&args.dir, args.threshold, &mut skeletons);
    skeletons.sort_by(|a, b| a.1.cmp(&b.1));

    if matches!(args.format, OutputFormat::Json) {
        return print_json(&skeletons).map(|()| ExitCode::SUCCESS);
    }

    for (path, lines) in &skeletons {
        eprintln!("{lines:>6}  {path}");
    }
    eprintln!("\nSkeleton files (<= {} lines): {}", args.threshold, skeletons.len());
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// diagnose hotspots
// ---------------------------------------------------------------------------

fn run_hotspots(args: &HotspotsArgs) -> Result<ExitCode> {
    let report = load_audit(&args.audit_json)?;

    #[derive(Serialize)]
    struct Hotspot {
        path: String,
        score: usize,
        token_errors: usize,
        parse_errors: usize,
        warnings: usize,
    }

    let mut hotspots: Vec<Hotspot> = report.phases.validation.file_results.iter()
        .map(|r| {
            let te = r.token_errors.as_ref().map_or(0, |t| t.total_count);
            let pe = r.errors.len();
            let w = r.warnings.len();
            Hotspot {
                path: r.path.clone(),
                score: te * 3 + pe * 2 + w,
                token_errors: te,
                parse_errors: pe,
                warnings: w,
            }
        })
        .filter(|h| h.score > 0)
        .collect();
    hotspots.sort_by(|a, b| b.score.cmp(&a.score));
    hotspots.truncate(args.top);

    if matches!(args.format, OutputFormat::Json) {
        return print_json(&hotspots).map(|()| ExitCode::SUCCESS);
    }

    eprintln!("{:>8}  {:>6}  {:>6}  {:>6}  {}", "SCORE", "TOKENS", "ERRORS", "WARNS", "FILE");
    for h in &hotspots {
        eprintln!("{:>8}  {:>6}  {:>6}  {:>6}  {}", h.score, h.token_errors, h.parse_errors, h.warnings, h.path);
    }
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// diagnose format
// ---------------------------------------------------------------------------

fn run_format(args: &FormatArgs) -> Result<ExitCode> {
    let source = crate::cobol_read::read_cobol_source(&args.file)
        .wrap_err_with(|| format!("failed to read {}", args.file.display()))?;

    let format = cobol_transpiler::parser::preprocess::detect_source_format(&source);
    let format_str = match format {
        cobol_transpiler::parser::preprocess::SourceFormat::Fixed => "fixed",
        cobol_transpiler::parser::preprocess::SourceFormat::Free => "free",
    };

    if matches!(args.format, OutputFormat::Json) {
        #[derive(Serialize)]
        struct FormatReport {
            file: String,
            format: String,
            evidence: Vec<String>,
        }
        // Show first 5 lines as evidence.
        let evidence: Vec<String> = source.lines().take(5).map(String::from).collect();
        return print_json(&FormatReport {
            file: args.file.display().to_string(),
            format: format_str.to_string(),
            evidence,
        }).map(|()| ExitCode::SUCCESS);
    }

    eprintln!("File:   {}", args.file.display());
    eprintln!("Format: {format_str}");
    eprintln!("\nFirst 5 lines:");
    for (i, line) in source.lines().take(5).enumerate() {
        eprintln!("  {:>3}: {}", i + 1, line);
    }
    if format_str == "fixed" {
        eprintln!("\n       123456|7|8--------------------------------------------72|");
        eprintln!("       SeqNum|I|Code Area                                     |ID");
    }
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// diagnose dialect
// ---------------------------------------------------------------------------

fn run_dialect(args: &DialectArgs) -> Result<ExitCode> {
    let source = crate::cobol_read::read_cobol_source(&args.file)
        .wrap_err_with(|| format!("failed to read {}", args.file.display()))?;
    let upper = source.to_uppercase();

    #[derive(Serialize)]
    struct DialectReport {
        file: String,
        indicators: Vec<DialectSignal>,
    }
    #[derive(Serialize)]
    struct DialectSignal {
        kind: String,
        count: usize,
    }

    let mut signals = Vec::new();
    let checks = [
        ("CBL/PROCESS directive", &["CBL ", "PROCESS "] as &[&str]),
        ("EXEC CICS", &["EXEC CICS"]),
        ("EXEC SQL", &["EXEC SQL"]),
        ("EXEC DLI", &["EXEC DLI"]),
        ("COMP-5 (IBM)", &["COMP-5"]),
        ("POINTER (IBM)", &[" POINTER"]),
        ("DISPLAY-1 (DBCS)", &["DISPLAY-1"]),
        ("SERVICE RELOAD", &["SERVICE RELOAD"]),
    ];

    for (name, patterns) in &checks {
        let count: usize = patterns.iter().map(|p| upper.matches(p).count()).sum();
        if count > 0 {
            signals.push(DialectSignal { kind: name.to_string(), count });
        }
    }

    if matches!(args.format, OutputFormat::Json) {
        return print_json(&DialectReport {
            file: args.file.display().to_string(),
            indicators: signals,
        }).map(|()| ExitCode::SUCCESS);
    }

    eprintln!("File: {}", args.file.display());
    if signals.is_empty() {
        eprintln!("No dialect-specific indicators found (standard COBOL).");
    } else {
        for s in &signals {
            eprintln!("  {:>4}x  {}", s.count, s.kind);
        }
    }
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// diagnose sanitize
// ---------------------------------------------------------------------------

fn run_sanitize(args: &SanitizeArgs) -> Result<ExitCode> {
    let extensions: Vec<&str> = args.extensions.split(',').collect();

    fn walk_files(dir: &std::path::Path, exts: &[&str], results: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_files(&path, exts, results);
            } else if let Some(ext) = path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if exts.iter().any(|e| e.to_lowercase() == ext_lower) {
                    results.push(path);
                }
            }
        }
    }

    let mut files = Vec::new();
    walk_files(&args.dir, &extensions, &mut files);

    #[derive(Serialize)]
    struct SanitizeEntry {
        path: String,
        non_ascii_bytes: usize,
        would_change: bool,
    }

    let mut entries = Vec::new();
    let mut changed_count = 0usize;
    let mut read_errors: u32 = 0;
    for file in &files {
        let bytes = match std::fs::read(file) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("  [WARN] Cannot read {}: {e}", file.display());
                read_errors += 1;
                continue;
            }
        };
        let non_ascii = bytes.iter().filter(|&&b| b > 127).count();
        if non_ascii == 0 { continue; }

        entries.push(SanitizeEntry {
            path: file.display().to_string(),
            non_ascii_bytes: non_ascii,
            would_change: true,
        });

        if args.apply {
            let cleaned: Vec<u8> = bytes.iter().map(|&b| if b > 127 { b' ' } else { b }).collect();
            if let Err(e) = std::fs::write(file, &cleaned) {
                eprintln!("Error writing {}: {e}", file.display());
            } else {
                changed_count += 1;
            }
        }
    }

    if matches!(args.format, OutputFormat::Json) {
        return print_json(&entries).map(|()| ExitCode::SUCCESS);
    }

    for e in &entries {
        eprintln!("{:>6} bytes  {}", e.non_ascii_bytes, e.path);
    }
    if args.apply {
        eprintln!("\nSanitized {changed_count} files.");
    } else {
        eprintln!("\n{} files would be sanitized. Use --apply to write changes.", entries.len());
    }
    if read_errors > 0 {
        eprintln!("  ({read_errors} file(s) could not be read)");
    }
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// diagnose compare
// ---------------------------------------------------------------------------

fn run_compare(args: &CompareArgs) -> Result<ExitCode> {
    let baseline = load_audit(&args.baseline)?;
    let current = load_audit(&args.current)?;

    #[derive(Serialize)]
    struct CompareReport {
        baseline_version: String,
        current_version: String,
        scores: ScoreDiff,
        files: FileDiff,
        missing_copybooks: CopybookDiff,
    }
    #[derive(Serialize)]
    struct ScoreDiff {
        encoding: (f64, f64, f64),
        dependencies: (f64, f64, f64),
        parsing: (f64, f64, f64),
        coverage: (f64, f64, f64),
        overall: (f64, f64, f64),
    }
    #[derive(Serialize)]
    struct FileDiff {
        source_files: (usize, usize),
        parsed_ok: (usize, usize),
        blocked: (usize, usize),
    }
    #[derive(Serialize)]
    struct CopybookDiff {
        baseline_missing: usize,
        current_missing: usize,
        resolved: Vec<String>,
        new_missing: Vec<String>,
    }

    let br = &baseline.summary.readiness;
    let cr = &current.summary.readiness;

    let baseline_missing: std::collections::BTreeSet<_> = baseline.phases.dependencies.copybooks_missing.iter().map(|m| m.name.clone()).collect();
    let current_missing: std::collections::BTreeSet<_> = current.phases.dependencies.copybooks_missing.iter().map(|m| m.name.clone()).collect();
    let resolved: Vec<String> = baseline_missing.difference(&current_missing).cloned().collect();
    let new_missing: Vec<String> = current_missing.difference(&baseline_missing).cloned().collect();

    let report = CompareReport {
        baseline_version: baseline.audit_version.clone(),
        current_version: current.audit_version.clone(),
        scores: ScoreDiff {
            encoding: (br.encoding.score, cr.encoding.score, cr.encoding.score - br.encoding.score),
            dependencies: (br.dependencies.score, cr.dependencies.score, cr.dependencies.score - br.dependencies.score),
            parsing: (br.parsing.score, cr.parsing.score, cr.parsing.score - br.parsing.score),
            coverage: (br.coverage.score, cr.coverage.score, cr.coverage.score - br.coverage.score),
            overall: (br.overall, cr.overall, cr.overall - br.overall),
        },
        files: FileDiff {
            source_files: (baseline.summary.total_source_files, current.summary.total_source_files),
            parsed_ok: (baseline.phases.validation.files_parsed_ok, current.phases.validation.files_parsed_ok),
            blocked: (baseline.phases.validation.files_blocked, current.phases.validation.files_blocked),
        },
        missing_copybooks: CopybookDiff {
            baseline_missing: baseline_missing.len(),
            current_missing: current_missing.len(),
            resolved,
            new_missing,
        },
    };

    if matches!(args.format, OutputFormat::Json) {
        return print_json(&report).map(|()| ExitCode::SUCCESS);
    }

    eprintln!("Audit Comparison: {} -> {}", report.baseline_version, report.current_version);
    eprintln!();
    eprintln!("{:>15}  {:>8}  {:>8}  {:>8}", "METRIC", "BEFORE", "AFTER", "DELTA");
    let s = &report.scores;
    eprintln!("{:>15}  {:>8.1}  {:>8.1}  {:>+8.1}", "Encoding", s.encoding.0, s.encoding.1, s.encoding.2);
    eprintln!("{:>15}  {:>8.1}  {:>8.1}  {:>+8.1}", "Dependencies", s.dependencies.0, s.dependencies.1, s.dependencies.2);
    eprintln!("{:>15}  {:>8.1}  {:>8.1}  {:>+8.1}", "Parsing", s.parsing.0, s.parsing.1, s.parsing.2);
    eprintln!("{:>15}  {:>8.1}  {:>8.1}  {:>+8.1}", "Coverage", s.coverage.0, s.coverage.1, s.coverage.2);
    eprintln!("{:>15}  {:>8.1}  {:>8.1}  {:>+8.1}", "Overall", s.overall.0, s.overall.1, s.overall.2);
    eprintln!();
    let f = &report.files;
    eprintln!("Files:   {} -> {} source, {} -> {} parsed, {} -> {} blocked",
        f.source_files.0, f.source_files.1, f.parsed_ok.0, f.parsed_ok.1, f.blocked.0, f.blocked.1);
    let mc = &report.missing_copybooks;
    eprintln!("Missing: {} -> {} ({} resolved, {} new)",
        mc.baseline_missing, mc.current_missing, mc.resolved.len(), mc.new_missing.len());
    if !mc.resolved.is_empty() {
        eprintln!("\nResolved: {}", mc.resolved.join(", "));
    }
    if !mc.new_missing.is_empty() {
        eprintln!("New missing: {}", mc.new_missing.join(", "));
    }
    Ok(ExitCode::SUCCESS)
}
