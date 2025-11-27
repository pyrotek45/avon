#!/bin/bash
# Test script for REPL functionality
# This tests the REPL by sending commands via stdin

AVON="./target/debug/avon"
TEST_INPUT="test_repl_input.txt"
OUTPUT_FILE="/tmp/repl_test_output.txt"

# Helper function to test REPL commands
test_repl_command() {
    local input="$1"
    local expected="$2"
    local test_name="$3"
    
    echo "$input" | timeout 5 "$AVON" repl > "$OUTPUT_FILE" 2>&1
    
    if grep -q "$expected" "$OUTPUT_FILE"; then
        echo "✓ PASS: $test_name"
        return 0
    else
        echo "✗ FAIL: $test_name"
        echo "  Expected: $expected"
        echo "  Output:"
        cat "$OUTPUT_FILE" | head -10
        return 1
    fi
}

PASSED=0
FAILED=0

echo "Testing REPL functionality..."

# Test basic expression evaluation
if test_repl_command "1 + 2
:exit" "3"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :let command
if test_repl_command ":let x = 42
:exit" "Stored: x"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :vars command
if test_repl_command ":let x = 42
:vars
:exit" "x : Number"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :inspect command
if test_repl_command ":let x = 42
:inspect x
:exit" "Variable: x"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :unlet command
if test_repl_command ":let x = 42
:unlet x
:vars
:exit" "No user-defined variables"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test variable persistence
if test_repl_command ":let double = \x x * 2
double 21
:exit" "42"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :help includes new commands
if test_repl_command ":help
:exit" ":let"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :read command
echo "test content" > /tmp/test_read.txt
if test_repl_command ":read /tmp/test_read.txt
:exit" "test content"; then
    ((PASSED++))
else
    ((FAILED++))
fi
rm -f /tmp/test_read.txt

# Test :write command
if test_repl_command ":write /tmp/test_write.txt \"Hello, World!\"
:exit" "Written to"; then
    ((PASSED++))
    if [ -f /tmp/test_write.txt ] && grep -q "Hello, World!" /tmp/test_write.txt; then
        ((PASSED++))
        rm -f /tmp/test_write.txt
    else
        echo "✗ FAIL: :write file content verification"
        ((FAILED++))
    fi
else
    ((FAILED++))
fi

# Test :save-session command
if test_repl_command ":let x = 42
:let y = \"hello\"
:save-session /tmp/test_session.avon
:exit" "Session saved"; then
    ((PASSED++))
    if [ -f /tmp/test_session.avon ]; then
        ((PASSED++))
    else
        echo "✗ FAIL: :save-session file creation"
        ((FAILED++))
    fi
else
    ((FAILED++))
fi

# Test :load-session command (need to create session first)
echo -e ":let x = 42\n:let y = \"hello\"\n:save-session /tmp/test_session_load.avon\n:exit" | timeout 5 "$AVON" repl > /dev/null 2>&1
if test_repl_command ":clear
:load-session /tmp/test_session_load.avon
:vars
:exit" "x : Number"; then
    ((PASSED++))
else
    echo "  Note: Session file may need proper evaluation - checking file format..."
    cat /tmp/test_session_load.avon 2>/dev/null | head -5
    ((FAILED++))
fi
rm -f /tmp/test_session_load.avon

# Test :benchmark command (expression)
if test_repl_command ":benchmark 1 + 1
:exit" "Time:"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :benchmark-file command (file)
echo "42" > /tmp/test_benchmark.av
if test_repl_command ":benchmark-file /tmp/test_benchmark.av
:exit" "Time:"; then
    ((PASSED++))
else
    ((FAILED++))
fi
rm -f /tmp/test_benchmark.av

# Test :test command
if test_repl_command ":test (1 + 1) 2
:exit" "PASS"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :assert command
if test_repl_command ":assert (1 == 1)
:exit" "PASS"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :pwd command
if test_repl_command ":pwd
:exit" "/"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :list command
if test_repl_command ":list
:exit" "Current directory:"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :doc command (builtin)
if test_repl_command ":doc map
:exit" "map ::"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :doc command (REPL command)
if test_repl_command ":doc pwd
:exit" ":pwd"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :sh command
if test_repl_command ":sh echo test123
:exit" "test123"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Test :watch command
if test_repl_command ":let x = 1
:watch x
:let x = 2
:exit" "Watching:"; then
    ((PASSED++))
else
    ((FAILED++))
fi

# Cleanup
rm -f "$TEST_INPUT" "$OUTPUT_FILE" /tmp/test_*.txt /tmp/test_*.avon /tmp/test_*.av

echo ""
echo "Test Summary:"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"

if [ $FAILED -eq 0 ]; then
    echo "✓ All REPL tests passed"
    exit 0
else
    echo "✗ Some REPL tests failed"
    exit 1
fi

