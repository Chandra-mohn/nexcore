# Nexflow DSL Emitter Capabilities

Tracks what each emitter can do today vs what needs enhancement.
Grep for `[PARTIAL]` or `[PLANNED]` to find work remaining.

Last updated: Session E4 (2026-03-20)

---

## E1: SchemaEmitter (schema_emitter.rs)

### Core Emission
- [DONE] Parse `#[cobol(...)]` field attributes via syn
- [DONE] PIC -> NexflowType mapping (type_mapping.rs)
- [DONE] Entity decomposition by common prefix grouping
- [DONE] Single-field prefixes -> misc schema
- [DONE] SchemaDSL grammar-valid output (schema ... end)
- [DONE] Identity block (key field heuristic)
- [DONE] Pattern heuristic (master_data / event_log / state_machine)

### Field Types
- [DONE] PIC X(n) -> string(n) / char(n)
- [DONE] PIC 9(p) -> integer / integer(p)
- [DONE] PIC 9(p)V9(s) -> decimal(p+s, s)
- [DONE] COMP-3 -> decimal
- [DONE] COMP/COMP-5 -> integer
- [DONE] COMP-1/COMP-2 -> float
- [DONE] OCCURS -> list(inner_type)
- [DONE] Date detection from PIC 9(8) + date-like name

