// NexCore -- Nexflow CLI: Output Formatting
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

//! Output formatters for parse command: json, tree, summary, graph.
//! Matches Python CLI output formats for downstream compatibility.

use std::fmt::Write;

use nexflow_parser::ast::api::ApiDefinition;
use nexflow_parser::ast::nexquery::NexQueryScript;
use nexflow_parser::ast::proc::ProcProgram;
use nexflow_parser::ast::rules::RulesProgram;
use nexflow_parser::ast::schema::SchemaDefinition;
use nexflow_parser::ast::service::ServiceDefinition;
use nexflow_parser::ast::transform::TransformProgram;

#[allow(unused_imports)]
use nexflow_compiler::Project;

// ---------------------------------------------------------------------------
// JSON format (matches Python's ast_to_json with _type fields)
// ---------------------------------------------------------------------------

/// Format project ASTs as JSON (pretty or compact).
pub fn format_json(project: &Project, pretty: bool) -> Result<String, String> {
    let mut output = serde_json::Map::new();

    if !project.apis.is_empty() {
        let v = serde_json::to_value(&project.apis)
            .map_err(|e| format!("JSON error: {e}"))?;
        output.insert("apis".to_string(), v);
    }
    if !project.services.is_empty() {
        let v = serde_json::to_value(&project.services)
            .map_err(|e| format!("JSON error: {e}"))?;
        output.insert("services".to_string(), v);
    }
    if !project.schemas.is_empty() {
        let v = serde_json::to_value(&project.schemas)
            .map_err(|e| format!("JSON error: {e}"))?;
        output.insert("schemas".to_string(), v);
    }
    if !project.transforms.is_empty() {
        let v = serde_json::to_value(&project.transforms)
            .map_err(|e| format!("JSON error: {e}"))?;
        output.insert("transforms".to_string(), v);
    }
    if !project.rules.is_empty() {
        let v = serde_json::to_value(&project.rules)
            .map_err(|e| format!("JSON error: {e}"))?;
        output.insert("rules".to_string(), v);
    }
    if !project.procs.is_empty() {
        let v = serde_json::to_value(&project.procs)
            .map_err(|e| format!("JSON error: {e}"))?;
        output.insert("procs".to_string(), v);
    }
    if !project.queries.is_empty() {
        let v = serde_json::to_value(&project.queries)
            .map_err(|e| format!("JSON error: {e}"))?;
        output.insert("queries".to_string(), v);
    }

    let val = serde_json::Value::Object(output);
    if pretty {
        serde_json::to_string_pretty(&val).map_err(|e| format!("JSON error: {e}"))
    } else {
        serde_json::to_string(&val).map_err(|e| format!("JSON error: {e}"))
    }
}

// ---------------------------------------------------------------------------
// Summary format (default) -- matches Python's ast_to_summary
// ---------------------------------------------------------------------------

/// Format project ASTs as human-readable summary.
pub fn format_summary(project: &Project, file_name: &str) -> String {
    let mut out = String::new();

    // Determine file type from what was parsed
    if !project.apis.is_empty() {
        for api in &project.apis {
            format_api_summary(&mut out, api, file_name);
        }
    }
    if !project.services.is_empty() {
        for svc in &project.services {
            format_service_summary(&mut out, svc, file_name);
        }
    }
    if !project.schemas.is_empty() {
        format_schemas_summary(&mut out, &project.schemas, file_name);
    }
    if !project.transforms.is_empty() {
        for xform in &project.transforms {
            format_transform_summary(&mut out, xform, file_name);
        }
    }
    if !project.rules.is_empty() {
        for rules in &project.rules {
            format_rules_summary(&mut out, rules, file_name);
        }
    }
    if !project.procs.is_empty() {
        for proc_prog in &project.procs {
            format_proc_summary(&mut out, proc_prog, file_name);
        }
    }
    if !project.queries.is_empty() {
        for query in &project.queries {
            format_query_summary(&mut out, query, file_name);
        }
    }

    out
}

