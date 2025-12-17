#!/usr/bin/env bash
# Comprehensive tests for parallel functions (pmap, pfilter, pfold)
# Ensures parallel versions produce identical results to sequential versions

# Don't exit on error - we want to see all test results
# set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
AVON="$PROJECT_ROOT/target/release/avon"
PASS=0
FAIL=0

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Build if needed
if [ ! -x "$AVON" ]; then
    echo "Building release binary..."
    cd "$PROJECT_ROOT" && cargo build --release
fi

# Test function: compare sequential vs parallel output
test_eq() {
    local name="$1"
    local seq_expr="$2"
    local par_expr="$3"
    
    local seq_result=$("$AVON" run "$seq_expr" 2>&1)
    local par_result=$("$AVON" run "$par_expr" 2>&1)
    
    if [ "$seq_result" = "$par_result" ]; then
        echo -e "${GREEN}✓${NC} $name"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $name"
        echo "    Sequential: $seq_result"
        echo "    Parallel:   $par_result"
        ((FAIL++))
    fi
}

# Test function: check single expression works without error
test_ok() {
    local name="$1"
    local expr="$2"
    local expected="$3"
    
    local result=$("$AVON" run "$expr" 2>&1)
    local exit_code=$?
    
    if [ $exit_code -eq 0 ] && [ "$result" = "$expected" ]; then
        echo -e "${GREEN}✓${NC} $name"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $name"
        echo "    Expected: $expected"
        echo "    Got:      $result"
        ((FAIL++))
    fi
}

echo "═══════════════════════════════════════════════════════════════"
echo "             Parallel Functions Test Suite"
echo "═══════════════════════════════════════════════════════════════"
echo ""

echo "── pmap tests ──────────────────────────────────────────────────"

# Basic pmap tests
test_eq "pmap: simple multiply" \
    'map (\x x * 2) [1, 2, 3, 4, 5]' \
    'pmap (\x x * 2) [1, 2, 3, 4, 5]'

test_eq "pmap: identity function" \
    'map (\x x) [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]' \
    'pmap (\x x) [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]'

test_eq "pmap: empty list" \
    'map (\x x * 2) []' \
    'pmap (\x x * 2) []'

test_eq "pmap: single element" \
    'map (\x x + 100) [42]' \
    'pmap (\x x + 100) [42]'

test_eq "pmap: string operations" \
    'map upper ["hello", "world", "avon"]' \
    'pmap upper ["hello", "world", "avon"]'

test_eq "pmap: nested expression" \
    'map (\x x * x + x * 2 + 1) [1, 2, 3, 4, 5]' \
    'pmap (\x x * x + x * 2 + 1) [1, 2, 3, 4, 5]'

test_eq "pmap: with closure capturing variable" \
    'let m = 10 in map (\x x * m) [1, 2, 3, 4, 5]' \
    'let m = 10 in pmap (\x x * m) [1, 2, 3, 4, 5]'

test_eq "pmap: with multiple captured variables" \
    'let a = 2 in let b = 3 in map (\x x * a + b) [1, 2, 3]' \
    'let a = 2 in let b = 3 in pmap (\x x * a + b) [1, 2, 3]'

test_eq "pmap: with named function" \
    'let double = \x x * 2 in map double [1, 2, 3, 4, 5]' \
    'let double = \x x * 2 in pmap double [1, 2, 3, 4, 5]'

test_eq "pmap: boolean result" \
    'map (\x x > 3) [1, 2, 3, 4, 5]' \
    'pmap (\x x > 3) [1, 2, 3, 4, 5]'

test_eq "pmap: mixed types in list" \
    'map (\x to_string x) [1, "hello", true]' \
    'pmap (\x to_string x) [1, "hello", true]'

echo ""
echo "── pfilter tests ───────────────────────────────────────────────"

test_eq "pfilter: simple predicate" \
    'filter (\x x > 3) [1, 2, 3, 4, 5, 6]' \
    'pfilter (\x x > 3) [1, 2, 3, 4, 5, 6]'

test_eq "pfilter: empty list" \
    'filter (\x x > 0) []' \
    'pfilter (\x x > 0) []'

test_eq "pfilter: all match" \
    'filter (\x x > 0) [1, 2, 3, 4, 5]' \
    'pfilter (\x x > 0) [1, 2, 3, 4, 5]'

test_eq "pfilter: none match" \
    'filter (\x x > 100) [1, 2, 3, 4, 5]' \
    'pfilter (\x x > 100) [1, 2, 3, 4, 5]'

test_eq "pfilter: single element match" \
    'filter (\x x == 3) [1, 2, 3, 4, 5]' \
    'pfilter (\x x == 3) [1, 2, 3, 4, 5]'

test_eq "pfilter: even numbers" \
    'filter (\x x % 2 == 0) [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]' \
    'pfilter (\x x % 2 == 0) [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]'

test_eq "pfilter: with closure" \
    'let threshold = 5 in filter (\x x >= threshold) [1, 2, 3, 4, 5, 6, 7, 8]' \
    'let threshold = 5 in pfilter (\x x >= threshold) [1, 2, 3, 4, 5, 6, 7, 8]'

test_eq "pfilter: string predicate" \
    'filter (\s (length s) > 3) ["a", "abc", "abcd", "abcde"]' \
    'pfilter (\s (length s) > 3) ["a", "abc", "abcd", "abcde"]'

test_eq "pfilter: order preservation with complex predicate" \
    'filter (\x (x * x) % 2 == 1) [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]' \
    'pfilter (\x (x * x) % 2 == 1) [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]'

