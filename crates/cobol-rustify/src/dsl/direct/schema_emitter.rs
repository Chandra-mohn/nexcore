//! Direct schema emitter: orchestrates strategy-based schema generation.
//!
//! Classifies each data source in the COBOL program and delegates to the
//! appropriate strategy module:
//!
//! - DB2 (EXEC SQL) -> schema_sql.rs (1:1 relational mapping)
//! - FILE SECTION (FD) -> file-based: pattern from org, key from SELECT, OCCURS/REDEFINES
//! - WORKING-STORAGE groups -> flat schema with heuristic key
//! - LINKAGE/COMMAREA -> command/response pattern
//!
//! Strategy modules:
//! - schema_fields.rs  -- DataEntry -> FieldDecl
//! - schema_pattern.rs -- File org -> MutationPattern
//! - schema_keys.rs    -- PK detection (RECORD KEY, heuristics, synthetic)
//! - schema_occurs.rs  -- OCCURS -> child schemas with FK
//! - schema_redefines.rs -- REDEFINES -> variant schemas with discriminator
//! - schema_sql.rs     -- EXEC SQL -> DB2 table schemas

use cobol_transpiler::ast::DataEntry;

use crate::dsl::dsl_ast::*;
use crate::dsl::{DslFile, DslLayer};

use super::cobol_extract::cobol_name_to_snake;
use super::schema_fields::{count_elementary, flatten_children, has_structural_children, is_filler};
use super::schema_keys::{detect_key_from_entries, detect_key_from_file};
use super::schema_occurs::extract_occurs_schemas;
use super::schema_redefines::extract_redefines_schemas;
use super::{DirectDslEmitter, DirectEmitterContext};

/// Direct schema emitter: strategy-based schema generation.
#[derive(Debug)]
pub struct DirectSchemaEmitter;

impl DirectDslEmitter for DirectSchemaEmitter {
    fn id(&self) -> &'static str {
        "schema"
    }

    fn layer(&self) -> DslLayer {
        DslLayer::Schema
    }

    fn emit(&self, ctx: &DirectEmitterContext<'_>) -> Vec<DslFile> {
        let mut dsl_files = Vec::new();

        // --- Strategy 1: EXEC SQL -> DB2 table schemas ---
        let sql_schemas = super::schema_sql::extract_sql_schemas(
            &ctx.cobol_program.exec_sql_statements,
            &ctx.program_name,
        );
        dsl_files.extend(sql_schemas);

        // --- Strategy 2: FILE SECTION -> file-based schemas ---
        if let Some(ref data_div) = ctx.cobol_program.data_division {
            for fd in &data_div.file_section {
                let file_schemas = emit_file_schemas(fd, &ctx.program_name);
                dsl_files.extend(file_schemas);
            }

            // --- Strategy 3: WORKING-STORAGE -> group/flat schemas ---
            let mut flat_fields = Vec::new();
            let mut flat_constraints = Vec::new();

            for entry in &data_div.working_storage {
                if is_filler(&entry.name) || entry.level == 88 || entry.level == 66 {
                    continue;
                }

                if has_structural_children(entry) && entry.redefines.is_none() {
                    // 01-level group -> schema + child schemas
                    let group_schemas =
                        emit_group_schema(entry, &ctx.program_name);
                    dsl_files.extend(group_schemas);
                } else if entry.redefines.is_some() {
                    // Top-level REDEFINES -> variant schema
                    let variants = extract_redefines_schemas(
                        &[entry.clone()],
                        &cobol_name_to_snake(&ctx.program_name),
                        &ctx.program_name,
                    );
                    for v in variants {
                        dsl_files.push(v.dsl_file);
                    }
                } else if entry.occurs.is_some() {
                    // Top-level OCCURS (rare) -> child schema
                    let occurs = extract_occurs_schemas(
                        &[entry.clone()],
                        &cobol_name_to_snake(&ctx.program_name),
                        &[],
                        &ctx.program_name,
                    );
                    for o in occurs {
                        if !o.dsl_file.path.is_empty() {
                            dsl_files.push(o.dsl_file);
                        }
                    }
                } else {
                    // Flat 01-level elementary
                    if let Some(field) = super::schema_fields::entry_to_field_decl(entry) {
                        flat_fields.push(field);
                    }
                    let l88 = super::schema_fields::collect_level88(entry);
                    if !l88.is_empty() {
                        flat_constraints.push(SchemaConstraint::Enum(
                            Ident::new(&cobol_name_to_snake(&entry.name)),
                            l88,
                        ));
                    }
                }
            }

            // Emit flat schema for ungrouped elementary items
            if !flat_fields.is_empty() {
                let prog_snake = cobol_name_to_snake(&ctx.program_name);
                let schema_name = format!("{prog_snake}_flat");

                let key = detect_key_from_flat_fields(&flat_fields);

                let schema_file = SchemaFile {
                    comments: vec![
                        Comment("Generated by cobol2rust Nexflow emitter".to_string()),
                        Comment(format!(
                            "Source: {} WORKING-STORAGE flat items ({} fields)",
                            ctx.program_name,
                            flat_fields.len()
                        )),
                    ],
                    imports: vec![],
                    schemas: vec![SchemaDef {
                        name: Ident::new(&schema_name),
                        pattern: Some(super::schema_pattern::detect_pattern_for_ws()),
                        identity: key,
                        fields: flat_fields,
                        nested_objects: vec![],
                        constraints: flat_constraints,
                    }],
                };

                dsl_files.push(DslFile {
                    path: format!("schema/{schema_name}.schema"),
                    content: schema_file.to_text(),
                    confidence: 1.0,
                    notes: vec![
                        "Flat 01-level elementary items (no group structure)".to_string(),
                    ],
                    source_fields: vec![],
                });
            }

            // --- Strategy 4: LINKAGE SECTION -> schemas ---
            for entry in &data_div.linkage {
                if is_filler(&entry.name) || !has_structural_children(entry) {
                    continue;
                }
                let linkage_schemas = emit_linkage_schema(entry, &ctx.program_name);
                dsl_files.extend(linkage_schemas);
            }
        }

        dsl_files
    }
}

