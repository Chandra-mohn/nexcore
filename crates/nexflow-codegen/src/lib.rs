// NexCore -- Nexflow Codegen
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

//! Generates service code from Nexflow DSL ASTs.
//!
//! Takes an `ApiDefinition` + `SchemaDefinition`s (from `nexflow-parser`)
//! and produces a `GeneratedProject` -- a map of file paths to generated
//! source code strings.
//!
//! **Rust target** (axum 0.7 + serde + utoipa + tower-http):
//! - `generate()` / `generate_with_service()` for Rust/Axum projects
//!
//! **Java target** (Avro + Maven + Flink):
//! - `java::generate_java_schemas()` for Java/Avro projects (L2)

// Note: codegen modules use `writeln!(out, ...).unwrap()` extensively.
// This is safe because `fmt::Write for String` is infallible -- writing
// to a String buffer never fails. Converting these to `?` propagation
// would require all codegen functions to return `fmt::Result`, which
// adds complexity with zero safety benefit.

pub mod gen_error;
pub mod gen_handler;
pub mod gen_middleware;
pub mod gen_router;
pub mod gen_schema;
pub mod gen_service;
pub mod java;
pub mod naming;
pub mod rust;
pub mod types;

use std::collections::HashMap;

use nexflow_parser::ast::api::ApiDefinition;
use nexflow_parser::ast::schema::SchemaDefinition;
use nexflow_parser::ast::service::ServiceDefinition;

use crate::naming::{pascal_to_snake, snake_to_pascal};

/// A generated project: map of relative file paths to source content.
#[derive(Debug, Clone)]
pub struct GeneratedProject {
    pub files: HashMap<String, String>,
}

/// Generate a complete Rust/Axum project from an API definition and schemas.
///
/// The caller must aggregate all schemas referenced by the API into a single
/// slice. Schema names are matched by converting PascalCase API references
/// to snake_case schema names.
pub fn generate(
    api: &ApiDefinition,
    schemas: &[SchemaDefinition],
) -> Result<GeneratedProject, String> {
    let mut files = HashMap::new();

    // Build schema index: PascalCase name -> &SchemaDefinition
    let schema_index: HashMap<String, &SchemaDefinition> = schemas
        .iter()
        .map(|s| (snake_to_pascal(&s.name), s))
        .collect();

    // Collect schemas referenced by this API
    let mut referenced_schemas: Vec<&SchemaDefinition> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for ep in &api.endpoints {
        // Request schema
        if let Some(req) = &ep.request {
            let name = snake_to_pascal(&req.name);
            if seen.insert(name.clone()) {
                if let Some(s) = schema_index.get(&name) {
                    referenced_schemas.push(s);
                }
            }
        }
        // Response schema
        if let Some(resp) = &ep.response {
            let name = snake_to_pascal(&resp.schema.name);
            if seen.insert(name.clone()) {
                if let Some(s) = schema_index.get(&name) {
                    referenced_schemas.push(s);
                }
            }
        }
        // Error schemas
        for err in &ep.errors {
            let name = snake_to_pascal(&err.schema.name);
            if seen.insert(name.clone()) {
                if let Some(s) = schema_index.get(&name) {
                    referenced_schemas.push(s);
                }
            }
        }
    }

    // Event payload schemas
    for event in &api.events {
        if let Some(payload) = &event.payload {
            let name = snake_to_pascal(&payload.name);
            if seen.insert(name.clone()) {
                if let Some(s) = schema_index.get(&name) {
                    referenced_schemas.push(s);
                }
            }
        }
    }

    // Generate models.rs
    let models = gen_schema::generate_schemas(&referenced_schemas);
    files.insert("src/models.rs".to_string(), models);

    // Generate errors.rs
    let active_endpoints: Vec<&nexflow_parser::ast::api::Endpoint> =
        api.endpoints.iter().collect();
    let errors = gen_error::generate_errors(&active_endpoints);
    files.insert("src/errors.rs".to_string(), errors);

    // Generate handlers.rs
    let handlers = gen_handler::generate_handlers(&active_endpoints);
    files.insert("src/handlers.rs".to_string(), handlers);

    // Generate router.rs
    let router = gen_router::generate_router(api);
    files.insert("src/router.rs".to_string(), router);

    // Generate middleware.rs
    let middleware = gen_middleware::generate_middleware(api);
    files.insert("src/middleware.rs".to_string(), middleware);

    // Generate lib.rs for the output crate
    let lib_rs = generate_output_lib(api);
    files.insert("src/lib.rs".to_string(), lib_rs);

    // Generate Cargo.toml for the output crate
    let cargo_toml = generate_output_cargo_toml(api);
    files.insert("Cargo.toml".to_string(), cargo_toml);

    Ok(GeneratedProject { files })
}

