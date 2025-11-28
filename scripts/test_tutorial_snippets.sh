#!/bin/bash
# Test all Avon code snippets from tutorial markdown files
# Verifies that all examples in the tutorials actually work

set -e

AVON="./target/debug/avon"
PASSED=0
FAILED=0
FAILED_TESTS=()

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "Testing Avon code snippets from tutorial files..."
echo "=================================================="
echo ""

# Test a file and report results
test_file() {
    local file="$1"
    local test_name="$2"
    
    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}⚠ SKIP:${NC} $test_name (file not found: $file)"
        return 0
    fi
    
    if $AVON eval "$file" > /tmp/avon_test_output.txt 2>&1; then
        echo -e "${GREEN}✓ PASS:${NC} $test_name"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}✗ FAIL:${NC} $test_name"
        echo "  Error output:"
        cat /tmp/avon_test_output.txt | sed 's/^/    /' | head -5
        ((FAILED++))
        FAILED_TESTS+=("$test_name")
        return 1
    fi
}

# Test BUILDING_CONTENTS.md examples
echo "Testing BUILDING_CONTENTS.md examples..."
test_file "examples/tutorial_building_contents.av" "BUILDING_CONTENTS.md - All examples"

# Test other tutorial examples (add more as needed)
# test_file "examples/tutorial_other.av" "Other tutorial examples"

echo ""
echo "=================================================="
echo "Test Results:"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Failed tests:"
    for test in "${FAILED_TESTS[@]}"; do
        echo "  - $test"
    done
    exit 1
else
    echo ""
    echo -e "${GREEN}All tutorial snippet tests passed!${NC}"
    exit 0
fi
