# NexQuery Specification

**Version**: 0.1.0 (Draft)
**Date**: 2026-03-27
**Status**: Design

---

## 1. Overview

NexQuery is a domain-specific query language for exploring COBOL codebases during
modernization. It provides composable, English-readable queries over a code
intelligence graph built from COBOL AST analysis.

NexQuery powers two product modes:
- **Discovery Mode**: automated scan producing a process catalog
- **Investigation Mode**: interactive query and drill-down

### Design Principles

1. **Reads like English**: `programs calling CLRG0100` not `MATCH (p)-[:CALLS]->()`
2. **One way to do things**: no aliases, no shortcuts, no alternative syntax
3. **Composable via newlines**: each line is a clause, newlines chain them
4. **Domain verbs + traversal verbs**: high-level commands mix with graph traversals
5. **JSON output**: results are always JSON, displayed as trees in the REPL


---

## 2. Language Syntax

### 2.1 Statement Structure

```
-- comments use SQL-style double-dash
clause
clause
clause;
```

- Each line is one clause
- Newlines separate clauses (pipeline flow)
- Semicolons terminate statements
- Multiple statements can appear in one file/session

### 2.2 Clause Types

**Traverse clause** -- navigate the graph:
```
<node-type> <traversal-verb> <target> <filter?>
```

**Expand clause** -- extract a dimension from the result set:
```
<node-type> <filter?>
```

**Verb clause** -- domain-specific command:
```
<domain-verb> <target?> <modifier*>
```

### 2.3 Node Types

Seven node types represent the entities in the code graph:

```
programs      -- COBOL programs (compilation units)
paragraphs    -- named paragraphs/sections within a program
fields        -- data items (WORKING-STORAGE, LINKAGE, FILE SECTION)
copybooks     -- COPY members (shared data definitions)
files         -- file descriptors (FD/SD entries, physical files)
tables        -- SQL tables (EXEC SQL references)
rules         -- extracted business rules (from DSL emitters)
```

### 2.4 Traversal Verbs

Each relationship has a forward (active) and reverse (passive) verb:

| Forward | Reverse | Meaning |
|---|---|---|
| `calling` | `called-by` | Program CALL relationship |
| `performing` | `performed-by` | Paragraph PERFORM relationship |
| `reading` | `read-by` | Paragraph reads a field |
| `writing` | `written-by` | Paragraph writes a field |
| `using` | `used-by` | Program uses a copybook |
| `accessing` | `accessed-by` | Program accesses a file or table |
| `containing` | `within` | Program contains a paragraph |

### 2.5 Domain Verbs

High-level commands that operate on the graph:

| Verb | Purpose | Example |
|---|---|---|
| `trace` | Vertical process slice (Layers 1-7) | `trace CLRG0100 depth full;` |
| `rank` | Sort result set by a metric | `rank programs by complexity top 20;` |
| `similar` | Find structurally similar entities | `similar CLRG0100 by structure;` |
| `find-dead` | Detect unreachable code | `find-dead level paragraph in CLRG0100;` |
| `deps` | Dependency chain / topological sort | `deps CLRG0100 order topological;` |
| `impact` | Change impact analysis | `impact CPYCLRG;` |
| `compare` | Side-by-side comparison | `compare rules in CLRG0100 with CLRG0200;` |
| `save` | Persist a named query | `save my-query as ...;` |
| `run` | Execute a saved query | `run my-query;` |

### 2.6 Modifiers

Keyword arguments for domain verbs (no dashes):

```
by         -- sort/group dimension       rank programs by complexity
top        -- limit result count         rank programs by complexity top 20
in         -- scope to a program         find-dead level paragraph in CLRG0100
with       -- comparison target          compare rules in CLRG0100 with CLRG0200
depth      -- trace depth                trace CLRG0100 depth full
level      -- granularity                find-dead level paragraph
order      -- sort order                 deps CLRG0100 order topological
scope      -- analysis boundary          find-dead level program scope all
threshold  -- similarity cutoff          similar CLRG0100 by structure threshold 0.7
as         -- alias for save             save my-query as ...
```

### 2.7 Filters

Inline predicates use parentheses:

```
programs calling CLRG0100(type = 'batch')
paragraphs writing WS-ACCT-BAL(section = 'PROCEDURE')
programs using each(complexity > 4.0 and type = 'batch')
```

**Operators:**

