# Artifact Architecture

> Three layers: Graph (topology) + AST Store (content) + DSL (intent)

**Status**: DECIDED (2026-04-10)
**Context**: Emerged from monster-file debugging and transpile audit

## The Three Artifacts

Each artifact serves a different query pattern and user persona.

```
Layer 1: Graph (.nxg)      -- WHO and WHERE
                               "Which programs modify ACCT-BAL?"
                               "What is the impact radius of changing CALC-TAX?"
                               Queryable via NexQuery.

Layer 2: AST Store (future) -- WHAT
                               "What does paragraph 2200-CALC do?"
                               "Find all COMPUTEs targeting NET-PAY with ON SIZE ERROR"
                               Decomposed NDJSON, queryable via DuckDB.

Layer 3: DSL Files          -- HOW
                               "How should this be modernized?"
                               .schema/.xform/.rules/.proc
                               Compiled by Nexflow to Java/Rust.
```

## Layer 1: Code Intelligence Graph (DONE)

**Format**: `.nxg` (encrypted bincode, AES-256-GCM)
**Built by**: `cobol2rust build-graph`
**Queried by**: `cobol2rust query` (NexQuery DSL)

### Nodes (7 types)

| Type | Properties | Source |
|---|---|---|
| Program | paragraph_count | IDENTIFICATION DIVISION |
| Paragraph | section parent | PROCEDURE DIVISION |
| Field | section, pic, usage | DATA DIVISION |
| Copybook | name | COPY statements |
| File | organization, access_mode | FILE-CONTROL + FD |
| Table | sql_type | EXEC SQL |
| Rule | (stub) | Future DSL extraction |

### Edges (7 types)

| Type | From -> To | Source |
|---|---|---|
| Calls | Program -> Program | CALL statement |
| Performs | Paragraph -> Paragraph | PERFORM statement |
| Reads | Paragraph -> Field | Statement operands |
| Writes | Paragraph -> Field | Statement targets |
| Uses | Program -> Copybook | COPY statement |
| Accesses | Program -> File/Table | OPEN/READ/WRITE, EXEC SQL |
| Contains | Program -> Paragraph | Structure |

### Capabilities

- 14 bidirectional traversal verbs (calling/called-by, etc.)
- 9 domain verbs (rank, find-dead, trace, deps, impact, similar, etc.)
- Property filters (where complexity > 50, where name matches "ACCT*")
- Interactive REPL, inline, and file execution modes
- Enrichment passes (11 analysis types)

## Layer 2: AST Store (DEFERRED)

**Format**: Decomposed NDJSON (one line per addressable unit)
**Queryable via**: DuckDB (`read_ndjson_auto()`)
**Status**: Design complete, implementation deferred

### Design Rationale

Raw AST JSON is too large and deeply nested for ad-hoc queries.
Decomposed NDJSON flattens the tree into queryable rows:

```jsonl
{"program":"SSUPDATE","division":"data","name":"WS-ACCT-BAL","level":1,"pic":"S9(7)V99","usage":"COMP-3"}
{"program":"SSUPDATE","division":"proc","paragraph":"2200-CALC","stmt_idx":5,"type":"compute","target":"WS-NET-PAY","expr":"WS-GROSS - WS-DEDUCTIONS","on_size_error":true}
```

Benefits:
- Git-friendly (line-level diffs)
- DuckDB queries natively
- Append-only (incremental rebuild)
- Consistent with lineage NDJSON format (P2 roadmap)

### Future Integration

Graph nodes gain an `ndjson_ref` property pointing to the AST store.
NexQuery gains a `drill` verb that fetches statement-level detail.

## Layer 3: DSL Files (DONE)

**Format**: .schema, .xform, .rules, .proc (7 grammars planned)
**Built by**: `cobol2rust rustify --emit-dsl`
**Compiled by**: Nexflow master compiler

### Grammars

| Layer | Grammar | Ext | Status |
|---|---|---|---|
| Building Block | SchemaDSL | .schema | DONE |
| Building Block | RulesDSL | .rules | DONE |
| Building Block | TransformDSL | .xform | DONE |
| Contract | ApiDSL | .api | DESIGNED |
| Contract | ScreenDSL | .screen | FUTURE |
| Orchestration | ServiceDSL | .service | DESIGNED |
| Orchestration | ProcDSL | .proc | DONE |

### Confidence Scoring

| Emitter | Range | Notes |
|---|---|---|
| Schema | 0.9-1.0 | PIC-to-type is deterministic |
| Transform | 0.7-0.9 | Expression extraction may be incomplete |
| Rules | 0.6-0.85 | Decision table detection is heuristic |
| Process | 0.75-0.95 | Call graph is sound, I/O detection is heuristic |

Grammar validity is guaranteed by construction (typed DSL AST).
Semantic correctness requires SME review for confidence < 0.85.

## Artifact Relationships

```
+------------------+
| COBOL Source     |
+--------+---------+
         |
    [ANTLR4 Parser]
         |
+--------v---------+
| COBOL AST        |  (in-memory, per-file, serializable)
+--+------+------+-+
   |      |      |
   |      |      +---> [DSL Emitters] ---> Layer 3: DSL Files
   |      |                                  |
   |      +----------> [Graph Builder] --> Layer 1: Graph (.nxg)
   |                                         |
   +--- (future) --> [AST Decomposer] --> Layer 2: AST Store (NDJSON)
```

The AST is the source of truth for all three artifacts.
DSL emitters will read the AST directly (see direct_dsl_emission_spec.md).

## Key Decision: AST is Foundation, Not Artifact

The AST is the richest per-file representation, but it is:
- Per-program (no cross-program view)
- Tree-shaped (wrong for relational queries)
- Large (~50-100MB JSON for monster files)

Therefore, the AST is the **foundation** that feeds the three artifacts,
not itself a persisted artifact. The graph, AST store, and DSLs are the
queryable, versionable, deployable outputs.
