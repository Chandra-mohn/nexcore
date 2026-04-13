# NexIntel Advanced Intelligence Spec

**Date**: 2026-03-28
**Status**: Design (review before implementation)
**Depends on**: cobol-intel crate (CI-1 through CI-10 DONE), NexStudio query UI (DONE)

---

## Overview

Three capabilities that represent the highest-value features of NexIntel:

1. **Business Process Discovery** -- automatically identify the 40-80 business processes
   in a 1500-program codebase and carve out which code belongs to which process
2. **Migration Cost Estimation** -- compute effort/cost for manual vs Nex-assisted
   migration, broken down by persona, process, and wave
3. **Data Complexity Impact** -- detect hidden complexity from external parameters,
   configurations, business rules stored in DB2/VSAM, and monster copybooks

These three capabilities compose into the complete NexIntel value proposition:
"Here are your business processes, here's what each costs to migrate, and here's
where the hidden complexity lives that will blow up your estimates."

---

## 1. Business Process Discovery

### The Problem

A "business process" in mainframe COBOL is NOT a single program. It is a chain of
programs connected by data flow, JCL sequencing, and shared resources:

```
JCL Job -> Step 1 (PROG-A) -> writes FILE-X
           Step 2 (PROG-B) -> reads FILE-X, writes FILE-Y
           Step 3 (PROG-C) -> reads FILE-Y, writes REPORT
```

Programs A, B, C never CALL each other. They are linked by JCL and shared files.
The business user knows this as "the clearing process." The code analysis must
discover this relationship automatically.

### How Business Processes Manifest in COBOL

| Signal | Strength | Detection |
|---|---|---|
| File-mediated coupling (A writes X, B reads X) | Strong | File access mode on Accesses edges |
| CALL chains (A calls B calls C) | Strong | Existing Calls edges |
| Shared business copybooks | Medium | Existing Uses edges, filter out utility copybooks |
| Naming conventions (CLRG0100, CLRG0200) | Weak | Existing naming_prefix property |
| CICS transaction IDs | Strong (online) | EXEC CICS RECEIVE MAP analysis |

Naming conventions are the weakest signal. Enterprise shops tend to follow them,
but the algorithm must work even with poor naming. File coupling and CALL chains
are the strong signals.

### Process Assembly Algorithm

```
Step 1: Build affinity edges
  - File-write -> File-read edges (program writes FILE-X, another reads FILE-X)
  - CALL edges (already have)
  - Copybook edges (already have, weight down utility copybooks)
  - Naming prefix (weak weight)

Step 2: Cluster programs into process groups
  - Union-Find with weighted edges:
    file coupling (weight 3) > CALL (weight 3) > business copybook (weight 2) > naming (weight 1)
  - Or: community detection (Louvain / label propagation) on the affinity graph
  - Merge small clusters (<3 programs) into nearest neighbor

Step 3: Name the processes
  - Common naming prefix (CLRG -> "Clearing") if >60% of programs share it
  - Dominant file names (ACCTMAST -> "Account Master Processing")
  - Fallback: PROCESS-001, PROCESS-002 (human labels applied later in NexStudio)

Step 4: Classify program roles within process
  - CORE: only participates in this process (clean extraction candidate)
  - SHARED: participates in 2+ processes (boundary crossing, needs careful handling)
  - UTILITY: participates in many processes (shared library, extract once)

Step 5: Compute process metrics
  - program_count, total_loc, total_complexity, total_interfaces
  - coupling_score = % CORE programs (higher = easier extraction)
  - entry_points = programs with no incoming CALL/file dependencies in the process
  - exit_points = programs whose outputs feed other processes
```

### File Access Mode Detection

Currently, Accesses edges exist but don't distinguish read vs write. The builder
needs to extract file open mode from the AST:

```cobol
OPEN INPUT  ACCTMAST     -> mode = "read"
OPEN OUTPUT REPORT-FILE  -> mode = "write"
OPEN I-O    MASTER-FILE  -> mode = "read-write"
OPEN EXTEND LOG-FILE     -> mode = "append"
```

This comes from Statement::Open in the AST (OpenStatement with mode and file list).

### New Graph Elements

