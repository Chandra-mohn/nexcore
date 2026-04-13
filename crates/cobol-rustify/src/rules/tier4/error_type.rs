//! T4-ERROR: Status code to error type transformation.
//!
//! Converts COBOL status-code patterns (set-then-check fields) into Rust
//! error types based on `[error_handling]` target config. Consumes Tier 3
//! status-to-result assessment data.

use std::fmt::Write;

use crate::target_config::{
    BinaryStatusTarget, ErrorStrategy, MultiStatusTarget, TargetConfig,
};

use super::structural::{CargoEdit, StructuralContext, StructuralPlan, StructuralRule};

#[derive(Debug)]
pub struct ErrorTypeRule;

impl StructuralRule for ErrorTypeRule {
    fn id(&self) -> &'static str {
        "t4-error"
    }

    fn description(&self) -> &'static str {
        "Convert status-code patterns to Rust error types"
    }

    fn plan(&self, ctx: &StructuralContext) -> StructuralPlan {
        let config = &ctx.target.error_handling;

        if config.strategy == ErrorStrategy::StatusQuo {
            return StructuralPlan::empty(self.id());
        }

        // Extract status field info from Tier 3 assessments
        let status_fields = extract_status_fields(ctx.assessments);
        if status_fields.is_empty() {
            return StructuralPlan::empty(self.id());
        }

        let mut plan = StructuralPlan::empty(self.id());
        let mut summary = String::new();

        // Generate error types
        if config.generate_program_error {
            let error_file =
                generate_error_module(&status_fields, config, ctx.target);
            plan.new_files
                .insert("src/error.rs".to_string(), error_file);
            let _ = writeln!(summary, "  Generated src/error.rs with error types");
        }

        // Rewrite status field usage in source files
        for (path, file) in ctx.files {
            let mut new_content = file.text.clone();
            let mut modified = false;

            for sf in &status_fields {
                if !new_content.contains(&format!("ws.{}", sf.name)) {
                    continue;
                }

                let rewritten = rewrite_status_usage(
                    &new_content,
                    sf,
                    config.binary_status,
                    config.multi_status,
                    config,
                );
                if let Some(r) = rewritten {
                    new_content = r;
                    modified = true;
                }
            }

            if modified {
                plan.modified_files
                    .insert(path.clone(), new_content);
                let _ = writeln!(summary, "  {path} -- rewrote status field usage");
            }
        }

        // Add dependencies
        match config.strategy {
            ErrorStrategy::Thiserror => {
                plan.cargo_edits.push(CargoEdit {
                    dependency: "thiserror".to_string(),
                    version: "1".to_string(),
                    features: vec![],
                });
            }
            ErrorStrategy::Anyhow => {
                plan.cargo_edits.push(CargoEdit {
                    dependency: "anyhow".to_string(),
                    version: "1".to_string(),
                    features: vec![],
                });
            }
            _ => {}
        }

        plan.description = format!(
            "Error types: {} status field(s), strategy={:?}",
            status_fields.len(),
            config.strategy
        );
        plan.summary = summary;
        plan
    }
}

/// Status field info extracted from Tier 3 assessments.
#[derive(Debug, Clone)]
struct StatusFieldInfo {
    name: String,
    values: Vec<String>,
    _check_count: usize,
}

