// NexCore -- Nexflow Parser: NexQueryDSL Integration Tests
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

use nexflow_parser::ast::nexquery::*;
use nexflow_parser::parse_nexquery;

#[test]
fn parse_investigation_queries() {
    let source = include_str!("../../../examples/nexflow/query/investigation.nxq");
    let script = parse_nexquery(source).expect("Failed to parse investigation.nxq");

    // Should have multiple statements (each terminated by ;)
    assert!(
        script.statements.len() >= 10,
        "Expected at least 10 statements, got {}",
        script.statements.len()
    );

    // First statement: programs calling CLRG0100
    let s0 = &script.statements[0];
    assert_eq!(s0.clauses.len(), 1);
    match &s0.clauses[0] {
        NexQueryClause::Traverse(tc) => {
            assert_eq!(tc.node_type, NodeType::Programs);
            assert_eq!(tc.verb, TraversalVerb::Calling);
            assert!(matches!(&tc.target, Target::Identifier(name) if name == "CLRG0100"));
            assert!(tc.filter.is_none());
        }
        other => panic!("Expected traverse clause, got {other:?}"),
    }

    // Second statement: programs calling CLRG0100(type = 'batch')
    let s1 = &script.statements[1];
    match &s1.clauses[0] {
        NexQueryClause::Traverse(tc) => {
            assert!(tc.filter.is_some(), "Should have filter");
            let filter = tc.filter.as_ref().unwrap();
            match &filter.expr {
                FilterExpr::Predicate(pred) => {
                    assert_eq!(pred.field, "type");
                    assert_eq!(pred.op, CompareOp::Eq);
                    assert!(matches!(&pred.value, QueryValue::String(s) if s == "batch"));
                }
                other => panic!("Expected predicate, got {other:?}"),
            }
        }
        other => panic!("Expected traverse, got {other:?}"),
    }

    // Third statement: multi-clause pipeline (traverse + expand)
    let s2 = &script.statements[2];
    assert_eq!(s2.clauses.len(), 2, "Pipeline should have 2 clauses");
    assert!(matches!(&s2.clauses[0], NexQueryClause::Traverse(_)));
    assert!(matches!(&s2.clauses[1], NexQueryClause::Expand(_)));

    // Fourth statement: traverse with list target
    let s3 = &script.statements[3];
    match &s3.clauses[0] {
        NexQueryClause::Traverse(tc) => {
            match &tc.target {
                Target::List(items) => {
                    assert_eq!(items.len(), 3);
                    assert_eq!(items[0], "CLRG0100");
                }
                other => panic!("Expected list target, got {other:?}"),
            }
        }
        other => panic!("Expected traverse, got {other:?}"),
    }

    // Fifth statement: rank verb with modifiers
    let s4 = &script.statements[4];
    match &s4.clauses[0] {
        NexQueryClause::Verb(vc) => {
            assert_eq!(vc.verb, DomainVerb::Rank);
            assert!(matches!(&vc.target, Some(Target::Identifier(name)) if name == "programs"));
            assert!(vc.modifiers.len() >= 2);
            assert_eq!(vc.modifiers[0].keyword, ModifierKeyword::By);
            assert_eq!(vc.modifiers[1].keyword, ModifierKeyword::Top);
        }
        other => panic!("Expected verb clause, got {other:?}"),
    }
}

#[test]
fn parse_simple_queries() {
    // Simple traverse
    let script = parse_nexquery("programs calling CLRG0100;")
        .expect("Failed to parse");
    assert_eq!(script.statements.len(), 1);
    assert_eq!(script.statements[0].clauses.len(), 1);

    // Simple expand
    let script = parse_nexquery("copybooks;")
        .expect("Failed to parse");
    assert_eq!(script.statements.len(), 1);
    match &script.statements[0].clauses[0] {
        NexQueryClause::Expand(ec) => {
            assert_eq!(ec.node_type, NodeType::Copybooks);
        }
        other => panic!("Expected expand, got {other:?}"),
    }

    // Domain verb
    let script = parse_nexquery("impact CPYCLRG;")
        .expect("Failed to parse");
    match &script.statements[0].clauses[0] {
        NexQueryClause::Verb(vc) => {
            assert_eq!(vc.verb, DomainVerb::Impact);
        }
        other => panic!("Expected verb, got {other:?}"),
    }

    // Find-dead (hyphenated verb)
    let script = parse_nexquery("find-dead level paragraph;")
        .expect("Failed to parse");
    match &script.statements[0].clauses[0] {
        NexQueryClause::Verb(vc) => {
            assert_eq!(vc.verb, DomainVerb::FindDead);
        }
        other => panic!("Expected verb, got {other:?}"),
    }
}

#[test]
fn parse_filters() {
    // Filter with comparison
    let script = parse_nexquery("programs calling CLRG0100(complexity > 3.0);")
        .expect("Failed to parse");
    match &script.statements[0].clauses[0] {
        NexQueryClause::Traverse(tc) => {
            let filter = tc.filter.as_ref().expect("Should have filter");
            match &filter.expr {
                FilterExpr::Predicate(pred) => {
                    assert_eq!(pred.field, "complexity");
                    assert_eq!(pred.op, CompareOp::Gt);
                    assert!(matches!(&pred.value, QueryValue::Number(n) if (*n - 3.0).abs() < 0.01));
                }
                other => panic!("Expected predicate, got {other:?}"),
            }
        }
        other => panic!("Expected traverse, got {other:?}"),
    }

    // Filter with glob
    let script = parse_nexquery("programs calling CLRG0100(name ~ 'CLR*');")
        .expect("Failed to parse");
    match &script.statements[0].clauses[0] {
        NexQueryClause::Traverse(tc) => {
            let filter = tc.filter.as_ref().unwrap();
            match &filter.expr {
                FilterExpr::Predicate(pred) => {
                    assert_eq!(pred.op, CompareOp::Glob);
                }
                _ => panic!("Expected predicate"),
            }
        }
        _ => panic!("Expected traverse"),
    }
}
