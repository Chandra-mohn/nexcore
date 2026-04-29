//! cobol-rustify: Phase 2 rustification engine.
//!
//! Transforms transpiler-generated "COBOL-shaped Rust" into idiomatic Rust
//! through a tiered, idempotent promotion pipeline.
//!
//! See `docs/cobol2rust_rustify_spec.md` for full specification.

pub mod config;
pub mod dsl;
pub mod error;
pub mod hints;
pub mod manifest;
pub mod patch;
pub mod plan;
pub mod report;
pub mod review;
pub mod rules;
pub mod safety;
pub mod target_config;

use std::collections::HashMap;
use std::path::Path;
#[cfg(feature = "direct-emit")]
use std::path::PathBuf;

use config::RustifyConfig;
use error::RustifyError;
use manifest::{compute_checksum, FileManifest, Manifest, ManifestSummary};
use report::AnalysisReport;
use rules::{AnalysisContext, RuleRegistry};

/// Analyze a workspace and return proposed transforms (report mode).
///
/// Reads Rust source files from `source_dir`, runs rules for the configured
/// tier, and returns an `AnalysisReport` with all proposed transforms.
///
/// # Errors
///
/// Returns `RustifyError::SourceNotFound` if `config.source_dir` does not exist.
/// Returns `RustifyError::Io` if reading source files or hints fails.
/// Returns `RustifyError::Json` if `hints.json` is malformed.
pub fn analyze_workspace(config: &RustifyConfig) -> Result<AnalysisReport, RustifyError> {
    let source_dir = &config.source_dir;
    if !source_dir.exists() {
        return Err(RustifyError::SourceNotFound(source_dir.clone()));
    }

    let registry = RuleRegistry::new();
    let rules = registry.filtered_rules(config.tier, &config.only_rules, &config.skip_rules);
    let hints = hints::read_hints(source_dir)?;

    // Find all .rs files (src/ or flat layout)
    let src_dir = resolve_rs_dir(source_dir);
    let rs_files = collect_rs_files(&src_dir, config.include.as_deref(), config.exclude.as_deref());

    let mut all_transforms = Vec::new();

    for file_path in &rs_files {
        let Ok(source_text) = std::fs::read_to_string(file_path) else {
            continue;
        };

        let parsed = match syn::parse_file(&source_text) {
            Ok(f) => f,
            Err(e) => {
                tracing::warn!(path = %file_path.display(), error = %e, "skipping file due to parse error");
                continue;
            }
        };

        let rel_path = file_path
            .strip_prefix(source_dir)
            .unwrap_or(file_path);
        let file_hints = hints.as_ref().and_then(|h| {
            h.files.get(&rel_path.to_string_lossy().replace('\\', "/"))
        });

        let ctx = AnalysisContext {
            source: &parsed,
            source_text: &source_text,
            file_path,
            hints: file_hints,
        };

        for rule in &rules {
            let transforms = rule.analyze(&ctx);
            all_transforms.extend(transforms);
        }
    }

    Ok(AnalysisReport {
        tier: config.tier,
        transforms: all_transforms,
        files_processed: rs_files.len(),
    })
}

/// Result of applying transforms to a workspace.
#[derive(Debug)]
pub struct ApplyReport {
    /// Analysis report (transforms proposed).
    pub analysis: AnalysisReport,
    /// Number of files copied to output.
    pub files_copied: usize,
    /// Number of files where transforms were applied.
    pub files_transformed: usize,
    /// Files that were preserved (user patches kept).
    pub files_preserved: Vec<String>,
}

