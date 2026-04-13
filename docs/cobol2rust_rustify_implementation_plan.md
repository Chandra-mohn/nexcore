# cobol2rust rustify -- Implementation Plan

Reference: `docs/cobol2rust_rustify_spec.md`

## Architecture Overview

```
cobol-transpiler (existing)         cobol-rustify (NEW)            cobol-cli (existing)
  |                                   |                               |
  | transpile() produces:             | analyze() reads:              | rustify_cmd.rs
  |   - generated .rs files           |   - .rs files (via syn)       |   wires CLI args
  |   - .rustify/hints.json (NEW)     |   - .rustify/hints.json       |   calls rustify API
  |                                   |                               |
  v                                   | apply() writes:               |
  raw-rust/                           |   - transformed .rs files     |
    src/program_a.rs                  |   - .rustify/manifest.json    |
    .rustify/hints.json               |   - .rustify/review.md        |
                                      v                               |
                                   cleaned-rust/                      |
                                     src/program_a.rs                 |
                                     .rustify/manifest.json           |
```

---

## Session-by-Session Plan

### Session 50A: Crate Skeleton + CLI Wiring + Report Infrastructure

**Goal**: `cobol2rust rustify <dir> --tier 1 --report` prints a stub report.
Engineer can invoke the command even though no rules exist yet.

**Tasks**:

1. Create `crates/cobol-rustify/Cargo.toml`
   ```toml
   [package]
   name = "cobol-rustify"
   version.workspace = true
   edition.workspace = true

   [dependencies]
   syn = { version = "2", features = ["full", "parsing", "visit", "visit-mut"] }
   quote = "1"
   proc-macro2 = "1"
   serde = { workspace = true }
   serde_json = { workspace = true }
   sha2 = "0.10"
   rust_decimal = { workspace = true }
   thiserror = { workspace = true }
   rayon = { workspace = true }

   [lints]
   workspace = true
   ```

2. Create crate structure:
   ```
   crates/cobol-rustify/src/
     lib.rs              # pub mod declarations, public API functions
     error.rs            # RustifyError enum
     hints.rs            # HintsFile, FileHints structs + serde
     manifest.rs         # Manifest struct + read/write + checksum
     patch.rs            # detect_patches() -> Vec<PatchedFile>
     report.rs           # ReportWriter (text, json, ndjson formats)
     review.rs           # ReviewWriter (generates review.md)
     plan.rs             # PlanWriter (generates plan.md) -- stub
     config.rs           # RustifyConfig (tier, include/exclude, rule selection)
     rules/
       mod.rs            # Rule trait, RuleRegistry, Tier enum, Safety enum
       transform.rs      # Transform, TransformDetail, InsertPosition structs
   ```

3. Define core types in `rules/mod.rs`:
   ```rust
   pub trait RustifyRule: Send + Sync {
       fn id(&self) -> &'static str;
       fn description(&self) -> &'static str;
       fn tier(&self) -> Tier;
       fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform>;
       fn apply(&self, ctx: &mut ApplyContext) -> Result<(), RustifyError>;
   }

   pub struct AnalysisContext<'a> {
       pub source: &'a syn::File,
       pub source_text: &'a str,
       pub file_path: &'a Path,
       pub hints: Option<&'a FileHints>,
   }

   pub struct ApplyContext<'a> {
       pub source: &'a str,
       pub file_path: &'a Path,
       pub hints: Option<&'a FileHints>,
       pub transforms: &'a [Transform],
   }

   pub struct RuleRegistry {
       rules: Vec<Box<dyn RustifyRule>>,
   }

   impl RuleRegistry {
       pub fn new() -> Self;                    // registers all built-in rules
       pub fn rules_for_tier(&self, tier: Tier) -> Vec<&dyn RustifyRule>;
       pub fn rule_by_id(&self, id: &str) -> Option<&dyn RustifyRule>;
   }
   ```

4. Implement `lib.rs` public API:
   ```rust
   /// Analyze a workspace and return transforms (report mode).
   pub fn analyze_workspace(
       source_dir: &Path,
       config: &RustifyConfig,
   ) -> Result<AnalysisReport, RustifyError>;

   /// Apply transforms and write to output directory.
   pub fn apply_workspace(
       source_dir: &Path,
       output_dir: &Path,
       config: &RustifyConfig,
   ) -> Result<ApplyReport, RustifyError>;

   /// List all available rules.
   pub fn list_rules() -> Vec<RuleInfo>;
   ```

