// NexCore -- Nexflow Parser: ApiDSL Integration Tests
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Parse the 3 .api example files and verify AST structure.

use nexflow_parser::ast::api::*;
use nexflow_parser::ast::common::*;
use nexflow_parser::parse_api;

#[test]
fn parse_account_api() {
    let source = include_str!("../../../examples/nexflow/api/account-api.api");
    let api = parse_api(source).expect("Failed to parse account-api.api");

    assert_eq!(api.name, "AccountAPI");
    assert_eq!(api.version.as_deref(), Some("2.1"));
    assert_eq!(api.base_path.as_deref(), Some("/api/v2/accounts"));
    assert!(api.description.is_some());

    // Targets
    assert_eq!(api.targets.len(), 3);
    assert!(api.targets.contains(&"rest".to_string()));
    assert!(api.targets.contains(&"graphql".to_string()));
    assert!(api.targets.contains(&"grpc".to_string()));

    // Auth default
    let auth = api.auth_default.as_ref().expect("missing auth default");
    assert_eq!(auth.scheme, "bearer");
    assert_eq!(auth.scope.as_deref(), Some("accounts:read"));

    // Rate limit with cfg. reference
    let rl = api.rate_limit.as_ref().expect("missing rate limit");
    assert!(matches!(rl.limit, ValueOrCfg::CfgRef { .. }));
    assert!(matches!(rl.burst, Some(ValueOrCfg::CfgRef { .. })));

    // Pagination
    let pag = api.pagination.as_ref().expect("missing pagination");
    assert_eq!(pag.style, "cursor");
    assert_eq!(pag.default_size, 50);
    assert_eq!(pag.max_size, 200);

    // Config block
    assert!(!api.config.is_empty());
    let rate_cfg = api.config.iter().find(|c| c.key == "default_rate_limit");
    assert!(rate_cfg.is_some());

    // CORS
    assert!(api.cors.is_some());

    // Endpoints: 7 active + 1 deprecated = 8 total
    assert_eq!(api.endpoints.len(), 8);

    // Check a specific endpoint
    let get_balance = api.endpoints.iter().find(|e| e.name == "getBalance");
    assert!(get_balance.is_some());
    let gb = get_balance.unwrap();
    assert_eq!(gb.method, Some(HttpMethod::Get));
    assert_eq!(gb.path.as_deref(), Some("/{account_id}/balance"));
    assert_eq!(gb.params.len(), 1);
    assert_eq!(gb.errors.len(), 2);
    assert!(gb.timeout.is_some());

    // Check async endpoint
    let transfer = api.endpoints.iter().find(|e| e.name == "transferFunds");
    assert!(transfer.is_some());
    assert!(transfer.unwrap().is_async);

    // Check deprecated endpoint
    let deprecated = api.endpoints.iter().find(|e| e.name == "getAccountV1");
    assert!(deprecated.is_some());
    let dep = deprecated.unwrap();
    assert!(dep.is_deprecated);
    assert_eq!(dep.sunset.as_deref(), Some("2026-12-31"));
    assert_eq!(dep.replacement.as_deref(), Some("getAccount"));

    // Events
    assert_eq!(api.events.len(), 4);
    let created = api.events.iter().find(|e| e.name == "AccountCreated");
    assert!(created.is_some());
    assert_eq!(
        created.unwrap().topic.as_deref(),
        Some("accounts.created")
    );

    // Dependencies
    assert_eq!(api.dependencies.len(), 2);

    // Health + Ready
    assert_eq!(api.health_path.as_deref(), Some("/health"));
    assert_eq!(api.ready_path.as_deref(), Some("/ready"));

    // Imports
    assert_eq!(api.imports.len(), 4);
}

#[test]
fn parse_catalog_api() {
    let source = include_str!("../../../examples/nexflow/api/catalog-api.api");
    let api = parse_api(source).expect("Failed to parse catalog-api.api");

    assert_eq!(api.name, "CatalogAPI");
    assert_eq!(api.version.as_deref(), Some("1.0"));
    assert_eq!(api.targets.len(), 2);
    assert_eq!(api.endpoints.len(), 8);
    assert_eq!(api.events.len(), 3);

    // Check search endpoint has required query param
    let search = api.endpoints.iter().find(|e| e.name == "searchProducts");
    assert!(search.is_some());
    let q_param = search.unwrap().query.iter().find(|p| p.name == "q");
    assert!(q_param.is_some());
    assert!(q_param.unwrap().required);
}

#[test]
fn parse_notification_api() {
    let source = include_str!("../../../examples/nexflow/api/notification-api.api");
    let api = parse_api(source).expect("Failed to parse notification-api.api");

    assert_eq!(api.name, "NotificationAPI");
    assert_eq!(api.version.as_deref(), Some("2.0"));

    // All send endpoints are async
    for name in &["sendEmail", "sendSms", "sendPush", "sendBatch"] {
        let ep = api.endpoints.iter().find(|e| e.name == *name);
        assert!(ep.is_some(), "Missing endpoint: {name}");
        assert!(ep.unwrap().is_async, "{name} should be async");
    }

    assert_eq!(api.events.len(), 4);
    assert_eq!(api.dependencies.len(), 1);
}
