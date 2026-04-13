//! Transform emitter: generates `.xform` files from paragraph functions.
//!
//! Reads `#[cobol(reads = "...", writes = "...", section = "...", performs = "...")]`
//! from paragraph functions and generates TransformDSL.g4-conformant output.
//!
//! Strategy:
//! - Paragraphs with reads+writes become `transform` or `transform_block`
//! - Section dispatchers (only call sub-paragraphs) become `compose sequential`
//! - Orchestrator paragraphs with PERFORMs become `compose` blocks
//! - Apply blocks attempt expression extraction from Rust body patterns

use std::collections::HashMap;

use super::cobol_attrs::{
    extract_annotated_fields, extract_annotated_fns, extract_program_name, AnnotatedField,
    AnnotatedFn, FieldCobolAttr,
};
use super::dsl_ast::*;
use super::{DslEmitter, DslFile, DslLayer, EmitterContext};

/// Emitter that produces `.xform` files from paragraph functions.
#[derive(Debug)]
pub struct TransformEmitter;

impl DslEmitter for TransformEmitter {
    fn id(&self) -> &'static str {
        "transform"
    }

    fn layer(&self) -> DslLayer {
        DslLayer::Transform
    }

    fn emit(&self, ctx: &EmitterContext<'_>) -> Vec<DslFile> {
        let fns = extract_annotated_fns(ctx.syn_file);
        let fields = extract_annotated_fields(ctx.syn_file);
        let program = extract_program_name(ctx.syn_file)
            .unwrap_or_else(|| ctx.program_name.clone());

        // Build field lookup: COBOL name -> (rust_name, FieldCobolAttr)
        let field_map = build_field_map(&fields);

        // Classify functions
        let mut transforms = Vec::new();
        for f in &fns {
            if let Some(classified) = classify_function(f, &field_map) {
                transforms.push(classified);
            }
        }

        if transforms.is_empty() {
            return vec![];
        }

        // Group by section for file organization
        let sections = group_by_section(&transforms);

        let mut dsl_files = Vec::new();
        for (section_name, section_transforms) in &sections {
            let (content, notes, confidence) =
                generate_transform_file(section_name, section_transforms, &program);
            let source_fns: Vec<String> = section_transforms
                .iter()
                .map(|t| t.cobol_name.clone())
                .collect();

            dsl_files.push(DslFile {
                path: format!(
                    "transform/{}.xform",
                    sanitize_identifier(section_name)
                ),
                content,
                confidence,
                notes,
                source_fields: source_fns,
            });
        }

        dsl_files
    }
}

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

/// Classification of a paragraph function for transform generation.
#[derive(Debug, Clone)]
pub struct ClassifiedTransform {
    /// COBOL paragraph name (original)
    pub cobol_name: String,
    /// Nexflow-valid transform name
    pub nexflow_name: String,
    /// Section this paragraph belongs to
    pub section: Option<String>,
    /// What kind of transform to generate
    pub kind: TransformKind,
    /// Fields this paragraph reads (COBOL names)
    pub reads: Vec<String>,
    /// Fields this paragraph writes (COBOL names)
    pub writes: Vec<String>,
    /// Other paragraphs this paragraph PERFORMs
    pub performs: Vec<String>,
}

/// How to render this transform in DSL.
#[derive(Debug, Clone)]
pub enum TransformKind {
    /// Single output field: `transform ... apply ... end`
    SingleField,
    /// Multiple output fields: `transform_block ... mappings ... end`
    MultiField,
    /// Only calls other paragraphs: `transform_block ... compose sequential ... end`
    Compose,
    /// Section dispatcher (calls its paragraphs sequentially)
    SectionDispatcher,
}


// ---------------------------------------------------------------------------
// Field map
// ---------------------------------------------------------------------------

fn build_field_map(fields: &[AnnotatedField]) -> HashMap<String, (String, FieldCobolAttr)> {
    let mut map = HashMap::new();
    for f in fields {
        if let Some(attr) = &f.cobol_attr {
            // Map COBOL name (uppercase with hyphens) to rust name + attr
            let cobol_name = rust_name_to_cobol(&f.rust_name);
            map.insert(cobol_name, (f.rust_name.clone(), attr.clone()));
        }
    }
    map
}

/// Convert a Rust snake_case name back to COBOL-style: ws_acct_number -> WS-ACCT-NUMBER
fn rust_name_to_cobol(name: &str) -> String {
    name.to_uppercase().replace('_', "-")
}

