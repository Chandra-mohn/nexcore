# Artifact Navigator -- Design Specification

A structured "Artifacts" view in NexStudio's left sidebar that organizes
project files by type and cross-references them. Complements the existing
file tree (which shows raw disk structure).

---

## Entry Point

The left toolbar gets a new icon button between "Project" and "Structure":

```
[FolderTree]   Project        <-- existing file tree
[Layers]       Artifacts      <-- NEW: structured artifact view
[ListTree]     Structure      <-- existing
[Database]     Data           <-- existing
...
```

When "Artifacts" is active, the left panel shows a collapsible tree
of artifact categories, each with a summary view and clickable items.

---

## Top-Level Tree

```
ARTIFACTS
  +-- Source (44)
  +-- Copybooks (35)
  +-- JCL (28)
  +-- Screens (12)
  +-- Data (8)
  +-- DSL Output (80)
```

Each category shows a count badge. Clicking a category expands it
to show sub-categories and items. Double-clicking an item opens it
in the editor (same as file tree double-click).

---

## Category Views

### 1. Source Programs

Sub-categories based on program characteristics (auto-classified
from AST analysis during audit scan):

```
Source (44)
  +-- Online (12)         programs with EXEC CICS
  |     COSGN00.cbl       Login screen handler
  |     COACTVW.cbl       Account view
  |     ...
  +-- Batch (20)          programs referenced by JCL EXEC PGM=
  |     ACCTUPD.cbl       Account update (called by NIGHTBAT)
  |     RPTGEN.cbl        Report generator
  |     ...
  +-- Subroutines (8)     programs CALLed by other programs
  |     DATECONV.cbl      Date conversion utility
  |     ...
  +-- Unclassified (4)    no CICS, not in JCL, not CALLed
        TESTPROG.cbl
        ...
```

**Item detail (on hover or single-click):**

```
+----------------------------------+
| ACCTUPD.cbl                      |
| Type: Batch program              |
| Lines: 2,450                     |
| Sections: 8                      |
| Called by JCL: NIGHTBAT STEP030  |
|               DAYBAT STEP020     |
| Calls: DATECONV, ERRHNDLR       |
| Copies: ACCT-RECORD, WS-COMMON  |
| EXEC SQL: 4 statements          |
| Audit: 89/100                    |
| Migration: T2 (transpiled)      |
+----------------------------------+
```

**Classification rules:**
- Online: program contains EXEC CICS statements
- Batch: program name appears in any JCL EXEC PGM= statement
- Subroutine: program name appears in any COBOL CALL statement
  but not in JCL EXEC PGM=
- A program can be both Batch + Subroutine (appears in JCL and CALLed)

**Data source:** Lineage graph (existing) + JCL cross-reference (new)

---

### 2. Copybooks

Sub-categories based on usage context:

```
Copybooks (35)
  +-- Data Layouts (22)    referenced in WORKING-STORAGE / LINKAGE
  |     ACCT-RECORD.cpy    used by 8 programs
  |     TRANS-RECORD.cpy   used by 5 programs
  |     ...
  +-- Screen Maps (8)      BMS symbolic maps (generated from BMS)
  |     COSGN0A.cpy        symbolic map for COSGN00.bms
  |     ...
  +-- SQL Includes (5)     EXEC SQL INCLUDE members
  |     DCLACCT.cpy        DECLARE TABLE for ACCT table
  |     ...
  +-- Unused (3)           not referenced by any program
        OLDCOPY.cpy
        ...
```

**Item detail:**

```
+----------------------------------+
| ACCT-RECORD.cpy                  |
| Type: Data layout                |
| Lines: 45                        |
| Fields: 12                       |
| Used by: 8 programs              |
|   ACCTUPD, ACCTINQ, RPTGEN,     |
|   BALCHK, AUDITLOG, ...         |
| Record length: 300 bytes         |
| Has REDEFINES: yes (2 groups)    |
| Has level-88: yes (5 values)    |
+----------------------------------+
```

**Classification rules:**
- Data Layout: COPY statement appears in WORKING-STORAGE, LOCAL-STORAGE,
  or LINKAGE SECTION
- Screen Map: copybook name matches a BMS map name pattern (xxxnnA, xxxnnB)
  or is referenced after EXEC CICS RECEIVE MAP
- SQL Include: referenced via EXEC SQL INCLUDE
- Unused: not referenced by any program in the project

**Data source:** Lineage graph (existing COPY edges) + BMS cross-reference

---

### 3. JCL

Sub-categories: Jobs vs Procs, with cross-reference data.

```
JCL (28)
  +-- Jobs (18)
  |     NIGHTBAT.jcl       6 steps, calls EXTRACT + PROCESS
  |     DAYBAT.jcl         4 steps, calls EXTRACT
  |     ACCTFILE.jcl       3 steps, no proc calls
  |     ...
  +-- Procs (10)
        EXTRACT.jcl        3 steps, called by NIGHTBAT + DAYBAT
        PROCESS.jcl        3 steps, called by NIGHTBAT
        SORTPROC.jcl       2 steps, called by EXTRACT
        ...
```

