# cobol2rust rustify -- Phase 2 Rustification Spec

## Overview

The `rustify` subcommand transforms transpiler-generated "COBOL-shaped Rust" into
idiomatic Rust through a tiered, idempotent promotion pipeline. Each tier writes
to a new output directory. The Rust engineer reviews reports, applies transforms,
and may patch results at any stage before promoting to the next tier.

**Design Principles:**
- Idempotent: same input + same rules = same output
- Non-destructive: never modifies source directory, always writes to new output
- Forward-only: COBOL is the starting point, not a living source
- Engineer authority: reports first, engineer decides what to apply
- GUI-ready: all metadata in JSON for future GUI consumption

## CLI Interface

### Basic Usage

```
# Report mode (default) -- analyze and show what would change
cobol2rust rustify <source-dir> --tier <N> --report

# Apply mode -- write transformed Rust to output directory
cobol2rust rustify <source-dir> --tier <N> --output <dest-dir>

# Both -- apply and also print report to stdout
cobol2rust rustify <source-dir> --tier <N> --output <dest-dir> --report
```

### Arguments and Flags

| Argument / Flag | Required | Description |
|-----------------|----------|-------------|
| `<source-dir>` | Yes | Path to input Rust workspace (transpile output or previous stage) |
| `--tier <N>` | Yes | Tier to apply: `1`, `2`, or `3` |
| `--output <dir>` | For apply | Destination directory (must not equal source) |
| `--report` | No | Print transform report to stdout (default when no --output) |
| `--format <fmt>` | No | Report format: `text` (default), `json`, `ndjson` |
| `--include <glob>` | No | Only process files matching glob (e.g. `src/acct*`) |
| `--exclude <glob>` | No | Skip files matching glob |
| `--only <rules>` | No | Comma-separated rule IDs to apply (e.g. `const-extract,dead-field`) |
| `--skip <rules>` | No | Comma-separated rule IDs to skip |
| `--rules` | No | List all available rules with descriptions and exit |
| `--force` | No | Overwrite output even if patch detection finds modified files |
| `--preserve-patches` | No | When re-running, keep user modifications in output |
| `--verbose` | No | Show per-transform details (default report is summary) |
| `-j, --jobs <N>` | No | Parallel file processing (default: num_cpus) |

### Examples

```bash
# Transpile first (existing command)
cobol2rust transpile --workspace cobol/ --output raw-rust/

# See what Tier 1 would do
cobol2rust rustify raw-rust/ --tier 1 --report

# Apply Tier 1
cobol2rust rustify raw-rust/ --tier 1 --output cleaned-rust/

# Engineer reviews, tests, patches cleaned-rust/ as needed...

# Promote to Tier 2
cobol2rust rustify cleaned-rust/ --tier 2 --output typed-rust/

# Single-file report
cobol2rust rustify raw-rust/ --tier 1 --include src/program_a.rs --report

# Only specific rules
cobol2rust rustify raw-rust/ --tier 1 --only const-extract,zero-literal \
    --output cleaned-rust/

# NDJSON output for tooling/GUI
cobol2rust rustify raw-rust/ --tier 1 --report --format ndjson \
    > rustify_report.ndjson

# List all rules
cobol2rust rustify --rules

# Re-run Tier 1 after source changed (re-transpile)
cobol2rust rustify raw-rust/ --tier 1 --output cleaned-rust/ --force
```

## Pipeline Folder Structure

The user names directories however they want. The `.rustify/` metadata inside
each directory tracks provenance. A typical progression:

```
project/
  cobol/                    # Original COBOL source (frozen reference)
  raw-rust/                 # cobol2rust transpile output
    src/
      program_a.rs
      program_b.rs
    Cargo.toml
    .rustify/
      hints.json            # COBOL semantic hints (emitted by transpiler)
  cleaned-rust/             # After Tier 1
    src/
      program_a.rs          # Cosmetically cleaned
      program_b.rs
    Cargo.toml
    .rustify/
      manifest.json         # What rustify produced
      hints.json            # Carried forward from source
  typed-rust/               # After Tier 2
    src/
      program_a.rs          # Types promoted
      program_b.rs
    Cargo.toml
    .rustify/
      manifest.json
      hints.json
      review.md             # Safety review items for engineer
  idiomatic-rust/           # After Tier 3
    src/
      program_a.rs          # Structurally transformed
    Cargo.toml
    .rustify/
      manifest.json
      hints.json
      review.md
      plan.md               # Rustification plan for manual work
```

