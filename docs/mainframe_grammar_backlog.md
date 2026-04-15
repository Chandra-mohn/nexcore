# Mainframe Grammar Implementation Backlog

Source grammars from mapa project (MIT licensed, Craig Schneiderwent).
These parse mainframe-specific languages embedded in COBOL programs
and feed into the Nexflow DSL emitters for .api, .service, .screen, .proc.

---

## Grammar Inventory

| Grammar | Commands | Lines | Target DSLs | Priority |
|---------|----------|-------|-------------|----------|
| CICS | 572 (294 API + 278 SPI) | 34,794 | .api, .service, .screen, .proc | P1 |
| DB2z SQL | 92 statement types | 21,901 | .schema, .proc | P2 |
| JCL | 15+ statement types + 5 variants | 14,857 | .proc (batch phases, job steps) | P3 |
| DLI | 25 commands | 1,717 | .schema (IMS segments), .proc | P4 |

---

## CICS Backlog (572 commands -> .api, .service, .screen, .proc)

CICS is the primary online transaction processing runtime. Every CICS
command reveals architectural intent about the modernized application.

### Phase C1: Grammar Integration + AST (Foundation)

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| C1.1 | Copy CICSzParser.g4 + CICSzLexer.g4 to grammar/mainframe/cics/ | File copy + LICENSE | Grammar available in nexcore |
| C1.2 | Create cobol-cics crate skeleton | New crate in workspace | Cargo.toml, lib.rs, mod structure |
| C1.3 | Generate Rust parser via antlr-rust | Build script | CICS tokens parsed from EXEC CICS blocks |
| C1.4 | Define CicsCommand AST enum | 572 variants, typed parameters | Structured AST from parse tree |
| C1.5 | Build EXEC CICS extractor | Walk cobol-transpiler AST, extract EXECCICSLINE raw text | Bridge cobol-transpiler -> cobol-cics |
| C1.6 | Parse extracted CICS text into CicsCommand AST | Wire parser to extractor | Every EXEC CICS block -> typed AST node |

### Phase C2: Screen Emission (.screen from SEND MAP / RECEIVE MAP)

BMS maps define CICS screen layouts. SEND MAP / RECEIVE MAP commands
reference these maps. Combined, they define the UI contract.

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| C2.1 | Extract SEND MAP commands | Map name, MAPSET, FROM field, CURSOR | Screen output operations |
| C2.2 | Extract RECEIVE MAP commands | Map name, INTO field | Screen input operations |
| C2.3 | Parse BMS map definitions | Use grammar/cobol/BmsDSL.g4 | Field positions, lengths, attributes |
| C2.4 | Build BMS -> ScreenDSL mapper | BMS fields -> .screen field declarations | Screen layout with types |
| C2.5 | Correlate MAP commands with BMS definitions | SEND MAP "X" -> BMS map X fields | Complete screen model |
| C2.6 | Emit .screen files | ScreenDSL grammar-valid output | One .screen per BMS mapset |
| C2.7 | Handle SEND TEXT / SEND CONTROL | Non-map screen output | Text-mode screens |

### Phase C3: API Emission (.api from LINK / XCTL / WEB)

Programs that are LINKed or XCTLed are services. Their COMMAREA
(LINKAGE SECTION) defines the request/response contract.

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| C3.1 | Extract LINK commands | Target program, COMMAREA, CHANNEL, LENGTH | Service call inventory |
| C3.2 | Extract XCTL commands | Target program, COMMAREA | Transfer-of-control inventory |
| C3.3 | Extract RETURN commands | TRANSID, COMMAREA | Transaction chaining |
| C3.4 | Map COMMAREA to request/response schema | LINKAGE SECTION fields -> .schema reference | Typed API contract |
| C3.5 | Extract WEB SEND / WEB RECEIVE | HTTP method, path, headers, body | REST endpoint definitions |
| C3.6 | Extract WEB OPEN / WEB CLOSE | Host, port, scheme | External service connections |
| C3.7 | Extract WEB EXTRACT | HTTP headers, query params | Request parsing |
| C3.8 | Build .api emitter | ApiDSL grammar-valid output | One .api per service boundary |
| C3.9 | Detect transaction boundaries | RETURN TRANSID chains | Conversation-based API flows |
| C3.10 | Extract SIGNON / SIGNOFF / VERIFY | Authentication patterns | .api auth block |
| C3.11 | Extract QUERY SECURITY | Authorization checks | .api permissions |

