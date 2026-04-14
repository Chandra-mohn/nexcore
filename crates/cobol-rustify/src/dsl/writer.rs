//! DSL file writer: materializes emitter output to the filesystem.
//!
//! Takes `Vec<DslFile>` from all 4 emitters and writes them under
//! `<output>/dsl/`, creating the directory structure and
//! a manifest summarizing what was generated.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::DslFile;
use crate::config::EmitMode;
use crate::error::RustifyError;

/// Summary of DSL generation for a single source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslWriteReport {
    /// Source file that was analyzed.
    pub source_file: String,
    /// COBOL program name.
    pub program_name: String,
    /// Files written, grouped by layer.
    pub files: Vec<DslFileEntry>,
    /// Total files written.
    pub total_files: usize,
    /// Average confidence across all files.
    pub avg_confidence: f64,
    /// All review notes across all files.
    pub review_notes: Vec<String>,
}

/// A single generated DSL file entry for the manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslFileEntry {
    /// Relative path under the dsl/ directory.
    pub path: String,
    /// DSL layer (schema, transform, rules, process).
    pub layer: String,
    /// Confidence score.
    pub confidence: f64,
    /// Number of review notes.
    pub note_count: usize,
    /// Source fields/functions that contributed.
    pub source_items: Vec<String>,
}

/// Write all DSL files to disk under `<output_dir>/dsl/`.
///
/// Creates the directory structure and writes a `dsl_manifest.json`
/// summarizing what was generated.
pub fn write_dsl_files(
    output_dir: &Path,
    program_name: &str,
    source_file: &str,
    all_files: &[DslFile],
) -> Result<DslWriteReport, RustifyError> {
    let dsl_dir = output_dir.join("dsl");

    let mut entries = Vec::new();
    let mut all_notes = Vec::new();

    for dsl_file in all_files {
        // Create parent directories
        let file_path = dsl_dir.join(&dsl_file.path);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write the DSL content
        std::fs::write(&file_path, &dsl_file.content)?;

        // Determine layer from path
        let layer = dsl_file
            .path
            .split('/')
            .next()
            .unwrap_or("unknown")
            .to_string();

        // Collect notes
        for note in &dsl_file.notes {
            all_notes.push(format!("[{}] {}", dsl_file.path, note));
        }

        entries.push(DslFileEntry {
            path: dsl_file.path.clone(),
            layer,
            confidence: dsl_file.confidence,
            note_count: dsl_file.notes.len(),
            source_items: dsl_file.source_fields.clone(),
        });
    }

    let total = entries.len();
    let avg_confidence = if total > 0 {
        entries.iter().map(|e| e.confidence).sum::<f64>() / total as f64
    } else {
        0.0
    };

    let report = DslWriteReport {
        source_file: source_file.to_string(),
        program_name: program_name.to_string(),
        files: entries,
        total_files: total,
        avg_confidence,
        review_notes: all_notes,
    };

    // Write manifest
    let manifest_path = dsl_dir.join("dsl_manifest.json");
    let json = serde_json::to_string_pretty(&report)
        .map_err(|e| RustifyError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    std::fs::write(&manifest_path, json)?;

    Ok(report)
}

/// Run all 4 emitters on a parsed Rust file and collect results.
pub fn emit_all_dsl(ctx: &super::EmitterContext<'_>) -> Vec<DslFile> {
    use super::DslEmitter;
    use super::process_emitter::ProcessEmitter;
    use super::rules_emitter::RulesEmitter;
    use super::schema_emitter::SchemaEmitter;
    use super::transform_emitter::TransformEmitter;

    let emitters: Vec<Box<dyn DslEmitter>> = vec![
        Box::new(SchemaEmitter),
        Box::new(TransformEmitter),
        Box::new(RulesEmitter),
        Box::new(ProcessEmitter),
    ];

    let mut all_files = Vec::new();
    for emitter in &emitters {
        all_files.extend(emitter.emit(ctx));
    }
    all_files
}

/// Mismatch detail for a single DSL file in comparison mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMismatch {
    /// Relative DSL file path.
    pub path: String,
    /// Number of lines in legacy output.
    pub legacy_lines: usize,
    /// Number of lines in direct output.
    pub direct_lines: usize,
    /// First line number where the outputs differ (1-based).
    pub first_diff_line: usize,
    /// Snippet from legacy output at the diff point.
    pub legacy_snippet: String,
    /// Snippet from direct output at the diff point.
    pub direct_snippet: String,
}

