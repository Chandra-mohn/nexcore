# NexCore -- Workspace Guide

Unified Rust backend engine for the Nex Platform.
Nexflow DSL compiler, COBOL transpiler, and intelligence engine in one Cargo workspace.
18 crates. Rust 1.85+, edition 2024.

## Session Naming

Sessions use NC-## prefix (NC-00 = brainstorm, NC-01 = workspace setup, etc.)

## Architecture

```
grammar/
    nexflow/    -- Nexflow grammars: .schema, .rules, .xform, .proc, .api, .service, .screen
    cobol/      -- Source grammars: Cobol85, Cobol85Preprocessor, BmsDSL
crates/
    # NexMod (Nexflow DSL toolchain)
    nexflow-parser/     -- ANTLR4 parsers + typed ASTs for 7 DSL grammars
    nexflow-compiler/   -- Type checker, cross-grammar reference resolver
    nexflow-codegen/    -- Code generation: Java/Avro/Flink + Rust/Axum
    nexflow-cli/        -- Nexflow CLI (parse, validate, build, generate)

    # NexMig (COBOL transpiler + rustification)
    cobol-core/         -- Foundation: traits, config, EBCDIC, errors
    cobol-types/        -- COBOL data types (PIC clause implementations)
    cobol-move/         -- MOVE engine (coercion, truncation, CORRESPONDING)
    cobol-arithmetic/   -- Arithmetic verbs (ADD, SUBTRACT, MULTIPLY, DIVIDE, COMPUTE)
    cobol-io/           -- File I/O (sequential, indexed via SQLite, relative)
    cobol-sort/         -- SORT/MERGE (adaptive in-memory/external)
    cobol-sql/          -- EXEC SQL runtime (trait-based, DuckDB backend)
    cobol-runtime/      -- Program lifecycle, prelude, special registers
    cobol-macros/       -- No-op proc-macro for #[cobol()] attributes
    cobol-transpiler/   -- ANTLR4 parser, AST, symbol table, Rust codegen
    cobol-rustify/      -- Tier 1-4 rustification, DSL emitters (E1-E4)

    # NexIntel (code + data intelligence)
    cobol-data/         -- Binary data intelligence, layout, decode
    cobol-intel/        -- Code intelligence, graph model, NexQuery

    # CLI
    nexmig-cli/         -- NexMig CLI (transpile, scan, rustify, audit, etc.)
```

## Three Products, One Workspace

Products packaged via Cargo feature flags:
- **NexMig**: COBOL transpiler, rustifier, DSL emitter (migration)
- **NexMod**: Nexflow parser, compiler, codegen (modernization)
- **NexIntel**: Data/code intelligence, lineage (analysis)

## Rust Guidelines

Follow Microsoft Pragmatic Rust Guidelines for all Rust code.
Reference: .claude/microsoft_rust_guidelines.txt (local copy)

## Parser Technology

- ANTLR4 (.g4) for ALL grammars. antlr-rust as Rust runtime.
- No pest, tree-sitter, nom, or other parser frameworks.
- .g4 files are the source of truth. Rust parsers are generated.

## 7-Grammar DSL Suite (3 Layers)

```
Layer 1 -- Building Blocks:  .schema  .rules  .xform
Layer 2 -- Contracts:        .api     .screen
Layer 3 -- Orchestration:    .service .proc
```

Cross-grammar reference matrix:
- .api -> .schema (request/response types), .rules (validation)
- .service -> .api (implements), .schema, .rules, .xform
- .proc -> .schema, .rules, .xform, .api
- .screen -> .schema (data display), .api (endpoint invocation)

## Conventions

- **Error handling**: thiserror + miette for all error types
- **Testing**: Inline #[test] blocks in source files (2000+ tests total)
- **Decimals**: rust_decimal::Decimal everywhere; use dec!() in tests
- **Serialization**: serde/serde_json for config and reporting
- **Codegen output**: #[cobol(...)] attributes on generated functions/structs
- **Lints**: Pedantic clippy; see workspace [lints] in Cargo.toml
- **Parallelism**: rayon for multi-threaded transpilation/rustification

## Key Rules

- Crate boundaries = product boundaries
- No cross-product hard dependencies (NexMod works without NexMig)
- Bridge crates are optional and feature-flagged
- Grammars canonical in grammar/ directory
- Generated code calls runtime crate functions (cobol_move, cobol_add, etc.)
- No dec!() in generated code; use `.parse::<Decimal>().unwrap()`
- DSL emitters read #[cobol(...)] attributes from Rust source
- Stress tests: run one at a time, NEVER bulk-build all 47

## Build Notes

- Rust 1.85+, edition 2024
- RAM disk recommended: /Volumes/RustBuild (macOS) for target dir
- DuckDB bundled C++ uses ~4GB RAM during compilation
- Feature flags: `duckdb` (scan command), `direct-emit` (DSL from COBOL AST)

## Key Docs

- docs/nexcore_architecture_spec.md -- Full workspace and product packaging design
- docs/nexflow_api_service_dsl_spec.md -- .api and .service grammar design
- docs/nex_platform_strategy.md -- Product strategy (NexMig, NexMod, NexIntel, NexStudio)
- docs/cobol2java_plan.md -- Java codegen plan (8 sessions)
- docs/cobol2rust_rustify_spec.md -- Rustification pipeline spec
- docs/cobol2rust_typed_dsl_emitter_spec.md -- DSL emitter spec
- docs/emitter_capabilities.md -- DSL emitter coverage tracking
- docs/artifact_architecture.md -- Graph + AST Store + DSL layers
