# NexCore CLI User Guide

NexCore ships two command-line tools:

- **nexmig** -- COBOL migration: transpile, analyze, rustify, emit DSL
- **nexflow** -- Nexflow DSL toolchain: parse, validate, build, generate

Both are built from the same workspace (`cargo build --workspace`).

---

## Quick Start

```bash
# Build both CLIs
cargo build -p nexmig-cli -p nexflow-cli --release

# Transpile a single COBOL file to Rust
nexmig transpile hello.cbl -o hello.rs

# Transpile a directory to a Cargo workspace
nexmig transpile cobol/ -o rust-out/ --workspace -j 8

# Parse a Nexflow DSL file
nexflow parse account-api.api

# Generate Java/Flink code from DSL
nexflow generate account-api.api --target java -o generated/
```

---

## nexmig -- COBOL Migration CLI

### Global Options

All nexmig subcommands accept these flags:

| Flag | Description |
|------|-------------|
| `-C, --copybook-path <DIR>` | Copybook search directory (repeatable) |
| `-f, --source-format <FMT>` | Override format detection: `auto`, `fixed`, `free` |
| `-v, --verbose` | Increase verbosity (repeat: `-v`, `-vv`, `-vvv`) |
| `-q, --quiet` | Suppress non-error output |
| `--color <MODE>` | Color output: `auto`, `always`, `never` |
| `--config <PATH>` | Configuration file (TOML) |

### Structured Logging

Library diagnostics use the `tracing` framework. Set `RUST_LOG` to see them:

```bash
RUST_LOG=warn nexmig transpile large_file.cbl -o out.rs
RUST_LOG=debug nexmig audit /path/to/cobol/
```

---

### Subcommands

#### transpile

Transpile COBOL source to Rust (or Java).

```bash
# Single file to stdout
nexmig transpile hello.cbl

# Single file to output
nexmig transpile hello.cbl -o hello.rs

# Directory to Cargo workspace (parallel)
nexmig transpile cobol/ -o rust-out/ --workspace -j 8

# Java target
nexmig transpile hello.cbl -o Hello.java --target java

# With copybook paths (client estates)
nexmig transpile -f fixed -C copy/code -C copy/layout program.cbl -o out.rs

# Incremental (skip unchanged files)
nexmig transpile cobol/ -o rust-out/ --workspace --incremental

# Continue on error (skip failures, report at end)
nexmig transpile cobol/ -o rust-out/ --workspace --continue-on-error
```

Key options:
- `--workspace` -- required for directory input, generates Cargo workspace
- `--target rust|java` -- output language (default: rust)
- `-j, --jobs N` -- parallel workers (default: CPU count)
- `--incremental` -- skip unchanged files (compares mtimes)
- `--continue-on-error` -- skip failures instead of stopping
- `--main` / `--lib` -- force main program or library output
- `-L NAME=DIR` -- COPY library mapping

#### check

Validate COBOL source without transpiling.

```bash
# Basic validation
nexmig check program.cbl

# Coverage analysis (which statements can be transpiled)
nexmig check --coverage program.cbl

# JSON output for tooling
nexmig check --format json --coverage program.cbl

# Strict mode (warnings are errors)
nexmig check --strict program.cbl
```

#### preprocess

Expand COPY statements and clean source format.

```bash
# Expand to stdout
nexmig preprocess -C copybooks/ program.cbl

# Save expanded source
nexmig preprocess -C copybooks/ program.cbl -o expanded.cbl

# Show COPY origins as comments
nexmig preprocess --show-origins -C copybooks/ program.cbl

# Strip columns only (no COPY expansion)
nexmig preprocess --no-copy program.cbl
```

#### parse

Parse COBOL source to AST.

```bash
# Tree view (default)
nexmig parse program.cbl

# JSON AST output
nexmig parse program.cbl --format json -o ast.json

# Data division only
nexmig parse program.cbl --section data --format json
```

#### rustify

Transform transpiled Rust into idiomatic Rust (Phase 2) and emit DSL.

```bash
# Analysis report (no output written)
nexmig rustify rust-out/ --report

# Apply tier 1 (cosmetic cleanup)
nexmig rustify rust-out/ -o phase2/ --tier 1

# Apply tier 2 (type promotion)
nexmig rustify rust-out/ -o phase2/ --tier 2

# Emit Nexflow DSL files alongside rustification
nexmig rustify rust-out/ -o phase2/ --emit-dsl

# Direct DSL emission from COBOL AST (bypass syn re-parsing)
nexmig rustify rust-out/ -o phase2/ --emit-dsl --emit-mode direct

# List available rules
nexmig rustify --rules

# Apply specific rules only
nexmig rustify rust-out/ -o phase2/ --only const-extract,zero-literal

# JSON report
nexmig rustify rust-out/ --report --format json
```

