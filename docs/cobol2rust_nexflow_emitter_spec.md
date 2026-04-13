# Nexflow DSL Emitter Specification

## Session E: Nexflow DSL Generation from Phase 1 Rust

### Overview

Session E generates Nexflow DSL files from Phase 1 transpiled Rust code. The emitters
read `#[cobol(...)]` attributes (Session D) and Rust AST via `syn`, combined with
Tier 3 assessments, to produce grammar-valid DSL that the Nexflow compiler accepts.

All output must parse against the four ANTLR4 grammars at:
`~/workspace/nexflow/nexflow-toolchain/grammar/`

### Architecture

```
Phase 1 Rust (.rs files with #[cobol(...)] attributes)
    |
    v
syn parser -> CobolAttributes + Rust AST
    |
    v
+-- SchemaEmitter (E1) -> .schema files (SchemaDSL.g4)
+-- TransformEmitter (E2) -> .xform files (TransformDSL.g4)
+-- RulesEmitter (E3) -> .rules files (RulesDSL.g4)
+-- ProcessEmitter (E4) -> .proc files (ProcDSL.g4)
```

### Input Sources

1. **#[cobol(...)] attributes** -- parsed from generated Rust via syn
   - Struct-level: `program = "CARDPROC"`
   - Field-level: `level, pic, comp3/comp/comp5, offset, len, signed, redefines, occurs, level88`
   - Paragraph-level: `section, performs, reads, writes`
   - Section-level: `section = "MONETARY-PROCESSING"`

2. **Rust AST** -- function bodies parsed via syn
   - Match arms (from EVALUATE) -> decision table shape
   - If/else chains (from IF) -> procedural rule shape
   - Arithmetic expressions (from COMPUTE) -> transform apply blocks
   - Assignment statements (from MOVE) -> field mappings

3. **Tier 3 Assessments** -- from cobol-rustify analysis
   - `ws-decomposition`: Proposed sub-structs with field groupings
   - `dispatcher-analysis`: Call graph, cycles, leaf paragraphs

4. **FileHints** -- COBOL semantic metadata (.rustify/hints.json)
   - FieldHint: pic, usage, level, redefines, paragraph_scope, read/write counts
   - ParagraphHint: performs, performed_by, local_only_fields
   - level_88_groups, call_targets, file_io_fields

### Fidelity Target

60-70% toward production-ready. Grammar-valid, structurally correct, decomposed.
Confidence scores per file. Comments mark uncertainty. SME refinement expected.

### Sub-Session Plan

| Session | Focus | Grammar | Key Challenge |
|---------|-------|---------|---------------|
| E1 | Schema Emission | SchemaDSL.g4 | PIC->type mapping, entity decomposition, 8000+ field programs |
| E2 | Transform Emission | TransformDSL.g4 | Rust body -> apply block, reads/writes -> input/output |
| E3 | Rules Emission | RulesDSL.g4 | Recognizing decision table vs procedural rule shape |
| E4 | Process Emission | ProcDSL.g4 | Paragraph ringfencing by processing path, I/O flow |

### Shared Infrastructure (Built in E1, used by E2-E4)

#### DslEmitter Trait

```rust
pub trait DslEmitter: Send + Sync {
    fn id(&self) -> &'static str;
    fn layer(&self) -> DslLayer;
    fn emit(&self, ctx: &EmitterContext) -> Vec<DslFile>;
}

pub enum DslLayer {
    Schema,     // L2 - SchemaDSL
    Transform,  // L3 - TransformDSL
    Rules,      // L4 - RulesDSL
    Process,    // L1 - ProcDSL
}

pub struct DslFile {
    pub path: String,          // e.g., "schema/account_info.schema"
    pub content: String,       // Grammar-valid DSL text
    pub confidence: f64,       // 0.0-1.0
    pub notes: Vec<String>,    // Human-readable review notes
    pub source_fields: Vec<String>,  // COBOL fields that contributed
}

pub struct EmitterContext<'a> {
    pub program_name: String,
    pub syn_file: &'a syn::File,
    pub source_text: &'a str,
    pub hints: Option<&'a FileHints>,
    pub assessments: &'a [Transform],
    pub target: Option<&'a TargetConfig>,
}
```