/// Extract status field information from Tier 3 assessment descriptions.
fn extract_status_fields(
    assessments: &[crate::rules::transform::Transform],
) -> Vec<StatusFieldInfo> {
    let mut fields = Vec::new();

    for t in assessments {
        if t.rule_id != "status-to-result" {
            continue;
        }

        // Parse description format:
        // "field_name: status-code pattern (set to N value(s), checked M time(s))"
        let desc = &t.description;
        if let Some(colon) = desc.find(':') {
            let name = desc[..colon].trim().to_string();

            // Extract values from "-- values: [v1, v2, ...]"
            let values = if let Some(vals_start) = desc.find("values: [") {
                let after = &desc[vals_start + 9..];
                if let Some(end) = after.find(']') {
                    after[..end]
                        .split(", ")
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            // Extract check count
            let check_count = desc
                .find("checked ")
                .and_then(|pos| {
                    let after = &desc[pos + 8..];
                    after.split(' ').next()?.parse::<usize>().ok()
                })
                .unwrap_or(0);

            fields.push(StatusFieldInfo {
                name,
                values,
                _check_count: check_count,
            });
        }
    }

    fields
}

/// Generate an error module with status enums and error types.
fn generate_error_module(
    fields: &[StatusFieldInfo],
    config: &crate::target_config::ErrorHandlingConfig,
    _target: &TargetConfig,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "//! Generated by cobol2rust rustify T4-ERROR.");
    let _ = writeln!(out);

    let use_thiserror = config.strategy == ErrorStrategy::Thiserror;

    // Generate status enums for multi-value fields
    for sf in fields {
        if sf.values.len() >= 3 {
            let enum_name = field_to_enum_name(&sf.name);

            let _ = writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
            let _ = writeln!(out, "pub enum {enum_name} {{");

            // Use custom mappings if available
            let custom = config.mappings.get(&sf.name);

            for val in &sf.values {
                let variant = if let Some(mapping) = custom {
                    mapping
                        .values
                        .get(val)
                        .cloned()
                        .unwrap_or_else(|| value_to_variant(val))
                } else {
                    value_to_variant(val)
                };
                let _ = writeln!(out, "    {variant},");
            }
            let _ = writeln!(out, "}}");
            let _ = writeln!(out);

            // Generate Display impl
            let _ = writeln!(out, "impl std::fmt::Display for {enum_name} {{");
            let _ = writeln!(
                out,
                "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
            );
            let _ = writeln!(out, "        write!(f, \"{{:?}}\", self)");
            let _ = writeln!(out, "    }}");
            let _ = writeln!(out, "}}");
            let _ = writeln!(out);
        }
    }

    // Generate program error enum
    if config.generate_program_error {
        if use_thiserror {
            let _ = writeln!(out, "#[derive(Debug, thiserror::Error)]");
        } else {
            let _ = writeln!(out, "#[derive(Debug)]");
        }
        let _ = writeln!(out, "pub enum ProgramError {{");

        for sf in fields {
            let variant = field_to_variant_name(&sf.name);
            if sf.values.len() >= 3 {
                let enum_name = field_to_enum_name(&sf.name);
                if use_thiserror {
                    let _ = writeln!(
                        out,
                        "    #[error(\"{} error: {{0}}\")]",
                        sf.name
                    );
                }
                let _ = writeln!(out, "    {variant}({enum_name}),");
            } else if sf.values.len() == 2 {
                if use_thiserror {
                    let _ = writeln!(out, "    #[error(\"{} failed\")]", sf.name);
                }
                let _ = writeln!(out, "    {variant},");
            }
        }

        let _ = writeln!(out, "}}");

        // If not using thiserror, generate manual Display + Error impls
        if !use_thiserror {
            let _ = writeln!(out);
            let _ = writeln!(out, "impl std::fmt::Display for ProgramError {{");
            let _ = writeln!(
                out,
                "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
            );
            let _ = writeln!(out, "        write!(f, \"{{:?}}\", self)");
            let _ = writeln!(out, "    }}");
            let _ = writeln!(out, "}}");
            let _ = writeln!(out);
            let _ = writeln!(out, "impl std::error::Error for ProgramError {{}}");
        }
    }

    out
}

/// Rewrite status field usage (pack/check patterns) in source.
fn rewrite_status_usage(
    source: &str,
    sf: &StatusFieldInfo,
    binary: BinaryStatusTarget,
    _multi: MultiStatusTarget,
    _config: &crate::target_config::ErrorHandlingConfig,
) -> Option<String> {
    let mut result = source.to_string();
    let mut changed = false;

    if sf.values.len() == 2 {
        // Binary status field
        let success_val = &sf.values[0]; // Assume first value is success

        match binary {
            BinaryStatusTarget::Bool => {
                // Replace pack(success) with = true, pack(fail) with = false
                let success_pack = format!(
                    "ws.{}.pack(Decimal::from({success_val}))",
                    sf.name
                );
                let fail_pack = format!(
                    "ws.{}.pack(Decimal::from({}))",
                    sf.name, sf.values[1]
                );
                if result.contains(&success_pack) {
                    result = result.replace(&success_pack, &format!("ws.{} = true", sf.name));
                    changed = true;
                }
                if result.contains(&fail_pack) {
                    result = result.replace(&fail_pack, &format!("ws.{} = false", sf.name));
                    changed = true;
                }
            }
            BinaryStatusTarget::Result => {
                // Replace check with .is_ok() / .is_err()
                let check_eq = format!(
                    "ws.{}.to_decimal() == Decimal::from({success_val})",
                    sf.name
                );
                let check_ne = format!(
                    "ws.{}.to_decimal() != Decimal::from({success_val})",
                    sf.name
                );
                if result.contains(&check_eq) {
                    result = result.replace(
                        &check_eq,
                        &format!("ws.{}_result.is_ok()", sf.name),
                    );
                    changed = true;
                }
                if result.contains(&check_ne) {
                    result = result.replace(
                        &check_ne,
                        &format!("ws.{}_result.is_err()", sf.name),
                    );
                    changed = true;
                }
            }
            BinaryStatusTarget::Option => {
                // Replace with Some/None pattern
                let check_eq = format!(
                    "ws.{}.to_decimal() == Decimal::from({success_val})",
                    sf.name
                );
                if result.contains(&check_eq) {
                    result = result.replace(
                        &check_eq,
                        &format!("ws.{}_result.is_some()", sf.name),
                    );
                    changed = true;
                }
            }
        }
    }

    if changed { Some(result) } else { None }
}

/// Convert field name to enum name: ws_status -> Status, ws_err_code -> ErrCode.
fn field_to_enum_name(field: &str) -> String {
    let stripped = field.strip_prefix("ws_").unwrap_or(field);
    stripped
        .split('_')
        .map(|p| {
            let mut chars = p.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{upper}{}", chars.as_str())
                }
                None => String::new(),
            }
        })
        .collect::<String>()
        + "Status"
}