| Op | Meaning | Example |
|---|---|---|
| `=` | equals | `type = 'batch'` |
| `!=` | not equals | `status != 'dead'` |
| `>` `<` `>=` `<=` | comparison | `complexity > 3.0` |
| `~` | glob match | `name ~ 'CLRG*'` |
| `~~` | regex match | `name ~~ 'CL[A-Z]{2}\d+'` |
| `in` | set membership | `type in ['batch', 'online']` |
| `has` | contains | `copybooks has 'CPYCLRG'` |
| `and` | logical and | `complexity > 3 and type = 'batch'` |
| `or` | logical or | `type = 'batch' or type = 'online'` |
| `not` | negation | `not type = 'dead'` |

### 2.8 Lists

Square brackets for arrays of identifiers:

```
programs calling [CLRG0100, CLRG0200, CLRG0300]
fields [WS-ACCT-NBR, WS-ACCT-BAL, WS-ACCT-TYPE]
type in ['batch', 'online']
```

### 2.9 The `each` Keyword

References the previous result set when the traversal direction needs clarity:

```
copybooks used-by CLRG0100
programs using each(complexity > 4.0);
-- "each" = each copybook from the previous clause
```

When omitted after a bare node type, `each` is implied:

```
programs called-by CLRG0100
copybooks;
-- equivalent to: copybooks used-by each;
```

### 2.10 Comments

```
-- this is a line comment (SQL-style)
```


---

## 3. Grammar (Formal)

```ebnf
script          = statement* ;
statement       = query ';' ;
query           = clause ( NEWLINE clause )* ;
clause          = verb_clause | traverse_clause | expand_clause ;

verb_clause     = DOMAIN_VERB target? modifier* ;
traverse_clause = NODE_TYPE RELATION target filter? ;
expand_clause   = NODE_TYPE filter? ;

target          = IDENTIFIER | 'each' | list ;
list            = '[' IDENTIFIER ( ',' IDENTIFIER )* ']' ;
filter          = '(' predicate ( ('and' | 'or') predicate )* ')' ;
predicate       = 'not'? field_ref OP value ;
field_ref       = IDENTIFIER ( '.' IDENTIFIER )? ;
value           = STRING | NUMBER | list ;
modifier        = MODIFIER_KW ( value | query_body ) ;
query_body      = NEWLINE INDENT clause ( NEWLINE INDENT clause )* ;

DOMAIN_VERB     = 'trace' | 'rank' | 'similar' | 'find-dead' | 'deps'
                | 'impact' | 'compare' | 'save' | 'run' ;
NODE_TYPE       = 'programs' | 'paragraphs' | 'fields' | 'copybooks'
                | 'files' | 'tables' | 'rules' ;
RELATION        = 'calling' | 'called-by' | 'performing' | 'performed-by'
                | 'reading' | 'read-by' | 'writing' | 'written-by'
                | 'using' | 'used-by' | 'accessing' | 'accessed-by'
                | 'containing' | 'within' ;
MODIFIER_KW     = 'by' | 'top' | 'in' | 'with' | 'depth' | 'level'
                | 'order' | 'scope' | 'threshold' | 'as' ;
OP              = '=' | '!=' | '>' | '<' | '>=' | '<='
                | '~' | '~~' | 'in' | 'has' ;

STRING          = "'" [^']* "'" ;
NUMBER          = [0-9]+ ('.' [0-9]+)? ;
IDENTIFIER      = [A-Za-z_][A-Za-z0-9_-]* ;
NEWLINE         = '\n' ;
INDENT          = '  ' ;
COMMENT         = '--' [^\n]* NEWLINE ;
```


---

## 4. Output Format

### 4.1 JSON Tree Output

All query results are JSON. The REPL displays JSON as a tree with indentation
and syntax coloring.

Example: `programs calling CLRG0100;`

```json
{
  "query": "programs calling CLRG0100",
  "count": 0,
  "results": []
}
```

Example: `trace CLRG0100 depth full;`

