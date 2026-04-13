# Direct DSL Emission Specification

> COBOL AST -> DSL files, bypassing Phase 1 Rust

**Status**: APPROVED (2026-04-10)
**Sessions**: 3 estimated
**Priority**: P0 (enables Java codegen, fixes monster-file DSL extraction)

## Problem Statement

The current DSL emission pipeline has a fragile intermediate step:

```
COBOL -> [ANTLR4] -> AST -> [Rust Codegen] -> Phase 1 Rust -> [syn re-parse] -> DSL
```

Phase 1 Rust is a lossy serialization of the AST. Known losses:

- EXIT SECTION becomes unconditional return (dead code bug)
- INSPECT figurative constants emit empty byte vectors
- Record lengths calculated incorrectly for some FDs
- 110K-line DATA DIVISIONs produce only 20 fields (parser choke)
- EXEC CICS blocks produce no Rust output
- Group hierarchy flattened (prefix heuristics needed to reconstruct)

Every Rust codegen bug silently corrupts DSL emitter input.

## Solution: Direct AST Emission

Add a parallel path that reads COBOL AST directly:

```
COBOL -> [ANTLR4] -> AST ----> [Direct Emitters] -> DSL  (new, primary)
                        |
                        +----> [Rust Codegen] -> Phase 1 Rust  (validation only)
```

Both paths produce identical `Vec<DslFile>` output. The direct path is
validated against the legacy path via comparison mode before becoming default.

## Blue/Green Coexistence

Both paths run simultaneously throughout the migration. Legacy code is
never modified beyond minimal visibility changes (`pub` on shared functions).

### Invariants

1. **Legacy emitter files are never deleted** -- they remain functional
2. **Default is always legacy** until cutover criteria are met
3. **Compare mode runs both** and returns legacy output as production
4. **Per-emitter switching** allows incremental rollover (E1 direct while E3 legacy)
5. **All 202 existing tests pass unchanged** throughout the migration

### Coexistence Timeline

```
Session 1: legacy = default, direct has E1 only
Session 2: legacy = default, direct has E1+E2+E4
Session 3: legacy = default, direct has E1+E2+E3+E4
Validation: compare mode on full corpus -> 0 diffs
Cutover:   direct = default, legacy = --emit-mode legacy
Future:    legacy removal (separate decision, not in scope)
```

## Architecture

### Dependency Chain

```
cobol-cli
  |-- cobol-transpiler  (ANTLR4 parser, AST, Rust codegen)
  |-- cobol-rustify     (rustification, DSL emitters)
       |-- [optional] cobol-transpiler  (for direct-emit feature)
```

The `cobol-rustify -> cobol-transpiler` dependency is behind a Cargo feature
flag (`direct-emit`). Default build is unchanged.

### Feature Flag

```toml
# crates/cobol-rustify/Cargo.toml
[features]
default = []
direct-emit = ["dep:cobol-transpiler"]

[dependencies]
cobol-transpiler = { workspace = true, optional = true }
```

### File Layout (both paths coexist)

```
crates/cobol-rustify/src/dsl/
  |
  |-- schema_emitter.rs        LEGACY (reads syn) -- pub on shared fns/structs
  |-- transform_emitter.rs     LEGACY (reads syn) -- pub on shared fns/structs
  |-- rules_emitter.rs         LEGACY (reads syn) -- pub on shared fns/structs
  |-- process_emitter.rs       LEGACY (reads syn) -- pub on shared fns/structs
  |-- cobol_attrs.rs           LEGACY (parses #[cobol()] from syn) -- untouched
  |-- expr_extract.rs          LEGACY (reverse-engineers Rust exprs) -- untouched
  |
  |-- dsl_ast.rs               SHARED (both paths produce these types)
  |-- type_mapping.rs          SHARED (both paths call pic_to_nexflow_type)
  |-- writer.rs                SHARED (routing added, legacy emit_all_dsl kept)
  |-- mod.rs                   SHARED (feature-gated `pub mod direct;` added)
  |
  |-- direct/                  NEW (parallel path, reads COBOL AST)
       |-- mod.rs              DirectEmitterContext, DirectDslEmitter, routing
       |-- schema_emitter.rs   E1: DataEntry -> reuses generate_schema()
       |-- transform_emitter.rs E2: Paragraph -> reuses generate_transform_file()
       |-- rules_emitter.rs    E3: IF/EVALUATE -> reuses generate_rules_file()
       |-- process_emitter.rs  E4: call graph -> reuses generate_process_file()
       |-- cobol_extract.rs    Shared: name conversion, DataEntry traversal
       |-- condition_extract.rs Condition/Operand -> DSL condition strings
```