**Item detail:**

```
+----------------------------------+
| NIGHTBAT.jcl                     |
| Type: Job                        |
| Class: A                         |
| Steps: 6                         |
| Proc calls: EXTRACT, PROCESS    |
| Programs: IEFBR14, RPTGEN,      |
|           ABNDHLR                |
| Input datasets: 3               |
|   PROD.TRANS.DAILY (SHR)        |
|   PROD.ACCT.MASTER (OLD)        |
|   PROD.TRANS.VALID (SHR)        |
| Output datasets: 5              |
|   PROD.WORK.SCRATCH (NEW)       |
|   PROD.ACCT.UPDATED (NEW)       |
|   ...                            |
| Conditions: 1 IF/ELSE           |
| .proc DSL: generated            |
+----------------------------------+
```

**Proc detail:**

```
+----------------------------------+
| EXTRACT.jcl                      |
| Type: Procedure                  |
| Steps: 3                         |
| Called by: NIGHTBAT, DAYBAT     |
| Sub-proc calls: SORTPROC        |
| Programs: DSNTEP2 [DB2],        |
|           VALIDATE               |
| Symbolic params: none            |
+----------------------------------+
```

**Cross-reference features:**
- Job -> Procs it calls (outgoing edges)
- Proc -> Jobs that call it (incoming edges)
- Job -> Programs it executes (EXEC PGM= across all steps + procs)
- Job -> Datasets it reads/writes (all DDs across all steps)
- Shared datasets: highlight DSNs that appear in multiple jobs

**Data source:** JCL parse + proc resolution (existing in cobol-jcl)

---

### 4. Screens

BMS map definitions with field inventory and program cross-references.

```
Screens (12)
  +-- Maps
        COSGN00.bms    Login           8 fields, used by COSGN00.cbl
        COACTVW.bms    Account View   15 fields, used by COACTVW.cbl + COACTU.cbl
        COCRDLI.bms    Credit List    22 fields, used by COCRDLI.cbl
        COTRN00.bms    Transaction    18 fields, used by COTRN00.cbl
        ...
```

**Item detail:**

```
+----------------------------------+
| COACTVW.bms                      |
| Maps: 1 (COACTVWA)              |
| Screen size: 24x80              |
| Title: "Account Activity View"  |
|                                  |
| Fields: 15 total                 |
|   Input: 2 (ACCTID, ACTION)     |
|   Display: 13                    |
|   Hidden: 0                      |
| Focus: ACCTID                   |
|                                  |
| Actions:                         |
|   ENTER -> Submit                |
|   F3 -> Exit                     |
|   F7 -> Previous page            |
|   F8 -> Next page                |
|                                  |
| Used by programs:                |
|   COACTVW.cbl (SEND + RECEIVE)  |
|   COACTU.cbl (SEND only)        |
|                                  |
| .screen DSL: generated           |
| .schema DSL: generated           |
+----------------------------------+
```

**Screen navigation graph (future, from C2.10-C2.12):**
When CICS screen emission is complete, show screen-to-screen
navigation: Login -> Menu -> Account View -> Transaction Detail.

**Data source:** BMS parse (existing) + bms_references cross-ref (existing)

---

### 5. Data Files

Binary datasets with copybook matching status (existing DataExplorer,
promoted to artifact view).

```
Data (8)
  +-- Matched (6)          copybook assigned
  |     ACCT.MASTER.dat    ACCT-RECORD.cpy, 15,000 records
  |     TRANS.DAILY.dat    TRANS-RECORD.cpy, 42,000 records
  |     ...
  +-- Unmatched (2)        no copybook yet
        ARCHIVE.dat        unknown layout
        TEMP.WORK.dat      unknown layout
```

**Item detail:** Same as existing DataViewer discovery panel.

**Data source:** Discovery cache (existing in cobol-data)

---

### 6. DSL Output

Generated Nexflow DSL files grouped by grammar type.
Shows generation status per source artifact.

```
DSL Output (80)
  +-- .schema (20)         from E1 SchemaEmitter
  |     acctupd.schema     from ACCTUPD.cbl (confidence: 0.95)
  |     ...
  +-- .xform (18)          from E2 TransformEmitter
  |     acctupd_main.xform from ACCTUPD.cbl MAIN section
  |     ...
  +-- .rules (12)          from E3 RulesEmitter
  |     acctupd_validate.rules from ACCTUPD.cbl
  |     ...
  +-- .proc (18)           from E4 ProcessEmitter + JCL proc_gen
  |     acctupd.proc       from ACCTUPD.cbl (E4)
  |     nightbat.proc      from NIGHTBAT.jcl (JCL proc_gen)
  |     ...
  +-- .screen (12)         from BMS screen_gen
  |     cosgn0a.screen     from COSGN00.bms
  |     ...
  +-- .api (0)             future (from CICS LINK/XCTL/WEB)
  +-- .service (0)         future (from CICS command sequences)
```

