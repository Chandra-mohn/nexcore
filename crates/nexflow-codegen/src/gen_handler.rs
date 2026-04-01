// NexCore -- Nexflow Codegen: Handler Stub Generation
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates async handler function stubs with axum extractors.

use std::fmt::Write;

use nexflow_parser::ast::api::Endpoint;
use nexflow_parser::ast::common::HttpMethod;

use crate::naming::{endpoint_to_fn_name, schema_ref_to_type_name, snake_to_pascal};

/// Generate a `handlers.rs` file with handler stubs for the given endpoints.
pub fn generate_handlers(endpoints: &[&Endpoint]) -> String {
    let mut out = String::with_capacity(4096);

    writeln!(out, "//! Generated handler stubs from Nexflow ApiDSL.").unwrap();
    writeln!(out, "//! DO NOT EDIT -- regenerate from .api files.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use axum::extract::{{Json, Path, Query}};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use crate::errors::ApiError;").unwrap();
    writeln!(out, "use crate::models;").unwrap();
    writeln!(out).unwrap();

    // Collect query structs needed
    let mut query_structs = Vec::new();

    for ep in endpoints {
        if ep.is_deprecated {
            continue;
        }
        generate_handler(&mut out, ep, &mut query_structs);
    }

    // Prepend query structs if any
    if !query_structs.is_empty() {
        let mut prefix = String::new();
        for qs in &query_structs {
            prefix.push_str(qs);
            prefix.push('\n');
        }
        out.insert_str(
            out.find("use crate::errors").unwrap_or(0),
            &prefix,
        );
    }

    out
}

fn generate_handler(out: &mut String, ep: &Endpoint, query_structs: &mut Vec<String>) {
    let fn_name = endpoint_to_fn_name(&ep.name);
    let method = ep
        .method
        .as_ref()
        .map(|m| format!("{m:?}").to_lowercase())
        .unwrap_or_else(|| "get".to_string());

    let path = ep.path.as_deref().unwrap_or("/");

    // Build utoipa path annotation
    let mut utoipa_responses = String::new();
    if let Some(resp) = &ep.response {
        let type_name = schema_ref_to_type_name(&resp.schema.name);
        write!(utoipa_responses, "(status = 200, body = models::{type_name})").unwrap();
    }
    for err in &ep.errors {
        let type_name = schema_ref_to_type_name(&err.schema.name);
        if !utoipa_responses.is_empty() {
            utoipa_responses.push_str(", ");
        }
        write!(
            utoipa_responses,
            "(status = {}, body = models::{type_name})",
            err.status_code
        )
        .unwrap();
    }

    writeln!(
        out,
        "#[utoipa::path({method}, path = \"{path}\", responses({utoipa_responses}))]"
    )
    .unwrap();

    // Build function parameters
    let mut params = Vec::new();

    // Path parameters
    if !ep.params.is_empty() {
        if ep.params.len() == 1 {
            let p = &ep.params[0];
            params.push(format!("Path({name}): Path<String>", name = p.name));
        } else {
            let tuple: Vec<String> = ep.params.iter().map(|p| p.name.clone()).collect();
            let types: Vec<&str> = ep.params.iter().map(|_| "String").collect();
            params.push(format!(
                "Path(({})): Path<({})>",
                tuple.join(", "),
                types.join(", ")
            ));
        }
    }

    // Query parameters
    if !ep.query.is_empty() {
        let query_struct_name = format!("{}Query", snake_to_pascal(&ep.name));
        let mut qs = String::new();
        writeln!(
            qs,
            "#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]"
        )
        .unwrap();
        writeln!(qs, "pub struct {query_struct_name} {{").unwrap();
        for q in &ep.query {
            let rust_ty = if q.type_ref.is_empty() {
                "String".to_string()
            } else {
                crate::naming::schema_ref_to_type_name(&q.type_ref)
            };
            if q.optional || !q.required {
                writeln!(qs, "    pub {}: Option<{rust_ty}>,", q.name).unwrap();
            } else {
                writeln!(qs, "    pub {}: {rust_ty},", q.name).unwrap();
            }
        }
        writeln!(qs, "}}").unwrap();
        query_structs.push(qs);

        params.push(format!("Query(query): Query<{query_struct_name}>"));
    }

    // Request body
    if let Some(req) = &ep.request {
        let type_name = schema_ref_to_type_name(&req.name);
        params.push(format!("Json(body): Json<models::{type_name}>"));
    }

    // Return type
    let return_type = if let Some(resp) = &ep.response {
        let type_name = schema_ref_to_type_name(&resp.schema.name);
        let is_list = resp.modifier.is_some();
        if is_list {
            format!("Result<Json<Vec<models::{type_name}>>, ApiError>")
        } else {
            format!("Result<Json<models::{type_name}>, ApiError>")
        }
    } else {
        "Result<(), ApiError>".to_string()
    };

    let params_str = params.join(",\n    ");
    if params.is_empty() {
        writeln!(out, "pub async fn {fn_name}() -> {return_type} {{").unwrap();
    } else {
        writeln!(out, "pub async fn {fn_name}(").unwrap();
        writeln!(out, "    {params_str},").unwrap();
        writeln!(out, ") -> {return_type} {{").unwrap();
    }

    // Allow unused variables in stubs
    for p in &ep.params {
        writeln!(out, "    let _ = {name};", name = p.name).unwrap();
    }
    if !ep.query.is_empty() {
        writeln!(out, "    let _ = query;").unwrap();
    }
    if ep.request.is_some() {
        writeln!(out, "    let _ = body;").unwrap();
    }

    writeln!(out, "    todo!()").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();
}

/// Map an HTTP method to the axum method filter name.
pub fn http_method_to_axum(method: &HttpMethod) -> &'static str {
    match method {
        HttpMethod::Get => "get",
        HttpMethod::Post => "post",
        HttpMethod::Put => "put",
        HttpMethod::Patch => "patch",
        HttpMethod::Delete => "delete",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::api::{Endpoint, ParamDecl, ResponseDecl};
    use nexflow_parser::ast::common::{HttpMethod, SchemaRef};

    fn make_endpoint() -> Endpoint {
        Endpoint {
            name: "getAccount".to_string(),
            method: Some(HttpMethod::Get),
            path: Some("/{account_id}".to_string()),
            description: None,
            params: vec![ParamDecl {
                name: "account_id".to_string(),
                type_ref: "string".to_string(),
                required: true,
                optional: false,
                default: None,
            }],
            query: Vec::new(),
            headers: Vec::new(),
            request: None,
            response: Some(ResponseDecl {
                modifier: None,
                schema: SchemaRef {
                    name: "AccountDetail".to_string(),
                    qualifier: None,
                },
            }),
            errors: Vec::new(),
            auth: None,
            validate: None,
            rate_limit: None,
            timeout: None,
            cache: None,
            idempotent_key: None,
            is_async: false,
            is_deprecated: false,
            sunset: None,
            replacement: None,
        }
    }

    #[test]
    fn test_handler_generation() {
        let ep = make_endpoint();
        let output = generate_handlers(&[&ep]);
        assert!(output.contains("pub async fn get_account("));
        assert!(output.contains("Path(account_id): Path<String>"));
        assert!(output.contains("Result<Json<models::AccountDetail>, ApiError>"));
        assert!(output.contains("todo!()"));
    }
}