## Code Reuse Strategy

Each legacy emitter has three internal layers:

```
[Layer A] Input extraction   -- syn-coupled, replaced by direct/ code
[Layer B] Classification     -- mostly reusable, some needs adaptation
[Layer C] DSL generation     -- fully reusable, called from direct/ emitters
```

### What the direct emitters reuse from legacy (unchanged logic)

The direct emitters import and call the generation functions and intermediate
types from the legacy emitters. This requires making ~8 functions and ~6
structs `pub` in the legacy files -- a visibility-only change, zero logic change.

| Legacy File | Made `pub` | Used By |
|---|---|---|
| schema_emitter.rs | `EntityGroup`, `SchemaField`, `generate_schema()` | direct/schema_emitter.rs |
| transform_emitter.rs | `ClassifiedTransform`, `generate_transform_file()`, `group_by_section()`, `sanitize_identifier()` | direct/transform_emitter.rs |
| rules_emitter.rs | `RuleCandidate`, `generate_rules_file()`, `group_by_section()` | direct/rules_emitter.rs |
| process_emitter.rs | `CallGraph`, `CallNode`, `generate_process_file()`, `sanitize_identifier()` | direct/process_emitter.rs |

### What the direct emitters write new

| Direct File | Lines | Purpose |
|---|---|---|
| cobol_extract.rs | ~60 | `cobol_name_to_snake()`, `data_entry_to_schema_field()`, `walk_data_entries()` |
| condition_extract.rs | ~80 | `Condition` -> DSL condition string, `Operand` -> DSL expr |
| schema_emitter.rs | ~100 | DataEntry -> SchemaField adapter, group by AST hierarchy |
| transform_emitter.rs | ~80 | Paragraph -> ClassifiedTransform via AST statements |
| rules_emitter.rs | ~150 | IF/EVALUATE AST -> RuleCandidate |
| process_emitter.rs | ~120 | ProcedureDivision -> CallGraph via AST sections |
| mod.rs | ~60 | DirectEmitterContext, trait, emit_all_dsl_direct() |
| **Total new** | **~650** | |

### Reuse breakdown

```
Total emitter code:     ~2500 lines
Shared (both paths):    ~800 lines  (dsl_ast, type_mapping, writer)
Legacy generation fns:  ~810 lines  (generate_schema, generate_transform_file, etc.)
Legacy input layer:     ~370 lines  (cobol_attrs, expr_extract) -- NOT reused
Legacy classification:  ~520 lines  (partially reused via pub structs)

Direct new code:        ~650 lines
Direct reuses:          ~810 lines  (generation fns from legacy, called not copied)

Total after migration:  ~3150 lines (2500 existing + 650 new)
Duplicated code:        0 lines     (generation logic is shared, not copied)
```

## CLI Interface

### Emit Mode Flag

```
--emit-mode <MODE>    DSL emission strategy [default: legacy]
  legacy              Read from Phase 1 Rust via syn (current behavior)
  direct              Read from COBOL AST directly (new path)
  compare             Run both, diff output, report discrepancies
```

### Per-Emitter Override

```
--direct-emitters <LIST>   Comma-separated emitters to use direct path
                           Values: schema, transform, rules, process
                           Overrides --emit-mode for named emitters
```

### Usage Examples

```bash
# Default: legacy (no change to existing behavior)
cobol2rust rustify --input src/ --output out/

# Direct path for schema only (testing E1 in isolation)
cobol2rust rustify --input src/ --output out/ --direct-emitters schema

# Full direct path
cobol2rust rustify --input src/ --output out/ --emit-mode direct

# Comparison mode: run both paths, diff output
cobol2rust rustify --input src/ --output out/ --emit-mode compare

# Mixed: schema+process direct, transform+rules legacy
cobol2rust rustify --input src/ --output out/ --direct-emitters schema,process
```

## Emitter Context

### Legacy (existing, unchanged)

```rust
pub struct EmitterContext<'a> {
    pub syn_file: &'a syn::File,
    pub program_name: String,
    pub hints: Option<&'a FileHints>,
    pub assessments: &'a [Transform],
    pub source_path: PathBuf,
}
```

