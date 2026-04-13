//! Primary key identification for schema entities.
//!
//! Sources in priority order:
//! 1. RECORD KEY IS from SELECT clause (FileDescription.record_key)
//! 2. RELATIVE KEY IS (FileDescription.relative_key)
//! 3. ALTERNATE RECORD KEY (FileDescription.alternate_keys) -- unique indexes
//! 4. Naming heuristics (-ID, -KEY, -CODE, -NUMBER, -NUM, -NO)
//! 5. First field in the record (COBOL convention)
//! 6. Synthetic surrogate (fallback)

use cobol_transpiler::ast::{DataEntry, FileDescription};

use crate::dsl::dsl_ast::{FieldDecl, FieldType, Ident};

use super::cobol_extract::cobol_name_to_snake;
use super::schema_fields::entry_to_field_decl;

/// Result of primary key detection.
#[derive(Debug)]
pub struct DetectedKey {
    /// Identity fields for the schema.
    pub identity: Vec<FieldDecl>,
    /// How the key was detected (for review notes).
    pub source: KeySource,
    /// Alternate keys (unique indexes), if any.
    pub alternate_keys: Vec<String>,
}

/// How the primary key was identified.
#[derive(Debug, Clone, Copy)]
pub enum KeySource {
    /// RECORD KEY IS from SELECT clause -- definitive.
    RecordKey,
    /// RELATIVE KEY IS from SELECT clause.
    RelativeKey,
    /// Field name heuristic (-ID, -KEY, etc.).
    NameHeuristic,
    /// First field in record (COBOL convention).
    FirstField,
    /// No key found -- synthetic surrogate needed.
    Synthetic,
}

impl KeySource {
    pub fn description(&self) -> &'static str {
        match self {
            Self::RecordKey => "RECORD KEY IS (from SELECT clause)",
            Self::RelativeKey => "RELATIVE KEY IS (from SELECT clause)",
            Self::NameHeuristic => "field name heuristic",
            Self::FirstField => "first field in record (COBOL convention)",
            Self::Synthetic => "synthetic surrogate (no key detected)",
        }
    }
}

/// Detect primary key for a file-associated record.
pub fn detect_key_from_file(
    fd: &FileDescription,
    record_entries: &[DataEntry],
) -> DetectedKey {
    // Priority 1: RECORD KEY IS
    if let Some(ref key_name) = fd.record_key {
        if let Some(key_field) = find_field_by_cobol_name(record_entries, key_name) {
            return DetectedKey {
                identity: vec![key_field],
                source: KeySource::RecordKey,
                alternate_keys: fd.alternate_keys.clone(),
            };
        }
    }

    // Priority 2: RELATIVE KEY IS
    if let Some(ref key_name) = fd.relative_key {
        return DetectedKey {
            identity: vec![FieldDecl {
                name: Ident::new(&cobol_name_to_snake(key_name)),
                field_type: FieldType::Integer(None),
                required: true,
                comment: Some(format!("RELATIVE KEY: {key_name}")),
            }],
            source: KeySource::RelativeKey,
            alternate_keys: vec![],
        };
    }

    // Priority 3+: heuristic detection on record fields
    detect_key_from_entries(record_entries)
}

/// Detect primary key from entries only (no file context).
/// Used for WORKING-STORAGE groups and LINKAGE SECTION.
/// Skips entries with OCCURS or REDEFINES (those belong to child entities).
pub fn detect_key_from_entries(entries: &[DataEntry]) -> DetectedKey {
    // Priority 4: naming heuristic (skip OCCURS/REDEFINES -- they belong to children)
    for entry in entries {
        if entry.occurs.is_some() || entry.redefines.is_some() {
            continue;
        }
        if is_key_name(&entry.name) && entry.pic.is_some() {
            if let Some(key_field) = entry_to_field_decl(entry) {
                return DetectedKey {
                    identity: vec![key_field],
                    source: KeySource::NameHeuristic,
                    alternate_keys: vec![],
                };
            }
        }
        // Check immediate children too (for 01-level groups), skip OCCURS/REDEFINES
        for child in &entry.children {
            if child.occurs.is_some() || child.redefines.is_some() {
                continue;
            }
            if is_key_name(&child.name) && child.pic.is_some() {
                if let Some(key_field) = entry_to_field_decl(child) {
                    return DetectedKey {
                        identity: vec![key_field],
                        source: KeySource::NameHeuristic,
                        alternate_keys: vec![],
                    };
                }
            }
        }
    }

    // Priority 5: first elementary field in first entry
    if let Some(first) = find_first_elementary(entries) {
        if let Some(key_field) = entry_to_field_decl(first) {
            return DetectedKey {
                identity: vec![key_field],
                source: KeySource::FirstField,
                alternate_keys: vec![],
            };
        }
    }

    // Priority 6: synthetic
    DetectedKey {
        identity: vec![FieldDecl {
            name: Ident::new("_synthetic_id"),
            field_type: FieldType::Integer(None),
            required: true,
            comment: Some("Synthetic surrogate -- no COBOL key detected".to_string()),
        }],
        source: KeySource::Synthetic,
        alternate_keys: vec![],
    }
}

