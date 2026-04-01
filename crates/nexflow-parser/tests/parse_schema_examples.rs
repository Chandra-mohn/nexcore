// NexCore -- Nexflow Parser: SchemaDSL Integration Tests
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Parse the 4 .schema example files and verify AST structure.

use nexflow_parser::ast::schema::*;
use nexflow_parser::parse_schema;

#[test]
fn parse_account_schemas() {
    let source = include_str!("../../../examples/nexflow/schema/account.schema");
    let program = parse_schema(source).expect("Failed to parse account.schema");

    // account.schema has 10 schema definitions
    assert_eq!(program.schemas.len(), 10);

    // -- account_summary --
    let summary = &program.schemas[0];
    assert_eq!(summary.name, "account_summary");
    assert_eq!(summary.patterns, vec![MutationPattern::MasterData]);
    assert_eq!(
        summary.version.as_ref().map(|v| v.number.as_str()),
        Some("1.0.0")
    );

    // Identity
    assert_eq!(summary.identity.len(), 1);
    assert_eq!(summary.identity[0].name, "account_id");
    assert!(summary.identity[0].required);

    // Fields
    assert_eq!(summary.fields.len(), 8);
    let balance = summary.fields.iter().find(|f| f.name == "balance").unwrap();
    assert!(balance.required);
    match &balance.field_type {
        FieldType::Base { name, params } => {
            assert_eq!(name, "decimal");
            assert_eq!(params, &[14, 2]);
        }
        _ => panic!("Expected base type for balance"),
    }

    let currency = summary.fields.iter().find(|f| f.name == "currency").unwrap();
    assert!(currency.required);
    assert_eq!(currency.default.as_deref(), Some("USD"));
    match &currency.field_type {
        FieldType::Base { name, params } => {
            assert_eq!(name, "char");
            assert_eq!(params, &[3]);
        }
        _ => panic!("Expected base type for currency"),
    }

    // -- account_detail --
    let detail = &program.schemas[1];
    assert_eq!(detail.name, "account_detail");
    assert_eq!(detail.patterns, vec![MutationPattern::MasterData]);
    assert_eq!(detail.fields.len(), 14);

    // PII fields
    let holder = detail.fields.iter().find(|f| f.name == "holder_name").unwrap();
    assert!(holder.pii.is_some());
    assert_eq!(holder.pii.as_ref().unwrap().profile.as_deref(), Some("name"));

    let email = detail.fields.iter().find(|f| f.name == "email").unwrap();
    assert!(email.pii.is_some());
    assert_eq!(email.pii.as_ref().unwrap().profile.as_deref(), Some("email"));

    // Constraints
    assert_eq!(detail.constraints.len(), 2);
    match &detail.constraints[0] {
        ConstraintDecl::Enum { field, values } => {
            assert_eq!(field, "status");
            assert_eq!(values, &["active", "suspended", "closed"]);
        }
        other => panic!("Expected enum constraint, got {other:?}"),
    }

    // -- account_closure_receipt (event_log) --
    let receipt = &program.schemas[3];
    assert_eq!(receipt.name, "account_closure_receipt");
    assert_eq!(receipt.patterns, vec![MutationPattern::EventLog]);

    // -- account_status (reference_data with entries) --
    let status = &program.schemas[5];
    assert_eq!(status.name, "account_status");
    assert_eq!(status.patterns, vec![MutationPattern::ReferenceData]);
    assert_eq!(status.entries.len(), 3);

    let active = &status.entries[0];
    assert_eq!(active.name, "active");
    assert_eq!(active.fields.len(), 2);
    assert_eq!(active.fields[0].key, "code");
    assert_eq!(active.fields[0].value, "active");

    // -- create_account_request (command pattern) --
    let create = &program.schemas[6];
    assert_eq!(create.name, "create_account_request");
    assert_eq!(create.patterns, vec![MutationPattern::Command]);
    assert_eq!(create.constraints.len(), 1);

    // -- account_created_event (event_log with streaming) --
    let event = &program.schemas[7];
    assert_eq!(event.name, "account_created_event");
    assert_eq!(event.patterns, vec![MutationPattern::EventLog]);

    let streaming = event.streaming.as_ref().expect("missing streaming block");
    assert_eq!(streaming.key_fields, vec!["account_id"]);
    assert_eq!(streaming.time_field.as_deref(), Some("created_at"));
    assert_eq!(streaming.time_semantics, Some(TimeSemantics::EventTime));
    let delay = streaming.watermark_delay.as_ref().expect("missing watermark_delay");
    assert_eq!(delay.value, 5);
    assert_eq!(delay.unit, "seconds");
}

