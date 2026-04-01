// NexCore -- Nexflow Codegen: Schema Struct Generation
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Rust structs from `SchemaDefinition` with serde + utoipa derives.

use std::fmt::Write;

use nexflow_parser::ast::schema::{ConstraintDecl, FieldDecl, SchemaDefinition};

use crate::naming::snake_to_pascal;
use crate::types::rust_type;

/// Generate a `models.rs` file containing Rust structs for the given schemas.
pub fn generate_schemas(schemas: &[&SchemaDefinition]) -> String {
    let mut out = String::with_capacity(4096);

    writeln!(out, "//! Generated data models from Nexflow SchemaDSL.").unwrap();
    writeln!(out, "//! DO NOT EDIT -- regenerate from .schema files.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use serde::{{Deserialize, Serialize}};").unwrap();
    writeln!(out).unwrap();

    for schema in schemas {
        generate_one_schema(&mut out, schema);
    }

    out
}

fn generate_one_schema(out: &mut String, schema: &SchemaDefinition) {
    let struct_name = snake_to_pascal(&schema.name);

    // Generate constraint enums first
    for constraint in &schema.constraints {
        if let ConstraintDecl::Enum { field, values } = constraint {
            generate_enum(out, &struct_name, field, values);
        }
    }

    // Struct doc comment
    if !schema.patterns.is_empty() {
        let patterns: Vec<String> = schema
            .patterns
            .iter()
            .map(|p| format!("{p:?}"))
            .collect();
        writeln!(out, "/// Schema: {} ({})", schema.name, patterns.join(", ")).unwrap();
    }

    writeln!(
        out,
        "#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]"
    )
    .unwrap();
    writeln!(out, "pub struct {struct_name} {{").unwrap();

    for field in &schema.fields {
        generate_field(out, field, &struct_name, &schema.constraints);
    }

    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();
}

fn generate_field(
    out: &mut String,
    field: &FieldDecl,
    parent_struct: &str,
    constraints: &[ConstraintDecl],
) {
    let is_optional = field.optional || !field.required;

    // Check if this field has an enum constraint
    let enum_constraint = constraints.iter().find(|c| {
        matches!(c, ConstraintDecl::Enum { field: f, .. } if f == &field.name)
    });

    let type_str = if enum_constraint.is_some() {
        format!("{parent_struct}{}", snake_to_pascal(&field.name))
    } else {
        rust_type(&field.field_type)
    };

    let full_type = if is_optional {
        format!("Option<{type_str}>")
    } else {
        type_str
    };

    // Serde default annotation
    if let Some(default_val) = &field.default {
        writeln!(
            out,
            "    #[serde(default = \"default_{parent_struct}_{}\")]",
            field.name
        )
        .unwrap();
        // We'll generate default functions after the struct
        let _ = default_val; // Used later if needed
    }

    // Skip serializing None
    if is_optional {
        writeln!(out, "    #[serde(skip_serializing_if = \"Option::is_none\")]").unwrap();
    }

    writeln!(out, "    pub {}: {full_type},", field.name).unwrap();
}

fn generate_enum(out: &mut String, parent: &str, field: &str, values: &[String]) {
    let enum_name = format!("{parent}{}", snake_to_pascal(field));

    writeln!(
        out,
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]"
    )
    .unwrap();
    writeln!(out, "pub enum {enum_name} {{").unwrap();

    for value in values {
        let variant = snake_to_pascal(value);
        writeln!(out, "    #[serde(rename = \"{value}\")]").unwrap();
        writeln!(out, "    {variant},").unwrap();
    }

    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();
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

    #[test]
    fn test_basic_schema_generation() {
        let schema = SchemaDefinition {
            imports: Vec::new(),
            name: "transfer_request".to_string(),
            patterns: vec![MutationPattern::Command],
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: vec![
                make_field("amount", "decimal", true),
                make_field("currency", "string", true),
                make_field("description", "string", false),
            ],
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: Vec::new(),
            immutable: None,
            state_machine: None,
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        };

        let output = generate_schemas(&[&schema]);
        assert!(output.contains("pub struct TransferRequest {"));
        assert!(output.contains("pub amount: rust_decimal::Decimal,"));
        assert!(output.contains("pub currency: String,"));
        assert!(output.contains("pub description: Option<String>,"));
    }

    #[test]
    fn test_enum_constraint_generation() {
        let schema = SchemaDefinition {
            imports: Vec::new(),
            name: "account_detail".to_string(),
            patterns: vec![MutationPattern::MasterData],
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: vec![make_field("status", "string", true)],
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: vec![ConstraintDecl::Enum {
                field: "status".to_string(),
                values: vec![
                    "active".to_string(),
                    "suspended".to_string(),
                    "closed".to_string(),
                ],
            }],
            immutable: None,
            state_machine: None,
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        };

        let output = generate_schemas(&[&schema]);
        assert!(output.contains("pub enum AccountDetailStatus {"));
        assert!(output.contains("Active,"));
        assert!(output.contains("Suspended,"));
        assert!(output.contains("Closed,"));
        assert!(output.contains("pub status: AccountDetailStatus,"));
    }
}
