# NexIntel: Code Intelligence

## Brainstorming Journal

**Started**: 2026-03-27
**Status**: Discovery phase -- defining scope, layers, priorities

---

## Context

NexIntel already has **Data Intelligence** (cobol-data crate, NSD1-NSD4):
- Binary COBOL data decode (EBCDIC, packed decimal, zoned, text)
- REDEFINES visualization with discriminator detection
- Auto-discovery (file-to-copybook matching)
- 96 tests, 3 CLI subcommands (decode, discover, data-analyze)

**Code Intelligence** is the counterpart: understand legacy *code* (not just data)
in the mainframe/COBOL modernization context.

---

## The Intelligence Stack (10 Layers)

```
Layer 10: Runtime Behavior    (production traces, volumes, performance)
Layer 9:  Migration Readiness (scoring, classification, ordering)
Layer 8:  Pattern & Idiom     (templates, anti-patterns, generation detection)
Layer 7:  Dependency & Impact (change radius, risk scoring)
Layer 6:  Business Logic      (rules, calculations, domain entities)
Layer 5:  External Interfaces (files, DB, CICS, MQ, JCL)
Layer 4:  Inter-Program       (CALL graph, parameter flow, copybook coupling)
Layer 3:  Data Flow           (field tracking, MOVE chains, value propagation)
Layer 2:  Control Flow        (PERFORM graph, loops, dead code)
Layer 1:  Structural          (inventory, complexity, size)
         ----------------------------------------------------------
Layer 0:  Data Intelligence   (DONE -- cobol-data crate)
```

---

## Layer 1: Structural Intelligence

**Question**: What is the shape of each program?

| Capability | Description | Difficulty |
|---|---|---|
| Division/Section/Paragraph inventory | Entry points, flow skeleton | Easy |
| Dead code detection | Paragraphs never PERFORMed, unreachable sections | Medium |
| Code complexity metrics | Cyclomatic complexity, nesting depth, McCabe index | Easy |
| Copy/paste detection | Duplicated logic across programs (not COPY -- actual clones) | Hard |
| Program size decomposition | Where are the monsters? Which sections are bulk? | Easy |
| GOTO/ALTER spaghetti mapping | Unstructured flow needing special treatment | Medium |

**Priority**: TBD
**What we have today**: AST gives us the raw material. scan command has basic inventory.

---

## Layer 2: Control Flow Intelligence

**Question**: How does execution move through a program?

| Capability | Description | Difficulty |
|---|---|---|
| PERFORM graph | Full call tree (PERFORM X THRU Y, nested) | Medium |
| PERFORM THRU range analysis | Implicit coupling via swept paragraphs | Medium |
| Conditional flow paths | IF/EVALUATE decision trees, all paths | Medium |
| Loop detection | PERFORM UNTIL, PERFORM VARYING, GOTO loops | Easy |
| Fall-through analysis | Paragraphs falling into next (no STOP RUN/exit) | Easy |
| Exception/error paths | ON SIZE ERROR, AT END, INVALID KEY, FILE STATUS | Medium |
| SORT/MERGE procedures | Input/output procedures as callbacks | Easy |

**Priority**: TBD
**What we have today**: AST has all PERFORM/IF/EVALUATE nodes. field_analysis.rs tracks some flow.

---

## Layer 3: Data Flow Intelligence

**Question**: How does data move through computation?

| Capability | Description | Difficulty |
|---|---|---|
| Field read/write tracking | Per-paragraph field access (read/write/both) | DONE (field_analysis.rs) |
| Field lifecycle | Initialize -> populate -> transform -> output | Medium |
| Value propagation | Constant propagation -- what values can a field hold? | Hard |
| MOVE chain analysis | A -> B -> C chains with type coercion at each step | Medium |
| Accumulator detection | Fields ADD'd in loops (running totals, counters) | Easy |
| REDEFINES aliasing | Two names = same memory, cross-alias data flow | Medium |
| Implicit flow via COPY | Fields in copybooks creating hidden coupling | Easy |

**Priority**: TBD
**What we have today**: field_analysis.rs computes ParagraphFieldAccess (reads/writes/performs).

---

## Layer 4: Inter-Program Intelligence

**Question**: How do programs interact with each other?

