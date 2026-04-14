//! T2-BOOL: Extract boolean fields from level-88 conditions.
//!
//! When a COBOL field has exactly one level-88 condition (e.g., `88 WS-FLAG-ON VALUE "Y"`)
//! on a single-byte PIC X field, it can be promoted to a Rust `bool`.
//!
//! Transforms applied:
//! - Struct field type: `PicX` -> `bool`
//! - Initialization: `PicX::new(1, b"N")` -> `false` (or `true` if init matches condition)
//! - Condition check: `ws.field.as_bytes() == b"Y"` -> `ws.field`
//! - Negated check: `ws.field.as_bytes() != b"Y"` -> `!ws.field`
//! - Assignment: `move_alphanumeric_literal(b"Y", ...)` -> `ws.field = true`

use crate::hints::FileHints;
use crate::rules::transform::{Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};
use crate::safety;

#[derive(Debug)]
pub struct BoolExtractRule;

impl RustifyRule for BoolExtractRule {
    fn id(&self) -> &'static str {
        "bool-extract"
    }

    fn description(&self) -> &'static str {
        "Promote single-condition PIC X(1) fields to bool (safety-gated)"
    }

    fn tier(&self) -> Tier {
        Tier::Tier2
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        let Some(hints) = ctx.hints else {
            return Vec::new();
        };
        find_bool_candidates(hints, ctx.source_text, ctx.file_path)
    }
}

/// A boolean candidate: field with a single level-88 condition on PIC X(1).
struct BoolCandidate {
    field_name: String,
    true_value: String,
}

/// Find fields that can be promoted to bool.
fn find_bool_candidates(
    hints: &FileHints,
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let mut transforms = Vec::new();
    let lines: Vec<&str> = source_text.lines().collect();

    // Find boolean candidates from level_88_groups
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
                rule_id: "bool-extract".to_string(),
                file: file_path.to_path_buf(),
                line: 0,
                description: format!(
                    "{}: PicX -> bool (BLOCKED)",
                    candidate.field_name
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

        // 1. Type declaration: PicX -> bool
        if let Some(line_num) = find_field_decl(&lines, &candidate.field_name, "PicX") {
            if let Some(edit) = make_type_edit(&lines, line_num, "PicX", "bool") {
                edits.push(edit);
            }
        }

        // 2. Initialization: PicX::new(1, b"X") -> true/false
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_bool_init(line, i + 1, candidate) {
                edits.push(edit);
            }
        }

        // 3. Condition checks: field.as_bytes() == b"Y" -> field
        for (i, line) in lines.iter().enumerate() {
            edits.extend(rewrite_bool_comparisons(line, i + 1, candidate));
        }

        // 4. Assignments: move_alphanumeric_literal(b"Y", ...) -> field = true
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_bool_assignment(line, i + 1, candidate) {
                edits.push(edit);
            }
        }

        if !edits.is_empty() {
            transforms.push(Transform {
                rule_id: "bool-extract".to_string(),
                file: file_path.to_path_buf(),
                line: find_field_decl(&lines, &candidate.field_name, "PicX").unwrap_or(0),
                description: format!(
                    "{}: PicX -> bool (true = \"{}\")",
                    candidate.field_name, candidate.true_value
                ),
                safety: Safety::Auto,
                edits,
            });
        }
    }

    transforms.sort_by(|a, b| a.description.cmp(&b.description));
    transforms
}

/// Collect boolean candidates from hints.
fn collect_candidates(hints: &FileHints) -> Vec<BoolCandidate> {
    let mut candidates = Vec::new();

    for (field_name, group) in &hints.level_88_groups {
        // Must have exactly 1 condition (single boolean)
        if group.conditions.len() != 1 {
            continue;
        }

        // Field must be PIC X(1) -- single byte
        let Some(hint) = hints.fields.get(field_name) else {
            continue;
        };
        if !is_single_byte_pic_x(hint) {
            continue;
        }

        let (_, value) = group.conditions.iter().next().expect("conditions.len() == 1 confirmed above");
        candidates.push(BoolCandidate {
            field_name: field_name.clone(),
            true_value: value.clone(),
        });
    }

    candidates
}

