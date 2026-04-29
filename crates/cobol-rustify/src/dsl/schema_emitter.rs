//! Schema emitter: generates `.schema` files from WorkingStorage fields.
//!
//! Decomposes large WorkingStorage structs into entity-scoped schemas
//! using Tier 3 ws-decomposition groupings. Each schema conforms to
//! the Nexflow SchemaDSL.g4 grammar.

use std::collections::BTreeMap;

use super::cobol_attrs::{extract_annotated_fields, extract_program_name, AnnotatedField};
use super::dsl_ast::*;
use super::type_mapping::{pic_to_nexflow_type, NexflowType};
use super::{DslEmitter, DslFile, DslLayer, EmitterContext};

/// Emitter that produces `.schema` files from WorkingStorage.
#[derive(Debug)]
pub struct SchemaEmitter;

impl DslEmitter for SchemaEmitter {
    fn id(&self) -> &'static str {
        "schema"
    }

    fn layer(&self) -> DslLayer {
        DslLayer::Schema
    }

    fn emit(&self, ctx: &EmitterContext<'_>) -> Vec<DslFile> {
        let fields = extract_annotated_fields(ctx.syn_file);
        let program = extract_program_name(ctx.syn_file)
            .unwrap_or_else(|| ctx.program_name.clone());

        // Filter to fields that have COBOL attributes
        let annotated: Vec<&AnnotatedField> = fields
            .iter()
            .filter(|f| f.cobol_attr.is_some())
            .collect();

        if annotated.is_empty() {
            return vec![];
        }

        // Decompose into entity groups (prefix-based)
        let min_group = ctx.target
            .map(|t| t.data_model.min_group_size)
            .unwrap_or(2);
        let groups = decompose_entities(&annotated, &program, min_group);

        let mut dsl_files = Vec::new();
        for group in groups {
            let (content, notes, confidence) = generate_schema(&group, &program);
            let source_fields: Vec<String> = group
                .fields
                .iter()
                .map(|f| f.cobol_name.clone())
                .collect();

            dsl_files.push(DslFile {
                path: format!("schema/{}.schema", group.name),
                content,
                confidence,
                notes,
                source_fields,
            });
        }

        // Co-access grouping from hints (fields used by the same paragraphs)
        if let Some(hints) = ctx.hints {
            let co_access = extract_co_access_schemas(&annotated, hints, &program);
            dsl_files.extend(co_access);
        }

        dsl_files
    }
}

/// A group of related fields forming a schema entity.
#[derive(Debug)]
pub struct EntityGroup {
    /// Schema name (snake_case, SchemaDSL-valid identifier)
    pub name: String,
    /// Fields in this entity
    pub fields: Vec<SchemaField>,
    /// Why these fields were grouped
    pub reason: String,
}

/// A field within a schema entity.
#[derive(Debug)]
pub struct SchemaField {
    /// Original COBOL name (for traceability comments)
    pub cobol_name: String,
    /// Nexflow field name (snake_case, stripped of common prefix)
    pub nexflow_name: String,
    /// Nexflow type
    pub nexflow_type: NexflowType,
    /// Raw PIC clause for comment
    pub pic: Option<String>,
    /// Whether this field has REDEFINES
    pub redefines: Option<String>,
    /// OCCURS count
    pub occurs: Option<usize>,
    /// Level-88 conditions as "NAME:VAL,NAME:VAL"
    pub level88: Option<String>,
    /// COBOL level number (used for future group-level handling)
    #[allow(dead_code)]
    pub level: u8,
    /// Whether this looks like a key field
    pub is_key_candidate: bool,
}