| Capability | Description | Difficulty |
|---|---|---|
| CALL graph | Which programs CALL which, with what parameters | Medium |
| CALL parameter analysis | Data flow via BY REFERENCE/CONTENT/VALUE | Medium |
| Program entry points | ENTRY statements (multiple entry points) | Easy |
| Subprogram classification | Utility vs. business module vs. system hook | Medium |
| CANCEL semantics | Programs losing WORKING-STORAGE state | Easy |
| Dynamic CALL resolution | CALL WS-PROG-NAME (variable target) | Hard |
| Copybook coupling | Programs sharing copybooks = shared data contracts | Easy |

**Priority**: TBD
**What we have today**: AST has CALL statements. Copybook tracking exists in transpiler.

---

## Layer 5: External Interface Intelligence

**Question**: How does the program talk to the outside world?

| Capability | Description | Difficulty |
|---|---|---|
| File I/O inventory | FDs, organization (SEQ/INDEXED/REL), access mode | Easy |
| Database interactions | EXEC SQL, tables, CRUD classification | Medium |
| Screen/CICS transactions | SEND MAP, RECEIVE MAP, terminal I/O | Medium |
| MQ/messaging | EXEC CICS WRITEQ, MQ PUT/GET patterns | Medium |
| JCL dependency mapping | Jobs -> programs, DD cards, execution order | Hard* |
| Sort/merge work files | SORT USING/GIVING, external sort interfaces | Easy |
| System calls | ACCEPT DATE, DISPLAY UPON CONSOLE, CALL 'SYSTEM' | Easy |

*JCL requires a separate parser

**Priority**: TBD
**What we have today**: AST has EXEC SQL, file FDs. scan command inventories these.

---

## Layer 6: Business Logic Intelligence

**Question**: What business rules are encoded in the code?

| Capability | Description | Difficulty |
|---|---|---|
| Rule extraction | IF/EVALUATE conditions as business decisions | DONE (DSL emitters) |
| Business function identification | Paragraphs = complete business operations | Medium |
| Validation logic detection | Range checks, format checks, required fields | Easy |
| Calculation logic | COMPUTE/arithmetic = formulas, rates, pricing | Medium |
| State machine detection | Status codes tracking workflow states | Hard |
| Temporal logic | Date arithmetic, aging, period boundaries | Medium |
| Domain entity inference | Field clusters -> business entities | PLANNED (cluster analysis) |

**Priority**: TBD
**What we have today**: DSL emitters extract rules. Cluster analysis planned.

---

## Layer 7: Dependency & Impact Intelligence

**Question**: What breaks if I change something?

| Capability | Description | Difficulty |
|---|---|---|
| Change impact radius | Modify field X -> affected paragraphs/programs/copybooks/JCL | Medium |
| Copybook impact analysis | Changing a copybook ripples to all COPY users | Easy |
| Batch job chain dependencies | Program A output -> Program B input (JCL wiring) | Hard* |
| Shared resource contention | Programs competing for files, tables, CICS resources | Medium |
| Test coverage inference | Which programs have test JCL/data? | Hard |
| Risk scoring | complexity * coupling * change-frequency | Medium |

*Requires JCL parsing

**Priority**: TBD
**What we have today**: Copybook tracking exists. scan has cross-references.

---

## Layer 8: Pattern & Idiom Intelligence

**Question**: What recurring patterns exist across the codebase?

| Capability | Description | Difficulty |
|---|---|---|
| COBOL idiom catalog | Common patterns -> standard library mappings | Medium |
| Anti-pattern detection | GOTO spaghetti, ALTER, deep nesting, global state abuse | Medium |
| Generated code detection | BMS maps, DCLGEN, report writers vs. hand-written | Medium |
| Template programs | Read-process-write, CICS pseudo-conversational skeletons | Medium |
| Batch vs. online classification | Different modernization strategies | Easy |
| Era detection | Pre-structured / structured / modern (OO, EXEC SQL) | Easy |

**Priority**: TBD
**What we have today**: Some pattern recognition in rustify tier classification.

---

## Layer 9: Migration Readiness Intelligence

**Question**: How hard is each program to modernize?

| Capability | Description | Difficulty |
|---|---|---|
| Difficulty scoring | 1-5 scale: complexity, coupling, external deps | Medium |
| Automated vs. manual classification | Transpiler handles vs. needs human review | Easy |
| Feature coverage gap | COBOL features used but not supported by transpiler | Easy |
| Strangler fig candidates | Clean interfaces = extract first | Medium |
| Batch window analysis | Timing constraints affecting migration order | Hard |
| Regression test feasibility | Can we compare before/after? Test datasets exist? | Medium |

**Priority**: TBD
**What we have today**: scan command has some readiness metrics.

---

## Layer 10: Runtime Behavior Intelligence (Advanced)

