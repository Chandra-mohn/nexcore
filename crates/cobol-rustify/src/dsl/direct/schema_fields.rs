//! Field mapping: DataEntry -> FieldDecl.
//!
//! Converts COBOL AST data entries into DSL field declarations.
//! Handles PIC -> type mapping, FILLER dropping, level-88 -> constraints.

use cobol_transpiler::ast::{ConditionValue, DataEntry, Literal, Usage};

use crate::dsl::dsl_ast::{FieldDecl, FieldType, Ident, SchemaConstraint};
use crate::dsl::type_mapping::{pic_to_nexflow_type, NexflowType};

use super::cobol_extract::cobol_name_to_snake;

/// Convert a single elementary DataEntry to a FieldDecl.
/// Returns None for FILLER, level-66, and level-88 entries.
pub fn entry_to_field_decl(entry: &DataEntry) -> Option<FieldDecl> {
    if is_filler(&entry.name) || entry.level == 66 || entry.level == 88 {
        return None;
    }

    let snake_name = cobol_name_to_snake(&entry.name);

    let nexflow_type = if let Some(pic) = &entry.pic {
        // Fix 4: Edited PIC types are display strings, not numbers
        if matches!(
            pic.category,
            cobol_transpiler::ast::PicCategory::NumericEdited
                | cobol_transpiler::ast::PicCategory::AlphanumericEdited
        ) {
            NexflowType::String(Some(pic.display_length as usize))
        } else {
            pic_to_nexflow_type(&pic.raw, usage_to_str(&entry.usage), pic.signed, &snake_name)
        }
    } else {
        // Fix 5: COMP-1/COMP-2 without PIC clause are floats, not strings
        match entry.usage {
            Usage::Comp1 | Usage::Comp2 => NexflowType::Float,
            Usage::Index | Usage::Pointer => NexflowType::Integer(None),
            _ => NexflowType::String(None),
        }
    };

    let comment = entry.pic.as_ref().map(|pic| {
        format!("COBOL: {} PIC {}", entry.name, pic.raw)
    });

    Some(FieldDecl {
        name: Ident::new(&snake_name),
        field_type: nexflow_type.to_field_type(),
        required: true,
        comment,
    })
}

/// Flatten a group's children into a list of FieldDecls.
/// Sub-groups become comment separators. FILLER/88/66 dropped.
/// OCCURS and REDEFINES children are SKIPPED (handled by their own strategies).
pub fn flatten_children(
    children: &[DataEntry],
    fields: &mut Vec<FieldDecl>,
    constraints: &mut Vec<SchemaConstraint>,
) {
    for child in children {
        if is_filler(&child.name) || child.level == 66 || child.level == 88 {
            continue;
        }

        // Skip OCCURS -- handled by schema_occurs.rs
        if child.occurs.is_some() {
            continue;
        }

        // Skip REDEFINES -- handled by schema_redefines.rs
        if child.redefines.is_some() {
            continue;
        }

        if child.children.is_empty() {
            // Elementary field
            if let Some(field) = entry_to_field_decl(child) {
                fields.push(field);
            }
            // Collect level-88 constraints
            let l88 = collect_level88(child);
            if !l88.is_empty() {
                let field_name = cobol_name_to_snake(&child.name);
                constraints.push(SchemaConstraint::Enum(Ident::new(&field_name), l88));
            }
        } else {
            // Sub-group: add comment separator then recurse
            fields.push(FieldDecl {
                name: Ident::new(&cobol_name_to_snake(&child.name)),
                field_type: FieldType::String(None),
                required: false,
                comment: Some(format!("-- group: {} --", child.name)),
            });
            flatten_children(&child.children, fields, constraints);
        }
    }
}

/// Collect level-88 condition values from an entry's children.
pub fn collect_level88(entry: &DataEntry) -> Vec<String> {
    if !entry.condition_values.is_empty() {
        return entry
            .condition_values
            .iter()
            .filter_map(|cv| match cv {
                ConditionValue::Single(lit) => Some(literal_to_string(lit)),
                ConditionValue::Range { .. } => None,
            })
            .collect();
    }

    let mut values = Vec::new();
    for child in &entry.children {
        if child.level == 88 {
            for cv in &child.condition_values {
                if let ConditionValue::Single(lit) = cv {
                    values.push(literal_to_string(lit));
                }
            }
        }
    }
    values
}

pub fn is_filler(name: &str) -> bool {
    name.eq_ignore_ascii_case("FILLER")
}

/// Check if an entry has structural children (not just level-88/66 metadata).
/// An entry with ONLY 88/66 children is still an elementary field.
pub fn has_structural_children(entry: &DataEntry) -> bool {
    entry.children.iter().any(|c| c.level != 88 && c.level != 66)
}

fn usage_to_str(usage: &Usage) -> Option<&'static str> {
    match usage {
        Usage::Display => None,
        Usage::Comp => Some("comp"),
        Usage::Comp3 => Some("comp3"),
        Usage::Comp5 => Some("comp5"),
        Usage::Comp1 => Some("comp1"),
        Usage::Comp2 => Some("comp2"),
        Usage::Index | Usage::Pointer => None,
    }
}

fn literal_to_string(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(s) | Literal::Alphanumeric(s) => s.clone(),
        Literal::Figurative(fc) => format!("{fc:?}"),
    }
}

/// Count elementary fields in a DataEntry tree (excluding FILLER/88/66).
pub fn count_elementary(children: &[DataEntry]) -> usize {
    let mut count = 0;
    for child in children {
        if child.level == 88 || child.level == 66 || is_filler(&child.name) {
            continue;
        }
        if child.children.is_empty() {
            count += 1;
        } else {
            count += count_elementary(&child.children);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::{PicCategory, PicClause};

    fn make_entry(name: &str, pic_raw: &str) -> DataEntry {
        DataEntry {
            level: 5,
            name: name.to_string(),
            pic: Some(PicClause {
                raw: pic_raw.to_string(),
                category: if pic_raw.contains('X') {
                    PicCategory::Alphanumeric
                } else {
                    PicCategory::Numeric
                },
                total_digits: 10,
                scale: 0,
                signed: pic_raw.contains('S'),
                display_length: 10,
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
            byte_length: None,
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }
    }

    #[test]
    fn filler_dropped() {
        assert!(entry_to_field_decl(&make_entry("FILLER", "X(10)")).is_none());
        assert!(entry_to_field_decl(&make_entry("filler", "X(10)")).is_none());
    }

    #[test]
    fn elementary_field_mapped() {
        let field = entry_to_field_decl(&make_entry("WS-ACCT-NUM", "X(10)")).unwrap();
        assert_eq!(field.name.as_str(), "ws_acct_num");
        assert_eq!(field.field_type, FieldType::String(Some(10)));
        assert!(field.required);
    }

    #[test]
    fn numeric_field_mapped() {
        let field = entry_to_field_decl(&make_entry("WS-BALANCE", "S9(9)V99")).unwrap();
        assert_eq!(field.name.as_str(), "ws_balance");
        assert!(matches!(field.field_type, FieldType::Decimal(_, _)));
    }
}
