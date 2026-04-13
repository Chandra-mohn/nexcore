//! T2-STR: Promote PicX fields to String.
//!
//! Finds all `PicX` fields in WorkingStorage structs and promotes safe
//! ones to `String`. Fields blocked by safety gates are emitted as
//! review items instead of auto-transformed.
//!
//! Transforms applied:
//! - Struct field type: `PicX` -> `String`
//! - Initialization: `PicX::new(N, b"...")` -> `"...".to_string()`
//! - Read: `field.to_string()` / `String::from_utf8_lossy(field.as_bytes())` -> `field.clone()`
//! - Write: `move_alphanumeric_literal(b"val", &mut field, ...)` -> `field = "val".to_string()`

use std::collections::HashMap;

use crate::hints::FileHints;
use crate::rules::transform::{Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};
use crate::safety;

#[derive(Debug)]
pub struct PicToStringRule;

impl RustifyRule for PicToStringRule {
    fn id(&self) -> &'static str {
        "pic-to-string"
    }

    fn description(&self) -> &'static str {
        "Promote PicX fields to String (safety-gated)"
    }

    fn tier(&self) -> Tier {
        Tier::Tier2
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        let Some(hints) = ctx.hints else {
            return Vec::new();
        };
        find_pic_promotions(hints, ctx.source_text, ctx.file_path)
    }
}

/// Find PicX fields eligible for String promotion.
fn find_pic_promotions(
    hints: &FileHints,
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let mut transforms = Vec::new();
    let lines: Vec<&str> = source_text.lines().collect();

    // Collect all PicX fields from hints
    let pic_x_fields: HashMap<&str, _> = hints
        .fields
        .iter()
        .filter(|(_, h)| is_pic_x(h))
        .map(|(name, hint)| (name.as_str(), hint))
        .collect();

    for (field_name, hint) in &pic_x_fields {
        let gates = safety::check_safety_gates(field_name, hint, hints);

        if !gates.is_empty() {
            // Blocked -- emit review item
            let reasons: Vec<String> = gates.iter().map(|g| g.reason.to_string()).collect();
            let recommendations: Vec<String> =
                gates.iter().map(|g| g.recommendation.clone()).collect();
            transforms.push(Transform {
                rule_id: "pic-to-string".to_string(),
                file: file_path.to_path_buf(),
                line: find_field_line(&lines, field_name).unwrap_or(0),
                description: format!("{field_name}: PicX -> String (BLOCKED)"),
                safety: Safety::Review {
                    reason: reasons.join("; "),
                    recommendation: recommendations.join("; "),
                },
                edits: vec![],
            });
            continue;
        }

        // Safe to promote -- generate edits
        let mut edits = Vec::new();

        // 1. Find struct field declaration: `field_name: PicX,`
        if let Some(line_num) = find_field_decl_line(&lines, field_name, "PicX") {
            if let Some(edit) = make_type_edit(&lines, line_num, field_name, "PicX", "String") {
                edits.push(edit);
            }
        }

        // 2. Find initialization: `PicX::new(N, b"...")` -> `"...".to_string()`
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_picx_init(line, i + 1, field_name) {
                edits.push(edit);
            }
        }

        // 3. Find `.to_decimal()` -- not applicable for PicX, skip
        // 4. Find `String::from_utf8_lossy(field.as_bytes())` patterns
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_utf8_lossy(line, i + 1, field_name) {
                edits.push(edit);
            }
        }

        // 5. Find `move_alphanumeric_literal(b"val", &mut ws.field, &ctx.config)`
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_move_alpha_literal(line, i + 1, field_name) {
                edits.push(edit);
            }
        }

        if !edits.is_empty() {
            transforms.push(Transform {
                rule_id: "pic-to-string".to_string(),
                file: file_path.to_path_buf(),
                line: find_field_line(&lines, field_name).unwrap_or(0),
                description: format!("{field_name}: PicX -> String"),
                safety: Safety::Auto,
                edits,
            });
        }
    }

    transforms.sort_by(|a, b| a.description.cmp(&b.description));
    transforms
}

/// Check if a field hint represents a PIC X field.
fn is_pic_x(hint: &crate::hints::FieldHint) -> bool {
    let pic = hint.pic.to_uppercase();
    // PIC X, PIC X(n), PIC A, PIC A(n) -- alphanumeric
    (pic.starts_with('X') || pic.starts_with('A'))
        && hint.usage.to_uppercase() == "DISPLAY"
}

/// Find the line number where a field name appears in source.
fn find_field_line(lines: &[&str], field_name: &str) -> Option<usize> {
    for (i, line) in lines.iter().enumerate() {
        if line.contains(field_name) && line.contains("PicX") {
            return Some(i + 1);
        }
    }
    None
}

