//! OCCURS -> child schema extraction.
//!
//! Each OCCURS group becomes a separate .schema file representing a child
//! table with a foreign key back to the parent. Nested OCCURS produce
//! grandchild schemas with compound FK chains.

use cobol_transpiler::ast::DataEntry;

use crate::dsl::dsl_ast::*;
use crate::dsl::DslFile;

use super::cobol_extract::cobol_name_to_snake;
use super::schema_fields::{count_elementary, flatten_children, is_filler};
use super::schema_keys::detect_key_from_entries;

/// Extracted child schema from an OCCURS group.
#[derive(Debug)]
pub struct OccursSchema {
    /// The generated DslFile for the child schema.
    pub dsl_file: DslFile,
    /// Annotation for the parent schema (field reference to child).
    pub parent_annotation: String,
}

/// A typed key field (name + type) for FK chain propagation.
#[derive(Debug, Clone)]
pub struct TypedKeyField {
    pub name: String,
    pub field_type: FieldType,
}

/// Extract child schemas from OCCURS groups within a list of entries.
///
/// `parent_name` is the schema name of the parent entity.
/// `parent_key_fields` are the PK fields (name + type) of the parent (for FK chain).
/// `program` is the program name for comments.
pub fn extract_occurs_schemas(
    children: &[DataEntry],
    parent_name: &str,
    parent_key_fields: &[TypedKeyField],
    program: &str,
) -> Vec<OccursSchema> {
    let mut results = Vec::new();

    for child in children {
        if child.occurs.is_none() || is_filler(&child.name) {
            continue;
        }

        let occurs_count = child.occurs.unwrap_or(0);
        let child_name = cobol_name_to_snake(&child.name);

        if child.children.is_empty() {
            // Elementary OCCURS: just note it (handled as list field in parent)
            results.push(OccursSchema {
                dsl_file: DslFile {
                    path: String::new(), // marker: no separate file needed
                    content: String::new(),
                    confidence: 1.0,
                    notes: vec![],
                    source_fields: vec![],
                },
                parent_annotation: format!(
                    "// OCCURS {occurs_count}: {} (elementary array, kept in parent as list)",
                    child.name
                ),
            });
            continue;
        }

        // Group OCCURS -> child schema
        let mut fields = Vec::new();
        let mut constraints = Vec::new();
        let mut notes = Vec::new();

        // FK fields from parent (type inherited)
        for pk_field in parent_key_fields {
            fields.push(FieldDecl {
                name: Ident::new(&pk_field.name),
                field_type: pk_field.field_type.clone(),
                required: true,
                comment: Some(format!("FK to {parent_name}")),
            });
        }

        // Sequence number for this OCCURS level
        let seq_field_name = format!("{child_name}_seq");
        fields.push(FieldDecl {
            name: Ident::new(&seq_field_name),
            field_type: FieldType::Integer(None),
            required: true,
            comment: Some(format!("Occurrence index (1 to {occurs_count})")),
        });

        // Detect natural key within the OCCURS group
        let child_key = detect_key_from_entries(&child.children);
        let has_natural_key = !matches!(
            child_key.source,
            super::schema_keys::KeySource::Synthetic | super::schema_keys::KeySource::FirstField
        );

        if has_natural_key {
            notes.push(format!(
                "Natural key detected: {} ({})",
                child_key.identity[0].name,
                child_key.source.description()
            ));
        }

        // Child's own fields (flattened)
        flatten_children(&child.children, &mut fields, &mut constraints);

        // Fix 7: Deduplicate fields by name (FK/seq fields added first take precedence,
        // data fields from flatten_children that duplicate FK names are removed)
        deduplicate_fields(&mut fields);

        // Recurse: nested OCCURS within this OCCURS -> grandchild schemas
        let mut child_key_fields = parent_key_fields.to_vec();
        child_key_fields.push(TypedKeyField {
            name: seq_field_name.clone(),
            field_type: FieldType::Integer(None),
        });
        let grandchildren = extract_occurs_schemas(
            &child.children,
            &child_name,
            &child_key_fields,
            program,
        );

        // Note grandchild references
        for gc in &grandchildren {
            if !gc.parent_annotation.is_empty() {
                notes.push(gc.parent_annotation.clone());
            }
        }

        // Build identity block
        let mut identity_fields = Vec::new();
        for pk_field in parent_key_fields {
            identity_fields.push(FieldDecl {
                name: Ident::new(&pk_field.name),
                field_type: pk_field.field_type.clone(),
                required: true,
                comment: None,
            });
        }
        if has_natural_key {
            identity_fields.extend(child_key.identity);
        } else {
            identity_fields.push(FieldDecl {
                name: Ident::new(&seq_field_name),
                field_type: FieldType::Integer(None),
                required: true,
                comment: None,
            });
        }

        let schema_file = SchemaFile {
            comments: vec![
                Comment("Generated by cobol2rust Nexflow emitter".to_string()),
                Comment(format!(
                    "Child of {parent_name}: {} OCCURS {occurs_count} ({} fields)",
                    child.name,
                    count_elementary(&child.children),
                )),
                Comment(format!("Source: {program}")),
            ],
            imports: vec![],
            schemas: vec![SchemaDef {
                name: Ident::new(&child_name),
                pattern: Some(MutationPattern::MasterData),
                identity: Some(identity_fields),
                fields,
                nested_objects: vec![],
                constraints,
            }],
        };

        let mut all_source_fields = vec![child.name.clone()];
        collect_child_names(&child.children, &mut all_source_fields);

        let occurs_schema = OccursSchema {
            dsl_file: DslFile {
                path: format!("schema/{child_name}.schema"),
                content: schema_file.to_text(),
                confidence: 1.0,
                notes,
                source_fields: all_source_fields,
            },
            parent_annotation: format!(
                "// Child table: {child_name}.schema (OCCURS {occurs_count}, FK: {})",
                parent_key_fields.iter().map(|k| k.name.as_str()).collect::<Vec<_>>().join(", "),
            ),
        };

        results.push(occurs_schema);

        // Add grandchild schemas
        for gc in grandchildren {
            if !gc.dsl_file.path.is_empty() {
                results.push(gc);
            }
        }
    }

    results
}

