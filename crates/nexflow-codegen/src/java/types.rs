// NexCore -- Nexflow Codegen: Java/Avro Type Mapping
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Maps SchemaDSL field types to Java type strings and Avro schema fragments.

use nexflow_parser::ast::schema::{CollectionKind, FieldType};

use super::naming::to_pascal_case;

// ---------------------------------------------------------------------------
// Java type mapping
// ---------------------------------------------------------------------------

/// Map a `FieldType` to its Java type string.
pub fn java_type(ft: &FieldType) -> String {
    match ft {
        FieldType::Base { name, params } => base_type_to_java(name, params),
        FieldType::Collection {
            kind,
            element_types,
        } => collection_to_java(kind, element_types),
        FieldType::InlineObject { .. } => "Object".to_string(),
        FieldType::Custom(name) => to_pascal_case(name),
        FieldType::Alias(name) => name.clone(),
    }
}

fn base_type_to_java(name: &str, _params: &[i64]) -> String {
    match name {
        "string" | "text" | "char" => "String".to_string(),
        "integer" => "Long".to_string(),
        "decimal" | "money" => "java.math.BigDecimal".to_string(),
        "float" => "Double".to_string(),
        "boolean" => "Boolean".to_string(),
        "date" | "bizdate" => "java.time.LocalDate".to_string(),
        "timestamp" | "datetime" => "java.time.Instant".to_string(),
        "uuid" => "String".to_string(), // Avro encodes UUID as string
        "bytes" => "byte[]".to_string(),
        "json" => "Object".to_string(),
        other => to_pascal_case(other),
    }
}

fn collection_to_java(kind: &CollectionKind, element_types: &[FieldType]) -> String {
    match kind {
        CollectionKind::List => {
            let inner = element_types
                .first()
                .map(|t| java_type(t))
                .unwrap_or_else(|| "Object".to_string());
            format!("java.util.List<{inner}>")
        }
        CollectionKind::Set => {
            let inner = element_types
                .first()
                .map(|t| java_type(t))
                .unwrap_or_else(|| "Object".to_string());
            format!("java.util.Set<{inner}>")
        }
        CollectionKind::Map => {
            let key = element_types
                .first()
                .map(|t| java_type(t))
                .unwrap_or_else(|| "String".to_string());
            let val = element_types
                .get(1)
                .map(|t| java_type(t))
                .unwrap_or_else(|| "Object".to_string());
            format!("java.util.Map<{key}, {val}>")
        }
    }
}

/// Return the Java imports needed for a given field type.
pub fn java_imports_for_type(ft: &FieldType) -> Vec<&'static str> {
    match ft {
        FieldType::Base { name, .. } => match name.as_str() {
            "decimal" | "money" => vec!["java.math.BigDecimal"],
            "date" | "bizdate" => vec!["java.time.LocalDate"],
            "timestamp" | "datetime" => vec!["java.time.Instant"],
            _ => vec![],
        },
        FieldType::Collection { kind, .. } => match kind {
            CollectionKind::List => vec!["java.util.List"],
            CollectionKind::Set => vec!["java.util.Set"],
            CollectionKind::Map => vec!["java.util.Map"],
        },
        _ => vec![],
    }
}

// ---------------------------------------------------------------------------
// Avro type mapping
// ---------------------------------------------------------------------------

/// Map a `FieldType` to its Avro schema JSON fragment (as a string).
///
/// Returns the JSON value that goes into the `"type"` field of an Avro field.
/// For optional fields, the caller wraps this in `["null", <type>]`.
pub fn avro_type(ft: &FieldType) -> String {
    match ft {
        FieldType::Base { name, params } => base_type_to_avro(name, params),
        FieldType::Collection {
            kind,
            element_types,
        } => collection_to_avro(kind, element_types),
        FieldType::InlineObject { fields } => inline_object_to_avro(fields),
        FieldType::Custom(name) => format!("\"{}\"", to_pascal_case(name)),
        FieldType::Alias(name) => format!("\"{name}\""),
    }
}

fn base_type_to_avro(name: &str, params: &[i64]) -> String {
    match name {
        "string" | "text" | "char" => "\"string\"".to_string(),
        "integer" => "\"int\"".to_string(),
        "boolean" => "\"boolean\"".to_string(),
        "bytes" => "\"bytes\"".to_string(),
        "float" => "\"double\"".to_string(),
        "decimal" | "money" => {
            let (precision, scale) = if name == "money" {
                (19, 4)
            } else if params.len() >= 2 {
                (params[0], params[1])
            } else {
                (38, 9) // default
            };
            format!(
                "{{\"type\": \"bytes\", \"logicalType\": \"decimal\", \"precision\": {precision}, \"scale\": {scale}}}"
            )
        }
        "date" | "bizdate" => {
            "{\"type\": \"int\", \"logicalType\": \"date\"}".to_string()
        }
        "timestamp" | "datetime" => {
            "{\"type\": \"long\", \"logicalType\": \"timestamp-millis\"}".to_string()
        }
        "uuid" => {
            "{\"type\": \"string\", \"logicalType\": \"uuid\"}".to_string()
        }
        "json" => "\"string\"".to_string(), // JSON stored as string in Avro
        other => format!("\"{}\"", to_pascal_case(other)),
    }
}

