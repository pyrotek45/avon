#!/bin/bash
# Do Mode Language Tests
# Tests that task file content evaluates correctly at the Avon language level.
# Covers: task dict evaluation, structured task fields, example file validation,
#         do mode CLI basics (execution, flags, errors, security)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

PASSED=0
FAILED=0

TMPDIR="/tmp/avon_do_lang_test_$$"
mkdir -p "$TMPDIR"

run_test() {
    local name="$1"
    local expr="$2"
    local expected="$3"

    result=$($AVON run "$expr" 2>&1) || true
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
    if echo "$result" | grep -qF -- "$expected"; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected substring: $expected"
        echo "  Got (first 5 lines): $(echo "$result" | head -5)"
        ((FAILED++))
    fi
}

run_test_not_contains() {
    local name="$1"
    local unexpected="$2"
    shift 2

    result=$("$@" 2>&1) || true
    if echo "$result" | grep -qF -- "$unexpected"; then
        echo "✗ $name"
        echo "  Should NOT contain: $unexpected"
        echo "  Got (first 5 lines): $(echo "$result" | head -5)"
        ((FAILED++))
    else
        echo "✓ $name"
        ((PASSED++))
    fi
}

run_error_test() {
    local name="$1"
    shift

    "$@" > /dev/null 2>&1
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        echo "✓ $name (exit=$exit_code)"
        ((PASSED++))
    else
        echo "✗ $name (should have failed, got exit=0)"
        ((FAILED++))
    fi
}

run_exit_test() {
    local name="$1"
    local expected_code="$2"
    shift 2

    "$@" > /dev/null 2>&1
    actual_code=$?
    if [ $actual_code -eq $expected_code ]; then
        echo "✓ $name (exit=$actual_code)"
        ((PASSED++))
    else
        echo "✗ $name (expected exit=$expected_code, got exit=$actual_code)"
        ((FAILED++))
    fi
}

echo "Testing Do Mode..."
echo "==================="
echo ""

# ─────────────────────────────────────────────────────────
# PART 1: Task file dicts evaluate correctly
# ─────────────────────────────────────────────────────────
echo "--- Task Dict Evaluation ---"

# Simple string tasks evaluate to a dict
run_test "simple string task dict" \
    '{build: "echo build"}' \
    '{build: "echo build"}'

# Multiple string tasks (dict order may vary)
result=$($AVON run '{a: "echo a", b: "echo b"}' 2>&1) || true
if echo "$result" | grep -qF 'a: "echo a"' && echo "$result" | grep -qF 'b: "echo b"'; then
    echo "✓ multiple string tasks"
    ((PASSED++))
else
    echo "✗ multiple string tasks"
    echo "  Got: $result"
    ((FAILED++))
fi

# Structured task with cmd field
run_test "structured task with cmd" \
    '{build: {cmd: "echo build"}}' \
    '{build: {cmd: "echo build"}}'

# Structured task with cmd and desc
result=$($AVON run '{build: {cmd: "echo build", desc: "Build it"}}' 2>&1) || true
if echo "$result" | grep -qF 'cmd: "echo build"' && echo "$result" | grep -qF 'desc: "Build it"'; then
    echo "✓ structured task with desc"
    ((PASSED++))
else
    echo "✗ structured task with desc"
    echo "  Got: $result"
    ((FAILED++))
fi

# Structured task with deps list
result=$($AVON run '{test: {cmd: "echo test", deps: ["build"]}}' 2>&1) || true
if echo "$result" | grep -qF 'cmd: "echo test"' && echo "$result" | grep -q 'deps:.*build'; then
    echo "✓ structured task with deps"
    ((PASSED++))
else
    echo "✗ structured task with deps"
    echo "  Got: $result"
    ((FAILED++))
fi

# Structured task with env dict
result=$($AVON run '{greet: {cmd: "echo hi", env: {NAME: "avon"}}}' 2>&1) || true
if echo "$result" | grep -qF 'cmd: "echo hi"' && echo "$result" | grep -qF 'NAME: "avon"'; then
    echo "✓ structured task with env"
    ((PASSED++))
else
    echo "✗ structured task with env"
    echo "  Got: $result"
    ((FAILED++))
fi

# Full structured task with all fields
result=$($AVON run '{deploy: {cmd: "deploy.sh", desc: "Deploy", deps: ["build", "test"], env: {ENV: "prod"}}}' 2>&1) || true
if echo "$result" | grep -qF 'cmd: "deploy.sh"' && echo "$result" | grep -qF 'desc: "Deploy"' && echo "$result" | grep -q 'deps:.*build' && echo "$result" | grep -qF 'ENV: "prod"'; then
    echo "✓ full structured task"
    ((PASSED++))
