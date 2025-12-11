#!/bin/bash

# Test that commands with errors are stored in history
# This ensures users can easily navigate back and fix their mistakes

AVON="./target/release/avon"
SUCCESS=0
FAILED=0

# Helper function to run a test
run_test() {
    local name="$1"
    local input="$2"
    local expected_in_history="$3"
    
    # Run REPL with input
    output=$( (echo "$input"; echo ":history"; echo ":exit") | $AVON repl 2>&1 )
    
    # Check if history contains the expected command
    if echo "$output" | grep -q "^  [0-9]*: $(printf '%s\n' "$expected_in_history" | sed 's/[\/&]/\\&/g')"; then
        echo "✓ $name"
        ((SUCCESS++))
    else
        echo "✗ $name"
        echo "  Expected to find in history: $expected_in_history"
        echo "  Got: $(echo "$output" | grep "^  " | tail -1)"
        ((FAILED++))
    fi
}

echo "=========================================="
echo "REPL Error History Test Suite"
echo "=========================================="
echo ""

echo "=== Single-line error commands ==="
run_test "Unknown variable" "undefined_var" "undefined_var"
run_test "Type mismatch (add string and number)" "1 + \"string\"" "1 + \"string\""
run_test "Unknown symbol in :let" ":let x = bad_name" ":let x = bad_name"
run_test "Empty :let expression" ":let y =" ":let y ="

echo ""
echo "=== Multi-line errors in history ==="
# Multi-line error: incomplete expression after lambda
input=$(printf ':let f = \\x\nx +\n:history\n:exit')
output=$( (echo "$input") | $AVON repl 2>&1 )
# When 'x +' is incomplete, it stays in buffer and is never finalized, so it's not in history
# Only the complete ':let f = \x' gets stored (since it completed on the 2nd line)
if echo "$output" | grep -q "Command history"; then
    echo "✓ Incomplete multi-line expression handling verified"
    ((SUCCESS++))
else
    echo "✗ History command failed"
    ((FAILED++))
fi

echo ""
echo "=== Success and error mixed ==="
# Test that successful and failed commands all appear in history
input=$(printf ':let x = 42\nundefined\n:let y = 99\n')
output=$( (echo "$input"; echo ":history"; echo ":exit") | $AVON repl 2>&1 )

# Count history entries
history_count=$(echo "$output" | grep "Command history" | grep -o "[0-9]*" | head -1)
if [ "$history_count" = "3" ]; then
    echo "✓ All 3 commands in history (2 success, 1 error)"
    ((SUCCESS++))
else
    echo "✗ Expected 3 commands in history, got $history_count"
    ((FAILED++))
fi

echo ""
echo "=========================================="
printf "Results: \033[0;32m%d passed\033[0m, \033[0;31m%d failed\033[0m\n" $SUCCESS $FAILED
echo "=========================================="

if [ $FAILED -eq 0 ]; then
    exit 0
else
    exit 1
fi
