//! `cobol2rust audit` -- preflight validation of a COBOL codebase.
//!
//! Runs a 5-phase pipeline: discovery, dependency analysis, parse validation,
//! transpilation coverage, and readiness scoring. Produces a structured JSON
//! report (stdout) with progress on stderr.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Mutex;
use std::time::Instant;

use clap::{Args, ValueEnum};
use indicatif::ProgressBar;
use miette::Result;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::analyze;
use crate::cobol_read;
use crate::scan::discover::{self, FileType};
use crate::workspace;
use crate::Cli;

// ---------------------------------------------------------------------------
// CLI arguments
// ---------------------------------------------------------------------------

/// Arguments for `cobol2rust audit`.
#[derive(Debug, Args)]
pub struct AuditArgs {
    /// Root directory containing COBOL source files.
    pub root_dir: PathBuf,

    /// Output format.
    #[arg(long, default_value = "json")]
    pub format: AuditFormat,

    /// Additional file extensions to scan (comma-separated).
    #[arg(long)]
    pub extensions: Vec<String>,

    /// Directory or glob patterns to exclude.
    #[arg(long)]
    pub exclude: Vec<String>,

    /// Skip transpilation coverage analysis (Phase 4) for faster results.
    #[arg(long)]
    pub skip_coverage: bool,

    /// Number of parallel workers (default: number of CPUs).
    #[arg(short = 'j', long)]
    pub jobs: Option<usize>,
}

/// Output format for audit results.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AuditFormat {
    Json,
    Text,
}

