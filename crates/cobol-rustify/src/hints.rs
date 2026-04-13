//! COBOL semantic hints emitted by the transpiler.
//!
//! The hints file (`rustify/hints.json`) contains COBOL-level metadata
//! that cannot be derived from the generated Rust source alone. It enables
//! safety analysis for Tier 2+ rules (e.g., REDEFINES detection).

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::RustifyError;

/// Top-level hints file structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintsFile {
    /// Schema version.
    pub version: String,
    /// Per-file hints keyed by relative path (e.g., "src/program_a.rs").
    pub files: HashMap<String, FileHints>,
}

/// Hints for a single generated Rust file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHints {
    /// Original COBOL source path.
    #[serde(default)]
    pub cobol_source: String,
    /// Field-level metadata keyed by Rust field name.
    #[serde(default)]
    pub fields: HashMap<String, FieldHint>,
    /// Paragraph-level metadata keyed by paragraph name.
    #[serde(default)]
    pub paragraphs: HashMap<String, ParagraphHint>,
    /// Level-88 condition groups keyed by parent field name.
    #[serde(default)]
    pub level_88_groups: HashMap<String, Level88Group>,
    /// CALL targets referenced by this program.
    #[serde(default)]
    pub call_targets: Vec<String>,
    /// Fields used in file I/O (FD record fields).
    #[serde(default)]
    pub file_io_fields: Vec<String>,
}

/// Metadata for a single WorkingStorage field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldHint {
    /// COBOL PIC clause (e.g., "X(10)", "S9(7)V99").
    #[serde(default)]
    pub pic: String,
    /// COBOL USAGE (e.g., "DISPLAY", "COMP-3").
    #[serde(default)]
    pub usage: String,
    /// COBOL level number.
    #[serde(default)]
    pub level: u32,
    /// Field this one REDEFINES (if any).
    #[serde(default)]
    pub redefines: Option<String>,
    /// Fields that REDEFINE this one.
    #[serde(default)]
    pub redefined_by: Vec<String>,
    /// Passed by reference in a CALL statement.
    #[serde(default)]
    pub call_by_ref: bool,
    /// Target of MOVE CORRESPONDING.
    #[serde(default)]
    pub move_corr_target: bool,
    /// Number of read accesses in PROCEDURE DIVISION.
    #[serde(default)]
    pub read_count: u32,
    /// Number of write accesses in PROCEDURE DIVISION.
    #[serde(default)]
    pub write_count: u32,
    /// Paragraphs where this field is accessed.
    #[serde(default)]
    pub paragraph_scope: Vec<String>,
}

/// Metadata for a paragraph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphHint {
    /// Paragraphs this one PERFORMs.
    #[serde(default)]
    pub performs: Vec<String>,
    /// Paragraphs that PERFORM this one.
    #[serde(default)]
    pub performed_by: Vec<String>,
    /// Fields only used within this paragraph (localization candidates).
    #[serde(default)]
    pub local_only_fields: Vec<String>,
}

/// Level-88 condition group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level88Group {
    /// Condition name -> value mapping.
    #[serde(default)]
    pub conditions: HashMap<String, String>,
    /// Whether the conditions cover all possible values.
    #[serde(default)]
    pub exhaustive: bool,
}

/// Read hints from a directory's `rustify/hints.json`.
pub fn read_hints(dir: &Path) -> Result<Option<HintsFile>, RustifyError> {
    let path = dir.join("rustify").join("hints.json");
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(&path)?;
    let hints: HintsFile = serde_json::from_str(&content)?;
    Ok(Some(hints))
}

/// Write hints to a directory's `rustify/hints.json`.
pub fn write_hints(dir: &Path, hints: &HintsFile) -> Result<(), RustifyError> {
    let rustify_dir = dir.join("rustify");
    std::fs::create_dir_all(&rustify_dir)?;
    let content = serde_json::to_string_pretty(hints)?;
    std::fs::write(rustify_dir.join("hints.json"), content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_hints() {
        let mut fields = HashMap::new();
        fields.insert(
            "ws_name".to_string(),
            FieldHint {
                pic: "X(30)".to_string(),
                usage: "DISPLAY".to_string(),
                level: 5,
                redefines: None,
                redefined_by: vec![],
                call_by_ref: false,
                move_corr_target: false,
                read_count: 7,
                write_count: 2,
                paragraph_scope: vec!["100-VALIDATE".to_string()],
            },
        );

        let mut file_hints = HashMap::new();
        file_hints.insert(
            "src/program_a.rs".to_string(),
            FileHints {
                cobol_source: "cobol/PROGRAM-A.CBL".to_string(),
                fields,
                paragraphs: HashMap::new(),
                level_88_groups: HashMap::new(),
                call_targets: vec![],
                file_io_fields: vec![],
            },
        );

        let hints = HintsFile {
            version: "1.0".to_string(),
            files: file_hints,
        };

        let json = serde_json::to_string_pretty(&hints).unwrap();
        let parsed: HintsFile = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.version, "1.0");
        assert!(parsed.files.contains_key("src/program_a.rs"));
        let fh = &parsed.files["src/program_a.rs"];
        assert_eq!(fh.fields["ws_name"].read_count, 7);
    }

    #[test]
    fn read_missing_hints_returns_none() {
        let dir = std::env::temp_dir().join("cobol2rust_test_no_hints");
        let _ = std::fs::create_dir_all(&dir);
        let result = read_hints(&dir).unwrap();
        assert!(result.is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
