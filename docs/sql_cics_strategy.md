# SQL and CICS Strategy

**Date**: 2026-04-07
**Status**: Planning

---

## EXEC SQL -- Current State: PRODUCTION-READY

### What exists today

The full pipeline is implemented for Rust codegen:

```
COBOL EXEC SQL block
  -> [preprocess.rs] Extract block, replace with CONTINUE
  -> [sql_extract.rs] Classify statement type, extract host variables
  -> [ast.rs] ExecSqlStatement { sql_type, raw_sql, input_vars, output_vars, cursor_name }
  -> [proc_gen.rs] Generate CobolSqlRuntime trait calls with HostVar binding
  -> [cobol-sql] DuckDB runtime implementation
```

### SQL statement types supported (15)

| Type | Codegen | Runtime |
|------|---------|---------|
| SELECT INTO | Full (parameterized query + output binding) | DuckDB |
| INSERT | Full (parameterized) | DuckDB |
| UPDATE | Full (parameterized) | DuckDB |
| DELETE | Full (parameterized) | DuckDB |
| DECLARE CURSOR | Full | DuckDB |
| OPEN CURSOR | Full | DuckDB |
| FETCH CURSOR | Full (output binding) | DuckDB |
| CLOSE CURSOR | Full | DuckDB |
| COMMIT | Full | DuckDB |
| ROLLBACK | Full | DuckDB |
| PREPARE | Full | DuckDB |
| EXECUTE | Full | DuckDB |
| EXECUTE IMMEDIATE | Full | DuckDB |
| SAVEPOINT | Full | DuckDB |
| INCLUDE SQLCA | Auto-injected into WorkingStorage | N/A |

### Architecture: trait-based, database-portable

```
CobolSqlRuntime (trait)
  |
  +-- DuckDbRuntime      (exists -- dev/testing)
  +-- OracleRuntime      (future)
  +-- PostgresRuntime    (future)
  +-- MsSqlRuntime       (future)
```

Generated Rust code calls `sql.exec_query(...)` etc. regardless of backend.
The runtime implementation handles dialect differences (placeholder syntax,
date functions, etc.).

### Java codegen gap

Java codegen is a stub -- passes raw SQL string to `CobolRuntime.execute()`.
Needs parameterized calls matching the Rust pattern. Not urgent for client
delivery since Java target is Phase 2.

### SQL-to-SQL transpiler (future product opportunity)

The `ExecSqlStatement` AST is database-agnostic. A standalone SQL-to-SQL
transpiler could convert DB2 SQL to Oracle/PostgreSQL/MS SQL by:
- Rewriting placeholder syntax (`:host-var` -> `?` -> `$1` -> `@param`)
- Mapping DB2-specific functions to target equivalents
- Converting data types and identifier casing

This could be a standalone product (NexSQL) independent of COBOL transpilation.
Deferred -- not in current roadmap.

---

## EXEC CICS -- Current State: EXTRACTION ONLY

### What exists today

- Preprocessing extracts `EXEC CICS ... END-EXEC` blocks (same mechanism as SQL)
- Blocks are identified with `exec_type: "CICS"` marker
- Then DISCARDED during codegen (filtered out in `build_exec_sql_ast()`)
- No AST, no codegen, no runtime
- A complete CICS grammar exists (`grammar/mapa/cics/CICSzParser.g4`, 26K lines)
  but is not integrated

### Key insight: CICS is infrastructure, not code

SQL is a data access language -- you transpile it to another SQL dialect.
CICS is a transaction processing runtime -- you REPLACE it with modern
equivalents. There is no CICS on the target platform.

### CICS command categories and modern equivalents

| CICS Command | What it does | Modern equivalent |
|-------------|-------------|-------------------|
| SEND MAP / RECEIVE MAP | 3270 terminal I/O | REST API / Web UI |
| LINK | Call program synchronously | Function call / service call |
| XCTL | Transfer control (no return) | Function call (replace stack) |
| RETURN TRANSID | End, schedule next txn | HTTP response + next route |
| READ / WRITE / REWRITE / DELETE | VSAM file via CICS | Database query |
| STARTBR / READNEXT / READPREV / ENDBR | Browse VSAM | SELECT with cursor |
| WRITEQ TS / READQ TS / DELETEQ TS | Temp storage queue | Redis / session store |
| WRITEQ TD / READQ TD | Transient data | Message queue (Kafka/RabbitMQ) |
| SYNCPOINT | Commit | DB transaction commit |
| ABEND | Crash transaction | panic / error return |
| HANDLE CONDITION / HANDLE ABEND | Error handling | try/catch / Result |
| GETMAIN / FREEMAIN | Memory allocation | No-op in Rust (managed) |
| ASKTIME / FORMATTIME | Clock | chrono / std::time |
| ENQ / DEQ | Resource locking | Mutex / distributed lock |
| ASSIGN | Get system info | Environment / config lookup |
| START / CANCEL | Schedule/cancel txns | Job scheduler / cron |

