// NexCore -- Nexflow Parser: ProcDSL Builder
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Builds a typed `ProcProgram` AST from ProcDSL source text.
//!
//! Uses ANTLR4 context accessors for top-level structure and key processing
//! blocks. Sub-rule details use `get_text()` where the generated parser's
//! 207 context types make direct accessor usage impractical.

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::InputStream;

use crate::ast::common::*;
use crate::ast::proc::*;
use crate::generated::procdsllexer::ProcDSLLexer;
use crate::generated::procdslparser::*;
use crate::parse::helpers::{proc_text as terminal_text, unquote};

/// Parse a `.proc` source string into a typed `ProcProgram`.
pub fn parse_proc(source: &str) -> Result<ProcProgram, String> {
    let input = InputStream::new(source);
    let lexer = ProcDSLLexer::new(input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = ProcDSLParser::new(token_stream);

    let tree = parser
        .program()
        .map_err(|e| format!("Parse error: {e:?}"))?;

    build_program(&*tree)
}

fn build_program(ctx: &ProgramContext<'_>) -> Result<ProcProgram, String> {
    let imports: Vec<ImportPath> = ctx
        .importStatement_all()
        .iter()
        .filter_map(|s| {
            s.importPath().map(|p| ImportPath {
                raw: p.get_text(),
            })
        })
        .collect();

    let processes: Vec<ProcessDef> = ctx
        .processDefinition_all()
        .iter()
        .map(|pd| build_process_def(pd))
        .collect();

    Ok(ProcProgram { imports, processes })
}

fn build_process_def(ctx: &ProcessDefinitionContext<'_>) -> ProcessDef {
    let name = ctx
        .processName()
        .map(|pn| pn.get_text())
        .unwrap_or_default();

    // Execution block
    let execution = ctx.executionBlock().map(|eb| build_execution_block(&*eb));

    // Business/processing dates
    let business_date = ctx.businessDateDecl().map(|bd| bd.get_text());
    let processing_date = ctx.processingDateDecl().map(|pd| pd.get_text());

    // Markers
    let markers: Vec<MarkerDecl> = ctx
        .markersBlock()
        .map(|mb| {
            mb.markerDef_all()
                .iter()
                .map(|md| {
                    let marker_name = md.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default();
                    let raw = md.get_text();
                    MarkerDecl {
                        name: marker_name,
                        raw,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    // State machine
    let state_machine = ctx.stateMachineDecl().map(|smd| smd.get_text());

    // Body content
    let mut body = Vec::new();
    let mut phases = Vec::new();

    if let Some(pbop) = ctx.processBodyOrPhases() {
        // Extract body content
        for bc in pbop.bodyContent_all() {
            if let Some(rd) = bc.receiveDecl() {
                body.push(build_receive_decl(&*rd));
            } else if let Some(ed) = bc.emitDecl() {
                body.push(build_emit_decl(&*ed));
            } else if let Some(pb) = bc.processingBlock() {
                if let Some(stmt) = build_processing_block(&*pb) {
                    body.push(stmt);
                }
            } else if bc.correlationBlock().is_some() {
                body.push(ProcessStatement::Correlation {
                    raw: bc.get_text(),
                });
            } else if bc.completionBlock().is_some() {
                body.push(ProcessStatement::Completion {
                    raw: bc.get_text(),
                });
            }
        }

        // Phase blocks
        for phase in pbop.phaseBlock_all() {
            phases.push(build_phase_block(&*phase));
        }
    }

    // State block
    let state = ctx.stateBlock().map(|sb| StateBlock {
        raw: sb.get_text(),
    });

    // Tail blocks (metrics + resilience)
    let (metrics, resilience) = ctx
        .processTailBlocks()
        .map(|ptb| {
            let m = ptb.metricsBlock().map(|mb| MetricsBlock {
                raw: mb.get_text(),
            });
            let r = ptb.resilienceBlock().map(|rb| ResilienceBlock {
                raw: rb.get_text(),
            });
            (m, r)
        })
        .unwrap_or((None, None));

    ProcessDef {
        name,
        execution,
        business_date,
        processing_date,
        markers,
        state_machine,
        body,
        phases,
        state,
        metrics,
        resilience,
    }
}

// -- Execution block --

fn build_execution_block(ctx: &ExecutionBlockContext<'_>) -> ExecutionBlock {
    let parallelism = ctx
        .parallelismDecl()
        .and_then(|pd| pd.INTEGER())
        .and_then(|n| terminal_text(&*n).parse().ok());

    let mode = ctx
        .modeDecl()
        .and_then(|md| md.modeType())
        .map(|mt| mt.get_text());

    let checkpoint_interval = None; // From checkpointDecl if present

    ExecutionBlock {
        mode,
        parallelism,
        checkpoint_interval,
    }
}

// -- Receive --

fn build_receive_decl(ctx: &ReceiveDeclContext<'_>) -> ProcessStatement {
    // Grammar: RECEIVE IDENTIFIER (FROM IDENTIFIER)? INTO IDENTIFIER receiveClause*
    let ids = ctx.IDENTIFIER_all();

    // First IDENTIFIER is the stream name
    let name = ids.first().map(|id| terminal_text(id)).unwrap_or_default();

    // If FROM is present, second IDENTIFIER is source, third is target
    // Otherwise, second is target
    let has_from = ctx.FROM().is_some();
    let (source_ref, _target) = if has_from && ids.len() >= 3 {
        (
            Some(terminal_text(&ids[1])),
            ids.get(2).map(|id| terminal_text(id)).unwrap_or_default(),
        )
    } else {
        (
            None,
            ids.get(1).map(|id| terminal_text(id)).unwrap_or_default(),
        )
    };

    // Parse receive clauses for connector, schema, etc.
    let mut source_type = "kafka".to_string();
    let mut source = String::new();
    let mut schema = None;
    let mut key = None;
    let mut options = Vec::new();

    for clause in ctx.receiveClause_all() {
        if let Some(sd) = clause.schemaDecl() {
            schema = Some(sd.get_text().replace("schema", "").trim().to_string());
        }
        if let Some(cc) = clause.connectorClause() {
            let cc_text = cc.get_text();
            // Connector type
            if let Some(ct) = cc.connectorType() {
                source_type = ct.get_text();
            }
            // Extract quoted string from connector config
            if let Some(cfg) = cc.connectorConfig() {
                let cfg_text = cfg.get_text();
                if let Some(quoted) = extract_quoted(&cfg_text) {
                    source = quoted;
                }
                // Extract key from connector options
                if cfg_text.contains("key") {
                    let after_key = cfg_text
                        .split("key")
                        .nth(1)
                        .unwrap_or("")
                        .trim();
                    let key_val = after_key
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .next()
                        .unwrap_or("");
                    if !key_val.is_empty() {
                        key = Some(key_val.to_string());
                    }
                }
            }
        }
    }

    let _ = source_ref; // Used for the FROM reference if needed
    let _ = &options;

    ProcessStatement::Receive {
        name,
        source_type,
        source,
        schema,
        key,
        options,
    }
}

// -- Emit --

fn build_emit_decl(ctx: &EmitDeclContext<'_>) -> ProcessStatement {
    // Grammar: EMIT IDENTIFIER TO sinkName emitClause*
    let name = ctx.IDENTIFIER().map(|id| terminal_text(&*id)).unwrap_or_default();

    let sink_name = ctx
        .sinkName()
        .map(|sn| sn.get_text())
        .unwrap_or_default();

    let mut sink_type = "kafka".to_string();
    let mut sink = String::new();
    let mut schema = None;
    let mut key = None;

    for clause in ctx.emitClause_all() {
        if let Some(sd) = clause.schemaDecl() {
            schema = Some(sd.get_text().replace("schema", "").trim().to_string());
        }
        if let Some(cc) = clause.connectorClause() {
            if let Some(ct) = cc.connectorType() {
                sink_type = ct.get_text();
            }
            if let Some(cfg) = cc.connectorConfig() {
                let cfg_text = cfg.get_text();
                if let Some(quoted) = extract_quoted(&cfg_text) {
                    sink = quoted;
                }
                if cfg_text.contains("key") {
                    let after_key = cfg_text
                        .split("key")
                        .nth(1)
                        .unwrap_or("")
                        .trim();
                    let key_val = after_key
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .next()
                        .unwrap_or("");
                    if !key_val.is_empty() {
                        key = Some(key_val.to_string());
                    }
                }
            }
        }
    }

    let _ = sink_name;

    ProcessStatement::Emit {
        name,
        sink_type,
        sink,
        schema,
        key,
    }
}

// -- Processing blocks --

fn build_processing_block(ctx: &ProcessingBlockContext<'_>) -> Option<ProcessStatement> {
    if let Some(ed) = ctx.enrichDecl() {
        return Some(build_enrich_decl(&*ed));
    }
    if let Some(td) = ctx.transformDecl() {
        return Some(build_transform_decl(&*td));
    }
    if let Some(rd) = ctx.routeDecl() {
        return Some(build_route_decl(&*rd));
    }
    if let Some(ad) = ctx.aggregateDecl() {
        return Some(build_aggregate_decl(&*ad));
    }
    if let Some(wd) = ctx.windowDecl() {
        return Some(build_window_decl(&*wd));
    }
    if let Some(jd) = ctx.joinDecl() {
        return Some(build_join_decl(&*jd));
    }
    if let Some(md) = ctx.mergeDecl() {
        return Some(ProcessStatement::Join {
            left: String::new(),
            right: String::new(),
            join_type: "merge".to_string(),
            on: md.get_text(),
            within: None,
        });
    }
    if let Some(es) = ctx.evaluateStatement() {
        return Some(ProcessStatement::Evaluate {
            raw: es.get_text(),
        });
    }
    if let Some(bs) = ctx.branchStatement() {
        return Some(ProcessStatement::Branch {
            raw: bs.get_text(),
        });
    }

    // Other statement types captured as raw text
    Some(ProcessStatement::Evaluate {
        raw: ctx.get_text(),
    })
}

fn build_enrich_decl(ctx: &EnrichDeclContext<'_>) -> ProcessStatement {
    // Grammar: ENRICH USING IDENTIFIER ON fieldList selectClause?
    let target = ctx
        .IDENTIFIER()
        .map(|id| terminal_text(&*id))
        .unwrap_or_default();

    // Enrich doesn't have individual lookups in the grammar -- it references
    // an enrichment definition by name. The field list is the ON clause.
    ProcessStatement::Enrich {
        target,
        lookups: Vec::new(),
    }
}

fn build_transform_decl(ctx: &TransformDeclContext<'_>) -> ProcessStatement {
    let ids = ctx.IDENTIFIER_all();
    let input = ids.first().map(|id| terminal_text(id)).unwrap_or_default();
    let using = ids.get(1).map(|id| terminal_text(id)).unwrap_or_default();
    let into = ids.get(2).map(|id| terminal_text(id)).unwrap_or_default();

    ProcessStatement::Transform {
        input,
        using,
        into,
    }
}

fn build_route_decl(ctx: &RouteDeclContext<'_>) -> ProcessStatement {
    // Grammar: ROUTE (USING routeSource | WHEN expression) routeDestination* otherwiseClause?
    let input = ctx
        .routeSource()
        .map(|rs| rs.get_text())
        .or_else(|| ctx.expression().map(|e| e.get_text()))
        .unwrap_or_default();

    let mut routes: Vec<RouteCase> = ctx
        .routeDestination_all()
        .iter()
        .map(|rd| {
            let text = rd.get_text();
            // Parse "value" to target or direct to target
            // Extract target as the identifier after "to"
            let target = text
                .rsplit("to")
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            let condition = if text.contains("to") {
                let before_to = text.split("to").next().unwrap_or("").trim();
                if before_to.is_empty() {
                    None
                } else {
                    Some(unquote(before_to))
                }
            } else {
                None
            };
            RouteCase {
                condition,
                target,
                is_otherwise: false,
            }
        })
        .collect();

    // Otherwise clause
    if let Some(oc) = ctx.otherwiseClause() {
        let target = oc.get_text()
            .replace("otherwise", "")
            .replace("to", "")
            .trim()
            .to_string();
        routes.push(RouteCase {
            condition: None,
            target,
            is_otherwise: true,
        });
    }

    ProcessStatement::Route { input, routes }
}

fn build_aggregate_decl(ctx: &AggregateDeclContext<'_>) -> ProcessStatement {
    // Grammar: AGGREGATE (USING IDENTIFIER | IDENTIFIER) aggregateOptions?
    let name = ctx
        .IDENTIFIER()
        .map(|id| terminal_text(&*id))
        .unwrap_or_default();

    // Aggregate is a reference to an aggregation definition, not inline fields
    ProcessStatement::Aggregate {
        name,
        fields: Vec::new(),
    }
}

fn build_window_decl(ctx: &WindowDeclContext<'_>) -> ProcessStatement {
    let window_type = ctx
        .windowType()
        .map(|wt| wt.get_text())
        .unwrap_or_else(|| "tumbling".to_string());

    let size = ctx
        .duration()
        .map(|d| d.get_text())
        .unwrap_or_default();

    // Extract group_by from window body's key by clause
    let group_by: Vec<String> = ctx
        .windowBody()
        .and_then(|wb| wb.keyByClause())
        .map(|kbc| {
            // KEY BY fieldPath -- extract from text
            let text = kbc.get_text();
            let field = text.replace("keyby", "").replace("key", "").replace("by", "").trim().to_string();
            if field.is_empty() { Vec::new() } else { vec![field] }
        })
        .unwrap_or_default();

    ProcessStatement::Window {
        window_type,
        size,
        group_by,
        options: Vec::new(),
    }
}

fn build_join_decl(ctx: &JoinDeclContext<'_>) -> ProcessStatement {
    let ids = ctx.IDENTIFIER_all();
    let left = ids.first().map(|id| terminal_text(id)).unwrap_or_default();
    let right = ids.get(1).map(|id| terminal_text(id)).unwrap_or_default();

    let join_type = ctx
        .joinType()
        .map(|jt| jt.get_text())
        .unwrap_or_else(|| "inner".to_string());

    let on = ctx
        .fieldList()
        .map(|fl| fl.get_text())
        .unwrap_or_default();

    let within = ctx.duration().map(|d| d.get_text());

    ProcessStatement::Join {
        left,
        right,
        join_type,
        on,
        within,
    }
}

// -- Phase blocks --

fn build_phase_block(ctx: &PhaseBlockContext<'_>) -> PhaseBlock {
    let phase_name = ctx
        .phaseSpec()
        .map(|ps| ps.get_text())
        .unwrap_or_default();

    let mut statements = Vec::new();
    for bc in ctx.bodyContent_all() {
        if let Some(rd) = bc.receiveDecl() {
            statements.push(build_receive_decl(&*rd));
        } else if let Some(ed) = bc.emitDecl() {
            statements.push(build_emit_decl(&*ed));
        } else if let Some(pb) = bc.processingBlock() {
            if let Some(stmt) = build_processing_block(&*pb) {
                statements.push(stmt);
            }
        }
    }

    PhaseBlock {
        name: phase_name,
        statements,
    }
}

// -- Helpers --

fn extract_quoted(text: &str) -> Option<String> {
    let start = text.find('"')?;
    let rest = &text[start + 1..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}
