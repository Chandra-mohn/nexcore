#!/usr/bin/env bash
# =============================================================================
# debug-monster.sh -- Diagnose why a large COBOL file produces tiny Rust output
#
# Usage:
#   ./scripts/debug-monster.sh <COBOL_FILE> [COPYBOOK_FLAGS...]
#
# Example:
#   ./scripts/debug-monster.sh /path/to/SSUPDATE.cbl -f fixed \
#       -C /path/to/code-copybooks -C /path/to/layout-copybooks
#
# Produces: /tmp/monster-debug/report.txt  (full analysis)
# =============================================================================
set -euo pipefail

# -- Configuration -----------------------------------------------------------
CR="${CR:-$(dirname "$0")/../RustBuild/cobol2rust-target/release/cobol2rust}"
OUTDIR="/tmp/monster-debug"
INPUT="${1:?Usage: $0 <COBOL_FILE> [FLAGS...]}"
shift
EXTRA_FLAGS=("$@")
BASENAME=$(basename "$INPUT" .cbl)
BASENAME=$(basename "$BASENAME" .CBL)
BASENAME=$(basename "$BASENAME" .cob)

if [ ! -f "$INPUT" ]; then
    echo "ERROR: File not found: $INPUT"
    exit 1
fi
if [ ! -x "$CR" ]; then
    echo "ERROR: CLI not found at: $CR"
    echo "Build with: cargo build -p cobol-cli --release"
    exit 1
fi

mkdir -p "$OUTDIR"
REPORT="$OUTDIR/report.txt"
> "$REPORT"

log() { echo "$@" | tee -a "$REPORT"; }
logf() { printf "$@" | tee -a "$REPORT"; }
ruler() { log "$(printf '=%.0s' {1..72})"; }

ruler
log "MONSTER FILE DEBUG REPORT"
log "File: $INPUT"
log "Date: $(date '+%Y-%m-%d %H:%M:%S')"
log "CLI:  $CR"
log "Flags: ${EXTRA_FLAGS[*]:-none}"
ruler
echo ""

# =============================================================================
# PHASE 1: Raw source metrics
# =============================================================================
log ""
log "== PHASE 1: RAW SOURCE METRICS =="
log ""

TOTAL_LINES=$(wc -l < "$INPUT")
RAW_BYTES=$(wc -c < "$INPUT")
BLANK_LINES=$(grep -c '^[[:space:]]*$' "$INPUT" || true)
COMMENT_LINES=$(grep -cE '^.{6}\*' "$INPUT" || true)
CODE_LINES=$((TOTAL_LINES - BLANK_LINES - COMMENT_LINES))

log "Total lines:    $TOTAL_LINES"
log "Blank lines:    $BLANK_LINES"
log "Comment lines:  $COMMENT_LINES"
log "Code lines:     $CODE_LINES"
log "File size:      $RAW_BYTES bytes"

# Count COPY statements
COPY_COUNT=$(grep -ciE '^\s.{5}\s.*COPY ' "$INPUT" || true)
log "COPY statements: $COPY_COUNT"

# Division boundaries
log ""
log "-- Division boundaries (line numbers) --"
grep -niE '^\s.{5}\s.*(IDENTIFICATION|ENVIRONMENT|DATA|PROCEDURE)\s+DIVISION' "$INPUT" \
    | head -10 | tee -a "$REPORT" || log "(none found)"

# Section counts in DATA DIVISION
log ""
log "-- DATA DIVISION sections --"
FILE_SEC_CT=$(grep -ciE 'FILE\s+SECTION' "$INPUT" || true)
WS_SEC_CT=$(grep -ciE 'WORKING-STORAGE\s+SECTION' "$INPUT" || true)
LINK_SEC_CT=$(grep -ciE 'LINKAGE\s+SECTION' "$INPUT" || true)
LOCAL_SEC_CT=$(grep -ciE 'LOCAL-STORAGE\s+SECTION' "$INPUT" || true)
log "  FILE SECTION markers:    ${FILE_SEC_CT:-0}"
log "  WORKING-STORAGE markers: ${WS_SEC_CT:-0}"
log "  LINKAGE SECTION markers: ${LINK_SEC_CT:-0}"
log "  LOCAL-STORAGE markers:   ${LOCAL_SEC_CT:-0}"