// ---------------------------------------------------------------------------
// Function classification
// ---------------------------------------------------------------------------

fn classify_function(
    f: &AnnotatedFn,
    _field_map: &HashMap<String, (String, FieldCobolAttr)>,
) -> Option<ClassifiedTransform> {
    let attr = f.cobol_attr.as_ref()?;

    let has_reads = !attr.reads.is_empty();
    let has_writes = !attr.writes.is_empty();
    let has_performs = !attr.performs.is_empty();

    // Skip boilerplate functions
    if f.name == "run" || f.name == "main" || f.name == "new" || f.name == "stop_run"
        || f.name == "goback"
    {
        return None;
    }

    // Determine kind
    let kind = if !has_reads && !has_writes && has_performs {
        // Section dispatcher or orchestrator -- only PERFORMs, no data access
        if is_section_name(&f.name) {
            TransformKind::SectionDispatcher
        } else {
            TransformKind::Compose
        }
    } else if has_writes && attr.writes.len() == 1 && !has_performs {
        TransformKind::SingleField
    } else if has_writes || has_reads {
        if has_performs {
            TransformKind::Compose
        } else {
            TransformKind::MultiField
        }
    } else {
        // No data flow attributes -- skip
        return None;
    };

    Some(ClassifiedTransform {
        cobol_name: rust_name_to_cobol(&f.name),
        nexflow_name: sanitize_identifier(&f.name),
        section: attr.section.clone(),
        kind,
        reads: attr.reads.clone(),
        writes: attr.writes.clone(),
        performs: attr.performs.clone(),
    })
}

/// Check if a function name looks like a section dispatcher.
/// Convention: section dispatchers match their section name in snake_case.
fn is_section_name(name: &str) -> bool {
    name.ends_with("_section")
}

// ---------------------------------------------------------------------------
// Grouping
// ---------------------------------------------------------------------------

