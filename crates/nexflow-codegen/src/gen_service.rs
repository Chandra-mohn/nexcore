// NexCore -- Nexflow Codegen: Service Handler Generation
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates handler implementations from `ServiceDefinition` with orchestration
//! pipeline stubs (authorize, validate, lookup, transform, persist, call, publish).

use std::fmt::Write;

use nexflow_parser::ast::common::SchemaRef;
use nexflow_parser::ast::service::*;

use crate::naming::{endpoint_to_fn_name, schema_ref_to_type_name};

/// Generate a `service_impl.rs` file with handler implementations.
pub fn generate_service(service: &ServiceDefinition) -> String {
    let mut out = String::with_capacity(8192);

    writeln!(out, "//! Generated service implementation: {}", service.name).unwrap();
    writeln!(out, "//! From Nexflow ServiceDSL.").unwrap();
    writeln!(out, "//! DO NOT EDIT -- regenerate from .service files.").unwrap();
    writeln!(out).unwrap();

    // Imports
    writeln!(out, "use crate::errors::ApiError;").unwrap();
    writeln!(out, "use crate::models;").unwrap();
    writeln!(out).unwrap();

    // Service struct
    writeln!(out, "/// Service: {}", service.name).unwrap();
    if let Some(desc) = &service.description {
        writeln!(out, "/// {desc}").unwrap();
    }
    if !service.implements.is_empty() {
        writeln!(out, "/// Implements: {}", service.implements.join(", ")).unwrap();
    }
    if !service.consumes.is_empty() {
        writeln!(out, "/// Consumes: {}", service.consumes.join(", ")).unwrap();
    }
    writeln!(out, "#[derive(Debug, Clone)]").unwrap();
    writeln!(out, "pub struct {} {{", service.name).unwrap();
    writeln!(out, "    // TODO: Add database pools, service clients, config").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // Impl block with handlers
    writeln!(out, "impl {} {{", service.name).unwrap();

    for handler in &service.handlers {
        generate_handler(&mut out, handler);
    }

    writeln!(out, "}}").unwrap();

    // Config struct
    if !service.config.is_empty() {
        writeln!(out).unwrap();
        generate_config_struct(&mut out, service);
    }

    // Cache config comment
    if !service.cache_entries.is_empty() {
        writeln!(out).unwrap();
        generate_cache_comment(&mut out, service);
    }

    out
}

fn generate_handler(out: &mut String, handler: &Handler) {
    let fn_name = endpoint_to_fn_name(&handler.name);

    writeln!(out, "    /// Handler: {}", handler.name).unwrap();
    writeln!(out, "    pub async fn {fn_name}(&self) -> Result<(), ApiError> {{").unwrap();

    // Generate pipeline steps as comments + stub code
    for (i, stmt) in handler.statements.iter().enumerate() {
        generate_statement(out, stmt, 2, i);
    }

    writeln!(out, "        todo!(\"Implement {fn_name}\")").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();
}

