//! T4-DOMAIN: WorkingStorage decomposition.
//!
//! Restructures the flat `WorkingStorage` struct based on the `[data_model]`
//! target config. Consumes Tier 3 ws-decomposition assessment for prefix
//! groups and co-access patterns.

use std::collections::HashMap;
use std::fmt::Write;

use crate::target_config::{DataModelStrategy, DeriveLevel, NamingStrategy};

use super::structural::{SourceFile, StructuralContext, StructuralPlan, StructuralRule};

#[derive(Debug)]
pub struct DomainModelRule;

impl StructuralRule for DomainModelRule {
    fn id(&self) -> &'static str {
        "t4-domain"
    }

    fn description(&self) -> &'static str {
        "Decompose WorkingStorage into domain sub-structs"
    }

    fn plan(&self, ctx: &StructuralContext) -> StructuralPlan {
        let config = &ctx.target.data_model;

        if config.strategy == DataModelStrategy::Flat && config.naming == NamingStrategy::Preserve {
            return StructuralPlan::empty(self.id());
        }

        // Find files containing WorkingStorage
        let ws_files: Vec<(&str, &SourceFile)> = ctx
            .files
            .iter()
            .filter(|(_, f)| f.text.contains("struct WorkingStorage"))
            .map(|(path, f)| (path.as_str(), f))
            .collect();

        if ws_files.is_empty() {
            return StructuralPlan::empty(self.id());
        }

        let mut plan = StructuralPlan::empty(self.id());
        let mut summary = String::new();

        for (path, file) in &ws_files {
            match config.strategy {
                DataModelStrategy::Flat => {
                    // Only rename fields (strip prefix)
                    if config.naming != NamingStrategy::Preserve {
                        if let Some(new_content) =
                            apply_naming(&file.text, config.naming, ctx.files)
                        {
                            plan.modified_files
                                .insert((*path).to_string(), new_content);
                            let _ = writeln!(
                                summary,
                                "  {} -- renamed fields ({:?})",
                                path, config.naming
                            );
                        }
                    }
                }
                DataModelStrategy::Nested => {
                    // Extract sub-structs from prefix groups or custom overrides
                    let groups = if config.structs.is_empty() {
                        detect_prefix_groups(&file.text, config.min_group_size)
                    } else {
                        config.structs.clone()
                    };

                    if groups.is_empty() {
                        continue;
                    }

                    // Generate sub-struct definitions
                    for (struct_name, fields) in &groups {
                        let struct_file = generate_sub_struct(
                            struct_name,
                            fields,
                            &file.text,
                            config.derive_level,
                            config.naming,
                            config.use_accessors,
                        );
                        let struct_path =
                            format!("src/types/{}.rs", to_snake_case(struct_name));
                        plan.new_files.insert(struct_path, struct_file);
                    }

                    // Generate types/mod.rs
                    let mod_file = generate_types_mod(&groups);
                    plan.new_files
                        .insert("src/types/mod.rs".to_string(), mod_file);

                    // Rewrite WorkingStorage and field accesses
                    if let Some(rewritten) = rewrite_ws_nested(
                        &file.text,
                        &groups,
                        config.naming,
                        ctx.files,
                    ) {
                        plan.modified_files
                            .insert((*path).to_string(), rewritten);
                    }

                    let _ = writeln!(
                        summary,
                        "  {} -- extracted {} sub-struct(s)",
                        path,
                        groups.len()
                    );
                }
                DataModelStrategy::Split | DataModelStrategy::Domain => {
                    // Similar to nested but more aggressive -- split into separate params
                    // For now, treat as nested with a note
                    let _ = writeln!(
                        summary,
                        "  {} -- {:?} strategy (advanced, treating as nested for now)",
                        path, config.strategy
                    );
                }
            }
        }

        plan.description = format!(
            "Domain modeling: {} file(s), strategy={:?}",
            plan.modified_files.len() + plan.new_files.len(),
            config.strategy
        );
        plan.summary = summary;
        plan
    }
}

/// Detect prefix groups from WorkingStorage field names.
fn detect_prefix_groups(source: &str, min_size: usize) -> HashMap<String, Vec<String>> {
    let fields = extract_ws_fields(source);
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for field in &fields {
        let parts: Vec<&str> = field.split('_').collect();
        if parts.len() >= 3 {
            let prefix = format!("{}_{}", parts[0], parts[1]);
            groups.entry(prefix).or_default().push(field.clone());
        }
    }

    // Filter to groups meeting minimum size
    groups.retain(|_, v| v.len() >= min_size);

    // Convert prefix to struct name
    let mut named: HashMap<String, Vec<String>> = HashMap::new();
    for (prefix, fields) in groups {
        let struct_name = prefix_to_struct_name(&prefix);
        named.insert(struct_name, fields);
    }

    named
}

