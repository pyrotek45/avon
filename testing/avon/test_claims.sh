#!/bin/bash
# Test all documentation claims
# This ensures that the docs are accurate and bulletproof

TESTING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT_ROOT="$(cd "$TESTING_DIR/.." && pwd)"
AVON="$PROJECT_ROOT/target/debug/avon"

PASSED=0
FAILED=0

test_claim() {
    local name="$1"
    local code="$2"
    local expected="$3"
    
    result=$($AVON run "$code" 2>&1)
    
    if [ "$result" = "$expected" ]; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name (expected '$expected', got '$result')"
        ((FAILED++))
    fi
}

test_error() {
    local name="$1"
    local code="$2"
    local pattern="$3"
    
    result=$($AVON run "$code" 2>&1)
    exit_code=$?
    
    if [ $exit_code -ne 0 ] && echo "$result" | grep -qi "$pattern"; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name (expected error with '$pattern', got '$result')"
        ((FAILED++))
    fi
}

# Helper for order-independent list checks
test_list_contains_all() {
    local name="$1"
    local code="$2"
    local elements="$3"  # comma-separated elements
    
    result=$($AVON run "$code" 2>&1)
    
    all_found=true
    IFS=',' read -ra ELEMS <<< "$elements"
    for elem in "${ELEMS[@]}"; do
        if ! echo "$result" | grep -q "$elem"; then
            all_found=false
            break
        fi
    done
    
    if $all_found; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name (expected to contain '$elements', got '$result')"
        ((FAILED++))
    fi
}

echo "=== Testing Documentation Claims ==="
echo ""

echo "--- Type System Claims ---"
test_claim "Numbers: integer" "42" "42"
test_claim "Numbers: negative" "-17" "-17"
test_claim "Numbers: float" "3.14" "3.14"
test_claim "Strings: basic" '"hello"' "hello"
test_claim "Booleans: true" "true" "true"
test_claim "Booleans: false" "false" "false"
test_claim "Lists: basic" "[1, 2, 3]" "[1, 2, 3]"
test_claim "Dicts: basic" "{a: 1}.a" "1"
test_claim "None: literal" "none" "None"
echo ""

echo "--- Arithmetic Claims ---"
test_claim "Addition" "2 + 3" "5"
test_claim "Subtraction" "5 - 2" "3"
test_claim "Multiplication" "3 * 4" "12"
test_claim "Division" "10 / 2" "5"
test_claim "Modulo" "10 % 3" "1"
test_error "Mixed num + string errors" '1 + "a"' "cannot add"
test_error "Mixed string + num errors" '"a" + 1' "cannot add"
echo ""

echo "--- Comparison Claims ---"
test_claim "Number equality" "1 == 1" "true"
test_claim "Number inequality" "1 != 2" "true"
test_claim "Number greater" "5 > 3" "true"
test_claim "Number less" "3 < 5" "true"
test_claim "Number greater-equal" "5 >= 5" "true"
test_claim "Number less-equal" "3 <= 5" "true"
test_claim "String equality" '"a" == "a"' "true"
test_claim "String ordering" '"a" < "b"' "true"
test_claim "Bool equality" "true == true" "true"
test_error "Bool ordering errors" "true > false" "type\|comparison"
test_claim "List equality" "[1] == [1]" "true"
test_error "List ordering errors" "[1] > [2]" "type\|comparison"
test_claim "Dict equality" "{a: 1} == {a: 1}" "true"
test_error "Dict ordering errors" "{a: 1} > {a: 2}" "type\|comparison"
test_claim "None equality" "none == none" "true"
test_claim "None vs value" "none == 1" "false"
test_error "Cross-type comparison errors" '1 == "1"' "type\|mismatch"
test_error "Cross-type ordering errors" '1 > "0"' "type\|mismatch"
echo ""

echo "--- Logical Operators Claims ---"
test_claim "AND true" "true && true" "true"
test_claim "AND false" "true && false" "false"
test_claim "OR true" "false || true" "true"
test_claim "OR false" "false || false" "false"
test_claim "NOT true" "not true" "false"
test_claim "NOT false" "not false" "true"
test_claim "NOT expression" "not (1 == 2)" "true"
test_error "AND requires bool" '1 && true' "cannot use"
test_error "OR requires bool" '1 || true' "cannot use"
test_error "NOT requires bool" 'not 1' "type"
echo ""

echo "--- Let Binding Claims ---"
test_claim "Let binding" "let x = 5 in x" "5"
test_error "Let requires in" "let x = 5" "expected"
test_claim "Nested let" "let x = 1 in let y = 2 in x + y" "3"
test_error "Shadowing errors" "let x = 1 in let x = 2 in x" "shadow\|already"
echo ""

echo "--- If Expression Claims ---"
test_claim "If true branch" "if true then 1 else 2" "1"
test_claim "If false branch" "if false then 1 else 2" "2"
test_error "If requires else" "if true then 1" "else"
test_error "If condition must be bool" 'if 1 then 2 else 3' "bool\|type"
echo ""

