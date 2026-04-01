// NexCore -- Nexflow Codegen: Avro Schema (.avsc) Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Apache Avro `.avsc` schema files from SchemaDSL ASTs.
//!
//! Each `SchemaDefinition` produces one `.avsc` file (JSON).
//! The Maven Avro plugin then generates Java `SpecificRecord` classes from these.

use std::fmt::Write;

use nexflow_parser::ast::schema::{FieldDecl, NestedObject, SchemaDefinition};

use super::naming::to_pascal_case;
use super::types::avro_type;

/// Default Avro namespace for generated schemas.
const DEFAULT_NAMESPACE: &str = "com.nexflow.schemas";

/// Generate an `.avsc` (Avro schema JSON) string from a `SchemaDefinition`.
pub fn generate_avsc(schema: &SchemaDefinition) -> String {
    let class_name = to_pascal_case(&schema.name);
    let mut out = String::with_capacity(2048);

    writeln!(out, "{{").unwrap();
    writeln!(out, "  \"type\": \"record\",").unwrap();
    writeln!(out, "  \"name\": \"{class_name}\",").unwrap();
    writeln!(out, "  \"namespace\": \"{DEFAULT_NAMESPACE}\",").unwrap();

    // Doc string from pattern
    if !schema.patterns.is_empty() {
        let pattern_str = format!("{:?}", schema.patterns[0]);
        writeln!(out, "  \"doc\": \"{pattern_str} schema\",").unwrap();
    }

    writeln!(out, "  \"fields\": [").unwrap();

    // Collect all fields: identity first, then regular fields (dedup by name)
    let all_fields = collect_all_fields(schema);

    for (i, field) in all_fields.iter().enumerate() {
        let comma = if i + 1 < all_fields.len() { "," } else { "" };
        let field_json = generate_avro_field(field);
        writeln!(out, "    {field_json}{comma}").unwrap();
    }

    // Nested objects become nested Avro records
    for nested in &schema.nested_objects {
        if !all_fields.is_empty() || schema.nested_objects.first() != Some(nested) {
            // Add comma before if there were prior fields
            // Actually, let's handle this by checking position
        }
        let nested_json = generate_nested_avro(nested);
        let is_last = schema.nested_objects.last() == Some(nested);
        let comma = if is_last { "" } else { "," };
        writeln!(out, "    {nested_json}{comma}").unwrap();
    }

    writeln!(out, "  ]").unwrap();
    writeln!(out, "}}").unwrap();

    out
}

/// Collect identity + fields, deduplicating by name (identity takes precedence).
fn collect_all_fields(schema: &SchemaDefinition) -> Vec<&FieldDecl> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();

    // Identity fields first
    for f in &schema.identity {
        if seen.insert(&f.name) {
            result.push(f);
        }
    }

    // Then regular fields
    for f in &schema.fields {
        if seen.insert(&f.name) {
            result.push(f);
        }
    }

    result
}

/// Generate a single Avro field JSON object.
fn generate_avro_field(field: &FieldDecl) -> String {
    let avro_t = avro_type(&field.field_type);
    let is_optional = field.optional || !field.required;

    let mut out = String::with_capacity(128);
    out.push('{');

    // Field name
    write!(out, "\"name\": \"{}\"", field.name).unwrap();

    // Type (wrap in union with null for optional)
    if is_optional {
        write!(out, ", \"type\": [\"null\", {avro_t}]").unwrap();
        out.push_str(", \"default\": null");
    } else {
        write!(out, ", \"type\": {avro_t}").unwrap();

        // Add default value if specified
        if let Some(default) = &field.default {
            let default_json = format_avro_default(default, &field.field_type);
            write!(out, ", \"default\": {default_json}").unwrap();
        }
    }

    // Add doc annotation for PII fields
    if let Some(pii) = &field.pii {
        let profile = pii
            .profile
            .as_deref()
            .unwrap_or("default");
        write!(out, ", \"doc\": \"PII: {profile} profile\"").unwrap();
    }

    out.push('}');
    out
}