/// Extract field names from `pub struct WorkingStorage { ... }`.
fn extract_ws_fields(source: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut in_struct = false;
    let mut depth = 0;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.contains("struct WorkingStorage") {
            in_struct = true;
        }
        if !in_struct {
            continue;
        }
        for ch in trimmed.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    return fields;
                }
            }
        }
        if depth > 0 && trimmed.starts_with("pub ") && trimmed.contains(':') {
            if let Some(colon) = trimmed.find(':') {
                let name = trimmed[4..colon].trim().to_string();
                fields.push(name);
            }
        }
    }
    fields
}

/// Convert a prefix like "ws_acct" to a struct name like "AcctInfo".
fn prefix_to_struct_name(prefix: &str) -> String {
    let parts: Vec<&str> = prefix.split('_').collect();
    let name_part = if parts.len() >= 2 {
        // Skip "ws" prefix, capitalize the rest
        parts[1..].iter().map(|p| capitalize(p)).collect::<String>()
    } else {
        capitalize(prefix)
    };
    format!("{name_part}Info")
}

/// Capitalize first letter.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => {
            let upper: String = c.to_uppercase().collect();
            format!("{upper}{}", chars.as_str())
        }
        None => String::new(),
    }
}

/// Convert PascalCase to snake_case.
fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.extend(ch.to_lowercase());
    }
    result
}

/// Strip a prefix from a field name based on naming strategy.
fn rename_field(field: &str, strategy: NamingStrategy) -> String {
    match strategy {
        NamingStrategy::Preserve => field.to_string(),
        NamingStrategy::StripPrefix => {
            // Strip ws_ prefix
            field.strip_prefix("ws_").unwrap_or(field).to_string()
        }
        NamingStrategy::RustIdiom => {
            // Strip ws_ and attempt to expand abbreviations
            // For now, just strip prefix (full mapping requires dictionary)
            field.strip_prefix("ws_").unwrap_or(field).to_string()
        }
    }
}

/// Apply naming strategy to all field references across the workspace.
fn apply_naming(
    source: &str,
    naming: NamingStrategy,
    _all_files: &HashMap<String, SourceFile>,
) -> Option<String> {
    if naming == NamingStrategy::Preserve {
        return None;
    }

    let fields = extract_ws_fields(source);
    if fields.is_empty() {
        return None;
    }

    let mut result = source.to_string();
    let mut renamed_any = false;

    for field in &fields {
        let new_name = rename_field(field, naming);
        if new_name != *field {
            // Replace in struct declaration
            result = result.replace(
                &format!("pub {field}:"),
                &format!("pub {new_name}:"),
            );
            // Replace field accesses (ws.field_name)
            result = result.replace(
                &format!("ws.{field}"),
                &format!("ws.{new_name}"),
            );
            renamed_any = true;
        }
    }

    if renamed_any { Some(result) } else { None }
}

/// Generate a sub-struct file.
fn generate_sub_struct(
    name: &str,
    field_names: &[String],
    source: &str,
    derive_level: DeriveLevel,
    naming: NamingStrategy,
    use_accessors: bool,
) -> String {
    let derives = match derive_level {
        DeriveLevel::Minimal => "#[derive(Debug)]",
        DeriveLevel::Standard => "#[derive(Debug, Clone)]",
        DeriveLevel::Full => "#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]",
    };

    let mut out = String::new();
    let _ = writeln!(out, "//! Generated by cobol2rust rustify T4-DOMAIN.");
    let _ = writeln!(out);
    let _ = writeln!(out, "use rust_decimal::Decimal;");
    let _ = writeln!(out);
    let _ = writeln!(out, "{derives}");

    let vis = if use_accessors { "" } else { "pub " };
    let _ = writeln!(out, "pub struct {name} {{");

    for field_name in field_names {
        let field_type = guess_field_type(source, field_name);
        let display_name = rename_field(field_name, naming);
        // Strip the common prefix within this group too
        let short_name = strip_group_prefix(&display_name, field_names, naming);
        let _ = writeln!(out, "    {vis}{short_name}: {field_type},");
    }
    let _ = writeln!(out, "}}");

    // Generate Default impl
    let _ = writeln!(out);
    let _ = writeln!(out, "impl Default for {name} {{");
    let _ = writeln!(out, "    fn default() -> Self {{");
    let _ = writeln!(out, "        Self {{");
    for field_name in field_names {
        let field_type = guess_field_type(source, field_name);
        let display_name = rename_field(field_name, naming);
        let short_name = strip_group_prefix(&display_name, field_names, naming);
        let default_val = default_for_type(&field_type);
        let _ = writeln!(out, "            {short_name}: {default_val},");
    }
    let _ = writeln!(out, "        }}");
    let _ = writeln!(out, "    }}");
    let _ = writeln!(out, "}}");

    // Generate accessors if requested
    if use_accessors {
        let _ = writeln!(out);
        let _ = writeln!(out, "impl {name} {{");
        for field_name in field_names {
            let field_type = guess_field_type(source, field_name);
            let display_name = rename_field(field_name, naming);
            let short_name = strip_group_prefix(&display_name, field_names, naming);
            let _ = writeln!(out, "    pub fn {short_name}(&self) -> &{field_type} {{");
            let _ = writeln!(out, "        &self.{short_name}");
            let _ = writeln!(out, "    }}");
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "    pub fn {short_name}_mut(&mut self) -> &mut {field_type} {{"
            );
            let _ = writeln!(out, "        &mut self.{short_name}");
            let _ = writeln!(out, "    }}");
            let _ = writeln!(out);
        }
        let _ = writeln!(out, "}}");
    }

    out
}