/// Report from running both emission paths and comparing output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonReport {
    /// DSL files that matched exactly.
    pub matches: Vec<String>,
    /// DSL files with content differences.
    pub mismatches: Vec<FileMismatch>,
    /// Files produced only by the legacy path.
    pub legacy_only: Vec<String>,
    /// Files produced only by the direct path.
    pub direct_only: Vec<String>,
}

impl ComparisonReport {
    /// True if both paths produced identical output.
    pub fn is_match(&self) -> bool {
        self.mismatches.is_empty() && self.legacy_only.is_empty() && self.direct_only.is_empty()
    }

    /// Print a summary to stderr.
    pub fn print_summary(&self) {
        let total = self.matches.len() + self.mismatches.len()
            + self.legacy_only.len() + self.direct_only.len();
        tracing::info!(
            total = total,
            matches = self.matches.len(),
            mismatches = self.mismatches.len(),
            legacy_only = self.legacy_only.len(),
            direct_only = self.direct_only.len(),
            "DSL comparison complete",
        );
        for m in &self.mismatches {
            tracing::warn!(
                path = %m.path,
                first_diff_line = m.first_diff_line,
                legacy_lines = m.legacy_lines,
                direct_lines = m.direct_lines,
                "DSL mismatch",
            );
        }
    }
}

/// Compare two sets of DSL files, producing a detailed comparison report.
pub fn compare_outputs(legacy: &[DslFile], direct: &[DslFile]) -> ComparisonReport {
    use std::collections::BTreeMap;

    let legacy_map: BTreeMap<&str, &DslFile> = legacy.iter().map(|f| (f.path.as_str(), f)).collect();
    let direct_map: BTreeMap<&str, &DslFile> = direct.iter().map(|f| (f.path.as_str(), f)).collect();

    let mut matches = Vec::new();
    let mut mismatches = Vec::new();
    let mut legacy_only = Vec::new();
    let mut direct_only = Vec::new();

    // Check all legacy files against direct
    for (path, lf) in &legacy_map {
        if let Some(df) = direct_map.get(path) {
            if lf.content == df.content {
                matches.push(path.to_string());
            } else {
                let mismatch = diff_files(path, &lf.content, &df.content);
                mismatches.push(mismatch);
            }
        } else {
            legacy_only.push(path.to_string());
        }
    }

    // Check for direct-only files
    for path in direct_map.keys() {
        if !legacy_map.contains_key(path) {
            direct_only.push(path.to_string());
        }
    }

    ComparisonReport {
        matches,
        mismatches,
        legacy_only,
        direct_only,
    }
}

/// Diff two file contents and produce a `FileMismatch`.
fn diff_files(path: &str, legacy: &str, direct: &str) -> FileMismatch {
    let legacy_lines: Vec<&str> = legacy.lines().collect();
    let direct_lines: Vec<&str> = direct.lines().collect();

    let mut first_diff = 0;
    for (i, (l, d)) in legacy_lines.iter().zip(direct_lines.iter()).enumerate() {
        if l != d {
            first_diff = i + 1;
            break;
        }
    }
    if first_diff == 0 {
        // Lines matched up to the shorter file; difference is length
        first_diff = legacy_lines.len().min(direct_lines.len()) + 1;
    }

    let snippet = |lines: &[&str], line: usize| -> String {
        if line > 0 && line <= lines.len() {
            lines[line - 1].to_string()
        } else {
            "<EOF>".to_string()
        }
    };

    FileMismatch {
        path: path.to_string(),
        legacy_lines: legacy_lines.len(),
        direct_lines: direct_lines.len(),
        first_diff_line: first_diff,
        legacy_snippet: snippet(&legacy_lines, first_diff),
        direct_snippet: snippet(&direct_lines, first_diff),
    }
}