5. Wire CLI subcommand in `cobol-cli`:
   - Add `cobol-rustify = { path = "../cobol-rustify" }` to cobol-cli's Cargo.toml
   - Create `crates/cobol-cli/src/rustify_cmd.rs` with clap args
   - Add `Command::Rustify(rustify_cmd::RustifyArgs)` to main.rs enum
   - Add match arm in dispatcher

6. Add `cobol-rustify` to workspace members (it's already covered by
   `members = ["crates/*"]`)

7. Add workspace dependencies: `syn`, `quote`, `proc-macro2`, `sha2`

**Validation**: `cobol2rust rustify --rules` prints empty rule list.
`cobol2rust rustify some-dir/ --tier 1 --report` prints "0 transforms found".
`cargo test -p cobol-rustify` passes (manifest round-trip, config parsing).

**Estimated effort**: 1 session

---

### Session 51: Manifest + Patch Detection + File Copy Pipeline

**Goal**: `cobol2rust rustify <source> --tier 1 --output <dest>` copies all
files, writes manifest.json with checksums. Patch detection works on re-run.

**Tasks**:

1. Implement `manifest.rs`:
   - `Manifest` struct with serde
   - `write_manifest(dir, manifest)` -> writes `.rustify/manifest.json`
   - `read_manifest(dir)` -> reads and validates
   - `compute_file_checksum(path)` -> SHA-256 hex string

2. Implement `patch.rs`:
   - `detect_patches(output_dir)` -> reads manifest, compares each file's
     current checksum vs `checksum_after`, returns list of modified files
   - Returns `PatchDetection { clean: Vec, modified: Vec, missing: Vec }`

3. Implement the file copy pipeline in `lib.rs`:
   - `apply_workspace()` does:
     a. Check if output_dir exists and has manifest -> patch detection
     b. If patches found and no --force/--preserve-patches -> error
     c. Copy source_dir to output_dir (excluding .rustify/)
     d. Copy .rustify/hints.json from source if present
     e. For each .rs file: parse with syn, run rules, write transformed source
     f. Write manifest.json with before/after checksums
   - For now, step (e) is identity (no rules yet) -- just validates the pipeline

4. Implement `hints.rs`:
   - `HintsFile` struct matching the spec's JSON schema
   - `read_hints(dir)` -> parses `.rustify/hints.json`
   - `FileHints` with field metadata, paragraph scope, level-88 groups
   - For now, hints are optional (gracefully absent)

5. Tests:
   - Create a minimal test fixture: a tiny generated Rust workspace
   - Test: copy pipeline produces identical output
   - Test: manifest checksums match file contents
   - Test: modify a file, re-run detects patch
   - Test: --force overwrites patched file
   - Test: --preserve-patches keeps patched file

**Validation**: Copy a real transpiled workspace (from stress tests),
run `rustify --tier 1 --output /tmp/test-out/`, verify identical content
with manifest.json written.

**Estimated effort**: 1 session

---

### Session 52: Tier 1 Rules -- const-extract + zero-literal

**Goal**: First two rules producing real transforms on generated Rust.

**Tasks**:

1. Create `rules/tier1/mod.rs` -- registers Tier 1 rules

2. Implement `rules/tier1/zero_literal.rs` (T1-ZERO):
   - **Easiest rule, good warmup for syn pattern matching**
   - Pattern: `"0".parse::<Decimal>().unwrap()` -> `Decimal::ZERO`
   - syn visitor: walk method call chains, match the pattern
   - Also: `"0".parse::<Decimal>().unwrap_or(Decimal::ZERO)` variants
   - Apply: token replacement in source text (line+column based)

3. Implement `rules/tier1/const_extract.rs` (T1-CONST):
   - **Most impactful Tier 1 rule**
   - Phase 1 -- Collect: walk all `.parse::<Decimal>().unwrap()` calls,
     extract the string literal, record file+line+column
   - Phase 2 -- Group: group by literal value, count occurrences
   - Phase 3 -- Name: generate const names from context
     (if literal is compared with a field, use field name; else use
     `LITERAL_<value>` e.g. `LITERAL_1000`)
   - Phase 4 -- Transform: insert `const` declarations at top of module,
     replace each occurrence with the const name
   - Handle: negative literals, decimal points, edge cases (empty string)

