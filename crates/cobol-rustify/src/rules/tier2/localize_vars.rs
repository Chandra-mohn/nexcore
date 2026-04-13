//! T2-LOCAL: Localize single-paragraph variables.
//!
//! Fields used in only one paragraph are extracted from WorkingStorage
//! and converted to local `let mut` bindings within that paragraph function.
//!
//! Requires hints with `paragraph_scope` per field and `local_only_fields`
//! per paragraph. Safety-gated: REDEFINES, CALL BY REF, File I/O, and
//! MOVE CORRESPONDING block localization.
//!
//! Transforms applied:
//! - Remove field from struct declaration
//! - Remove field from `new()` constructor
//! - Insert `let mut field = init;` at top of paragraph function
//! - Replace `ws.field` with `field` within that paragraph

use crate::hints::FileHints;
use crate::rules::transform::{Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};
use crate::safety;

#[derive(Debug)]
pub struct LocalizeVarsRule;

impl RustifyRule for LocalizeVarsRule {
    fn id(&self) -> &'static str {
        "localize-vars"
    }

    fn description(&self) -> &'static str {
        "Extract single-paragraph fields to local variables (safety-gated)"
    }

    fn tier(&self) -> Tier {
        Tier::Tier2
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        let Some(hints) = ctx.hints else {
            return Vec::new();
        };
        find_localizable_fields(hints, ctx.source_text, ctx.file_path)
    }
}

/// Find fields that are only used in a single paragraph.
fn find_localizable_fields(
    hints: &FileHints,
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let mut transforms = Vec::new();
    let lines: Vec<&str> = source_text.lines().collect();

    for (field_name, hint) in &hints.fields {
        // Must be used in exactly one paragraph
        if hint.paragraph_scope.len() != 1 {
            continue;
        }

        // Must actually be used (not dead)
        if hint.read_count == 0 && hint.write_count == 0 {
            continue;
        }

        let paragraph = &hint.paragraph_scope[0];

        // Cross-check: paragraph hints should list this as local_only
        let is_confirmed_local = hints
            .paragraphs
            .get(paragraph)
            .is_some_and(|ph| ph.local_only_fields.contains(field_name));

        if !is_confirmed_local {
            continue;
        }

        // Safety gate check
        let gates = safety::check_safety_gates(field_name, hint, hints);

        if !gates.is_empty() {
            let reasons: Vec<String> = gates.iter().map(|g| g.reason.to_string()).collect();
            let recommendations: Vec<String> =
                gates.iter().map(|g| g.recommendation.clone()).collect();
            transforms.push(Transform {
                rule_id: "localize-vars".to_string(),
                file: file_path.to_path_buf(),
                line: 0,
                description: format!(
                    "{field_name}: localize to {paragraph} (BLOCKED)"
                ),
                safety: Safety::Review {
                    reason: reasons.join("; "),
                    recommendation: recommendations.join("; "),
                },
                edits: vec![],
            });
            continue;
        }

        // Generate edits
        let mut edits = Vec::new();

        // 1. Find and blank the struct field declaration line
        if let Some(line_num) = find_struct_field_line(&lines, field_name) {
            let line = lines[line_num - 1];
            edits.push(TextEdit {
                line: line_num,
                col_start: 0,
                col_end: line.len(),
                new_text: String::new(),
            });
        }

        // 2. Find and blank the field initialization in new()
        if let Some(line_num) = find_init_line(&lines, field_name) {
            let line = lines[line_num - 1];
            edits.push(TextEdit {
                line: line_num,
                col_start: 0,
                col_end: line.len(),
                new_text: String::new(),
            });
        }

        // 3. Find the paragraph function and insert `let mut field = init;`
        let rust_para_name = cobol_to_rust_fn_name(paragraph);
        if let Some(fn_line) = find_fn_line(&lines, &rust_para_name) {
            // Extract the initialization expression from the new() before we blank it
            let init_expr = extract_init_expr(&lines, field_name)
                .unwrap_or_else(|| "Default::default()".to_string());

            // Insert after the opening brace of the function
            // The function opening line ends with `{`, so insert on next line
            let insert_line = fn_line + 1;
            if insert_line <= lines.len() {
                let line = lines[insert_line - 1];
                let indent = leading_spaces(line).max(8);
                let indent_str = " ".repeat(indent);
                edits.push(TextEdit {
                    line: insert_line,
                    col_start: 0,
                    col_end: 0,
                    new_text: format!("{indent_str}let mut {field_name} = {init_expr};\n"),
                });
            }
        }

        // 4. Replace `ws.field_name` with `field_name` within the paragraph function
        if let Some(fn_range) = find_fn_range(&lines, &rust_para_name) {
            let ws_prefix = format!("ws.{field_name}");
            for (line_idx, line) in lines.iter().enumerate().skip(fn_range.0).take(fn_range.1 - fn_range.0) {
                let line = *line;
                // Find all occurrences of `ws.field_name` in this line
                let mut search_from = 0;
                while let Some(pos) = line[search_from..].find(&ws_prefix) {
                    let abs_pos = search_from + pos;
                    let after = abs_pos + ws_prefix.len();

                    // Make sure we're matching a complete token (not a prefix of a longer name)
                    let is_boundary = after >= line.len()
                        || !line.as_bytes()[after].is_ascii_alphanumeric()
                            && line.as_bytes()[after] != b'_';

                    if is_boundary {
                        edits.push(TextEdit {
                            line: line_idx + 1,
                            col_start: abs_pos,
                            col_end: after,
                            new_text: field_name.clone(),
                        });
                    }
                    search_from = after;
                }
            }
        }

        if !edits.is_empty() {
            transforms.push(Transform {
                rule_id: "localize-vars".to_string(),
                file: file_path.to_path_buf(),
                line: 0,
                description: format!("{field_name}: localize to {paragraph}"),
                safety: Safety::Auto,
                edits,
            });
        }
    }

    transforms.sort_by(|a, b| a.description.cmp(&b.description));
    transforms
}

