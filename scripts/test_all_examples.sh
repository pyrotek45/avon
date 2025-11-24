#!/usr/bin/env bash
# Test all example files to ensure they execute without errors

set -e

EXAMPLES_DIR="examples"
FAILED=()
PASSED=()
SKIPPED=()

echo "Testing all examples in $EXAMPLES_DIR..."
echo "=========================================="
echo ""

# Get all .av files
for file in "$EXAMPLES_DIR"/*.av; do
    filename=$(basename "$file")
    
    # Skip import_target.av as it's meant to be imported, not run directly
    if [[ "$filename" == "import_target.av" ]]; then
        echo "⊘ SKIP: $filename (import target)"
        SKIPPED+=("$filename")
        continue
    fi
    
    echo -n "Testing $filename... "
    
    # Run the example and capture output
    if output=$(cargo run --quiet -- eval "$file" 2>&1); then
        echo "✓ PASS"
        PASSED+=("$filename")
    else
        echo "✗ FAIL"
        echo "  Error output:"
        echo "$output" | sed 's/^/    /'
        FAILED+=("$filename")
    fi
done

echo ""
echo "=========================================="
echo "Summary:"
echo "  Passed: ${#PASSED[@]}"
echo "  Failed: ${#FAILED[@]}"
echo "  Skipped: ${#SKIPPED[@]}"
echo ""

if [ ${#FAILED[@]} -gt 0 ]; then
    echo "Failed examples:"
    for f in "${FAILED[@]}"; do
        echo "  - $f"
    done
    exit 1
else
    echo "All examples passed! ✓"
    exit 0
fi
