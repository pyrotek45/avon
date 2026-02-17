#!/bin/bash
# Error Handling & Edge Case Tests
# Tests type errors, boundary conditions, invalid operations,
# division by zero, file I/O errors, conversion errors, and none handling.
# Covers gaps identified in TEST_COVERAGE_REPORT.md

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

PASSED=0
FAILED=0

run_test() {
    local name="$1"
    local expr="$2"
    local expected="$3"

    result=$($AVON run "$expr" 2>&1) || true
    if [ "$result" = "$expected" ]; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected: $expected"
        echo "  Got:      $result"
        ((FAILED++))
    fi
}

# Expects non-zero exit code
run_error_test() {
    local name="$1"
    local expr="$2"

    result=$($AVON run "$expr" 2>&1)
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        echo "✓ $name (error as expected)"
        ((PASSED++))
    else
        echo "✗ $name (should have failed, got: $result)"
        ((FAILED++))
    fi
}

# Expects non-zero exit code in eval mode
run_error_test_eval() {
    local name="$1"
    local code="$2"
    local file="/tmp/avon_error_test_$$.av"

    echo "$code" > "$file"
    result=$($AVON eval "$file" 2>&1)
    exit_code=$?
    rm -f "$file"
    if [ $exit_code -ne 0 ]; then
        echo "✓ $name (error as expected)"
        ((PASSED++))
    else
        echo "✗ $name (should have failed, got: $result)"
        ((FAILED++))
    fi
}

echo "Testing Error Handling & Edge Cases..."
echo "======================================="
echo ""

# ── Type Mismatch Errors ─────────────────────────────────
echo "--- Type Mismatch ---"
run_error_test "add string to number"    '"hello" + 5'
run_error_test "multiply list by string" '[1, 2] * "x"'
run_error_test "subtract string"         '"hello" - 3'

# ── Division by Zero ─────────────────────────────────────
echo ""
echo "--- Division by Zero ---"
run_error_test "divide by zero"          '5 / 0'
run_error_test "integer divide by zero"  '5 // 0'
run_error_test "modulo by zero"          '5 % 0'

# ── Undefined Variable Errors ────────────────────────────
echo ""
echo "--- Undefined Variables ---"
run_error_test "undefined variable"      'undefined_var_xyz'
run_error_test "undefined in expression" 'x + 5'
run_error_test "forward reference"       'let result = x + 1 in let x = 10 in result'

# ── File I/O Errors ──────────────────────────────────────
echo ""
echo "--- File I/O Errors ---"
run_error_test "readfile nonexistent"    'readfile "/nonexistent/path/file.txt"'
run_error_test "readlines nonexistent"   'readlines "/tmp/avon_nonexistent_$$"'

# ── Type Conversion Errors ───────────────────────────────
echo ""
echo "--- Conversion Errors ---"
run_error_test "to_int invalid"          'to_int "not_a_number"'
run_error_test "to_float invalid"        'to_float "not_a_float"'
run_test "to_int valid"                  'to_int "42"'      "42"
run_test "to_float valid"               'to_float "3.14"'   "3.14"

# ── Boundary Conditions ──────────────────────────────────
echo ""
echo "--- Boundary Conditions ---"
run_test "empty list length"             'length []'         "0"
run_test "empty string length"           'length ""'         "0"
run_test "sum empty"                     'sum []'            "0"
run_test "product empty"                 'product []'        "1"
run_test "min single"                    'min [42]'          "42"
run_test "max single"                    'max [42]'          "42"
run_test "head empty"                    'head []'           "None"
run_test "nth out of bounds"             'nth 10 [1, 2, 3]' "None"
run_test "find no match"                 'find (\x x > 10) [1, 2, 3]'  "None"

# ── Numeric Edge Cases ───────────────────────────────────
echo ""
echo "--- Numeric Edge Cases ---"
run_test "zero minus zero"               '0 - 0'            "0"
run_test "small float"                   '0.0000001 + 0'    "0.0000001"
# Wrapping arithmetic
run_test "integer overflow wraps"        '9223372036854775807 + 1' "-9223372036854775808"
# Float precision
run_test "float precision"               '0.1 + 0.2 != 0.3' "true"

# ── String Edge Cases ────────────────────────────────────
echo ""
echo "--- String Edge Cases ---"
run_test "empty + string"                '"" + "test"'      "test"
run_test "string repeat 0"              'repeat "abc" 0'    ""
run_test "string repeat 1"              'repeat "abc" 1'    "abc"
run_test "long string length"           'length (repeat "a" 1000)' "1000"

# ── None Handling ────────────────────────────────────────
echo ""
echo "--- None Handling ---"
run_test "none == none"                  'none == none'      "true"
run_test "none != 5"                     'none != 5'         "true"
run_test "typeof none"                   'typeof none'       "None"
run_test "is_none none"                 'is_none none'       "true"
run_test "is_none 42"                   'is_none 42'         "false"
run_test "is_none string"              'is_none "hello"'     "false"
run_test "default replaces none"        'default 42 none'    "42"
run_test "default keeps value"          'default 42 7'       "7"
run_test "head empty is none"           'is_none (head [])'  "true"
run_test "get missing is none"          'is_none (get {a: 1} "b")' "true"

# ── Conditional Edge Cases ───────────────────────────────
echo ""
echo "--- Conditional Edge Cases ---"
run_test "if with ==none"               'let x = none in if x == none then "yes" else "no"' "yes"
run_test "if with is_none"              'let x = head [] in if is_none x then "empty" else "has"' "empty"
run_test "nested if-else"               'if false then 1 else if false then 2 else 3'  "3"

# ── Lambda Edge Cases ───────────────────────────────────
echo ""
echo "--- Lambda Edge Cases ---"
run_test "identity lambda"              '(\x x) 42'         "42"
run_test "const lambda"                 '(\x \y x) 1 2'     "1"
run_test "lambda with let"              'let f = \x let y = x + 1 in y * 2 in f 5' "12"
run_test "higher order"                 'let apply = \f \x f x in apply (\x x * 3) 7'  "21"

# ── Pipe Error Propagation ───────────────────────────────
echo ""
echo "--- Pipe Errors ---"
run_error_test "undefined in pipe"       '[1, 2, 3] -> undefined_func_xyz'
run_error_test "type error in pipe"      '"hello" -> sum'

# ── Assert Error ─────────────────────────────────────────
echo ""
echo "--- Assert ---"
run_test "assert true passes"            'assert true 42'   "42"
run_error_test "assert false fails"      'assert false 42'

# ── Case Sensitivity ─────────────────────────────────────
echo ""
echo "--- Case Sensitivity ---"
run_error_test "LET not keyword"         'LET x = 5'
run_error_test "TRUE not keyword"        'TRUE'
run_error_test "FALSE not keyword"       'FALSE'
run_error_test "NONE not keyword"        'NONE'
run_test "let is keyword"               'let x = 5 in x'    "5"

# ── Boolean Arithmetic Guard ─────────────────────────────
echo ""
echo "--- Boolean Arithmetic ---"
run_error_test "true + 1"               'true + 1'
run_error_test "false * 2"              'false * 2'

# ── Regex Errors ─────────────────────────────────────────
echo ""
echo "--- Regex Errors ---"
run_error_test "invalid regex pattern"   'regex_match "[0-9" "test"'

echo ""
echo "======================================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✓ All error handling tests passed!"
    exit 0
else
    echo "✗ Some error handling tests failed"
    exit 1
fi
