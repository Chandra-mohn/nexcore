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
- [DONE] Nested group objects (01/05/10 hierarchy) -> NestedObjectDef with `object ... end`
- [DONE] OCCURS groups -> `list<object> ... end` nested objects (is_list: true)
- [DONE] Deep nesting (01/05/10/15+) -> recursive NestedObjectDef
- [DONE] Co-access grouping from FileHints (paragraph_scope -> co-accessed field groups)
- [DONE] Level hierarchy grouping (COBOL level structure drives nested object boundaries)

### Tests: 27 passing

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
- [DONE] PIC-based type resolution from DataDivision (direct path, falls back to heuristic)
- [DONE] Field type map built from WORKING-STORAGE/LOCAL-STORAGE/LINKAGE

### Expression Extraction
- [DONE] cobol_add/subtract/multiply/divide -> arithmetic expressions (legacy path via expr_extract)
- [DONE] move_alphanumeric_literal/move_numeric_literal -> assignment expressions (legacy path)
- [DONE] COMPUTE/MOVE/ADD/SUBTRACT/MULTIPLY/DIVIDE -> real expressions (direct path via condition_extract)
- [DONE] Expression extraction wired into legacy transform emitter apply/mappings blocks

### Purity Detection
- [DONE] Side-effect analysis from COBOL AST (Display, Accept, Read, Write, Call, ExecSql, etc.)
- [DONE] Pure/impure detection wired into direct transform emitter
- [DONE] Syn-based purity detection for legacy path (scans for display/cobol_read/exec_sql calls)
- [DONE] `pure : true/false` always emitted in TransformDSL output

### on_error Blocks
- [DONE] ErrorStatement AST types (Action, LogError, ErrorCode, Emit, Reject, Default)
- [DONE] Serializer: on_error ... end block with action/log_error/error_code statements
- [DONE] Extract ON SIZE ERROR from COBOL AST (ADD/SUBTRACT/MULTIPLY/DIVIDE/COMPUTE)
- [DONE] Wired into direct transform emitter (SingleField and MultiField)

### validate_input / validate_output Blocks
- [DONE] ValidationRule AST types (Require, Simple) with ValidationMessage and Severity
- [DONE] Serializer: validate_input/validate_output ... end blocks
- [DONE] PIC-based constraint generation (unsigned numeric -> non-negative, string -> max length)
- [DONE] Wired into direct transform emitter from DataDivision field types

### Conditional Compose
- [DONE] ComposeRef enum (Simple, When, Otherwise) replaces flat Vec<Ident>
- [DONE] ComposeType::Conditional variant
- [DONE] Serializer: compose conditional with when/otherwise syntax
- [DONE] Detect EVALUATE...PERFORM pattern from COBOL AST -> conditional compose

### Metadata, Use Blocks, Cache Hints
- [DONE] TransformMetadata AST (version, description) with serializer
- [DONE] CacheDecl AST (ttl, key) with short-form and long-form serializers
- [DONE] use block AST (Vec<Ident>) with serializer in transform_block
- [DONE] All fields default to None in emitters (no COBOL source maps to these)

### Tests: 43 passing

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
- [DONE] return: block from writes
- [DONE] Scrutinee captured in traceability comment
- [DONE] Extract match arm literal patterns -> exact conditions (legacy + direct)
- [DONE] Extract match arm body assignments -> action values (via extract_move_actions / extract_action_from_expr)
- [DONE] Range pattern support in match arms (lo to hi syntax)
- [DONE] Wildcard arm -> * condition
- [DONE] Real conditions from EVALUATE WHEN values (direct path via evaluate_to_rule_shape)

### Procedural Rule Generation
- [DONE] rule ... : ... end structure
- [DONE] Description inferred from function name
- [DONE] Local variable declarations from reads
- [DONE] if/elseif/else/endif skeleton matching branch count
- [DONE] has_else tracking
- [DONE] Extract if condition expressions -> RulesDSL boolean expressions (both paths)
- [DONE] Extract set/let statements from branch bodies (via extract_if_chain / extract_if_branches)
- [DONE] Flag-setting patterns detected via MOVE action extraction

### Decision Table Grammar Features
- [DONE] description field on DecisionTable with serializer
- [DONE] version field on DecisionTable with serializer
- [DONE] post_calculate block (let/set statements after return block)
- [DONE] hit_policy variants (FirstMatch, SingleHit, MultiHit, CollectAll -- AST types exist)

### Grammar Features Not Yet Used
- [PLANNED] services block (external service declarations -- no COBOL mapping)
- [PLANNED] actions block (action method declarations -- no COBOL mapping)
- [PLANNED] aggregate block (for collect_all results -- no COBOL mapping)
- [PLANNED] Collection expressions (any, all, sum, filter, etc. -- no COBOL mapping)

### Tests: 16 passing

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
- [DONE] Map COBOL FD entries to connector types (Sequential->file, LineSequential->csv, Indexed->db)
- [DONE] Map COBOL OPEN modes to I/O direction (INPUT->receive, OUTPUT->emit, I-O->both)
- [DONE] ConnectorSpec with connector_type + config on receive/emit blocks
- [DONE] FD-based detection preferred, falls back to name heuristic when no DataDivision

### Import References
- [DONE] Import schema from E1
- [DONE] Import per-section transforms from E2
- [DONE] Import per-section rules from E3

### Execution + Loops
- [DONE] ExecutionBlock AST (parallelism, partition_by) with serializer
- [DONE] LoopBlock AST from PERFORM UNTIL (condition extracted via condition_to_string)
- [DONE] Loop wraps performed paragraph steps when PERFORM UNTIL detected
- [DONE] mode batch always emitted for COBOL programs

### Parallel + Routing
- [DONE] ParallelBlock AST with branch serialization
- [DONE] Independent section detection (no shared read/write fields)
- [DONE] Automatic parallel fan-out when entry point performs independent sections
- [DONE] RouteBlock AST with when/otherwise serialization

### Grammar Features Not Yet Used
- [PLANNED] state_machine declaration (no direct COBOL mapping)
- [PLANNED] Metrics and resilience blocks (could map from FILE STATUS checks)
- [PLANNED] Phase blocks (could map from EOD marker patterns)

### Tests: 15 passing

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
- [DONE] Parse Rust function bodies for cobol_*() call patterns (expr_extract.rs)
- [DONE] Map cobol_compute() -> arithmetic expressions
- [DONE] Map cobol_add/subtract/multiply/divide -> simple expressions
- [DONE] Map move_*_literal() -> assignments
- [DONE] Map match arms -> condition/action pairs (extract_match_branches)
- [DONE] Map if predicates -> boolean expressions (extract_if_chain / extract_if_branches)
- [DONE] COBOL AST direct extraction for all arithmetic verbs (condition_extract.rs)

### Tier 3 Assessment Integration
- [DONE] Use ws-decomposition for better entity grouping (E1 -- co-access grouping from FileHints)
- [PLANNED] Use dispatcher-analysis for process flow detection (E4)
- [PLANNED] Use call graph for compose block ordering (E2)

### FileHints Integration
- [DONE] Use FieldHint.paragraph_scope for co-access schema grouping (E1)
- [PLANNED] Use ParagraphHint.local_only_fields for input/output refinement (E2)
- [PLANNED] Use level_88_groups for enum constraint enrichment (E1)
