// Nexflow DSL Toolchain
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
// Author: Chandra Mohn
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

# NexCore Architecture Specification

**Status:** DECIDED
**Date:** 2026-03-30
**Author:** Chandra Mohn + Claude (AI-assisted design)

---

## 1. Overview

NexCore is the unified Rust backend engine for the Nex Platform. It consolidates
all parsing, compilation, code generation, and intelligence capabilities into a
single Cargo workspace, replacing the current split between cobol2rust (Rust) and
nexflow-toolchain (Python).

---

## 2. Repository Architecture

### 2.1 Two Repos, Three Products

```
nexcore/       = All backend engines (Rust, single Cargo workspace)
nexstudio/     = All GUI (Tauri v2 + Svelte 5, desktop IDE)
nexflow/       = Legacy Python + AI agents (gradually retired)
```

### 2.2 Nexflow DSL Suite -- 3-Layer / 7-Grammar Model

```
Layer 1 -- Building Blocks (define WHAT)
+----------+   +----------+   +----------+
|  .schema |   |  .rules  |   |  .xform  |
+----------+   +----------+   +----------+
      \              |              /
       \             |             /
        +------------+------------+
                     |
Layer 2 -- Contracts (define the INTERFACE)
         +-----------+-----------+
         |                       |
    +--------+             +-----------+
    |  .api  |             |  .screen  |
    +--------+             +-----------+
         |                       |
Layer 3 -- Orchestration (define HOW things wire together)
         |                       |
    +----------+           +----------+
    | .service |           |  .proc   |
    +----------+           +----------+
```

Cross-grammar reference matrix (all verified by the Nexflow compiler):

```
              ---- references ---->
              .schema  .rules  .xform  .api
  .api           x       x                     contract -> building blocks
  .service       x       x       x       x     orchestration -> all
  .proc          x       x       x       x     orchestration -> all
  .screen        x                       x     contract -> schema + api
```

Reading the rows: ".service references .api, .schema, .rules, and .xform"
Reading the columns: ".schema is referenced by .api, .service, .proc, and .screen"

Key relationships:
- .api -> .schema for request/response/error types
- .api -> .rules for endpoint validation (e.g., `validate using X.rules`)
- .service -> .api via `implements` (inbound) and `consumes` (outbound)
- .proc -> .api for service calls and event subscriptions
- .screen -> .api for endpoint invocation, .schema for data display

Event flow (the async bridge between orchestration grammars):

```
.api declares event --> .service publishes event --> .proc subscribes to event
```

### 2.3 Product-to-Repo Mapping

```
                        nexcore (repo)                    nexstudio (repo)
                   +-----------------------+         +--------------------+
  NexMig           |  cobol-transpiler     |         |  Migration UI      |
  (product)        |  cobol-rustify        |         |  Pipeline ribbon   |
                   |  cobol-runtime/*      |         |  Folder grid       |
                   |  dsl-emitter          |         |                    |
                   +-----------------------+         +--------------------+
  NexMod           |  nexflow-parser       |         |  DSL editors       |
  (product)        |  nexflow-compiler     |         |  Visual designer   |
                   |  nexflow-codegen      |         |  Kafka/Flink tools |
                   |  nexflow-lsp          |         |                    |
                   +-----------------------+         +--------------------+
  NexIntel         |  cobol-data           |         |  Query editor      |
  (product)        |  nexintel             |         |  Data viewer       |
                   |  lineage              |         |  Dependency graphs |
                   +-----------------------+         +--------------------+
```

Every product draws from BOTH repos: nexcore for backend, nexstudio for frontend.

---

## 3. NexCore Workspace Structure

### 3.1 Current Directory Layout (Phase 0 -- in progress)

```
~/workspace/nexcore/
    Cargo.toml                              # Workspace root (resolver = "2")
    CLAUDE.md                               # Project-level instructions
    grammar/
        nexflow/                            # Nexflow DSL grammars (canonical)
            SchemaDSL.g4                    # HAVE (ported from Python)
            RulesDSL.g4                     # HAVE (ported from Python)
            TransformDSL.g4                 # HAVE (ported from Python)
            ProcDSL.g4                      # HAVE (ported from Python)
            ApiDSL.g4                       # DRAFTING (in cobol2rust/grammar/nexflow/)
            ServiceDSL.g4                   # DRAFTING (in cobol2rust/grammar/nexflow/)
    crates/
        nexflow-parser/                     # HAVE (scaffold, no generated parsers yet)
            src/lib.rs
            Cargo.toml
    docs/
        nexcore_architecture_spec.md        # This file
        nexflow_api_service_dsl_spec.md     # ApiDSL + ServiceDSL grammar design
```

