# Silent Failure Audit -- cobol2rust Workspace

**Date**: 2026-04-06
**Version**: 0.2.6
**Scope**: All `.rs` files in `crates/` (non-test production code)

Full-codebase audit of patterns that swallow errors, discard error context,
or silently produce default values on failure. Three passes completed.
49 sites fixed across 23 files.

---

## Pass 1 -- Status: FIXED

All items from the initial audit have been instrumented. Behavior is
unchanged (same defaults/fallbacks), but diagnostics are now emitted.

### CRITICAL (C1-C5)

| ID | File | Status | What was done |
|----|------|--------|---------------|
| C1 | `cobol-core/src/numeric_parse.rs:46` | DEFERRED | COBOL-correct zero-on-bad-data; needs logging framework for hot path |
| C2 | `cobol-transpiler/src/parser/proc_listener.rs` | **FIXED** | 17 name-extraction sites now emit `TranspileDiagnostic` via `warn_missing_name()`. 21 optional-clause sites left alone (correct semantics). |
| C3 | `cobol-transpiler/src/parser/data_listener.rs` | **FIXED** | Added `diagnostics: Vec<TranspileDiagnostic>` field. 3 `.ok()` sites (PIC, OCCURS count, OCCURS max) now push diagnostics. Wired to stderr via `parser/mod.rs`. |
| C4 | `cobol-transpiler/src/symbol_table.rs:361` | SKIPPED | Test-only code (`#[cfg(test)]` module). |
| C5 | `cobol-types/src/numeric_edited.rs:363` | DEFERRED | Same as C1: COBOL semantics. |

### HIGH (H1-H10)

| ID | File | Status | What was done |
|----|------|--------|---------------|
| H1 | `cobol-cli/src/audit_cmd.rs` | **FIXED** | Added `AtomicU32` read_error_count, `read_errors` field on `AuditSummary` with `#[serde(default)]`, eprintln on all 3 Err sites, count shown in text report. |
| H2 | `cobol-cli/src/corpus_cmd.rs` | **FIXED** | Replaced `.ok()` chains in `load_repo_meta()` with explicit match + eprintln. Added eprintln on directory read failure. |
| H3 | `cobol-cli/src/diagnose_cmd.rs` | **FIXED** | Added `read_errors` counter + eprintln on 2 Err sites, summary printed at end. |
| H4 | `cobol-cli/src/scan/phase1.rs` | **FIXED** | Panic payload extracted via `downcast_ref::<String>()` / `downcast_ref::<&str>()`. |
| H5 | `cobol-cli/src/scan/phase2.rs` | **FIXED** | Same as H4. |
| H6 | `cobol-cli/src/scan/db.rs` | **FIXED** | See Pass 2 item N3. Replaced `.filter_map(\|r\| r.ok())` with explicit match + eprintln. |
| H7 | `cobol-cli/src/transpile_cmd.rs` | ACCEPTABLE | Lines 370, 379 already return `TranspileOutcome::Failed`. No silent swallowing. |
| H8 | `cobol-transpiler/src/parser/copy_expand.rs` | **FIXED** | Distinguishes `CopyNotFound` from other errors; eprintln on non-NotFound failures. |
| H9 | `cobol-rustify/src/rules/tier4/structural.rs` | **FIXED** | Replaced `syn::parse_file().ok()` with match + eprintln on 2 sites. |
| H10 | `cobol-io/src/indexed.rs, relative.rs, sequential.rs` | DEFERRED | Runtime code, COBOL semantics. |

### MEDIUM (M1-M10)

| ID | File | Status | What was done |
|----|------|--------|---------------|
| M2 | `cobol-cli/src/scan/worker.rs` | **FIXED** | Panic payload now included in NDJSON `"error"` field. |
| M4 | `cobol-cli/src/corpus_cmd.rs` | **FIXED** | `load_repo_meta()` now logs read and JSON parse errors. |
| M6 | `cobol-rustify/src/dsl/cobol_attrs.rs` | **FIXED** | Level parse failure now emits eprintln warning. |
| M1 | `pipeline/phase0.rs` | ACCEPTABLE | Standard `.flatten()` on directory iteration |
| M3 | `scan/ndjson.rs` | **FIXED** | See Pass 3 item P3 (FileRecord parse errors now logged) |
| M5 | `cobol-intel/src/query/parser.rs` | OPEN | Lower priority; query parse errors lose context |
| M7 | `cobol-rustify/src/dsl/type_mapping.rs` | OPEN | Lower priority; numeric parse returns None |
| M8 | `cobol-data/src/session.rs` | **FIXED** | See Pass 3 items P4+P5 (discriminator + encoding) |
| M9 | `cobol-core/src/decimal_ext.rs` | OPEN | Lower priority; scale setting failure |
| M10 | various | ACCEPTABLE | Standard `.flatten()` on directory/line iterators |

