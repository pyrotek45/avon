#!/bin/bash
# Test script for bulletproof comprehensive test file
# Ensures the bulletproof test file parses and evaluates correctly

TESTING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT_ROOT="$(cd "$TESTING_DIR/.." && pwd)"
AVON_BIN="$PROJECT_ROOT/target/debug/avon"
BULLETPROOF_FILE="$PROJECT_ROOT/examples/bulletproof.av"

echo "Testing bulletproof comprehensive file..."
echo "=========================================="
echo ""

# Test the bulletproof file
echo "Running bulletproof.av..."
echo "This may take a moment due to the comprehensive nature of the test..."
if timeout 60 "$AVON_BIN" eval "$BULLETPROOF_FILE" > /tmp/bulletproof_output.txt 2>&1; then
    echo "✓ Bulletproof test passed!"
    echo "  Output written to /tmp/bulletproof_output.txt"
    echo "  Lines of output: $(wc -l < /tmp/bulletproof_output.txt)"
    echo "  File size: $(wc -l < "$BULLETPROOF_FILE") lines"
else
    echo "✗ Bulletproof test failed!"
    echo "  Error output:"
    cat /tmp/bulletproof_output.txt
    exit 1
fi

echo ""
echo "Bulletproof test complete!"