Key options:
- `--tier 1|2|3` -- rustification tier (1=cosmetic, 2=types, 3=structural)
- `--emit-dsl` -- generate .schema, .xform, .rules, .proc files
- `--emit-mode legacy|direct|compare` -- DSL emission strategy
- `--only / --skip` -- include/exclude specific rule IDs
- `--force` -- overwrite even if user patches detected
- `--preserve-patches` -- keep manual modifications

#### audit

Preflight audit: encoding, dependencies, validation, coverage, readiness.

```bash
# Full audit
nexmig audit /path/to/cobol/ -C copybooks/ -j 8

# Text output
nexmig audit /path/to/cobol/ --format text

# Skip coverage for faster results
nexmig audit /path/to/cobol/ --skip-coverage

# Fixed format for client estates
nexmig audit -f fixed -C copy/code -C copy/layout /path/to/cobol/
```

The audit runs 6 phases:
1. File inventory and encoding detection
2. Copybook dependency analysis
3. COBOL validation
4. Transpilation coverage
5. Readiness scoring
6. BMS screen analysis (if .bms files found)

#### diagnose

Diagnose COBOL files with specialized sub-tools.

```bash
# Token error breakdown
nexmig diagnose tokens program.cbl

# Verb inventory (supported vs unsupported)
nexmig diagnose verbs program.cbl

# COPY dependencies
nexmig diagnose copybooks -C copybooks/ program.cbl

# Encoding scan (find non-ASCII issues)
nexmig diagnose encoding /path/to/cobol/

# Source format detection
nexmig diagnose format program.cbl

# Dialect detection (IBM, Microfocus, ANSI)
nexmig diagnose dialect program.cbl

# Skeleton analysis (why transpilation produced minimal output)
nexmig diagnose skeleton rust-out/

# Hotspot ranking from audit report
nexmig diagnose hotspots audit_report.json

# Compare two audit reports
nexmig diagnose compare old_audit.json new_audit.json

# Preview character sanitization
nexmig diagnose sanitize --dry-run /path/to/cobol/
```

#### build-graph

Build a code intelligence graph from COBOL sources.

```bash
nexmig build-graph \
    --input /path/to/cobol/ \
    --output analysis.nxg \
    --license YOUR_LICENSE_KEY \
    -C copybooks/ \
    --enrich \
    --cost-config cost_config.json
```

The graph captures call chains, data flow, and program dependencies for querying.

#### query

Query a code intelligence graph (interactive REPL or batch).

```bash
# Interactive REPL
nexmig query --graph analysis.nxg --license YOUR_LICENSE_KEY

# Inline query
nexmig query --graph analysis.nxg --license KEY \
    -e "SHOW PROGRAMS WHERE calls > 10"

# Query file
nexmig query --graph analysis.nxg --license KEY \
    --file investigation.nxq
```

#### bms

BMS mapset tools for CICS screen migration.

```bash
# Convert BMS to .screen + .schema DSL
nexmig bms to-screen /path/to/bms/ -o dsl-out/

# Inspect BMS structure
nexmig bms inspect SCREEN.bms

# JSON output
nexmig bms inspect SCREEN.bms --format json

# Cross-reference: which programs use which maps
nexmig bms references /path/to/cobol/
```

#### decode

Decode binary COBOL dataset records.

```bash
# Decode first 10 records as JSON
nexmig decode data.dat --copybook layout.cpy

# CSV output, records 100-200
nexmig decode data.dat --copybook layout.cpy --format csv --start 100 --count 100

# EBCDIC encoding
nexmig decode data.dat --copybook layout.cpy --encoding ebcdic --pretty
```

#### discover

Auto-match binary data files to copybooks.

```bash
nexmig discover /path/to/data/ \
    --copybooks /path/to/copybooks/ \
    --programs /path/to/cobol/ \
    --format table
```

#### data-analyze

Analyze copybook layout, REDEFINES groups, and discriminators.

```bash
# Table output
nexmig data-analyze layout.cpy

# With program for discriminator detection
nexmig data-analyze layout.cpy --program processor.cbl --format json
```

#### Other Subcommands

