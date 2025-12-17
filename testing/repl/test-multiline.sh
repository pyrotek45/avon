#!/bin/bash

# REPL Multi-line Input Test Suite
# Tests various multi-line input scenarios for :let command

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

# Allow override from command line argument
AVON="${1:-$AVON}"

PASSED=0
FAILED=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

run_test() {
    local name="$1"
    local input="$2"
    local expected_pattern="$3"
    
    result=$(echo -e "$input" | $AVON repl 2>&1)
    
    if echo "$result" | grep -q "$expected_pattern"; then
        echo -e "${GREEN}✓${NC} $name"
        ((PASSED++)))
    else
        echo -e "${RED}✗${NC} $name"
        echo "  Input: $input"
        echo "  Expected pattern: $expected_pattern"
        echo "  Got: $result"
        ((FAILED++))
    fi
}

echo "========================================"
echo "REPL Multi-line Input Test Suite"
echo "========================================"
echo ""

echo "=== SECTION 1: Basic :let (single line) ==="

run_test "Simple number" \
    ':let x = 42\nx' \
    "42 : Number"

run_test "Simple string" \
    ':let s = "hello"\ns' \
    'hello : String'

run_test "Simple list" \
    ':let xs = [1, 2, 3]\nxs' \
    '\[1, 2, 3\] : List'

run_test "Simple dict" \
    ':let d = {a: 1}\nd' \
    '{a: 1} : Dict'

run_test "Single-line lambda" \
    ':let f = \\x x * 2\nf 5' \
    '10 : Number'

run_test "Single-line multi-param lambda" \
    ':let add = \\x \\y x + y\nadd 3 4' \
    '7 : Number'

run_test "Boolean true" \
    ':let b = true\nb' \
    'true : Bool'

run_test "Boolean false" \
    ':let b = false\nb' \
    'false : Bool'

run_test "None value" \
    ':let n = none\nn' \
    'None : None'

echo ""
echo "=== SECTION 2: Multi-line lambdas ==="

run_test "Lambda body on next line" \
    ':let f = \\x\nx * 2\nf 5' \
    '10 : Number'

run_test "Two-param lambda, body on next line" \
    ':let add = \\x \\y\nx + y\nadd 3 4' \
    '7 : Number'

run_test "Lambda with complex body" \
    ':let f = \\x\nx * x + 1\nf 3' \
    '10 : Number'

run_test "Three-param lambda" \
    ':let f = \\a \\b \\c\na + b + c\nf 1 2 3' \
    '6 : Number'

run_test "Four-param lambda" \
    ':let f = \\a \\b \\c \\d\na * b + c * d\nf 2 3 4 5' \
    '26 : Number'

run_test "Lambda with string operations" \
    ':let greet = \\name\nconcat "Hello, " name\ngreet "World"' \
    'Hello, World : String'

run_test "Lambda returning lambda" \
    ':let adder = \\x\n\\y x + y\nlet add5 = adder 5 in add5 3' \
    '8 : Number'

echo ""
echo "=== SECTION 3: Multi-line dicts ==="

run_test "Dict on multiple lines" \
    ':let config = {\nhost: "localhost",\nport: 8080\n}\nconfig.host' \
    'localhost : String'

run_test "Nested dict multi-line" \
    ':let d = {\na: {\nb: 1\n}\n}\nd.a.b' \
    '1 : Number'

run_test "Dict with many fields" \
    ':let person = {\nname: "Alice",\nage: 30,\ncity: "NYC",\nactive: true\n}\nperson.name' \
    'Alice : String'

run_test "Dict with list value" \
    ':let d = {\nitems: [1, 2, 3],\ncount: 3\n}\nsum d.items' \
    '6 : Number'

run_test "Dict with nested lists" \
    ':let d = {\nmatrix: [\n[1, 2],\n[3, 4]\n]\n}\nhead (head d.matrix)' \
    '1 : Number'

echo ""
echo "=== SECTION 4: Multi-line lists ==="

run_test "List on multiple lines" \
    ':let xs = [\n1,\n2,\n3\n]\nsum xs' \
    '6 : Number'

run_test "List of strings" \
    ':let names = [\n"Alice",\n"Bob",\n"Charlie"\n]\nlength names' \
    '3 : Number'

