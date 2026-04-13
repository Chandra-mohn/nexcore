# cobol-cli

The `cobol2rust` command-line tool. 10+ subcommands for transpilation, analysis, and scanning.

## Commands

| Command | Purpose |
|---------|---------|
| transpile | Transpile COBOL to Rust (single file or batch) |
| check | Validate COBOL syntax without transpiling |
| preprocess | Expand COPY statements, clean format |
| parse | Parse COBOL to AST (tree or JSON output) |
| init | Scaffold Cargo workspace from COBOL sources |
| compile | Transpile + cargo build in one step |
| diff | Compare transpilation outputs (unified/JSON/AST) |
| project | Full pipeline for single COBOL project |
| corpus | Pipeline across multi-repo COBOL corpus |
| scan | Enterprise scanner with DuckDB persistence |
| status | Show scan progress/history |
| report | Generate reports from completed scan |
| rustify | Phase 2 rustification |

## Source Structure

```
src/
  main.rs              -- clap CLI definition
  transpile_cmd.rs     -- transpile subcommand
  rustify_cmd.rs       -- rustify subcommand
  compile_cmd.rs       -- compile subcommand
  check.rs, preprocess.rs, parse_cmd.rs
  init_cmd.rs, diff_cmd.rs
  project_cmd.rs, corpus_cmd.rs
  pipeline/
    mod.rs, config.rs, phase0.rs  -- Shared pipeline machinery
  scan/
    mod.rs, discover.rs           -- File discovery
    phase1.rs, phase2.rs, phase3.rs -- Multi-phase scanning
    db.rs                         -- DuckDB schema and queries
    worker.rs                     -- Parallel worker pool
    ndjson.rs                     -- NDJSON output
  analyze.rs, workspace.rs
```

## Global Flags

- `-C/--copybook-path` -- Copybook search path (repeatable)
- `-f/--source-format` -- auto|fixed|free
- `-v/--verbose` -- Verbosity (-v, -vv, -vvv)
- `-q/--quiet` -- Suppress output
- `--color` -- auto|always|never
- `--config` -- TOML config file path

## Workspace Mode

- .cobol2rust.toml per-repo: copy_paths, extensions, exclude patterns
- Auto-discovers copybook directories
- NDJSON reports: files.ndjson, transpile_results.ndjson, scan_meta.json

## Dependencies

cobol-transpiler, cobol-rustify, clap, miette, rayon, duckdb (optional), glob, toml

## Features

- `duckdb` (default) -- Enable scan command with DuckDB persistence