## Manifest Format (`.rustify/manifest.json`)

Written by `rustify --output` into the destination directory.

```json
{
  "version": "1.0",
  "cobol2rust_version": "0.1.0",
  "source": "../raw-rust",
  "source_resolved": "/abs/path/to/raw-rust",
  "tier": 1,
  "timestamp": "2026-03-16T14:30:00Z",
  "rules_applied": [
    "const-extract",
    "dead-field",
    "zero-literal",
    "allow-cleanup",
    "unused-import",
    "display-simplify"
  ],
  "rules_skipped": [],
  "rules_available": ["const-extract", "dead-field", "..."],
  "include_filter": null,
  "exclude_filter": null,
  "files": {
    "src/program_a.rs": {
      "checksum_before": "sha256:def456...",
      "checksum_after": "sha256:abc123...",
      "transforms_applied": 14,
      "rules_hit": ["const-extract:8", "dead-field:3", "zero-literal:3"]
    },
    "src/program_b.rs": {
      "checksum_before": "sha256:...",
      "checksum_after": "sha256:...",
      "transforms_applied": 6,
      "rules_hit": ["const-extract:4", "unused-import:2"]
    }
  },
  "summary": {
    "files_processed": 47,
    "files_changed": 42,
    "files_unchanged": 5,
    "total_transforms": 207,
    "by_rule": {
      "const-extract": 184,
      "dead-field": 12,
      "zero-literal": 5,
      "allow-cleanup": 3,
      "unused-import": 2,
      "display-simplify": 1
    }
  }
}
```

### Patch Detection

When `rustify --output <existing-dir>`:

1. Read `<existing-dir>/.rustify/manifest.json`
2. For each file, compare current disk checksum vs `checksum_after`
3. If mismatch: file was patched by the engineer
4. Default: warn and abort ("3 files modified since last rustify")
5. `--force`: overwrite all (patches lost)
6. `--preserve-patches`: apply transforms only to unmodified files, copy
   modified files as-is from the existing output

## Hints File (`.rustify/hints.json`)

Emitted by `cobol2rust transpile` alongside the generated Rust. Contains COBOL
semantic information that the Rust source code alone cannot convey. Carried
forward through each promotion stage.

```json
{
  "version": "1.0",
  "files": {
    "src/program_a.rs": {
      "cobol_source": "cobol/PROGRAM-A.CBL",
      "fields": {
        "ws_account_number": {
          "pic": "X(10)",
          "usage": "DISPLAY",
          "level": 5,
          "redefines": null,
          "redefined_by": ["ws_acct_num_parts"],
          "call_by_ref": false,
          "move_corr_target": false,
          "read_count": 7,
          "write_count": 2,
          "paragraph_scope": ["100-VALIDATE", "200-PROCESS"]
        },
        "ws_amount": {
          "pic": "S9(7)V99",
          "usage": "COMP-3",
          "level": 5,
          "redefines": null,
          "redefined_by": [],
          "call_by_ref": true,
          "move_corr_target": false,
          "read_count": 12,
          "write_count": 5,
          "paragraph_scope": ["200-PROCESS", "300-CALCULATE"]
        }
      },
      "paragraphs": {
        "100-VALIDATE": {
          "performs": ["110-CHECK-ACCOUNT"],
          "performed_by": ["MAIN-LOGIC"],
          "local_only_fields": ["ws_temp_flag"]
        }
      },
      "level_88_groups": {
        "ws_status": {
          "conditions": {
            "ws_status_active": "A",
            "ws_status_inactive": "I",
            "ws_status_pending": "P"
          },
          "exhaustive": true
        }
      },
      "call_targets": ["SUB-PROGRAM-1"],
      "file_io_fields": ["fd_input_record"]
    }
  }
}
```

### Hint Fields and Their Consumers

| Hint | Used by Rule | Purpose |
|------|-------------|---------|
| `redefines` / `redefined_by` | `pic-to-string`, `packed-to-native` | Block type promotion if field is redefined |
| `call_by_ref` | `pic-to-string`, `packed-to-native` | Block promotion if passed by reference to CALL |
| `move_corr_target` | `pic-to-string` | Block promotion if used in MOVE CORRESPONDING |
| `read_count` / `write_count` | `dead-field` | Zero reads + zero writes = dead field |
| `paragraph_scope` | `localize-vars` | Single-paragraph scope = safe to localize |
| `local_only_fields` | `localize-vars` | Pre-computed candidates for local extraction |
| `level_88_groups` | `bool-extract`, `enum-extract` | Map conditions to bool or enum |
| `file_io_fields` | `pic-to-string` | Block promotion for FD record fields |