New properties on Accesses edges:
- `file_mode`: read / write / read-write / append

New node type:
- `Process` nodes with properties:
  - name, program_count, total_loc, total_complexity
  - coupling_score, entry_programs, exit_programs

New edge type:
- `BelongsTo` (Program -> Process) with property: role (core/shared/utility)
- `FeedsInto` (Process -> Process) with property: via_file (the connecting file)

### NexQuery

```
-- Discover all business processes
discover-processes;

-- Show programs in a specific process
programs within process CLEARING;

-- Show process dependencies (which processes feed which)
processes;

-- Show shared programs (boundary crossings)
programs(process_role = 'shared');
```

---

## 2. Migration Cost Estimation

### The Problem

Clients need to answer: "How much will this migration cost? How long will it take?
What do we save by using Nex vs doing it manually?"

### Who Does The Work

The migration team is NOT COBOL developers. COBOL SMEs provide consultation.

| Role | What they do | Rate basis |
|---|---|---|
| Target engineers (Java/Rust) | Write/review migrated code | Senior dev |
| COBOL SMEs (client side) | Answer questions, validate logic | Part-time consultation |
| QA engineers | Test functional equivalence | QA rate |
| Data engineers | Migrate data, validate transformations | Data eng |
| Integration engineers | Wire migrated components to systems | Senior dev |

**QA is the dominant cost** in both manual and automated approaches. The key Nex
selling point: deterministic transpilation means functional equivalence testing
is simpler -- the transpiler produces the same output for the same input.

### Cost Formula

```
MANUAL MIGRATION (per program):
  analysis_days    = 3.0 * complexity_multiplier
                     (target engineer + COBOL SME pair)
  rewrite_days     = LOC / 100 * complexity_multiplier
                     (target engineer -- slower than COBOL dev, learning curve)
  data_mapping     = interface_count * 2.0
                     (data engineer -- map files, tables, copybooks)
  qa_testing       = rewrite_days * 1.5
                     (QA -- functional equivalence, the BIGGEST cost)
  integration      = process_boundary_crossings * 3.0
                     (integration engineer)
  regression       = LOC / 500
                     (comparing before/after behavior)
  cobol_sme        = total_days * 0.15
                     (15% of total, part-time consultation)

  TOTAL_MANUAL     = sum of above

NEX-ASSISTED MIGRATION (per program):
  automated        = 0 (transpile + rustify + DSL = machine time)
  review           = LOC / 2000 * complexity_multiplier
                     (engineer reviews automated output, much faster than rewrite)
  qa_testing       = manual_qa * 0.4
                     (60% reduction -- deterministic transpilation)
  data_mapping     = manual_data * 0.5
                     (Nex data intelligence assists)
  integration      = manual_integration * 0.3
                     (Nex handles most wiring)
  cobol_sme        = total_days * 0.10
                     (less needed -- automated analysis provides context)

  TOTAL_NEX        = sum of above

SAVINGS            = TOTAL_MANUAL - TOTAL_NEX
SAVINGS_PCT        = SAVINGS / TOTAL_MANUAL * 100
```

### Complexity Multipliers

Multipliers are NOT hardcoded ceilings. They are COMPUTED from the actual data
and are unbounded. A program using a monster copybook with 8000 fields, 12K
parameters, and a rule engine could legitimately have a 20x+ multiplier.

The formula computes multipliers rather than looking them up:

```
code_multiplier =
  1.0 + (complexity_score / 5.0)         (0-10 score -> 1.0x to 3.0x)

copybook_multiplier =
  1.0 + (inherited_copybook_complexity / 20.0)
  -- 8000-field copybook with 8 REDEFINES -> complexity ~400 -> multiplier ~21x
  -- this is real: that copybook IS 20x harder to migrate

parameter_multiplier =
  1.0 + (parameter_table_count * 0.3)
       + (estimated_parameter_count / 1000.0)
  -- 12 config tables with ~12K params -> 1.0 + 3.6 + 12.0 = 16.6x
  -- this is real: 12K parameters IS a separate migration project

rule_engine_multiplier =
  1.0 + (rule_table_count * 2.0)
       + (has_dynamic_perform ? 3.0 : 0.0)
  -- rule engine with 2 rule tables + dynamic PERFORM -> 1 + 4 + 3 = 8x

total_multiplier = code_multiplier
                 * max(copybook_multiplier, 1.0)
                 * max(parameter_multiplier, 1.0)
                 * max(rule_engine_multiplier, 1.0)

-- These multiply, not add. A program that hits all three can be 50x+.
-- That's not a bug -- that program genuinely IS 50x harder to migrate.
```