### Phase C4: Service Emission (.service from orchestration patterns)

The calling program's CICS command sequence reveals service orchestration.

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| C4.1 | Build CICS program model | All commands per program -> CicsProgramProfile | Full program characterization |
| C4.2 | Detect service pattern | LINK/XCTL graph -> service dependency map | Service topology |
| C4.3 | Extract data access patterns | READ/WRITE/REWRITE/DELETE + STARTBR/READNEXT/ENDBR | Data service operations |
| C4.4 | Extract queue operations | WRITEQ/READQ/DELETEQ TS and TD | State management + messaging |
| C4.5 | Extract async patterns | START + RETRIEVE, CANCEL | Async transaction invocation |
| C4.6 | Detect transaction boundaries | SYNCPOINT, SYNCPOINT ROLLBACK | Transaction scope |
| C4.7 | Extract error handling | HANDLE ABEND, HANDLE CONDITION, IGNORE CONDITION | Error recovery patterns |
| C4.8 | Extract concurrency | ENQ, DEQ | Locking requirements |
| C4.9 | Build .service emitter | ServiceDSL grammar-valid output | One .service per online program |
| C4.10 | Map document operations | DOCUMENT CREATE/INSERT/SET/RETRIEVE | Response assembly |

### Phase C5: Process Enhancement (.proc enrichment)

CICS commands that enhance the existing .proc emitter output.

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| C5.1 | Map VSAM file ops to connectors | READ/WRITE/STARTBR -> receive/emit with connector config | Richer .proc I/O |
| C5.2 | Map queue ops to event streams | WRITEQ TD -> emit to queue, READQ TD -> receive from queue | Event-driven .proc |
| C5.3 | Map START/RETRIEVE to async | START TRANSID -> emit event, RETRIEVE -> receive event | Async process steps |
| C5.4 | Extract ASSIGN fields | System/transaction context variables | Process metadata |

### Phase C6: Advanced CICS (SPI + System)

Lower priority -- system programming interface for operations, not app logic.

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| C6.1 | INQUIRE commands (167 types) | System resource interrogation | Operational context |
| C6.2 | SET commands (100+ types) | System resource modification | Admin operations |
| C6.3 | CREATE / DISCARD commands | Resource lifecycle | Deployment automation |
| C6.4 | CSD commands | CICS system definition | Configuration management |
| C6.5 | PERFORM / COLLECT | Statistics and monitoring | Observability |
| C6.6 | SPOOL commands | Print/report spooling | Report generation |

---

## DB2z SQL Backlog (92 statement types -> .schema, .proc)

DB2z SQL extends standard SQL with mainframe-specific features.
The current cobol-transpiler extracts EXEC SQL as raw text with
basic heuristics. A proper DB2z parser enables richer extraction.

### Phase D1: Grammar Integration + AST

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| D1.1 | Copy DB2zSQLParser.g4 + DB2zSQLLexer.g4 to grammar/mainframe/db2z/ | File copy + LICENSE | Grammar available |
| D1.2 | Create cobol-db2 crate skeleton | New crate | Cargo.toml, lib.rs |
| D1.3 | Generate Rust parser via antlr-rust | Build script | DB2z SQL parsed |
| D1.4 | Define Db2Statement AST enum | 92 statement types | Typed AST |
| D1.5 | Wire to existing ExecSqlStatement | Replace raw_sql text parsing | Structured SQL analysis |

### Phase D2: Schema Enhancement (.schema from DDL)

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| D2.1 | Extract CREATE TABLE | Columns, types, constraints, keys | .schema from DDL |
| D2.2 | Extract ALTER TABLE | Column additions, modifications | Schema evolution |
| D2.3 | Extract CREATE INDEX | Index definitions | .schema identity/key hints |
| D2.4 | Map DB2 types to Nexflow types | DECIMAL, VARCHAR, DATE, TIMESTAMP, BLOB | Accurate .schema types |
| D2.5 | Extract DECLARE TABLE | Embedded table declarations | Compile-time schema info |
| D2.6 | Extract FOREIGN KEY / REFERENCES | Table relationships | Cross-schema references |