else
    echo "✗ full structured task"
    echo "  Got: $result"
    ((FAILED++))
fi

# Mixed simple and structured tasks
result=$($AVON run '{clean: "rm -rf build", build: {cmd: "make", deps: ["clean"]}}' 2>&1) || true
if echo "$result" | grep -qF 'clean: "rm -rf build"' && echo "$result" | grep -qF 'cmd: "make"'; then
    echo "✓ mixed simple and structured"
    ((PASSED++))
else
    echo "✗ mixed simple and structured"
    echo "  Got: $result"
    ((FAILED++))
fi

# Task dict built with let/in
run_test "task dict with let/in" \
    'let tasks = {build: "echo build"} in tasks' \
    '{build: "echo build"}'

# Empty dict (valid but no tasks)
run_test "empty task dict" '{}' '{}'

echo ""

# ─────────────────────────────────────────────────────────
# PART 2: Example task files evaluate without error
# ─────────────────────────────────────────────────────────
echo "--- Example Task Files ---"

for f in "$PROJECT_ROOT/examples/tasks_"*.av; do
    fname=$(basename "$f")
    result=$($AVON eval "$f" 2>&1)
    exit_code=$?
    if [ $exit_code -eq 0 ]; then
        echo "✓ $fname evaluates successfully"
        ((PASSED++))
    else
        echo "✗ $fname failed to evaluate"
        echo "  Exit code: $exit_code"
        echo "  Output: $(echo "$result" | head -3)"
        ((FAILED++))
    fi
done

echo ""

# ─────────────────────────────────────────────────────────
# PART 3: Do mode CLI - basic execution
# ─────────────────────────────────────────────────────────
echo "--- Do Mode Execution ---"

cat > "$TMPDIR/tasks.av" << 'EOF'
{
  hello: "echo HELLO_WORLD",
  build: {cmd: "echo BUILD_DONE", desc: "Build project"},
  test: {cmd: "echo TEST_DONE", deps: ["build"], desc: "Run tests"},
  greet: {cmd: "echo Hi $NAME", env: {NAME: "Avon"}}
}
EOF

# Simple task runs
run_test_contains "simple task execution" "HELLO_WORLD" $AVON do hello "$TMPDIR/tasks.av"
run_test_contains "structured task execution" "BUILD_DONE" $AVON do build "$TMPDIR/tasks.av"

# Dependencies run in order
run_test_contains "deps: build runs first" "BUILD_DONE" $AVON do test "$TMPDIR/tasks.av"
run_test_contains "deps: test runs after" "TEST_DONE" $AVON do test "$TMPDIR/tasks.av"

# Env var expansion
run_test_contains "env var expansion" "Hi Avon" $AVON do greet "$TMPDIR/tasks.av"

# Running: prefix displayed
run_test_contains "Running prefix shown" "Running: hello" $AVON do hello "$TMPDIR/tasks.av"

# Completion message
run_test_contains "completion message" "completed successfully" $AVON do hello "$TMPDIR/tasks.av"

# Description displayed during run
run_test_contains "desc shown during run" "Build project" $AVON do build "$TMPDIR/tasks.av"

echo ""

# ─────────────────────────────────────────────────────────
# PART 4: Do mode CLI flags
# ─────────────────────────────────────────────────────────
echo "--- Do Mode Flags ---"

# --list
run_test_contains "list: Available Tasks header" "Available Tasks" $AVON do --list "$TMPDIR/tasks.av"
run_test_contains "list: shows task name" "build" $AVON do --list "$TMPDIR/tasks.av"
run_test_contains "list: shows description" "Build project" $AVON do --list "$TMPDIR/tasks.av"

# --info
run_test_contains "info: shows task name" "Task: test" $AVON do --info test "$TMPDIR/tasks.av"
run_test_contains "info: shows description" "Run tests" $AVON do --info test "$TMPDIR/tasks.av"
run_test_contains "info: shows deps" "build" $AVON do --info test "$TMPDIR/tasks.av"

# --dry-run
run_test_contains "dry-run: shows plan" "Execution Plan" $AVON do --dry-run test "$TMPDIR/tasks.av"
run_test_contains "dry-run: shows deps in order" "build" $AVON do --dry-run test "$TMPDIR/tasks.av"
run_test_not_contains "dry-run: does not execute" "Running:" $AVON do --dry-run test "$TMPDIR/tasks.av"

echo ""

# ─────────────────────────────────────────────────────────
# PART 5: Do mode error handling
# ─────────────────────────────────────────────────────────
echo "--- Do Mode Errors ---"