/// Emit schemas for a FILE SECTION file description.
fn emit_file_schemas(
    fd: &cobol_transpiler::ast::FileDescription,
    program: &str,
) -> Vec<DslFile> {
    let mut dsl_files = Vec::new();
    let pattern = super::schema_pattern::detect_pattern(fd);

    // Process each 01-level record under the FD
    for record in &fd.records {
        if is_filler(&record.name) || !has_structural_children(record) {
            continue;
        }

        let schema_name = cobol_name_to_snake(&record.name);
        let mut fields = Vec::new();
        let mut constraints = Vec::new();
        let mut notes = Vec::new();

        // Detect primary key from file description
        let key = detect_key_from_file(fd, &record.children);
        notes.push(format!("Primary key: {} ({})", key.identity[0].name, key.source.description()));
        if !key.alternate_keys.is_empty() {
            notes.push(format!("Alternate keys: {}", key.alternate_keys.join(", ")));
        }

        // Flatten fields (excluding OCCURS and REDEFINES)
        flatten_children(&record.children, &mut fields, &mut constraints);

        // Extract OCCURS child schemas
        let pk_fields: Vec<super::schema_occurs::TypedKeyField> = key.identity.iter().map(|f| {
        super::schema_occurs::TypedKeyField { name: f.name.as_str().to_string(), field_type: f.field_type.clone() }
    }).collect();
        let occurs = extract_occurs_schemas(&record.children, &schema_name, &pk_fields, program);
        for o in &occurs {
            if !o.parent_annotation.is_empty() {
                notes.push(o.parent_annotation.clone());
            }
        }

        // Extract REDEFINES variant schemas
        let redefines = extract_redefines_schemas(&record.children, &schema_name, program);
        for r in &redefines {
            notes.push(r.parent_annotation.clone());
        }

        let schema_file = SchemaFile {
            comments: vec![
                Comment("Generated by cobol2rust Nexflow emitter".to_string()),
                Comment(format!(
                    "File: {} ({})",
                    fd.file_name,
                    format!("{:?}", fd.organization).to_lowercase()
                )),
                Comment(format!("Source: {program}")),
            ],
            imports: vec![],
            schemas: vec![SchemaDef {
                name: Ident::new(&schema_name),
                pattern: Some(pattern),
                identity: Some(key.identity),
                fields,
                nested_objects: vec![],
                constraints,
            }],
        };

        dsl_files.push(DslFile {
            path: format!("schema/{schema_name}.schema"),
            content: schema_file.to_text(),
            confidence: 1.0,
            notes,
            source_fields: vec![record.name.clone()],
        });

        // Add OCCURS child schemas
        for o in occurs {
            if !o.dsl_file.path.is_empty() {
                dsl_files.push(o.dsl_file);
            }
        }

        // Add REDEFINES variant schemas
        for r in redefines {
            dsl_files.push(r.dsl_file);
        }
    }

    dsl_files
}

