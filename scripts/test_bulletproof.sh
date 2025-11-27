#!/bin/bash
# Test script for bulletproof comprehensive test file
# Ensures the bulletproof test file parses and evaluates correctly

set -e

echo "Testing bulletproof comprehensive file..."
echo "=========================================="
echo ""

# Build avon if needed
if [ ! -f "./target/debug/avon" ]; then
    echo "Building avon..."
    cargo build --quiet
fi

# Test the bulletproof file
echo "Running bulletproof.av..."
echo "This may take a moment due to the comprehensive nature of the test..."
if timeout 60 ./target/debug/avon eval tests/bulletproof.av > /tmp/bulletproof_output.txt 2>&1; then
    echo "✓ Bulletproof test passed!"
    echo "  Output written to /tmp/bulletproof_output.txt"
    echo "  Lines of output: $(wc -l < /tmp/bulletproof_output.txt)"
    echo "  File size: $(wc -l < tests/bulletproof.av) lines"
else
    echo "✗ Bulletproof test failed!"
    echo "  Error output:"
    cat /tmp/bulletproof_output.txt
    exit 1
fi

echo ""
echo "Bulletproof test complete!"


