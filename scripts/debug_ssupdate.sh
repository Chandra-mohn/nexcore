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

# Timing helper -- prints elapsed seconds since $1
elapsed() {
    local start=$1
    local end=$(date +%s)
    echo "[$(( end - start ))s]"
}

SCRIPT_START=$(date +%s)

echo "=========================================="
echo "TEST 0: Version check"
echo "=========================================="
T=$(date +%s)
$NEXMIG --version
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 1: Does the file exist?"
echo "=========================================="
T=$(date +%s)
ls -la "$SRC"
wc -l "$SRC"
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 2: Format detection on raw source"
echo "=========================================="
T=$(date +%s)
$NEXMIG diagnose format "$SRC"
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 3: Token errors on raw source (SKIPPED -- redundant with Test 8 stderr)"
echo "=========================================="
echo ""

echo "=========================================="
echo "TEST 4: Preprocess WITHOUT -f fixed (auto-detect)"
echo "=========================================="
T=$(date +%s)
$NEXMIG preprocess $COPY_ARGS "$SRC" 2>/dev/null | wc -l
echo "Leaked sequence numbers (auto):"
$NEXMIG preprocess $COPY_ARGS "$SRC" 2>/dev/null | grep -cE '^\s*[0-9]{6,8}\s*$'
echo "SKIP directives remaining (auto):"
$NEXMIG preprocess $COPY_ARGS "$SRC" 2>/dev/null | grep -ciE '^\s*(SKIP1|SKIP2|SKIP3|EJECT)\s*$'
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 5: Preprocess WITH -f fixed"
echo "=========================================="
T=$(date +%s)
$NEXMIG preprocess -f fixed $COPY_ARGS "$SRC" 2>/dev/null | wc -l
echo "Leaked sequence numbers (-f fixed):"
$NEXMIG preprocess -f fixed $COPY_ARGS "$SRC" 2>/dev/null | grep -cE '^\s*[0-9]{6,8}\s*$'
echo "SKIP directives remaining (-f fixed):"
$NEXMIG preprocess -f fixed $COPY_ARGS "$SRC" 2>/dev/null | grep -ciE '^\s*(SKIP1|SKIP2|SKIP3|EJECT)\s*$'
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 6: Does PROCEDURE DIVISION survive preprocessing?"
echo "=========================================="
T=$(date +%s)
echo "Auto-detect:"
$NEXMIG preprocess $COPY_ARGS "$SRC" 2>/dev/null | grep -c "PROCEDURE DIVISION"
echo "Fixed:"
$NEXMIG preprocess -f fixed $COPY_ARGS "$SRC" 2>/dev/null | grep -c "PROCEDURE DIVISION"
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 7: Parse with auto-detect"
echo "=========================================="
T=$(date +%s)
$NEXMIG parse $COPY_ARGS "$SRC" --format json 2>&1 | head -5
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 8: Parse with -f fixed"
echo "=========================================="
T=$(date +%s)
$NEXMIG parse -f fixed $COPY_ARGS "$SRC" --format json 2>&1 | head -20
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 9: Check what parse produces with -f fixed"
echo "=========================================="
T=$(date +%s)
echo "Procedure division present:"
$NEXMIG parse -f fixed $COPY_ARGS "$SRC" --format json 2>/dev/null | grep -c '"sections"'
echo "Section names:"
$NEXMIG parse -f fixed $COPY_ARGS "$SRC" --format json 2>/dev/null | grep -oE '"name": "[A-Z0-9-]+ SECTION"' | head -20
echo "Paragraph count:"
$NEXMIG parse -f fixed $COPY_ARGS "$SRC" --format json 2>/dev/null | grep -c '"sentences"'
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 10: Save preprocessed output for inspection"
echo "=========================================="
T=$(date +%s)
$NEXMIG preprocess -f fixed $COPY_ARGS "$SRC" -o /tmp/ss_fixed.cbl 2>&1
echo "Saved to /tmp/ss_fixed.cbl"
echo "First 50 non-blank lines:"
grep -v '^\s*$' /tmp/ss_fixed.cbl | head -50
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 11: Show 20 lines around first FD"
echo "=========================================="
grep -n -A 20 '^FD ' /tmp/ss_fixed.cbl | head -30
echo ""

echo "=========================================="
echo "TEST 12: Show lines around NONMON (where parse stops)"
echo "=========================================="
grep -n -B 5 -A 20 'NONMON' /tmp/ss_fixed.cbl | head -40
echo ""

echo "=========================================="
echo "TEST 13: Any warnings from our silent failure instrumentation?"
echo "=========================================="
T=$(date +%s)
$NEXMIG parse -f fixed $COPY_ARGS "$SRC" --format json 2>&1 1>/dev/null | head -20
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 14: Small slice test -- first 500 lines of preprocessed"
echo "=========================================="
T=$(date +%s)
head -500 /tmp/ss_fixed.cbl > /tmp/ss_500.cbl
$NEXMIG parse -f fixed /tmp/ss_500.cbl --format json 2>/dev/null | head -10
echo "$(elapsed $T)"
echo ""

echo "=========================================="
echo "TEST 15: Progressively larger slices"
echo "=========================================="
T=$(date +%s)
for N in 100 200 500 1000 2000 5000 10000; do
    head -$N /tmp/ss_fixed.cbl > /tmp/ss_slice.cbl
    RESULT=$($NEXMIG parse -f fixed /tmp/ss_slice.cbl --format json 2>/dev/null | grep -c "procedure_division")
    PROC=$($NEXMIG parse -f fixed /tmp/ss_slice.cbl --format json 2>/dev/null | grep -c "paragraphs")
    echo "  ${N} lines: procedure_division=${RESULT}, paragraphs=${PROC}"
done
echo "$(elapsed $T)"
echo ""

echo "=========================================="
TOTAL=$(( $(date +%s) - SCRIPT_START ))
echo "DONE. Total time: ${TOTAL}s. Share the output above."
echo "=========================================="
