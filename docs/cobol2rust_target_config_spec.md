# Target Architecture Config Spec

## Overview

`.cobol2rust-target.toml` captures the target architecture decisions that Tier 4
(structural transformation) rules consume. This decouples the creative/architectural
decisions (made by a human or AI architect) from the mechanical code transformations
(executed deterministically by Tier 4 rules).

**Design Principles:**
- Config is the boundary between judgment and automation
- Every config value maps to a deterministic transformation
- Sensible defaults mean the file can be minimal
- Per-program overrides are supported
- Schema is extensible for future backends

## Schema

### [control_flow] -- Dispatcher Elimination

Controls how the `match _pc { ... }` paragraph dispatch table is transformed.

```toml
[control_flow]
# How to handle the dispatch loop
# "inline"        -- inline leaf paragraphs at call sites (default)
# "functions"     -- extract each paragraph as a standalone function
# "state-machine" -- keep dispatch but use a proper enum state machine
# "async"         -- convert to async functions with structured concurrency
strategy = "inline"

# Maximum body lines for a paragraph to be inlined (default: 30)
inline_threshold = 30

# Collapse single-call chains: A -> B -> C becomes A_B_C (default: true)
collapse_chains = true

# How to handle detected cycles in the call graph
# "preserve"  -- keep as-is (loop)
# "trampoline" -- convert to trampoline pattern
# "recursive"  -- allow direct recursion (if depth is bounded)
cycle_handling = "preserve"

# Generate structured error propagation from paragraph returns
# "none"     -- no change
# "result"   -- paragraphs return Result<(), ProgramError>
# "anyhow"   -- use anyhow::Result
error_propagation = "result"
```

### [data_model] -- WorkingStorage Decomposition

Controls how the flat `WorkingStorage` struct is decomposed into domain types.

```toml
[data_model]
# How to restructure WorkingStorage
# "flat"       -- keep flat struct, just rename fields (default)
# "nested"     -- extract sub-structs based on prefix groups + co-access
# "split"      -- split into multiple independent structs
# "domain"     -- full domain type extraction (most aggressive)
strategy = "nested"

# Minimum fields in a prefix group to trigger sub-struct extraction (default: 3)
min_group_size = 3

# Whether to generate accessor methods or use pub fields (default: false)
use_accessors = false

# Whether to derive common traits on generated structs
# Options: "minimal" (Debug), "standard" (Debug, Clone), "full" (Debug, Clone, Serialize, Deserialize)
derive_level = "standard"

# Field naming strategy
# "preserve"    -- keep COBOL names (ws_acct_number)
# "strip-prefix" -- remove ws_ prefix (acct_number)
# "rust-idiom"  -- full Rust naming (account_number) [requires mapping]
naming = "strip-prefix"

# Custom sub-struct overrides (takes precedence over auto-detection)
# Format: "StructName" = ["field1", "field2", ...]
[data_model.structs]
# AccountInfo = ["ws_acct_number", "ws_acct_type", "ws_acct_balance"]
# TransactionData = ["ws_txn_date", "ws_txn_amount", "ws_txn_code"]
```

### [error_handling] -- Status Code to Error Type Mapping

Controls how COBOL status-code patterns are converted to Rust error types.

```toml
[error_handling]
# Error handling strategy
# "status-quo"  -- keep numeric status fields as-is (default)
# "result"      -- convert to Result<T, E> with generated error types
# "thiserror"   -- use thiserror-derived error enums
# "anyhow"      -- use anyhow for all errors
strategy = "result"

# How to handle status fields with 2 values (success/failure)
# "bool"     -- convert to bool
# "result"   -- convert to Result<(), Error>
# "option"   -- convert to Option<T>
binary_status = "result"

# How to handle status fields with 3+ values
# "enum"     -- generate an enum per status field
# "result"   -- Result<T, StatusError> with error variants
# "i32"      -- promote to i32 (minimal change)
multi_status = "enum"

# Generate a top-level error enum for the program (default: true)
generate_program_error = true

# Name pattern for generated error types (default: "{Program}Error")
error_type_pattern = "{Program}Error"

# Custom status-to-variant mappings
# Format: "field_name" = { values = { "0" = "Success", "1" = "NotFound" } }
[error_handling.mappings]
# ws_status = { values = { "0" = "Ok", "1" = "InvalidInput", "2" = "NotFound" } }
```

### [io] -- File I/O Modernization

Controls how COBOL file I/O operations are modernized.

