---

marp: true
theme: uncover
paginate: true
style: |
/* VS Code Dark Modern Theme */

*   { letter-spacing: normal !important; }
    :root {
    --bg-primary: #1f1f1f;
    --bg-secondary: #252526;
    --bg-elevated: #2d2d2d;
    --fg-primary: #cccccc;
    --fg-secondary: #9d9d9d;
    --accent-blue: #0078d4;
    --accent-blue-light: #3794ff;
    --accent-green: #4ec9b0;
    --accent-orange: #ce9178;
    --accent-yellow: #dcdcaa;
    --accent-purple: #c586c0;
    --border: #3c3c3c;
    }
    section { background: var(--bg-primary); color: var(--fg-primary); font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 18px; letter-spacing: normal; padding: 35px 50px; justify-content: flex-start; text-align: left; align-items: flex-start; }
    h1 { color: var(--accent-blue-light); font-weight: 600; font-size: 1.5em; border-bottom: 2px solid var(--accent-blue); padding-bottom: 6px; margin-bottom: 16px; }
    h2 { color: var(--accent-green); font-size: 1.25em; }
    h3 { color: var(--accent-yellow); font-size: 1.05em; }
    h6 { color: var(--fg-secondary); font-weight: 400; font-size: 0.8em; }
    strong { color: var(--accent-orange); }
    table { width: 100%; border-collapse: collapse; font-size: 0.75em; border: 1px solid var(--border); }
    th { background: var(--bg-elevated); color: var(--accent-blue-light); padding: 8px 12px; text-align: left; font-weight: 600; border-bottom: 2px solid var(--accent-blue); }
    td { padding: 6px 12px; border-bottom: 1px solid var(--border); }
    tr:nth-child(even) { background: var(--bg-secondary); }
    code { font-family: 'Fira Code', monospace; background: var(--bg-secondary); color: var(--accent-orange); padding: 2px 6px; border-radius: 4px; border: 1px solid var(--border); }
    pre { background: var(--bg-secondary); padding: 12px 16px; border-radius: 6px; font-family: 'Fira Code', monospace; font-size: 0.65em; line-height: 1.4; border: 1px solid var(--border); border-left: 3px solid var(--accent-blue); }
    pre code { background: transparent; border: none; padding: 0; color: var(--fg-primary); }
    blockquote { border-left: 4px solid var(--accent-blue); background: var(--bg-secondary); padding: 10px 16px; font-style: italic; border-radius: 0 6px 6px 0; }
    blockquote strong { color: var(--accent-green); }
    ul, ol { font-size: 0.95em; line-height: 1.5; margin: 8px 0; }
    li { margin-bottom: 4px; }
    li::marker { color: var(--accent-blue); }
    section.dense { font-size: 16px; padding: 30px 40px; }
    section.dense table { font-size: 0.85em; }
    section::after { color: var(--fg-secondary); }

---

## cobol2rust + Nexflow

## Roadmap: Next Phase

###### 2026-03-22

---

# What's Done (Sessions 1-49 + A-F)

| Area | Status | Key Numbers |
|---|---|---|
| COBOL Parser + AST | DONE | Full IBM COBOL coverage |
| Transpiler (COBOL -> Rust) | DONE | 90.3% success, 51K files validated |
| Runtime Library (10 crates) | DONE | 906 unit tests |
| CLI (10 subcommands) | DONE | transpile, scan, check, parse, compile, diff... |
| Enterprise Scanner | DONE | 280K+ files, DuckDB persistence |
| Rustify Tiers 1-4 | DONE | 202 tests, .cobol2rust-target.toml |
| Nexflow DSL Emitters (E1-E4) | DONE | 104 tests, typed AST, confidence 1.00 |
| Expression Extraction | DONE | Conditions + actions from Rust AST |
| Typed DSL AST | DONE | Grammar-valid by construction |
| DSL Writer + CLI --emit-dsl | DONE | .rustify/dsl/ with manifest |