fn format_api_summary(out: &mut String, api: &ApiDefinition, file_name: &str) {
    writeln!(out, "File: {file_name}").unwrap();
    writeln!(out, "Type: ApiDefinition").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Name: {}", api.name).unwrap();
    if let Some(v) = &api.version {
        writeln!(out, "Version: {v}").unwrap();
    }
    if let Some(bp) = &api.base_path {
        writeln!(out, "Base Path: {bp}").unwrap();
    }
    if !api.targets.is_empty() {
        writeln!(out, "Targets: {}", api.targets.join(", ")).unwrap();
    }

    let active = api.endpoints.iter().filter(|e| !e.is_deprecated).count();
    let deprecated = api.endpoints.iter().filter(|e| e.is_deprecated).count();
    writeln!(out, "Endpoints: {active}").unwrap();
    for ep in &api.endpoints {
        if ep.is_deprecated {
            continue;
        }
        let method = ep.method.as_ref().map(|m| format!("{m:?}")).unwrap_or_default();
        let path = ep.path.as_deref().unwrap_or("");
        writeln!(out, "  - {} {} {}", ep.name, method, path).unwrap();
    }
    if deprecated > 0 {
        writeln!(out, "Deprecated Endpoints: {deprecated}").unwrap();
        for ep in api.endpoints.iter().filter(|e| e.is_deprecated) {
            writeln!(out, "  - {}", ep.name).unwrap();
        }
    }

    if !api.events.is_empty() {
        writeln!(out, "Events: {}", api.events.len()).unwrap();
        for ev in &api.events {
            writeln!(out, "  - {}", ev.name).unwrap();
        }
    }

    if !api.dependencies.is_empty() {
        writeln!(out, "Dependencies: {}", api.dependencies.len()).unwrap();
        for dep in &api.dependencies {
            writeln!(out, "  - {} {}", dep.api_name, dep.version).unwrap();
        }
    }
}

fn format_service_summary(out: &mut String, svc: &ServiceDefinition, file_name: &str) {
    writeln!(out, "File: {file_name}").unwrap();
    writeln!(out, "Type: ServiceDefinition").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Name: {}", svc.name).unwrap();
    if !svc.implements.is_empty() {
        writeln!(out, "Implements: {}", svc.implements.join(", ")).unwrap();
    }
    if !svc.consumes.is_empty() {
        writeln!(out, "Consumes: {}", svc.consumes.join(", ")).unwrap();
    }
    writeln!(out, "Handlers: {}", svc.handlers.len()).unwrap();
    for h in &svc.handlers {
        writeln!(out, "  - {} ({} steps)", h.name, h.statements.len()).unwrap();
    }
    if !svc.cache_entries.is_empty() {
        writeln!(out, "Cache Entries: {}", svc.cache_entries.len()).unwrap();
    }
}

fn format_schemas_summary(out: &mut String, schemas: &[SchemaDefinition], file_name: &str) {
    writeln!(out, "File: {file_name}").unwrap();
    writeln!(out, "Type: SchemaProgram").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Schemas: {}", schemas.len()).unwrap();
    for s in schemas {
        let patterns: Vec<String> = s.patterns.iter().map(|p| format!("{p:?}")).collect();
        let pattern_str = if patterns.is_empty() {
            String::new()
        } else {
            format!(" ({})", patterns.join(", "))
        };
        writeln!(out, "  - {}{} [{} fields]", s.name, pattern_str, s.fields.len()).unwrap();
    }
}

fn format_transform_summary(out: &mut String, xform: &TransformProgram, file_name: &str) {
    writeln!(out, "File: {file_name}").unwrap();
    writeln!(out, "Type: TransformProgram").unwrap();
    writeln!(out).unwrap();
    if !xform.transforms.is_empty() {
        writeln!(out, "Transforms: {}", xform.transforms.len()).unwrap();
        for t in &xform.transforms {
            let pure_str = if t.pure == Some(true) { " (pure)" } else { "" };
            writeln!(out, "  - {}{pure_str}", t.name).unwrap();
        }
    }
    if !xform.transform_blocks.is_empty() {
        writeln!(out, "Transform Blocks: {}", xform.transform_blocks.len()).unwrap();
        for tb in &xform.transform_blocks {
            writeln!(out, "  - {} [{} mappings]", tb.name, tb.mappings.len()).unwrap();
        }
    }
}

fn format_rules_summary(out: &mut String, rules: &RulesProgram, file_name: &str) {
    writeln!(out, "File: {file_name}").unwrap();
    writeln!(out, "Type: RulesProgram").unwrap();
    writeln!(out).unwrap();
    if !rules.decision_tables.is_empty() {
        writeln!(out, "Decision Tables: {}", rules.decision_tables.len()).unwrap();
        for dt in &rules.decision_tables {
            let policy = dt
                .hit_policy
                .map(|hp| format!(" ({hp:?})"))
                .unwrap_or_default();
            writeln!(out, "  - {}{policy} [{} rows]", dt.name, dt.rows.len()).unwrap();
        }
    }
    if !rules.procedural_rules.is_empty() {
        writeln!(out, "Procedural Rules: {}", rules.procedural_rules.len()).unwrap();
        for pr in &rules.procedural_rules {
            writeln!(out, "  - {}", pr.name).unwrap();
        }
    }
}