/// Apply transforms and write to an output directory.
///
/// Copies the source workspace to `output_dir`, applies transforms to .rs files,
/// and writes a manifest recording checksums and provenance.
///
/// # Errors
///
/// Returns `RustifyError::Io` if `config.output_dir` is `None`.
/// Returns `RustifyError::SourceNotFound` if `config.source_dir` does not exist.
/// Returns `RustifyError::SameDirectory` if source and output directories are the same.
/// Returns `RustifyError::PatchesDetected` if the output has user patches and neither
/// `--force` nor `--preserve-patches` is set.
/// Returns `RustifyError::Io` if file copying, writing, or manifest serialization fails.
/// Returns `RustifyError::Json` if `hints.json` is malformed.
pub fn apply_workspace(config: &RustifyConfig) -> Result<ApplyReport, RustifyError> {
    let source_dir = &config.source_dir;
    let output_dir = match &config.output_dir {
        Some(d) => d.clone(),
        None => return Err(RustifyError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "output directory required for apply mode",
        ))),
    };

    if !source_dir.exists() {
        return Err(RustifyError::SourceNotFound(source_dir.clone()));
    }

    // Prevent in-place modification
    if source_dir == &output_dir {
        return Err(RustifyError::SameDirectory);
    }
    let source_canon = std::fs::canonicalize(source_dir).unwrap_or(source_dir.clone());
    if output_dir.exists() {
        let output_canon = std::fs::canonicalize(&output_dir).unwrap_or(output_dir.clone());
        if source_canon == output_canon {
            return Err(RustifyError::SameDirectory);
        }
    }

    // Step 1: Patch detection on existing output
    let mut files_preserved = Vec::new();
    let patch_detection = patch::detect_patches(&output_dir)?;
    if let Some(ref pd) = patch_detection {
        if pd.has_patches() {
            if !config.force && !config.preserve_patches {
                return Err(RustifyError::PatchesDetected(output_dir.clone()));
            }
            if config.preserve_patches {
                files_preserved.clone_from(&pd.modified);
            }
        }
    }

    // Step 2: Set up rules and hints
    let registry = RuleRegistry::new();
    let rules = registry.filtered_rules(config.tier, &config.only_rules, &config.skip_rules);
    let hints = hints::read_hints(source_dir)?;

    // Step 3: Copy source to output (excluding rustify/ and preserved files)
    let files_copied = copy_workspace(source_dir, &output_dir, &files_preserved)?;

    // Copy hints.json from source if present
    let source_hints = source_dir.join("rustify").join("hints.json");
    if source_hints.exists() {
        let rustify_dir = output_dir.join("rustify");
        std::fs::create_dir_all(&rustify_dir)?;
        std::fs::copy(&source_hints, rustify_dir.join("hints.json"))?;
    }

    // Step 4: Process .rs files -- analyze and apply transforms (src/ or flat layout)
    let src_dir = resolve_rs_dir(source_dir);
    let rs_files = collect_rs_files(&src_dir, config.include.as_deref(), config.exclude.as_deref());

    let mut all_transforms = Vec::new();
    let mut manifest_files = HashMap::new();
    let mut files_changed = 0;
    let mut files_unchanged = 0;
    let mut files_transformed = 0;

    for file_path in &rs_files {
        let rel_path = file_path
            .strip_prefix(source_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/");

        // If preserving patches, skip modified files
        if files_preserved.contains(&rel_path) {
            continue;
        }

        let Ok(source_text) = std::fs::read_to_string(file_path) else {
            continue;
        };

        let checksum_before = compute_checksum(source_text.as_bytes());

        let parsed = match syn::parse_file(&source_text) {
            Ok(f) => f,
            Err(e) => {
                tracing::warn!(path = %file_path.display(), error = %e, "skipping file due to parse error");
                continue;
            }
        };

        let file_hints = hints.as_ref().and_then(|h| h.files.get(&rel_path));

        let ctx = AnalysisContext {
            source: &parsed,
            source_text: &source_text,
            file_path,
            hints: file_hints,
        };

        let mut file_transforms = Vec::new();
        for rule in &rules {
            let transforms = rule.analyze(&ctx);
            file_transforms.extend(transforms);
        }

        // Apply text edits from transforms
        let all_edits: Vec<_> = file_transforms
            .iter()
            .flat_map(|t| &t.edits)
            .cloned()
            .collect();
        let output_text = rules::transform::apply_edits(&source_text, &all_edits);

        // Apply const-extract insertions if any
        let insertions =
            rules::tier1::const_extract::generate_const_insertions(&output_text, &file_transforms);
        let output_text = rules::transform::apply_insertions(&output_text, &insertions);
        let checksum_after = compute_checksum(output_text.as_bytes());

        // Write to output directory
        let output_path = output_dir.join(&rel_path);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&output_path, &output_text)?;

        let transforms_applied = file_transforms.len();
        let mut rules_hit = HashMap::new();
        for t in &file_transforms {
            *rules_hit.entry(t.rule_id.clone()).or_insert(0usize) += 1;
        }
        let rules_hit_vec: Vec<String> = rules_hit
            .iter()
            .map(|(k, v)| format!("{k}:{v}"))
            .collect();

        if checksum_before == checksum_after {
            files_unchanged += 1;
        } else {
            files_changed += 1;
            files_transformed += 1;
        }
        if !file_transforms.is_empty() {
            files_transformed = files_transformed.max(1);
        }

        manifest_files.insert(
            rel_path,
            FileManifest {
                checksum_before,
                checksum_after,
                transforms_applied,
                rules_hit: rules_hit_vec,
            },
        );

        all_transforms.extend(file_transforms);
    }

    // Step 5: Build and write manifest
    let rules_applied: Vec<String> = rules.iter().map(|r| r.id().to_string()).collect();

    let mut by_rule = HashMap::new();
    for t in &all_transforms {
        *by_rule.entry(t.rule_id.clone()).or_insert(0usize) += 1;
    }

    let manifest = Manifest {
        version: "1.0".to_string(),
        cobol2rust_version: env!("CARGO_PKG_VERSION").to_string(),
        source: source_dir.to_string_lossy().to_string(),
        source_resolved: source_canon.to_string_lossy().to_string(),
        tier: config.tier as u8,
        timestamp: now_iso8601(),
        rules_applied,
        rules_skipped: config.skip_rules.clone(),
        include_filter: config.include.clone(),
        exclude_filter: config.exclude.clone(),
        files: manifest_files,
        summary: ManifestSummary {
            files_processed: rs_files.len(),
            files_changed,
            files_unchanged,
            total_transforms: all_transforms.len(),
            by_rule,
        },
    };

    manifest::write_manifest(&output_dir, &manifest)?;

    // Step 6: Write review.md if any review items
    review::write_review(&output_dir, &all_transforms, config.tier as u8)?;

    // Step 7: Write plan.md if any assessment items (Tier 3)
    plan::write_plan(&output_dir, &all_transforms, config.tier as u8)?;

    // Step 8: Emit Nexflow DSL files (Tier 5) if requested
    if config.emit_dsl {
        // Filter assessment-only transforms (Tier 3) for emitter context
        let assessments: Vec<_> = all_transforms
            .iter()
            .filter(|t| matches!(t.safety, rules::transform::Safety::Assessment))
            .cloned()
            .collect();
        emit_dsl_for_workspace_with_assessments(config, &output_dir, &assessments)?;
    }

    Ok(ApplyReport {
        analysis: AnalysisReport {
            tier: config.tier,
            transforms: all_transforms,
            files_processed: rs_files.len(),
        },
        files_copied,
        files_transformed,
        files_preserved,
    })
}

/// Copy a workspace directory to output, excluding `rustify/` and preserved files.
fn copy_workspace(
    source: &Path,
    output: &Path,
    preserved: &[String],
) -> Result<usize, RustifyError> {
    let mut count = 0;
    copy_dir_recursive(source, source, output, preserved, &mut count)?;
    Ok(count)
}

fn copy_dir_recursive(
    root: &Path,
    src: &Path,
    dst: &Path,
    preserved: &[String],
    count: &mut usize,
) -> Result<(), RustifyError> {
    std::fs::create_dir_all(dst)?;
    let entries = std::fs::read_dir(src)?;
    for entry in entries {
        let entry = entry?;
        let src_path = entry.path();
        let file_name = entry.file_name();

        // Skip rustify/ and dsl/ directories (we manage them ourselves)
        if file_name == "rustify" || file_name == "dsl" {
            continue;
        }

        let dst_path = dst.join(&file_name);

        if src_path.is_dir() {
            copy_dir_recursive(root, &src_path, &dst_path, preserved, count)?;
        } else {
            // Check if this file is preserved (user-patched)
            let rel = src_path
                .strip_prefix(root)
                .unwrap_or(&src_path)
                .to_string_lossy()
                .replace('\\', "/");
            if preserved.contains(&rel) {
                continue;
            }
            std::fs::copy(&src_path, &dst_path)?;
            *count += 1;
        }
    }
    Ok(())
}

/// ISO 8601 timestamp (simple UTC).
fn now_iso8601() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs();
    // Simple calculation -- no chrono dependency needed
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;

    // Days since epoch to Y-M-D (simplified leap year calc)
    let (year, month, day) = days_to_ymd(days);
    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