# Count 01-levels in raw source
LEVEL01_COUNT=$(grep -cE '^\s.{5}\s+01\s' "$INPUT" || true)
log "  Raw 01-level items: $LEVEL01_COUNT"

# =============================================================================
# PHASE 2: Preprocessed (COPY-expanded) metrics
# =============================================================================
log ""
log "== PHASE 2: PREPROCESSED (COPY-EXPANDED) SOURCE =="
log ""

EXPANDED="$OUTDIR/${BASENAME}_expanded.cbl"
log "Expanding COPY statements..."
if "$CR" preprocess "${EXTRA_FLAGS[@]}" "$INPUT" -o "$EXPANDED" 2>"$OUTDIR/preprocess_stderr.txt"; then
    EXP_LINES=$(wc -l < "$EXPANDED")
    EXP_BLANK=$(grep -c '^[[:space:]]*$' "$EXPANDED" || true)
    EXP_CODE=$((EXP_LINES - EXP_BLANK))
    EXPANSION_RATIO=$(echo "scale=1; $EXP_LINES / $TOTAL_LINES" | bc 2>/dev/null || echo "?")

    log "Expanded lines:  $EXP_LINES"
    log "Expanded code:   $EXP_CODE (non-blank)"
    log "Expansion ratio: ${EXPANSION_RATIO}x"

    # Count duplicate section markers (the key hypothesis)
    log ""
    log "-- Duplicate section markers in expanded source --"
    for MARKER in "FILE SECTION" "WORKING-STORAGE SECTION" "LINKAGE SECTION" "PROCEDURE DIVISION"; do
        COUNT=$(grep -ciE "$MARKER" "$EXPANDED" || true)
        logf "  %-30s %d\n" "$MARKER:" "$COUNT"
    done

    # Count 01-levels in expanded source
    EXP_01=$(grep -cE '^\s*01\s' "$EXPANDED" || true)
    log ""
    log "  Expanded 01-level items: $EXP_01"

    # DATA vs PROC size
    log ""
    log "-- Division sizes in expanded source --"
    PROC_LINE=$(grep -niE 'PROCEDURE\s+DIVISION' "$EXPANDED" | head -1 | cut -d: -f1 || echo "0")
    WS_LINE=$(grep -niE 'WORKING-STORAGE\s+SECTION' "$EXPANDED" | head -1 | cut -d: -f1 || echo "0")
    if [ "$PROC_LINE" -gt 0 ] && [ "$WS_LINE" -gt 0 ]; then
        DATA_SIZE=$((PROC_LINE - WS_LINE))
        PROC_SIZE=$((EXP_LINES - PROC_LINE))
        log "  DATA DIVISION (WS to PROC):    ~$DATA_SIZE lines (from line $WS_LINE)"
        log "  PROCEDURE DIVISION:            ~$PROC_SIZE lines (from line $PROC_LINE)"
    fi

    # Count EXEC CICS / EXEC SQL in expanded
    log ""
    log "-- Embedded SQL/CICS in expanded source --"
    EXEC_SQL=$(grep -ciE 'EXEC\s+SQL' "$EXPANDED" || true)
    EXEC_CICS=$(grep -ciE 'EXEC\s+CICS' "$EXPANDED" || true)
    EXEC_DLI=$(grep -ciE 'EXEC\s+DLI' "$EXPANDED" || true)
    log "  EXEC SQL:  $EXEC_SQL"
    log "  EXEC CICS: $EXEC_CICS"
    log "  EXEC DLI:  $EXEC_DLI"

    # Count common COBOL verbs in PROCEDURE DIVISION
    if [ "$PROC_LINE" -gt 0 ]; then
        log ""
        log "-- Verb inventory in PROCEDURE DIVISION --"
        PROC_TEXT="$OUTDIR/${BASENAME}_proc_only.txt"
        tail -n +"$PROC_LINE" "$EXPANDED" > "$PROC_TEXT"
        for VERB in MOVE ADD SUBTRACT MULTIPLY DIVIDE COMPUTE PERFORM IF EVALUATE \
                    DISPLAY READ WRITE OPEN CLOSE CALL GO STRING UNSTRING INSPECT \
                    SEARCH SET INITIALIZE ACCEPT SORT MERGE STOP EXIT GOBACK \
                    CONTINUE NEXT ALTER; do
            COUNT=$(grep -cwi "$VERB" "$PROC_TEXT" || true)
            if [ "$COUNT" -gt 0 ]; then
                logf "  %-15s %6d\n" "$VERB" "$COUNT"
            fi
        done
        rm -f "$PROC_TEXT"
    fi
