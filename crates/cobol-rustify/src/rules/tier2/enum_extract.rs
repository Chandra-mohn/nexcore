//! T2-ENUM: Extract enums from exhaustive level-88 condition groups.
//!
//! When a COBOL field has multiple level-88 conditions that cover all
//! possible values (`exhaustive: true`), it can be promoted to a Rust enum.
//!
//! Transforms applied:
//! - Generate `#[derive(Debug, Clone, PartialEq)] enum FieldEnum { Variant1, ... }`
//! - Struct field type: `PicX` -> `FieldEnum`
//! - Initialization: `PicX::new(...)` -> `FieldEnum::Default`
//! - Condition checks: `field.as_bytes() == b"X"` -> `field == FieldEnum::VariantX`
//! - Assignments: `move_alphanumeric_literal(b"X", ...)` -> `field = FieldEnum::VariantX`

use crate::hints::FileHints;
use crate::rules::transform::{Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};
use crate::safety;

#[derive(Debug)]
pub struct EnumExtractRule;

impl RustifyRule for EnumExtractRule {
    fn id(&self) -> &'static str {
        "enum-extract"
    }

    fn description(&self) -> &'static str {
        "Extract enums from exhaustive level-88 groups (safety-gated)"
    }

    fn tier(&self) -> Tier {
        Tier::Tier2
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        let Some(hints) = ctx.hints else {
            return Vec::new();
        };
        find_enum_candidates(hints, ctx.source_text, ctx.file_path)
    }
}

/// An enum candidate with its variants.
struct EnumCandidate {
    field_name: String,
    enum_name: String,
    /// (condition_name, value, variant_name)
    variants: Vec<(String, String, String)>,
}

/// Find fields eligible for enum extraction.
fn find_enum_candidates(
    hints: &FileHints,
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let mut transforms = Vec::new();
    let lines: Vec<&str> = source_text.lines().collect();

    let candidates = collect_candidates(hints);

    for candidate in &candidates {
        let Some(hint) = hints.fields.get(&candidate.field_name) else {
            continue;
        };

        let gates = safety::check_safety_gates(&candidate.field_name, hint, hints);

        if !gates.is_empty() {
            let reasons: Vec<String> = gates.iter().map(|g| g.reason.to_string()).collect();
            let recommendations: Vec<String> =
                gates.iter().map(|g| g.recommendation.clone()).collect();
            transforms.push(Transform {
                rule_id: "enum-extract".to_string(),
                file: file_path.to_path_buf(),
                line: 0,
                description: format!(
                    "{}: PicX -> {} (BLOCKED)",
                    candidate.field_name, candidate.enum_name
                ),
                safety: Safety::Review {
                    reason: reasons.join("; "),
                    recommendation: recommendations.join("; "),
                },
                edits: vec![],
            });
            continue;
        }

        let mut edits = Vec::new();

        // 1. Type declaration: PicX -> EnumName
        if let Some(line_num) = find_field_decl(&lines, &candidate.field_name, "PicX") {
            if let Some(edit) = make_type_edit(&lines, line_num, "PicX", &candidate.enum_name) {
                edits.push(edit);
            }
        }

        // 2. Initialization: PicX::new(...) -> EnumName::DefaultVariant
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_enum_init(line, i + 1, candidate) {
                edits.push(edit);
            }
        }

        // 3. Condition checks
        for (i, line) in lines.iter().enumerate() {
            edits.extend(rewrite_enum_comparisons(line, i + 1, candidate));
        }

        // 4. Assignments
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_enum_assignment(line, i + 1, candidate) {
                edits.push(edit);
            }
        }

        if !edits.is_empty() {
            let variant_names: Vec<&str> =
                candidate.variants.iter().map(|(_, _, v)| v.as_str()).collect();
            transforms.push(Transform {
                rule_id: "enum-extract".to_string(),
                file: file_path.to_path_buf(),
                line: find_field_decl(&lines, &candidate.field_name, "PicX").unwrap_or(0),
                description: format!(
                    "{}: PicX -> {} {{ {} }}",
                    candidate.field_name,
                    candidate.enum_name,
                    variant_names.join(", ")
                ),
                safety: Safety::Auto,
                edits,
            });
        }
    }

    transforms.sort_by(|a, b| a.description.cmp(&b.description));
    transforms
}