fn days_to_ymd(mut days: u64) -> (u64, u64, u64) {
    // Algorithm from https://howardhinnant.github.io/date_algorithms.html
    days += 719_468;
    let era = days / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// Emit Nexflow DSL files for all .rs source files in the workspace.
///
/// Routes through `emit_dsl_routed()` based on `config.emit_mode`:
/// - Legacy: parses transpiled Rust via syn (default)
/// - Direct: parses original COBOL source via cobol-transpiler (requires `direct-emit` feature)
/// - Compare: runs both paths and diffs output
///
/// Direct/compare modes require `config.cobol_source_dir` to locate original .cbl files.
/// The COBOL source paths are resolved via `cobol2rust-manifest.toml` in the workspace.
///
/// # Errors
///
/// Returns `RustifyError::Io` if reading source files or writing DSL output fails.
/// Returns `RustifyError::Io` if direct/compare mode is requested but `cobol_source_dir` is not set.
pub fn emit_dsl_for_workspace(
    config: &RustifyConfig,
    output_dir: &Path,
) -> Result<Vec<dsl::writer::DslWriteReport>, RustifyError> {
    emit_dsl_for_workspace_with_assessments(config, output_dir, &[])
}

/// Like `emit_dsl_for_workspace` but also passes Tier 3 assessment results
/// to emitters for richer DSL output.
pub fn emit_dsl_for_workspace_with_assessments(
    config: &RustifyConfig,
    output_dir: &Path,
    assessments: &[rules::transform::Transform],
) -> Result<Vec<dsl::writer::DslWriteReport>, RustifyError> {
    let needs_direct = config.emit_mode != config::EmitMode::Legacy
        || !config.emitter_overrides.is_empty();

    // Load COBOL source mapping from manifest when direct path is needed
    #[cfg(feature = "direct-emit")]
    let cobol_source_map: HashMap<String, PathBuf> = if needs_direct {
        let cobol_root = config.cobol_source_dir.as_ref().ok_or_else(|| {
            RustifyError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "direct/compare emit mode requires --cobol-source <DIR>",
            ))
        })?;
        load_cobol_source_map(output_dir, cobol_root)?
    } else {
        HashMap::new()
    };

    #[cfg(not(feature = "direct-emit"))]
    if needs_direct {
        return Err(RustifyError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "direct-emit feature not enabled; rebuild with --features direct-emit",
        )));
    }

    // Load hints once (from <output_dir>/rustify/hints.json if present)
    let hints_file = hints::read_hints(output_dir)?;

    // Load target config (from <output_dir>/.cobol2rust-target.toml if present)
    let target_config = target_config::load_target_config(output_dir)?;

    let src_dir = resolve_rs_dir(output_dir);
    let rs_files = collect_rs_files(&src_dir, None, None);

    let mut reports = Vec::new();
    let mut errors = Vec::new();

    for rs_path in &rs_files {
        let source_text = match std::fs::read_to_string(rs_path) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!(path = %rs_path.display(), error = %e, "Cannot read Rust file");
                continue;
            }
        };
        let syn_file = match syn::parse_str::<syn::File>(&source_text) {
            Ok(f) => f,
            Err(e) => {
                tracing::warn!(path = %rs_path.display(), error = %e, "Cannot parse Rust file");
                continue;
            }
        };

        let program_name = dsl::cobol_attrs::extract_program_name(&syn_file)
            .unwrap_or_default();
        if program_name.is_empty() {
            continue; // Not a COBOL-generated file
        }

        // Look up per-file hints by relative path
        let rel_path = rs_path
            .strip_prefix(output_dir)
            .unwrap_or(rs_path)
            .to_string_lossy()
            .replace('\\', "/");
        let file_hints = hints_file.as_ref().and_then(|h| h.files.get(&rel_path));

        // Resolve per-program target config overrides
        let resolved_target = target_config::resolve_for_program(&target_config, &program_name);

        // Filter assessments for this file
        let file_assessments: Vec<_> = assessments
            .iter()
            .filter(|a| {
                a.file.to_string_lossy().replace('\\', "/").ends_with(&rel_path)
            })
            .cloned()
            .collect();

        let legacy_ctx = dsl::EmitterContext {
            program_name: program_name.clone(),
            syn_file: &syn_file,
            source_text: &source_text,
            hints: file_hints,
            assessments: &file_assessments,
            target: Some(&resolved_target),
            source_path: rs_path.clone(),
        };

        // Route through emit_dsl_routed for proper mode dispatch
        let dsl_files = match emit_dsl_for_program(
            config,
            &legacy_ctx,
            &program_name,
            #[cfg(feature = "direct-emit")]
            &cobol_source_map,
        ) {
            Ok(files) => files,
            Err(e) => {
                tracing::warn!(
                    program = %program_name,
                    error = %e,
                    "DSL emission failed, skipping program"
                );
                errors.push(dsl::writer::ProgramError {
                    program: program_name,
                    error: e.to_string(),
                });
                continue;
            }
        };

        if dsl_files.is_empty() {
            continue;
        }

        match dsl::writer::write_dsl_files(
            output_dir,
            &program_name,
            &rel_path,
            &dsl_files,
        ) {
            Ok(report) => reports.push(report),
            Err(e) => {
                tracing::warn!(
                    program = %program_name,
                    error = %e,
                    "DSL file write failed, skipping program"
                );
                errors.push(dsl::writer::ProgramError {
                    program: program_name,
                    error: e.to_string(),
                });
            }
        }
    }

    // Write aggregated workspace manifest once, after all programs
    dsl::writer::write_workspace_manifest(output_dir, &reports, &errors)?;

    if !errors.is_empty() {
        tracing::warn!(
            succeeded = reports.len(),
            failed = errors.len(),
            "DSL emission completed with errors"
        );
    }

    Ok(reports)
}

/// Emit DSL files for a single program, routing through the configured emit mode.
#[cfg(not(feature = "direct-emit"))]
fn emit_dsl_for_program(
    config: &RustifyConfig,
    legacy_ctx: &dsl::EmitterContext<'_>,
    _program_name: &str,
) -> Result<Vec<dsl::DslFile>, RustifyError> {
    let (files, _) = dsl::writer::emit_dsl_routed(legacy_ctx, config.emit_mode)?;
    Ok(files)
}

