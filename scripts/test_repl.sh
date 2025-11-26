#!/bin/bash
# Test script for REPL functionality
# This tests the REPL by sending commands via stdin

AVON="./target/debug/avon"
TEST_INPUT="test_repl_input.txt"

# Create test input
cat > "$TEST_INPUT" << 'EOF'
1 + 2
let x = 42 in x * 2
map (\x x * 2) [1, 2, 3]
typeof "hello"
:vars
:type [1, 2, 3]
:help
:exit
EOF

echo "Testing REPL with sample commands..."
timeout 5 "$AVON" repl < "$TEST_INPUT" > /tmp/repl_test_output.txt 2>&1

if [ $? -eq 0 ] || [ $? -eq 124 ]; then
    echo "REPL test completed"
    if grep -q "84" /tmp/repl_test_output.txt; then
        echo "PASS: Expression evaluation works"
    else
        echo "FAIL: Expression evaluation may not work"
        cat /tmp/repl_test_output.txt
        exit 1
    fi
else
    echo "REPL test failed"
    cat /tmp/repl_test_output.txt
    exit 1
fi

rm -f "$TEST_INPUT" /tmp/repl_test_output.txt
echo "REPL basic test passed"