#[test]
fn parse_address_schemas() {
    let source = include_str!("../../../examples/nexflow/schema/address.schema");
    let program = parse_schema(source).expect("Failed to parse address.schema");

    assert_eq!(program.schemas.len(), 2);

    // -- address_update (command with pattern constraints) --
    let update = &program.schemas[0];
    assert_eq!(update.name, "address_update");
    assert_eq!(update.patterns, vec![MutationPattern::Command]);
    assert_eq!(update.fields.len(), 6);

    // Pattern constraints
    assert_eq!(update.constraints.len(), 3);
    match &update.constraints[0] {
        ConstraintDecl::Pattern { field, regex } => {
            assert_eq!(field, "state");
            assert_eq!(regex, "[A-Z]{2}");
        }
        other => panic!("Expected pattern constraint, got {other:?}"),
    }

    // Default value
    let country = update.fields.iter().find(|f| f.name == "country").unwrap();
    assert_eq!(country.default.as_deref(), Some("US"));
    match &country.field_type {
        FieldType::Base { name, params } => {
            assert_eq!(name, "char");
            assert_eq!(params, &[2]);
        }
        _ => panic!("Expected char(2)"),
    }

    // -- address_changed_event --
    let event = &program.schemas[1];
    assert_eq!(event.name, "address_changed_event");
    assert_eq!(event.patterns, vec![MutationPattern::EventLog]);
    assert!(event.streaming.is_some());
}

#[test]
fn parse_transfer_schemas() {
    let source = include_str!("../../../examples/nexflow/schema/transfer.schema");
    let program = parse_schema(source).expect("Failed to parse transfer.schema");

    assert_eq!(program.schemas.len(), 2);

    // -- transfer_request (command with range constraint) --
    let request = &program.schemas[0];
    assert_eq!(request.name, "transfer_request");
    assert_eq!(request.patterns, vec![MutationPattern::Command]);
    assert_eq!(request.fields.len(), 6);

    // Range constraint
    assert_eq!(request.constraints.len(), 1);
    match &request.constraints[0] {
        ConstraintDecl::Range { field, min, max } => {
            assert_eq!(field, "amount");
            assert_eq!(min, "0.01");
            assert_eq!(max, "999999999.99");
        }
        other => panic!("Expected range constraint, got {other:?}"),
    }

    // -- transfer_result (event_log with enum constraint) --
    let result = &program.schemas[1];
    assert_eq!(result.name, "transfer_result");
    assert_eq!(result.patterns, vec![MutationPattern::EventLog]);
    assert_eq!(result.constraints.len(), 1);
    match &result.constraints[0] {
        ConstraintDecl::Enum { field, values } => {
            assert_eq!(field, "status");
            assert_eq!(
                values,
                &["pending", "completed", "failed", "reversed"]
            );
        }
        other => panic!("Expected enum constraint, got {other:?}"),
    }
}

#[test]
fn parse_errors_schemas() {
    let source = include_str!("../../../examples/nexflow/schema/errors.schema");
    let program = parse_schema(source).expect("Failed to parse errors.schema");

    assert_eq!(program.schemas.len(), 6);

    // All are response pattern
    for schema in &program.schemas {
        assert_eq!(schema.patterns, vec![MutationPattern::Response]);
    }

    // -- validation_error --
    let ve = &program.schemas[0];
    assert_eq!(ve.name, "validation_error");
    assert_eq!(ve.fields.len(), 4);

    // list(string) type
    let field_errors = ve.fields.iter().find(|f| f.name == "field_errors").unwrap();
    match &field_errors.field_type {
        FieldType::Collection {
            kind,
            element_types,
        } => {
            assert_eq!(*kind, CollectionKind::List);
            assert_eq!(element_types.len(), 1);
            match &element_types[0] {
                FieldType::Base { name, .. } => assert_eq!(name, "string"),
                _ => panic!("Expected string element type"),
            }
        }
        _ => panic!("Expected collection type for field_errors"),
    }

    // -- account_not_found --
    let anf = &program.schemas[1];
    assert_eq!(anf.name, "account_not_found");

    // Default value for error_code
    let error_code = anf.fields.iter().find(|f| f.name == "error_code").unwrap();
    assert_eq!(
        error_code.default.as_deref(),
        Some("ACCOUNT_NOT_FOUND")
    );

    // -- insufficient_funds --
    let isf = &program.schemas[5];
    assert_eq!(isf.name, "insufficient_funds");
    assert_eq!(isf.fields.len(), 5);
}
