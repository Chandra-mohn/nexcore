// NexCore -- Nexflow Codegen: Java Migration Helper Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates `*Migration.java` classes for schemas with version/migration blocks.
//!
//! Provides version constants, compatibility checks, and migration helpers.

use std::fmt::Write;

use nexflow_parser::ast::schema::{CompatibilityMode, SchemaDefinition};

use super::naming::to_pascal_case;

const DEFAULT_PACKAGE: &str = "com.nexflow.schemas";

/// Check whether a schema needs a migration helper.
pub fn needs_migration(schema: &SchemaDefinition) -> bool {
    schema.version.is_some() || !schema.migration.is_empty()
}

/// Generate a Migration Java class for the given schema.
///
/// Returns `Some((filename, content))` if the schema has version info, `None` otherwise.
pub fn generate_migration(schema: &SchemaDefinition) -> Option<(String, String)> {
    if !needs_migration(schema) {
        return None;
    }

    let class_name = to_pascal_case(&schema.name);
    let migration_name = format!("{class_name}Migration");
    let filename = format!("{migration_name}.java");

    let mut out = String::with_capacity(2048);

    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/**").unwrap();
    writeln!(
        out,
        " * Schema migration helper for {{@link {class_name}}}."
    )
    .unwrap();
    writeln!(out, " * Generated from {}.schema -- DO NOT EDIT.", schema.name).unwrap();
    writeln!(out, " */").unwrap();
    writeln!(out, "public final class {migration_name} {{").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "    private {migration_name}() {{}}").unwrap();
    writeln!(out).unwrap();

    // Version constants
    if let Some(version) = &schema.version {
        let parts: Vec<&str> = version.number.split('.').collect();
        let major = parts.first().unwrap_or(&"1");
        let minor = parts.get(1).unwrap_or(&"0");
        let patch = parts.get(2).unwrap_or(&"0");

        writeln!(out, "    // =========================================================================").unwrap();
        writeln!(out, "    // Version Constants").unwrap();
        writeln!(out, "    // =========================================================================").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "    public static final int VERSION_MAJOR = {major};").unwrap();
        writeln!(out, "    public static final int VERSION_MINOR = {minor};").unwrap();
        writeln!(out, "    public static final int VERSION_PATCH = {patch};").unwrap();
        writeln!(
            out,
            "    public static final String VERSION_STRING = \"{}\";",
            version.number
        )
        .unwrap();

        // Compatibility mode
        let compat = version
            .compatibility
            .or(schema.compatibility)
            .unwrap_or(CompatibilityMode::Backward);
        let compat_str = match compat {
            CompatibilityMode::Backward => "backward",
            CompatibilityMode::Forward => "forward",
            CompatibilityMode::Full => "full",
            CompatibilityMode::None => "none",
        };
        writeln!(
            out,
            "    public static final String COMPATIBILITY_MODE = \"{compat_str}\";"
        )
        .unwrap();

        // Previous version
        if let Some(prev) = &version.previous_version {
            writeln!(
                out,
                "    public static final String PREVIOUS_VERSION = \"{prev}\";"
            )
            .unwrap();
        }
        writeln!(out).unwrap();

        // isCompatible method
        writeln!(out, "    /**").unwrap();
        writeln!(
            out,
            "     * Check if the given version is compatible with this schema version."
        )
        .unwrap();
        writeln!(out, "     * Compatibility mode: {compat_str}").unwrap();
        writeln!(out, "     */").unwrap();
        writeln!(
            out,
            "    public static boolean isCompatible(int otherMajor, int otherMinor) {{"
        )
        .unwrap();

        match compat {
            CompatibilityMode::Backward => {
                writeln!(
                    out,
                    "        return otherMajor == VERSION_MAJOR && otherMinor <= VERSION_MINOR;"
                )
                .unwrap();
            }
            CompatibilityMode::Forward => {
                writeln!(
                    out,
                    "        return otherMajor == VERSION_MAJOR && otherMinor >= VERSION_MINOR;"
                )
                .unwrap();
            }
            CompatibilityMode::Full => {
                writeln!(
                    out,
                    "        return otherMajor == VERSION_MAJOR;"
                )
                .unwrap();
            }
            CompatibilityMode::None => {
                writeln!(
                    out,
                    "        return otherMajor == VERSION_MAJOR && otherMinor == VERSION_MINOR;"
                )
                .unwrap();
            }
        }
        writeln!(out, "    }}").unwrap();
        writeln!(out).unwrap();

        // canMigrate method
        writeln!(out, "    /**").unwrap();
        writeln!(
            out,
            "     * Check if migration is possible from the given version."
        )
        .unwrap();
        writeln!(out, "     */").unwrap();
        writeln!(
            out,
            "    public static boolean canMigrate(int fromMajor, int fromMinor) {{"
        )
        .unwrap();
        writeln!(
            out,
            "        return fromMajor == VERSION_MAJOR && fromMinor < VERSION_MINOR;"
        )
        .unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out).unwrap();

        // getVersionString
        writeln!(out, "    /**").unwrap();
        writeln!(out, "     * Get the full version string.").unwrap();
        writeln!(out, "     */").unwrap();
        writeln!(out, "    public static String getVersionString() {{").unwrap();
        writeln!(out, "        return VERSION_STRING;").unwrap();
        writeln!(out, "    }}").unwrap();
    }

    // Close class
    writeln!(out, "}}").unwrap();

    Some((filename, out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::schema::*;

    fn make_versioned_schema() -> SchemaDefinition {
        SchemaDefinition {
            imports: Vec::new(),
            name: "account_detail".to_string(),
            patterns: Vec::new(),
            targets: Vec::new(),
            version: Some(VersionInfo {
                number: "2.1.0".to_string(),
                compatibility: Some(CompatibilityMode::Backward),
                previous_version: Some("2.0.0".to_string()),
                deprecation: None,
                migration_guide: None,
            }),
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: Vec::new(),
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
    fn test_needs_migration() {
        assert!(needs_migration(&make_versioned_schema()));

        let no_version = SchemaDefinition {
            version: None,
            ..make_versioned_schema()
        };
        assert!(!needs_migration(&no_version));
    }

    #[test]
    fn test_migration_filename() {
        let (filename, _) = generate_migration(&make_versioned_schema()).unwrap();
        assert_eq!(filename, "AccountDetailMigration.java");
    }

    #[test]
    fn test_migration_version_constants() {
        let (_, content) = generate_migration(&make_versioned_schema()).unwrap();

        assert!(content.contains("VERSION_MAJOR = 2;"));
        assert!(content.contains("VERSION_MINOR = 1;"));
        assert!(content.contains("VERSION_PATCH = 0;"));
        assert!(content.contains("VERSION_STRING = \"2.1.0\""));
        assert!(content.contains("COMPATIBILITY_MODE = \"backward\""));
        assert!(content.contains("PREVIOUS_VERSION = \"2.0.0\""));
    }

    #[test]
    fn test_migration_backward_compat() {
        let (_, content) = generate_migration(&make_versioned_schema()).unwrap();

        assert!(content.contains("otherMajor == VERSION_MAJOR && otherMinor <= VERSION_MINOR"));
    }

    #[test]
    fn test_migration_forward_compat() {
        let mut schema = make_versioned_schema();
        schema.version.as_mut().unwrap().compatibility = Some(CompatibilityMode::Forward);

        let (_, content) = generate_migration(&schema).unwrap();
        assert!(content.contains("otherMajor == VERSION_MAJOR && otherMinor >= VERSION_MINOR"));
    }

    #[test]
    fn test_migration_none_compat() {
        let mut schema = make_versioned_schema();
        schema.version.as_mut().unwrap().compatibility = Some(CompatibilityMode::None);

        let (_, content) = generate_migration(&schema).unwrap();
        assert!(content.contains("otherMajor == VERSION_MAJOR && otherMinor == VERSION_MINOR"));
    }

    #[test]
    fn test_no_version_returns_none() {
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
            fields: Vec::new(),
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

        assert!(generate_migration(&schema).is_none());
    }
}
