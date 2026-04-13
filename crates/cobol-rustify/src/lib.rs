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

use config::RustifyConfig;
use error::RustifyError;
use manifest::{compute_checksum, FileManifest, Manifest, ManifestSummary};
use report::AnalysisReport;
use rules::{AnalysisContext, RuleRegistry};

/// Analyze a workspace and return proposed transforms (report mode).
///
/// Reads Rust source files from `source_dir`, runs rules for the configured
/// tier, and returns an `AnalysisReport` with all proposed transforms.
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
                eprintln!(
                    "warning: skipping {} (parse error: {})",
                    file_path.display(),
                    e
                );
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
                eprintln!(
                    "warning: skipping {} (parse error: {})",
                    file_path.display(),
                    e
                );
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
        emit_dsl_for_workspace(&output_dir)?;
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
/// Walks `<output>/src/` (or flat .rs layout), parses each .rs file with syn,
/// runs all 4 DSL emitters, and writes output under `<output>/dsl/`.
pub fn emit_dsl_for_workspace(output_dir: &Path) -> Result<Vec<dsl::writer::DslWriteReport>, RustifyError> {
    let src_dir = resolve_rs_dir(output_dir);
    let rs_files = collect_rs_files(&src_dir, None, None);

    let mut reports = Vec::new();

    for rs_path in &rs_files {
        let source_text = std::fs::read_to_string(rs_path)?;
        let syn_file = match syn::parse_str::<syn::File>(&source_text) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("[WARN] Cannot parse Rust file {}: {e}", rs_path.display());
                continue;
            }
        };

        let program_name = dsl::cobol_attrs::extract_program_name(&syn_file)
            .unwrap_or_default();
        if program_name.is_empty() {
            continue; // Not a COBOL-generated file
        }

        let ctx = dsl::EmitterContext {
            program_name: program_name.clone(),
            syn_file: &syn_file,
            source_text: &source_text,
            hints: None,
            assessments: &[],
            target: None,
            source_path: rs_path.clone(),
        };

        let dsl_files = dsl::writer::emit_all_dsl(&ctx);
        if dsl_files.is_empty() {
            continue;
        }

        let rel_path = rs_path
            .strip_prefix(output_dir)
            .unwrap_or(rs_path)
            .to_string_lossy()
            .to_string();

        let report = dsl::writer::write_dsl_files(
            output_dir,
            &program_name,
            &rel_path,
            &dsl_files,
        )?;
        reports.push(report);
    }

    Ok(reports)
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
}