echo ""
echo "── pfold tests (associative operations only) ───────────────────"

test_eq "pfold: sum (associative)" \
    'fold (\a \b a + b) 0 [1, 2, 3, 4, 5]' \
    'pfold (\a \b a + b) 0 [1, 2, 3, 4, 5]'

test_eq "pfold: product (associative)" \
    'fold (\a \b a * b) 1 [1, 2, 3, 4, 5]' \
    'pfold (\a \b a * b) 1 [1, 2, 3, 4, 5]'

test_eq "pfold: empty list" \
    'fold (\a \b a + b) 0 []' \
    'pfold (\a \b a + b) 0 []'

test_eq "pfold: single element" \
    'fold (\a \b a + b) 0 [42]' \
    'pfold (\a \b a + b) 0 [42]'

# Note: pfold requires associative operations. Non-associative ops give wrong results.
# This test uses pure addition which is associative.
test_eq "pfold: sum with range" \
    'fold (\a \b a + b) 0 (range 1 50)' \
    'pfold (\a \b a + b) 0 (range 1 50)'

test_eq "pfold: large sum" \
    'fold (\a \b a + b) 0 (range 1 100)' \
    'pfold (\a \b a + b) 0 (range 1 100)'

test_eq "pfold: string concat (associative)" \
    'fold (\a \b concat a b) "" ["a", "b", "c", "d", "e"]' \
    'pfold (\a \b concat a b) "" ["a", "b", "c", "d", "e"]'

echo ""
echo "── Error handling tests ────────────────────────────────────────"

# Test that errors are properly propagated
test_err() {
    local name="$1"
    local expr="$2"
    
    local result=$("$AVON" run "$expr" 2>&1)
    local exit_code=$?
    
    # Check if result contains error message (case insensitive)
    if echo "$result" | grep -qi "error\|mismatch\|unknown"; then
        echo -e "${GREEN}✓${NC} $name (error correctly raised)"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $name (expected error, got: $result)"
        ((FAIL++))
    fi
}

test_err "pmap: type error (not a list)" \
    'pmap (\x x * 2) "not a list"'

test_err "pfilter: type error (not a list)" \
    'pfilter (\x x > 0) 42'

test_err "pfold: type error (not a list)" \
    'pfold (\a \b a + b) 0 "not a list"'

test_err "pfilter: predicate returns non-bool" \
    'pfilter (\x x * 2) [1, 2, 3]'

echo ""
echo "── Additional edge case tests ─────────────────────────────────"

# Larger lists to stress test parallelism
test_eq "pmap: larger list (100 elements)" \
    'length (map (\x x * 2) (range 1 100))' \
    'length (pmap (\x x * 2) (range 1 100))'

test_eq "pfilter: larger list (100 elements)" \
    'filter (\x x % 3 == 0) (range 1 100)' \
    'pfilter (\x x % 3 == 0) (range 1 100)'

# Test with nested data structures
test_eq "pmap: nested lists" \
    'map (\x head x) [[1, 2], [3, 4], [5, 6]]' \
    'pmap (\x head x) [[1, 2], [3, 4], [5, 6]]'

# Test with builtin functions
test_eq "pmap: using builtin function" \
    'map length ["a", "ab", "abc", "abcd"]' \
    'pmap length ["a", "ab", "abc", "abcd"]'

test_eq "pfilter: using builtin predicate" \
    'filter is_string [1, "a", 2, "b", 3]' \
    'pfilter is_string [1, "a", 2, "b", 3]'

# Test order preservation with many elements
test_eq "pmap: order preserved (20 elements)" \
    'map (\x x) (range 1 20)' \
    'pmap (\x x) (range 1 20)'

test_eq "pfilter: order preserved (evens from 20)" \
    'filter (\x x % 2 == 0) (range 1 20)' \
    'pfilter (\x x % 2 == 0) (range 1 20)'

# Test with deeply nested closures  
test_eq "pmap: deeply nested closure" \
    'let a = 1 in let b = 2 in let c = 3 in map (\x x + a + b + c) [10, 20, 30]' \
    'let a = 1 in let b = 2 in let c = 3 in pmap (\x x + a + b + c) [10, 20, 30]'

echo ""
echo "── Known limitations (documented behavior) ────────────────────"

# This test documents that pfold gives DIFFERENT results for non-associative ops
test_diff() {
    local name="$1"
    local seq_expr="$2"
    local par_expr="$3"
    
    local seq_result=$("$AVON" run "$seq_expr" 2>&1)
    local par_result=$("$AVON" run "$par_expr" 2>&1)
    
    if [ "$seq_result" != "$par_result" ]; then
        echo -e "${YELLOW}⚠${NC} $name (results differ as documented)"
        echo "    Sequential: $seq_result"
        echo "    Parallel:   $par_result"
        ((PASS++))  # This is expected behavior
    else
        echo -e "${RED}✗${NC} $name (expected different results but got same)"
        ((FAIL++))
    fi
}

test_diff "pfold: subtraction is NOT associative" \
    'fold (\a \b a - b) 0 [1, 2, 3, 4, 5]' \
    'pfold (\a \b a - b) 0 [1, 2, 3, 4, 5]'

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo -e "Results: ${GREEN}$PASS passed${NC}, ${RED}$FAIL failed${NC}"
echo "═══════════════════════════════════════════════════════════════"

if [ $FAIL -gt 0 ]; then
    exit 1
fi