fn collection_to_avro(kind: &CollectionKind, element_types: &[FieldType]) -> String {
    match kind {
        CollectionKind::List | CollectionKind::Set => {
            let items = element_types
                .first()
                .map(|t| avro_type(t))
                .unwrap_or_else(|| "\"string\"".to_string());
            format!("{{\"type\": \"array\", \"items\": {items}}}")
        }
        CollectionKind::Map => {
            let values = element_types
                .get(1)
                .map(|t| avro_type(t))
                .unwrap_or_else(|| "\"string\"".to_string());
            format!("{{\"type\": \"map\", \"values\": {values}}}")
        }
    }
}

fn inline_object_to_avro(fields: &[nexflow_parser::ast::schema::FieldDecl]) -> String {
    use std::fmt::Write;

    let mut out = String::with_capacity(256);
    out.push_str("{\"type\": \"record\", \"name\": \"InlineRecord\", \"fields\": [");
    for (i, f) in fields.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let avro_t = avro_type(&f.field_type);
        let is_optional = f.optional || !f.required;
        if is_optional {
            write!(
                out,
                "{{\"name\": \"{}\", \"type\": [\"null\", {}], \"default\": null}}",
                f.name, avro_t
            )
            .unwrap();
        } else {
            write!(
                out,
                "{{\"name\": \"{}\", \"type\": {}}}",
                f.name, avro_t
            )
            .unwrap();
        }
    }
    out.push_str("]}");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_base_types() {
        let cases = vec![
            ("string", "String"),
            ("integer", "Long"),
            ("decimal", "java.math.BigDecimal"),
            ("boolean", "Boolean"),
            ("uuid", "String"),
            ("date", "java.time.LocalDate"),
            ("timestamp", "java.time.Instant"),
            ("bytes", "byte[]"),
            ("float", "Double"),
            ("char", "String"),
            ("text", "String"),
        ];
        for (input, expected) in cases {
            let ft = FieldType::Base {
                name: input.to_string(),
                params: Vec::new(),
            };
            assert_eq!(java_type(&ft), expected, "Failed for type: {input}");
        }
    }

    #[test]
    fn test_java_collection_types() {
        let list = FieldType::Collection {
            kind: CollectionKind::List,
            element_types: vec![FieldType::Base {
                name: "string".to_string(),
                params: Vec::new(),
            }],
        };
        assert_eq!(java_type(&list), "java.util.List<String>");

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
        assert_eq!(java_type(&map), "java.util.Map<String, Long>");
    }

    #[test]
    fn test_avro_base_types() {
        let cases = vec![
            ("string", vec![], "\"string\""),
            ("integer", vec![], "\"int\""),
            ("boolean", vec![], "\"boolean\""),
            ("bytes", vec![], "\"bytes\""),
            ("float", vec![], "\"double\""),
        ];
        for (name, params, expected) in cases {
            let ft = FieldType::Base {
                name: name.to_string(),
                params,
            };
            assert_eq!(avro_type(&ft), expected, "Failed for avro type: {name}");
        }
    }

    #[test]
    fn test_avro_decimal_params() {
        let ft = FieldType::Base {
            name: "decimal".to_string(),
            params: vec![14, 2],
        };
        let result = avro_type(&ft);
        assert!(result.contains("\"precision\": 14"));
        assert!(result.contains("\"scale\": 2"));
    }

    #[test]
    fn test_avro_logical_types() {
        let date = FieldType::Base {
            name: "date".to_string(),
            params: Vec::new(),
        };
        assert!(avro_type(&date).contains("\"logicalType\": \"date\""));

        let ts = FieldType::Base {
            name: "timestamp".to_string(),
            params: Vec::new(),
        };
        assert!(avro_type(&ts).contains("\"logicalType\": \"timestamp-millis\""));

        let uuid = FieldType::Base {
            name: "uuid".to_string(),
            params: Vec::new(),
        };
        assert!(avro_type(&uuid).contains("\"logicalType\": \"uuid\""));
    }

    #[test]
    fn test_avro_collection() {
        let list = FieldType::Collection {
            kind: CollectionKind::List,
            element_types: vec![FieldType::Base {
                name: "string".to_string(),
                params: Vec::new(),
            }],
        };
        let result = avro_type(&list);
        assert!(result.contains("\"type\": \"array\""));
        assert!(result.contains("\"items\": \"string\""));
    }
}