else
    log "WARNING: Preprocessing failed. Stderr:"
    cat "$OUTDIR/preprocess_stderr.txt" | head -20 | tee -a "$REPORT"
    EXPANDED="$INPUT"  # Fall back to raw source
fi

# =============================================================================
# PHASE 3: Parse AST metrics
# =============================================================================
log ""
log "== PHASE 3: AST PARSE RESULTS =="
log ""

# Parse to JSON
AST_JSON="$OUTDIR/${BASENAME}_ast.json"
log "Parsing to JSON AST..."
if "$CR" parse "${EXTRA_FLAGS[@]}" "$INPUT" --format json -o "$AST_JSON" 2>"$OUTDIR/parse_stderr.txt"; then
    # Count data entries
    if command -v jq &>/dev/null; then
        WS_FIELDS=$(jq '[.. | objects | select(.level != null)] | length' "$AST_JSON" 2>/dev/null || echo "?")
        FD_COUNT=$(jq '.data_division.file_section | length' "$AST_JSON" 2>/dev/null || echo "?")
        PARA_COUNT=$(jq '.procedure_division.paragraphs | length' "$AST_JSON" 2>/dev/null || echo "?")
        SECT_COUNT=$(jq '.procedure_division.sections | length' "$AST_JSON" 2>/dev/null || echo "?")
        LINKAGE_COUNT=$(jq '.data_division.linkage | length' "$AST_JSON" 2>/dev/null || echo "?")

        log "AST data fields:    $WS_FIELDS"
        log "File descriptions:  $FD_COUNT"
        log "Linkage entries:    $LINKAGE_COUNT"
        log "Sections:           $SECT_COUNT"
        log "Paragraphs:         $PARA_COUNT"

        # List paragraph names
        if [ "$PARA_COUNT" != "?" ] && [ "$PARA_COUNT" != "0" ] && [ "$PARA_COUNT" != "null" ]; then
            log ""
            log "-- Paragraph names --"
            jq -r '.procedure_division.paragraphs[]?.name // empty' "$AST_JSON" 2>/dev/null \
                | head -50 | tee -a "$REPORT"
        fi

        # List section names
        if [ "$SECT_COUNT" != "?" ] && [ "$SECT_COUNT" != "0" ] && [ "$SECT_COUNT" != "null" ]; then
            log ""
            log "-- Section names --"
            jq -r '.procedure_division.sections[]?.name // empty' "$AST_JSON" 2>/dev/null \
                | head -20 | tee -a "$REPORT"
        fi
    else
        log "(jq not installed -- skipping JSON analysis)"
        log "Install with: brew install jq"
    fi

    # Show parse warnings
    PARSE_WARNS=$(wc -l < "$OUTDIR/parse_stderr.txt" | tr -d ' ')
    if [ "$PARSE_WARNS" -gt 0 ]; then
        log ""
        log "-- Parse warnings ($PARSE_WARNS lines) --"
        head -30 "$OUTDIR/parse_stderr.txt" | tee -a "$REPORT"
        if [ "$PARSE_WARNS" -gt 30 ]; then
            log "  ... ($((PARSE_WARNS - 30)) more lines, see $OUTDIR/parse_stderr.txt)"
        fi
    fi
else
    log "WARNING: Parse failed. Stderr:"
    cat "$OUTDIR/parse_stderr.txt" | head -20 | tee -a "$REPORT"
fi

# =============================================================================
# PHASE 4: Transpile and measure output
# =============================================================================
log ""
log "== PHASE 4: TRANSPILE OUTPUT ANALYSIS =="
log ""

