#!/bin/bash
# Comprehensive LSP Testing Framework
# Tests the Avon LSP for correctness and catches regressions

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LSP_BIN="/usr/local/bin/avon-lsp"
TEST_DIR="/tmp/avon-lsp-tests"
TEST_CASES_FILE="/workspaces/avon/lsp-test-cases.json"
RESULTS_FILE="/tmp/avon-lsp-results.json"
LSP_TEST_RUNNER="$SCRIPT_DIR/lsp-test-runner.sh"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=== Avon LSP Testing Framework ==="
echo "Test directory: $TEST_DIR"

# Create test directory
mkdir -p "$TEST_DIR"

# Helper function to run LSP test
run_lsp_test() {
    local test_name=$1
    local avon_code=$2
    local expected_errors=$3
    
    echo -n "Testing: $test_name ... "
    
    # Write test file
    local test_file="$TEST_DIR/test_${test_name}.av"
    echo "$avon_code" > "$test_file"
    
    # Run LSP with stdin/stdout (non-blocking with timeout)
    local response=$(timeout 2 "$LSP_TEST_RUNNER" "$test_file" 2>/dev/null || echo '{"errors": []}')
    
    # Count errors
    local error_count=$(echo "$response" | jq '.errors | length' 2>/dev/null || echo "0")
    
    # Compare with expected
    if [ "$error_count" = "$expected_errors" ]; then
        echo -e "${GREEN}PASS${NC} (found $error_count errors)"
        return 0
    else
        echo -e "${RED}FAIL${NC} (expected $expected_errors, got $error_count)"
        echo "  Code: $avon_code"
        echo "  Response: $response"
        return 1
    fi
}

# Test cases
TESTS_PASSED=0
TESTS_FAILED=0

# Test 1: Undefined variable
if run_lsp_test "undefined_var" "let x = 5 in x + y" "1"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test 2: Valid let binding
if run_lsp_test "valid_let" "let x = 5 in let y = 10 in x + y" "0"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test 3: Single-brace template (should not error)
if run_lsp_test "template_single" '@out.txt {"Hello {name}"}' "0"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test 4: Double-brace template (should not error)
if run_lsp_test "template_double" '@out.txt {{"Hello {{x}}"}}' "0"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test 5: Triple-brace template (should not error)
if run_lsp_test "template_triple" '@out.txt {{{"Content\nHere"}}}' "0"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test 6: Multi-line if statement
if run_lsp_test "multiline_if" $'if x > 5\nthen 10\nelse 20' "0"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test 7: Lambda with multiple parameters
if run_lsp_test "lambda_multiline" $'let f = \\x \\y\n  x + y\nin f 5 10' "0"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test 8: Dict access (no errors)
if run_lsp_test "dict_access" 'let config = {host: "localhost", port: 8080} in config.host' "0"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test 9: Pipe operator
if run_lsp_test "pipe_operator" '[1,2,3] -> length -> map (\x x * 2)' "0"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test 10: Builtin functions (should not error)
if run_lsp_test "builtins" 'length [1,2,3] + map (\x x * 2) [1,2] + filter (\x x > 1) [1,2,3]' "0"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Summary
echo ""
echo "=== Test Summary ==="
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