**Question**: What happens when the code actually runs?

| Capability | Description | Difficulty |
|---|---|---|
| Hot path identification | Most-executed paragraphs (from production traces) | Hard* |
| Data volume profiling | Record counts per program (from JCL/SMF data) | Hard* |
| Performance characteristics | CPU-bound vs. I/O-bound vs. DB-bound | Hard* |
| Failure mode analysis | Common abend codes, error recovery, restart points | Medium |
| Concurrency patterns | Multi-region CICS, parallel batch streams | Hard |

*Requires production logs/traces from mainframe

**Priority**: TBD
**What we have today**: Nothing (requires runtime data from client environment).

---

## Key Insight: Process-Centric Vertical Slicing

The real question users ask is NOT "show me Layer 3 data flow."
It is: **"Show me the code that runs this business process."**

Examples:
- "What code handles address changes?"
- "Show me everything involved in the clearing process."
- "What happens when we process a loan application?"

This requires a **vertical slice through multiple layers** from a single entry point:

```
Given entry point (program, CICS transaction, JCL step):
  Layer 1 -> identify the program, its paragraphs
  Layer 2 -> PERFORM graph (intra-program execution tree)
  Layer 4 -> CALL graph (inter-program: subprograms invoked)
  Layer 3 -> data flow through the execution path
  Layer 5 -> external I/O (files, DB, screens, MQ)
  Layer 6 -> business rules applied along the path
  Layer 7 -> downstream consumers (what depends on outputs?)
```

### Process Trace: 5 Levels of Detail

```
Level 1: ENTRY POINT
   Where does the process START?
   -> JCL job step, CICS transaction, program entry

Level 2: EXECUTION TREE
   What code gets invoked?
   -> PERFORM graph (within program) + CALL graph (across programs)

Level 3: DATA TOUCHED
   What data does this process read and write?
   -> Files, DB tables, working-storage, copybooks

Level 4: BUSINESS RULES APPLIED
   What decisions does the process make?
   -> IF/EVALUATE conditions, validations, calculations

Level 5: PROCESS BOUNDARY
   Where does THIS process end and the NEXT begin?
   -> Output files feeding other jobs, CICS RETURN, status codes
```

### Product Feature: `cobol2rust trace`

```
cobol2rust trace --entry CLRG0100 --depth full --output clearing.json

{
  "process": "CLRG0100",
  "entry_type": "batch",
  "execution_tree": { ... },
  "programs_involved": ["CLRG0100", "VALUTIL", "BALUPD", "ERRHAND"],
  "paragraphs_executed": 23,
  "files_accessed": { "read": [...], "write": [...] },
  "sql_operations": { "select": [...], "update": [...] },
  "copybooks_used": ["CPYCLRG", "CPYACCT", "CPYTXN"],
  "business_rules": [ ... ],
  "downstream": ["CLRGRPT", "CLRGEXC"],
  "complexity_score": 3.7,
  "estimated_loc": 1847
}
```

---

## Two Product Modes

### Mode 1: Discovery Mode (Automated Scan -- "the X-ray")

Push-based. The system scans the entire codebase and surfaces structure humans
haven't articulated yet. Produces a **process catalog** -- the table of contents.

```
cobol2rust discover-processes --output processes.json

Result:
  47 processes discovered
  12 high-complexity (>5 programs, >10K LOC)
   8 medium-complexity (2-5 programs, 2-10K LOC)
  27 simple (1 program, <2K LOC)
```

**Auto-discovery algorithm:**

1. **Find entry points** -- programs never CALLed by others = top-level.
   Also: CICS transactions, JCL EXEC PGM= (if available)
2. **Build execution trees** -- trace PERFORM + CALL graph from each entry point
3. **Cluster by affinity** -- naming prefix (CLRG*), shared copybooks,
   CALL chains, shared file I/O (one writes, another reads = pipeline)
4. **Name the clusters** -- common prefix, JCL job name, CICS transaction ID,
   fallback: CLUSTER_NNN (human labels later)
5. **Rank** -- complexity = programs * paragraphs * external_interfaces

Characteristics:
- Exhaustive (covers entire codebase)
- Opinionated (makes guesses, human reviews/corrects)
- One-time + incremental (run on onboarding, re-run on code changes)

### Mode 2: Investigation Mode (Interactive Query -- "SQL for code")

Pull-based. The user explores, queries, drills down. Like DBeaver for code.

