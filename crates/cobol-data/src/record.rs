//! Record-level decoding with REDEFINES variant selection.
//!
//! Walks a `DataEntry` tree over a fixed-length binary record, applies
//! discriminator logic to select REDEFINES variants, and decodes leaf
//! fields into a JSON map.

use crate::decode::{self, DecodedValue};
use crate::encoding::Encoding;
use crate::redefines::RedefinesGroup;
use cobol_transpiler::ast::DataEntry;
use serde::Serialize;

/// A decoded record: field values + selected variant + warnings.
#[derive(Debug, Clone, Serialize)]
pub struct DecodedRecord {
    /// Decoded field values keyed by COBOL field name.
    pub fields: serde_json::Map<String, serde_json::Value>,
    /// Name of the selected REDEFINES variant (if any).
    pub variant: Option<String>,
    /// Warnings generated during decoding (non-fatal errors).
    pub warnings: Vec<String>,
}

/// Decode a single fixed-length record from binary bytes.
pub fn decode_record(
    data: &[u8],
    entries: &[DataEntry],
    groups: &[RedefinesGroup],
    encoding: Encoding,
) -> DecodedRecord {
    let mut record = DecodedRecord {
        fields: serde_json::Map::new(),
        variant: None,
        warnings: Vec::new(),
    };

    for entry in entries {
        decode_children(data, &entry.children, groups, encoding, &mut record, 0);
    }

    record
}

/// Batch decode: split data into fixed-length records and decode each.
pub fn decode_records(
    data: &[u8],
    record_length: usize,
    entries: &[DataEntry],
    groups: &[RedefinesGroup],
    encoding: Encoding,
    limit: Option<usize>,
) -> Vec<DecodedRecord> {
    if record_length == 0 || data.is_empty() {
        return Vec::new();
    }

    let total = data.len() / record_length;
    let count = limit.map_or(total, |l| l.min(total));
    let mut records = Vec::with_capacity(count);

    for i in 0..count {
        let start = i * record_length;
        let end = start + record_length;
        if end > data.len() {
            break;
        }
        records.push(decode_record(&data[start..end], entries, groups, encoding));
    }

    records
}

/// Convert a `DecodedRecord` to a JSON value (adding _variant and _warnings metadata).
pub fn decoded_to_json(record: &DecodedRecord) -> serde_json::Value {
    let mut map = record.fields.clone();
    if let Some(ref v) = record.variant {
        map.insert(
            "_variant".to_string(),
            serde_json::Value::String(v.clone()),
        );
    }
    if !record.warnings.is_empty() {
        map.insert(
            "_warnings".to_string(),
            serde_json::json!(record.warnings),
        );
    }
    serde_json::Value::Object(map)
}

/// Recursively decode children, handling REDEFINES groups, OCCURS, and leaves.
fn decode_children(
    data: &[u8],
    children: &[DataEntry],
    groups: &[RedefinesGroup],
    encoding: Encoding,
    record: &mut DecodedRecord,
    offset_adj: usize,
) {
    let mut skip_redefines_of: Option<String> = None;

    for child in children {
        // Skip 88-level conditions
        if child.level == 88 {
            continue;
        }

        // Check if this child is a REDEFINES variant we should skip
        if let Some(ref target) = child.redefines {
            if let Some(ref skip) = skip_redefines_of {
                if target.eq_ignore_ascii_case(skip) {
                    continue; // Already handled by base field's group logic
                }
            }
        }

        // Check if this child is the base of a REDEFINES group
        let child_name_upper = child.name.to_uppercase();
        if let Some(group) = groups
            .iter()
            .find(|g| g.base_field == child_name_upper)
        {
            skip_redefines_of = Some(child_name_upper.clone());
            decode_redefines_group(
                data, child, children, group, encoding, record, offset_adj,
            );
            continue;
        }

        // Skip standalone REDEFINES that weren't handled by a group
        if child.redefines.is_some() {
            continue;
        }

        // Group or leaf?
        let has_data_children = child.children.iter().any(|c| c.level != 88);

        if has_data_children {
            // Group item
            if child.occurs.is_some() {
                // Group with OCCURS: decode array of groups
                let per_len = child.byte_length.unwrap_or(0);
                let count = effective_occurs(child, record);
                for occ in 0..count {
                    let adj = offset_adj + occ * per_len;
                    decode_children(data, &child.children, groups, encoding, record, adj);
                }
            } else {
                decode_children(
                    data,
                    &child.children,
                    groups,
                    encoding,
                    record,
                    offset_adj,
                );
            }
        } else {
            // Leaf item
            decode_leaf(data, child, encoding, record, offset_adj);
        }
    }
}

