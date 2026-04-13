# cobol-sql

COBOL EXEC SQL runtime. Trait-based database abstraction with SQLCA.

## Key Types

- `CobolSqlRuntime` trait -- Abstract DB interface (PREPARE, EXECUTE, FETCH, CLOSE, COMMIT, ROLLBACK)
- `Sqlca` -- SQL Communication Area (mirrors IBM COBOL SQLCA layout)
- `HostVar` / `HostVarMut` -- Host variable binding for EXEC SQL
- `DuckDbRuntime` -- DuckDB implementation of CobolSqlRuntime

## Modules

- `runtime.rs` -- CobolSqlRuntime trait definition
- `sqlca.rs` -- SQLCA structure (SQLCODE, SQLERRM, SQLSTATE)
- `host_var.rs` -- Type-safe host variable binding
- `duckdb_runtime.rs` -- DuckDB backend implementation

## Dependencies

cobol-core, rust_decimal, thiserror, duckdb (optional)

## Features

- `duckdb` -- Enable DuckDB runtime (optional, heavy compilation)
- Default: trait only (users implement for their database)

## Rules

- SQLCA semantics must match IBM COBOL exactly
- Host variables bind COBOL fields to SQL parameters
- The trait allows swapping backends (DuckDB, JDBC bridge, etc.)
