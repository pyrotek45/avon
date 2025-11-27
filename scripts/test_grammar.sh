#!/bin/bash
# Test script for grammar comprehensive file
# Ensures the grammar file parses and evaluates correctly

set -e

echo "Testing grammar comprehensive file..."
echo "======================================"
echo

# Build avon if needed
if [ ! -f "./target/debug/avon" ]; then
    echo "Building avon..."
    cargo build --quiet
fi

# Test the grammar comprehensive file
echo "Running grammar_comprehensive.av..."
if ./target/debug/avon eval tests/grammar_comprehensive.av > /tmp/grammar_output.txt 2>&1; then
    echo "✓ Grammar comprehensive test passed!"
    echo "  Output written to /tmp/grammar_output.txt"
    echo "  Lines of output: $(wc -l < /tmp/grammar_output.txt)"
else
    echo "✗ Grammar comprehensive test failed!"
    echo "  Error output:"
    cat /tmp/grammar_output.txt
    exit 1
fi

# Note: Recursion is not supported in Avon
# See tutorial/WHY_NO_RECURSION.md for details

echo
echo "Grammar test complete!"