## Rule Definitions

### Tier 1: Cosmetic Cleanup

Fully automatic, zero semantic risk. Can be applied without review.

#### T1-CONST: const-extract

Repeated `.parse::<Decimal>().unwrap()` literals become named constants.

**Before:**
```rust
if ws.ws_amount.to_decimal() > "1000".parse::<Decimal>().unwrap() {
    // ...
}
// ... elsewhere ...
if ws.ws_balance.to_decimal() < "1000".parse::<Decimal>().unwrap() {
```

**After:**
```rust
const AMOUNT_THRESHOLD: Decimal = dec!(1000);
// ...
if ws.ws_amount.to_decimal() > AMOUNT_THRESHOLD {
    // ...
}
if ws.ws_balance.to_decimal() < AMOUNT_THRESHOLD {
```

**Naming heuristic**: If the literal is used with a specific field comparison,
derive the name from the field. If used in multiple contexts, use a generic
name with a comment noting the COBOL source line.

#### T1-ZERO: zero-literal

`"0".parse::<Decimal>().unwrap()` becomes `Decimal::ZERO`.

#### T1-DEAD: dead-field

Remove WorkingStorage fields where `read_count == 0 AND write_count == 0`
(from hints.json). Remove the field declaration and any initialization.

#### T1-ALLOW: allow-cleanup

Remove `#[allow(...)]` attributes that suppress no actual warnings for the
current code. Detected by running `cargo clippy` with the allow removed.

#### T1-IMPORT: unused-import

Remove `use` statements for items not referenced in the file.

#### T1-DISPLAY: display-simplify

`format!("{}", x.to_decimal())` becomes `x.to_decimal().to_string()`.

### Tier 2: Type Promotion

Requires COBOL semantic hints for safety analysis. Produces `review.md` for
fields that cannot be automatically promoted.

#### T2-STR: pic-to-string

Promote `PicX` fields to `String` where safe.

**Safety gates** (any blocks promotion, sends to review.md):
- Field has `redefined_by` or `redefines` in hints
- Field has `call_by_ref: true`
- Field has `move_corr_target: true`
- Field is in `file_io_fields`

**Before:**
```rust
pub struct WorkingStorage {
    pub ws_name: PicX,       // PIC X(30)
}
// ...
ws.ws_name = PicX::new(30, b"JOHN DOE");
let name_str = String::from_utf8_lossy(ws.ws_name.as_bytes());
```

**After:**
```rust
pub struct WorkingStorage {
    pub ws_name: String,
}
// ...
ws.ws_name = "JOHN DOE".to_string();
let name_str = &ws.ws_name;
```

#### T2-NUM: packed-to-native

Promote `PackedDecimal` / `ZonedDecimal` fields to `Decimal` where safe.

**Safety gates**: same as pic-to-string plus:
- Field used in binary I/O (raw byte access)

**Before:**
```rust
pub ws_amount: PackedDecimal,  // PIC S9(7)V99 COMP-3
// ...
ws.ws_amount.pack(dec!(100));
let val = ws.ws_amount.to_decimal();
```

**After:**
```rust
pub ws_amount: Decimal,
// ...
ws.ws_amount = dec!(100);
let val = ws.ws_amount;
```

#### T2-LOCAL: localize-vars

Hoist WorkingStorage fields to local variables where they are only used
within a single paragraph (from hints `paragraph_scope` and `local_only_fields`).

**Before:**
```rust
pub struct WorkingStorage {
    pub ws_temp_idx: PackedDecimal,  // only used in 200-PROCESS
    // ... 50 other fields
}
fn para_200_process(ws: &mut WorkingStorage, config: &RuntimeConfig) {
    ws.ws_temp_idx.pack(dec!(0));
    // ...
}
```

**After:**
```rust
pub struct WorkingStorage {
    // ws_temp_idx removed
    // ... 50 other fields
}
fn para_200_process(ws: &mut WorkingStorage, config: &RuntimeConfig) {
    let mut temp_idx = Decimal::ZERO;
    // ...
}
```

#### T2-BOOL: bool-extract

Level-88 condition with a single value on a single-byte field becomes `bool`.

