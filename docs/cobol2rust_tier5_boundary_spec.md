# Tier 4/5 Boundary Specification

## Overview

This document defines the boundary between Tier 4 (deterministic structural
transformation) and Tier 5 (business-level specification via Nexflow DSL).
These are complementary, not sequential -- both operate on Phase 1 Rust output
and produce different artifacts for different purposes.

## The Boundary

```
Tier 4: "Make this Rust structurally better"
        Input:  .cobol2rust-target.toml (Rust-level architectural choices)
        Output: Restructured Rust code (idiomatic, compilable)
        Scope:  Code shape -- dispatcher, structs, error types, I/O backends
        Engine: cobol-rustify StructuralRule trait (deterministic)

Tier 5: "Express what this code DOES in business terms"
        Input:  Phase 1 Rust + #[cobol(...)] attributes + Tier 3 assessments
        Output: Nexflow DSL (.schema, .rules, .xform, .proc, .infra)
        Scope:  Business specification -- entities, rules, transforms, processes
        Engine: cobol-rustify DSL emitters -> Nexflow compiler
```

## Architecture

### Two Production Paths

```
Phase 1 Rust (with #[cobol(...)] attributes)
    |
    +---> Tiers 1-3 (per-file analysis + assessment)
    |         |
    |         +---> Tier 4 (.cobol2rust-target.toml -> restructured Rust)
    |         |         |
    |         |         v
    |         |     Idiomatic Rust -- production system TODAY
    |         |     (mainframe exit, behavioral equivalence preserved)
    |         |
    |         +---> Tier 5 (analysis -> Nexflow DSL generation)
    |                   |
    |                   v
    |               .schema / .rules / .xform / .proc / .infra
    |               (the "design surface" for SME + architect)
    |                   |
    |                   v (human reviews, evolves)
    |               Approved Nexflow DSL
    |                   |
    |                   v
    |               Nexflow compiler -> MODERN system TOMORROW
    |               (Rust services, Flink streams, database DDL, etc.)
```

### Key Principle: Tier 4 Remains the Single Execution Engine

Tier 5 does NOT modify Rust code directly. It produces Nexflow DSL as a
forward-looking specification. The architecture is A+C hybrid:

- **A (Config Generator)**: Tier 5 analysis produces Nexflow DSL files that
  serve as the comprehensive specification of the target system.
- **C (Annotation Layer)**: The DSL itself IS the human-readable specification.
  No separate annotation reports needed -- Nexflow DSL achieves 100-200x
  compression vs source code, making SME review practical.

No circular feedback loop. Tier 5 feeds FORWARD into Nexflow compiler,
not backward into Tier 4.

## What Each Tier Handles

### Tier 4 Scope (Rust-Level, Deterministic)

Tier 4 operates on Rust syntax. It knows about structs, functions, match arms,
type names. All decisions are encoded in `.cobol2rust-target.toml`.

| Decision | Config Key | Mechanical Transform |
|---|---|---|
| Dispatcher strategy | `control_flow.strategy` | Inline/functions/state-machine/async |
| Data model shape | `data_model.strategy` | Flat/nested/split/domain structs |
| Error handling | `error_handling.strategy` | Status-quo/Result/thiserror/anyhow |
| I/O backend | `io.default_backend` | File/database/stream/API |
| Field naming | `data_model.naming` | Preserve/strip-prefix/rust-idiom |
| Struct grouping | `data_model.structs.*` | Explicit field-to-struct mapping |

Given identical config + source, Tier 4 output is identical. Reproducible.
Deterministic. No judgment required.

### Tier 5 Scope (Business-Level, Requires Understanding)

Tier 5 operates on business semantics. It requires understanding what the
code DOES, not just how it's structured.