---

## Pass 2 -- Status: FIXED

Second full-codebase scan found additional sites missed in Pass 1.
All HIGH items now instrumented.

### NEW CRITICAL / HIGH

#### N1: codegen/proc_gen.rs -- numeric parse defaults to 0 in GENERATED code

- **File**: `crates/cobol-transpiler/src/codegen/proc_gen.rs`
- **Lines**: 1443, 1563
- **Patterns**:
  - Line 1443: generates `.parse::<i64>().unwrap_or(0)` in output Rust code for GO TO DEPENDING
  - Line 1563: generates `.parse::<i64>().unwrap_or(0)` in output Rust code for CALL BY VALUE
- **Impact**: This is runtime behavior in generated code, not a transpiler bug.
  COBOL semantics: invalid data in GO TO DEPENDING means no transfer (index 0
  falls through the match). Acceptable.
- **Severity**: RECLASSIFIED as ACCEPTABLE (runtime COBOL semantics)

#### N2: parser/file_listener.rs -- FD level number parse failure -- FIXED

- **File**: `crates/cobol-transpiler/src/parser/file_listener.rs:245`
- **Pattern**: `.parse::<u8>().unwrap_or(0)` on INTEGERLITERAL level number
- **Impact**: File section records with malformed level numbers silently get
  level 0, which fails the 01-level check and the record is silently dropped.
  Entire FD record hierarchies can vanish.
- **Fix applied**: Replaced with explicit match; emits `[WARN] FD: invalid level number` and returns early.

#### N3: scan/db.rs -- DuckDB row errors silently filtered (H6 from Pass 1) -- FIXED

- **File**: `crates/cobol-cli/src/scan/db.rs`
- **Lines**: 332, 390
- **Pattern**: `.filter_map(|r| r.ok())`
- **Impact**: Row extraction errors in `get_processed_file_ids()` and
  `get_parseable_uncovered_files()` silently dropped. Resume logic may
  reprocess files or skip them.
- **Fix applied**: Replaced with explicit match; emits `[WARN] DuckDB row error` with function name.

#### N4: scan/phase3.rs -- coverage histogram row errors -- FIXED

- **File**: `crates/cobol-cli/src/scan/phase3.rs:269`
- **Pattern**: `.filter_map(|r| r.ok())`
- **Impact**: Coverage distribution histogram silently loses buckets.
- **Fix applied**: Replaced with explicit match; emits `[WARN] DuckDB row error in coverage histogram`.

#### N5: scan/discover.rs -- directory entry errors silently skipped -- FIXED

- **File**: `crates/cobol-cli/src/scan/discover.rs:109`
- **Pattern**: `Err(_) => continue` on directory entry
- **Impact**: Files silently excluded from discovery. Large batches could
  lose significant portions of codebase.
- **Fix applied**: Emits `[WARN] Cannot read directory entry in {dir}` with error.

#### N6: discover_cmd.rs -- file read and parse failures silently skipped -- FIXED

- **File**: `crates/cobol-cli/src/discover_cmd.rs`
- **Lines**: 173, 206, 226
- **Patterns**:
  - Line 173: `Err(_) => continue` on file read in data file scanning
  - Line 206: `Err(_) => continue` on file read in program scanning
  - Line 226: `Err(_) => vec![]` on parse failure in program scanning
- **Impact**: Files that fail to read or parse are silently skipped. Program
  linking and data file discovery are incomplete.
- **Fix applied**: All 3 sites now emit `[WARN]` with filename and error message.

#### N7: rustify/lib.rs -- program name extraction failure silently skips files -- FIXED

- **File**: `crates/cobol-rustify/src/lib.rs:428`
- **Pattern**: `Err(_) => continue` on `syn::parse_str()` (the skip was here, not on program name)
- **Impact**: Files that fail to parse are silently skipped from DSL emission.
- **Fix applied**: Emits `[WARN] Cannot parse Rust file {path}` with syn error. The
  `unwrap_or_default()` on program name is intentional: empty name = not a COBOL file.

#### N8: rules/tier4/dispatch.rs -- empty dispatch body on extraction failure -- FIXED