/// Emit DSL files for a single program, routing through the configured emit mode.
/// When direct/compare mode is active, parses the original COBOL source.
#[cfg(feature = "direct-emit")]
fn emit_dsl_for_program(
    config: &RustifyConfig,
    legacy_ctx: &dsl::EmitterContext<'_>,
    program_name: &str,
    cobol_source_map: &HashMap<String, PathBuf>,
) -> Result<Vec<dsl::DslFile>, RustifyError> {
    let needs_direct = config.emit_mode != config::EmitMode::Legacy
        || !config.emitter_overrides.is_empty();

    let direct_ctx_holder: Option<(cobol_transpiler::ast::CobolProgram, PathBuf)>;
    let direct_ctx = if needs_direct {
        let cobol_path = cobol_source_map.get(program_name).ok_or_else(|| {
            RustifyError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "COBOL source for program '{}' not found in manifest; \
                     check cobol2rust-manifest.toml and --cobol-source path",
                    program_name
                ),
            ))
        })?;

        let cobol_source = std::fs::read_to_string(cobol_path).map_err(|e| {
            RustifyError::Io(std::io::Error::new(
                e.kind(),
                format!("failed to read COBOL source {}: {}", cobol_path.display(), e),
            ))
        })?;

        tracing::info!(
            program = %program_name,
            path = %cobol_path.display(),
            "Parsing COBOL source for direct DSL emission"
        );

        let cobol_program = cobol_transpiler::parser::parse_cobol(&cobol_source)
            .map_err(|e| {
                RustifyError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("failed to parse COBOL source {}: {}", cobol_path.display(), e),
                ))
            })?;

        direct_ctx_holder = Some((cobol_program, cobol_path.clone()));
        let (ref prog, ref path) = *direct_ctx_holder.as_ref().unwrap();
        Some(dsl::direct::DirectEmitterContext {
            cobol_program: prog,
            program_name: program_name.to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: path.clone(),
        })
    } else {
        direct_ctx_holder = None;
        #[allow(clippy::let_unit_value)]
        let _ = &direct_ctx_holder; // suppress unused-assignment warning
        None
    };

    let (files, comparison) = dsl::writer::emit_dsl_routed(
        legacy_ctx,
        direct_ctx.as_ref(),
        config.emit_mode,
        &config.emitter_overrides,
    )?;

    if let Some(report) = comparison {
        report.print_summary();
    }

    Ok(files)
}

#[cfg(feature = "direct-emit")]
/// Load the program-name -> COBOL source path mapping from `cobol2rust-manifest.toml`.
///
/// The manifest stores relative source paths per program. We resolve them
/// against `cobol_root` to get absolute paths.
fn load_cobol_source_map(
    workspace_dir: &Path,
    cobol_root: &Path,
) -> Result<HashMap<String, PathBuf>, RustifyError> {
    let manifest_path = workspace_dir.join("cobol2rust-manifest.toml");
    if !manifest_path.exists() {
        return Err(RustifyError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "cobol2rust-manifest.toml not found in {}; \
                 run `nexmig transpile --workspace` first",
                workspace_dir.display()
            ),
        )));
    }

    let content = std::fs::read_to_string(&manifest_path)?;
    let table: toml::Table = content.parse().map_err(|e: toml::de::Error| {
        RustifyError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid cobol2rust-manifest.toml: {}", e),
        ))
    })?;

    let mut map = HashMap::new();
    if let Some(programs) = table.get("programs").and_then(|v| v.as_table()) {
        for (_crate_name, info) in programs {
            let info = match info.as_table() {
                Some(t) => t,
                None => continue,
            };
            let source_rel = match info.get("source").and_then(|v| v.as_str()) {
                Some(s) => s,
                None => continue,
            };
            // Prefer explicit program_id from manifest (added in v0.3.4).
            // Fall back to uppercased filename stem for older manifests.
            let program_id = if let Some(id) = info.get("program_id").and_then(|v| v.as_str()) {
                id.to_string()
            } else {
                let source_path = std::path::Path::new(source_rel);
                source_path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_uppercase()
            };

            let absolute_path = cobol_root.join(source_rel);
            map.insert(program_id, absolute_path);
        }
    }

    if map.is_empty() {
        tracing::warn!(
            manifest = %manifest_path.display(),
            "No programs found in cobol2rust-manifest.toml"
        );
    } else {
        tracing::info!(
            programs = map.len(),
            "Loaded COBOL source mapping from manifest"
        );
    }

    Ok(map)
}

/// List all available rules.
pub fn list_rules() -> Vec<rules::RuleInfo> {
    let registry = RuleRegistry::new();
    registry.list_rules()
}

/// Resolve the directory containing .rs files.
///
/// Prefers `<base>/src/` if it exists and contains .rs files,
/// otherwise falls back to `<base>/` itself (flat layout).
fn resolve_rs_dir(base: &Path) -> std::path::PathBuf {
    let src = base.join("src");
    if src.exists() && std::fs::read_dir(&src).ok().is_some_and(|mut entries| {
        entries.any(|e| e.ok().is_some_and(|e| e.path().extension().is_some_and(|ext| ext == "rs")))
    }) {
        src
    } else {
        base.to_path_buf()
    }
}

/// Collect .rs files from a directory, applying include/exclude filters.
fn collect_rs_files(
    dir: &Path,
    include: Option<&str>,
    exclude: Option<&str>,
) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    if !dir.exists() {
        return files;
    }

    let walker = walkdir(dir);
    for entry in walker {
        let path = entry;
        if path.extension().is_some_and(|e| e == "rs") {
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            if let Some(inc) = include {
                if !glob_match(inc, &name) {
                    continue;
                }
            }
            if let Some(exc) = exclude {
                if glob_match(exc, &name) {
                    continue;
                }
            }

            files.push(path);
        }
    }

    files.sort();
    files
}

/// Simple recursive directory walk (no external dependency).
fn walkdir(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                result.extend(walkdir(&path));
            } else {
                result.push(path);
            }
        }
    }
    result
}

/// Simple glob matching (supports * and ? only).
fn glob_match(pattern: &str, name: &str) -> bool {
    let pattern = pattern.as_bytes();
    let name = name.as_bytes();
    glob_match_bytes(pattern, name)
}