### Phase D3: Query Analysis (.proc + .xform enrichment)

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| D3.1 | Extract SELECT with JOIN/WHERE | Query structure, table relationships | .proc data flow |
| D3.2 | Extract INSERT/UPDATE/DELETE | DML patterns | .proc write operations |
| D3.3 | Extract CURSOR operations | DECLARE/OPEN/FETCH/CLOSE lifecycle | .proc batch iteration |
| D3.4 | Extract COMMIT/ROLLBACK | Transaction boundaries | .proc transaction scope |
| D3.5 | Extract stored procedure CALL | External procedure invocations | .api / .service calls |
| D3.6 | Extract MERGE statement | Upsert patterns | .xform merge logic |

### Phase D4: Advanced DB2z

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| D4.1 | Extract EXPLAIN | Query plans | Performance analysis |
| D4.2 | Extract GRANT/REVOKE | Access control | .api permissions |
| D4.3 | Extract CREATE FUNCTION/PROCEDURE | DB-side logic | .xform candidates |
| D4.4 | Extract CREATE TRIGGER | Event-driven DB logic | .proc event sources |
| D4.5 | Extract CREATE VIEW | Derived data definitions | .schema views |
| D4.6 | Extract LOCK TABLE | Concurrency patterns | .service locking |

---

## JCL Backlog (15+ statement types -> .proc batch phases)

JCL defines batch job structure: job steps, file allocations, program
execution, conditional processing. This maps to .proc batch orchestration.

### Phase J1: Grammar Integration + AST

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| J1.1 | Copy JCL grammars to grammar/mainframe/jcl/ (all 10 .g4 files) | File copy + LICENSE | Grammar available |
| J1.2 | Create cobol-jcl crate skeleton | New crate | Cargo.toml, lib.rs |
| J1.3 | Generate Rust parser for JCLParser.g4 | Build script | Core JCL parsed |
| J1.4 | Define JclJob AST | JOB, EXEC, DD structured types | Typed AST |
| J1.5 | Parse JCL source files (.jcl) | Read from file system | Job definitions loaded |

### Phase J2: Job Step -> Process Mapping

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| J2.1 | Extract JOB card | Job name, class, priority, notify | .proc process name + metadata |
| J2.2 | Extract EXEC PGM steps | Program name, PARM, COND | .proc transform steps (one per EXEC) |
| J2.3 | Extract EXEC PROC steps | Procedure invocations | .proc sub-process references |
| J2.4 | Extract DD statements | DSN, DISP, DCB (RECFM/LRECL/BLKSIZE) | .proc receive/emit with file connector |
| J2.5 | Map SYSOUT/SYSIN DDs | Standard I/O streams | .proc system connectors |
| J2.6 | Map SORTIN/SORTOUT DDs | SORT utility I/O | .proc sort/order operations |
| J2.7 | Extract COND parameters | Step conditional execution | .proc if/route logic |
| J2.8 | Extract IF/THEN/ELSE | JCL conditional processing | .proc conditional steps |

### Phase J3: File Allocation -> Connector Mapping

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| J3.1 | Map DSN patterns | Dataset naming conventions -> logical names | .proc connector config |
| J3.2 | Map DISP parameters | NEW/OLD/SHR/MOD -> create/read/append | .proc I/O direction |
| J3.3 | Map DCB parameters | RECFM/LRECL/BLKSIZE -> record format | .schema field layout hints |
| J3.4 | Map UNIT/VOL | Storage allocation | Deployment config (not DSL) |
| J3.5 | Map GDG (Generation Data Group) | Versioned datasets | .proc batch versioning |

### Phase J4: Multi-Step Job -> Process Orchestration

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| J4.1 | Build job step dependency graph | COND codes + IF/THEN/ELSE | Step sequencing |
| J4.2 | Detect SORT steps | EXEC PGM=SORT/DFSORT/SYNCSORT | .proc order by + group by |
| J4.3 | Detect IDCAMS steps | EXEC PGM=IDCAMS (VSAM utility) | .proc data management |
| J4.4 | Detect IEFBR14 steps | Allocation-only steps | .proc lifecycle management |
| J4.5 | Map PROC invocations | Cataloged procedures -> reusable sub-processes | .proc import references |
| J4.6 | Emit .proc from job structure | One .proc per JCL job | Complete batch orchestration |