The point: the system tells the truth. If a program is genuinely 50x harder
because of data complexity, the estimate should reflect that. Capping at 5x
would lie to the client.

### Cost Configuration

All cost parameters are user-configurable via a settings panel in NexStudio
and saved per-project in `nexintel/cost_config.json`:

```json
{
  "day_rates": {
    "target_engineer": 1200,
    "cobol_sme": 800,
    "qa_engineer": 900,
    "data_engineer": 1100,
    "integration_engineer": 1200,
    "currency": "USD"
  },
  "productivity": {
    "manual_loc_per_day": 100,
    "review_loc_per_day": 2000,
    "qa_ratio": 1.5,
    "regression_loc_per_day": 500,
    "cobol_sme_percentage": 0.15
  },
  "nex_reduction": {
    "qa_reduction": 0.6,
    "data_mapping_reduction": 0.5,
    "integration_reduction": 0.7,
    "sme_reduction": 0.33
  },
  "overrides": {
    "programs": {
      "CLRG0100": { "manual_days_override": 45, "note": "Known monster, manually estimated" }
    }
  }
}
```

The configurator allows:
- **Day rates by persona** -- client adjusts to their market rates
- **Productivity factors** -- LOC/day, QA ratio, etc.
- **Nex reduction factors** -- how much Nex saves per category
- **Per-program overrides** -- human can override computed estimate for specific programs
- **Currency** -- USD, EUR, GBP, etc.

NexStudio UI: Intel menu > Cost Settings opens a dialog with these fields.
Defaults are provided but everything is editable. Saved per project.

### Aggregation Levels

- **Per program**: individual estimate with breakdown
- **Per process**: sum of programs in the discovered process
- **Per migration wave**: sum of programs in topological wave
- **Portfolio total**: entire codebase

### Output Format

```json
{
  "scope": "portfolio",
  "total_programs": 1500,
  "total_loc": 2000000,
  "manual": {
    "total_days": 12500,
    "total_cost_usd": 3125000,
    "breakdown": {
      "analysis": 1800,
      "rewrite": 4200,
      "data_mapping": 1500,
      "qa_testing": 3200,
      "integration": 900,
      "regression": 400,
      "cobol_sme": 500
    },
    "duration_months": 24,
    "team_size": 22
  },
  "nex": {
    "total_days": 3400,
    "total_cost_usd": 850000,
    "breakdown": {
      "automated": 0,
      "review": 1100,
      "qa_testing": 1280,
      "data_mapping": 750,
      "integration": 270
    },
    "duration_months": 8,
    "team_size": 18
  },
  "savings": {
    "days": 9100,
    "cost_usd": 2275000,
    "percent": 73,
    "time_reduction_months": 16
  },
  "risk_factors": [
    "3 programs classified as RULE-DRIVEN (4x multiplier)",
    "1 monster copybook (8000 fields) used by 500 programs",
    "12K parameters in DB2 require separate migration project"
  ]
}
```

### NexQuery

```
-- Full portfolio estimate
estimate-cost scope all;

-- Estimate for a specific process
estimate-cost scope process CLEARING;

-- Estimate for a migration wave
estimate-cost scope wave 0;

-- Show high-risk programs
programs(automation_eligibility = 'manual');
```

---

## 3. Data Complexity Impact

### The Problem

The code is only part of the story. External data -- parameters, configurations,
business rules stored in databases and VSAM files -- creates hidden complexity
that doesn't appear in static code analysis.

Real-world example from client engagement:
- 8000+ fields in a single copybook (~32K record length)
- 8 different cascading REDEFINES
- Multiple OCCURS definitions
- 12,000+ parameters stored in DB2 tables and VSAM files
- Business rules stored in database, evaluated at runtime (effectively code-as-data)