/// Find a field by COBOL name in the entry tree.
fn find_field_by_cobol_name(entries: &[DataEntry], cobol_name: &str) -> Option<FieldDecl> {
    let upper = cobol_name.to_uppercase();
    for entry in entries {
        if entry.name.to_uppercase() == upper {
            return entry_to_field_decl(entry);
        }
        if let Some(found) = find_field_by_cobol_name(&entry.children, cobol_name) {
            return Some(found);
        }
    }
    None
}

/// Find the first elementary (PIC-bearing) field in the tree.
/// Skips OCCURS/REDEFINES entries (they belong to child entities).
fn find_first_elementary(entries: &[DataEntry]) -> Option<&DataEntry> {
    for entry in entries {
        if entry.occurs.is_some() || entry.redefines.is_some() {
            continue;
        }
        if entry.pic.is_some() && !super::schema_fields::is_filler(&entry.name) {
            return Some(entry);
        }
        if let Some(found) = find_first_elementary(&entry.children) {
            return Some(found);
        }
    }
    None
}

/// Check if a field name suggests it's a key/identifier.
fn is_key_name(name: &str) -> bool {
    let upper = name.to_uppercase();
    upper.ends_with("-ID")
        || upper.ends_with("-KEY")
        || upper.ends_with("-CODE")
        || upper.ends_with("-NUMBER")
        || upper.ends_with("-NUM")
        || upper.ends_with("-NO")
        || upper.ends_with("-NBR")
        || upper.contains("-RECORD-KEY")
        || upper.contains("-PRIMARY")
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::*;

    fn make_entry(name: &str, pic_raw: &str) -> DataEntry {
        DataEntry {
            level: 5,
            name: name.to_string(),
            pic: Some(PicClause {
                raw: pic_raw.to_string(),
                category: PicCategory::Alphanumeric,
                total_digits: 10, scale: 0, signed: false, display_length: 10,
                edit_symbols: vec![],
            }),
            usage: Usage::Display, value: None, redefines: None,
            occurs: None, occurs_depending: None, sign: None,
            justified_right: false, blank_when_zero: false,
            children: vec![], condition_values: vec![],
            byte_offset: None, byte_length: None,
            renames_target: None, renames_thru: None, index_names: vec![],
        }
    }

    fn make_fd_with_key(key: &str) -> FileDescription {
        FileDescription {
            descriptor_type: FileDescriptorType::Fd,
            file_name: "TEST-FILE".to_string(),
            assign_to: None,
            organization: FileOrganization::Indexed,
            access_mode: AccessMode::Dynamic,
            record_key: Some(key.to_string()),
            alternate_keys: vec![],
            relative_key: None,
            file_status: None,
            records: vec![],
            record_names: vec![],
            recording_mode: None,
            record_size: None,
        }
    }

    #[test]
    fn record_key_is_detected() {
        let fd = make_fd_with_key("ACCT-NUMBER");
        let entries = vec![
            make_entry("ACCT-NUMBER", "X(16)"),
            make_entry("ACCT-NAME", "X(30)"),
        ];
        let key = detect_key_from_file(&fd, &entries);
        assert!(matches!(key.source, KeySource::RecordKey));
        assert_eq!(key.identity[0].name.as_str(), "acct_number");
    }

    #[test]
    fn name_heuristic_finds_id_field() {
        let entries = vec![
            make_entry("WS-CUST-ID", "X(10)"),
            make_entry("WS-CUST-NAME", "X(30)"),
        ];
        let key = detect_key_from_entries(&entries);
        assert!(matches!(key.source, KeySource::NameHeuristic));
        assert_eq!(key.identity[0].name.as_str(), "ws_cust_id");
    }

    #[test]
    fn first_field_fallback() {
        let entries = vec![
            make_entry("WS-FIELD-A", "X(10)"),
            make_entry("WS-FIELD-B", "X(20)"),
        ];
        let key = detect_key_from_entries(&entries);
        assert!(matches!(key.source, KeySource::FirstField));
        assert_eq!(key.identity[0].name.as_str(), "ws_field_a");
    }

    #[test]
    fn empty_entries_get_synthetic() {
        let key = detect_key_from_entries(&[]);
        assert!(matches!(key.source, KeySource::Synthetic));
    }
}
