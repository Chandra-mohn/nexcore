//! Extraction of `#[cobol(...)]` attributes from syn AST.
//!
//! Parses the inert attributes emitted by Session D's transpiler codegen
//! into structured metadata for DSL emitters.

use std::collections::HashMap;

/// Struct-level `#[cobol(program = "...")]` metadata.
#[derive(Debug, Clone, Default)]
pub struct StructCobolAttr {
    pub program: String,
}

/// Field-level `#[cobol(level = 5, pic = "S9(9)V99", ...)]` metadata.
#[derive(Debug, Clone, Default)]
pub struct FieldCobolAttr {
    pub level: u8,
    pub pic: Option<String>,
    pub usage: Option<String>,
    pub offset: Option<usize>,
    pub len: Option<usize>,
    pub signed: bool,
    pub redefines: Option<String>,
    pub occurs: Option<usize>,
    pub occurs_depending: Option<String>,
    pub level88: Option<String>,
    pub blank_when_zero: bool,
    pub justified_right: bool,
}

/// Function-level `#[cobol(section = "...", performs = "...", reads = "...", writes = "...")]`.
#[derive(Debug, Clone, Default)]
pub struct FnCobolAttr {
    pub section: Option<String>,
    pub performs: Vec<String>,
    pub reads: Vec<String>,
    pub writes: Vec<String>,
}

/// A struct field with its name, type, and COBOL attribute.
#[derive(Debug, Clone)]
pub struct AnnotatedField {
    pub rust_name: String,
    pub rust_type: String,
    pub cobol_attr: Option<FieldCobolAttr>,
}

/// A function with its name and COBOL attribute.
#[derive(Debug, Clone)]
pub struct AnnotatedFn {
    pub name: String,
    pub cobol_attr: Option<FnCobolAttr>,
}

/// Extract the program name from `#[cobol(program = "...")]` on the WorkingStorage struct.
pub fn extract_struct_attr(attrs: &[syn::Attribute]) -> Option<StructCobolAttr> {
    for attr in attrs {
        if !is_cobol_attr(attr) {
            continue;
        }
        let tokens = attr_tokens_string(attr);
        if let Some(program) = extract_kv_string(&tokens, "program") {
            return Some(StructCobolAttr { program });
        }
    }
    None
}

/// Extract field-level COBOL metadata from `#[cobol(...)]`.
pub fn extract_field_attr(attrs: &[syn::Attribute]) -> Option<FieldCobolAttr> {
    for attr in attrs {
        if !is_cobol_attr(attr) {
            continue;
        }
        let tokens = attr_tokens_string(attr);
        let kv = parse_kv_pairs(&tokens);

        let level = kv
            .get("level")
            .and_then(|v| match v.parse::<u8>() {
                Ok(l) => Some(l),
                Err(_) => {
                    tracing::warn!(value = %v, "Invalid #[cobol(level=...)] value");
                    None
                }
            })
            .unwrap_or(0);
        if level == 0 && kv.is_empty() {
            return None;
        }

        return Some(FieldCobolAttr {
            level,
            pic: kv.get("pic").cloned(),
            usage: extract_usage_flag(&tokens),
            offset: kv.get("offset").and_then(|v| v.parse().ok()),
            len: kv.get("len").and_then(|v| v.parse().ok()),
            signed: tokens.contains("signed") && !tokens.contains("signed ="),
            redefines: kv.get("redefines").cloned(),
            occurs: kv.get("occurs").and_then(|v| v.parse().ok()),
            occurs_depending: kv.get("occurs_depending").cloned(),
            level88: kv.get("level88").cloned(),
            blank_when_zero: tokens.contains("blank_when_zero"),
            justified_right: tokens.contains("justified_right"),
        });
    }
    None
}

