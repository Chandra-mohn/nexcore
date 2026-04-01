// NexCore -- Nexflow Codegen: Java/Flink Transform Generator (L3)
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Flink `MapFunction` / `RichMapFunction` Java classes from
//! TransformDSL ASTs.
//!
//! **L3 Generator** -- depends on L2 (.schema -> .avsc) for type references.
//!
//! For each `TransformDef`, produces a `*Function.java` implementing:
//! - `MapFunction<I, O>` (simple, no state/cache/lookups)
//! - `RichMapFunction<I, O>` (with open() lifecycle for state/cache/lookups)
//!
//! For each `TransformBlockDef`, produces a `*Function.java` with:
//! - Multi-field mappings via Avro builder pattern
//! - Transform composition (sequential/parallel/conditional)

pub mod cache;
pub mod compose;
pub mod expression;
pub mod lookup;
pub mod map_function;

use std::collections::HashMap;

use nexflow_parser::ast::transform::TransformProgram;

use crate::GeneratedProject;

/// Configuration for Java transform generation.
#[derive(Debug, Clone)]
pub struct XformGenConfig {
    /// Java package name (default: `com.nexflow.transform`).
    pub package: String,
    /// Output directory prefix (default: `src/main/java/com/nexflow/transform`).
    pub java_dir: String,
}

impl Default for XformGenConfig {
    fn default() -> Self {
        Self {
            package: "com.nexflow.transform".to_string(),
            java_dir: "src/main/java/com/nexflow/transform".to_string(),
        }
    }
}

/// Generate all Java/Flink MapFunction classes for a transform program.
///
/// Returns a `GeneratedProject` with file paths mapped to content.
pub fn generate_java_transforms(
    program: &TransformProgram,
    config: &XformGenConfig,
) -> Result<GeneratedProject, String> {
    let mut files = HashMap::new();

    // Generate MapFunction for each TransformDef
    for transform in &program.transforms {
        let (filename, content) = map_function::generate_map_function(transform);
        files.insert(format!("{}/{filename}", config.java_dir), content);
    }

    // Generate MapFunction for each TransformBlockDef
    for block in &program.transform_blocks {
        let (filename, content) = map_function::generate_block_function(block);
        files.insert(format!("{}/{filename}", config.java_dir), content);
    }

    Ok(GeneratedProject { files })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::transform::*;

    fn make_program() -> TransformProgram {
        TransformProgram {
            imports: Vec::new(),
            transforms: vec![
                TransformDef {
                    name: "normalize_name".to_string(),
                    version: None,
                    description: None,
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
                        name: Some("normalized".to_string()),
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
                        target: "output.normalized".to_string(),
                        expression: "upper(input.name)".to_string(),
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
                    cache: Some(CacheDecl {
                        ttl: Some("300s".to_string()),
                        key: vec!["account_id".to_string()],
                    }),
                    inputs: vec![FieldSpec {
                        name: None,
                        field_type: TransformFieldType::Reference(
                            "account_detail".to_string(),
                        ),
                        nullable: false,
                        required: true,
                        default: None,
                    }],
                    outputs: vec![FieldSpec {
                        name: None,
                        field_type: TransformFieldType::Reference(
                            "account_summary".to_string(),
                        ),
                        nullable: false,
                        required: true,
                        default: None,
                    }],
                    lookup: None,
                    lookups: vec![LookupFieldDecl {
                        name: "customer".to_string(),
                        source: "customer_profile".to_string(),
                    }],
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
                name: "format_account_summary".to_string(),
                version: None,
                description: None,
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
                        target: "summary.account_id".to_string(),
                        expression: "input.account_id".to_string(),
                    },
                    Mapping {
                        target: "summary.holder_name".to_string(),
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
    fn test_generate_java_transforms_produces_all_files() {
        let program = make_program();
        let config = XformGenConfig::default();
        let project = generate_java_transforms(&program, &config).unwrap();

        // Should have 3 files: 2 transforms + 1 block
        assert_eq!(project.files.len(), 3);

        let dir = &config.java_dir;

        assert!(
            project
                .files
                .contains_key(&format!("{dir}/NormalizeNameFunction.java")),
            "Missing NormalizeNameFunction"
        );
        assert!(
            project
                .files
                .contains_key(&format!("{dir}/EnrichAccountFunction.java")),
            "Missing EnrichAccountFunction"
        );
        assert!(
            project
                .files
                .contains_key(&format!("{dir}/FormatAccountSummaryFunction.java")),
            "Missing FormatAccountSummaryFunction"
        );
    }

    #[test]
    fn test_simple_transform_is_map_function() {
        let program = make_program();
        let config = XformGenConfig::default();
        let project = generate_java_transforms(&program, &config).unwrap();

        let content = &project.files[&format!(
            "{}/NormalizeNameFunction.java",
            config.java_dir
        )];
        assert!(content.contains("implements MapFunction<String, String>"));
        assert!(!content.contains("RichMapFunction"));
    }

    #[test]
    fn test_cached_transform_is_rich() {
        let program = make_program();
        let config = XformGenConfig::default();
        let project = generate_java_transforms(&program, &config).unwrap();

        let content = &project.files[&format!(
            "{}/EnrichAccountFunction.java",
            config.java_dir
        )];
        assert!(content.contains("extends RichMapFunction<AccountDetail, AccountSummary>"));
        assert!(content.contains("open(Configuration parameters)"));
        assert!(content.contains("transformWithCache"));
        assert!(content.contains("MapState<String, Object>"));
    }

    #[test]
    fn test_block_transform_has_composition() {
        let program = make_program();
        let config = XformGenConfig::default();
        let project = generate_java_transforms(&program, &config).unwrap();

        let content = &project.files[&format!(
            "{}/FormatAccountSummaryFunction.java",
            config.java_dir
        )];
        assert!(content.contains("NormalizeNameFunction normalizeName"));
        assert!(content.contains("AccountSummary.newBuilder()"));
    }
}