```json
{
  "query": "trace CLRG0100 depth full",
  "entry": {
    "program": "CLRG0100",
    "type": "batch",
    "loc": 1847,
    "complexity": 4.2
  },
  "execution_tree": {
    "paragraphs": 23,
    "performs": [
      {
        "from": "0000-MAIN",
        "to": "1000-INITIALIZE",
        "type": "perform"
      }
    ],
    "calls": [
      {
        "target": "VALUTIL",
        "parameters": ["WS-ACCT-NBR", "WS-ACCT-BAL"],
        "by": "reference"
      }
    ]
  },
  "data_touched": {
    "files_read": ["ACCTMAST", "TXNFILE"],
    "files_written": ["CLRGOUT", "ERRLOG"],
    "sql_tables": [],
    "copybooks": ["CPYCLRG", "CPYACCT", "CPYTXN"]
  },
  "rules_applied": [
    {
      "paragraph": "2000-VALIDATE",
      "type": "validation",
      "condition": "WS-ACCT-STATUS = 'ACTIVE'"
    }
  ],
  "downstream": ["CLRGRPT", "CLRGEXC"],
  "complexity_score": 4.2,
  "migration_readiness": 3
}
```

### 4.2 Output Modifiers

Future consideration: `format json` (default), `format table`, `format csv`.
For v1, JSON only.


---

## 5. REPL Design

### 5.1 Invocation

```bash
# Start the REPL (interactive)
cobol2rust query

# Execute a single query string
cobol2rust query "rank programs by complexity top 10;"

# Execute a query file
cobol2rust query file queries/clearing-analysis.nxq

# Execute from stdin
echo "rank programs by complexity top 10;" | cobol2rust query
```

File extension: `.nxq` (NexQuery)

### 5.2 REPL Features

```
nxq> rank programs by complexity top 5;
{
  "query": "rank programs by complexity top 5",
  "count": 5,
  "results": [...]
}

nxq> trace CLRG0100 depth full;
{...}

nxq> save clearing as
  .. programs using CPYCLRG
  .. programs calling each
  .. rank by risk;
Saved: clearing

nxq> run clearing;
{...}

nxq> .help          -- REPL commands start with dot
nxq> .history       -- show query history
nxq> .saved         -- list saved queries
nxq> .export last   -- export last result to file
nxq> .explain       -- show query execution plan (for debugging)
nxq> .quit
```

- Dot-prefixed commands (`.help`, `.quit`) are REPL meta-commands
- Multi-line input: lines without `;` continue to next line
- `..` prefix on continuation lines (visual aid, optional)
- Arrow key history, tab completion for node types/verbs/program names
- JSON tree output with syntax highlighting

### 5.3 NexStudio Integration

NexStudio is a thin UI layer. It invokes the same `cobol2rust query` binary
via Tauri's sidecar/shell mechanism:

```
NexStudio (Svelte UI)
  |
  +-- Tauri sidecar: cobol2rust query "<nxq>"
  |
  +-- Parses JSON response
  |
  +-- Renders as tree / table / graph visualization
```

The query engine has zero knowledge of NexStudio. NexStudio has zero knowledge
of graph internals. The contract is: NexQuery string in, JSON out.


---

## 6. Graph Data Model

### 6.1 Node Schema

```rust
enum NodeKind {
    Program,
    Paragraph,
    Field,
    Copybook,
    File,
    Table,
    Rule,
}

struct Node {
    id: NodeId,             // u64, auto-assigned
    kind: NodeKind,
    name: String,           // e.g., "CLRG0100", "WS-ACCT-NBR"
    program: Option<String>,// parent program (for paragraphs, fields)
    properties: Properties, // kind-specific metadata
}
```

**Kind-specific properties:**

| Kind | Properties |
|---|---|
| Program | type (batch/online), loc, complexity, division_count, section_count |
| Paragraph | section, loc, nesting_depth, is_entry_point |
| Field | level, pic, usage, section (WS/LINKAGE/FILE), size_bytes |
| Copybook | member_name, loc, field_count, used_by_count |
| File | organization (seq/indexed/rel), access_mode, record_size |
| Table | table_name, operations (select/insert/update/delete) |
| Rule | type (validation/calculation/decision), condition, paragraph |

### 6.2 Edge Schema

```rust
enum EdgeKind {
    Calls,          // Program -> Program
    Performs,       // Paragraph -> Paragraph
    Reads,          // Paragraph -> Field
    Writes,         // Paragraph -> Field
    Uses,           // Program -> Copybook
    Accesses,       // Program -> File/Table
    Contains,       // Program -> Paragraph
}

struct Edge {
    source: NodeId,
    target: NodeId,
    kind: EdgeKind,
    properties: Properties, // edge-specific metadata
}
```

**Edge properties:**

| Kind | Properties |
|---|---|
| Calls | parameters, call_type (static/dynamic), by_ref/by_content |
| Performs | perform_type (simple/thru/until/varying), thru_target |
| Reads/Writes | (none -- the relationship itself is the data) |
| Uses | (none) |
| Accesses | mode (read/write/both), verb (OPEN/READ/WRITE/REWRITE/DELETE) |
| Contains | (none) |