/// Extract function-level COBOL metadata from `#[cobol(...)]`.
pub fn extract_fn_attr(attrs: &[syn::Attribute]) -> Option<FnCobolAttr> {
    for attr in attrs {
        if !is_cobol_attr(attr) {
            continue;
        }
        let tokens = attr_tokens_string(attr);
        let kv = parse_kv_pairs(&tokens);

        let section = kv.get("section").cloned();
        let performs: Vec<String> = kv
            .get("performs")
            .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
            .unwrap_or_default();
        let reads: Vec<String> = kv
            .get("reads")
            .map(|s| s.split(',').map(|r| r.trim().to_string()).collect())
            .unwrap_or_default();
        let writes: Vec<String> = kv
            .get("writes")
            .map(|s| s.split(',').map(|w| w.trim().to_string()).collect())
            .unwrap_or_default();

        if section.is_none() && performs.is_empty() && reads.is_empty() && writes.is_empty() {
            continue;
        }

        return Some(FnCobolAttr {
            section,
            performs,
            reads,
            writes,
        });
    }
    None
}

/// Extract all annotated fields from the WorkingStorage struct in a syn::File.
pub fn extract_annotated_fields(file: &syn::File) -> Vec<AnnotatedField> {
    let mut fields = Vec::new();
    for item in &file.items {
        if let syn::Item::Struct(s) = item {
            if let syn::Fields::Named(named) = &s.fields {
                for field in &named.named {
                    let name = field
                        .ident
                        .as_ref()
                        .map(std::string::ToString::to_string)
                        .unwrap_or_default();
                    let ty = type_to_string(&field.ty);
                    let attr = extract_field_attr(&field.attrs);
                    fields.push(AnnotatedField {
                        rust_name: name,
                        rust_type: ty,
                        cobol_attr: attr,
                    });
                }
            }
        }
    }
    fields
}

/// Extract all annotated functions from a syn::File.
pub fn extract_annotated_fns(file: &syn::File) -> Vec<AnnotatedFn> {
    let mut fns = Vec::new();
    for item in &file.items {
        if let syn::Item::Fn(f) = item {
            let name = f.sig.ident.to_string();
            let attr = extract_fn_attr(&f.attrs);
            fns.push(AnnotatedFn {
                name,
                cobol_attr: attr,
            });
        }
    }
    fns
}

/// Extract the program name from the first `#[cobol(program = "...")]` in a syn::File.
pub fn extract_program_name(file: &syn::File) -> Option<String> {
    for item in &file.items {
        if let syn::Item::Struct(s) = item {
            if let Some(attr) = extract_struct_attr(&s.attrs) {
                return Some(attr.program);
            }
        }
    }
    None
}

// --- Internal helpers ---

fn is_cobol_attr(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("cobol")
}

fn attr_tokens_string(attr: &syn::Attribute) -> String {
    match &attr.meta {
        syn::Meta::List(list) => list.tokens.to_string(),
        _ => String::new(),
    }
}

/// Parse `key = value` pairs from attribute token string.
/// Handles: `key = "string"`, `key = 123`, bare flags like `comp3`, `signed`.
fn parse_kv_pairs(tokens: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    // Split on commas, but respect quotes
    let parts = split_respecting_quotes(tokens);
    for part in parts {
        let part = part.trim();
        if let Some(eq_pos) = part.find('=') {
            let key = part[..eq_pos].trim();
            let val = part[eq_pos + 1..].trim();
            // Strip surrounding quotes
            let val = val.trim_matches('"');
            map.insert(key.to_string(), val.to_string());
        }
    }
    map
}

fn split_respecting_quotes(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in s.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            ',' if !in_quotes => {
                parts.push(current.clone());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

fn extract_kv_string(tokens: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} = \"");
    if let Some(start) = tokens.find(&prefix) {
        let rest = &tokens[start + prefix.len()..];
        if let Some(end) = rest.find('"') {
            return Some(rest[..end].to_string());
        }
    }
    None
}

/// Extract usage flags (comp3, comp, comp5, comp1, comp2, index, pointer)
/// that appear as bare tokens (not `key = value`).
fn extract_usage_flag(tokens: &str) -> Option<String> {
    let flags = [
        "comp3", "comp5", "comp1", "comp2", "comp", "index", "pointer",
    ];
    for flag in flags {
        // Must appear as standalone token, not part of a key=value
        let pattern = flag.to_string();
        for part in split_respecting_quotes(tokens) {
            let trimmed = part.trim();
            if trimmed == pattern {
                return Some(flag.to_string());
            }
        }
    }
    None
}

