#!/bin/bash
# Debug script for SSUPDATE monster file parsing
# Run each test one at a time. Stop and share output when something fails.
#
# Usage: bash scripts/debug_ssupdate.sh
#
# Paths (adjust if different on your environment)
SRC="./cobol/cobolfile/SSUPDATE.CBL"
COPY_ARGS="-C ./cobol/copybook/code-copybooks -C ./cobol/copybook/layout-copybooks"
NEXMIG="${NEXMIG:-nexmig}"

# Timing helper
elapsed() {
    local start=$1
    local end=$(date +%s)
    echo "[$(( end - start ))s]"
}

SCRIPT_START=$(date +%s)

echo "=========================================="
echo "TEST 0: Version check"
echo "=========================================="
$NEXMIG --version
echo ""

echo "=========================================="
echo "TEST 1: Does the file exist?"
echo "=========================================="
ls -la "$SRC"
wc -l "$SRC"
echo ""

echo "=========================================="
echo "TEST 2: Format detection on raw source"
echo "=========================================="
T=$(date +%s)
$NEXMIG diagnose format "$SRC"
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 3: (SKIPPED)"
echo "=========================================="
echo ""

echo "=========================================="
echo "TEST 4: Preprocess WITHOUT -f fixed (auto-detect)"
echo "=========================================="
T=$(date +%s)
$NEXMIG preprocess $COPY_ARGS "$SRC" 2>/dev/null | wc -l
echo "Leaked sequence numbers (auto):"
$NEXMIG preprocess $COPY_ARGS "$SRC" 2>/dev/null | grep -cE '^\s*[0-9]{6,8}\s*$'
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 5: Preprocess WITH -f fixed"
echo "=========================================="
T=$(date +%s)
$NEXMIG preprocess -f fixed $COPY_ARGS "$SRC" 2>/dev/null | wc -l
echo "Leaked sequence numbers (-f fixed):"
$NEXMIG preprocess -f fixed $COPY_ARGS "$SRC" 2>/dev/null | grep -cE '^\s*[0-9]{6,8}\s*$'
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 6: Does PROCEDURE DIVISION survive preprocessing?"
echo "=========================================="
echo "Auto-detect:"
$NEXMIG preprocess $COPY_ARGS "$SRC" 2>/dev/null | grep -c "PROCEDURE DIVISION"
echo "Fixed:"
$NEXMIG preprocess -f fixed $COPY_ARGS "$SRC" 2>/dev/null | grep -c "PROCEDURE DIVISION"
echo ""

echo "=========================================="
echo "TEST 7: Parse with -f fixed (SINGLE PARSE -- reused by tests 8-9)"
echo "=========================================="
T=$(date +%s)
echo "Parsing to /tmp/ss_parse.json (stdout) and /tmp/ss_parse.err (stderr)..."
$NEXMIG parse -f fixed $COPY_ARGS "$SRC" --format json > /tmp/ss_parse.json 2> /tmp/ss_parse.err
PARSE_EXIT=$?
echo "Exit code: $PARSE_EXIT"
echo "JSON size: $(wc -c < /tmp/ss_parse.json) bytes"
echo "Stderr lines: $(wc -l < /tmp/ss_parse.err)"
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 8: Parse output (first 20 lines)"
echo "=========================================="
head -20 /tmp/ss_parse.json
echo ""

echo "=========================================="
echo "TEST 9: Check what parse produced"
echo "=========================================="
echo "Procedure division present:"
grep -c '"sections"' /tmp/ss_parse.json
echo "Section count:"
grep -c '"paragraphs"' /tmp/ss_parse.json
echo "Paragraph count:"
grep -c '"sentences"' /tmp/ss_parse.json
echo "First 20 section names:"
grep -oE '"name": "[A-Z0-9-]+"' /tmp/ss_parse.json | head -20
echo ""

echo "=========================================="
echo "TEST 10: Stderr output (diagnostics/errors)"
echo "=========================================="
head -30 /tmp/ss_parse.err
echo ""

echo "=========================================="
echo "TEST 11: Save preprocessed output for inspection"
echo "=========================================="
T=$(date +%s)
$NEXMIG preprocess -f fixed $COPY_ARGS "$SRC" -o /tmp/ss_fixed.cbl 2>&1
echo "Saved to /tmp/ss_fixed.cbl"
echo "Line count: $(wc -l < /tmp/ss_fixed.cbl)"
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 12: Show 20 lines around first FD"
echo "=========================================="
grep -n -A 20 '^FD ' /tmp/ss_fixed.cbl | head -30
echo ""

echo "=========================================="
echo "TEST 13: Progressively larger proc division slices"
echo "=========================================="
T=$(date +%s)
# Find the PROCEDURE DIVISION line number in preprocessed output
PROC_LINE=$(grep -n "PROCEDURE DIVISION" /tmp/ss_fixed.cbl | head -1 | cut -d: -f1)
echo "PROCEDURE DIVISION starts at line: $PROC_LINE"
TOTAL_LINES=$(wc -l < /tmp/ss_fixed.cbl)
echo "Total preprocessed lines: $TOTAL_LINES"
echo "Proc division lines: $(( TOTAL_LINES - PROC_LINE ))"
echo ""
# Build a minimal program header + proc division slices of increasing size
for N in 500 2000 10000 50000; do
    # Extract proc division slice and wrap in minimal program
    cat > /tmp/ss_slice.cbl <<HEREDOC
IDENTIFICATION DIVISION.
PROGRAM-ID. SLICE.
DATA DIVISION.
WORKING-STORAGE SECTION.
01 FILLER PIC X.
HEREDOC
    tail -n +$PROC_LINE /tmp/ss_fixed.cbl | head -$N >> /tmp/ss_slice.cbl
    SLICE_LINES=$(wc -l < /tmp/ss_slice.cbl)
    echo "  --- Slice ${N}: first 3 proc lines ---"
    tail -n +$PROC_LINE /tmp/ss_fixed.cbl | head -3
    ST=$(date +%s)
    $NEXMIG parse -f fixed /tmp/ss_slice.cbl --format json > /tmp/ss_slice.json 2>/dev/null
    SECTIONS=$(grep -c '"paragraphs"' /tmp/ss_slice.json)
    PARAS=$(grep -c '"sentences"' /tmp/ss_slice.json)
    echo "  ${N} proc lines (${SLICE_LINES} total): sections=${SECTIONS}, paragraphs=${PARAS} $(elapsed $ST)"
done
echo "$(elapsed $T)"
echo ""

echo "=========================================="
TOTAL=$(( $(date +%s) - SCRIPT_START ))
echo "DONE. Total time: ${TOTAL}s"
echo "=========================================="