### Direct (new)

```rust
pub struct DirectEmitterContext<'a> {
    pub cobol_program: &'a CobolProgram,
    pub program_name: String,
    pub hints: Option<&'a FileHints>,
    pub assessments: &'a [Transform],
    pub source_path: PathBuf,
}
```

Both produce `Vec<DslFile>`. The `DirectDslEmitter` trait mirrors
`DslEmitter` but takes `DirectEmitterContext`.

## Routing Logic

```rust
// writer.rs -- both paths available, routing by flag

pub fn emit_dsl_routed(
    legacy_ctx: &EmitterContext<'_>,
    direct_ctx: Option<&DirectEmitterContext<'_>>,
    mode: EmitMode,
    overrides: &EmitterOverrides,
) -> (Vec<DslFile>, Option<ComparisonReport>) {
    match mode {
        EmitMode::Legacy => {
            (emit_all_dsl(legacy_ctx), None)   // existing function, unchanged
        }
        EmitMode::Direct => {
            let ctx = direct_ctx.expect("direct mode requires CobolProgram");
            (emit_all_dsl_direct(ctx), None)   // new function in direct/mod.rs
        }
        EmitMode::Compare => {
            let legacy = emit_all_dsl(legacy_ctx);
            let direct = emit_all_dsl_direct(
                direct_ctx.expect("compare mode requires CobolProgram")
            );
            let report = compare_outputs(&legacy, &direct);
            (legacy, Some(report))             // legacy is production output
        }
    }
}
```

Per-emitter overrides are handled inside `emit_dsl_routed` by calling
individual emitters from the appropriate path:

```rust
// Pseudocode for mixed mode:
let schema = if overrides.get("schema") == Direct {
    direct_schema.emit(direct_ctx)
} else {
    legacy_schema.emit(legacy_ctx)
};
// ... same for transform, rules, process
```

## Information Advantage: AST vs Phase 1 Rust

| Data Point | Phase 1 Rust (via syn) | COBOL AST (direct) |
|---|---|---|
| PIC clause | Raw string "9(5)V99" | Parsed: PicClause { digits, scale, category, signed } |
| Level-88 | Encoded "ACTIVE:A,CLOSED:C" | Typed: ConditionValue::Single(Literal) |
| Groups | Flat struct, prefix heuristics | DataEntry.children tree |
| Conditions | Reverse-engineer `ws.field.to_decimal() > "5".parse()` | Condition::Comparison { left, op, right } |
| EVALUATE | Reverse-engineer Rust if/else-if | EvaluateStatement { subjects, when_branches } |
| EXEC SQL | Lost (not in Phase 1 Rust) | ExecSqlStatement { sql_type, raw_sql, host_vars } |
| PERFORM VARYING | Lost (becomes while loop) | PerformStatement { loop_type: Varying { from, by, until } } |
| File descriptions | Partial (handle type only) | Full: organization, access_mode, record_key, status |

The AST is a strict superset. No information is lost going direct.

## Comparison Mode

When `--emit-mode compare` is used:

1. Run legacy emitters -> `Vec<DslFile>` (set A)
2. Run direct emitters -> `Vec<DslFile>` (set B)
3. Diff A vs B line-by-line per DSL file
4. Produce `comparison_report.json`:

```json
{
  "matches": ["schema/account_info.schema", "process/cardproc.proc"],
  "mismatches": [
    {
      "path": "rules/validation.rules",
      "legacy_lines": 42,
      "direct_lines": 45,
      "first_diff_line": 18,
      "legacy_snippet": "when account-status == \"A\"",
      "direct_snippet": "when ACCT-STATUS == \"A\""
    }
  ],
  "legacy_only": [],
  "direct_only": ["transform/sql_access.xform"]
}
```

5. Return legacy output as production (safe default)
6. Print summary to stderr

## Migration Order

