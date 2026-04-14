// NexCore -- Nexflow Parser: ServiceDSL Builder
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Builds a typed `ServiceDefinition` AST from ServiceDSL source text.
//!
//! Entry point: `parse_service(source) -> Result<ServiceDefinition>`

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::InputStream;

use crate::ast::common::*;
use crate::ast::service::*;
use crate::generated::servicedsllexer::ServiceDSLLexer;
use crate::generated::servicedslparser::*;
use crate::parse::helpers::{svc_text as terminal_text, unquote};
use super::ParseError;

/// Parse a `.service` source string into a typed `ServiceDefinition`.
pub fn parse_service(source: &str) -> Result<ServiceDefinition, ParseError> {
    let input = InputStream::new(source);
    let lexer = ServiceDSLLexer::new(input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = ServiceDSLParser::new(token_stream);

    let tree = parser
        .compilationUnit()
        .map_err(|e| ParseError::grammar("ServiceDSL", format!("{e:?}")))?;

    build_compilation_unit(&*tree)
}

fn build_compilation_unit(
    ctx: &CompilationUnitContext<'_>,
) -> Result<ServiceDefinition, ParseError> {
    let imports: Vec<ImportPath> = ctx
        .importStatement_all()
        .iter()
        .filter_map(|s| {
            s.importPath().map(|p| ImportPath {
                raw: p.get_text(),
            })
        })
        .collect();

    let svc_ctx = ctx
        .serviceDefinition()
        .ok_or_else(|| ParseError::ast("ServiceDSL", "Missing service definition"))?;

    let mut def = build_service_definition(&*svc_ctx)?;
    def.imports = imports;
    Ok(def)
}

fn build_service_definition(
    ctx: &ServiceDefinitionContext<'_>,
) -> Result<ServiceDefinition, ParseError> {
    let name = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let mut def = ServiceDefinition {
        imports: Vec::new(),
        name,
        description: None,
        implements: Vec::new(),
        consumes: Vec::new(),
        config: Vec::new(),
        handlers: Vec::new(),
        cache_entries: Vec::new(),
        health: None,
        ready_path: None,
    };

    if let Some(body) = ctx.serviceBody() {
        for elem in body.serviceElement_all() {
            apply_service_element(&mut def, &*elem);
        }
    }

    Ok(def)
}

fn apply_service_element(def: &mut ServiceDefinition, ctx: &ServiceElementContext<'_>) {
    if let Some(d) = ctx.descriptionDecl() {
        def.description = d.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
    if let Some(i) = ctx.implementsDecl() {
        if let Some(list) = i.identifierList() {
            def.implements = list
                .IDENTIFIER_all()
                .iter()
                .map(|n| terminal_text(n))
                .filter(|s| s != ",")
                .collect();
        }
    }
    if let Some(c) = ctx.consumesDecl() {
        if let Some(list) = c.identifierList() {
            def.consumes = list
                .IDENTIFIER_all()
                .iter()
                .map(|n| terminal_text(n))
                .filter(|s| s != ",")
                .collect();
        }
    }
    if let Some(cfg) = ctx.configBlock() {
        def.config = build_config_block(&*cfg);
    }
    if let Some(h) = ctx.handlerDecl() {
        def.handlers.push(build_handler(&*h));
    }
    if let Some(c) = ctx.cacheBlock() {
        def.cache_entries = build_cache_block(&*c);
    }
    if let Some(h) = ctx.healthBlock() {
        def.health = Some(build_health_block(&*h));
    }
    if let Some(r) = ctx.readyDecl() {
        def.ready_path = r.STRING_LITERAL().map(|n| unquote(&terminal_text(&*n)));
    }
}

fn build_config_block(ctx: &ConfigBlockContext<'_>) -> Vec<ConfigDirective> {
    ctx.configDirective_all()
        .iter()
        .map(|d| {
            let key = d
                .IDENTIFIER()
                .map(|n| terminal_text(&*n))
                .unwrap_or_default();
            let values = d
                .configValue_all()
                .iter()
                .map(|v| build_config_value(v))
                .collect();
            ConfigDirective { key, values }
        })
        .collect()
}

fn build_config_value(ctx: &ConfigValueContext<'_>) -> ConfigValue {
    if let Some(n) = ctx.INTEGER() {
        ConfigValue::Integer(terminal_text(&*n).parse().unwrap_or(0))
    } else if let Some(n) = ctx.DECIMAL_LITERAL() {
        ConfigValue::Decimal(terminal_text(&*n).parse().unwrap_or(0.0))
    } else if let Some(n) = ctx.STRING_LITERAL() {
        ConfigValue::String(unquote(&terminal_text(&*n)))
    } else if ctx.TRUE().is_some() {
        ConfigValue::Boolean(true)
    } else if ctx.FALSE().is_some() {
        ConfigValue::Boolean(false)
    } else if let Some(n) = ctx.IDENTIFIER() {
        ConfigValue::Identifier(terminal_text(&*n))
    } else {
        ConfigValue::String(ctx.get_text())
    }
}

fn build_handler(ctx: &HandlerDeclContext<'_>) -> Handler {
    let name = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    let statements = ctx
        .handlerBody()
        .map(|b| {
            b.handlerStatement_all()
                .iter()
                .map(|s| build_handler_statement(s))
                .collect()
        })
        .unwrap_or_default();

    Handler { name, statements }
}

fn build_handler_statement(ctx: &HandlerStatementContext<'_>) -> HandlerStatement {
    if let Some(a) = ctx.authorizeStmt() {
        let scope = a
            .STRING_LITERAL()
            .map(|n| unquote(&terminal_text(&*n)))
            .unwrap_or_default();
        return HandlerStatement::Authorize { scope };
    }

    if let Some(v) = ctx.validateStmt() {
        let expression = v
            .expression()
            .map(|e| build_expression(&*e))
            .unwrap_or(Expression::Path(vec!["request".to_string()]));
        let using = v
            .qualifiedRef()
            .map(|q| build_qualified_ref(&*q))
            .unwrap_or_else(|| QualifiedRef {
                qualifier: String::new(),
                name: String::new(),
            });
        return HandlerStatement::Validate { expression, using };
    }

    if let Some(l) = ctx.lookupStmt() {
        return build_lookup(&*l);
    }

    if let Some(t) = ctx.transformStmt() {
        return build_transform(&*t);
    }

    if let Some(p) = ctx.persistStmt() {
        return build_persist(&*p);
    }

    if let Some(c) = ctx.callStmt() {
        return build_call(&*c);
    }

    if let Some(p) = ctx.publishStmt() {
        let event_name = p
            .IDENTIFIER()
            .map(|n| terminal_text(&*n))
            .unwrap_or_default();
        return HandlerStatement::Publish { event_name };
    }

    if let Some(r) = ctx.respondStmt() {
        let status_code = r
            .INTEGER()
            .map(|n| terminal_text(&*n).parse().unwrap_or(200))
            .unwrap_or(200);
        let schema = r.schemaRef().map(|s| build_schema_ref(&*s));
        return HandlerStatement::Respond {
            status_code,
            schema,
        };
    }

    if let Some(oe) = ctx.onErrorBlock() {
        return build_on_error(&*oe);
    }

    if let Some(tx) = ctx.transactionBlock() {
        return build_transaction(&*tx);
    }

    if let Some(s) = ctx.sagaBlock() {
        return build_saga(&*s);
    }

    if let Some(p) = ctx.parallelBlock() {
        let statements = p
            .handlerStatement_all()
            .iter()
            .map(|s| build_handler_statement(s))
            .collect();
        return HandlerStatement::Parallel { statements };
    }

    // Fallback -- should not reach here if grammar is correct
    HandlerStatement::Respond {
        status_code: 500,
        schema: None,
    }
}

fn build_lookup(ctx: &LookupStmtContext<'_>) -> HandlerStatement {
    let ids: Vec<_> = ctx.IDENTIFIER_all();
    let target = ids.first().map(|n| terminal_text(n)).unwrap_or_default();
    let from = ids.get(1).map(|n| terminal_text(n)).unwrap_or_default();

    let where_clauses = ctx
        .whereClause_all()
        .iter()
        .map(|w| {
            let exprs: Vec<_> = w.expression_all();
            let left = exprs
                .first()
                .map(|e| build_expression(e))
                .unwrap_or(Expression::Path(Vec::new()));
            let right = exprs
                .get(1)
                .map(|e| build_expression(e))
                .unwrap_or(Expression::Path(Vec::new()));
            let comparator = w
                .comparator()
                .map(|c| build_comparator(&*c))
                .unwrap_or(Comparator::Eq);
            WhereClause {
                left,
                comparator,
                right,
            }
        })
        .collect();

    HandlerStatement::Lookup {
        target,
        from,
        where_clauses,
    }
}

fn build_transform(ctx: &TransformStmtContext<'_>) -> HandlerStatement {
    let sources = ctx
        .expressionList()
        .map(|el| {
            el.expression_all()
                .iter()
                .map(|e| build_expression(e))
                .collect()
        })
        .unwrap_or_default();

    let using = ctx
        .qualifiedRef()
        .map(|q| build_qualified_ref(&*q))
        .unwrap_or_else(|| QualifiedRef {
            qualifier: String::new(),
            name: String::new(),
        });

    let into = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    HandlerStatement::Transform {
        sources,
        using,
        into,
    }
}

fn build_persist(ctx: &PersistStmtContext<'_>) -> HandlerStatement {
    let exprs: Vec<_> = ctx.expression_all();
    let target = exprs
        .first()
        .map(|e| build_expression(e))
        .unwrap_or(Expression::Path(Vec::new()));

    let assignment = if exprs.len() > 1 {
        exprs.get(1).map(|e| build_expression(e))
    } else {
        None
    };

    let to = ctx
        .IDENTIFIER()
        .map(|n| terminal_text(&*n))
        .unwrap_or_default();

    HandlerStatement::Persist {
        target,
        assignment,
        to,
    }
}

fn build_call(ctx: &CallStmtContext<'_>) -> HandlerStatement {
    let service_endpoint = ctx
        .qualifiedRef()
        .map(|q| build_qualified_ref(&*q))
        .unwrap_or_else(|| QualifiedRef {
            qualifier: String::new(),
            name: String::new(),
        });

    let with_args: Vec<WithArg> = ctx
        .withClause_all()
        .iter()
        .map(|w| {
            let key = w
                .IDENTIFIER()
                .map(|n| terminal_text(&*n))
                .unwrap_or_default();
            let value = if let Some(e) = w.expression() {
                build_expression(&*e)
            } else if let Some(s) = w.STRING_LITERAL() {
                Expression::Literal(LiteralValue::String(unquote(&terminal_text(&*s))))
            } else {
                Expression::Path(Vec::new())
            };
            WithArg { key, value }
        })
        .collect();

    let for_each = ctx.forEachClause().map(|fe| {
        let variable = fe
            .IDENTIFIER()
            .map(|n| terminal_text(&*n))
            .unwrap_or_default();
        let collection = fe
            .expression()
            .map(|e| build_expression(&*e))
            .unwrap_or(Expression::Path(Vec::new()));
        ForEachClause {
            variable,
            collection,
        }
    });

    let into = ctx.intoClause().and_then(|ic| {
        ic.IDENTIFIER().map(|n| terminal_text(&*n))
    });

    HandlerStatement::Call {
        service_endpoint,
        with_args,
        for_each,
        into,
    }
}

fn build_on_error(ctx: &OnErrorBlockContext<'_>) -> HandlerStatement {
    let cases = ctx
        .errorCase_all()
        .iter()
        .map(|ec| {
            if let Some(pred) = ec.predicate() {
                let status_code = ec
                    .INTEGER()
                    .map(|n| terminal_text(&*n).parse().unwrap_or(500))
                    .unwrap_or(500);
                ErrorCase::WhenPredicate {
                    status_code,
                    predicate: build_predicate(&*pred),
                }
            } else {
                // fallback case
                let expr = ec
                    .expression()
                    .map(|e| build_expression(&*e))
                    .unwrap_or(Expression::Path(Vec::new()));
                let to = ec
                    .IDENTIFIER()
                    .map(|n| terminal_text(&*n))
                    .unwrap_or_default();
                ErrorCase::Fallback {
                    expression: expr,
                    to,
                }
            }
        })
        .collect();

    HandlerStatement::OnError { cases }
}

fn build_predicate(ctx: &PredicateContext<'_>) -> Predicate {
    let exprs: Vec<_> = ctx.expression_all();

    if ctx.NULL().is_some() && ctx.IS().is_some() {
        let expr = exprs
            .first()
            .map(|e| build_expression(e))
            .unwrap_or(Expression::Path(Vec::new()));
        return Predicate::IsNull(expr);
    }

    if ctx.CONTAINS().is_some() {
        let expr = exprs
            .first()
            .map(|e| build_expression(e))
            .unwrap_or(Expression::Path(Vec::new()));
        let value = if ctx.NULL().is_some() {
            "null".to_string()
        } else {
            ctx.IDENTIFIER()
                .map(|n| terminal_text(&*n))
                .unwrap_or_default()
        };
        return Predicate::Contains {
            expression: expr,
            value,
        };
    }

    // Comparison
    let left = exprs
        .first()
        .map(|e| build_expression(e))
        .unwrap_or(Expression::Path(Vec::new()));
    let right = exprs
        .get(1)
        .map(|e| build_expression(e))
        .unwrap_or(Expression::Path(Vec::new()));
    let comparator = ctx
        .comparator()
        .map(|c| build_comparator(&*c))
        .unwrap_or(Comparator::Eq);

    Predicate::Comparison {
        left,
        comparator,
        right,
    }
}

fn build_transaction(ctx: &TransactionBlockContext<'_>) -> HandlerStatement {
    let statements = ctx
        .handlerStatement_all()
        .iter()
        .map(|s| build_handler_statement(s))
        .collect();

    let on_rollback = ctx
        .onRollbackBlock()
        .map(|rb| {
            rb.handlerStatement_all()
                .iter()
                .map(|s| build_handler_statement(s))
                .collect()
        })
        .unwrap_or_default();

    HandlerStatement::Transaction {
        statements,
        on_rollback,
    }
}

fn build_saga(ctx: &SagaBlockContext<'_>) -> HandlerStatement {
    let steps = ctx
        .sagaStep_all()
        .iter()
        .map(|s| {
            let name = s
                .IDENTIFIER()
                .map(|n| terminal_text(&*n))
                .unwrap_or_default();
            let statements = s
                .handlerStatement_all()
                .iter()
                .map(|st| build_handler_statement(st))
                .collect();
            let compensate = s
                .compensateBlock()
                .map(|cb| {
                    cb.handlerStatement_all()
                        .iter()
                        .map(|st| build_handler_statement(st))
                        .collect()
                })
                .unwrap_or_default();
            SagaStep {
                name,
                statements,
                compensate,
            }
        })
        .collect();

    HandlerStatement::Saga { steps }
}

fn build_cache_block(ctx: &CacheBlockContext<'_>) -> Vec<CacheEntry> {
    ctx.cacheEntry_all()
        .iter()
        .map(|e| {
            let ids: Vec<_> = e.IDENTIFIER_all();
            let handler_name = ids.first().map(|n| terminal_text(n)).unwrap_or_default();
            let ttl_unit = ids.get(1).map(|n| terminal_text(n)).unwrap_or_else(|| "seconds".to_string());

            let ttl = e
                .valueOrCfg()
                .map(|v| build_value_or_cfg(&*v))
                .unwrap_or(ValueOrCfg::Integer(0));

            let invalidate_on = e
                .identifierList()
                .map(|list| {
                    list.IDENTIFIER_all()
                        .iter()
                        .map(|n| terminal_text(n))
                        .filter(|s| s != ",")
                        .collect()
                })
                .unwrap_or_default();

            CacheEntry {
                handler_name,
                ttl,
                ttl_unit,
                invalidate_on,
            }
        })
        .collect()
}

fn build_health_block(ctx: &HealthBlockContext<'_>) -> HealthBlock {
    let path = ctx
        .STRING_LITERAL()
        .map(|n| unquote(&terminal_text(&*n)))
        .unwrap_or_default();

    let checks = ctx
        .healthCheck_all()
        .iter()
        .map(|c| {
            let words: Vec<_> = c.word_all();
            let kind = words.first().map(|w| w.get_text()).unwrap_or_default();
            let resource = words.get(1).map(|w| w.get_text()).unwrap_or_default();
            let timeout_unit = words.get(2).map(|w| w.get_text()).unwrap_or_else(|| "seconds".to_string());

            let timeout = c
                .valueOrCfg()
                .map(|v| build_value_or_cfg(&*v))
                .unwrap_or(ValueOrCfg::Integer(5));

            HealthCheck {
                kind,
                resource,
                timeout,
                timeout_unit,
            }
        })
        .collect();

    HealthBlock { path, checks }
}

// --- Shared builders ------------------------------------------------

fn build_expression(ctx: &ExpressionContext<'_>) -> Expression {
    if let Some(lit) = ctx.literal() {
        return build_literal_expr(&*lit);
    }

    if let Some(qa) = ctx.qualifiedAnnotation() {
        let ids: Vec<_> = qa.IDENTIFIER_all();
        let namespace = ids.first().map(|n| terminal_text(n)).unwrap_or_default();
        let key = ids.get(1).map(|n| terminal_text(n)).unwrap_or_default();
        return Expression::QualifiedAnnotation { namespace, key };
    }

    // Dotted path: IDENTIFIER (DOT word)*
    let mut parts = Vec::new();
    if let Some(id) = ctx.IDENTIFIER() {
        parts.push(terminal_text(&*id));
    }
    for w in ctx.word_all() {
        parts.push(w.get_text());
    }

    Expression::Path(parts)
}

fn build_literal_expr(ctx: &LiteralContext<'_>) -> Expression {
    if let Some(n) = ctx.INTEGER() {
        Expression::Literal(LiteralValue::Integer(
            terminal_text(&*n).parse().unwrap_or(0),
        ))
    } else if let Some(n) = ctx.DECIMAL_LITERAL() {
        Expression::Literal(LiteralValue::Decimal(
            terminal_text(&*n).parse().unwrap_or(0.0),
        ))
    } else if let Some(n) = ctx.STRING_LITERAL() {
        Expression::Literal(LiteralValue::String(unquote(&terminal_text(&*n))))
    } else if ctx.TRUE().is_some() {
        Expression::Literal(LiteralValue::Boolean(true))
    } else if ctx.FALSE().is_some() {
        Expression::Literal(LiteralValue::Boolean(false))
    } else if ctx.NULL().is_some() {
        Expression::Literal(LiteralValue::Null)
    } else {
        Expression::Path(vec![ctx.get_text()])
    }
}

fn build_value_or_cfg(ctx: &ValueOrCfgContext<'_>) -> ValueOrCfg {
    if let Some(n) = ctx.INTEGER() {
        ValueOrCfg::Integer(terminal_text(&*n).parse().unwrap_or(0))
    } else if let Some(n) = ctx.DECIMAL_LITERAL() {
        ValueOrCfg::Decimal(terminal_text(&*n).parse().unwrap_or(0.0))
    } else if let Some(qa) = ctx.qualifiedAnnotation() {
        let ids: Vec<_> = qa.IDENTIFIER_all();
        let namespace = ids.first().map(|n| terminal_text(n)).unwrap_or_default();
        let key = ids.get(1).map(|n| terminal_text(n)).unwrap_or_default();
        ValueOrCfg::CfgRef { namespace, key }
    } else {
        ValueOrCfg::String(ctx.get_text())
    }
}

fn build_schema_ref(ctx: &SchemaRefContext<'_>) -> SchemaRef {
    let ids: Vec<_> = ctx.IDENTIFIER_all();
    let name = ids
        .last()
        .map(|n| terminal_text(n))
        .unwrap_or_default();
    let qualifier = if ids.len() > 1 {
        ids.first().map(|n| terminal_text(n))
    } else {
        None
    };
    SchemaRef { name, qualifier }
}

fn build_qualified_ref(ctx: &QualifiedRefContext<'_>) -> QualifiedRef {
    let ids: Vec<_> = ctx.IDENTIFIER_all();
    let qualifier = ids.first().map(|n| terminal_text(n)).unwrap_or_default();
    let name = ids.get(1).map(|n| terminal_text(n)).unwrap_or_default();
    QualifiedRef { qualifier, name }
}

fn build_comparator(ctx: &ComparatorContext<'_>) -> Comparator {
    let text = ctx.get_text();
    match text.as_str() {
        "=" => Comparator::Eq,
        "!=" => Comparator::Neq,
        "<" => Comparator::Lt,
        ">" => Comparator::Gt,
        "<=" => Comparator::Lte,
        ">=" => Comparator::Gte,
        _ => Comparator::Eq,
    }
}
