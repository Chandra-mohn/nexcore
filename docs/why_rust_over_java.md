# Why Rust: Architectural Strengths for COBOL Transpilation

## Executive Summary

Choosing Rust as the transpilation target for cobol2rust was a deliberate architectural
decision, not a language preference. COBOL's memory model -- flat byte buffers, aliased
fields, packed binary representations, and silent truncation -- maps naturally to Rust's
ownership and type system. Languages with garbage collection and high-level numeric
abstractions (Java, C#, Python) allow implementations that appear correct on simple
programs but silently produce wrong results on the complex, decades-old code that matters
most.

This document explains why Rust's constraints are strengths for COBOL migration fidelity.


## The Core Problem: COBOL Has No Objects

COBOL does not have types in the modern sense. It has a flat byte buffer (Working Storage)
where fields are defined as views over contiguous memory. Two fields can occupy the same
bytes (REDEFINES). A GROUP record is simultaneously a structured record AND a flat
alphanumeric string. Numeric values are stored as packed BCD nibbles, zoned decimals,
or raw binary -- and the exact byte representation is observable by other fields.

Any transpilation target must faithfully reproduce this byte-level behavior. The question
is: does the target language help you get this right, or does it let you get it wrong?


## Where Java/JVM Would Paper Over Correctness

### 1. Shared Memory via REDEFINES

COBOL:
```cobol
01  WS-RECORD.
    05  WS-AMOUNT    PIC 9(8).
01  WS-RECORD-ALT   REDEFINES WS-RECORD.
    05  WS-AMT-STR   PIC X(8).
```

Write `12345678` to `WS-AMOUNT`, read `WS-AMT-STR` -- you get ASCII bytes `"12345678"`.
Same physical memory, two views. Write through one view, read through the other. This is
not a corner case; real COBOL programs depend on this constantly.

**Java risk**: The natural Java approach uses separate objects (`BigDecimal` for the
numeric view, `String` for the alpha view). GC manages lifetimes, so there is no
compiler error when the implementation *copies* instead of *shares* memory. Tests pass
on simple cases. Production fails on the 30-year-old code that writes through one
REDEFINES view and reads through another.

**Rust advantage**: Our `RedefinesGroup` shares a `Vec<u8>` byte buffer. The borrow
checker enforces that aliased access follows Rust's ownership rules. Incorrect aliasing
is a compile-time error, not a runtime surprise. The type system makes the "two views,
one memory" invariant structural.


### 2. PackedDecimal Byte-Level Storage

COBOL COMP-3 stores digits as BCD nibbles: `+123` becomes bytes `0x12 0x3C` (two digits
per byte, trailing sign nibble). The exact byte representation matters because:

- Other fields can REDEFINE over packed storage
- GROUP MOVEs copy raw bytes, not logical values
- File I/O writes the raw bytes to disk; partner systems parse them

**Java risk**: `BigDecimal` stores the correct *value* but has no byte-level
representation. When another field reads those bytes via REDEFINES, or when a GROUP
MOVE copies the containing record, the Java version needs a separate byte buffer. Now
there are two sources of truth (the `BigDecimal` value and the byte buffer) that can
drift out of sync. Every mutation must update both, and forgetting one is silent.

**Rust advantage**: Our `PackedDecimal` IS the byte buffer. The `pack()` method writes
BCD nibbles directly. `to_decimal()` reads them back. There is one source of truth. When
a REDEFINES reads the same bytes, it reads the actual packed representation. When a GROUP
MOVE copies the parent record, it copies the real bytes. No sync problem exists because
there is nothing to sync.


### 3. Silent Left-Truncation

```cobol
MOVE 12345 TO WS-SMALL.    *> PIC 9(3) receives 345, not 123
```

COBOL truncates from the LEFT. This is not a bug; it is specified behavior that real
programs depend on (e.g., extracting the last N digits of an account number).

**Java risk**: `BigDecimal` preserves full precision. This is mathematically "better"
but produces wrong COBOL behavior. Every assignment must add explicit truncation logic:
`value.remainder(BigDecimal.TEN.pow(targetDigits))`. Missing one truncation point is
silent -- the value is just "too precise," which can cascade through subsequent
arithmetic and comparisons. The failure mode is *subtly wrong numbers*, not crashes.

**Rust advantage**: Our `PackedDecimal`, `ZonedDecimal`, and display-numeric types have
truncation baked into `pack()` and `store()`. The type's precision (declared at
construction from the PIC clause) is enforced on every mutation. You cannot store a
value without truncation being applied. The truncation behavior is a property of the
type, not a discipline imposed on every call site.