| Session | Emitter | New Code | Reused From Legacy | Visibility Changes |
|---|---|---|---|---|
| 1 | **E1 Schema** + foundation | ~220 lines (direct/mod.rs, direct/schema_emitter.rs, direct/cobol_extract.rs, writer routing, CLI flags) | `generate_schema()`, `EntityGroup`, `SchemaField` | 3 items made `pub` in schema_emitter.rs |
| 2 | **E4 Process** + **E2 Transform** | ~260 lines (direct/process_emitter.rs, direct/transform_emitter.rs) | `generate_process_file()`, `generate_transform_file()`, `CallGraph`, `CallNode`, `ClassifiedTransform` | 5 items made `pub` in process_emitter.rs, transform_emitter.rs |
| 3 | **E3 Rules** + full comparison | ~230 lines (direct/rules_emitter.rs, direct/condition_extract.rs) | `generate_rules_file()`, `RuleCandidate` | 2 items made `pub` in rules_emitter.rs |

### Changes to legacy files (exhaustive list)

Only **visibility changes** (adding `pub`) -- zero logic changes:

```
schema_emitter.rs:
  struct EntityGroup        -> pub struct EntityGroup
  struct SchemaField        -> pub struct SchemaField
  fn generate_schema()      -> pub fn generate_schema()

transform_emitter.rs:
  struct ClassifiedTransform -> pub struct ClassifiedTransform
  fn generate_transform_file() -> pub fn generate_transform_file()
  fn group_by_section()     -> pub fn group_by_section()
  fn sanitize_identifier()  -> pub fn sanitize_identifier()

rules_emitter.rs:
  struct RuleCandidate      -> pub struct RuleCandidate
  fn generate_rules_file()  -> pub fn generate_rules_file()

process_emitter.rs:
  struct CallGraph          -> pub struct CallGraph
  struct CallNode           -> pub struct CallNode
  fn generate_process_file() -> pub fn generate_process_file()
  fn sanitize_identifier()  -> pub fn sanitize_identifier()
```

No function signatures change. No return types change. No logic changes.
All existing tests pass without modification.

## Cutover Criteria

Direct becomes the default when:

1. `--emit-mode compare` produces 0 mismatches on full client corpus
2. All 202 rustify tests pass in both modes
3. Direct path has independent test coverage >= legacy
4. Performance is equal or better (expected faster -- no syn parsing)

Legacy path remains available via `--emit-mode legacy` indefinitely.
Legacy code is never deleted until a separate future decision.

## Testing Strategy

### Existing tests (unchanged)

All 202 rustify tests continue to exercise the legacy path.
They pass without modification throughout the migration.

### New tests per session

| Session | Test Type | What It Validates |
|---|---|---|
| 1 | Unit tests in direct/schema_emitter.rs | DataEntry -> SchemaField conversion, group hierarchy decomposition |
| 1 | Comparison test on COBOL corpus | Legacy schema vs direct schema -> assert identical |
| 2 | Unit tests in direct/process_emitter.rs, direct/transform_emitter.rs | AST -> CallGraph, AST -> ClassifiedTransform |
| 2 | Comparison tests | Legacy process+transform vs direct -> assert identical |
| 3 | Unit tests in direct/rules_emitter.rs, direct/condition_extract.rs | IF/EVALUATE AST -> RuleCandidate, Condition -> DSL string |
| 3 | Full comparison test | All 4 emitters, all COBOL files -> assert zero diffs |

### Comparison tests use real COBOL

```rust
#[test]
fn compare_schema_emission() {
    for cobol_file in glob("cobol/**/*.cbl") {
        let program = parse_cobol(&read(cobol_file));
        let rust_code = transpile(&program);

        let legacy = legacy_schema.emit(&EmitterContext::from_rust(&rust_code));
        let direct = direct_schema.emit(&DirectEmitterContext::from_ast(&program));

        assert_eq!(legacy, direct, "mismatch for {cobol_file}");
    }
}
```

## Risk Mitigations

| Risk | Mitigation |
|---|---|
| Codegen normalizations lost | Comparison mode catches discrepancies |
| Circular dependency | One-way only, feature-gated, cobol-transpiler is a leaf crate |
| Test coverage gap | Unit tests + comparison tests on real COBOL corpus |
| Rollback needed | Per-emitter `--direct-emitters` flag, default is always legacy |
| AST info gaps | Verified: none. AST is strictly richer than Phase 1 Rust. |
| Rules emitter complexity | Direct path is actually simpler (reads native IF/EVALUATE) |
| Legacy code rot | Legacy stays default and tested; no risk of rot during migration |
| Generation fn signature mismatch | Direct emitters build same intermediate structs (EntityGroup, CallGraph, etc.) that generation fns expect |