# Task not found
run_error_test "nonexistent task fails" $AVON do nope "$TMPDIR/tasks.av"
run_test_contains "not found error message" "not found" $AVON do nope "$TMPDIR/tasks.av"

# Typo suggestion
cat > "$TMPDIR/typo.av" << 'EOF'
{build: "echo build", test: "echo test", deploy: "echo deploy"}
EOF
run_test_contains "typo suggestion" "Did you mean" $AVON do bild "$TMPDIR/typo.av"

# Cyclic dependency
cat > "$TMPDIR/cycle.av" << 'EOF'
{a: {cmd: "echo a", deps: ["b"]}, b: {cmd: "echo b", deps: ["a"]}}
EOF
run_error_test "cyclic dep fails" $AVON do a "$TMPDIR/cycle.av"
run_test_contains "cycle error message" "Cyclic" $AVON do a "$TMPDIR/cycle.av"

# Undefined dependency
cat > "$TMPDIR/undef.av" << 'EOF'
{main: {cmd: "echo main", deps: ["ghost"]}}
EOF
run_error_test "undefined dep fails" $AVON do main "$TMPDIR/undef.av"
run_test_contains "undef dep names missing task" "ghost" $AVON do main "$TMPDIR/undef.av"

# Missing cmd field
cat > "$TMPDIR/no_cmd.av" << 'EOF'
{bad: {desc: "No cmd"}}
EOF
run_error_test "missing cmd fails" $AVON do bad "$TMPDIR/no_cmd.av"
run_test_contains "missing cmd mentions cmd" "cmd" $AVON do bad "$TMPDIR/no_cmd.av"

# No task name given
run_error_test "no task name fails" $AVON do

# Failing task stops chain
cat > "$TMPDIR/fail_chain.av" << 'EOF'
{fail: "exit 1", after: {cmd: "echo SHOULD_NOT_RUN", deps: ["fail"]}}
EOF
run_error_test "failing task returns error" $AVON do fail "$TMPDIR/fail_chain.av"
result=$($AVON do after "$TMPDIR/fail_chain.av" 2>&1) || true
if echo "$result" | grep -qF "SHOULD_NOT_RUN"; then
    echo "✗ failing dep should stop downstream"
    ((FAILED++))
else
    echo "✓ failing dep stops downstream"
    ((PASSED++))
fi

echo ""

# ─────────────────────────────────────────────────────────
# PART 6: Auto-discovery
# ─────────────────────────────────────────────────────────
echo "--- Auto-Discovery ---"

AUTODIR="$TMPDIR/autodir"
mkdir -p "$AUTODIR"
cat > "$AUTODIR/Avon.av" << 'EOF'
{hello: "echo DISCOVERED"}
EOF

# Task runs from Avon.av
result=$(cd "$AUTODIR" && $AVON do hello 2>&1) || true
if echo "$result" | grep -qF "DISCOVERED"; then
    echo "✓ auto-discovery finds Avon.av"
    ((PASSED++))
else
    echo "✗ auto-discovery finds Avon.av"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

# --list works with auto-discovery
result=$(cd "$AUTODIR" && $AVON do --list 2>&1) || true
if echo "$result" | grep -qF "hello"; then
    echo "✓ auto-discovery: --list works"
    ((PASSED++))
else
    echo "✗ auto-discovery: --list works"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

# --info works with auto-discovery
result=$(cd "$AUTODIR" && $AVON do --info hello 2>&1) || true
if echo "$result" | grep -qF "Task: hello"; then
    echo "✓ auto-discovery: --info works"
    ((PASSED++))
else
    echo "✗ auto-discovery: --info works"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

# Missing Avon.av errors
NOFILE="$TMPDIR/empty_dir"
mkdir -p "$NOFILE"
result=$(cd "$NOFILE" && $AVON do build 2>&1)
nofile_exit=$?
if [ $nofile_exit -ne 0 ]; then
    echo "✓ auto-discovery: errors when no Avon.av"
    ((PASSED++))
else
    echo "✗ auto-discovery: should error when no Avon.av"
    ((FAILED++))
fi

echo ""

# ─────────────────────────────────────────────────────────
# PART 7: Security - --git and --stdin require confirmation
# ─────────────────────────────────────────────────────────
echo "--- Security ---"

# --git requires confirmation (auto-declines when stdin is /dev/null)
result=$(echo 'N' | $AVON do build --git user/repo/tasks.av 2>&1) || true
if [ $? -ne 0 ] || echo "$result" | grep -qF "Aborted"; then
    echo "✓ git requires confirmation for do mode"
    ((PASSED++))
else
    echo "✗ git requires confirmation for do mode"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

