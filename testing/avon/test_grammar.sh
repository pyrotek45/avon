#!/bin/bash
# Test script for grammar comprehensive file
# Ensures the grammar file parses and evaluates correctly

TESTING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT_ROOT="$(cd "$TESTING_DIR/.." && pwd)"
AVON_BIN="$PROJECT_ROOT/target/debug/avon"
GRAMMAR_FILE="$PROJECT_ROOT/examples/grammar_comprehensive.av"

echo "Testing grammar comprehensive file..."
echo "======================================"
echo

# Test the grammar comprehensive file
echo "Running grammar_comprehensive.av..."
if "$AVON_BIN" eval "$GRAMMAR_FILE" > /tmp/grammar_output.txt 2>&1; then
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