- **File**: `crates/cobol-rustify/src/rules/tier4/dispatch.rs:263`
- **Pattern**: `.unwrap_or_default()` on `extract_fn_body()`
- **Impact**: If function body extraction fails, generates an empty body in
  dispatch code. Produces broken output without error signal.
- **Fix applied**: Replaced with `unwrap_or_else` + eprintln warning naming the function.

### NEW MEDIUM

#### N9: codegen/proc_gen.rs -- INSPECT TALLYING defaults to 0

- **File**: `crates/cobol-transpiler/src/codegen/proc_gen.rs:3144`
- **Pattern**: `.to_u32().unwrap_or(0)` in INSPECT TALLYING
- **Impact**: Invalid tally field silently becomes 0 instead of error.
- **Severity**: MEDIUM

#### N10: cobol-data/export.rs -- missing CSV field becomes empty string

- **File**: `crates/cobol-data/src/export.rs:93`
- **Pattern**: `.unwrap_or_default()` on record field lookup
- **Impact**: Missing fields export as empty strings without warning.
- **Severity**: MEDIUM

#### N11: cobol-data/session.rs -- same CSV field pattern

- **File**: `crates/cobol-data/src/session.rs:291`
- **Pattern**: `.unwrap_or_default()` on record field lookup
- **Impact**: Same as N10.
- **Severity**: MEDIUM

#### N12: cobol-intel query parser -- error context lost

- **File**: `crates/cobol-intel/src/query/parser.rs`
- **Lines**: 137, 144, 147
- **Pattern**: `.ok()?` / `.ok().map(...)` on parse results
- **Impact**: Query parse errors lose their messages.
- **Severity**: MEDIUM

### NEW -- Acceptable (no action)

| File | Line(s) | Pattern | Rationale |
|------|---------|---------|-----------|
| `codegen/data_gen.rs` | 342, 473, 587, 746 | `.unwrap_or(0)` on byte_length | Defensive; 0 bytes is safe default |
| `codegen/java/mod.rs` | 37, 43 | `.unwrap_or_default()` on data division | Empty Vec is valid for empty programs |
| `parser/hierarchy.rs` | 120 | `.unwrap_or(0)` on byte_length | Layout computation, safe default |
| `parser/pic_parser.rs` | 412 | `.ok()?` | Proper propagation via `?` |
| `scan/discover.rs` | 135, 183 | `.unwrap_or_default()` on path ext | Cosmetic/classification only |
| `data_analyze_cmd.rs` | 126-147 | `.unwrap_or_default()` x4 | Display formatting only |
| `rustify/lib.rs` | 385 | `.unwrap_or_default()` on SystemTime | Epoch-0 astronomically unlikely |
| `build_graph_cmd.rs` | 150, 152 | `Err(_) => None` | Already logs warning |
| `java/proc_gen.rs` | 1297, 1298, 1345 | `.unwrap_or_default()` on sort file names | Comment text only, not codegen logic |

---

## Patterns Summary (updated)

| Pattern | Count | Fixed | Remaining |
|---------|-------|-------|-----------|
| `.unwrap_or_default()` name extraction (proc_listener) | 17 | 17 | 0 |
| `.ok()` in data_listener | 3 | 3 | 0 |
| CLI Err(_) skip (audit, diagnose, corpus) | 7 | 7 | 0 |
| catch_unwind payload (phase1, phase2, worker) | 3 | 3 | 0 |
| copy_expand CopyNotFound distinction | 1 | 1 | 0 |
| syn parse .ok() (structural) | 2 | 2 | 0 |
| cobol_attrs level parse | 1 | 1 | 0 |
| **Pass 2: codegen numeric parse** | 2 | -- | 0 (reclassified: runtime COBOL semantics) |
| **Pass 2: file_listener level parse** | 1 | 1 | 0 |
| **Pass 2: DuckDB row filter** | 3 | 3 | 0 |
| **Pass 2: discover Err skip** | 4 | 4 | 0 |
| **Pass 2: rustify parse/body defaults** | 2 | 2 | 0 |
| **Pass 2: medium** | 4 | 0 | 4 (N9-N12, lower priority) |
| **Pass 3: NDJSON parse in reader threads** | 2 | 2 | 0 |
| **Pass 3: FileRecord parse in resume** | 1 | 1 | 0 |
| **Pass 3: session discriminator parse** | 1 | 1 | 0 |
| **Pass 3: session encoding detection** | 1 | 1 | 0 |