/// Find the struct field declaration for a given field name.
fn find_struct_field_line(lines: &[&str], field_name: &str) -> Option<usize> {
    // Look for `pub field_name:` inside a struct definition
    let pattern = format!("pub {field_name}:");
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with(&pattern) && !trimmed.contains("::new(") {
            return Some(i + 1);
        }
    }
    None
}

/// Find the initialization line for a field in the new() constructor.
fn find_init_line(lines: &[&str], field_name: &str) -> Option<usize> {
    // Look for `field_name:` inside new() that has an initialization value
    let pattern = format!("{field_name}:");
    let mut in_new = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.contains("fn new()") {
            in_new = true;
        }
        if in_new && trimmed.starts_with(&pattern) {
            return Some(i + 1);
        }
        // Simple heuristic: stop at next fn or closing brace at indent 0
        if in_new && (trimmed.starts_with("fn ") && !trimmed.contains("fn new()")) {
            in_new = false;
        }
    }
    None
}

/// Extract the initialization expression from the new() constructor.
fn extract_init_expr(lines: &[&str], field_name: &str) -> Option<String> {
    let pattern = format!("{field_name}:");
    let mut in_new = false;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.contains("fn new()") {
            in_new = true;
        }
        if in_new && trimmed.starts_with(&pattern) {
            // Extract everything after "field_name:"
            let after_colon = trimmed[pattern.len()..].trim();
            // Remove trailing comma if present
            let expr = after_colon.trim_end_matches(',').trim();
            if !expr.is_empty() {
                return Some(expr.to_string());
            }
        }
        if in_new && trimmed.starts_with("fn ") && !trimmed.contains("fn new()") {
            in_new = false;
        }
    }
    None
}

/// Convert COBOL paragraph name to Rust function name (same logic as transpiler).
fn cobol_to_rust_fn_name(para: &str) -> String {
    para.to_lowercase().replace('-', "_")
}

/// Find the line number where a function definition starts.
fn find_fn_line(lines: &[&str], fn_name: &str) -> Option<usize> {
    let pattern = format!("fn {fn_name}(");
    for (i, line) in lines.iter().enumerate() {
        if line.contains(&pattern) {
            return Some(i + 1);
        }
    }
    None
}

/// Find the line range (start_idx, end_idx exclusive) of a function body.
fn find_fn_range(lines: &[&str], fn_name: &str) -> Option<(usize, usize)> {
    let pattern = format!("fn {fn_name}(");
    let mut start = None;
    for (i, line) in lines.iter().enumerate() {
        if line.contains(&pattern) {
            start = Some(i);
            break;
        }
    }
    let start = start?;

    // Find matching closing brace by tracking depth
    let mut depth = 0;
    let mut body_start = start;
    for (i, line) in lines.iter().enumerate().skip(start) {
        for ch in line.chars() {
            if ch == '{' {
                if depth == 0 {
                    body_start = i + 1;
                }
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    return Some((body_start, i));
                }
            }
        }
    }
    None
}