4. Implement the apply mechanism:
   - Transforms are line-based replacements (old span -> new text)
   - Sort transforms by line descending (apply bottom-up to preserve offsets)
   - For insertions (const declarations): insert after the `use` block
   - Write the modified source text (NOT syn's output -- preserve formatting)

5. Wire rules into RuleRegistry

6. Tests:
   - Unit tests with Rust code snippets as input
   - Test: zero-literal replaces `"0".parse::<Decimal>().unwrap()`
   - Test: const-extract groups identical literals
   - Test: const naming heuristic
   - Test: apply produces valid Rust (parse with syn after transform)

**Validation**: Run on a real transpiled file from stress tests. Verify
`cargo check` passes after transform.

**Estimated effort**: 1 session

---

### Session 53: Tier 1 Rules -- dead-field + unused-import + allow-cleanup + display-simplify

**Goal**: All 6 Tier 1 rules complete. Full Tier 1 pipeline works end-to-end.

**Tasks**:

1. Implement `rules/tier1/dead_field.rs` (T1-DEAD):
   - Requires hints.json (read_count + write_count per field)
   - If no hints: skip rule (report "hints not available")
   - Find `WorkingStorage` struct, match field names against hints
   - Remove fields with read_count == 0 AND write_count == 0
   - Also remove corresponding initialization in `new()`
   - Safety: never remove fields that are part of REDEFINES groups

2. Implement `rules/tier1/unused_import.rs` (T1-IMPORT):
   - Parse `use` statements
   - Scan file for references to each imported item
   - Remove `use` lines with zero references
   - Exception: keep `use cobol_runtime::prelude::*` (wildcard)

3. Implement `rules/tier1/allow_cleanup.rs` (T1-ALLOW):
   - Find `#[allow(...)]` attributes
   - For each: temporarily remove, check if the suppressed warning exists
   - Implementation: this is tricky without running clippy. Simpler approach:
     remove known-unnecessary allows that the transpiler always emits
     (e.g., `#[allow(non_snake_case)]` on WorkingStorage is always needed,
     but `#[allow(unused_variables)]` on paragraphs with no local temps
     may not be needed)
   - Conservative: only remove allows where we can statically verify

4. Implement `rules/tier1/display_simplify.rs` (T1-DISPLAY):
   - Pattern: `format!("{}", expr)` -> `expr.to_string()` when expr is
     a method call returning a Display type
   - Specifically: `format!("{}", x.to_decimal())` -> `x.to_decimal().to_string()`
   - Also: `String::from_utf8_lossy(&x.display_bytes()).to_string()` patterns

5. Text report formatting:
   - Summary by rule with counts
   - Per-file details in --verbose mode
   - Totals at bottom

6. NDJSON report output (--format ndjson)

7. Integration test:
   - Transpile a COBOL file -> apply all Tier 1 rules -> cargo check passes
   - Idempotency: apply twice, output identical

**Validation**: Run Tier 1 on 5 transpiled stress test programs.
All must `cargo check` after transform. Report shows meaningful transform counts.

**Estimated effort**: 1 session

---

### Session 54: Transpiler Hints Emission

**Goal**: `cobol2rust transpile --workspace` emits `.rustify/hints.json`
with field metadata, paragraph scoping, and usage counts.

**Tasks**:

1. Create `crates/cobol-transpiler/src/hints.rs`:
   - `HintsCollector` struct that accumulates metadata during codegen
   - Methods: `record_field_read(name)`, `record_field_write(name)`,
     `record_call_by_ref(name)`, `record_paragraph_scope(para, field)`,
     `set_redefines(field, target)`, `add_level_88(field, conditions)`
   - `finalize() -> HintsFile`

2. Integrate HintsCollector into codegen:
   - `proc_gen.rs`: at each field access (read or write), call collector
   - `data_gen.rs`: at each field declaration, record metadata (PIC, USAGE,
     LEVEL, REDEFINES)
   - `proc_gen.rs`: at each CALL statement, record BY REFERENCE params
   - `proc_gen.rs`: at each MOVE CORRESPONDING, record targets
   - `proc_gen.rs`: at paragraph boundaries, track which fields are accessed

3. Wire into `transpile_with_config_and_diagnostics()`:
   - Create HintsCollector at start
   - Pass to codegen functions
   - After codegen, finalize and include in TranspileResult

4. Wire into CLI's workspace transpile:
   - After writing each .rs file, write `.rustify/hints.json`
   - Create `.rustify/` directory in output workspace

5. Tests:
   - Transpile a known COBOL file, verify hints.json contains expected fields
   - Verify read/write counts match actual usage
   - Verify REDEFINES relationships are captured
   - Verify paragraph scope is correct

**Validation**: Transpile 3 stress test programs, inspect hints.json manually.
Run `rustify --tier 1 --report` on the output -- dead-field rule now uses hints.

**Estimated effort**: 1-2 sessions (codegen integration touches many functions)

---

### Session 55: Tier 2 Rules -- pic-to-string + packed-to-native

**Goal**: Two highest-impact Tier 2 rules with full safety analysis.

**Tasks**:

1. Implement safety gate framework:
   ```rust
   pub struct SafetyGate {
       pub field_name: String,
       pub reason: SafetyReason,
       pub recommendation: String,
   }

   pub enum SafetyReason {
       Redefines { redefined_by: Vec<String> },
       CallByReference { target_program: String },
       MoveCorresponding,
       FileIoField,
       BinaryIoAccess,
   }
   ```
   - `check_safety_gates(field, hints) -> Vec<SafetyGate>`
   - If any gates triggered -> Safety::Review, added to review.md

2. Implement `rules/tier2/pic_to_string.rs` (T2-STR):
   - **Analysis**: find all `PicX` fields in WorkingStorage
   - **Gate check**: consult hints for each field
   - **Transform** (for safe fields):
     a. Change struct field type: `PicX` -> `String`
     b. Change initialization: `PicX::new(30, b"...")` -> `"...".to_string()`
     c. Change reads: `.as_bytes()` -> `.as_bytes()` (String has this too)
     d. Change `String::from_utf8_lossy(x.as_bytes())` -> just `&x`
     e. Change `PicX::new(n, val)` assignments -> `= String::from(val)`
   - **Scope**: find all references to the field in the file using syn visitor

3. Implement `rules/tier2/packed_to_native.rs` (T2-NUM):
   - Similar structure to pic-to-string
   - `PackedDecimal` -> `Decimal`, `ZonedDecimal` -> `Decimal`
   - Change `.pack(val)` -> `= val`
   - Change `.to_decimal()` -> identity (just the field)
   - Change `PackedDecimal::new(p, s, signed)` -> `Decimal::ZERO`
   - Gate: same safety gates plus binary I/O access check

4. Implement `review.rs` -- ReviewWriter:
   - Generates `.rustify/review.md` from collected SafetyGate items
   - Format matches spec (numbered items, action checkboxes)
   - Groups by rule, then by file

5. Tests:
   - Unit tests with before/after Rust snippets for each transform
   - Test: REDEFINES field blocked, appears in review.md
   - Test: CALL BY REF field blocked
   - Test: safe field promoted correctly, cargo check passes

**Validation**: Run Tier 2 on 3 transpiled programs with known REDEFINES.
Verify review.md correctly flags them. Verify safe promotions produce valid Rust.

**Estimated effort**: 2 sessions

---

### Session 56: Tier 2 Rules -- localize-vars + bool-extract + enum-extract

**Goal**: All 5 Tier 2 rules complete. Full Tier 1+2 pipeline validated.

**Tasks**:

1. Implement `rules/tier2/localize_vars.rs` (T2-LOCAL):
   - Read hints `paragraph_scope` and `local_only_fields`
   - For each field used in only one paragraph: extract from WorkingStorage
   - Insert `let mut <name> = <init>;` at top of paragraph function
   - Remove field from struct and new()
   - Replace `ws.<name>` with just `<name>` within that paragraph

2. Implement `rules/tier2/bool_extract.rs` (T2-BOOL):
   - Read hints `level_88_groups`
   - Single condition on a single-byte field -> bool
   - Replace `ws.ws_flag.as_bytes() == b"Y"` -> `ws.ws_flag`
   - Replace `PicX` type with `bool`, init with `false`

3. Implement `rules/tier2/enum_extract.rs` (T2-ENUM):
   - Read hints `level_88_groups` where `exhaustive: true`
   - Generate enum definition from condition names + values
   - Replace comparisons with enum variant matches
   - Insert `#[derive(Debug, Clone, PartialEq)]` on generated enum
   - Place enum definition before WorkingStorage struct

4. Integration testing -- Tier 1 then Tier 2 pipeline:
   - Transpile -> Tier 1 -> Tier 2 -> cargo check
   - Verify transforms are additive (Tier 2 builds on Tier 1 output)
   - Verify review.md accumulates correctly

5. Run on a larger set (10 transpiled programs from corpus)

**Validation**: Full pipeline on 10 programs. All cargo check.
review.md is accurate and useful.

**Estimated effort**: 1-2 sessions

---

### Session 57: Tier 3 Assessment Rules

**Goal**: Tier 3 produces plan.md with structural analysis. No auto-apply.

**Tasks**:

1. Implement `rules/tier3/dispatcher_analysis.rs` (T3-DISPATCH):
   - Parse the dispatch loop (match on `_pc`)
   - Count paragraphs, map call graph from `para_x(ws, ctx, sql)` calls
   - Detect cycles (paragraph A calls B calls A)
   - Report: paragraph count, max call depth, cycle list, candidates
     for extraction (leaf paragraphs with no side effects)

2. Implement `rules/tier3/ws_decomposition.rs` (T3-WS):
   - Analyze WorkingStorage fields
   - Group by common prefix (e.g., `ws_acct_*` fields -> AcctInfo struct)
   - Group by co-access (fields always read/written together via hints)
   - Report: proposed sub-structs with field lists

3. Implement `rules/tier3/status_to_result.rs` (T3-RESULT):
   - Find fields that are set to literal values then checked with if/match
   - Pattern: `ws.ws_status.pack(dec!(0))` ... `if ws.ws_status.to_decimal() == ...`
   - Report: field name, value set, enum candidate, Result<T,E> mapping

4. Implement `rules/tier3/io_modernize.rs` (T3-IO):
   - Find file I/O patterns: OPEN/READ/WRITE/CLOSE sequences
   - Map to idiomatic Rust patterns (BufReader, Iterator, etc.)
   - Report: file handle, access pattern, recommended Rust idiom

5. Implement `plan.rs` -- PlanWriter:
   - Generates `.rustify/plan.md` from Tier 3 analysis
   - Organized by recommended decomposition phases
   - Includes estimated effort per phase
   - Includes dependency order (which phases must come first)

6. Tests:
   - Each rule tested against a synthetic generated Rust file
   - Plan.md output verified for structure and content

**Validation**: Run Tier 3 on the most complex stress test programs.
Plan.md should be readable and actionable by a Rust engineer.

**Estimated effort**: 1-2 sessions

---

### Session 58: Polish, Performance, Edge Cases

**Goal**: Production-ready for large files. All report formats working.

**Tasks**:

1. **Performance for large files**:
   - The critical client files produce ~50K-100K line Rust files
   - Profile syn parsing time for large files
   - If slow: investigate incremental parsing or line-based analysis
     for Tier 1 rules (many are pattern-match on text, not AST)
   - Parallel file processing with rayon (already in deps)

2. **JSON report format**:
   - `--format json` outputs full analysis as single JSON object
   - Matches manifest.json schema for summary, adds transform details
   - Suitable for GUI consumption

3. **Edge case hardening**:
   - Empty files, files with no WorkingStorage
   - Files with syntax errors (syn parse failure -> skip with warning)
   - Unicode in string literals
   - Very long lines (>2000 chars from COBOL DISPLAY)
   - Nested REDEFINES chains
   - OCCURS DEPENDING ON fields

4. **--preserve-patches implementation**:
   - When re-running on existing output with patches:
   - Read manifest, identify patched files
   - For patched files: copy from existing output (preserve patch)
   - For clean files: regenerate from source

5. **Cargo.toml handling**:
   - When Tier 2 removes runtime crate dependencies (e.g., PicX no longer
     imported after pic-to-string), update Cargo.toml accordingly
   - Conservative: only remove if zero references remain in all files

6. **Documentation**:
   - `cobol2rust rustify --help` with examples
   - Rule descriptions in `--rules` output

7. **Full validation suite**:
   - Run Tier 1 on 100 corpus programs -> all cargo check
   - Run Tier 1+2 on 20 programs -> all cargo check
   - Run Tier 1+2+3 on 5 programs -> plan.md reviewed for quality
   - Idempotency test: run twice, SHA-256 of output matches

**Estimated effort**: 1-2 sessions

---

## Dependency Graph

```
Session 50A -----> Session 51 -----> Session 52 -----> Session 53
(skeleton)        (manifest/copy)   (first rules)     (all T1 rules)
                                         |
                                         v
                                    Session 54 -------> Session 55 -----> Session 56
                                    (hints emission)   (T2 str+num)     (T2 local+enum)
                                                                              |
                                                                              v
                                                                         Session 57
                                                                         (T3 assessment)
                                                                              |
                                                                              v
                                                                         Session 58
                                                                         (polish)
```

Sessions 50A-53 are sequential (each builds on prior).
Session 54 (hints) can start after 51 (needs manifest format) but
is on the critical path for Tier 2 safety gates.
Sessions 55-58 are sequential.

**Total: 9-12 sessions** depending on complexity encountered.

---

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| syn parsing too slow for 100K-line files | Medium | High | Tier 1 rules can use text-based matching as fallback; only Tier 2 needs full AST |
| Tier 2 transforms break compilability | Medium | High | Every rule must have `cargo check` integration test; transforms are conservative |
| Hints.json schema changes during implementation | Low | Medium | Version field in hints.json; backward-compat reader |
| REDEFINES safety gates miss edge cases | Medium | High | Default to "review needed" when uncertain; false positive is safe, false negative is not |
| Generated Rust patterns vary across COBOL dialects | Low | Medium | Rules match on runtime API calls (cobol_move, etc.) which are consistent regardless of COBOL source |
| const-extract naming heuristic produces duplicates | Medium | Low | Append numeric suffix on collision (LITERAL_1000, LITERAL_1000_2) |

---

## New Workspace Dependencies

Add to root `Cargo.toml` `[workspace.dependencies]`:

```toml
syn = { version = "2", features = ["full", "parsing", "visit", "visit-mut"] }
quote = "1"
proc-macro2 = "1"
sha2 = "0.10"
```

---

## Files Changed Summary

### New Files
| File | Purpose |
|------|---------|
| `crates/cobol-rustify/Cargo.toml` | New crate manifest |
| `crates/cobol-rustify/src/lib.rs` | Public API |
| `crates/cobol-rustify/src/error.rs` | RustifyError |
| `crates/cobol-rustify/src/hints.rs` | hints.json types + reader |
| `crates/cobol-rustify/src/manifest.rs` | manifest.json types + read/write |
| `crates/cobol-rustify/src/patch.rs` | Patch detection |
| `crates/cobol-rustify/src/report.rs` | Report generation |
| `crates/cobol-rustify/src/review.rs` | review.md generation |
| `crates/cobol-rustify/src/plan.rs` | plan.md generation |
| `crates/cobol-rustify/src/config.rs` | RustifyConfig |
| `crates/cobol-rustify/src/rules/mod.rs` | Rule trait + registry |
| `crates/cobol-rustify/src/rules/transform.rs` | Transform types |
| `crates/cobol-rustify/src/rules/tier1/*.rs` | 6 Tier 1 rule files |
| `crates/cobol-rustify/src/rules/tier2/*.rs` | 5 Tier 2 rule files |
| `crates/cobol-rustify/src/rules/tier3/*.rs` | 4 Tier 3 rule files |
| `crates/cobol-cli/src/rustify_cmd.rs` | CLI subcommand |
| `crates/cobol-transpiler/src/hints.rs` | HintsCollector |

### Modified Files
| File | Change |
|------|--------|
| `Cargo.toml` | Add syn, quote, proc-macro2, sha2 to workspace deps |
| `crates/cobol-cli/Cargo.toml` | Add cobol-rustify dependency |
| `crates/cobol-cli/src/main.rs` | Add Rustify command variant + dispatch |
| `crates/cobol-transpiler/src/lib.rs` | pub mod hints |
| `crates/cobol-transpiler/src/transpile.rs` | Wire HintsCollector |
| `crates/cobol-transpiler/src/codegen/proc_gen.rs` | Record field usage in collector |
| `crates/cobol-transpiler/src/codegen/data_gen.rs` | Record field metadata in collector |

---

## Success Criteria (Per Session)

| Session | Gate |
|---------|------|
| 50A | `cobol2rust rustify --rules` works, `--report` on empty dir works |
| 51 | Copy pipeline + manifest + patch detection passes all tests |
| 52 | zero-literal + const-extract produce valid transforms, cargo check passes |
| 53 | All 6 Tier 1 rules work, 5 stress test programs pass cargo check after Tier 1 |
| 54 | hints.json emitted by transpiler, dead-field rule uses it |
| 55 | pic-to-string + packed-to-native work with safety gates, review.md generated |
| 56 | All 5 Tier 2 rules work, 10 programs pass pipeline, review.md accurate |
| 57 | Tier 3 plan.md generated, readable, actionable |
| 58 | 100 corpus programs pass Tier 1, performance acceptable for large files |
