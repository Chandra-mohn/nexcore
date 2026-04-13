# DSL Emission Staging: When to Run Which Emitters

## The Insight

The DSL emitters read transpiled Rust. But Phase 1 Rust (raw transpiler output)
and Phase 2 Rust (after Tier 1-4 rustification) are very different:

```
Phase 1 Rust (transpiler output):
    cobol_add("100".parse::<Decimal>().unwrap(), &mut ws.ws_count, None, &ctx.config);
    move_alphanumeric_literal(b"HIGH", &mut ws.ws_result, &ctx.config);
    if ws.ws_a.to_decimal() > "75".parse::<Decimal>().unwrap() { ... }

Phase 2 Rust (after Tier 2 rustification):
    ws.count += 100;
    ws.result = "HIGH".to_string();
    if ws.a > 75 { ... }
```

Expression extraction from Phase 1 means fighting the transpiler's encoding
(cobol_add, move_alphanumeric_literal, .parse::<Decimal>().unwrap()). From
Phase 2, it's standard Rust that any syn-based analyzer can read trivially.

## Two-Stage DSL Emission

### Stage 1: Structure (run on Phase 1 Rust)

What works well on raw transpiler output:

| Emitter | What's Extracted | Source |
|---|---|---|
| SchemaEmitter | Entity decomposition, field types, level-88 constraints | #[cobol(...)] attributes on struct fields |
| RulesEmitter (structure) | Decision table shape, if/else chain branching | syn body analysis: match arms, if/else branches |
| RulesEmitter (conditions) | Comparison expressions (>, ==, >=, and, or) | syn BinOp on if conditions -- these ARE standard Rust |
| RulesEmitter (actions) | move_alphanumeric_literal -> set field = "TEXT" | Pattern-matched function calls |
| ProcessEmitter | Call graph, section structure, I/O detection | #[cobol(performs, reads, writes)] attributes |
| TransformEmitter (structure) | Function classification, compose blocks, I/O specs | #[cobol(...)] attributes + function body shape |

What does NOT work well:

| Emitter | What's Missing | Why |
|---|---|---|
| TransformEmitter (apply) | Arithmetic expressions (ADD, SUBTRACT, COMPUTE) | cobol_add() / cobol_compute() are not standard Rust |
| TransformEmitter (mappings) | Field-to-field mappings from MOVE | cobol_move() / move_numeric_literal() encoding |

### Stage 2: Expressions (run on Phase 2 Rust, after Tier 2)

After Tier 2 rustification promotes types (PackedDecimal -> Decimal, PicX -> String)
and simplifies runtime calls (cobol_add -> +=), the same emitters produce richer output:

| Emitter | What Improves | Why |
|---|---|---|
| TransformEmitter (apply) | `ws.count += 100` -> `count = count + 100` | Standard Rust arithmetic, trivial to extract |
| TransformEmitter (mappings) | `ws.result = "HIGH".to_string()` -> `result = "HIGH"` | Standard Rust assignment |

Everything else (schema, rules, process) stays the same -- the structural
information doesn't change between Phase 1 and Phase 2.

## CLI Usage

```bash
# Stage 1: structural DSL from raw transpiler output
cobol2rust rustify transpiled/ --emit-dsl
# Produces: .rustify/dsl/ with schemas, rules (with conditions), process, transforms (structural)

# Stage 2: full DSL from rustified output
cobol2rust rustify transpiled/ -o rustified/ --tier 2 --emit-dsl
# Produces: rustified/.rustify/dsl/ with enhanced transform apply/mapping blocks
```

## Implication for Transform Expression Extraction

Do NOT invest in extracting expressions from Phase 1 patterns like:
- cobol_add() / cobol_subtract() / cobol_multiply() / cobol_divide()
- cobol_compute() / operand_to_decimal_expr()
- move_numeric_literal() / move_alphanumeric_literal()

These patterns will be eliminated by Tier 2 rustification. The expression
extraction engine (expr_extract.rs) should focus on:
1. Standard Rust comparisons (already done -- if/else conditions)
2. Standard Rust assignments (Phase 2: ws.field = value)
3. Standard Rust arithmetic (Phase 2: ws.field += value)

## Current Status

Stage 1 is DONE. The emitters produce grammar-valid DSL with:
- Full structural information from attributes
- Extracted decision conditions and actions (rules)
- Placeholder apply/mapping blocks (transforms) -- marked with notes

Stage 2 depends on Tier 2 rustification being applied first.
Track progress in docs/emitter_capabilities.md.
