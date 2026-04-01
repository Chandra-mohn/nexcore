// NexCore -- Nexflow Parser: TransformDSL Integration Tests
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

use nexflow_parser::ast::transform::*;
use nexflow_parser::parse_transform;

#[test]
fn parse_account_transforms() {
    let source = include_str!("../../../examples/nexflow/transform/account-transform.xform");
    let program = parse_transform(source).expect("Failed to parse account-transform.xform");

    // 4 field-level transforms + 1 transform_block
    assert_eq!(program.transforms.len(), 4);
    assert_eq!(program.transform_blocks.len(), 1);

    // -- normalize_name --
    let norm = &program.transforms[0];
    assert_eq!(norm.name, "normalize_name");
    assert_eq!(norm.version.as_deref(), Some("1.0.0"));
    assert!(norm.description.is_some());
    assert_eq!(norm.pure, Some(true));
    assert_eq!(norm.inputs.len(), 1);
    assert!(matches!(norm.inputs[0].field_type, TransformFieldType::Base { ref name, .. } if name == "string"));
    assert_eq!(norm.outputs.len(), 1);
    assert!(matches!(norm.outputs[0].field_type, TransformFieldType::Base { ref name, .. } if name == "string"));
    assert_eq!(norm.apply.len(), 1);

    // -- calculate_balance (multi-input) --
    let calc = &program.transforms[1];
    assert_eq!(calc.name, "calculate_balance");
    assert_eq!(calc.inputs.len(), 3);
    assert_eq!(calc.outputs.len(), 1);
    assert!(matches!(calc.outputs[0].field_type, TransformFieldType::Base { ref name, .. } if name == "decimal"));
    assert_eq!(calc.apply.len(), 1);

    // -- format_account_summary (transform_block with mappings) --
    let block = &program.transform_blocks[0];
    assert_eq!(block.name, "format_account_summary");
    assert_eq!(block.inputs.len(), 1);
    // Input type is a schema reference (account_detail)
    assert!(matches!(block.inputs[0].field_type, TransformFieldType::Reference(ref name) if name == "account_detail"));
    assert_eq!(block.outputs.len(), 1);
    assert_eq!(block.mappings.len(), 7);

    // -- validate_transfer_amount (with validate_input) --
    let validate = &program.transforms[2];
    assert_eq!(validate.name, "validate_transfer_amount");
    assert_eq!(validate.inputs.len(), 2);
    assert_eq!(validate.outputs.len(), 2);
    assert_eq!(validate.validate_input.len(), 2);

    // -- safe_division (with on_error) --
    let safe = &program.transforms[3];
    assert_eq!(safe.name, "safe_division");
    assert!(safe.on_error.is_some());
    let err = safe.on_error.as_ref().unwrap();
    assert_eq!(err.action.as_deref(), Some("use_default"));
}

#[test]
fn parse_comprehensive_transforms() {
    let source = include_str!("../../../examples/nexflow/transform/comprehensive-transform.xform");
    let program = parse_transform(source).expect("Failed to parse comprehensive-transform.xform");

    // 5 transforms + 3 transform_blocks
    assert_eq!(program.transforms.len(), 5, "transforms count");
    assert_eq!(program.transform_blocks.len(), 3, "transform_blocks count");

    // -- validate_account_input (qualifiers on fields) --
    let vai = &program.transforms[0];
    assert_eq!(vai.name, "validate_account_input");
    assert_eq!(vai.pure, Some(true));
    assert_eq!(vai.idempotent, Some(true));
    assert_eq!(vai.inputs.len(), 4);

    // Required field
    assert!(vai.inputs[0].required, "account_name should be required");
    // Nullable field with default
    assert!(vai.inputs[3].nullable, "currency should be nullable");
    assert!(vai.inputs[3].default.is_some(), "currency should have default");

    // -- aggregate_transactions (collection types) --
    let agg = &program.transforms[1];
    assert_eq!(agg.name, "aggregate_transactions");
    assert_eq!(agg.inputs.len(), 1);
    // Input should be list<transaction_event>
    match &agg.inputs[0].field_type {
        TransformFieldType::Collection { kind, element_types } => {
            assert_eq!(*kind, TransformCollectionKind::List);
            assert_eq!(element_types.len(), 1);
        }
        other => panic!("Expected collection type, got {other:?}"),
    }

    // -- enrich_with_exchange_rate (lookups, state) --
    let enrich = &program.transforms[2];
    assert_eq!(enrich.name, "enrich_with_exchange_rate");
    assert_eq!(enrich.pure, Some(false));
    assert!(enrich.lookup.is_some(), "should have lookup");
    assert_eq!(enrich.lookups.len(), 2, "should have 2 lookups");
    assert!(enrich.state.is_some(), "should have state");
    assert_eq!(enrich.on_error.as_ref().unwrap().error_code.as_deref(), Some("EXCHANGE_RATE_UNAVAILABLE"));

    // -- apply_discount (params with defaults) --
    let discount = &program.transforms[3];
    assert_eq!(discount.name, "apply_discount");
    assert_eq!(discount.params.len(), 3);
    assert!(discount.params[0].required);
    assert!(discount.params[0].default.is_some(), "discount_rate should have default");
    assert!(discount.params[1].optional);

    // -- cached_lookup (cache with ttl and key) --
    let cached = &program.transforms[4];
    assert_eq!(cached.name, "cached_lookup");
    let cache = cached.cache.as_ref().expect("should have cache");
    assert!(cache.ttl.is_some(), "cache should have ttl");
    assert_eq!(cache.key.len(), 2, "cache should have 2 key fields");

    // -- process_order (compose sequential, validate, invariant, use, on_error) --
    let process = &program.transform_blocks[0];
    assert_eq!(process.name, "process_order");
    assert!(!process.uses.is_empty(), "should have uses");
    assert!(!process.validate_input.is_empty(), "should have validate_input");
    assert!(!process.invariants.is_empty(), "should have invariants");
    let compose = process.compose.as_ref().expect("should have compose");
    assert_eq!(compose.compose_type.as_deref(), Some("sequential"));
    assert_eq!(compose.refs.len(), 3);
    assert!(process.on_error.is_some());
    assert_eq!(process.on_error.as_ref().unwrap().action.as_deref(), Some("reject"));

    // -- map_account_fields (mappings + on_change) --
    let map_block = &program.transform_blocks[1];
    assert_eq!(map_block.name, "map_account_fields");
    assert_eq!(map_block.mappings.len(), 5);
    let on_change = map_block.on_change.as_ref().expect("should have on_change");
    assert_eq!(on_change.watch_fields.len(), 2);
    assert_eq!(on_change.recalculate.len(), 2);

    // -- route_by_type (conditional compose) --
    let route = &program.transform_blocks[2];
    assert_eq!(route.name, "route_by_type");
    let compose = route.compose.as_ref().expect("should have compose");
    assert_eq!(compose.compose_type.as_deref(), Some("conditional"));
    // Should have conditional refs + otherwise
    let conditional_count = compose.refs.iter().filter(|r| matches!(r, ComposeRef::Conditional { .. })).count();
    assert!(conditional_count >= 2, "should have at least 2 conditional refs");
    let has_otherwise = compose.refs.iter().any(|r| matches!(r, ComposeRef::Otherwise(_)));
    assert!(has_otherwise, "should have otherwise ref");
}
