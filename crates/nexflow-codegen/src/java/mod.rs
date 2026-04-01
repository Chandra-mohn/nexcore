// NexCore -- Nexflow Codegen: Java/Avro Code Generator (L2)
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Java/Avro artifacts from Nexflow DSL ASTs.
//!
//! **L2 Generator** (.schema -> .avsc + helpers):
//! - `.avsc` (always) -- Avro schema JSON; Maven Avro plugin generates Java SpecificRecord
//! - `*Builder.java` (always) -- fluent builder for Avro SpecificRecord
//! - `*PiiHelper.java` (if PII fields) -- encrypt/decrypt helpers
//! - `*Migration.java` (if version block) -- version constants, compatibility checks
//! - `*StateMachine.java` (if state_machine block) -- state validation, transitions
//! - `Validation.java` (shared, if any constraints) -- zero-dependency validation utility
//!
//! **L3 Generator** (.xform -> Flink MapFunction):
//! - `*Function.java` -- Flink MapFunction/RichMapFunction per transform
//! - Lookups, caching (ValueState/MapState), validation, composition
//!
//! **L4 Generator** (.rules -> Decision engine):
//! - `*Table.java` -- decision table evaluator with hit policies
//! - `*Output.java` -- output records for multi-return tables
//! - `*Rule.java` -- procedural rules with if-then-else logic
//!
//! **L0 Generator** (runtime library):
//! - `NexflowRuntime.java` -- static utility (time, math, string, collection, PII)
//! - `LookupService.java` + `LookupServiceFactory.java` -- pluggable data enrichment
//! - `BusinessCalendar.java` + `DefaultBusinessCalendar.java` -- pluggable calendar
//!
//! **L1 Generator** (.proc -> Flink Job):
//! - `*Job.java` -- Flink streaming job with sources, operators, sinks
//! - Kafka source/sink with Avro, watermarks, checkpointing
//! - Pipeline operators: transform, route, window, join, enrich, aggregate

pub mod avro;
pub mod builder;
pub mod migration;
pub mod naming;
pub mod pii;
pub mod proc;
pub mod rules;
pub mod runtime;
pub mod statemachine;
pub mod types;
pub mod validation;
pub mod xform;

use std::collections::HashMap;

use nexflow_parser::ast::schema::SchemaDefinition;

use crate::GeneratedProject;

/// Configuration for Java code generation.
#[derive(Debug, Clone)]
pub struct JavaGenConfig {
    /// Java package name (default: `com.nexflow.schemas`).
    pub package: String,
    /// Output directory prefix for Avro schemas (default: `src/main/avro`).
    pub avro_dir: String,
    /// Output directory prefix for Java source (default: `src/main/java`).
    pub java_dir: String,
}

impl Default for JavaGenConfig {
    fn default() -> Self {
        Self {
            package: "com.nexflow.schemas".to_string(),
            avro_dir: "src/main/avro".to_string(),
            java_dir: "src/main/java/com/nexflow/schemas".to_string(),
        }
    }
}

/// Generate all Java/Avro artifacts for a set of schemas.
///
/// Returns a `GeneratedProject` with file paths mapped to content.
/// File paths use the Maven standard layout.
pub fn generate_java_schemas(
    schemas: &[SchemaDefinition],
    config: &JavaGenConfig,
) -> Result<GeneratedProject, String> {
    let mut files = HashMap::new();

    for schema in schemas {
        generate_schema_artifacts(schema, config, &mut files);
    }

    // Shared Validation.java (if any schema has constraints)
    let schema_refs: Vec<&SchemaDefinition> = schemas.iter().collect();
    if validation::needs_validation(&schema_refs) {
        let (filename, content) = validation::generate_validation();
        files.insert(format!("{}/{filename}", config.java_dir), content);
    }

    // Generate pom.xml with Avro plugin
    let pom = generate_pom_xml(config);
    files.insert("pom.xml".to_string(), pom);

    Ok(GeneratedProject { files })
}

