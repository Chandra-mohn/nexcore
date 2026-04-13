#!/usr/bin/env bash
# Run all nexmig fuzz targets for a given number of iterations.
# Usage: scripts/run_fuzz.sh [iterations]
set -euo pipefail

ITERATIONS="${1:-10000}"

TARGETS=(
    packed_decimal_unpack
    packed_decimal_roundtrip
    zoned_decimal_to_decimal
    zoned_decimal_set_bytes
    parse_numeric_display
    parse_zoned_decimal
)

echo "Running ${#TARGETS[@]} fuzz targets for $ITERATIONS iterations each..."

for target in "${TARGETS[@]}"; do
    echo ""
    echo "=== Fuzzing: $target ==="
    cargo +nightly fuzz run "$target" -- -runs="$ITERATIONS" 2>&1 || {
        echo "[FAIL] $target crashed -- check fuzz/artifacts/$target/"
        exit 1
    }
    echo "[OK] $target -- $ITERATIONS iterations"
done

echo ""
echo "All fuzz targets passed."