```toml
[io]
# Default I/O backend for all file handles
# "file"     -- keep as Rust std::fs file operations (default)
# "database" -- generate database access code
# "api"      -- generate REST/gRPC client calls
# "stream"   -- generate message queue producer/consumer
backend = "file"

# Default database for "database" backend
# "sqlite"   -- rusqlite
# "postgres" -- sqlx with PostgreSQL
# "duckdb"   -- duckdb
database = "sqlite"

# Default message broker for "stream" backend
# "kafka"    -- rdkafka
# "nats"     -- async-nats
# "rabbitmq" -- lapin
broker = "kafka"

# Per-handle backend overrides
# Each handle gets: backend, format, and backend-specific options
[io.handles]
# input_file = { backend = "file", format = "csv" }
# master_file = { backend = "database", table = "master_records" }
# output_file = { backend = "stream", topic = "processed-records" }
# report_file = { backend = "file", format = "json" }

# File format for "file" backend (affects reader/writer generation)
# "fixed"  -- fixed-width records (COBOL default)
# "csv"    -- CSV with headers
# "json"   -- JSON lines
# "parquet" -- Apache Parquet
[io.formats]
default = "fixed"
```

### [services] -- Service Decomposition

Controls how the monolithic program is split into services/modules.

```toml
[services]
# Decomposition granularity
# "monolith"   -- keep as single binary (default)
# "modules"    -- split into Rust modules within one binary
# "libraries"  -- extract shared logic into library crates
# "services"   -- generate separate service binaries
granularity = "modules"

# How to define service boundaries
# "program"    -- one service per COBOL program
# "section"    -- one service per COBOL SECTION
# "paragraph-group" -- group related paragraphs into services
# "custom"     -- use [[services.boundaries]] for manual specification
boundary = "program"

# Communication pattern between services (when granularity = "services")
# "direct"  -- direct function calls (in-process)
# "grpc"    -- generate gRPC service definitions
# "rest"    -- generate REST API endpoints
# "events"  -- generate event-driven communication
ipc = "direct"

# Web framework (when ipc = "grpc" or "rest")
# "axum"    -- tokio + axum
# "actix"   -- actix-web
# "tonic"   -- tonic for gRPC
framework = "axum"

# Custom service boundaries
# Each entry defines a named service with its paragraph groups
# [[services.boundaries]]
# name = "account-service"
# paragraphs = ["process_account_*", "validate_account_*"]
# io_handles = ["master_file"]
#
# [[services.boundaries]]
# name = "report-service"
# paragraphs = ["generate_report_*", "print_*"]
# io_handles = ["report_file"]
```

### [async] -- Concurrency Model

Controls the async/concurrency transformation strategy.

```toml
[async]
# Whether to generate async code
# "sync"     -- keep synchronous (default)
# "tokio"    -- use tokio runtime
# "async-std" -- use async-std runtime
runtime = "sync"

# Which operations to make async (when runtime != "sync")
# "io-only"   -- only file/database/network operations
# "all"       -- all paragraph functions become async
# "selective" -- only paragraphs that touch I/O
scope = "io-only"

# Whether to add structured concurrency
# "none"       -- sequential execution
# "join"       -- parallel independent paragraphs with tokio::join!
# "select"     -- race patterns where applicable
concurrency = "none"
```

### [testing] -- Test Generation

Controls what test scaffolding is generated alongside transforms.

```toml
[testing]
# Generate tests for transformed code
# "none"       -- no test generation (default)
# "unit"       -- unit tests per function
# "integration" -- integration tests with test fixtures
# "both"       -- unit + integration
generate = "unit"

# Test framework
# "builtin"  -- #[test] + assert!
# "rstest"   -- rstest for parameterized tests
framework = "builtin"

# Generate property-based tests for numeric transforms
property_tests = false
```

### [dependencies] -- Target Dependency Preferences

Controls which crates the generated code should depend on.

```toml
[dependencies]
# Serialization
serde = true
serde_json = false

# Database
sqlx = false
rusqlite = false

# Async
tokio = false

# Error handling
thiserror = true
anyhow = false

# Logging
tracing = false
log = false
```

### Per-Program Overrides

Any section can be overridden per program using `[program."PROGRAM-ID"]`.

```toml
# Override for a specific program
[program.ACCT0100]
control_flow.strategy = "functions"
data_model.strategy = "domain"
io.handles.master_file = { backend = "database", table = "accounts" }

[program.RPTGEN]
services.granularity = "services"
io.handles.report_file = { backend = "stream", topic = "reports" }
```