fn glob_match_bytes(pattern: &[u8], name: &[u8]) -> bool {
    let mut pi = 0;
    let mut ni = 0;
    let mut star_pi = usize::MAX;
    let mut star_ni = 0;

    while ni < name.len() {
        if pi < pattern.len() && (pattern[pi] == b'?' || pattern[pi] == name[ni]) {
            pi += 1;
            ni += 1;
        } else if pi < pattern.len() && pattern[pi] == b'*' {
            star_pi = pi;
            star_ni = ni;
            pi += 1;
        } else if star_pi != usize::MAX {
            pi = star_pi + 1;
            star_ni += 1;
            ni = star_ni;
        } else {
            return false;
        }
    }

    while pi < pattern.len() && pattern[pi] == b'*' {
        pi += 1;
    }

    pi == pattern.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use manifest::compute_file_checksum;

    #[test]
    fn glob_exact_match() {
        assert!(glob_match("foo.rs", "foo.rs"));
    }

    #[test]
    fn glob_star() {
        assert!(glob_match("*.rs", "program_a.rs"));
        assert!(glob_match("acct*", "acct_main.rs"));
        assert!(!glob_match("*.rs", "program_a.txt"));
    }

    #[test]
    fn glob_question() {
        assert!(glob_match("foo?.rs", "foo1.rs"));
        assert!(!glob_match("foo?.rs", "foo12.rs"));
    }

    #[test]
    fn analyze_nonexistent_dir() {
        let config = RustifyConfig {
            source_dir: "/tmp/cobol2rust_nonexistent_dir_xyz".into(),
            ..RustifyConfig::default()
        };
        let result = analyze_workspace(&config);
        assert!(result.is_err());
    }

    #[test]
    fn analyze_empty_dir() {
        let dir = std::env::temp_dir().join("cobol2rust_test_empty_ws");
        let _ = std::fs::create_dir_all(dir.join("src"));
        let config = RustifyConfig {
            source_dir: dir.clone(),
            ..RustifyConfig::default()
        };
        let report = analyze_workspace(&config).unwrap();
        assert_eq!(report.files_processed, 0);
        assert!(report.transforms.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    // ---- Session 51: apply_workspace tests ----

    fn setup_test_workspace(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let base = std::env::temp_dir().join(format!("cobol2rust_test_apply_{name}"));
        let source = base.join("source");
        let output = base.join("output");
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(source.join("src")).unwrap();
        (source, output)
    }

    fn cleanup_test(name: &str) {
        let base = std::env::temp_dir().join(format!("cobol2rust_test_apply_{name}"));
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn apply_copies_files_and_writes_manifest() {
        let (source, output) = setup_test_workspace("basic_copy");

        // Create a minimal Rust workspace in source
        std::fs::write(
            source.join("Cargo.toml"),
            "[package]\nname = \"test\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        std::fs::write(source.join("src/main.rs"), "fn main() {}\n").unwrap();
        std::fs::write(source.join("src/lib.rs"), "pub fn hello() {}\n").unwrap();

        let config = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            ..RustifyConfig::default()
        };

        let report = apply_workspace(&config).unwrap();

        // Files copied
        assert!(output.join("Cargo.toml").exists());
        assert!(output.join("src/main.rs").exists());
        assert!(output.join("src/lib.rs").exists());
        assert_eq!(report.files_copied, 3); // Cargo.toml + 2 rs files

        // Manifest written
        let manifest = manifest::read_manifest(&output).unwrap().unwrap();
        assert_eq!(manifest.files.len(), 2); // 2 .rs files tracked
        assert!(manifest.files.contains_key("src/main.rs"));
        assert!(manifest.files.contains_key("src/lib.rs"));

        // Checksums match actual file contents
        for (rel_path, fm) in &manifest.files {
            let actual = compute_file_checksum(&output.join(rel_path)).unwrap();
            assert_eq!(actual, fm.checksum_after);
        }

        cleanup_test("basic_copy");
    }

    #[test]
    fn apply_produces_identical_output() {
        let (source, output) = setup_test_workspace("identity");

        std::fs::write(source.join("src/test.rs"), "fn foo() -> i32 { 42 }\n").unwrap();

        let config = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            ..RustifyConfig::default()
        };

        apply_workspace(&config).unwrap();

        // Source and output content should be identical (no rules applied yet)
        let src_content = std::fs::read_to_string(source.join("src/test.rs")).unwrap();
        let out_content = std::fs::read_to_string(output.join("src/test.rs")).unwrap();
        assert_eq!(src_content, out_content);

        cleanup_test("identity");
    }

    #[test]
    fn apply_detects_patches_on_rerun() {
        let (source, output) = setup_test_workspace("patch_detect");

        std::fs::write(source.join("src/test.rs"), "fn foo() {}\n").unwrap();

        let config = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            ..RustifyConfig::default()
        };

        // First run
        apply_workspace(&config).unwrap();

        // User modifies a file in output
        std::fs::write(output.join("src/test.rs"), "fn foo() { /* user edit */ }\n").unwrap();

        // Second run without --force should fail
        let result = apply_workspace(&config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("patches"),
            "Expected patches error, got: {err}"
        );

        cleanup_test("patch_detect");
    }

    #[test]
    fn apply_force_overwrites_patches() {
        let (source, output) = setup_test_workspace("force_overwrite");

        std::fs::write(source.join("src/test.rs"), "fn foo() {}\n").unwrap();

        let config_first = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            ..RustifyConfig::default()
        };
        apply_workspace(&config_first).unwrap();

        // User modifies a file
        std::fs::write(output.join("src/test.rs"), "fn foo() { /* patched */ }\n").unwrap();

        // Run with --force
        let config_force = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            force: true,
            ..RustifyConfig::default()
        };
        let report = apply_workspace(&config_force).unwrap();
        assert!(report.files_preserved.is_empty());

        // File should be overwritten back to source content
        let content = std::fs::read_to_string(output.join("src/test.rs")).unwrap();
        assert_eq!(content, "fn foo() {}\n");

        cleanup_test("force_overwrite");
    }

    #[test]
    fn apply_preserve_patches_keeps_user_edits() {
        let (source, output) = setup_test_workspace("preserve_patches");

        std::fs::write(source.join("src/test.rs"), "fn foo() {}\n").unwrap();
        std::fs::write(source.join("src/other.rs"), "fn bar() {}\n").unwrap();

        let config_first = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            ..RustifyConfig::default()
        };
        apply_workspace(&config_first).unwrap();

        // User modifies one file
        std::fs::write(output.join("src/test.rs"), "fn foo() { /* my fix */ }\n").unwrap();

        // Run with --preserve-patches
        let config_preserve = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            preserve_patches: true,
            ..RustifyConfig::default()
        };
        let report = apply_workspace(&config_preserve).unwrap();
        assert_eq!(report.files_preserved.len(), 1);
        assert!(report.files_preserved.contains(&"src/test.rs".to_string()));

        // Patched file should keep user's content
        let patched = std::fs::read_to_string(output.join("src/test.rs")).unwrap();
        assert!(patched.contains("my fix"));

        // Other file should be fresh from source
        let other = std::fs::read_to_string(output.join("src/other.rs")).unwrap();
        assert_eq!(other, "fn bar() {}\n");

        cleanup_test("preserve_patches");
    }

    #[test]
    fn apply_same_directory_errors() {
        let (source, _output) = setup_test_workspace("same_dir");
        std::fs::write(source.join("src/test.rs"), "fn foo() {}\n").unwrap();

        let config = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(source.clone()),
            ..RustifyConfig::default()
        };
        let result = apply_workspace(&config);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string().contains("different"),
            "Expected 'different' in error message"
        );

        cleanup_test("same_dir");
    }

    #[test]
    fn apply_copies_hints_from_source() {
        let (source, output) = setup_test_workspace("hints_copy");

        std::fs::write(source.join("src/test.rs"), "fn foo() {}\n").unwrap();

        // Create hints in source
        let hints = hints::HintsFile {
            version: "1.0".to_string(),
            files: HashMap::new(),
        };
        hints::write_hints(&source, &hints).unwrap();

        let config = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            ..RustifyConfig::default()
        };
        apply_workspace(&config).unwrap();

        // Hints should be copied to output
        assert!(output.join("rustify/hints.json").exists());

        cleanup_test("hints_copy");
    }

    #[test]
    fn apply_excludes_rustify_dir_from_copy() {
        let (source, output) = setup_test_workspace("exclude_rustify");

        std::fs::write(source.join("src/test.rs"), "fn foo() {}\n").unwrap();

        // Create some rustify/ content in source
        let rustify_dir = source.join("rustify");
        std::fs::create_dir_all(&rustify_dir).unwrap();
        std::fs::write(rustify_dir.join("old_manifest.json"), "{}").unwrap();

        let config = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            ..RustifyConfig::default()
        };
        apply_workspace(&config).unwrap();

        // old_manifest.json should NOT be copied (only our new manifest)
        assert!(!output.join("rustify/old_manifest.json").exists());
        // But our manifest should exist
        assert!(output.join("rustify/manifest.json").exists());

        cleanup_test("exclude_rustify");
    }

    #[test]
    fn idempotent_apply() {
        let (source, output) = setup_test_workspace("idempotent");

        std::fs::write(source.join("src/test.rs"), "fn hello() -> &'static str { \"world\" }\n")
            .unwrap();

        let config = RustifyConfig {
            source_dir: source.clone(),
            output_dir: Some(output.clone()),
            ..RustifyConfig::default()
        };

        // First apply
        apply_workspace(&config).unwrap();
        let first_checksum = compute_file_checksum(&output.join("src/test.rs")).unwrap();

        // Second apply (with --force to allow overwrite)
        let config_force = RustifyConfig {
            force: true,
            ..config
        };
        apply_workspace(&config_force).unwrap();
        let second_checksum = compute_file_checksum(&output.join("src/test.rs")).unwrap();

        assert_eq!(first_checksum, second_checksum);

        cleanup_test("idempotent");
    }

    // ---- E2E tests for direct-emit integration ----

    /// Minimal COBOL source that exercises all 4 emitters.
    const TEST_COBOL: &str = "\
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TESTDSL.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-ACCT-NUMBER        PIC X(10).
       01  WS-ACCT-BALANCE       PIC S9(9)V99 COMP-3.
       01  WS-ACCT-TYPE          PIC X(1).
           88 SAVINGS             VALUE 'S'.
           88 CHECKING            VALUE 'C'.
       01  WS-RESULT             PIC X(20).
       PROCEDURE DIVISION.
       MAIN-SECTION SECTION.
       MAIN-PARA.
           PERFORM VALIDATE-PARA
           PERFORM PROCESS-PARA
           STOP RUN.
       VALIDATE-PARA.
           IF WS-ACCT-TYPE = 'S'
               MOVE 'SAVINGS-OK' TO WS-RESULT
           ELSE
               MOVE 'CHECK-OK' TO WS-RESULT
           END-IF.
       PROCESS-PARA.
           COMPUTE WS-ACCT-BALANCE =
               WS-ACCT-BALANCE + 100.
