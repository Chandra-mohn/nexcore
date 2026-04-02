// NexCore -- Nexflow Codegen: Rust Transform Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates pure Rust functions from TransformDSL ASTs.
//!
//! No framework dependencies -- just typed functions using serde structs
//! and rust_decimal. Each `TransformDef` becomes a `pub fn` and each
//! `TransformBlockDef` becomes a `pub fn` that calls composed transforms.
//!
//! Generated code depends on:
//! - `serde` (for struct derive)
//! - `rust_decimal::Decimal` (for decimal types)
//! - Schema structs (from gen_schema.rs)

use std::fmt::Write;

use nexflow_parser::ast::transform::{
    ComposeBlock, ComposeRef, FieldSpec, Mapping, Statement, TransformBlockDef, TransformDef,
    TransformFieldType, TransformProgram,
};

use crate::naming::snake_to_pascal;

use super::expression::translate_rust_expr;

/// Generate a complete Rust module from a `TransformProgram`.
///
/// Returns `(filename, content)` -- typically `("transforms.rs", ...)`.
pub fn generate_rust_transforms(program: &TransformProgram) -> (String, String) {
    let mut out = String::with_capacity(8192);

    writeln!(out, "//! Generated transform functions from Nexflow TransformDSL.").unwrap();
    writeln!(out, "//! DO NOT EDIT -- regenerate from .xform files.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#![allow(unused_variables, dead_code)]").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use rust_decimal::Decimal;").unwrap();
    writeln!(out, "use super::models::*;").unwrap();
    writeln!(out).unwrap();

    for transform in &program.transforms {
        generate_transform_fn(&mut out, transform);
    }

    for block in &program.transform_blocks {
        generate_block_fn(&mut out, block);
    }

    ("transforms.rs".to_string(), out)
}

/// Generate a pure function from a `TransformDef`.
fn generate_transform_fn(out: &mut String, transform: &TransformDef) {
    let fn_name = &transform.name;
    let (input_type, input_name) = resolve_rust_io(&transform.inputs, "input");
    let (output_type, _) = resolve_rust_io(&transform.outputs, "output");

    // Doc comment
    if let Some(desc) = &transform.description {
        writeln!(out, "/// {desc}").unwrap();
    }
    if transform.pure == Some(true) {
        writeln!(out, "///").unwrap();
        writeln!(out, "/// Pure function (no side effects).").unwrap();
    }

    // Function signature
    let input_param = if input_type == "String" || input_type == "i64" || input_type == "bool" {
        format!("{input_name}: {input_type}")
    } else if input_type == "&str" {
        format!("{input_name}: &str")
    } else {
        format!("{input_name}: &{input_type}")
    };

    writeln!(out, "pub fn {fn_name}({input_param}) -> {output_type} {{").unwrap();

    // Local variable bindings
    let locals: Vec<&Statement> = transform.apply.iter().filter(|s| s.is_let).collect();
    let assignments: Vec<&Statement> = transform.apply.iter().filter(|s| !s.is_let).collect();

    for s in &locals {
        let var_name = &s.target;
        let rust_expr = translate_rust_expr(&s.expression);
        writeln!(out, "    let {var_name} = {rust_expr};").unwrap();
    }

    if !locals.is_empty() && !assignments.is_empty() {
        writeln!(out).unwrap();
    }

    // If output is a struct, build it
    let is_struct_output = !is_primitive_type(&output_type);

    if is_struct_output && !assignments.is_empty() {
        writeln!(out, "    {output_type} {{").unwrap();
        for s in &assignments {
            let field_name = extract_field_name(&s.target);
            let rust_expr = translate_rust_expr(&s.expression);
            writeln!(out, "        {field_name}: {rust_expr},").unwrap();
        }
        writeln!(out, "    }}").unwrap();
    } else if assignments.len() == 1 {
        // Single expression return
        let rust_expr = translate_rust_expr(&assignments[0].expression);
        writeln!(out, "    {rust_expr}").unwrap();
    } else if assignments.is_empty() && locals.len() == 1 {
        // Single let binding returned
        writeln!(out, "    {}", locals[0].target).unwrap();
    } else {
        writeln!(out, "    todo!(\"implement transform body\")").unwrap();
    }

    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();
}