/// Check if a field is PIC X(1) or PIC X -- single byte alphanumeric.
fn is_single_byte_pic_x(hint: &crate::hints::FieldHint) -> bool {
    let pic = hint.pic.to_uppercase();
    (pic == "X" || pic == "X(1)") && hint.usage.to_uppercase() == "DISPLAY"
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

/// Rewrite initialization: `PicX::new(1, b"Y")` -> `true`, `PicX::new(1, b"N")` -> `false`
fn rewrite_bool_init(line: &str, line_num: usize, candidate: &BoolCandidate) -> Option<TextEdit> {
    if !line.contains(&candidate.field_name) {
        return None;
    }

    let marker = "PicX::new(";
    let start = line.find(marker)?;
    let rest = &line[start + marker.len()..];
    let close = find_matching_paren(rest)?;
    let end = start + marker.len() + close + 1;
    let inner = &rest[..close];

    // Extract byte string value
    let val = extract_byte_string_value(inner)?;

    let is_true = val == candidate.true_value;
    let replacement = if is_true { "true" } else { "false" };

    Some(TextEdit {
        line: line_num,
        col_start: start,
        col_end: end,
        new_text: replacement.to_string(),
    })
}

/// Rewrite boolean comparisons.
///
/// Patterns:
/// - `ws.field.as_bytes() == b"Y"` -> `ws.field`
/// - `ws.field.as_bytes() != b"Y"` -> `!ws.field`
fn rewrite_bool_comparisons(
    line: &str,
    line_num: usize,
    candidate: &BoolCandidate,
) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    let eq_pattern = format!(
        "ws.{}.as_bytes() == b\"{}\"",
        candidate.field_name, candidate.true_value
    );
    let ne_pattern = format!(
        "ws.{}.as_bytes() != b\"{}\"",
        candidate.field_name, candidate.true_value
    );

    if let Some(start) = line.find(&eq_pattern) {
        edits.push(TextEdit {
            line: line_num,
            col_start: start,
            col_end: start + eq_pattern.len(),
            new_text: format!("ws.{}", candidate.field_name),
        });
    } else if let Some(start) = line.find(&ne_pattern) {
        edits.push(TextEdit {
            line: line_num,
            col_start: start,
            col_end: start + ne_pattern.len(),
            new_text: format!("!ws.{}", candidate.field_name),
        });
    }

    edits
}

