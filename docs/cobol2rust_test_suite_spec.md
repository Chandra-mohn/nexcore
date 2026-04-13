# cobol2rust Expanded Test Suite Specification

## Current State

| Category | Count | Description |
|----------|------:|-------------|
| Rust unit tests | 931 | In-crate #[test] functions across all crates |
| Language tests | 42 | cobol/language/**/*.cbl -- medium complexity |
| Volume tests | 5 | cobol/volume/*.cbl -- stress/scale tests |
| Rustify tests | 15 | cobol/rustify/*.cbl -- Phase 2 transformation |
| **Total COBOL fixtures** | **62** | |

### Gaps (from ProLeap analysis)

1. **No micro-tests** -- Smallest tests are 40+ lines mixing multiple features
2. **No parse-layer tests** -- We test transpile+compile+run but not AST correctness
3. **No codegen baselines** -- We don't diff generated Rust against expected output
4. **No negative tests** -- No tests for expected errors or unsupported features
5. **No per-verb isolation** -- Can't pinpoint which verb breaks when a test fails
6. **No variant coverage** -- 1 file per feature, not positive/negative/edge variants
7. **No output baselines** -- Expected program output not committed for regression


## Proposed Architecture

### Test Layers

```
Layer 1: Rust Unit Tests (existing 931)
  Tests runtime types and functions in isolation
  Fast, no COBOL parsing involved

Layer 2: Parse Tests (NEW)
  COBOL -> AST -> verify AST structure
  Tests parser correctness without codegen

Layer 3: Codegen Baseline Tests (NEW)
  COBOL -> Rust source -> diff against committed .rs baseline
  Catches codegen regressions without compilation

Layer 4: E2E Execution Tests (existing + expanded)
  COBOL -> transpile -> compile -> run -> diff output against baseline
  Full pipeline validation

Layer 5: Negative Tests (NEW)
  COBOL with known unsupported features -> verify meaningful error
  Tests error reporting quality
```

### Directory Structure

```
cobol/
  micro/                     # NEW: 5-15 line per-verb tests
    move/
      move_alpha_to_alpha.cbl
      move_alpha_truncate.cbl
      move_numeric_to_numeric.cbl
      move_numeric_left_truncate.cbl
      move_numeric_to_alpha.cbl
      move_alpha_to_numeric.cbl
      move_spaces.cbl
      move_zeros.cbl
      move_corresponding.cbl
    add/
      add_to.cbl
      add_giving.cbl
      add_multiple.cbl
      add_rounded.cbl
      add_size_error.cbl
      add_packed.cbl
      add_comp.cbl
    subtract/
      subtract_from.cbl
      subtract_giving.cbl
      subtract_size_error.cbl
    multiply/
      multiply_by.cbl
      multiply_giving.cbl
      multiply_rounded.cbl
      multiply_size_error.cbl
    divide/
      divide_into.cbl
      divide_giving.cbl
      divide_remainder.cbl
      divide_rounded.cbl
      divide_by_zero.cbl
    compute/
      compute_simple.cbl
      compute_precedence.cbl
      compute_parentheses.cbl
      compute_exponent.cbl
      compute_rounded.cbl
      compute_size_error.cbl
      compute_mixed_types.cbl
    if/
      if_simple.cbl
      if_else.cbl
      if_nested.cbl
      if_and_or.cbl
      if_not.cbl
      if_numeric.cbl
      if_alphabetic.cbl
    evaluate/
      evaluate_variable.cbl
      evaluate_true.cbl
      evaluate_thru.cbl
      evaluate_also.cbl
      evaluate_other.cbl
      evaluate_nested.cbl
    perform/
      perform_times.cbl
      perform_until.cbl
      perform_varying.cbl
      perform_varying_decrement.cbl
      perform_varying_by_step.cbl
      perform_test_after.cbl
      perform_nested.cbl
      perform_thru.cbl
      perform_paragraph.cbl
      perform_section.cbl
    display/
      display_numeric.cbl
      display_alpha.cbl
      display_packed.cbl
      display_comp.cbl
      display_edited.cbl
      display_concat.cbl
      display_literal.cbl
    string/
      string_simple.cbl
      string_pointer.cbl
      string_overflow.cbl
      string_delimited_spaces.cbl
    unstring/
      unstring_simple.cbl
      unstring_tallying.cbl
      unstring_multi_delim.cbl
    inspect/
      inspect_tallying_all.cbl
      inspect_tallying_leading.cbl
      inspect_replacing_all.cbl
      inspect_replacing_leading.cbl
      inspect_converting.cbl
    set/
      set_to_true.cbl
      set_up_by.cbl
      set_down_by.cbl
    initialize/
      initialize_simple.cbl
      initialize_replacing.cbl
    goto/
      goto_simple.cbl
      goto_depending.cbl
    search/
      search_sequential.cbl
      search_all_binary.cbl
    accept/
      accept_date.cbl
      accept_time.cbl
      accept_day.cbl
    types/
      pic9_display.cbl
      pic9_signed.cbl
      pic9_decimal.cbl
      picx_basic.cbl
      picx_spaces_value.cbl
      comp3_basic.cbl
      comp3_signed.cbl
      comp_binary.cbl
      comp5_binary.cbl
      comp1_float.cbl
      comp2_float.cbl
      zoned_decimal.cbl
      numeric_edited.cbl
      alpha_edited.cbl
      group_simple.cbl
      group_nested.cbl
      group_filler.cbl
      redefines_simple.cbl
      redefines_group.cbl
      occurs_fixed.cbl
      occurs_depending.cbl
      occurs_2d.cbl
      level88_basic.cbl
      level88_thru.cbl
      level88_multiple.cbl
      level77.cbl
      renames_basic.cbl
  language/                  # EXISTING: medium complexity
    ...                      # (42 files, keep as-is)
  rustify/                   # EXISTING: Phase 2 transformation
    ...                      # (15 files, keep as-is)
  volume/                    # EXISTING: stress/scale
    ...                      # (5 files, keep as-is)
  negative/                  # NEW: expected error cases
    unsupported_cics.cbl
    unsupported_report_writer.cbl
    missing_copybook.cbl
    invalid_pic_clause.cbl
    duplicate_paragraph.cbl
    circular_copy.cbl
    undeclared_variable.cbl
    type_mismatch_comp.cbl

baselines/                   # NEW: committed expected outputs
  micro/                     # Expected program output per micro-test
    move/
      move_alpha_to_alpha.expected
      ...
    add/
      add_to.expected
      ...
  language/                  # Expected output per language test
    ...
  rustify/                   # Expected output per rustify test
    ...
  codegen/                   # Expected generated Rust (Layer 3)
    micro/
      move/
        move_alpha_to_alpha.rs
        ...
```


## Micro-Test Design Rules

Each micro-test follows a strict template:

```cobol
       IDENTIFICATION DIVISION.
       PROGRAM-ID. <VERB>-<VARIANT>-TEST.
      *-------------------------------------------------------
      * Micro-test: <VERB> <variant description>
      * Tests: <specific behavior being tested>
      * Expected: <expected output, 1-3 lines>
      *-------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       <2-5 field declarations, minimal>
       PROCEDURE DIVISION.
       MAIN-PARA.
       <1-3 statements exercising the specific behavior>
           DISPLAY <result>.
           STOP RUN.
```

Properties:
- **5-15 lines** of non-comment COBOL
- **One behavior** per file (e.g., "ADD TO with size error")
- **1-3 DISPLAY lines** of output
- **Self-documenting**: Expected output in header comment
- **Deterministic**: VALUE clauses, no ACCEPT, no files
- **Independent**: No COPY, no CALL, no external dependencies


## Test Runner Infrastructure

### Layer 2: Parse Tests (Rust integration test)

```rust
// crates/cobol-transpiler/tests/parse_micro.rs

#[test]
fn parse_move_alpha_to_alpha() {
    let ast = parse_file("cobol/micro/move/move_alpha_to_alpha.cbl");
    assert!(ast.procedure_division.is_some());
    let stmts = &ast.procedure_division.unwrap().statements;
    assert_eq!(stmts.len(), 2); // MOVE + DISPLAY
    assert!(matches!(stmts[0], Statement::Move { .. }));
}
```

Scope: Verify AST structure, not values. Catches parser regressions.

### Layer 3: Codegen Baseline Tests (Rust integration test)

```rust
// crates/cobol-transpiler/tests/codegen_baseline.rs

#[test]
fn codegen_move_alpha_to_alpha() {
    let generated = transpile_to_string("cobol/micro/move/move_alpha_to_alpha.cbl");
    let expected = read_to_string("baselines/codegen/micro/move/move_alpha_to_alpha.rs");
    assert_eq!(normalize(generated), normalize(expected));
}
```

Scope: Diff generated Rust against committed baseline. Catches codegen drift.
Update baselines with: `BLESS=1 cargo test codegen_baseline`

### Layer 4: E2E Execution Tests (shell or Rust)

```bash
#!/bin/bash
# scripts/test_micro.sh -- run all micro E2E tests
PASS=0; FAIL=0
for cbl in cobol/micro/**/*.cbl; do
    name=$(basename "$cbl" .cbl)
    dir=$(dirname "$cbl" | sed 's|cobol/micro/||')
    expected="baselines/micro/$dir/${name}.expected"

    # Transpile
    cobol2rust transpile "$cbl" -o "/tmp/micro_test/${name}.rs" || { FAIL=$((FAIL+1)); continue; }

    # Build
    # (setup Cargo.toml, build)

    # Run and diff
    cargo run --release 2>/dev/null > "/tmp/micro_test/${name}.actual"
    if diff -q "$expected" "/tmp/micro_test/${name}.actual" > /dev/null 2>&1; then
        PASS=$((PASS+1))
    else
        echo "FAIL: $cbl"
        diff "$expected" "/tmp/micro_test/${name}.actual"
        FAIL=$((FAIL+1))
    fi