/// Route DSL emission based on emit mode.
///
/// When `direct-emit` feature is not enabled, only legacy mode is available;
/// requesting direct or compare mode returns an error.
#[cfg(not(feature = "direct-emit"))]
pub fn emit_dsl_routed(
    legacy_ctx: &super::EmitterContext<'_>,
    mode: EmitMode,
) -> Result<(Vec<DslFile>, Option<ComparisonReport>), crate::error::RustifyError> {
    match mode {
        EmitMode::Legacy => Ok((emit_all_dsl(legacy_ctx), None)),
        EmitMode::Direct | EmitMode::Compare => {
            Err(crate::error::RustifyError::Io(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "direct-emit feature not enabled; rebuild with --features direct-emit",
            )))
        }
    }
}

/// Route DSL emission based on emit mode (feature-gated version with direct path).
#[cfg(feature = "direct-emit")]
pub fn emit_dsl_routed(
    legacy_ctx: &super::EmitterContext<'_>,
    direct_ctx: Option<&super::direct::DirectEmitterContext<'_>>,
    mode: EmitMode,
    overrides: &crate::config::EmitterOverrides,
) -> Result<(Vec<DslFile>, Option<ComparisonReport>), crate::error::RustifyError> {
    match mode {
        EmitMode::Legacy if overrides.is_empty() => {
            Ok((emit_all_dsl(legacy_ctx), None))
        }
        EmitMode::Direct => {
            let ctx = direct_ctx.ok_or_else(|| {
                crate::error::RustifyError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "direct mode requires CobolProgram (pass COBOL source path)",
                ))
            })?;
            Ok((super::direct::emit_all_dsl_direct(ctx), None))
        }
        EmitMode::Compare => {
            let dctx = direct_ctx.ok_or_else(|| {
                crate::error::RustifyError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "compare mode requires CobolProgram (pass COBOL source path)",
                ))
            })?;
            let legacy = emit_all_dsl(legacy_ctx);
            let direct = super::direct::emit_all_dsl_direct(dctx);
            let report = compare_outputs(&legacy, &direct);
            report.print_summary();
            Ok((legacy, Some(report)))
        }
        // Legacy mode with per-emitter overrides: mixed mode
        EmitMode::Legacy => {
            let dctx = direct_ctx.ok_or_else(|| {
                crate::error::RustifyError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "per-emitter direct overrides require CobolProgram",
                ))
            })?;

            let mut all_files = Vec::new();

            // Schema
            if overrides.is_direct("schema") {
                let emitter = super::direct::schema_emitter::DirectSchemaEmitter;
                all_files.extend(super::direct::DirectDslEmitter::emit(&emitter, dctx));
            } else {
                let emitter = super::schema_emitter::SchemaEmitter;
                all_files.extend(super::DslEmitter::emit(&emitter, legacy_ctx));
            }

            // Transform
            if overrides.is_direct("transform") {
                let emitter = super::direct::transform_emitter::DirectTransformEmitter;
                all_files.extend(super::direct::DirectDslEmitter::emit(&emitter, dctx));
            } else {
                let emitter = super::transform_emitter::TransformEmitter;
                all_files.extend(super::DslEmitter::emit(&emitter, legacy_ctx));
            }

            // Rules
            if overrides.is_direct("rules") {
                let emitter = super::direct::rules_emitter::DirectRulesEmitter;
                all_files.extend(super::direct::DirectDslEmitter::emit(&emitter, dctx));
            } else {
                let emitter = super::rules_emitter::RulesEmitter;
                all_files.extend(super::DslEmitter::emit(&emitter, legacy_ctx));
            }

            // Process
            if overrides.is_direct("process") {
                let emitter = super::direct::process_emitter::DirectProcessEmitter;
                all_files.extend(super::direct::DirectDslEmitter::emit(&emitter, dctx));
            } else {
                let emitter = super::process_emitter::ProcessEmitter;
                all_files.extend(super::DslEmitter::emit(&emitter, legacy_ctx));
            }

            Ok((all_files, None))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::EmitterContext;
    use std::path::PathBuf;

    const TEST_RUST: &str = r#"
        #[cobol(program = "TESTPROG")]
        pub struct WorkingStorage {
            #[cobol(level = 5, pic = "X(10)")]
            pub ws_acct_number: PicX,
            #[cobol(level = 5, pic = "X(1)", level88 = "YES:Y,NO:N")]
            pub ws_acct_flag: PicX,
            #[cobol(level = 5, pic = "9(5)")]
            pub ws_acct_count: PackedDecimal,
        }

        #[cobol(performs = "PROC-PARA")]
        fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

        #[cobol(section = "MAIN-SECTION", reads = "WS-ACCT-FLAG", writes = "WS-ACCT-COUNT")]
        fn proc_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            if ws.ws_acct_flag.to_string() == "Y" {
                ws.ws_acct_count.pack(dec!(1));
            } else {
                ws.ws_acct_count.pack(dec!(0));
            }
        }
    "#;

    #[test]
    fn emit_all_produces_files() {
        let syn_file = syn::parse_str::<syn::File>(TEST_RUST).unwrap();
        let ctx = EmitterContext {
            program_name: "TESTPROG".to_string(),
            syn_file: &syn_file,
            source_text: TEST_RUST,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emit_all_dsl(&ctx);
        assert!(!files.is_empty(), "Should produce DSL files");

        // Should have at least schema + transform + process
        let layers: Vec<&str> = files
            .iter()
            .map(|f| f.path.split('/').next().unwrap_or(""))
            .collect();
        assert!(layers.contains(&"schema"), "Should have schema files");
        assert!(layers.contains(&"transform"), "Should have transform files");
        assert!(layers.contains(&"process"), "Should have process files");
    }

    fn make_temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("cobol2rust_dsl_test_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn write_creates_directory_structure() {
        let syn_file = syn::parse_str::<syn::File>(TEST_RUST).unwrap();
        let ctx = EmitterContext {
            program_name: "TESTPROG".to_string(),
            syn_file: &syn_file,
            source_text: TEST_RUST,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emit_all_dsl(&ctx);
        let tmp = make_temp_dir("write");

        let report = write_dsl_files(&tmp, "TESTPROG", "test.rs", &files).unwrap();

        // Verify files exist on disk
        let dsl_dir = tmp.join("dsl");
        assert!(dsl_dir.exists(), "DSL directory should exist");
        assert!(dsl_dir.join("schema").exists(), "schema/ should exist");
        assert!(dsl_dir.join("transform").exists(), "transform/ should exist");
        assert!(dsl_dir.join("process").exists(), "process/ should exist");

        // Verify manifest exists
        assert!(
            dsl_dir.join("dsl_manifest.json").exists(),
            "dsl_manifest.json should exist"
        );

        // Verify report
        assert!(report.total_files > 0, "Should report files written");
        assert!(report.avg_confidence > 0.0, "Should have positive confidence");

        // Verify actual DSL content is on disk
        for entry in &report.files {
            let path = dsl_dir.join(&entry.path);
            assert!(path.exists(), "File should exist: {}", entry.path);
            let content = std::fs::read_to_string(&path).unwrap();
            assert!(!content.is_empty(), "File should not be empty: {}", entry.path);
            assert!(content.contains("end"), "File should contain 'end': {}", entry.path);
        }

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn manifest_is_valid_json() {
        let syn_file = syn::parse_str::<syn::File>(TEST_RUST).unwrap();
        let ctx = EmitterContext {
            program_name: "TESTPROG".to_string(),
            syn_file: &syn_file,
            source_text: TEST_RUST,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emit_all_dsl(&ctx);
        let tmp = make_temp_dir("manifest");

        write_dsl_files(&tmp, "TESTPROG", "test.rs", &files).unwrap();

        let manifest_path = tmp.join("dsl/dsl_manifest.json");
        let json = std::fs::read_to_string(&manifest_path).unwrap();
        let parsed: DslWriteReport = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.program_name, "TESTPROG");
        assert_eq!(parsed.source_file, "test.rs");
        assert!(parsed.total_files > 0);

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
