// NexCore -- Nexflow Codegen: Type Mapping
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Maps SchemaDSL field types to Rust type strings.

use nexflow_parser::ast::schema::{CollectionKind, FieldDecl, FieldType};

use crate::naming::snake_to_pascal;

/// Map a `FieldType` to its Rust type string.
pub fn rust_type(ft: &FieldType) -> String {
    match ft {
        FieldType::Base { name, .. } => base_type_to_rust(name),
        FieldType::Collection {
            kind,
            element_types,
        } => {
            match kind {
                CollectionKind::List => {
                    let inner = element_types
                        .first()
                        .map(|t| rust_type(t))
                        .unwrap_or_else(|| "serde_json::Value".to_string());
                    format!("Vec<{inner}>")
                }
                CollectionKind::Set => {
                    let inner = element_types
                        .first()
                        .map(|t| rust_type(t))
                        .unwrap_or_else(|| "String".to_string());
                    format!("std::collections::HashSet<{inner}>")
                }
                CollectionKind::Map => {
                    let key = element_types
                        .first()
                        .map(|t| rust_type(t))
                        .unwrap_or_else(|| "String".to_string());
                    let val = element_types
                        .get(1)
                        .map(|t| rust_type(t))
                        .unwrap_or_else(|| "serde_json::Value".to_string());
                    format!("std::collections::HashMap<{key}, {val}>")
                }
            }
        }
        FieldType::InlineObject { .. } => {
            // v1: use serde_json::Value for inline objects
            "serde_json::Value".to_string()
        }
        FieldType::Custom(name) => snake_to_pascal(name),
        FieldType::Alias(name) => name.clone(),
    }
}

/// Map a base type name to a Rust type string.
fn base_type_to_rust(name: &str) -> String {
    match name {
        "string" | "text" => "String".to_string(),
        "char" => "String".to_string(),
        "integer" => "i64".to_string(),
        "decimal" => "rust_decimal::Decimal".to_string(),
        "float" => "f64".to_string(),
        "boolean" => "bool".to_string(),
        "date" | "bizdate" => "chrono::NaiveDate".to_string(),
        "timestamp" | "datetime" => "chrono::DateTime<chrono::Utc>".to_string(),
        "uuid" => "uuid::Uuid".to_string(),
        "bytes" => "Vec<u8>".to_string(),
        "json" => "serde_json::Value".to_string(),
        other => snake_to_pascal(other),
    }
}

/// Convert a `FieldDecl` to `(field_name, rust_type_string, is_optional)`.
pub fn field_to_rust(f: &FieldDecl) -> (String, String, bool) {
    let type_str = rust_type(&f.field_type);
    let is_optional = f.optional || !f.required;
    (f.name.clone(), type_str, is_optional)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::schema::FieldType;

    #[test]
    fn test_base_types() {
        let cases = vec![
            ("string", "String"),
            ("integer", "i64"),
            ("decimal", "rust_decimal::Decimal"),
            ("boolean", "bool"),
            ("uuid", "uuid::Uuid"),
            ("date", "chrono::NaiveDate"),
            ("timestamp", "chrono::DateTime<chrono::Utc>"),
            ("bytes", "Vec<u8>"),
            ("json", "serde_json::Value"),
            ("char", "String"),
            ("text", "String"),
            ("float", "f64"),
        ];
        for (input, expected) in cases {
            let ft = FieldType::Base {
                name: input.to_string(),
                params: Vec::new(),
            };
            assert_eq!(rust_type(&ft), expected, "Failed for type: {input}");
        }
    }

    #[test]
    fn test_collection_types() {
        let list = FieldType::Collection {
            kind: CollectionKind::List,
            element_types: vec![FieldType::Base {
                name: "string".to_string(),
                params: Vec::new(),
            }],
        };
        assert_eq!(rust_type(&list), "Vec<String>");

        let map = FieldType::Collection {
            kind: CollectionKind::Map,
            element_types: vec![
                FieldType::Base {
                    name: "string".to_string(),
                    params: Vec::new(),
                },
                FieldType::Base {
                    name: "integer".to_string(),
                    params: Vec::new(),
                },
            ],
        };
        assert_eq!(
            rust_type(&map),
            "std::collections::HashMap<String, i64>"
        );
    }

    #[test]
    fn test_custom_and_alias() {
        assert_eq!(
            rust_type(&FieldType::Custom("account_status".to_string())),
            "AccountStatus"
        );
        assert_eq!(
            rust_type(&FieldType::Alias("MoneyAmount".to_string())),
            "MoneyAmount"
        );
    }
}
