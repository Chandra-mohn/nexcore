//! T1-DISPLAY: Simplify verbose display/format patterns.
//!
//! Targets patterns the transpiler emits for COBOL DISPLAY:
//! - `format!("{}", expr)` -> `expr.to_string()`
//! - `format!("{}", x.to_decimal())` -> `x.to_decimal().to_string()`
//!
//! These patterns appear when DISPLAY results are captured into
//! String variables rather than printed directly.

use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::rules::transform::{Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct DisplaySimplifyRule;

impl RustifyRule for DisplaySimplifyRule {
    fn id(&self) -> &'static str {
        "display-simplify"
    }

    fn description(&self) -> &'static str {
        "Simplify verbose format!/display patterns"
    }

    fn tier(&self) -> Tier {
        Tier::Tier1
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        find_display_simplifications(ctx.source, ctx.source_text, ctx.file_path)
    }
}

fn find_display_simplifications(
    parsed: &syn::File,
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let lines: Vec<&str> = source_text.lines().collect();
    let mut collector = FormatCollector {
        lines: &lines,
        transforms: Vec::new(),
        file_path,
    };
    collector.visit_file(parsed);
    collector.transforms
}

struct FormatCollector<'a> {
    lines: &'a [&'a str],
    transforms: Vec<Transform>,
    file_path: &'a std::path::Path,
}

impl<'ast> Visit<'ast> for FormatCollector<'_> {
    fn visit_expr_macro(&mut self, mac: &'ast syn::ExprMacro) {
        // Look for format!("{}", expr) patterns
        if mac.mac.path.is_ident("format") {
            let tokens_str = mac.mac.tokens.to_string();
            // Match: format!("{}", some_expr) or format!("{}" , some_expr)
            // The token stream normalizes whitespace, so we check trimmed form
            if parse_single_display_format(&tokens_str).is_some() {
                let line = mac.mac.span().start().line;
                if line > 0 && line <= self.lines.len() {
                    let line_text = self.lines[line - 1];
                    // Find the format!(...) span in the line
                    if let Some(start) = line_text.find("format!") {
                        // Find matching closing paren
                        if let Some(end) = find_matching_paren(line_text, start + 7) {
                            // Extract the inner expression from original source
                            // format!("{}", expr) -- find the comma after "{}"
                            let inner = &line_text[start + 7..=end]; // (\"{}\" , expr)
                            if let Some(comma_pos) = inner.find(',') {
                                let expr_text =
                                    inner[comma_pos + 1..inner.len() - 1].trim();
                                let replacement =
                                    format!("{expr_text}.to_string()");
                                self.transforms.push(Transform {
                                    rule_id: "display-simplify".to_string(),
                                    file: self.file_path.to_path_buf(),
                                    line,
                                    description:
                                        "format!(\"{}\", ...) -> .to_string()"
                                            .to_string(),
                                    safety: Safety::Auto,
                                    edits: vec![TextEdit {
                                        line,
                                        col_start: start,
                                        col_end: end + 1,
                                        new_text: replacement,
                                    }],
                                });
                            }
                        }
                    }
                }
            }
        }
        syn::visit::visit_expr_macro(self, mac);
    }
}

/// Parse `format!("{}", expr)` -- returns the inner expression if it matches
/// the single-placeholder Display pattern.
fn parse_single_display_format(tokens: &str) -> Option<String> {
    // Token stream from syn looks like: "\"{}\"" , expr
    // or: "\"{}\", expr"
    let trimmed = tokens.trim();

    // Must start with "{}" string literal
    if !trimmed.starts_with("\"{}\"") {
        return None;
    }

    let rest = trimmed["\"{}\"".len()..].trim();
    if !rest.starts_with(',') {
        return None;
    }

    let expr = rest[1..].trim();
    if expr.is_empty() {
        return None;
    }

    Some(expr.to_string())
}

/// Find the closing paren matching the one at `line[start]`.
fn find_matching_paren(line: &str, start: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    if start >= bytes.len() || bytes[start] != b'(' {
        return None;
    }
    let mut depth = 1;
    for (i, &b) in bytes.iter().enumerate().skip(start + 1) {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn analyze_source(source: &str) -> Vec<Transform> {
        let parsed = syn::parse_file(source).unwrap();
        find_display_simplifications(&parsed, source, &PathBuf::from("test.rs"))
    }

    #[test]
    fn simplifies_format_display() {
        let source = r#"fn foo() {
    let s = format!("{}", x.to_decimal());
}
"#;
        let transforms = analyze_source(source);
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("to_string()"));
    }

    #[test]
    fn ignores_multi_placeholder_format() {
        let source = r#"fn foo() {
    let s = format!("{} {}", a, b);
}
"#;
        let transforms = analyze_source(source);
        assert!(transforms.is_empty());
    }

    #[test]
    fn ignores_format_with_specifiers() {
        let source = r#"fn foo() {
    let s = format!("{:?}", x);
}
"#;
        let transforms = analyze_source(source);
        assert!(transforms.is_empty());
    }

    #[test]
    fn ignores_non_format_macros() {
        let source = r#"fn foo() {
    println!("{}", x);
}
"#;
        let transforms = analyze_source(source);
        assert!(transforms.is_empty());
    }

    #[test]
    fn handles_nested_method_calls() {
        let source = r#"fn foo() {
    let s = format!("{}", ws.ws_amount.to_decimal());
}
"#;
        let transforms = analyze_source(source);
        assert_eq!(transforms.len(), 1);
    }
}