#### Attribute Extraction (cobol_attrs.rs)

Shared utility to parse #[cobol(...)] from syn AST:

```rust
pub struct StructCobolAttr {
    pub program: String,
}

pub struct FieldCobolAttr {
    pub level: u8,
    pub pic: Option<String>,
    pub usage: Option<String>,     // comp3, comp, comp5, comp1, comp2, index, pointer
    pub offset: Option<usize>,
    pub len: Option<usize>,
    pub signed: bool,
    pub redefines: Option<String>,
    pub occurs: Option<usize>,
    pub occurs_depending: Option<String>,
    pub level88: Option<String>,   // "ACTIVE:A,CLOSED:C"
    pub blank_when_zero: bool,
    pub justified_right: bool,
}

pub struct FnCobolAttr {
    pub section: Option<String>,
    pub performs: Vec<String>,
    pub reads: Vec<String>,
    pub writes: Vec<String>,
}

// Extraction functions
pub fn extract_struct_attr(attrs: &[syn::Attribute]) -> Option<StructCobolAttr>;
pub fn extract_field_attr(attrs: &[syn::Attribute]) -> Option<FieldCobolAttr>;
pub fn extract_fn_attr(attrs: &[syn::Attribute]) -> Option<FnCobolAttr>;
```

#### PIC to Nexflow Type Mapping (type_mapping.rs)

```rust
pub fn pic_to_nexflow_type(pic: &str, usage: Option<&str>, signed: bool) -> NexflowType;

pub enum NexflowType {
    String(Option<usize>),           // string or string(n)
    Char(usize),                     // char(n) for fixed-length
    Integer(Option<usize>),          // integer or integer(precision)
    Decimal(usize, usize),           // decimal(precision, scale)
    Float,                           // COMP-1
    Boolean,                         // Promoted from level-88 single-value
    Date,                            // Detected from PIC 9(8) with date-like name
    Timestamp,                       // Detected from PIC X(26) with timestamp-like name
}
```

Mapping rules:
- `PIC X(n)` -> `string(n)` (or `char(n)` if n <= 3)
- `PIC 9(p)` -> `integer` (or `integer(p)` if p > 9)
- `PIC 9(p)V9(s)` / `PIC S9(p)V9(s)` -> `decimal(p+s, s)`
- `COMP-3` with PIC -> `decimal(precision, scale)` (storage irrelevant for schema)
- `COMP` / `COMP-5` with PIC 9(n) -> `integer`
- `COMP-1` -> `float`
- `COMP-2` -> `float` (SchemaDSL has no double)
- Level-88 with single value on PIC X(1) -> candidate for `enum` constraint
- `OCCURS n` -> `list(inner_type)`

---

## E1: Schema Emission -- Detailed Design

### Goal

Generate `.schema` files from WorkingStorage struct fields. One schema per
entity group (decomposed by Tier 3 ws-decomposition). Each schema conforms
to SchemaDSL.g4.

### Entity Decomposition Strategy

For a program with 8000+ fields, the ws-decomposition assessment proposes
sub-structs based on:

1. **Common prefix grouping** -- fields sharing `ws_acct_*` become `account` entity
2. **Co-access grouping** -- fields accessed by same paragraphs become entity
3. **Level hierarchy** -- COBOL level structure (01/05/10) defines natural groups
4. **Fallback** -- ungrouped fields go into a `program_name_misc` schema

Decomposition produces `Vec<EntityCandidate>`:

