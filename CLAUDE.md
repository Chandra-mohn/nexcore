# NexCore -- Workspace Guide

Unified Rust backend engine for the Nex Platform.
Nexflow DSL compiler, COBOL transpiler, and intelligence engine in one Cargo workspace.

## Session Naming

Sessions use NC-## prefix (NC-00 = brainstorm, NC-01 = workspace setup, etc.)

## Architecture

```
grammar/
    nexflow/    -- OUR grammars: .schema, .rules, .xform, .proc, .api, .service
    mainframe/  -- SOURCE grammars: COBOL85, CICS, JCL, SQL, BMS (from antlr-grammars)
crates/
    nexflow-parser/     -- ANTLR4 parsers + typed ASTs for all 7 DSL grammars
    nexflow-compiler/   -- Type checker, cross-grammar reference resolver
    nexflow-codegen/    -- Code generation: Java/Spring/Flink/Kafka/GraphQL/gRPC/OpenAPI
    nexflow-lsp/        -- LSP server (tower-lsp) for DSL editing
    nexcore-cli/        -- Unified CLI (feature-flag selected products)
    cobol-*/            -- COBOL transpiler crates (migrate from cobol2rust later)
    nexintel/           -- Code/data intelligence (migrate later)
```

## Three Products, One Workspace

Products packaged via Cargo feature flags:
- **NexMig**: COBOL transpiler, rustifier, DSL emitter (migration)
- **NexMod**: Nexflow parser, compiler, codegen, LSP (modernization)
- **NexIntel**: Data/code intelligence, lineage (analysis)

Build individual products: `cargo build -p nexcore-cli --features nexmod`

## Rust Guidelines

Follow Microsoft Pragmatic Rust Guidelines for all Rust code.
Reference: .claude/microsoft_rust_guidelines.txt (local copy)

## Parser Technology

- ANTLR4 (.g4) for ALL grammars. antlr-rust as Rust runtime.
- No pest, tree-sitter, nom, or other parser frameworks.
- .g4 files are the source of truth. Rust parsers are generated.
- cobol2rust proves antlr-rust works (COBOL grammar, 50+ sessions).

## 7-Grammar DSL Suite (3 Layers)

```
Layer 1 -- Building Blocks:  .schema  .rules  .xform
Layer 2 -- Contracts:        .api     .screen(future)
Layer 3 -- Orchestration:    .service .proc
```

- .api = service contracts (endpoints, events, auth, versioning, dependencies)
- .service = microservice orchestration (handlers, middleware, transactions)
- .api <-> .service is M:N (implements/consumes)
- No inline schema in .api -- always reference .schema
- GraphQL is a compilation target for .api, not a separate grammar

## Key Rules

- Crate boundaries = product boundaries
- No cross-product hard dependencies (NexMod works without NexMig)
- Bridge crates are optional and feature-flagged
- Grammars canonical in grammar/ directory
- Stress tests: run one at a time, never bulk-build

## Key Docs

- docs/nexcore_architecture_spec.md -- Full workspace and product packaging design
- docs/nexflow_api_service_dsl_spec.md -- .api and .service grammar design (7 grammars, 3 layers)

## Build Notes

- Rust 1.85+, edition 2024
- antlr-rust 0.3.0-beta for ANTLR4 parser generation
- RAM disk recommended: /Volumes/RustBuild (macOS) for target dir
- Pedantic clippy; see workspace [lints] in Cargo.toml

## Conventions

- **Error handling**: thiserror + miette for all error types
- **Testing**: Inline #[test] blocks in source files
- **Decimals**: rust_decimal::Decimal everywhere
- **Serialization**: serde/serde_json for config and reporting
- **Lints**: Pedantic clippy
