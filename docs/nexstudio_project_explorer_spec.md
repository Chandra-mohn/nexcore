# NexStudio Project Explorer Spec

## Overview

Unified tree with five synthetic top-level sections. Each section maps to
real directories on disk and is presented with clean labels.

## Tree Structure

```
PROJECT: <name>
|
+-- Source                          All original mainframe artifacts
|   +-- cobol/                      Programs (.CBL, .COB)
|   +-- copybooks/                  Copybooks (.CPY)
|   +-- jcl/                        JCL scripts (if present)
|   +-- sql/                        SQL DDL/DML (if present)
|   +-- (any other source dirs)
|
+-- Target                          All generated output
|   +-- Rust/                       Phase 1 Rust (direct transpile)
|   +-- Idiomatic Rust/             Phase 2 Rust (after rustify tiers)
|   +-- DSL/                        Nexflow DSL (.schema, .xform, .rules, .proc)
|   +-- Java/                       Java output (future, --target java)
|   +-- Idiomatic Java/             Refined Java (future)
|
+-- Reports                         Human-readable assessments and manifests
|   +-- manifest.json               Rustify manifest (checksums, rules applied)
|   +-- review.md                   Tier 2+ safety review items
|   +-- plan.md                     Tier 3 structural assessment
|   +-- dsl_manifest.json           DSL generation summary
|   +-- scan_meta.json              Enterprise scan metadata
|
+-- Data                            User's mainframe data files
|   +-- CUSTFILE.DAT                EBCDIC data
|   +-- ACCTFILE.VSAM               VSAM extract
|   +-- transactions.csv            ASCII export
|   +-- (any user-provided data)
|
+-- NexIntel                        Analytical output from NexIntel
    +-- fields.ndjson               Field catalog
    +-- field_access.ndjson         Who reads/writes what
    +-- clusters.ndjson             Business concept groups
    +-- functions.ndjson            Input -> process -> output mapping
    +-- copybook_usage.ndjson       Cross-program sharing
    +-- coverage.ndjson             Transpile coverage stats
```

## Directory Mapping

| Explorer Label         | Actual Directory       | Created When         |
|------------------------|------------------------|----------------------|
| Source                 | source.repoPath        | Project creation     |
| Target / Rust          | target/                | After transpile      |
| Target / Idiomatic Rust| target-rustified/      | After rustify        |
| Target / DSL           | dsl/                   | After --emit-dsl     |
| Target / Java          | target-java/           | After --target java  |
| Reports                | rustify/ + dsl/        | Aggregated from both |
| Data                   | data/                  | User-managed         |
| NexIntel               | nexintel/              | After analysis runs  |

## Behavior

### Empty/Missing Sections
- If a target sub-section doesn't exist yet, show it greyed with hint text:
  "Run pipeline to generate"
- Data section is always visible (user may add files before running pipeline)
- NexIntel section appears after first analysis run

### Click Behavior
| Click on...              | Editor shows                              |
|--------------------------|-------------------------------------------|
| COBOL source file        | COBOL (left) + Phase 1 Rust (right)       |
| Phase 1 .rs file         | Phase 1 Rust (left) + Phase 2 Rust (right)|
| .schema / .proc / etc.   | DSL file (full width, syntax highlighted) |
| Report file              | Markdown/JSON viewer (full width)         |
| Data file (EBCDIC)       | Hex viewer or decoded table view          |
| Data file (CSV)          | Table viewer                              |
| NexIntel .ndjson file    | Table view with DuckDB query bar          |

### Context Menu
- Source files: "Transpile", "View AST", "Parse"
- Target files: "Compare with Source", "Rustify", "Emit DSL"
- DSL files: "Validate", "View Source COBOL"
- Data files: "Open in Table View", "Export as CSV"
- NexIntel files: "Query with DuckDB", "Visualize"

## Product Mapping

The five explorer sections map directly to the Nex product portfolio:

| Explorer Section | Product   | Role                                    |
|------------------|-----------|-----------------------------------------|
| Source           | NexMig    | Migration input (COBOL, copybooks, JCL) |
| Target           | NexMod    | Modernization output (Rust, Java, DSL)  |
| Reports          | NexMig    | Migration quality and status tracking   |
| Data             | --        | User's mainframe data (migration input) |
| NexIntel         | NexIntel  | Intelligence: lineage, clusters, graphs |

### Data vs NexIntel

- **Data** = user-provided mainframe data files participating in migration.
  EBCDIC files, VSAM extracts, CSV exports. These are the raw artifacts
  the user brings from the mainframe. User-managed, never generated.

- **NexIntel** = analytical output produced by the intelligence engine.
  Lineage, field catalogs, cluster discovery, coverage stats, cross-program
  analysis. Machine-generated, queryable via DuckDB.

NexIntel reads from both Source (COBOL AST) and Data (actual data shapes)
to produce its analytical output. The separation keeps user data clean
and NexIntel output clearly identified as generated artifacts.

## Performance: Large-Scale Discovery

Current `count_lines_batch` is single-threaded `fs::read_to_string` + line
count. Fine for 1,500 files (~3s). Too slow for 250K files (~60s).

**When to optimize**: When targeting 50K+ file codebases.

**Fix**: Parallelize with rayon + memchr byte-level newline counting:
```rust
paths.par_iter().map(|p| {
    let bytes = fs::read(p).unwrap_or_default();
    memchr::memchr_iter(b'\n', &bytes).count()
})
```
Expected: 250K files / 30M lines in ~5 seconds (matching scc performance).

**Note**: Discovery only runs once per project (first open). Subsequent opens
restore from `nexintel/pipeline_files.ndjson` (instant at any scale). So this
optimization only affects first-time project setup.

Also expand `discover_recursive` extensions from `["cbl", "cob"]` to include
`["cpy", "jcl", "sql", "prc"]` when those migration paths are added.

## Design Principles

1. Five sections, each with clear purpose:
   Source = what you have, Target = what you're building,
   Reports = how it went, Data = your data, NexIntel = what we learned
2. No hidden directories -- everything visible in the explorer
3. No badges -- keep the tree clean and simple
4. Labels are clean display names, not directory names
5. Grey out what doesn't exist yet (shows the full pipeline ahead)
6. Data is strictly user-managed; NexIntel is strictly machine-generated
