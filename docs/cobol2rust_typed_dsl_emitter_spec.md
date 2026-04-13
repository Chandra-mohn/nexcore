# Typed DSL Emitter Specification

## Problem Statement

The current DSL emitters use string concatenation to produce DSL text. This is
inherently fragile -- nothing prevents generating text that violates the grammar.
Confidence scores are a workaround for this uncertainty.

Since the grammars are fixed and the emission is deterministic, **we should never
generate invalid DSL**. The solution: typed Rust structs that mirror the grammar
rules, making grammar violations a compile-time error.

## Design Principle

**If it compiles, it parses.** The Rust type system enforces grammar validity.
No confidence scores needed. No post-hoc validation needed.

## Architecture

```
Phase 1 Rust (syn AST)
    |
    v
Expression Extraction (expr_extract.rs)
    |
    v
Typed DSL AST nodes (dsl_ast/*.rs)
    |
    +-> DslSerializer::to_text()  -> .schema/.xform/.rules/.proc files
    +-> DslSerializer::to_json()  -> JSON AST for Nexflow consumption
```

## Typed AST Design

### Common Types

```rust
/// A valid DSL identifier: [a-z_][a-z0-9_]*
pub struct Identifier(String);

impl Identifier {
    pub fn new(s: &str) -> Result<Self, InvalidIdentifier> {
        // Validates at construction time
    }
}

/// Import path: ../schema/foo.schema
pub struct ImportPath {
    pub segments: Vec<String>,
    pub extension: DslExtension,
}

pub enum DslExtension {
    Schema,
    Transform,
    Xform,
    Rules,
}
```

### SchemaDSL Types (from SchemaDSL.g4)

```rust
pub struct SchemaFile {
    pub imports: Vec<ImportPath>,
    pub schemas: Vec<SchemaDefinition>,
}

pub struct SchemaDefinition {
    pub name: Identifier,
    pub pattern: Option<MutationPattern>,
    pub identity: Option<IdentityBlock>,
    pub fields: Option<FieldsBlock>,
    pub constraints: Option<ConstraintsBlock>,
}

pub enum MutationPattern {
    MasterData,
    EventLog,
    StateMachine,
    ReferenceData,
    // ... 14 patterns from grammar
}

pub struct IdentityBlock {
    pub fields: Vec<FieldDecl>,
}

pub struct FieldsBlock {
    pub fields: Vec<FieldDecl>,
}

pub struct FieldDecl {
    pub name: Identifier,
    pub field_type: FieldType,
    pub required: bool,
    pub comment: Option<String>,
}

pub enum FieldType {
    String(Option<usize>),      // string or string(n)
    Char(usize),                // char(n)
    Integer(Option<usize>),     // integer or integer(p)
    Decimal(usize, usize),      // decimal(p, s)
    Boolean,
    Date,
    Timestamp,
    Uuid,
    Bytes,
    Float,
    List(Box<FieldType>),       // list(inner)
    Object(Vec<FieldDecl>),     // nested object ... end
    SchemaRef(Identifier),      // reference to another schema
}

pub struct ConstraintsBlock {
    pub constraints: Vec<Constraint>,
}

pub enum Constraint {
    Enum(Identifier, Vec<String>),          // field enum("a", "b")
    Range(Identifier, Option<f64>, Option<f64>), // field range(min, max)
    Pattern(Identifier, String),            // field pattern("regex")
}
```

### RulesDSL Types (from RulesDSL.g4)

```rust
pub struct RulesFile {
    pub imports: Vec<ImportPath>,
    pub decision_tables: Vec<DecisionTable>,
    pub procedural_rules: Vec<ProceduralRule>,
}

pub struct DecisionTable {
    pub name: Identifier,
    pub hit_policy: HitPolicy,
    pub description: Option<String>,
    pub given: Vec<InputParam>,
    pub decide: DecideMatrix,
    pub return_params: Vec<ReturnParam>,
}

pub enum HitPolicy {
    FirstMatch,
    SingleHit,
    MultiHit,
    CollectAll,
}

pub struct InputParam {
    pub name: Identifier,
    pub param_type: ParamType,
}

pub enum ParamType {
    Text,
    Number,
    Integer,
    Boolean,
    Date,
    Timestamp,
    Money,
    Percentage,
}

pub struct DecideMatrix {
    pub headers: Vec<ColumnHeader>,
    pub rows: Vec<TableRow>,
}

pub struct ColumnHeader {
    pub name: Identifier,
}

pub struct TableRow {
    pub cells: Vec<CellContent>,
}

pub enum CellContent {
    Wildcard,                           // *
    Exact(Literal),                     // "value" or 42
    Comparison(ComparisonOp, Literal),  // > 700, <= 0.5
    Range(Literal, Literal),            // 700 to 799
    InSet(Vec<Literal>),                // in ("a", "b")
    Expression(String),                 // complex expression
}

pub enum ComparisonOp { Gt, Lt, Ge, Le, Eq, Ne }

pub enum Literal {
    Str(String),
    Int(i64),
    Decimal(String), // Keep as string to preserve precision
    Bool(bool),
    Null,
}

pub struct ProceduralRule {
    pub name: Identifier,
    pub description: Option<String>,
    pub body: Vec<RuleStatement>,
}

pub enum RuleStatement {
    If(IfStatement),
    Set(Identifier, DslExpr),
    Let(Identifier, DslExpr),
    ActionCall(Identifier, Vec<DslExpr>),
    Return,
}

pub struct IfStatement {
    pub condition: DslExpr,
    pub then_block: Vec<RuleStatement>,
    pub elseif_blocks: Vec<(DslExpr, Vec<RuleStatement>)>,
    pub else_block: Option<Vec<RuleStatement>>,
}

/// DSL expression (shared across Rules and Transform)
pub enum DslExpr {
    Field(Identifier),
    Literal(Literal),
    Binary(Box<DslExpr>, BinaryOp, Box<DslExpr>),
    Unary(UnaryOp, Box<DslExpr>),
    FunctionCall(Identifier, Vec<DslExpr>),
    FieldPath(Vec<Identifier>),
}

pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Gt, Lt, Ge, Le, Eq, Ne,
    And, Or,
}

pub enum UnaryOp { Not, Neg }
```

