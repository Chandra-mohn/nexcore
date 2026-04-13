//! T2-NUM: Promote PackedDecimal/ZonedDecimal fields to Decimal.
//!
//! Finds COMP-3 (PackedDecimal) and DISPLAY numeric (ZonedDecimal)
//! fields in WorkingStorage and promotes safe ones to `Decimal`.
//!
//! Transforms applied:
//! - Struct field type: `PackedDecimal` / `ZonedDecimal` -> `Decimal`
//! - Initialization: `PackedDecimal::new(p, s, signed)` -> `Decimal::ZERO`
//!   or `{ let mut _p = PackedDecimal::new(...); _p.pack(val); _p }` -> `val`
//! - Read: `.to_decimal()` -> identity (remove the call)
//! - Write: `.pack(val)` -> `= val`

use std::collections::HashMap;

use crate::hints::FileHints;
use crate::rules::transform::{Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};
use crate::safety;

#[derive(Debug)]
pub struct PackedToNativeRule;

impl RustifyRule for PackedToNativeRule {
    fn id(&self) -> &'static str {
        "packed-to-native"
    }

    fn description(&self) -> &'static str {
        "Promote PackedDecimal/ZonedDecimal to Decimal (safety-gated)"
    }

    fn tier(&self) -> Tier {
        Tier::Tier2
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        let Some(hints) = ctx.hints else {
            return Vec::new();
        };
        find_numeric_promotions(hints, ctx.source_text, ctx.file_path)
    }
}

/// The COBOL numeric types we promote.
#[derive(Debug, Clone, Copy)]
enum NumericType {
    PackedDecimal,
    ZonedDecimal,
}

impl NumericType {
    fn as_str(self) -> &'static str {
        match self {
            Self::PackedDecimal => "PackedDecimal",
            Self::ZonedDecimal => "ZonedDecimal",
        }
    }
}