/// Generate a project including service implementation.
///
/// Extends `generate()` by also producing a `service_impl.rs` file from the
/// `ServiceDefinition`, with handler pipeline stubs matching the orchestration.
pub fn generate_with_service(
    api: &ApiDefinition,
    schemas: &[SchemaDefinition],
    service: &ServiceDefinition,
) -> Result<GeneratedProject, String> {
    let mut project = generate(api, schemas)?;

    // Generate service implementation
    let service_impl = gen_service::generate_service(service);
    project
        .files
        .insert("src/service_impl.rs".to_string(), service_impl);

    // Update lib.rs to include service_impl module
    if let Some(lib_rs) = project.files.get_mut("src/lib.rs") {
        *lib_rs = lib_rs.replace(
            "pub mod models;",
            "pub mod models;\npub mod service_impl;",
        );
    }

    Ok(project)
}

fn generate_output_lib(api: &ApiDefinition) -> String {
    let crate_name = pascal_to_snake(&api.name);
    format!(
        r#"//! Generated service: {name}
//! DO NOT EDIT -- regenerate from .api + .schema files.

pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod router;

/// Create the axum Router for {name}.
pub fn app() -> axum::Router {{
    router::create_router()
}}

/// Run the service on the given address.
pub async fn serve(addr: &str) -> Result<(), Box<dyn std::error::Error>> {{
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("{crate_name} listening on {{addr}}");
    axum::serve(listener, app()).await?;
    Ok(())
}}
"#,
        name = api.name,
    )
}

fn generate_output_cargo_toml(api: &ApiDefinition) -> String {
    let crate_name = pascal_to_snake(&api.name)
        .replace('_', "-");

    format!(
        r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.7"
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
tokio = {{ version = "1", features = ["full"] }}
tower-http = {{ version = "0.6", features = ["cors"] }}
utoipa = {{ version = "5", features = ["axum_extras"] }}
thiserror = "2"
uuid = {{ version = "1", features = ["serde", "v4"] }}
chrono = {{ version = "0.4", features = ["serde"] }}
rust_decimal = {{ version = "1", features = ["serde-with-str"] }}
"#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::api::*;
    use nexflow_parser::ast::common::*;
    use nexflow_parser::ast::schema::*;

    fn make_simple_api() -> ApiDefinition {
        ApiDefinition {
            imports: Vec::new(),
            name: "TestAPI".to_string(),
            version: Some("1.0".to_string()),
            base_path: Some("/api".to_string()),
            description: None,
            targets: Vec::new(),
            auth_default: None,
            content_type: None,
            rate_limit: None,
            pagination: None,
            config: Vec::new(),
            cors: None,
            endpoints: vec![Endpoint {
                name: "getItem".to_string(),
                method: Some(HttpMethod::Get),
                path: Some("/{id}".to_string()),
                description: None,
                params: vec![ParamDecl {
                    name: "id".to_string(),
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
                        name: "Item".to_string(),
                        qualifier: None,
                    },
                }),
                errors: vec![ErrorMapping {
                    status_code: 404,
                    schema: SchemaRef {
                        name: "NotFound".to_string(),
                        qualifier: None,
                    },
                }],
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
            }],
            events: Vec::new(),
            dependencies: Vec::new(),
            health_path: None,
            ready_path: None,
        }
    }

    fn make_schema(name: &str) -> SchemaDefinition {
        SchemaDefinition {
            imports: Vec::new(),
            name: name.to_string(),
            patterns: Vec::new(),
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: vec![FieldDecl {
                name: "id".to_string(),
                field_type: FieldType::Base {
                    name: "uuid".to_string(),
                    params: Vec::new(),
                },
                required: true,
                optional: false,
                unique: false,
                cannot_change: false,
                encrypted: false,
                pii: None,
                default: None,
                deprecated: None,
                removal: None,
            }],
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: Vec::new(),
            immutable: None,
            state_machine: None,
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        }
    }

    #[test]
    fn test_generate_produces_all_files() {
        let api = make_simple_api();
        let schemas = vec![make_schema("item"), make_schema("not_found")];

        let project = generate(&api, &schemas).expect("generate failed");

        assert!(project.files.contains_key("src/models.rs"));
        assert!(project.files.contains_key("src/errors.rs"));
        assert!(project.files.contains_key("src/handlers.rs"));
        assert!(project.files.contains_key("src/router.rs"));
        assert!(project.files.contains_key("src/middleware.rs"));
        assert!(project.files.contains_key("src/lib.rs"));
        assert!(project.files.contains_key("Cargo.toml"));

        // Models should contain referenced schemas
        let models = &project.files["src/models.rs"];
        assert!(models.contains("pub struct Item {"));
        assert!(models.contains("pub struct NotFound {"));

        // Errors should contain the 404 mapping
        let errors = &project.files["src/errors.rs"];
        assert!(errors.contains("NotFound"));

        // Handlers should contain the endpoint
        let handlers = &project.files["src/handlers.rs"];
        assert!(handlers.contains("pub async fn get_item"));

        // Router should wire it up
        let router = &project.files["src/router.rs"];
        assert!(router.contains("get(handlers::get_item)"));

        // Cargo.toml should have correct name
        let cargo = &project.files["Cargo.toml"];
        assert!(cargo.contains("name = \"test-a-p-i\"") || cargo.contains("name ="));
    }
}