#### T2-ENUM: enum-extract

Level-88 group marked `exhaustive: true` in hints becomes a Rust `enum`.

**Before:**
```rust
// ws_status: PicX(1), with level-88: A=active, I=inactive, P=pending
if ws.ws_status.as_bytes() == b"A" { ... }
```

**After:**
```rust
#[derive(Debug, Clone, PartialEq)]
enum Status { Active, Inactive, Pending }

if ws.ws_status == Status::Active { ... }
```

### Tier 3: Structural Assessment

Tier 3 does NOT auto-apply transforms. It produces two outputs:
- `review.md`: items needing engineer decisions
- `plan.md`: a rustification plan document for manual/AI-assisted work

#### T3-DISPATCH: dispatcher-analysis

Analyze the paragraph dispatch table and report:
- How many paragraphs exist
- Call graph depth and cycles
- Candidates for extraction into standalone functions
- Candidates for conversion to `match` expressions

#### T3-WS: ws-decomposition

Analyze WorkingStorage field clusters:
- Fields always accessed together -> candidate for a struct
- Fields with common prefix -> candidate for a sub-struct
- Fields passed together to CALL -> candidate for a parameter struct

#### T3-RESULT: status-to-result

Identify status code patterns:
- Field set to values, then checked with IF/EVALUATE
- Map to `Result<T, E>` or custom enum

#### T3-IO: io-modernize

Analyze file I/O patterns:
- Sequential read loops -> iterator pattern
- Record-at-a-time processing -> streaming pattern
- FD record structures -> typed record structs

## Review File (`.rustify/review.md`)

Generated by Tier 2 and Tier 3. Engineer reads this, addresses items at
their own pace and priority, then re-runs or promotes.

```markdown
# Rustify Review -- Tier 2

Generated: 2026-03-16T14:30:00Z
Source: ../cleaned-rust/
Rules: pic-to-string, packed-to-native, localize-vars, bool-extract, enum-extract

## Needs Review (9 items)

### pic-to-string

#### [PTS-001] ws_account_number in program_a.rs
- **Reason**: Field is redefined by `ws_acct_num_parts`
- **Location**: src/program_a.rs, WorkingStorage struct
- **COBOL**: `05 WS-ACCOUNT-NUMBER PIC X(10)`
- **Risk**: Changing to String would break the REDEFINES overlay
- **Recommendation**: Keep as PicX unless REDEFINES is also eliminated
- **Action**: [ ] Approve promotion  [ ] Keep as PicX  [ ] Defer

#### [PTS-002] ws_message_buffer in program_b.rs
- **Reason**: Field passed by reference to CALL 'LOG-WRITER'
- **Location**: src/program_b.rs, WorkingStorage struct
- **COBOL**: `05 WS-MESSAGE-BUFFER PIC X(256)`
- **Risk**: Called program expects PicX byte layout
- **Recommendation**: Keep as PicX until LOG-WRITER is also rustified
- **Action**: [ ] Approve promotion  [ ] Keep as PicX  [ ] Defer

### packed-to-native

#### [PTN-001] ws_total_amount in program_a.rs
- **Reason**: Field passed by reference to CALL 'CALC-TAX'
- **Location**: src/program_a.rs, WorkingStorage struct
- **COBOL**: `05 WS-TOTAL-AMOUNT PIC S9(9)V99 COMP-3`
- **Risk**: Called program expects PackedDecimal byte layout
- **Recommendation**: Promote after CALC-TAX is also rustified
- **Action**: [ ] Approve promotion  [ ] Keep as PackedDecimal  [ ] Defer

## Auto-Applied (62 items)

- 28 PicX -> String promotions (no safety gates triggered)
- 19 PackedDecimal -> Decimal promotions
- 15 WorkingStorage -> local variable extractions

See manifest.json for per-file details.
```

## Plan File (`.rustify/plan.md`) -- Tier 3 Only

