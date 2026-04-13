# cobol-transpiler

COBOL-to-Rust transpiler. ANTLR4 parser, typed AST, symbol table, Rust code generation.

## Architecture

```
COBOL source
  -> [Preprocessor] fixed-format handling + COPY/REPLACE expansion
  -> [ANTLR4 Lexer/Parser] tokens -> parse tree
  -> [AST Builder] typed AST (Program, DataDivision, ProcedureDivision)
  -> [Symbol Resolution] ConditionMap, RecordFileMap, ParagraphFieldAccess
  -> [Rust Codegen] Phase 1 Rust (COBOL-shaped)
```

Single-pass: AST -> Rust code directly (no IR).

## Source Structure

```
src/
  codegen/
    mod.rs            -- Codegen orchestration
    rust_writer.rs    -- Rust syntax output
    data_gen.rs       -- DATA DIVISION -> struct fields
    proc_gen.rs       -- PROCEDURE DIVISION -> fn body (79 functions)
    field_analysis.rs -- ParagraphFieldAccess tracking
    attributes.rs     -- #[cobol(...)] annotation generation
  parser/
    mod.rs, copybook.rs, copy_expand.rs     -- COPY expansion
    data_listener.rs, file_listener.rs      -- DATA/FILE SECTION
    proc_listener.rs                        -- PROCEDURE DIVISION
    pic_parser.rs, hierarchy.rs             -- PIC clause, level hierarchy
    preprocess.rs, sql_extract.rs           -- Preprocessing, EXEC SQL
  generated/          -- ANTLR4 generated (DO NOT EDIT)
  ast.rs              -- Typed AST definitions
  symbol_table.rs     -- Symbol resolution and scoping
  transpile.rs        -- Top-level transpile() entry point
  diagnostics.rs, error.rs, hints.rs
```

## Precomputed Maps (built before codegen)

- ConditionMap: level-88 conditions -> inline comparisons
- RecordFileMap: record name -> file name
- SortFieldMap: field -> (offset, length)
- GroupChildMap: group -> [(child, is_numeric)]
- ParagraphFieldAccess: reads/writes/performs per paragraph

## Codegen Patterns

- PackedDecimal: `PackedDecimal::new(prec, scale, signed)` then `.pack()`
- PicX: `PicX::new(length, b"value")`
- Arithmetic: `cobol_add(&op, &mut tgt, rnd, cfg)` via trait objects
- COMPUTE: `cobol_compute(expr, &mut tgt, rnd, cfg)`
- MOVE: `move_numeric_literal()`, `move_alphanumeric_literal()`, `cobol_move()`
- No dec!() in generated code; use `.parse::<Decimal>().unwrap()`

## COBOL Quirks (critical for correctness)

- Overflow = silent LEFT-truncation (12345 -> PIC 9(3) = 345)
- COMP vs COMP-5: same storage, different range limits
- 1-based array indexing everywhere
- GROUP items: dual nature (structured record AND flat alphanumeric)
- NEXT SENTENCE != CONTINUE
- IBM COBOL: NO short-circuit evaluation (both AND/OR sides always evaluated)
- Paragraph fall-through: sequential unless redirected
- CALL BY REFERENCE is default; aliasing same var to multiple params is legal

## Testing

- 47 stress tests; 46 pass (realistic_batch needs data files)
- COBOL sources: /Users/chandramohn/workspace/cobol2rust/cobol/
- MANDATORY: run stress tests one at a time, NEVER bulk-build all 47
- `cargo test --features compile_tests` to enable

## Dependencies

antlr-rust, thiserror, miette, serde_json