fn generate_statement(out: &mut String, stmt: &HandlerStatement, indent: usize, _idx: usize) {
    let pad = "    ".repeat(indent);

    match stmt {
        HandlerStatement::Authorize { scope } => {
            writeln!(out, "{pad}// Step: authorize scope \"{scope}\"").unwrap();
            writeln!(out, "{pad}// auth.require_scope(\"{scope}\")?;").unwrap();
        }
        HandlerStatement::Validate { expression, using } => {
            let expr = format_expression(expression);
            writeln!(
                out,
                "{pad}// Step: validate {expr} using {}.{}",
                using.qualifier, using.name
            )
            .unwrap();
        }
        HandlerStatement::Lookup {
            target,
            from,
            where_clauses,
        } => {
            writeln!(out, "{pad}// Step: lookup {target} from {from}").unwrap();
            for wc in where_clauses {
                let left = format_expression(&wc.left);
                let right = format_expression(&wc.right);
                let op = format_comparator(&wc.comparator);
                writeln!(out, "{pad}//   where {left} {op} {right}").unwrap();
            }
            writeln!(out, "{pad}let {target} = todo!(\"lookup {target} from {from}\");").unwrap();
        }
        HandlerStatement::Transform {
            sources,
            using,
            into,
        } => {
            let source_names: Vec<String> =
                sources.iter().map(format_expression).collect();
            writeln!(
                out,
                "{pad}// Step: transform {} using {}.{} into {into}",
                source_names.join(", "),
                using.qualifier,
                using.name
            )
            .unwrap();
            writeln!(
                out,
                "{pad}let {into} = todo!(\"transform into {into}\");"
            )
            .unwrap();
        }
        HandlerStatement::Persist {
            target,
            assignment,
            to,
        } => {
            let target_str = format_expression(target);
            if let Some(assign) = assignment {
                let assign_str = format_expression(assign);
                writeln!(
                    out,
                    "{pad}// Step: persist {target_str} = {assign_str} to {to}"
                )
                .unwrap();
            } else {
                writeln!(out, "{pad}// Step: persist {target_str} to {to}").unwrap();
            }
        }
        HandlerStatement::Call {
            service_endpoint,
            with_args,
            for_each,
            into,
        } => {
            let call_target = format!("{}.{}", service_endpoint.qualifier, service_endpoint.name);
            let args: Vec<String> = with_args
                .iter()
                .map(|a| {
                    let val = format_expression(&a.value);
                    format!("{} = {val}", a.key)
                })
                .collect();
            let args_str = if args.is_empty() {
                String::new()
            } else {
                format!(" with {}", args.join(", "))
            };
            let for_each_str = for_each
                .as_ref()
                .map(|fe| {
                    let coll = format_expression(&fe.collection);
                    format!(" for each {} in {coll}", fe.variable)
                })
                .unwrap_or_default();
            let into_str = into
                .as_ref()
                .map(|i| format!(" into {i}"))
                .unwrap_or_default();
            writeln!(
                out,
                "{pad}// Step: call {call_target}{args_str}{for_each_str}{into_str}"
            )
            .unwrap();
        }
        HandlerStatement::Publish { event_name } => {
            writeln!(out, "{pad}// Step: publish {event_name}").unwrap();
        }
        HandlerStatement::Respond {
            status_code,
            schema,
        } => {
            let schema_str = schema
                .as_ref()
                .map(|s| format!(" {}", schema_ref_name(s)))
                .unwrap_or_default();
            writeln!(out, "{pad}// Step: respond {status_code}{schema_str}").unwrap();
        }
        HandlerStatement::OnError { cases } => {
            writeln!(out, "{pad}// Error handling:").unwrap();
            for case in cases {
                match case {
                    ErrorCase::WhenPredicate {
                        status_code,
                        predicate,
                    } => {
                        let pred_str = format_predicate(predicate);
                        writeln!(out, "{pad}//   {status_code} when {pred_str}").unwrap();
                    }
                    ErrorCase::Fallback { expression, to } => {
                        let expr = format_expression(expression);
                        writeln!(out, "{pad}//   fallback {expr} to {to}").unwrap();
                    }
                }
            }
        }
        HandlerStatement::Transaction {
            statements,
            on_rollback,
        } => {
            writeln!(out, "{pad}// -- transaction begin --").unwrap();
            for (i, s) in statements.iter().enumerate() {
                generate_statement(out, s, indent + 1, i);
            }
            if !on_rollback.is_empty() {
                writeln!(out, "{pad}// -- on_rollback --").unwrap();
                for (i, s) in on_rollback.iter().enumerate() {
                    generate_statement(out, s, indent + 1, i);
                }
            }
            writeln!(out, "{pad}// -- transaction end --").unwrap();
        }
        HandlerStatement::Saga { steps } => {
            writeln!(out, "{pad}// -- saga begin --").unwrap();
            for step in steps {
                writeln!(out, "{pad}// saga step: {}", step.name).unwrap();
                for (i, s) in step.statements.iter().enumerate() {
                    generate_statement(out, s, indent + 1, i);
                }
                if !step.compensate.is_empty() {
                    writeln!(out, "{pad}// compensate:").unwrap();
                    for (i, s) in step.compensate.iter().enumerate() {
                        generate_statement(out, s, indent + 1, i);
                    }
                }
            }
            writeln!(out, "{pad}// -- saga end --").unwrap();
        }
        HandlerStatement::Parallel { statements } => {
            writeln!(out, "{pad}// -- parallel begin --").unwrap();
            for (i, s) in statements.iter().enumerate() {
                generate_statement(out, s, indent + 1, i);
            }
            writeln!(out, "{pad}// -- parallel end --").unwrap();
        }
    }
}

fn generate_config_struct(out: &mut String, service: &ServiceDefinition) {
    writeln!(out, "/// Configuration for {}", service.name).unwrap();
    writeln!(out, "#[derive(Debug, Clone, serde::Deserialize)]").unwrap();
    writeln!(out, "pub struct {}Config {{", service.name).unwrap();
    for directive in &service.config {
        // Use first value's type to determine Rust type
        let rust_ty = if let Some(val) = directive.values.first() {
            match val {
                nexflow_parser::ast::common::ConfigValue::Integer(_) => "i64",
                nexflow_parser::ast::common::ConfigValue::Decimal(_) => "f64",
                nexflow_parser::ast::common::ConfigValue::Boolean(_) => "bool",
                _ => "&'static str",
            }
        } else {
            "&'static str"
        };
        writeln!(out, "    pub {}: {rust_ty},", directive.key).unwrap();
    }
    writeln!(out, "}}").unwrap();
}