| Command | Description |
|---------|-------------|
| `init` | Scaffold a Cargo workspace from COBOL sources |
| `compile` | Transpile and `cargo build` in one step |
| `diff` | Compare transpilation outputs of two COBOL files |
| `project` | Full pipeline for a single COBOL project (phases 0-5) |
| `corpus` | Pipeline across a multi-repo COBOL corpus |
| `scan` | Enterprise scan with DuckDB persistence |
| `status` | Show scan progress from DuckDB |
| `report` | Generate reports from completed scan |

---

## nexflow -- Nexflow DSL Toolchain

### Subcommands

#### parse

Parse a single DSL file and show its AST.

```bash
# Summary (default)
nexflow parse account-api.api

# JSON output
nexflow parse account-api.api --format json

# Tree view
nexflow parse account.schema --format tree
```

Supported file types: `.api`, `.service`, `.schema`, `.xform`, `.rules`, `.proc`, `.nxq`

#### validate

Validate DSL files for cross-grammar reference consistency.

```bash
# Validate specific files
nexflow validate account-api.api account.schema account-service.service

# Validate a directory (recursive)
nexflow validate src/

# JSON output
nexflow validate src/ --format json
```

Checks:
- Schema references in .api resolve to .schema definitions
- Service `implements` references match .api names
- Handler names match endpoint names
- Field types are valid (base types or schema references)

#### build

Full pipeline: parse -> validate -> generate.

```bash
# Build Java/Flink (default)
nexflow build

# Build Rust/Axum
nexflow build --target rust

# Build both targets
nexflow build --target all

# Dry run (validate only)
nexflow build --dry-run

# Custom output directory
nexflow build -o build-output/

# Verify generated code compiles
nexflow build --verify
```

Reads project config from `nexflow.toml` in the current directory.

#### generate

Generate code from a single entry point (without project config).

```bash
# Generate Java from .api
nexflow generate account-api.api --target java -o generated/

# Generate Rust/Axum from .api + .service
nexflow generate account-api.api --target rust -o generated/ \
    --include account-service.service

# Include additional files
nexflow generate account-api.api --include schema/ --include rules/
```

#### init

Initialize a new Nexflow project.

```bash
nexflow init --name my-project
```

Creates `nexflow.toml` and `.gitignore`.

#### clean

Remove generated files.

```bash
nexflow clean        # Remove generated/ directory
nexflow clean --all  # Also remove .nexflow-cache/
```

#### info

Show project information from `nexflow.toml`.

```bash
nexflow info
```

---

## Pipeline: COBOL -> DSL -> Java/Rust

The full modernization pipeline uses both CLIs:

```bash
# Step 1: Transpile COBOL to Phase 1 Rust
nexmig transpile -f fixed -C copybooks/ cobol/ -o rust-phase1/ --workspace -j 8

# Step 2: Rustify + emit DSL
nexmig rustify rust-phase1/ -o rust-phase2/ --tier 1 --emit-dsl

# Step 3: Validate DSL
nexflow validate rust-phase2/dsl/

# Step 4: Generate target code from DSL
nexflow generate rust-phase2/dsl/ --target java -o java-output/
```

---

## Configuration

### TOML Config File

Pass `--config config.toml` to nexmig for persistent settings:

```toml
[workspace]
copy_paths = ["copybooks/code", "copybooks/layout"]
source_format = "fixed"
jobs = 8
continue_on_error = true
```

### nexflow.toml

The nexflow CLI reads project config from `nexflow.toml`:

```toml
[project]
name = "account-service"
version = "0.1.0"
src_dir = "src"
output_dir = "generated"
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Tracing filter (e.g., `warn`, `debug`, `cobol_transpiler=debug`) |
| `RUST_BACKTRACE` | Enable backtraces on errors (`1` or `full`) |
| `NEXMIG` | Override nexmig binary path (used by scripts) |
| `NEXFLOW` | Override nexflow binary path (used by scripts) |

---

## Diagnostic Scripts

The `scripts/` directory contains diagnostic tools:

| Script | Purpose |
|--------|---------|
| `test-data-chunking.sh` | Verify DATA DIVISION parsing captures all 01-levels |
| `test-dsl-emission.sh` | Transpile + emit DSL + report file inventory |
| `test-pipeline-e2e.sh` | Full COBOL -> DSL -> validate pipeline |
| `test-batch-transpile.sh` | Batch transpile all test COBOL files |
| `test-bms-screens.sh` | BMS -> .screen/.schema generation report |
| `debug-monster.sh` | Diagnose why a large COBOL file produces tiny output |

Usage:
```bash
# Override binary path
CR=target/debug/nexmig ./scripts/test-data-chunking.sh /path/to/large.cbl -f fixed -C copybooks/

# Use default path
./scripts/test-batch-transpile.sh cobol/
```