### Special Handling
- [DONE] REDEFINES -> comment + review note (not emitted as field)
- [DONE] Level-88 -> enum() constraints block
- [DONE] Traceability comments (// COBOL: original PIC)
- [PARTIAL] Nested group objects (01/05/10 hierarchy) -- flat only, no nested `object ... end`
- [PLANNED] Co-access grouping (uses Tier 3 ws-decomposition hints)
- [PLANNED] Level hierarchy grouping (COBOL level structure as entity boundary)

### Tests: 20 passing

---

## E2: TransformEmitter (transform_emitter.rs)

### Core Emission
- [DONE] Parse `#[cobol(...)]` function attributes via syn
- [DONE] Function classification (SingleField / MultiField / Compose / SectionDispatcher)
- [DONE] Section-based file grouping (one .xform per section)
- [DONE] TransformDSL grammar-valid output
- [DONE] Boilerplate function filtering (run, main, new, stop_run, goback)

### Transform Shapes
- [DONE] Single-write paragraph -> `transform ... apply ... end`
- [DONE] Multi-write paragraph -> `transform_block ... mappings ... end`
- [DONE] Orchestrator with PERFORMs -> `compose sequential ... end`
- [DONE] Section dispatcher detection and handling

### Input/Output
- [DONE] reads attribute -> input spec (single or multi)
- [DONE] writes attribute -> output spec (single or multi)
- [DONE] Type hint heuristics from field names (flag->boolean, date->date, etc.)
- [PARTIAL] Field map built but unused (_field_map) -- could resolve PIC-based types
- [PLANNED] Actual PIC-based type resolution from field map lookup

### Expression Extraction
- [PARTIAL] Placeholder apply blocks with TODO comments
- [PARTIAL] Placeholder mappings with TODO comments
- [PLANNED] Parse `cobol_compute()` calls -> TransformDSL arithmetic expressions
- [PLANNED] Parse `cobol_add()` / `cobol_subtract()` -> simple apply expressions
- [PLANNED] Parse `move_numeric_literal()` -> assignment expressions
- [PLANNED] Parse `move_alphanumeric_literal()` -> string assignment expressions

### Grammar Features Not Yet Used
- [PLANNED] validate_input / validate_output blocks
- [PLANNED] on_error blocks
- [PLANNED] Metadata (version, description)
- [PLANNED] use blocks in transform_block
- [PLANNED] Purity detection (does function have side effects?)
- [PLANNED] Cache hints
- [PLANNED] Conditional compose (`when ... : ...` in compose block)

### Tests: 13 passing

---

## E3: RulesEmitter (rules_emitter.rs)

### Core Emission
- [DONE] Rust function body analysis via syn (match + if/else detection)
- [DONE] match expression -> decision_table classification
- [DONE] if/else if chain -> procedural rule classification
- [DONE] Recursive descent through nested expressions
- [DONE] Decision table precedence over procedural rule
- [DONE] Section-based file grouping
- [DONE] RulesDSL grammar-valid output
- [DONE] Boilerplate and orchestrator filtering

### Decision Table Generation
- [DONE] hit_policy first_match
- [DONE] given: block from reads (with type heuristics)
- [DONE] decide: matrix header from reads (conditions) + writes (actions)
- [DONE] Placeholder rows (one per match arm, capped at 10)
- [DONE] return: block from writes
- [DONE] Scrutinee captured in traceability comment
- [PARTIAL] Placeholder conditions (all wildcards) -- TODO: extract match arm patterns
- [PARTIAL] Placeholder actions (all wildcards) -- TODO: extract match arm bodies
- [PLANNED] Extract match arm literal patterns -> exact conditions
- [PLANNED] Extract match arm guard expressions -> comparison conditions
- [PLANNED] Extract match arm body assignments -> action values
- [PLANNED] Detect range patterns (e.g., 700..=799) -> range conditions

### Procedural Rule Generation
- [DONE] rule ... : ... end structure
- [DONE] Description inferred from function name
- [DONE] Local variable declarations from reads
- [DONE] if/elseif/else/endif skeleton matching branch count
- [DONE] has_else tracking
- [PARTIAL] Placeholder conditions -- TODO: extract if predicates
- [PARTIAL] Placeholder actions (set var = 0) -- TODO: extract branch bodies
- [PLANNED] Extract if condition expressions -> RulesDSL boolean expressions
- [PLANNED] Extract set/let statements from branch bodies
- [PLANNED] Detect flag-setting patterns -> add_flag() action calls
- [PLANNED] Detect lookup patterns -> lookup() action calls

### Grammar Features Not Yet Used
- [PLANNED] services block (external service declarations)
- [PLANNED] actions block (action method declarations)
- [PLANNED] hit_policy variants (single_hit, multi_hit, collect_all)
- [PLANNED] post_calculate block (derived computations)
- [PLANNED] aggregate block (for collect_all results)
- [PLANNED] description on decision tables
- [PLANNED] version tracking
- [PLANNED] Priority column in decision matrix
- [PLANNED] Collection expressions (any, all, sum, filter, etc.)

### Tests: 11 passing

---

## E4: ProcessEmitter (process_emitter.rs)

### Core Emission
- [DONE] Build call graph from `performs` attributes
- [DONE] Entry point detection (run function or unperformed node)
- [DONE] Section dispatcher expansion (walks into child paragraphs)
- [DONE] Node role classification (EntryPoint, SectionDispatcher, ProcessingStep, Leaf)
- [DONE] Cycle detection (prevents infinite recursion on PERFORM loops)
- [DONE] One .proc file per COBOL program
- [DONE] ProcDSL grammar-valid output (process ... end)
- [DONE] Boilerplate filtering (main, new, stop_run, goback -- keeps run)

### Process Steps
- [DONE] Leaf paragraphs with single I/O -> `transform ... using ... into`
- [DONE] Leaf paragraphs with multi I/O -> `evaluate using` (decision logic)
- [DONE] Processing steps -> `transform` + recurse into performs
- [DONE] Section dispatchers -> expand inline with section comment

### I/O Detection
- [DONE] Input fields detected by naming (FILE, INPUT, RECORD) -> `receive` block
- [DONE] Output fields detected by naming (OUTPUT, PRINT, REPORT) -> `emit to` block
- [PARTIAL] Data source configuration placeholder (TODO comments)
- [PLANNED] Map COBOL FD entries to specific connector types (file, kafka, db)
- [PLANNED] Map COBOL OPEN/CLOSE patterns to process lifecycle

### Import References
- [DONE] Import schema from E1
- [DONE] Import per-section transforms from E2
- [DONE] Import per-section rules from E3

### Grammar Features Not Yet Used
- [PLANNED] execution block (parallelism, partition, time, mode)
- [PLANNED] state_machine declaration
- [PLANNED] PERFORM UNTIL loops -> process loops or windowed processing
- [PLANNED] Parallel fan-out/fan-in for independent sections
- [PLANNED] Branch statements for conditional routing
- [PLANNED] Metrics and resilience blocks
- [PLANNED] Phase blocks (for EOD marker patterns)

### Tests: 8 passing

---

## Shared Infrastructure

### cobol_attrs.rs
- [DONE] StructCobolAttr extraction (program name)
- [DONE] FieldCobolAttr extraction (level, pic, usage, offset, len, signed, redefines, occurs, level88, etc.)
- [DONE] FnCobolAttr extraction (section, performs, reads, writes)
- [DONE] extract_annotated_fields() -- all struct fields with COBOL attrs
- [DONE] extract_annotated_fns() -- all functions with COBOL attrs
- [DONE] extract_program_name() -- from struct-level attribute

### type_mapping.rs
- [DONE] pic_to_nexflow_type() with full PIC clause parsing
- [DONE] NexflowType enum with Display impl
- [DONE] Date detection heuristic (PIC 9(8) + date-like name)
- [DONE] 13 tests

### DslEmitter trait (mod.rs)
- [DONE] DslEmitter trait with id(), layer(), emit()
- [DONE] DslLayer enum (Schema, Transform, Rules, Process)
- [DONE] DslFile struct with path, content, confidence, notes, source_fields
- [DONE] EmitterContext with program_name, syn_file, source_text, hints, assessments, target

---

## Cross-Cutting Enhancements (apply to multiple emitters)

### Expression Extraction Engine
- [PLANNED] Parse Rust function bodies for cobol_*() call patterns
- [PLANNED] Map cobol_compute() -> arithmetic expressions
- [PLANNED] Map cobol_add/subtract/multiply/divide -> simple expressions
- [PLANNED] Map move_*_literal() -> assignments
- [PLANNED] Map match arms -> condition/action pairs
- [PLANNED] Map if predicates -> boolean expressions
- Applies to: E2 (apply/mappings), E3 (conditions/actions)

### Tier 3 Assessment Integration
- [PLANNED] Use ws-decomposition for better entity grouping (E1)
- [PLANNED] Use dispatcher-analysis for process flow detection (E4)
- [PLANNED] Use call graph for compose block ordering (E2)
- Applies to: E1, E2, E4

### FileHints Integration
- [PLANNED] Use FieldHint.paragraph_scope for better field -> rule mapping
- [PLANNED] Use ParagraphHint.local_only_fields for input/output refinement
- [PLANNED] Use level_88_groups for enum constraint enrichment
- Applies to: E1, E2, E3
