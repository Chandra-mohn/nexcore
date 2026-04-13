# Nex Platform -- Strategic Product Vision

**Status**: DRAFT -- under review
**Date**: 2026-03-22
**Purpose**: Capture holistic product strategy across the Nex product family

---

## Product Family Overview

The Nex Platform is a COBOL modernization suite comprising three products
unified under the "Nex" brand. Each product delivers standalone value and
can be sold independently, but together they form a complete modernization
pipeline.

```
+===================================================================+
|                      NEX PLATFORM                                 |
|===================================================================|
|                                                                   |
|  NexMig    -- Migration       "Get off the mainframe"             |
|  NexMod    -- Modernization   "Redesign for modern stack"         |
|  NexIntel  -- Intelligence    "Understand your COBOL"             |
|  NexStudio -- Desktop UI      "The COBOL modernization workbench" |
|                                                                   |
+===================================================================+
```

---

## Product Definitions

### NexMig (Migration)

**Tagline**: Get off the mainframe -- fast, safe, automated.

**What it does**: Automated COBOL-to-target-language migration with
idiomatic refinement and production-ready build artifacts.

**Two flavors** (customer chooses upfront):
- **NexMig/Rust**: COBOL -> COBOL-shaped Rust -> Rustification -> Production
- **NexMig/Java**: COBOL -> COBOL-shaped Java -> Javafication -> Production

**Output includes**: Source code + build artifacts (Cargo.toml / pom.xml),
project scaffolding, ready for production deployment.

**Current repo**: `cobol2rust` (both Rust and Java backends)

**Maturity**:
- Rust path: Mature (51K files transpiled, 1000+ tests, zero failures)
- Java path: Planned (8-session implementation plan documented)

### NexMod (Modernization)

**Tagline**: Redesign legacy systems for the modern stack.

**What it does**: Takes migrated code (from NexMig/Rust or NexMig/Java) and
provides a design surface for full modernization -- manual and AI-assisted
decisions about target architecture.

**Two modes of operation**:

```
MODE 1 -- MODERNIZATION (from legacy, with NexMig)
================================================
NexMig output (Rust or Java)
    |
    v
DSL Emission (.schema / .rules / .xform / .proc)
    |
    v
Nexflow Design Surface (manual + AI-assisted decisions)
    |
    v
Modern Stack Targets

MODE 2 -- GREENFIELD (new development, standalone)
================================================
User designs from scratch
    |
    v
DSL authoring (.schema / .rules / .xform / .proc)
    |
    v
Nexflow Design Surface (manual + AI-assisted decisions)
    |
    v
Modern Stack Targets
```

**Modern Stack Targets** (both modes):
- Data:    Spark, Flink, Kafka, MongoDB
- APIs:    REST, GraphQL, gRPC
- Compute: Microservices, serverless
- Infra:   Cloud / private / hybrid
- UI:      Web, mobile, GUI

**NexMod is a standalone product.** It can be used for greenfield
development without any legacy input. It also integrates with NexMig
as a modernization add-on for legacy-to-modern transformations.

**Current repos**:
- DSL emitters live in `cobol2rust` (104 tests, typed AST)
- Nexflow toolchain + NexflowAI live in `nexflow`

### NexIntel (Intelligence)

**Tagline**: Understand your COBOL before you touch it.

**What it does**: Standalone analysis and investigation tools for COBOL
codebases and Data. Helps teams understand what they have before deciding what to do.

**Capabilities**:
- **Lineage**: Data and program flow tracing (NDJSON format)
- **Cluster Analysis**: Program grouping and relationship discovery
- **Data Intelligence**: Copybook + code analysis, data structure understanding
- **Query/Exploration**: Interactive investigation of COBOL structures

**Standalone value**: NexIntel can be sold to customers who aren't ready
to migrate yet. Understanding is the first step. NexIntel engagements
become natural lead-gen for NexMig and NexMod.

**Current repos**:
- coqu (data intelligence = coqu-di, working)
- Lineage and cluster analysis planned in cobol2rust