fn type_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_struct_attr() {
        let code = r#"
            #[cobol(program = "CARDPROC")]
            pub struct WorkingStorage {
                pub x: i32,
            }
        "#;
        let file = syn::parse_str::<syn::File>(code).unwrap();
        let name = extract_program_name(&file);
        assert_eq!(name, Some("CARDPROC".to_string()));
    }

    #[test]
    fn parse_field_attr_full() {
        let code = r#"
            pub struct WS {
                #[cobol(level = 5, pic = "S9(9)V99", comp3, offset = 16, len = 6, signed)]
                pub credit_limit: PackedDecimal,
            }
        "#;
        let file = syn::parse_str::<syn::File>(code).unwrap();
        let fields = extract_annotated_fields(&file);
        assert_eq!(fields.len(), 1);
        let attr = fields[0].cobol_attr.as_ref().unwrap();
        assert_eq!(attr.level, 5);
        assert_eq!(attr.pic.as_deref(), Some("S9(9)V99"));
        assert_eq!(attr.usage.as_deref(), Some("comp3"));
        assert_eq!(attr.offset, Some(16));
        assert_eq!(attr.len, Some(6));
        assert!(attr.signed);
    }

    #[test]
    fn parse_field_attr_occurs() {
        let code = r#"
            pub struct WS {
                #[cobol(level = 5, pic = "X(15)", occurs = 5)]
                pub phones: Vec<PicX>,
            }
        "#;
        let file = syn::parse_str::<syn::File>(code).unwrap();
        let fields = extract_annotated_fields(&file);
        let attr = fields[0].cobol_attr.as_ref().unwrap();
        assert_eq!(attr.occurs, Some(5));
    }

    #[test]
    fn parse_field_attr_redefines() {
        let code = r#"
            pub struct WS {
                #[cobol(level = 5, redefines = "CARD-DATA", len = 100)]
                pub credit_data: PicX,
            }
        "#;
        let file = syn::parse_str::<syn::File>(code).unwrap();
        let fields = extract_annotated_fields(&file);
        let attr = fields[0].cobol_attr.as_ref().unwrap();
        assert_eq!(attr.redefines.as_deref(), Some("CARD-DATA"));
    }

    #[test]
    fn parse_field_attr_level88() {
        let code = r#"
            pub struct WS {
                #[cobol(level = 5, pic = "X(1)", level88 = "ACTIVE:A,CLOSED:C")]
                pub status: PicX,
            }
        "#;
        let file = syn::parse_str::<syn::File>(code).unwrap();
        let fields = extract_annotated_fields(&file);
        let attr = fields[0].cobol_attr.as_ref().unwrap();
        assert_eq!(attr.level88.as_deref(), Some("ACTIVE:A,CLOSED:C"));
    }

    #[test]
    fn parse_fn_attr() {
        let code = r#"
            #[cobol(section = "MONETARY", performs = "CALC-PARA", reads = "WS-A,WS-B", writes = "WS-C")]
            fn process_monetary(ws: &mut WS) {}
        "#;
        let file = syn::parse_str::<syn::File>(code).unwrap();
        let fns = extract_annotated_fns(&file);
        assert_eq!(fns.len(), 1);
        let attr = fns[0].cobol_attr.as_ref().unwrap();
        assert_eq!(attr.section.as_deref(), Some("MONETARY"));
        assert_eq!(attr.performs, vec!["CALC-PARA"]);
        assert_eq!(attr.reads, vec!["WS-A", "WS-B"]);
        assert_eq!(attr.writes, vec!["WS-C"]);
    }

    #[test]
    fn no_cobol_attr_returns_none() {
        let code = r#"
            pub struct WS {
                pub x: i32,
            }
        "#;
        let file = syn::parse_str::<syn::File>(code).unwrap();
        let fields = extract_annotated_fields(&file);
        assert_eq!(fields.len(), 1);
        assert!(fields[0].cobol_attr.is_none());
    }

    #[test]
    fn extract_fns_without_attr() {
        let code = r#"
            fn helper() {}
        "#;
        let file = syn::parse_str::<syn::File>(code).unwrap();
        let fns = extract_annotated_fns(&file);
        assert_eq!(fns.len(), 1);
        assert!(fns[0].cobol_attr.is_none());
    }
}