**Total fixed**: 49 sites across 3 passes
**Total remaining**: 4 MEDIUM (N9-N12) + 4 DEFERRED (C1, C5, H10, need logging framework)

---

## Relationship to Known Bugs

### Monster File (SSUPDATE, 439K lines, W003)

The 0-paragraph / empty-working-storage problem is almost certainly caused by
C2 + C3 + N2 working together:

1. `proc_listener.rs` (C2, FIXED): 17 name-extraction sites now emit diagnostics
   when the parse tree node is missing.

2. `data_listener.rs` (C3, FIXED): PIC and OCCURS parse failures now push
   diagnostics instead of silently returning None.

3. `file_listener.rs` (N2, FIXED): FD level number parse failure now emits
   warning and returns early instead of silently dropping records.

4. All three diagnostics will now surface in the next client run,
   revealing WHERE the parser fails on the monster file.

### Audit Score (95.9)

H1 is FIXED. The `read_errors` field now appears in `AuditSummary` JSON
and text output. Re-running will show the true count of skipped files.

### COPY Expansion

H8 is FIXED. Non-NotFound errors (permissions, encoding) now print a
`[WARN]` to stderr with the actual error, while still recording the
copybook as missing for downstream processing.

---

## Pass 3 -- Status: FIXED

Third full-codebase scan focused on subtle patterns: `if let Ok(...)` without
else, NDJSON parse failures in reader threads, and data session error handling.

### P1: scan/phase1.rs -- NDJSON reader thread parse failures -- FIXED

- **File**: `crates/cobol-cli/src/scan/phase1.rs:394`
- **Pattern**: `if let Ok(val) = serde_json::from_str(...)` with no else
- **Impact**: Malformed worker NDJSON output silently discarded. Lost analysis data.
- **Fix applied**: Replaced with match; emits `[WARN] Phase 1 reader {i}: failed to parse NDJSON`.

### P2: scan/phase2.rs -- coverage worker NDJSON parse failures -- FIXED

- **File**: `crates/cobol-cli/src/scan/phase2.rs:249`
- **Pattern**: Same as P1
- **Impact**: Coverage worker output parse failures silently discarded.
- **Fix applied**: Same pattern as P1.

### P3: scan/ndjson.rs -- FileRecord parse errors in resume -- FIXED

- **File**: `crates/cobol-cli/src/scan/ndjson.rs:728`
- **Pattern**: `if let Ok(record) = serde_json::from_str::<FileRecord>(...)` with no else
- **Impact**: Malformed records in files.ndjson silently skipped. Resume logic
  could get wrong max_file_id if corrupted entries appear after the true max.
- **Fix applied**: Replaced with match; emits `[WARN] Malformed FileRecord in {path}`.

### P4: cobol-data/session.rs -- discriminator detection parse failure -- FIXED

- **File**: `crates/cobol-data/src/session.rs:118`
- **Pattern**: `if let Ok(program) = parser::parse_cobol(...)` with no else
- **Impact**: If program source fails to parse, discriminator detection silently
  does not run. Data variant selection falls back to defaults with no warning.
- **Fix applied**: Replaced with match; emits `[WARN] Failed to parse program source for discriminator detection`.

### P5: cobol-data/session.rs -- encoding auto-detection read failure -- FIXED

- **File**: `crates/cobol-data/src/session.rs:132`
- **Pattern**: `if let Ok(sample) = self.fa.read_bytes(...)` with no else
- **Impact**: If sample read fails (permissions, I/O), encoding detection is
  silently skipped and defaults to EBCDIC. Could misinterpret ASCII data.
- **Fix applied**: Replaced with match; emits `[WARN] Cannot read sample for encoding detection`.

---

## Files Changed

### Pass 1

| File | Changes |
|------|---------|
| `crates/cobol-transpiler/src/parser/proc_listener.rs` | Added `warn_missing_name()` helper; 17 sites instrumented |
| `crates/cobol-transpiler/src/parser/data_listener.rs` | Added diagnostics field; 3 `.ok()` sites instrumented |
| `crates/cobol-transpiler/src/parser/mod.rs` | Wire data_listener diagnostics to stderr |
| `crates/cobol-transpiler/src/parser/copy_expand.rs` | Distinguish CopyNotFound from other errors |
| `crates/cobol-cli/src/audit_cmd.rs` | AtomicU32 counter, `read_errors` field, 3 Err sites + text report |
| `crates/cobol-cli/src/diagnose_cmd.rs` | read_errors counter, 2 Err sites + summary |
| `crates/cobol-cli/src/corpus_cmd.rs` | Explicit match in load_repo_meta, directory read warning |
| `crates/cobol-cli/src/scan/phase1.rs` | Panic payload extraction |
| `crates/cobol-cli/src/scan/phase2.rs` | Panic payload extraction |
| `crates/cobol-cli/src/scan/worker.rs` | Panic payload in NDJSON error field |
| `crates/cobol-rustify/src/rules/tier4/structural.rs` | syn parse match + eprintln on 2 sites |
| `crates/cobol-rustify/src/dsl/cobol_attrs.rs` | Level parse warning |