### 6.3 In-Memory Representation

```rust
use petgraph::stable_graph::StableDiGraph;

struct CodeGraph {
    graph: StableDiGraph<Node, Edge>,
    // Indexes for fast lookup
    by_name: HashMap<(NodeKind, String), NodeId>,
    by_program: HashMap<String, Vec<NodeId>>,
    by_copybook: HashMap<String, Vec<NodeId>>,
}
```

### 6.4 Persistence: Encrypted Binary

**Build phase** (during `cobol2rust scan` or dedicated `cobol2rust build-graph`):

```
COBOL sources --> AST parser --> Graph builder --> Encrypted .nxg file
```

**Storage format:**

```
+-----------------------------------+
| Header (16 bytes)                 |
|   magic: "NXG\0" (4 bytes)       |
|   version: u32                    |
|   flags: u32                      |
|   reserved: u32                   |
+-----------------------------------+
| Salt (32 bytes)                   |
+-----------------------------------+
| IV / Nonce (12 bytes)             |
+-----------------------------------+
| Encrypted payload                 |
|   bincode(CodeGraph)              |
|   encrypted with AES-256-GCM     |
+-----------------------------------+
| Auth tag (16 bytes)               |
+-----------------------------------+
```

File extension: `.nxg` (NexGraph)

**Key derivation:**

```
License key (from NexMig/NexIntel license)
  --> HKDF-SHA256(salt, license_key, "nexquery-graph-v1")
  --> 256-bit AES key
```

The license key is the encryption key. No license = cannot open .nxg files.
Graph files are portable across machines but tied to a license.

**Load phase** (at REPL startup or query execution):

```
.nxg file --> decrypt --> bincode deserialize --> petgraph in memory
```

For the 1500-program client estate (~250K nodes, ~1M edges):
- Serialized size estimate: ~50-100 MB encrypted
- Load time estimate: <2 seconds (bincode is fast)
- Memory footprint: ~200-400 MB (with indexes)


---

## 7. Code Structure

### 7.1 New Crate: `cobol-intel`

```
crates/cobol-intel/
  Cargo.toml
  src/
    lib.rs              -- public API
    error.rs            -- IntelError (thiserror + miette)

    -- Graph data model
    graph/
      mod.rs            -- CodeGraph struct, construction, queries
      node.rs           -- Node, NodeKind, Properties
      edge.rs           -- Edge, EdgeKind
      index.rs          -- secondary indexes (by_name, by_program, etc.)
      builder.rs        -- AST -> Graph construction
      storage.rs        -- .nxg encrypted serialization / deserialization

    -- NexQuery language
    query/
      mod.rs            -- public query API: parse + execute
      lexer.rs          -- tokenizer
      parser.rs         -- recursive descent parser -> NexQueryAst
      ast.rs            -- NexQuery AST types
      compiler.rs       -- AST -> ExecutionPlan (graph traversal steps)
      executor.rs       -- ExecutionPlan -> JSON results
      builtins.rs       -- domain verb implementations (trace, rank, etc.)

    -- Intelligence layers (scoring algorithms)
    intel/
      mod.rs            -- trait IntelligenceLayer
      structural.rs     -- Layer 1: complexity, size, dead code
      control_flow.rs   -- Layer 2: PERFORM graph, loops
      data_flow.rs      -- Layer 3: field tracking, MOVE chains
      inter_program.rs  -- Layer 4: CALL graph, parameter flow
      external.rs       -- Layer 5: file/DB/CICS interface inventory
      business.rs       -- Layer 6: rule extraction, domain entities
      dependency.rs     -- Layer 7: impact analysis, risk scoring
      patterns.rs       -- Layer 8: idiom detection, similarity
      readiness.rs      -- Layer 9: migration scoring

    -- REPL
    repl/
      mod.rs            -- REPL main loop
      completer.rs      -- tab completion (node types, verbs, names)
      history.rs        -- query history persistence
      display.rs        -- JSON tree rendering with syntax coloring
      commands.rs       -- dot-commands (.help, .export, .explain)

    -- Saved queries
    saved/
      mod.rs            -- save/load/list named queries
      store.rs          -- file-backed query store (.nxq files)
```

### 7.2 Dependencies