| Decision | DSL Output | Why Judgment Is Needed |
|---|---|---|
| What is `ws_acct_01_typ`? | `account_type` in `Account.schema` | Domain vocabulary |
| What does this IF chain do? | `CreditLimitDecision.rules` | Business rule comprehension |
| What does this COMPUTE do? | `InterestCalculation.xform` | Financial logic understanding |
| Where to split 1M-line program? | 400 `.proc` files by domain | Process boundary identification |
| What infrastructure is needed? | `production.infra` bindings | Architecture decisions |

These decisions require domain knowledge, business context, and architectural
vision. They cannot be encoded as deterministic config.

### Comparison Table

| Question | Tier 4 Answer | Tier 5 Answer |
|---|---|---|
| Field naming | `acct_01_typ` (strip-prefix) | `account_type` in `Account.schema` |
| Dispatcher restructuring | `strategy = "functions"` | `payment_processing.proc` with typed steps |
| What does IF/EVALUATE do? | (not its job) | Named decision table in `.rules` |
| Where to split monolith? | (not its job) | Domain-organized `.proc` files |
| Error types | `ErrorStrategy::Thiserror` | Validation failures in `.rules` |
| I/O modernization | `IoBackend::Database` | Full `.infra` with connection details |

## Tier 5 Implementation: DSL Emitters

Tier 5 in cobol-rustify is a set of emitters that read Phase 1 Rust via `syn`
and produce Nexflow DSL files. Each emitter targets one DSL layer.

### 5 Emitters

#### 1. SchemaEmitter -> .schema files

Reads `#[cobol(level, pic, redefines, level88, occurs)]` attributes on struct
fields. Uses Tier 3 `ws-decomposition` assessment (prefix groups, co-access
clusters) to detect entity boundaries. Produces one `.schema` file per entity.

Source: WorkingStorage struct fields with COBOL attributes
Output: Entity definitions with typed fields, constraints, enums

#### 2. RulesEmitter -> .rules files

Reads nested EVALUATE/IF patterns in paragraph function bodies. Uses Tier 3
`status-to-result` assessment for status field analysis. Extracts condition
trees and converts to decision tables.

Source: Control flow patterns in paragraph functions
Output: Decision tables, validation rules, business rule sets

#### 3. TransformEmitter -> .xform files

Reads COMPUTE/MOVE/arithmetic patterns. Uses field dependency graph derived
from `#[cobol(reads, writes)]` function attributes. Produces field-level
transform specifications.

Source: Arithmetic and assignment patterns
Output: Input/output typed transforms with field mappings

#### 4. ProcessEmitter -> .proc files

Reads `#[cobol(section, performs)]` function attributes. Uses Tier 3
`dispatcher-analysis` assessment (call graph, SCCs, cycle detection).
Clusters paragraphs by section, naming prefix, and data affinity.

Source: Paragraph call graph + section grouping
Output: Process definitions organized by business domain

#### 5. InfraEmitter -> .infra file

Reads file handle usage patterns and EXEC SQL connection references. Uses
Tier 3 `io-modernize` assessment for access pattern classification.

Source: File I/O and database access patterns
Output: Infrastructure resource inventory (legacy bindings)

### Emitter Trait

```rust
/// Trait for Tier 5 DSL emitters.
pub trait DslEmitter: Send + Sync {
    /// Unique emitter identifier.
    fn id(&self) -> &'static str;

    /// Which Nexflow DSL layer this emitter targets.
    fn layer(&self) -> DslLayer;

    /// Analyze workspace and produce DSL file contents.
    fn emit(&self, ctx: &EmitterContext) -> Vec<DslFile>;
}

pub enum DslLayer {
    Schema,    // .schema
    Rules,     // .rules
    Transform, // .xform
    Process,   // .proc
    Infra,     // .infra
}

pub struct DslFile {
    /// Relative path for the DSL file (e.g., "schema/Account.schema").
    pub path: String,
    /// Generated DSL content.
    pub content: String,
    /// Confidence score (0.0 - 1.0) for AI-assisted suggestions.
    pub confidence: f64,
    /// Human-readable notes for SME review.
    pub notes: Vec<String>,
}

pub struct EmitterContext<'a> {
    /// All source files with parsed ASTs.
    pub files: &'a HashMap<String, SourceFile>,
    /// COBOL semantic hints.
    pub hints: Option<&'a HintsFile>,
    /// Tier 3 assessment transforms.
    pub assessments: &'a [Transform],
    /// Target config (for reference, not driving transforms).
    pub target: Option<&'a TargetConfig>,
}
```

