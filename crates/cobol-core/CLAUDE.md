# cobol-core

Foundation crate. Traits, config, EBCDIC, numeric parsing, and error types.
Upstream dependency for ALL other crates -- changes here ripple everywhere.

## Key Modules

- `traits.rs` -- CobolField, CobolGroup, CobolNumeric, CobolNumericEdited
- `config.rs` -- RuntimeConfig, CobolDialect, ArithMode, RoundingMode, NumProc
- `error.rs` -- ArithError, CallError, CobolError, DataError, FileError, SortError
- `ebcdic.rs` -- CodePage for EBCDIC/ASCII conversion
- `category.rs` -- DataCategory enum (Alphabetic, Alphanumeric, Numeric, etc.)
- `decimal_ext.rs` -- Decimal extension methods
- `numeric_parse.rs` -- Numeric string parsing utilities
- `editing.rs` -- EditSymbol for PIC clause editing patterns

## Dependencies

External only: rust_decimal, thiserror, serde. No workspace crate deps.

## Rules

- This is the foundation -- keep it minimal and stable
- All error types live here (other crates use `#[from]` to wrap)
- Traits define the runtime polymorphism contract
- Changes to traits affect ALL downstream crates