/// Generate artifacts for a single schema.
fn generate_schema_artifacts(
    schema: &SchemaDefinition,
    config: &JavaGenConfig,
    files: &mut HashMap<String, String>,
) {
    let class_name = naming::to_pascal_case(&schema.name);

    // 1. .avsc (always)
    let avsc = avro::generate_avsc(schema);
    files.insert(
        format!("{}/{class_name}.avsc", config.avro_dir),
        avsc,
    );

    // 2. Builder.java (always)
    let (filename, content) = builder::generate_builder(schema);
    files.insert(format!("{}/{filename}", config.java_dir), content);

    // 3. PiiHelper.java (if PII fields)
    if let Some((filename, content)) = pii::generate_pii_helper(schema) {
        files.insert(format!("{}/{filename}", config.java_dir), content);
    }

    // 4. Migration.java (if version block)
    if let Some((filename, content)) = migration::generate_migration(schema) {
        files.insert(format!("{}/{filename}", config.java_dir), content);
    }

    // 5. StateMachine.java (if state_machine block)
    if let Some((filename, content)) = statemachine::generate_state_machine(schema) {
        files.insert(format!("{}/{filename}", config.java_dir), content);
    }
}

/// Generate a minimal Maven pom.xml with Avro plugin configured.
fn generate_pom_xml(config: &JavaGenConfig) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0
         http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>com.nexflow</groupId>
    <artifactId>nexflow-schemas</artifactId>
    <version>1.0-SNAPSHOT</version>
    <packaging>jar</packaging>

    <properties>
        <maven.compiler.source>17</maven.compiler.source>
        <maven.compiler.target>17</maven.compiler.target>
        <avro.version>1.11.3</avro.version>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

    <dependencies>
        <dependency>
            <groupId>org.apache.avro</groupId>
            <artifactId>avro</artifactId>
            <version>${{avro.version}}</version>
        </dependency>
    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.apache.avro</groupId>
                <artifactId>avro-maven-plugin</artifactId>
                <version>${{avro.version}}</version>
                <executions>
                    <execution>
                        <phase>generate-sources</phase>
                        <goals>
                            <goal>schema</goal>
                        </goals>
                        <configuration>
                            <sourceDirectory>${{project.basedir}}/{avro_dir}</sourceDirectory>
                            <outputDirectory>${{project.basedir}}/{java_src}</outputDirectory>
                            <stringType>String</stringType>
                        </configuration>
                    </execution>
                </executions>
            </plugin>
        </plugins>
    </build>