### 4. GROUP MOVE (Flat Byte Copy)

```cobol
MOVE WS-EMPLOYEE TO WS-DISPLAY-REC.
```

COBOL copies raw bytes from source to destination. The receiver gets the exact byte
representation, including packed decimals, binary integers, and FILLER bytes. This is
NOT field-by-field value copying.

**Java risk**: A naive Java implementation copies field-by-field using
`field.toString()` or `field.toBigDecimal()`. This gives different results when the
GROUP contains packed decimals (whose byte representation differs from their display
representation) or binary COMP fields (whose byte representation is big-endian binary,
not ASCII digits). The test passes with all-alphanumeric groups and fails on mixed-type
groups.

**Rust advantage**: GROUP MOVE in our runtime is `dest_bytes.copy_from_slice(src_bytes)`.
Since every field type stores its actual COBOL byte representation, the byte copy IS the
correct behavior. There is no translation layer to get wrong.


### 5. Unsigned Fields Receiving Negative Values

```cobol
01  WS-SIGNED    PIC S9(3) VALUE -50.
01  WS-UNSIGNED  PIC 9(3).
MOVE WS-SIGNED TO WS-UNSIGNED.     *> Result: 050 (sign stripped)
```

COBOL silently strips the sign. The unsigned field receives the absolute value,
truncated to fit.

**Java risk**: `BigDecimal` carries the sign. Assigning `-50` to a variable that
"should be unsigned" requires explicit `abs()` + truncation at every MOVE target.
Java has no concept of an unsigned decimal with fixed precision. Every assignment is a
potential miss.

**Rust advantage**: Our `PackedDecimal::new(precision, scale, signed)` carries the
signedness as a type-level property. The `pack()` method handles sign stripping
automatically when storing into an unsigned field. The behavior is structural, not
discretionary.


## Quantified Comparison

| Aspect | Java/JVM approach | Rust/cobol2rust approach |
|--------|-------------------|--------------------------|
| Memory aliasing (REDEFINES) | Runtime discipline, no compiler help | Borrow checker enforces correctness |
| Byte-level storage | Dual representation (object + bytes) | Single representation (bytes ARE the type) |
| Truncation semantics | Manual at every call site | Baked into type's pack/store methods |
| GROUP MOVE fidelity | Requires serialization layer | Direct byte copy (zero translation) |
| Sign handling | Manual abs() + truncation | Type property (signed flag at construction) |
| Failure mode | Silent wrong values in production | Compile-time errors during development |
| Performance | GC pauses, object allocation per field | Zero-cost abstractions, stack allocation |


## The 90/10 Problem

Java-based COBOL transpilers work for ~90% of programs. The remaining 10% -- which
includes complex, hand-maintained programs with decades of patches -- relies on exact
byte-level behavior that Java's abstractions silently discard.

For the cobol2rust client, the critical files are exactly in this 10%: ~1M and ~500K
line programs, hand-written over 30 years, with deep REDEFINES chains, mixed-type
GROUP MOVEs, and arithmetic that depends on truncation behavior. These files ARE the
project. A transpiler that gets them wrong has zero value regardless of how well it
handles the simple cases.

Rust does not make COBOL transpilation easy. It makes it *correct*, by turning the hard
problems into compile-time constraints rather than runtime hopes.


## Codebase Evidence

The cobol2rust runtime is 48K lines of hand-written Rust across 10 crates. The type
system (cobol-types, 4,634 lines) explicitly models:

- `PackedDecimal` -- BCD nibble storage with precision/scale/sign
- `ZonedDecimal` -- zone-digit pairs with EBCDIC sign embedding
- `PicX` -- fixed-length byte buffer with space-padding semantics
- `CompBinary` -- big-endian binary with PIC-limited vs full-range modes
- `RedefinesGroup` -- shared byte buffer for aliased field access
- `VarArray` -- 1-based indexing with OCCURS DEPENDING ON bounds

Each type enforces COBOL semantics at the Rust type level. The generated code cannot
bypass truncation, ignore sign rules, or break memory sharing -- because the types do
not permit it.


## Conclusion

Rust was chosen not because it is trendy or fast, but because its type system and
ownership model align with COBOL's fundamental memory semantics in ways that GC-based
languages cannot match. The constraints Rust imposes -- ownership, borrowing, explicit
mutability, no implicit conversions -- are exactly the constraints needed to faithfully
reproduce COBOL's flat-memory, byte-aliased, silently-truncating execution model.

The architectural bet: pay the complexity cost upfront in the type system, so that
correctness is structural rather than aspirational. For enterprise COBOL migration where
"almost correct" has zero value, this trade-off is definitively worth it.
