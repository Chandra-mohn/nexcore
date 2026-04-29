//! Direct transform emitter: generates `.xform` files from COBOL AST.
//!
//! Reads the COBOL AST's paragraphs and their statements to classify
//! transforms and extract real expressions from COMPUTE/MOVE/arithmetic
//! statements. Unlike the legacy emitter, this produces actual mappings
//! instead of placeholders.

use std::collections::{HashMap, HashSet};

use cobol_transpiler::ast::{CobolProgram, Paragraph, ProcedureDivision};

use super::cobol_extract::{analyze_statement, extract_on_size_error, has_side_effects, usage_to_str, walk_elementary_entries};
use super::condition_extract::{evaluate_to_conditional_compose, extract_mappings, ExtractedMapping};
use super::{DirectDslEmitter, DirectEmitterContext};
use crate::dsl::dsl_ast::*;
use crate::dsl::transform_emitter::{
    group_by_section, sanitize_identifier, ClassifiedTransform, TransformKind,
};
use crate::dsl::type_mapping::pic_to_nexflow_type;
use crate::dsl::{DslFile, DslLayer};

/// Direct transform emitter: reads COBOL AST ProcedureDivision.
#[derive(Debug)]
pub struct DirectTransformEmitter;

impl DirectDslEmitter for DirectTransformEmitter {
    fn id(&self) -> &'static str {
        "transform"
    }

    fn layer(&self) -> DslLayer {
        DslLayer::Transform
    }

    fn emit(&self, ctx: &DirectEmitterContext<'_>) -> Vec<DslFile> {
        let proc_div = match &ctx.cobol_program.procedure_division {
            Some(pd) => pd,
            None => return vec![],
        };

        let field_type_map = build_field_type_map(ctx.cobol_program);

        let enriched = classify_and_extract(proc_div);
        if enriched.is_empty() {
            return vec![];
        }

        // Group by section using the ClassifiedTransform inside each enriched entry
        let transforms: Vec<ClassifiedTransform> = enriched.iter().map(|e| e.classified.clone()).collect();
        let sections = group_by_section(&transforms);

        // Build lookups from nexflow_name -> extracted mappings and purity
        let mapping_lookup: HashMap<&str, &[ExtractedMapping]> = enriched
            .iter()
            .map(|e| (e.classified.nexflow_name.as_str(), e.mappings.as_slice()))
            .collect();
        let purity_lookup: HashMap<&str, bool> = enriched
            .iter()
            .map(|e| (e.classified.nexflow_name.as_str(), e.pure))
            .collect();
        let error_lookup: HashMap<&str, &Option<Vec<ErrorStatement>>> = enriched
            .iter()
            .map(|e| (e.classified.nexflow_name.as_str(), &e.on_error))
            .collect();
        let compose_lookup: HashMap<&str, &Option<ComposeBlock>> = enriched
            .iter()
            .map(|e| (e.classified.nexflow_name.as_str(), &e.conditional_compose))
            .collect();

        let mut dsl_files = Vec::new();
        for (section_name, section_transforms) in &sections {
            let (content, notes, confidence) =
                generate_transform_file_direct(section_name, section_transforms, &ctx.program_name, &mapping_lookup, &purity_lookup, &field_type_map, &error_lookup, &compose_lookup);
            let source_fns: Vec<String> = section_transforms
                .iter()
                .map(|t| t.cobol_name.clone())
                .collect();

            dsl_files.push(DslFile {
                path: format!("transform/{}.xform", sanitize_identifier(section_name)),
                content,
                confidence,
                notes,
                source_fields: source_fns,
            });
        }

        // Detect string operation patterns and emit review-noted transforms
        let mut patterns = std::collections::HashSet::new();
        for section in &proc_div.sections {
            for para in &section.paragraphs {
                for sentence in &para.sentences {
                    for stmt in &sentence.statements {
                        super::cobol_extract::collect_patterns(stmt, &mut patterns);
                    }
                }
            }
        }

        let has_string_ops = patterns.iter().any(|p| matches!(
            p,
            super::cobol_extract::CobolPattern::StringInspect
                | super::cobol_extract::CobolPattern::StringConcat
                | super::cobol_extract::CobolPattern::StringSplit
        ));

        if has_string_ops {
            let mut string_notes = Vec::new();
            for pattern in &patterns {
                use super::cobol_extract::CobolPattern;
                match pattern {
                    CobolPattern::StringInspect => {
                        string_notes.push("INSPECT: regex/replace mapping needed".to_string());
                    }
                    CobolPattern::StringConcat => {
                        string_notes.push("STRING: concatenation -> format!/join mapping needed".to_string());
                    }
                    CobolPattern::StringSplit => {
                        string_notes.push("UNSTRING: split/parse mapping needed".to_string());
                    }
                    _ => {}
                }
            }

            let xform_content = format!(
                "// String operation transforms (manual mapping required)\n\
                 // Detected patterns:\n{}\n\
                 transform {}_string_ops {{\n\
                 \n  // TODO: map COBOL string operations to Rust equivalents\n\
                 \n  apply {{\n    // placeholder\n  }}\n}}\n",
                string_notes.iter().map(|n| format!("//   - {n}\n")).collect::<String>(),
                sanitize_identifier(&ctx.program_name.to_lowercase()),
            );

            dsl_files.push(DslFile {
                path: format!(
                    "transform/{}_string_ops.xform",
                    sanitize_identifier(&ctx.program_name.to_lowercase())
                ),
                content: xform_content,
                confidence: 0.3,
                notes: string_notes,
                source_fields: vec![ctx.program_name.clone()],
            });
        }

        dsl_files
    }
}

