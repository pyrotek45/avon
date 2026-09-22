#!/bin/bash
# Tests for new builtin functions: file_size, file_is_dir, file_is_file, file_mtime, lines_grep, whoami, hostname, random_range

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

PASSED=0
FAILED=0

run_test() {
    local name="$1"
    local expected="$2"
    shift 2
    
    result=$("$@" 2>&1) || true
    if [ "$result" = "$expected" ]; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected: $expected"
        echo "  Got:      $result"
        ((FAILED++))
    fi
}

run_test_contains() {
    local name="$1"
    local expected="$2"
    shift 2

    result=$("$@" 2>&1) || true
    if echo "$result" | grep -qF "$expected"; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected substring: $expected"
        echo "  Got (first 5 lines): $(echo "$result" | head -5)"
        ((FAILED++))
    fi
}

run_test_regex() {
    local name="$1"
    local pattern="$2"
    shift 2

    result=$("$@" 2>&1) || true
    if echo "$result" | grep -qE "$pattern"; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected pattern: $pattern"
        echo "  Got: $result"
        ((FAILED++))
    fi
}

# Create temporary test files
TEST_FILE="/tmp/avon_test_file.txt"
TEST_FILE_CONTENT="test content here"
printf "%s" "$TEST_FILE_CONTENT" > "$TEST_FILE"  # Use printf to avoid adding newline

TEST_GREP_FILE="/tmp/avon_test_grep.txt"
printf "foo\nbar\nbaz\nfoo123\nbarfoo\n" > "$TEST_GREP_FILE"

echo "=== File Operation Tests ==="

# Test file_size
SIZE=$("$AVON" "file_size \"$TEST_FILE\"" 2>&1)
if [ "$SIZE" = "${#TEST_FILE_CONTENT}" ]; then
    echo "✓ file_size returns correct byte count"
    ((PASSED++))
else
    echo "✗ file_size - expected ${#TEST_FILE_CONTENT}, got $SIZE"
    ((FAILED++))
fi

# Test file_is_file
run_test "file_is_file for file" "true" "$AVON" "file_is_file \"$TEST_FILE\""
run_test "file_is_file for directory" "false" "$AVON" "file_is_file \"src\""

# Test file_is_dir
run_test "file_is_dir for directory" "true" "$AVON" "file_is_dir \"src\""
run_test "file_is_dir for file" "false" "$AVON" "file_is_dir \"README.md\""

# Test file_mtime - just check it returns a number
run_test_regex "file_mtime returns timestamp" "^[0-9]+$" "$AVON" "file_mtime \"$TEST_FILE\""

echo "=== String/Regex Tests ==="

# Test lines_grep with substring
result=$("$AVON" "lines_grep \"$TEST_GREP_FILE\" \"foo\"" 2>&1)
if echo "$result" | grep -qF "foo" && echo "$result" | grep -qF "foo123" && echo "$result" | grep -qF "barfoo"; then
    echo "✓ lines_grep finds lines containing substring"
    ((PASSED++))
else
    echo "✗ lines_grep substring test - got: $result"
    ((FAILED++))
fi

# Test lines_grep with regex (lines starting with foo)
result=$("$AVON" 'lines_grep "'$TEST_GREP_FILE'" "^foo"' 2>&1)
if echo "$result" | grep -qF "foo" && ! echo "$result" | grep -qF "barfoo"; then
    echo "✓ lines_grep with regex pattern"
    ((PASSED++))
else
    echo "✗ lines_grep regex test - got: $result"
    ((FAILED++))
fi

echo "=== System Function Tests ==="

# Test whoami - just check it returns a non-empty string
WHOAMI_RESULT=$("$AVON" "whoami" 2>&1)
if [ -n "$WHOAMI_RESULT" ] && [ "$WHOAMI_RESULT" != "<builtin:whoami>" ]; then
    echo "✓ whoami returns username"
    ((PASSED++))
else
    echo "✗ whoami - got: $WHOAMI_RESULT"
    ((FAILED++))
fi

# Test random_range - check that results are in range
echo "Testing random_range (5 iterations)..."
for i in {1..5}; do
    RAND_RESULT=$("$AVON" "random_range 1 10" 2>&1)
    if [[ $RAND_RESULT =~ ^[0-9]+$ ]] && [ "$RAND_RESULT" -ge 1 ] && [ "$RAND_RESULT" -le 10 ]; then
        ((PASSED++))
    else
        echo "✗ random_range iteration $i - got: $RAND_RESULT"
        ((FAILED++))
    fi
done
echo "✓ random_range produces values in range"

# Test random_range with same min and max
run_test "random_range with same min/max" "5" "$AVON" "random_range 5 5"

# Test hostname (may not have HOSTNAME env var set)
HOSTNAME_RESULT=$("$AVON" "hostname" 2>&1)
if echo "$HOSTNAME_RESULT" | grep -qE "^[a-zA-Z0-9-]+$|HOSTNAME"; then
    echo "✓ hostname works or returns helpful error"
    ((PASSED++))
else
    echo "✗ hostname - unexpected result: $HOSTNAME_RESULT"
    ((FAILED++))
fi

echo ""
echo "=== Test Summary ==="
echo "Passed: $PASSED"
echo "Failed: $FAILED"

# Cleanup
rm -f "$TEST_FILE" "$TEST_GREP_FILE"

exit $FAILED
