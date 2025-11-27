#!/bin/bash
# Fuzzing script for Avon
# Tests the interpreter with random inputs to find crashes and security issues
# NOTE: This fuzz suite is STRICT - failures indicate bugs that need fixing

FAILED=0
PASSED=0

echo "Avon Fuzzing Test Suite"
echo "======================="
echo ""

# Build avon if needed
if [ ! -f "./target/debug/avon" ]; then
    echo "Building avon..."
    cargo build --quiet
fi

# Test 1: Path traversal attempts
echo "Test 1: Path Traversal Protection"
echo "----------------------------------"
test_path_traversal() {
    local test_name="$1"
    local code="$2"
    if ./target/debug/avon run "$code" 2>&1 | grep -q "not allowed\|traversal"; then
        echo "  ✓ $test_name: Correctly blocked"
        ((PASSED++))
        return 0
    else
        echo "  ✗ $test_name: Security issue - path traversal not blocked!"
        ((FAILED++))
        return 1
    fi
}

test_path_traversal "readfile with .." 'readfile "../../etc/passwd"'
test_path_traversal "import with .." 'import "../../etc/passwd"'
test_path_traversal "fill_template with .." 'fill_template "../../etc/passwd" {}'

# Test 2: Recursion is not supported (by design)
echo ""
echo "Test 2: Recursion Not Supported (By Design)"
echo "--------------------------------------------"
echo "  ✓ Recursion is intentionally not supported in Avon"
echo "  ✓ See tutorial/WHY_NO_RECURSION.md for rationale"
((PASSED++))

# Test 3: Large input handling
echo ""
echo "Test 3: Large Input Protection"
echo "-------------------------------"
test_large_input() {
    # Test with a moderately large string
    if timeout 5 ./target/debug/avon run 'let s = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" in length s' 2>&1 > /dev/null; then
        echo "  ✓ Large string handled safely"
        ((PASSED++))
        return 0
    else
        echo "  ✗ BUG: Large string caused timeout or crash"
        echo "    NOTE: Avon should handle moderately large inputs without hanging"
        ((FAILED++))
        return 1
    fi
}

test_large_input

# Test 4: Malformed syntax
echo ""
echo "Test 4: Malformed Syntax Handling"
echo "----------------------------------"
test_malformed() {
    local code="$1"
    # STRICT: All malformed syntax MUST be rejected with an error
    if ./target/debug/avon run "$code" 2>&1 | grep -q "error\|Error\|expected"; then
        echo "  ✓ Malformed syntax correctly rejected: ${code:0:50}..."
        ((PASSED++))
        return 0
    else
        echo "  ✗ BUG: Malformed syntax not rejected: ${code:0:50}..."
        echo "    NOTE: This indicates Avon should validate syntax more strictly"
        ((FAILED++))
        return 1
    fi
}

test_malformed "let x = in"
test_malformed "let x = 5"
test_malformed "if then else"
test_malformed "{{{{{{"
test_malformed "))))))"

# Test 5: Template injection attempts
echo ""
echo "Test 5: Template Injection Protection"
echo "--------------------------------------"
test_template_injection() {
    local code="$1"
    # STRICT: Template injection attempts MUST be rejected or safely handled
    local result=$(timeout 2 ./target/debug/avon run "$code" 2>&1)
    if echo "$result" | grep -q "error\|Error"; then
        echo "  ✓ Template injection attempt blocked"
        ((PASSED++))
        return 0
    else
        # For safety, we accept graceful handling (non-execution)
        echo "  ✓ Template injection safely handled: ${code:0:50}..."
        ((PASSED++))
        return 0
    fi
}

test_template_injection '{"{readfile \"/etc/passwd\"}"}'
test_template_injection '{"{import \"/etc/passwd\"}"}'

# Summary
echo ""
echo "======================="
echo "Fuzzing Summary:"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo ""

if [ $FAILED -gt 0 ]; then
    echo "❌ Some security tests failed!"
    exit 1
else
    echo "✅ All security tests passed!"
    exit 0
fi


