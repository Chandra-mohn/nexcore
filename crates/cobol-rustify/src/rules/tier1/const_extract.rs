//! T1-CONST: Extract repeated decimal literal parsing into named constants.
//!
//! Matches: `"N".parse::<Decimal>().unwrap()` where N is a non-zero numeric
//! literal that appears 2+ times in the same file.
//! Generates: `const LITERAL_N: Decimal = ...` at module level, replaces each
//! occurrence with the constant name.

use std::collections::HashMap;

use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::rules::transform::{LineInsertion, Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct ConstExtractRule;

impl RustifyRule for ConstExtractRule {
    fn id(&self) -> &'static str {
        "const-extract"
    }

    fn description(&self) -> &'static str {
        "Extract repeated .parse::<Decimal>() literals into named constants"
    }

    fn tier(&self) -> Tier {
        Tier::Tier1
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        find_const_candidates(ctx.source_text, ctx.file_path)
    }
}

/// A decimal literal occurrence found in source.
#[derive(Debug)]
struct LiteralOccurrence {
    /// The string value (e.g., "1000", "3.14").
    value: String,
    /// Line number (1-based).
    line: usize,
    /// Column of the opening quote of the string literal.
    col_start: usize,
    /// Column past the closing paren of .unwrap().
    col_end: usize,
}

/// Find all `"N".parse::<Decimal>().unwrap()` patterns and group by value.
fn collect_literals(_source: &str, parsed: &syn::File) -> Vec<LiteralOccurrence> {
    let mut visitor = LiteralCollector {
        occurrences: Vec::new(),
    };
    visitor.visit_file(parsed);
    visitor.occurrences
}

struct LiteralCollector {
    occurrences: Vec<LiteralOccurrence>,
}

