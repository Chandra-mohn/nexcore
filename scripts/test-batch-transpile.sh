#!/usr/bin/env bash
# =============================================================================
# test-batch-transpile.sh -- Batch transpile all test COBOL files and report
#
# Runs nexmig transpile on every .cbl file under cobol/ and reports
# pass/fail/field counts. Useful for regression testing after parser changes.
#
# Usage:
#   ./scripts/test-batch-transpile.sh [COBOL_DIR]
#
# Default COBOL_DIR: ./cobol (the test suite shipped with nexcore)
# =============================================================================
set -euo pipefail

CR="${CR:-$(command -v nexmig 2>/dev/null || echo "$(dirname "$0")/../target/debug/nexmig")}"
COBOL_DIR="${1:-$(dirname "$0")/../cobol}"

if [ ! -d "$COBOL_DIR" ]; then
    echo "ERROR: COBOL directory not found: $COBOL_DIR"
    exit 1
fi
if [ ! -x "$CR" ]; then
    echo "ERROR: nexmig not found at: $CR"
    echo "Build with: cargo build -p nexmig-cli"
    exit 1
fi

OUTDIR=$(mktemp -d)
TOTAL=0
PASSED=0
FAILED=0
ZERO_FIELDS=0

echo "========================================"
echo "  Batch Transpile Test"
echo "  Source: $COBOL_DIR"
echo "  Binary: $CR"
echo "========================================"
echo ""

# Find all .cbl files
FILES=$(find "$COBOL_DIR" -name '*.cbl' -o -name '*.CBL' | sort)
TOTAL=$(echo "$FILES" | wc -l | tr -d ' ')

printf "%-50s %8s %8s %8s\n" "FILE" "STATUS" "FIELDS" "FNS"
printf "%-50s %8s %8s %8s\n" "----" "------" "------" "---"

for CBL in $FILES; do
    REL=$(echo "$CBL" | sed "s|$COBOL_DIR/||")
    RS_OUT="$OUTDIR/rs"
    rm -rf "$RS_OUT"

    if "$CR" transpile "$CBL" -o "$RS_OUT" 2>/dev/null; then
        RS_FILE=$(find "$RS_OUT" -name '*.rs' -not -name 'mod.rs' -not -name 'lib.rs' 2>/dev/null | head -1)
        if [ -n "$RS_FILE" ]; then
            FIELDS=$(grep -c 'pub ' "$RS_FILE" 2>/dev/null || echo "0")
            FNS=$(grep -c '^fn \|^pub fn ' "$RS_FILE" 2>/dev/null || echo "0")
            printf "%-50s %8s %8s %8s\n" "$REL" "[OK]" "$FIELDS" "$FNS"
            PASSED=$((PASSED + 1))
            if [ "$FIELDS" -eq 0 ]; then
                ZERO_FIELDS=$((ZERO_FIELDS + 1))
            fi
        else
            printf "%-50s %8s %8s %8s\n" "$REL" "[EMPTY]" "0" "0"
            PASSED=$((PASSED + 1))
            ZERO_FIELDS=$((ZERO_FIELDS + 1))
        fi
    else
        printf "%-50s %8s %8s %8s\n" "$REL" "[FAIL]" "-" "-"
        FAILED=$((FAILED + 1))
    fi
done

echo ""
echo "========================================"
echo "  SUMMARY"
echo "========================================"
echo "  Total:       $TOTAL"
echo "  Passed:      $PASSED"
echo "  Failed:      $FAILED"
echo "  Zero fields: $ZERO_FIELDS"
echo ""

if [ "$FAILED" -gt 0 ]; then
    echo "  [FAIL] $FAILED files failed to transpile"
    exit 1
else
    echo "  [PASS] All $TOTAL files transpiled successfully"
fi

rm -rf "$OUTDIR"
