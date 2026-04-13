# NexStudio Rust Code Review

## Against Microsoft Pragmatic Rust Guidelines

**Date**: 2026-03-27
**Codebase**: /Users/chandramohn/workspace/nexstudio/src-tauri/
**Size**: 2,669 lines across 9 Rust files, 47 Tauri commands
**Scope**: Report only (no changes made)

---

## Executive Summary

| Category | Score | Notes |
|---|---|---|
| Safety | PASS | Zero unsafe, proper Arc/Mutex |
| Error Handling | NEEDS WORK | All errors collapsed to String |
| Documentation | PARTIAL | Good function docs, no rustdoc examples |
| Testing | FAIL | Zero unit tests |
| Performance | GOOD | Dev opt-level, lazy loading, depth limits |
| Naming | GOOD | Consistent conventions |
| Security | NEEDS WORK | Symlink traversal, no input validation |

---

## Guideline-by-Guideline Assessment

### PASS (Compliant)

| Guideline | Status | Evidence |
|---|---|---|
| M-UNSAFE | PASS | Zero unsafe blocks in entire codebase |
| M-UNSOUND | PASS | No unsafe = no soundness risk |
| M-UNSAFE-IMPLIES-UB | PASS | N/A (no unsafe used) |
| M-PUBLIC-DEBUG | PASS | All public structs derive Debug |
| M-PANIC-IS-STOP | PASS | Only 1 .expect() at app entry point (lib.rs) |
| M-PANIC-ON-BUG | PASS | No panics in business logic, errors propagated |
| M-SERVICES-CLONE | PASS | ViewerState uses Arc<Mutex<HashMap>> pattern |
| M-TYPES-SEND | PASS | All types are Send (no Rc, no RefCell) |
| M-AVOID-STATICS | PASS | No static or thread_local in library code |
| M-FEATURES-ADDITIVE | PASS | No conflicting feature flags |
| M-SMALLER-CRATES | PASS | Already split: cobol-data, cobol-transpiler as deps |
| M-CONCISE-NAMES | PASS | No "Manager"/"Service"/"Factory" weasel words |
| M-OOBE | PASS | Compiles without external prerequisites |

### NEEDS WORK (Partially Compliant)

| Guideline | Gap | Detail |
|---|---|---|
| M-APP-ERROR | String errors | All 22 command handlers return `Result<T, String>`. Should use anyhow, eyre, or custom error enum with Serialize. No error categorization for UI. |
| M-ERRORS-CANONICAL-STRUCTS | No error structs | Errors are format!() strings. No Backtrace, no upstream cause chain, no helper methods. Example: `map_err(\|e\| format!("Cannot read {}: {}", path, e))` loses the original error type. |
| M-CANONICAL-DOCS | Missing sections | Function docs exist but lack: Examples, Errors, Panics sections. 31+ functions documented but none have rustdoc examples. |
| M-FIRST-DOC-SENTENCE | Mostly OK | Most first sentences are concise. A few are multi-line. |
| M-MODULE-DOCS | Partial | data_viewer.rs has //! docs. Other modules (commands.rs, lineage.rs, migration.rs, cluster.rs, search.rs) have none. |
| M-DOCUMENTED-MAGIC | Some gaps | Depth limits (3, 5, 10) used in recursive functions without explaining why those values. Search max_results=500 undocumented. |
| M-LOG-STRUCTURED | Missing | Zero logging in entire codebase. No tracing, no log crate. Silent failures on non-critical paths. |
| M-STATIC-VERIFICATION | Partial | Uses clippy (workspace lints in Cargo.toml). Missing: cargo-audit, cargo-udeps, cargo-hack, miri. No CI verification pipeline visible. |
| M-STRONG-TYPES | Partial | Good use of enums and Option. But paths are `String` not `PathBuf` in command signatures. JSON config accessed via untyped serde_json::Value indexing. |
| M-MOCKABLE-SYSCALLS | Partial | ViewerSession uses generic FileAccess trait (good). But migration.rs calls Command::new() directly with no abstraction. File I/O in commands.rs not mockable. |
| M-IMPL-ASREF | Not used | Command handlers take `String` instead of `impl AsRef<Path>` for file paths. (Acceptable for Tauri IPC boundary -- Tauri deserializes from JSON.) |
| M-DI-HIERARCHY | Partial | ViewerState uses concrete types. migration.rs hardcodes binary paths. No dependency injection for external tool paths. |
| M-HOTPATH | Unknown | No benchmarks. Dev profile optimizes parser crates (good), but no profiling of command handlers. |

### FAIL (Non-Compliant)

| Guideline | Gap | Detail |
|---|---|---|
| M-DESIGN-FOR-AI | No tests | Guideline specifically says "Ensure Test Coverage" for AI refactoring. Zero #[test] in codebase. |
| M-TEST-UTIL | N/A | No tests exist to need test-util features. |
| M-PUBLIC-DISPLAY | Missing | Public types (FileEntry, SearchResult, FieldInfo, etc.) derive Debug but not Display. |
| M-LINT-OVERRIDE-EXPECT | Not checked | No #[allow] or #[expect] found. (Neutral -- no lint overrides needed.) |

### NOT APPLICABLE