/// Emit schemas for a WORKING-STORAGE 01-level group.
fn emit_group_schema(entry: &DataEntry, program: &str) -> Vec<DslFile> {
    let schema_name = cobol_name_to_snake(&entry.name);
    let mut fields = Vec::new();
    let mut constraints = Vec::new();
    let mut notes = Vec::new();

    // Detect key from group's children
    let key = detect_key_from_entries(&entry.children);
    notes.push(format!(
        "Key: {} ({})",
        key.identity[0].name,
        key.source.description()
    ));

    // Flatten fields
    flatten_children(&entry.children, &mut fields, &mut constraints);

    // Extract OCCURS child schemas
    let pk_fields: Vec<super::schema_occurs::TypedKeyField> = key.identity.iter().map(|f| {
        super::schema_occurs::TypedKeyField { name: f.name.as_str().to_string(), field_type: f.field_type.clone() }
    }).collect();
    let occurs = extract_occurs_schemas(&entry.children, &schema_name, &pk_fields, program);
    for o in &occurs {
        if !o.parent_annotation.is_empty() {
            notes.push(o.parent_annotation.clone());
        }
    }

    // Extract REDEFINES variant schemas
    let redefines = extract_redefines_schemas(&entry.children, &schema_name, program);
    for r in &redefines {
        notes.push(r.parent_annotation.clone());
    }

    // Fix 1: If no own elementary fields, this is just a container.
    // Don't emit a parent schema -- only emit OCCURS/REDEFINES children.
    let mut all_files = Vec::new();

    if !fields.is_empty() {
        let schema_file = SchemaFile {
            comments: vec![
                Comment("Generated by cobol2rust Nexflow emitter".to_string()),
                Comment(format!(
                    "Source: {} WORKING-STORAGE group {} ({} fields)",
                    program,
                    entry.name,
                    count_elementary(&entry.children)
                )),
            ],
            imports: vec![],
            schemas: vec![SchemaDef {
                name: Ident::new(&schema_name),
                pattern: Some(super::schema_pattern::detect_pattern_for_ws()),
                identity: Some(key.identity),
                fields,
                nested_objects: vec![],
                constraints,
            }],
        };

        all_files.push(DslFile {
            path: format!("schema/{schema_name}.schema"),
            content: schema_file.to_text(),
            confidence: 1.0,
            notes,
            source_fields: vec![entry.name.clone()],
        });
    }

    for o in occurs {
        if !o.dsl_file.path.is_empty() {
            all_files.push(o.dsl_file);
        }
    }
    for r in redefines {
        all_files.push(r.dsl_file);
    }

    all_files
}

/// Emit schema for a LINKAGE SECTION group (COMMAREA/interface).
fn emit_linkage_schema(entry: &DataEntry, program: &str) -> Vec<DslFile> {
    let schema_name = cobol_name_to_snake(&entry.name);
    let mut fields = Vec::new();
    let mut constraints = Vec::new();

    flatten_children(&entry.children, &mut fields, &mut constraints);

    if fields.is_empty() {
        return vec![];
    }

    let schema_file = SchemaFile {
        comments: vec![
            Comment("Generated by cobol2rust Nexflow emitter".to_string()),
            Comment(format!(
                "LINKAGE SECTION: {} ({} fields)",
                entry.name,
                count_elementary(&entry.children)
            )),
            Comment(format!("Source: {program}")),
        ],
        imports: vec![],
        schemas: vec![SchemaDef {
            name: Ident::new(&schema_name),
            pattern: Some(super::schema_pattern::detect_pattern_for_linkage()),
            identity: None,
            fields,
            nested_objects: vec![],
            constraints,
        }],
    };

    vec![DslFile {
        path: format!("schema/{schema_name}.schema"),
        content: schema_file.to_text(),
        confidence: 1.0,
        notes: vec![format!("LINKAGE SECTION interface: {}", entry.name)],
        source_fields: vec![entry.name.clone()],
    }]
}