/// A ClassifiedTransform enriched with extracted expression mappings.
struct EnrichedTransform {
    classified: ClassifiedTransform,
    mappings: Vec<ExtractedMapping>,
    /// Whether the paragraph contains only pure computation (no I/O or external calls).
    pure: bool,
    /// Error handling extracted from ON SIZE ERROR in arithmetic statements.
    on_error: Option<Vec<ErrorStatement>>,
    /// Conditional compose block from EVALUATE...PERFORM pattern.
    conditional_compose: Option<ComposeBlock>,
}

/// Classify all paragraphs and extract expressions from their statements.
fn classify_and_extract(proc_div: &ProcedureDivision) -> Vec<EnrichedTransform> {
    let mut enriched = Vec::new();

    for section in &proc_div.sections {
        for para in &section.paragraphs {
            if let Some(et) = classify_and_extract_paragraph(para, Some(&section.name)) {
                enriched.push(et);
            }
        }
    }

    for para in &proc_div.paragraphs {
        if let Some(et) = classify_and_extract_paragraph(para, None) {
            enriched.push(et);
        }
    }

    enriched
}

fn classify_and_extract_paragraph(
    para: &Paragraph,
    section: Option<&str>,
) -> Option<EnrichedTransform> {
    let nexflow_name = sanitize_identifier(&para.name.to_lowercase());

    if is_boilerplate(&nexflow_name) {
        return None;
    }

    let mut performs = Vec::new();
    let mut reads = HashSet::new();
    let mut writes = HashSet::new();

    for sentence in &para.sentences {
        for stmt in &sentence.statements {
            analyze_statement(stmt, &mut performs, &mut reads, &mut writes);
        }
    }

    let has_performs = !performs.is_empty();
    let has_reads = !reads.is_empty();
    let has_writes = !writes.is_empty();

    let kind = if !has_reads && !has_writes && has_performs {
        if is_section_name(&nexflow_name) {
            TransformKind::SectionDispatcher
        } else {
            TransformKind::Compose
        }
    } else if has_writes && writes.len() == 1 && !has_performs {
        TransformKind::SingleField
    } else if has_writes || has_reads {
        if has_performs {
            TransformKind::Compose
        } else {
            TransformKind::MultiField
        }
    } else {
        return None;
    };

    // Extract expression mappings from all statements in the paragraph
    let all_stmts: Vec<&cobol_transpiler::ast::Statement> = para
        .sentences
        .iter()
        .flat_map(|s| &s.statements)
        .collect();
    let stmts_owned: Vec<cobol_transpiler::ast::Statement> = all_stmts.into_iter().cloned().collect();
    let mappings = extract_mappings(&stmts_owned);
    let pure = !has_side_effects(&stmts_owned);
    let on_error = extract_on_size_error(&stmts_owned);

    // Detect EVALUATE...PERFORM pattern for conditional compose
    let conditional_compose = stmts_owned.iter().find_map(|stmt| {
        if let cobol_transpiler::ast::Statement::Evaluate(eval) = stmt {
            evaluate_to_conditional_compose(eval)
        } else {
            None
        }
    });

    Some(EnrichedTransform {
        classified: ClassifiedTransform {
            cobol_name: para.name.clone(),
            nexflow_name,
            section: section.map(String::from),
            kind,
            reads: sorted_vec(reads),
            writes: sorted_vec(writes),
            performs,
        },
        mappings,
        pure,
        on_error,
        conditional_compose,
    })
}

// ---------------------------------------------------------------------------
// Direct-specific generation (replaces placeholders with real expressions)
// ---------------------------------------------------------------------------