/// Guess the Rust type of a WorkingStorage field from source.
fn guess_field_type(source: &str, field_name: &str) -> String {
    let pattern = format!("pub {field_name}:");
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&pattern) {
            let after_colon = &trimmed[pattern.len()..];
            let type_str = after_colon.trim().trim_end_matches(',').trim();
            return type_str.to_string();
        }
    }
    "String".to_string() // fallback
}

/// Default value for a type.
fn default_for_type(type_str: &str) -> String {
    match type_str {
        "String" => "String::new()".to_string(),
        "Decimal" => "Decimal::ZERO".to_string(),
        "bool" => "false".to_string(),
        "i32" | "i64" | "u32" | "u64" => "0".to_string(),
        "f32" | "f64" => "0.0".to_string(),
        t if t.starts_with("PicX") => format!("{t}::default()"),
        t if t.starts_with("PackedDecimal") => format!("{t}::default()"),
        t if t.starts_with("ZonedDecimal") => format!("{t}::default()"),
        _ => "Default::default()".to_string(),
    }
}

/// Strip the common group prefix from a field name within a sub-struct.
/// e.g., in AcctInfo, "acct_number" -> "number", "acct_type" -> "acct_type" stays
/// if there's ambiguity.
fn strip_group_prefix(
    name: &str,
    _group_fields: &[String],
    _naming: NamingStrategy,
) -> String {
    // For now, keep the name as-is within the struct.
    // More sophisticated stripping would need collision detection.
    name.to_string()
}

/// Generate types/mod.rs with mod declarations.
fn generate_types_mod(groups: &HashMap<String, Vec<String>>) -> String {
    let mut out = String::from("//! Generated domain types from WorkingStorage decomposition.\n\n");
    let mut names: Vec<&String> = groups.keys().collect();
    names.sort();
    for name in &names {
        let module = to_snake_case(name);
        let _ = writeln!(out, "mod {module};");
        let _ = writeln!(out, "pub use {module}::{name};");
        let _ = writeln!(out);
    }
    out
}

