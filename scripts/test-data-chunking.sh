#!/usr/bin/env bash
# =============================================================================
# test-data-chunking.sh -- Verify DATA DIVISION chunked parsing on a COBOL file
#
# Compares raw 01-level count against parsed field count to confirm chunking
# captures all records. This is the primary diagnostic for the monster file fix.
#
# Usage:
#   ./scripts/test-data-chunking.sh <COBOL_FILE> [FLAGS...]
#
# Example:
#   ./scripts/test-data-chunking.sh /path/to/SSUPDATE.cbl -f fixed \
#       -C /path/to/code-copybooks -C /path/to/layout-copybooks
#
# Exit code: 0 if >= 90% of expected 01-levels were parsed, 1 otherwise.
# =============================================================================
set -euo pipefail

CR="${CR:-$(command -v nexmig 2>/dev/null || echo "$(dirname "$0")/../target/debug/nexmig")}"
INPUT="${1:?Usage: $0 <COBOL_FILE> [FLAGS...]}"
shift
EXTRA_FLAGS=("$@")

if [ ! -f "$INPUT" ]; then
    echo "ERROR: File not found: $INPUT"
    exit 1
fi
if [ ! -x "$CR" ]; then
    echo "ERROR: nexmig not found at: $CR"
    echo "Build with: cargo build -p nexmig-cli"
    exit 1
fi

BASENAME=$(basename "$INPUT" .cbl)
BASENAME=$(basename "$BASENAME" .CBL)

echo "========================================"
echo "  DATA DIVISION Chunking Test"
echo "  File: $BASENAME"
echo "========================================"
echo ""

# Step 1: Count raw 01-levels in source
echo "-- Raw source 01-level count --"
RAW_01=$(grep -cE '^\s.{0,6}\s*01\s' "$INPUT" || true)
RAW_77=$(grep -cE '^\s.{0,6}\s*77\s' "$INPUT" || true)
RAW_TOTAL=$((RAW_01 + RAW_77))
echo "  01-levels: $RAW_01"
echo "  77-levels: $RAW_77"
echo "  Total:     $RAW_TOTAL"
echo ""

# Step 2: Preprocess and count expanded 01-levels
TMPDIR=$(mktemp -d)
EXPANDED="$TMPDIR/expanded.cbl"
echo "-- Expanded source 01-level count --"
if "$CR" preprocess "${EXTRA_FLAGS[@]}" "$INPUT" -o "$EXPANDED" 2>/dev/null; then
    EXP_01=$(grep -cE '^\s*01\s' "$EXPANDED" || true)
    EXP_77=$(grep -cE '^\s*77\s' "$EXPANDED" || true)
    EXP_TOTAL=$((EXP_01 + EXP_77))
    EXP_LINES=$(wc -l < "$EXPANDED")
    echo "  01-levels: $EXP_01"
    echo "  77-levels: $EXP_77"
    echo "  Total:     $EXP_TOTAL"
    echo "  Lines:     $EXP_LINES"
else
    echo "  (preprocessing failed, using raw count)"
    EXP_TOTAL=$RAW_TOTAL
fi
echo ""

# Step 3: Parse and count AST fields
echo "-- Parsed AST field count --"
AST_JSON="$TMPDIR/ast.json"
PARSE_STDERR="$TMPDIR/parse_stderr.txt"
if "$CR" parse "${EXTRA_FLAGS[@]}" "$INPUT" --format json -o "$AST_JSON" 2>"$PARSE_STDERR"; then
    if command -v jq &>/dev/null; then
        # Count level-01 and level-77 entries in the AST
        PARSED_01=$(jq '[.. | objects | select(.level == 1)] | length' "$AST_JSON" 2>/dev/null || echo "0")
        PARSED_77=$(jq '[.. | objects | select(.level == 77)] | length' "$AST_JSON" 2>/dev/null || echo "0")
        PARSED_TOTAL=$((PARSED_01 + PARSED_77))
        ALL_FIELDS=$(jq '[.. | objects | select(.level != null)] | length' "$AST_JSON" 2>/dev/null || echo "0")

        echo "  Parsed 01-levels: $PARSED_01"
        echo "  Parsed 77-levels: $PARSED_77"
        echo "  Total top-level:  $PARSED_TOTAL"
        echo "  All fields (nested): $ALL_FIELDS"
    else
        echo "  (jq not installed -- cannot analyze JSON)"
        PARSED_TOTAL=0
        ALL_FIELDS=0
    fi

    # Show chunking info from stderr
    CHUNK_INFO=$(grep -i 'chunk' "$PARSE_STDERR" || true)
    if [ -n "$CHUNK_INFO" ]; then
        echo ""
        echo "-- Chunking diagnostics --"
        echo "$CHUNK_INFO"
    fi
else
    echo "  Parse FAILED"
    head -5 "$PARSE_STDERR"
    PARSED_TOTAL=0
    ALL_FIELDS=0
fi
echo ""

# Step 4: Verdict
echo "========================================"
echo "  RESULTS"
echo "========================================"
echo ""
echo "  Expected (expanded): $EXP_TOTAL top-level records"
echo "  Parsed:              ${PARSED_TOTAL:-0} top-level records"
echo "  Total fields:        ${ALL_FIELDS:-0} (all levels)"

if [ "$EXP_TOTAL" -eq 0 ]; then
    echo ""
    echo "  [SKIP] No expected 01-levels found in source."
elif [ "${PARSED_TOTAL:-0}" -ge "$((EXP_TOTAL * 90 / 100))" ]; then
    PCT=$((PARSED_TOTAL * 100 / EXP_TOTAL))
    echo ""
    echo "  [PASS] ${PCT}% capture rate (>= 90% threshold)"
else
    PCT=$((${PARSED_TOTAL:-0} * 100 / EXP_TOTAL))
    echo ""
    echo "  [FAIL] Only ${PCT}% capture rate (< 90% threshold)"
    echo "  Missing: $((EXP_TOTAL - PARSED_TOTAL)) top-level records"
    rm -rf "$TMPDIR"
    exit 1
fi

rm -rf "$TMPDIR"