### Types of External Data Complexity

#### 3.1 Monster Copybooks

A copybook with 8000 fields and cascading REDEFINES is a complexity multiplier
for every program that COPY-includes it:

```
copybook_complexity =
  field_count / 100                        (8000 fields = 80 base points)
  * (1 + redefines_depth * 0.5)            (8 cascading = 5x multiplier)
  * (1 + distinct_occurs * 0.2)            (each OCCURS adds 20%)
  * (1 + record_length / 4096)             (32K record = 8x)

inherited_complexity (per program) =
  sum(copybook_complexity for each COPY used)
```

Detection: fully available from AST analysis. We already parse copybooks.

#### 3.2 Parameter-Driven Programs

Programs that read configuration from DB2 tables or VSAM files and branch based
on parameter values. The code looks simple but behavior is parameterized:

```cobol
EXEC SQL SELECT PARM-VALUE INTO :WS-PARM
  FROM CONFIG-TABLE WHERE PARM-KEY = 'INTEREST-RATE-TYPE'
END-EXEC
EVALUATE WS-PARM
  WHEN 'FIXED'  PERFORM CALC-FIXED-RATE
  WHEN 'FLOAT'  PERFORM CALC-FLOAT-RATE
  WHEN 'HYBRID' PERFORM CALC-HYBRID-RATE
END-EVALUATE
```

With 12,000 parameters, the interaction space is enormous. A "simple" 200-line
program can have hundreds of behavioral modes.

Detection signals (from code analysis alone):
- EXEC SQL SELECT from tables with names matching: *PARM*, *CONFIG*, *CONTROL*,
  *SETUP*, *OPTION*, *SETTING*, *PARAMETER*
- READ from VSAM files with similar naming patterns in FD or DD names
- EVALUATE / IF chains immediately after a file READ or SQL SELECT
- Count of distinct parameter table/file references
- Fields with names like WS-PARM-*, WS-CONFIG-*, WS-OPTION-*

Note: parameters come from BOTH DB2 and VSAM files. The detection must cover:
- EXEC SQL SELECT ... FROM parameter-named-table
- READ parameter-named-file INTO WS-PARM-AREA
- ACCEPT ... FROM ENVIRONMENT (less common but possible)

#### 3.3 Rule-Driven Programs (Code-as-Data)

Programs that act as rule execution engines. The business logic is not in the
COBOL code -- it's stored in database tables or VSAM files and evaluated at runtime.

This is the hardest to migrate because:
- Migrating the COBOL executor is easy
- Migrating 10,000 rules stored in DB2/VSAM is a separate project
- The rules may reference other rules, creating a dependency chain
- Rules may contain expressions that are effectively arbitrary code

Detection signals:
- EXEC SQL SELECT from tables named: *RULE*, *LOGIC*, *FORMULA*, *CALC*,
  *DECISION*, *CRITERIA*, *CONDITION*
- READ from VSAM files with similar naming patterns
- Dynamic PERFORM (PERFORM WS-PROC-NAME) -- executing a procedure whose
  name comes from data
- Cursor-based loops reading rule tables with FETCH/process/FETCH patterns
- Large EVALUATE blocks with many WHEN branches (>10)
- Fields named WS-RULE-*, WS-FORMULA-*, WS-CALC-*

#### 3.4 Classification

Each program gets classified:

| Classification | Meaning | Cost Multiplier |
|---|---|---|
| SELF-CONTAINED | All logic is in the code | 1.0x |
| PARAMETER-DRIVEN | Behavior depends on external parameters | 2.0x |
| RULE-DRIVEN | Code is an executor for externally stored rules | 4.0x |
| HYBRID | Both parameter-driven and rule-driven | 5.0x |

### Data Complexity Score

```
data_complexity (per program) =
  inherited_copybook_complexity          (from monster copybooks)
  + parameter_sensitivity_score          (parameter table/file access count * branching)
  + rule_engine_score                    (rule table access + dynamic execution patterns)
  + sql_join_complexity                  (multi-table SQL with complex WHERE clauses)
  + file_key_complexity                  (alternate keys, dynamic VSAM access)

Scale: 0.0 - 10.0 (normalized)
```