### Pass 2

| File | Changes |
|------|---------|
| `crates/cobol-transpiler/src/parser/file_listener.rs` | FD level parse: explicit match + eprintln + return |
| `crates/cobol-cli/src/scan/db.rs` | 2 filter_map sites: explicit match + eprintln |
| `crates/cobol-cli/src/scan/phase3.rs` | Coverage histogram filter_map: explicit match + eprintln |
| `crates/cobol-cli/src/scan/discover.rs` | Directory entry error: eprintln with dir path |
| `crates/cobol-cli/src/discover_cmd.rs` | 3 sites: file read + parse failure warnings |
| `crates/cobol-rustify/src/lib.rs` | syn parse failure in DSL emission: eprintln with path |
| `crates/cobol-rustify/src/rules/tier4/dispatch.rs` | extract_fn_body failure: eprintln with function name |

### Pass 3

| File | Changes |
|------|---------|
| `crates/cobol-cli/src/scan/phase1.rs` | NDJSON reader thread: match on parse result + eprintln |
| `crates/cobol-cli/src/scan/phase2.rs` | Coverage reader thread: match on parse result + eprintln |
| `crates/cobol-cli/src/scan/ndjson.rs` | FileRecord parse: match + eprintln with path |
| `crates/cobol-data/src/session.rs` | Discriminator parse + encoding sample read: match + eprintln |

---

## Pending Items (not blocking release)

### MEDIUM -- Lower priority, no behavior change needed

| ID | File | Pattern | Why deferred |
|----|------|---------|--------------|
| N9 | `codegen/proc_gen.rs:3144` | `.to_u32().unwrap_or(0)` INSPECT TALLYING | Generated runtime code; COBOL semantics |
| N10 | `cobol-data/src/export.rs:93` | `.unwrap_or_default()` CSV field | Empty string is valid CSV output for missing field |
| N11 | `cobol-data/src/session.rs:291` | `.unwrap_or_default()` CSV field | Same as N10 |
| N12 | `cobol-intel/src/query/parser.rs:137,144,147` | `.ok()?` parse results | Query rejection works; only error message is lost |
| M5 | `cobol-intel/src/query/parser.rs` | `.ok()?` | Same as N12 |
| M7 | `cobol-rustify/src/dsl/type_mapping.rs:215` | `.parse().ok()` | Fallback type used; cosmetic |
| M9 | `cobol-core/src/decimal_ext.rs:83` | `.set_scale().ok()` | Extremely rare; scale overflow on >28 digits |

### DEFERRED -- Needs architectural change

| ID | File | Pattern | Why deferred |
|----|------|---------|--------------|
| C1 | `cobol-core/src/numeric_parse.rs:46` | `unwrap_or(Decimal::ZERO)` | COBOL-correct; needs `tracing` crate for hot-path logging |
| C5 | `cobol-types/src/numeric_edited.rs:363` | `Err(_) => Some(Decimal::ZERO)` | Same as C1 |
| H10 | `cobol-io/src/*.rs` | `Err(_) => FileStatusCode::PERM_ERROR` | Runtime code; all I/O errors map to 2-byte COBOL status codes by design |

### ACCEPTABLE -- No action needed

~30 sites across the workspace confirmed as correct Rust idioms:
- `.flatten()` on directory iterators (standard `read_dir` pattern)
- `.unwrap_or(0)` on `byte_length` fields (defensive layout computation)
- `.unwrap_or_default()` on optional display fields (formatting only)
- `let _ = writeln!()` / `.flush().ok()` (stdout/stderr output)
- `.strip_prefix().ok()` in `filter_map` (correct path filtering)
- `while let Ok(...) = rx.recv()` (normal mpsc channel shutdown)
- Generated runtime code defaults (COBOL semantics: bad data = zero)
