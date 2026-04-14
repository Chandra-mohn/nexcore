#!/usr/bin/env bash
# =============================================================================
# test-bms-screens.sh -- Test BMS -> .screen DSL generation
#
# Parses all .bms files in a directory, generates .screen + .schema DSL,
# and reports field counts and action counts per map.
#
# Usage:
#   ./scripts/test-bms-screens.sh <BMS_DIR> [OUTPUT_DIR]
#
# Example:
#   ./scripts/test-bms-screens.sh ~/workspace/aws-mainframe-modernization-carddemo/app/bms
# =============================================================================
set -euo pipefail

CR="${CR:-$(command -v nexmig 2>/dev/null || echo "$(dirname "$0")/../target/debug/nexmig")}"
BMS_DIR="${1:?Usage: $0 <BMS_DIR> [OUTPUT_DIR]}"
OUTPUT_DIR="${2:-$(mktemp -d)}"

if [ ! -d "$BMS_DIR" ]; then
    echo "ERROR: BMS directory not found: $BMS_DIR"
    exit 1
fi
if [ ! -x "$CR" ]; then
    echo "ERROR: nexmig not found at: $CR"
    echo "Build with: cargo build -p nexmig-cli"
    exit 1
fi

echo "========================================"
echo "  BMS Screen Generation Test"
echo "  Source: $BMS_DIR"
echo "  Output: $OUTPUT_DIR"
echo "========================================"
echo ""

# Generate screens
if "$CR" bms to-screen "$BMS_DIR" -o "$OUTPUT_DIR" 2>"$OUTPUT_DIR/bms.err"; then
    echo "[OK] Generation complete"
else
    echo "[FAIL] Generation failed"
    cat "$OUTPUT_DIR/bms.err"
    exit 1
fi
echo ""

# Inventory results
SCREEN_DIR="$OUTPUT_DIR/screen"
SCHEMA_DIR="$OUTPUT_DIR/schema"

SCREEN_CT=0
SCHEMA_CT=0
[ -d "$SCREEN_DIR" ] && SCREEN_CT=$(find "$SCREEN_DIR" -name '*.screen' | wc -l | tr -d ' ')
[ -d "$SCHEMA_DIR" ] && SCHEMA_CT=$(find "$SCHEMA_DIR" -name '*.schema' | wc -l | tr -d ' ')

echo "  .screen files: $SCREEN_CT"
echo "  .schema files: $SCHEMA_CT"
echo ""

# Analyze each screen
if [ "$SCREEN_CT" -gt 0 ]; then
    printf "%-25s %8s %8s %8s %8s\n" "SCREEN" "LINES" "FIELDS" "GROUPS" "ACTIONS"
    printf "%-25s %8s %8s %8s %8s\n" "------" "-----" "------" "------" "-------"

    find "$SCREEN_DIR" -name '*.screen' | sort | while read -r f; do
        NAME=$(basename "$f" .screen)
        LINES=$(wc -l < "$f")
        FIELDS=$(grep -c '^\s*field ' "$f" || true)
        GROUPS=$(grep -c '^\s*group ' "$f" || true)
        ACTIONS=$(grep -c '^\s*action ' "$f" || true)
        printf "%-25s %8d %8d %8d %8d\n" "$NAME" "$LINES" "$FIELDS" "$GROUPS" "$ACTIONS"
    done
    echo ""

    # Show sample screen
    FIRST=$(find "$SCREEN_DIR" -name '*.screen' | sort | head -1)
    echo "-- Sample: $(basename "$FIRST") --"
    cat "$FIRST"
fi
echo ""

# Analyze each schema
if [ "$SCHEMA_CT" -gt 0 ]; then
    printf "%-25s %8s %8s\n" "SCHEMA" "LINES" "FIELDS"
    printf "%-25s %8s %8s\n" "------" "-----" "------"

    find "$SCHEMA_DIR" -name '*.schema' | sort | while read -r f; do
        NAME=$(basename "$f" .schema)
        LINES=$(wc -l < "$f")
        FIELDS=$(grep -cE '^\s+\w+\s+(string|integer|decimal|char)' "$f" || true)
        printf "%-25s %8d %8d\n" "$NAME" "$LINES" "$FIELDS"
    done
fi
echo ""

echo "========================================"
echo "  Output: $OUTPUT_DIR"
echo "========================================"
