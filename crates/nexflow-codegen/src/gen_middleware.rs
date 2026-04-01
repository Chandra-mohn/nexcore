// NexCore -- Nexflow Codegen: Middleware Generation
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates CORS configuration and auth extractor stubs.

use std::fmt::Write;

use nexflow_parser::ast::api::ApiDefinition;
use nexflow_parser::ast::common::ValueOrCfg;

/// Generate a `middleware.rs` file with CORS and auth stubs.
pub fn generate_middleware(api: &ApiDefinition) -> String {
    let mut out = String::with_capacity(2048);

    writeln!(out, "//! Generated middleware from Nexflow ApiDSL.").unwrap();
    writeln!(out, "//! DO NOT EDIT -- regenerate from .api files.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use axum::http::Method;").unwrap();
    writeln!(out, "use tower_http::cors::{{Any, CorsLayer}};").unwrap();
    writeln!(out).unwrap();

    generate_cors(&mut out, api);
    writeln!(out).unwrap();
    generate_auth_stub(&mut out, api);

    out
}

fn generate_cors(out: &mut String, api: &ApiDefinition) {
    writeln!(out, "pub fn cors_layer() -> CorsLayer {{").unwrap();

    if let Some(cors) = &api.cors {
        writeln!(out, "    CorsLayer::new()").unwrap();

        // Origins
        if cors.origins.is_empty() {
            writeln!(out, "        .allow_origin(Any)").unwrap();
        } else {
            let origins: Vec<String> = cors
                .origins
                .iter()
                .filter_map(|o| match o {
                    ValueOrCfg::String(s) => Some(format!("\"{s}\".parse().unwrap()")),
                    _ => None,
                })
                .collect();
            if origins.is_empty() {
                // cfg refs -- use Any as placeholder
                writeln!(out, "        .allow_origin(Any)").unwrap();
            } else {
                writeln!(out, "        .allow_origin([{}])", origins.join(", ")).unwrap();
            }
        }

        // Methods
        if !cors.methods.is_empty() {
            let methods: Vec<String> = cors
                .methods
                .iter()
                .map(|m| format!("Method::{}", format!("{m:?}").to_uppercase()))
                .collect();
            writeln!(
                out,
                "        .allow_methods([{}])",
                methods.join(", ")
            )
            .unwrap();
        }

        // Headers
        if !cors.headers.is_empty() {
            let headers: Vec<String> = cors
                .headers
                .iter()
                .map(|h| format!("\"{h}\".parse().unwrap()"))
                .collect();
            writeln!(
                out,
                "        .allow_headers([{}])",
                headers.join(", ")
            )
            .unwrap();
        }
    } else {
        writeln!(out, "    CorsLayer::permissive()").unwrap();
    }

    writeln!(out, "}}").unwrap();
}

fn generate_auth_stub(out: &mut String, api: &ApiDefinition) {
    let auth_scheme = api
        .auth_default
        .as_ref()
        .map(|a| a.scheme.as_str())
        .unwrap_or("bearer");

    writeln!(out, "/// Authentication claims extracted from the request.").unwrap();
    writeln!(out, "/// Scheme: {auth_scheme}").unwrap();
    writeln!(out, "#[derive(Debug, Clone)]").unwrap();
    writeln!(out, "pub struct AuthClaims {{").unwrap();
    writeln!(out, "    pub subject: String,").unwrap();
    writeln!(out, "    pub scopes: Vec<String>,").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "// TODO: Implement FromRequestParts for AuthClaims").unwrap();
    writeln!(out, "// to extract and validate auth tokens.").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::api::*;

    fn make_api_with_cors() -> ApiDefinition {
        ApiDefinition {
            imports: Vec::new(),
            name: "TestAPI".to_string(),
            version: None,
            base_path: None,
            description: None,
            targets: Vec::new(),
            auth_default: Some(AuthScheme {
                scheme: "bearer".to_string(),
                scope: Some("read".to_string()),
                header: None,
            }),
            content_type: None,
            rate_limit: None,
            pagination: None,
            config: Vec::new(),
            cors: Some(nexflow_parser::ast::api::CorsConfig {
                origins: vec![ValueOrCfg::String("https://example.com".to_string())],
                methods: vec![
                    nexflow_parser::ast::common::HttpMethod::Get,
                    nexflow_parser::ast::common::HttpMethod::Post,
                ],
                headers: vec!["Authorization".to_string()],
                max_age: None,
            }),
            endpoints: Vec::new(),
            events: Vec::new(),
            dependencies: Vec::new(),
            health_path: None,
            ready_path: None,
        }
    }

    #[test]
    fn test_middleware_generation() {
        let api = make_api_with_cors();
        let output = generate_middleware(&api);
        assert!(output.contains("pub fn cors_layer() -> CorsLayer {"));
        assert!(output.contains("https://example.com"));
        assert!(output.contains("Method::GET"));
        assert!(output.contains("pub struct AuthClaims {"));
    }
}
