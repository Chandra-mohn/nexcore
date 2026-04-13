# cobol-intel -- Code Intelligence Engine

NexQuery graph model, query language, executor, REPL, and intelligence layers.

## Architecture

```
cobol-intel/src/
  lib.rs              Public API: parse(), execute()
  error.rs            IntelError (thiserror + miette)
  graph/
    mod.rs            CodeGraph (petgraph + indexes), GraphSnapshot (serde)
    node.rs           Node, NodeKind (7 types), Properties, PropValue
    edge.rs           Edge, EdgeKind (7 types), NexQuery verb mapping
    index.rs          GraphIndex (secondary indexes by name, program, copybook, file)
    builder.rs        GraphBuilder: AST -> CodeGraph
    storage.rs        Encrypted .nxg files (AES-256-GCM, HKDF-SHA256)
  query/
    mod.rs            Public parse() + execute() API
    ast.rs            NexQuery AST (Script, Statement, Clause, Filter, etc.)
    lexer.rs          Tokenizer (keywords, operators, identifiers, literals)
    parser.rs         Recursive descent parser -> NexQuery AST
    executor.rs       AST -> JSON results (pipeline execution against CodeGraph)
  repl/
    mod.rs            rustyline REPL: multi-line input, .help/.stats/.quit, file execution
  intel/
    mod.rs            IntelligencePass trait, run_all() orchestrator
    structural.rs     Layer 1: complexity, field/para/call counts, program classification
    control_flow.rs   Layer 2: entry points, dead paragraphs, call depth
    data_flow.rs      Layer 3: field access mode/role, data coupling score
    inter_program.rs  Layer 4: copybook coupling, naming prefix clusters
    external.rs       Layer 5: file/table inventory, interface complexity, SQL ops
    business.rs       Layer 6: rule counts, business density, has_rules flags
    dependency.rs     Layer 7: impact radius, risk score, migration waves
    patterns.rs       Layer 8: structural fingerprints, pattern/era classification, similarity
    process_discovery.rs  Layer 9: business process discovery, union-find clustering
    data_complexity.rs    Layer 10: data complexity impact, param/rule detection, classification
    cost/
      mod.rs              Layer 11: migration cost estimation, multipliers, aggregation
      expr.rs             Arithmetic expression evaluator for user-modelable formulas
      config.rs           CostConfig: day rates, productivity, Nex reduction, formula defs
  saved/
    mod.rs            QueryStore: save/load/list/delete named .nxq queries
  saved/              Named query persistence (CI-9)
```

## Key Types

- `CodeGraph` -- in-memory graph (petgraph StableDiGraph + GraphIndex)
- `GraphSnapshot` -- portable serialization format (nodes vec + edges vec)
- `GraphBuilder` -- consumes cobol-transpiler AST, produces CodeGraph
- `Node` / `Edge` -- graph entities with kind + name + properties
- `NodeKind` -- Program, Paragraph, Field, Copybook, File, Table, Rule
- `EdgeKind` -- Calls, Performs, Reads, Writes, Uses, Accesses, Contains

## Query Execution Model

Each NexQuery statement is a pipeline of clauses:
- **Traverse**: `programs calling CLRG0100` -- resolve anchor, traverse edges, filter
- **Expand**: `fields` -- for each node in result, find related nodes of target kind
- **Verb**: `rank by complexity top 20` -- domain-specific operation producing JSON

Pipeline: clause 1 produces result_set -> clause 2 operates on it -> ... -> JSON output.

Domain verbs implemented: rank, find-dead, trace, deps, impact, similar, discover-processes, estimate-cost.
Stub: compare.

## CLI Integration

Two new subcommands in cobol-cli:

```bash
cobol2rust build-graph -i /cobol/src -o graph.nxg -l LICENSE_KEY
cobol2rust query -g graph.nxg -l LICENSE_KEY              # REPL
cobol2rust query -g graph.nxg -l LICENSE_KEY -e "..."     # inline
cobol2rust query -g graph.nxg -l LICENSE_KEY -f file.nxq  # file
```

## REPL Usage

```rust
use cobol_intel::repl;
repl::run(&graph)?;                          // interactive
repl::execute_and_print(&graph, "...");      // single query
repl::execute_file(&graph, Path::new("f"));  // .nxq file
```

## Tests

193 inline tests covering:
- Node/edge creation, property access, kind display
- Index lookup (by kind+name, by program, by copybook, by file)
- Graph traversal (all 7 forward + 7 reverse verbs), multi-hop
- Snapshot roundtrip, encrypted storage, wrong-key rejection
- Builder: empty program, copybook dedup, SQL table extraction
- Lexer: all token types, comments, continuation dots, operators
- Parser: all 8 patterns, composed queries, filters, save/run, real-world session
- Executor: traverse, pipeline, expand, rank, find-dead, trace, deps, impact, filters
- Structural pass: complexity scoring, field/para counts, program classification
- Control flow pass: entry points, dead paragraphs, call depth, run_all orchestrator
- Data flow pass: field access counts/mode/role, paragraph read/write counts, data coupling
- Inter-program pass: copybook sharing, coupling partners, naming prefix, cluster assignment
- External pass: file/table inventory, accessor counts, SQL operations, interface complexity
- Business pass: paragraph/program rule counts, business density, has_rules flags
- Dependency pass: impact radius, risk scoring, migration wave assignment, circular deps
- Pattern pass: fingerprints, similarity detection, pattern/era classification, bucketing
- Process discovery: clustering, naming, role classification, discover-processes verb
- Data complexity: param/rule table detection, field patterns, copybook complexity, classification, composite scoring
- Cost estimation: expression evaluator, config serde, multipliers, manual/nex effort, aggregation by scope
- Domain verbs implemented: rank, find-dead, trace, deps, impact, similar, discover-processes, estimate-cost.
- Saved queries: save/load/delete, persistence across opens, overwrite

## Dependencies

- petgraph: in-memory directed graph
- aes-gcm + hkdf + sha2: encrypted storage
- bincode: binary serialization
- rustyline: REPL (readline, history, multi-line)
- cobol-transpiler: AST types for graph building
- cobol-core: traits and config

## Spec

Full NexQuery language spec: docs/nexquery_spec.md
