//! COBOL metadata attribute generation.
//!
//! Builds `#[cobol(...)]` attribute strings from AST metadata for embedding
//! in generated Rust code. These attributes are inert at compile time but
//! machine-readable by `syn` in Phase 2 (Tier 5 DSL emitters).

use crate::ast::{ConditionValue, DataEntry, Literal, Usage};
use crate::codegen::field_analysis::ParagraphFieldAccess;

/// Build a `#[cobol(...)]` attribute for the WorkingStorage struct.
pub fn build_struct_attribute(program_id: &str) -> String {
    format!("#[cobol(program = \"{program_id}\")]")
}

/// Build a `#[cobol(...)]` attribute for a data field.
///
/// Returns `None` if the entry has no meaningful metadata (e.g., FILLER
/// with no PIC and no special properties).
pub fn build_field_attribute(entry: &DataEntry) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();

    // Level number
    parts.push(format!("level = {}", entry.level));

    // PIC clause (raw string)
    if let Some(pic) = &entry.pic {
        parts.push(format!("pic = \"{}\"", escape_attr_string(&pic.raw)));
    }

    // Usage/storage format
    match entry.usage {
        Usage::Comp3 => parts.push("comp3".to_string()),
        Usage::Comp => parts.push("comp".to_string()),
        Usage::Comp5 => parts.push("comp5".to_string()),
        Usage::Comp1 => parts.push("comp1".to_string()),
        Usage::Comp2 => parts.push("comp2".to_string()),
        Usage::Index => parts.push("index".to_string()),
        Usage::Pointer => parts.push("pointer".to_string()),
        Usage::Display => {} // Default, don't emit
    }

    // Byte offset
    if let Some(offset) = entry.byte_offset {
        parts.push(format!("offset = {offset}"));
    }

    // Byte length
    if let Some(len) = entry.byte_length {
        parts.push(format!("len = {len}"));
    }

    // Signed
    if entry.pic.as_ref().is_some_and(|p| p.signed) {
        parts.push("signed".to_string());
    }

    // REDEFINES
    if let Some(target) = &entry.redefines {
        parts.push(format!("redefines = \"{target}\""));
    }

    // OCCURS
    if let Some(count) = entry.occurs {
        parts.push(format!("occurs = {count}"));
    }

    // OCCURS DEPENDING ON
    if let Some(dep) = &entry.occurs_depending {
        parts.push(format!("occurs_depending = \"{dep}\""));
    }

    // Level-88 conditions
    let level88 = format_level88(&entry.condition_values, &entry.children);
    if !level88.is_empty() {
        parts.push(format!("level88 = \"{}\"", escape_attr_string(&level88)));
    }

    // BLANK WHEN ZERO
    if entry.blank_when_zero {
        parts.push("blank_when_zero".to_string());
    }

    // JUSTIFIED RIGHT
    if entry.justified_right {
        parts.push("justified_right".to_string());
    }

    if parts.len() <= 1 {
        // Only level number, not very useful
        return None;
    }

    Some(format!("#[cobol({})]", parts.join(", ")))
}

/// Build a `#[cobol(...)]` attribute for a paragraph function.
pub fn build_paragraph_attribute(
    section_name: Option<&str>,
    access: &ParagraphFieldAccess,
) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();

    // Section
    if let Some(section) = section_name {
        parts.push(format!("section = \"{section}\""));
    }

    // PERFORM targets
    if !access.performs.is_empty() {
        let performs_str = access.performs.join(",");
        parts.push(format!("performs = \"{performs_str}\""));
    }

    // Reads
    if !access.reads.is_empty() {
        let reads: Vec<&str> = access.reads.iter().map(String::as_str).collect();
        parts.push(format!("reads = \"{}\"", reads.join(",")));
    }

    // Writes
    if !access.writes.is_empty() {
        let writes: Vec<&str> = access.writes.iter().map(String::as_str).collect();
        parts.push(format!("writes = \"{}\"", writes.join(",")));
    }

    if parts.is_empty() {
        return None;
    }

    Some(format!("#[cobol({})]", parts.join(", ")))
}

/// Format level-88 condition values as "NAME1:VAL1,NAME2:VAL2".
fn format_level88(condition_values: &[ConditionValue], children: &[DataEntry]) -> String {
    // Level-88 conditions are stored as children with level == 88
    let mut pairs = Vec::new();
    for child in children {
        if child.level != 88 {
            continue;
        }
        for cv in &child.condition_values {
            let val = match cv {
                ConditionValue::Single(lit) => literal_to_string(lit),
                ConditionValue::Range { low, high } => {
                    format!("{}-{}", literal_to_string(low), literal_to_string(high))
                }
            };
            pairs.push(format!("{}:{}", child.name, val));
        }
    }

    // Also handle condition_values on the entry itself (less common)
    for cv in condition_values {
        let val = match cv {
            ConditionValue::Single(lit) => literal_to_string(lit),
            ConditionValue::Range { low, high } => {
                format!("{}-{}", literal_to_string(low), literal_to_string(high))
            }
        };
        pairs.push(val);
    }

    pairs.join(",")
}

/// Convert a literal to a display string for attribute values.
fn literal_to_string(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(s) | Literal::Alphanumeric(s) => s.clone(),
        Literal::Figurative(f) => format!("{f:?}"),
    }
}