/// Decode a REDEFINES group: read discriminator, select variant, decode it.
fn decode_redefines_group(
    data: &[u8],
    base_child: &DataEntry,
    siblings: &[DataEntry],
    group: &RedefinesGroup,
    encoding: Encoding,
    record: &mut DecodedRecord,
    offset_adj: usize,
) {
    if let Some(ref disc) = group.discriminator {
        // Read discriminator value
        if let Some(disc_value) = read_discriminator_value(data, disc, siblings, encoding, offset_adj)
        {
            // Match against rules
            if let Some(variant_name) = disc
                .rules
                .iter()
                .find(|r| r.value == disc_value)
                .map(|r| &r.selects)
            {
                // Find the variant DataEntry among siblings
                if let Some(variant_entry) = siblings
                    .iter()
                    .find(|s| s.name.eq_ignore_ascii_case(variant_name))
                {
                    record.variant = Some(variant_name.clone());
                    let has_data_children =
                        variant_entry.children.iter().any(|c| c.level != 88);
                    if has_data_children {
                        decode_children(
                            data,
                            &variant_entry.children,
                            &[],
                            encoding,
                            record,
                            offset_adj,
                        );
                    } else {
                        decode_leaf(data, variant_entry, encoding, record, offset_adj);
                    }
                    return;
                }
            }

            // No matching rule -- warn and fall through to base
            record.warnings.push(format!(
                "REDEFINES '{}': discriminator '{}' = '{}' did not match any variant",
                group.base_field, disc.field, disc_value
            ));
        }
    }

    // Fallback: decode base field as-is
    let has_data_children = base_child.children.iter().any(|c| c.level != 88);
    if has_data_children {
        decode_children(
            data,
            &base_child.children,
            &[],
            encoding,
            record,
            offset_adj,
        );
    } else {
        decode_leaf(data, base_child, encoding, record, offset_adj);
    }
}

/// Read the discriminator field's value from the binary data.
fn read_discriminator_value(
    data: &[u8],
    disc: &crate::redefines::Discriminator,
    siblings: &[DataEntry],
    encoding: Encoding,
    offset_adj: usize,
) -> Option<String> {
    // Find the discriminator field in the tree
    let disc_entry = find_field_by_name(siblings, &disc.field)?;

    let offset = disc_entry.byte_offset.unwrap_or(0) + offset_adj;
    let length = disc_entry.byte_length.unwrap_or(0);

    if offset + length > data.len() || length == 0 {
        return None;
    }

    let bytes = &data[offset..offset + length];

    // Decode based on field type
    if let Some(ref pic) = disc_entry.pic {
        match decode::decode_field(bytes, pic, disc_entry.usage, encoding) {
            Ok(DecodedValue::Text(s)) => Some(s.trim().to_string()),
            Ok(DecodedValue::Integer(n)) => Some(n.to_string()),
            Ok(DecodedValue::Decimal { value, scale }) => {
                if scale == 0 {
                    Some(value.to_string())
                } else {
                    Some(format!("{value}E-{scale}"))
                }
            }
            _ => None,
        }
    } else {
        // Group or no PIC: decode as text
        Some(crate::encoding::decode_text(bytes, encoding).trim().to_string())
    }
}

/// Find a DataEntry by name (case-insensitive) in a flat list or recursively.
fn find_field_by_name<'a>(entries: &'a [DataEntry], name: &str) -> Option<&'a DataEntry> {
    for entry in entries {
        if entry.name.eq_ignore_ascii_case(name) {
            return Some(entry);
        }
        if let Some(found) = find_field_by_name(&entry.children, name) {
            return Some(found);
        }
    }
    None
}