## Dependencies

### Prerequisite: #[cobol(...)] Attributes (Session D)

Tier 5 emitters depend on COBOL structural metadata embedded in Phase 1 Rust
output. The transpiler must emit `#[cobol(...)]` attributes on:

- Structs: `copy`, `record_len`
- Fields: `level`, `pic`, `offset`, `comp3`, `redefines`, `level88`, `occurs`
- Functions: `section`, `performs`, `reads`, `writes`

This is a separate implementation session (Session D) in the cobol-transpiler
crate. See `docs/cobol2rust_nexflow_integration.md` for the full attribute
specification.

### Nexflow Project (External)

Nexflow DSL grammar and compiler evolve independently in the nexflow workspace.
cobol2rust generates DSL text that conforms to whatever grammar version is
current. Key nexflow evolutions needed (tracked in nexflow project):

- `.proc` type extension: batch, service, scheduled process types
- File source/sink support for batch processes
- Rust code generation backend
- `.infra` legacy resource types (DB2, flat files)

cobol2rust does NOT depend on the nexflow compiler at build time. It generates
DSL text files. The nexflow compiler consumes them separately.

## The Compression Effect

Why Nexflow DSL is the right format for Tier 5 output:

| Artifact | COBOL Lines | Rust Lines | Nexflow DSL | Compression |
|---|---|---|---|---|
| 1 entity (100 fields) | ~400 | ~200 | ~30 | 13x |
| 1 business rule set | ~200 | ~150 | ~20 | 10x |
| 1 transform | ~50 | ~40 | ~5 | 10x |
| 1 process definition | ~660 | ~500 | ~15 | 44x |
| Full monolith (1M lines) | 1,000,000 | ~800,000 | ~5,000-10,000 | 100-200x |

A domain expert can review 5,000-10,000 lines of Nexflow DSL in days.
They cannot review 800,000 lines of Rust in any reasonable timeframe.

## GUI Integration

The planned Tauri GUI wraps both Tier 4 and Tier 5:

```
GUI Panels:
  [Tier 1-3 Analysis] -- assessment results, safety reports
  [Tier 4 Config]     -- .cobol2rust-target.toml editor (dropdowns, toggles)
  [Tier 5 DSL]        -- Nexflow DSL viewer/editor (syntax-highlighted)
  [Diff View]         -- before/after comparison for Tier 4 transforms
  [DSL Preview]       -- generated .schema/.rules/.proc preview

Workflow:
  1. Load Phase 1 Rust workspace
  2. Run Tiers 1-3 analysis (automatic)
  3. Review assessments in GUI
  4. Configure Tier 4 via GUI -> .cobol2rust-target.toml
  5. Run Tier 4 (deterministic transform)
  6. Review Tier 5 DSL suggestions in GUI
  7. SME edits/approves DSL
  8. Export approved DSL for Nexflow compiler
```

## Session Plan

| Session | Topic | Scope |
|---|---|---|
| A | Target architecture config | .cobol2rust-target.toml schema (DONE) |
| B | Tier 4 structural rules | 4 StructuralRule implementations (DONE) |
| C | Tier 4/5 boundary | This spec (DONE) |
| D | #[cobol(...)] attributes | Transpiler codegen for structural metadata |
| E | Tier 5 DSL emitters | SchemaEmitter, RulesEmitter, etc. |
| F | Integration testing | End-to-end: COBOL -> Tier 4 Rust + Tier 5 DSL |