/// Deduplicate fields by name. Keeps the FIRST occurrence (FK/structural fields).
/// If a data field has the same name as an FK field, the FK version stays.
fn deduplicate_fields(fields: &mut Vec<FieldDecl>) {
    let mut seen = std::collections::HashSet::new();
    fields.retain(|f| seen.insert(f.name.as_str().to_string()));
}

fn collect_child_names(entries: &[DataEntry], out: &mut Vec<String>) {
    for entry in entries {
        if !is_filler(&entry.name) && entry.level != 88 && entry.level != 66 {
            out.push(entry.name.clone());
            collect_child_names(&entry.children, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::*;

    fn make_entry(name: &str, pic_raw: &str, level: u8) -> DataEntry {
        DataEntry {
            level, name: name.to_string(),
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

    fn make_occurs_group(name: &str, occurs: u32, children: Vec<DataEntry>) -> DataEntry {
        DataEntry {
            level: 5, name: name.to_string(),
            pic: None, usage: Usage::Display, value: None, redefines: None,
            occurs: Some(occurs), occurs_depending: None, sign: None,
            justified_right: false, blank_when_zero: false,
            children, condition_values: vec![],
            byte_offset: None, byte_length: None,
            renames_target: None, renames_thru: None, index_names: vec![],
        }
    }

    #[test]
    fn occurs_group_produces_child_schema() {
        let entries = vec![make_occurs_group("WS-LINE-ITEM", 50, vec![
            make_entry("WS-ITEM-SKU", "X(12)", 10),
            make_entry("WS-ITEM-QTY", "9(5)", 10),
        ])];

        let results = extract_occurs_schemas(&entries, "ws_order", &[TypedKeyField { name: "ws_order_id".to_string(), field_type: FieldType::String(Some(10)) }], "TEST");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].dsl_file.path, "schema/ws_line_item.schema");

        let content = &results[0].dsl_file.content;
        assert!(content.contains("schema ws_line_item"), "Content: {content}");
        assert!(content.contains("ws_order_id"), "Should have FK: {content}");
        assert!(content.contains("ws_line_item_seq"), "Should have seq: {content}");
        assert!(content.contains("ws_item_sku"), "Content: {content}");
        assert!(content.contains("OCCURS 50"), "Content: {content}");
    }

    #[test]
    fn nested_occurs_produces_grandchild() {
        let inner = make_occurs_group("WS-SKILL", 10, vec![
            make_entry("WS-SKILL-CODE", "X(8)", 20),
        ]);
        let outer = make_occurs_group("WS-EMPLOYEE", 100, vec![
            make_entry("WS-EMP-ID", "X(10)", 10),
            inner,
        ]);

        let results = extract_occurs_schemas(&[outer], "ws_dept", &[TypedKeyField { name: "ws_dept_code".to_string(), field_type: FieldType::String(Some(4)) }], "TEST");
        // Should produce 2: ws_employee + ws_skill
        assert!(results.len() >= 2, "Got {} results", results.len());

        let emp = results.iter().find(|r| r.dsl_file.path.contains("ws_employee"));
        assert!(emp.is_some(), "Should have employee schema");

        let skill = results.iter().find(|r| r.dsl_file.path.contains("ws_skill"));
        assert!(skill.is_some(), "Should have skill schema");
    }
}
