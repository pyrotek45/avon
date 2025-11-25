#!/usr/bin/env bash
# Test all example files to ensure they execute without errors

set -e

EXAMPLES_DIR="examples"
FAILED=()
PASSED=()
SKIPPED=()

# Parse command line options
SHOW_OUTPUT=false
if [[ "${1:-}" == "--show-output" ]] || [[ "${1:-}" == "-v" ]]; then
    SHOW_OUTPUT=true
fi

echo "Building avon..."
cargo build --quiet 2>&1 || { echo "Build failed"; exit 1; }

echo ""
echo "Testing all examples in $EXAMPLES_DIR..."
echo "=========================================="
echo ""

# Get all .av files
for file in "$EXAMPLES_DIR"/*.av; do
    filename=$(basename "$file")
    
    # Skip import_target.av as it's meant to be imported, not run directly
    if [[ "$filename" == "import_target.av" ]]; then
        if [[ "$SHOW_OUTPUT" == true ]]; then
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "⊘ SKIP: $filename (import target)"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo ""
        else
            echo "⊘ SKIP: $filename (import target)"
        fi
        SKIPPED+=("$filename")
        continue
    fi
    
    # Skip error span examples as they test the old span-based error system
    if [[ "$filename" == error_spans_*.av ]]; then
        if [[ "$SHOW_OUTPUT" == true ]]; then
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "⊘ SKIP: $filename (old error span test)"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo ""
        else
            echo "⊘ SKIP: $filename (old error span test)"
        fi
        SKIPPED+=("$filename")
        continue
    fi
    
    if [[ "$SHOW_OUTPUT" == true ]]; then
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "📄 Testing: $filename"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    else
        echo -n "Testing $filename... "
    fi
    
    # Run the example and capture output
    if output=$(cargo run --quiet -- eval "$file" 2>&1); then
        if [[ "$SHOW_OUTPUT" == true ]]; then
            echo "✅ SUCCESS"
            echo ""
            echo "Output:"
            echo "----------------------------------------"
            # Truncate very long output
            if [ ${#output} -gt 2000 ]; then
                echo "${output:0:2000}"
                echo "... (output truncated, total length: ${#output} chars)"
            else
                echo "$output"
            fi
            echo "----------------------------------------"
            echo ""
        else
            echo "✓ PASS"
        fi
        PASSED+=("$filename")
    else
        if [[ "$SHOW_OUTPUT" == true ]]; then
            echo "❌ FAILED"
            echo ""
            echo "Error output:"
            echo "----------------------------------------"
            echo "$output"
            echo "----------------------------------------"
            echo ""
        else
            echo "✗ FAIL"
            echo "  Error output:"
            echo "$output" | sed 's/^/    /'
        fi
        FAILED+=("$filename")
    fi
done

echo ""
echo "=========================================="
echo "Summary:"
echo "  Total: $((${#PASSED[@]} + ${#FAILED[@]} + ${#SKIPPED[@]}))"
echo "  Passed: ${#PASSED[@]}"
echo "  Failed: ${#FAILED[@]}"
echo "  Skipped: ${#SKIPPED[@]}"
echo ""

if [ ${#FAILED[@]} -gt 0 ]; then
    echo "❌ Failed examples:"
    for f in "${FAILED[@]}"; do
        echo "  - $f"
    done
    echo ""
    echo "Run with --show-output to see detailed output from all examples"
    exit 1
else
    echo "🎉 All examples passed! ✓"
    echo ""
    if [[ "$SHOW_OUTPUT" != true ]]; then
        echo "Tip: Run with --show-output or -v to see output from each example"
    fi
    exit 0
fi