### 3.2 Target Directory Layout (full build-out)

```
~/workspace/nexcore/
    Cargo.toml                              # Workspace root
    grammar/
        nexflow/                            # Nexflow DSL grammars (canonical)
            SchemaDSL.g4
            RulesDSL.g4
            TransformDSL.g4
            ProcDSL.g4
            ApiDSL.g4
            ServiceDSL.g4
        cobol/                              # COBOL grammars (migrate later)
            COBOL85.g4
            CICS.g4                         # Future
            JCL.g4                          # Future
        sql/
            SQL.g4                          # Future
    crates/
        # --- Foundation (always included) ---
        nexcore-core/                       # Shared traits, config, errors
        nexcore-types/                      # Shared type system

        # --- NexMig product crates (migrate from cobol2rust) ---
        cobol-parser/                       # COBOL ANTLR4 parser
        cobol-transpiler/                   # COBOL -> Rust/Java codegen
        cobol-rustify/                      # Rustification engine
        cobol-move/                         # MOVE semantics
        cobol-arithmetic/                   # Arithmetic verbs
        cobol-io/                           # File I/O
        cobol-sort/                         # SORT/MERGE
        cobol-sql/                          # EXEC SQL (DuckDB)
        cobol-runtime/                      # Program lifecycle, prelude

        # --- NexMod product crates ---
        nexflow-parser/                     # DSL parsers (all 7 grammars)
        nexflow-compiler/                   # Type checker, semantic analysis
        nexflow-codegen/                    # Code generation backends
        nexflow-lsp/                        # LSP server (tower-lsp)

        # --- NexIntel product crates (migrate from cobol2rust) ---
        cobol-data/                         # Data intelligence
        nexintel/                           # Code intelligence
        lineage/                            # Lineage engine

        # --- Bridge (NexMig + NexMod) ---
        dsl-emitter/                        # COBOL AST -> Nexflow DSL

        # --- CLI ---
        nexcore-cli/                        # Unified CLI (feature-flag selected)
    examples/                               # Example DSL files
        nexflow/
            api/                            # .api examples (in cobol2rust/examples/)
            service/                        # .service examples (in cobol2rust/examples/)
    docs/                                   # Architecture + grammar docs
```

### 3.3 Crate Dependency Graph

```
                nexcore-core
                nexcore-types
               /      |       \
          NexMig    NexMod    NexIntel
          crates    crates    crates
            \         |         /
             \   dsl-emitter  /      (bridge, optional)
              \       |      /
               nexcore-cli
          (feature flags select which products)
```

---

## 4. Product Packaging with Cargo Feature Flags

### 4.1 CLI Feature Flags

```toml
# nexcore/crates/nexcore-cli/Cargo.toml
[features]
default = ["full"]
full = ["nexmig", "nexmod", "nexintel", "bridge"]

# Product feature flags
nexmig = [
    "dep:cobol-parser",
    "dep:cobol-transpiler",
    "dep:cobol-rustify",
    "dep:cobol-runtime",
]
nexmod = [
    "dep:nexflow-parser",
    "dep:nexflow-compiler",
    "dep:nexflow-codegen",
]
nexintel = [
    "dep:cobol-data",
    "dep:nexintel",
    "dep:lineage",
]
bridge = ["nexmig", "nexmod", "dep:dsl-emitter"]
```

### 4.2 Build Targets

```bash
# Full platform (internal dev, NexStudio integration)
cargo build --features full                 # -> nexcore (everything)

# Individual products (customer deployment)
cargo build --features nexmig               # -> NexMig only
cargo build --features nexmod               # -> NexMod only (greenfield customers)
cargo build --features nexintel             # -> NexIntel only

# Common combinations
cargo build --features nexmig,nexmod,bridge # -> Migration + Modernization
cargo build --features nexmod,nexintel      # -> Modernization + Intelligence
```