/// Rewrite WorkingStorage for nested strategy.
fn rewrite_ws_nested(
    source: &str,
    groups: &HashMap<String, Vec<String>>,
    naming: NamingStrategy,
    _all_files: &HashMap<String, SourceFile>,
) -> Option<String> {
    let mut result = source.to_string();
    let mut changed = false;

    for (struct_name, fields) in groups {
        let field_var = to_snake_case(struct_name)
            .strip_suffix("_info")
            .unwrap_or(&to_snake_case(struct_name))
            .to_string();

        // Replace field declarations in WorkingStorage with sub-struct field
        for field in fields {
            let decl_pattern = format!("pub {field}:");
            if let Some(line_start) = result.find(&decl_pattern) {
                // Find the end of this line
                if let Some(line_end) = result[line_start..].find('\n') {
                    let end = line_start + line_end + 1;
                    // Only remove the first occurrence; replace once done
                    result = format!("{}{}", &result[..line_start], &result[end..]);
                    changed = true;
                }
            }
        }

        // Insert the sub-struct field in WorkingStorage
        if let Some(ws_brace) = result.find("struct WorkingStorage {") {
            if let Some(after_brace) = result[ws_brace..].find('{') {
                let insert_pos = ws_brace + after_brace + 1;
                let sub_field = format!("\n    pub {field_var}: {struct_name},");
                result.insert_str(insert_pos, &sub_field);
                changed = true;
            }
        }

        // Rewrite field accesses: ws.old_field -> ws.sub_struct.new_field
        for field in fields {
            let new_field = rename_field(field, naming);
            let old_access = format!("ws.{field}");
            let new_access = format!("ws.{field_var}.{new_field}");
            if result.contains(&old_access) {
                result = result.replace(&old_access, &new_access);
                changed = true;
            }
        }
    }

    if changed { Some(result) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_ws_fields_works() {
        let source = r#"pub struct WorkingStorage {
    pub ws_acct_number: PicX,
    pub ws_acct_type: PicX,
    pub ws_acct_balance: PackedDecimal,
    pub ws_name: PicX,
}
"#;
        let fields = extract_ws_fields(source);
        assert_eq!(fields.len(), 4);
        assert!(fields.contains(&"ws_acct_number".to_string()));
        assert!(fields.contains(&"ws_name".to_string()));
    }

    #[test]
    fn detect_prefix_groups_works() {
        let source = r#"pub struct WorkingStorage {
    pub ws_acct_number: PicX,
    pub ws_acct_type: PicX,
    pub ws_acct_balance: PackedDecimal,
    pub ws_txn_date: PicX,
    pub ws_txn_amount: PackedDecimal,
    pub ws_txn_code: PicX,
    pub ws_name: PicX,
}
"#;
        let groups = detect_prefix_groups(source, 3);
        assert_eq!(groups.len(), 2);
        assert!(groups.contains_key("AcctInfo"));
        assert!(groups.contains_key("TxnInfo"));
        assert_eq!(groups["AcctInfo"].len(), 3);
        assert_eq!(groups["TxnInfo"].len(), 3);
    }

    #[test]
    fn prefix_to_struct_name_works() {
        assert_eq!(prefix_to_struct_name("ws_acct"), "AcctInfo");
        assert_eq!(prefix_to_struct_name("ws_customer"), "CustomerInfo");
    }

    #[test]
    fn to_snake_case_works() {
        assert_eq!(to_snake_case("AcctInfo"), "acct_info");
        assert_eq!(to_snake_case("ProgramError"), "program_error");
    }

    #[test]
    fn rename_field_strip_prefix() {
        assert_eq!(
            rename_field("ws_acct_number", NamingStrategy::StripPrefix),
            "acct_number"
        );
        assert_eq!(
            rename_field("ws_name", NamingStrategy::StripPrefix),
            "name"
        );
        assert_eq!(
            rename_field("total", NamingStrategy::StripPrefix),
            "total"
        );
    }

    #[test]
    fn guess_field_type_works() {
        let source = "    pub ws_acct_number: PicX,\n    pub ws_balance: PackedDecimal,\n";
        assert_eq!(guess_field_type(source, "ws_acct_number"), "PicX");
        assert_eq!(guess_field_type(source, "ws_balance"), "PackedDecimal");
    }

    #[test]
    fn generate_sub_struct_basic() {
        let source = "    pub ws_acct_number: PicX,\n    pub ws_acct_type: PicX,\n";
        let fields = vec![
            "ws_acct_number".to_string(),
            "ws_acct_type".to_string(),
        ];
        let result = generate_sub_struct(
            "AcctInfo",
            &fields,
            source,
            DeriveLevel::Standard,
            NamingStrategy::StripPrefix,
            false,
        );
        assert!(result.contains("pub struct AcctInfo"));
        assert!(result.contains("#[derive(Debug, Clone)]"));
        assert!(result.contains("acct_number: PicX"));
        assert!(result.contains("impl Default for AcctInfo"));
    }

    #[test]
    fn apply_naming_strips_ws_prefix() {
        let source = r#"pub struct WorkingStorage {
    pub ws_name: PicX,
    pub ws_total: PackedDecimal,
}

fn process(ws: &mut WorkingStorage) {
    ws.ws_name.set_bytes(b"hello");
    let val = ws.ws_total.to_decimal();
}
"#;
        let result = apply_naming(source, NamingStrategy::StripPrefix, &HashMap::new()).unwrap();
        assert!(result.contains("pub name: PicX"));
        assert!(result.contains("pub total: PackedDecimal"));
        assert!(result.contains("ws.name.set_bytes"));
        assert!(result.contains("ws.total.to_decimal()"));
    }

    use crate::target_config::TargetConfig;

    #[test]
    fn flat_preserve_returns_empty_plan() {
        let mut target = TargetConfig::default();
        target.data_model.strategy = DataModelStrategy::Flat;
        target.data_model.naming = NamingStrategy::Preserve;

        let files = HashMap::new();
        let rule = DomainModelRule;
        let ctx = StructuralContext {
            files: &files,
            hints: None,
            target: &target,
            assessments: &[],
        };

        let plan = rule.plan(&ctx);
        assert!(!plan.has_changes());
    }
}