```rust
pub struct EntityCandidate {
    pub name: String,              // "account_info", "transaction_detail"
    pub fields: Vec<SchemaField>,
    pub reason: String,            // Why grouped (prefix, co-access, level)
    pub confidence: f64,
}

pub struct SchemaField {
    pub cobol_name: String,        // "WS-ACCT-NUMBER"
    pub rust_name: String,         // "ws_acct_number"
    pub nexflow_name: String,      // "acct_number" (prefix stripped)
    pub nexflow_type: NexflowType,
    pub required: bool,            // true unless has default/optional indicator
    pub cobol_attr: FieldCobolAttr,
}
```

### Schema Generation Rules

1. **Schema name**: Entity candidate name, snake_case (SchemaDSL IDENTIFIER = `[a-z_][a-z0-9_]*`)
2. **Pattern**: `master_data` for reference-like entities, `event_log` for transactional
   - Heuristic: if entity has timestamp-like fields -> `event_log`
   - If entity is lookup/reference table -> `reference_data`
   - Default: `master_data`
3. **Identity block**: First field that looks like a key (contains "id", "key", "code", "number")
4. **Fields block**: Each SchemaField -> `field_name type [required]`
5. **Constraints block**: Level-88 values -> `enum()` constraints
6. **Comments**: `// COBOL: original-name PIC X(10)` above each field for traceability

### Output Format Example

Given COBOL WorkingStorage with fields:
```
05 WS-ACCT-NUMBER    PIC X(10).
05 WS-ACCT-TYPE      PIC X(1).
   88 SAVINGS         VALUE 'S'.
   88 CHECKING        VALUE 'C'.
05 WS-ACCT-BALANCE   PIC S9(9)V99 COMP-3.
05 WS-ACCT-OPEN-DATE PIC 9(8).
```

Generated schema:
```
// Generated by cobol2rust Session E1
// Source: CARDPROC WorkingStorage (ws_acct prefix group)
// Confidence: 0.75

schema account_info
    pattern master_data

    identity
        acct_number string(10) required
    end

    fields
        // COBOL: WS-ACCT-NUMBER PIC X(10)
        acct_number string(10) required
        // COBOL: WS-ACCT-TYPE PIC X(1)
        acct_type char(1) required
        // COBOL: WS-ACCT-BALANCE PIC S9(9)V99 COMP-3
        acct_balance decimal(11, 2) required
        // COBOL: WS-ACCT-OPEN-DATE PIC 9(8)
        acct_open_date integer(8)
    end

    constraints
        acct_type enum("S", "C")
    end
end
```

### REDEFINES Handling

REDEFINES fields represent union types in COBOL. In Nexflow schema:
- Emit the primary field normally
- Add comment: `// COBOL REDEFINES: field_name redefines target_name`
- Do NOT emit the redefining field as a separate schema field
- Add a review note: "REDEFINES detected -- manual review needed for union type"

### OCCURS Handling

`OCCURS n` maps to `list(inner_type)`:
```
05 WS-PHONE-NUMBERS  PIC X(15) OCCURS 5.
->
phone_numbers list(string(15))
```

`OCCURS DEPENDING ON` adds a review note about variable-length arrays.

### Group (Level 01/05) Handling

COBOL group items that contain children become nested objects in SchemaDSL:
```
01 WS-ADDRESS.
   05 WS-STREET    PIC X(30).
   05 WS-CITY      PIC X(20).
   05 WS-STATE     PIC X(2).
->
address object
    street string(30) required
    city string(20) required
    state char(2) required
end
```

### Implementation Files

```
crates/cobol-rustify/src/
    dsl/
        mod.rs                -- DslEmitter trait, DslFile, DslLayer, EmitterContext
        cobol_attrs.rs        -- #[cobol(...)] attribute extraction from syn
        type_mapping.rs       -- PIC clause to NexflowType conversion
        schema_emitter.rs     -- SchemaEmitter implementation (E1)
        transform_emitter.rs  -- TransformEmitter implementation (E2)
```

### Validation

