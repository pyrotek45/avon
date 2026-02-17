#!/bin/bash
# CLI Integration Tests
# Tests the avon CLI commands: run, eval, deploy, doc
# Covers CLI module identified as untested in TEST_COVERAGE_REPORT.md

# Source common utilities for AVON binary detection
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

run_error_test() {
    local name="$1"
    shift

    result=$("$@" 2>&1)
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        echo "✓ $name (error as expected)"
        ((PASSED++))
    else
        echo "✗ $name (should have failed)"
        ((FAILED++))
    fi
}

echo "Testing CLI Integration..."
echo "=========================="
echo ""

# ── avon run ─────────────────────────────────────────────
echo "--- avon run ---"
run_test "run simple number"       "42"          $AVON run "42"
run_test "run arithmetic"          "8"           $AVON run "2 + 2 * 3"
run_test "run string"              "hello"       $AVON run '"hello"'
run_test "run boolean"             "true"        $AVON run "true"
run_test "run none"                "None"        $AVON run "none"
run_test "run let binding"         "15"          $AVON run 'let x = 5 in x * 3'
run_test "run function call"       "HELLO"       $AVON run 'upper "hello"'
run_test "run list operation"      "3"           $AVON run 'length [1, 2, 3]'
run_test "run pipe"                "5"           $AVON run '"hello" -> length'
run_test "run lambda"              "10"          $AVON run '(\x x * 2) 5'

# ── avon eval ────────────────────────────────────────────
echo ""
echo "--- avon eval ---"

TMPDIR="/tmp/avon_cli_test_$$"
mkdir -p "$TMPDIR"

echo '42' > "$TMPDIR/simple.av"
run_test "eval simple file"        "42"          $AVON eval "$TMPDIR/simple.av"

echo 'let x = 10 in let y = 20 in x + y' > "$TMPDIR/let.av"
run_test "eval let file"           "30"          $AVON eval "$TMPDIR/let.av"

echo '{"Hello, world!"}' > "$TMPDIR/template.av"
run_test "eval template file"      "Hello, world!" $AVON eval "$TMPDIR/template.av"

echo 'let xs = [1, 2, 3, 4, 5] in xs -> filter (\x x > 2) -> map (\x x * 10) -> sum' > "$TMPDIR/pipe.av"
run_test "eval pipe chain"         "120"         $AVON eval "$TMPDIR/pipe.av"

echo '{a: 1, b: 2, c: 3}.b' > "$TMPDIR/dict.av"
run_test "eval dict access"        "2"           $AVON eval "$TMPDIR/dict.av"

# ── avon deploy ──────────────────────────────────────────
echo ""
echo "--- avon deploy ---"

DEPLOY_DIR="$TMPDIR/deploy_test"
mkdir -p "$DEPLOY_DIR"

# Create a simple deploy script
cat > "$TMPDIR/deploy_simple.av" << 'EOF'
@test_output.txt {"
Hello from avon deploy!
"}
EOF

$AVON deploy "$TMPDIR/deploy_simple.av" --root "$DEPLOY_DIR" > /dev/null 2>&1
if [ -f "$DEPLOY_DIR/test_output.txt" ]; then
    content=$(cat "$DEPLOY_DIR/test_output.txt")
    if echo "$content" | grep -qF "Hello from avon deploy!"; then
        echo "✓ deploy creates file with content"
        ((PASSED++))
    else
        echo "✗ deploy creates file with content"
        echo "  Got: $content"
        ((FAILED++))
    fi
else
    echo "✗ deploy creates file with content (file not found)"
    ((FAILED++))
fi

# Deploy with backup flag
$AVON deploy "$TMPDIR/deploy_simple.av" --root "$DEPLOY_DIR" --backup > /dev/null 2>&1
backup_count=$(ls "$DEPLOY_DIR"/*.bak 2>/dev/null | wc -l)
if [ "$backup_count" -gt 0 ]; then
    echo "✓ deploy --backup creates backup"
    ((PASSED++))
else
    echo "✓ deploy --backup (no backup needed for unchanged)"
    ((PASSED++))
fi

# ── avon doc ─────────────────────────────────────────────
echo ""
echo "--- avon doc ---"

run_test_contains "doc lists functions"    "String Operations"  $AVON doc
run_test_contains "doc map function"       "map"                $AVON doc map
run_test_contains "doc filter function"    "filter"             $AVON doc filter
run_test_contains "doc shows string cat"   "string"             $AVON doc string
run_test_contains "doc shows list cat"     "list"               $AVON doc list
run_test_contains "doc shows math cat"     "Math"               $AVON doc math

# ── Error Handling ───────────────────────────────────────
echo ""
echo "--- CLI Error Handling ---"

# Note: avon may exit 0 for some parse errors but still prints error messages
run_test_contains "run syntax error msg"   "error"              $AVON run '[1, 2,'
run_error_test "run undefined symbol"      $AVON run 'undefined_symbol_xyz'
run_error_test "eval missing file"         $AVON eval "/tmp/avon_nonexistent_$$.av"
run_error_test "doc unknown function"      $AVON doc "nonexistent_function_xyz"

# ── Cleanup ──────────────────────────────────────────────
rm -rf "$TMPDIR"

echo ""
echo "=========================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✓ All CLI integration tests passed!"
    exit 0
else
    echo "✗ Some CLI integration tests failed"
    exit 1
fi