```
Query: "programs that write to ACCTMAST"
Result: CLRG0100 (paragraph 1400-UPDATE-BALANCES)
        ACMT0200 (paragraph 2300-UPDATE-MASTER)
        LNOR0300 (paragraph 3100-CREATE-ACCOUNT)

Action: [Trace CLRG0100] -> full process vertical slice
Action: Click paragraph -> see code, fields, rules
Action: Click field -> pivot to "where else is this used?"
```

Characteristics:
- Ad-hoc queries, user-driven
- Drill-down navigation (program -> paragraph -> field -> cross-refs)
- Cross-reference pivoting (I'm looking at X, show me everything touching X)
- Investigations can be saved, resumed, shared

### How They Connect

```
DISCOVERY MODE          -->  PROCESS CATALOG        -->  INVESTIGATION MODE
(automated scan)             (table of contents)         (interactive query)
"Here are your 47            User picks one               "Drill into clearing:
 business processes"                                       show me everything"
```

**Discovery = breadth (what processes exist?)**
**Investigation = depth (what does THIS process do?)**

---

## Eight Investigation Patterns

```
#  Pattern              Direction           Question
-- -------------------  ------------------  ---------------------------------
1  Process Trace        code -> data        "What does this process do?"
2  Data Carving         data -> code        "What code touches this entity?"
3  Impact Analysis      change -> blast     "What breaks if I change this?"
4  Similarity Search    code -> code        "What else looks like this?"
5  Dead Code Detection  code -> status      "What can we ignore?"
6  Dependency Chain     code -> order       "What must I migrate first?"
7  Complexity Hotspots  code -> metrics     "Where are the dragons?"
8  Rule Comparison      rule -> rule        "Are these rules consistent?"
```

### Pattern 4: Similarity Search
Once you modernize one program, find all structurally similar ones.
Same conversion recipe, same target architecture, batch processing.
Similarity axes: structural (skeleton), data (copybooks/files), logic,
interface (CALL/I/O pattern), template (generated code).

### Pattern 5: Dead Code Detection
20-40% of a legacy estate may be dead. Highest-ROI query -- shrinks scope.
Levels: program (never called), paragraph (never PERFORMed), field (never
used), copybook (never COPYed), branch (condition always false).
Classification: LIVE / SUSPECT (dynamic CALL) / DEAD (provably unreachable).

### Pattern 6: Dependency Chain
Migration sequencing -- topological sort of dependencies.
"Migrate CLRG0100" -> must migrate VALUTIL, BALUPD first (or wrap).
Output: wave plan (Wave 1: no deps, Wave 2: depends on Wave 1, ...).
Identifies where bridges/wrappers are needed for phased migration.

### Pattern 7: Complexity Hotspots
Ranked by composite score: LOC, cyclomatic complexity, nesting depth,
GOTO count, external dependencies. Risk assessment and resource allocation.
Distribution histogram across codebase.

### Pattern 8: Rule Comparison
Find duplicated or inconsistent business rules across programs.
Side-by-side comparison. "Do CLRG0100 and CLRG0100-V2 apply the same
clearing rules?" Also: search for all locations applying a specific rule.

### Data Carving: 3-Phase Workflow

```
Phase 1: FIELD QUERY (buildable now)
   Input:  set of COBOL field names
   Output: programs + paragraphs that read/write/both each field

Phase 2: DATA MAPPING (later)
   Map legacy fields to target entities:
     WS-ACCT-NBR    --> Account.id
     WS-ACCT-BAL    --> Account.balance
   Enables entity-level queries instead of field-level

Phase 3: SERVICE BOUNDARY CARVING (the payoff)
   Input:  target entity "Account" + its mapped fields
   Output: code categorized as:
     PURE   -- touches only this entity's fields (clean extraction)
     SHARED -- touches 2+ entities (boundary crossing, needs splitting)
     INFRA  -- no business fields (shared library candidate)
   Coupling score: % pure vs shared = extraction feasibility
```

---

## Open Questions

1. Which layers feel most valuable to the client right now?
2. How much do we get "for free" from existing AST/transpiler?
3. Is JCL in scope? (Layers 5, 7 depend heavily on it)
4. Is the client estate batch-only, online (CICS), or mixed?
5. ~~Should we build a single `cobol-intel` crate or per-layer crates?~~ **DECIDED**: single `cobol-intel` crate
6. NexStudio visualization for each layer?
7. ~~Output format: NDJSON (like lineage plan) or something else?~~ **DECIDED**: JSON tree output, encrypted .nxg binary for graph persistence
8. ~~Investigation query language: natural language? structured DSL? graphical builder?~~ **DECIDED**: NexQuery (see docs/nexquery_spec.md)
9. Discovery: how much human review is needed before catalog is trustworthy?
10. ~~Can investigation queries be saved as "views" for ongoing monitoring?~~ **DECIDED**: yes, `save`/`run` commands in NexQuery
11. Data mapping: manual? assisted? part of NexIntel or separate tool?
12. How does cluster-to-entity mapping (roadmap P2) feed into data carving?

---

## Discussion Log

### 2026-03-27 -- Session 1: Initial Taxonomy

- Defined 10-layer intelligence stack (Layer 0-10)
- Greenfield approach: NexIntel Code Intelligence as counterpart to Data Intelligence

### 2026-03-27 -- Session 1 (cont): Process-Centric Slicing

- Key realization: users ask process questions, not layer questions
- "Show me the clearing process" = vertical slice through Layers 1-7
- Articulated 5 levels of process trace detail
- Proposed `cobol2rust trace` CLI subcommand
- Two examples: "change client address" (CICS/online) and "clearing" (batch/monetary)
- Individual layers are building blocks; process trace is the product feature

### 2026-03-27 -- Session 1 (cont): Two Product Modes

- **Discovery Mode**: automated scan -> process catalog (breadth, push-based)
- **Investigation Mode**: interactive query -> drill-down (depth, pull-based, "SQL for code")
- Discovery produces the map; Investigation explores the territory
- Auto-discovery algorithm: entry points -> execution trees -> clustering -> naming -> ranking
- Investigation UI: query bar + results + trace/drill-down/pivot actions
- Analogy: DBeaver queries databases; NexIntel Investigation queries code

### 2026-03-27 -- Session 1 (cont): Data-Driven Code Carving

- Third investigation pattern: start from DATA, find CODE (inverse of process trace)
- Use case: microservice decomposition ("what code belongs to AccountService?")
- 3-phase workflow: field query (now) -> data mapping (later) -> service boundary carving
- Code categorization: PURE (one service) / SHARED (boundary crossing) / INFRA (library)
- Coupling score = % pure vs shared = extraction feasibility metric
- Data mapping bridges legacy field names to target entity names
- Connects to existing roadmap: cluster-to-entity mapping (P2 lineage)

### 2026-03-27 -- Session 1 (cont): Full Query Pattern Catalog

- Expanded from 3 to 8 investigation patterns
- Pattern 4 (Similarity): batch-process similar programs with same recipe
- Pattern 5 (Dead Code): highest-ROI query, can shrink scope 20-40%
- Pattern 6 (Dependency Chain): migration sequencing, wave planning
- Pattern 7 (Complexity Hotspots): risk assessment, resource allocation
- Pattern 8 (Rule Comparison): consistency checking across programs
- Next: prioritize patterns, discuss underlying data model

### 2026-03-27 -- Session 2: NexQuery Language Design

- Designed NexQuery, a composable domain-specific query language for code intelligence
- Syntax: newline-separated clauses, semicolon terminator, SQL-style comments
- Traversal verbs use English words (calling, called-by, using, used-by, etc.)
- Domain verbs: trace, rank, similar, find-dead, deps, impact, compare, save, run
- Lists use `[]`, filters use `()`, no pipes, no arrows
- Modifiers are bare keywords (by, top, in, with, depth, level, order, scope, threshold, as)
- `each` keyword for back-referencing previous result set
- Output: JSON displayed as tree in REPL
- REPL: `cobol2rust query` (interactive), inline string, or file input (.nxq extension)
- NexStudio: thin UI, calls CLI sidecar, renders JSON response
- Graph storage: petgraph in-memory, encrypted .nxg binary files (AES-256-GCM)
- Key derivation: HKDF-SHA256 from license key + per-file salt
- IP strategy: syntax is public, engine is compiled binary, scoring is trade secret
- Code structure: single `cobol-intel` crate (graph/ + query/ + intel/ + repl/)
- Implementation plan: ~10 sessions across 3 phases
- Full spec: docs/nexquery_spec.md

### Decisions Made (Session 2)

- **Query language**: NexQuery (composable domain verbs, not SQL, not Cypher)
- **Graph DB**: No external DB. petgraph in-memory, .nxg encrypted persistence
- **Encryption**: AES-256-GCM with license-key-derived keys (Option A)
- **Crate structure**: single cobol-intel crate
- **Output format**: JSON (tree display in REPL)
- **REPL**: rustyline-based, dot-commands for meta, .nxq file extension
- **NexStudio**: thin UI, CLI sidecar only, no direct Rust API