---

# Decisions Made This Session

| Decision | Rationale |
|---|---|
| DSL emitters are target-agnostic | Business logic is the same regardless of runtime |
| Typed AST, no confidence scores | If it compiles, it parses |
| DSL emission allowed on Phase 1 | Schemas, rules, process flow are complete on Phase 1 |
| Transform expressions deferred to Stage 2 | cobol_add() patterns disappear after Tier 2 rustification |
| 40/60 split (COBOL/Nexflow) | 40% = extracted business logic, 60% = modern architecture |
| DSL = enterprise knowledge graph | Queryable semantic layer, living knowledge base |
| NDJSON for lineage storage | Git-friendly, DuckDB queries natively |
| Cluster analysis over single-field lineage | Business concepts are groups of fields, not individuals |
| DSL emitters should read COBOL AST directly | Makes emission target-independent (Rust and Java) |

---

# Open Items: Prioritized

### P0: Client-Driven (do first)

| Item | Sessions | Details |
|---|---|---|
| **Java codegen (cobol2java)** | 8 (J1-J8) | Parallel codegen module, Java runtime library, Maven template |
| **Monster-file splitting** | 1 | transpile --workspace --split-by section for 1M-line programs |
| **E2E on client's 1500 files** | 1-2 | Transpile + rustify + emit-dsl on real codebase |

### P1: Product Quality (do next)

| Item | Sessions | Details |
|---|---|---|
| **Nexflow CLI validation** | 1 | Shell out to nexflow validate --format json after emission |
| **DSL emitters read COBOL AST** | 1 | Remove syn dependency, make emission target-independent |
| **Transform expr extraction (Stage 2)** | 1 | Run emitters on Tier 2+ Rust for ws.count += 100 patterns |

---

# Open Items: Continued

### P2: Enterprise Features (plan then build)

| Item | Sessions | Details |
|---|---|---|
| **Lineage NDJSON emission** | 1 | Emit field_access.ndjson during transpile, queryable via DuckDB |
| **Cluster discovery** | 2 | Co-access analysis, naming conventions, copybook boundaries |
| **Business function identification** | 1 | Input cluster -> paragraph -> output cluster mapping |
| **Cross-program lineage** | 1 | Which programs share fields via copybooks |
| **Migration mapping database** | 2 | Source cluster -> target entity mapping with SME workflow |

### P3: Architecture (future)

| Item | Sessions | Details |
|---|---|---|
| **Semantic knowledge graph** | 3-4 | Full graph model: nodes=entities, edges=lineage |
| **Graph storage decision** | -- | DuckDB+recursive CTE for now, evaluate CozoDB/Neo4j at scale |
| **Nexflow JSON AST interchange** | 1 | Emit JSON AST for direct Nexflow consumption (skip parsing) |
| **Nexflow Rust migration** | -- | Out of scope, but typed AST is the foundation |

---

# cobol2java: Detailed Plan

### Architecture

```
COBOL AST (shared)
    |
    +---> codegen/       (existing Rust codegen, untouched)
    |
    +---> codegen_java/  (NEW: parallel Java codegen)
    |
    +---> dsl emitters   (shared, target-independent)
```

### Sessions

| Session | Deliverable |
|---|---|
| J1 | JavaWriter + WorkingStorage class generation |
| J2 | IF, EVALUATE, PERFORM, run() dispatch |
| J3 | MOVE, COMPUTE, ADD/SUB/MUL/DIV, DISPLAY |
| J4 | File I/O (Sequential/Relative/Indexed) + EXEC SQL (JDBC) |
| J5 | Java runtime library (CobolDecimal, CobolString, CobolRuntime) |
| J6 | CLI --target java, Maven pom.xml template, E2E test |
| J7 | Stress tests (47 tests against Java output) |
| J8 | @Cobol annotations for DSL emitter compatibility |

> Full spec: `docs/cobol2java_plan.md`

---

# Lineage + Cluster Analysis: Design

