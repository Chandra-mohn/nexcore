//! T1-IMPORT: Remove unused `use` statements.
//!
//! Parses `use` statements, scans the file for references to imported items,
//! and removes imports with zero references.
//! Exception: keeps wildcard imports (e.g., `use cobol_runtime::prelude::*`).

use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::rules::transform::{Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct UnusedImportRule;

impl RustifyRule for UnusedImportRule {
    fn id(&self) -> &'static str {
        "unused-import"
    }

    fn description(&self) -> &'static str {
        "Remove unused `use` statements"
    }

    fn tier(&self) -> Tier {
        Tier::Tier1
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        find_unused_imports(ctx.source, ctx.source_text, ctx.file_path)
    }
}

/// Info about a use statement.
struct UseInfo {
    /// The imported name(s) - last segment or group.
    names: Vec<String>,
    /// Line number of the use statement.
    line: usize,
    /// Whether this is a wildcard import (e.g., `use foo::*`).
    is_glob: bool,
    /// Full line text for replacement.
    col_start: usize,
    col_end: usize,
}

fn find_unused_imports(
    parsed: &syn::File,
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let lines: Vec<&str> = source_text.lines().collect();
    let mut use_infos = Vec::new();

    // Collect use statements
    for item in &parsed.items {
        if let syn::Item::Use(use_item) = item {
            let line = use_item.span().start().line;
            let mut names = Vec::new();
            let mut is_glob = false;
            collect_use_names(&use_item.tree, &mut names, &mut is_glob);

            let col_end = if line > 0 && line <= lines.len() {
                lines[line - 1].len()
            } else {
                0
            };

            use_infos.push(UseInfo {
                names,
                line,
                is_glob,
                col_start: 0,
                col_end,
            });
        }
    }

    // Collect all identifiers used in the file (excluding use statements themselves)
    let mut ident_collector = IdentCollector {
        idents: std::collections::HashSet::new(),
    };
    for item in &parsed.items {
        if !matches!(item, syn::Item::Use(_)) {
            ident_collector.visit_item(item);
        }
    }
    // Also scan for identifiers in file-level attributes
    for attr in &parsed.attrs {
        ident_collector.visit_attribute(attr);
    }

    let used_idents = ident_collector.idents;

    // Find unused imports
    let mut transforms = Vec::new();
    for info in &use_infos {
        // Skip wildcard imports
        if info.is_glob {
            continue;
        }

        // Check if any imported name is referenced
        let any_used = info.names.iter().any(|name| used_idents.contains(name));
        if !any_used {
            transforms.push(Transform {
                rule_id: "unused-import".to_string(),
                file: file_path.to_path_buf(),
                line: info.line,
                description: format!(
                    "unused import: {}",
                    info.names.join(", ")
                ),
                safety: Safety::Auto,
                edits: vec![TextEdit {
                    line: info.line,
                    col_start: info.col_start,
                    col_end: info.col_end,
                    new_text: String::new(), // Remove the entire line
                }],
            });
        }
    }

    transforms
}

/// Extract imported names from a use tree.
fn collect_use_names(tree: &syn::UseTree, names: &mut Vec<String>, is_glob: &mut bool) {
    match tree {
        syn::UseTree::Path(p) => collect_use_names(&p.tree, names, is_glob),
        syn::UseTree::Name(n) => names.push(n.ident.to_string()),
        syn::UseTree::Rename(r) => names.push(r.rename.to_string()),
        syn::UseTree::Glob(_) => *is_glob = true,
        syn::UseTree::Group(g) => {
            for item in &g.items {
                collect_use_names(item, names, is_glob);
            }
        }
    }
}

/// Collects all identifier names used in the AST.
struct IdentCollector {
    idents: std::collections::HashSet<String>,
}

impl<'ast> Visit<'ast> for IdentCollector {
    fn visit_ident(&mut self, ident: &'ast proc_macro2::Ident) {
        self.idents.insert(ident.to_string());
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        // Record last segment of paths (the actual type/function name)
        if let Some(seg) = path.segments.last() {
            self.idents.insert(seg.ident.to_string());
        }
        syn::visit::visit_path(self, path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn analyze_source(source: &str) -> Vec<Transform> {
        let parsed = syn::parse_file(source).unwrap();
        find_unused_imports(&parsed, source, &PathBuf::from("test.rs"))
    }

    #[test]
    fn keeps_used_import() {
        let source = r#"use std::collections::HashMap;

fn foo() -> HashMap<String, i32> {
    HashMap::new()
}
"#;
        let transforms = analyze_source(source);
        assert!(transforms.is_empty(), "Used import should not be flagged");
    }

    #[test]
    fn removes_unused_import() {
        let source = r#"use std::collections::HashMap;
use std::collections::BTreeMap;

fn foo() -> HashMap<String, i32> {
    HashMap::new()
}
"#;
        let transforms = analyze_source(source);
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("BTreeMap"));
    }

    #[test]
    fn keeps_wildcard_imports() {
        let source = r#"use cobol_runtime::prelude::*;

fn foo() {}
"#;
        let transforms = analyze_source(source);
        assert!(transforms.is_empty(), "Wildcard imports should be kept");
    }

    #[test]
    fn handles_renamed_imports() {
        let source = r#"use std::collections::HashMap as Map;

fn foo() -> Map<String, i32> {
    Map::new()
}
"#;
        let transforms = analyze_source(source);
        assert!(transforms.is_empty(), "Renamed import used as Map");
    }

    #[test]
    fn removes_multiple_unused() {
        let source = r#"use std::io::Read;
use std::io::Write;
use std::fmt::Display;

fn foo() {}
"#;
        let transforms = analyze_source(source);
        assert_eq!(transforms.len(), 3, "All 3 imports are unused");
    }

    #[test]
    fn detects_sql_import_unused() {
        let source = r#"use cobol_runtime::prelude::*;
use cobol_sql::DuckDbRuntime;

fn foo() {}
"#;
        let transforms = analyze_source(source);
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("DuckDbRuntime"));
    }
}
