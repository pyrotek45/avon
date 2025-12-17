#!/bin/bash
# Test script for None handling in Avon
# Verifies that none literal, is_none, and functions returning None work correctly

set -e

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

echo "Testing None Handling..."
echo "========================="
echo ""

PASSED=0
FAILED=0

# Test helper
test_expr() {
    local description="$1"
    local expr="$2"
    local expected="$3"
    
    result=$($AVON run "$expr" 2>&1) || true
    
    if [ "$result" = "$expected" ]; then
        echo "✓ $description"
        PASSED=$((PASSED + 1))
    else
        echo "✗ $description"
        echo "  Expression: $expr"
        echo "  Expected: $expected"
        echo "  Got: $result"
        FAILED=$((FAILED + 1))
    fi
}

echo "--- None Literal Tests ---"
test_expr "none literal evaluates to None" "none" "None"
test_expr "none equals none" "none == none" "true"
test_expr "none not-equals none is false" "none != none" "false"

echo ""
echo "--- is_none Function Tests ---"
test_expr "is_none none returns true" "is_none none" "true"
test_expr "is_none 42 returns false" "is_none 42" "false"
test_expr "is_none \"hello\" returns false" 'is_none "hello"' "false"
test_expr "is_none [] returns false" "is_none []" "false"
test_expr "is_none true returns false" "is_none true" "false"
test_expr "is_none false returns false" "is_none false" "false"

echo ""
echo "--- head Function None Tests ---"
test_expr "head of empty list returns None" "head []" "None"
test_expr "is_none (head []) returns true" "is_none (head [])" "true"
test_expr "head of non-empty list returns first element" "head [1, 2, 3]" "1"
test_expr "is_none (head [1]) returns false" "is_none (head [1])" "false"

echo ""
echo "--- get Function None Tests ---"
test_expr "get missing key returns None" 'get {a: 1} "b"' "None"
test_expr "is_none (get missing key) returns true" 'is_none (get {a: 1} "b")' "true"
test_expr "get existing key returns value" 'get {a: 1, b: 2} "b"' "2"
test_expr "is_none (get existing key) returns false" 'is_none (get {a: 1} "a")' "false"

echo ""
echo "--- None in Conditionals ---"
test_expr "if with is_none check (true branch)" 'let x = head [] in if is_none x then "empty" else "has value"' "empty"
test_expr "if with is_none check (false branch)" 'let x = head [42] in if is_none x then "empty" else "has value"' "has value"
test_expr "if with none equality check" 'let x = none in if x == none then "is none" else "not none"' "is none"

echo ""
echo "--- None with Other Type Checks ---"
test_expr "typeof none returns None" "typeof none" "None"
test_expr "is_string none returns false" "is_string none" "false"
test_expr "is_number none returns false" "is_number none" "false"
test_expr "is_list none returns false" "is_list none" "false"
test_expr "is_bool none returns false" "is_bool none" "false"
test_expr "is_dict none returns false" "is_dict none" "false"

echo ""
echo "--- Practical None Patterns ---"
test_expr "Default value pattern with none" 'let x = head [] in if is_none x then 0 else x' "0"
test_expr "Safe dict access pattern" 'let cfg = {port: 8080} in let host = get cfg "host" in if is_none host then "localhost" else host' "localhost"
test_expr "Filter with none check" 'let items = [1, 2, 3] in let first = head items in if is_none first then "no items" else to_string first' "1"

echo ""
echo "========================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✓ All None handling tests passed!"
    exit 0
else
    echo "✗ Some tests failed!"
    exit 1
fi
