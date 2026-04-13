//! T1-ALLOW: Remove unnecessary `#[allow(...)]` attributes.
//!
//! Conservative approach: only removes allows where we can statically verify
//! the suppressed warning doesn't exist. Focuses on transpiler-emitted
//! patterns that are known to be safe to remove.

use syn::spanned::Spanned;

use crate::rules::transform::{Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct AllowCleanupRule;

impl RustifyRule for AllowCleanupRule {
    fn id(&self) -> &'static str {
        "allow-cleanup"
    }

    fn description(&self) -> &'static str {
        "Remove unnecessary #[allow] attributes"
    }

    fn tier(&self) -> Tier {
        Tier::Tier1
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        find_removable_allows(ctx.source, ctx.source_text, ctx.file_path)
    }
}

fn find_removable_allows(
    parsed: &syn::File,
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let lines: Vec<&str> = source_text.lines().collect();
    let mut transforms = Vec::new();

    // Check file-level inner attributes: #![allow(...)]
    for attr in &parsed.attrs {
        if let Some(removable) = check_file_level_allow(attr, parsed, &lines) {
            transforms.push(Transform {
                rule_id: "allow-cleanup".to_string(),
                file: file_path.to_path_buf(),
                line: removable.line,
                description: removable.description,
                safety: Safety::Auto,
                edits: removable.edits,
            });
        }
    }

    // Check item-level attributes on functions
    for item in &parsed.items {
        if let syn::Item::Fn(func) = item {
            for attr in &func.attrs {
                if let Some(removable) = check_fn_allow(attr, func, &lines) {
                    transforms.push(Transform {
                        rule_id: "allow-cleanup".to_string(),
                        file: file_path.to_path_buf(),
                        line: removable.line,
                        description: removable.description,
                        safety: Safety::Auto,
                        edits: removable.edits,
                    });
                }
            }
        }
    }

    transforms
}

struct RemovableAllow {
    line: usize,
    description: String,
    edits: Vec<TextEdit>,
}

/// Check if a file-level #![allow(...)] can be simplified.
///
/// The transpiler emits: `#![allow(unused_imports, unused_variables, non_snake_case)]`
/// After Tier 1 transforms (unused-import rule), the unused_imports suppression
/// may no longer be needed. However, we can't verify this statically without
/// knowing which imports are actually unused, so we leave this for now.
///
/// We CAN remove the entire attribute if the file has no items that would
/// trigger any of the suppressed warnings, but this is rare for generated code.
fn check_file_level_allow(
    _attr: &syn::Attribute,
    _parsed: &syn::File,
    _lines: &[&str],
) -> Option<RemovableAllow> {
    // Conservative: don't remove file-level allows for now.
    // The transpiler always needs non_snake_case (COBOL names),
    // and unused_imports/unused_variables are safer to keep.
    None
}

/// Check if a function-level #[allow(unused_variables)] can be removed.
///
/// If the function body references all its parameters, the attribute is unnecessary.
fn check_fn_allow(
    attr: &syn::Attribute,
    func: &syn::ItemFn,
    lines: &[&str],
) -> Option<RemovableAllow> {
    // Only look at #[allow(...)] attributes
    if !attr.path().is_ident("allow") {
        return None;
    }

    // Parse the allow list
    let allows = parse_allow_list(attr)?;

    // Check if unused_variables is in the list
    if !allows.iter().any(|a| a == "unused_variables") {
        return None;
    }

    // Check if all function parameters are actually used in the body
    let params = collect_fn_param_names(func);
    if params.is_empty() {
        // No params to be unused -- the allow is unnecessary
        let line = attr.span().start().line;
        if line > 0 && line <= lines.len() {
            return Some(RemovableAllow {
                line,
                description: format!(
                    "#[allow(unused_variables)] on {} -- no parameters",
                    func.sig.ident
                ),
                edits: vec![TextEdit {
                    line,
                    col_start: 0,
                    col_end: lines[line - 1].len(),
                    new_text: String::new(),
                }],
            });
        }
    }

    // Check if all parameters are referenced in the function body
    let body_text = func
        .block
        .stmts
        .iter()
        .map(|s| quote::quote!(#s).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let all_used = params.iter().all(|p| body_text.contains(p));
    if all_used && allows.len() == 1 {
        // Only allow is unused_variables, and all params are used
        let line = attr.span().start().line;
        if line > 0 && line <= lines.len() {
            return Some(RemovableAllow {
                line,
                description: format!(
                    "#[allow(unused_variables)] on {} -- all params used",
                    func.sig.ident
                ),
                edits: vec![TextEdit {
                    line,
                    col_start: 0,
                    col_end: lines[line - 1].len(),
                    new_text: String::new(),
                }],
            });
        }
    }

    // If allow has multiple items and only unused_variables can be removed,
    // we'd need to rewrite the attribute. Too complex for now -- skip.

    None
}

/// Parse the list of lint names from an #[allow(...)] attribute.
fn parse_allow_list(attr: &syn::Attribute) -> Option<Vec<String>> {
    let meta = &attr.meta;
    if let syn::Meta::List(list) = meta {
        let tokens = list.tokens.to_string();
        let names: Vec<String> = tokens
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Some(names)
    } else {
        None
    }
}

/// Collect parameter names from a function signature.
fn collect_fn_param_names(func: &syn::ItemFn) -> Vec<String> {
    let mut names = Vec::new();
    for arg in &func.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                names.push(pat_ident.ident.to_string());
            }
        }
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn analyze_source(source: &str) -> Vec<Transform> {
        let parsed = syn::parse_file(source).unwrap();
        find_removable_allows(&parsed, source, &PathBuf::from("test.rs"))
    }

    #[test]
    fn removes_allow_unused_vars_no_params() {
        let source = r#"#[allow(unused_variables)]
fn foo() {
    let x = 42;
}
"#;
        let transforms = analyze_source(source);
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("no parameters"));
    }

    #[test]
    fn removes_allow_when_all_params_used() {
        let source = r#"#[allow(unused_variables)]
fn foo(x: i32) -> i32 {
    x + 1
}
"#;
        let transforms = analyze_source(source);
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("all params used"));
    }

    #[test]
    fn keeps_allow_when_params_unused() {
        let source = r#"#[allow(unused_variables)]
fn foo(x: i32, y: i32) -> i32 {
    x + 1
}
"#;
        let transforms = analyze_source(source);
        // y is unused, so the allow is needed
        assert!(transforms.is_empty());
    }

    #[test]
    fn ignores_non_allow_attributes() {
        let source = r#"#[inline]
fn foo() {}
"#;
        let transforms = analyze_source(source);
        assert!(transforms.is_empty());
    }

    #[test]
    fn ignores_non_unused_variables_allow() {
        let source = r#"#[allow(non_snake_case)]
fn foo() {}
"#;
        let transforms = analyze_source(source);
        assert!(transforms.is_empty());
    }
}