## How Config Maps to Tier 4 Rules

Each config section drives specific deterministic transformations:

### T4-DISPATCH (consumes [control_flow])

| Config | Transformation |
|--------|---------------|
| strategy = "inline" | Replace dispatch match with inlined paragraph bodies at call sites |
| strategy = "functions" | Extract each paragraph arm to a standalone `fn`, replace dispatch with sequential calls |
| strategy = "state-machine" | Generate `enum ProgramState { ... }`, replace `usize _pc` with typed state |
| collapse_chains = true | Detect A->B->C with no branching, merge into single function |
| cycle_handling = "trampoline" | Wrap cyclic calls in `loop { match state { ... } }` pattern |
| error_propagation = "result" | Change paragraph signatures to return `Result<(), E>`, add `?` propagation |

### T4-DOMAIN (consumes [data_model])

| Config | Transformation |
|--------|---------------|
| strategy = "nested" | Use Tier 3 prefix groups to generate nested structs, rewrite field accesses |
| strategy = "split" | Use Tier 3 co-access groups to generate independent structs, thread as params |
| naming = "strip-prefix" | Regex-replace `ws_` prefix on all field names |
| use_accessors = true | Generate `impl` blocks with getter/setter methods, make fields private |
| derive_level = "full" | Add `#[derive(Debug, Clone, Serialize, Deserialize)]` to generated structs |
| structs.X = [...] | Override auto-grouping with explicit field-to-struct assignments |

### T4-ERROR (consumes [error_handling])

| Config | Transformation |
|--------|---------------|
| strategy = "result" | Use Tier 3 status-field detection, generate error enum, rewrite assignments to Ok/Err |
| binary_status = "bool" | 2-value status fields -> bool, rewrite checks to `if flag { ... }` |
| multi_status = "enum" | 3+ value fields -> enum, generate From<i32> impl, rewrite match arms |
| generate_program_error = true | Generate top-level `enum ProgramError { ... }` collecting all status variants |
| mappings.X = {...} | Use user-provided value->variant names instead of auto-generating |

### T4-IO (consumes [io])

| Config | Transformation |
|--------|---------------|
| backend = "file" | Use Tier 3 pattern classification -> BufReader/BufWriter/HashMap |
| backend = "database" | Generate table structs, replace open/read/write with SQL queries |
| backend = "stream" | Generate producer/consumer, replace sequential reads with message consumption |
| handles.X.backend | Per-handle override of the above |
| formats.default = "csv" | Generate CSV reader/writer instead of fixed-width |

### T4-SERVICE (consumes [services])

| Config | Transformation |
|--------|---------------|
| granularity = "modules" | Split single file into `mod` per paragraph group, shared state via params |
| granularity = "services" | Generate separate Cargo crate per service boundary, shared types crate |
| ipc = "grpc" | Generate `.proto` definitions from function signatures, tonic server/client |
| ipc = "rest" | Generate axum/actix handler functions, OpenAPI spec from types |
| boundaries[N] | Use explicit boundary definitions to group paragraphs into services |

## Interaction with Existing Config Files

```
.cobol2rust.toml          -- Phase 1: transpiler config (copy paths, extensions, pipeline)
.cobol2rust-target.toml   -- Phase 2+: target architecture config (THIS SPEC)
.rustify/hints.json       -- Auto-generated: COBOL semantic hints per file
.rustify/manifest.json    -- Auto-generated: transform tracking per tier
.rustify/plan.md          -- Auto-generated: Tier 3 assessment report
```

The target config is:
- Written once by the architect (human or AI)
- Read by Tier 4 rules during transformation
- Independent of the transpiler config
- Versioned in the project repo alongside the COBOL source

## Defaults

A minimal `.cobol2rust-target.toml` with all defaults:

```toml
# All sections optional -- sensible defaults applied
# This is equivalent to the most conservative transformation
```

Which gives:
- control_flow.strategy = "inline" (inline leaf paragraphs only)
- data_model.strategy = "flat" (keep flat struct)
- error_handling.strategy = "status-quo" (no change)
- io.backend = "file" (std::fs)
- services.granularity = "monolith" (single binary)
- async.runtime = "sync" (synchronous)
- testing.generate = "none" (no tests)

A "cloud-native" preset:

