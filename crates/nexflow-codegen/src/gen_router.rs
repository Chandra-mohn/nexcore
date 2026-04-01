// NexCore -- Nexflow Codegen: Router Generation
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates an axum Router from API endpoints.

use std::collections::BTreeMap;
use std::fmt::Write;

use nexflow_parser::ast::api::{ApiDefinition, Endpoint};
use nexflow_parser::ast::common::HttpMethod;

use crate::naming::endpoint_to_fn_name;

/// Generate a `router.rs` file with a `create_router()` function.
pub fn generate_router(api: &ApiDefinition) -> String {
    let mut out = String::with_capacity(2048);

    writeln!(out, "//! Generated router from Nexflow ApiDSL.").unwrap();
    writeln!(out, "//! DO NOT EDIT -- regenerate from .api files.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use axum::Router;").unwrap();
    writeln!(out, "use axum::routing::{{get, post, put, patch, delete}};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use crate::handlers;").unwrap();
    writeln!(out, "use crate::middleware;").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "pub fn create_router() -> Router {{").unwrap();
    writeln!(out, "    Router::new()").unwrap();

    // Group endpoints by path to chain methods
    let mut routes: BTreeMap<String, Vec<&Endpoint>> = BTreeMap::new();
    for ep in &api.endpoints {
        if ep.is_deprecated {
            continue;
        }
        let path = ep.path.as_deref().unwrap_or("/").to_string();
        routes.entry(path).or_default().push(ep);
    }

    for (path, endpoints) in &routes {
        let mut methods = Vec::new();
        for ep in endpoints {
            let method = ep
                .method
                .as_ref()
                .unwrap_or(&HttpMethod::Get);
            let axum_method = match method {
                HttpMethod::Get => "get",
                HttpMethod::Post => "post",
                HttpMethod::Put => "put",
                HttpMethod::Patch => "patch",
                HttpMethod::Delete => "delete",
            };
            let fn_name = endpoint_to_fn_name(&ep.name);
            methods.push(format!("{axum_method}(handlers::{fn_name})"));
        }

        let chained = methods.join(".");
        writeln!(out, "        .route(\"{path}\", {chained})").unwrap();
    }

    // Add middleware layers
    writeln!(out, "        .layer(middleware::cors_layer())").unwrap();
    writeln!(out, "}}").unwrap();

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::common::HttpMethod;

    fn make_api() -> ApiDefinition {
        ApiDefinition {
            imports: Vec::new(),
            name: "TestAPI".to_string(),
            version: None,
            base_path: Some("/api/v1".to_string()),
            description: None,
            targets: Vec::new(),
            auth_default: None,
            content_type: None,
            rate_limit: None,
            pagination: None,
            config: Vec::new(),
            cors: None,
            endpoints: vec![
                Endpoint {
                    name: "listItems".to_string(),
                    method: Some(HttpMethod::Get),
                    path: Some("/".to_string()),
                    description: None,
                    params: Vec::new(),
                    query: Vec::new(),
                    headers: Vec::new(),
                    request: None,
                    response: None,
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
                },
                Endpoint {
                    name: "createItem".to_string(),
                    method: Some(HttpMethod::Post),
                    path: Some("/".to_string()),
                    description: None,
                    params: Vec::new(),
                    query: Vec::new(),
                    headers: Vec::new(),
                    request: None,
                    response: None,
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
                },
            ],
            events: Vec::new(),
            dependencies: Vec::new(),
            health_path: None,
            ready_path: None,
        }
    }

    #[test]
    fn test_router_generation() {
        let api = make_api();
        let output = generate_router(&api);
        assert!(output.contains("pub fn create_router() -> Router {"));
        assert!(output.contains("get(handlers::list_items)"));
        assert!(output.contains("post(handlers::create_item)"));
        // Same path should be chained
        assert!(output.contains(".route(\"/\","));
    }
}