| Guideline | Why |
|---|---|
| M-MIMALLOC-APPS | Tauri manages allocator; not user-controlled |
| M-ISOLATE-DLL-STATE | No DLL loading |
| M-FFI / M-ESCAPE-HATCHES | No FFI in application code |
| M-SYS-CRATES | No -sys crates |
| M-NO-GLOB-REEXPORTS | No glob re-exports found |
| M-DOC-INLINE | No re-exports |
| M-YIELD-POINTS | Uses spawn_blocking, not long-running async loops |
| M-THROUGHPUT | Not a throughput-oriented application |
| M-INIT-BUILDER / M-INIT-CASCADED | No types with 4+ params in public API |
| M-AVOID-WRAPPERS | No generic wrappers in public API |
| M-SIMPLE-ABSTRACTIONS | API surface is flat (Tauri commands) |
| M-IMPL-IO / M-IMPL-RANGEBOUNDS | Not applicable to Tauri command layer |
| M-ESSENTIAL-FN-INHERENT | No trait-heavy design |
| M-REGULAR-FN | Functions are already regular (not associated) |
| M-DONT-LEAK-TYPES | Internal application, no downstream consumers |
| M-M-UPSTREAM-GUIDELINES | Meta-guideline |

---

## File-Level Findings

### commands.rs (965 lines) -- Tauri IPC Surface

**Issues:**
- All 22 commands return `Result<T, String>` -- no structured errors
- JSON config accessed via untyped indexing: `config["source"]["repoPath"].as_str().unwrap_or("")`
  - Silent empty string on missing keys masks config errors
  - Should deserialize into typed Config struct
- Path parameters are `String`, not validated before use
- No symlink checks on directory traversal (potential security issue)
- Recursive depth limits inconsistent: some use 3, some 5, some 10

**Strengths:**
- Good function documentation (28 functions documented)
- Consistent error context in map_err messages
- Proper use of Option for optional parameters

### cobol_parser.rs (452 lines) -- COBOL Field Parser

**Issues:**
- 1 `.unwrap()` at line 341 (stack.pop() after guard -- safe but should use .expect() with reason per M-PANIC-ON-BUG)
- Complex parsing logic (lines 295-317) lacks inline comments
- No module-level docs

**Strengths:**
- Handles edge cases: column stripping, continuation lines, PIC IS variant
- Proper level number validation (01-49, 66, 77, 88)
- Good struct documentation

### migration.rs (363 lines) -- External Binary Integration

**Issues:**
- Hardcoded binary name "cobol2rust" -- not configurable
- Command::new() not abstracted (not mockable for testing)
- Error messages could lose context in spawn_blocking chain

**Strengths:**
- Proper async spawn_blocking for blocking process calls
- Good output parsing and structured result types

### lineage.rs (265 lines) -- Call Graph Extraction

**Issues:**
- Regex patterns for CALL/COPY extraction not documented
- No module-level docs
- No tests for regex edge cases

**Strengths:**
- Clean extraction of CALL graph and copybook dependencies
- Proper result types

### cluster.rs (267 lines) -- Program Clustering

**Issues:**
- No module docs
- Clustering algorithm not documented with complexity analysis
- Magic threshold values undocumented

**Strengths:**
- Safe string handling with unwrap_or fallbacks
- Clean graph-based clustering

### data_viewer.rs (155 lines) -- ViewerState

**Issues:**
- Mutex lock held across I/O operations (acceptable for current scale)

**Strengths:**
- Only file with module-level //! docs
- Clean Arc<Mutex<HashMap>> pattern
- Generic FileAccess trait enables testability

### search.rs (146 lines) -- Text Search

**Issues:**
- No module docs
- max_results=500 hardcoded without documentation

**Strengths:**
- Early termination on result limit
- Clean SearchResponse type

---

## Top Recommendations (Prioritized)

### P0: Testing (M-DESIGN-FOR-AI compliance)

Zero tests is the single biggest gap. Without tests, safe refactoring
(by humans or AI) is impossible.

Recommended test targets:
1. cobol_parser.rs -- field parsing edge cases, REDEFINES, OCCURS
2. lineage.rs -- regex extraction patterns
3. cluster.rs -- clustering algorithm correctness
4. commands.rs -- error paths, invalid input handling

### P1: Error Handling (M-APP-ERROR, M-ERRORS-CANONICAL-STRUCTS)

Replace `Result<T, String>` with structured errors:

```rust
// Option A: anyhow (simplest, per M-APP-ERROR)
use anyhow::Result;

// Option B: custom enum (richer, enables UI error handling)
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum AppError {
    FileNotFound { path: String },
    ParseError { file: String, detail: String },
    ConfigError { field: String, detail: String },
    MigrationError { program: String, stderr: String },
    IoError { detail: String },
}
```

### P2: Module Documentation (M-MODULE-DOCS)

Add //! module docs to: commands.rs, lineage.rs, migration.rs,
cluster.rs, search.rs, cobol_parser.rs.

### P3: Structured Logging (M-LOG-STRUCTURED)

Add tracing crate with structured events for:
- Command invocations (which command, duration)
- Error occurrences (with context)
- Migration tool invocations (binary path, args, exit code)
- Discovery scan progress

### P4: Input Validation (M-STRONG-TYPES)

- Typed Config struct instead of serde_json::Value indexing
- Path validation (symlink checks, canonicalization)
- Consistent recursive depth limits with documented rationale

### P5: Display Implementations (M-PUBLIC-DISPLAY)

Add Display for user-facing types: FileEntry, SearchResult,
MigrationResult, FieldInfo.

---

## Metrics Summary

```
Total guidelines assessed:  52
Applicable:                 35
Passing:                    13  (37%)
Needs work:                 14  (40%)
Failing:                     4  (11%)
Not applicable:             17

Lines of code:           2,669
Unit tests:                  0
Unsafe blocks:               0
Unwrap calls:                1  (safe)
Error types:          String only
Logging:                  none
Module docs:             1 of 7
```