```toml
[dependencies]
# Graph
petgraph = "0.7"

# Serialization
bincode = "1"
serde = { workspace = true }
serde_json = { workspace = true }

# Encryption
aes-gcm = "0.10"
hkdf = "0.12"
sha2 = { workspace = true }
rand = "0.8"                  # for salt/nonce generation

# Error handling
thiserror = { workspace = true }
miette = { workspace = true }

# REPL
rustyline = "14"              # readline with history + completion

# Internal
cobol-core = { workspace = true }
cobol-transpiler = { workspace = true }  # AST types for graph building
```

### 7.3 CLI Integration

New subcommands in `cobol-cli`:

```
cobol2rust build-graph   -- scan COBOL sources, build .nxg file
cobol2rust query         -- start REPL or execute query
cobol2rust query "..."   -- execute inline query
cobol2rust query file X  -- execute query file
```

```
crates/cobol-cli/src/
  build_graph_cmd.rs    -- build-graph subcommand
  query_cmd.rs          -- query subcommand (delegates to cobol-intel)
```

### 7.4 NexStudio Integration

NexStudio calls the CLI as a sidecar process:

```
[NexStudio Svelte UI]
    |
    | (Tauri sidecar)
    v
[cobol2rust query "<nxq-string>"]
    |
    | (stdout)
    v
[JSON response] --> Svelte renders tree / visualization
```

No Rust code in NexStudio talks to cobol-intel directly.
The contract is NexQuery string in, JSON out, via CLI.

### 7.5 Dependency Graph (Crate Level)

```
cobol-cli
  |
  +-- cobol-intel
  |     |
  |     +-- cobol-transpiler  (AST types for graph building)
  |     +-- cobol-core        (traits, config, errors)
  |     +-- petgraph          (graph engine)
  |     +-- aes-gcm           (encryption)
  |     +-- rustyline          (REPL)
  |
  +-- (existing crates unchanged)
```

cobol-intel depends on cobol-transpiler for AST types but NOT on any runtime
crates. It reads AST, builds graph, answers queries. It never executes COBOL.


---

## 8. IP Protection

### 8.1 Strategy

| Asset | Protection | Mechanism |
|---|---|---|
| NexQuery syntax | Public (docs, tutorials) | Users need to learn it |
| Query results (JSON) | Client's data, exportable | No restrictions |
| Graph data (.nxg files) | Encrypted at rest | AES-256-GCM, license-keyed |
| Query engine source | Trade secret | Compiled binary distribution |
| Intelligence algorithms | Trade secret | Compiled into binary |
| Advanced features | License-gated | License key required |

### 8.2 Encryption Details

- Algorithm: AES-256-GCM (authenticated encryption)
- Key derivation: HKDF-SHA256 from license key + random salt
- Salt: 32 bytes, generated per .nxg file, stored in header
- Nonce: 12 bytes, generated per encryption, stored in header
- Auth tag: 16 bytes, appended (GCM provides this)
- No license key = cannot decrypt = cannot query

### 8.3 License Tiers (Future)

```
Tier 0 (Free / OSS):
  - Graph building from COBOL AST
  - Basic traversals (calling, used-by, etc.)
  - NDJSON export (unencrypted)

Tier 1 (NexIntel Standard):
  - Encrypted .nxg storage
  - All traversal verbs + filters
  - rank, find-dead, deps
  - REPL with history
  - Saved queries

Tier 2 (NexIntel Enterprise):
  - trace (full process vertical slice)
  - similar (pattern matching)
  - compare (rule comparison)
  - impact (change analysis)
  - Discovery mode (auto-clustering)
  - NexStudio visualization
  - Watermarked exports
```


---

## 9. Implementation Plan

### Phase 1: Foundation (2-3 sessions)

```
Session CI-1: Graph data model + builder
  - Node/Edge types, CodeGraph struct
  - AST -> Graph builder (from cobol-transpiler output)
  - .nxg encrypted serialization
  - build-graph CLI subcommand
  - Tests: build graph from sample COBOL, serialize/deserialize

Session CI-2: NexQuery parser
  - Lexer (tokenizer)
  - Recursive descent parser -> AST
  - AST types for all clause types
  - Tests: parse all 8 patterns + composed queries

Session CI-3: Query executor + basic REPL
  - AST -> ExecutionPlan compiler
  - Executor: traverse_clause, expand_clause, filters
  - Basic domain verbs: rank, find-dead
  - REPL with rustyline (history, multi-line)
  - query CLI subcommand
  - Tests: execute queries against sample graph
```