/// Count leading spaces in a line.
fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hints::{FieldHint, FileHints, ParagraphHint};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_local_field_hint(
        name: &str,
        paragraph: &str,
    ) -> (String, FieldHint) {
        (
            name.to_string(),
            FieldHint {
                pic: "S9(5)".to_string(),
                usage: "COMP-3".to_string(),
                level: 5,
                redefines: None,
                redefined_by: vec![],
                call_by_ref: false,
                move_corr_target: false,
                read_count: 3,
                write_count: 2,
                paragraph_scope: vec![paragraph.to_string()],
            },
        )
    }

    fn make_hints_with_locals(
        fields: Vec<(String, FieldHint)>,
        paragraph: &str,
        local_fields: Vec<&str>,
    ) -> FileHints {
        let mut paragraphs = HashMap::new();
        paragraphs.insert(
            paragraph.to_string(),
            ParagraphHint {
                performs: vec![],
                performed_by: vec![],
                local_only_fields: local_fields.iter().map(|s| s.to_string()).collect(),
            },
        );
        FileHints {
            cobol_source: "TEST.CBL".to_string(),
            fields: fields.into_iter().collect(),
            paragraphs,
            level_88_groups: HashMap::new(),
            call_targets: vec![],
            file_io_fields: vec![],
        }
    }

    #[test]
    fn localizes_single_paragraph_field() {
        let source = r#"pub struct WorkingStorage {
    pub ws_temp: PackedDecimal,
    pub ws_name: PicX,
}

impl WorkingStorage {
    pub fn new() -> Self {
        Self {
            ws_temp: PackedDecimal::new(5, 0, true),
            ws_name: PicX::new(30, b""),
        }
    }
}

fn validate_100(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    ws.ws_temp.pack(Decimal::from(42));
    println!("{}", ws.ws_temp.to_decimal());
}
"#;
        let hints = make_hints_with_locals(
            vec![make_local_field_hint("ws_temp", "VALIDATE-100")],
            "VALIDATE-100",
            vec!["ws_temp"],
        );

        let transforms = find_localizable_fields(&hints, source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_auto());
        assert!(transforms[0].description.contains("localize"));

        // Should have edits: blank struct decl, blank init, insert let, replace ws.field
        assert!(transforms[0].edits.len() >= 3);
    }

    #[test]
    fn skips_multi_paragraph_field() {
        let source = "fn main() {}\n";
        let hints = make_hints_with_locals(
            vec![(
                "ws_shared".to_string(),
                FieldHint {
                    pic: "S9(5)".to_string(),
                    usage: "COMP-3".to_string(),
                    level: 5,
                    redefines: None,
                    redefined_by: vec![],
                    call_by_ref: false,
                    move_corr_target: false,
                    read_count: 5,
                    write_count: 3,
                    paragraph_scope: vec!["PARA-A".to_string(), "PARA-B".to_string()],
                },
            )],
            "PARA-A",
            vec![],
        );

        let transforms = find_localizable_fields(&hints, source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn blocks_redefines_field() {
        let source = "fn main() {}\n";
        let mut fields = vec![make_local_field_hint("ws_temp", "VALIDATE-100")];
        fields[0].1.redefined_by = vec!["ws_overlay".to_string()];
        let hints = make_hints_with_locals(fields, "VALIDATE-100", vec!["ws_temp"]);

        let transforms = find_localizable_fields(&hints, source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].safety.is_review());
    }

    #[test]
    fn skips_dead_fields() {
        let source = "fn main() {}\n";
        let hints = make_hints_with_locals(
            vec![(
                "ws_dead".to_string(),
                FieldHint {
                    pic: "9(3)".to_string(),
                    usage: "DISPLAY".to_string(),
                    level: 5,
                    redefines: None,
                    redefined_by: vec![],
                    call_by_ref: false,
                    move_corr_target: false,
                    read_count: 0,
                    write_count: 0,
                    paragraph_scope: vec!["PARA-A".to_string()],
                },
            )],
            "PARA-A",
            vec!["ws_dead"],
        );

        let transforms = find_localizable_fields(&hints, source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn no_hints_no_transforms() {
        let rule = LocalizeVarsRule;
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
    fn cobol_fn_name_conversion() {
        assert_eq!(cobol_to_rust_fn_name("VALIDATE-100"), "validate_100");
        assert_eq!(cobol_to_rust_fn_name("MAIN-LOOP"), "main_loop");
    }

    #[test]
    fn find_fn_range_works() {
        let source = "fn foo() {\n    let x = 1;\n    let y = 2;\n}\n";
        let lines: Vec<&str> = source.lines().collect();
        let range = find_fn_range(&lines, "foo");
        assert!(range.is_some());
        let (start, end) = range.unwrap();
        assert_eq!(start, 1); // body starts on line index 1
        assert_eq!(end, 3); // closing brace on line index 3
    }
}
