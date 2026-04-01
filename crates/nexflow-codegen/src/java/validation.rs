// NexCore -- Nexflow Codegen: Java Validation Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates a shared `Validation.java` utility class from schema constraints.
//!
//! Zero external dependencies -- uses only Java standard library.
//! One `Validation.java` is shared across all schemas in a project.

use std::fmt::Write;

use nexflow_parser::ast::schema::{ConstraintDecl, SchemaDefinition};

const DEFAULT_PACKAGE: &str = "com.nexflow.schemas";

/// Check whether any schema has constraints that warrant a Validation utility.
pub fn needs_validation(schemas: &[&SchemaDefinition]) -> bool {
    schemas.iter().any(|s| !s.constraints.is_empty())
}

/// Generate the shared `Validation.java` utility class.
///
/// Returns `(filename, content)`.
pub fn generate_validation() -> (String, String) {
    let mut out = String::with_capacity(4096);

    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "import java.math.BigDecimal;").unwrap();
    writeln!(out, "import java.util.Set;").unwrap();
    writeln!(out, "import java.util.regex.Pattern;").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/**").unwrap();
    writeln!(
        out,
        " * Shared validation utilities for Nexflow schema constraints."
    )
    .unwrap();
    writeln!(out, " * Generated -- DO NOT EDIT.").unwrap();
    writeln!(out, " */").unwrap();
    writeln!(out, "public final class Validation {{").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "    private Validation() {{}}").unwrap();
    writeln!(out).unwrap();

    // requireIn (enum validation)
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Validate that value is one of the allowed values.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static void requireIn(String value, String fieldName, String... allowed) {{"
    )
    .unwrap();
    writeln!(out, "        if (value == null) return;").unwrap();
    writeln!(out, "        Set<String> allowedSet = Set.of(allowed);").unwrap();
    writeln!(out, "        if (!allowedSet.contains(value)) {{").unwrap();
    writeln!(
        out,
        "            throw new IllegalArgumentException("
    )
    .unwrap();
    writeln!(
        out,
        "                fieldName + \": value '\" + value + \"' not in \" + allowedSet);"
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // requireRange (long)
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Validate that a long value is within range.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static void requireRange(Long value, long min, long max, String fieldName) {{"
    )
    .unwrap();
    writeln!(out, "        if (value == null) return;").unwrap();
    writeln!(out, "        if (value < min || value > max) {{").unwrap();
    writeln!(
        out,
        "            throw new IllegalArgumentException("
    )
    .unwrap();
    writeln!(
        out,
        "                fieldName + \": \" + value + \" not in [\" + min + \", \" + max + \"]\");"
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // requireDecimalRange
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Validate that a BigDecimal value is within range.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(out, "    public static void requireDecimalRange(").unwrap();
    writeln!(
        out,
        "            BigDecimal value, BigDecimal min, BigDecimal max, String fieldName) {{"
    )
    .unwrap();
    writeln!(out, "        if (value == null) return;").unwrap();
    writeln!(
        out,
        "        if (value.compareTo(min) < 0 || value.compareTo(max) > 0) {{"
    )
    .unwrap();
    writeln!(
        out,
        "            throw new IllegalArgumentException("
    )
    .unwrap();
    writeln!(
        out,
        "                fieldName + \": \" + value + \" not in [\" + min + \", \" + max + \"]\");"
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // requirePattern
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Validate that a string matches the given regex pattern.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static void requirePattern(String value, String regex, String fieldName) {{"
    )
    .unwrap();
    writeln!(out, "        if (value == null) return;").unwrap();
    writeln!(
        out,
        "        if (!Pattern.matches(regex, value)) {{"
    )
    .unwrap();
    writeln!(
        out,
        "            throw new IllegalArgumentException("
    )
    .unwrap();
    writeln!(
        out,
        "                fieldName + \": '\" + value + \"' does not match pattern '\" + regex + \"'\");"
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // requireLength
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Validate string length is within bounds.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static void requireLength(String value, int min, int max, String fieldName) {{"
    )
    .unwrap();
    writeln!(out, "        if (value == null) return;").unwrap();
    writeln!(out, "        int len = value.length();").unwrap();
    writeln!(out, "        if (len < min || len > max) {{").unwrap();
    writeln!(
        out,
        "            throw new IllegalArgumentException("
    )
    .unwrap();
    writeln!(
        out,
        "                fieldName + \": length \" + len + \" not in [\" + min + \", \" + max + \"]\");"
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // requirePositive
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Validate that a BigDecimal is positive.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static void requirePositive(BigDecimal value, String fieldName) {{"
    )
    .unwrap();
    writeln!(out, "        if (value == null) return;").unwrap();
    writeln!(
        out,
        "        if (value.compareTo(BigDecimal.ZERO) <= 0) {{"
    )
    .unwrap();
    writeln!(
        out,
        "            throw new IllegalArgumentException(fieldName + \": must be positive, got \" + value);"
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // requireNonNegative
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Validate that a BigDecimal is non-negative.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static void requireNonNegative(BigDecimal value, String fieldName) {{"
    )
    .unwrap();
    writeln!(out, "        if (value == null) return;").unwrap();
    writeln!(
        out,
        "        if (value.compareTo(BigDecimal.ZERO) < 0) {{"
    )
    .unwrap();
    writeln!(
        out,
        "            throw new IllegalArgumentException(fieldName + \": must be non-negative, got \" + value);"
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(out, "}}").unwrap();

    ("Validation.java".to_string(), out)
}

/// Generate per-schema validation call-site code (for use in constructors or validators).
///
/// Returns a block of Java statements that validate the schema's constraints.
pub fn generate_constraint_calls(schema: &SchemaDefinition) -> String {
    let class_name = super::naming::to_pascal_case(&schema.name);
    let mut out = String::with_capacity(1024);

    // Required field null checks (from identity + required fields)
    for f in schema
        .identity
        .iter()
        .chain(schema.fields.iter())
        .filter(|f| f.required)
    {
        let getter = format!("get{}()", super::naming::to_pascal_case(&f.name));
        writeln!(
            out,
            "        java.util.Objects.requireNonNull({getter}, \"{}.{} is required\");",
            class_name, f.name
        )
        .unwrap();
    }

    // Constraint-specific validations
    for c in &schema.constraints {
        match c {
            ConstraintDecl::Enum { field, values } => {
                let getter = format!("get{}()", super::naming::to_pascal_case(field));
                let allowed: String = values
                    .iter()
                    .map(|v| format!("\"{v}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(
                    out,
                    "        Validation.requireIn({getter}, \"{field}\", {allowed});"
                )
                .unwrap();
            }
            ConstraintDecl::Range { field, min, max } => {
                let getter = format!("get{}()", super::naming::to_pascal_case(field));
                // Detect if the field is decimal type
                let is_decimal = schema
                    .fields
                    .iter()
                    .chain(schema.identity.iter())
                    .find(|f| f.name == *field)
                    .is_some_and(|f| matches!(&f.field_type, nexflow_parser::ast::schema::FieldType::Base { name, .. } if name == "decimal" || name == "money"));

                if is_decimal {
                    writeln!(
                        out,
                        "        Validation.requireDecimalRange({getter}, new java.math.BigDecimal(\"{min}\"), new java.math.BigDecimal(\"{max}\"), \"{field}\");"
                    )
                    .unwrap();
                } else {
                    writeln!(
                        out,
                        "        Validation.requireRange({getter}, {min}L, {max}L, \"{field}\");"
                    )
                    .unwrap();
                }
            }
            ConstraintDecl::Pattern { field, regex } => {
                let getter = format!("get{}()", super::naming::to_pascal_case(field));
                // Escape backslashes for Java string literal
                let escaped = regex.replace('\\', "\\\\");
                writeln!(
                    out,
                    "        Validation.requirePattern({getter}, \"{escaped}\", \"{field}\");"
                )
                .unwrap();
            }
            ConstraintDecl::Length { field, min, max } => {
                let getter = format!("get{}()", super::naming::to_pascal_case(field));
                let max_val = max.unwrap_or(i64::MAX);
                writeln!(
                    out,
                    "        Validation.requireLength({getter}, {min}, {max_val}, \"{field}\");"
                )
                .unwrap();
            }
            ConstraintDecl::BusinessRule { .. } => {
                // Business rules require expression evaluation -- skip for now
            }
        }
    }

    out
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
    fn test_validation_class_structure() {
        let (filename, content) = generate_validation();

        assert_eq!(filename, "Validation.java");
        assert!(content.contains("public final class Validation {"));
        assert!(content.contains("private Validation() {}"));
        assert!(content.contains("public static void requireIn("));
        assert!(content.contains("public static void requireRange("));
        assert!(content.contains("public static void requireDecimalRange("));
        assert!(content.contains("public static void requirePattern("));
        assert!(content.contains("public static void requireLength("));
        assert!(content.contains("public static void requirePositive("));
        assert!(content.contains("public static void requireNonNegative("));
    }

    #[test]
    fn test_constraint_calls_enum() {
        let schema = SchemaDefinition {
            imports: Vec::new(),
            name: "account".to_string(),
            patterns: Vec::new(),
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

        let calls = generate_constraint_calls(&schema);
        assert!(calls.contains("Validation.requireIn(getStatus()"));
        assert!(calls.contains("\"active\", \"suspended\", \"closed\""));
    }

    #[test]
    fn test_constraint_calls_range_decimal() {
        let schema = SchemaDefinition {
            imports: Vec::new(),
            name: "transfer".to_string(),
            patterns: Vec::new(),
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: vec![make_field("amount", "decimal", true)],
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

        let calls = generate_constraint_calls(&schema);
        assert!(calls.contains("Validation.requireDecimalRange(getAmount()"));
        assert!(calls.contains("new java.math.BigDecimal(\"0.01\")"));
    }

    #[test]
    fn test_constraint_calls_pattern() {
        let schema = SchemaDefinition {
            imports: Vec::new(),
            name: "contact".to_string(),
            patterns: Vec::new(),
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: vec![make_field("email", "string", true)],
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: vec![ConstraintDecl::Pattern {
                field: "email".to_string(),
                regex: "^[a-z]+@[a-z]+\\.[a-z]+$".to_string(),
            }],
            immutable: None,
            state_machine: None,
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        };

        let calls = generate_constraint_calls(&schema);
        assert!(calls.contains("Validation.requirePattern(getEmail()"));
    }

    #[test]
    fn test_needs_validation() {
        let s1 = SchemaDefinition {
            imports: Vec::new(),
            name: "a".to_string(),
            patterns: Vec::new(),
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: Vec::new(),
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: vec![ConstraintDecl::Enum {
                field: "x".to_string(),
                values: vec!["y".to_string()],
            }],
            immutable: None,
            state_machine: None,
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        };
        let s2 = SchemaDefinition {
            constraints: Vec::new(),
            ..s1.clone()
        };

        assert!(needs_validation(&[&s1]));
        assert!(!needs_validation(&[&s2]));
    }
}