/// Collect enum candidates from hints.
fn collect_candidates(hints: &FileHints) -> Vec<EnumCandidate> {
    let mut candidates = Vec::new();

    for (field_name, group) in &hints.level_88_groups {
        // Must be exhaustive
        if !group.exhaustive {
            continue;
        }

        // Must have 2+ conditions (1 condition is bool-extract territory)
        if group.conditions.len() < 2 {
            continue;
        }

        // Field must exist in hints
        let Some(_hint) = hints.fields.get(field_name) else {
            continue;
        };

        let enum_name = field_to_enum_name(field_name);

        // Sort variants deterministically
        let mut variants: Vec<(String, String, String)> = group
            .conditions
            .iter()
            .map(|(cond_name, value)| {
                let variant = condition_to_variant(cond_name);
                (cond_name.clone(), value.clone(), variant)
            })
            .collect();
        variants.sort_by(|a, b| a.0.cmp(&b.0));

        candidates.push(EnumCandidate {
            field_name: field_name.clone(),
            enum_name,
            variants,
        });
    }

    candidates
}

/// Convert a field name to an enum name.
/// e.g., "ws_status" -> "WsStatus", "ws_acct_type" -> "WsAcctType"
fn field_to_enum_name(field_name: &str) -> String {
    field_name
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
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

/// Convert a condition name to a variant name.
/// e.g., "is_active" -> "Active", "status_pending" -> "Pending"
fn condition_to_variant(name: &str) -> String {
    // Strip common prefixes
    let stripped = name
        .strip_prefix("is_")
        .or_else(|| name.strip_prefix("ws_"))
        .unwrap_or(name);

    stripped
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
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

/// Find struct field declaration line.
fn find_field_decl(lines: &[&str], field_name: &str, type_name: &str) -> Option<usize> {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.contains(&format!("{field_name}: {type_name}"))
            && !trimmed.contains("::new(")
        {
            return Some(i + 1);
        }
    }
    None
}

/// Create a TextEdit to change the type name.
fn make_type_edit(
    lines: &[&str],
    line_num: usize,
    old_type: &str,
    new_type: &str,
) -> Option<TextEdit> {
    let line = lines.get(line_num - 1)?;
    let col = line.find(old_type)?;
    Some(TextEdit {
        line: line_num,
        col_start: col,
        col_end: col + old_type.len(),
        new_text: new_type.to_string(),
    })
}

/// Rewrite initialization to use enum variant.
fn rewrite_enum_init(line: &str, line_num: usize, candidate: &EnumCandidate) -> Option<TextEdit> {
    if !line.contains(&candidate.field_name) {
        return None;
    }

    let marker = "PicX::new(";
    let start = line.find(marker)?;
    let rest = &line[start + marker.len()..];
    let close = find_matching_paren(rest)?;
    let end = start + marker.len() + close + 1;
    let inner = &rest[..close];

    // Extract the init value
    let init_val = extract_byte_string_value(inner)?;

    // Find matching variant
    let variant = match candidate.variants.iter().find(|(_, v, _)| *v == init_val) {
        Some((_, _, variant)) => variant.clone(),
        None => candidate
            .variants
            .first()
            .map_or_else(|| "Unknown".to_string(), |(_, _, v)| v.clone()),
    };

    Some(TextEdit {
        line: line_num,
        col_start: start,
        col_end: end,
        new_text: format!("{}::{variant}", candidate.enum_name),
    })
}

/// Rewrite condition comparisons to use enum variants.
fn rewrite_enum_comparisons(
    line: &str,
    line_num: usize,
    candidate: &EnumCandidate,
) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    for (_, value, variant) in &candidate.variants {
        let eq_pattern = format!(
            "ws.{}.as_bytes() == b\"{}\"",
            candidate.field_name, value
        );
        let ne_pattern = format!(
            "ws.{}.as_bytes() != b\"{}\"",
            candidate.field_name, value
        );

        if let Some(start) = line.find(&eq_pattern) {
            edits.push(TextEdit {
                line: line_num,
                col_start: start,
                col_end: start + eq_pattern.len(),
                new_text: format!(
                    "ws.{} == {}::{variant}",
                    candidate.field_name, candidate.enum_name
                ),
            });
        } else if let Some(start) = line.find(&ne_pattern) {
            edits.push(TextEdit {
                line: line_num,
                col_start: start,
                col_end: start + ne_pattern.len(),
                new_text: format!(
                    "ws.{} != {}::{variant}",
                    candidate.field_name, candidate.enum_name
                ),
            });
        }
    }

    edits
}

