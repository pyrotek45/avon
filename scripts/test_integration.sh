#!/bin/bash
# Integration Test Suite for Avon
# Runs all tests to verify the interpreter works correctly

set -e  # Exit on first error

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0

# Test wrapper
run_test() {
    local test_name="$1"
    local test_script="$2"
    
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Running: $test_name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    if bash "$test_script" > /tmp/test_output.txt 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}: $test_name"
        ((PASSED++)) || true
    else
        echo -e "${RED}✗ FAILED${NC}: $test_name"
        echo "Error output:"
        cat /tmp/test_output.txt
        ((FAILED++)) || true
    fi
}

echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║          Avon Integration Test Suite                          ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Build the project
echo "Building project..."
if cargo build --quiet 2>&1; then
    echo -e "${GREEN}✓${NC} Build successful"
else
    echo -e "${RED}✗${NC} Build failed"
    exit 1
fi

# Run each test suite
run_test "Grammar Test" "scripts/test_grammar.sh"
run_test "Bulletproof Test" "scripts/test_bulletproof.sh"
run_test "Tutorial Snippets Test" "scripts/test_tutorial_snippets.sh"
run_test "Scoping Rules Test" "scripts/test_scoping_rules.sh"
run_test "All Examples Test" "scripts/test_all_examples.sh"
run_test "Atomic Deployment Test" "scripts/test_atomic_deployment.sh"
run_test "Fuzz Security Test" "scripts/fuzz.sh"
run_test "Comprehensive Security Test" "scripts/test_security_comprehensive.sh"

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Test Summary:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "  ${GREEN}Passed: $PASSED${NC}"
echo -e "  ${RED}Failed: $FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║          🎉 All Integration Tests Passed! 🎉                  ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    exit 0
else
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║          ❌ Some Tests Failed ❌                              ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    exit 1
fi