/// Find numeric fields eligible for Decimal promotion.
fn find_numeric_promotions(
    hints: &FileHints,
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let mut transforms = Vec::new();
    let lines: Vec<&str> = source_text.lines().collect();

    // Collect numeric fields
    let numeric_fields: HashMap<&str, (&crate::hints::FieldHint, NumericType)> = hints
        .fields
        .iter()
        .filter_map(|(name, h)| {
            classify_numeric(h).map(|nt| (name.as_str(), (h, nt)))
        })
        .collect();

    for (field_name, (hint, num_type)) in &numeric_fields {
        let gates = safety::check_safety_gates(field_name, hint, hints);

        if !gates.is_empty() {
            let reasons: Vec<String> = gates.iter().map(|g| g.reason.to_string()).collect();
            let recommendations: Vec<String> =
                gates.iter().map(|g| g.recommendation.clone()).collect();
            transforms.push(Transform {
                rule_id: "packed-to-native".to_string(),
                file: file_path.to_path_buf(),
                line: find_field_line(&lines, field_name, num_type.as_str()).unwrap_or(0),
                description: format!(
                    "{field_name}: {} -> Decimal (BLOCKED)",
                    num_type.as_str()
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
        let type_str = num_type.as_str();

        // 1. Struct field type declaration
        if let Some(line_num) = find_field_decl_line(&lines, field_name, type_str) {
            if let Some(edit) = make_type_edit(&lines, line_num, type_str, "Decimal") {
                edits.push(edit);
            }
        }

        // 2. Initialization patterns
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_numeric_init(line, i + 1, field_name, type_str) {
                edits.push(edit);
            }
        }

        // 3. `.to_decimal()` calls -> remove the method call
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_to_decimal(line, i + 1, field_name) {
                edits.push(edit);
            }
        }

        // 4. `.pack(val)` calls -> `= val`
        for (i, line) in lines.iter().enumerate() {
            if let Some(edit) = rewrite_pack_call(line, i + 1, field_name) {
                edits.push(edit);
            }
        }

        if !edits.is_empty() {
            transforms.push(Transform {
                rule_id: "packed-to-native".to_string(),
                file: file_path.to_path_buf(),
                line: find_field_line(&lines, field_name, type_str).unwrap_or(0),
                description: format!("{field_name}: {type_str} -> Decimal"),
                safety: Safety::Auto,
                edits,
            });
        }
    }

    transforms.sort_by(|a, b| a.description.cmp(&b.description));
    transforms
}

/// Classify a field hint as a promotable numeric type.
fn classify_numeric(hint: &crate::hints::FieldHint) -> Option<NumericType> {
    let usage = hint.usage.to_uppercase();
    match usage.as_str() {
        "COMP-3" | "COMPUTATIONAL-3" | "PACKED-DECIMAL" => Some(NumericType::PackedDecimal),
        "DISPLAY" => {
            // DISPLAY with numeric PIC -> ZonedDecimal
            let pic = hint.pic.to_uppercase();
            if is_numeric_pic(&pic) {
                Some(NumericType::ZonedDecimal)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Check if a PIC clause is numeric (contains 9, S, V but not X or A).
fn is_numeric_pic(pic: &str) -> bool {
    if pic.is_empty() {
        return false;
    }
    // Numeric PICs contain 9, may have S (sign) and V (decimal point)
    // but NOT X (alphanumeric) or A (alphabetic)
    let has_digit = pic.contains('9');
    let no_alpha = !pic.contains('X') && !pic.contains('A');
    has_digit && no_alpha
}

/// Find the line number where a field is declared with a given type.
fn find_field_line(lines: &[&str], field_name: &str, type_name: &str) -> Option<usize> {
    for (i, line) in lines.iter().enumerate() {
        if line.contains(field_name) && line.contains(type_name) {
            return Some(i + 1);
        }
    }
    None
}

/// Find struct field declaration line.
fn find_field_decl_line(lines: &[&str], field_name: &str, type_name: &str) -> Option<usize> {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if (trimmed.contains(&format!("{field_name}: {type_name}"))
            || trimmed.contains(&format!("{field_name}: {type_name},")))
            // Avoid matching initialization lines
            && !trimmed.contains("::new(")
        {
            return Some(i + 1);
        }
    }
    None
}

/// Create a TextEdit to change the type name in a declaration.
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

/// Rewrite numeric type initialization.
///
/// Patterns:
/// - `PackedDecimal::new(p, s, signed)` -> `Decimal::ZERO`
/// - `{ let mut _p = PackedDecimal::new(...); _p.pack(val); _p }` -> `val`
/// - `ZonedDecimal::new(p, s)` -> `Decimal::ZERO`
fn rewrite_numeric_init(
    line: &str,
    line_num: usize,
    field_name: &str,
    type_name: &str,
) -> Option<TextEdit> {
    if !line.contains(field_name) {
        return None;
    }

    // Pattern: `{ let mut _p = TypeName::new(...); _p.pack(val); _p }`
    let block_marker = format!("{{ let mut _p = {type_name}::new(");
    if let Some(block_start) = line.find(&block_marker) {
        // Find the closing `_p }`
        let rest = &line[block_start..];
        if let Some(block_end_rel) = rest.find("_p }") {
            let block_end = block_start + block_end_rel + 4;

            // Extract the pack value: `_p.pack(VAL);`
            let pack_marker = "_p.pack(";
            if let Some(pack_pos) = rest.find(pack_marker) {
                let val_start = pack_pos + pack_marker.len();
                let val_rest = &rest[val_start..];
                if let Some(val_end) = val_rest.find(");") {
                    let val = &val_rest[..val_end];
                    return Some(TextEdit {
                        line: line_num,
                        col_start: block_start,
                        col_end: block_end,
                        new_text: val.to_string(),
                    });
                }
            }

            // Fallback: just replace entire block with Decimal::ZERO
            return Some(TextEdit {
                line: line_num,
                col_start: block_start,
                col_end: block_end,
                new_text: "Decimal::ZERO".to_string(),
            });
        }
    }

    // Pattern: `TypeName::new(p, s, signed)` (bare, no pack)
    let simple_marker = format!("{type_name}::new(");
    if let Some(start) = line.find(&simple_marker) {
        let rest = &line[start + simple_marker.len()..];
        if let Some(close) = find_matching_paren(rest) {
            let end = start + simple_marker.len() + close + 1;
            return Some(TextEdit {
                line: line_num,
                col_start: start,
                col_end: end,
                new_text: "Decimal::ZERO".to_string(),
            });
        }
    }

    None
}

/// Rewrite `ws.field.to_decimal()` -> `ws.field`.
///
/// In expressions like `ws.ws_amount.to_decimal() + ws.ws_tax.to_decimal()`,
/// we remove `.to_decimal()` for the specific field.
fn rewrite_to_decimal(line: &str, line_num: usize, field_name: &str) -> Option<TextEdit> {
    let pattern = format!("{field_name}.to_decimal()");
    let pos = line.find(&pattern)?;

    // Replace `field_name.to_decimal()` with just `field_name`
    Some(TextEdit {
        line: line_num,
        col_start: pos,
        col_end: pos + pattern.len(),
        new_text: field_name.to_string(),
    })
}

/// Rewrite `ws.field.pack(val)` -> `ws.field = val`.
fn rewrite_pack_call(line: &str, line_num: usize, field_name: &str) -> Option<TextEdit> {
    let trimmed = line.trim();

    // Pattern: `ws.field.pack(val);`
    let ws_pack = format!("ws.{field_name}.pack(");
    if !trimmed.starts_with(&ws_pack) {
        return None;
    }

    let after_pack = &trimmed[ws_pack.len()..];
    let close = after_pack.find(");")?;
    let val = &after_pack[..close];

    let indent = line.len() - trimmed.len();
    let stmt_end = line.trim_end().len();

    Some(TextEdit {
        line: line_num,
        col_start: indent,
        col_end: stmt_end,
        new_text: format!("ws.{field_name} = {val};"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hints::{FieldHint, FileHints};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_packed_hint(name: &str) -> (String, FieldHint) {
        (
            name.to_string(),
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
        )
    }

    fn make_zoned_hint(name: &str) -> (String, FieldHint) {
        (
            name.to_string(),
            FieldHint {
                pic: "9(5)".to_string(),
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
    fn promotes_packed_decimal() {
        let source = r#"pub struct WorkingStorage {
    pub ws_amount: PackedDecimal,
}

impl WorkingStorage {
    pub fn new() -> Self {
        Self {
            ws_amount: PackedDecimal::new(7, 2, true),
        }
    }
}
"#;
        let hints = make_file_hints(vec![make_packed_hint("ws_amount")]);
        let transforms = find_numeric_promotions(&hints, source, &PathBuf::from("test.rs"));

        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_auto());
        assert!(transforms[0].description.contains("PackedDecimal -> Decimal"));

        // Should have type edit + init edit
        assert!(transforms[0].edits.len() >= 2);
        assert_eq!(transforms[0].edits[0].new_text, "Decimal");
        assert_eq!(transforms[0].edits[1].new_text, "Decimal::ZERO");
    }

    #[test]
    fn promotes_packed_with_initial_value() {
        let source = r#"    ws_amount: { let mut _p = PackedDecimal::new(7, 2, true); _p.pack("100.00".parse::<Decimal>().unwrap()); _p },
"#;
        let hints = make_file_hints(vec![make_packed_hint("ws_amount")]);
        let lines: Vec<&str> = source.lines().collect();

        let edit = rewrite_numeric_init(lines[0], 1, "ws_amount", "PackedDecimal");
        assert!(edit.is_some());
        let edit = edit.unwrap();
        assert_eq!(
            edit.new_text,
            "\"100.00\".parse::<Decimal>().unwrap()"
        );
    }

    #[test]
    fn rewrites_to_decimal() {
        let line = "    let total = ws.ws_amount.to_decimal() + ws.ws_tax.to_decimal();";
        let edit = rewrite_to_decimal(line, 5, "ws_amount");
        assert!(edit.is_some());
        let edit = edit.unwrap();
        assert_eq!(edit.new_text, "ws_amount");
    }

    #[test]
    fn rewrites_pack_call() {
        let line = "    ws.ws_amount.pack(result);";
        let edit = rewrite_pack_call(line, 10, "ws_amount");
        assert!(edit.is_some());
        let edit = edit.unwrap();
        assert_eq!(edit.new_text, "ws.ws_amount = result;");
    }

    #[test]
    fn blocks_redefines_field() {
        let source = "pub ws_amount: PackedDecimal,\n";
        let mut fields = vec![make_packed_hint("ws_amount")];
        fields[0].1.redefined_by = vec!["ws_overlay".to_string()];
        let hints = make_file_hints(fields);

        let transforms = find_numeric_promotions(&hints, source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_review());
        assert!(transforms[0].description.contains("BLOCKED"));
    }

    #[test]
    fn promotes_zoned_decimal() {
        let source = r#"pub struct WorkingStorage {
    pub ws_count: ZonedDecimal,
}
"#;
        let hints = make_file_hints(vec![make_zoned_hint("ws_count")]);
        let transforms = find_numeric_promotions(&hints, source, &PathBuf::from("test.rs"));

        // Should find the field (even though ZonedDecimal::new isn't in this snippet,
        // the type decl edit should be there)
        assert!(!transforms.is_empty());
        if !transforms[0].edits.is_empty() {
            assert_eq!(transforms[0].edits[0].new_text, "Decimal");
        }
    }

    #[test]
    fn no_hints_no_transforms() {
        let rule = PackedToNativeRule;
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
    fn skips_pic_x_fields() {
        let source = "pub ws_name: PicX,\n";
        let hints = make_file_hints(vec![(
            "ws_name".to_string(),
            FieldHint {
                pic: "X(30)".to_string(),
                usage: "DISPLAY".to_string(),
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

        let transforms = find_numeric_promotions(&hints, source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn is_numeric_pic_tests() {
        assert!(is_numeric_pic("9(5)"));
        assert!(is_numeric_pic("S9(7)V99"));
        assert!(is_numeric_pic("9"));
        assert!(!is_numeric_pic("X(10)"));
        assert!(!is_numeric_pic("A(5)"));
        assert!(!is_numeric_pic(""));
    }

    #[test]
    fn file_io_field_blocked() {
        let source = "pub ws_record_count: PackedDecimal,\n";
        let mut hints = make_file_hints(vec![make_packed_hint("ws_record_count")]);
        hints.file_io_fields = vec!["ws_record_count".to_string()];

        let transforms = find_numeric_promotions(&hints, source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_review());
    }
}
