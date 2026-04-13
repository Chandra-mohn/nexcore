# cobol-arithmetic

COBOL arithmetic verbs: ADD, SUBTRACT, MULTIPLY, DIVIDE, COMPUTE.

## Key Functions

- `cobol_add(op, target, rounded, config)` / `cobol_add_giving(...)`
- `cobol_subtract(op, target, rounded, config)` / `cobol_subtract_giving(...)`
- `cobol_multiply(op, target, rounded, config)` / `cobol_multiply_giving(...)`
- `cobol_divide(op, target, rounded, config)` / `cobol_divide_giving(...)`
- `cobol_compute(expr, target, rounded, config)` -- Expression evaluation
- `cobol_checked_div(a, b)` -- Division with ON SIZE ERROR
- `store_arithmetic_result(result, target, rounded, config)` -- Store with rounding

## Modules

- `add.rs`, `subtract.rs`, `multiply.rs`, `divide.rs` -- Verb implementations
- `compute.rs` -- COMPUTE expression evaluation
- `result.rs` -- ArithResult type (value + overflow info)
- `store.rs` -- Result storage with ROUNDED/ON SIZE ERROR

## Dependencies

cobol-core, cobol-types, rust_decimal

## Rules

- All arithmetic uses rust_decimal::Decimal for precision
- ROUNDED clause: half-even rounding by default (configurable in RuntimeConfig)
- ON SIZE ERROR: overflow detection via ArithResult
- COMPUTE compiles complex expressions to nested arithmetic calls
- Generated code: `cobol_add(&op, &mut tgt, rnd, cfg)` pattern