RUST_OUT="$OUTDIR/${BASENAME}.rs"
log "Transpiling..."
if "$CR" transpile "${EXTRA_FLAGS[@]}" "$INPUT" -o "$RUST_OUT" 2>"$OUTDIR/transpile_stderr.txt"; then
    RUST_LINES=$(wc -l < "$RUST_OUT")
    RUST_BYTES=$(wc -c < "$RUST_OUT")

    log "Rust output:     $RUST_LINES lines, $RUST_BYTES bytes"
    log "Compression:     $TOTAL_LINES COBOL -> $RUST_LINES Rust ($(echo "scale=2; $RUST_LINES * 100 / $TOTAL_LINES" | bc 2>/dev/null || echo "?")%)"

    # Count generated constructs
    STRUCT_FIELDS=$(grep -c 'pub ' "$RUST_OUT" || true)
    FUNCTIONS=$(grep -c '^fn \|^pub fn ' "$RUST_OUT" || true)
    COBOL_ATTRS=$(grep -c '#\[cobol(' "$RUST_OUT" || true)
    MOVE_CALLS=$(grep -c 'cobol_move\|move_numeric\|move_alphanumeric' "$RUST_OUT" || true)
    ADD_CALLS=$(grep -c 'cobol_add' "$RUST_OUT" || true)
    COMPUTE_CALLS=$(grep -c 'cobol_compute' "$RUST_OUT" || true)
    PERFORM_CALLS=$(grep -cE '^\s+(n[0-9]|para_|main_)' "$RUST_OUT" || true)
    DISPLAY_CALLS=$(grep -c 'print!' "$RUST_OUT" || true)
    FILE_OPS=$(grep -c 'open\|close\|read_next\|write_record' "$RUST_OUT" || true)
    EMPTY_FNS=$(grep -cE '^\s*return;\s*$' "$RUST_OUT" || true)

    log ""
    log "-- Generated Rust constructs --"
    logf "  %-25s %6d\n" "pub fields:" "$STRUCT_FIELDS"
    logf "  %-25s %6d\n" "functions:" "$FUNCTIONS"
    logf "  %-25s %6d\n" "#[cobol()] attributes:" "$COBOL_ATTRS"
    logf "  %-25s %6d\n" "MOVE calls:" "$MOVE_CALLS"
    logf "  %-25s %6d\n" "ADD calls:" "$ADD_CALLS"
    logf "  %-25s %6d\n" "COMPUTE calls:" "$COMPUTE_CALLS"
    logf "  %-25s %6d\n" "PERFORM/call stmts:" "$PERFORM_CALLS"
    logf "  %-25s %6d\n" "DISPLAY (print!) calls:" "$DISPLAY_CALLS"
    logf "  %-25s %6d\n" "File I/O ops:" "$FILE_OPS"
    logf "  %-25s %6d\n" "Bare return statements:" "$EMPTY_FNS"

    # Show first 5 and last 5 functions
    log ""
    log "-- Function signatures (first 10) --"
    grep -n '^fn \|^pub fn ' "$RUST_OUT" | head -10 | tee -a "$REPORT"
    log ""
    log "-- Function signatures (last 10) --"
    grep -n '^fn \|^pub fn ' "$RUST_OUT" | tail -10 | tee -a "$REPORT"

    # Function body sizes (lines per function)
    log ""
    log "-- Function body sizes (lines between fn signatures) --"
    grep -n '^fn \|^pub fn ' "$RUST_OUT" | awk -F: '
        NR > 1 { size = $1 - prev - 1; printf("  %-40s %5d lines\n", prev_name, size) }
        { prev = $1; prev_name = $2 }
        END { printf("  %-40s %5d lines (to EOF)\n", prev_name, 999) }
    ' | head -30 | tee -a "$REPORT"
else
    log "WARNING: Transpile failed. Stderr:"
    cat "$OUTDIR/transpile_stderr.txt" | head -20 | tee -a "$REPORT"
fi

# Show transpile warnings
TRANS_WARNS=$(wc -l < "$OUTDIR/transpile_stderr.txt" | tr -d ' ')
if [ "$TRANS_WARNS" -gt 0 ]; then
    log ""
    log "-- Transpile diagnostics ($TRANS_WARNS lines) --"
    head -30 "$OUTDIR/transpile_stderr.txt" | tee -a "$REPORT"
    if [ "$TRANS_WARNS" -gt 30 ]; then
        log "  ... ($((TRANS_WARNS - 30)) more lines, see $OUTDIR/transpile_stderr.txt)"
    fi