---

## Four Paths to Production

```
PATH A -- Migration to Rust (NexMig/Rust)
================================================================
COBOL -> [Parse] -> [Rust Codegen] -> [Rustify] -> PRODUCTION
                                                    (Cargo.toml,
                                                     idiomatic Rust)

PATH B -- Migration to Java (NexMig/Java)
================================================================
COBOL -> [Parse] -> [Java Codegen] -> [Javafy] -> PRODUCTION
                                                   (pom.xml,
                                                    idiomatic Java)

PATH C -- Legacy Modernization (NexMig + NexMod)
================================================================
COBOL -> [NexMig A or B] --+-> [DSL Emit] -> [Nexflow] -> PRODUCTION
                            |   .schema       Design       (Spark, Flink,
                            |   .rules        Surface      Kafka, Mongo,
                            |   .xform        Manual +     APIs, uSvc,
                            |   .proc         AI-assisted  Cloud, Web,
                            |                              Mobile)
                            |
                            +-> Direct DSL from COBOL AST

PATH D -- Greenfield Development (NexMod standalone)
================================================================
User -> [DSL Authoring] -> [Nexflow] -> PRODUCTION
         .schema            Design       (same modern stack
         .rules             Surface      targets as Path C)
         .xform             Manual +
         .proc              AI-assisted
```

NexIntel supports paths A, B, C (and standalone use):

```
[NexIntel] -----> insights feed into -----> Path A, B, or C decisions
    |
    +---> also usable standalone (no migration required)
```

---

## Architecture Diagram

```
UNDERSTAND              TRANSFORM                DESIGN                PRODUCE
----------              ---------                ------                -------

                     +-> [Rust Codegen] -> [Rustify] -----------------------> PROD
                     |   COBOL-shaped      idiomatic     Path A               Rust
                     |   Rust              Rust
+=============+      |
| COBOL       |      |                                 +============+
| ~1500 src   |----->+                                 |            |      +=========+
| ~800 copy   |      |                                 | NEXFLOW    |----->| PROD    |
+=============+      +-> [Java Codegen] -> [Javafy] -->| (Design    |      | Modern  |
      |              |   COBOL-shaped      idiomatic   |  Surface)  |      | Stack   |
      |              |   Java              Java        |            |      +=========+
      |              |                         |       | .schema    |
      |              +-> [DSL Emitters] -------+------>| .rules     |
      |                  (from COBOL AST)              | .xform     |
      |                                                | .proc      |
      |                                                +============+
      |                           Path B
      |              +-> [Java Codegen] -> [Javafy] -----------------------> PROD
      |                                                                      Java
      |
      |          +======================================+
      +--------->| NexIntel                             |
                 |                                      |
                 | [Lineage]   data/program flow         |
                 | [Cluster]   program grouping          |
                 | [DataIntel] copybook + code analysis  |
                 | [Query]     interactive exploration   |
                 +======================================+
                          |
                          v
                   Informs all paths + standalone value
```

---

## Repo Strategy (Current -> Target)

### Current State

```
REPO              LANGUAGE    CONTAINS
----              --------    --------
cobol2rust        Rust        Parser, transpiler, rustify, DSL emitters,
                              CLI (10 subcommands), 11 crates
cobol2java        --          NACA reference code (RETIRING)
nexflow           Python      Nexflow toolchain, NexflowAI
coqu              Python      ANTLR parser (redundant), REPL, coqu-di
coqu-di           --          Empty (.idea only)
```

### Target State (under consideration)

```
REPO              PRODUCT       CONTAINS
----              -------       --------
cobol2rust        NexMig        Parser (shared), Rust codegen, Java codegen,
                                rustify, javafy, DSL emitters, CLI
                                Parser exposed as reusable crate.

nexflow           NexMod        Nexflow toolchain, NexflowAI
                                Consumes DSL files emitted by NexMig.

coqu              NexIntel      Lineage, cluster analysis, data intelligence,
(rebuilt)                       query/exploration.
                                Depends on cobol2rust parser crate.
                                Retire redundant Python/ANTLR parser.

cobol2java        --            RETIRED
coqu-di           --            RETIRED (absorbed into coqu/NexIntel)
```

