// NexCore -- Nexflow Compiler: Cross-Grammar Validation
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Validates consistency across `.api`, `.service`, and `.schema` files.

use std::collections::HashSet;

use nexflow_parser::ast::api::ApiDefinition;
use nexflow_parser::ast::proc::ProcProgram;
use nexflow_parser::ast::rules::RulesProgram;
use nexflow_parser::ast::schema::SchemaDefinition;
use nexflow_parser::ast::service::ServiceDefinition;
use nexflow_parser::ast::transform::TransformProgram;

use crate::diagnostics::{DiagnosticSource, ValidationResult};

/// Input to the validator: parsed ASTs from all grammar types.
#[derive(Debug)]
pub struct ValidationInput<'a> {
    pub apis: &'a [ApiDefinition],
    pub services: &'a [ServiceDefinition],
    pub schemas: &'a [SchemaDefinition],
    pub transforms: &'a [TransformProgram],
    pub rules: &'a [RulesProgram],
    pub procs: &'a [ProcProgram],
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

    // Validate transforms: schema references and compose references
    let transform_names: HashSet<String> = input
        .transforms
        .iter()
        .flat_map(|tp| {
            tp.transforms
                .iter()
                .map(|t| t.name.clone())
                .chain(tp.transform_blocks.iter().map(|b| b.name.clone()))
        })
        .collect();

    for tp in input.transforms {
        validate_transform_refs(tp, &schema_names, &transform_names, &mut result);
    }

    // Validate rules: schema/field references
    for rp in input.rules {
        validate_rules_refs(rp, &schema_names, &mut result);
    }

    // Validate procs: transform/rules/schema references
    for pp in input.procs {
        validate_proc_refs(pp, &schema_names, &transform_names, &mut result);
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

/// Validate transform schema references and compose references.
fn validate_transform_refs(
    tp: &TransformProgram,
    schema_names: &HashSet<String>,
    transform_names: &HashSet<String>,
    result: &mut ValidationResult,
) {
    for t in &tp.transforms {
        // Check input/output schema references
        for field in t.inputs.iter().chain(t.outputs.iter()) {
            validate_transform_field_type(&field.field_type, &t.name, schema_names, result);
        }
    }

    for block in &tp.transform_blocks {
        // Check input/output schema references
        for field in block.inputs.iter().chain(block.outputs.iter()) {
            validate_transform_field_type(&field.field_type, &block.name, schema_names, result);
        }

        // Check `uses` references resolve to known transforms
        for uses_ref in &block.uses {
            if !transform_names.contains(uses_ref) {
                result.error(
                    DiagnosticSource::Transform,
                    format!(
                        "Transform block '{}' uses unknown transform '{uses_ref}'",
                        block.name
                    ),
                    Some(block.name.clone()),
                );
            }
        }

        // Check compose references
        if let Some(compose) = &block.compose {
            validate_compose_refs(&compose.refs, &block.name, transform_names, result);
            if let Some(then) = &compose.then {
                validate_compose_refs(&then.refs, &block.name, transform_names, result);
            }
        }
    }
}

/// Check that compose refs resolve to known transforms.
fn validate_compose_refs(
    refs: &[nexflow_parser::ast::transform::ComposeRef],
    block_name: &str,
    transform_names: &HashSet<String>,
    result: &mut ValidationResult,
) {
    use nexflow_parser::ast::transform::ComposeRef;
    for r in refs {
        let name = match r {
            ComposeRef::Simple(n) => n,
            ComposeRef::Conditional { transform, .. } => transform,
            ComposeRef::Otherwise(n) => n,
        };
        if !transform_names.contains(name) {
            result.error(
                DiagnosticSource::Transform,
                format!(
                    "Compose in '{}' references unknown transform '{}'",
                    block_name, name
                ),
                Some(block_name.to_string()),
            );
        }
    }
}

/// Validate a transform field type's schema references.
fn validate_transform_field_type(
    ft: &nexflow_parser::ast::transform::TransformFieldType,
    transform_name: &str,
    schema_names: &HashSet<String>,
    result: &mut ValidationResult,
) {
    use nexflow_parser::ast::transform::TransformFieldType;
    match ft {
        TransformFieldType::Reference(name) => {
            let pascal = snake_to_pascal(name);
            if !schema_names.contains(name) && !schema_names.contains(&pascal) {
                result.warning(
                    DiagnosticSource::Transform,
                    format!(
                        "Transform '{transform_name}' references unknown schema '{name}'"
                    ),
                    Some(transform_name.to_string()),
                );
            }
        }
        TransformFieldType::AliasRef(name) => {
            if !schema_names.contains(name) {
                result.warning(
                    DiagnosticSource::Transform,
                    format!(
                        "Transform '{transform_name}' references unknown alias '{name}'"
                    ),
                    Some(transform_name.to_string()),
                );
            }
        }
        TransformFieldType::Collection { element_types, .. } => {
            for et in element_types {
                validate_transform_field_type(et, transform_name, schema_names, result);
            }
        }
        TransformFieldType::Base { .. } => {}
    }
}

/// Validate rules schema references.
fn validate_rules_refs(
    rp: &RulesProgram,
    schema_names: &HashSet<String>,
    result: &mut ValidationResult,
) {
    // Decision tables: check condition/action field references
    for dt in &rp.decision_tables {
        // Check service declarations
        for svc in &rp.services {
            if svc.return_type != "void" {
                let pascal = snake_to_pascal(&svc.return_type);
                if !schema_names.contains(&svc.return_type) && !schema_names.contains(&pascal) {
                    result.warning(
                        DiagnosticSource::Rules,
                        format!(
                            "Service '{}' in rules returns unknown type '{}'",
                            svc.name, svc.return_type
                        ),
                        Some(dt.name.clone()),
                    );
                }
            }
        }
    }

    // Action declarations: check emit targets reference known schemas
    for action in &rp.actions {
        use nexflow_parser::ast::rules::ActionTarget;
        if let ActionTarget::Emit { stream } = &action.target {
            // Stream names are not schemas, but flag for cross-grammar check
            result.info(
                DiagnosticSource::Rules,
                format!(
                    "Action '{}' emits to stream '{}' (verify stream exists in proc)",
                    action.name, stream
                ),
                Some(action.name.clone()),
            );
        }
    }
}

/// Validate proc references to transforms, rules, and schemas.
fn validate_proc_refs(
    pp: &ProcProgram,
    schema_names: &HashSet<String>,
    transform_names: &HashSet<String>,
    result: &mut ValidationResult,
) {
    use nexflow_parser::ast::proc::ProcessStatement;

    for process in &pp.processes {
        for stmt in &process.body {
            match stmt {
                ProcessStatement::Receive { schema, name, .. } => {
                    if let Some(schema_ref) = schema {
                        let pascal = snake_to_pascal(schema_ref);
                        if !schema_names.contains(schema_ref) && !schema_names.contains(&pascal) {
                            result.error(
                                DiagnosticSource::Process,
                                format!(
                                    "Receive '{}' in process '{}' references unknown schema '{}'",
                                    name, process.name, schema_ref
                                ),
                                Some(process.name.clone()),
                            );
                        }
                    }
                }
                ProcessStatement::Transform { using, .. } => {
                    if !transform_names.contains(using) {
                        result.error(
                            DiagnosticSource::Process,
                            format!(
                                "Transform step in process '{}' references unknown transform '{}'",
                                process.name, using
                            ),
                            Some(process.name.clone()),
                        );
                    }
                }
                ProcessStatement::Emit { schema, name, .. } => {
                    if let Some(schema_ref) = schema {
                        let pascal = snake_to_pascal(schema_ref);
                        if !schema_names.contains(schema_ref) && !schema_names.contains(&pascal) {
                            result.error(
                                DiagnosticSource::Process,
                                format!(
                                    "Emit '{}' in process '{}' references unknown schema '{}'",
                                    name, process.name, schema_ref
                                ),
                                Some(process.name.clone()),
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        // Also check phases
        for phase in &process.phases {
            for stmt in &phase.statements {
                if let ProcessStatement::Transform { using, .. } = stmt {
                    if !transform_names.contains(using) {
                        result.error(
                            DiagnosticSource::Process,
                            format!(
                                "Transform step in phase '{}' of process '{}' references unknown transform '{}'",
                                phase.name, process.name, using
                            ),
                            Some(process.name.clone()),
                        );
                    }
                }
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
            transforms: &[],
            rules: &[],
            procs: &[],
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
            transforms: &[],
            rules: &[],
            procs: &[],
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
            transforms: &[],
            rules: &[],
            procs: &[],
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
            transforms: &[],
            rules: &[],
            procs: &[],
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
            transforms: &[],
            rules: &[],
            procs: &[],
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
            transforms: &[],
            rules: &[],
            procs: &[],
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
            transforms: &[],
            rules: &[],
            procs: &[],
        };
        let result = validate(&input);
        assert!(!result.has_errors());
        assert!(result.warning_count() > 0);
        let orphan = result.diagnostics.iter().find(|d| d.message.contains("OrphanAPI"));
        assert!(orphan.is_some());
    }

    // ---- DSL03: Transform/Rules/Proc validation tests ----

    use nexflow_parser::ast::transform::*;
    use nexflow_parser::ast::rules::*;
    use nexflow_parser::ast::proc::*;

    fn make_transform(name: &str, input_ref: Option<&str>) -> TransformDef {
        let inputs = input_ref
            .map(|r| vec![FieldSpec {
                name: Some("input".to_string()),
                field_type: TransformFieldType::Reference(r.to_string()),
                nullable: false,
                required: true,
                default: None,
            }])
            .unwrap_or_default();

        TransformDef {
            name: name.to_string(),
            version: None,
            description: None,
            previous_version: None,
            compatibility: None,
            pure: None,
            idempotent: None,
            cache: None,
            inputs,
            outputs: Vec::new(),
            lookup: None,
            lookups: Vec::new(),
            state: None,
            params: Vec::new(),
            validate_input: Vec::new(),
            apply: Vec::new(),
            validate_output: Vec::new(),
            on_error: None,
        }
    }

    fn make_transform_program(transforms: Vec<TransformDef>) -> TransformProgram {
        TransformProgram {
            imports: Vec::new(),
            transforms,
            transform_blocks: Vec::new(),
        }
    }

    fn make_proc_program(name: &str, stmts: Vec<ProcessStatement>) -> ProcProgram {
        ProcProgram {
            imports: Vec::new(),
            processes: vec![ProcessDef {
                name: name.to_string(),
                execution: None,
                business_date: None,
                processing_date: None,
                markers: Vec::new(),
                state_machine: None,
                body: stmts,
                phases: Vec::new(),
                state: None,
                metrics: None,
                resilience: None,
            }],
        }
    }

    #[test]
    fn test_transform_valid_schema_ref() {
        let schemas = vec![make_schema("account")];
        let tp = make_transform_program(vec![
            make_transform("update_account", Some("account")),
        ]);

        let input = ValidationInput {
            apis: &[],
            services: &[],
            schemas: &schemas,
            transforms: &[tp],
            rules: &[],
            procs: &[],
        };
        let result = validate(&input);
        assert!(!result.has_errors(), "Valid schema ref should not error: {:?}", result.diagnostics);
    }

    #[test]
    fn test_transform_unknown_schema_ref_warns() {
        let tp = make_transform_program(vec![
            make_transform("update_account", Some("missing_schema")),
        ]);

        let input = ValidationInput {
            apis: &[],
            services: &[],
            schemas: &[],
            transforms: &[tp],
            rules: &[],
            procs: &[],
        };
        let result = validate(&input);
        assert!(result.warning_count() > 0);
        let warn = result.diagnostics.iter().find(|d| d.message.contains("missing_schema"));
        assert!(warn.is_some(), "Should warn about missing schema ref");
    }

    #[test]
    fn test_proc_valid_transform_ref() {
        let tp = make_transform_program(vec![
            make_transform("enrich_account", None),
        ]);
        let pp = make_proc_program("main_proc", vec![
            ProcessStatement::Transform {
                input: "raw".to_string(),
                using: "enrich_account".to_string(),
                into: "enriched".to_string(),
            },
        ]);

        let input = ValidationInput {
            apis: &[],
            services: &[],
            schemas: &[],
            transforms: &[tp],
            rules: &[],
            procs: &[pp],
        };
        let result = validate(&input);
        assert!(!result.has_errors(), "Valid transform ref should not error: {:?}", result.diagnostics);
    }

    #[test]
    fn test_proc_unknown_transform_ref_errors() {
        let pp = make_proc_program("main_proc", vec![
            ProcessStatement::Transform {
                input: "raw".to_string(),
                using: "nonexistent_transform".to_string(),
                into: "out".to_string(),
            },
        ]);

        let input = ValidationInput {
            apis: &[],
            services: &[],
            schemas: &[],
            transforms: &[],
            rules: &[],
            procs: &[pp],
        };
        let result = validate(&input);
        assert!(result.has_errors());
        let err = result.diagnostics.iter().find(|d| d.message.contains("nonexistent_transform"));
        assert!(err.is_some(), "Should error about missing transform");
    }

    #[test]
    fn test_proc_unknown_schema_in_receive_errors() {
        let pp = make_proc_program("main_proc", vec![
            ProcessStatement::Receive {
                name: "orders".to_string(),
                source_type: "topic".to_string(),
                source: "orders-topic".to_string(),
                schema: Some("missing_order".to_string()),
                key: None,
                options: Vec::new(),
            },
        ]);

        let input = ValidationInput {
            apis: &[],
            services: &[],
            schemas: &[],
            transforms: &[],
            rules: &[],
            procs: &[pp],
        };
        let result = validate(&input);
        assert!(result.has_errors());
        let err = result.diagnostics.iter().find(|d| d.message.contains("missing_order"));
        assert!(err.is_some(), "Should error about missing schema in receive");
    }
}