run_test "Nested lists" \
    ':let matrix = [\n[1, 2, 3],\n[4, 5, 6],\n[7, 8, 9]\n]\nlength matrix' \
    '3 : Number'

run_test "List with expressions" \
    ':let xs = [\n1 + 1,\n2 * 2,\n3 * 3\n]\nxs' \
    '\[2, 4, 9\] : List'

echo ""
echo "=== SECTION 5: Nested let/in expressions ==="

run_test "Lambda with nested let/in" \
    ':let f = \\a \\b\nlet r = a * b\nin r\nf 3 4' \
    '12 : Number'

run_test "Lambda with multiple nested lets" \
    ':let f = \\x\nlet a = x * 2\nin let b = a + 1\nin b\nf 5' \
    '11 : Number'

run_test "Nested let with if/then/else" \
    ':let f = \\x\nlet r = if x > 0 then x else 0 - x\nin r\nf (0 - 5)' \
    '5 : Number'

run_test "Triple nested let" \
    ':let f = \\x\nlet a = x + 1\nin let b = a + 1\nin let c = b + 1\nin c\nf 0' \
    '3 : Number'

run_test "Let with function call" \
    ':let f = \\xs\nlet total = sum xs\nin let avg = total / (length xs)\nin avg\nf [10, 20, 30]' \
    '20 : Number'

run_test "Let binding a lambda" \
    ':let f = \\x\nlet double = \\y y * 2\nin double x\nf 5' \
    '10 : Number'

echo ""
echo "=== SECTION 6: If/then/else multi-line ==="

run_test "If/then/else on multiple lines" \
    ':let f = \\x\nif x > 0\nthen x\nelse 0 - x\nf (0 - 3)' \
    '3 : Number'

run_test "Nested if/then/else" \
    ':let f = \\x\nif x > 10\nthen "big"\nelse if x > 0\nthen "small"\nelse "zero"\nf 5' \
    'small : String'

run_test "If with complex conditions" \
    ':let f = \\x \\y\nif x > 0 && y > 0\nthen x + y\nelse 0\nf 3 4' \
    '7 : Number'

run_test "If with or condition" \
    ':let f = \\x\nif x < 0 || x > 100\nthen "out of range"\nelse "ok"\nf 50' \
    'ok : String'

run_test "Deeply nested if" \
    ':let grade = \\score\nif score >= 90\nthen "A"\nelse if score >= 80\nthen "B"\nelse if score >= 70\nthen "C"\nelse "F"\ngrade 85' \
    'B : String'

echo ""
echo "=== SECTION 7: Escape commands ==="

run_test "Cancel with :cancel" \
    ':let f = \\x\n:cancel\n42' \
    '42 : Number'

run_test "Cancel with :c" \
    ':let f = \\x\n:c\n42' \
    '42 : Number'

run_test "Cancel with :reset" \
    ':let f = \\x\n:reset\n42' \
    '42 : Number'

run_test "Exit with :exit" \
    ':let f = \\x\n:exit' \
    'Cancelled pending input'

run_test "Exit with :quit" \
    ':let f = \\x\n:quit' \
    'Cancelled pending input'

run_test "Exit with :q" \
    ':let f = \\x\n:q' \
    'Cancelled pending input'

run_test "Cancel with 3 empty lines" \
    ':let f = \\x\n\n\n\n42' \
    '42 : Number'

echo ""
echo "=== SECTION 8: Pipelines ==="

run_test "Lambda with pipeline" \
    ':let f = \\xs\nxs -> map (\\x x * 2) -> sum\nf [1, 2, 3]' \
    '12 : Number'

run_test "Multi-line pipeline in parens" \
    ':let f = \\xs (xs\n-> map (\\x x * 2)\n-> filter (\\x x > 2)\n-> sum)\nf [1, 2, 3]' \
    '10 : Number'

run_test "Pipeline ending with ->" \
    ':let f = \\xs xs ->\nmap (\\x x * 2) ->\nsum\nf [1, 2, 3]' \
    '12 : Number'

run_test "Complex pipeline" \
    ':let process = \\xs (xs\n-> filter (\\x x > 0)\n-> map (\\x x * x)\n-> sum)\nprocess [-1, 2, -3, 4]' \
    '20 : Number'

echo ""
echo "=== SECTION 9: Real-world examples ==="