### TransformDSL Types (from TransformDSL.g4)

```rust
pub struct TransformFile {
    pub imports: Vec<ImportPath>,
    pub transforms: Vec<TransformDef>,
    pub transform_blocks: Vec<TransformBlockDef>,
}

pub struct TransformDef {
    pub name: Identifier,
    pub pure: Option<bool>,
    pub input: InputSpec,
    pub output: OutputSpec,
    pub apply: ApplyBlock,
}

pub struct TransformBlockDef {
    pub name: Identifier,
    pub input: InputSpec,
    pub output: OutputSpec,
    pub body: TransformBlockBody,
}

pub enum TransformBlockBody {
    Mappings(Vec<Mapping>),
    Compose(ComposeBlock),
}

pub enum InputSpec {
    Single(FieldType),
    Multi(Vec<InputFieldDecl>),
}

pub struct InputFieldDecl {
    pub name: Identifier,
    pub field_type: FieldType,
}

pub struct ApplyBlock {
    pub statements: Vec<ApplyStatement>,
}

pub enum ApplyStatement {
    Assignment(Identifier, DslExpr),
    LetBinding(Identifier, DslExpr),
}

pub struct Mapping {
    pub target: Identifier,
    pub source: DslExpr,
}

pub struct ComposeBlock {
    pub compose_type: ComposeType,
    pub refs: Vec<ComposeRef>,
}

pub enum ComposeType { Sequential, Parallel, Conditional }

pub enum ComposeRef {
    Simple(Identifier),
    When(DslExpr, Identifier),
    Otherwise(Identifier),
}
```

### ProcDSL Types (from ProcDSL.g4)

```rust
pub struct ProcessFile {
    pub imports: Vec<ImportPath>,
    pub processes: Vec<ProcessDefinition>,
}

pub struct ProcessDefinition {
    pub name: Identifier,
    pub execution: Option<ExecutionBlock>,
    pub body: Vec<ProcessStatement>,
}

pub struct ExecutionBlock {
    pub mode: Option<ProcessMode>,
    pub parallelism: Option<u32>,
}

pub enum ProcessMode { Stream, Batch, MicroBatch(Duration) }

pub enum ProcessStatement {
    Receive(ReceiveDecl),
    Transform(TransformDecl),
    Evaluate(EvaluateDecl),
    Emit(EmitDecl),
    Comment(String),
}

pub struct ReceiveDecl {
    pub name: Identifier,
    pub schema: Option<Identifier>,
}

pub struct TransformDecl {
    pub input: Identifier,
    pub using: Identifier,
    pub output: Identifier,
}

pub struct EvaluateDecl {
    pub using: Identifier,
}

pub struct EmitDecl {
    pub target: Identifier,
    pub schema: Option<Identifier>,
}
```

## Serializer

One `to_text()` method per AST type that produces grammar-valid DSL text:

```rust
impl SchemaDefinition {
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        writeln!(out, "schema {}", self.name);
        if let Some(p) = &self.pattern {
            writeln!(out, "    pattern {}", p.to_text());
        }
        if let Some(id) = &self.identity {
            id.write_to(&mut out, 1);
        }
        if let Some(f) = &self.fields {
            f.write_to(&mut out, 1);
        }
        if let Some(c) = &self.constraints {
            c.write_to(&mut out, 1);
        }
        writeln!(out, "end");
        out
    }
}
```

The key insight: `SchemaDefinition` MUST have a `name`. It CAN have `pattern`,
`identity`, `fields`, `constraints`. The `end` keyword is always emitted. The
ordering matches the grammar: `schema name pattern? identity? fields? constraints? end`.

If you construct a `SchemaDefinition`, it serializes to valid SchemaDSL. Period.

## Migration Path

### Step 1: Define types (this spec)
Build `dsl_ast/` module with types for all 4 grammars.

### Step 2: Refactor emitters
Change each emitter from `fn emit() -> Vec<DslFile>` (string content)
to `fn emit() -> Vec<TypedDslFile>` where content is a typed AST node.
The writer calls `.to_text()` for file output.

### Step 3: Drop confidence scores
Once all emitters produce typed AST nodes, confidence = 1.0 by construction
for grammar validity. The only remaining uncertainty is semantic correctness
(are the field references valid? do the types match?), which is Nexflow's
validator's job, not ours.

### Step 4: JSON serialization
Add `to_json()` alongside `to_text()`. The JSON format matches Nexflow's
ParseResult AST structure, enabling direct consumption without re-parsing.

## What This Means for the Monster File

For the 1M-line program, the emitters will produce potentially hundreds of
typed AST nodes. Each one is guaranteed grammar-valid. The question shifts
from "is this valid DSL?" to "does this accurately represent the business
logic?" -- which is a semantic question for SMEs, not a grammar question
for tooling.

## Scope

This spec covers the typed AST types and serializer. The emitter refactoring
(changing from string output to typed output) is implementation work that
builds on this foundation.

Estimated: 2 sessions for types + serializer, 1 session for emitter refactor.
