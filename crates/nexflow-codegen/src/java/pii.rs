// NexCore -- Nexflow Codegen: Java PII Helper Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates `*PiiHelper.java` classes for schemas with PII-annotated fields.
//!
//! Each PII field gets per-field encrypt/decrypt methods.
//! Bulk `encryptAll()` / `decryptAll()` create new records with all PII fields processed.
//! Uses `NexflowRuntime.encrypt()` / `NexflowRuntime.decrypt()` for actual crypto.

use std::fmt::Write;

use nexflow_parser::ast::schema::{FieldDecl, SchemaDefinition};

use super::naming::{to_camel_case, to_pascal_case};

const DEFAULT_PACKAGE: &str = "com.nexflow.schemas";

/// Check whether a schema has any PII fields.
pub fn has_pii_fields(schema: &SchemaDefinition) -> bool {
    schema
        .identity
        .iter()
        .chain(schema.fields.iter())
        .any(|f| f.pii.is_some())
}

/// Generate a PiiHelper Java class for the given schema.
///
/// Returns `Some((filename, content))` if the schema has PII fields, `None` otherwise.
pub fn generate_pii_helper(schema: &SchemaDefinition) -> Option<(String, String)> {
    if !has_pii_fields(schema) {
        return None;
    }

    let class_name = to_pascal_case(&schema.name);
    let helper_name = format!("{class_name}PiiHelper");
    let builder_name = format!("{class_name}Builder");
    let filename = format!("{helper_name}.java");

    let pii_fields: Vec<&FieldDecl> = schema
        .identity
        .iter()
        .chain(schema.fields.iter())
        .filter(|f| f.pii.is_some())
        .collect();

    let mut out = String::with_capacity(4096);

    // Package
    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();

    // Class header
    writeln!(out, "/**").unwrap();
    writeln!(
        out,
        " * PII encryption/decryption helper for {{@link {class_name}}}."
    )
    .unwrap();
    writeln!(out, " * Generated from {}.schema -- DO NOT EDIT.", schema.name).unwrap();
    writeln!(out, " */").unwrap();
    writeln!(out, "public final class {helper_name} {{").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "    private {helper_name}() {{}}").unwrap();
    writeln!(out).unwrap();

    // Per-field encrypt/decrypt methods
    for f in &pii_fields {
        let field_pascal = to_pascal_case(&f.name);
        let profile = f
            .pii
            .as_ref()
            .and_then(|p| p.profile.as_deref())
            .unwrap_or("default");

        // encrypt
        writeln!(out, "    /**").unwrap();
        writeln!(
            out,
            "     * Encrypt the {{@code {}}} field using '{}' profile.",
            f.name, profile
        )
        .unwrap();
        writeln!(out, "     */").unwrap();
        writeln!(
            out,
            "    public static String encrypt{field_pascal}({class_name} record) {{"
        )
        .unwrap();
        writeln!(
            out,
            "        if (record.get{field_pascal}() == null) return null;"
        )
        .unwrap();
        writeln!(
            out,
            "        return NexflowRuntime.encrypt(record.get{field_pascal}().toString(), \"{profile}\");"
        )
        .unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out).unwrap();

        // decrypt
        writeln!(out, "    /**").unwrap();
        writeln!(
            out,
            "     * Decrypt the {{@code {}}} field using '{}' profile.",
            f.name, profile
        )
        .unwrap();
        writeln!(out, "     */").unwrap();
        writeln!(
            out,
            "    public static String decrypt{field_pascal}({class_name} record) {{"
        )
        .unwrap();
        writeln!(
            out,
            "        if (record.get{field_pascal}() == null) return null;"
        )
        .unwrap();
        writeln!(
            out,
            "        return NexflowRuntime.decrypt(record.get{field_pascal}().toString(), \"{profile}\");"
        )
        .unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out).unwrap();
    }

    // Bulk encryptAll
    writeln!(out, "    /**").unwrap();
    writeln!(
        out,
        "     * Create a new record with all PII fields encrypted."
    )
    .unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static {class_name} encryptAll({class_name} record) {{"
    )
    .unwrap();
    writeln!(
        out,
        "        return {builder_name}.from(record)"
    )
    .unwrap();
    for f in &pii_fields {
        let java_name = to_camel_case(&f.name);
        let field_pascal = to_pascal_case(&f.name);
        writeln!(
            out,
            "            .{java_name}(encrypt{field_pascal}(record))"
        )
        .unwrap();
    }
    writeln!(out, "            .build();").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // Bulk decryptAll
    writeln!(out, "    /**").unwrap();
    writeln!(
        out,
        "     * Create a new record with all PII fields decrypted."
    )
    .unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static {class_name} decryptAll({class_name} record) {{"
    )
    .unwrap();
    writeln!(
        out,
        "        return {builder_name}.from(record)"
    )
    .unwrap();
    for f in &pii_fields {
        let java_name = to_camel_case(&f.name);
        let field_pascal = to_pascal_case(&f.name);
        writeln!(
            out,
            "            .{java_name}(decrypt{field_pascal}(record))"
        )
        .unwrap();
    }
    writeln!(out, "            .build();").unwrap();
    writeln!(out, "    }}").unwrap();

    // Close class
    writeln!(out, "}}").unwrap();

    Some((filename, out))
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

    fn make_schema_with_pii() -> SchemaDefinition {
        SchemaDefinition {
            imports: Vec::new(),
            name: "account_detail".to_string(),
            patterns: Vec::new(),
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: vec![make_field("account_id", "uuid", true)],
            streaming: None,
            serialization: None,
            fields: vec![
                make_pii_field("holder_name", "name"),
                make_pii_field("email", "email"),
                make_field("status", "string", true),
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
        }
    }

    #[test]
    fn test_has_pii_fields() {
        assert!(has_pii_fields(&make_schema_with_pii()));

        let no_pii = SchemaDefinition {
            fields: vec![make_field("name", "string", true)],
            ..make_schema_with_pii()
        };
        assert!(!has_pii_fields(&no_pii));
    }

    #[test]
    fn test_pii_helper_filename() {
        let result = generate_pii_helper(&make_schema_with_pii());
        let (filename, _) = result.unwrap();
        assert_eq!(filename, "AccountDetailPiiHelper.java");
    }

    #[test]
    fn test_pii_helper_structure() {
        let (_, content) = generate_pii_helper(&make_schema_with_pii()).unwrap();

        assert!(content.contains("public final class AccountDetailPiiHelper {"));
        assert!(content.contains("public static String encryptHolderName(AccountDetail record)"));
        assert!(content.contains("public static String decryptHolderName(AccountDetail record)"));
        assert!(content.contains("public static String encryptEmail(AccountDetail record)"));
        assert!(content.contains("public static String decryptEmail(AccountDetail record)"));
    }

    #[test]
    fn test_pii_helper_profiles() {
        let (_, content) = generate_pii_helper(&make_schema_with_pii()).unwrap();

        assert!(content.contains("NexflowRuntime.encrypt(record.getHolderName().toString(), \"name\")"));
        assert!(content.contains("NexflowRuntime.encrypt(record.getEmail().toString(), \"email\")"));
    }

    #[test]
    fn test_pii_helper_bulk_ops() {
        let (_, content) = generate_pii_helper(&make_schema_with_pii()).unwrap();

        assert!(content.contains("public static AccountDetail encryptAll(AccountDetail record)"));
        assert!(content.contains("public static AccountDetail decryptAll(AccountDetail record)"));
        assert!(content.contains("AccountDetailBuilder.from(record)"));
        assert!(content.contains(".holderName(encryptHolderName(record))"));
        assert!(content.contains(".email(encryptEmail(record))"));
    }

    #[test]
    fn test_no_pii_returns_none() {
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

        assert!(generate_pii_helper(&schema).is_none());
    }
}