### Key Prerequisite

The parser in cobol2rust must be extractable as a shared dependency.
Currently it is `crates/cobol-core` + `crates/cobol-transpiler`. NexIntel
needs to depend on the parser without pulling in the transpiler.

Possible approach: ensure `cobol-core` (or a new `cobol-parser` crate)
is independently publishable or usable as a git dependency.

---

## Product-Market Positioning

```
PRODUCT     BUYER NEED                    REQUIRES NEXMIG?   STANDALONE?
-------     ----------                    ----------------   -----------
NexIntel    "Help us understand           No                 Yes
             our COBOL codebase & data"

NexMig      "Get us off the               No                 Yes
             mainframe to Rust/Java"

NexMod      "Modernize legacy to          No (but benefits   Yes
             modern architecture"          from NexMig input)

NexMod      "Build new apps on            No                 Yes
             modern stack" (greenfield)
```

### Sales Motions

```
MOTION 1 -- Legacy Customer (bottom-up)
=========================================
NexIntel (assessment/discovery)
    |
    | "Now that you understand your code..."
    v
NexMig (migration to Rust or Java)
    |
    | "Want to go further than lift-and-shift?"
    v
NexMod (full modernization)

MOTION 2 -- Greenfield Customer
=========================================
NexMod (design + generate modern apps from scratch)

MOTION 3 -- Mixed (legacy + new development)
=========================================
NexMig -> NexMod (modernize legacy)
              +
          NexMod (greenfield for new modules)
```

---

## UI Strategy

### Application Type

Desktop-first (developer-grade snappiness like JetBrains RustRover) with a
lightweight web dashboard for PMs/executives.

**DECIDED: Tauri v2** (Rust backend + system webview frontend):
- Direct FFI to cobol2rust crates (no API server, no serialization)
- Native Rust worker pool for parsing/transpiling (background threads)
- Memory-mapped file I/O for monster files (1M+ lines)
- Smaller binary than Electron (~10 MB vs ~150 MB), better performance
- Cross-platform: Windows, macOS, Linux

**DECIDED: Svelte 5 + shadcn-svelte + TypeScript**:
- Svelte 5 runes: fine-grained reactivity, no virtual DOM, smallest bundle (~2 KB)
- shadcn-svelte: copy-paste components, JetBrains Darcula-like aesthetic
- shadcn-svelte-extras: Tree View for file browser
- Monaco Editor for code viewing/editing (1M+ lines via virtual scrolling)
- Svelte Flow (xyflow, 35K stars) for lineage/pipeline graph visualization
- D3 for cluster maps, custom charts
- PaneForge for IDE panel layout (resizable panels)
- Tailwind CSS v4 for styling, dark theme

**UI Target**: Reproduce JetBrains RustRover layout as closely as possible.
Full specification: docs/nex_desktop_ui_spec.md

### User Types

```
USER TYPE          PRIMARY TOOL        NEEDS
-----------        ----------------    ---------------------------------
Developer/SME      Desktop app         Code exploration, migration runs,
                                       code review, DSL editing
Analyst            Desktop app         Lineage, cluster analysis,
                                       data intelligence queries
PM/Executive       Web dashboard       Progress metrics, quality reports,
                                       timeline, status overview
```

### Monster File Handling (1M+ lines)

The backend (Rust parser) handles large files natively. The UI challenges
and solutions:

- **Code viewing**: Monaco with virtual scrolling (only renders viewport)
- **Code review**: Structure-based navigation (division -> section ->
  paragraph), not raw line scrolling
- **Lineage graphs**: Progressive disclosure (zoom levels like Google Maps)
- **Batch processing**: Background worker pool, progress streaming to UI
- **Full codebase (51K files)**: Batch mode (background) or headless
  server for CI/CD pipelines

