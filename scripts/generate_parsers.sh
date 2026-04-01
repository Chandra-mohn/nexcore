#!/usr/bin/env bash
# generate_parsers.sh -- Generate Rust parsers from Nexflow ANTLR4 grammars.
#
# Uses the project-local ANTLR4 fork JAR (tools/antlr4-rust.jar) which
# supports the Rust target.
#
# Usage:
#   ./scripts/generate_parsers.sh          # generate ApiDSL + ServiceDSL
#   ./scripts/generate_parsers.sh --check  # verify prerequisites only

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

JAR="$PROJECT_ROOT/tools/antlr4-rust.jar"
GRAMMAR_DIR="$PROJECT_ROOT/grammar/nexflow"
OUTPUT_DIR="$PROJECT_ROOT/crates/nexflow-parser/src/generated"

info()  { echo "[INFO] $*"; }
error() { echo "[ERROR] $*" >&2; }

check_prerequisites() {
    if ! command -v java &>/dev/null; then
        error "Java not found. ANTLR4 requires Java to run."
        exit 1
    fi

    if [ ! -f "$JAR" ]; then
        error "ANTLR4 Rust JAR not found at: $JAR"
        exit 1
    fi

    info "Java: $(java -version 2>&1 | head -1)"
    info "ANTLR4 JAR: $JAR"
}

generate() {
    local grammar="$1"
    local name
    name="$(basename "$grammar" .g4)"

    info "Generating Rust from $name.g4 ..."

    java -jar "$JAR" \
        -Dlanguage=Rust \
        -visitor \
        -o "$OUTPUT_DIR" \
        "$grammar"

    info "$name -- done"
}

main() {
    if [ "${1:-}" = "--check" ]; then
        check_prerequisites
        info "All prerequisites OK."
        exit 0
    fi

    check_prerequisites

    # Clean previous generated .rs files (but not mod.rs)
    info "Cleaning previous generated files ..."
    find "$OUTPUT_DIR" -name '*.rs' ! -name 'mod.rs' -delete 2>/dev/null || true
    find "$OUTPUT_DIR" -name '*.interp' -delete 2>/dev/null || true
    find "$OUTPUT_DIR" -name '*.tokens' -delete 2>/dev/null || true

    # Generate parsers for all grammars
    generate "$GRAMMAR_DIR/ApiDSL.g4"
    generate "$GRAMMAR_DIR/ServiceDSL.g4"
    generate "$GRAMMAR_DIR/SchemaDSL.g4"
    generate "$GRAMMAR_DIR/TransformDSL.g4"
    generate "$GRAMMAR_DIR/RulesDSL.g4"
    generate "$GRAMMAR_DIR/ProcDSL.g4"
    generate "$GRAMMAR_DIR/NexQueryDSL.g4"

    # Summary
    echo ""
    info "Generated files:"
    ls -lh "$OUTPUT_DIR"/*.rs 2>/dev/null | while read -r line; do
        echo "  $line"
    done

    echo ""
    info "Parser generation complete. Run 'cargo build -p nexflow-parser' to verify."
}

main "$@"