**Item detail:**

```
+----------------------------------+
| nightbat.proc                    |
| Source: NIGHTBAT.jcl             |
| Generator: JCL proc_gen         |
| Confidence: 1.0 (skeleton)      |
| Pending enrichment:              |
|   - SORT control cards (J4.1)   |
|   - PGM cross-ref (J4.4)        |
|   - COND guards (J4.6)          |
+----------------------------------+
```

**Data source:** DSL file scan + generation metadata from emitters

---

## Data Model

### ArtifactIndex (Rust struct, cached per project)

```rust
struct ArtifactIndex {
    source: SourceInventory,
    copybooks: CopybookInventory,
    jcl: JclInventory,
    screens: ScreenInventory,
    data: DataInventory,
    dsl: DslInventory,
    cross_refs: CrossRefIndex,
}

struct SourceInventory {
    online: Vec<ProgramSummary>,
    batch: Vec<ProgramSummary>,
    subroutines: Vec<ProgramSummary>,
    unclassified: Vec<ProgramSummary>,
}

struct ProgramSummary {
    path: String,
    name: String,
    lines: usize,
    sections: usize,
    has_cics: bool,
    has_sql: bool,
    calls: Vec<String>,       // programs this one CALLs
    copies: Vec<String>,      // copybooks this one COPYs
    called_by_jcl: Vec<String>,  // JCL steps that EXEC PGM= this
    audit_score: Option<u8>,
    migration_tier: Option<String>,
}

struct JclInventory {
    jobs: Vec<JclJobSummary>,
    procs: Vec<JclProcSummary>,
}

struct JclJobSummary {
    path: String,
    name: String,
    class: Option<String>,
    step_count: usize,
    proc_calls: Vec<String>,
    programs: Vec<String>,
    input_datasets: Vec<String>,
    output_datasets: Vec<String>,
    has_conditions: bool,
}

struct JclProcSummary {
    path: String,
    name: String,
    step_count: usize,
    called_by: Vec<String>,    // job names
    sub_proc_calls: Vec<String>,
    programs: Vec<String>,
}

struct ScreenInventory {
    maps: Vec<ScreenSummary>,
}

struct ScreenSummary {
    path: String,
    mapset_name: String,
    map_names: Vec<String>,
    title: Option<String>,
    field_count: usize,
    input_count: usize,
    display_count: usize,
    actions: Vec<String>,
    used_by: Vec<String>,     // program names
}

struct CrossRefIndex {
    // Dataset -> programs/jobs that read it
    dataset_readers: HashMap<String, Vec<String>>,
    // Dataset -> programs/jobs that write it
    dataset_writers: HashMap<String, Vec<String>>,
    // Program -> JCL jobs that execute it
    program_to_jobs: HashMap<String, Vec<String>>,
    // Proc -> JCL jobs that call it
    proc_to_jobs: HashMap<String, Vec<String>>,
    // Copybook -> programs that COPY it
    copybook_to_programs: HashMap<String, Vec<String>>,
}
```

---

## Tauri Commands

```
scan_artifact_index(path) -> ArtifactIndex
    Scans the project directory, parses all recognized file types,
    builds cross-reference index. Caches result. ~2-5 seconds for
    a 50-program project.

get_artifact_detail(path, kind) -> ArtifactDetail
    Returns detailed info for a single artifact (on click).
    Uses cached index + lazy parsing for detail fields.
```

---

## Implementation Phases

### Phase 1: Index + JCL/Screen summaries (highest value)

- scan_artifact_index command (Rust)
- ArtifactNavigator.svelte (tree + category panels)
- JCL inventory (jobs, procs, cross-refs)
- Screen inventory (maps, fields, program refs)
- Wire into left toolbar as new "Artifacts" panel

### Phase 2: Source classification + Copybook grouping

- Classify programs (online/batch/subroutine) using lineage + JCL index
- Group copybooks by usage (data/screen/sql/unused)
- Program detail panel with cross-references

### Phase 3: DSL output tracking

- Scan generated DSL files and link to source artifacts
- Show generation status (generated/pending/stale)
- Confidence scores from emitter metadata

### Phase 4: Cross-reference overlays

- Dataset lineage: which jobs share datasets (input of one = output of another)
- Program call chains: COBOL CALL graph + JCL EXEC graph unified
- Screen navigation flow: XCTL/RETURN TRANSID chains

---

## UX Notes

- Artifact tree uses same styling as FileTree (indent, icons, badges)
- Single-click: show detail tooltip/panel on the right
- Double-click: open in editor (same as file tree)
- Counts update when project is re-scanned
- Badge colors match existing conventions:
  - Blue: normal
  - Green: migrated/generated
  - Orange: warnings
  - Red: errors/unresolved
- Category headers are collapsible (click to expand/collapse)
- Search/filter within each category
