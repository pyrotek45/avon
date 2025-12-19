#!/bin/bash
# Test arithmetic operations including overflow/underflow handling

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"
AVON_BIN="$AVON"

TEST_FILE="/tmp/test_arithmetic.av"

cat > "$TEST_FILE" << 'EOF'
# Test wrapping arithmetic
let max_i64 = 9223372036854775807 in
# Construct min_i64 to avoid parser literal overflow
let min_i64 = -9223372036854775807 - 1 in

# Chain tests using let bindings
let t1 = (
    let add_overflow = max_i64 + 1 in
    if add_overflow == min_i64 then
        trace "PASS: Addition overflow wrapped correctly" "ok"
    else
        trace ("FAIL: Addition overflow: " + (to_string add_overflow)) "fail"
) in

let t2 = (
    let sub_underflow = min_i64 - 1 in
    if sub_underflow == max_i64 then
        trace "PASS: Subtraction underflow wrapped correctly" "ok"
    else
        trace ("FAIL: Subtraction underflow: " + (to_string sub_underflow)) "fail"
) in

let t3 = (
    let mul_overflow = max_i64 * 2 in
    if mul_overflow == -2 then
        trace "PASS: Multiplication overflow wrapped correctly" "ok"
    else
        trace ("FAIL: Multiplication overflow: " + (to_string mul_overflow)) "fail"
) in

let t4 = (
    let div_edge = min_i64 // -1 in
    if div_edge == min_i64 then
        trace "PASS: Division edge case (MIN // -1) handled correctly" "ok"
    else
        trace ("FAIL: Division edge case: " + (to_string div_edge)) "fail"
) in

let t5 = (
    let neg_prec = 5 * -2 in
    if neg_prec == -10 then
        trace "PASS: 5 * -2 parsed correctly" "ok"
    else
        trace ("FAIL: 5 * -2 parsed incorrectly: " + (to_string neg_prec)) "fail"
) in

"DONE"
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
    echo "Tests failed:"
    echo "$OUTPUT"
    exit 1
fi

# Check that we have expected passes
PASS_COUNT=$(echo "$OUTPUT" | grep -c "PASS")
if [ "$PASS_COUNT" -ne 5 ]; then
    echo "Expected 5 passing tests, got $PASS_COUNT"
    echo "$OUTPUT"
    exit 1
fi

echo "All arithmetic tests passed"
exit 0
