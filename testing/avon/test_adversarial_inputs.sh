#!/bin/bash
# Adversarial inputs for Avon: malformed programs, invalid operations, and
# bounded resource-pressure cases must fail safely or produce stable results.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

PASSED=0
FAILED=0

pass() {
    echo "✓ $1"
    ((PASSED++))
}

fail() {
    echo "✗ $1"
    echo "  $2"
    ((FAILED++))
}

assert_rejected() {
    local name="$1"
    local code="$2"
    local output
    local status

    output=$(timeout 5 "$AVON" run "$code" 2>&1)
    status=$?

    if [ "$status" -eq 124 ]; then
        fail "$name" "timed out instead of rejecting the input"
    elif [ "$status" -ne 0 ] || echo "$output" | grep -qiE 'error|expected|unexpected|unimplemented|not allowed|traversal|limit exceeded'; then
        pass "$name"
    else
        fail "$name" "unexpectedly succeeded: $output"
    fi
}

assert_output() {
    local name="$1"
    local code="$2"
    local expected="$3"
    local output
    local status

    output=$(timeout 5 "$AVON" run "$code" 2>&1)
    status=$?

    if [ "$status" -eq 0 ] && [ "$output" = "$expected" ]; then
        pass "$name"
    else
        fail "$name" "expected '$expected', got '$output' (exit $status)"
    fi
}

assert_deploy_rejected() {
    local name="$1"
    local code="$2"
    local test_dir
    local output
    local status

    test_dir=$(mktemp -d)
    printf '%s\n' "$code" > "$test_dir/input.av"
    output=$(timeout 5 "$AVON" deploy "$test_dir/input.av" --root "$test_dir/root" 2>&1)
    status=$?
    rm -rf "$test_dir"

    if [ "$status" -eq 124 ]; then
        fail "$name" "timed out instead of rejecting the deployment"
    elif [ "$status" -ne 0 ] || echo "$output" | grep -qiE 'error|not allowed|traversal'; then
        pass "$name"
    else
        fail "$name" "deployment unexpectedly succeeded: $output"
    fi
}

make_deep_lets() {
    local depth="$1"
    local program="let v0 = 0 in"
    local i

    for ((i = 1; i <= depth; i++)); do
        program+=" let v$i = v$((i - 1)) in"
    done

    printf '%s v%s' "$program" "$depth"
}

echo "── Adversarial parser and lexer inputs ──"
assert_rejected "Unterminated string" '"unterminated'
assert_rejected "Unterminated template" '{"unterminated'
assert_rejected "Unterminated interpolation" '{"value: {1 + 2"}'
assert_rejected "Mismatched template delimiters" '{{"value"}'
assert_rejected "Unexpected closing delimiters" '])}'
assert_rejected "Incomplete lambda body" '\x'
assert_rejected "Incomplete conditional" 'if true then 1'
assert_rejected "Malformed dictionary entry" '{name "avon"}'

# Deployments must not allow template syntax or publish() to escape the root.
assert_deploy_rejected "Relative deployment path traversal" '@../outside.txt {"nope"}'
assert_deploy_rejected "Absolute deployment path" '@/tmp/outside.txt {"nope"}'
assert_deploy_rejected "Publish path traversal" 'publish "../outside.txt" "nope"'

echo "── Adversarial runtime inputs ──"
assert_rejected "Division by zero" '1 / 0'
assert_rejected "Integer division by zero" '1 // 0'
assert_rejected "Modulo by zero" '1 % 0'
assert_rejected "Invalid regular expression" 'regex_match "[" "input"'
assert_rejected "Unknown identifier" 'this_symbol_does_not_exist'
assert_rejected "Calling a number" '42 1'
assert_rejected "Invalid map callback" 'map 42 [1, 2, 3]'
assert_rejected "Invalid dictionary field" '({name: "avon"}).missing'

# Evaluation must reject excessive nesting instead of overflowing or hanging.
assert_rejected "Evaluation depth limit" "$(make_deep_lets 250)"

# A moderately large, valid pipeline guards against accidental quadratic behavior
# while ensuring valid high-volume expressions continue to complete.
assert_output "Bounded high-volume map pipeline" \
    'range 1 10000 -> map (\x x + 1) -> length' \
    '10000'

# Unicode, embedded newlines, and templates with a first-line interpolation must
# remain valid under adversarial-looking text and whitespace.
assert_output "Unicode and escaped text remain stable" \
    '{"🚀 {"line 1\\nline 2"}"}' \
    '🚀 line 1\nline 2'

printf '\nAdversarial input results: %d passed, %d failed\n' "$PASSED" "$FAILED"

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi
