// NexCore -- Nexflow Compiler: Cross-Grammar Validation
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Validates consistency across `.api`, `.service`, and `.schema` files.

use std::collections::HashSet;

use nexflow_parser::ast::api::ApiDefinition;
use nexflow_parser::ast::schema::SchemaDefinition;
use nexflow_parser::ast::service::ServiceDefinition;

use crate::diagnostics::{DiagnosticSource, ValidationResult};

/// Input to the validator: parsed ASTs from all three grammar types.
pub struct ValidationInput<'a> {
    pub apis: &'a [ApiDefinition],
    pub services: &'a [ServiceDefinition],
    pub schemas: &'a [SchemaDefinition],
}

/// Run all validation checks and return diagnostics.
pub fn validate(input: &ValidationInput<'_>) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Build schema name index (both snake_case and PascalCase)
    let schema_names: HashSet<String> = input
        .schemas
        .iter()
        .flat_map(|s| {
            let snake = s.name.clone();
            let pascal = snake_to_pascal(&snake);
            vec![snake, pascal]
        })
        .collect();

    // Validate each API
    for api in input.apis {
        validate_api_schema_refs(api, &schema_names, &mut result);
    }

    // Validate each service
    for service in input.services {
        validate_service_api_refs(service, input.apis, &mut result);
        validate_service_handler_coverage(service, input.apis, &mut result);
    }

    // Validate schema internal consistency
    for schema in input.schemas {
        validate_schema_types(schema, &schema_names, &mut result);
    }

    // Cross-grammar: API-Service pairing
    for api in input.apis {
        let implementing_services: Vec<&ServiceDefinition> = input
            .services
            .iter()
            .filter(|s| s.implements.contains(&api.name))
            .collect();

        if implementing_services.is_empty() {
            result.warning(
                DiagnosticSource::CrossGrammar,
                format!("API '{}' has no implementing service", api.name),
                Some(api.name.clone()),
            );
        }
    }

    result
}

/// Check that all schema references in API endpoints resolve to known schemas.
fn validate_api_schema_refs(
    api: &ApiDefinition,
    schema_names: &HashSet<String>,
    result: &mut ValidationResult,
) {
    for ep in &api.endpoints {
        // Request schema
        if let Some(req) = &ep.request {
            if !schema_names.contains(&req.name) {
                result.error(
                    DiagnosticSource::Api,
                    format!(
                        "Endpoint '{}' references unknown request schema '{}'",
                        ep.name, req.name
                    ),
                    Some(format!("{}.{}", api.name, ep.name)),
                );
            }
        }

        // Response schema
        if let Some(resp) = &ep.response {
            if !schema_names.contains(&resp.schema.name) {
                result.error(
                    DiagnosticSource::Api,
                    format!(
                        "Endpoint '{}' references unknown response schema '{}'",
                        ep.name, resp.schema.name
                    ),
                    Some(format!("{}.{}", api.name, ep.name)),
                );
            }
        }

        // Error schemas
        for err in &ep.errors {
            if !schema_names.contains(&err.schema.name) {
                result.error(
                    DiagnosticSource::Api,
                    format!(
                        "Endpoint '{}' references unknown error schema '{}'",
                        ep.name, err.schema.name
                    ),
                    Some(format!("{}.{}", api.name, ep.name)),
                );
            }
        }
    }

    // Event payload schemas
    for event in &api.events {
        if let Some(payload) = &event.payload {
            if !schema_names.contains(&payload.name) {
                result.error(
                    DiagnosticSource::Api,
                    format!(
                        "Event '{}' references unknown payload schema '{}'",
                        event.name, payload.name
                    ),
                    Some(format!("{}.{}", api.name, event.name)),
                );
            }
        }
    }
}

/// Check that service `implements` references match known API names.
fn validate_service_api_refs(
    service: &ServiceDefinition,
    apis: &[ApiDefinition],
    result: &mut ValidationResult,
) {
    let api_names: HashSet<String> = apis.iter().map(|a| a.name.clone()).collect();

    for impl_name in &service.implements {
        if !api_names.contains(impl_name) {
            result.error(
                DiagnosticSource::Service,
                format!(
                    "Service '{}' implements unknown API '{impl_name}'",
                    service.name
                ),
                Some(service.name.clone()),
            );
        }
    }

    for consume_name in &service.consumes {
        if !api_names.contains(consume_name) {
            result.warning(
                DiagnosticSource::Service,
                format!(
                    "Service '{}' consumes unknown API '{consume_name}' (may be external)",
                    service.name
                ),
                Some(service.name.clone()),
            );
        }
    }
}

/// Check that service handlers cover all non-deprecated API endpoints.
fn validate_service_handler_coverage(
    service: &ServiceDefinition,
    apis: &[ApiDefinition],
    result: &mut ValidationResult,
) {
    let handler_names: HashSet<&str> = service
        .handlers
        .iter()
        .map(|h| h.name.as_str())
        .collect();

    for impl_name in &service.implements {
        if let Some(api) = apis.iter().find(|a| &a.name == impl_name) {
            for ep in &api.endpoints {
                if ep.is_deprecated {
                    continue;
                }
                if !handler_names.contains(ep.name.as_str()) {
                    result.error(
                        DiagnosticSource::CrossGrammar,
                        format!(
                            "Service '{}' is missing handler for endpoint '{}'",
                            service.name, ep.name
                        ),
                        Some(format!("{} -> {}", service.name, api.name)),
                    );
                }
            }

            // Warn about handlers that don't correspond to any endpoint
            let endpoint_names: HashSet<&str> =
                api.endpoints.iter().map(|e| e.name.as_str()).collect();
            for handler in &service.handlers {
                if !endpoint_names.contains(handler.name.as_str()) {
                    result.warning(
                        DiagnosticSource::CrossGrammar,
                        format!(
                            "Handler '{}' in service '{}' does not match any endpoint in '{}'",
                            handler.name, service.name, api.name
                        ),
                        Some(format!("{} -> {}", service.name, api.name)),
                    );
                }
            }
        }
    }
}

/// Check schema field types reference valid base types or other schemas.
fn validate_schema_types(
    schema: &SchemaDefinition,
    schema_names: &HashSet<String>,
    result: &mut ValidationResult,
) {
    for field in &schema.fields {
        validate_field_type(&field.field_type, &field.name, &schema.name, schema_names, result);
    }
    for field in &schema.identity {
        validate_field_type(&field.field_type, &field.name, &schema.name, schema_names, result);
    }
}

fn validate_field_type(
    ft: &nexflow_parser::ast::schema::FieldType,
    field_name: &str,
    schema_name: &str,
    schema_names: &HashSet<String>,
    result: &mut ValidationResult,
) {
    use nexflow_parser::ast::schema::FieldType;

    match ft {
        FieldType::Base { name, .. } => {
            let valid_bases = [
                "string", "char", "text", "integer", "decimal", "float", "boolean",
                "date", "timestamp", "uuid", "bytes", "bizdate", "json",
            ];
            if !valid_bases.contains(&name.as_str()) {
                result.warning(
                    DiagnosticSource::Schema,
                    format!(
                        "Field '{field_name}' in schema '{schema_name}' uses unknown base type '{name}'"
                    ),
                    Some(schema_name.to_string()),
                );
            }
        }
        FieldType::Collection { element_types, .. } => {
            for et in element_types {
                validate_field_type(et, field_name, schema_name, schema_names, result);
            }
        }
        FieldType::InlineObject { fields } => {
            for f in fields {
                validate_field_type(&f.field_type, &f.name, schema_name, schema_names, result);
            }
        }
        FieldType::Custom(name) => {
            let pascal = snake_to_pascal(name);
            if !schema_names.contains(name) && !schema_names.contains(&pascal) {
                result.warning(
                    DiagnosticSource::Schema,
                    format!(
                        "Field '{field_name}' in schema '{schema_name}' references unknown type '{name}'"
                    ),
                    Some(schema_name.to_string()),
                );
            }
        }
        FieldType::Alias(name) => {
            if !schema_names.contains(name) {
                result.warning(
                    DiagnosticSource::Schema,
                    format!(
                        "Field '{field_name}' in schema '{schema_name}' references unknown alias '{name}'"
                    ),
                    Some(schema_name.to_string()),
                );
            }
        }
    }
}

fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => {
                    let mut r = c.to_uppercase().to_string();
                    r.extend(chars);
                    r
                }
                None => String::new(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::api::*;
    use nexflow_parser::ast::common::*;
    use nexflow_parser::ast::service::*;

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
            fields: Vec::new(),
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

    fn make_api(name: &str, endpoints: Vec<Endpoint>) -> ApiDefinition {
        ApiDefinition {
            imports: Vec::new(),
            name: name.to_string(),
            version: None,
            base_path: None,
            description: None,
            targets: Vec::new(),
            auth_default: None,
            content_type: None,
            rate_limit: None,
            pagination: None,
            config: Vec::new(),
            cors: None,
            endpoints,
            events: Vec::new(),
            dependencies: Vec::new(),
            health_path: None,
            ready_path: None,
        }
    }

    fn make_endpoint(name: &str, response_schema: &str) -> Endpoint {
        Endpoint {
            name: name.to_string(),
            method: Some(HttpMethod::Get),
            path: Some("/".to_string()),
            description: None,
            params: Vec::new(),
            query: Vec::new(),
            headers: Vec::new(),
            request: None,
            response: Some(ResponseDecl {
                modifier: None,
                schema: SchemaRef {
                    name: response_schema.to_string(),
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

    fn make_service(name: &str, implements: Vec<&str>, handlers: Vec<&str>) -> ServiceDefinition {
        ServiceDefinition {
            imports: Vec::new(),
            name: name.to_string(),
            description: None,
            implements: implements.iter().map(|s| s.to_string()).collect(),
            consumes: Vec::new(),
            config: Vec::new(),
            handlers: handlers
                .iter()
                .map(|h| Handler {
                    name: h.to_string(),
                    statements: Vec::new(),
                })
                .collect(),
            cache_entries: Vec::new(),
            health: None,
            ready_path: None,
        }
    }

    #[test]
    fn test_valid_api_schema_refs() {
        let schemas = vec![make_schema("account_detail"), make_schema("not_found")];
        let api = make_api(
            "TestAPI",
            vec![make_endpoint("getAccount", "AccountDetail")],
        );

        let input = ValidationInput {
            apis: &[api],
            services: &[],
            schemas: &schemas,
        };
        let result = validate(&input);
        assert!(!result.has_errors(), "Expected no errors: {:?}", result.diagnostics);
    }

    #[test]
    fn test_missing_schema_ref() {
        let schemas = vec![make_schema("account_detail")];
        let api = make_api(
            "TestAPI",
            vec![make_endpoint("getAccount", "MissingSchema")],
        );

        let input = ValidationInput {
            apis: &[api],
            services: &[],
            schemas: &schemas,
        };
        let result = validate(&input);
        assert!(result.has_errors());
        assert_eq!(result.error_count(), 1);
        assert!(result.diagnostics[0].message.contains("MissingSchema"));
    }

    #[test]
    fn test_service_implements_unknown_api() {
        let service = make_service("TestService", vec!["NonExistentAPI"], vec!["getItem"]);
        let input = ValidationInput {
            apis: &[],
            services: &[service],
            schemas: &[],
        };
        let result = validate(&input);
        assert!(result.has_errors());
        assert!(result.diagnostics[0].message.contains("NonExistentAPI"));
    }

    #[test]
    fn test_missing_handler_coverage() {
        let api = make_api(
            "TestAPI",
            vec![
                make_endpoint("getItem", "Item"),
                make_endpoint("listItems", "Item"),
            ],
        );
        let schemas = vec![make_schema("item")];
        let service = make_service("TestService", vec!["TestAPI"], vec!["getItem"]);
        // Missing handler for "listItems"

        let input = ValidationInput {
            apis: &[api],
            services: &[service],
            schemas: &schemas,
        };
        let result = validate(&input);
        assert!(result.has_errors());
        let missing = result.diagnostics.iter().find(|d| d.message.contains("listItems"));
        assert!(missing.is_some(), "Should flag missing listItems handler");
    }

    #[test]
    fn test_extra_handler_warning() {
        let api = make_api("TestAPI", vec![make_endpoint("getItem", "Item")]);
        let schemas = vec![make_schema("item")];
        let service = make_service(
            "TestService",
            vec!["TestAPI"],
            vec!["getItem", "extraHandler"],
        );

        let input = ValidationInput {
            apis: &[api],
            services: &[service],
            schemas: &schemas,
        };
        let result = validate(&input);
        assert!(!result.has_errors(), "Extra handler should be warning, not error");
        assert!(result.warning_count() > 0);
        let extra = result.diagnostics.iter().find(|d| d.message.contains("extraHandler"));
        assert!(extra.is_some(), "Should warn about extra handler");
    }

    #[test]
    fn test_full_valid_setup() {
        let schemas = vec![
            make_schema("item"),
            make_schema("not_found"),
        ];
        let api = make_api(
            "ItemAPI",
            vec![make_endpoint("getItem", "Item")],
        );
        let service = make_service("ItemService", vec!["ItemAPI"], vec!["getItem"]);

        let input = ValidationInput {
            apis: &[api],
            services: &[service],
            schemas: &schemas,
        };
        let result = validate(&input);
        assert!(!result.has_errors(), "Expected no errors: {:?}", result.diagnostics);
    }

    #[test]
    fn test_api_without_service_warning() {
        let api = make_api("OrphanAPI", vec![]);
        let input = ValidationInput {
            apis: &[api],
            services: &[],
            schemas: &[],
        };
        let result = validate(&input);
        assert!(!result.has_errors());
        assert!(result.warning_count() > 0);
        let orphan = result.diagnostics.iter().find(|d| d.message.contains("OrphanAPI"));
        assert!(orphan.is_some());
    }
}
