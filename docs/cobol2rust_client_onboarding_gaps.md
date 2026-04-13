# Client Onboarding Gaps

Product gaps discovered during first real client codebase engagement.
Environment: ~1500 COBOL files + 800 copybooks, ~2M total lines, disk-constrained Linux server.

Date: 2026-04-01

---

## Summary

Running cobol2rust against a real enterprise COBOL codebase revealed 8 product gaps
that block or significantly impede the client onboarding and due diligence process.
These gaps fall into three priority tiers:

- **P0 (3 open, 1 fixed)**: Block onboarding entirely -- client cannot assess their codebase
- **P1 (2 open)**: Significant friction -- client resorts to shell scripts and manual analysis
- **P2 (2 open)**: Rough edges -- confusing output, undocumented workarounds

---

## P0 -- Blocking

### G1: No Pre-flight Validation Command

**Problem**: There is no single command that validates readiness before transpilation.
The client must manually piece together `check`, `preprocess`, `grep`, and shell scripts
to answer basic questions:

- Are all referenced copybooks present?
- Are all source files readable (encoding issues)?
- What COBOL dialect features are used?
- What is the expected transpilation coverage?

**Impact**: The first hours of every engagement are spent on manual discovery that the
tool should automate. This is the product's first impression.

**Recommendation**: A `cobol2rust audit` command that runs all pre-flight checks in
one pass and produces a structured report:

```
cobol2rust audit /path/to/src -C /path/to/copybooks --format json

Output:
{
  "files_scanned": 1523,
  "encoding_issues": [ { "file": "...", "line": 73, "byte": "0xC7" }, ... ],
  "missing_copybooks": [ "ABCD", "EFGH", ... ],
  "dialect_features": { "exec_sql": 6, "exec_cics": 0, "alter": 4 },
  "estimated_coverage": 0.87,
  "warnings": [ ... ],
  "ready_to_transpile": false,
  "blockers": [ "23 missing copybooks", "3 files with encoding issues" ]
}
```

### G2: Non-UTF-8 Files Crash Instead of Recovering

**Status**: FIXED (commit cde13c2, 2026-04-01)

**Problem**: All COBOL/copybook file reading used `fs::read_to_string()` which requires
valid UTF-8. Mainframe-transferred files commonly have stray EBCDIC bytes in columns 1-6
(sequence number area). These files crashed the tool instead of being processed.

**Root cause**: COBOL is a pure ASCII language. Non-ASCII bytes in source files are always
transfer artifacts, never meaningful code. The tool should tolerate them.

**Fix applied**: Replaced `fs::read_to_string` with `fs::read` + `String::from_utf8_lossy`
across 18 COBOL/copybook reading locations in cobol-cli and cobol-transpiler. Added
`cobol_read.rs` helper module to centralize the pattern.

**Files changed**: copybook.rs (transpiler), check.rs, parse_cmd.rs, transpile_cmd.rs,
preprocess.rs, diff_cmd.rs, data_analyze_cmd.rs, decode_cmd.rs, discover_cmd.rs,
workspace.rs, build_graph_cmd.rs, pipeline/phase0.rs, scan/worker.rs, scan/phase1.rs,
scan/phase2.rs, main.rs, cobol_read.rs (new).

### G3: check Command Does Not Expand COPY Statements

**Problem**: The `check` command calls `parse_cobol()` on raw source without expanding
COPY statements first. The `transpile` command properly uses `CopyExpander` before parsing.

This means `check` sees:

```cobol
WORKING-STORAGE SECTION.
    COPY ACCTFILE.
    COPY CUSTFILE.
```

Instead of the actual data definitions from those copybooks. The parser reports empty
WORKING-STORAGE, empty LINKAGE, and no PROCEDURE DIVISION structure.

**Impact**: 411 out of 416 W003 warnings ("parser extracted 0 paragraphs") are caused
by this bug. The check command is essentially useless for any codebase that uses COPY
statements -- which is every real COBOL codebase.

**Root cause**: `analyze_source()` in `analyze.rs` does not accept a `CopybookResolver`
and does not call `CopyExpander::expand()` before `parse_cobol()`.

**Fix**: Modify `analyze_source()` to optionally accept copybook search paths. When
provided, expand COPY statements before parsing. The `check` command already accepts
`-C` flags but never passes them to the analysis pipeline.

### G4: Missing Copybooks Not Reported Upfront

**Problem**: When copybooks are missing from the `-C` search paths:

- `transpile` stops at the FIRST missing copybook per file and reports an error.
  It does not continue to find all missing copybooks.
- `check` does not report missing copybooks at all (see G3 -- it skips COPY expansion).
- `preprocess` also stops at the first missing copybook per file.

**Impact**: Client had to use grep and shell scripts to cross-reference COPY targets
against available copybook files to produce a list of missing copybooks for the
mainframe team. This took significant manual effort.

**The workaround used**:

```bash
# Extract all COPY targets from source
grep -rhi "\sCOPY\s\+[A-Za-z]" /src/*.cbl | sed ... | sort -u > needed.txt
# List available copybooks
ls /copybooks/*.cpy | xargs basename | sed ... | sort -u > available.txt
# Find missing
comm -23 needed.txt available.txt > missing.txt
```

This should be a single command.

**Recommendation**: Add to the `audit` command (G1), or add a `--missing-copybooks`
flag to `check` that extracts all COPY targets and validates against the resolver
without stopping at the first failure.

---

## P1 -- Significant Friction

### G5: check Output Not Designed for Enterprise Analysis