</project>
"#,
        avro_dir = config.avro_dir,
        java_src = "src/main/java",
    )
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

    fn make_pii_field(name: &str, profile: &str) -> FieldDecl {
        let mut f = make_field(name, "string", true);
        f.pii = Some(PiiModifier {
            profile: Some(profile.to_string()),
        });
        f
    }

    fn make_account_schema() -> SchemaDefinition {
        SchemaDefinition {
            imports: Vec::new(),
            name: "account_detail".to_string(),
            patterns: vec![MutationPattern::MasterData],
            targets: Vec::new(),
            version: Some(VersionInfo {
                number: "1.0.0".to_string(),
                compatibility: Some(CompatibilityMode::Backward),
                previous_version: None,
                deprecation: None,
                migration_guide: None,
            }),
            compatibility: None,
            retention: None,
            identity: vec![make_field("account_id", "uuid", true)],
            streaming: None,
            serialization: None,
            fields: vec![
                make_field("account_number", "string", true),
                make_pii_field("holder_name", "name"),
                make_pii_field("email", "email"),
                make_field("status", "string", true),
                make_field("balance", "decimal", true),
            ],
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: vec![
                ConstraintDecl::Enum {
                    field: "status".to_string(),
                    values: vec![
                        "active".to_string(),
                        "suspended".to_string(),
                        "closed".to_string(),
                    ],
                },
            ],
            immutable: None,
            state_machine: None,
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        }
    }

    #[test]
    fn test_generate_java_schemas_produces_all_files() {
        let schemas = vec![make_account_schema()];
        let config = JavaGenConfig::default();
        let project = generate_java_schemas(&schemas, &config).unwrap();

        // .avsc
        assert!(
            project.files.contains_key("src/main/avro/AccountDetail.avsc"),
            "Missing .avsc file"
        );

        // Builder
        assert!(
            project
                .files
                .contains_key("src/main/java/com/nexflow/schemas/AccountDetailBuilder.java"),
            "Missing Builder"
        );

        // PiiHelper (has PII fields)
        assert!(
            project
                .files
                .contains_key("src/main/java/com/nexflow/schemas/AccountDetailPiiHelper.java"),
            "Missing PiiHelper"
        );

        // Migration (has version)
        assert!(
            project
                .files
                .contains_key("src/main/java/com/nexflow/schemas/AccountDetailMigration.java"),
            "Missing Migration"
        );

        // Validation (has constraints)
        assert!(
            project
                .files
                .contains_key("src/main/java/com/nexflow/schemas/Validation.java"),
            "Missing Validation"
        );

        // pom.xml
        assert!(project.files.contains_key("pom.xml"), "Missing pom.xml");
    }

    #[test]
    fn test_no_optional_files_for_simple_schema() {
        let schema = SchemaDefinition {
            imports: Vec::new(),
            name: "simple".to_string(),
            patterns: Vec::new(),
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: vec![make_field("name", "string", true)],
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

        let config = JavaGenConfig::default();
        let project = generate_java_schemas(&[schema], &config).unwrap();

        // Should have: .avsc, Builder, pom.xml (3 files)
        // Should NOT have: PiiHelper, Migration, StateMachine, Validation
        assert!(project.files.contains_key("src/main/avro/Simple.avsc"));
        assert!(project
            .files
            .contains_key("src/main/java/com/nexflow/schemas/SimpleBuilder.java"));
        assert!(project.files.contains_key("pom.xml"));

        // No optional helpers
        assert!(!project
            .files
            .values()
            .any(|v| v.contains("PiiHelper") || v.contains("Migration") || v.contains("StateMachine")));
        assert!(!project
            .files
            .contains_key("src/main/java/com/nexflow/schemas/Validation.java"));
    }

    #[test]
    fn test_multiple_schemas() {
        let s1 = make_account_schema();
        let s2 = SchemaDefinition {
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
                make_field("source_account_id", "uuid", true),
                make_field("target_account_id", "uuid", true),
                make_field("amount", "decimal", true),
            ],
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: vec![ConstraintDecl::Range {
                field: "amount".to_string(),
                min: "0.01".to_string(),
                max: "999999999.99".to_string(),
            }],
            immutable: None,
            state_machine: None,
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        };

        let config = JavaGenConfig::default();
        let project = generate_java_schemas(&[s1, s2], &config).unwrap();

        // Both schemas should have .avsc
        assert!(project.files.contains_key("src/main/avro/AccountDetail.avsc"));
        assert!(project.files.contains_key("src/main/avro/TransferRequest.avsc"));

        // Both should have builders
        assert!(project
            .files
            .contains_key("src/main/java/com/nexflow/schemas/AccountDetailBuilder.java"));
        assert!(project
            .files
            .contains_key("src/main/java/com/nexflow/schemas/TransferRequestBuilder.java"));

        // Only account has PII
        assert!(project
            .files
            .contains_key("src/main/java/com/nexflow/schemas/AccountDetailPiiHelper.java"));
        assert!(!project
            .files
            .contains_key("src/main/java/com/nexflow/schemas/TransferRequestPiiHelper.java"));

        // Shared Validation (both have constraints)
        assert!(project
            .files
            .contains_key("src/main/java/com/nexflow/schemas/Validation.java"));
    }

    #[test]
    fn test_pom_has_avro_plugin() {
        let config = JavaGenConfig::default();
        let project = generate_java_schemas(&[make_account_schema()], &config).unwrap();
        let pom = &project.files["pom.xml"];

        assert!(pom.contains("avro-maven-plugin"));
        assert!(pom.contains("<goal>schema</goal>"));
        assert!(pom.contains("src/main/avro"));
    }

    #[test]
    fn test_avsc_content_is_valid_json_structure() {
        let config = JavaGenConfig::default();
        let project = generate_java_schemas(&[make_account_schema()], &config).unwrap();
        let avsc = &project.files["src/main/avro/AccountDetail.avsc"];

        // Basic structure checks (not full JSON parse without serde_json dep)
        assert!(avsc.contains("\"type\": \"record\""));
        assert!(avsc.contains("\"name\": \"AccountDetail\""));
        assert!(avsc.contains("\"namespace\": \"com.nexflow.schemas\""));
        assert!(avsc.contains("\"fields\": ["));
        assert!(avsc.contains("\"name\": \"account_id\""));
        assert!(avsc.contains("\"logicalType\": \"uuid\""));
    }
}