/// Decode a leaf field and insert into the record.
fn decode_leaf(
    data: &[u8],
    entry: &DataEntry,
    encoding: Encoding,
    record: &mut DecodedRecord,
    offset_adj: usize,
) {
    let pic = match &entry.pic {
        Some(p) => p,
        None => return, // No PIC = group or 88-level, skip
    };

    if entry.occurs.is_some() {
        // Leaf with OCCURS: decode array
        let per_len = entry.byte_length.unwrap_or(0);
        let count = effective_occurs(entry, record);
        let mut arr = Vec::with_capacity(count);

        for occ in 0..count {
            let offset = entry.byte_offset.unwrap_or(0) + offset_adj + occ * per_len;
            if offset + per_len <= data.len() {
                match decode::decode_field(&data[offset..offset + per_len], pic, entry.usage, encoding)
                {
                    Ok(val) => arr.push(val.to_json()),
                    Err(e) => {
                        record.warnings.push(format!(
                            "{}[{}]: {}",
                            entry.name,
                            occ + 1,
                            e
                        ));
                        arr.push(serde_json::Value::Null);
                    }
                }
            }
        }

        let key = unique_key(&entry.name, &record.fields);
        record.fields.insert(key, serde_json::Value::Array(arr));
    } else {
        // Single value
        let offset = entry.byte_offset.unwrap_or(0) + offset_adj;
        let length = entry.byte_length.unwrap_or(0);

        if offset + length <= data.len() && length > 0 {
            match decode::decode_field(&data[offset..offset + length], pic, entry.usage, encoding) {
                Ok(val) => {
                    let key = unique_key(&entry.name, &record.fields);
                    record.fields.insert(key, val.to_json());
                }
                Err(e) => {
                    record.warnings.push(format!("{}: {}", entry.name, e));
                }
            }
        }
    }
}

/// Compute effective OCCURS count, respecting DEPENDING ON.
fn effective_occurs(entry: &DataEntry, record: &DecodedRecord) -> usize {
    let max = entry.occurs.unwrap_or(0) as usize;

    if let Some(ref dep_field) = entry.occurs_depending {
        // Look up the controlling field's value in already-decoded fields
        let dep_upper = dep_field.to_uppercase();
        if let Some(val) = record.fields.get(&dep_upper) {
            if let Some(n) = val.as_i64() {
                return (n.max(0) as usize).min(max);
            }
        }
        // Controlling field not decoded yet -- use max
        max
    } else {
        max
    }
}

