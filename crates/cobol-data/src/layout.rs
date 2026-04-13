//! Byte offset computation for COBOL record layouts.
//!
//! Given a hierarchical `DataEntry` tree from cobol-transpiler, compute
//! byte offsets for every field. Handles sequential fields, REDEFINES
//! overlays, OCCURS arrays, and 88-level condition names.

use cobol_transpiler::ast::DataEntry;
use std::collections::HashMap;

/// Compute byte offsets for all fields in a record tree.
///
/// Sets `byte_offset` on every `DataEntry`, starting from offset 0.
/// Also updates `byte_length` for group items based on children span.
///
/// Rules:
/// - Sequential fields get consecutive offsets
/// - REDEFINES fields share the base field's offset (no offset advance)
/// - OCCURS: total space = byte_length * count
/// - Level 88: zero space, inherits parent's offset
/// - Group byte_length = span of children
pub fn compute_layout(record: &mut DataEntry) {
    compute_offsets(record, 0);
}

/// Get the total record length from a list of 01-level entries.
///
/// Returns the byte_length of the first entry, which should be the
/// primary record definition.
pub fn record_length(entries: &[DataEntry]) -> Option<usize> {
    entries.first().and_then(|e| e.byte_length)
}

/// Recursively compute offsets starting at `start_offset`.
/// Returns the next available offset after this item.
fn compute_offsets(item: &mut DataEntry, start_offset: usize) -> usize {
    item.byte_offset = Some(start_offset);

    // Level 88 conditions consume zero space
    if item.level == 88 {
        return start_offset;
    }

    // Separate data children from 88-level conditions
    let has_data_children = item.children.iter().any(|c| c.level != 88);

    if !has_data_children {
        // Leaf item (or group with only 88-level children)
        // Set 88-level children to parent's offset
        for child in &mut item.children {
            child.byte_offset = Some(start_offset);
        }
        let length = item.byte_length.unwrap_or(0);
        let occurs = item.occurs.unwrap_or(1).max(1) as usize;
        return start_offset + length * occurs;
    }

    // Group item: walk children
    let mut offset = start_offset;
    let mut base_offsets: HashMap<String, usize> = HashMap::new();

    for child in &mut item.children {
        if child.level == 88 {
            child.byte_offset = Some(offset);
            continue;
        }

        if let Some(ref target) = child.redefines {
            // REDEFINES: use the base field's offset, don't advance
            let base_off = base_offsets
                .get(&target.to_uppercase())
                .copied()
                .unwrap_or(offset);
            compute_offsets(child, base_off);
        } else {
            // Normal field: record its offset for potential REDEFINES
            base_offsets.insert(child.name.to_uppercase(), offset);
            offset = compute_offsets(child, offset);
        }
    }

    // Update group's byte_length from children span
    let group_length = offset - start_offset;
    item.byte_length = Some(group_length);

    // Apply OCCURS to the group itself
    let occurs = item.occurs.unwrap_or(1).max(1) as usize;
    start_offset + group_length * occurs
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::{PicCategory, PicClause, Usage};

    /// Helper to create a leaf DataEntry.
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

    /// Helper to create a group DataEntry.
    fn group(level: u8, name: &str, children: Vec<DataEntry>) -> DataEntry {
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

    #[test]
    fn test_sequential_fields() {
        // 01 RECORD
        //   05 FIELD-A  PIC X(10)
        //   05 FIELD-B  PIC X(20)
        //   05 FIELD-C  PIC X(5)
        let mut record = group(
            1,
            "RECORD",
            vec![leaf(5, "FIELD-A", 10), leaf(5, "FIELD-B", 20), leaf(5, "FIELD-C", 5)],
        );

        compute_layout(&mut record);

        assert_eq!(record.byte_offset, Some(0));
        assert_eq!(record.byte_length, Some(35));
        assert_eq!(record.children[0].byte_offset, Some(0));
        assert_eq!(record.children[1].byte_offset, Some(10));
        assert_eq!(record.children[2].byte_offset, Some(30));
    }

    #[test]
    fn test_redefines_shares_offset() {
        // 01 RECORD
        //   05 FIELD-A  PIC X(20)
        //   05 FIELD-B REDEFINES FIELD-A  PIC X(20)
        //   05 FIELD-C  PIC X(10)
        let mut field_b = leaf(5, "FIELD-B", 20);
        field_b.redefines = Some("FIELD-A".to_string());

        let mut record = group(
            1,
            "RECORD",
            vec![leaf(5, "FIELD-A", 20), field_b, leaf(5, "FIELD-C", 10)],
        );

        compute_layout(&mut record);

        assert_eq!(record.byte_length, Some(30)); // 20 + 10, not 20 + 20 + 10
        assert_eq!(record.children[0].byte_offset, Some(0));  // FIELD-A
        assert_eq!(record.children[1].byte_offset, Some(0));  // FIELD-B (redefines A)
        assert_eq!(record.children[2].byte_offset, Some(20)); // FIELD-C
    }

    #[test]
    fn test_level_88_no_space() {
        // 01 RECORD
        //   05 STATUS-CODE  PIC X(1)
        //     88 STATUS-ACTIVE VALUE "A"
        //     88 STATUS-INACTIVE VALUE "I"
        //   05 DATA-FIELD  PIC X(10)
        let mut status = leaf(5, "STATUS-CODE", 1);
        let cond_a = DataEntry {
            level: 88,
            name: "STATUS-ACTIVE".to_string(),
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
            byte_length: Some(0),
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        };
        let cond_i = DataEntry {
            level: 88,
            name: "STATUS-INACTIVE".to_string(),
            ..cond_a.clone()
        };
        status.children = vec![cond_a, cond_i];

        let mut record = group(1, "RECORD", vec![status, leaf(5, "DATA-FIELD", 10)]);

        compute_layout(&mut record);

        assert_eq!(record.byte_length, Some(11)); // 1 + 10
        assert_eq!(record.children[0].byte_offset, Some(0)); // STATUS-CODE
        // 88-level conditions get parent's offset
        assert_eq!(record.children[0].children[0].byte_offset, Some(0));
        assert_eq!(record.children[0].children[1].byte_offset, Some(0));
        assert_eq!(record.children[1].byte_offset, Some(1)); // DATA-FIELD
    }

    #[test]
    fn test_occurs_array() {
        // 01 RECORD
        //   05 ITEM  PIC X(10) OCCURS 5
        //   05 TRAILER  PIC X(2)
        let mut item = leaf(5, "ITEM", 10);
        item.occurs = Some(5);

        let mut record = group(1, "RECORD", vec![item, leaf(5, "TRAILER", 2)]);

        compute_layout(&mut record);

        assert_eq!(record.byte_length, Some(52)); // 10*5 + 2
        assert_eq!(record.children[0].byte_offset, Some(0));
        assert_eq!(record.children[1].byte_offset, Some(50));
    }

    #[test]
    fn test_nested_group() {
        // 01 RECORD
        //   05 HEADER
        //     10 HDR-TYPE  PIC X(1)
        //     10 HDR-LEN   PIC X(2)
        //   05 BODY  PIC X(50)
        let header = group(
            5,
            "HEADER",
            vec![leaf(10, "HDR-TYPE", 1), leaf(10, "HDR-LEN", 2)],
        );
        let mut record = group(1, "RECORD", vec![header, leaf(5, "BODY", 50)]);

        compute_layout(&mut record);

        assert_eq!(record.byte_length, Some(53));
        assert_eq!(record.children[0].byte_offset, Some(0)); // HEADER
        assert_eq!(record.children[0].byte_length, Some(3)); // HDR-TYPE + HDR-LEN
        assert_eq!(record.children[0].children[0].byte_offset, Some(0)); // HDR-TYPE
        assert_eq!(record.children[0].children[1].byte_offset, Some(1)); // HDR-LEN
        assert_eq!(record.children[1].byte_offset, Some(3)); // BODY
    }

    #[test]
    fn test_group_with_occurs() {
        // 01 RECORD
        //   05 LINE-ITEM OCCURS 3
        //     10 DESC  PIC X(20)
        //     10 QTY   PIC X(5)
        //   05 TOTAL  PIC X(10)
        let mut line_item = group(
            5,
            "LINE-ITEM",
            vec![leaf(10, "DESC", 20), leaf(10, "QTY", 5)],
        );
        line_item.occurs = Some(3);

        let mut record = group(1, "RECORD", vec![line_item, leaf(5, "TOTAL", 10)]);

        compute_layout(&mut record);

        assert_eq!(record.byte_length, Some(85)); // 25*3 + 10
        assert_eq!(record.children[0].byte_offset, Some(0));
        assert_eq!(record.children[0].byte_length, Some(25)); // per occurrence
        assert_eq!(record.children[1].byte_offset, Some(75)); // after 3 occurrences
    }

    #[test]
    fn test_record_length_helper() {
        let mut entries = vec![group(
            1,
            "RECORD",
            vec![leaf(5, "A", 10), leaf(5, "B", 20)],
        )];
        compute_layout(&mut entries[0]);

        assert_eq!(record_length(&entries), Some(30));
    }

    #[test]
    fn test_record_length_empty() {
        let entries: Vec<DataEntry> = vec![];
        assert_eq!(record_length(&entries), None);
    }

    #[test]
    fn test_comp3_field_in_layout() {
        // COMP-3 field: PIC S9(5)V99 COMP-3 = 4 bytes
        let field = DataEntry {
            level: 5,
            name: "AMOUNT".to_string(),
            pic: Some(PicClause {
                raw: "S9(5)V99".to_string(),
                category: PicCategory::Numeric,
                total_digits: 7,
                scale: 2,
                signed: true,
                display_length: 7,
                edit_symbols: vec![],
            }),
            usage: Usage::Comp3,
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
            byte_length: Some(4), // (7+1)/2 = 4
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        };

        let mut record = group(
            1,
            "RECORD",
            vec![leaf(5, "NAME", 20), field.clone(), leaf(5, "CODE", 3)],
        );

        compute_layout(&mut record);

        assert_eq!(record.byte_length, Some(27)); // 20 + 4 + 3
        assert_eq!(record.children[1].byte_offset, Some(20)); // AMOUNT
        assert_eq!(record.children[2].byte_offset, Some(24)); // CODE
    }

    #[test]
    fn test_multiple_redefines() {
        // 01 RECORD
        //   05 BASE-DATA  PIC X(50)
        //   05 PERSONAL REDEFINES BASE-DATA
        //     10 FIRST-NAME PIC X(25)
        //     10 LAST-NAME  PIC X(25)
        //   05 BUSINESS REDEFINES BASE-DATA
        //     10 COMPANY    PIC X(40)
        //     10 DEPT       PIC X(10)
        //   05 RECORD-TYPE PIC X(1)
        let personal = {
            let mut g = group(
                5,
                "PERSONAL",
                vec![leaf(10, "FIRST-NAME", 25), leaf(10, "LAST-NAME", 25)],
            );
            g.redefines = Some("BASE-DATA".to_string());
            g
        };
        let business = {
            let mut g = group(
                5,
                "BUSINESS",
                vec![leaf(10, "COMPANY", 40), leaf(10, "DEPT", 10)],
            );
            g.redefines = Some("BASE-DATA".to_string());
            g
        };

        let mut record = group(
            1,
            "RECORD",
            vec![
                leaf(5, "BASE-DATA", 50),
                personal,
                business,
                leaf(5, "RECORD-TYPE", 1),
            ],
        );

        compute_layout(&mut record);

        assert_eq!(record.byte_length, Some(51)); // 50 + 1
        assert_eq!(record.children[0].byte_offset, Some(0));  // BASE-DATA
        assert_eq!(record.children[1].byte_offset, Some(0));  // PERSONAL (redefines)
        assert_eq!(record.children[1].children[0].byte_offset, Some(0));  // FIRST-NAME
        assert_eq!(record.children[1].children[1].byte_offset, Some(25)); // LAST-NAME
        assert_eq!(record.children[2].byte_offset, Some(0));  // BUSINESS (redefines)
        assert_eq!(record.children[2].children[0].byte_offset, Some(0));  // COMPANY
        assert_eq!(record.children[2].children[1].byte_offset, Some(40)); // DEPT
        assert_eq!(record.children[3].byte_offset, Some(50)); // RECORD-TYPE
    }
}
