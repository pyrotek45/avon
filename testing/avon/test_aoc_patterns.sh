#!/bin/bash
# Test AoC 2024 Day 15 pattern detection - Find product IDs that are patterns repeated twice

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"
AVON_BIN="$AVON"

TEST_FILE="/tmp/test_aoc_patterns.av"

cat > "$TEST_FILE" << 'EOF'
# Pattern detection: Check if a number is a pattern repeated exactly twice
# For example: 55 = 5 repeated twice, 6464 = 64 repeated twice, 123123 = 123 repeated twice
# Mathematical approach: num % (10^half_len + 1) == 0

let is_invalid_id = \num
  let str = to_string num in
  let len = length str in
  if len % 2 != 0 then false
  else
    let half_len = len / 2 in
    let pow10 = fold (\acc \_ acc * 10) 1 (range 0 (half_len - 1)) in
    let divisor = pow10 + 1 in
    num % divisor == 0
in

# Test single-digit repeats (11, 22, 33, etc.)
let test_single_digit = (
    let cases = [11, 22, 33, 44, 55, 66, 77, 88, 99] in
    let results = map is_invalid_id cases in
    let all_true = fold (\acc \v acc && v) true results in
    all_true
) in

# Test double-digit repeats (1111, 2222, 6464, 7777, 9191, etc.)
let test_double_digit = (
    let cases = [1111, 2222, 6464, 7777, 9191] in
    let results = map is_invalid_id cases in
    let all_true = fold (\acc \v acc && v) true results in
    all_true
) in

# Test triple-digit repeats (123123, 999999, 555555, 111111, 456456)
let test_triple_digit = (
    let cases = [123123, 999999, 555555, 111111, 456456] in
    let results = map is_invalid_id cases in
    let all_true = fold (\acc \v acc && v) true results in
    all_true
) in

# Test non-repeats (should all be false)
let test_non_repeats = (
    let cases = [10, 12, 123, 1234, 12345, 100, 123456] in
    let results = map is_invalid_id cases in
    let all_false = fold (\acc \v acc && (not v)) true results in
    all_false
) in

# Test specific known invalid IDs from AoC challenge
let test_known_invalid = (
    let known = [11, 22, 55, 99, 6464, 123123] in
    let results = map is_invalid_id known in
    let all_true = fold (\acc \v acc && v) true results in
    all_true
) in

# Test specific known valid IDs (not patterns) - use numbers that truly aren't patterns
let test_known_valid = (
    let known = [10, 12, 50, 95, 96, 97, 98, 100, 1000, 1001] in
    let results = map is_invalid_id known in
    let all_false = fold (\acc \v acc && (not v)) true results in
    all_false
) in

# Test range-based detection (like AoC challenge uses)
let test_range_detection = (
    let invalid_in_range = filter is_invalid_id (range 11 22) in
    invalid_in_range == [11, 22]
) in

# Test edge cases - 1010 IS actually a pattern (10+10), so it should be invalid
let test_edge_cases = (
    let check_1 = (not (is_invalid_id 1)) in
    let check_10_single = (not (is_invalid_id 10)) in
    let check_101 = (not (is_invalid_id 101)) in
    let check_1010 = (is_invalid_id 1010) in
    let check_1111 = (is_invalid_id 1111) in
    let check_111111 = (is_invalid_id 111111) in
    check_1 && check_10_single && check_101 && check_1010 && check_1111 && check_111111
) in

# Run all tests and return summary
let results = [
    test_single_digit,
    test_double_digit,
    test_triple_digit,
    test_non_repeats,
    test_known_invalid,
    test_known_valid,
    test_range_detection,
    test_edge_cases
] in

# Count passes
let pass_count = fold (\acc \v if v then acc + 1 else acc) 0 results in
let fail_count = fold (\acc \v if v then acc else acc + 1) 0 results in

# Report results
if fail_count == 0 then
    trace "PASS" (concat "All 8 tests passed" "")
else
    trace "FAIL" (concat "Tests failed: " (to_string fail_count))
EOF

# Run the test
OUTPUT=$($AVON_BIN "$TEST_FILE" 2>&1)
EXIT_CODE=$?

# Clean up
rm "$TEST_FILE"

# Check results
if [ $EXIT_CODE -ne 0 ]; then
    echo "Avon exited with error code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

if echo "$OUTPUT" | grep -q "FAIL"; then
    echo "AoC Pattern Detection Tests failed:"
    echo "$OUTPUT"
    exit 1
fi

# Check for PASS
if echo "$OUTPUT" | grep -q "PASS"; then
    echo "✓ All AoC pattern detection tests passed"
    exit 0
else
    echo "AoC Pattern Detection Tests - No pass found"
    echo "$OUTPUT"
    exit 1
fi