/// Detect key from flat FieldDecls (already converted, no DataEntry).
fn detect_key_from_flat_fields(fields: &[FieldDecl]) -> Option<Vec<FieldDecl>> {
    // Look for -id, -key, -code naming
    for f in fields {
        let name = f.name.as_str();
        if name.ends_with("_id")
            || name.ends_with("_key")
            || name.ends_with("_code")
            || name.ends_with("_number")
            || name.ends_with("_num")
            || name.ends_with("_no")
        {
            return Some(vec![FieldDecl {
                name: f.name.clone(),
                field_type: f.field_type.clone(),
                required: true,
                comment: None,
            }]);
        }
    }
    None
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
                category: if pic_raw.contains('X') { PicCategory::Alphanumeric } else { PicCategory::Numeric },
                total_digits: 10, scale: 0, signed: pic_raw.contains('S'),
                display_length: 10, edit_symbols: vec![],
            }),
            usage: Usage::Display, value: None, redefines: None,
            occurs: None, occurs_depending: None, sign: None,
            justified_right: false, blank_when_zero: false,
            children: vec![], condition_values: vec![],
            byte_offset: None, byte_length: None,
            renames_target: None, renames_thru: None, index_names: vec![],
        }
    }

    fn make_group(name: &str, level: u8, children: Vec<DataEntry>) -> DataEntry {
        DataEntry {
            level, name: name.to_string(), pic: None,
            usage: Usage::Display, value: None, redefines: None,
            occurs: None, occurs_depending: None, sign: None,
            justified_right: false, blank_when_zero: false,
            children, condition_values: vec![],
            byte_offset: None, byte_length: None,
            renames_target: None, renames_thru: None, index_names: vec![],
        }
    }

    fn make_program(entries: Vec<DataEntry>) -> CobolProgram {
        CobolProgram {
            program_id: "TESTPROG".to_string(),
            data_division: Some(DataDivision {
                working_storage: entries,
                local_storage: vec![],
                linkage: vec![],
                file_section: vec![],
            }),
            procedure_division: None,
            source_path: None,
            exec_sql_statements: vec![],
        }
    }

    #[test]
    fn group_becomes_schema_with_key() {
        let group = make_group("WS-CUSTOMER", 1, vec![
            make_entry("WS-CUST-ID", "X(10)", 5),
            make_entry("WS-CUST-NAME", "X(30)", 5),
        ]);
        let program = make_program(vec![group]);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None, assessments: &[], target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectSchemaEmitter.emit(&ctx);
        assert!(!files.is_empty());

        let schema = &files[0];
        assert_eq!(schema.path, "schema/ws_customer.schema");
        assert!(schema.content.contains("ws_cust_id"), "Content: {}", schema.content);
        assert!(schema.content.contains("identity"), "Should detect key: {}", schema.content);
    }

    #[test]
    fn occurs_produces_child_schema() {
        let mut occurs = make_group("WS-LINE-ITEM", 5, vec![
            make_entry("WS-ITEM-CODE", "X(8)", 10),
            make_entry("WS-ITEM-QTY", "9(5)", 10),
        ]);
        occurs.occurs = Some(50);

        let group = make_group("WS-ORDER", 1, vec![
            make_entry("WS-ORDER-ID", "X(10)", 5),
            occurs,
        ]);
        let program = make_program(vec![group]);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None, assessments: &[], target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectSchemaEmitter.emit(&ctx);
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.contains(&"schema/ws_order.schema"), "Paths: {paths:?}");
        assert!(paths.contains(&"schema/ws_line_item.schema"), "Paths: {paths:?}");

        let child = files.iter().find(|f| f.path.contains("ws_line_item")).unwrap();
        assert!(child.content.contains("OCCURS 50"), "Content: {}", child.content);
        assert!(child.content.contains("ws_item_code"), "Content: {}", child.content);
    }

    #[test]
    fn flat_items_go_to_flat_schema() {
        let entries = vec![
            make_entry("WS-COUNTER", "9(5)", 1),
            make_entry("WS-FLAG", "X(1)", 1),
        ];
        let program = make_program(entries);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None, assessments: &[], target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectSchemaEmitter.emit(&ctx);
        assert_eq!(files.len(), 1);
        assert!(files[0].path.contains("flat"));
    }

    #[test]
    fn empty_data_div() {
        let program = CobolProgram {
            program_id: "EMPTY".to_string(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: vec![],
        };
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "EMPTY".to_string(),
            hints: None, assessments: &[], target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };
        assert!(DirectSchemaEmitter.emit(&ctx).is_empty());
    }

    #[test]
    fn sql_program_produces_db2_schema() {
        let program = CobolProgram {
            program_id: "SQLPROG".to_string(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: vec![ExecSqlStatement {
                sql_type: SqlStatementType::SelectInto,
                raw_sql: "SELECT ACCT_NUM, BALANCE FROM ACCOUNT WHERE ACCT_NUM = :WS-ACCT".to_string(),
                input_vars: vec![HostVarRef { field_name: "WS-ACCT".to_string(), indicator: None }],
                output_vars: vec![
                    HostVarRef { field_name: "WS-ACCT-NUM".to_string(), indicator: None },
                    HostVarRef { field_name: "WS-BALANCE".to_string(), indicator: None },
                ],
                cursor_name: None,
                prepared_name: None,
            }],
        };
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "SQLPROG".to_string(),
            hints: None, assessments: &[], target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectSchemaEmitter.emit(&ctx);
        assert!(!files.is_empty());
        assert!(files[0].path.contains("db2_account"), "Path: {}", files[0].path);
        assert!(files[0].content.contains("DB2 table"), "Content: {}", files[0].content);
    }
}