### 4.3 NexStudio Feature Flags (mirror)

```toml
# nexstudio/src-tauri/Cargo.toml
[features]
default = ["full"]
full = ["nexmig-ui", "nexmod-ui", "nexintel-ui"]
nexmig-ui = ["dep:cobol-transpiler", "dep:dsl-emitter"]
nexmod-ui = ["dep:nexflow-parser", "dep:nexflow-compiler", "dep:nexflow-codegen"]
nexintel-ui = ["dep:cobol-data", "dep:nexintel"]
```

### 4.4 Deployment Artifacts

| Product | CLI Binary | NexStudio Module | Target Customer |
|---|---|---|---|
| NexMig | `nexmig` | Migration tab | COBOL modernization projects |
| NexMod | `nexmod` | DSL editor + visual designer | Greenfield + modernization |
| NexIntel | `nexintel` | Query editor + data viewer | Analysis + planning |
| NexCore | `nexcore` | Full desktop app | Internal + enterprise |

---

## 5. Implementation Rules

### 5.1 Crate Boundary = Product Boundary

Every crate belongs to exactly one product. Shared infrastructure goes in
foundation crates (nexcore-core, nexcore-types).

### 5.2 No Cross-Product Hard Dependencies

NexMod compiles and runs WITHOUT NexMig crates (greenfield customers).
NexIntel works WITHOUT NexMod (analysis-only customers).
Cross-product functionality lives in optional bridge crates.

### 5.3 Bridge Crate Pattern

The dsl-emitter crate bridges NexMig and NexMod. It depends on both product
layers and is only included when both features are enabled.

---

## 6. Parser Technology Strategy

### 6.1 ANTLR4 Everywhere

```
+------------------------------------------------------------------+
|                    ANTLR4 (.g4 grammars)                         |
|  Single source of truth for ALL structured language parsing      |
+------------------------------------------------------------------+
       |                              |
       v                              v
  antlr-rust                    antlr4-python3-runtime
  (permanent target)            (legacy, being retired)
       |                              |
  nexcore                       nexflow-toolchain (legacy)
  nexstudio (linked)
```

### 6.2 Rules

1. ALL grammars for structured languages use ANTLR4 .g4 format
2. The .g4 file is the source of truth -- runtime targets are generated
3. Hand-crafted parsers are exceptions, only for small leaf-node DSLs (NexQuery)
4. No new parser frameworks (pest, tree-sitter, nom) for grammar-level parsing
5. NexStudio syntax highlighting uses TextMate grammars (derived from .g4)

### 6.3 Rationale

ANTLR4 provides free, battle-tested grammars for COBOL, CICS, JCL, SQL, Java,
and other languages needed for modernization. Rewriting these in any other
framework would be massive effort with zero business value. cobol2rust has
proven antlr-rust works for the most complex grammar (COBOL).

---

## 7. Migration Strategy

### 7.1 Phase Plan

| Phase | Scope | What Happens |
|---|---|---|
| 0 | Create nexcore workspace | New Rust workspace, ApiDSL.g4, ServiceDSL.g4 |
| 1 | nexflow-parser crate | ANTLR4 parsers + typed ASTs for .api and .service |
| 2 | nexflow-compiler crate | Type checker, cross-grammar reference resolver |
| 3 | nexflow-codegen crate | Code generation backends (Java/Spring/Flink/etc.) |
| 4 | nexflow-lsp crate | LSP server (tower-lsp) for DSL editing |
| 5 | Port existing DSL grammars | SchemaDSL, RulesDSL, TransformDSL, ProcDSL to antlr-rust |
| 6 | Migrate cobol2rust crates | Move cobol-* crates into nexcore workspace |
| 7 | Migrate intelligence crates | Move cobol-data, nexintel, lineage into nexcore |
| 8 | NexStudio integration | Link nexcore crates directly into Tauri backend |
| 9 | Retire Python toolchain | Archive nexflow-toolchain, VS Code plugin uses Rust LSP |

### 7.2 Phase 0 Progress