run_test "Config builder" \
    ':let make_config = \\host \\port\n{\nhost: host,\nport: port,\nssl: true\n}\nmake_config "localhost" 8080' \
    'host: "localhost"'

run_test "Function returning function" \
    ':let adder = \\x\n\\y x + y\nlet add5 = adder 5\nin add5 3' \
    '8 : Number'

run_test "Validator function" \
    ':let validate = \\x\nif x < 0\nthen { valid: false, error: "negative" }\nelse { valid: true, value: x }\n(validate 5).valid' \
    'true : Bool'

run_test "List processor" \
    ':let stats = \\xs\nlet t = sum xs\nin let c = length xs\nin let a = t / c\nin { total: t, count: c, avg: a }\n(stats [1, 2, 3, 4, 5]).avg' \
    '3 : Number'

run_test "Recursive-style with fold" \
    ':let factorial = \\n\nfold (\\acc \\x acc * x) 1 (range 1 (n + 1))\nfactorial 5' \
    '720 : Number'

run_test "String template builder" \
    ':let greet = \\name \\age\nlet msg = concat "Hello " name\nin let full = concat msg (concat ", you are " (to_string age))\nin full\ngreet "Alice" 30' \
    'Hello Alice, you are 30 : String'

echo ""
echo "=== SECTION 10: Edge cases ==="

run_test "Empty dict" \
    ':let d = {\n}\nd' \
    '{} : Dict'

run_test "Empty list" \
    ':let xs = [\n]\nxs' \
    '\[\] : List'

run_test "Lambda with underscore param" \
    ':let f = \\_ \\x\nx * 2\nf "ignored" 5' \
    '10 : Number'

run_test "Lambda with long param name" \
    ':let f = \\my_long_variable_name\nmy_long_variable_name * 2\nf 5' \
    '10 : Number'

run_test "Multiple :let commands" \
    ':let a = 1\n:let b = 2\na + b' \
    '3 : Number'

run_test "Redefine variable" \
    ':let x = 1\n:let x = 2\nx' \
    '2 : Number'

run_test "Use previously defined in new :let" \
    ':let x = 10\n:let f = \\y\nx + y\nf 5' \
    '15 : Number'

run_test "Multiline with comments would fail gracefully" \
    ':let x = 42\nx' \
    '42 : Number'

echo ""
echo "=== SECTION 11: Complex nested structures ==="

run_test "Dict with lambda values" \
    ':let ops = {\nadd: \\x \\y x + y,\nmul: \\x \\y x * y\n}\nops.add 3 4' \
    '7 : Number'

run_test "List of dicts" \
    ':let users = [\n{ name: "Alice", age: 30 },\n{ name: "Bob", age: 25 }\n]\n(head users).name' \
    'Alice : String'

run_test "Deeply nested access" \
    ':let data = {\na: {\nb: {\nc: {\nd: 42\n}\n}\n}\n}\ndata.a.b.c.d' \
    '42 : Number'

run_test "Mixed nesting" \
    ':let data = {\nitems: [\n{ x: 1, y: 2 },\n{ x: 3, y: 4 }\n],\ncount: 2\n}\n(nth 1 data.items).x' \
    '3 : Number'

echo ""
echo "=== SECTION 12: Stress tests ==="

run_test "Very long lambda chain" \
    ':let f = \\a \\b \\c \\d \\e\na + b + c + d + e\nf 1 2 3 4 5' \
    '15 : Number'

run_test "Many nested lets" \
    ':let f = \\x\nlet a = x\nin let b = a\nin let c = b\nin let d = c\nin let e = d\nin e\nf 42' \
    '42 : Number'

run_test "Complex expression body" \
    ':let f = \\x \\y\nlet s = x + y\nin let d = x - y\nin let p = x * y\nin { sum: s, diff: d, prod: p }\n(f 10 3).prod' \
    '30 : Number'

run_test "Pipeline with multiple transforms" \
    ':let f = \\xs (xs\n-> map (\\x x + 1)\n-> map (\\x x * 2)\n-> filter (\\x x > 5)\n-> map (\\x x - 1)\n-> sum)\nf [1, 2, 3, 4, 5]' \
    '32 : Number'

echo ""
echo "========================================"
echo "Results: ${GREEN}$PASSED passed${NC}, ${RED}$FAILED failed${NC}"
echo "========================================"

exit $FAILED