fn generate_transform_file_direct(
    section_name: &str,
    transforms: &[&ClassifiedTransform],
    program: &str,
    mapping_lookup: &HashMap<&str, &[ExtractedMapping]>,
    purity_lookup: &HashMap<&str, bool>,
    field_map: &HashMap<String, FieldType>,
    error_lookup: &HashMap<&str, &Option<Vec<ErrorStatement>>>,
    compose_lookup: &HashMap<&str, &Option<ComposeBlock>>,
) -> (String, Vec<String>, f64) {
    let mut items = Vec::new();
    let mut notes = Vec::new();

    for t in transforms {
        let extracted = mapping_lookup.get(t.nexflow_name.as_str()).copied().unwrap_or(&[]);
        let pure = purity_lookup.get(t.nexflow_name.as_str()).copied().unwrap_or(true);
        let on_error = error_lookup
            .get(t.nexflow_name.as_str())
            .and_then(|opt| opt.as_ref())
            .cloned();
        let cond_compose = compose_lookup
            .get(t.nexflow_name.as_str())
            .and_then(|opt| opt.as_ref())
            .cloned();
        match t.kind {
            TransformKind::SingleField => {
                items.push(build_single_transform(t, extracted, pure, field_map, on_error, &mut notes));
            }
            TransformKind::MultiField => {
                items.push(build_multi_transform(t, extracted, field_map, on_error, &mut notes));
            }
            TransformKind::Compose | TransformKind::SectionDispatcher => {
                items.push(build_compose_transform(t, field_map, cond_compose));
            }
        }
    }

    let file = TransformFile {
        comments: vec![
            Comment("Generated by cobol2rust Nexflow emitter".to_string()),
            Comment(format!("Source: {program} section {section_name}")),
        ],
        imports: vec![ImportPath::schema(program)],
        items,
    };

    (file.to_text(), notes, 1.0)
}

fn build_single_transform(
    t: &ClassifiedTransform,
    extracted: &[ExtractedMapping],
    pure: bool,
    field_map: &HashMap<String, FieldType>,
    on_error: Option<Vec<ErrorStatement>>,
    notes: &mut Vec<String>,
) -> TransformItem {
    let input = build_io_spec(&t.reads, field_map);
    let output = if t.writes.len() == 1 {
        IoSpec::Single(
            Ident::new(&cobol_to_snake(&t.writes[0])),
            resolve_field_type(&t.writes[0], field_map),
        )
    } else {
        IoSpec::Single(Ident::new("result"), FieldType::Integer(None))
    };

    // Use extracted expression if available, otherwise note it
    let apply = if let Some(mapping) = extracted.iter().find(|m| {
        t.writes.iter().any(|w| cobol_to_snake(w) == m.target)
    }) {
        vec![ApplyStmt::Assign(
            Ident::new(&mapping.target),
            Expr::Raw(mapping.expr.clone()),
        )]
    } else {
        notes.push(format!(
            "{}: no expression extracted -- manual review needed",
            t.cobol_name
        ));
        vec![ApplyStmt::Assign(
            Ident::new("result"),
            Expr::Lit(Literal::Int(0)),
        )]
    };

    TransformItem::Transform(TransformDef {
        name: Ident::new(&t.nexflow_name),
        comment: Some(format!("COBOL paragraph: {}", t.cobol_name)),
        metadata: None,
        pure,
        cache: None,
        input,
        output,
        apply,
        validate_input: generate_input_validations(&t.reads, field_map),
        validate_output: None,
        on_error,
    })
}

fn build_multi_transform(
    t: &ClassifiedTransform,
    extracted: &[ExtractedMapping],
    field_map: &HashMap<String, FieldType>,
    on_error: Option<Vec<ErrorStatement>>,
    notes: &mut Vec<String>,
) -> TransformItem {
    let (input, output) = if t.reads.is_empty() && t.writes.is_empty() {
        (
            IoSpec::Single(Ident::new("input"), FieldType::Integer(None)),
            IoSpec::Single(Ident::new("output"), FieldType::Integer(None)),
        )
    } else {
        (build_io_spec(&t.reads, field_map), build_io_spec(&t.writes, field_map))
    };

    let mappings: Vec<MappingEntry> = if extracted.is_empty() {
        // Fallback: identity mappings with a note
        notes.push(format!(
            "{}: no expressions extracted -- manual review needed",
            t.cobol_name
        ));
        t.writes
            .iter()
            .map(|w| {
                let field_name = Ident::new(&cobol_to_snake(w));
                MappingEntry {
                    target: field_name.clone(),
                    source: Expr::Field(field_name),
                }
            })
            .collect()
    } else {
        // Use real extracted expressions
        let mut result = Vec::new();
        let mut covered_targets: HashSet<String> = HashSet::new();

        for mapping in extracted {
            // Only emit mappings for fields in the writes set
            if t.writes.iter().any(|w| cobol_to_snake(w) == mapping.target) {
                covered_targets.insert(mapping.target.clone());
                result.push(MappingEntry {
                    target: Ident::new(&mapping.target),
                    source: Expr::Raw(mapping.expr.clone()),
                });
            }
        }

        // Any write fields not covered by extraction get a note
        let uncovered: Vec<&str> = t.writes.iter()
            .filter(|w| !covered_targets.contains(&cobol_to_snake(w)))
            .map(|w| w.as_str())
            .collect();
        if !uncovered.is_empty() {
            notes.push(format!(
                "{}: {} write field(s) not extracted: {}",
                t.cobol_name, uncovered.len(), uncovered.join(", ")
            ));
        }

        result
    };

    TransformItem::TransformBlock(TransformBlockDef {
        name: Ident::new(&t.nexflow_name),
        comment: Some(format!("COBOL paragraph: {}", t.cobol_name)),
        metadata: None,
        use_decls: None,
        input,
        output,
        body: TransformBlockBody::Mappings(mappings),
        validate_input: generate_input_validations(&t.reads, field_map),
        validate_output: None,
        on_error,
    })
}