fn generate_cache_comment(out: &mut String, service: &ServiceDefinition) {
    writeln!(out, "// Cache configuration:").unwrap();
    for entry in &service.cache_entries {
        let invalidate = if entry.invalidate_on.is_empty() {
            String::new()
        } else {
            format!(" invalidate_on: {}", entry.invalidate_on.join(", "))
        };
        writeln!(
            out,
            "//   {} -- ttl: {:?} {}{invalidate}",
            entry.handler_name, entry.ttl, entry.ttl_unit
        )
        .unwrap();
    }
}

fn format_expression(expr: &Expression) -> String {
    match expr {
        Expression::Path(parts) => parts.join("."),
        Expression::Literal(lit) => match lit {
            LiteralValue::Integer(n) => n.to_string(),
            LiteralValue::Decimal(n) => n.to_string(),
            LiteralValue::String(s) => format!("\"{s}\""),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Null => "null".to_string(),
        },
        Expression::QualifiedAnnotation { namespace, key } => {
            format!("{namespace}.{key}")
        }
    }
}

fn format_comparator(c: &Comparator) -> &'static str {
    match c {
        Comparator::Eq => "=",
        Comparator::Neq => "!=",
        Comparator::Lt => "<",
        Comparator::Gt => ">",
        Comparator::Lte => "<=",
        Comparator::Gte => ">=",
    }
}

fn format_predicate(pred: &Predicate) -> String {
    match pred {
        Predicate::IsNull(expr) => {
            let e = format_expression(expr);
            format!("{e} is null")
        }
        Predicate::Comparison {
            left,
            comparator,
            right,
        } => {
            let l = format_expression(left);
            let r = format_expression(right);
            let op = format_comparator(comparator);
            format!("{l} {op} {r}")
        }
        Predicate::Contains { expression, value } => {
            let e = format_expression(expression);
            format!("{e} contains {value}")
        }
    }
}

fn schema_ref_name(sr: &SchemaRef) -> String {
    schema_ref_to_type_name(&sr.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::common::QualifiedRef;

    fn make_simple_service() -> ServiceDefinition {
        ServiceDefinition {
            imports: Vec::new(),
            name: "TestService".to_string(),
            description: Some("A test service".to_string()),
            implements: vec!["TestAPI".to_string()],
            consumes: vec!["AuditAPI".to_string()],
            config: Vec::new(),
            handlers: vec![Handler {
                name: "getItem".to_string(),
                statements: vec![
                    HandlerStatement::Authorize {
                        scope: "items:read".to_string(),
                    },
                    HandlerStatement::Lookup {
                        target: "item".to_string(),
                        from: "ItemDB".to_string(),
                        where_clauses: vec![WhereClause {
                            left: Expression::Path(vec!["item_id".to_string()]),
                            comparator: Comparator::Eq,
                            right: Expression::Path(vec![
                                "request".to_string(),
                                "item_id".to_string(),
                            ]),
                        }],
                    },
                    HandlerStatement::OnError {
                        cases: vec![ErrorCase::WhenPredicate {
                            status_code: 404,
                            predicate: Predicate::IsNull(Expression::Path(vec![
                                "item".to_string(),
                            ])),
                        }],
                    },
                    HandlerStatement::Respond {
                        status_code: 200,
                        schema: None,
                    },
                ],
            }],
            cache_entries: Vec::new(),
            health: None,
            ready_path: None,
        }
    }

    #[test]
    fn test_service_generation() {
        let service = make_simple_service();
        let output = generate_service(&service);

        assert!(output.contains("pub struct TestService {"));
        assert!(output.contains("impl TestService {"));
        assert!(output.contains("pub async fn get_item"));
        assert!(output.contains("authorize scope \"items:read\""));
        assert!(output.contains("lookup item from ItemDB"));
        assert!(output.contains("where item_id = request.item_id"));
        assert!(output.contains("404 when item is null"));
        assert!(output.contains("respond 200"));
    }

    #[test]
    fn test_transaction_generation() {
        let service = ServiceDefinition {
            imports: Vec::new(),
            name: "TxService".to_string(),
            description: None,
            implements: Vec::new(),
            consumes: Vec::new(),
            config: Vec::new(),
            handlers: vec![Handler {
                name: "doTx".to_string(),
                statements: vec![HandlerStatement::Transaction {
                    statements: vec![HandlerStatement::Persist {
                        target: Expression::Path(vec!["data".to_string()]),
                        assignment: None,
                        to: "MyDB".to_string(),
                    }],
                    on_rollback: vec![HandlerStatement::Call {
                        service_endpoint: QualifiedRef {
                            qualifier: "AuditAPI".to_string(),
                            name: "logFailure".to_string(),
                        },
                        with_args: Vec::new(),
                        for_each: None,
                        into: None,
                    }],
                }],
            }],
            cache_entries: Vec::new(),
            health: None,
            ready_path: None,
        };

        let output = generate_service(&service);
        assert!(output.contains("-- transaction begin --"));
        assert!(output.contains("persist data to MyDB"));
        assert!(output.contains("-- on_rollback --"));
        assert!(output.contains("call AuditAPI.logFailure"));
        assert!(output.contains("-- transaction end --"));
    }
}