### Phase J5: TSO/DB2 Integration

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| J5.1 | Parse SYSTSIN DD for DSN commands | TSOParser + DSNTSOParser | DB2 program execution context |
| J5.2 | Extract RUN PROGRAM from DSN | Actual DB2 program being executed | Link JCL step -> COBOL program |
| J5.3 | Extract BIND/FREE from DSN | DB2 plan management | Deployment automation |

---

## DLI Backlog (25 commands -> .schema, .proc)

IMS/DLI is a hierarchical database. DLI commands access segment
hierarchies. Relevant for clients using IMS (banking, insurance).

### Phase I1: Grammar Integration + AST

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| I1.1 | Copy DLIParser.g4 + DLILexer.g4 to grammar/mainframe/dli/ | File copy + LICENSE | Grammar available |
| I1.2 | Create cobol-dli crate skeleton | New crate | Cargo.toml, lib.rs |
| I1.3 | Generate Rust parser via antlr-rust | Build script | DLI commands parsed |
| I1.4 | Define DliCommand AST enum | 25 command types | Typed AST |
| I1.5 | Wire to existing EXEC DLI extraction | Replace raw text heuristics | Structured DLI analysis |

### Phase I2: Schema Extraction

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| I2.1 | Extract segment hierarchy from GU/GN/GNP patterns | PCB references, segment names | .schema from IMS segments |
| I2.2 | Map SSA (Segment Search Arguments) | Key fields, qualification | .schema identity/constraints |
| I2.3 | Detect parent-child relationships | GNP (Get Next in Parent) usage | .schema nested objects |
| I2.4 | Map DLI data types | IMS field definitions -> Nexflow types | .schema field types |

### Phase I3: Process Enhancement

| ID | Task | Scope | Outcome |
|----|------|-------|---------|
| I3.1 | Map GU/GN to receive operations | Database read patterns | .proc receive from db |
| I3.2 | Map ISRT/REPL/DLET to emit operations | Database write patterns | .proc emit to db |
| I3.3 | Map CHKP/XRST to checkpoints | IMS checkpoint/restart | .proc resilience |
| I3.4 | Map ROLB/ROLL/ROLS to rollback | Transaction rollback patterns | .proc error handling |
| I3.5 | Detect batch vs BMP patterns | DLI SCHEDULE command | .proc mode batch/stream |

---

## Implementation Dependencies

```
Phase 1 (Foundation):
    C1 (CICS grammar) -----> C2 (.screen), C3 (.api), C4 (.service)
    D1 (DB2z grammar) -----> D2 (.schema), D3 (.proc)
    J1 (JCL grammar) ------> J2-J4 (.proc batch)
    I1 (DLI grammar) ------> I2 (.schema), I3 (.proc)

Phase 2 (Emitters):
    C2 (.screen) ----------> needs BmsDSL.g4 parser (have grammar)
    C3 (.api) + C4 (.service) -> needs C1 + cobol-transpiler LINKAGE data
    D2 (.schema) ----------> enhances existing E1 SchemaEmitter
    J2-J4 (.proc) ---------> enhances existing E4 ProcessEmitter

Phase 3 (Integration):
    C3 + C4 + C5 ---------> full online program model
    D3 + J2 ---------------> full batch program model
    All --------------------> NexIntel code intelligence graph
```

## Effort Estimates

| Grammar | Foundation | Emitters | Advanced | Total |
|---------|-----------|----------|----------|-------|
| CICS | 3-4 weeks | 4-6 weeks | 2-3 weeks | 9-13 weeks |
| DB2z SQL | 2-3 weeks | 2-3 weeks | 1-2 weeks | 5-8 weeks |
| JCL | 2-3 weeks | 2-3 weeks | 1 week | 5-7 weeks |
| DLI | 1-2 weeks | 1-2 weeks | 1 week | 3-5 weeks |

**Recommended start: CICS (highest value, unlocks .api/.screen/.service)**