fn build_compose_transform(t: &ClassifiedTransform, field_map: &HashMap<String, FieldType>, cond_compose: Option<ComposeBlock>) -> TransformItem {
    let input = if !t.reads.is_empty() {
        build_io_spec(&t.reads, field_map)
    } else {
        IoSpec::Single(Ident::new("input"), FieldType::Integer(None))
    };

    let output = if !t.writes.is_empty() {
        build_io_spec(&t.writes, field_map)
    } else {
        IoSpec::Single(Ident::new("output"), FieldType::Integer(None))
    };

    let refs: Vec<ComposeRef> = t
        .performs
        .iter()
        .map(|p| ComposeRef::Simple(Ident::new(&cobol_to_snake(p))))
        .collect();

    let kind_label = if matches!(t.kind, TransformKind::SectionDispatcher) {
        "section"
    } else {
        "paragraph"
    };

    // Use conditional compose if EVALUATE...PERFORM pattern detected, otherwise sequential
    let compose = cond_compose.unwrap_or(ComposeBlock {
        compose_type: ComposeType::Sequential,
        refs,
    });

    TransformItem::TransformBlock(TransformBlockDef {
        name: Ident::new(&t.nexflow_name),
        comment: Some(format!("COBOL {}: {}", kind_label, t.cobol_name)),
        metadata: None,
        use_decls: None,
        input,
        output,
        body: TransformBlockBody::Compose(compose),
        validate_input: None,
        validate_output: None,
        on_error: None,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a map from uppercase COBOL field name to resolved FieldType.
/// Walks WORKING-STORAGE, LOCAL-STORAGE, and LINKAGE sections.
fn build_field_type_map(program: &CobolProgram) -> HashMap<String, FieldType> {
    let mut map = HashMap::new();

    let data_div = match &program.data_division {
        Some(dd) => dd,
        None => return map,
    };

    let all_sections = [
        data_div.working_storage.as_slice(),
        data_div.local_storage.as_slice(),
        data_div.linkage.as_slice(),
    ];

    for section in all_sections {
        let entries = walk_elementary_entries(section);
        for entry in entries {
            if let Some(pic) = &entry.pic {
                let snake_name = cobol_to_snake(&entry.name);
                let nexflow_type = pic_to_nexflow_type(
                    &pic.raw,
                    usage_to_str(&entry.usage),
                    pic.signed,
                    &snake_name,
                );
                map.insert(entry.name.to_uppercase(), nexflow_type.to_field_type());
            }
        }
    }

    map
}

/// Look up field type from PIC map, falling back to name heuristic.
fn resolve_field_type(cobol_name: &str, field_map: &HashMap<String, FieldType>) -> FieldType {
    field_map
        .get(&cobol_name.to_uppercase())
        .cloned()
        .unwrap_or_else(|| cobol_name_to_type_hint(cobol_name))
}

/// Generate validate_input rules from PIC-based field types.
///
/// - Unsigned numeric fields (Integer, Decimal without sign) get a >= 0 constraint
/// - String/Char fields get a length constraint from PIC
fn generate_input_validations(
    reads: &[String],
    field_map: &HashMap<String, FieldType>,
) -> Option<Vec<ValidationRule>> {
    let mut rules = Vec::new();

    for field_name in reads {
        let ft = match field_map.get(&field_name.to_uppercase()) {
            Some(ft) => ft,
            None => continue,
        };
        let snake = cobol_to_snake(field_name);

        match ft {
            // Unsigned integer: require >= 0
            FieldType::Integer(_) => {
                rules.push(ValidationRule::Require(
                    Expr::Binary(
                        Box::new(Expr::Field(Ident::new(&snake))),
                        BinOp::Ge,
                        Box::new(Expr::Lit(Literal::Int(0))),
                    ),
                    ValidationMessage {
                        text: format!("{snake} must be non-negative"),
                        code: None,
                        severity: None,
                    },
                ));
            }
            // Unsigned decimal: require >= 0
            FieldType::Decimal(_, _) => {
                rules.push(ValidationRule::Require(
                    Expr::Binary(
                        Box::new(Expr::Field(Ident::new(&snake))),
                        BinOp::Ge,
                        Box::new(Expr::Lit(Literal::Int(0))),
                    ),
                    ValidationMessage {
                        text: format!("{snake} must be non-negative"),
                        code: None,
                        severity: None,
                    },
                ));
            }
            // String with known length: require length constraint
            FieldType::String(Some(n)) => {
                rules.push(ValidationRule::Require(
                    Expr::Binary(
                        Box::new(Expr::Call(
                            Ident::new("length"),
                            vec![Expr::Field(Ident::new(&snake))],
                        )),
                        BinOp::Le,
                        Box::new(Expr::Lit(Literal::Int(*n as i64))),
                    ),
                    ValidationMessage {
                        text: format!("{snake} exceeds max length {n}"),
                        code: None,
                        severity: Some(Severity::Warning),
                    },
                ));
            }
            // Char with known length
            FieldType::Char(n) => {
                rules.push(ValidationRule::Require(
                    Expr::Binary(
                        Box::new(Expr::Call(
                            Ident::new("length"),
                            vec![Expr::Field(Ident::new(&snake))],
                        )),
                        BinOp::Le,
                        Box::new(Expr::Lit(Literal::Int(*n as i64))),
                    ),
                    ValidationMessage {
                        text: format!("{snake} exceeds max length {n}"),
                        code: None,
                        severity: Some(Severity::Warning),
                    },
                ));
            }
            _ => {}
        }
    }

    if rules.is_empty() { None } else { Some(rules) }
}

fn build_io_spec(fields: &[String], field_map: &HashMap<String, FieldType>) -> IoSpec {
    if fields.is_empty() {
        IoSpec::Single(Ident::new("value"), FieldType::Integer(None))
    } else if fields.len() == 1 {
        IoSpec::Single(
            Ident::new(&cobol_to_snake(&fields[0])),
            resolve_field_type(&fields[0], field_map),
        )
    } else {
        IoSpec::Multi(
            fields
                .iter()
                .map(|f| IoField {
                    name: Ident::new(&cobol_to_snake(f)),
                    field_type: resolve_field_type(f, field_map),
                })
                .collect(),
        )
    }
}

fn cobol_to_snake(name: &str) -> String {
    name.to_lowercase().replace('-', "_")
}

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
        FieldType::Decimal(18, 2)
    }
}

fn is_section_name(name: &str) -> bool {
    name.ends_with("_section")
}

fn is_boilerplate(name: &str) -> bool {
    matches!(name, "main" | "new" | "stop_run" | "goback")
}

fn sorted_vec(set: HashSet<String>) -> Vec<String> {
    let mut v: Vec<String> = set.into_iter().collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::*;

    fn make_compute(target: &str, expr: ArithExpr) -> Statement {
        Statement::Compute(ComputeStatement {
            targets: vec![ArithTarget {
                field: DataReference {
                    name: target.to_string(),
                    qualifiers: vec![],
                    subscripts: vec![],
                    ref_mod: None,
                },
                rounded: false,
            }],
            expression: expr,
            on_size_error: vec![],
            not_on_size_error: vec![],
        })
    }

    fn make_field_ref(name: &str) -> ArithExpr {
        ArithExpr::Operand(Operand::DataRef(DataReference {
            name: name.to_string(),
            qualifiers: vec![],
            subscripts: vec![],
            ref_mod: None,
        }))
    }

    fn make_perform(target: &str) -> Statement {
        Statement::Perform(PerformStatement {
            target: Some(PerformTarget {
                name: target.to_string(),
            }),
            thru: None,
            loop_type: PerformLoopType::Once,
            body: vec![],
        })
    }

    fn make_move(source: &str, dest: &str) -> Statement {
        Statement::Move(MoveStatement {
            corresponding: false,
            source: Operand::DataRef(DataReference {
                name: source.to_string(),
                qualifiers: vec![],
                subscripts: vec![],
                ref_mod: None,
            }),
            destinations: vec![DataReference {
                name: dest.to_string(),
                qualifiers: vec![],
                subscripts: vec![],
                ref_mod: None,
            }],
        })
    }

    fn make_paragraph(name: &str, stmts: Vec<Statement>) -> Paragraph {
        Paragraph {
            name: name.to_string(),
            sentences: vec![Sentence { statements: stmts }],
        }
    }

    fn make_program_with_proc(proc_div: ProcedureDivision) -> CobolProgram {
        CobolProgram {
            program_id: "TESTPROG".to_string(),
            data_division: None,
            procedure_division: Some(proc_div),
            source_path: None,
            exec_sql_statements: vec![],
        }
    }

    #[test]
    fn direct_transform_extracts_compute_expression() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "CALC-SECTION".to_string(),
                paragraphs: vec![make_paragraph("CALC-PARA", vec![
                    make_compute("WS-RESULT", ArithExpr::BinaryOp {
                        left: Box::new(make_field_ref("WS-V01")),
                        op: ArithOp::Add,
                        right: Box::new(make_field_ref("WS-V02")),
                    }),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());

        let content = &files[0].content;
        assert!(
            content.contains("ws_v01 + ws_v02"),
            "Should contain real expression, got: {content}"
        );
        assert!(
            !content.contains("ws_result = ws_result"),
            "Should NOT contain identity mapping, got: {content}"
        );
    }

    #[test]
    fn direct_transform_extracts_move() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "INIT-SECTION".to_string(),
                paragraphs: vec![make_paragraph("INIT-PARA", vec![
                    make_move("WS-INPUT", "WS-OUTPUT"),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());

        let content = &files[0].content;
        assert!(
            content.contains("ws_input"),
            "Should reference source field: {content}"
        );
    }

    #[test]
    fn direct_transform_compose_still_works() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "MAIN-SECTION".to_string(),
                paragraphs: vec![
                    make_paragraph("ORCHESTRATOR", vec![
                        make_perform("STEP-A"),
                        make_perform("STEP-B"),
                    ]),
                    make_paragraph("STEP-A", vec![
                        make_move("WS-IN", "WS-OUT"),
                    ]),
                    make_paragraph("STEP-B", vec![
                        make_move("WS-X", "WS-Y"),
                    ]),
                ],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());

        let content = &files[0].content;
        assert!(
            content.contains("compose sequential"),
            "Orchestrator should still use compose: {content}"
        );
    }

    #[test]
    fn direct_transform_empty_proc_div() {
        let program = CobolProgram {
            program_id: "EMPTY".to_string(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: vec![],
        };
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "EMPTY".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(files.is_empty());
    }

    #[test]
    fn direct_transform_multi_section() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![
                Section {
                    name: "SECTION-A".to_string(),
                    paragraphs: vec![make_paragraph("PARA-A", vec![
                        make_move("WS-IN", "WS-OUT"),
                    ])],
                },
                Section {
                    name: "SECTION-B".to_string(),
                    paragraphs: vec![make_paragraph("PARA-B", vec![
                        make_move("WS-X", "WS-Y"),
                    ])],
                },
            ],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert_eq!(files.len(), 2, "Two sections -> two xform files");
    }

    #[test]
    fn pure_paragraph_emits_pure_true() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "CALC-SECTION".to_string(),
                paragraphs: vec![make_paragraph("CALC-PARA", vec![
                    make_compute("WS-RESULT", ArithExpr::BinaryOp {
                        left: Box::new(make_field_ref("WS-A")),
                        op: ArithOp::Add,
                        right: Box::new(make_field_ref("WS-B")),
                    }),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        assert!(
            content.contains("pure : true"),
            "Pure paragraph should emit pure : true, got: {content}"
        );
    }

    #[test]
    fn impure_paragraph_emits_pure_false() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "IO-SECTION".to_string(),
                paragraphs: vec![make_paragraph("IO-PARA", vec![
                    Statement::Display(DisplayStatement {
                        items: vec![],
                        upon: DisplayTarget::Sysout,
                        no_advancing: false,
                    }),
                    make_move("WS-INPUT", "WS-OUTPUT"),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        assert!(
            content.contains("pure : false"),
            "Impure paragraph should emit pure : false, got: {content}"
        );
    }

    // -----------------------------------------------------------------------
    // PIC-based type resolution tests
    // -----------------------------------------------------------------------

    fn make_data_entry(name: &str, pic_raw: &str, signed: bool) -> DataEntry {
        DataEntry {
            level: 5,
            name: name.to_string(),
            pic: Some(PicClause {
                raw: pic_raw.to_string(),
                category: if pic_raw.starts_with('X') || pic_raw.starts_with('A') {
                    PicCategory::Alphanumeric
                } else {
                    PicCategory::Numeric
                },
                total_digits: 0,
                scale: 0,
                signed,
                display_length: 0,
                edit_symbols: vec![],
            }),
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: vec![],
            condition_values: vec![],
            byte_offset: None,
            byte_length: None,
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }
    }

    #[test]
    fn build_field_type_map_basic() {
        let program = CobolProgram {
            program_id: "TEST".to_string(),
            data_division: Some(DataDivision {
                working_storage: vec![
                    make_data_entry("WS-NAME", "X(30)", false),
                    make_data_entry("WS-AMOUNT", "S9(9)V99", true),
                ],
                local_storage: vec![],
                linkage: vec![],
                file_section: vec![],
            }),
            procedure_division: None,
            source_path: None,
            exec_sql_statements: vec![],
        };

        let map = build_field_type_map(&program);
        assert_eq!(
            map.get("WS-NAME"),
            Some(&FieldType::String(Some(30))),
            "PIC X(30) should map to String(30)"
        );
        // S9(9)V99 -> decimal(11, 2) via pic_to_nexflow_type
        assert!(
            matches!(map.get("WS-AMOUNT"), Some(FieldType::Decimal(_, _))),
            "PIC S9(9)V99 should map to Decimal, got: {:?}",
            map.get("WS-AMOUNT")
        );
    }

    #[test]
    fn pic_type_overrides_heuristic() {
        // "WS-RESULT" would be Decimal(18,2) by name heuristic, but
        // PIC X(10) should resolve to String(Some(10))
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "CALC-SECTION".to_string(),
                paragraphs: vec![make_paragraph("CALC-PARA", vec![
                    make_move("WS-INPUT", "WS-RESULT"),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = CobolProgram {
            program_id: "TESTPROG".to_string(),
            data_division: Some(DataDivision {
                working_storage: vec![
                    make_data_entry("WS-RESULT", "X(10)", false),
                    make_data_entry("WS-INPUT", "9(5)", false),
                ],
                local_storage: vec![],
                linkage: vec![],
                file_section: vec![],
            }),
            procedure_division: Some(proc_div),
            source_path: None,
            exec_sql_statements: vec![],
        };

        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        assert!(
            content.contains("string(10)"),
            "Should use PIC-resolved type string(10), got: {content}"
        );
    }

    #[test]
    fn pic_fallback_to_heuristic_when_no_data_division() {
        // When there's no DataDivision, should fall back to name heuristics
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "INIT-SECTION".to_string(),
                paragraphs: vec![make_paragraph("INIT-PARA", vec![
                    make_move("WS-INPUT", "WS-COUNT"),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = CobolProgram {
            program_id: "TESTPROG".to_string(),
            data_division: None,
            procedure_division: Some(proc_div),
            source_path: None,
            exec_sql_statements: vec![],
        };

        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        // "WS-COUNT" should heuristically map to integer
        assert!(
            content.contains("integer"),
            "Should fall back to heuristic type, got: {content}"
        );
    }

    // -----------------------------------------------------------------------
    // on_error extraction tests
    // -----------------------------------------------------------------------

    #[test]
    fn on_size_error_emits_on_error_block() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "CALC-SECTION".to_string(),
                paragraphs: vec![make_paragraph("CALC-PARA", vec![
                    Statement::Compute(ComputeStatement {
                        targets: vec![ArithTarget {
                            field: DataReference {
                                name: "WS-RESULT".to_string(),
                                qualifiers: vec![],
                                subscripts: vec![],
                                ref_mod: None,
                            },
                            rounded: false,
                        }],
                        expression: ArithExpr::BinaryOp {
                            left: Box::new(make_field_ref("WS-A")),
                            op: ArithOp::Add,
                            right: Box::new(make_field_ref("WS-B")),
                        },
                        on_size_error: vec![Statement::Display(DisplayStatement {
                            items: vec![],
                            upon: DisplayTarget::Sysout,
                            no_advancing: false,
                        })],
                        not_on_size_error: vec![],
                    }),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        assert!(
            content.contains("on_error"),
            "Should contain on_error block, got: {content}"
        );
        assert!(
            content.contains("action : raise"),
            "Should contain action : raise, got: {content}"
        );
        assert!(
            content.contains("SIZE_ERROR"),
            "Should contain error code, got: {content}"
        );
    }

    #[test]
    fn no_size_error_no_on_error_block() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "CALC-SECTION".to_string(),
                paragraphs: vec![make_paragraph("CALC-PARA", vec![
                    make_compute("WS-RESULT", ArithExpr::BinaryOp {
                        left: Box::new(make_field_ref("WS-A")),
                        op: ArithOp::Add,
                        right: Box::new(make_field_ref("WS-B")),
                    }),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        assert!(
            !content.contains("on_error"),
            "Should NOT contain on_error block, got: {content}"
        );
    }

    // -----------------------------------------------------------------------
    // validate_input generation tests
    // -----------------------------------------------------------------------

    #[test]
    fn numeric_pic_generates_validate_input() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "CALC-SECTION".to_string(),
                paragraphs: vec![make_paragraph("CALC-PARA", vec![
                    make_move("WS-AMOUNT", "WS-RESULT"),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = CobolProgram {
            program_id: "TESTPROG".to_string(),
            data_division: Some(DataDivision {
                working_storage: vec![
                    make_data_entry("WS-AMOUNT", "9(7)V99", false),
                    make_data_entry("WS-RESULT", "9(9)V99", false),
                ],
                local_storage: vec![],
                linkage: vec![],
                file_section: vec![],
            }),
            procedure_division: Some(proc_div),
            source_path: None,
            exec_sql_statements: vec![],
        };

        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        assert!(
            content.contains("validate_input"),
            "Numeric PIC should generate validate_input, got: {content}"
        );
        assert!(
            content.contains("non-negative"),
            "Should have non-negative constraint, got: {content}"
        );
    }

    #[test]
    fn string_pic_generates_length_validation() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "INIT-SECTION".to_string(),
                paragraphs: vec![make_paragraph("INIT-PARA", vec![
                    make_move("WS-NAME", "WS-OUTPUT"),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = CobolProgram {
            program_id: "TESTPROG".to_string(),
            data_division: Some(DataDivision {
                working_storage: vec![
                    make_data_entry("WS-NAME", "X(30)", false),
                    make_data_entry("WS-OUTPUT", "X(50)", false),
                ],
                local_storage: vec![],
                linkage: vec![],
                file_section: vec![],
            }),
            procedure_division: Some(proc_div),
            source_path: None,
            exec_sql_statements: vec![],
        };

        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        assert!(
            content.contains("validate_input"),
            "String PIC should generate validate_input, got: {content}"
        );
        assert!(
            content.contains("max length"),
            "Should have length constraint, got: {content}"
        );
    }

    #[test]
    fn no_data_division_no_validate_input() {
        // Without DataDivision, field types are heuristic -- skip validation
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "CALC-SECTION".to_string(),
                paragraphs: vec![make_paragraph("CALC-PARA", vec![
                    make_move("WS-UNKNOWN", "WS-OUTPUT"),
                ])],
            }],
            paragraphs: vec![],
        };

        let program = CobolProgram {
            program_id: "TESTPROG".to_string(),
            data_division: None,
            procedure_division: Some(proc_div),
            source_path: None,
            exec_sql_statements: vec![],
        };

        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        assert!(
            !content.contains("validate_input"),
            "No DataDivision should skip validate_input, got: {content}"
        );
    }

    // -----------------------------------------------------------------------
    // Conditional compose tests
    // -----------------------------------------------------------------------

    fn make_evaluate_perform(status_field: &str, when_values: &[(&str, &str)], otherwise: Option<&str>) -> Statement {
        let mut branches = Vec::new();
        for (val, target) in when_values {
            branches.push(WhenBranch {
                values: vec![WhenValue::Value(Operand::Literal(
                    cobol_transpiler::ast::Literal::Alphanumeric(val.to_string()),
                ))],
                body: vec![make_perform(target)],
            });
        }

        let when_other = if let Some(target) = otherwise {
            vec![make_perform(target)]
        } else {
            vec![]
        };

        Statement::Evaluate(EvaluateStatement {
            subjects: vec![EvaluateSubject::Expr(Operand::DataRef(DataReference {
                name: status_field.to_string(),
                qualifiers: vec![],
                subscripts: vec![],
                ref_mod: None,
            }))],
            when_branches: branches,
            when_other,
        })
    }

    #[test]
    fn evaluate_perform_emits_conditional_compose() {
        let proc_div = ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![Section {
                name: "DISPATCH-SECTION".to_string(),
                paragraphs: vec![
                    make_paragraph("DISPATCH-PARA", vec![
                        make_evaluate_perform("WS-STATUS", &[
                            ("A", "PROCESS-ACTIVE"),
                            ("C", "PROCESS-CLOSED"),
                        ], Some("PROCESS-DEFAULT")),
                    ]),
                    make_paragraph("PROCESS-ACTIVE", vec![
                        make_move("WS-IN", "WS-OUT"),
                    ]),
                    make_paragraph("PROCESS-CLOSED", vec![
                        make_move("WS-X", "WS-Y"),
                    ]),
                    make_paragraph("PROCESS-DEFAULT", vec![
                        make_move("WS-A", "WS-B"),
                    ]),
                ],
            }],
            paragraphs: vec![],
        };

        let program = make_program_with_proc(proc_div);
        let ctx = DirectEmitterContext {
            cobol_program: &program,
            program_name: "TESTPROG".to_string(),
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.cbl"),
        };

        let files = DirectTransformEmitter.emit(&ctx);
        assert!(!files.is_empty());
        let content = &files[0].content;
        assert!(
            content.contains("compose conditional"),
            "EVALUATE...PERFORM should produce conditional compose, got: {content}"
        );
        assert!(
            content.contains("when "),
            "Should have when branches, got: {content}"
        );
        assert!(
            content.contains("process_active"),
            "Should reference process_active, got: {content}"
        );
        assert!(
            content.contains("otherwise"),
            "Should have otherwise clause, got: {content}"
        );
    }
}