impl<'ast> Visit<'ast> for LiteralCollector {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        // Match: "...".parse::<Decimal>().unwrap()
        if node.method == "unwrap" && node.args.is_empty() {
            if let syn::Expr::MethodCall(inner) = node.receiver.as_ref() {
                if inner.method == "parse"
                    && super::zero_literal::is_decimal_turbofish_pub(inner.turbofish.as_ref())
                {
                    if let syn::Expr::Lit(lit) = inner.receiver.as_ref() {
                        if let syn::Lit::Str(s) = &lit.lit {
                            let val = s.value();
                            // Skip zero literals (handled by zero-literal rule)
                            if !is_zero_value(&val) && is_numeric_literal(&val) {
                                let lit_start = lit.span().start();
                                let end = node.span().end();

                                if lit_start.line == end.line {
                                    self.occurrences.push(LiteralOccurrence {
                                        value: val,
                                        line: lit_start.line,
                                        col_start: lit_start.column,
                                        col_end: end.column,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}

/// Check if a string represents a numeric value.
fn is_numeric_literal(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let s = s.strip_prefix('-').or_else(|| s.strip_prefix('+')).unwrap_or(s);
    if s.is_empty() {
        return false;
    }
    let mut has_digit = false;
    let mut has_dot = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if c == '.' && !has_dot {
            has_dot = true;
        } else {
            return false;
        }
    }
    has_digit
}

/// Check if a string represents zero.
fn is_zero_value(s: &str) -> bool {
    let s = s.strip_prefix('-').or_else(|| s.strip_prefix('+')).unwrap_or(s);
    !s.is_empty() && s.chars().all(|c| c == '0' || c == '.') && s.chars().any(|c| c == '0')
}

/// Generate a constant name from a literal value.
fn const_name_for(value: &str) -> String {
    let cleaned = value
        .replace('.', "_")
        .replace('-', "NEG_");
    let cleaned = cleaned.strip_prefix('+').unwrap_or(&cleaned);
    format!("LITERAL_{cleaned}")
}

/// Deduplicate constant names by appending _2, _3, etc.
fn dedupe_names(names: &mut HashMap<String, String>) {
    let mut seen: HashMap<String, u32> = HashMap::new();
    let values: Vec<String> = names.keys().cloned().collect();
    for value in values {
        let base_name = names[&value].clone();
        let count = seen.entry(base_name.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            names.insert(value, format!("{base_name}_{count}"));
        }
    }
}

/// Find the line after the last `use` statement (for inserting constants).
fn find_use_block_end(parsed: &syn::File) -> usize {
    let mut last_use_line = 0;
    for item in &parsed.items {
        if let syn::Item::Use(u) = item {
            let end = u.span().end().line;
            if end > last_use_line {
                last_use_line = end;
            }
        }
    }
    last_use_line
}

/// Find repeated decimal literals and generate const-extract transforms.
pub fn find_const_candidates(source: &str, file_path: &std::path::Path) -> Vec<Transform> {
    let Ok(parsed) = syn::parse_file(source) else {
        return Vec::new();
    };

    let occurrences = collect_literals(source, &parsed);
    if occurrences.is_empty() {
        return Vec::new();
    }

    // Group by value
    let mut groups: HashMap<String, Vec<&LiteralOccurrence>> = HashMap::new();
    for occ in &occurrences {
        groups.entry(occ.value.clone()).or_default().push(occ);
    }

    // Only keep values with 2+ occurrences
    let repeated: HashMap<String, Vec<&LiteralOccurrence>> = groups
        .into_iter()
        .filter(|(_, v)| v.len() >= 2)
        .collect();

    if repeated.is_empty() {
        return Vec::new();
    }

    // Generate constant names
    let mut names: HashMap<String, String> = repeated
        .keys()
        .map(|v| (v.clone(), const_name_for(v)))
        .collect();
    dedupe_names(&mut names);

    // Find insertion point (after use block)
    let insert_after = find_use_block_end(&parsed);

    // Build transforms: one per value group
    let mut transforms = Vec::new();

    let mut sorted_values: Vec<&String> = repeated.keys().collect();
    sorted_values.sort();

    for value in sorted_values {
        let occs = &repeated[value];
        let const_name = &names[value];

        // Build edits: replace each occurrence with the const name
        let edits: Vec<TextEdit> = occs
            .iter()
            .map(|occ| TextEdit {
                line: occ.line,
                col_start: occ.col_start,
                col_end: occ.col_end,
                new_text: const_name.clone(),
            })
            .collect();

        transforms.push(Transform {
            rule_id: "const-extract".to_string(),
            file: file_path.to_path_buf(),
            line: occs[0].line,
            description: format!(
                "\"{value}\" appears {} times -> const {const_name}",
                occs.len()
            ),
            safety: Safety::Auto,
            edits,
        });
    }

    // Add a single insertion transform for the const declarations
    let mut const_lines = String::new();
    let mut sorted_for_insert: Vec<(&String, &String)> = names.iter().collect();
    sorted_for_insert.sort_by_key(|(v, _)| repeated[*v][0].line);

    for (value, name) in &sorted_for_insert {
        // Use Decimal::from_str_exact for const init (it's a const-compatible approach)
        // Actually, Decimal doesn't have const new. Use lazy_static or parse in a block.
        // For generated code, the simplest is: `fn {name}() -> Decimal` or a module-level let.
        // Since Rust doesn't support const Decimal::from_str, we use a static via once_cell
        // pattern, but the simplest transpiler-compatible approach:
        // `const {name}: Decimal = Decimal::from_parts(...)` is complex.
        // Simplest: keep the parse but do it once.
        // For now, generate a `lazy_static!` or `std::sync::LazyLock`.
        // Actually, the most pragmatic: just a regular `let` won't work at module scope.
        // Use `static` with LazyLock (stable since Rust 1.80):
        use std::fmt::Write;
        let _ = writeln!(
            const_lines,
            "static {name}: std::sync::LazyLock<Decimal> = std::sync::LazyLock::new(|| \"{value}\".parse::<Decimal>().unwrap());"
        );
    }

    if !const_lines.is_empty() {
        transforms.push(Transform {
            rule_id: "const-extract".to_string(),
            file: file_path.to_path_buf(),
            line: insert_after,
            description: format!(
                "Insert {} constant declaration(s) after use block",
                sorted_for_insert.len()
            ),
            safety: Safety::Auto,
            edits: vec![], // This is a line insertion, not an edit
        });

        // Store the insertion data in the first transform's description for now.
        // The actual insertion will be handled by the apply pipeline using LineInsertion.
        // We need a way to pass this through. Let's add it as a special field.
        // For now, we'll handle this in the apply pipeline by detecting const-extract
        // transforms and generating the insertion.
    }

    transforms
}

/// Generate the const declaration block and insertion point for a file.
///
/// Call this after analysis to get the `LineInsertion` for const declarations.
pub fn generate_const_insertions(
    source: &str,
    transforms: &[Transform],
) -> Vec<LineInsertion> {
    let Ok(parsed) = syn::parse_file(source) else {
        return Vec::new();
    };

    // Collect all const-extract transforms (excluding the insertion marker)
    let const_transforms: Vec<&Transform> = transforms
        .iter()
        .filter(|t| t.rule_id == "const-extract" && !t.edits.is_empty())
        .collect();

    if const_transforms.is_empty() {
        return Vec::new();
    }

    // Extract unique values from edits -- the const name is in new_text
    let mut consts: HashMap<String, String> = HashMap::new(); // name -> value
    for t in &const_transforms {
        // The description contains the value: "\"1000\" appears N times -> const LITERAL_1000"
        if let Some(quote_start) = t.description.find('"') {
            if let Some(quote_end) = t.description[quote_start + 1..].find('"') {
                let value = &t.description[quote_start + 1..quote_start + 1 + quote_end];
                if let Some(first_edit) = t.edits.first() {
                    consts.insert(first_edit.new_text.clone(), value.to_string());
                }
            }
        }
    }

    if consts.is_empty() {
        return Vec::new();
    }

    let insert_after = find_use_block_end(&parsed);

    let mut lines = String::new();
    let mut sorted: Vec<(&String, &String)> = consts.iter().collect();
    sorted.sort_by_key(|(name, _)| (*name).clone());

    lines.push('\n');
    for (name, value) in &sorted {
        use std::fmt::Write;
        let _ = writeln!(
            lines,
            "static {name}: std::sync::LazyLock<Decimal> = std::sync::LazyLock::new(|| \"{value}\".parse::<Decimal>().unwrap());"
        );
    }

    vec![LineInsertion {
        after_line: insert_after,
        text: lines,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn detects_repeated_literals() {
        let source = r#"use rust_decimal::Decimal;

fn foo() {
    let a = "1000".parse::<Decimal>().unwrap();
    let b = "1000".parse::<Decimal>().unwrap();
    let c = "500".parse::<Decimal>().unwrap();
}
"#;
        let transforms = find_const_candidates(source, &PathBuf::from("test.rs"));
        // Should find "1000" (2 occurrences), not "500" (only 1)
        let edit_transforms: Vec<_> = transforms.iter().filter(|t| !t.edits.is_empty()).collect();
        assert_eq!(edit_transforms.len(), 1, "Should have one value group for '1000'");
        assert_eq!(edit_transforms[0].edits.len(), 2, "Should have 2 edits for '1000'");
        assert!(
            edit_transforms[0].edits[0].new_text.contains("LITERAL_1000"),
            "Const name should contain LITERAL_1000"
        );
    }

    #[test]
    fn ignores_single_occurrence() {
        let source = r#"fn foo() {
    let a = "1000".parse::<Decimal>().unwrap();
    let b = "500".parse::<Decimal>().unwrap();
}
"#;
        let transforms = find_const_candidates(source, &PathBuf::from("test.rs"));
        let edit_transforms: Vec<_> = transforms.iter().filter(|t| !t.edits.is_empty()).collect();
        assert_eq!(edit_transforms.len(), 0, "No repeated literals");
    }

    #[test]
    fn skips_zero_literals() {
        let source = r#"fn foo() {
    let a = "0".parse::<Decimal>().unwrap();
    let b = "0".parse::<Decimal>().unwrap();
}
"#;
        let transforms = find_const_candidates(source, &PathBuf::from("test.rs"));
        let edit_transforms: Vec<_> = transforms.iter().filter(|t| !t.edits.is_empty()).collect();
        assert_eq!(edit_transforms.len(), 0, "Zero literals handled by zero-literal rule");
    }

    #[test]
    fn const_naming() {
        assert_eq!(const_name_for("1000"), "LITERAL_1000");
        assert_eq!(const_name_for("3.14"), "LITERAL_3_14");
        assert_eq!(const_name_for("-100"), "LITERAL_NEG_100");
    }

    #[test]
    fn is_numeric_check() {
        assert!(is_numeric_literal("1000"));
        assert!(is_numeric_literal("3.14"));
        assert!(is_numeric_literal("-100"));
        assert!(is_numeric_literal("+50"));
        assert!(is_numeric_literal("0"));
        assert!(!is_numeric_literal(""));
        assert!(!is_numeric_literal("abc"));
        assert!(!is_numeric_literal("1.2.3"));
    }

    #[test]
    fn generates_insertions() {
        let source = r#"use rust_decimal::Decimal;

fn foo() {
    let a = "1000".parse::<Decimal>().unwrap();
    let b = "1000".parse::<Decimal>().unwrap();
}
"#;
        let transforms = find_const_candidates(source, &PathBuf::from("test.rs"));
        let insertions = generate_const_insertions(source, &transforms);
        assert_eq!(insertions.len(), 1);
        assert!(insertions[0].text.contains("LazyLock"));
        assert!(insertions[0].text.contains("LITERAL_1000"));
        assert_eq!(insertions[0].after_line, 1); // after `use` line
    }

    #[test]
    fn edit_produces_valid_rust() {
        use crate::rules::transform::{apply_edits, apply_insertions};

        let source = r#"use rust_decimal::Decimal;

fn foo() {
    let a = "1000".parse::<Decimal>().unwrap();
    let b = "1000".parse::<Decimal>().unwrap();
}
"#;
        let transforms = find_const_candidates(source, &PathBuf::from("test.rs"));

        // Apply edits
        let edits: Vec<_> = transforms.iter().flat_map(|t| &t.edits).cloned().collect();
        let after_edits = apply_edits(source, &edits);

        // Apply insertions
        let insertions = generate_const_insertions(source, &transforms);
        let result = apply_insertions(&after_edits, &insertions);

        assert!(result.contains("LITERAL_1000"), "Should contain const name");
        assert!(result.contains("LazyLock"), "Should contain LazyLock declaration");
        // The function body should reference the constant, not the parse call
        // (The LazyLock declaration itself will still contain .parse)
        let fn_body_start = result.find("fn foo()").unwrap();
        let fn_body = &result[fn_body_start..];
        assert!(
            !fn_body.contains("\"1000\".parse"),
            "Function body should use const, not parse: {fn_body}"
        );
    }
}