```markdown
# Rustification Plan -- program_a.rs

Generated: 2026-03-16T14:30:00Z
Complexity: High (47 paragraphs, 3 cycles in call graph)

## Recommended Decomposition

### Phase 1: Extract Pure Functions
The following paragraphs have no side effects (no file I/O, no CALL):
- 300-CALCULATE (12 lines) -> fn calculate(amount: Decimal, rate: Decimal) -> Decimal
- 310-VALIDATE-DATE (8 lines) -> fn validate_date(date: &str) -> bool

### Phase 2: Group Related Paragraphs into Modules
- 100-INIT, 110-LOAD-CONFIG, 120-OPEN-FILES -> mod initialization
- 200-PROCESS, 210-READ-NEXT, 220-TRANSFORM -> mod processing
- 900-CLEANUP, 910-CLOSE-FILES, 920-WRITE-SUMMARY -> mod finalization

### Phase 3: Eliminate Dispatch Table
Current: 47 entries in dispatch_paragraph() match
After Phase 1+2: estimated 12 entries remaining
Recommended: Replace with direct function calls in main loop

### Phase 4: Modernize I/O
- fd_input_file: sequential read loop -> BufReader + Iterator
- fd_output_file: sequential write -> BufWriter
- fd_report_file: formatted output -> write! macros

## Estimated Effort
- Phase 1: Low (mechanical extraction)
- Phase 2: Medium (requires understanding groupings)
- Phase 3: Medium-High (control flow redesign)
- Phase 4: Medium (I/O pattern conversion)
```

## NDJSON Report Format

For tooling and future GUI integration:

```json
{"type":"file_start","file":"src/program_a.rs","tier":1,"timestamp":"2026-03-16T14:30:00Z"}
{"type":"transform","file":"src/program_a.rs","rule":"const-extract","line":42,"description":"\"1000\".parse::<Decimal>() -> AMOUNT_THRESHOLD (used 3x)","safety":"auto","applied":true}
{"type":"transform","file":"src/program_a.rs","rule":"dead-field","line":8,"description":"ws_filler_1: PicX (zero reads, zero writes)","safety":"auto","applied":true}
{"type":"review","file":"src/program_a.rs","rule":"pic-to-string","id":"PTS-001","field":"ws_account_number","reason":"redefined by ws_acct_num_parts","safety":"review","applied":false}
{"type":"file_end","file":"src/program_a.rs","transforms_applied":14,"transforms_review":1}
{"type":"summary","files":47,"transforms_applied":207,"transforms_review":9,"tier":1}
```

## Crate Architecture

```
crates/
  cobol-rustify/                    # NEW crate
    Cargo.toml                      # deps: syn, quote, proc-macro2, serde, sha2
    src/
      lib.rs                        # Public API: analyze(), apply(), report()
      hints.rs                      # Parse/write hints.json
      manifest.rs                   # Parse/write manifest.json
      patch.rs                      # Patch detection (checksum comparison)
      report.rs                     # Report generation (text, json, ndjson)
      review.rs                     # Generate review.md
      plan.rs                       # Generate plan.md (Tier 3)
      rules/
        mod.rs                      # Rule trait, registry, tier classification
        tier1/
          mod.rs
          const_extract.rs          # T1-CONST
          zero_literal.rs           # T1-ZERO
          dead_field.rs             # T1-DEAD
          allow_cleanup.rs          # T1-ALLOW
          unused_import.rs          # T1-IMPORT
          display_simplify.rs       # T1-DISPLAY
        tier2/
          mod.rs
          pic_to_string.rs          # T2-STR
          packed_to_native.rs       # T2-NUM
          localize_vars.rs          # T2-LOCAL
          bool_extract.rs           # T2-BOOL
          enum_extract.rs           # T2-ENUM
        tier3/
          mod.rs
          dispatcher_analysis.rs    # T3-DISPATCH
          ws_decomposition.rs       # T3-WS
          status_to_result.rs       # T3-RESULT
          io_modernize.rs           # T3-IO
  cobol-cli/
    src/
      rustify.rs                    # CLI subcommand handler
  cobol-transpiler/
    src/
      hints.rs                      # NEW: emit hints.json during transpile
```

### Rule Trait

```rust
pub trait RustifyRule: Send + Sync {
    /// Unique rule identifier (e.g., "const-extract").
    fn id(&self) -> &'static str;

    /// Human-readable description.
    fn description(&self) -> &'static str;

    /// Which tier this rule belongs to.
    fn tier(&self) -> Tier;

    /// Analyze a file and return proposed transforms.
    fn analyze(
        &self,
        source: &syn::File,
        hints: Option<&FileHints>,
    ) -> Vec<Transform>;

    /// Apply transforms to produce modified source.
    fn apply(
        &self,
        source: &syn::File,
        transforms: &[Transform],
    ) -> syn::File;
}
```

### Transform Struct

