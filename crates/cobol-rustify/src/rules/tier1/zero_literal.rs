//! T1-ZERO: Replace zero decimal parsing with Decimal::ZERO.
//!
//! Matches: `"0".parse::<Decimal>().unwrap()` and variants
//! ("00", "0.0", "0.00", "000", etc.)
//! Replaces with: `Decimal::ZERO`

use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::rules::transform::{Safety, TextEdit, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct ZeroLiteralRule;

impl RustifyRule for ZeroLiteralRule {
    fn id(&self) -> &'static str {
        "zero-literal"
    }

    fn description(&self) -> &'static str {
        "Replace \"0\".parse::<Decimal>().unwrap() with Decimal::ZERO"
    }

    fn tier(&self) -> Tier {
        Tier::Tier1
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        find_zero_literals(ctx.source_text, ctx.file_path)
    }
}

struct ZeroLiteralVisitor {
    file_path: std::path::PathBuf,
    transforms: Vec<Transform>,
}

impl ZeroLiteralVisitor {
    /// Check if a string literal represents zero.
    fn is_zero_literal(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        // Strip leading sign
        let s = s.strip_prefix('-').or_else(|| s.strip_prefix('+')).unwrap_or(s);
        // All chars must be '0' or '.'
        !s.is_empty() && s.chars().all(|c| c == '0' || c == '.')
            && s.chars().any(|c| c == '0')
    }
}

impl<'ast> Visit<'ast> for ZeroLiteralVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        // Match pattern: "...".parse::<Decimal>().unwrap()
        // Structure: MethodCall { receiver: MethodCall { receiver: Lit, method: parse }, method: unwrap }
        if node.method == "unwrap" && node.args.is_empty() {
            if let syn::Expr::MethodCall(inner) = node.receiver.as_ref() {
                if inner.method == "parse" && is_decimal_turbofish(inner.turbofish.as_ref()) {
                    if let syn::Expr::Lit(lit) = inner.receiver.as_ref() {
                        if let syn::Lit::Str(s) = &lit.lit {
                            if ZeroLiteralVisitor::is_zero_literal(&s.value()) {
                                let lit_start = lit.lit.span().start();
                                let end = node.span().end();

                                // Only handle single-line expressions
                                if lit_start.line == end.line {
                                    self.transforms.push(Transform {
                                        rule_id: "zero-literal".to_string(),
                                        file: self.file_path.clone(),
                                        line: lit_start.line,
                                        description: format!(
                                            "\"{}\" -> Decimal::ZERO",
                                            s.value()
                                        ),
                                        safety: Safety::Auto,
                                        edits: vec![TextEdit {
                                            line: lit_start.line,
                                            col_start: lit_start.column,
                                            col_end: end.column,
                                            new_text: "Decimal::ZERO".to_string(),
                                        }],
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Continue visiting child nodes
        syn::visit::visit_expr_method_call(self, node);
    }
}

/// Check if turbofish contains `Decimal` type.
pub fn is_decimal_turbofish_pub(
    turbofish: Option<&syn::AngleBracketedGenericArguments>,
) -> bool {
    is_decimal_turbofish(turbofish)
}

fn is_decimal_turbofish(turbofish: Option<&syn::AngleBracketedGenericArguments>) -> bool {
    let Some(args) = turbofish else {
        return false;
    };
    args.args.iter().any(|arg| {
        if let syn::GenericArgument::Type(syn::Type::Path(tp)) = arg {
            tp.path.segments.last().is_some_and(|s| s.ident == "Decimal")
        } else {
            false
        }
    })
}

/// Parse source text and find zero-literal transforms (standalone helper for testing).
pub fn find_zero_literals(source: &str, file_path: &std::path::Path) -> Vec<Transform> {
    let Ok(parsed) = syn::parse_file(source) else {
        return Vec::new();
    };

    let mut visitor = ZeroLiteralVisitor {
        file_path: file_path.to_path_buf(),
        transforms: Vec::new(),
    };
    visitor.visit_file(&parsed);
    visitor.transforms
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn detects_zero_literal() {
        let source = r#"fn foo() {
    let x = "0".parse::<Decimal>().unwrap();
}
"#;
        let transforms = find_zero_literals(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert_eq!(transforms[0].rule_id, "zero-literal");
        assert_eq!(transforms[0].edits.len(), 1);
        assert_eq!(transforms[0].edits[0].new_text, "Decimal::ZERO");
    }

    #[test]
    fn detects_zero_variants() {
        let source = r#"fn foo() {
    let a = "0".parse::<Decimal>().unwrap();
    let b = "00".parse::<Decimal>().unwrap();
    let c = "0.0".parse::<Decimal>().unwrap();
    let d = "0.00".parse::<Decimal>().unwrap();
    let e = "000".parse::<Decimal>().unwrap();
}
"#;
        let transforms = find_zero_literals(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 5, "Should detect all zero variants");
    }

    #[test]
    fn ignores_non_zero() {
        let source = r#"fn foo() {
    let x = "1".parse::<Decimal>().unwrap();
    let y = "0.5".parse::<Decimal>().unwrap();
    let z = "100".parse::<Decimal>().unwrap();
}
"#;
        let transforms = find_zero_literals(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 0, "Should not match non-zero literals");
    }

    #[test]
    fn ignores_non_decimal_parse() {
        let source = r#"fn foo() {
    let x = "0".parse::<i32>().unwrap();
    let y = "0".parse::<f64>().unwrap();
}
"#;
        let transforms = find_zero_literals(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 0, "Should only match Decimal");
    }

    #[test]
    fn is_zero_checks() {
        assert!(ZeroLiteralVisitor::is_zero_literal("0"));
        assert!(ZeroLiteralVisitor::is_zero_literal("00"));
        assert!(ZeroLiteralVisitor::is_zero_literal("0.0"));
        assert!(ZeroLiteralVisitor::is_zero_literal("0.00"));
        assert!(ZeroLiteralVisitor::is_zero_literal("-0"));
        assert!(ZeroLiteralVisitor::is_zero_literal("+0.0"));
        assert!(!ZeroLiteralVisitor::is_zero_literal("1"));
        assert!(!ZeroLiteralVisitor::is_zero_literal("0.5"));
        assert!(!ZeroLiteralVisitor::is_zero_literal(""));
        assert!(!ZeroLiteralVisitor::is_zero_literal("."));
    }

    #[test]
    fn edit_produces_valid_rust() {
        use crate::rules::transform::apply_edits;

        let source = r#"fn foo() -> Decimal {
    "0".parse::<Decimal>().unwrap()
}
"#;
        let transforms = find_zero_literals(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);

        let edits: Vec<_> = transforms.iter().flat_map(|t| &t.edits).cloned().collect();
        let result = apply_edits(source, &edits);

        // Should parse as valid Rust
        assert!(syn::parse_file(&result).is_ok(), "Result should be valid Rust: {result}");
        assert!(result.contains("Decimal::ZERO"));
    }
}