/// Escape special characters in attribute string values.
fn escape_attr_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Build a `#[cobol(...)]` attribute for a section wrapper function.
pub fn build_section_attribute(section_name: &str) -> String {
    format!("#[cobol(section = \"{section_name}\")]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    fn make_entry(level: u8, name: &str) -> DataEntry {
        DataEntry {
            level,
            name: name.to_string(),
            pic: None,
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: vec![],
            condition_values: vec![],
            byte_offset: None,
            byte_length: None,
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }
    }

    #[test]
    fn struct_attribute() {
        let attr = build_struct_attribute("CARDPROC");
        assert_eq!(attr, "#[cobol(program = \"CARDPROC\")]");
    }

    #[test]
    fn field_with_pic_comp3() {
        let mut entry = make_entry(5, "CREDIT-LIMIT");
        entry.pic = Some(PicClause {
            raw: "S9(9)V99".to_string(),
            category: PicCategory::Numeric,
            total_digits: 11,
            scale: 2,
            signed: true,
            display_length: 11,
            edit_symbols: vec![],
        });
        entry.usage = Usage::Comp3;
        entry.byte_offset = Some(16);
        entry.byte_length = Some(6);

        let attr = build_field_attribute(&entry).unwrap();
        assert!(attr.contains("level = 5"));
        assert!(attr.contains("pic = \"S9(9)V99\""));
        assert!(attr.contains("comp3"));
        assert!(attr.contains("offset = 16"));
        assert!(attr.contains("len = 6"));
        assert!(attr.contains("signed"));
    }

    #[test]
    fn field_with_occurs() {
        let mut entry = make_entry(5, "PHONE-NUMBERS");
        entry.pic = Some(PicClause {
            raw: "X(15)".to_string(),
            category: PicCategory::Alphanumeric,
            total_digits: 0,
            scale: 0,
            signed: false,
            display_length: 15,
            edit_symbols: vec![],
        });
        entry.occurs = Some(5);

        let attr = build_field_attribute(&entry).unwrap();
        assert!(attr.contains("occurs = 5"));
    }

    #[test]
    fn field_with_redefines() {
        let mut entry = make_entry(5, "CREDIT-DATA");
        entry.redefines = Some("CARD-DATA".to_string());
        entry.byte_length = Some(100);

        let attr = build_field_attribute(&entry).unwrap();
        assert!(attr.contains("redefines = \"CARD-DATA\""));
    }

    #[test]
    fn field_with_level88_children() {
        let mut entry = make_entry(5, "CARD-STATUS");
        entry.pic = Some(PicClause {
            raw: "X(1)".to_string(),
            category: PicCategory::Alphanumeric,
            total_digits: 0,
            scale: 0,
            signed: false,
            display_length: 1,
            edit_symbols: vec![],
        });
        entry.children = vec![
            DataEntry {
                level: 88,
                name: "ACTIVE".to_string(),
                condition_values: vec![ConditionValue::Single(Literal::Alphanumeric(
                    "A".to_string(),
                ))],
                ..make_entry(88, "ACTIVE")
            },
            DataEntry {
                level: 88,
                name: "CLOSED".to_string(),
                condition_values: vec![ConditionValue::Single(Literal::Alphanumeric(
                    "C".to_string(),
                ))],
                ..make_entry(88, "CLOSED")
            },
        ];

        let attr = build_field_attribute(&entry).unwrap();
        assert!(attr.contains("level88 = \"ACTIVE:A,CLOSED:C\""));
    }

    #[test]
    fn paragraph_attribute_full() {
        let access = ParagraphFieldAccess {
            reads: ["WS-A", "WS-B"].iter().map(|s| s.to_string()).collect(),
            writes: ["WS-C"].iter().map(|s| s.to_string()).collect(),
            performs: vec!["CALC-PARA".to_string()],
        };
        let attr = build_paragraph_attribute(Some("MONETARY"), &access).unwrap();
        assert!(attr.contains("section = \"MONETARY\""));
        assert!(attr.contains("performs = \"CALC-PARA\""));
        assert!(attr.contains("reads = \"WS-A,WS-B\""));
        assert!(attr.contains("writes = \"WS-C\""));
    }

    #[test]
    fn paragraph_attribute_no_section() {
        let access = ParagraphFieldAccess {
            reads: ["WS-X"].iter().map(|s| s.to_string()).collect(),
            writes: std::collections::BTreeSet::new(),
            performs: vec![],
        };
        let attr = build_paragraph_attribute(None, &access).unwrap();
        assert!(!attr.contains("section"));
        assert!(attr.contains("reads = \"WS-X\""));
    }

    #[test]
    fn paragraph_attribute_empty_returns_none() {
        let access = ParagraphFieldAccess::default();
        assert!(build_paragraph_attribute(None, &access).is_none());
    }

    #[test]
    fn section_attribute() {
        let attr = build_section_attribute("MONETARY-PROCESSING");
        assert_eq!(attr, "#[cobol(section = \"MONETARY-PROCESSING\")]");
    }

    #[test]
    fn field_only_level_returns_none() {
        let entry = make_entry(5, "FILLER");
        assert!(build_field_attribute(&entry).is_none());
    }
}