/// Generate a function from a `TransformBlockDef`.
fn generate_block_fn(out: &mut String, block: &TransformBlockDef) {
    let fn_name = &block.name;
    let (input_type, input_name) = resolve_rust_io(&block.inputs, "input");
    let (output_type, _) = resolve_rust_io(&block.outputs, "output");

    if let Some(desc) = &block.description {
        writeln!(out, "/// {desc}").unwrap();
    }

    let input_param = if is_primitive_type(&input_type) {
        format!("{input_name}: {input_type}")
    } else {
        format!("{input_name}: &{input_type}")
    };

    writeln!(out, "pub fn {fn_name}({input_param}) -> {output_type} {{").unwrap();

    if let Some(compose) = &block.compose {
        write_rust_compose(out, compose, &input_name, &output_type);
    } else if !block.mappings.is_empty() {
        write_rust_mappings(out, &block.mappings, &output_type);
    } else {
        writeln!(out, "    todo!(\"implement block transform\")").unwrap();
    }

    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();
}

/// Write struct construction from mappings.
fn write_rust_mappings(out: &mut String, mappings: &[Mapping], output_type: &str) {
    writeln!(out, "    {output_type} {{").unwrap();
    for m in mappings {
        let field_name = extract_field_name(&m.target);
        let rust_expr = translate_rust_expr(&m.expression);
        writeln!(out, "        {field_name}: {rust_expr},").unwrap();
    }
    writeln!(out, "    }}").unwrap();
}

