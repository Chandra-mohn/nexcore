// NexCore -- Nexflow Codegen: Error Type Generation
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates an `ApiError` enum with `IntoResponse` impl from endpoint error mappings.

use std::collections::BTreeMap;
use std::fmt::Write;

use nexflow_parser::ast::api::Endpoint;
use nexflow_parser::ast::common::SchemaRef;

use crate::naming::schema_ref_to_type_name;

/// Collected error mapping: status code + schema type name.
struct ErrorVariant {
    status_code: i64,
    type_name: String,
}

/// Generate an `errors.rs` file with an `ApiError` enum.
pub fn generate_errors(endpoints: &[&Endpoint]) -> String {
    let mut out = String::with_capacity(2048);

    writeln!(out, "//! Generated error types from Nexflow ApiDSL.").unwrap();
    writeln!(out, "//! DO NOT EDIT -- regenerate from .api files.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use axum::http::StatusCode;").unwrap();
    writeln!(out, "use axum::response::{{IntoResponse, Response}};").unwrap();
    writeln!(out, "use axum::Json;").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use crate::models;").unwrap();
    writeln!(out).unwrap();

    // Collect unique error variants (deduplicate by type name)
    let mut variants: BTreeMap<String, ErrorVariant> = BTreeMap::new();
    for ep in endpoints {
        for err in &ep.errors {
            let type_name = schema_ref_name(&err.schema);
            variants.entry(type_name.clone()).or_insert(ErrorVariant {
                status_code: err.status_code,
                type_name,
            });
        }
    }

    if variants.is_empty() {
        writeln!(out, "// No error types defined in the API.").unwrap();
        return out;
    }

    // Generate enum
    writeln!(out, "#[derive(Debug)]").unwrap();
    writeln!(out, "pub enum ApiError {{").unwrap();
    for variant in variants.values() {
        writeln!(
            out,
            "    {v}(models::{v}),",
            v = variant.type_name
        )
        .unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // Generate Display impl
    writeln!(out, "impl std::fmt::Display for ApiError {{").unwrap();
    writeln!(
        out,
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
    )
    .unwrap();
    writeln!(out, "        match self {{").unwrap();
    for variant in variants.values() {
        writeln!(
            out,
            "            Self::{v}(_) => write!(f, \"{v}\"),",
            v = variant.type_name
        )
        .unwrap();
    }
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "impl std::error::Error for ApiError {{}}").unwrap();
    writeln!(out).unwrap();

    // Generate IntoResponse impl
    writeln!(out, "impl IntoResponse for ApiError {{").unwrap();
    writeln!(
        out,
        "    fn into_response(self) -> Response {{"
    )
    .unwrap();
    writeln!(out, "        match self {{").unwrap();
    for variant in variants.values() {
        let status = status_code_ident(variant.status_code);
        writeln!(
            out,
            "            Self::{v}(body) => (StatusCode::{status}, Json(body)).into_response(),",
            v = variant.type_name,
        )
        .unwrap();
    }
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();

    out
}

fn schema_ref_name(sr: &SchemaRef) -> String {
    schema_ref_to_type_name(&sr.name)
}

fn status_code_ident(code: i64) -> &'static str {
    match code {
        400 => "BAD_REQUEST",
        401 => "UNAUTHORIZED",
        403 => "FORBIDDEN",
        404 => "NOT_FOUND",
        409 => "CONFLICT",
        422 => "UNPROCESSABLE_ENTITY",
        429 => "TOO_MANY_REQUESTS",
        500 => "INTERNAL_SERVER_ERROR",
        502 => "BAD_GATEWAY",
        503 => "SERVICE_UNAVAILABLE",
        _ => "INTERNAL_SERVER_ERROR",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::api::{ErrorMapping, Endpoint};
    use nexflow_parser::ast::common::{HttpMethod, SchemaRef};

    fn make_endpoint(name: &str, errors: Vec<ErrorMapping>) -> Endpoint {
        Endpoint {
            name: name.to_string(),
            method: Some(HttpMethod::Get),
            path: Some("/test".to_string()),
            description: None,
            params: Vec::new(),
            query: Vec::new(),
            headers: Vec::new(),
            request: None,
            response: None,
            errors,
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
    fn test_error_generation() {
        let ep = make_endpoint("getAccount", vec![
            ErrorMapping {
                status_code: 404,
                schema: SchemaRef {
                    name: "AccountNotFound".to_string(),
                    qualifier: None,
                },
            },
            ErrorMapping {
                status_code: 403,
                schema: SchemaRef {
                    name: "InsufficientPermission".to_string(),
                    qualifier: None,
                },
            },
        ]);

        let output = generate_errors(&[&ep]);
        assert!(output.contains("pub enum ApiError {"));
        assert!(output.contains("AccountNotFound(models::AccountNotFound)"));
        assert!(output.contains("InsufficientPermission(models::InsufficientPermission)"));
        assert!(output.contains("StatusCode::NOT_FOUND"));
        assert!(output.contains("StatusCode::FORBIDDEN"));
    }
}
