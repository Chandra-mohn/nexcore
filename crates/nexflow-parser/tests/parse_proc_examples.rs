// NexCore -- Nexflow Parser: ProcDSL Integration Tests
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

use nexflow_parser::ast::proc::*;
use nexflow_parser::parse_proc;

#[test]
fn parse_order_processing() {
    let source = include_str!("../../../examples/nexflow/proc/order-processing.proc");
    let program = parse_proc(source).expect("Failed to parse order-processing.proc");

    // 1 import
    assert_eq!(program.imports.len(), 1);

    // 2 processes
    assert_eq!(program.processes.len(), 2);

    // -- order_processing --
    let order = &program.processes[0];
    assert_eq!(order.name, "order_processing");

    // Should have body statements (receive, emit)
    assert!(
        !order.body.is_empty(),
        "order_processing should have body statements"
    );

    // Check for receive statement
    let has_receive = order.body.iter().any(|s| matches!(s, ProcessStatement::Receive { .. }));
    assert!(has_receive, "Should have a receive statement");

    // Check for emit statement
    let has_emit = order.body.iter().any(|s| matches!(s, ProcessStatement::Emit { .. }));
    assert!(has_emit, "Should have an emit statement");

    // -- daily_settlement --
    let settlement = &program.processes[1];
    assert_eq!(settlement.name, "daily_settlement");
    assert!(!settlement.body.is_empty());
}