/// Rewrite assignment to use enum variant.
fn rewrite_enum_assignment(
    line: &str,
    line_num: usize,
    candidate: &EnumCandidate,
) -> Option<TextEdit> {
    let trimmed = line.trim();
    let prefix = "move_alphanumeric_literal(b\"";
    if !trimmed.starts_with(prefix) {
        return None;
    }

    let ws_field = format!("&mut ws.{}", candidate.field_name);
    if !trimmed.contains(&ws_field) {
        return None;
    }

    let after_b = &trimmed[prefix.len()..];
    let quote_end = after_b.find('"')?;
    let val = &after_b[..quote_end];

    let variant = candidate
        .variants
        .iter()
        .find(|(_, v, _)| v == val)
        .map(|(_, _, variant)| variant.clone())?;

    let indent = line.len() - trimmed.len();
    let stmt_end = line.trim_end().len();

    Some(TextEdit {
        line: line_num,
        col_start: indent,
        col_end: stmt_end,
        new_text: format!(
            "ws.{} = {}::{variant};",
            candidate.field_name, candidate.enum_name
        ),
    })
}

/// Generate the enum definition text (for insertion before WorkingStorage).
pub fn generate_enum_definition(candidate_desc: &str, hints: &FileHints) -> Option<String> {
    // Parse candidate description to find field name and enum name
    // Format: "field_name: PicX -> EnumName { Var1, Var2 }"
    let colon_pos = candidate_desc.find(':')?;
    let field_name = &candidate_desc[..colon_pos];

    let group = hints.level_88_groups.get(field_name)?;
    let enum_name = field_to_enum_name(field_name);

    let mut variants: Vec<String> = group
        .conditions
        .keys()
        .map(|name| condition_to_variant(name))
        .collect();
    variants.sort();

    let mut def = String::new();
    def.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    def.push_str("pub enum ");
    def.push_str(&enum_name);
    def.push_str(" {\n");
    for variant in &variants {
        def.push_str("    ");
        def.push_str(variant);
        def.push_str(",\n");
    }
    def.push_str("}\n");

    Some(def)
}

/// Find matching closing parenthesis.
fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 1;
    let mut in_string = false;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' if !in_string => in_string = true,
            b'"' if in_string => {
                if i > 0 && bytes[i - 1] != b'\\' {
                    in_string = false;
                }
            }
            b'(' if !in_string => depth += 1,
            b')' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// Extract string value from byte literal.
