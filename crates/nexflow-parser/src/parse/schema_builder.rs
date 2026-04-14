// NexCore -- Nexflow Parser: SchemaDSL Builder
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Builds a typed `SchemaProgram` AST from SchemaDSL source text.
//!
//! Entry point: `parse_schema(source) -> Result<SchemaProgram>`

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::InputStream;

use crate::ast::common::*;
use crate::ast::schema::*;
use crate::generated::schemadsllexer::SchemaDSLLexer;
use crate::generated::schemadslparser::*;
use crate::parse::helpers::{schema_text as terminal_text, unquote};
use super::ParseError;

/// Parse a `.schema` source string into a typed `SchemaProgram`.
pub fn parse_schema(source: &str) -> Result<SchemaProgram, ParseError> {
    let input = InputStream::new(source);
    let lexer = SchemaDSLLexer::new(input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = SchemaDSLParser::new(token_stream);

    let tree = parser
        .program()
        .map_err(|e| ParseError::grammar("SchemaDSL", format!("{e:?}")))?;

    build_program(&*tree)
}

fn build_program(ctx: &ProgramContext<'_>) -> Result<SchemaProgram, ParseError> {
    let imports: Vec<ImportPath> = ctx
        .importStatement_all()
        .iter()
        .filter_map(|s| {
            s.importPath().map(|p| ImportPath {
                raw: p.get_text(),
            })
        })
        .collect();

    let mut schemas = Vec::new();
    for sd in ctx.schemaDefinition_all() {
        let mut def = build_schema_definition(&*sd)?;
        // Top-level imports are shared across all schemas in the file
        def.imports = imports.clone();
        schemas.push(def);
    }

    let type_aliases: Vec<TypeAliasBlock> = ctx
        .typeAliasBlock_all()
        .iter()
        .map(|tab| build_type_alias_block(tab))
        .collect();

    Ok(SchemaProgram {
        imports,
        schemas,
        type_aliases,
    })
}

fn build_schema_definition(ctx: &SchemaDefinitionContext<'_>) -> Result<SchemaDefinition, ParseError> {
    let name = ctx
        .schemaName()
        .map(|n| n.get_text())
        .unwrap_or_default();

    let patterns: Vec<MutationPattern> = ctx
        .patternDecl()
        .map(|pd| {
            pd.mutationPattern_all()
                .iter()
                .filter_map(|mp| parse_mutation_pattern(&mp.get_text()))
                .collect()
        })
        .unwrap_or_default();

    let targets: Vec<String> = ctx
        .targetsDecl()
        .and_then(|td| td.targetList())
        .map(|tl| {
            tl.target_all()
                .iter()
                .map(|t| t.get_text())
                .collect()
        })
        .unwrap_or_default();

    let version = ctx.versionBlock().map(|vb| build_version_block(&*vb));

    // Standalone compatibility (outside version block)
    let compatibility = ctx
        .compatibilityDecl()
        .and_then(|cd| cd.compatibilityMode())
        .and_then(|cm| parse_compatibility_mode(&cm.get_text()));

    let retention = ctx
        .retentionDecl()
        .and_then(|rd| rd.duration())
        .map(|d| build_duration(&*d));

    let identity: Vec<FieldDecl> = ctx
        .identityBlock()
        .map(|ib| {
            ib.identityFieldV2_all()
                .iter()
                .map(|f| build_field_from_identity(f))
                .collect()
        })
        .unwrap_or_default();

    let streaming = ctx.streamingBlock().map(|sb| build_streaming_block(&*sb));

    let serialization = ctx
        .serializationBlock()
        .map(|sb| build_serialization_block(&*sb));

    let fields: Vec<FieldDecl> = ctx
        .fieldsBlock()
        .map(|fb| {
            fb.fieldDeclV2_all()
                .iter()
                .map(|f| build_field_decl(f))
                .collect()
        })
        .unwrap_or_default();

    let nested_objects: Vec<NestedObject> = ctx
        .nestedObjectBlock_all()
        .iter()
        .map(|nob| build_nested_object(nob))
        .collect();

    let computed: Vec<ComputedField> = ctx
        .computedBlock()
        .map(|cb| {
            cb.computedField_all()
                .iter()
                .map(|cf| build_computed_field(cf))
                .collect()
        })
        .unwrap_or_default();

    let constraints: Vec<ConstraintDecl> = ctx
        .constraintsBlock()
        .map(|cb| {
            cb.constraintDecl_all()
                .iter()
                .map(|cd| build_constraint_decl(cd))
                .collect()
        })
        .unwrap_or_default();

    let immutable = ctx.immutableDecl().map(|id| {
        let text = id.get_text();
        text.contains("true")
    });

    let state_machine = ctx
        .stateMachineBlock()
        .map(|smb| build_state_machine_block(&*smb));

    let parameters: Vec<ParameterDecl> = ctx
        .parametersBlock()
        .map(|pb| {
            pb.parameterDeclV2_all()
                .iter()
                .map(|pd| build_parameter_decl(pd))
                .collect()
        })
        .unwrap_or_default();

    let entries: Vec<EntryDecl> = ctx
        .entriesBlock()
        .map(|eb| {
            eb.entryDecl_all()
                .iter()
                .map(|ed| build_entry_decl(ed))
                .collect()
        })
        .unwrap_or_default();

    let rules: Vec<RuleBlock> = ctx
        .ruleBlock_all()
        .iter()
        .map(|rb| build_rule_block(rb))
        .collect();

    let migration: Vec<MigrationStatement> = ctx
        .migrationBlock()
        .map(|mb| {
            mb.migrationStatement_all()
                .iter()
                .map(|ms| MigrationStatement {
                    raw: ms.get_text(),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(SchemaDefinition {
        imports: Vec::new(),
        name,
        patterns,
        targets,
        version,
        compatibility,
        retention,
        identity,
        streaming,
        serialization,
        fields,
        nested_objects,
        computed,
        constraints,
        immutable,
        state_machine,
        parameters,
        entries,
        rules,
        migration,
    })
}

// -- Pattern parsing --

fn parse_mutation_pattern(text: &str) -> Option<MutationPattern> {
    match text {
        "master_data" => Some(MutationPattern::MasterData),
        "immutable_ledger" => Some(MutationPattern::ImmutableLedger),
        "versioned_configuration" => Some(MutationPattern::VersionedConfiguration),
        "operational_parameters" => Some(MutationPattern::OperationalParameters),
        "event_log" => Some(MutationPattern::EventLog),
        "state_machine" => Some(MutationPattern::StateMachine),
        "temporal_data" => Some(MutationPattern::TemporalData),
        "reference_data" => Some(MutationPattern::ReferenceData),
        "business_logic" => Some(MutationPattern::BusinessLogic),
        "command" => Some(MutationPattern::Command),
        "response" => Some(MutationPattern::Response),
        "aggregate" => Some(MutationPattern::Aggregate),
        "document" => Some(MutationPattern::Document),
        "audit_event" => Some(MutationPattern::AuditEvent),
        _ => None,
    }
}

fn parse_compatibility_mode(text: &str) -> Option<CompatibilityMode> {
    match text {
        "backward" | "backward_compatible" => Some(CompatibilityMode::Backward),
        "forward" | "forward_compatible" => Some(CompatibilityMode::Forward),
        "full" => Some(CompatibilityMode::Full),
        "none" => Some(CompatibilityMode::None),
        _ => None,
    }
}

// -- Version --

fn build_version_block(ctx: &VersionBlockContext<'_>) -> VersionInfo {
    let number = ctx
        .VERSION_NUMBER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let compatibility = ctx
        .compatibilityDecl()
        .and_then(|cd| cd.compatibilityMode())
        .and_then(|cm| parse_compatibility_mode(&cm.get_text()));

    let previous_version = ctx
        .previousVersionDecl()
        .and_then(|pvd| pvd.VERSION_NUMBER())
        .map(|n| terminal_text(&*n));

    let deprecation = ctx.deprecationDecl().map(|dd| {
        let strings: Vec<_> = dd.STRING_all();
        let message = strings
            .first()
            .map(|s| unquote(&terminal_text(s)))
            .unwrap_or_default();
        let since = strings.get(1).map(|s| unquote(&terminal_text(s)));
        let removal_version = dd
            .VERSION_NUMBER()
            .map(|n| terminal_text(&*n));
        DeprecationInfo {
            message,
            since,
            removal_version,
        }
    });

    let migration_guide = ctx.migrationGuideDecl().map(|mgd| {
        if let Some(s) = mgd.STRING() {
            unquote(&terminal_text(&*s))
        } else if let Some(ms) = mgd.MULTILINE_STRING() {
            let raw = terminal_text(&*ms);
            // Strip leading/trailing """
            raw.strip_prefix("\"\"\"")
                .and_then(|s| s.strip_suffix("\"\"\""))
                .unwrap_or(&raw)
                .to_string()
        } else {
            String::new()
        }
    });

    VersionInfo {
        number,
        compatibility,
        previous_version,
        deprecation,
        migration_guide,
    }
}

// -- Duration --

fn build_duration(ctx: &DurationContext<'_>) -> Duration {
    if let Some(dl) = ctx.DURATION_LITERAL() {
        let text = terminal_text(&*dl);
        // Parse "5s", "10m", "1h", etc.
        let (val, unit) = split_duration_literal(&text);
        Duration { value: val, unit }
    } else {
        let value = ctx
            .INTEGER()
            .and_then(|n| terminal_text(&*n).parse().ok())
            .unwrap_or(0);
        let unit = ctx
            .timeUnit()
            .map(|tu| tu.get_text())
            .unwrap_or_else(|| "seconds".to_string());
        Duration { value, unit }
    }
}

fn split_duration_literal(text: &str) -> (i64, String) {
    let num_end = text.find(|c: char| !c.is_ascii_digit()).unwrap_or(text.len());
    let val: i64 = text[..num_end].parse().unwrap_or(0);
    let suffix = &text[num_end..];
    let unit = match suffix {
        "ms" => "milliseconds",
        "s" => "seconds",
        "m" => "minutes",
        "h" => "hours",
        "d" => "days",
        "w" => "weeks",
        _ => suffix,
    };
    (val, unit.to_string())
}

// -- Fields --

fn build_field_decl(ctx: &FieldDeclV2Context<'_>) -> FieldDecl {
    let name = ctx
        .fieldName()
        .map(|fn_ctx| fn_ctx.get_text())
        .unwrap_or_default();

    let field_type = ctx
        .fieldTypeV2()
        .map(|ft| build_field_type(&*ft))
        .unwrap_or_else(|| FieldType::Base {
            name: "string".to_string(),
            params: Vec::new(),
        });

    let (required, optional, unique, cannot_change, encrypted, pii, default, deprecated, removal) =
        extract_qualifiers(&ctx.fieldQualifierV2_all());

    FieldDecl {
        name,
        field_type,
        required,
        optional,
        unique,
        cannot_change,
        encrypted,
        pii,
        default,
        deprecated,
        removal,
    }
}

fn build_field_from_identity(ctx: &IdentityFieldV2Context<'_>) -> FieldDecl {
    let name = ctx
        .fieldName()
        .map(|fn_ctx| fn_ctx.get_text())
        .unwrap_or_default();

    let field_type = ctx
        .fieldTypeV2()
        .map(|ft| build_field_type(&*ft))
        .unwrap_or_else(|| FieldType::Base {
            name: "string".to_string(),
            params: Vec::new(),
        });

    let (required, optional, unique, cannot_change, encrypted, pii, default, deprecated, removal) =
        extract_qualifiers(&ctx.fieldQualifierV2_all());

    FieldDecl {
        name,
        field_type,
        required,
        optional,
        unique,
        cannot_change,
        encrypted,
        pii,
        default,
        deprecated,
        removal,
    }
}

#[allow(clippy::type_complexity)]
fn extract_qualifiers(
    qualifiers: &[std::rc::Rc<FieldQualifierV2ContextAll<'_>>],
) -> (
    bool,
    bool,
    bool,
    bool,
    bool,
    Option<PiiModifier>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let mut required = false;
    let mut optional = false;
    let mut unique = false;
    let mut cannot_change = false;
    let mut encrypted = false;
    let mut pii = None;
    let mut default = None;
    let mut deprecated = None;
    let mut removal = None;

    for q in qualifiers {
        let text = q.get_text();
        if text == "required" {
            required = true;
        } else if text == "optional" {
            optional = true;
        } else if text == "unique" {
            unique = true;
        } else if text == "cannot_change" {
            cannot_change = true;
        } else if text == "encrypted" {
            encrypted = true;
        } else if text.starts_with("pii") {
            if let Some(pm) = q.piiModifier() {
                let profile = pm.IDENTIFIER().map(|id| terminal_text(&*id));
                pii = Some(PiiModifier { profile });
            }
        } else if text.starts_with("default") {
            if let Some(dc) = q.defaultClauseV2() {
                // Get everything between the parens
                let raw = dc.get_text();
                // Strip "default(" prefix and ")" suffix
                let inner = raw
                    .strip_prefix("default(")
                    .and_then(|s| s.strip_suffix(')'))
                    .unwrap_or(&raw);
                default = Some(unquote(inner));
            }
        } else if text.starts_with("deprecated") {
            if let Some(dc) = q.deprecatedClauseV2() {
                if let Some(s) = dc.STRING() {
                    deprecated = Some(unquote(&terminal_text(&*s)));
                }
            }
        } else if text.starts_with("removal") {
            if let Some(dc) = q.deprecatedClauseV2() {
                if let Some(v) = dc.VERSION_NUMBER() {
                    removal = Some(terminal_text(&*v));
                }
            }
        }
    }

    (required, optional, unique, cannot_change, encrypted, pii, default, deprecated, removal)
}

// -- Field types --

fn build_field_type(ctx: &FieldTypeV2Context<'_>) -> FieldType {
    if let Some(bt) = ctx.baseTypeV2() {
        return build_base_type(&*bt);
    }
    if let Some(ct) = ctx.collectionTypeV2() {
        return build_collection_type(&*ct);
    }
    if let Some(iot) = ctx.inlineObjectTypeV2() {
        let fields: Vec<FieldDecl> = iot
            .inlineFieldDeclV2_all()
            .iter()
            .map(|f| {
                let name = f.fieldName().map(|fn_ctx| fn_ctx.get_text()).unwrap_or_default();
                let field_type = f
                    .fieldTypeV2()
                    .map(|ft| build_field_type(&*ft))
                    .unwrap_or_else(|| FieldType::Base {
                        name: "string".to_string(),
                        params: Vec::new(),
                    });
                let (required, optional, unique, cannot_change, encrypted, pii, default, deprecated, removal) =
                    extract_qualifiers(&f.fieldQualifierV2_all());
                FieldDecl {
                    name,
                    field_type,
                    required,
                    optional,
                    unique,
                    cannot_change,
                    encrypted,
                    pii,
                    default,
                    deprecated,
                    removal,
                }
            })
            .collect();
        return FieldType::InlineObject { fields };
    }
    if let Some(id) = ctx.UPPER_IDENTIFIER() {
        return FieldType::Alias(terminal_text(&*id));
    }
    if let Some(id) = ctx.IDENTIFIER() {
        return FieldType::Custom(terminal_text(&*id));
    }
    FieldType::Base {
        name: ctx.get_text(),
        params: Vec::new(),
    }
}

fn build_base_type(ctx: &BaseTypeV2Context<'_>) -> FieldType {
    let full_text = ctx.get_text();

    // Extract the type name (everything before optional params)
    let type_name = if full_text.contains('(') {
        full_text.split('(').next().unwrap_or(&full_text)
    } else {
        &full_text
    };

    let params: Vec<i64> = ctx
        .typeParams()
        .map(|tp| {
            tp.INTEGER_all()
                .iter()
                .filter_map(|n| terminal_text(n).parse().ok())
                .collect()
        })
        .unwrap_or_default();

    FieldType::Base {
        name: type_name.to_string(),
        params,
    }
}

fn build_collection_type(ctx: &CollectionTypeV2Context<'_>) -> FieldType {
    let text = ctx.get_text();
    let kind = if text.starts_with("list") {
        CollectionKind::List
    } else if text.starts_with("set") {
        CollectionKind::Set
    } else {
        CollectionKind::Map
    };

    let element_types: Vec<FieldType> = ctx
        .fieldTypeV2_all()
        .iter()
        .map(|ft| build_field_type(ft))
        .collect();

    FieldType::Collection {
        kind,
        element_types,
    }
}

// -- Nested objects --

fn build_nested_object(ctx: &NestedObjectBlockContext<'_>) -> NestedObject {
    let name = ctx
        .fieldName()
        .map(|fn_ctx| fn_ctx.get_text())
        .unwrap_or_default();

    // If LANGLE is present, it's list<object>
    let is_list = ctx.LANGLE().is_some();

    let fields: Vec<FieldDecl> = ctx
        .fieldDeclV2_all()
        .iter()
        .map(|f| build_field_decl(f))
        .collect();

    let nested: Vec<NestedObject> = ctx
        .nestedObjectBlock_all()
        .iter()
        .map(|nob| build_nested_object(nob))
        .collect();

    NestedObject {
        name,
        is_list,
        fields,
        nested,
    }
}

// -- Computed fields --

fn build_computed_field(ctx: &ComputedFieldContext<'_>) -> ComputedField {
    let name = ctx
        .fieldName()
        .map(|fn_ctx| fn_ctx.get_text())
        .unwrap_or_default();

    let expression = ctx
        .computedExpression()
        .map(|ce| ce.get_text())
        .unwrap_or_default();

    ComputedField { name, expression }
}

// -- Constraints --

fn build_constraint_decl(ctx: &ConstraintDeclContext<'_>) -> ConstraintDecl {
    let field = ctx
        .fieldPath()
        .map(|fp| fp.get_text())
        .unwrap_or_default();

    if let Some(ec) = ctx.enumConstraint() {
        let values: Vec<String> = ec
            .enumValue_all()
            .iter()
            .map(|ev| {
                if let Some(s) = ev.STRING() {
                    unquote(&terminal_text(&*s))
                } else {
                    ev.get_text()
                }
            })
            .collect();
        return ConstraintDecl::Enum { field, values };
    }

    if let Some(rc) = ctx.rangeConstraint() {
        let nums: Vec<_> = rc.numberLiteral_all();
        let min = nums.first().map(|n| n.get_text()).unwrap_or_default();
        let max = nums.get(1).map(|n| n.get_text()).unwrap_or_default();
        return ConstraintDecl::Range { field, min, max };
    }

    if let Some(pc) = ctx.patternConstraint() {
        let regex = pc
            .STRING()
            .map(|s| unquote(&terminal_text(&*s)))
            .unwrap_or_default();
        return ConstraintDecl::Pattern { field, regex };
    }

    if let Some(lc) = ctx.lengthConstraint() {
        let ints: Vec<_> = lc.INTEGER_all();
        let min: i64 = ints
            .first()
            .and_then(|n| terminal_text(n).parse().ok())
            .unwrap_or(0);
        let max: Option<i64> = ints.get(1).and_then(|n| terminal_text(n).parse().ok());
        return ConstraintDecl::Length { field, min, max };
    }

    // Business rule: condition as "message"
    let condition = ctx
        .condition()
        .map(|c| c.get_text())
        .unwrap_or_default();
    let message = ctx
        .STRING()
        .map(|s| unquote(&terminal_text(&*s)))
        .unwrap_or_default();

    ConstraintDecl::BusinessRule { condition, message }
}

// -- Streaming --

fn build_streaming_block(ctx: &StreamingBlockContext<'_>) -> StreamingBlock {
    let mut block = StreamingBlock {
        key_fields: Vec::new(),
        time_field: None,
        time_semantics: None,
        watermark_delay: None,
        watermark_strategy: None,
        max_out_of_orderness: None,
        watermark_interval: None,
        watermark_field: None,
        late_data_handling: None,
        late_data_stream: None,
        allowed_lateness: None,
        idle_timeout: None,
        idle_behavior: None,
    };

    for decl in ctx.streamingDecl_all() {
        if let Some(kf) = decl.keyFieldsDecl() {
            if let Some(fa) = kf.fieldArray() {
                block.key_fields = fa
                    .fieldPath_all()
                    .iter()
                    .map(|fp| fp.get_text())
                    .collect();
            }
        }
        if let Some(tf) = decl.timeFieldDecl() {
            block.time_field = Some(
                tf.fieldPath()
                    .map(|fp| fp.get_text())
                    .or_else(|| tf.timeSemanticsType().map(|t| t.get_text()))
                    .unwrap_or_default(),
            );
        }
        if let Some(ts) = decl.timeSemanticsDecl() {
            if let Some(tst) = ts.timeSemanticsType() {
                block.time_semantics = parse_time_semantics(&tst.get_text());
            }
        }
        if let Some(wd) = decl.watermarkDecl() {
            let text = wd.get_text();
            if text.starts_with("watermark_delay") || text.starts_with("max_out_of_orderness") {
                if let Some(d) = wd.duration() {
                    let dur = build_duration(&*d);
                    if text.starts_with("watermark_delay") {
                        block.watermark_delay = Some(dur);
                    } else {
                        block.max_out_of_orderness = Some(dur);
                    }
                }
            } else if text.starts_with("watermark_strategy") {
                if let Some(ws) = wd.watermarkStrategy() {
                    block.watermark_strategy = Some(ws.get_text());
                }
            } else if text.starts_with("watermark_interval") {
                if let Some(d) = wd.duration() {
                    block.watermark_interval = Some(build_duration(&*d));
                }
            } else if text.starts_with("watermark_field") {
                block.watermark_field = wd.fieldPath().map(|fp| fp.get_text());
            }
        }
        if let Some(ld) = decl.lateDataDecl() {
            let text = ld.get_text();
            if text.starts_with("late_data_handling") {
                block.late_data_handling = ld.lateDataStrategy().map(|s| s.get_text());
            } else if text.starts_with("late_data_stream") {
                block.late_data_stream = ld.IDENTIFIER().map(|id| terminal_text(&*id));
            }
        }
        if let Some(al) = decl.allowedLatenessDecl() {
            if let Some(d) = al.duration() {
                block.allowed_lateness = Some(build_duration(&*d));
            }
        }
        if let Some(id) = decl.idleDecl() {
            let text = id.get_text();
            if text.starts_with("idle_timeout") {
                if let Some(d) = id.duration() {
                    block.idle_timeout = Some(build_duration(&*d));
                }
            } else if text.starts_with("idle_behavior") {
                block.idle_behavior = id.idleBehavior().map(|ib| ib.get_text());
            }
        }
    }

    block
}

fn parse_time_semantics(text: &str) -> Option<TimeSemantics> {
    match text {
        "event_time" => Some(TimeSemantics::EventTime),
        "processing_time" => Some(TimeSemantics::ProcessingTime),
        "ingestion_time" => Some(TimeSemantics::IngestionTime),
        _ => None,
    }
}

// -- Serialization --

fn build_serialization_block(ctx: &SerializationBlockContext<'_>) -> SerializationBlock {
    let mut block = SerializationBlock {
        format: None,
        compatibility: None,
        subject: None,
        registry: None,
    };

    for decl in ctx.serializationDecl_all() {
        if let Some(fd) = decl.formatDecl() {
            if let Some(sf) = fd.serializationFormat() {
                block.format = Some(sf.get_text());
            }
        }
        if let Some(cd) = decl.serializationCompatibilityDecl() {
            if let Some(cm) = cd.compatibilityMode() {
                block.compatibility = parse_compatibility_mode(&cm.get_text());
            }
        }
        if let Some(sd) = decl.subjectDecl() {
            block.subject = sd.STRING().map(|s| unquote(&terminal_text(&*s)));
        }
        if let Some(rd) = decl.registryDecl() {
            block.registry = rd.STRING().map(|s| unquote(&terminal_text(&*s)));
        }
    }

    block
}

// -- State machine --

fn build_state_machine_block(ctx: &StateMachineBlockContext<'_>) -> StateMachineBlock {
    let for_entity = ctx
        .forEntityDecl()
        .and_then(|fe| fe.IDENTIFIER())
        .map(|id| terminal_text(&*id));

    let mut states = Vec::new();
    if let Some(sb) = ctx.statesBlock() {
        // Array syntax: states [s1, s2, s3]
        if let Some(sd) = sb.statesDecl() {
            if let Some(sa) = sd.stateArray() {
                for id in sa.IDENTIFIER_all() {
                    states.push(StateDecl {
                        name: terminal_text(&*id),
                        qualifier: None,
                    });
                }
            }
        }
        // Inline syntax: each state on its own line
        if let Some(sdl) = sb.stateDefList() {
            for sdef in sdl.stateDef_all() {
                let name = sdef
                    .IDENTIFIER()
                    .map(|id| terminal_text(&*id))
                    .unwrap_or_default();
                let qualifier = sdef.stateQualifier().and_then(|sq| {
                    let text = sq.get_text();
                    match text.as_str() {
                        "initial" => Some(StateQualifier::Initial),
                        "terminal" | "final" => Some(StateQualifier::Terminal),
                        _ => None,
                    }
                });
                states.push(StateDecl { name, qualifier });
            }
        }
    }

    let initial_state = ctx
        .initialStateDecl()
        .and_then(|isd| isd.IDENTIFIER())
        .map(|id| terminal_text(&*id));

    let mut transitions = Vec::new();
    if let Some(tb) = ctx.transitionsBlock() {
        for td in tb.transitionDecl_all() {
            let state = td
                .IDENTIFIER()
                .map(|id| terminal_text(&*id))
                .unwrap_or_default();
            let targets: Vec<String> = td
                .stateArray()
                .map(|sa| {
                    sa.IDENTIFIER_all()
                        .iter()
                        .map(|id| terminal_text(id))
                        .collect()
                })
                .unwrap_or_default();
            transitions.push(TransitionDecl::From { state, targets });
        }
        for tad in tb.transitionArrowDecl_all() {
            let ids: Vec<_> = tad.IDENTIFIER_all();
            let from_text = ids.first().map(|id| terminal_text(id));
            // Check for wildcard '*'
            let from = from_text
                .or_else(|| {
                    if tad.get_text().starts_with('*') {
                        Some("*".to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            let to = ids
                .get(1)
                .or(ids.first())
                .map(|id| terminal_text(id))
                .unwrap_or_default();
            // Fix: if from == to and there are 2+ ids, from is first, to is second
            let (from, to) = if ids.len() >= 2 {
                (terminal_text(&*ids[0]), terminal_text(&*ids[1]))
            } else if from == "*" {
                (from, ids.first().map(|id| terminal_text(id)).unwrap_or_default())
            } else {
                (from, to)
            };
            let trigger = ids.get(2).map(|id| terminal_text(id));
            transitions.push(TransitionDecl::Arrow { from, to, trigger });
        }
    }

    let mut on_transition = Vec::new();
    if let Some(otb) = ctx.onTransitionBlock() {
        for ta in otb.transitionAction_all() {
            let to_state = ta
                .IDENTIFIER()
                .map(|id| terminal_text(&*id))
                .unwrap_or_default();
            if let Some(ac) = ta.actionCall() {
                let action_name = ac
                    .IDENTIFIER()
                    .map(|id| terminal_text(&*id))
                    .unwrap_or_default();
                let args: Vec<String> = ac
                    .STRING_all()
                    .iter()
                    .map(|s| unquote(&terminal_text(s)))
                    .collect();
                on_transition.push(TransitionAction {
                    to_state,
                    action_name,
                    args,
                });
            }
        }
    }

    StateMachineBlock {
        for_entity,
        states,
        initial_state,
        transitions,
        on_transition,
    }
}

// -- Parameters --

fn build_parameter_decl(ctx: &ParameterDeclV2Context<'_>) -> ParameterDecl {
    let name = ctx
        .fieldName()
        .map(|fn_ctx| fn_ctx.get_text())
        .unwrap_or_default();

    let field_type = ctx
        .fieldTypeV2()
        .map(|ft| build_field_type(&*ft))
        .unwrap_or_else(|| FieldType::Base {
            name: "string".to_string(),
            params: Vec::new(),
        });

    let mut default = None;
    let mut range = None;
    let mut can_schedule = None;
    let mut change_frequency = None;

    for opt in ctx.parameterOption_all() {
        let text = opt.get_text();
        if text.starts_with("default") {
            if let Some(lit) = opt.literal() {
                let raw = lit.get_text();
                default = Some(unquote(&raw));
            }
        } else if text.starts_with("range") {
            let nums: Vec<_> = opt.numberLiteral_all();
            if nums.len() >= 2 {
                range = Some((nums[0].get_text(), nums[1].get_text()));
            }
        } else if text.starts_with("can_schedule") {
            can_schedule = Some(text.contains("true"));
        } else if text.starts_with("change_frequency") {
            change_frequency = opt.IDENTIFIER().map(|id| terminal_text(&*id));
        }
    }

    ParameterDecl {
        name,
        field_type,
        default,
        range,
        can_schedule,
        change_frequency,
    }
}

// -- Entries --

fn build_entry_decl(ctx: &EntryDeclContext<'_>) -> EntryDecl {
    let name = ctx
        .IDENTIFIER()
        .map(|id| terminal_text(&*id))
        .unwrap_or_default();

    let fields: Vec<EntryField> = ctx
        .entryFieldV2_all()
        .iter()
        .map(|ef| {
            let key = ef
                .fieldName()
                .map(|fn_ctx| fn_ctx.get_text())
                .unwrap_or_else(|| {
                    // Handle "deprecated" keyword fields
                    let text = ef.get_text();
                    if text.starts_with("deprecated") {
                        "deprecated".to_string()
                    } else if text.starts_with("deprecated_reason") {
                        "deprecated_reason".to_string()
                    } else {
                        text
                    }
                });
            let value = ef
                .literal()
                .map(|lit| {
                    let raw = lit.get_text();
                    unquote(&raw)
                })
                .or_else(|| ef.BOOLEAN().map(|b| terminal_text(&*b)))
                .unwrap_or_default();
            EntryField { key, value }
        })
        .collect();

    EntryDecl { name, fields }
}

// -- Rules --

fn build_rule_block(ctx: &RuleBlockContext<'_>) -> RuleBlock {
    let name = ctx
        .IDENTIFIER()
        .map(|id| terminal_text(&*id))
        .unwrap_or_default();

    let given: Vec<RuleFieldDecl> = ctx
        .givenBlock()
        .map(|gb| {
            gb.ruleFieldDeclV2_all()
                .iter()
                .map(|rfd| {
                    let field_name = rfd
                        .fieldName()
                        .map(|fn_ctx| fn_ctx.get_text())
                        .unwrap_or_default();
                    let field_type = rfd
                        .fieldTypeV2()
                        .map(|ft| build_field_type(&*ft))
                        .unwrap_or_else(|| FieldType::Base {
                            name: "string".to_string(),
                            params: Vec::new(),
                        });
                    RuleFieldDecl {
                        name: field_name,
                        field_type,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let calculate: Vec<Calculation> = ctx
        .calculateBlock()
        .map(|cb| {
            cb.calculation_all()
                .iter()
                .map(|calc| {
                    let calc_name = calc
                        .fieldName()
                        .map(|fn_ctx| fn_ctx.get_text())
                        .unwrap_or_default();
                    let expression = calc
                        .expression()
                        .map(|e| e.get_text())
                        .unwrap_or_default();
                    Calculation {
                        name: calc_name,
                        expression,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let returns: Vec<RuleFieldDecl> = ctx
        .returnBlock()
        .map(|rb| {
            rb.ruleFieldDeclV2_all()
                .iter()
                .map(|rfd| {
                    let field_name = rfd
                        .fieldName()
                        .map(|fn_ctx| fn_ctx.get_text())
                        .unwrap_or_default();
                    let field_type = rfd
                        .fieldTypeV2()
                        .map(|ft| build_field_type(&*ft))
                        .unwrap_or_else(|| FieldType::Base {
                            name: "string".to_string(),
                            params: Vec::new(),
                        });
                    RuleFieldDecl {
                        name: field_name,
                        field_type,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    RuleBlock {
        name,
        given,
        calculate,
        returns,
    }
}

// -- Type aliases --

fn build_type_alias_block(ctx: &TypeAliasBlockContext<'_>) -> TypeAliasBlock {
    let aliases: Vec<TypeAlias> = ctx
        .typeAliasV2_all()
        .iter()
        .map(|ta| {
            let name = ta
                .aliasName()
                .map(|an| an.get_text())
                .unwrap_or_default();
            let field_type = ta.fieldTypeV2().map(|ft| build_field_type(&*ft));
            let fields: Vec<FieldDecl> = ta
                .fieldDeclV2_all()
                .iter()
                .map(|f| build_field_decl(f))
                .collect();
            TypeAlias {
                name,
                field_type,
                fields,
            }
        })
        .collect();

    TypeAliasBlock { aliases }
}