### Execution Modes

```
MODE            USE CASE                    WHERE
-----------     -------------------------   -------------------
Interactive     Single file, small batch    Desktop app
Batch           Full codebase migration     Desktop (background)
Headless        CI/CD, overnight runs       Server (same binary)
Remote          Connect to server for       Desktop -> Server
                heavy workloads
```

### UI Phases (Priority Order)

```
PHASE 1: NexIntel
  Desktop:
    - File browser (COBOL source tree from git repo)
    - Code viewer (syntax-highlighted COBOL)
    - Data explorer (copybook hierarchy, fields, usage)
    - Lineage graph (interactive flow diagram)
    - Cluster map (program groupings)
    - Query interface (search/filter)
  Web dashboard:
    - Project summary (file counts, complexity)
    - Coverage map (what's analyzed)

PHASE 2: NexMig
  Desktop (adds to Phase 1):
    - Migration config (source, target, options)
    - Migration run (progress, logs, errors)
    - Code review (side-by-side COBOL vs Rust/Java)
    - Rustify/Javafy view (before/after idiomatic transform)
    - Build output (project structure, Cargo.toml/pom.xml)
  Web dashboard (adds to Phase 1):
    - Migration progress (files done/pending/failed)
    - Quality metrics (test results, warnings)
    - Timeline (migration history, velocity)

PHASE 3: NexMod (later -- significant Nexflow UI already exists)
  Desktop (adds to Phase 2):
    - DSL editor (Monaco + Nexflow grammar)
    - Architecture canvas (visual design surface)
    - AI assistant (modernization recommendations)
    - Preview (generated code before commit)
    - Target config (Spark/Flink/Kafka/etc)
  Web dashboard (adds to Phase 2):
    - Modernization progress
    - Architecture overview
```

---

## Version Control and Authentication

### Prerequisite

Customer commits COBOL source to Git. We do not integrate with mainframe
SCM (Endevor, ClearCase, SCLM, etc.). Getting COBOL into Git is the
customer's responsibility.

### Git-Based Architecture

```
INPUT:  Git repo containing COBOL source
OUTPUT: Git repo containing migrated code (Rust/Java/DSL)
AUTH:   Git provider OAuth

All three (input, output, auth) go through the same Git provider.
```

### Auth Model

Identity and permissions delegated to git provider:

```
Git Role       Nex Platform Role
--------       -----------------
Read           Viewer (PM/exec dashboard, read-only)
Write          Developer (run migrations, commit results)
Admin          Project Admin (configure migration options)
Owner          Full access
```

### VCS Support Matrix

```
Provider          OAuth    Git    PR API   Priority
-----------       -----    ---    ------   --------
GitHub            Yes      Yes    Yes      Phase 1
GitLab            Yes      Yes    Yes      Phase 1
Azure DevOps      Yes      Yes    Yes      Phase 2
Bitbucket         Yes      Yes    Yes      Phase 2
Gitea             Yes      Yes    Yes      Phase 3
Self-hosted Git   SSH key  Yes    No       Phase 3
```

### Auth Phases

```
Phase 1 (MVP):    License key (local, no network needed)
                  Input/output: local git repos
                  No OAuth, no SSO -- CLI-style workflow

Phase 2:          GitHub + GitLab OAuth
                  Auto-clone, auto-commit, PR creation
                  Web dashboard with same OAuth

Phase 3:          Azure DevOps, Bitbucket OAuth
                  OIDC/SAML for enterprise SSO (Okta, Azure AD)
                  Audit logging, service accounts for headless mode
```

### Desktop App Auth Flow

```
First launch:
  [Nex Platform] -> "Sign in with GitHub/GitLab"
                 -> [OAuth in browser]
                 -> [Token stored locally]
                 -> [List repos with access]
                 -> "Select COBOL project"
                 -> Ready.

Subsequent launches:
  [Cached token] -> [Refresh if needed] -> Ready.

Offline/air-gapped:
  [License key file] -> [Local repos only] -> Ready.
```