/// Find struct field declaration line: `field_name: PicX,`
fn find_field_decl_line(lines: &[&str], field_name: &str, type_name: &str) -> Option<usize> {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        // Match patterns like: `pub field_name: PicX,` or `field_name: PicX,`
        if trimmed.contains(&format!("{field_name}: {type_name}"))
            || trimmed.contains(&format!("{field_name}: {type_name},"))
        {
            return Some(i + 1);
        }
    }
    None
}

/// Create a TextEdit to change the type in a struct field declaration.
fn make_type_edit(
    lines: &[&str],
    line_num: usize,
    _field_name: &str,
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

/// Rewrite `PicX::new(N, b"value")` -> `"value".to_string()` on the initialization line.
fn rewrite_picx_init(line: &str, line_num: usize, field_name: &str) -> Option<TextEdit> {
    // Match pattern: `field_name: PicX::new(N, b"value")`
    // or: `field_name: PicX::new(N, b"")`
    if !line.contains(field_name) {
        return None;
    }

    let marker = "PicX::new(";
    let start_pos = line.find(marker)?;

    // Find the matching closing paren
    let after_marker = start_pos + marker.len();
    let rest = &line[after_marker..];
    let close = find_matching_paren(rest)?;
    let end_pos = after_marker + close + 1; // include the closing ')'

    // Extract the inner content to find the byte string literal
    let inner = &rest[..close];

    // Find b"..." in the inner content
    let val = extract_byte_string_value(inner)?;

    // Build replacement: "value".to_string() or String::new() for empty
    let replacement = if val.is_empty() {
        "String::new()".to_string()
    } else {
        format!("\"{val}\".to_string()")
    };

    Some(TextEdit {
        line: line_num,
        col_start: start_pos,
        col_end: end_pos,
        new_text: replacement,
    })
}

/// Rewrite `String::from_utf8_lossy(ws.field.as_bytes())` -> `ws.field.clone()`
fn rewrite_utf8_lossy(line: &str, line_num: usize, field_name: &str) -> Option<TextEdit> {
    let pattern = format!("String::from_utf8_lossy({field_name}.as_bytes())");
    let ws_pattern = format!("String::from_utf8_lossy(ws.{field_name}.as_bytes())");

    if let Some(start) = line.find(&ws_pattern) {
        return Some(TextEdit {
            line: line_num,
            col_start: start,
            col_end: start + ws_pattern.len(),
            new_text: format!("ws.{field_name}.clone()"),
        });
    }

    if let Some(start) = line.find(&pattern) {
        return Some(TextEdit {
            line: line_num,
            col_start: start,
            col_end: start + pattern.len(),
            new_text: format!("{field_name}.clone()"),
        });
    }

    None
}

/// Rewrite `move_alphanumeric_literal(b"val", &mut ws.field, &ctx.config);`
/// -> `ws.field = "val".to_string();`
fn rewrite_move_alpha_literal(line: &str, line_num: usize, field_name: &str) -> Option<TextEdit> {
    let trimmed = line.trim();

    // Pattern: move_alphanumeric_literal(b"...", &mut ws.field, &ctx.config);
    let prefix = "move_alphanumeric_literal(b\"";
    if !trimmed.starts_with(prefix) {
        return None;
    }

    // Check this move targets our field
    let ws_field = format!("&mut ws.{field_name}");
    if !trimmed.contains(&ws_field) {
        return None;
    }

    // Extract the value from b"..."
    let after_b = &trimmed[prefix.len()..];
    let quote_end = after_b.find('"')?;
    let val = &after_b[..quote_end];

    // Calculate positions in the original line (with indentation)
    let indent = line.len() - trimmed.len();
    let stmt_end = line.trim_end().len();

    let replacement = if val.is_empty() {
        format!("ws.{field_name} = String::new();")
    } else {
        format!("ws.{field_name} = \"{val}\".to_string();")
    };

    Some(TextEdit {
        line: line_num,
        col_start: indent,
        col_end: stmt_end,
        new_text: replacement,
    })
}

/// Find the matching closing parenthesis, handling nesting.
fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 1;
    let mut in_string = false;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' if !in_string => in_string = true,
            b'"' if in_string => {
                // Check for escaped quote
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

/// Extract the string value from a byte string literal b"value".
fn extract_byte_string_value(inner: &str) -> Option<String> {
    // inner looks like: `N, b"value"` or `N, b""`
    let b_start = inner.find("b\"")?;
    let val_start = b_start + 2;
    let rest = &inner[val_start..];
    let val_end = rest.find('"')?;
    Some(rest[..val_end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hints::{FieldHint, FileHints};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_pic_x_hint(name: &str, pic: &str) -> (String, FieldHint) {
        (
            name.to_string(),
            FieldHint {
                pic: pic.to_string(),
                usage: "DISPLAY".to_string(),
                level: 5,
                redefines: None,
                redefined_by: vec![],
                call_by_ref: false,
                move_corr_target: false,
                read_count: 3,
                write_count: 1,
                paragraph_scope: vec![],
            },
        )
    }

    fn make_file_hints(fields: Vec<(String, FieldHint)>) -> FileHints {
        FileHints {
            cobol_source: "TEST.CBL".to_string(),
            fields: fields.into_iter().collect(),
            paragraphs: HashMap::new(),
            level_88_groups: HashMap::new(),
            call_targets: vec![],
            file_io_fields: vec![],
        }
    }

    #[test]
    fn promotes_safe_picx_field() {
        let source = r#"pub struct WorkingStorage {
    pub ws_name: PicX,
}

impl WorkingStorage {
    pub fn new() -> Self {
        Self {
            ws_name: PicX::new(30, b"HELLO"),
        }
    }
}
"#;
        let hints = make_file_hints(vec![make_pic_x_hint("ws_name", "X(30)")]);
        let transforms = find_pic_promotions(&hints, source, &PathBuf::from("test.rs"));

        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_auto());
        assert_eq!(transforms[0].edits.len(), 2); // type decl + init

        // Check type edit: PicX -> String
        let type_edit = &transforms[0].edits[0];
        assert_eq!(type_edit.new_text, "String");

        // Check init edit: PicX::new(30, b"HELLO") -> "HELLO".to_string()
        let init_edit = &transforms[0].edits[1];
        assert_eq!(init_edit.new_text, "\"HELLO\".to_string()");
    }

    #[test]
    fn blocks_redefines_field() {
        let source = "pub ws_full: PicX,\n";
        let mut fields = vec![make_pic_x_hint("ws_full", "X(80)")];
        fields[0].1.redefined_by = vec!["ws_parts".to_string()];
        let hints = make_file_hints(fields);

        let transforms = find_pic_promotions(&hints, source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_review());
        assert!(transforms[0].description.contains("BLOCKED"));
    }

    #[test]
    fn blocks_call_by_ref() {
        let source = "pub ws_param: PicX,\n";
        let mut fields = vec![make_pic_x_hint("ws_param", "X(10)")];
        fields[0].1.call_by_ref = true;
        let hints = make_file_hints(fields);

        let transforms = find_pic_promotions(&hints, source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_review());
    }

    #[test]
    fn rewrites_utf8_lossy() {
        let line = "    let name = String::from_utf8_lossy(ws.ws_name.as_bytes());";
        let edit = rewrite_utf8_lossy(line, 10, "ws_name");
        assert!(edit.is_some());
        let edit = edit.unwrap();
        assert_eq!(edit.new_text, "ws.ws_name.clone()");
    }

    #[test]
    fn rewrites_move_alpha_literal() {
        let line = "    move_alphanumeric_literal(b\"JOHN\", &mut ws.ws_name, &ctx.config);";
        let edit = rewrite_move_alpha_literal(line, 15, "ws_name");
        assert!(edit.is_some());
        let edit = edit.unwrap();
        assert_eq!(edit.new_text, "ws.ws_name = \"JOHN\".to_string();");
    }

    #[test]
    fn empty_picx_init() {
        let line = "    ws_name: PicX::new(10, b\"\"),";
        let edit = rewrite_picx_init(line, 5, "ws_name");
        assert!(edit.is_some());
        assert_eq!(edit.unwrap().new_text, "String::new()");
    }

    #[test]
    fn no_hints_no_transforms() {
        let rule = PicToStringRule;
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

    #[test]
    fn skips_numeric_fields() {
        let source = "pub ws_amount: PackedDecimal,\n";
        let hints = make_file_hints(vec![(
            "ws_amount".to_string(),
            FieldHint {
                pic: "S9(7)V99".to_string(),
                usage: "COMP-3".to_string(),
                level: 5,
                redefines: None,
                redefined_by: vec![],
                call_by_ref: false,
                move_corr_target: false,
                read_count: 5,
                write_count: 2,
                paragraph_scope: vec![],
            },
        )]);

        let transforms = find_pic_promotions(&hints, source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn file_io_field_blocked() {
        let source = "pub ws_record: PicX,\n";
        let mut hints = make_file_hints(vec![make_pic_x_hint("ws_record", "X(80)")]);
        hints.file_io_fields = vec!["ws_record".to_string()];

        let transforms = find_pic_promotions(&hints, source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_review());
    }
}