### Storage: NDJSON (git) + DuckDB (query)

```
reports/lineage/
    fields.ndjson           -- {program, field, pic, level, copybook}
    field_access.ndjson     -- {program, section, paragraph, field, access_type}
    performs.ndjson          -- {program, paragraph, target}
    clusters.ndjson         -- {cluster_id, fields[], signal, confidence}
    functions.ndjson        -- {paragraph, input_cluster, output_cluster}
    copybook_usage.ndjson   -- {copybook, field, programs[]}
```

### Cluster Discovery Signals

| Signal | How It Works |
|---|---|
| **Co-access** | Fields read together in 80%+ of their paragraphs form a cluster |
| **Naming convention** | WS-ACCT-* fields = Account cluster (prefix grouping) |
| **Copybook boundary** | Fields from same COPY = one business entity |
| **Data flow** | Read-cluster -> paragraph -> write-cluster = business function |

### Lineage Queries (DuckDB on NDJSON)

```sql
-- Where is WS-ACCT-BAL read/written?
SELECT * FROM read_ndjson_auto('field_access.ndjson')
WHERE field_name = 'WS-ACCT-BAL';

-- What cluster does WS-ACCT-BAL belong to?
SELECT * FROM read_ndjson_auto('clusters.ndjson')
WHERE list_contains(fields, 'WS-ACCT-BAL');

-- What business function produces the credit decision?
SELECT * FROM read_ndjson_auto('functions.ndjson')
WHERE output_cluster = 'credit_decision';
```

---

# Data Mapping: Source -> Target

### The Problem

COBOL has `WS-ACCT-NBR`. Target has `account_id`. Same concept, different names,
different structures. Tens of thousands of mappings needed.

### The Solution: Cluster-to-Entity Mapping

```
COBOL Cluster                    Target Entity
--------------                   -------------
{WS-SCORE, WS-DTI, WS-INCOME}   CreditApplication.riskProfile
{WS-DECISION, WS-REASON-CODE}   CreditDecision
{WS-BASE-RATE, WS-ADJUSTED-RATE} PricingResult
{WS-CUST-NAME, ..., WS-CUST-SSN} Customer
```

Mapping is **cluster-to-entity**, not field-to-field. This reduces tens of
thousands of individual mappings to hundreds of cluster mappings.

### Storage

Same NDJSON pattern:
```
reports/mapping/
    cluster_entity_map.ndjson  -- {source_cluster, target_entity, mapped_by, mapped_at}
    field_attr_map.ndjson      -- {source_field, target_attr, transform_expr}
```

---

# Grammar Change Impact

### When Nexflow grammars change:

| Change Type | Frequency | Impact | Fix Time |
|---|---|---|---|
| New feature added | Common | Zero (we skip it) | N/A |
| Syntax change | Rare | Update to_text() serializer | 15 minutes |
| Construct removed | Very rare | Rename in AST | 30 minutes |

The typed AST (`dsl_ast.rs`) isolates grammar changes to the serializer layer.
Emitter logic (what to extract from COBOL, how to classify functions) is unaffected.

> **You control both projects.** Grammar changes are coordinated, not surprises.

---

# Key Documents

| Document | What It Covers |
|---|---|
| `docs/cobol2rust_nexflow_value_proposition.md` | Client-facing: 40/60 split, knowledge graph |
| `docs/cobol2java_plan.md` | Java target: 8 sessions, architecture |
| `docs/cobol2rust_dsl_staging.md` | Two-stage emission: structure now, expressions after Tier 2 |
| `docs/cobol2rust_typed_dsl_emitter_spec.md` | Typed AST design: grammar-valid by construction |
| `docs/cobol2rust_nexflow_emitter_spec.md` | E1-E4 detailed design + test cases |
| `docs/emitter_capabilities.md` | What's done, partial, planned per emitter |
| `docs/cobol2rust_roadmap_next.md` | This document: all open items and decisions |