---

## Open Questions (for review)

1. **Product names**: NexMig, NexMod, NexIntel -- final or working names?

2. **Parser sharing**: Extract cobol-core as publishable crate, or use
   git dependency? Versioning strategy?

3. **NexIntel language**: Rebuild in Rust (share parser natively) or keep
   Python with FFI bridge? Data science workflows favor Python, but
   parser sharing favors Rust.

4. **coqu-di migration**: coqu-di has progressed significantly in Python.
   Rewrite cost vs. FFI bridge cost vs. keeping Python with exported
   AST (e.g., cobol2rust emits JSON AST that Python consumes)?

5. **DSL contract**: The .schema/.rules/.xform/.proc files are the
   interface between NexMig and NexMod. Should there be a shared
   schema/grammar repo or package that both depend on?

6. **Mixed-language migration**: Future possibility of Rust for
   high-performance code + Java for general code in the same project.
   Not for now ("user decides upfront") but worth designing for?

7. **Nexflow as product name**: Does "Nexflow" become "NexMod" or does
   Nexflow remain the toolchain name under the NexMod product umbrella?

8. ~~**Desktop framework**~~: **DECIDED -- Tauri v2**. Lighter than Electron,
   Rust-native backend, cross-platform. Pure Rust UI frameworks (Iced, Slint,
   Dioxus) rejected due to lack of Monaco equivalent and graph viz components.

9. ~~**Frontend framework**~~: **DECIDED -- Svelte 5 + shadcn-svelte**.
   Performance-first (no virtual DOM, smallest bundle). shadcn-svelte provides
   closest match to JetBrains Darcula aesthetic. Svelte Flow for graphs.
   React rejected (larger bundle, VDOM overhead). SolidJS rejected (smaller
   ecosystem, no Svelte Flow equivalent). Full spec: docs/nex_desktop_ui_spec.md

10. **Web dashboard priority**: Day-one requirement or Phase 2?

11. **Team frontend experience**: Affects framework choice and
    AI-assisted development approach.

---

## Decision Log

| Date       | Decision                                            | Status  |
|------------|-----------------------------------------------------|---------|
| 2026-03-22 | cobol2java repo retired, Java codegen in cobol2rust | DECIDED |
| 2026-03-22 | coqu-di absorbed into coqu project                  | DECIDED |
| 2026-03-22 | Single COBOL parser (in cobol2rust), shared         | DECIDED |
| 2026-03-22 | Four paths to production (A, B, C, D)               | DECIDED |
| 2026-03-22 | Customer picks Rust or Java upfront (not both)      | DECIDED |
| 2026-03-22 | Analysis tools usable standalone + with migration   | DECIDED |
| 2026-03-22 | NexMod supports greenfield development (no legacy)  | DECIDED |
| 2026-03-22 | No mainframe SCM integration -- COBOL must be in Git| DECIDED |
| 2026-03-22 | Auth delegated to Git provider OAuth                | DECIDED |
| 2026-03-22 | Desktop-first, web dashboard for PMs                | DECIDED |
| 2026-03-22 | Product names: NexMig, NexMod, NexIntel             | DRAFT   |
| 2026-03-22 | Repo consolidation strategy                         | DRAFT   |
| 2026-03-22 | Desktop framework: Tauri v2                         | DECIDED |
| 2026-03-22 | Frontend framework: Svelte 5 + shadcn-svelte        | DECIDED |
| 2026-03-22 | Component library: shadcn-svelte + shadcn-extras     | DECIDED |
| 2026-03-22 | UI target: reproduce RustRover IDE layout            | DECIDED |
| 2026-03-22 | Performance-first: smallest bundle, no virtual DOM   | DECIDED |
| 2026-03-22 | Product name: NexStudio (desktop workbench)          | DECIDED |
| 2026-03-22 | NexStudio repo: nex-studio                           | DECIDED |
| 2026-03-22 | Full spec: docs/nex_desktop_ui_spec.md               | DECIDED |