echo "--- Function Claims ---"
test_claim "Lambda basic" '(\x x + 1) 5' "6"
test_claim "Lambda multi-arg" '(\x \y x + y) 2 3' "5"
test_claim "Currying" 'let add = \x \y x + y in let add5 = add 5 in add5 3' "8"
test_claim "Default param uses default" '(\x ? 10 x)' "10"
test_claim "Default param with arg" '(\x ? 10 x) 5' "5"
echo ""

echo "--- String Operations Claims ---"
test_claim "concat" 'concat "a" "b"' "ab"
test_claim "upper" 'upper "hello"' "HELLO"
test_claim "lower" 'lower "HELLO"' "hello"
test_claim "trim" 'trim "  hi  "' "hi"
test_claim "length string" 'length "hello"' "5"
test_claim "contains true" 'contains "hello" "ell"' "true"
test_claim "contains false" 'contains "hello" "xyz"' "false"
test_claim "starts_with" 'starts_with "hello" "hel"' "true"
test_claim "ends_with" 'ends_with "hello" "llo"' "true"
test_claim "replace" 'replace "hello" "l" "L"' "heLLo"
test_claim "split" 'split "a,b,c" ","' '[a, b, c]'
test_claim "join" 'join ["a", "b"] ", "' "a, b"
echo ""

echo "--- List Operations Claims ---"
test_claim "length list" "length [1, 2, 3]" "3"
test_claim "head" "head [1, 2, 3]" "1"
test_claim "head empty returns none" "head []" "None"
test_claim "tail" "tail [1, 2, 3]" "[2, 3]"
test_claim "map" 'map (\x x * 2) [1, 2, 3]' "[2, 4, 6]"
test_claim "filter" 'filter (\x x > 1) [1, 2, 3]' "[2, 3]"
test_claim "fold" 'fold (\acc \x acc + x) 0 [1, 2, 3]' "6"
test_claim "reverse" "reverse [1, 2, 3]" "[3, 2, 1]"
test_claim "sort" "sort [3, 1, 2]" "[1, 2, 3]"
test_claim "unique" "unique [1, 1, 2]" "[1, 2]"
test_claim "flatten" "flatten [[1], [2, 3]]" "[1, 2, 3]"
test_claim "zip" "zip [1, 2] [3, 4]" "[[1, 3], [2, 4]]"
test_claim "take" "take 2 [1, 2, 3]" "[1, 2]"
test_claim "drop" "drop 1 [1, 2, 3]" "[2, 3]"
test_claim "range simple" "[1..5]" "[1, 2, 3, 4, 5]"
test_claim "range no spaces" "[1..5]" "[1, 2, 3, 4, 5]"
test_claim "range with spaces" "[1 .. 5]" "[1, 2, 3, 4, 5]"
test_claim "range with step" "[1, 2..8]" "[1, 3, 5, 7]"
echo ""

echo "--- Dict Operations Claims ---"
test_claim "dict dot access" "{a: 1}.a" "1"
test_claim "dict get" 'get {a: 1} "a"' "1"
test_claim "dict get missing returns none" 'get {a: 1} "b"' "None"
test_list_contains_all "dict keys" 'keys {a: 1, b: 2}' "a,b"
test_claim "dict has_key true" 'has_key {a: 1} "a"' "true"
test_claim "dict has_key false" 'has_key {a: 1} "b"' "false"
test_claim "dict set" 'get (set {a: 1} "b" 2) "b"' "2"
test_claim "dict merge" '(dict_merge {a: 1} {b: 2}).b' "2"
echo ""

echo "--- Type Checking Claims ---"
test_claim "is_string true" 'is_string "hello"' "true"
test_claim "is_string false" "is_string 42" "false"
test_claim "is_number true" "is_number 42" "true"
test_claim "is_number false" 'is_number "42"' "false"
test_claim "is_int true" "is_int 42" "true"
test_claim "is_int false" "is_int 3.14" "false"
test_claim "is_float true" "is_float 3.14" "true"
test_claim "is_float false" "is_float 42" "false"
test_claim "is_bool true" "is_bool true" "true"
test_claim "is_bool false" "is_bool 1" "false"
test_claim "is_list true" "is_list [1, 2]" "true"
test_claim "is_list false" "is_list 42" "false"
test_claim "is_dict true" "is_dict {a: 1}" "true"
test_claim "is_dict false" "is_dict [1]" "false"
test_claim "is_none true" "is_none none" "true"
test_claim "is_none false" "is_none 1" "false"
test_claim "is_function true" 'is_function (\x x)' "true"
test_claim "is_function false" "is_function 42" "false"
echo ""

echo "--- Type Conversion Claims ---"
test_claim "to_string num" "to_string 42" "42"
test_claim "to_int string" 'to_int "42"' "42"
test_claim "to_int float truncates" "to_int 3.9" "3"
test_claim "to_float string" 'to_float "3.14"' "3.14"
test_claim "to_bool yes" 'to_bool "yes"' "true"
test_claim "to_bool no" 'to_bool "no"' "false"
test_claim "to_bool 1" "to_bool 1" "true"
test_claim "to_bool 0" "to_bool 0" "false"
test_claim "to_char" "to_char 65" "A"
test_claim "to_list" 'to_list "Hi"' '[H, i]'
echo ""