1. Generated `.schema` files must parse with Nexflow's SchemaDSL parser
2. All SchemaDSL IDENTIFIER rules satisfied (lowercase + underscore only)
3. Type parameters match grammar: `decimal(p, s)`, `string(n)`, `char(n)`
4. Constraints block uses `enum()` syntax from SchemaDSL.g4
5. Every `schema ... end` block is properly terminated
6. Unit tests with sample WorkingStorage -> expected .schema output

### Test Cases

1. Simple program (5 fields, no groups) -> single schema
2. Program with prefix groups -> multiple decomposed schemas
3. COMP-3 / COMP / COMP-5 fields -> correct decimal/integer types
4. Level-88 conditions -> constraints block with enum()
5. OCCURS fields -> list() types
6. REDEFINES fields -> comments + review notes
7. Nested groups (01/05/10) -> nested object blocks
8. 8000+ field program -> verify decomposition produces manageable schemas

---

## E2: Transform Emission -- Detailed Design

### Goal

Generate `.xform` files from paragraph functions. Each paragraph with reads/writes
attributes becomes a `transform` or `transform_block`. Section dispatchers and
orchestrator paragraphs become `compose` blocks.

### Function Classification Strategy

Each Rust function with `#[cobol(...)]` attributes is classified into one of:

1. **SingleField** -- One write field, no PERFORMs -> `transform ... apply ... end`
2. **MultiField** -- Multiple write fields, no PERFORMs -> `transform_block ... mappings ... end`
3. **Compose** -- Has PERFORMs (orchestrator) -> `transform_block ... compose sequential ... end`
4. **SectionDispatcher** -- Name ends with `_section`, only PERFORMs -> `compose sequential`

Boilerplate functions (`run`, `main`, `new`, `stop_run`, `goback`) are skipped.

### File Organization

Transforms are grouped by COBOL section into separate `.xform` files:
- `transform/processing_section.xform` -- all paragraphs in PROCESSING-SECTION
- `transform/calculation_section.xform` -- all paragraphs in CALCULATION-SECTION
- `transform/program_misc.xform` -- unsectioned paragraphs

### Input/Output Resolution

- `reads` attribute fields -> `input` block with heuristic type hints
- `writes` attribute fields -> `output` block with heuristic type hints
- Type hints based on naming conventions (flag->boolean, date->date, amount->decimal, etc.)

### Apply Block Strategy

Current implementation generates placeholder expressions with TODO comments.
Future enhancement: parse `cobol_compute()`, `cobol_add()`, `move_numeric_literal()`
calls from the Rust function body to extract actual TransformDSL expressions.

### Compose Block

Orchestrator paragraphs that PERFORM other paragraphs generate:
```
compose sequential
    proc_init
    proc_work
    proc_finish
end
```

This maps directly to TransformDSL.g4's `composeBlock` with `composeType: 'sequential'`.

### Output Format Example

Given COBOL:
```
SECTION PROCESSING-SECTION.
    PROC-INIT.
        MOVE ZERO TO WS-COUNT.
    PROC-WORK.
        ADD 100 TO WS-COUNT.
    PROC-FINISH.
        ADD 50 TO WS-COUNT.
```

Generated:
```
// Generated by cobol2rust Nexflow emitter (Session E2)
// Source: TESTPROG section processing_section

import ../schema/TESTPROG.schema

// COBOL paragraph: PROC-INIT
transform proc_init
    pure : true

    input : integer

    output : integer

    apply
        ws_count = 0 // TODO: extract expression from Rust body
    end
end

// COBOL paragraph: PROC-WORK
transform_block proc_work

    input
        ws_count : integer
    end

    output
        ws_count : integer
    end

    mappings
        ws_count = ws_count // TODO: extract mapping from Rust body
    end
end
```

### Confidence Scoring

- Base: 0.65 (lower than Schema because apply blocks need manual review)
- -0.05: No reads detected (placeholder input)
- -0.10: Mappings/apply need manual expression extraction
- Minimum: 0.10

### Test Cases