/// Generate a unique key for the field name in the record map.
/// Handles duplicate names (e.g., FILLER) by appending -2, -3, etc.
fn unique_key(name: &str, fields: &serde_json::Map<String, serde_json::Value>) -> String {
    if !fields.contains_key(name) {
        return name.to_string();
    }
    let mut n = 2;
    loop {
        let candidate = format!("{name}-{n}");
        if !fields.contains_key(&candidate) {
            return candidate;
        }
        n += 1;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::Encoding;
    use crate::layout::compute_layout;
    use crate::redefines::{
        Confidence, Discriminator, DiscriminatorPattern, DiscriminatorRule, RedefinesGroup,
        RedefinesVariant,
    };
    use cobol_transpiler::ast::{PicCategory, PicClause, Usage};

    fn leaf(level: u8, name: &str, length: usize) -> DataEntry {
        DataEntry {
            level,
            name: name.to_string(),
            pic: Some(PicClause {
                raw: format!("X({length})"),
                category: PicCategory::Alphanumeric,
                total_digits: 0,
                scale: 0,
                signed: false,
                display_length: length as u32,
                edit_symbols: vec![],
            }),
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
            byte_length: Some(length),
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }
    }

    fn numeric_leaf(level: u8, name: &str, digits: u32, scale: u32, usage: Usage) -> DataEntry {
        let byte_len = match usage {
            Usage::Comp3 => ((digits + 2) / 2) as usize, // packed: (digits + 1 sign nibble + 1) / 2
            Usage::Comp | Usage::Comp5 => {
                if digits <= 4 {
                    2
                } else if digits <= 9 {
                    4
                } else {
                    8
                }
            }
            _ => digits as usize, // display
        };
        DataEntry {
            level,
            name: name.to_string(),
            pic: Some(PicClause {
                raw: format!("S9({digits})"),
                category: PicCategory::Numeric,
                total_digits: digits,
                scale,
                signed: true,
                display_length: digits,
                edit_symbols: vec![],
            }),
            usage,
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
            byte_length: Some(byte_len),
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }
    }

    fn group_entry(level: u8, name: &str, children: Vec<DataEntry>) -> DataEntry {
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
            children,
            condition_values: vec![],
            byte_offset: None,
            byte_length: None,
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }
    }

    /// Build a simple record and compute layout.
    fn build_simple_record() -> Vec<DataEntry> {
        // 01 RECORD (35 bytes)
        //   05 NAME    PIC X(20)   offset 0
        //   05 CITY    PIC X(10)   offset 20
        //   05 CODE    PIC X(5)    offset 30
        let mut record = group_entry(
            1,
            "RECORD",
            vec![leaf(5, "NAME", 20), leaf(5, "CITY", 10), leaf(5, "CODE", 5)],
        );
        compute_layout(&mut record);
        vec![record]
    }

    #[test]
    fn test_decode_simple_ascii_record() {
        let entries = build_simple_record();
        // 35-byte ASCII record
        let data = b"John Doe            New York  12345";
        let record = decode_record(data, &entries, &[], Encoding::Ascii);

        assert_eq!(record.fields.get("NAME").unwrap(), "John Doe");
        assert_eq!(record.fields.get("CITY").unwrap(), "New York");
        assert_eq!(record.fields.get("CODE").unwrap(), "12345");
        assert!(record.warnings.is_empty());
    }

    #[test]
    fn test_decode_records_batch() {
        let entries = build_simple_record();
        // Two 35-byte records
        let mut data = Vec::new();
        data.extend_from_slice(b"Alice               Boston    AAAAA");
        data.extend_from_slice(b"Bob                 Chicago   BBBBB");

        let records = decode_records(&data, 35, &entries, &[], Encoding::Ascii, None);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].fields.get("NAME").unwrap(), "Alice");
        assert_eq!(records[1].fields.get("NAME").unwrap(), "Bob");
    }

    #[test]
    fn test_decode_records_with_limit() {
        let entries = build_simple_record();
        let mut data = Vec::new();
        data.extend_from_slice(b"Alice               Boston    AAAAA");
        data.extend_from_slice(b"Bob                 Chicago   BBBBB");

        let records = decode_records(&data, 35, &entries, &[], Encoding::Ascii, Some(1));
        assert_eq!(records.len(), 1);
    }

    #[test]
    fn test_decode_with_redefines_discriminator() {
        // 01 RECORD (22 bytes)
        //   05 REC-TYPE  PIC X(1)    offset 0
        //   05 BASE-DATA PIC X(20)   offset 1
        //   05 PERSONAL REDEFINES BASE-DATA
        //     10 FIRST-NAME PIC X(10)
        //     10 LAST-NAME  PIC X(10)
        //   05 BUSINESS REDEFINES BASE-DATA
        //     10 COMPANY    PIC X(20)
        //   05 UNUSED     PIC X(1)   offset 21

        let mut personal = group_entry(
            5,
            "PERSONAL",
            vec![leaf(10, "FIRST-NAME", 10), leaf(10, "LAST-NAME", 10)],
        );
        personal.redefines = Some("BASE-DATA".to_string());

        let mut business = group_entry(5, "BUSINESS", vec![leaf(10, "COMPANY", 20)]);
        business.redefines = Some("BASE-DATA".to_string());

        let mut record = group_entry(
            1,
            "RECORD",
            vec![
                leaf(5, "REC-TYPE", 1),
                leaf(5, "BASE-DATA", 20),
                personal,
                business,
                leaf(5, "UNUSED", 1),
            ],
        );
        compute_layout(&mut record);

        let groups = vec![RedefinesGroup {
            base_field: "BASE-DATA".to_string(),
            byte_offset: 1,
            byte_length: 20,
            variants: vec![
                RedefinesVariant {
                    name: "PERSONAL".to_string(),
                    fields: vec!["FIRST-NAME".to_string(), "LAST-NAME".to_string()],
                },
                RedefinesVariant {
                    name: "BUSINESS".to_string(),
                    fields: vec!["COMPANY".to_string()],
                },
            ],
            discriminator: Some(Discriminator {
                field: "REC-TYPE".to_string(),
                pattern_type: DiscriminatorPattern::DirectIf,
                rules: vec![
                    DiscriminatorRule {
                        value: "P".to_string(),
                        selects: "PERSONAL".to_string(),
                    },
                    DiscriminatorRule {
                        value: "B".to_string(),
                        selects: "BUSINESS".to_string(),
                    },
                ],
                confidence: Confidence::High,
            }),
        }];

        // Personal record: P(1) + FIRST-NAME(10) + LAST-NAME(10) + UNUSED(1) = 22 bytes
        let data_p = b"PJohn      Doe       X";
        assert_eq!(data_p.len(), 22);
        let rec = decode_record(data_p, &[record.clone()], &groups, Encoding::Ascii);
        assert_eq!(rec.variant, Some("PERSONAL".to_string()));
        assert_eq!(rec.fields.get("FIRST-NAME").unwrap(), "John");
        assert_eq!(rec.fields.get("LAST-NAME").unwrap(), "Doe");
        assert!(rec.fields.get("COMPANY").is_none());

        // Business record: B(1) + COMPANY(20) + UNUSED(1) = 22 bytes
        let data_b = b"BAcme Corporation    X";
        assert_eq!(data_b.len(), 22);
        let rec = decode_record(data_b, &[record.clone()], &groups, Encoding::Ascii);
        assert_eq!(rec.variant, Some("BUSINESS".to_string()));
        // COMPANY field is 20 bytes starting at offset 1: "Acme Corporation   X"
        // But wait -- UNUSED is at offset 21. COMPANY reads bytes 1..21 (20 bytes).
        // "Acme Corporation   " (19 chars) + "X" = the X is still in the 20-byte range.
        // Actually the full 22 bytes: B(0) | Acme Corporation   (1-20) | X(21)
        // COMPANY is at offset 1, length 20, reads "Acme Corporation   " -- no X!
        // Let me verify: "BAcme Corporation   X" has 22 chars.
        // offset 1..21 = "Acme Corporation   " (20 chars with trailing spaces)
        assert_eq!(rec.fields.get("COMPANY").unwrap(), "Acme Corporation");
    }

    #[test]
    fn test_decode_unresolved_redefines() {
        let mut record = group_entry(
            1,
            "RECORD",
            vec![leaf(5, "REC-TYPE", 1), leaf(5, "BASE-DATA", 10)],
        );
        compute_layout(&mut record);

        let groups = vec![RedefinesGroup {
            base_field: "BASE-DATA".to_string(),
            byte_offset: 1,
            byte_length: 10,
            variants: vec![RedefinesVariant {
                name: "VAR-A".to_string(),
                fields: vec!["FIELD-A".to_string()],
            }],
            discriminator: Some(Discriminator {
                field: "REC-TYPE".to_string(),
                pattern_type: DiscriminatorPattern::DirectIf,
                rules: vec![DiscriminatorRule {
                    value: "A".to_string(),
                    selects: "VAR-A".to_string(),
                }],
                confidence: Confidence::High,
            }),
        }];

        // Unknown discriminator value "X"
        let data = b"XSomeData__";
        let rec = decode_record(data, &[record], &groups, Encoding::Ascii);
        assert!(rec.variant.is_none());
        assert!(!rec.warnings.is_empty());
    }

    #[test]
    fn test_decode_occurs_leaf() {
        // 01 RECORD
        //   05 ITEM PIC X(5) OCCURS 3
        let mut item = leaf(5, "ITEM", 5);
        item.occurs = Some(3);
        let mut record = group_entry(1, "RECORD", vec![item]);
        compute_layout(&mut record);

        let data = b"AAAAABBBBBCCCCC";
        let rec = decode_record(data, &[record], &[], Encoding::Ascii);
        let arr = rec.fields.get("ITEM").unwrap().as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], "AAAAA");
        assert_eq!(arr[1], "BBBBB");
        assert_eq!(arr[2], "CCCCC");
    }

    #[test]
    fn test_decode_comp3_field() {
        // 01 RECORD
        //   05 NAME    PIC X(10)
        //   05 AMOUNT  PIC S9(5)V99 COMP-3  (4 bytes)
        let mut record = group_entry(
            1,
            "RECORD",
            vec![
                leaf(5, "NAME", 10),
                numeric_leaf(5, "AMOUNT", 7, 2, Usage::Comp3),
            ],
        );
        compute_layout(&mut record);

        // 10 bytes name + 4 bytes COMP-3
        let mut data = Vec::new();
        data.extend_from_slice(b"Test      "); // NAME (10 bytes)
        data.extend_from_slice(&[0x01, 0x23, 0x45, 0x6C]); // +12345.6 packed (1234560 / scale=2 -> wait)
        // Actually: nibbles 0,1,2,3,4,5,6,C -> value 123456, scale 2 -> 1234.56

        let rec = decode_record(&data, &[record], &[], Encoding::Ascii);
        assert_eq!(rec.fields.get("NAME").unwrap(), "Test");
        // COMP-3: 0x01 0x23 0x45 0x6C -> digits 0,1,2,3,4,5,6, sign C
        // value = 123456, scale = 2 -> 1234.56
        let amount = rec.fields.get("AMOUNT").unwrap().as_f64().unwrap();
        assert!((amount - 1234.56).abs() < 0.01);
    }

    #[test]
    fn test_unique_key_dedup() {
        let mut fields = serde_json::Map::new();
        fields.insert("FILLER".to_string(), serde_json::json!("a"));
        assert_eq!(unique_key("FILLER", &fields), "FILLER-2");

        fields.insert("FILLER-2".to_string(), serde_json::json!("b"));
        assert_eq!(unique_key("FILLER", &fields), "FILLER-3");
    }

    #[test]
    fn test_decoded_to_json() {
        let mut fields = serde_json::Map::new();
        fields.insert("NAME".to_string(), serde_json::json!("Alice"));
        let record = DecodedRecord {
            fields,
            variant: Some("PERSONAL".to_string()),
            warnings: vec!["test warning".to_string()],
        };

        let json = decoded_to_json(&record);
        let obj = json.as_object().unwrap();
        assert_eq!(obj.get("NAME").unwrap(), "Alice");
        assert_eq!(obj.get("_variant").unwrap(), "PERSONAL");
        assert!(obj.get("_warnings").unwrap().as_array().unwrap().len() == 1);
    }

    #[test]
    fn test_nested_group_decode() {
        // 01 RECORD
        //   05 HEADER
        //     10 TYPE  PIC X(1)
        //     10 LEN   PIC X(2)
        //   05 BODY  PIC X(10)
        let header = group_entry(
            5,
            "HEADER",
            vec![leaf(10, "TYPE", 1), leaf(10, "LEN", 2)],
        );
        let mut record = group_entry(1, "RECORD", vec![header, leaf(5, "BODY", 10)]);
        compute_layout(&mut record);

        let data = b"A10HelloWorld!";
        let rec = decode_record(data, &[record], &[], Encoding::Ascii);
        assert_eq!(rec.fields.get("TYPE").unwrap(), "A");
        assert_eq!(rec.fields.get("LEN").unwrap(), "10");
        // "HelloWorld!" is 11 chars but field is 10
        assert_eq!(rec.fields.get("BODY").unwrap(), "HelloWorld");
    }

    #[test]
    fn test_decode_empty_data() {
        let records = decode_records(&[], 10, &[], &[], Encoding::Ascii, None);
        assert!(records.is_empty());
    }

    #[test]
    fn test_decode_zero_record_length() {
        let records = decode_records(b"data", 0, &[], &[], Encoding::Ascii, None);
        assert!(records.is_empty());
    }

    #[test]
    fn test_occurs_depending_on() {
        // 01 RECORD
        //   05 COUNT   PIC 9(2)     offset 0 (2 bytes display numeric)
        //   05 ITEMS   PIC X(5)     OCCURS 10 DEPENDING ON COUNT
        let count_field = DataEntry {
            level: 5,
            name: "COUNT".to_string(),
            pic: Some(PicClause {
                raw: "9(2)".to_string(),
                category: PicCategory::Numeric,
                total_digits: 2,
                scale: 0,
                signed: false,
                display_length: 2,
                edit_symbols: vec![],
            }),
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
            byte_length: Some(2),
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        };

        let mut items_field = leaf(5, "ITEMS", 5);
        items_field.occurs = Some(10);
        items_field.occurs_depending = Some("COUNT".to_string());

        let mut record = group_entry(1, "RECORD", vec![count_field, items_field]);
        compute_layout(&mut record);

        // COUNT = "03" (ASCII), then 3 items of 5 bytes each
        let mut data = Vec::new();
        data.extend_from_slice(b"03"); // COUNT
        data.extend_from_slice(b"AAAAABBBBBCCCCC"); // 3 items
        // Pad to fill potential 10 items
        data.extend_from_slice(&[0x20; 35]); // 7 more items of spaces

        let rec = decode_record(&data, &[record], &[], Encoding::Ascii);
        assert_eq!(rec.fields.get("COUNT").unwrap(), 3);
        let arr = rec.fields.get("ITEMS").unwrap().as_array().unwrap();
        assert_eq!(arr.len(), 3); // DEPENDING ON limits to 3
        assert_eq!(arr[0], "AAAAA");
        assert_eq!(arr[1], "BBBBB");
        assert_eq!(arr[2], "CCCCC");
    }
}
