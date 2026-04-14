#!/usr/bin/env bash
# =============================================================================
# test-dsl-emission.sh -- Run DSL emission on a COBOL file and report results
#
# Transpiles, rustifies with DSL emission, then summarizes what was produced:
# .schema, .xform, .rules, .proc files with line counts and confidence scores.
#
# Usage:
#   ./scripts/test-dsl-emission.sh <COBOL_FILE> [FLAGS...]
#
# Example:
#   ./scripts/test-dsl-emission.sh cobol/micro/move/move_alpha_to_alpha.cbl
#   ./scripts/test-dsl-emission.sh /path/to/SSUPDATE.cbl -f fixed \
#       -C /path/to/code-copybooks -C /path/to/layout-copybooks
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
OUTDIR=$(mktemp -d)
RUST_OUT="$OUTDIR/rust"
DSL_OUT="$OUTDIR/rustify"

echo "========================================"
echo "  DSL Emission Test"
echo "  File: $BASENAME"
echo "========================================"
echo ""

# Step 1: Transpile COBOL -> Phase 1 Rust
echo "-- Phase 1: Transpile --"
if "$CR" transpile "${EXTRA_FLAGS[@]}" "$INPUT" -o "$RUST_OUT" 2>"$OUTDIR/transpile.err"; then
    RS_FILE=$(find "$RUST_OUT" -name '*.rs' -not -name 'mod.rs' -not -name 'lib.rs' | head -1)
    if [ -n "$RS_FILE" ]; then
        RUST_LINES=$(wc -l < "$RS_FILE")
        RUST_FIELDS=$(grep -c 'pub ' "$RS_FILE" || true)
        RUST_FNS=$(grep -c '^fn \|^pub fn ' "$RS_FILE" || true)
        echo "  Output: $RS_FILE"
        echo "  Lines: $RUST_LINES | Fields: $RUST_FIELDS | Functions: $RUST_FNS"
    else
        echo "  WARNING: No .rs files produced"
    fi
else
    echo "  FAILED -- see $OUTDIR/transpile.err"
    head -5 "$OUTDIR/transpile.err"
    exit 1
fi
echo ""

# Step 2: Rustify with DSL emission
echo "-- Phase 2: Rustify + DSL Emit --"
if "$CR" rustify "$RUST_OUT" -o "$DSL_OUT" --emit-dsl 2>"$OUTDIR/rustify.err"; then
    echo "  Output directory: $DSL_OUT"
else
    echo "  FAILED -- see $OUTDIR/rustify.err"
    head -10 "$OUTDIR/rustify.err"
    exit 1
fi
echo ""

# Step 3: Inventory DSL files
DSL_DIR="$DSL_OUT/dsl"
if [ ! -d "$DSL_DIR" ]; then
    echo "  WARNING: No dsl/ directory produced"
    exit 1
fi

echo "-- DSL Files Produced --"
echo ""

for LAYER in schema transform rules process screen; do
    LAYER_DIR="$DSL_DIR/$LAYER"
    if [ -d "$LAYER_DIR" ]; then
        COUNT=$(find "$LAYER_DIR" -type f | wc -l | tr -d ' ')
        TOTAL_LINES=$(find "$LAYER_DIR" -type f -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')
        printf "  %-12s %3d files, %6d lines\n" ".$LAYER:" "$COUNT" "$TOTAL_LINES"

        # Show individual files
        find "$LAYER_DIR" -type f | sort | while read -r f; do
            FLINES=$(wc -l < "$f")
            FNAME=$(basename "$f")
            printf "    %-40s %5d lines\n" "$FNAME" "$FLINES"
        done
    fi
done
echo ""

# Step 4: Manifest analysis
MANIFEST="$DSL_DIR/dsl_manifest.json"
if [ -f "$MANIFEST" ] && command -v jq &>/dev/null; then
    echo "-- Manifest Summary --"
    TOTAL_FILES=$(jq '.total_files' "$MANIFEST" 2>/dev/null || echo "?")
    AVG_CONF=$(jq '.avg_confidence' "$MANIFEST" 2>/dev/null || echo "?")
    NOTES=$(jq '.review_notes | length' "$MANIFEST" 2>/dev/null || echo "0")
    echo "  Total DSL files: $TOTAL_FILES"
    echo "  Avg confidence:  $AVG_CONF"
    echo "  Review notes:    $NOTES"

    if [ "$NOTES" -gt 0 ]; then
        echo ""
        echo "-- Review Notes (first 10) --"
        jq -r '.review_notes[:10][]' "$MANIFEST" 2>/dev/null
    fi
fi
echo ""

# Step 5: Sample .schema content
echo "-- Sample Schema (first file, first 30 lines) --"
FIRST_SCHEMA=$(find "$DSL_DIR/schema" -name '*.schema' 2>/dev/null | head -1)
if [ -n "$FIRST_SCHEMA" ]; then
    head -30 "$FIRST_SCHEMA"
else
    echo "  (no .schema files)"
fi
echo ""

# Step 6: Sample .proc content
echo "-- Sample Process (first file, first 30 lines) --"
FIRST_PROC=$(find "$DSL_DIR/process" -name '*.proc' 2>/dev/null | head -1)
if [ -n "$FIRST_PROC" ]; then
    head -30 "$FIRST_PROC"
else
    echo "  (no .proc files)"
fi
echo ""

echo "========================================"
echo "  Full output: $DSL_OUT/dsl/"
echo "========================================"