echo "--- Pipe Operator Claims ---"
test_claim "pipe basic" "[1, 2, 3] -> length" "3"
test_claim "pipe chain" '[1, 2, 3] -> map (\x x * 2) -> head' "2"
echo ""

echo "--- Template Claims ---"
test_claim "Template interpolation" 'let x = 5 in {"Value: {x}"}' "Value: 5"
test_claim "Template math" '{"Result: {2 + 2}"}' "Result: 4"
echo ""

echo "--- Assert Claims ---"
test_claim "assert true" "assert true 42" "42"
test_error "assert false errors" "assert false 42" "assertion"
echo ""

echo ""
echo "=== Template Syntax Claims (TEMPLATE_SYNTAX.md) ==="

# Helper for file-based tests (avoids shell escaping issues)
test_file_claim() {
    local name="$1"
    local code="$2"
    local expected="$3"
    
    echo "$code" > /tmp/avon_claim_test.av
    result=$($AVON eval /tmp/avon_claim_test.av 2>&1)
    
    if [ "$result" = "$expected" ]; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected: $expected"
        echo "  Got: $result"
        ((FAILED++))
    fi
}

echo "--- Single-Brace Delimiter ---"
test_file_claim "basic template" '{"Hello, world!"}' "Hello, world!"
test_file_claim "single brace interpolation" 'let x = 5 in {"Value: {x}"}' "Value: 5"
test_file_claim "expression interpolation" 'let x = 5 in {"Result: {x * 2}"}' "Result: 10"

echo "--- Double-Brace Delimiter ---"
test_file_claim "double brace interpolation" 'let x = 5 in {{"Value: {{x}}"}}' "Value: 5"
test_file_claim "single braces literal in double" '{{"literal {braces} here"}}' "literal {braces} here"
test_file_claim "double brace mixed" 'let name = "test" in {{"Hello {{name}}, use {var}"}}' "Hello test, use {var}"

echo "--- Triple-Brace Delimiter ---"
test_file_claim "triple brace interpolation" 'let x = 5 in {{{"Value: {{{x}}}"}}}' "Value: 5"
test_file_claim "single braces literal in triple" '{{{"use {x}"}}}' "use {x}"
test_file_claim "double braces literal in triple" '{{{"use {{x}}"}}}' "use {{x}}"
test_file_claim "triple brace mixed" 'let v = 1 in {{{"val={{{v}}}, {{lit}}, {also}"}}}' "val=1, {{lit}}, {also}"

echo "--- Braces in Single-Brace Templates (no escape hatch) ---"
# After removing escape hatch, braces that don't match interpolation count stay literal
test_file_claim "double braces stay literal" '{"Hello {{ world"}' "Hello {{ world"
test_file_claim "double close braces stay literal" '{"Hello }} world"}' "Hello }} world"
test_file_claim "both double braces stay literal" '{"Set: {{ x }}"}' "Set: {{ x }}"

echo "--- Practical Use Cases ---"
# JSON generation
cat > /tmp/avon_json.av << 'AVONEOF'
let name = "Alice" in
let age = 30 in
{{"{"name": "{{name}}", "age": {{age}}}"}}
AVONEOF
result=$($AVON eval /tmp/avon_json.av 2>&1)
expected='{"name": "Alice", "age": 30}'
if [ "$result" = "$expected" ]; then
    echo "✓ JSON generation"
    ((PASSED++))
else
    echo "✗ JSON generation"
    echo "  Expected: $expected"
    echo "  Got: $result"
    ((FAILED++))
fi

# Shell/Bash generation - use helper function for shell variable syntax
cat > /tmp/avon_shell.av << 'AVONEOF'
let shell_var = \v "$" + "{" + v + "}" in
let var = "PATH" in
"echo " + (shell_var var)
AVONEOF
result=$($AVON eval /tmp/avon_shell.av 2>&1)
expected='echo ${PATH}'
if [ "$result" = "$expected" ]; then
    echo "✓ Shell variable generation"
    ((PASSED++))
else
    echo "✗ Shell variable generation"
    echo "  Expected: $expected"
    echo "  Got: $result"
    ((FAILED++))
fi

# JSX curly braces
test_file_claim "JSX literal braces" 'let value = "hello" in {{"<div>{{value}} and {props}</div>"}}' "<div>hello and {props}</div>"

# Template nesting at different levels
test_file_claim "level 1 nesting" 'let inner = {"world"} in {"Hello {inner}!"}' "Hello world!"
test_file_claim "level 2 nesting" 'let inner = {{"val"}} in {{"got {{inner}}"}}' "got val"

# No shadowing builtins
test_error "let cannot shadow builtin" 'let map = 5 in map' "already defined"
test_error "lambda cannot shadow builtin" 'let f = \filter filter in f 5' "shadows builtin"

echo ""
echo "=== Summary ==="
echo "Passed: $PASSED"
echo "Failed: $FAILED"

if [ $FAILED -gt 0 ]; then
    echo "✗ Some claims failed!"
    exit 1
else
    echo "✓ All claims verified!"
    exit 0
fi
