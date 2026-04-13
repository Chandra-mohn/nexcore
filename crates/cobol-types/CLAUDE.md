# cobol-types

COBOL data type implementations. Every PIC clause variant has a Rust struct here.

## Key Types

- `PackedDecimal` -- COMP-3 (BCD nibbles), most common numeric storage
- `PicX` -- Alphanumeric (PICTURE X), fixed-length byte array
- `PicA` -- Alphabetic (PICTURE A)
- `ZonedDecimal` -- Display numeric (PICTURE 9)
- `CompBinary` -- COMP/COMP-5 binary encoding
- `Comp1Float`, `Comp2Float` -- IEEE floating point (COMP-1, COMP-2)
- `NumericEdited` -- Edited numeric display (Z, *, CR, DB patterns)
- `AlphanumericEdited` -- Edited alphanumeric
- `CobolArray` -- OCCURS clause (fixed-length)
- `CobolVarArray` -- OCCURS DEPENDING ON (variable-length)
- `Level88Predicate` -- Level-88 condition names
- `FigurativeConstant` -- ZERO, SPACE, HIGH-VALUE, LOW-VALUE, ALL
- `RedefinesGroup` -- REDEFINES overlay synchronization

## Dependencies

cobol-core, rust_decimal, byteorder, thiserror

## Rules

- All types implement CobolField and CobolNumeric traits from cobol-core
- PackedDecimal: pack()/unpack() for BCD encoding; precision and scale are immutable
- PicX: fixed-length, right-padded with spaces; byte-level operations
- COBOL overflow = silent LEFT-truncation (12345 into PIC 9(3) = 345)
- 1-based array indexing (COBOL convention)
- Tests use dec!() macro for Decimal literals