/// Decompose annotated fields into entity groups using prefix-based grouping.
fn decompose_entities(
    fields: &[&AnnotatedField],
    program: &str,
    min_group_size: usize,
) -> Vec<EntityGroup> {
    // Group by common prefix (first two underscore segments)
    let mut prefix_groups: BTreeMap<String, Vec<&AnnotatedField>> = BTreeMap::new();

    for field in fields {
        let prefix = extract_field_prefix(&field.rust_name);
        prefix_groups.entry(prefix).or_default().push(field);
    }

    let mut entities = Vec::new();

    for (prefix, group_fields) in &prefix_groups {
        if group_fields.len() < min_group_size {
            // Below threshold -- goes to misc group
            continue;
        }

        let schema_name = prefix_to_schema_name(prefix);
        let fields: Vec<SchemaField> = group_fields
            .iter()
            .map(|f| annotated_to_schema_field(f))
            .collect();

        entities.push(EntityGroup {
            name: schema_name,
            fields,
            reason: format!("common prefix '{prefix}' ({} fields)", group_fields.len()),
        });
    }

    // Collect ungrouped fields (below-threshold prefixes) into misc
    let grouped_fields: std::collections::HashSet<&str> = prefix_groups
        .iter()
        .filter(|(_, v)| v.len() >= min_group_size)
        .flat_map(|(_, v)| v.iter().map(|f| f.rust_name.as_str()))
        .collect();

    let misc_fields: Vec<&AnnotatedField> = fields
        .iter()
        .filter(|f| !grouped_fields.contains(f.rust_name.as_str()))
        .copied()
        .collect();

    if !misc_fields.is_empty() {
        let misc_name = format!("{}_misc", cobol_name_to_snake(program));
        let fields: Vec<SchemaField> = misc_fields
            .iter()
            .map(|f| annotated_to_schema_field(f))
            .collect();
        entities.push(EntityGroup {
            name: misc_name,
            fields,
            reason: format!("ungrouped fields ({} fields)", misc_fields.len()),
        });
    }

    entities
}

/// Convert an AnnotatedField to a SchemaField.
fn annotated_to_schema_field(field: &AnnotatedField) -> SchemaField {
    let attr = field.cobol_attr.as_ref().unwrap();
    let cobol_name = rust_name_to_cobol(&field.rust_name);

    let nexflow_name = sanitize_identifier(&field.rust_name);

    let nexflow_type = if let Some(pic) = &attr.pic {
        pic_to_nexflow_type(pic, attr.usage.as_deref(), attr.signed, &field.rust_name)
    } else {
        // No PIC -- group field, use string as fallback
        NexflowType::String(None)
    };

    let is_key_candidate = is_likely_key(&field.rust_name, &cobol_name);

    SchemaField {
        cobol_name,
        nexflow_name,
        nexflow_type,
        pic: attr.pic.clone(),
        redefines: attr.redefines.clone(),
        occurs: attr.occurs,
        level88: attr.level88.clone(),
        level: attr.level,
        is_key_candidate,
    }
}

/// Generate SchemaDSL text for an entity group using typed AST.
pub fn generate_schema(group: &EntityGroup, program: &str) -> (String, Vec<String>, f64) {
    let mut notes = Vec::new();

    // Pattern heuristic
    let pattern = match guess_pattern(group) {
        "event_log" => MutationPattern::EventLog,
        "state_machine" => MutationPattern::StateMachine,
        "reference_data" => MutationPattern::ReferenceData,
        _ => MutationPattern::MasterData,
    };

    // Identity block -- find the best key candidate
    let identity = if let Some(key_field) = group.fields.iter().find(|f| f.is_key_candidate) {
        Some(vec![FieldDecl {
            name: Ident::new(&key_field.nexflow_name),
            field_type: key_field.nexflow_type.to_field_type(),
            required: true,
            comment: None,
        }])
    } else {
        notes.push("No identity field detected -- manual review needed".to_string());
        None
    };

    // Fields block
    let mut fields = Vec::new();
    for field in &group.fields {
        // Skip REDEFINES fields (emit as comment only)
        if let Some(target) = &field.redefines {
            // Add a comment-only field to note the REDEFINES
            fields.push(FieldDecl {
                name: Ident::new(&field.nexflow_name),
                field_type: FieldType::String(None),
                required: false,
                comment: Some(format!("COBOL REDEFINES: {} redefines {}", field.cobol_name, target)),
            });
            notes.push(format!(
                "REDEFINES detected: {} redefines {} -- manual review needed for union type",
                field.cobol_name, target
            ));
            continue;
        }

        let base_type = field.nexflow_type.to_field_type();
        let field_type = if field.occurs.is_some() {
            FieldType::List(Box::new(base_type))
        } else {
            base_type
        };

        let comment = field.pic.as_ref().map(|pic| {
            format!("COBOL: {} PIC {}", field.cobol_name, pic)
        });

        fields.push(FieldDecl {
            name: Ident::new(&field.nexflow_name),
            field_type,
            required: true,
            comment,
        });
    }

    // Constraints block for level-88 fields
    let mut constraints = Vec::new();
    for field in &group.fields {
        if let Some(l88) = &field.level88 {
            let values = parse_level88_values(l88);
            if !values.is_empty() {
                constraints.push(SchemaConstraint::Enum(
                    Ident::new(&field.nexflow_name),
                    values,
                ));
            }
        }
    }

    // Detect streaming semantics for event_log patterns
    let streaming = if matches!(pattern, MutationPattern::EventLog) {
        detect_event_time_field(&group.fields).map(|field_name| {
            notes.push(format!(
                "Streaming block: detected event_time field '{field_name}'"
            ));
            StreamingBlock {
                event_time: Ident::new(&field_name),
                watermark: Some("5m".to_string()),
            }
        })
    } else {
        None
    };

    let schema_file = SchemaFile {
        comments: vec![
            Comment(format!("Generated by cobol2rust Nexflow emitter")),
            Comment(format!("Source: {program} WorkingStorage ({})", group.reason)),
        ],
        imports: vec![],
        schemas: vec![SchemaDef {
            name: Ident::new(&group.name),
            pattern: Some(pattern),
            identity,
            fields,
            nested_objects: vec![],
            constraints,
            streaming,
        }],
    };

    // Confidence is 1.0 -- output is grammar-valid by construction
    (schema_file.to_text(), notes, 1.0)
}

