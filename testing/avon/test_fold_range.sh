#!/bin/bash
# Test fold with range iteration to ensure correct behavior with inclusive ranges

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"
AVON_BIN="$AVON"

TEST_FILE="/tmp/test_fold_range.av"

cat > "$TEST_FILE" << 'EOF'
# Important: range is inclusive on BOTH ends
# range 0 2 produces [0, 1, 2] with 3 elements
# This is critical for fold iterations

# Test 1: Verify range produces correct elements
let test_range_inclusive = (
    let r1 = range 0 2 in
    let r2 = range 1 5 in
    r1 == [0, 1, 2] && r2 == [1, 2, 3, 4, 5]
) in

# Test 2: fold with range to count iterations
let test_fold_count = (
    let count_1 = fold (\acc \_ acc + 1) 0 (range 0 0) in
    let count_2 = fold (\acc \_ acc + 1) 0 (range 0 1) in
    let count_3 = fold (\acc \_ acc + 1) 0 (range 0 2) in
    let count_5 = fold (\acc \_ acc + 1) 0 (range 1 5) in
    count_1 == 1 && count_2 == 2 && count_3 == 3 && count_5 == 5
) in

# Test 3: Computing powers with fold and range
# 10^n requires n multiplications (not n+1)
# So for 10^1, we need 1 iteration: start with 1, multiply by 10 once
# For 10^2, we need 2 iterations: start with 1, multiply by 10 twice
# This means: fold (\acc \_ acc * 10) 1 (range 0 (n-1))
let test_power_calculation = (
    let pow10_1 = fold (\acc \_ acc * 10) 1 (range 0 0) in
    let pow10_2 = fold (\acc \_ acc * 10) 1 (range 0 1) in
    let pow10_3 = fold (\acc \_ acc * 10) 1 (range 0 2) in
    let pow10_4 = fold (\acc \_ acc * 10) 1 (range 0 3) in
    pow10_1 == 10 && pow10_2 == 100 && pow10_3 == 1000 && pow10_4 == 10000
) in

# Test 4: Direct range length calculation
let test_range_length = (
    let len_0_0 = length (range 0 0) in
    let len_0_1 = length (range 0 1) in
    let len_0_2 = length (range 0 2) in
    let len_1_5 = length (range 1 5) in
    len_0_0 == 1 && len_0_1 == 2 && len_0_2 == 3 && len_1_5 == 5
) in

# Test 5: Modulo divisor calculation for AoC pattern detection
# For a number to be pattern+pattern of length n:
# len / 2 = half_len, divisor = 10^half_len + 1
# For pattern length 1 (55 = 5+5): half_len=1, divisor = 10^1+1 = 11, 55%11==0 ✓
# For pattern length 2 (6464 = 64+64): half_len=2, divisor = 10^2+1 = 101, 6464%101==0 ✓
let test_aoc_divisor_calc = (
    let calc_divisor = \half_len
        let pow10 = fold (\acc \_ acc * 10) 1 (range 0 (half_len - 1)) in
        pow10 + 1
    in
    let div_1 = calc_divisor 1 in
    let div_2 = calc_divisor 2 in
    let div_3 = calc_divisor 3 in
    div_1 == 11 && div_2 == 101 && div_3 == 1001
) in

# Test 6: Verify AoC example patterns with calculated divisors
let test_aoc_patterns = (
    let check_pattern = \num
        let str = to_string num in
        let len = length str in
        if len % 2 != 0 then
            false
        else
            let half_len = len / 2 in
            let pow10 = fold (\acc \_ acc * 10) 1 (range 0 (half_len - 1)) in
            let divisor = pow10 + 1 in
            num % divisor == 0
    in
    let results = [
        check_pattern 55,
        check_pattern 6464,
        check_pattern 123123,
        not (check_pattern 56),
        not (check_pattern 123456)
    ] in
    let all_true = fold (\acc \v acc && v) true results in
    all_true
) in

# Run all tests and return summary
let results = [
    test_range_inclusive,
    test_fold_count,
    test_power_calculation,
    test_range_length,
    test_aoc_divisor_calc,
    test_aoc_patterns
] in

# Count passes
let pass_count = fold (\acc \v if v then acc + 1 else acc) 0 results in
let fail_count = fold (\acc \v if v then acc else acc + 1) 0 results in

# Report results
if fail_count == 0 then
    trace "PASS" (concat "All 6 tests passed" "")
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
    echo "Fold + Range Tests failed:"
    echo "$OUTPUT"
    exit 1
fi

# Check for PASS
if echo "$OUTPUT" | grep -q "PASS"; then
    echo "✓ All fold + range tests passed"
    exit 0
else
    echo "Fold + Range Tests - No pass found"
    echo "$OUTPUT"
    exit 1
fi