fi

# =============================================================================
# PHASE 5: Coverage analysis
# =============================================================================
log ""
log "== PHASE 5: TRANSPILE COVERAGE =="
log ""

if "$CR" check "${EXTRA_FLAGS[@]}" --coverage --format json "$INPUT" \
    > "$OUTDIR/${BASENAME}_coverage.json" 2>"$OUTDIR/coverage_stderr.txt"; then
    if command -v jq &>/dev/null; then
        COV_JSON="$OUTDIR/${BASENAME}_coverage.json"
        # Coverage data is under .files[0].coverage
        TOTAL_STMTS=$(jq '.files[0].coverage.total_statements // 0' "$COV_JSON" 2>/dev/null || echo "?")
        MAPPED_STMTS=$(jq '.files[0].coverage.mapped_statements // 0' "$COV_JSON" 2>/dev/null || echo "?")
        DATA_ENTRIES=$(jq '.files[0].coverage.total_data_entries // 0' "$COV_JSON" 2>/dev/null || echo "?")
        COV_PCT=$(jq '.files[0].coverage.coverage_pct // 0' "$COV_JSON" 2>/dev/null || echo "?")
        PARA_INFO=$(jq '.files[0].info.paragraphs // 0' "$COV_JSON" 2>/dev/null || echo "?")
        FILE_OPS_INFO=$(jq '.files[0].info.file_ops // 0' "$COV_JSON" 2>/dev/null || echo "?")
        SQL_INFO=$(jq '.files[0].info.sql_statements // 0' "$COV_JSON" 2>/dev/null || echo "?")

        log "Total statements:  $TOTAL_STMTS"
        log "Mapped statements: $MAPPED_STMTS"
        log "Coverage:          ${COV_PCT}%"
        log "Data entries:      $DATA_ENTRIES"
        log "Paragraphs:        $PARA_INFO"
        log "File operations:   $FILE_OPS_INFO"
        log "SQL statements:    $SQL_INFO"

        # Unhandled verbs
        UNHANDLED=$(jq -r '.files[0].coverage.unhandled[]? // empty' "$COV_JSON" 2>/dev/null)
        if [ -n "$UNHANDLED" ]; then
            log ""
            log "-- Unhandled statements --"
            echo "$UNHANDLED" | sort | uniq -c | sort -rn | head -20 | tee -a "$REPORT"
        fi

        # Errors and warnings
        ERRORS=$(jq '.files[0].errors | length' "$COV_JSON" 2>/dev/null || echo "0")
        WARNINGS=$(jq '.files[0].warnings | length' "$COV_JSON" 2>/dev/null || echo "0")
        if [ "$ERRORS" -gt 0 ] || [ "$WARNINGS" -gt 0 ]; then
            log ""
            log "-- Check errors: $ERRORS, warnings: $WARNINGS --"
            jq -r '.files[0].errors[]?, .files[0].warnings[]?' "$COV_JSON" 2>/dev/null \
                | head -20 | tee -a "$REPORT"
        fi
    fi
else
    log "WARNING: Coverage check failed."
    head -10 "$OUTDIR/coverage_stderr.txt" | tee -a "$REPORT"
fi

# =============================================================================
# PHASE 6: Verb diagnosis (if available)
# =============================================================================
log ""
log "== PHASE 6: VERB DIAGNOSIS =="
log ""

if "$CR" diagnose verbs "${EXTRA_FLAGS[@]}" "$INPUT" \
    > "$OUTDIR/${BASENAME}_verbs.txt" 2>"$OUTDIR/verbs_stderr.txt"; then
    cat "$OUTDIR/${BASENAME}_verbs.txt" | head -50 | tee -a "$REPORT"
else
    log "WARNING: Verb diagnosis failed."
    head -5 "$OUTDIR/verbs_stderr.txt" | tee -a "$REPORT"
fi

# =============================================================================
# PHASE 7: Hypothesis validation
# =============================================================================
log ""
ruler
log "== HYPOTHESIS VALIDATION =="
ruler
log ""