### Two types of CICS programs

1. **Screen programs** (SEND MAP / RECEIVE MAP) -- conversational or
   pseudo-conversational 3270 I/O. These need UI replacement (web frontend).
   Heaviest lift because the interaction model changes fundamentally.

2. **Batch/service programs** (LINK, file I/O, queues) -- backend logic
   called by other programs or triggered by events. These map more
   naturally to modern services (REST endpoints, message handlers).

The approach is very different for each category.

### Phased implementation plan

**Phase 1 -- Capture and represent (transpiler work)**

Goal: Parse CICS blocks into AST so transpiled code compiles.

- Add `ExecCicsStatement` to AST with command type + named options
- Add `CicsCommand` enum (start with top 10-15 commands from client estate)
- Route extracted CICS blocks through `build_exec_cics_ast()` (like SQL)
- Rust codegen: generate `cics.command(args)` calls against a trait
- Define `CobolCicsRuntime` trait (abstract interface)
- Generated code compiles but CICS calls are stubs

Implementation:
```rust
// AST
pub struct ExecCicsStatement {
    pub command: CicsCommand,
    pub options: Vec<CicsOption>,
    pub raw_text: String,
}

pub enum CicsCommand {
    SendMap, ReceiveMap, SendText,
    Link, Xctl, Return,
    Read, Write, Rewrite, Delete,
    StartBrowse, ReadNext, ReadPrev, EndBrowse,
    WriteqTs, ReadqTs, DeleteqTs,
    WriteqTd, ReadqTd,
    Syncpoint, Abend,
    HandleCondition, HandleAbend,
    Getmain, Freemain,
    Asktime, Formattime,
    Enqueue, Dequeue,
    Assign,
    Other(String),
}

pub struct CicsOption {
    pub name: String,       // e.g., "MAP", "MAPSET", "FROM", "INTO"
    pub value: Option<String>,  // e.g., "MAPNAME", "WS-FIELD", "LENGTH(WS-LEN)"
}
```

Codegen pattern:
```rust
// EXEC CICS SEND MAP('ACCTMAP') MAPSET('ACCTSET') FROM(WS-MAP-DATA) END-EXEC
cics.send_map("ACCTMAP", "ACCTSET", &ws.ws_map_data);

// EXEC CICS LINK PROGRAM('SUBPROG') COMMAREA(WS-COMM) LENGTH(WS-LEN) END-EXEC
cics.link("SUBPROG", &mut ws.ws_comm, ws.ws_len.to_usize());

// EXEC CICS READ FILE('ACCTFL') INTO(WS-RECORD) RIDFLD(WS-KEY) END-EXEC
cics.read_file("ACCTFL", &mut ws.ws_record, &ws.ws_key);
```

**Phase 2 -- Parse CICS arguments (transpiler enhancement)**

Goal: Parse CICS option keywords and map fields to working-storage.

- Use keyword extraction from raw CICS text (not the full 26K grammar)
- Parse CICS options: `MAP('name')`, `FROM(WS-FIELD)`, `LENGTH(WS-LEN)`
- Resolve field references to working-storage names
- Generate typed calls with proper host variable binding

**Phase 3 -- Modern mapping (NexMod / DSL territory)**

Goal: Map CICS commands to modern architectural patterns.

- Screen programs -> REST API endpoints (SEND/RECEIVE -> request/response)
- CICS file I/O -> database calls (READ/WRITE -> SQL)
- TS queues -> cache/session layer (Redis, in-memory)
- TD queues -> message queue (Kafka, RabbitMQ)
- LINK/XCTL -> function calls or microservice invocations
- HANDLE CONDITION -> Rust error handling (Result/match)
- This is where DSL emitters generate `.api` and `.service` files

### Discovery step (before any implementation)

Need to know what CICS commands the client estate actually uses:

```bash
grep -rh 'EXEC CICS' ./cobol/cobolfile/ | grep -oE 'EXEC CICS [A-Z]+' | sort | uniq -c | sort -rn | head -20
```

This determines:
- Whether the estate is screen-heavy (SEND/RECEIVE MAP) or service-heavy (LINK, file I/O)
- Which 10-15 commands to implement first
- Whether Phase 3 needs a web UI framework or just backend services

---

## Priority Summary

| Item | Priority | Status |
|------|----------|--------|
| SQL Rust codegen | Done | 15 statement types, DuckDB runtime |
| SQL Java codegen | P1 (client needs Java) | Stub -- needs parameterized calls |
| SQL-to-SQL transpiler | P3 (future product) | Not started |
| CICS Phase 1 (capture) | P1 (after client discovery) | Not started |
| CICS Phase 2 (parse args) | P2 | Not started |
| CICS Phase 3 (modern mapping) | P2 (NexMod) | Not started |

---

## Files Changed

None. This is a strategy document.
