// NexCore -- Nexflow Parser: TransformDSL Builder
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Builds a typed `TransformProgram` AST from TransformDSL source text.

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::InputStream;

use crate::ast::common::*;
use crate::ast::transform::*;
use crate::generated::transformdsllexer::TransformDSLLexer;
use crate::generated::transformdslparser::*;
use crate::parse::helpers::{xform_text as terminal_text, unquote};
use super::ParseError;

/// Parse a `.xform` source string into a typed `TransformProgram`.
pub fn parse_transform(source: &str) -> Result<TransformProgram, ParseError> {
    let input = InputStream::new(source);
    let lexer = TransformDSLLexer::new(input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = TransformDSLParser::new(token_stream);

    let tree = parser
        .program()
        .map_err(|e| ParseError::grammar("TransformDSL", format!("{e:?}")))?;

    build_program(&*tree)
}

fn build_program(ctx: &ProgramContext<'_>) -> Result<TransformProgram, ParseError> {
    let imports: Vec<ImportPath> = ctx
        .importStatement_all()
        .iter()
        .filter_map(|s| {
            s.importPath().map(|p| ImportPath {
                raw: p.get_text(),
            })
        })
        .collect();

    let transforms: Vec<TransformDef> = ctx
        .transformDef_all()
        .iter()
        .map(|td| build_transform_def(td))
        .collect();

    let transform_blocks: Vec<TransformBlockDef> = ctx
        .transformBlockDef_all()
        .iter()
        .map(|tb| build_transform_block_def(tb))
        .collect();

    Ok(TransformProgram {
        imports,
        transforms,
        transform_blocks,
    })
}

fn build_transform_def(ctx: &TransformDefContext<'_>) -> TransformDef {
    let name = ctx
        .transformName()
        .map(|n| n.get_text())
        .unwrap_or_default();

    let (version, description, previous_version, compatibility) = ctx
        .transformMetadata()
        .map(|m| extract_metadata(&*m))
        .unwrap_or_default();

    let pure = ctx.purityDecl().map(|p| p.get_text().contains("true"));
    let idempotent = ctx.idempotentDecl().map(|i| i.get_text().contains("true"));

    let inputs = ctx
        .inputSpec()
        .map(|is| build_input_spec(&*is))
        .unwrap_or_default();

    let outputs = ctx
        .outputSpec()
        .map(|os| build_output_spec(&*os))
        .unwrap_or_default();

    let lookup = ctx
        .lookupDecl()
        .and_then(|ld| ld.IDENTIFIER())
        .map(|id| terminal_text(&*id));

    let lookups: Vec<LookupFieldDecl> = ctx
        .lookupsBlock()
        .map(|lb| {
            lb.lookupFieldDecl_all()
                .iter()
                .map(|lfd| {
                    let ids: Vec<_> = lfd.IDENTIFIER_all();
                    LookupFieldDecl {
                        name: ids.first().map(|id| terminal_text(id)).unwrap_or_default(),
                        source: ids.get(1).map(|id| terminal_text(id)).unwrap_or_default(),
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let state = ctx
        .stateDecl()
        .and_then(|sd| sd.IDENTIFIER())
        .map(|id| terminal_text(&*id));

    let params: Vec<ParamDecl> = ctx
        .paramsBlock()
        .map(|pb| {
            pb.paramDecl_all()
                .iter()
                .map(|pd| build_param_decl(pd))
                .collect()
        })
        .unwrap_or_default();

    let validate_input: Vec<ValidationRule> = ctx
        .validateInputBlock()
        .map(|vib| build_validation_rules(&*vib))
        .unwrap_or_default();

    let apply: Vec<Statement> = ctx
        .applyBlock()
        .map(|ab| {
            ab.statement_all()
                .iter()
                .map(|s| build_statement(s))
                .collect()
        })
        .unwrap_or_default();

    let validate_output: Vec<ValidationRule> = ctx
        .validateOutputBlock()
        .map(|vob| build_validation_rules_output(&*vob))
        .unwrap_or_default();

    let on_error = ctx.onErrorBlock().map(|oeb| build_error_block(&*oeb));
    let cache = ctx.cacheDecl().map(|cd| build_cache_decl(&*cd));

    TransformDef {
        name,
        version,
        description,
        previous_version,
        compatibility,
        pure,
        idempotent,
        cache,
        inputs,
        outputs,
        lookup,
        lookups,
        state,
        params,
        validate_input,
        apply,
        validate_output,
        on_error,
    }
}

fn build_transform_block_def(ctx: &TransformBlockDefContext<'_>) -> TransformBlockDef {
    let name = ctx
        .transformName()
        .map(|n| n.get_text())
        .unwrap_or_default();

    let (version, description, _, _) = ctx
        .transformMetadata()
        .map(|m| extract_metadata(&*m))
        .unwrap_or_default();

    let uses: Vec<String> = ctx
        .useBlock()
        .map(|ub| {
            ub.IDENTIFIER_all()
                .iter()
                .map(|id| terminal_text(id))
                .collect()
        })
        .unwrap_or_default();

    let inputs = ctx
        .inputSpec()
        .map(|is| build_input_spec(&*is))
        .unwrap_or_default();
    let outputs = ctx
        .outputSpec()
        .map(|os| build_output_spec(&*os))
        .unwrap_or_default();

    let validate_input: Vec<ValidationRule> = ctx
        .validateInputBlock()
        .map(|vib| build_validation_rules(&*vib))
        .unwrap_or_default();

    let invariants: Vec<ValidationRule> = ctx
        .invariantBlock()
        .map(|ib| {
            ib.validationRule_all()
                .iter()
                .map(|vr| ValidationRule {
                    expression: vr.get_text(),
                    message: None,
                })
                .collect()
        })
        .unwrap_or_default();

    let mappings: Vec<Mapping> = ctx
        .mappingsBlock()
        .map(|mb| {
            mb.mapping_all()
                .iter()
                .map(|m| {
                    let text = m.get_text();
                    let parts: Vec<&str> = text.splitn(2, '=').collect();
                    Mapping {
                        target: parts.first().unwrap_or(&"").trim().to_string(),
                        expression: parts.get(1).unwrap_or(&"").trim().to_string(),
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let compose = ctx.composeBlock().map(|cb| build_compose_block(&*cb));

    let validate_output: Vec<ValidationRule> = ctx
        .validateOutputBlock()
        .map(|vob| build_validation_rules_output(&*vob))
        .unwrap_or_default();

    let on_change = ctx.onChangeBlock().map(|ocb| {
        let watch_fields: Vec<String> = ocb
            .fieldArray()
            .map(|fa| {
                fa.fieldPath_all()
                    .iter()
                    .map(|fp| fp.get_text())
                    .collect()
            })
            .unwrap_or_default();

        let recalculate: Vec<Statement> = ocb
            .recalculateBlock()
            .map(|rb| {
                rb.assignment_all()
                    .iter()
                    .map(|a| Statement {
                        target: a
                            .fieldPath()
                            .map(|fp| fp.get_text())
                            .or_else(|| a.IDENTIFIER().map(|id| terminal_text(&*id)))
                            .unwrap_or_default(),
                        expression: a
                            .expression()
                            .map(|e| e.get_text())
                            .unwrap_or_default(),
                        is_let: false,
                    })
                    .collect()
            })
            .unwrap_or_default();

        OnChangeBlock {
            watch_fields,
            recalculate,
        }
    });

    let on_error = ctx.onErrorBlock().map(|oeb| build_error_block(&*oeb));

    TransformBlockDef {
        name,
        version,
        description,
        uses,
        inputs,
        outputs,
        validate_input,
        invariants,
        mappings,
        compose,
        validate_output,
        on_change,
        on_error,
    }
}

fn extract_metadata(
    ctx: &TransformMetadataContext<'_>,
) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
    let version = ctx
        .versionDecl()
        .and_then(|vd| vd.VERSION_NUMBER())
        .map(|v| terminal_text(&*v));
    let description = ctx
        .descriptionDecl()
        .and_then(|dd| dd.STRING())
        .map(|s| unquote(&terminal_text(&*s)));
    let previous_version = ctx
        .previousVersionDecl()
        .and_then(|pvd| pvd.VERSION_NUMBER())
        .map(|v| terminal_text(&*v));
    let compatibility = ctx
        .compatibilityDecl()
        .and_then(|cd| cd.compatibilityMode())
        .map(|cm| cm.get_text());

    (version, description, previous_version, compatibility)
}

fn build_input_spec(ctx: &InputSpecContext<'_>) -> Vec<FieldSpec> {
    // Single input: input: type
    if let Some(ft) = ctx.fieldType() {
        let (nullable, required, default) = ctx
            .qualifiers()
            .map(|q| extract_qualifiers(&*q))
            .unwrap_or_default();
        return vec![FieldSpec {
            name: None,
            field_type: build_field_type(&*ft),
            nullable,
            required,
            default,
        }];
    }

    // Multiple inputs
    ctx.inputFieldDecl_all()
        .iter()
        .map(|ifd| {
            let name = ifd.fieldName().map(|fn_ctx| fn_ctx.get_text());
            let field_type = ifd
                .fieldType()
                .map(|ft| build_field_type(&*ft))
                .unwrap_or_else(|| TransformFieldType::Base {
                    name: "string".to_string(),
                    constraints: Vec::new(),
                });
            let (nullable, required, default) = ifd
                .qualifiers()
                .map(|q| extract_qualifiers(&*q))
                .unwrap_or_default();
            FieldSpec {
                name,
                field_type,
                nullable,
                required,
                default,
            }
        })
        .collect()
}

fn build_output_spec(ctx: &OutputSpecContext<'_>) -> Vec<FieldSpec> {
    if let Some(ft) = ctx.fieldType() {
        let (nullable, required, default) = ctx
            .qualifiers()
            .map(|q| extract_qualifiers(&*q))
            .unwrap_or_default();
        return vec![FieldSpec {
            name: None,
            field_type: build_field_type(&*ft),
            nullable,
            required,
            default,
        }];
    }

    ctx.outputFieldDecl_all()
        .iter()
        .map(|ofd| {
            let name = ofd.fieldName().map(|fn_ctx| fn_ctx.get_text());
            let field_type = ofd
                .fieldType()
                .map(|ft| build_field_type(&*ft))
                .unwrap_or_else(|| TransformFieldType::Base {
                    name: "string".to_string(),
                    constraints: Vec::new(),
                });
            let (nullable, required, default) = ofd
                .qualifiers()
                .map(|q| extract_qualifiers(&*q))
                .unwrap_or_default();
            FieldSpec {
                name,
                field_type,
                nullable,
                required,
                default,
            }
        })
        .collect()
}

fn build_field_type(ctx: &FieldTypeContext<'_>) -> TransformFieldType {
    // Check for base type
    if let Some(bt) = ctx.baseType() {
        let name = bt.get_text();
        let constraints: Vec<FieldConstraint> = ctx
            .constraint_all()
            .iter()
            .flat_map(|c| {
                c.constraintSpec_all()
                    .iter()
                    .map(|cs| build_constraint_spec(cs))
                    .collect::<Vec<_>>()
            })
            .collect();
        return TransformFieldType::Base { name, constraints };
    }

    // Check for collection type
    if let Some(ct) = ctx.collectionType() {
        let text = ct.get_text();
        let kind = if text.starts_with("list") {
            TransformCollectionKind::List
        } else if text.starts_with("set") {
            TransformCollectionKind::Set
        } else {
            TransformCollectionKind::Map
        };
        let element_types: Vec<TransformFieldType> = ct
            .fieldType_all()
            .iter()
            .map(|ft| build_field_type(ft))
            .collect();
        return TransformFieldType::Collection { kind, element_types };
    }

    // Check for UPPER_IDENTIFIER (type alias)
    if let Some(uid) = ctx.UPPER_IDENTIFIER() {
        return TransformFieldType::AliasRef(terminal_text(&*uid));
    }

    // Check for IDENTIFIER (schema reference)
    if let Some(id) = ctx.IDENTIFIER() {
        return TransformFieldType::Reference(terminal_text(&*id));
    }

    // Fallback
    TransformFieldType::Base {
        name: ctx.get_text(),
        constraints: Vec::new(),
    }
}

fn build_constraint_spec(ctx: &ConstraintSpecContext<'_>) -> FieldConstraint {
    let text = ctx.get_text();
    if let Some(rs) = ctx.rangeSpec() {
        let nums: Vec<_> = rs.numberLiteral_all();
        return FieldConstraint::Range {
            min: nums.first().map(|n| n.get_text()),
            max: nums.get(1).map(|n| n.get_text()),
        };
    }
    if let Some(ls) = ctx.lengthSpec() {
        let ints: Vec<_> = ls.INTEGER_all();
        return FieldConstraint::Length {
            min: ints.first().and_then(|n| terminal_text(n).parse().ok()),
            max: ints.get(1).and_then(|n| terminal_text(n).parse().ok()),
        };
    }
    if let Some(s) = ctx.STRING() {
        if text.contains("pattern") {
            return FieldConstraint::Pattern(unquote(&terminal_text(&*s)));
        }
    }
    if ctx.valueList().is_some() {
        // values: a, b, c
        let values: Vec<String> = ctx
            .valueList()
            .map(|vl| {
                vl.IDENTIFIER_all()
                    .iter()
                    .map(|id| terminal_text(id))
                    .chain(vl.STRING_all().iter().map(|s| unquote(&terminal_text(s))))
                    .collect()
            })
            .unwrap_or_default();
        return FieldConstraint::Values(values);
    }
    // Precision/scale
    let ints: Vec<_> = ctx.INTEGER_all();
    if !ints.is_empty() {
        if text.contains("precision") {
            return FieldConstraint::Precision {
                precision: ints.first().and_then(|n| terminal_text(n).parse().ok()).unwrap_or(0),
                scale: ints.get(1).and_then(|n| terminal_text(n).parse().ok()),
            };
        }
        if text.contains("scale") {
            return FieldConstraint::Precision {
                precision: 0,
                scale: ints.first().and_then(|n| terminal_text(n).parse().ok()),
            };
        }
    }

    // Fallback
    FieldConstraint::Pattern(text)
}

fn extract_qualifiers(ctx: &QualifiersContext<'_>) -> (bool, bool, Option<String>) {
    let mut nullable = false;
    let mut required = false;
    let mut default = None;

    for q in ctx.qualifier_all() {
        let text = q.get_text();
        if text == "nullable" {
            nullable = true;
        } else if text == "required" {
            required = true;
        } else if text.starts_with("default") {
            default = q.expression().map(|e| e.get_text());
        }
    }

    (nullable, required, default)
}

fn build_param_decl(ctx: &ParamDeclContext<'_>) -> ParamDecl {
    let name = ctx
        .IDENTIFIER()
        .map(|id| terminal_text(&*id))
        .unwrap_or_default();
    let field_type = ctx
        .fieldType()
        .map(|ft| build_field_type(&*ft))
        .unwrap_or_else(|| TransformFieldType::Base {
            name: "string".to_string(),
            constraints: Vec::new(),
        });

    let mut required = false;
    let mut optional = false;
    let mut default = None;

    if let Some(pq) = ctx.paramQualifiers() {
        let text = pq.get_text();
        required = text.contains("required");
        optional = text.contains("optional");
        // Extract default from paramDefault context
        if let Some(pd) = pq.paramDefault() {
            default = pd.expression().map(|e| e.get_text());
        }
    }

    ParamDecl {
        name,
        field_type,
        required,
        optional,
        default,
    }
}

fn build_statement(ctx: &StatementContext<'_>) -> Statement {
    if let Some(la) = ctx.letAssignment() {
        let name = la
            .IDENTIFIER()
            .map(|id| terminal_text(&*id))
            .unwrap_or_default();
        let expression = la
            .expression()
            .map(|e| e.get_text())
            .unwrap_or_default();
        return Statement {
            target: name,
            expression,
            is_let: true,
        };
    }

    if let Some(a) = ctx.assignment() {
        let target = a
            .fieldPath()
            .map(|fp| fp.get_text())
            .or_else(|| a.IDENTIFIER().map(|id| terminal_text(&*id)))
            .unwrap_or_default();
        let expression = a
            .expression()
            .map(|e| e.get_text())
            .unwrap_or_default();
        return Statement {
            target,
            expression,
            is_let: false,
        };
    }

    Statement {
        target: String::new(),
        expression: ctx.get_text(),
        is_let: false,
    }
}

fn build_validation_rules(ctx: &ValidateInputBlockContext<'_>) -> Vec<ValidationRule> {
    ctx.validationRule_all()
        .iter()
        .map(|vr| {
            let text = vr.get_text();
            let expression = vr
                .expression()
                .map(|e| e.get_text())
                .unwrap_or_else(|| text.clone());
            let message = vr
                .validationMessage()
                .map(|vm| {
                    if let Some(s) = vm.STRING() {
                        unquote(&terminal_text(&*s))
                    } else {
                        vm.get_text()
                    }
                });
            ValidationRule {
                expression,
                message,
            }
        })
        .collect()
}

fn build_validation_rules_output(ctx: &ValidateOutputBlockContext<'_>) -> Vec<ValidationRule> {
    ctx.validationRule_all()
        .iter()
        .map(|vr| ValidationRule {
            expression: vr
                .expression()
                .map(|e| e.get_text())
                .unwrap_or_else(|| vr.get_text()),
            message: vr
                .validationMessage()
                .map(|vm| vm.get_text()),
        })
        .collect()
}

fn build_error_block(ctx: &OnErrorBlockContext<'_>) -> ErrorBlock {
    let mut block = ErrorBlock {
        action: None,
        default: None,
        log_level: None,
        emit_to: None,
        error_code: None,
    };

    for stmt in ctx.errorStatement_all() {
        let text = stmt.get_text();
        if let Some(ea) = stmt.errorAction() {
            let ea_text = ea.get_text();
            if ea_text.starts_with("action") {
                if let Some(eat) = ea.errorActionType() {
                    block.action = Some(eat.get_text());
                }
            } else if ea_text.starts_with("default") {
                block.default = ea.expression().map(|e| e.get_text());
            } else if ea_text.starts_with("log_level") {
                block.log_level = ea.logLevel().map(|ll| ll.get_text());
            } else if ea_text.starts_with("emit_to") {
                block.emit_to = ea.IDENTIFIER().map(|id| terminal_text(&*id));
            } else if ea_text.starts_with("error_code") {
                block.error_code = ea.STRING().map(|s| unquote(&terminal_text(&*s)));
            }
        }
        let _ = text;
    }

    block
}

fn build_cache_decl(ctx: &CacheDeclContext<'_>) -> CacheDecl {
    let ttl = ctx
        .cacheTtl()
        .and_then(|ct| ct.duration())
        .map(|d| d.get_text())
        .or_else(|| ctx.duration().map(|d| d.get_text()));

    let key: Vec<String> = ctx
        .cacheKey()
        .and_then(|ck| ck.fieldArray())
        .map(|fa| {
            fa.fieldPath_all()
                .iter()
                .map(|fp| fp.get_text())
                .collect()
        })
        .unwrap_or_default();

    CacheDecl { ttl, key }
}

fn build_compose_block(ctx: &ComposeBlockContext<'_>) -> ComposeBlock {
    let compose_type = ctx.composeType().map(|ct| ct.get_text());

    let refs: Vec<ComposeRef> = ctx
        .composeRef_all()
        .iter()
        .map(|cr| {
            let text = cr.get_text();
            if let Some(id) = cr.IDENTIFIER() {
                let id_text = terminal_text(&*id);
                if text.starts_with("when") {
                    ComposeRef::Conditional {
                        condition: cr
                            .expression()
                            .map(|e| e.get_text())
                            .unwrap_or_default(),
                        transform: id_text,
                    }
                } else if text.starts_with("otherwise") {
                    ComposeRef::Otherwise(id_text)
                } else {
                    ComposeRef::Simple(id_text)
                }
            } else {
                ComposeRef::Simple(text)
            }
        })
        .collect();

    let then = ctx.thenBlock().map(|tb| {
        let then_type = tb.composeType().map(|ct| ct.get_text());
        let then_refs: Vec<ComposeRef> = tb
            .composeRef_all()
            .iter()
            .map(|cr| {
                if let Some(id) = cr.IDENTIFIER() {
                    ComposeRef::Simple(terminal_text(&*id))
                } else {
                    ComposeRef::Simple(cr.get_text())
                }
            })
            .collect();
        ThenBlock {
            compose_type: then_type,
            refs: then_refs,
        }
    });

    ComposeBlock {
        compose_type,
        refs,
        then,
    }
}