### Phase 2: Intelligence Layers (3-4 sessions)

```
Session CI-4: Structural + Control Flow (Layers 1-2)
  - Complexity scoring, dead code detection
  - PERFORM graph analysis, loop detection

Session CI-5: Data Flow + Inter-Program (Layers 3-4)
  - Field read/write tracking per paragraph
  - CALL graph with parameter flow

Session CI-6: External + Business Logic (Layers 5-6)
  - File/DB/CICS interface inventory
  - Rule extraction integration (from DSL emitters)

Session CI-7: Dependency + Pattern (Layers 7-8)
  - Impact analysis, risk scoring
  - Similarity search, idiom detection
```

### Phase 3: Advanced Features (2-3 sessions)

```
Session CI-8: trace verb (vertical process slice)
  - Full process trace combining all layers
  - Auto-discovery algorithm (entry points, clustering)

Session CI-9: REPL polish + saved queries
  - Tab completion (program names, node types, verbs)
  - .explain command (show execution plan)
  - Named query save/load/list
  - JSON tree display with syntax coloring

Session CI-10: NexStudio integration
  - Sidecar invocation from Tauri
  - Svelte components for query results
  - Graph visualization (optional, if time permits)
```

### Total: ~10 sessions


---

## 10. Example Session (End-to-End)

```bash
# Step 1: Build the graph from COBOL sources
$ cobol2rust build-graph --input /cobol/src --output clearing.nxg --license KEY123
Building graph... 1,500 programs scanned
  247,382 nodes, 1,023,847 edges
  Encrypted: clearing.nxg (87 MB)

# Step 2: Start the REPL
$ cobol2rust query --graph clearing.nxg --license KEY123
Loading graph... 1.3s
NexQuery REPL v0.1.0 (247,382 nodes, 1,023,847 edges)
Type .help for commands.

nxq> rank programs by complexity top 5;
{
  "query": "rank programs by complexity top 5",
  "count": 5,
  "results": [
    { "program": "CLRG0100", "complexity": 8.7, "loc": 12847 },
    { "program": "ACMT0500", "complexity": 7.2, "loc": 9234 },
    { "program": "LNOR0100", "complexity": 6.8, "loc": 8912 },
    { "program": "RPTG0200", "complexity": 6.1, "loc": 7456 },
    { "program": "VALUTIL",  "complexity": 5.9, "loc": 6203 }
  ]
}

nxq> trace CLRG0100 depth full;
{
  "query": "trace CLRG0100 depth full",
  "entry": {
    "program": "CLRG0100",
    "type": "batch",
    "loc": 12847
  },
  "execution_tree": {
    "paragraphs": 47,
    "calls": [
      { "target": "VALUTIL", "parameters": ["WS-ACCT-NBR"] },
      { "target": "BALUPD",  "parameters": ["WS-ACCT-BAL", "WS-TXN-AMT"] },
      { "target": "ERRHAND", "parameters": ["WS-ERR-CODE"] }
    ]
  },
  "data_touched": {
    "files_read": ["ACCTMAST", "TXNFILE"],
    "files_written": ["CLRGOUT", "ERRLOG"],
    "copybooks": ["CPYCLRG", "CPYACCT", "CPYTXN"]
  },
  "rules_applied": 12,
  "downstream": ["CLRGRPT", "CLRGEXC"],
  "complexity_score": 8.7
}

nxq> copybooks used-by CLRG0100
  .. programs using each(name !~ 'CLRG*');
{
  "query": "copybooks used-by CLRG0100 | programs using each ...",
  "count": 14,
  "results": [
    { "copybook": "CPYACCT", "programs": ["ACMT0100", "ACMT0200", "LNOR0100"] },
    { "copybook": "CPYTXN",  "programs": ["TXNP0100", "TXNP0200"] },
    { "copybook": "CPYCLRG", "programs": [] }
  ]
}

nxq> save clearing-scope as
  .. programs called-by CLRG0100
  .. programs calling each;
Saved: clearing-scope

nxq> .quit
```


---

## 11. Open Items

1. **Error messages**: How verbose? Show query position, expected tokens?
2. **Pagination**: For large result sets (>1000 items), auto-paginate or require `top`?
3. **Explain mode**: Show graph traversal plan before executing?
4. **Incremental graph updates**: Rebuild entire .nxg or patch it?
5. **Graph versioning**: Track which COBOL source version produced the graph?
6. **Multi-graph queries**: Query across multiple .nxg files?