### Properties Added

Per-program:
- `data_complexity`: 0-10 composite score
- `external_dependency_class`: self-contained / parameter-driven / rule-driven / hybrid
- `data_sensitivity`: list of detected signals
  ["param-table: CONFIG-TABLE", "rule-table: BUSINESS-RULES", "monster-copybook: CPYACCT"]
- `inherited_copybook_complexity`: sum of copybook complexity scores
- `parameter_table_count`: number of parameter/config tables accessed
- `rule_table_count`: number of rule/logic tables accessed

Per-copybook:
- `copybook_complexity`: 0-100 score based on fields, REDEFINES, OCCURS, record length
- `is_monster`: true if complexity > 50

### Risk Factors Surfacing

The cost estimator uses data complexity to surface risk factors:

```json
"risk_factors": [
  "MONSTER COPYBOOK: CPYACCT (8000 fields, 8 REDEFINES, 32K record) used by 523 programs",
  "PARAMETER-DRIVEN: 47 programs read from 12 config tables (12K+ parameters)",
  "RULE-DRIVEN: 3 programs execute rules from BUSINESS-RULES table",
  "DATA MIGRATION: 12K parameters + rule definitions require separate migration project",
  "TESTING: Monster copybook creates ~400 valid record variants for QA coverage"
]
```

### NexQuery

```
-- Show data complexity scores
rank programs by data_complexity top 20;

-- Find rule-driven programs
programs(external_dependency_class = 'rule-driven');

-- Find programs affected by monster copybooks
copybooks(is_monster = true)
programs using each;

-- Show all parameter-driven programs with their config tables
programs(external_dependency_class = 'parameter-driven');
```

---

## Implementation Plan

### Session CI-11: Process Discovery + File Coupling

1. Enhance AST-to-graph builder:
   - Track file open mode (INPUT/OUTPUT/I-O/EXTEND) from Statement::Open
   - Store `file_mode` property on Accesses edges
2. File-mediated coupling:
   - New edge or affinity scoring: program A writes FILE-X, program B reads FILE-X
3. ProcessDiscoveryPass (new intelligence layer):
   - Union-Find clustering with weighted edges
   - Process naming heuristics
   - Program role classification (core/shared/utility)
   - Process metrics (program count, LOC, complexity, coupling score)
4. Process node type in graph
5. `discover-processes` NexQuery verb
6. NexStudio: process list view in results panel
7. Tests against CardDemo

### Session CI-12: Data Complexity + Copybook Propagation

1. CopybookComplexityPass:
   - Compute copybook_complexity from field count, REDEFINES depth, OCCURS, record length
   - Propagate inherited_complexity to programs via Uses edges
2. Parameter sensitivity detection:
   - SQL table name heuristics (*PARM*, *CONFIG*, *CONTROL*, *SETUP*, etc.)
   - VSAM file name heuristics (same patterns on FD/file names)
   - Post-read branching detection (EVALUATE/IF after READ or SQL SELECT)
3. Rule engine detection:
   - Rule table name heuristics (*RULE*, *LOGIC*, *FORMULA*, *CALC*, etc.)
   - Dynamic PERFORM detection
   - Cursor loop patterns
4. Classification: self-contained / parameter-driven / rule-driven / hybrid
5. Properties: data_complexity, external_dependency_class, data_sensitivity
6. Tests with synthetic programs + CardDemo validation

### Session CI-13: Cost Estimation

1. CostEstimatePass:
   - 5-persona effort model (target engineer, COBOL SME, QA, data engineer, integration)
   - Complexity multipliers from all sources (structural + data + copybook + classification)
   - Manual vs Nex estimates per program
2. Portfolio aggregation: by process, by wave, total
3. Risk factor surfacing: monster copybooks, parameter counts, rule-driven programs
4. `estimate-cost` verb with scope (all / process / wave)
5. Dollar cost calculation with configurable day rates
6. NexStudio: cost summary card (manual vs nex, savings %, risk factors)
7. Tests with realistic scenarios

### Session CI-14: Integration + Polish

1. Connect cobol-data (Data Intelligence) to cobol-intel (Code Intelligence):
   - Link File nodes to decoded data stats if available
   - Record count, variant distribution from actual data files