/// Group transforms by section. Unsectioned transforms go under "program_misc".
pub fn group_by_section(transforms: &[ClassifiedTransform]) -> Vec<(String, Vec<&ClassifiedTransform>)> {
    let mut map: HashMap<String, Vec<&ClassifiedTransform>> = HashMap::new();

    for t in transforms {
        let key = t
            .section
            .as_deref().map_or_else(|| "program_misc".to_string(), sanitize_identifier);
        map.entry(key).or_default().push(t);
    }

    let mut result: Vec<(String, Vec<&ClassifiedTransform>)> = map.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

// ---------------------------------------------------------------------------
// DSL generation
// ---------------------------------------------------------------------------

pub fn generate_transform_file(
    section_name: &str,
    transforms: &[&ClassifiedTransform],
    program: &str,
) -> (String, Vec<String>, f64) {
    let mut items = Vec::new();
    let mut notes = Vec::new();

    for t in transforms {
        match t.kind {
            TransformKind::SingleField => {
                items.push(build_single_transform(t, &mut notes));
            }
            TransformKind::MultiField => {
                items.push(build_multi_transform(t, &mut notes));
            }
            TransformKind::Compose | TransformKind::SectionDispatcher => {
                items.push(build_compose_transform(t, &mut notes));
            }
        }
    }

    let file = TransformFile {
        comments: vec![
            Comment(format!("Generated by cobol2rust Nexflow emitter")),
            Comment(format!("Source: {program} section {section_name}")),
        ],
        imports: vec![ImportPath::schema(program)],
        items,
    };

    // Confidence is 1.0 -- output is grammar-valid by construction
    (file.to_text(), notes, 1.0)
}

/// Build IoSpec from a list of COBOL field names.
fn build_io_spec(fields: &[String]) -> IoSpec {
    if fields.is_empty() {
        IoSpec::Single(Ident::new("value"), FieldType::Integer(None))
    } else if fields.len() == 1 {
        IoSpec::Single(
            Ident::new(&cobol_to_snake(&fields[0])),
            cobol_name_to_type_hint(&fields[0]),
        )
    } else {
        IoSpec::Multi(
            fields
                .iter()
                .map(|f| IoField {
                    name: Ident::new(&cobol_to_snake(f)),
                    field_type: cobol_name_to_type_hint(f),
                })
                .collect(),
        )
    }
}

fn build_single_transform(
    t: &ClassifiedTransform,
    notes: &mut Vec<String>,
) -> TransformItem {
    let input = build_io_spec(&t.reads);
    if t.reads.is_empty() {
        notes.push(format!(
            "{}: no reads detected -- input type placeholder",
            t.cobol_name
        ));
    }

    let output = if t.writes.len() == 1 {
        IoSpec::Single(
            Ident::new(&cobol_to_snake(&t.writes[0])),
            cobol_name_to_type_hint(&t.writes[0]),
        )
    } else {
        IoSpec::Single(Ident::new("result"), FieldType::Integer(None))
    };

    // Apply block
    let apply = if t.writes.len() == 1 {
        vec![ApplyStmt::Assign(
            Ident::new(&cobol_to_snake(&t.writes[0])),
            Expr::Raw(generate_placeholder_expr(&t.reads)),
        )]
    } else {
        vec![ApplyStmt::Assign(
            Ident::new("result"),
            Expr::Lit(Literal::Int(0)),
        )]
    };
    notes.push(format!(
        "{}: apply block needs manual review -- expression extraction not yet implemented",
        t.cobol_name
    ));

    TransformItem::Transform(TransformDef {
        name: Ident::new(&t.nexflow_name),
        comment: Some(format!("COBOL paragraph: {}", t.cobol_name)),
        pure: true,
        input,
        output,
        apply,
    })
}

fn build_multi_transform(
    t: &ClassifiedTransform,
    notes: &mut Vec<String>,
) -> TransformItem {
    let (input, output) = if t.reads.is_empty() && t.writes.is_empty() {
        (
            IoSpec::Single(Ident::new("input"), FieldType::Integer(None)),
            IoSpec::Single(Ident::new("output"), FieldType::Integer(None)),
        )
    } else {
        (build_io_spec(&t.reads), build_io_spec(&t.writes))
    };

    let mappings: Vec<MappingEntry> = t
        .writes
        .iter()
        .map(|w| {
            let field_name = Ident::new(&cobol_to_snake(w));
            MappingEntry {
                target: field_name.clone(),
                source: Expr::Field(field_name),
            }
        })
        .collect();
    notes.push(format!(
        "{}: mappings need manual review -- expression extraction not yet implemented",
        t.cobol_name
    ));

    TransformItem::TransformBlock(TransformBlockDef {
        name: Ident::new(&t.nexflow_name),
        comment: Some(format!("COBOL paragraph: {}", t.cobol_name)),
        input,
        output,
        body: TransformBlockBody::Mappings(mappings),
    })
}

fn build_compose_transform(
    t: &ClassifiedTransform,
    _notes: &mut Vec<String>,
) -> TransformItem {
    let input = if !t.reads.is_empty() {
        build_io_spec(&t.reads)
    } else {
        IoSpec::Single(Ident::new("input"), FieldType::Integer(None))
    };

    let output = if !t.writes.is_empty() {
        build_io_spec(&t.writes)
    } else {
        IoSpec::Single(Ident::new("output"), FieldType::Integer(None))
    };

    let refs: Vec<Ident> = t
        .performs
        .iter()
        .map(|p| Ident::new(&cobol_to_snake(p)))
        .collect();

    let kind_label = if matches!(t.kind, TransformKind::SectionDispatcher) {
        "section"
    } else {
        "paragraph"
    };

    TransformItem::TransformBlock(TransformBlockDef {
        name: Ident::new(&t.nexflow_name),
        comment: Some(format!("COBOL {}: {}", kind_label, t.cobol_name)),
        input,
        output,
        body: TransformBlockBody::Compose(ComposeBlock {
            compose_type: ComposeType::Sequential,
            refs,
        }),
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert a COBOL name (WS-ACCT-NUMBER) to snake_case.
fn cobol_to_snake(name: &str) -> String {
    name.to_lowercase().replace('-', "_")
}

/// Make a DSL-valid identifier from a string.
pub fn sanitize_identifier(name: &str) -> String {
    let s = name
        .to_lowercase()
        .replace(['-', ' '], "_");
    // Ensure starts with letter or underscore
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{s}")
    } else {
        s
    }
}

/// Heuristic type hint from a COBOL field name.
/// Without access to the actual field map at generation time,
/// we use naming conventions to guess the type.
fn cobol_name_to_type_hint(cobol_name: &str) -> FieldType {
    let lower = cobol_name.to_lowercase();
    if lower.contains("flag") || lower.contains("switch") || lower.contains("ind") {
        FieldType::Boolean
    } else if lower.contains("name") || lower.contains("desc") || lower.contains("text")
        || lower.contains("addr") || lower.contains("msg")
    {
        FieldType::String(None)
    } else if lower.contains("date") || lower.ends_with("-dt") || lower.ends_with("-dte") {
        FieldType::Date
    } else if lower.contains("timestamp") || lower.ends_with("-ts") {
        FieldType::Timestamp
    } else if lower.contains("amt") || lower.contains("amount") || lower.contains("balance")
        || lower.contains("rate") || lower.contains("price") || lower.contains("total")
    {
        FieldType::Decimal(18, 2)
    } else if lower.contains("count") || lower.contains("num") || lower.contains("qty")
        || lower.contains("id") || lower.contains("code") || lower.contains("idx")
    {
        FieldType::Integer(None)
    } else {
        // Safe default for COBOL numeric fields
        FieldType::Decimal(18, 2)
    }
}

/// Generate a placeholder expression referencing input fields.
fn generate_placeholder_expr(reads: &[String]) -> String {
    if reads.is_empty() {
        return "0".to_string();
    }
    if reads.len() == 1 {
        return sanitize_identifier(&cobol_to_snake(&reads[0]));
    }
    // Join first two reads with + as a placeholder
    format!(
        "{} + {}",
        sanitize_identifier(&cobol_to_snake(&reads[0])),
        sanitize_identifier(&cobol_to_snake(&reads[1]))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_cobol_names() {
        assert_eq!(sanitize_identifier("WS-ACCT-NUMBER"), "ws_acct_number");
        assert_eq!(sanitize_identifier("PROCESSING-SECTION"), "processing_section");
        assert_eq!(sanitize_identifier("123bad"), "_123bad");
    }

    #[test]
    fn cobol_to_snake_conversion() {
        assert_eq!(cobol_to_snake("WS-ACCT-NUMBER"), "ws-acct-number".to_lowercase().replace('-', "_"));
        assert_eq!(cobol_to_snake("WS-COUNT"), "ws_count");
    }

    #[test]
    fn type_hint_heuristics() {
        assert_eq!(cobol_name_to_type_hint("WS-ERR-FLAG"), FieldType::Boolean);
        assert_eq!(cobol_name_to_type_hint("WS-ACCT-NAME"), FieldType::String(None));
        assert_eq!(cobol_name_to_type_hint("WS-OPEN-DATE"), FieldType::Date);
        assert_eq!(cobol_name_to_type_hint("WS-BALANCE"), FieldType::Decimal(18, 2));
        assert_eq!(cobol_name_to_type_hint("WS-COUNT"), FieldType::Integer(None));
        assert_eq!(cobol_name_to_type_hint("WS-RESULT"), FieldType::Decimal(18, 2));
    }

    #[test]
    fn rust_name_roundtrip() {
        assert_eq!(rust_name_to_cobol("ws_acct_number"), "WS-ACCT-NUMBER");
    }

    #[test]
    fn classify_skip_boilerplate() {
        let f = AnnotatedFn {
            name: "run".to_string(),
            cobol_attr: Some(super::super::cobol_attrs::FnCobolAttr::default()),
        };
        assert!(classify_function(&f, &HashMap::new()).is_none());
    }

    #[test]
    fn classify_single_write_paragraph() {
        let f = AnnotatedFn {
            name: "proc_init".to_string(),
            cobol_attr: Some(super::super::cobol_attrs::FnCobolAttr {
                section: Some("PROCESSING-SECTION".to_string()),
                performs: vec![],
                reads: vec![],
                writes: vec!["WS-COUNT".to_string()],
            }),
        };
        let classified = classify_function(&f, &HashMap::new()).unwrap();
        assert!(matches!(classified.kind, TransformKind::SingleField));
        assert_eq!(classified.nexflow_name, "proc_init");
    }

    #[test]
    fn classify_multi_write_paragraph() {
        let f = AnnotatedFn {
            name: "calc_para".to_string(),
            cobol_attr: Some(super::super::cobol_attrs::FnCobolAttr {
                section: Some("CALCULATION-SECTION".to_string()),
                performs: vec![],
                reads: vec!["WS-I".to_string(), "WS-SUM".to_string()],
                writes: vec!["WS-I".to_string(), "WS-SUM".to_string()],
            }),
        };
        let classified = classify_function(&f, &HashMap::new()).unwrap();
        assert!(matches!(classified.kind, TransformKind::MultiField));
    }

    #[test]
    fn classify_section_dispatcher() {
        let f = AnnotatedFn {
            name: "processing_section".to_string(),
            cobol_attr: Some(super::super::cobol_attrs::FnCobolAttr {
                section: Some("PROCESSING-SECTION".to_string()),
                performs: vec![],
                reads: vec![],
                writes: vec![],
            }),
        };
        // No reads, no writes, no performs -- should return None (empty dispatcher)
        assert!(classify_function(&f, &HashMap::new()).is_none());
    }

    #[test]
    fn classify_orchestrator() {
        let f = AnnotatedFn {
            name: "main_para".to_string(),
            cobol_attr: Some(super::super::cobol_attrs::FnCobolAttr {
                section: Some("MAIN-SECTION".to_string()),
                performs: vec![
                    "PROCESSING-SECTION".to_string(),
                    "CALC-PARA".to_string(),
                ],
                reads: vec!["WS-COUNT".to_string(), "WS-SUM".to_string()],
                writes: vec![],
            }),
        };
        let classified = classify_function(&f, &HashMap::new()).unwrap();
        assert!(matches!(classified.kind, TransformKind::Compose));
    }

    #[test]
    fn end_to_end_transform_emission() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)", offset = 0, len = 5)]
                pub ws_count: PackedDecimal,
                #[cobol(level = 1, pic = "9(7)", offset = 0, len = 7)]
                pub ws_sum: PackedDecimal,
            }

            #[cobol(section = "CALC-SECTION", reads = "WS-COUNT", writes = "WS-SUM")]
            fn add_to_sum(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "CALC-SECTION", performs = "CALC-SECTION", reads = "WS-SUM")]
            fn main_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = TransformEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert!(!files.is_empty(), "Should produce at least one .xform file");

        let content = &files[0].content;
        assert!(content.contains("transform"), "Should contain transform keyword");
        assert!(content.contains("end"), "Should be properly terminated");
        assert!(content.contains("import"), "Should import schemas");

        // Verify file path
        assert!(files[0].path.ends_with(".xform"), "Should have .xform extension");
    }

    #[test]
    fn compose_paragraph_generates_compose_block() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
            }

            #[cobol(section = "MAIN-SECTION", performs = "PROC-INIT,PROC-WORK", reads = "WS-COUNT")]
            fn main_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "MAIN-SECTION", writes = "WS-COUNT")]
            fn proc_init(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "MAIN-SECTION", reads = "WS-COUNT", writes = "WS-COUNT")]
            fn proc_work(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = TransformEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert!(!files.is_empty());

        let content = &files[0].content;
        // main_para should produce compose block
        assert!(
            content.contains("compose sequential"),
            "Orchestrator should use compose sequential"
        );
        assert!(
            content.contains("proc_init"),
            "Compose should reference child transforms"
        );
    }

    #[test]
    fn multi_section_produces_multiple_files() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
                #[cobol(level = 1, pic = "9(7)")]
                pub ws_sum: PackedDecimal,
            }

            #[cobol(section = "SECTION-A", writes = "WS-COUNT")]
            fn para_a(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}

            #[cobol(section = "SECTION-B", reads = "WS-COUNT", writes = "WS-SUM")]
            fn para_b(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = TransformEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert_eq!(files.len(), 2, "Two sections should produce two .xform files");

        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.contains(&"transform/section_a.xform"));
        assert!(paths.contains(&"transform/section_b.xform"));
    }

    #[test]
    fn single_field_transform_shape() {
        let code = r#"
            #[cobol(program = "TESTPROG")]
            pub struct WorkingStorage {
                #[cobol(level = 1, pic = "9(5)")]
                pub ws_count: PackedDecimal,
            }

            #[cobol(section = "INIT-SECTION", writes = "WS-COUNT")]
            fn init_count(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {}
        "#;
        let syn_file = syn::parse_str::<syn::File>(code).unwrap();

        let emitter = TransformEmitter;
        let ctx = EmitterContext {
            program_name: "testprog".to_string(),
            syn_file: &syn_file,
            source_text: code,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };

        let files = emitter.emit(&ctx);
        assert_eq!(files.len(), 1);
        let content = &files[0].content;

        // Single-write paragraph -> `transform` (not transform_block)
        assert!(content.contains("transform init_count"));
        assert!(content.contains("apply"));
        assert!(content.contains("pure : true"));
        // Should NOT contain transform_block
        assert!(!content.contains("transform_block init_count"));
    }
}