fn format_proc_summary(out: &mut String, proc_prog: &ProcProgram, file_name: &str) {
    writeln!(out, "File: {file_name}").unwrap();
    writeln!(out, "Type: ProcProgram").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Processes: {}", proc_prog.processes.len()).unwrap();
    for p in &proc_prog.processes {
        writeln!(out, "  - {} [{} statements]", p.name, p.body.len()).unwrap();
    }
}

fn format_query_summary(out: &mut String, query: &NexQueryScript, file_name: &str) {
    writeln!(out, "File: {file_name}").unwrap();
    writeln!(out, "Type: NexQueryScript").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Statements: {}", query.statements.len()).unwrap();
    for (i, stmt) in query.statements.iter().enumerate() {
        writeln!(out, "  - Statement {} [{} clauses]", i + 1, stmt.clauses.len()).unwrap();
    }
}

// ---------------------------------------------------------------------------
// Tree format -- matches Python's ast_to_tree
// ---------------------------------------------------------------------------

/// Format project ASTs as indented tree view.
pub fn format_tree(project: &Project) -> String {
    let mut out = String::new();

    for api in &project.apis {
        format_api_tree(&mut out, api, 0);
    }
    for svc in &project.services {
        format_service_tree(&mut out, svc, 0);
    }
    for schema in &project.schemas {
        format_schema_tree(&mut out, schema, 0);
    }

    out
}

fn indent(out: &mut String, level: usize) {
    for _ in 0..level {
        out.push_str("  ");
    }
}

fn format_api_tree(out: &mut String, api: &ApiDefinition, level: usize) {
    indent(out, level);
    writeln!(out, "ApiDefinition").unwrap();

    indent(out, level + 1);
    writeln!(out, "name: {}", api.name).unwrap();

    if let Some(v) = &api.version {
        indent(out, level + 1);
        writeln!(out, "version: {v}").unwrap();
    }
    if let Some(bp) = &api.base_path {
        indent(out, level + 1);
        writeln!(out, "base_path: {bp}").unwrap();
    }

    if !api.endpoints.is_empty() {
        indent(out, level + 1);
        writeln!(out, "endpoints:").unwrap();
        for ep in &api.endpoints {
            indent(out, level + 2);
            writeln!(out, "Endpoint").unwrap();
            indent(out, level + 3);
            writeln!(out, "name: {}", ep.name).unwrap();
            if let Some(m) = &ep.method {
                indent(out, level + 3);
                writeln!(out, "method: {m:?}").unwrap();
            }
            if let Some(p) = &ep.path {
                indent(out, level + 3);
                writeln!(out, "path: {p}").unwrap();
            }
            if !ep.params.is_empty() {
                indent(out, level + 3);
                writeln!(out, "params: {}", ep.params.len()).unwrap();
            }
            if !ep.errors.is_empty() {
                indent(out, level + 3);
                writeln!(out, "errors: {}", ep.errors.len()).unwrap();
            }
        }
    }

    if !api.events.is_empty() {
        indent(out, level + 1);
        writeln!(out, "events:").unwrap();
        for ev in &api.events {
            indent(out, level + 2);
            writeln!(out, "Event").unwrap();
            indent(out, level + 3);
            writeln!(out, "name: {}", ev.name).unwrap();
            if let Some(t) = &ev.topic {
                indent(out, level + 3);
                writeln!(out, "topic: {t}").unwrap();
            }
        }
    }
}

fn format_service_tree(out: &mut String, svc: &ServiceDefinition, level: usize) {
    indent(out, level);
    writeln!(out, "ServiceDefinition").unwrap();

    indent(out, level + 1);
    writeln!(out, "name: {}", svc.name).unwrap();

    if !svc.implements.is_empty() {
        indent(out, level + 1);
        writeln!(out, "implements: {}", svc.implements.join(", ")).unwrap();
    }

    if !svc.handlers.is_empty() {
        indent(out, level + 1);
        writeln!(out, "handlers:").unwrap();
        for h in &svc.handlers {
            indent(out, level + 2);
            writeln!(out, "Handler").unwrap();
            indent(out, level + 3);
            writeln!(out, "name: {}", h.name).unwrap();
            indent(out, level + 3);
            writeln!(out, "statements: {}", h.statements.len()).unwrap();
        }
    }
}