/// Rewrite assignments: `move_alphanumeric_literal(b"Y", &mut ws.field, ...)` -> `ws.field = true`
fn rewrite_bool_assignment(
    line: &str,
    line_num: usize,
    candidate: &BoolCandidate,
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

    // Extract the value
    let after_b = &trimmed[prefix.len()..];
    let quote_end = after_b.find('"')?;
    let val = &after_b[..quote_end];

    let is_true = val == candidate.true_value;
    let bool_val = if is_true { "true" } else { "false" };

    let indent = line.len() - trimmed.len();
    let stmt_end = line.trim_end().len();

    Some(TextEdit {
        line: line_num,
        col_start: indent,
        col_end: stmt_end,
        new_text: format!("ws.{} = {bool_val};", candidate.field_name),
    })
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

/// Extract string value from a byte string literal b"value".
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

    fn make_flag_hints(field_name: &str, true_val: &str) -> FileHints {
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
                read_count: 4,
                write_count: 2,
                paragraph_scope: vec![],
            },
        );

        let mut conditions = HashMap::new();
        conditions.insert("is_active".to_string(), true_val.to_string());

        let mut level_88_groups = HashMap::new();
        level_88_groups.insert(
            field_name.to_string(),
            Level88Group {
                conditions,
                exhaustive: false,
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
    fn promotes_single_flag_to_bool() {
        let source = r#"pub struct WorkingStorage {
    pub ws_flag: PicX,
}

impl WorkingStorage {
    pub fn new() -> Self {
        Self {
            ws_flag: PicX::new(1, b"N"),
        }
    }
}
"#;
        let hints = make_flag_hints("ws_flag", "Y");
        let transforms = find_bool_candidates(&hints, source, &PathBuf::from("test.rs"));

        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_auto());
        assert!(transforms[0].description.contains("bool"));

        // Type edit + init edit
        assert!(transforms[0].edits.len() >= 2);
        assert_eq!(transforms[0].edits[0].new_text, "bool");
        assert_eq!(transforms[0].edits[1].new_text, "false"); // "N" != "Y", so false
    }

    #[test]
    fn init_true_when_matches() {
        let line = "    ws_flag: PicX::new(1, b\"Y\"),";
        let candidate = BoolCandidate {
            field_name: "ws_flag".to_string(),
            true_value: "Y".to_string(),
        };
        let edit = rewrite_bool_init(line, 5, &candidate);
        assert!(edit.is_some());
        assert_eq!(edit.unwrap().new_text, "true");
    }

    #[test]
    fn rewrites_equality_check() {
        let line = "    if ws.ws_flag.as_bytes() == b\"Y\" {";
        let candidate = BoolCandidate {
            field_name: "ws_flag".to_string(),
            true_value: "Y".to_string(),
        };
        let edits = rewrite_bool_comparisons(line, 10, &candidate);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "ws.ws_flag");
    }

    #[test]
    fn rewrites_inequality_check() {
        let line = "    if ws.ws_flag.as_bytes() != b\"Y\" {";
        let candidate = BoolCandidate {
            field_name: "ws_flag".to_string(),
            true_value: "Y".to_string(),
        };
        let edits = rewrite_bool_comparisons(line, 10, &candidate);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "!ws.ws_flag");
    }

    #[test]
    fn rewrites_set_to_true() {
        let line = "    move_alphanumeric_literal(b\"Y\", &mut ws.ws_flag, &ctx.config);";
        let candidate = BoolCandidate {
            field_name: "ws_flag".to_string(),
            true_value: "Y".to_string(),
        };
        let edit = rewrite_bool_assignment(line, 15, &candidate);
        assert!(edit.is_some());
        assert_eq!(edit.unwrap().new_text, "ws.ws_flag = true;");
    }

    #[test]
    fn rewrites_set_to_false() {
        let line = "    move_alphanumeric_literal(b\"N\", &mut ws.ws_flag, &ctx.config);";
        let candidate = BoolCandidate {
            field_name: "ws_flag".to_string(),
            true_value: "Y".to_string(),
        };
        let edit = rewrite_bool_assignment(line, 15, &candidate);
        assert!(edit.is_some());
        assert_eq!(edit.unwrap().new_text, "ws.ws_flag = false;");
    }

    #[test]
    fn skips_multi_condition_field() {
        let mut hints = make_flag_hints("ws_status", "A");
        // Add a second condition -- no longer boolean
        hints
            .level_88_groups
            .get_mut("ws_status")
            .unwrap()
            .conditions
            .insert("is_inactive".to_string(), "I".to_string());

        let source = "pub ws_status: PicX,\n";
        let transforms = find_bool_candidates(&hints, source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn skips_multi_byte_field() {
        let mut hints = make_flag_hints("ws_code", "AB");
        hints.fields.get_mut("ws_code").unwrap().pic = "X(2)".to_string();

        let source = "pub ws_code: PicX,\n";
        let transforms = find_bool_candidates(&hints, source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn blocks_redefines() {
        let mut hints = make_flag_hints("ws_flag", "Y");
        hints.fields.get_mut("ws_flag").unwrap().redefined_by =
            vec!["ws_overlay".to_string()];

        let source = "pub ws_flag: PicX,\n";
        let transforms = find_bool_candidates(&hints, source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_review());
    }

    #[test]
    fn no_hints_no_transforms() {
        let rule = BoolExtractRule;
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