/// Convert field name to ProgramError variant name.
fn field_to_variant_name(field: &str) -> String {
    let stripped = field.strip_prefix("ws_").unwrap_or(field);
    stripped
        .split('_')
        .map(|p| {
            let mut chars = p.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{upper}{}", chars.as_str())
                }
                None => String::new(),
            }
        })
        .collect()
}

/// Convert a numeric value to a variant name.
fn value_to_variant(val: &str) -> String {
    match val {
        "0" => "Success".to_string(),
        "1" => "Error".to_string(),
        v => format!("Code{v}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::transform::{Safety, Transform};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_status_assessment() -> Vec<Transform> {
        vec![Transform {
            rule_id: "status-to-result".to_string(),
            file: PathBuf::from("src/main.rs"),
            line: 0,
            description:
                "ws_status: status-code pattern (set to 2 value(s), checked 3 time(s)) -- values: [0, 1]"
                    .to_string(),
            safety: Safety::Assessment,
            edits: vec![],
        }]
    }

    #[test]
    fn extract_status_fields_works() {
        let assessments = make_status_assessment();
        let fields = extract_status_fields(&assessments);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "ws_status");
        assert_eq!(fields[0].values, vec!["0", "1"]);
        assert_eq!(fields[0]._check_count, 3);
    }

    #[test]
    fn field_to_enum_name_works() {
        assert_eq!(field_to_enum_name("ws_status"), "StatusStatus");
        assert_eq!(field_to_enum_name("ws_err_code"), "ErrCodeStatus");
        assert_eq!(field_to_enum_name("result_flag"), "ResultFlagStatus");
    }

    #[test]
    fn status_quo_returns_empty() {
        let mut target = TargetConfig::default();
        target.error_handling.strategy = ErrorStrategy::StatusQuo;

        let files = HashMap::new();
        let rule = ErrorTypeRule;
        let ctx = StructuralContext {
            files: &files,
            hints: None,
            target: &target,
            assessments: &[],
        };

        let plan = rule.plan(&ctx);
        assert!(!plan.has_changes());
    }

    #[test]
    fn result_strategy_generates_error_file() {
        let mut target = TargetConfig::default();
        target.error_handling.strategy = ErrorStrategy::Result;

        let assessments = make_status_assessment();

        let files = HashMap::new();
        let rule = ErrorTypeRule;
        let ctx = StructuralContext {
            files: &files,
            hints: None,
            target: &target,
            assessments: &assessments,
        };

        let plan = rule.plan(&ctx);
        assert!(plan.has_changes());
        assert!(plan.new_files.contains_key("src/error.rs"));
        let error_content = &plan.new_files["src/error.rs"];
        assert!(error_content.contains("ProgramError"));
    }

    #[test]
    fn thiserror_strategy_adds_dependency() {
        let mut target = TargetConfig::default();
        target.error_handling.strategy = ErrorStrategy::Thiserror;

        let assessments = make_status_assessment();
        let files = HashMap::new();
        let rule = ErrorTypeRule;
        let ctx = StructuralContext {
            files: &files,
            hints: None,
            target: &target,
            assessments: &assessments,
        };

        let plan = rule.plan(&ctx);
        assert!(plan
            .cargo_edits
            .iter()
            .any(|e| e.dependency == "thiserror"));
    }

    #[test]
    fn multi_value_generates_enum() {
        let assessments = vec![Transform {
            rule_id: "status-to-result".to_string(),
            file: PathBuf::from("src/main.rs"),
            line: 0,
            description:
                "ws_err_code: status-code pattern (set to 3 value(s), checked 2 time(s)) -- values: [0, 1, 2]"
                    .to_string(),
            safety: Safety::Assessment,
            edits: vec![],
        }];

        let config = crate::target_config::ErrorHandlingConfig {
            strategy: ErrorStrategy::Thiserror,
            ..Default::default()
        };

        let fields = extract_status_fields(&assessments);
        let target = TargetConfig::default();
        let error_file = generate_error_module(&fields, &config, &target);
        assert!(error_file.contains("enum ErrCodeStatus"));
        assert!(error_file.contains("Success"));
        assert!(error_file.contains("Error"));
        assert!(error_file.contains("Code2"));
    }
}
