// NexCore -- Nexflow Codegen: Service Integration Test
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! End-to-end test: parse .api + .schema + .service, generate full project.

use nexflow_parser::{parse_api, parse_schema, parse_service};

#[test]
fn generate_with_account_service() {
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

    // Generate with service
    let project = nexflow_codegen::generate_with_service(&api, &all_schemas, &service)
        .expect("Code generation failed");

    // Should have service_impl.rs
    assert!(
        project.files.contains_key("src/service_impl.rs"),
        "missing service_impl.rs"
    );

    let svc_impl = &project.files["src/service_impl.rs"];

    // Service struct
    assert!(
        svc_impl.contains("pub struct AccountService {"),
        "missing AccountService struct"
    );
    assert!(
        svc_impl.contains("impl AccountService {"),
        "missing impl block"
    );
    assert!(
        svc_impl.contains("Implements: AccountAPI"),
        "missing implements doc"
    );
    assert!(
        svc_impl.contains("Consumes: AuditAPI, NotificationAPI"),
        "missing consumes doc"
    );

    // All 7 handlers
    let expected_handlers = [
        "list_accounts",
        "get_account",
        "get_balance",
        "create_account",
        "update_address",
        "close_account",
        "transfer_funds",
    ];
    for name in &expected_handlers {
        assert!(
            svc_impl.contains(&format!("pub async fn {name}")),
            "missing handler: {name}"
        );
    }

    // Pipeline steps in handlers

    // Authorization
    assert!(svc_impl.contains("authorize scope \"accounts:read\""), "missing authorize read");
    assert!(svc_impl.contains("authorize scope \"accounts:write\""), "missing authorize write");
    assert!(svc_impl.contains("authorize scope \"accounts:admin\""), "missing authorize admin");
    assert!(svc_impl.contains("authorize scope \"accounts:balance\""), "missing authorize balance");
    assert!(svc_impl.contains("authorize scope \"accounts:transfer\""), "missing authorize transfer");

    // Lookups
    assert!(svc_impl.contains("Step: lookup"), "missing lookup step");

    // On_error
    assert!(svc_impl.contains("404 when account is null"), "missing 404 error case");

    // Transforms
    assert!(svc_impl.contains("Step: transform"), "missing transform step");
    assert!(svc_impl.contains("into response"), "missing transform into response");

    // Validate
    assert!(svc_impl.contains("Step: validate"), "missing validate step");

    // Persist
    assert!(svc_impl.contains("Step: persist"), "missing persist step");

    // Call
    assert!(svc_impl.contains("Step: call"), "missing call step");

    // Publish
    assert!(svc_impl.contains("publish AccountCreated"), "missing publish AccountCreated");
    assert!(svc_impl.contains("publish AccountClosed"), "missing publish AccountClosed");
    assert!(svc_impl.contains("publish BalanceChanged"), "missing publish BalanceChanged");
    assert!(svc_impl.contains("publish AddressChanged"), "missing publish AddressChanged");

    // Respond
    assert!(svc_impl.contains("respond 201"), "missing respond 201");
    assert!(svc_impl.contains("respond 200"), "missing respond 200");
    assert!(svc_impl.contains("respond 202"), "missing respond 202");

    // Transaction with on_rollback
    assert!(svc_impl.contains("-- transaction begin --"), "missing transaction begin");
    assert!(svc_impl.contains("-- on_rollback --"), "missing on_rollback");
    assert!(svc_impl.contains("-- transaction end --"), "missing transaction end");

    // Config struct
    assert!(svc_impl.contains("pub struct AccountServiceConfig {"));
    assert!(svc_impl.contains("pub account_db_pool_size: i64"));

    // Cache comments
    assert!(svc_impl.contains("Cache configuration:"));
    assert!(svc_impl.contains("getAccount"));
    assert!(svc_impl.contains("invalidate_on:"));

    // lib.rs should include service_impl module
    let lib_rs = &project.files["src/lib.rs"];
    assert!(
        lib_rs.contains("pub mod service_impl;"),
        "missing service_impl module in lib.rs"
    );

    // Print sizes
    for (path, content) in &project.files {
        println!("{path}: {} bytes, {} lines", content.len(), content.lines().count());
    }
}