1. Boilerplate functions (`run`, `main`) -> skipped
2. Single-write paragraph -> `transform` with `apply` block
3. Multi-write paragraph -> `transform_block` with `mappings` block
4. Section dispatcher -> skipped (empty data flow)
5. Orchestrator with PERFORMs -> `compose sequential` block
6. Multiple sections -> separate `.xform` files per section
7. Type hint heuristics -> flag->boolean, date->date, amount->decimal

---

## E3: Rules Emission -- Detailed Design

### Goal

Generate `.rules` files from paragraph functions that contain decision logic.
Analyzes Rust function bodies via `syn` to detect `match` (EVALUATE) and
`if/else if` (IF/ELSE IF) patterns, then classifies them as `decision_table`
or procedural `rule` in RulesDSL.g4.

### Key Distinction from E2

- **E2 (TransformEmitter)**: Describes data flow (reads -> writes through computation)
- **E3 (RulesEmitter)**: Describes decision logic (conditions -> actions/assignments)

The same COBOL paragraph may contribute to both layers. A paragraph with a
match expression that also does arithmetic would appear in E2 as a transform
and in E3 as a decision table.

### Function Body Analysis Strategy

The RulesEmitter parses Rust function bodies via `syn` to detect patterns:

1. **`match` expressions** (from COBOL EVALUATE) -> `decision_table`
   - 2+ arms required to qualify
   - Arm count determines placeholder row count (capped at 10)
   - Scrutinee expression captured for traceability comment

2. **`if/else if` chains** (from COBOL IF/ELSE IF) -> procedural `rule`
   - 2+ branches required (single if is too simple for rules)
   - Presence of final `else` is tracked
   - Nested decisions are detected recursively

3. **No decision logic** -> skipped (E2 handles pure data flow)

4. **Pure orchestrators** (only performs, no reads/writes) -> skipped (E4 handles)

When both patterns exist in one function, `match` (decision table) takes
precedence over `if/else` (procedural rule).

### Decision Table Generation

For functions with `match` arms:

```
decision_table {nexflow_name}
    hit_policy first_match

    given:
        {reads -> input parameters with type heuristics}

    decide:
        | {read columns} | {write columns or "result"} |
        |================|
        | * | "result_0" |    // placeholder per match arm
        | * | "result_1" |

    return:
        {writes -> output parameters with type heuristics}
end
```

### Procedural Rule Generation

For functions with `if/else if` chains:

```
rule {nexflow_name}:
    description "{inferred from name}"

    let {read_var} = input.{read_var}

    if {condition} then          // TODO: extract from Rust body
        set {write_var} = 0
    elseif true then             // TODO: extract condition N
        set {write_var} = 0
    else
        set {write_var} = 0      // TODO: default action
    endif

    return
end
```

### Type Mapping for RulesDSL

RulesDSL uses different base types than TransformDSL/SchemaDSL:

| Naming Pattern | RulesDSL Type |
|---|---|
| flag, switch, ind, active, valid | boolean |
| name, desc, text, code, type, status | text |
| date, -dt, -dte | date |
| amt, amount, balance, rate, limit, income | money |
| count, num, qty, score, idx | integer |
| pct, percent, ratio | percentage |
| (default) | number |

### Confidence Scoring

- Base: 0.55 (lower than schema/transform -- body analysis is heuristic)
- -0.05: No reads detected (placeholder given params)
- -0.10: Placeholder conditions/actions need manual extraction
- Minimum: 0.10

### File Organization

Rules grouped by section into separate `.rules` files:
- `rules/rate_section.rules` -- paragraphs in RATE-SECTION
- `rules/validation_section.rules` -- paragraphs in VALIDATION-SECTION
- `rules/program_misc.rules` -- unsectioned paragraphs

### Implementation Files

```
crates/cobol-rustify/src/
    dsl/
        rules_emitter.rs  -- RulesEmitter implementation (E3)
```

### Test Cases