log "H1: DATA DIVISION field loss"
log "   Expected 01-levels (raw):      $LEVEL01_COUNT"
log "   Expected 01-levels (expanded): ${EXP_01:-?}"
log "   AST parsed fields:             ${WS_FIELDS:-?}"
log "   Rust pub fields:               ${STRUCT_FIELDS:-?}"
if [ "${STRUCT_FIELDS:-0}" -lt "${EXP_01:-1}" ] 2>/dev/null; then
    log "   --> CONFIRMED: Massive data field loss"
else
    log "   --> NOT CONFIRMED (fields look reasonable)"
fi

log ""
log "H2: PROCEDURE DIVISION statement loss"
log "   Total COBOL statements:        ${TOTAL_STMTS:-?}"
log "   Mapped (codegen) statements:   ${MAPPED_STMTS:-?}"
log "   Statement coverage:            ${COV_PCT:-?}%"
log "   Rust MOVE calls:               ${MOVE_CALLS:-?}"
log "   Rust COMPUTE calls:            ${COMPUTE_CALLS:-?}"
log "   Rust function count:           ${FUNCTIONS:-?}"
log "   Bare return (empty fn) count:  ${EMPTY_FNS:-?}"
COV_INT="${COV_PCT%%.*}"
if [ "${COV_INT:-100}" -lt 50 ] 2>/dev/null; then
    log "   --> CONFIRMED: <50% statement coverage"
elif [ "${FUNCTIONS:-0}" -gt 0 ] && [ "${MOVE_CALLS:-0}" -lt 10 ] && [ "${TOTAL_STMTS:-0}" -gt 100 ]; then
    log "   --> LIKELY: Functions exist but bodies are nearly empty"
else
    log "   --> NOT CONFIRMED (coverage looks reasonable)"
fi

log ""
log "H3: EXEC CICS/SQL dominance"
log "   EXEC SQL blocks:               ${EXEC_SQL:-?}"
log "   EXEC CICS blocks:              ${EXEC_CICS:-?}"
log "   EXEC DLI blocks:               ${EXEC_DLI:-?}"
EXEC_TOTAL=$((${EXEC_SQL:-0} + ${EXEC_CICS:-0} + ${EXEC_DLI:-0}))
log "   Total embedded code blocks:    $EXEC_TOTAL"
if [ "$EXEC_TOTAL" -gt 50 ]; then
    log "   --> CONFIRMED: Heavy embedded code usage ($EXEC_TOTAL blocks)"
else
    log "   --> NOT CONFIRMED (only $EXEC_TOTAL blocks)"
fi

log ""
log "H4: COPY expansion bloat"
log "   Raw lines:                     $TOTAL_LINES"
log "   Expanded lines:                ${EXP_LINES:-?}"
log "   Expansion ratio:               ${EXPANSION_RATIO:-?}x"
log "   COPY statements:               $COPY_COUNT"
EXP_INT="${EXPANSION_RATIO%%.*}"
if [ "${EXP_INT:-0}" -ge 2 ] 2>/dev/null; then
    log "   --> CONFIRMED: ${EXPANSION_RATIO}x expansion from COPY"
else
    log "   --> NOT CONFIRMED (expansion < 2x)"
fi

log ""
log "H5: Duplicate section markers (extraction boundary confusion)"
for MARKER in "WORKING-STORAGE SECTION" "PROCEDURE DIVISION"; do
    COUNT=$(grep -ciE "$MARKER" "${EXPANDED:-$INPUT}" || true)
    logf "   %-30s %d occurrences\n" "$MARKER:" "$COUNT"
done
WS_MARKER_COUNT=$(grep -ciE 'WORKING-STORAGE SECTION' "${EXPANDED:-$INPUT}" || true)
PROC_MARKER_COUNT=$(grep -ciE 'PROCEDURE DIVISION' "${EXPANDED:-$INPUT}" || true)
if [ "$WS_MARKER_COUNT" -gt 1 ] || [ "$PROC_MARKER_COUNT" -gt 1 ]; then
    log "   --> CONFIRMED: Duplicate markers will confuse .find() extraction"
else
    log "   --> NOT CONFIRMED (unique markers)"
fi

# =============================================================================
# PHASE 8: Summary
# =============================================================================
log ""
ruler
log "== FILES PRODUCED =="
ruler
log ""
ls -lh "$OUTDIR"/ 2>/dev/null | grep -v '^total' | tee -a "$REPORT"
log ""
log "Full report: $REPORT"
log "Done."