/// Guess mutation pattern based on field names.
/// Find the best event_time candidate field in a group.
/// Prefers fields named *timestamp*, *_ts, then *date*, *_dt.
fn detect_event_time_field(fields: &[SchemaField]) -> Option<String> {
    // Priority 1: explicit timestamp
    for f in fields {
        let lower = f.nexflow_name.to_lowercase();
        if lower.contains("timestamp") || lower.ends_with("_ts") {
            return Some(f.nexflow_name.clone());
        }
    }
    // Priority 2: date field
    for f in fields {
        let lower = f.nexflow_name.to_lowercase();
        if lower.contains("date") || lower.ends_with("_dt") {
            return Some(f.nexflow_name.clone());
        }
    }
    None
}

fn guess_pattern(group: &EntityGroup) -> &'static str {
    let has_timestamp = group.fields.iter().any(|f| {
        let lower = f.nexflow_name.to_lowercase();
        lower.contains("timestamp")
            || lower.contains("_ts")
            || lower.contains("_dt")
            || lower.contains("date")
    });

    let has_status = group.fields.iter().any(|f| {
        let lower = f.nexflow_name.to_lowercase();
        lower.contains("status") || lower.contains("state") || lower.contains("flag")
    });

    if has_timestamp && has_status {
        "event_log"
    } else if has_status {
        "state_machine"
    } else {
        "master_data"
    }
}

/// Heuristic: is this field likely a key/identifier?
pub fn is_likely_key(rust_name: &str, cobol_name: &str) -> bool {
    let lower = rust_name.to_lowercase();
    let cobol_lower = cobol_name.to_lowercase();
    lower.ends_with("_id")
        || lower.ends_with("_key")
        || lower.ends_with("_code")
        || lower.ends_with("_number")
        || lower.ends_with("_num")
        || lower.ends_with("_no")
        || cobol_lower.ends_with("-ID")
        || cobol_lower.ends_with("-KEY")
        || cobol_lower.ends_with("-NUMBER")
}

/// Extract the first two underscore segments as prefix.
/// "ws_acct_number" -> "ws_acct", "card_type" -> "card", "x" -> "x"
fn extract_field_prefix(name: &str) -> String {
    let parts: Vec<&str> = name.split('_').collect();
    if parts.len() >= 3 {
        format!("{}_{}", parts[0], parts[1])
    } else if parts.len() == 2 {
        parts[0].to_string()
    } else {
        name.to_string()
    }
}

/// Convert prefix to a clean schema name.
/// "ws_acct" -> "acct_info", "ws" -> "ws_data"
fn prefix_to_schema_name(prefix: &str) -> String {
    let stripped = prefix
        .strip_prefix("ws_")
        .or_else(|| prefix.strip_prefix("ws"))
        .unwrap_or(prefix);

    if stripped.is_empty() {
        "working_storage".to_string()
    } else {
        sanitize_identifier(stripped)
    }
}


/// Ensure a name is a valid SchemaDSL IDENTIFIER: [a-z_][a-z0-9_]*
pub fn sanitize_identifier(name: &str) -> String {
    let lower = name.to_lowercase();
    let mut result = String::new();
    for (i, ch) in lower.chars().enumerate() {
        if i == 0 {
            if ch.is_ascii_lowercase() || ch == '_' {
                result.push(ch);
            } else {
                result.push('_');
                if ch.is_ascii_alphanumeric() {
                    result.push(ch);
                }
            }
        } else if ch.is_ascii_alphanumeric() || ch == '_' {
            result.push(ch);
        } else {
            result.push('_');
        }
    }
    if result.is_empty() {
        "field".to_string()
    } else {
        result
    }
}

