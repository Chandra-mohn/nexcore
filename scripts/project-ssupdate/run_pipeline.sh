#!/bin/bash
# Full transpile + rustify + DSL pipeline for all COBOL programs.
#
# Runs from the client project root (where ./cobol/ lives).
# Requires: .cobol2rust.toml in the project root (copy from this directory).
#
# Usage:
#   cd /path/to/client-repo
#   bash /path/to/nexcore/scripts/project-ssupdate/run_pipeline.sh
#
# Environment:
#   NEXMIG    Path to nexmig binary   (default: ~/nexsuite/bin/nexmig)
#   JOBS      Parallel worker count   (default: 2)
#   EMIT_MODE DSL emit mode           (default: direct)
#             Options: legacy, direct, compare
#
# Outputs:
#   ./rust-out/           Cargo workspace with transpiled Rust
#   ./rust-out/dsl/       DSL files (.schema, .xform, .rules, .proc)
#   ./rust-out/reports/   NDJSON transpile diagnostics
#   ./rustified/          Rustified + DSL output (when using apply mode)
#
set -euo pipefail

NEXMIG="${NEXMIG:-$HOME/nexsuite/bin/nexmig}"
JOBS="${JOBS:-2}"
EMIT_MODE="${EMIT_MODE:-direct}"

COBOL_DIR="./cobol/cobolfile"
WS_OUT="./rust-out"
RUSTIFIED_OUT="./rustified"
COPY_ARGS="-C ./cobol/copybook/code-copybooks -C ./cobol/copybook/layout-copybooks"

# Timing helper
elapsed() {
    local start=$1
    local end=$(date +%s)
    echo "$((end - start))s"
}

PIPELINE_START=$(date +%s)

# ---------- Preflight ----------

echo "=========================================="
echo "PREFLIGHT"
echo "=========================================="

if [ ! -x "$NEXMIG" ] && ! command -v "$NEXMIG" >/dev/null 2>&1; then
    echo "ERROR: nexmig not found at $NEXMIG"
    echo "  Set NEXMIG=/path/to/nexmig or install via ~/nexsuite/nexsuite.sh"
    exit 1
fi

$NEXMIG --version

if [ ! -d "$COBOL_DIR" ]; then
    echo "ERROR: COBOL source directory not found: $COBOL_DIR"
    echo "  Run this script from the project root (where ./cobol/ lives)."
    exit 1
fi

CBL_COUNT=$(find "$COBOL_DIR" -maxdepth 1 -name '*.CBL' -o -name '*.cbl' -o -name '*.cob' | wc -l | tr -d ' ')
echo "COBOL programs found: $CBL_COUNT"
echo "Emit mode: $EMIT_MODE"
echo "Workers: $JOBS"
echo ""

# ---------- Phase 1: Transpile to Cargo Workspace ----------

echo "=========================================="
echo "PHASE 1: Transpile (workspace mode)"
echo "=========================================="
T=$(date +%s)

$NEXMIG -f fixed $COPY_ARGS \
    transpile "$COBOL_DIR" \
    --workspace \
    --parallel \
    -j "$JOBS" \
    --continue-on-error \
    --incremental \
    -o "$WS_OUT"

echo "Phase 1 complete: $(elapsed $T)"
echo ""

# ---------- Phase 2: DSL Emission ----------
#
# Two modes:
#   A) DSL-only: emit DSL from the transpiled workspace (no rustify transforms)
#   B) Rustify + DSL: apply Tier 1 transforms, then emit DSL
#
# Mode A is faster and sufficient for initial DSL generation.
# Mode B is for when you also want idiomatic Rust output.
#
# Uncomment the mode you want. Default: Mode A.

echo "=========================================="
echo "PHASE 2: DSL Emission (${EMIT_MODE} mode)"
echo "=========================================="
T=$(date +%s)

# --- Mode A: DSL-only (no tier transforms) ---
$NEXMIG rustify "$WS_OUT" \
    --emit-dsl \
    --emit-mode "$EMIT_MODE" \
    --cobol-source "$COBOL_DIR" \
    -j "$JOBS"

# --- Mode B: Rustify + DSL (uncomment to use) ---
# $NEXMIG rustify "$WS_OUT" \
#     --emit-dsl \
#     --emit-mode "$EMIT_MODE" \
#     --cobol-source "$COBOL_DIR" \
#     --tier 1 \
#     -o "$RUSTIFIED_OUT" \
#     -j "$JOBS"

echo "Phase 2 complete: $(elapsed $T)"
echo ""

# ---------- Summary ----------

echo "=========================================="
echo "PIPELINE COMPLETE"
echo "=========================================="
echo "Total time: $(elapsed $PIPELINE_START)"
echo ""
echo "Outputs:"
echo "  Rust workspace: $WS_OUT/"
if [ -d "$WS_OUT/dsl" ]; then
    SCHEMA_COUNT=$(find "$WS_OUT/dsl/schema" -name '*.schema' 2>/dev/null | wc -l | tr -d ' ')
    XFORM_COUNT=$(find "$WS_OUT/dsl/transform" -name '*.xform' 2>/dev/null | wc -l | tr -d ' ')
    RULES_COUNT=$(find "$WS_OUT/dsl/rules" -name '*.rules' 2>/dev/null | wc -l | tr -d ' ')
    PROC_COUNT=$(find "$WS_OUT/dsl/process" -name '*.proc' 2>/dev/null | wc -l | tr -d ' ')
    echo "  DSL files:      $WS_OUT/dsl/"
    echo "    .schema: $SCHEMA_COUNT"
    echo "    .xform:  $XFORM_COUNT"
    echo "    .rules:  $RULES_COUNT"
    echo "    .proc:   $PROC_COUNT"
fi
if [ -f "$WS_OUT/dsl/dsl_manifest.json" ]; then
    echo "  DSL manifest:   $WS_OUT/dsl/dsl_manifest.json"
fi
if [ -d "$RUSTIFIED_OUT" ]; then
    echo "  Rustified:      $RUSTIFIED_OUT/"
fi
echo ""
echo "Next steps:"
echo "  - Review DSL files in $WS_OUT/dsl/"
echo "  - Check dsl_manifest.json for confidence scores and review notes"
echo "  - Run compare mode to validate direct vs legacy:"
echo "    EMIT_MODE=compare bash $0"
