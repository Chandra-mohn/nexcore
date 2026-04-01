// NexCore -- Nexflow Parser: ServiceDSL Integration Tests
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Parse the 3 .service example files and verify AST structure.

use nexflow_parser::ast::common::*;
use nexflow_parser::ast::service::*;
use nexflow_parser::parse_service;

#[test]
fn parse_account_service() {
    let source = include_str!("../../../examples/nexflow/service/account-service.service");
    let svc = parse_service(source).expect("Failed to parse account-service.service");

    assert_eq!(svc.name, "AccountService");
    assert!(svc.description.is_some());

    // Implements / Consumes
    assert_eq!(svc.implements, vec!["AccountAPI"]);
    assert_eq!(svc.consumes.len(), 2);
    assert!(svc.consumes.contains(&"AuditAPI".to_string()));
    assert!(svc.consumes.contains(&"NotificationAPI".to_string()));

    // Config block
    assert!(!svc.config.is_empty());
    let tracing = svc.config.iter().find(|c| c.key == "tracing_provider");
    assert!(tracing.is_some());

    // Handlers
    assert_eq!(svc.handlers.len(), 7);
    let handler_names: Vec<&str> = svc.handlers.iter().map(|h| h.name.as_str()).collect();
    assert!(handler_names.contains(&"listAccounts"));
    assert!(handler_names.contains(&"getBalance"));
    assert!(handler_names.contains(&"transferFunds"));
    assert!(handler_names.contains(&"closeAccount"));

    // Check transferFunds handler has transaction
    let transfer = svc.handlers.iter().find(|h| h.name == "transferFunds");
    assert!(transfer.is_some());
    let has_transaction = transfer.unwrap().statements.iter().any(|s| {
        matches!(s, HandlerStatement::Transaction { .. })
    });
    assert!(has_transaction, "transferFunds should have a transaction block");

    // Check closeAccount has on_error
    let close = svc.handlers.iter().find(|h| h.name == "closeAccount");
    assert!(close.is_some());
    let has_on_error = close.unwrap().statements.iter().any(|s| {
        matches!(s, HandlerStatement::OnError { .. })
    });
    assert!(has_on_error, "closeAccount should have an on_error block");

    // Cache entries
    assert_eq!(svc.cache_entries.len(), 3);
    let balance_cache = svc.cache_entries.iter().find(|c| c.handler_name == "getBalance");
    assert!(balance_cache.is_some());
    assert!(!balance_cache.unwrap().invalidate_on.is_empty());

    // Health block with cfg. references
    let health = svc.health.as_ref().expect("missing health block");
    assert_eq!(health.path, "/health");
    assert_eq!(health.checks.len(), 3);

    // Ready
    assert_eq!(svc.ready_path.as_deref(), Some("/ready"));

    // Imports
    assert_eq!(svc.imports.len(), 8);
}

#[test]
fn parse_order_service() {
    let source = include_str!("../../../examples/nexflow/service/order-service.service");
    let svc = parse_service(source).expect("Failed to parse order-service.service");

    assert_eq!(svc.name, "OrderService");
    assert_eq!(svc.implements, vec!["OrderAPI"]);
    assert_eq!(svc.consumes.len(), 3);

    // Check placeOrder has a saga
    let place = svc.handlers.iter().find(|h| h.name == "placeOrder");
    assert!(place.is_some());
    let has_saga = place.unwrap().statements.iter().any(|s| {
        matches!(s, HandlerStatement::Saga { .. })
    });
    assert!(has_saga, "placeOrder should have a saga block");

    // Check placeOrder has a parallel block
    let has_parallel = place.unwrap().statements.iter().any(|s| {
        matches!(s, HandlerStatement::Parallel { .. })
    });
    assert!(has_parallel, "placeOrder should have a parallel block");

    // Check cancelOrder also has a saga
    let cancel = svc.handlers.iter().find(|h| h.name == "cancelOrder");
    assert!(cancel.is_some());
    let has_saga = cancel.unwrap().statements.iter().any(|s| {
        matches!(s, HandlerStatement::Saga { .. })
    });
    assert!(has_saga, "cancelOrder should have a saga block");

    // Handlers count
    assert_eq!(svc.handlers.len(), 5);
}

#[test]
fn parse_mobile_bff_service() {
    let source = include_str!("../../../examples/nexflow/service/mobile-bff-service.service");
    let svc = parse_service(source).expect("Failed to parse mobile-bff-service.service");

    assert_eq!(svc.name, "MobileBffService");
    assert_eq!(svc.implements, vec!["MobileBffAPI"]);
    assert_eq!(svc.consumes.len(), 4);

    // getDashboard has parallel fan-out
    let dashboard = svc.handlers.iter().find(|h| h.name == "getDashboard");
    assert!(dashboard.is_some());
    let has_parallel = dashboard.unwrap().statements.iter().any(|s| {
        matches!(s, HandlerStatement::Parallel { .. })
    });
    assert!(has_parallel, "getDashboard should have a parallel block");

    // getDashboard has fallback error cases
    let has_fallback = dashboard.unwrap().statements.iter().any(|s| {
        if let HandlerStatement::OnError { cases } = s {
            cases.iter().any(|c| matches!(c, ErrorCase::Fallback { .. }))
        } else {
            false
        }
    });
    assert!(has_fallback, "getDashboard should have fallback error cases");

    // Config with cfg. cache references
    let cache = svc.cache_entries.iter().find(|c| c.handler_name == "getDashboard");
    assert!(cache.is_some());
    assert!(matches!(cache.unwrap().ttl, ValueOrCfg::CfgRef { .. }));

    // Health checks use cfg. references
    let health = svc.health.as_ref().expect("missing health block");
    assert_eq!(health.checks.len(), 4);
    assert!(matches!(health.checks[0].timeout, ValueOrCfg::CfgRef { .. }));
}