fn format_schema_tree(out: &mut String, schema: &SchemaDefinition, level: usize) {
    indent(out, level);
    writeln!(out, "SchemaDefinition").unwrap();

    indent(out, level + 1);
    writeln!(out, "name: {}", schema.name).unwrap();

    if !schema.patterns.is_empty() {
        indent(out, level + 1);
        let patterns: Vec<String> = schema.patterns.iter().map(|p| format!("{p:?}")).collect();
        writeln!(out, "patterns: {}", patterns.join(", ")).unwrap();
    }

    if !schema.fields.is_empty() {
        indent(out, level + 1);
        writeln!(out, "fields:").unwrap();
        for f in &schema.fields {
            indent(out, level + 2);
            let type_str = format!("{:?}", f.field_type);
            // Truncate long type representations
            let type_display = if type_str.len() > 40 {
                format!("{}...", &type_str[..37])
            } else {
                type_str
            };
            let req = if f.required { " required" } else { "" };
            writeln!(out, "{}: {}{}", f.name, type_display, req).unwrap();
        }
    }

    if !schema.constraints.is_empty() {
        indent(out, level + 1);
        writeln!(out, "constraints: {}", schema.constraints.len()).unwrap();
    }

    if !schema.identity.is_empty() {
        indent(out, level + 1);
        writeln!(out, "identity: {}", schema.identity.len()).unwrap();
    }

    if schema.streaming.is_some() {
        indent(out, level + 1);
        writeln!(out, "streaming: present").unwrap();
    }

    if !schema.entries.is_empty() {
        indent(out, level + 1);
        writeln!(out, "entries: {}", schema.entries.len()).unwrap();
    }
}

// ---------------------------------------------------------------------------
// Validate JSON output -- matches Python's validate JSON format
// ---------------------------------------------------------------------------

/// Format validation result as JSON (matches Python format).
pub fn format_validate_json(
    valid: bool,
    errors: &[ValidateError],
    warnings: &[String],
    files_validated: usize,
    validation_time_ms: u64,
) -> Result<String, String> {
    let mut result = serde_json::Map::new();

    result.insert("valid".to_string(), serde_json::Value::Bool(valid));
    result.insert(
        "version".to_string(),
        serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()),
    );

    let errors_json: Vec<serde_json::Value> = errors
        .iter()
        .map(|e| {
            let mut m = serde_json::Map::new();
            if let Some(f) = &e.file {
                m.insert("file".to_string(), serde_json::Value::String(f.clone()));
            }
            if let Some(line) = e.line {
                m.insert("line".to_string(), serde_json::json!(line));
            }
            if let Some(col) = e.column {
                m.insert("column".to_string(), serde_json::json!(col));
            }
            m.insert(
                "message".to_string(),
                serde_json::Value::String(e.message.clone()),
            );
            m.insert(
                "severity".to_string(),
                serde_json::Value::String(e.severity.clone()),
            );
            serde_json::Value::Object(m)
        })
        .collect();

    let warnings_json: Vec<serde_json::Value> = warnings
        .iter()
        .map(|w| {
            let mut m = serde_json::Map::new();
            m.insert(
                "message".to_string(),
                serde_json::Value::String(w.clone()),
            );
            m.insert(
                "severity".to_string(),
                serde_json::Value::String("warning".to_string()),
            );
            serde_json::Value::Object(m)
        })
        .collect();

    result.insert("errors".to_string(), serde_json::Value::Array(errors_json));
    result.insert(
        "warnings".to_string(),
        serde_json::Value::Array(warnings_json),
    );

    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "validation_time_ms".to_string(),
        serde_json::json!(validation_time_ms),
    );
    metadata.insert(
        "files_validated".to_string(),
        serde_json::json!(files_validated),
    );
    result.insert("metadata".to_string(), serde_json::Value::Object(metadata));

    serde_json::to_string_pretty(&serde_json::Value::Object(result))
        .map_err(|e| format!("JSON error: {e}"))
}

/// Validation error with location info.
pub struct ValidateError {
    pub file: Option<String>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub message: String,
    pub severity: String,
}
