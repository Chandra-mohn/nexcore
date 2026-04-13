# cobol-move

COBOL MOVE engine. Type coercion, truncation, padding, MOVE CORRESPONDING.

## Key Functions

- `cobol_move(src, dst, config)` -- Main MOVE handler (dispatches by type pair)
- `cobol_move_numeric(src, dst, config)` -- Numeric-to-numeric with rounding/truncation
- `move_alphanumeric_literal(bytes, dst)` -- Literal string MOVE
- `move_numeric_literal(decimal, dst)` -- Literal numeric MOVE
- `is_legal_move(src_cat, dst_cat)` -- Category compatibility check
- `move_corresponding(src, dst)` -- MOVE CORRESPONDING (name matching)
- `cobol_initialize(field)` -- INITIALIZE verb

## Modules

- `engine.rs` -- Core MOVE dispatch and type coercion
- `corresponding.rs` -- MOVE CORRESPONDING implementation
- `initialize.rs` -- INITIALIZE verb
- `diagnostics.rs` -- Truncation/padding warnings

## Dependencies

cobol-core, cobol-types, rust_decimal

## Rules

- MOVE is the most common COBOL verb -- correctness is critical
- Alphanumeric: left-justified, right-padded with spaces, right-truncated
- Numeric: right-justified, left-padded with zeros, left-truncated on overflow
- GROUP MOVE = alphanumeric move of the entire group as bytes
- Generated code calls these functions directly (not trait methods)
