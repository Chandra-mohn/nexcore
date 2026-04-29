// NexCore -- Nexflow Compiler: Integration Test
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Validate the account API + service + schema system end-to-end.

use nexflow_compiler::validate::ValidationInput;
use nexflow_parser::{parse_api, parse_schema, parse_service};

#[test]
fn validate_account_system_is_consistent() {
    // Parse API
    let api_source = include_str!("../../../examples/nexflow/api/account-api.api");
    let api = parse_api(api_source).expect("Failed to parse account-api.api");

    // Parse schemas
    let schema_sources = [
        include_str!("../../../examples/nexflow/schema/account.schema"),
        include_str!("../../../examples/nexflow/schema/address.schema"),
        include_str!("../../../examples/nexflow/schema/transfer.schema"),
        include_str!("../../../examples/nexflow/schema/errors.schema"),
    ];
    let mut all_schemas = Vec::new();
    for source in &schema_sources {
        let program = parse_schema(source).expect("Failed to parse schema");
        all_schemas.extend(program.schemas);
    }

    // Parse service
    let svc_source = include_str!("../../../examples/nexflow/service/account-service.service");
    let service = parse_service(svc_source).expect("Failed to parse account-service.service");

    // Validate
    let input = ValidationInput {
        apis: &[api],
        services: &[service],
        schemas: &all_schemas,
        transforms: &[],
        rules: &[],
        procs: &[],
    };
    let result = nexflow_compiler::validate(&input);

    // Print all diagnostics for visibility
    for diag in &result.diagnostics {
        println!("{diag}");
    }

    // The account system should have NO errors
    // (it's a well-formed example set)
    assert!(
        !result.has_errors(),
        "Expected no errors, got {}: {:?}",
        result.error_count(),
        result
            .diagnostics
            .iter()
            .filter(|d| d.level == nexflow_compiler::DiagnosticLevel::Error)
            .collect::<Vec<_>>()
    );

    // Should validate successfully with some info/warnings
    println!(
        "Validation complete: {} errors, {} warnings",
        result.error_count(),
        result.warning_count()
    );
}

#[test]
fn validate_detects_missing_schema() {
    // Create an API that references a non-existent schema
    let api_source = include_str!("../../../examples/nexflow/api/account-api.api");
    let api = parse_api(api_source).expect("Failed to parse");

    // Only provide errors.schema -- missing account/address/transfer schemas
    let errors_source = include_str!("../../../examples/nexflow/schema/errors.schema");
    let program = parse_schema(errors_source).expect("Failed to parse");

    let input = ValidationInput {
        apis: &[api],
        services: &[],
        schemas: &program.schemas,
        transforms: &[],
        rules: &[],
        procs: &[],
    };
    let result = nexflow_compiler::validate(&input);

    // Should have errors for missing schemas like AccountDetail, AccountSummary, etc.
    assert!(result.has_errors(), "Should detect missing schema references");
    assert!(
        result.error_count() >= 5,
        "Expected at least 5 errors for missing schemas, got {}",
        result.error_count()
    );

    // Print for debugging
    for diag in &result.diagnostics {
        if diag.level == nexflow_compiler::DiagnosticLevel::Error {
            println!("  ERROR: {diag}");
        }
    }
}

#[test]
fn validate_api_without_service_warns() {
    let api_source = include_str!("../../../examples/nexflow/api/catalog-api.api");
    let api = parse_api(api_source).expect("Failed to parse");

    let input = ValidationInput {
        apis: &[api],
        services: &[],
        schemas: &[],
        transforms: &[],
        rules: &[],
        procs: &[],
    };
    let result = nexflow_compiler::validate(&input);

    // Should warn about no implementing service
    assert!(result.warning_count() > 0);
    let orphan = result
        .diagnostics
        .iter()
        .find(|d| d.message.contains("no implementing service"));
    assert!(orphan.is_some(), "Should warn about missing service");
}