```toml
[control_flow]
strategy = "functions"
error_propagation = "result"

[data_model]
strategy = "nested"
naming = "strip-prefix"
derive_level = "full"

[error_handling]
strategy = "thiserror"

[io]
backend = "database"
database = "postgres"

[services]
granularity = "services"
ipc = "grpc"
framework = "tonic"

[async]
runtime = "tokio"
scope = "all"

[testing]
generate = "both"
```

A "lift-and-shift" preset (minimal change):

```toml
[control_flow]
strategy = "inline"
inline_threshold = 20

[data_model]
strategy = "flat"
naming = "strip-prefix"

[error_handling]
strategy = "result"
binary_status = "bool"

[io]
backend = "file"
```

## Validation Rules

The config loader validates:
1. All enum values are from the allowed set
2. Per-handle overrides reference handles detected by Tier 3 I/O analysis
3. Custom struct overrides reference fields that exist in hints.json
4. Service boundary paragraphs reference paragraphs detected by Tier 3 dispatch analysis
5. Dependency choices are consistent (e.g., framework = "tonic" requires tokio = true)

Validation errors are reported as warnings, not hard failures -- the rules skip
config they cannot apply rather than failing the entire run.

## Implementation Plan

### Phase 1: Config Crate (Session A output)
- `crates/cobol-rustify/src/target_config.rs` -- schema types + TOML deserialization
- `crates/cobol-rustify/src/target_config.rs` -- validation logic
- Unit tests for parsing, defaults, per-program overrides, validation

### Phase 2: Tier 4 Rules (Session B)
- T4-DISPATCH -- consumes [control_flow], Tier 3 dispatcher-analysis output
- T4-DOMAIN -- consumes [data_model], Tier 3 ws-decomposition output
- T4-ERROR -- consumes [error_handling], Tier 3 status-to-result output
- T4-IO -- consumes [io], Tier 3 io-modernize output

### Phase 3: Service Generation (Session C+)
- T4-SERVICE -- consumes [services], generates multi-crate workspace
- Async wrapper generation
- Test scaffolding generation


## GUI Integration

All configuration will be exposed through a simple-to-use GUI. This has design
implications for the config schema:

### GUI-Friendly Schema Properties

1. **Every enum is a dropdown**: All strategy/backend/framework choices are closed
   enums with human-readable descriptions. No free-form strings for core choices.

2. **Sensible defaults everywhere**: A user can click "Apply" on an empty config
   and get the conservative (lift-and-shift) result. Every setting has a safe default.

3. **Presets as starting points**: The GUI offers named presets (lift-and-shift,
   cloud-native, serverless) that populate the form. User adjusts from there.

4. **Per-handle visual mapping**: The I/O section shows detected file handles
   (from Tier 3 analysis) as a table. Each row has a dropdown for backend
   (file/database/stream/API) and backend-specific fields that appear/hide
   based on selection.

5. **Service boundary visualization**: When `services.boundary = "custom"`, the
   GUI shows the call graph (from Tier 3 dispatcher analysis) and lets the user
   drag-select paragraph groups into named services.

6. **Validation feedback**: The `validate_config()` warnings display inline next
   to the conflicting fields (e.g., red outline on `async.runtime = "tokio"` when
   `dependencies.tokio = false`).

7. **Per-program overrides as tabs**: Each COBOL program gets a tab. Unchecked
   fields inherit from the base config. Checked fields show the override value.

### Data Flow

```
GUI Form
  |
  v
.cobol2rust-target.toml   (written by GUI, readable by CLI)
  |
  v
Tier 4 Rules               (consume config + Tier 3 assessment)
  |
  v
Transformed Rust            (output workspace)
  |
  v
GUI Report View             (reads .rustify/manifest.json + plan.md)
```

### API Surface for GUI

The GUI needs these functions from the cobol-rustify crate:

| Function | Purpose |
|----------|---------|
| `load_target_config()` | Load existing config for editing |
| `validate_config()` | Real-time validation as user edits |
| `resolve_for_program()` | Preview effective config per program |
| `analyze_workspace()` | Run analysis (Tier 3) to populate handle/paragraph lists |
| `apply_workspace()` | Execute transforms with the config |
| `list_rules()` | Populate rule enable/disable checkboxes |

The config is the single source of truth. The GUI reads/writes the TOML file.
The CLI and GUI use the same `cobol-rustify` crate functions. No separate
backend needed -- the GUI can be a Tauri app wrapping the Rust crate directly.
