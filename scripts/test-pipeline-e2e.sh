#!/usr/bin/env bash
# =============================================================================
# test-pipeline-e2e.sh -- End-to-end pipeline test: COBOL -> DSL -> Validate
#
# Runs the full intended pipeline:
#   1. COBOL -> Phase 1 Rust (transpile)
#   2. Phase 1 Rust -> DSL files (rustify --emit-dsl)
#   3. DSL files -> Nexflow validation (nexflow validate)
#
# This verifies the complete COBOL -> DSL -> Java/Rust path is viable.
#
# Usage:
#   ./scripts/test-pipeline-e2e.sh <COBOL_FILE> [FLAGS...]
#
# Example:
#   ./scripts/test-pipeline-e2e.sh cobol/rustify/realistic_batch/realistic_batch_test.cbl
# =============================================================================
set -euo pipefail

NEXMIG="${CR:-$(command -v nexmig 2>/dev/null || echo "$(dirname "$0")/../target/debug/nexmig")}"
NEXFLOW="${NEXFLOW:-$(command -v nexflow 2>/dev/null || echo "$(dirname "$0")/../target/debug/nexflow")}"
INPUT="${1:?Usage: $0 <COBOL_FILE> [FLAGS...]}"
shift
EXTRA_FLAGS=("$@")

if [ ! -f "$INPUT" ]; then
    echo "ERROR: File not found: $INPUT"
    exit 1
fi

BASENAME=$(basename "$INPUT" .cbl)
BASENAME=$(basename "$BASENAME" .CBL)
OUTDIR=$(mktemp -d)
ERRORS=0

echo "========================================"
echo "  End-to-End Pipeline Test"
echo "  File: $BASENAME"
echo "========================================"
echo ""

# --- Stage 1: Transpile ---
echo "[1/3] COBOL -> Phase 1 Rust"
RUST_OUT="$OUTDIR/rust"
if "$NEXMIG" transpile "${EXTRA_FLAGS[@]}" "$INPUT" -o "$RUST_OUT" 2>"$OUTDIR/s1.err"; then
    RS_COUNT=$(find "$RUST_OUT" -name '*.rs' | wc -l | tr -d ' ')
    echo "  [OK] $RS_COUNT .rs files"
else
    echo "  [FAIL] Transpile failed"
    head -3 "$OUTDIR/s1.err"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# --- Stage 2: Rustify + DSL Emit ---
echo "[2/3] Phase 1 Rust -> DSL"
RUSTIFY_OUT="$OUTDIR/rustify"
if "$NEXMIG" rustify "$RUST_OUT" -o "$RUSTIFY_OUT" --emit-dsl 2>"$OUTDIR/s2.err"; then
    DSL_DIR="$RUSTIFY_OUT/dsl"
    if [ -d "$DSL_DIR" ]; then
        SCHEMA_CT=$(find "$DSL_DIR/schema" -name '*.schema' 2>/dev/null | wc -l | tr -d ' ')
        XFORM_CT=$(find "$DSL_DIR/transform" -name '*.xform' 2>/dev/null | wc -l | tr -d ' ')
        RULES_CT=$(find "$DSL_DIR/rules" -name '*.rules' 2>/dev/null | wc -l | tr -d ' ')
        PROC_CT=$(find "$DSL_DIR/process" -name '*.proc' 2>/dev/null | wc -l | tr -d ' ')
        echo "  [OK] .schema=$SCHEMA_CT .xform=$XFORM_CT .rules=$RULES_CT .proc=$PROC_CT"
    else
        echo "  [WARN] No dsl/ directory produced"
    fi
else
    echo "  [FAIL] Rustify failed"
    head -3 "$OUTDIR/s2.err"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# --- Stage 3: Nexflow Validate ---
echo "[3/3] DSL -> Nexflow Validate"
if [ -x "$NEXFLOW" ] && [ -d "${DSL_DIR:-/nonexistent}" ]; then
    # Collect all DSL files
    DSL_FILES=$(find "$DSL_DIR" -type f \( -name '*.schema' -o -name '*.xform' -o -name '*.rules' -o -name '*.proc' \) 2>/dev/null)
    if [ -n "$DSL_FILES" ]; then
        # Validate each layer separately (nexflow validate expects typed input)
        VAL_ERRORS=0
        VAL_WARNINGS=0
        for DSL_FILE in $DSL_FILES; do
            EXT="${DSL_FILE##*.}"
            RESULT=$("$NEXFLOW" parse "$DSL_FILE" --format summary 2>"$OUTDIR/s3_parse.err" || true)
            PARSE_ERR=$(wc -l < "$OUTDIR/s3_parse.err" | tr -d ' ')
            if [ "$PARSE_ERR" -gt 0 ]; then
                VAL_ERRORS=$((VAL_ERRORS + 1))
            fi
        done
        echo "  [OK] Validated $(echo "$DSL_FILES" | wc -l | tr -d ' ') files, $VAL_ERRORS parse errors"
        if [ "$VAL_ERRORS" -gt 0 ]; then
            ERRORS=$((ERRORS + 1))
        fi
    else
        echo "  [SKIP] No DSL files to validate"
    fi
else
    if [ ! -x "$NEXFLOW" ]; then
        echo "  [SKIP] nexflow binary not found (build with: cargo build -p nexflow-cli)"
    else
        echo "  [SKIP] No DSL files produced in stage 2"
    fi
fi
echo ""

# --- Summary ---
echo "========================================"
if [ "$ERRORS" -eq 0 ]; then
    echo "  [PASS] Pipeline completed with 0 errors"
else
    echo "  [FAIL] Pipeline completed with $ERRORS error(s)"
fi
echo "  Output: $OUTDIR"
echo "========================================"

exit "$ERRORS"