2. NexStudio process view: clickable process -> shows programs in process
3. Cost report export (JSON/CSV)
4. End-to-end test: CardDemo full pipeline (build graph -> discover processes -> estimate cost)

---

---

## 4. JCL Support (CI-15)

### Why JCL Matters

JCL is the missing link for process discovery. Without it, we guess which programs
run together (via file naming heuristics). With JCL, we KNOW:
- Which programs run in sequence (JOB -> STEP1 -> STEP2 -> STEP3)
- Which physical datasets map to which FD names (DD cards)
- Which steps read/write which datasets (DISP=SHR vs DISP=NEW)
- Conditional execution (COND=, IF/THEN/ELSE)

DD cards are the Rosetta Stone: they map COBOL's logical FD file name to the
physical dataset. When two JCL steps reference the same dataset, those programs
are in the same business process -- proven, not guessed.

### Grammar Complexity

JCL is syntactically simpler than COBOL:
- Lines start with `//`
- JOB, EXEC PGM=, EXEC PROC=, DD DSN= are the key statements
- Continuation via `//` + spaces
- Symbolic parameters `&VAR`
- Conditional: COND=, IF/THEN/ELSE/ENDIF

A hand-written parser (like NexQuery) would work. No ANTLR needed.
Estimate: 2-3 sessions for a `cobol-jcl` crate.

### What It Produces

JCL nodes and edges in the graph:
- Job node: name, class, priority
- Step nodes: program name, order, conditions
- DD edges: step -> dataset, with DISP (read/write/new/append)
- Dataset -> FD mapping: connects JCL datasets to COBOL file descriptors

---

## 5. Sub-Program Granularity for Monster Files

### The Problem

A million-line COBOL file is not one program. It's potentially hundreds of business
functions in one file. Process discovery at the program level would assign the entire
file to one process -- useless.

### Paragraph is the Right Granularity

In COBOL, a paragraph is the natural "function" unit. Each paragraph has:
- A defined entry point (name)
- Fields it reads/writes (already tracked)
- Paragraphs it PERFORMs (already tracked)
- Files it accesses (NEED TO ADD at paragraph level)

Going lower than paragraphs (statement level) is too granular.

### Monster File Process Discovery

```
For files with >100 paragraphs:
  1. Build paragraph PERFORM tree (already have)
  2. Identify root paragraphs (entry points from MAIN, not called by other paragraphs)
  3. Each root + its transitive PERFORM closure = a "function cluster"
  4. Assign function clusters to processes based on:
     - Fields touched (shared fields = same process)
     - Files accessed (shared files = same process)
     - Naming conventions (2000-VALIDATE-ACCT vs 3000-GENERATE-REPORT)
  5. A single monster file may span 5-10 business processes
```

### Data Model Impact

Need paragraph-level file access edges:
- Currently: Program -[Accesses]-> File
- Needed: Paragraph -[Accesses]-> File (which paragraph does the READ/WRITE)

This requires tracking which paragraph contains each OPEN/READ/WRITE/REWRITE/DELETE
statement. The AST has this information -- the builder needs to extract it.

---

## 6. User-Modelable Cost Formulas

### The Problem

Hardcoded cost formulas don't work. Different consulting teams, different client
contexts, different market rates. The formula must be a template users can edit.

### Formula Configuration

Stored in `nexintel/cost_config.json` alongside day rates:

