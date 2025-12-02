#!/bin/bash

# Test script for Tier 1 AoC builtins: sum, min, max, all, any, count

cd /home/pyrotek45/projects/v7/avon

run_test() {
    local name=$1
    local expr=$2
    local expected=$3
    
    result=$(cargo run --quiet --release -- -c "$expr" 2>&1)
    if [ "$result" = "$expected" ]; then
        echo "✓ $name"
        return 0
    else
        echo "✗ $name"
        echo "  Expected: $expected"
        echo "  Got: $result"
        return 1
    fi
}

echo "=== Testing Tier 1 AoC Builtins ==="
echo ""

# Test sum
echo "sum tests:"
run_test "sum basic" "sum [1, 2, 3, 4, 5]" "15"
run_test "sum empty" "sum []" "0"
run_test "sum single" "sum [42]" "42"
run_test "sum floats" "sum [1.5, 2.5, 3.0]" "7"
echo ""

# Test min
echo "min tests:"
run_test "min numbers" "min [3, 1, 4, 1, 5]" "1"
run_test "min strings" "min [\"zebra\", \"apple\", \"banana\"]" "apple"
run_test "min mixed int/float" "min [3.5, 1, 4.2]" "1"
run_test "min empty" "min []" "none"
echo ""

# Test max
echo "max tests:"
run_test "max numbers" "max [3, 1, 4, 1, 5]" "5"
run_test "max strings" "max [\"zebra\", \"apple\", \"banana\"]" "zebra"
run_test "max mixed int/float" "max [3.5, 1, 4.2]" "4.2"
run_test "max empty" "max []" "none"
echo ""

# Test all
echo "all tests:"
run_test "all true" "all (\\x x > 0) [1, 2, 3]" "true"
run_test "all false" "all (\\x x > 0) [1, -2, 3]" "false"
run_test "all empty" "all (\\x x > 0) []" "true"
echo ""

# Test any
echo "any tests:"
run_test "any true" "any (\\x x < 0) [1, 2, -3]" "true"
run_test "any false" "any (\\x x < 0) [1, 2, 3]" "false"
run_test "any empty" "any (\\x x < 0) []" "false"
echo ""

# Test count
echo "count tests:"
run_test "count basic" "count (\\x x > 5) [1, 6, 3, 8, 5]" "2"
run_test "count strings" "count (\\x x == \"a\") [\"a\", \"b\", \"a\"]" "2"
run_test "count none match" "count (\\x x > 10) [1, 2, 3]" "0"
run_test "count all match" "count (\\x x > 0) [1, 2, 3]" "3"
echo ""

echo "=== All Tests Complete ==="