1. `match` with 4 arms -> `decision_table` with hit_policy, given, decide, return
2. `if/else if/else` chain -> procedural `rule` with if/elseif/else/endif
3. Simple assignment (no match/if) -> skipped
4. Pure orchestrator (only performs) -> skipped
5. Multiple sections -> separate `.rules` files
6. Reads/writes reflected in given/return blocks with type heuristics
7. Confidence degrades for missing reads and placeholder conditions

---

## E4: Process Emission -- Detailed Design

### Goal

Generate one `.proc` file per COBOL program from the paragraph call graph.
The process orchestrates transforms (E2) and rules (E3) into a sequential
pipeline that mirrors the original COBOL execution flow.

### Key Distinction from E2/E3

- **E2 (TransformEmitter)**: Describes *what* each paragraph transforms
- **E3 (RulesEmitter)**: Describes *what* each paragraph decides
- **E4 (ProcessEmitter)**: Describes *how* paragraphs are orchestrated

### Call Graph Construction

The `performs` attribute on each function defines directed edges:

```
run -> [PROCESSING-SECTION, REPORTING-SECTION]
PROCESSING-SECTION -> [INIT-PARA, CALC-PARA, FINISH-PARA]
REPORTING-SECTION -> [PRINT-HEADER, PRINT-DETAIL]
```

Each node is classified:

| Role | Criteria | Process Output |
|---|---|---|
| EntryPoint | `run` function | Walk performs in order |
| SectionDispatcher | Only performs, no reads/writes | Expand inline |
| ProcessingStep | Has performs AND reads/writes | `transform using` + recurse |
| Leaf | No performs, has reads/writes | `transform using` or `evaluate using` |

Leaf paragraphs with multiple inputs AND multiple outputs use `evaluate using`
(decision logic). Single I/O leaves use `transform ... using ... into`.

### Entry Point Detection

1. If a function named `run` exists with `#[cobol(...)]`, it is the entry point
2. Otherwise, find the node not performed by any other node

### I/O Detection

Field names containing FILE, INPUT, OUTPUT, PRINT, REPORT, RECORD are
flagged as I/O fields. When detected:
- Input fields trigger a `receive` block at the start of the process
- Output fields trigger an `emit to` block at the end

### Cycle Detection

COBOL `PERFORM ... UNTIL` creates cycles in the call graph. The emitter
detects revisited nodes and emits a comment rather than infinite recursion.
These cycles are flagged as review notes ("may indicate PERFORM UNTIL loop").

### Output Format Example

```
// Generated by cobol2rust Nexflow emitter (Session E4)
// Source: CARDPROC

import ../schema/CARDPROC.schema
import ../transform/processing_section.xform
import ../rules/processing_section.rules

process cardproc
    mode batch

    receive input_records
        schema cardproc_input
    // TODO: configure data source

    // Entry: RUN
    // Section: PROCESSING-SECTION
    // Transform: INIT-PARA
    transform input_records using init_para into output_records
    // Step: CALC-PARA (reads: WS-COUNT, writes: WS-COUNT)
    transform input_records using calc_para into output_records
    // Evaluate: DECIDE-PARA (decision logic)
    evaluate using decide_para

    emit to output_records
        schema cardproc_output

end
```

### Confidence Scoring

- Base: 0.50 (lowest -- process reconstruction is the hardest emitter)
- -0.10: No entry point detected
- Cycle detection does not reduce confidence (it is a correct observation)
- Minimum: 0.10

### Implementation Files

```
crates/cobol-rustify/src/
    dsl/
        process_emitter.rs  -- ProcessEmitter implementation (E4)
```

### Test Cases

1. Empty program (no annotated functions) -> no output
2. Linear process (run -> 3 paragraphs) -> sequential steps
3. Section dispatcher -> expands into child paragraphs
4. I/O fields -> receive and emit blocks
5. Cycle in call graph -> detected and commented
6. Multi-input leaf -> `evaluate using` (not transform)
7. Program name with hyphens -> sanitized file path
8. Section imports -> references E2 and E3 files