/// Generate an Avro field for a nested object definition.
fn generate_nested_avro(nested: &NestedObject) -> String {
    let record_name = to_pascal_case(&nested.name);
    let mut out = String::with_capacity(512);

    // The field itself
    write!(out, "{{\"name\": \"{}\", \"type\": ", nested.name).unwrap();

    // Build the nested record type
    let mut record = String::with_capacity(256);
    write!(
        record,
        "{{\"type\": \"record\", \"name\": \"{record_name}\", \"fields\": ["
    )
    .unwrap();
    for (i, f) in nested.fields.iter().enumerate() {
        if i > 0 {
            record.push_str(", ");
        }
        record.push_str(&generate_avro_field(f));
    }
    record.push_str("]}");

    if nested.is_list {
        // Wrap in array
        write!(out, "{{\"type\": \"array\", \"items\": {record}}}").unwrap();
    } else {
        out.push_str(&record);
    }

    out.push('}');
    out
}

/// Format a default value for Avro JSON.
fn format_avro_default(default: &str, ft: &nexflow_parser::ast::schema::FieldType) -> String {
    match ft {
        nexflow_parser::ast::schema::FieldType::Base { name, .. } => match name.as_str() {
            "string" | "text" | "char" | "uuid" => {
                // Strip surrounding quotes if present
                let cleaned = default.trim_matches('"');
                format!("\"{cleaned}\"")
            }
            "integer" => default.to_string(),
            "boolean" => default.to_lowercase(),
            "decimal" | "money" | "float" => default.to_string(),
            _ => format!("\"{default}\""),
        },
        _ => format!("\"{default}\""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::schema::*;

    fn make_field(name: &str, type_name: &str, required: bool) -> FieldDecl {
        FieldDecl {
            name: name.to_string(),
            field_type: FieldType::Base {
                name: type_name.to_string(),
                params: Vec::new(),
            },
            required,
            optional: !required,
            unique: false,
            cannot_change: false,
            encrypted: false,
            pii: None,
            default: None,
            deprecated: None,
            removal: None,
        }
    }

    fn make_schema(name: &str, fields: Vec<FieldDecl>) -> SchemaDefinition {
        SchemaDefinition {
            imports: Vec::new(),
            name: name.to_string(),
            patterns: vec![MutationPattern::MasterData],
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields,
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: Vec::new(),
            immutable: None,
            state_machine: None,
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        }
    }

    #[test]
    fn test_basic_avsc_generation() {
        let schema = make_schema(
            "account_detail",
            vec![
                make_field("account_id", "uuid", true),
                make_field("holder_name", "string", true),
                make_field("balance", "decimal", true),
                make_field("email", "string", false),
            ],
        );

        let avsc = generate_avsc(&schema);

        assert!(avsc.contains("\"name\": \"AccountDetail\""));
        assert!(avsc.contains("\"namespace\": \"com.nexflow.schemas\""));
        assert!(avsc.contains("\"name\": \"account_id\""));
        assert!(avsc.contains("\"name\": \"holder_name\""));
        assert!(avsc.contains("\"name\": \"balance\""));
        assert!(avsc.contains("\"name\": \"email\""));

        // Optional field should have null union
        assert!(avsc.contains("[\"null\""));

        // Required uuid field should have logical type
        assert!(avsc.contains("\"logicalType\": \"uuid\""));
    }

    #[test]
    fn test_identity_fields_first() {
        let mut schema = make_schema(
            "account",
            vec![
                make_field("holder_name", "string", true),
                make_field("account_id", "uuid", true),
            ],
        );
        schema.identity = vec![make_field("account_id", "uuid", true)];

        let avsc = generate_avsc(&schema);

        // account_id should appear before holder_name (identity first)
        let id_pos = avsc.find("\"name\": \"account_id\"").unwrap();
        let name_pos = avsc.find("\"name\": \"holder_name\"").unwrap();
        assert!(id_pos < name_pos);
    }

    #[test]
    fn test_identity_dedup() {
        let mut schema = make_schema(
            "account",
            vec![
                make_field("account_id", "uuid", true),
                make_field("name", "string", true),
            ],
        );
        // Same field in identity and fields -- should not duplicate
        schema.identity = vec![make_field("account_id", "uuid", true)];

        let avsc = generate_avsc(&schema);
        let count = avsc.matches("\"name\": \"account_id\"").count();
        assert_eq!(count, 1, "account_id should appear exactly once");
    }

    #[test]
    fn test_pii_doc_annotation() {
        let mut field = make_field("ssn", "string", true);
        field.pii = Some(PiiModifier {
            profile: Some("ssn".to_string()),
        });

        let schema = make_schema("person", vec![field]);
        let avsc = generate_avsc(&schema);

        assert!(avsc.contains("\"doc\": \"PII: ssn profile\""));
    }

    #[test]
    fn test_default_value() {
        let mut field = make_field("currency", "char", true);
        field.default = Some("\"USD\"".to_string());

        let schema = make_schema("account", vec![field]);
        let avsc = generate_avsc(&schema);

        assert!(avsc.contains("\"default\": \"USD\""));
    }

    #[test]
    fn test_decimal_with_params() {
        let field = FieldDecl {
            name: "balance".to_string(),
            field_type: FieldType::Base {
                name: "decimal".to_string(),
                params: vec![14, 2],
            },
            required: true,
            optional: false,
            unique: false,
            cannot_change: false,
            encrypted: false,
            pii: None,
            default: None,
            deprecated: None,
            removal: None,
        };

        let schema = make_schema("account", vec![field]);
        let avsc = generate_avsc(&schema);

        assert!(avsc.contains("\"precision\": 14"));
        assert!(avsc.contains("\"scale\": 2"));
    }

    #[test]
    fn test_collection_field() {
        let field = FieldDecl {
            name: "tags".to_string(),
            field_type: FieldType::Collection {
                kind: CollectionKind::List,
                element_types: vec![FieldType::Base {
                    name: "string".to_string(),
                    params: Vec::new(),
                }],
            },
            required: false,
            optional: true,
            unique: false,
            cannot_change: false,
            encrypted: false,
            pii: None,
            default: None,
            deprecated: None,
            removal: None,
        };

        let schema = make_schema("item", vec![field]);
        let avsc = generate_avsc(&schema);

        assert!(avsc.contains("\"type\": \"array\""));
        assert!(avsc.contains("\"items\": \"string\""));
    }

    #[test]
    fn test_nested_object() {
        let mut schema = make_schema("order", vec![make_field("order_id", "uuid", true)]);
        schema.nested_objects = vec![NestedObject {
            name: "shipping_address".to_string(),
            is_list: false,
            fields: vec![
                make_field("street", "string", true),
                make_field("city", "string", true),
            ],
            nested: Vec::new(),
        }];

        let avsc = generate_avsc(&schema);

        assert!(avsc.contains("\"name\": \"ShippingAddress\""));
        assert!(avsc.contains("\"name\": \"street\""));
        assert!(avsc.contains("\"name\": \"city\""));
    }

    #[test]
    fn test_nested_list_object() {
        let mut schema = make_schema("order", vec![make_field("order_id", "uuid", true)]);
        schema.nested_objects = vec![NestedObject {
            name: "line_items".to_string(),
            is_list: true,
            fields: vec![
                make_field("product_id", "uuid", true),
                make_field("quantity", "integer", true),
            ],
            nested: Vec::new(),
        }];

        let avsc = generate_avsc(&schema);

        // Should be an array of records
        assert!(avsc.contains("\"type\": \"array\""));
        assert!(avsc.contains("\"name\": \"LineItems\""));
    }
}
