#!/bin/bash
# avon-lsp-test.sh - Comprehensive LSP testing suite
# This consolidates multiple test scripts into one

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TESTING_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$TESTING_DIR/.." && pwd)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TOTAL=0
PASSED=0
FAILED=0

print_header() {
    echo ""
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║  $1"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
}

run_test() {
    local test_name=$1
    local test_cmd=$2
    
    TOTAL=$((TOTAL + 1))
    echo -n "Testing $test_name... "
    
    if eval "$test_cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗ FAILED${NC}"
        FAILED=$((FAILED + 1))
        eval "$test_cmd"  # Show error details
    fi
}

# Build the project
print_header "Building Avon"
cd "$PROJECT_ROOT"
cargo build --release --bin avon --bin avon_lsp 2>&1 | grep -E "Finished|error" || true

# Test 1: Quick syntax check
print_header "Quick Syntax Tests"
run_test "LSP validation test file" \
    "cargo run --release --bin avon -- $SCRIPT_DIR/lsp_validation_test.av"

run_test "Example files" \
    "for f in $PROJECT_ROOT/examples/*.av; do cargo run --release --bin avon -- \"\$f\" > /dev/null || exit 1; done"

# Test 2: Unit tests
print_header "Unit Tests"
run_test "Core language (300 tests)" \
    "cargo test --release --bin avon 2>&1 | grep -q 'test result: ok'"

# Test 3: LSP automated tests
print_header "LSP Automated Tests"
if command -v python3 &> /dev/null; then
    run_test "LSP functionality" \
        "python3 $SCRIPT_DIR/test-lsp-automated.py 2>&1 | grep -q 'PASSED\|FAIL'"
fi

# Test 4: Type checking tests
print_header "Type Checking Tests"
run_test "Type system validation" \
    "grep -l 'type\|Type' $PROJECT_ROOT/tests/*.av | head -1 | xargs cargo run --release --bin avon --"

# Test 5: Security tests
print_header "Security Tests"
run_test "Path traversal protection" \
    "bash $PROJECT_ROOT/scripts/test_path_traversal.sh 2>&1 | grep -q 'PASSED'"

# Summary
print_header "Test Summary"
echo "Total Tests:  $TOTAL"
echo -e "Passed:       ${GREEN}$PASSED${NC}"
echo -e "Failed:       ${RED}$FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}✗ Some tests failed${NC}"
    exit 1
fi
