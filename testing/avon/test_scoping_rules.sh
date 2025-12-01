#!/bin/bash
# Comprehensive scoping rules test
# Verifies all scoping behaviors documented in the tutorial

set +e

AVON="./target/debug/avon"
PASSED=0
FAILED=0

test_scope() {
    local name=$1
    local code=$2
    local expected=$3
    local file="/tmp/test_scope_${name}.av"
    
    echo "$code" > "$file"
    local result=$("$AVON" eval "$file" 2>&1)
    local exit_code=$?
    
    if [ $exit_code -ne 0 ]; then
        echo "✗ $name (evaluation failed)"
        echo "$result" | head -3
        ((FAILED++))
        return 1
    fi
    
    if [ -n "$expected" ]; then
        if echo "$result" | grep -q "$expected"; then
            echo "✓ $name"
            ((PASSED++))
            return 0
        else
            echo "✗ $name (expected: $expected, got: $(echo "$result" | head -1))"
            ((FAILED++))
            return 1
        fi
    else
        echo "✓ $name: $(echo "$result" | head -1)"
        ((PASSED++))
        return 0
    fi
}

echo "Testing Scoping Rules..."
echo "========================"
echo ""

# Test 1: Basic scoping
test_scope "basic" 'let x = 10 in x * 2' "20"

# Test 2: Cascading lets
test_scope "cascading" 'let a = "A" in let b = "B" in concat a b' "AB"

# Test 3: Nested let - temp not visible outside
test_scope "nested_temp" 'let x = 10 in let y = 20 in let result = let temp = x + y in temp * 2 in result' "60"

# Test 4: Shadowing prevention (should fail)
echo 'let x = 1 in let y = 2 in let inner = let x = 10 in x + y in let outer = x + y in [inner, outer]' > /tmp/test_scope_shadowing.av
if "$AVON" eval /tmp/test_scope_shadowing.av > /dev/null 2>&1; then
    echo "✗ shadowing (should have failed but didn't)"
    ((FAILED++))
else
    echo "✓ shadowing (correctly prevents shadowing)"
    ((PASSED++))
fi

# Test 5: Template variable capture (no shadowing needed)
test_scope "template_capture" 'let name = "Alice" in let template = {"Hello, {name}"} in template' "Hello, Alice"

# Test 6: Function closure (no shadowing needed)
test_scope "function_closure" 'let x = 10 in let add_x = \y x + y in add_x 5' "15"

# Test 7: Forward reference should fail
echo 'let result = x + 1 in let x = 10 in result' > /tmp/test_scope_forward_ref.av
if "$AVON" eval /tmp/test_scope_forward_ref.av > /dev/null 2>&1; then
    echo "✗ forward_ref (should have failed but didn't)"
    ((FAILED++))
else
    echo "✓ forward_ref (correctly fails on forward reference)"
    ((PASSED++))
fi

# Test 8: Multiple nesting levels (no shadowing)
test_scope "multiple_nesting" 'let a = 1 in let b = 2 in let c = let temp_a = 10 in let temp_b = 20 in temp_a + temp_b in let d = a + b in [c, d]' "30"

# Test 9: Let in function
test_scope "let_in_function" 'let make_adder = \base let offset = 5 in \x base + offset + x in let add10 = make_adder 10 in add10 3' "18"

echo ""
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    exit 0
else
    exit 1
fi