**Problem**: The check command output is difficult to analyze at scale.

Issues discovered:

1. **Text output goes to stderr, not stdout** -- `cobol2rust check ... > file.txt`
   produces an empty file. Must use `2>` instead. Confusing for users.

2. **JSON output drops ANTLR token recognition errors** -- The text format includes
   28,000+ individual token recognition errors. The JSON format captures only the
   W001/W002/W003 summary warnings. JSON is 297KB vs text 1.5MB for the same run.
   After filtering ANTLR noise, the content is equivalent (167KB text vs 297KB JSON
   with structural overhead). So JSON is not losing warning-level data, but it IS
   losing the ANTLR token-level detail.

3. **No built-in summary or aggregation** -- User must use jq, python, or Excel to
   group results by warning code, count failures, or identify patterns.

4. **No CSV/TSV export** -- Enterprise users need spreadsheet-friendly output for
   triage meetings and stakeholder reporting.

5. **W003 warnings give no root cause** -- All 416 W003 entries have zero errors
   attached, making it impossible to distinguish between different failure modes
   without manual file-by-file investigation.

**Recommendation**:

- Add `--format csv` output option
- Add `--summary` flag for aggregated statistics
- Add `--filter W003` to focus on specific warning codes
- Structured output to stdout, progress/logs to stderr
- Include ANTLR error counts per file in JSON output

### G6: No Encoding Detection or Auto-Cleanup

**Problem**: COBOL files from mainframe transfers commonly have encoding issues:

- EBCDIC remnants in sequence number area (columns 1-6)
- Mixed encoding within files (ASCII body, non-ASCII line numbers)
- Non-printable control characters
- Files that `file` command reports as "NON-ISO extended-ASCII" or "data"

The user must use external tools (python scripts, `iconv`, `file` command) to identify
and clean encoding issues before cobol2rust can process files.

**Discovery process that should be automated**:

1. Run `file *.cbl` to find non-ASCII files
2. Write python script to find line/column of non-ASCII bytes
3. Write python script to replace non-ASCII bytes with spaces
4. Repeat for copybooks
5. Re-run cobol2rust

**Recommendation**:

- `cobol2rust audit --encoding` to scan and report encoding issues with line/column
- `cobol2rust clean` command to auto-fix common encoding issues (replace non-ASCII
  with spaces in columns 1-6, flag non-ASCII in code area for review)
- Or handle transparently in the preprocessor: the fixed-format preprocessor already
  strips columns 1-6, so non-ASCII bytes there should be silently ignored

---

## P2 -- Rough Edges

### G7: ANTLR Token Recognition Errors Not Actionable

**Problem**: The ANTLR parser produces 28,000+ "token recognition error" messages with:

- No file attribution (just line/column within the parsed stream)
- No grouping by character or pattern
- No indication of severity (benign vs blocking)
- All dumped to stderr in a wall of text

**Impact**: User cannot determine which files produce the errors, which characters
are the cause, or whether the errors actually matter. The errors may be largely
harmless (unsupported but non-critical syntax) but there is no way to tell without
manual investigation.

**Recommendation**:

- Capture ANTLR errors per-file in the analysis pipeline
- Include error counts and top error patterns per file in JSON output
- Group by offending character in summary output
- Distinguish between "lexer noise" (columns 1-6 artifacts) and "real syntax issues"

### G8: Build for Constrained Environments

**Problem**: The default build includes DuckDB (bundled C++ compilation) which requires
~4GB RAM and significant disk space. Building on a constrained Linux server required
discovering the `--no-default-features` flag through trial and error.

The `scripts/rust-dev-setup.sh` installs sccache, cargo-nextest, and cargo-watch which
are not needed for deployment and consume disk space.

**Recommendation**:

- Document minimal build instructions in README or BUILDING.md:
  ```
  # Minimal build (no DuckDB, no dev tools)
  cargo build --release -p cobol-cli --no-default-features
  ```
- Document which features require DuckDB (scan, status, report commands)
- Consider a `cobol2rust-lite` build profile or binary distribution

---

## Proposed Resolution Roadmap

### Phase 1: Unblock Client (1-2 sessions)

1. Fix G3: Add COPY expansion to check/analyze pipeline
2. Fix G4: Add missing copybook reporting (all missing, not just first)
3. Verify: Re-run check on client codebase -- W003 count should drop from 411 to near zero

### Phase 2: Enterprise Polish (2-3 sessions)

4. Build G1: `cobol2rust audit` command combining encoding check, copybook validation,
   dialect detection, and coverage estimation
5. Fix G5: Add --format csv, --summary, structured stdout/stderr
6. Fix G6: Encoding detection and auto-cleanup in audit command

### Phase 3: Quality of Life (1 session)

7. Fix G7: Per-file ANTLR error capture and grouping
8. Fix G8: Document minimal build, add BUILDING.md

---

## Appendix: Session Timeline

1. Built cobol2rust on disk-constrained Linux server
2. Discovered non-UTF-8 crash on critical 250K-line file and 3 copybooks (G2)
3. Fixed G2 with from_utf8_lossy across 18 locations
4. Ran check on full codebase: 416 W003, 6 W001, 4 W002
5. Discovered check output goes to stderr, JSON drops ANTLR detail (G5)
6. Analyzed W003: all 416 have zero errors -- silent parse failures
7. Discovered check does not expand COPY statements (G3)
8. Ran transpile on critical file: immediate "copybook not found" (G4)
9. Cross-referenced COPY targets vs available copybooks: multiple missing
10. Sent missing copybook list to client mainframe team