| Step | Description | Status |
|---|---|---|
| 1 | Create nexcore workspace | DONE -- ~/workspace/nexcore/ |
| 2 | Initialize Cargo workspace with workspace-level dependencies | DONE -- Cargo.toml, workspace lints |
| 3 | Copy existing .g4 grammars to nexcore/grammar/nexflow/ | DONE -- Schema, Rules, Transform, Proc |
| 4 | Draft ApiDSL.g4 and ServiceDSL.g4 | DONE -- validated with ANTLR4, all examples parse clean |
| 5 | Write .api and .service example files | DONE -- 3+3 in nexcore/examples/nexflow/ |
| 6 | Create nexflow-parser crate scaffold | DONE -- crate exists, no generated parsers yet |
| 7 | Set up antlr-rust build pipeline | PENDING (NC-02) |
| 8 | Build typed AST structs for ApiDSL and ServiceDSL | PENDING (NC-02) |
| 9 | Write parser integration tests | PENDING (NC-02) |

Grammar design decisions made during NC-01 (2026-03-30):
- First-principles analysis: 7 design principles (P1-P7) established
- Qualified annotation pattern: namespace.value (cfg.X, pii.X, audit.X, sec.X)
- cfg.X = config indirection, resolved from config block or external file
- Config block replaces middleware in .service (generic key-value, not hardcoded)
- Structural keywords only; config values parsed as IDENTIFIER (compiler validates)
- P6 enforced strictly: all computation in .xform, .service is pure orchestration
- 39 keywords (ApiDSL) + 37 keywords (ServiceDSL) = 76 total (down from 100)
- `word` rule in both grammars allows keywords in import paths and dotted expressions
- Validated: antlr4 compiles both .g4 clean, antlr4-parse succeeds on all 6 examples

---

## 8. Nexflow DSL Suite (Reference)

See docs/nexflow_api_service_dsl_spec.md for the full 7-grammar design:

| Layer | Grammar | Extension | .g4 in nexcore | Status |
|---|---|---|---|---|
| Building Block | SchemaDSL | .schema | YES | Ported from Python |
| Building Block | RulesDSL | .rules | YES | Ported from Python |
| Building Block | TransformDSL | .xform | YES | Ported from Python |
| Contract | ApiDSL | .api | YES | DONE -- validated, 3 examples parse clean |
| Contract | ScreenDSL | .screen | -- | FUTURE |
| Orchestration | ServiceDSL | .service | YES | DONE -- validated, 3 examples parse clean |
| Orchestration | ProcDSL | .proc | YES | Ported from Python |

---

## 9. Decision Log

| # | Decision | Rationale | Date |
|---|---|---|---|
| D1 | nexcore as unified Rust workspace | Single Cargo workspace for all backend engines; feature flags for product packaging | 2026-03-30 |
| D2 | 2 repos: nexcore + nexstudio | Backend/frontend separation; nexcore is library, nexstudio is GUI shell | 2026-03-30 |
| D3 | Cargo feature flags for products | NexMig/NexMod/NexIntel packaged as feature-flagged builds from same workspace | 2026-03-30 |
| D4 | ANTLR4 for all grammars | Free COBOL/CICS/SQL grammars; antlr-rust proven by cobol2rust; no new parser tech | 2026-03-30 |
| D5 | Start nexcore with nexflow crates | Begin with ApiDSL + ServiceDSL; cobol2rust crates migrate later when stable | 2026-03-30 |
| D6 | Grammars in nexcore/grammar/ | Canonical location; both Rust and legacy Python consume from here | 2026-03-30 |
| D7 | No cross-product hard deps | NexMod works without NexMig (greenfield); bridge crates are optional | 2026-03-30 |
| D8 | Python toolchain gradually retired | nexflow-toolchain superseded by nexcore; VS Code plugin rewired to Rust LSP | 2026-03-30 |
| D9 | Qualified annotation pattern (namespace.value) | Extends pii.X precedent from SchemaDSL; cfg.X for config indirection, audit.X/sec.X future | 2026-03-30 |
| D10 | Config block replaces middleware in .service | Generic key-value pairs; compiler validates known keys; overridable via nexflow.config.yaml | 2026-03-30 |
| D11 | Structural keywords only; config values as IDENTIFIER | Reduces keyword count ~25%; auth schemes, targets, time units parsed as identifiers | 2026-03-30 |
| D12 | Grammar = syntax, Compiler = semantics | Grammar is permissive; name resolution, type checking, cross-grammar validation are compiler concerns | 2026-03-30 |