```json
{
  "day_rates": {
    "target_engineer": 1200,
    "cobol_sme": 800,
    "qa_engineer": 900,
    "data_engineer": 1100,
    "integration_engineer": 1200,
    "currency": "USD"
  },
  "productivity": {
    "manual_loc_per_day": 100,
    "review_loc_per_day": 2000,
    "qa_ratio": 1.5,
    "regression_loc_per_day": 500,
    "cobol_sme_percentage": 0.15
  },
  "nex_reduction": {
    "qa_reduction": 0.6,
    "data_mapping_reduction": 0.5,
    "integration_reduction": 0.7,
    "sme_reduction": 0.33
  },
  "formulas": {
    "manual": {
      "analysis":     "3.0 * code_multiplier",
      "rewrite":      "loc / productivity.manual_loc_per_day * code_multiplier",
      "data_mapping": "interface_count * 2.0",
      "qa_testing":   "rewrite * productivity.qa_ratio",
      "integration":  "process_boundary_crossings * 3.0",
      "regression":   "loc / productivity.regression_loc_per_day",
      "cobol_sme":    "total * productivity.cobol_sme_percentage"
    },
    "nex": {
      "automated":    "0",
      "review":       "loc / productivity.review_loc_per_day * code_multiplier",
      "qa_testing":   "manual.qa_testing * (1 - nex_reduction.qa_reduction)",
      "data_mapping": "manual.data_mapping * (1 - nex_reduction.data_mapping_reduction)",
      "integration":  "manual.integration * (1 - nex_reduction.integration_reduction)"
    }
  },
  "overrides": {
    "programs": {
      "CLRG0100": { "manual_days_override": 45, "note": "Known monster, manually estimated" }
    }
  }
}
```

### Formula Variables

Available in formulas from graph properties:
- `loc`, `complexity`, `field_count`, `paragraph_count`, `call_count`
- `interface_count`, `file_count`, `data_complexity`
- `copybook_multiplier`, `parameter_multiplier`, `rule_engine_multiplier`
- `process_boundary_crossings`
- References to other formula results: `manual.rewrite`, `manual.qa_testing`
- Config values: `productivity.qa_ratio`, `nex_reduction.qa_reduction`

### NexStudio Formula Editor

Intel menu > Cost Settings opens a dialog with tabs:
- Day Rates: persona rates + currency
- Productivity: LOC/day, QA ratio, etc.
- Nex Savings: reduction percentages per category
- Formulas: editable text fields for each formula line
- Overrides: per-program manual overrides
- Reset to Defaults button

The expression evaluator is simple arithmetic (+ - * / parentheses) with
variable references. No loops, no conditionals -- just math expressions.

---

## Revised Implementation Plan

### Session CI-11: Process Discovery + File Coupling

1. Enhance builder: track file open mode on Accesses edges
2. Add paragraph-level file access edges (READ/WRITE in which paragraph)
3. File-mediated coupling (program A writes FILE-X, program B reads FILE-X)
4. ProcessDiscoveryPass: clustering with weighted edges
5. Sub-program granularity: function clusters within monster files
6. Process nodes, program/paragraph role classification
7. `discover-processes` NexQuery verb
8. Tests against CardDemo

### Session CI-12: Data Complexity + Copybook Propagation

1. CopybookComplexityPass: computed scores (unbounded)
2. Parameter sensitivity: detect DB2 AND VSAM parameter/config access
3. Rule engine detection: dynamic PERFORM, rule table patterns
4. Classification: self-contained / parameter-driven / rule-driven / hybrid
5. Properties + propagation to programs via Uses edges
6. Tests

### Session CI-13: Cost Estimation

1. Expression evaluator for user-modelable formulas
2. Default formula set (shipped out of the box)
3. cost_config.json read/write
4. CostEstimatePass with computed multipliers (unbounded)
5. Portfolio aggregation by process, wave, total
6. Risk factor surfacing
7. `estimate-cost` verb
8. NexStudio: Cost Settings dialog + cost summary card

### Session CI-14: Integration + Polish

1. Connect cobol-data to cobol-intel (file stats from data intelligence)
2. NexStudio process view + cost report export
3. End-to-end test with CardDemo

### Session CI-15: JCL Support

1. cobol-jcl crate: hand-written JCL parser
2. Job/Step/DD extraction
3. Dataset -> FD mapping
4. JCL-enhanced process discovery (proven links, not guessed)
5. NexStudio: JCL file viewing + job visualization

---

## Open Questions

1. Should process discovery produce a "draft" that humans review in NexStudio?
2. Should the cost model account for learning curve (first wave takes longer)?
3. How do we validate cost estimates against real engagement data?
4. For rule-driven programs: attempt to parse rule table schemas from SQL DDL?
5. Monster copybook: recommend decomposition as a pre-migration step?
6. JCL PROC libraries: how deep do we expand nested PROCs?
7. Formula editor: how much expression power? Just arithmetic, or also conditionals?