/// Write composition logic.
fn write_rust_compose(
    out: &mut String,
    compose: &ComposeBlock,
    input_name: &str,
    _output_type: &str,
) {
    let compose_type = compose.compose_type.as_deref().unwrap_or("sequential");

    match compose_type {
        "sequential" => {
            writeln!(out, "    // Sequential composition").unwrap();
            writeln!(out, "    let mut current = {input_name}.clone();").unwrap();
            for (i, r) in compose.refs.iter().enumerate() {
                if let ComposeRef::Simple(name) = r {
                    let is_last = i + 1 == compose.refs.len();
                    if is_last {
                        writeln!(out, "    {name}(&current)").unwrap();
                    } else {
                        writeln!(out, "    let current = {name}(&current);").unwrap();
                    }
                }
            }
        }
        "conditional" => {
            writeln!(out, "    // Conditional composition").unwrap();
            let mut first = true;
            for r in &compose.refs {
                match r {
                    ComposeRef::Conditional {
                        condition,
                        transform,
                    } => {
                        let rust_cond = translate_rust_expr(condition);
                        let keyword = if first { "if" } else { "else if" };
                        first = false;
                        writeln!(out, "    {keyword} {rust_cond} {{").unwrap();
                        writeln!(out, "        {transform}({input_name})").unwrap();
                        writeln!(out, "    }}").unwrap();
                    }
                    ComposeRef::Otherwise(name) => {
                        writeln!(out, "    else {{").unwrap();
                        writeln!(out, "        {name}({input_name})").unwrap();
                        writeln!(out, "    }}").unwrap();
                    }
                    ComposeRef::Simple(name) => {
                        writeln!(out, "    {name}({input_name})").unwrap();
                    }
                }
            }
        }
        _ => {
            // Parallel -- in Rust, just call sequentially (no async runtime assumed)
            writeln!(out, "    // Parallel (executed sequentially without async runtime)").unwrap();
            for r in &compose.refs {
                if let ComposeRef::Simple(name) = r {
                    writeln!(out, "    let _{name}_result = {name}({input_name});").unwrap();
                }
            }
            writeln!(out, "    todo!(\"merge parallel results\")").unwrap();
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Resolve input/output type from FieldSpecs.
fn resolve_rust_io(fields: &[FieldSpec], default_name: &str) -> (String, String) {
    if fields.len() == 1 {
        let f = &fields[0];
        let name = f
            .name
            .as_deref()
            .unwrap_or(default_name)
            .to_string();
        let type_str = transform_field_type_to_rust(&f.field_type);
        (type_str, name)
    } else if fields.is_empty() {
        ("()".to_string(), default_name.to_string())
    } else {
        // Multiple fields -> tuple or custom struct
        // For now, use a generic approach
        ("serde_json::Value".to_string(), default_name.to_string())
    }
}

fn transform_field_type_to_rust(ft: &TransformFieldType) -> String {
    match ft {
        TransformFieldType::Base { name, .. } => match name.as_str() {
            "string" | "text" => "String".to_string(),
            "integer" => "i64".to_string(),
            "decimal" | "money" => "Decimal".to_string(),
            "boolean" => "bool".to_string(),
            "date" | "bizdate" => "chrono::NaiveDate".to_string(),
            "timestamp" => "chrono::DateTime<chrono::Utc>".to_string(),
            "uuid" => "uuid::Uuid".to_string(),
            other => snake_to_pascal(other),
        },
        TransformFieldType::Collection { kind, element_types } => {
            let inner = element_types
                .first()
                .map(|t| transform_field_type_to_rust(t))
                .unwrap_or_else(|| "serde_json::Value".to_string());
            match kind {
                nexflow_parser::ast::transform::TransformCollectionKind::List => {
                    format!("Vec<{inner}>")
                }
                nexflow_parser::ast::transform::TransformCollectionKind::Set => {
                    format!("std::collections::HashSet<{inner}>")
                }
                nexflow_parser::ast::transform::TransformCollectionKind::Map => {
                    format!("std::collections::HashMap<String, {inner}>")
                }
            }
        }
        TransformFieldType::Reference(name) => snake_to_pascal(name),
        TransformFieldType::AliasRef(name) => name.clone(),
    }
}

fn extract_field_name(target: &str) -> &str {
    // "output.field_name" -> "field_name"
    target.split('.').last().unwrap_or(target)
}

fn is_primitive_type(t: &str) -> bool {
    matches!(
        t,
        "String"
            | "&str"
            | "i64"
            | "i32"
            | "f64"
            | "bool"
            | "Decimal"
            | "()"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_simple_program() -> TransformProgram {
        TransformProgram {
            imports: Vec::new(),
            transforms: vec![
                TransformDef {
                    name: "normalize_name".to_string(),
                    version: None,
                    description: Some("Normalize a name to uppercase".to_string()),
                    previous_version: None,
                    compatibility: None,
                    pure: Some(true),
                    idempotent: None,
                    cache: None,
                    inputs: vec![FieldSpec {
                        name: Some("name".to_string()),
                        field_type: TransformFieldType::Base {
                            name: "string".to_string(),
                            constraints: Vec::new(),
                        },
                        nullable: false,
                        required: true,
                        default: None,
                    }],
                    outputs: vec![FieldSpec {
                        name: None,
                        field_type: TransformFieldType::Base {
                            name: "string".to_string(),
                            constraints: Vec::new(),
                        },
                        nullable: false,
                        required: true,
                        default: None,
                    }],
                    lookup: None,
                    lookups: Vec::new(),
                    state: None,
                    params: Vec::new(),
                    validate_input: Vec::new(),
                    apply: vec![Statement {
                        target: "output".to_string(),
                        expression: "upper(name)".to_string(),
                        is_let: false,
                    }],
                    validate_output: Vec::new(),
                    on_error: None,
                },
                TransformDef {
                    name: "enrich_account".to_string(),
                    version: None,
                    description: None,
                    previous_version: None,
                    compatibility: None,
                    pure: None,
                    idempotent: None,
                    cache: None,
                    inputs: vec![FieldSpec {
                        name: None,
                        field_type: TransformFieldType::Reference("account_detail".to_string()),
                        nullable: false,
                        required: true,
                        default: None,
                    }],
                    outputs: vec![FieldSpec {
                        name: None,
                        field_type: TransformFieldType::Reference("account_summary".to_string()),
                        nullable: false,
                        required: true,
                        default: None,
                    }],
                    lookup: None,
                    lookups: Vec::new(),
                    state: None,
                    params: Vec::new(),
                    validate_input: Vec::new(),
                    apply: vec![
                        Statement {
                            target: "output.account_id".to_string(),
                            expression: "input.account_id".to_string(),
                            is_let: false,
                        },
                        Statement {
                            target: "output.holder_name".to_string(),
                            expression: "input.holder_name".to_string(),
                            is_let: false,
                        },
                    ],
                    validate_output: Vec::new(),
                    on_error: None,
                },
            ],
            transform_blocks: vec![TransformBlockDef {
                name: "format_account".to_string(),
                version: None,
                description: Some("Format for display".to_string()),
                uses: vec!["normalize_name".to_string()],
                inputs: vec![FieldSpec {
                    name: None,
                    field_type: TransformFieldType::Reference("account_detail".to_string()),
                    nullable: false,
                    required: true,
                    default: None,
                }],
                outputs: vec![FieldSpec {
                    name: None,
                    field_type: TransformFieldType::Reference("account_summary".to_string()),
                    nullable: false,
                    required: true,
                    default: None,
                }],
                validate_input: Vec::new(),
                invariants: Vec::new(),
                mappings: vec![
                    Mapping {
                        target: "output.account_id".to_string(),
                        expression: "input.account_id".to_string(),
                    },
                    Mapping {
                        target: "output.holder_name".to_string(),
                        expression: "input.holder_name".to_string(),
                    },
                ],
                compose: None,
                validate_output: Vec::new(),
                on_change: None,
                on_error: None,
            }],
        }
    }

    #[test]
    fn test_generates_module() {
        let (filename, content) = generate_rust_transforms(&make_simple_program());
        assert_eq!(filename, "transforms.rs");
        assert!(content.contains("//! Generated transform functions"));
        assert!(content.contains("use rust_decimal::Decimal;"));
    }

    #[test]
    fn test_simple_string_transform() {
        let (_, content) = generate_rust_transforms(&make_simple_program());
        assert!(content.contains("pub fn normalize_name(name: String) -> String {"));
        assert!(content.contains("/// Normalize a name to uppercase"));
        assert!(content.contains("/// Pure function"));
    }

    #[test]
    fn test_struct_transform() {
        let (_, content) = generate_rust_transforms(&make_simple_program());
        assert!(content.contains("pub fn enrich_account(input: &AccountDetail) -> AccountSummary {"));
        assert!(content.contains("AccountSummary {"));
        assert!(content.contains("account_id:"));
        assert!(content.contains("holder_name:"));
    }

    #[test]
    fn test_block_transform_with_mappings() {
        let (_, content) = generate_rust_transforms(&make_simple_program());
        assert!(content.contains("pub fn format_account(input: &AccountDetail) -> AccountSummary {"));
        assert!(content.contains("AccountSummary {"));
    }

    #[test]
    fn test_type_resolution() {
        assert_eq!(
            transform_field_type_to_rust(&TransformFieldType::Base {
                name: "string".to_string(),
                constraints: Vec::new()
            }),
            "String"
        );
        assert_eq!(
            transform_field_type_to_rust(&TransformFieldType::Base {
                name: "decimal".to_string(),
                constraints: Vec::new()
            }),
            "Decimal"
        );
        assert_eq!(
            transform_field_type_to_rust(&TransformFieldType::Reference(
                "account_detail".to_string()
            )),
            "AccountDetail"
        );
    }
}