";

    /// Transpiled Rust that matches TEST_COBOL (simulates transpiler output).
    const TEST_TRANSPILED_RUST: &str = r#"
        #![allow(unused_imports, unused_variables, non_snake_case)]
        use cobol_runtime::prelude::*;

        #[cobol(program = "TESTDSL")]
        pub struct WorkingStorage {
            #[cobol(level = 1, pic = "X(10)")]
            pub ws_acct_number: PicX,
            #[cobol(level = 1, pic = "S9(9)V99", comp3, signed)]
            pub ws_acct_balance: PackedDecimal,
            #[cobol(level = 1, pic = "X(1)", level88 = "SAVINGS:S,CHECKING:C")]
            pub ws_acct_type: PicX,
            #[cobol(level = 1, pic = "X(20)")]
            pub ws_result: PicX,
        }

        #[cobol(section = "MAIN-SECTION", performs = "VALIDATE-PARA,PROCESS-PARA")]
        fn main_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            validate_para(ws, ctx);
            process_para(ws, ctx);
        }

        #[cobol(section = "MAIN-SECTION", reads = "WS-ACCT-TYPE", writes = "WS-RESULT")]
        fn validate_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            if ws.ws_acct_type.to_string() == "S" {
                ws.ws_result.set("SAVINGS-OK");
            } else {
                ws.ws_result.set("CHECK-OK");
            }
        }

        #[cobol(section = "MAIN-SECTION", reads = "WS-ACCT-BALANCE", writes = "WS-ACCT-BALANCE")]
        fn process_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            ws.ws_acct_balance.pack(ws.ws_acct_balance.unpack() + dec!(100));
        }
    "#;

    /// Manifest TOML that maps TESTDSL to its COBOL source.
    fn test_manifest_toml(cobol_rel_path: &str) -> String {
        format!(
            "# test manifest\n\
             [programs.testdsl]\n\
             program_id = \"TESTDSL\"\n\
             source = \"{cobol_rel_path}\"\n\
             type = \"main\"\n\
             \n\
             [copybooks]\n\
             sources = []\n\
             files = []\n"
        )
    }

    /// Set up a test workspace with COBOL source, transpiled Rust, and manifest.
    fn setup_dsl_test(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let base = std::env::temp_dir().join(format!("cobol2rust_dsl_e2e_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);

        // Workspace dir (transpiled Rust output)
        let ws_dir = base.join("workspace");
        std::fs::create_dir_all(ws_dir.join("src")).unwrap();
        std::fs::write(ws_dir.join("src/main.rs"), TEST_TRANSPILED_RUST).unwrap();
        std::fs::write(
            ws_dir.join("cobol2rust-manifest.toml"),
            test_manifest_toml("TESTDSL.CBL"),
        ).unwrap();

        // COBOL source dir
        let cobol_dir = base.join("cobol");
        std::fs::create_dir_all(&cobol_dir).unwrap();
        std::fs::write(cobol_dir.join("TESTDSL.CBL"), TEST_COBOL).unwrap();

        (ws_dir, cobol_dir)
    }

    fn cleanup_dsl_test(name: &str) {
        let base = std::env::temp_dir().join(format!("cobol2rust_dsl_e2e_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn e2e_legacy_emit_produces_dsl_files() {
        let (ws_dir, _cobol_dir) = setup_dsl_test("legacy");

        let config = RustifyConfig {
            source_dir: ws_dir.clone(),
            emit_dsl: true,
            emit_mode: config::EmitMode::Legacy,
            ..RustifyConfig::default()
        };

        let reports = emit_dsl_for_workspace(&config, &ws_dir).unwrap();
        assert_eq!(reports.len(), 1, "Should have 1 program report");

        let report = &reports[0];
        assert_eq!(report.program_name, "TESTDSL");
        assert!(report.total_files > 0, "Should produce DSL files");

        // Verify files exist on disk
        let dsl_dir = ws_dir.join("dsl");
        assert!(dsl_dir.exists(), "dsl/ directory should exist");
        assert!(dsl_dir.join("dsl_manifest.json").exists(), "manifest should exist");

        // Verify at least schema and process layers produced
        let layers: Vec<&str> = report.files.iter().map(|f| f.layer.as_str()).collect();
        assert!(layers.contains(&"schema"), "Should have schema files");
        assert!(layers.contains(&"process"), "Should have process files");

        cleanup_dsl_test("legacy");
    }

    #[cfg(feature = "direct-emit")]
    #[test]
    fn e2e_direct_emit_produces_dsl_files() {
        let (ws_dir, cobol_dir) = setup_dsl_test("direct");

        let config = RustifyConfig {
            source_dir: ws_dir.clone(),
            emit_dsl: true,
            emit_mode: config::EmitMode::Direct,
            cobol_source_dir: Some(cobol_dir.clone()),
            ..RustifyConfig::default()
        };

        let reports = emit_dsl_for_workspace(&config, &ws_dir).unwrap();
        assert_eq!(reports.len(), 1, "Should have 1 program report");

        let report = &reports[0];
        assert_eq!(report.program_name, "TESTDSL");
        assert!(report.total_files > 0, "Direct path should produce DSL files");

        // Verify at least schema layer (direct emitter reads DATA DIVISION)
        let layers: Vec<&str> = report.files.iter().map(|f| f.layer.as_str()).collect();
        assert!(layers.contains(&"schema"), "Direct should produce schema files");

        cleanup_dsl_test("direct");
    }

    #[cfg(feature = "direct-emit")]
    #[test]
    fn e2e_compare_mode_runs_both_paths() {
        let (ws_dir, cobol_dir) = setup_dsl_test("compare");

        let config = RustifyConfig {
            source_dir: ws_dir.clone(),
            emit_dsl: true,
            emit_mode: config::EmitMode::Compare,
            cobol_source_dir: Some(cobol_dir.clone()),
            ..RustifyConfig::default()
        };

        let reports = emit_dsl_for_workspace(&config, &ws_dir).unwrap();
        assert_eq!(reports.len(), 1, "Compare mode should produce 1 report");

        let report = &reports[0];
        assert!(report.total_files > 0, "Compare should produce DSL files (legacy output)");

        // Compare mode returns legacy output; the comparison report is printed via tracing
        // but the files written should be the legacy path's output
        let dsl_dir = ws_dir.join("dsl");
        assert!(dsl_dir.join("dsl_manifest.json").exists());

        cleanup_dsl_test("compare");
    }

    #[cfg(feature = "direct-emit")]
    #[test]
    fn e2e_mixed_overrides_use_direct_for_named_emitters() {
        let (ws_dir, cobol_dir) = setup_dsl_test("mixed");

        let config = RustifyConfig {
            source_dir: ws_dir.clone(),
            emit_dsl: true,
            emit_mode: config::EmitMode::Legacy,
            emitter_overrides: config::EmitterOverrides {
                direct: vec!["schema".to_string()],
            },
            cobol_source_dir: Some(cobol_dir.clone()),
            ..RustifyConfig::default()
        };

        let reports = emit_dsl_for_workspace(&config, &ws_dir).unwrap();
        assert_eq!(reports.len(), 1);
        assert!(reports[0].total_files > 0, "Mixed mode should produce DSL files");

        cleanup_dsl_test("mixed");
    }

    #[cfg(feature = "direct-emit")]
    #[test]
    fn e2e_direct_without_cobol_source_errors() {
        let (ws_dir, _cobol_dir) = setup_dsl_test("no_cobol");

        let config = RustifyConfig {
            source_dir: ws_dir.clone(),
            emit_dsl: true,
            emit_mode: config::EmitMode::Direct,
            cobol_source_dir: None, // intentionally missing
            ..RustifyConfig::default()
        };

        let result = emit_dsl_for_workspace(&config, &ws_dir);
        assert!(result.is_err(), "Should error without cobol_source_dir");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("--cobol-source"),
            "Error should mention --cobol-source flag, got: {err}"
        );

        cleanup_dsl_test("no_cobol");
    }

    #[cfg(feature = "direct-emit")]
    #[test]
    fn e2e_manifest_without_program_id_falls_back_to_filename() {
        let base = std::env::temp_dir().join(format!(
            "cobol2rust_dsl_e2e_fallback_{}", std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&base);

        let ws_dir = base.join("workspace");
        std::fs::create_dir_all(ws_dir.join("src")).unwrap();
        std::fs::write(ws_dir.join("src/main.rs"), TEST_TRANSPILED_RUST).unwrap();

        // Old-style manifest WITHOUT program_id field
        let old_manifest = "\
            # old manifest\n\
            [programs.testdsl]\n\
            source = \"TESTDSL.CBL\"\n\
            type = \"main\"\n\
            \n\
            [copybooks]\n\
            sources = []\n\
            files = []\n";
        std::fs::write(ws_dir.join("cobol2rust-manifest.toml"), old_manifest).unwrap();

        let cobol_dir = base.join("cobol");
        std::fs::create_dir_all(&cobol_dir).unwrap();
        std::fs::write(cobol_dir.join("TESTDSL.CBL"), TEST_COBOL).unwrap();

        let config = RustifyConfig {
            source_dir: ws_dir.clone(),
            emit_dsl: true,
            emit_mode: config::EmitMode::Direct,
            cobol_source_dir: Some(cobol_dir),
            ..RustifyConfig::default()
        };

        let reports = emit_dsl_for_workspace(&config, &ws_dir).unwrap();
        assert_eq!(reports.len(), 1, "Should work with filename fallback");
        assert_eq!(reports[0].program_name, "TESTDSL");

        let _ = std::fs::remove_dir_all(&base);
    }

    /// Second program's transpiled Rust (different program name).
    const TEST_TRANSPILED_RUST_B: &str = r#"
        #![allow(unused_imports, unused_variables, non_snake_case)]
        use cobol_runtime::prelude::*;

        #[cobol(program = "PROGB")]
        pub struct WorkingStorage {
            #[cobol(level = 1, pic = "X(20)")]
            pub ws_name: PicX,
            #[cobol(level = 1, pic = "9(5)")]
            pub ws_count: PackedDecimal,
        }

        #[cobol(section = "MAIN-SECTION")]
        fn main_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
    "#;

    #[test]
    fn e2e_multi_program_workspace_aggregated_manifest() {
        let base = std::env::temp_dir().join(format!(
            "cobol2rust_dsl_e2e_multi_{}", std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&base);

        let ws_dir = base.join("workspace");
        std::fs::create_dir_all(ws_dir.join("src")).unwrap();

        // Two valid programs
        std::fs::write(ws_dir.join("src/testdsl.rs"), TEST_TRANSPILED_RUST).unwrap();
        std::fs::write(ws_dir.join("src/progb.rs"), TEST_TRANSPILED_RUST_B).unwrap();

        let config = RustifyConfig {
            source_dir: ws_dir.clone(),
            emit_dsl: true,
            emit_mode: config::EmitMode::Legacy,
            ..RustifyConfig::default()
        };

        let reports = emit_dsl_for_workspace(&config, &ws_dir).unwrap();
        assert_eq!(reports.len(), 2, "Should have 2 program reports");

        // Verify aggregated manifest
        let manifest_path = ws_dir.join("dsl/dsl_manifest.json");
        assert!(manifest_path.exists(), "Workspace manifest should exist");

        let json = std::fs::read_to_string(&manifest_path).unwrap();
        let manifest: dsl::writer::WorkspaceManifest =
            serde_json::from_str(&json).unwrap();

        assert_eq!(manifest.total_programs, 2);
        assert!(manifest.total_files > 0);
        assert!(manifest.errors.is_empty());

        let names: Vec<&str> = manifest.programs.iter()
            .map(|p| p.program_name.as_str())
            .collect();
        assert!(names.contains(&"TESTDSL"), "Should contain TESTDSL");
        assert!(names.contains(&"PROGB"), "Should contain PROGB");

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn e2e_broken_file_skipped_others_succeed() {
        let base = std::env::temp_dir().join(format!(
            "cobol2rust_dsl_e2e_skip_{}", std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&base);

        let ws_dir = base.join("workspace");
        std::fs::create_dir_all(ws_dir.join("src")).unwrap();

        // One valid program
        std::fs::write(ws_dir.join("src/testdsl.rs"), TEST_TRANSPILED_RUST).unwrap();
        // One broken file (invalid Rust, but has .rs extension)
        std::fs::write(ws_dir.join("src/broken.rs"), "this is not valid rust {{{{").unwrap();
        // One more valid program
        std::fs::write(ws_dir.join("src/progb.rs"), TEST_TRANSPILED_RUST_B).unwrap();

        let config = RustifyConfig {
            source_dir: ws_dir.clone(),
            emit_dsl: true,
            emit_mode: config::EmitMode::Legacy,
            ..RustifyConfig::default()
        };

        // Should NOT error -- broken file is skipped
        let reports = emit_dsl_for_workspace(&config, &ws_dir).unwrap();
        assert_eq!(reports.len(), 2, "Should have 2 reports (broken file skipped)");

        // Manifest should still be written with both successful programs
        let manifest_path = ws_dir.join("dsl/dsl_manifest.json");
        let json = std::fs::read_to_string(&manifest_path).unwrap();
        let manifest: dsl::writer::WorkspaceManifest =
            serde_json::from_str(&json).unwrap();

        assert_eq!(manifest.total_programs, 2);
        assert!(manifest.errors.is_empty(), "Parse failures are silent skips, not errors");

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn e2e_hints_passthrough_enables_co_access_schemas() {
        let base = std::env::temp_dir().join(format!(
            "cobol2rust_dsl_e2e_hints_{}", std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&base);

        let ws_dir = base.join("workspace");
        std::fs::create_dir_all(ws_dir.join("src")).unwrap();
        std::fs::write(ws_dir.join("src/main.rs"), TEST_TRANSPILED_RUST).unwrap();

        // Create hints.json with co-access data:
        // 3 fields share the same paragraph scope -> should produce co-access schema
        let hints_json = r#"{
            "version": "1.0",
            "files": {
                "src/main.rs": {
                    "cobol_source": "TESTDSL.CBL",
                    "fields": {
                        "ws_acct_number": {
                            "pic": "X(10)",
                            "usage": "DISPLAY",
                            "level": 1,
                            "read_count": 5,
                            "write_count": 2,
                            "paragraph_scope": ["VALIDATE-PARA", "PROCESS-PARA"]
                        },
                        "ws_acct_balance": {
                            "pic": "S9(9)V99",
                            "usage": "COMP-3",
                            "level": 1,
                            "read_count": 3,
                            "write_count": 1,
                            "paragraph_scope": ["VALIDATE-PARA", "PROCESS-PARA"]
                        },
                        "ws_acct_type": {
                            "pic": "X(1)",
                            "usage": "DISPLAY",
                            "level": 1,
                            "read_count": 4,
                            "write_count": 0,
                            "paragraph_scope": ["VALIDATE-PARA", "PROCESS-PARA"]
                        }
                    },
                    "paragraphs": {},
                    "level_88_groups": {},
                    "call_targets": [],
                    "file_io_fields": []
                }
            }
        }"#;
        let hints_dir = ws_dir.join("rustify");
        std::fs::create_dir_all(&hints_dir).unwrap();
        std::fs::write(hints_dir.join("hints.json"), hints_json).unwrap();

        let config = RustifyConfig {
            source_dir: ws_dir.clone(),
            emit_dsl: true,
            emit_mode: config::EmitMode::Legacy,
            ..RustifyConfig::default()
        };

        let reports = emit_dsl_for_workspace(&config, &ws_dir).unwrap();
        assert_eq!(reports.len(), 1);

        // Should have at least one co_access schema file
        let co_access_files: Vec<_> = reports[0]
            .files
            .iter()
            .filter(|f| f.path.contains("co_access"))
            .collect();
        assert!(
            !co_access_files.is_empty(),
            "Should produce co-access schema from hints (found files: {:?})",
            reports[0].files.iter().map(|f| &f.path).collect::<Vec<_>>()
        );

        let _ = std::fs::remove_dir_all(&base);
    }
}
