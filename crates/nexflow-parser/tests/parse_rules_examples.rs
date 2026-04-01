// NexCore -- Nexflow Parser: RulesDSL Integration Tests
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

use nexflow_parser::ast::rules::*;
use nexflow_parser::parse_rules;

#[test]
fn parse_credit_rules() {
    let source = include_str!("../../../examples/nexflow/rules/credit-rules.rules");
    let program = parse_rules(source).expect("Failed to parse credit-rules.rules");

    // 2 decision tables + 1 procedural rule
    assert_eq!(program.decision_tables.len(), 2);
    assert_eq!(program.procedural_rules.len(), 1);

    // -- credit_scoring decision table --
    let scoring = &program.decision_tables[0];
    assert_eq!(scoring.name, "credit_scoring");
    assert_eq!(scoring.hit_policy, Some(HitPolicy::FirstMatch));
    assert!(scoring.description.is_some());
    assert_eq!(scoring.version.as_deref(), Some("1.0.0"));

    // Given inputs
    assert_eq!(scoring.inputs.len(), 3);
    assert_eq!(scoring.inputs[0].name, "income");
    assert_eq!(scoring.inputs[0].param_type, "number");
    assert_eq!(scoring.inputs[1].name, "credit_history");
    assert_eq!(scoring.inputs[1].param_type, "text");

    // Table columns
    assert!(scoring.columns.len() >= 5);

    // Table rows (6 data rows)
    assert_eq!(scoring.rows.len(), 6);

    // First row: >= 100000 | "excellent" | < 10000 | "gold" | 50000
    let row0 = &scoring.rows[0];
    assert_eq!(row0.cells.len(), 5);
    // First cell: comparison condition >= 100000
    assert!(
        matches!(&row0.cells[0], CellContent::Condition(ConditionExpr::Comparison { .. })),
        "First cell should be comparison, got {:?}", row0.cells[0]
    );
    // Second cell: exact match "excellent"
    assert!(
        matches!(&row0.cells[1], CellContent::Condition(ConditionExpr::ExactMatch(_))),
        "Second cell should be exact match, got {:?}", row0.cells[1]
    );
    // Third cell: comparison < 10000
    assert!(
        matches!(&row0.cells[2], CellContent::Condition(ConditionExpr::Comparison { .. })),
        "Third cell should be comparison, got {:?}", row0.cells[2]
    );

    // Wildcard row: * | "poor" | * | "denied" | 0
    let row3 = &scoring.rows[3];
    assert!(
        matches!(&row3.cells[0], CellContent::Wildcard),
        "First cell of row 3 should be wildcard"
    );
    assert!(
        matches!(&row3.cells[2], CellContent::Wildcard),
        "Third cell of row 3 should be wildcard"
    );

    // Return params
    assert_eq!(scoring.returns.len(), 2);
    assert_eq!(scoring.returns[0].name, "tier");
    assert_eq!(scoring.returns[1].name, "limit");

    // -- transfer_validation procedural rule --
    let transfer = &program.procedural_rules[0];
    assert_eq!(transfer.name, "transfer_validation");
    assert!(transfer.description.is_some());
    assert!(!transfer.body.is_empty());

    // Should have at least one if-then-else block
    let has_if = transfer.body.iter().any(|item| {
        matches!(item, BlockItem::IfThenElse { .. })
    });
    assert!(has_if, "transfer_validation should have if-then-else");

    // -- fee_calculation decision table (multi_hit) --
    let fees = &program.decision_tables[1];
    assert_eq!(fees.name, "fee_calculation");
    assert_eq!(fees.hit_policy, Some(HitPolicy::MultiHit));
    assert_eq!(fees.inputs.len(), 3);
    assert_eq!(fees.rows.len(), 4);
}