// ---------------------------------------------------------------------------
// Report data structures
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditReport {
    pub audit_version: String,
    pub timestamp: String,
    pub source_dir: String,
    pub copybook_dirs: Vec<String>,
    pub summary: AuditSummary,
    pub blockers: Vec<Blocker>,
    pub phases: PhaseResults,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditSummary {
    pub total_source_files: usize,
    pub total_copybook_files: usize,
    pub total_lines: usize,
    pub readiness: ReadinessScores,
    pub verdict: String,
    #[serde(default)]
    pub read_errors: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadinessScores {
    pub encoding: ScoreEntry,
    pub dependencies: ScoreEntry,
    pub parsing: ScoreEntry,
    pub coverage: ScoreEntry,
    pub overall: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreEntry {
    pub score: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blocker {
    pub category: String,
    pub severity: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub affected_files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhaseResults {
    pub discovery: DiscoveryResult,
    pub dependencies: DependencyResult,
    pub validation: ValidationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<CoveragePhaseResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screens: Option<ScreenAuditResult>,
}

// -- Phase 6: Screen Audit --

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenAuditResult {
    pub total_bms_files: usize,
    pub total_maps: usize,
    pub total_fields: usize,
    pub total_input_fields: usize,
    pub connected_maps: usize,
    pub orphaned_maps: usize,
    pub orphaned_map_names: Vec<String>,
    pub missing_maps: Vec<String>,
    pub map_details: Vec<MapAuditEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapAuditEntry {
    pub map_name: String,
    pub bms_file: String,
    pub fields: usize,
    pub input_fields: usize,
    pub display_fields: usize,
    pub labels: usize,
    pub size: (u8, u8),
    pub connected_programs: Vec<String>,
}

// -- Phase 1: Discovery --

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryResult {
    pub source_file_count: usize,
    pub copybook_file_count: usize,
    pub total_lines: usize,
    pub file_types: FileTypeSummary,
    pub files: Vec<DiscoveredFileInfo>,
    pub encoding_issues: Vec<EncodingIssue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_format_summary: Option<SourceFormatSummary>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dialect_indicators: Vec<DialectIndicator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFormatSummary {
    pub fixed: usize,
    pub free: usize,
    pub fixed_pct: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialectIndicator {
    pub kind: String,
    pub count: usize,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveredFileInfo {
    pub path: String,
    pub file_type: String,
    pub file_encoding: String,
    pub lines: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeSummary {
    pub ascii: usize,
    pub iso_8859: usize,
    pub non_iso_extended_ascii: usize,
    pub binary_data: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodingIssue {
    pub path: String,
    pub file_encoding: String,
    pub non_ascii_count: usize,
    pub sample_lines: Vec<usize>,
}

// -- Phase 2: Dependencies --

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyResult {
    pub total_copy_references: usize,
    pub unique_copybooks_referenced: usize,
    pub copybooks_found: usize,
    pub copybooks_missing: Vec<MissingCopybook>,
    pub copybooks_unused: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub per_file_dependencies: Vec<FileDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDependency {
    pub path: String,
    pub found: Vec<String>,
    pub missing: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MissingCopybook {
    pub name: String,
    pub referenced_by: Vec<String>,
}

// -- Phase 3: Validation --

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub files_attempted: usize,
    pub files_parsed_ok: usize,
    pub files_blocked: usize,
    pub files_with_errors: usize,
    pub files_with_warnings: usize,
    pub file_results: Vec<ValidationFileResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationFileResult {
    pub path: String,
    pub program_id: String,
    pub status: String,
    pub source_format: String,
    pub line_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<DiagnosticInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<DiagnosticInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<StatsInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_errors: Option<TokenErrorInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb_inventory: Option<VerbInventory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parse_time_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenErrorInfo {
    pub total_count: usize,
    pub by_character: BTreeMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerbInventory {
    pub supported: Vec<String>,
    pub unsupported: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsInfo {
    pub paragraphs: usize,
    pub sections: usize,
    pub calls: usize,
    pub data_items: usize,
    pub sql_statements: usize,
}

// -- Phase 4: Coverage --

#[derive(Debug, Serialize, Deserialize)]
pub struct CoveragePhaseResult {
    pub files_analyzed: usize,
    pub average_coverage_pct: f64,
    pub file_coverages: Vec<FileCoverage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCoverage {
    pub path: String,
    pub total_statements: usize,
    pub mapped_statements: usize,
    pub coverage_pct: f64,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

pub fn run(cli: &Cli, args: &AuditArgs) -> Result<ExitCode> {
    use std::sync::atomic::{AtomicU32, Ordering};
    let start = Instant::now();
    let read_error_count = AtomicU32::new(0);

    // Configure rayon thread pool.
    let jobs = args.jobs.unwrap_or_else(num_cpus::get);
    crate::util::setup_thread_pool(jobs);

    // Resolve copybook search paths.
    let mut copy_paths: Vec<PathBuf> = cli.copybook_paths.clone();
    // Auto-discover copybook directories under root.
    for d in workspace::discover_copybook_dirs(&args.root_dir) {
        if !copy_paths.contains(&d) {
            copy_paths.push(d);
        }
    }

    let pipeline_config = crate::cobol_pipeline::build_config_from_paths(copy_paths.clone());

    let copy_path_strings: Vec<String> =
        copy_paths.iter().map(|p| p.display().to_string()).collect();

    if !cli.quiet {
        eprintln!("cobol2rust audit: {}", args.root_dir.display());
        eprintln!(
            "  Copybook paths: {}",
            if copy_paths.is_empty() {
                "(none)".to_string()
            } else {
                copy_paths
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        );
        eprintln!();
    }

    // -----------------------------------------------------------------------
    // Phase 1: Discovery
    // -----------------------------------------------------------------------
    let phase1_start = Instant::now();
    if !cli.quiet {
        eprintln!("Phase 1: Discovering files...");
    }

    let mut discovered = discover::discover_files(&args.root_dir, &args.extensions, &args.exclude)?;

    // Also scan copybook directories so they appear in the inventory.
    for cp in &copy_paths {
        if cp != &args.root_dir && cp.is_dir() {
            if let Ok(extra) = discover::discover_files(cp, &args.extensions, &args.exclude) {
                for f in extra {
                    // Avoid duplicates if copybook dir overlaps with root.
                    if !discovered.iter().any(|d| d.absolute_path == f.absolute_path) {
                        discovered.push(f);
                    }
                }
            }
        }
    }

    let source_files: Vec<_> = discovered
        .iter()
        .filter(|f| f.file_type == FileType::Source)
        .collect();
    let copybook_files: Vec<_> = discovered
        .iter()
        .filter(|f| f.file_type == FileType::Copybook)
        .collect();

    // Scan for encoding issues, classify file types, and count lines.
    let mut encoding_issues = Vec::new();
    let mut file_infos = Vec::new();
    let mut total_lines: usize = 0;
    let mut ft_ascii: usize = 0;
    let mut ft_iso8859: usize = 0;
    let mut ft_non_iso: usize = 0;
    let mut ft_binary: usize = 0;

    let all_files_iter = source_files.iter().chain(copybook_files.iter());
    let all_files_count = source_files.len() + copybook_files.len();
    let pb = make_progress_bar(all_files_count as u64, "  Scanning encoding");
    for sf in all_files_iter {
        if let Ok(bytes) = std::fs::read(&sf.absolute_path) {
            let line_count = bytecount_lines(&bytes);
            if sf.file_type == FileType::Source {
                total_lines += line_count;
            }

            let classification = classify_encoding(&bytes);
            match classification.as_str() {
                "ASCII text" => ft_ascii += 1,
                "ISO-8859 text" => ft_iso8859 += 1,
                "Non-ISO extended ASCII text" => ft_non_iso += 1,
                _ => ft_binary += 1,
            }

            file_infos.push(DiscoveredFileInfo {
                path: sf.relative_path.clone(),
                file_type: sf.file_type.as_str().to_string(),
                file_encoding: classification.clone(),
                lines: line_count,
            });

            if classification != "ASCII text" {
                let mut non_ascii_count = 0usize;
                let mut sample_lines = Vec::new();
                for (line_idx, line) in bytes.split(|&b| b == b'\n').enumerate() {
                    let has_non_ascii = line.iter().any(|&b| b > 127);
                    if has_non_ascii {
                        non_ascii_count += line.iter().filter(|&&b| b > 127).count();
                        if sample_lines.len() < 5 {
                            sample_lines.push(line_idx + 1);
                        }
                    }
                }
                encoding_issues.push(EncodingIssue {
                    path: sf.relative_path.clone(),
                    file_encoding: classification,
                    non_ascii_count,
                    sample_lines,
                });
            }
        }
        pb.inc(1);
    }
    pb.finish_and_clear();

    let mut discovery = DiscoveryResult {
        source_file_count: source_files.len(),
        copybook_file_count: copybook_files.len(),
        total_lines,
        file_types: FileTypeSummary {
            ascii: ft_ascii,
            iso_8859: ft_iso8859,
            non_iso_extended_ascii: ft_non_iso,
            binary_data: ft_binary,
        },
        files: file_infos,
        encoding_issues,
        source_format_summary: None,
        dialect_indicators: Vec::new(),
    };

    if !cli.quiet {
        eprintln!(
            "  {} source files, {} copybooks, {} lines ({:.1}s)",
            source_files.len(),
            copybook_files.len(),
            total_lines,
            phase1_start.elapsed().as_secs_f64()
        );
    }

    // -----------------------------------------------------------------------
    // Phase 2: Dependency analysis
    // -----------------------------------------------------------------------
    let phase2_start = Instant::now();
    if !cli.quiet {
        eprintln!("Phase 2: Analyzing dependencies...");
    }

    // Build a set of available copybook stems (uppercase) for unused detection.
    let available_stems: BTreeSet<String> = copybook_files
        .iter()
        .filter_map(|f| {
            std::path::Path::new(&f.relative_path)
                .file_stem()
                .map(|s| s.to_string_lossy().to_uppercase())
        })
        .collect();

    // Recursively resolve COPY dependencies per file using the same
    // CopyExpander the transpiler uses. This follows nested COPYs inside
    // copybooks so the missing list is complete and matches what transpile
    // would need.
    let mut all_found: BTreeSet<String> = BTreeSet::new();
    let mut all_missing: BTreeSet<String> = BTreeSet::new();
    let mut copy_ref_count: usize = 0;
    // Map: copybook name (upper) -> list of source files that need it.
    let mut ref_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    // Per-file dependency info (found + missing for each file).
    let mut file_deps: Vec<FileDependency> = Vec::new();

    let pb = make_progress_bar(source_files.len() as u64, "  Resolving dependencies");
    for sf in &source_files {
        let source = match cobol_read::read_cobol_source(std::path::Path::new(&sf.absolute_path)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("  [WARN] Cannot read {}: {e}", sf.absolute_path);
                read_error_count.fetch_add(1, Ordering::Relaxed);
                pb.inc(1);
                continue;
            }
        };

        let (found, missing) = crate::cobol_pipeline::collect_dependencies(&pipeline_config, &source);

        copy_ref_count += found.len() + missing.len();

        for name in &found {
            all_found.insert(name.clone());
            ref_map
                .entry(name.clone())
                .or_default()
                .push(sf.relative_path.clone());
        }
        for name in &missing {
            all_missing.insert(name.clone());
            ref_map
                .entry(name.clone())
                .or_default()
                .push(sf.relative_path.clone());
        }

        let file_found: Vec<String> = found.into_iter().collect();
        let file_missing: Vec<String> = missing.into_iter().collect();
        file_deps.push(FileDependency {
            path: sf.relative_path.clone(),
            found: file_found,
            missing: file_missing,
        });
        pb.inc(1);
    }
    pb.finish_and_clear();

    let all_referenced: BTreeSet<String> = all_found.union(&all_missing).cloned().collect();
    let found_set = all_found;

    let mut missing_list: Vec<MissingCopybook> = Vec::new();
    for name in &all_missing {
        let refs = ref_map.get(name).cloned().unwrap_or_default();
        missing_list.push(MissingCopybook {
            name: name.clone(),
            referenced_by: refs,
        });
    }

    // Unused copybooks: available but never referenced (directly or transitively).
    let unused: Vec<String> = available_stems
        .difference(&all_referenced)
        .cloned()
        .collect();

    let dependencies = DependencyResult {
        total_copy_references: copy_ref_count,
        unique_copybooks_referenced: all_referenced.len(),
        copybooks_found: found_set.len(),
        copybooks_missing: missing_list,
        copybooks_unused: unused,
        per_file_dependencies: file_deps.clone(),
    };

    if !cli.quiet {
        eprintln!(
            "  {} referenced, {} found, {} missing ({:.1}s)",
            all_referenced.len(),
            found_set.len(),
            dependencies.copybooks_missing.len(),
            phase2_start.elapsed().as_secs_f64()
        );
    }

    // -----------------------------------------------------------------------
    // Phase 3: Validation (parse with COPY expansion)
    // -----------------------------------------------------------------------
    let phase3_start = Instant::now();
    if !cli.quiet {
        eprintln!("Phase 3: Validating COBOL...");
    }

    // Partition: ready (no missing deps) vs blocked (has missing deps).
    // file_deps already contains per-file dependency info from Phase 2.
    let mut ready_files: Vec<&str> = Vec::new();
    let mut blocked_files: Vec<(&str, Vec<String>)> = Vec::new();

    for fd in &file_deps {
        if fd.missing.is_empty() {
            ready_files.push(&fd.path);
        } else {
            blocked_files.push((&fd.path, fd.missing.clone()));
        }
    }
    // Files with no COPY statements are also ready.
    let files_with_deps: BTreeSet<&str> = file_deps.iter().map(|fd| fd.path.as_str()).collect();
    for sf in &source_files {
        if !files_with_deps.contains(sf.relative_path.as_str()) {
            ready_files.push(&sf.relative_path);
        }
    }

    // Build abs path lookup.
    let abs_path_map: HashMap<&str, &str> = source_files
        .iter()
        .map(|f| (f.relative_path.as_str(), f.absolute_path.as_str()))
        .collect();

    let pb = make_progress_bar(ready_files.len() as u64, "  Parsing");
    let validation_results: Mutex<Vec<ValidationFileResult>> = Mutex::new(Vec::new());
    let parsed_ok_paths: Mutex<Vec<String>> = Mutex::new(Vec::new());

    ready_files.par_iter().for_each(|rel_path| {
        let abs = match abs_path_map.get(rel_path) {
            Some(a) => *a,
            None => {
                pb.inc(1);
                return;
            }
        };
        let source = match cobol_read::read_cobol_source(std::path::Path::new(abs)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("  [WARN] Cannot read {abs}: {e}");
                read_error_count.fetch_add(1, Ordering::Relaxed);
                pb.inc(1);
                return;
            }
        };

        // Expand COPY statements.
        let expanded = crate::cobol_pipeline::expand_source(&pipeline_config, &source);

        let analysis = analyze::analyze_source(&expanded.source, false);

        let status = if analysis.valid { "ok" } else { "error" };
        if analysis.valid {
            parsed_ok_paths
                .lock()
                .unwrap()
                .push(rel_path.to_string());
        }

        // Build token error summary if any were collected.
        let token_errors = if analysis.token_errors.is_empty() {
            None
        } else {
            let mut by_character = BTreeMap::new();
            for te in &analysis.token_errors {
                if !te.offending_text.is_empty() {
                    *by_character.entry(te.offending_text.clone()).or_insert(0usize) += 1;
                }
            }
            Some(TokenErrorInfo {
                total_count: analysis.token_errors.len(),
                by_character,
            })
        };

        // Build verb inventory if any verbs were found.
        let verb_inventory = if analysis.verbs_used.is_empty() && analysis.verbs_unsupported.is_empty() {
            None
        } else {
            Some(VerbInventory {
                supported: analysis.verbs_used,
                unsupported: analysis.verbs_unsupported,
            })
        };

        let result = ValidationFileResult {
            path: rel_path.to_string(),
            program_id: analysis.program_id,
            status: status.to_string(),
            source_format: analysis.source_format,
            line_count: analysis.line_count,
            errors: analysis
                .errors
                .iter()
                .map(|e| DiagnosticInfo {
                    line: e.line,
                    code: e.code.clone(),
                    message: e.message.clone(),
                })
                .collect(),
            warnings: analysis
                .warnings
                .iter()
                .map(|w| DiagnosticInfo {
                    line: w.line,
                    code: w.code.clone(),
                    message: w.message.clone(),
                })
                .collect(),
            stats: Some(StatsInfo {
                paragraphs: analysis.info.paragraphs,
                sections: analysis.info.sections,
                calls: analysis.info.calls,
                data_items: analysis.info.data_items,
                sql_statements: analysis.info.sql_statements,
            }),
            token_errors,
            verb_inventory,
            parse_time_ms: Some(analysis.parse_time_ms),
        };

        validation_results.lock().unwrap().push(result);
        pb.inc(1);
    });
    pb.finish_and_clear();

    // Add blocked file results.
    let mut all_results = validation_results.into_inner().unwrap();
    for (path, missing) in &blocked_files {
        all_results.push(ValidationFileResult {
            path: path.to_string(),
            program_id: String::new(),
            status: "blocked".to_string(),
            source_format: String::new(),
            line_count: 0,
            errors: missing
                .iter()
                .map(|m| DiagnosticInfo {
                    line: None,
                    code: "B001".to_string(),
                    message: format!("missing copybook: {m}"),
                })
                .collect(),
            warnings: Vec::new(),
            stats: None,
            token_errors: None,
            verb_inventory: None,
            parse_time_ms: None,
        });
    }

    let files_parsed_ok = all_results.iter().filter(|r| r.status == "ok").count();
    let files_with_errors = all_results.iter().filter(|r| r.status == "error").count();
    let files_with_warnings = all_results
        .iter()
        .filter(|r| !r.warnings.is_empty())
        .count();

    let validation = ValidationResult {
        files_attempted: ready_files.len(),
        files_parsed_ok,
        files_blocked: blocked_files.len(),
        files_with_errors,
        files_with_warnings,
        file_results: all_results,
    };

    if !cli.quiet {
        eprintln!(
            "  {}/{} parsed OK, {} blocked, {} errors ({:.1}s)",
            files_parsed_ok,
            source_files.len(),
            blocked_files.len(),
            files_with_errors,
            phase3_start.elapsed().as_secs_f64()
        );
    }

    // Aggregate source format summary from Phase 3 results.
    {
        let mut fixed = 0usize;
        let mut free = 0usize;
        for r in &validation.file_results {
            match r.source_format.as_str() {
                "fixed" => fixed += 1,
                "free" => free += 1,
                _ => {}
            }
        }
        let total = (fixed + free).max(1) as f64;
        discovery.source_format_summary = Some(SourceFormatSummary {
            fixed,
            free,
            fixed_pct: (fixed as f64 / total) * 100.0,
        });
    }

    // -----------------------------------------------------------------------
    // Phase 4: Coverage (optional)
    // -----------------------------------------------------------------------
    let coverage = if args.skip_coverage {
        if !cli.quiet {
            eprintln!("Phase 4: Skipped (--skip-coverage)");
        }
        None
    } else {
        let phase4_start = Instant::now();
        let ok_paths = parsed_ok_paths.into_inner().unwrap();
        if !cli.quiet {
            eprintln!("Phase 4: Analyzing coverage ({} files)...", ok_paths.len());
        }

        let pb = make_progress_bar(ok_paths.len() as u64, "  Coverage");
        let coverages: Mutex<Vec<FileCoverage>> = Mutex::new(Vec::new());

        ok_paths.par_iter().for_each(|rel_path| {
            let abs = match abs_path_map.get(rel_path.as_str()) {
                Some(a) => *a,
                None => {
                    pb.inc(1);
                    return;
                }
            };
            let source = match cobol_read::read_cobol_source(std::path::Path::new(abs)) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("  [WARN] Cannot read {abs}: {e}");
                    read_error_count.fetch_add(1, Ordering::Relaxed);
                    pb.inc(1);
                    return;
                }
            };

            let expanded = crate::cobol_pipeline::expand_source(&pipeline_config, &source);

            let analysis = analyze::analyze_source(&expanded.source, true);
            if let Some(cov) = analysis.coverage {
                coverages.lock().unwrap().push(FileCoverage {
                    path: rel_path.clone(),
                    total_statements: cov.total_statements,
                    mapped_statements: cov.mapped_statements,
                    coverage_pct: cov.coverage_pct,
                });
            }
            pb.inc(1);
        });
        pb.finish_and_clear();

        let file_coverages = coverages.into_inner().unwrap();
        let avg_coverage = if file_coverages.is_empty() {
            0.0
        } else {
            file_coverages.iter().map(|c| c.coverage_pct).sum::<f64>()
                / file_coverages.len() as f64
        };

        if !cli.quiet {
            eprintln!(
                "  {:.1}% average coverage ({:.1}s)",
                avg_coverage,
                phase4_start.elapsed().as_secs_f64()
            );
        }

        Some(CoveragePhaseResult {
            files_analyzed: file_coverages.len(),
            average_coverage_pct: avg_coverage,
            file_coverages,
        })
    };

    // -----------------------------------------------------------------------
    // Phase 6: Screen Audit (BMS)
    // -----------------------------------------------------------------------
    let screens = {
        use cobol_transpiler::parser::{bms_parser, bms_references};

        // Discover BMS files in the source tree
        let mut bms_files: Vec<std::path::PathBuf> = Vec::new();
        fn find_bms(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        find_bms(&path, out);
                    } else if path.extension()
                        .map_or(false, |e| e.eq_ignore_ascii_case("bms"))
                    {
                        out.push(path);
                    }
                }
            }
        }
        find_bms(&args.root_dir, &mut bms_files);

        if bms_files.is_empty() {
            None
        } else {
            if !cli.quiet {
                eprintln!("Phase 6: Auditing BMS screens ({} files)...", bms_files.len());
            }

            // Parse all BMS files
            let mut map_details: Vec<MapAuditEntry> = Vec::new();
            let mut all_map_names: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            let mut total_fields: usize = 0;
            let mut total_input_fields: usize = 0;

            for bms_path in &bms_files {
                match bms_parser::parse_bms_file(bms_path) {
                    Ok(mapset) => {
                        let bms_file = bms_path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_string();
                        for map in &mapset.maps {
                            let data_fields = map.data_fields();
                            let input = map.input_fields();
                            let labels = map.fields.iter().filter(|f| f.is_label()).count();
                            total_fields += data_fields.len();
                            total_input_fields += input.len();
                            all_map_names.insert(map.name.to_uppercase());
                            map_details.push(MapAuditEntry {
                                map_name: map.name.clone(),
                                bms_file: bms_file.clone(),
                                fields: data_fields.len(),
                                input_fields: input.len(),
                                display_fields: data_fields.len() - input.len(),
                                labels,
                                size: map.size,
                                connected_programs: Vec::new(),
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!("  [WARN] Cannot parse {}: {e}", bms_path.display());
                    }
                }
            }

            // Cross-reference with COBOL programs
            let xref = bms_references::scan_bms_references(
                &args.root_dir,
                &["cbl", "CBL", "cob", "COB"],
            );

            // Fill in connected programs
            for entry in &mut map_details {
                let map_upper = entry.map_name.to_uppercase();
                if let Some(progs) = xref.map_to_programs.get(&map_upper) {
                    let mut prog_names: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
                    for p in progs {
                        prog_names.insert(p.program_file.clone());
                    }
                    entry.connected_programs = prog_names.into_iter().collect();
                }
            }

            let connected = map_details.iter().filter(|m| !m.connected_programs.is_empty()).count();
            let orphaned = map_details.iter().filter(|m| m.connected_programs.is_empty()).count();
            let orphaned_names: Vec<String> = map_details.iter()
                .filter(|m| m.connected_programs.is_empty())
                .map(|m| m.map_name.clone())
                .collect();

            // Find maps referenced in COBOL but not in any BMS file
            let missing: Vec<String> = xref.map_to_programs.keys()
                .filter(|name| !all_map_names.contains(name.as_str()))
                .cloned()
                .collect();

            Some(ScreenAuditResult {
                total_bms_files: bms_files.len(),
                total_maps: map_details.len(),
                total_fields,
                total_input_fields,
                connected_maps: connected,
                orphaned_maps: orphaned,
                orphaned_map_names: orphaned_names,
                missing_maps: missing,
                map_details,
            })
        }
    };

    // Phase 5: Readiness scoring
    // -----------------------------------------------------------------------
    let total_src = source_files.len().max(1) as f64;

    let encoding_score = 100.0
        * (1.0 - discovery.encoding_issues.len() as f64 / total_src);

    let dependency_score = if all_referenced.is_empty() {
        100.0
    } else {
        100.0 * (found_set.len() as f64 / all_referenced.len() as f64)
    };

    let parsing_score = if validation.files_attempted == 0 {
        0.0
    } else {
        100.0 * (files_parsed_ok as f64 / validation.files_attempted as f64)
    };

    let coverage_score = coverage
        .as_ref()
        .map_or(0.0, |c| c.average_coverage_pct);

    let overall = 0.15 * encoding_score
        + 0.35 * dependency_score
        + 0.35 * parsing_score
        + 0.15 * coverage_score;

    let verdict = if overall >= 80.0 {
        "ready"
    } else if overall >= 50.0 {
        "needs_work"
    } else {
        "blocked"
    };

    // Build blockers list.
    let mut blockers: Vec<Blocker> = Vec::new();

    for mc in &dependencies.copybooks_missing {
        blockers.push(Blocker {
            category: "dependency".to_string(),
            severity: "error".to_string(),
            message: format!(
                "Missing copybook: {} (referenced by {} file(s))",
                mc.name,
                mc.referenced_by.len()
            ),
            affected_files: mc.referenced_by.clone(),
        });
    }

    for ei in &discovery.encoding_issues {
        blockers.push(Blocker {
            category: "encoding".to_string(),
            severity: "warning".to_string(),
            message: format!(
                "{}: {} non-ASCII byte(s), first at line {}",
                ei.path,
                ei.non_ascii_count,
                ei.sample_lines.first().copied().unwrap_or(0)
            ),
            affected_files: vec![ei.path.clone()],
        });
    }

    // Sort blockers: errors first, then by category.
    blockers.sort_by(|a, b| {
        a.severity
            .cmp(&b.severity)
            .then_with(|| a.category.cmp(&b.category))
    });

    let report = AuditReport {
        audit_version: "1.1".to_string(),
        timestamp: chrono_timestamp(),
        source_dir: args.root_dir.display().to_string(),
        copybook_dirs: copy_path_strings,
        summary: AuditSummary {
            total_source_files: source_files.len(),
            total_copybook_files: copybook_files.len(),
            total_lines,
            readiness: ReadinessScores {
                encoding: score_entry(encoding_score),
                dependencies: score_entry(dependency_score),
                parsing: score_entry(parsing_score),
                coverage: score_entry(coverage_score),
                overall,
            },
            verdict: verdict.to_string(),
            read_errors: read_error_count.load(Ordering::Relaxed),
        },
        blockers,
        phases: PhaseResults {
            discovery,
            dependencies,
            validation,
            coverage,
            screens,
        },
    };

    // -----------------------------------------------------------------------
    // Output
    // -----------------------------------------------------------------------
    match args.format {
        AuditFormat::Json => {
            crate::util::print_json(&report)?;
        }
        AuditFormat::Text => {
            print_text_report(&report);
        }
    }

    if !cli.quiet {
        eprintln!();
        eprintln!(
            "Audit complete in {:.1}s. Readiness: {:.1}/100 [{}]",
            start.elapsed().as_secs_f64(),
            overall,
            verdict.to_uppercase()
        );
    }

    if verdict == "ready" {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::from(1))
    }
}

// ---------------------------------------------------------------------------
// Text output
// ---------------------------------------------------------------------------

fn print_text_report(report: &AuditReport) {
    let s = &report.summary;
    let r = &s.readiness;
    eprintln!("cobol2rust audit: {}", report.source_dir);
    eprintln!("================================================");
    eprintln!(
        "Discovery:    {} source files, {} copybooks, {} lines",
        s.total_source_files, s.total_copybook_files, s.total_lines
    );
    let ft = &report.phases.discovery.file_types;
    eprintln!(
        "File types:   {} ASCII, {} ISO-8859, {} Non-ISO, {} binary",
        ft.ascii, ft.iso_8859, ft.non_iso_extended_ascii, ft.binary_data
    );
    eprintln!(
        "Encoding:     {} files with non-ASCII bytes              [{:.1}]",
        report.phases.discovery.encoding_issues.len(),
        r.encoding.score
    );
    eprintln!(
        "Dependencies: {} referenced, {} found, {} missing     [{:.1}]",
        report.phases.dependencies.unique_copybooks_referenced,
        report.phases.dependencies.copybooks_found,
        report.phases.dependencies.copybooks_missing.len(),
        r.dependencies.score
    );
    eprintln!(
        "Validation:   {}/{} parsed OK, {} blocked               [{:.1}]",
        report.phases.validation.files_parsed_ok,
        s.total_source_files,
        report.phases.validation.files_blocked,
        r.parsing.score
    );
    if let Some(ref cov) = report.phases.coverage {
        eprintln!(
            "Coverage:     {:.1}% average transpilation coverage       [{:.1}]",
            cov.average_coverage_pct, r.coverage.score
        );
    } else {
        eprintln!("Coverage:     (skipped)");
    }
    if let Some(ref scr) = report.phases.screens {
        eprintln!(
            "Screens:      {} BMS files, {} maps, {} fields ({} input)",
            scr.total_bms_files, scr.total_maps, scr.total_fields, scr.total_input_fields
        );
        eprintln!(
            "              {} connected, {} orphaned, {} missing",
            scr.connected_maps, scr.orphaned_maps, scr.missing_maps.len()
        );
        if !scr.orphaned_map_names.is_empty() {
            eprintln!("  Orphaned:   {}", scr.orphaned_map_names.join(", "));
        }
        if !scr.missing_maps.is_empty() {
            eprintln!("  Missing:    {}", scr.missing_maps.join(", "));
        }
    }
    if s.read_errors > 0 {
        eprintln!("Read errors:  {} file(s) could not be read", s.read_errors);
    }
    eprintln!("------------------------------------------------");
    eprintln!(
        "Overall Readiness: {:.1} / 100  [{}]",
        r.overall,
        s.verdict.to_uppercase()
    );

    if !report.blockers.is_empty() {
        eprintln!();
        eprintln!("Blockers ({}):", report.blockers.len());
        for b in &report.blockers {
            let files_hint = if b.affected_files.len() > 2 {
                format!(
                    "{}, {} (and {} more)",
                    b.affected_files[0],
                    b.affected_files[1],
                    b.affected_files.len() - 2
                )
            } else {
                b.affected_files.join(", ")
            };
            eprintln!(
                "  [{}] {} -- {}",
                b.category.to_uppercase(),
                b.message,
                files_hint
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn score_entry(score: f64) -> ScoreEntry {
    let status = if score >= 90.0 {
        "ok"
    } else if score >= 60.0 {
        "warning"
    } else {
        "critical"
    };
    ScoreEntry {
        score,
        status: status.to_string(),
    }
}

fn make_progress_bar(total: u64, prefix: &str) -> ProgressBar {
    crate::util::make_progress_bar(total, prefix)
}

fn bytecount_lines(bytes: &[u8]) -> usize {
    bytes.iter().filter(|&&b| b == b'\n').count() + 1
}

/// Classify file encoding by examining byte content.
///
/// Mimics the `file` command's classification:
/// - "ASCII text": all bytes 0-127, has line breaks
/// - "ISO-8859 text": bytes 128-255 present, all in ISO-8859-1 printable range
/// - "Non-ISO extended ASCII text": bytes 128-255 outside ISO-8859-1 printable range
/// - "data": contains NUL bytes or excessive non-printable content (likely binary)
/// Public wrapper for encoding classification, used by `diagnose encoding`.
pub fn classify_encoding_pub(bytes: &[u8]) -> String {
    classify_encoding(bytes)
}

fn classify_encoding(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "ASCII text".to_string();
    }

    let mut has_non_ascii = false;
    let mut has_non_iso = false;
    let mut nul_count = 0usize;
    let mut non_printable_count = 0usize;

    for &b in bytes {
        if b == 0 {
            nul_count += 1;
        } else if b > 127 {
            has_non_ascii = true;
            // ISO-8859-1 printable range: 0xA0-0xFF (plus 0x80-0x9F are control chars)
            if b < 0xA0 {
                has_non_iso = true;
            }
        } else if b < 0x20 && b != b'\n' && b != b'\r' && b != b'\t' && b != 0x0C {
            non_printable_count += 1;
        }
    }

    // Binary: has NUL bytes or >10% non-printable
    if nul_count > 0 || non_printable_count * 10 > bytes.len() {
        return "data".to_string();
    }

    if !has_non_ascii {
        "ASCII text".to_string()
    } else if has_non_iso {
        "Non-ISO extended ASCII text".to_string()
    } else {
        "ISO-8859 text".to_string()
    }
}

fn chrono_timestamp() -> String {
    // Simple ISO 8601 without pulling in chrono crate.
    // Uses seconds since UNIX epoch as a fallback.
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}Z", dur.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_entry_thresholds() {
        assert_eq!(score_entry(95.0).status, "ok");
        assert_eq!(score_entry(75.0).status, "warning");
        assert_eq!(score_entry(40.0).status, "critical");
    }

    #[test]
    fn bytecount_lines_basic() {
        assert_eq!(bytecount_lines(b"a\nb\nc"), 3);
        assert_eq!(bytecount_lines(b"single"), 1);
        assert_eq!(bytecount_lines(b""), 1);
    }

    #[test]
    fn readiness_score_formula() {
        let enc: f64 = 100.0;
        let dep: f64 = 50.0;
        let parse: f64 = 80.0;
        let cov: f64 = 70.0;
        let overall = 0.15 * enc + 0.35 * dep + 0.35 * parse + 0.15 * cov;
        // 15 + 17.5 + 28 + 10.5 = 71.0
        assert!((overall - 71.0).abs() < 0.01);
    }
}
