//! Direct transform emitter: generates `.xform` files from COBOL AST.
//!
//! Reads the COBOL AST's paragraphs and their statements to classify
//! transforms and extract real expressions from COMPUTE/MOVE/arithmetic
//! statements. Unlike the legacy emitter, this produces actual mappings
//! instead of placeholders.

use std::collections::HashSet;

use cobol_transpiler::ast::{Paragraph, ProcedureDivision};

use super::cobol_extract::analyze_statement;
use super::condition_extract::{extract_mappings, ExtractedMapping};
use super::{DirectDslEmitter, DirectEmitterContext};
use crate::dsl::dsl_ast::*;
use crate::dsl::transform_emitter::{
    group_by_section, sanitize_identifier, ClassifiedTransform, TransformKind,
};
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

        let enriched = classify_and_extract(proc_div);
        if enriched.is_empty() {
            return vec![];
        }

        // Group by section using the ClassifiedTransform inside each enriched entry
        let transforms: Vec<ClassifiedTransform> = enriched.iter().map(|e| e.classified.clone()).collect();
        let sections = group_by_section(&transforms);

        // Build a lookup from nexflow_name -> extracted mappings
        let mapping_lookup: std::collections::HashMap<&str, &[ExtractedMapping]> = enriched
            .iter()
            .map(|e| (e.classified.nexflow_name.as_str(), e.mappings.as_slice()))
            .collect();

        let mut dsl_files = Vec::new();
        for (section_name, section_transforms) in &sections {
            let (content, notes, confidence) =
                generate_transform_file_direct(section_name, section_transforms, &ctx.program_name, &mapping_lookup);
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

        dsl_files
    }
}

/// A ClassifiedTransform enriched with extracted expression mappings.
struct EnrichedTransform {
    classified: ClassifiedTransform,
    mappings: Vec<ExtractedMapping>,
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
    })
}

// ---------------------------------------------------------------------------
// Direct-specific generation (replaces placeholders with real expressions)
// ---------------------------------------------------------------------------

fn generate_transform_file_direct(
    section_name: &str,
    transforms: &[&ClassifiedTransform],
    program: &str,
    mapping_lookup: &std::collections::HashMap<&str, &[ExtractedMapping]>,
) -> (String, Vec<String>, f64) {
    let mut items = Vec::new();
    let mut notes = Vec::new();

    for t in transforms {
        let extracted = mapping_lookup.get(t.nexflow_name.as_str()).copied().unwrap_or(&[]);
        match t.kind {
            TransformKind::SingleField => {
                items.push(build_single_transform(t, extracted, &mut notes));
            }
            TransformKind::MultiField => {
                items.push(build_multi_transform(t, extracted, &mut notes));
            }
            TransformKind::Compose | TransformKind::SectionDispatcher => {
                items.push(build_compose_transform(t));
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
    notes: &mut Vec<String>,
) -> TransformItem {
    let input = build_io_spec(&t.reads);
    let output = if t.writes.len() == 1 {
        IoSpec::Single(
            Ident::new(&cobol_to_snake(&t.writes[0])),
            cobol_name_to_type_hint(&t.writes[0]),
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
        pure: true,
        input,
        output,
        apply,
    })
}

fn build_multi_transform(
    t: &ClassifiedTransform,
    extracted: &[ExtractedMapping],
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
        input,
        output,
        body: TransformBlockBody::Mappings(mappings),
    })
}

fn build_compose_transform(t: &ClassifiedTransform) -> TransformItem {
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
}