/// Convert a Rust field name back to approximate COBOL name.
/// "ws_acct_number" -> "WS-ACCT-NUMBER"
fn rust_name_to_cobol(name: &str) -> String {
    name.to_uppercase().replace('_', "-")
}

/// Convert a COBOL program name to snake_case.
/// "CARDPROC" -> "cardproc", "CARD-PROC" -> "card_proc"
pub fn cobol_name_to_snake(name: &str) -> String {
    name.to_lowercase().replace('-', "_")
}

/// Parse level-88 values from "ACTIVE:A,CLOSED:C" format.
/// Returns just the values: ["A", "C"]
pub fn parse_level88_values(l88: &str) -> Vec<String> {
    l88.split(',')
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.split(':').collect();
            if parts.len() == 2 {
                Some(parts[1].trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Extract co-access schemas from hints: fields accessed by the same paragraphs
/// are semantically related. Produces schemas for groups with 3+ fields.
fn extract_co_access_schemas(
    annotated: &[&AnnotatedField],
    hints: &crate::hints::FileHints,
    program: &str,
) -> Vec<DslFile> {
    use std::collections::HashMap;

    // Group fields by their paragraph_scope signature
    let mut scope_groups: HashMap<Vec<String>, Vec<&AnnotatedField>> = HashMap::new();

    for field in annotated {
        if let Some(hint) = hints.fields.get(&field.rust_name) {
            if !hint.paragraph_scope.is_empty() {
                let mut scope = hint.paragraph_scope.clone();
                scope.sort();
                scope_groups.entry(scope).or_default().push(field);
            }
        }
    }

    let mut dsl_files = Vec::new();
    let mut group_idx = 0;

    for (scope, fields) in &scope_groups {
        if fields.len() < 3 {
            continue; // Only group 3+ fields
        }

        group_idx += 1;
        let group_name = format!(
            "{}_co_access_{}",
            cobol_name_to_snake(program),
            group_idx,
        );

        let schema_fields: Vec<SchemaField> = fields
            .iter()
            .map(|f| annotated_to_schema_field(f))
            .collect();

        let group = EntityGroup {
            name: group_name.clone(),
            fields: schema_fields,
            reason: format!("co-access: fields used by paragraphs {:?}", scope),
        };

        let (content, mut notes, confidence) = generate_schema(&group, program);
        notes.push(format!(
            "co-access group: {} fields used by same paragraph scope",
            fields.len()
        ));

        let source_fields: Vec<String> = fields
            .iter()
            .map(|f| f.rust_name.clone())
            .collect();

        dsl_files.push(DslFile {
            path: format!("schema/{}.schema", group_name),
            content,
            confidence: confidence * 0.8, // lower confidence for heuristic grouping
            notes,
            source_fields,
        });
    }

    dsl_files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_prefix() {
        assert_eq!(extract_field_prefix("ws_acct_number"), "ws_acct");
        assert_eq!(extract_field_prefix("ws_acct_type"), "ws_acct");
        assert_eq!(extract_field_prefix("card_type"), "card");
        assert_eq!(extract_field_prefix("simple"), "simple");
    }

    #[test]
    fn prefix_to_name() {
        assert_eq!(prefix_to_schema_name("ws_acct"), "acct");
        assert_eq!(prefix_to_schema_name("ws"), "working_storage");
        assert_eq!(prefix_to_schema_name("card"), "card");
    }

    #[test]
    fn sanitize_ident() {
        assert_eq!(sanitize_identifier("WS-ACCT"), "ws_acct");
        assert_eq!(sanitize_identifier("_ok"), "_ok");
        assert_eq!(sanitize_identifier("123bad"), "_123bad");
        assert_eq!(sanitize_identifier(""), "field");
    }

    #[test]
    fn rust_to_cobol() {
        assert_eq!(rust_name_to_cobol("ws_acct_number"), "WS-ACCT-NUMBER");
    }

    #[test]
    fn level88_parsing() {
        let vals = parse_level88_values("ACTIVE:A,CLOSED:C");
        assert_eq!(vals, vec!["A", "C"]);
    }

    #[test]
    fn level88_parsing_single() {
        let vals = parse_level88_values("YES:Y");
        assert_eq!(vals, vec!["Y"]);
    }

    #[test]
    fn end_to_end_schema_emission() {
        let code = r#"
            #![allow(unused_imports, unused_variables, non_snake_case, unused_attributes)]
            use cobol_runtime::prelude::*;

            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 5, pic = "X(10)")]
                pub ws_acct_number: PicX,
                #[cobol(level = 5, pic = "X(1)", level88 = "SAVINGS:S,CHECKING:C")]
                pub ws_acct_type: PicX,
                #[cobol(level = 5, pic = "S9(9)V99", comp3, offset = 16, len = 6, signed)]
                pub ws_acct_balance: PackedDecimal,
                #[cobol(level = 5, pic = "9(8)")]
                pub ws_acct_open_date: PicX,
                #[cobol(level = 5, pic = "X(20)")]
                pub ws_customer_name: PicX,
            }
        "#;

        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = SchemaEmitter;
        let ctx = EmitterContext {
            program_name: "TESTPROG".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);

        // Should produce at least one schema (ws_acct group has 4 fields)
        assert!(!files.is_empty(), "Expected at least one schema file");

        // Find the acct schema
        let acct_schema = files.iter().find(|f| f.path.contains("acct"));
        assert!(acct_schema.is_some(), "Expected an acct schema, got: {:?}", files.iter().map(|f| &f.path).collect::<Vec<_>>());

        let acct = acct_schema.unwrap();
        assert!(acct.content.contains("schema acct"), "Should have schema name: {}", acct.content);
        assert!(acct.content.contains("pattern"), "Should have pattern");
        assert!(acct.content.contains("decimal(11, 2)"), "Should have decimal type for balance: {}", acct.content);
        assert!(acct.content.contains("string(10)"), "Should have string type for acct number: {}", acct.content);
        assert!(acct.content.contains("date"), "Should have date type for open_date: {}", acct.content);
        assert!(acct.content.contains("enum("), "Should have enum constraint for acct_type: {}", acct.content);
        assert!(acct.content.contains("end"), "Should be terminated with end");

        // Check grammar validity basics
        let lines: Vec<&str> = acct.content.lines().collect();
        let first_schema_line = lines.iter().find(|l| l.trim().starts_with("schema "));
        assert!(first_schema_line.is_some());
        let last_line = lines.last().unwrap().trim();
        assert_eq!(last_line, "end");
    }

    #[test]
    fn single_field_goes_to_misc() {
        let code = r#"
            #[cobol(program = "SMALL")]
            pub struct WorkingStorage {
                #[cobol(level = 5, pic = "X(10)")]
                pub ws_lonely_field: PicX,
            }
        "#;

        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = SchemaEmitter;
        let ctx = EmitterContext {
            program_name: "SMALL".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert_eq!(files.len(), 1);
        assert!(files[0].path.contains("misc"));
    }

    #[test]
    fn redefines_generates_comment() {
        let code = r#"
            #[cobol(program = "REDEF")]
            pub struct WorkingStorage {
                #[cobol(level = 5, pic = "X(100)")]
                pub ws_data_original: PicX,
                #[cobol(level = 5, redefines = "WS-DATA-ORIGINAL", len = 100)]
                pub ws_data_alt: PicX,
                #[cobol(level = 5, pic = "X(10)")]
                pub ws_data_name: PicX,
            }
        "#;

        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = SchemaEmitter;
        let ctx = EmitterContext {
            program_name: "REDEF".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        let schema = &files[0];
        assert!(
            schema.content.contains("REDEFINES"),
            "Should have REDEFINES comment: {}",
            schema.content
        );
        assert!(
            schema.notes.iter().any(|n| n.contains("REDEFINES")),
            "Should have REDEFINES review note"
        );
    }

    #[test]
    fn occurs_generates_list() {
        let code = r#"
            #[cobol(program = "OCCUR")]
            pub struct WorkingStorage {
                #[cobol(level = 5, pic = "X(15)", occurs = 5)]
                pub ws_phone_numbers: PicX,
                #[cobol(level = 5, pic = "X(10)")]
                pub ws_phone_type: PicX,
            }
        "#;

        let syn_file = syn::parse_str::<syn::File>(code).unwrap();
        let emitter = SchemaEmitter;
        let ctx = EmitterContext {
            program_name: "OCCUR".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        let schema = &files[0];
        assert!(
            schema.content.contains("list(string(15))"),
            "Should have list type for OCCURS: {}",
            schema.content
        );
    }
}