done
echo "PASS: $PASS  FAIL: $FAIL"
```

### Layer 5: Negative Tests

```rust
// crates/cobol-transpiler/tests/negative.rs

#[test]
fn error_missing_copybook() {
    let result = transpile_file("cobol/negative/missing_copybook.cbl");
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("copybook") || err.contains("COPY"));
}
```

Scope: Verify meaningful error messages, not just panics.


## File Counts

| Category | Existing | New | Total |
|----------|------:|----:|------:|
| micro/ (per-verb) | 0 | ~120 | 120 |
| language/ | 42 | 0 | 42 |
| rustify/ | 15 | 0 | 15 |
| volume/ | 5 | 0 | 5 |
| negative/ | 0 | ~8 | 8 |
| **COBOL fixtures** | **62** | **~128** | **~190** |
| baselines/ (output) | 0 | ~190 | 190 |
| baselines/ (codegen) | 0 | ~120 | 120 |

Test runner infrastructure:
- `parse_micro.rs` -- ~300 lines (parse-layer assertions)
- `codegen_baseline.rs` -- ~200 lines (baseline diffing framework)
- `negative.rs` -- ~100 lines (error path tests)
- `scripts/test_micro.sh` -- ~100 lines (E2E runner)
- `scripts/bless_baselines.sh` -- ~50 lines (update expected outputs)


## Micro-Test Coverage Matrix

| Verb/Feature | Variants | Priority | Notes |
|-------------|------:|----------|-------|
| MOVE | 9 | P0 | Most common verb, truncation edge cases |
| ADD | 7 | P0 | TO, GIVING, ROUNDED, SIZE ERROR |
| SUBTRACT | 3 | P0 | FROM, GIVING, SIZE ERROR |
| MULTIPLY | 4 | P0 | BY, GIVING, ROUNDED, SIZE ERROR |
| DIVIDE | 5 | P0 | INTO, GIVING, REMAINDER, ROUNDED, /0 |
| COMPUTE | 7 | P0 | Precedence, parens, exponent, mixed |
| IF | 7 | P0 | Simple, ELSE, nested, AND/OR, NOT |
| EVALUATE | 6 | P0 | Variable, TRUE, THRU, ALSO, nested |
| PERFORM | 10 | P0 | All variants including TEST AFTER |
| DISPLAY | 7 | P1 | Each type, concat, literal |
| STRING | 4 | P1 | Pointer, overflow, delimited |
| UNSTRING | 3 | P1 | Simple, tallying, multi-delimiter |
| INSPECT | 5 | P1 | ALL/LEADING/FIRST tallying + replacing |
| SET | 3 | P1 | TO TRUE, UP BY, DOWN BY |
| INITIALIZE | 2 | P2 | Simple, REPLACING |
| GO TO | 2 | P2 | Simple, DEPENDING |
| SEARCH | 2 | P2 | Sequential, ALL |
| ACCEPT | 3 | P2 | DATE, TIME, DAY |
| Types | 26 | P0 | Every data type we support |
| **Total** | **~120** | | |


## Baseline Workflow

### Creating baselines (first time)

```bash
# Generate all baselines from current transpiler output
scripts/bless_baselines.sh --all
git add baselines/
git commit -m "initial test baselines"
```

### Updating baselines (after intentional change)

```bash
# After changing codegen, review diffs, then bless
cargo test codegen_baseline 2>&1 | grep FAIL  # see what changed
scripts/bless_baselines.sh --category micro/move
git diff baselines/  # review the changes
git add baselines/ && git commit -m "update baselines: move codegen change"
```

### CI integration

```bash
# In CI, baselines must match exactly
cargo test parse_micro codegen_baseline negative
scripts/test_micro.sh  # E2E execution tests
```


## Implementation Order

### Phase 1: Foundation (this session or next)
1. Create `cobol/micro/` directory structure
2. Write P0 micro-tests (~80 files, verbs + types)
3. Transpile all, capture initial baselines
4. Commit baselines

### Phase 2: Test Infrastructure
5. Write `parse_micro.rs` (parse-layer assertions)
6. Write `codegen_baseline.rs` (diff framework with BLESS support)
7. Write `scripts/test_micro.sh` (E2E runner)

### Phase 3: Coverage Expansion
8. Write P1 micro-tests (~22 files)
9. Write P2 micro-tests (~10 files)
10. Write negative tests (~8 files)
11. Add baselines for language/ and rustify/ tests

### Phase 4: CI Integration
12. Add test layers to CI pipeline
13. Document baseline update workflow in CONTRIBUTING.md


## Design Principles (from ProLeap)

1. **One behavior per file** -- isolate regressions to specific verb/edge case
2. **Three layers of validation** -- parse correctness, codegen stability, execution correctness
3. **Baseline-driven regression** -- committed expected output, explicit bless workflow
4. **Negative tests exist** -- error quality matters, not just happy path
5. **Self-documenting fixtures** -- expected output in COBOL header comment
6. **Deterministic always** -- no ACCEPT, no timestamps, no file I/O in micro-tests