fn extract_byte_string_value(inner: &str) -> Option<String> {
    let b_start = inner.find("b\"")?;
    let val_start = b_start + 2;
    let rest = &inner[val_start..];
    let val_end = rest.find('"')?;
    Some(rest[..val_end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hints::{FieldHint, FileHints, Level88Group};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_enum_hints(field_name: &str, conditions: Vec<(&str, &str)>) -> FileHints {
        let mut fields = HashMap::new();
        fields.insert(
            field_name.to_string(),
            FieldHint {
                pic: "X(1)".to_string(),
                usage: "DISPLAY".to_string(),
                level: 5,
                redefines: None,
                redefined_by: vec![],
                call_by_ref: false,
                move_corr_target: false,
                read_count: 10,
                write_count: 3,
                paragraph_scope: vec![],
            },
        );

        let cond_map: HashMap<String, String> = conditions
            .iter()
            .map(|(name, val)| (name.to_string(), val.to_string()))
            .collect();

        let mut level_88_groups = HashMap::new();
        level_88_groups.insert(
            field_name.to_string(),
            Level88Group {
                conditions: cond_map,
                exhaustive: true,
            },
        );

        FileHints {
            cobol_source: "TEST.CBL".to_string(),
            fields,
            paragraphs: HashMap::new(),
            level_88_groups,
            call_targets: vec![],
            file_io_fields: vec![],
        }
    }

    #[test]
    fn extracts_enum_from_exhaustive_group() {
        let source = r#"pub struct WorkingStorage {
    pub ws_status: PicX,
}

impl WorkingStorage {
    pub fn new() -> Self {
        Self {
            ws_status: PicX::new(1, b"A"),
        }
    }
}
"#;
        let hints = make_enum_hints(
            "ws_status",
            vec![("is_active", "A"), ("is_inactive", "I"), ("is_pending", "P")],
        );
        let transforms = find_enum_candidates(&hints, source, &PathBuf::from("test.rs"));

        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_auto());
        assert!(transforms[0].description.contains("WsStatus"));

        // Type edit + init edit at minimum
        assert!(transforms[0].edits.len() >= 2);
        assert_eq!(transforms[0].edits[0].new_text, "WsStatus");
    }

    #[test]
    fn skips_non_exhaustive() {
        let mut hints = make_enum_hints("ws_code", vec![("is_a", "A"), ("is_b", "B")]);
        hints
            .level_88_groups
            .get_mut("ws_code")
            .unwrap()
            .exhaustive = false;

        let source = "pub ws_code: PicX,\n";
        let transforms = find_enum_candidates(&hints, source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn skips_single_condition() {
        // Single condition -> bool-extract, not enum-extract
        let hints = make_enum_hints("ws_flag", vec![("is_on", "Y")]);
        let source = "pub ws_flag: PicX,\n";
        let transforms = find_enum_candidates(&hints, source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn field_to_enum_name_conversion() {
        assert_eq!(field_to_enum_name("ws_status"), "WsStatus");
        assert_eq!(field_to_enum_name("ws_acct_type"), "WsAcctType");
        assert_eq!(field_to_enum_name("flag"), "Flag");
    }

    #[test]
    fn condition_to_variant_conversion() {
        assert_eq!(condition_to_variant("is_active"), "Active");
        assert_eq!(condition_to_variant("is_pending"), "Pending");
        assert_eq!(condition_to_variant("ws_status_open"), "StatusOpen");
        assert_eq!(condition_to_variant("closed"), "Closed");
    }

    #[test]
    fn rewrites_enum_comparison() {
        let candidate = EnumCandidate {
            field_name: "ws_status".to_string(),
            enum_name: "WsStatus".to_string(),
            variants: vec![
                ("is_active".to_string(), "A".to_string(), "Active".to_string()),
                ("is_inactive".to_string(), "I".to_string(), "Inactive".to_string()),
            ],
        };

        let line = "    if ws.ws_status.as_bytes() == b\"A\" {";
        let edits = rewrite_enum_comparisons(line, 10, &candidate);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "ws.ws_status == WsStatus::Active");
    }

    #[test]
    fn rewrites_enum_assignment() {
        let candidate = EnumCandidate {
            field_name: "ws_status".to_string(),
            enum_name: "WsStatus".to_string(),
            variants: vec![
                ("is_active".to_string(), "A".to_string(), "Active".to_string()),
                ("is_inactive".to_string(), "I".to_string(), "Inactive".to_string()),
            ],
        };

        let line = "    move_alphanumeric_literal(b\"I\", &mut ws.ws_status, &ctx.config);";
        let edit = rewrite_enum_assignment(line, 15, &candidate);
        assert!(edit.is_some());
        assert_eq!(edit.unwrap().new_text, "ws.ws_status = WsStatus::Inactive;");
    }

    #[test]
    fn generates_enum_definition() {
        let hints = make_enum_hints(
            "ws_status",
            vec![("is_active", "A"), ("is_inactive", "I"), ("is_pending", "P")],
        );
        let def = generate_enum_definition("ws_status: PicX -> WsStatus { ... }", &hints);
        assert!(def.is_some());
        let def = def.unwrap();
        assert!(def.contains("#[derive(Debug, Clone, PartialEq)]"));
        assert!(def.contains("pub enum WsStatus"));
        assert!(def.contains("Active"));
        assert!(def.contains("Inactive"));
        assert!(def.contains("Pending"));
    }

    #[test]
    fn blocks_redefines() {
        let mut hints = make_enum_hints("ws_status", vec![("is_a", "A"), ("is_b", "B")]);
        hints.fields.get_mut("ws_status").unwrap().redefined_by =
            vec!["ws_overlay".to_string()];

        let source = "pub ws_status: PicX,\n";
        let transforms = find_enum_candidates(&hints, source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_review());
    }

    #[test]
    fn no_hints_no_transforms() {
        let rule = EnumExtractRule;
        let source = "fn main() {}\n";
        let parsed = syn::parse_file(source).unwrap();
        let ctx = AnalysisContext {
            source: &parsed,
            source_text: source,
            file_path: &PathBuf::from("test.rs"),
            hints: None,
        };
        assert!(rule.analyze(&ctx).is_empty());
    }
}