```rust
pub struct Transform {
    pub rule_id: String,
    pub file: PathBuf,
    pub line: usize,
    pub description: String,
    pub safety: Safety,
    pub detail: TransformDetail,
}

pub enum Safety {
    /// Fully automatic, no review needed.
    Auto,
    /// Needs engineer review (with reason).
    Review { reason: String, recommendation: String },
    /// Assessment only, no auto-apply (Tier 3).
    Assessment,
}

pub enum TransformDetail {
    /// Replace a span of tokens with new tokens.
    Replace { old: String, new: String },
    /// Remove a span (dead field, unused import).
    Remove { target: String },
    /// Insert new code (const declaration).
    Insert { position: InsertPosition, code: String },
    /// Structural note (Tier 3 plan item).
    Note { category: String, content: String },
}
```

## Transpiler Hints Emission

The transpiler already collects the information needed for hints.json:
- Field declarations with PIC, USAGE, LEVEL from AST
- REDEFINES relationships from AST
- CALL BY REFERENCE parameters from PROCEDURE DIVISION analysis
- MOVE CORRESPONDING targets from PROCEDURE DIVISION analysis
- Field read/write counts from codegen traversal
- Paragraph call graph from PERFORM analysis

### Changes to cobol-transpiler

1. Add `hints.rs` module with `HintsCollector` that accumulates field metadata
   during codegen traversal (the same pass that generates Rust code)
2. At end of `generate_rust_for_program()`, serialize hints to JSON
3. Write `<output>/.rustify/hints.json` alongside the generated Rust
4. New CLI flag: `--emit-hints` (default: on for `--workspace` mode)

## Implementation Phases

### Phase 1: Infrastructure (2-3 sessions)
- cobol-rustify crate skeleton
- Rule trait and registry
- Manifest read/write
- Hints read (not yet emitted by transpiler)
- Report generation (text format)
- CLI subcommand wiring (report mode only)

### Phase 2: Tier 1 Rules (2-3 sessions)
- Implement 6 Tier 1 rules using syn for Rust parsing
- Transpiler emits hints.json (at least read/write counts for dead-field)
- Apply mode (write to output directory)
- Patch detection
- NDJSON report format

### Phase 3: Tier 2 Rules (3-4 sessions)
- Full hints.json emission from transpiler
- Implement 5 Tier 2 rules with safety gate analysis
- review.md generation
- Validate on corpus subset (5-10 programs across complexity tiers)

### Phase 4: Tier 3 Assessment (2 sessions)
- Implement 4 Tier 3 analysis rules
- plan.md generation
- Validate on critical client files

### Phase 5: Polish (1-2 sessions)
- Performance optimization for large files (1M line programs)
- Parallel file processing
- JSON report format
- Edge case hardening from real-world usage

## Testing Strategy

### Unit Tests
- Each rule has tests with before/after Rust snippets
- Hints parsing and manifest round-trip tests
- Patch detection tests

### Integration Tests
- Transpile a COBOL file -> apply Tier 1 -> cargo check passes
- Transpile -> Tier 1 -> Tier 2 -> cargo check passes
- Idempotency: apply twice, output identical (checksum match)
- Patch preservation: modify file, re-run with --preserve-patches

### Corpus Validation
- Run Tier 1 on 100 transpiled programs from corpus, all must cargo check
- Run Tier 2 on 20 selected programs, verify review.md catches all REDEFINES
- Run Tier 3 on 5 programs, verify plan.md is useful

## Success Metrics

| Metric | Tier 1 Target | Tier 2 Target | Tier 3 Target |
|--------|--------------|--------------|--------------|
| Line count reduction | 5-15% | 20-35% | Assessment only |
| Runtime crate imports reduced | Minimal | 30-50% fewer | N/A |
| cargo clippy warnings | Fewer | Significantly fewer | N/A |
| Idiomatic Rust patterns | Constants, cleanup | Native types, locals | Plan for full idiom |
| Engineer review items | 0 | <15% of transforms | 100% (by design) |
| cargo check after apply | 100% pass | 100% pass | N/A |

## Relationship to Existing Specs

- **cobol2rust_rustification_roadmap.md**: This spec implements the roadmap's
  3-tier model as a CLI tool with the promotion pipeline UX
- **cobol2rust_pipeline_spec.md**: Rustify is NOT part of the 6-phase pipeline.
  It runs AFTER the pipeline completes (post-transpile)
- **cobol2rust_transpile_reporting_spec.md**: NDJSON format is consistent with
  existing reporting infrastructure
