#!/usr/bin/env bash
# audit_inspect.sh -- Interactive investigator for nexmig audit JSON reports.
# Usage: ./scripts/audit_inspect.sh <audit_report.json>

set -euo pipefail

if [ $# -lt 1 ] || [ ! -f "$1" ]; then
    echo "Usage: $0 <audit_report.json>"
    exit 1
fi

REPORT="$1"

# Verify jq is available.
if ! command -v jq &>/dev/null; then
    echo "Error: jq is required. Install with: brew install jq"
    exit 1
fi

show_menu() {
    echo ""
    echo "=========================================="
    echo "  NexMig Audit Report Inspector"
    echo "=========================================="
    echo ""
    echo "  1) Summary              -- scores and phase counts"
    echo "  2) Missing copybooks    -- all missing with ref counts"
    echo "  3) Missing for program  -- copybooks blocking a file"
    echo "  4) Programs for copybook-- files needing a copybook"
    echo "  5) Blocked programs     -- all blocked files and why"
    echo "  6) Encoding issues      -- files with non-ASCII bytes"
    echo "  7) Unused copybooks     -- discovered but unreferenced"
    echo "  8) Largest source files -- top N by line count"
    echo "  9) Parse warnings       -- token recognition issues"
    echo " 10) Most referenced      -- copybooks by reference count"
    echo " 11) Copybooks in file    -- COPY statements in a source file"
    echo ""
    echo "  q) Quit"
    echo ""
}

opt_summary() {
    echo ""
    jq -r '
        .summary as $s | .phases as $p |
        "Source files:     \($s.total_source_files)",
        "Copybook files:   \($s.total_copybook_files)",
        "Total lines:      \($s.total_lines)",
        "",
        "Scores:",
        "  Encoding:       \($s.readiness.encoding.score | . * 10 | round / 10)  [\($s.readiness.encoding.status)]",
        "  Dependencies:   \($s.readiness.dependencies.score | . * 10 | round / 10)  [\($s.readiness.dependencies.status)]",
        "  Parsing:        \($s.readiness.parsing.score | . * 10 | round / 10)  [\($s.readiness.parsing.status)]",
        "  Coverage:       \($s.readiness.coverage.score | . * 10 | round / 10)  [\($s.readiness.coverage.status)]",
        "  Overall:        \($s.readiness.overall | . * 10 | round / 10)",
        "  Verdict:        \($s.verdict | ascii_upcase)",
        "",
        "Phase 2 - Dependencies:",
        "  Referenced:     \($p.dependencies.unique_copybooks_referenced)",
        "  Found:          \($p.dependencies.copybooks_found)",
        "  Missing:        \($p.dependencies.copybooks_missing | length)",
        "",
        "Phase 3 - Validation:",
        "  Parsed OK:      \($p.validation.files_parsed_ok)",
        "  Blocked:        \($p.validation.files_blocked)",
        "  Errors:         \($p.validation.files_with_errors)",
        "  With warnings:  \($p.validation.files_with_warnings)",
        "",
        "Phase 4 - Coverage:",
        "  Files analyzed: \($p.coverage.files_analyzed // "n/a")",
        "  Avg coverage:   \($p.coverage.average_coverage_pct // "n/a" | if type == "number" then "\(. * 10 | round / 10)%" else . end)"
    ' "$REPORT"
}

opt_missing_copybooks() {
    echo ""
    jq -r '
        .phases.dependencies.copybooks_missing
        | sort_by(-(.referenced_by | length))
        | .[] | "\(.referenced_by | length)\t\(.name)"
    ' "$REPORT" | column -t -s$'\t' -N "REFS,COPYBOOK"
    echo ""
    echo "Total missing: $(jq '.phases.dependencies.copybooks_missing | length' "$REPORT")"
}

opt_missing_for_program() {
    read -rp "Enter program name (or partial path): " pattern
    echo ""
    jq -r --arg pat "$pattern" '
        .phases.dependencies.copybooks_missing[]
        | select(.referenced_by[] | ascii_downcase | contains($pat | ascii_downcase))
        | .name
    ' "$REPORT" | sort
    count=$(jq -r --arg pat "$pattern" '
        [.phases.dependencies.copybooks_missing[]
        | select(.referenced_by[] | ascii_downcase | contains($pat | ascii_downcase))]
        | length
    ' "$REPORT")
    echo ""
    echo "Missing copybooks for *${pattern}*: ${count}"
}

opt_programs_for_copybook() {
    read -rp "Enter copybook name: " cbname
    echo ""
    jq -r --arg cb "$cbname" '
        .phases.dependencies.copybooks_missing[]
        | select(.name | ascii_downcase == ($cb | ascii_downcase))
        | .referenced_by[]
    ' "$REPORT" | sort
}

opt_blocked_programs() {
    echo ""
    jq -r '
        .phases.validation.file_results[]
        | select(.status == "blocked")
        | "\(.path)\t\([.errors[] | .message | ltrimstr("missing copybook: ")] | join(", "))"
    ' "$REPORT" | column -t -s$'\t' -N "FILE,MISSING COPYBOOKS"
    echo ""
    echo "Total blocked: $(jq '[.phases.validation.file_results[] | select(.status == "blocked")] | length' "$REPORT")"
}

opt_encoding_issues() {
    echo ""
    jq -r '
        .phases.discovery.encoding_issues
        | sort_by(-.non_ascii_count)
        | .[] | "\(.non_ascii_count)\t\(.file_encoding // "unknown")\t\(.path)"
    ' "$REPORT" | column -t -s$'\t' -N "NON-ASCII,ENCODING,FILE"
    echo ""
    echo "Total files with encoding issues: $(jq '.phases.discovery.encoding_issues | length' "$REPORT")"
}

opt_unused_copybooks() {
    echo ""
    jq -r '.phases.dependencies.copybooks_unused[]' "$REPORT" | sort
    echo ""
    echo "Total unused: $(jq '.phases.dependencies.copybooks_unused | length' "$REPORT")"
}

opt_largest_files() {
    read -rp "How many? [20]: " n
    n=${n:-20}
    echo ""
    jq -r --argjson n "$n" '
        .phases.validation.file_results
        | sort_by(-.line_count)
        | .[:$n]
        | .[] | "\(.line_count)\t\(.status)\t\(.path)"
    ' "$REPORT" | column -t -s$'\t' -N "LINES,STATUS,FILE"
}

opt_parse_warnings() {
    echo ""
    jq -r '
        [.phases.validation.file_results[]
         | select(.warnings | length > 0)
         | {path, count: (.warnings | length)}]
        | sort_by(-.count)
        | .[] | "\(.count)\t\(.path)"
    ' "$REPORT" | column -t -s$'\t' -N "WARNINGS,FILE"
    total=$(jq '[.phases.validation.file_results[] | .warnings | length] | add // 0' "$REPORT")
    files=$(jq '[.phases.validation.file_results[] | select(.warnings | length > 0)] | length' "$REPORT")
    echo ""
    echo "Total warnings: ${total} across ${files} files"
}

opt_most_referenced() {
    echo ""
    jq -r '
        .phases.dependencies.copybooks_missing
        | sort_by(-(.referenced_by | length))
        | .[:20]
        | .[] | "\(.referenced_by | length)\t\(.name)"
    ' "$REPORT"
    echo ""
    echo "(showing top 20 missing copybooks by reference count)"
}

opt_copybooks_in_file() {
    read -rp "Enter path to COBOL source file: " srcfile
    if [ ! -f "$srcfile" ]; then
        echo "File not found: $srcfile"
        return
    fi
    echo ""
    grep -i "COPY " "$srcfile" | \
        sed 's/.*[Cc][Oo][Pp][Yy]  *//' | \
        sed 's/[. ].*//' | \
        tr '[:lower:]' '[:upper:]' | \
        sort -u
    echo ""
    total=$(grep -ci "COPY " "$srcfile" || echo 0)
    unique=$(grep -i "COPY " "$srcfile" | sed 's/.*[Cc][Oo][Pp][Yy]  *//' | sed 's/[. ].*//' | tr '[:lower:]' '[:upper:]' | sort -u | wc -l | tr -d ' ')
    echo "Total COPY statements: ${total}, unique copybooks: ${unique}"
}

# Main loop
while true; do
    show_menu
    read -rp "Pick [1-10, q]: " choice
    case "$choice" in
        1)  opt_summary ;;
        2)  opt_missing_copybooks ;;
        3)  opt_missing_for_program ;;
        4)  opt_programs_for_copybook ;;
        5)  opt_blocked_programs ;;
        6)  opt_encoding_issues ;;
        7)  opt_unused_copybooks ;;
        8)  opt_largest_files ;;
        9)  opt_parse_warnings ;;
        10) opt_most_referenced ;;
        11) opt_copybooks_in_file ;;
        q|Q) echo "Bye."; exit 0 ;;
        *)  echo "Invalid option." ;;
    esac
done