result=$(echo 'N' | $AVON do build --git user/repo/tasks.av 2>&1) || true
if echo "$result" | grep -qF "Warning"; then
    echo "✓ git confirmation: shows warning"
    ((PASSED++))
else
    echo "✗ git confirmation: shows warning"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

result=$(echo 'N' | $AVON do build --git user/repo/tasks.av 2>&1) || true
if echo "$result" | grep -qF "remote source"; then
    echo "✓ git confirmation: mentions remote source"
    ((PASSED++))
else
    echo "✗ git confirmation: mentions remote source"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

# --stdin requires confirmation (auto-declines when prompt can't read)
result=$(echo '{build: "echo pwned"}' | $AVON do build --stdin 2>&1) || true
if echo "$result" | grep -qF "Aborted" || echo "$result" | grep -qF "Warning"; then
    echo "✓ stdin confirmation: shows warning or aborts"
    ((PASSED++))
else
    echo "✗ stdin confirmation: shows warning or aborts"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

# Local files still work fine
run_exit_test "local file works after blocks" 0 $AVON do hello "$TMPDIR/tasks.av"

echo ""

# ─────────────────────────────────────────────────────────
# PART 8: Diamond dependency (each task runs exactly once)
# ─────────────────────────────────────────────────────────
echo "--- Diamond Dependencies ---"

cat > "$TMPDIR/diamond.av" << 'EOF'
{
  base: "echo BASE_RUN",
  left: {cmd: "echo LEFT_RUN", deps: ["base"]},
  right: {cmd: "echo RIGHT_RUN", deps: ["base"]},
  top: {cmd: "echo TOP_RUN", deps: ["left", "right"]}
}
EOF

result=$($AVON do top "$TMPDIR/diamond.av" 2>&1) || true

# All tasks run
echo "$result" | grep -qF "BASE_RUN" && { echo "✓ diamond: base runs"; ((PASSED++)); } || { echo "✗ diamond: base runs"; ((FAILED++)); }
echo "$result" | grep -qF "LEFT_RUN" && { echo "✓ diamond: left runs"; ((PASSED++)); } || { echo "✗ diamond: left runs"; ((FAILED++)); }
echo "$result" | grep -qF "RIGHT_RUN" && { echo "✓ diamond: right runs"; ((PASSED++)); } || { echo "✗ diamond: right runs"; ((FAILED++)); }
echo "$result" | grep -qF "TOP_RUN" && { echo "✓ diamond: top runs"; ((PASSED++)); } || { echo "✗ diamond: top runs"; ((FAILED++)); }

# Base runs exactly once (should appear exactly once in output)
base_count=$(echo "$result" | grep -c "BASE_RUN") || true
if [ "$base_count" -eq 1 ]; then
    echo "✓ diamond: base runs exactly once"
    ((PASSED++))
else
    echo "✗ diamond: base runs $base_count times (expected 1)"
    ((FAILED++))
fi

echo ""

# ─────────────────────────────────────────────────────────
# PART 9: Environment variable edge cases
# ─────────────────────────────────────────────────────────
echo "--- Env Var Edge Cases ---"

cat > "$TMPDIR/env_edge.av" << 'EOF'
{
  dollar: {cmd: "echo $GREETING", env: {GREETING: "hello"}},
  braces: {cmd: "echo ${GREETING}", env: {GREETING: "hello"}},
  multi: {cmd: "echo $A and $B", env: {A: "one", B: "two"}},
  system: {cmd: "echo $HOME", desc: "Uses system env var"}
}
EOF

run_test_contains "env: dollar syntax" "hello" $AVON do dollar "$TMPDIR/env_edge.av"
run_test_contains "env: braces syntax" "hello" $AVON do braces "$TMPDIR/env_edge.av"
run_test_contains "env: multiple vars" "one and two" $AVON do multi "$TMPDIR/env_edge.av"
run_test_contains "env: system var fallback" "/" $AVON do system "$TMPDIR/env_edge.av"

echo ""

# ─────────────────────────────────────────────────────────
# PART 10: CLI help mentions do mode
# ─────────────────────────────────────────────────────────
echo "--- CLI Help ---"

run_test_contains "help: mentions do" "do" $AVON help
run_test_contains "help: git/stdin prompt in do mode" "prompt for confirmation" $AVON help
run_test_contains "help do: Safety section" "Safety" $AVON help do
run_test_contains "help do: shows usage" "avon do" $AVON help do

# ── Cleanup ──────────────────────────────────────────────
rm -rf "$TMPDIR"

echo ""
echo "==================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✓ All do mode tests passed!"
    exit 0
else
    echo "✗ Some do mode tests failed"
    exit 1
fi
