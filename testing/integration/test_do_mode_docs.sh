#!/bin/bash
# Do Mode Documentation Verification Tests
# Verifies every documented claim about do mode is actually true.
# Covers: security blocks (--git/--stdin), file resolution, auto-discovery,
#         error messages, CLI help output, mode differences

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

AVON="$PROJECT_ROOT/target/debug/avon"
if [ ! -x "$AVON" ]; then
    echo "ERROR: Debug binary not found at $AVON. Run 'cargo build' first." >&2
    exit 1
fi

PASSED=0
FAILED=0

TMPDIR="/tmp/avon_do_docs_test_$$"
mkdir -p "$TMPDIR"

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

run_exit_code_test() {
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

echo "Do Mode Documentation Verification Tests"
echo "========================================="
echo ""

# ── Create fixture files ─────────────────────────────────
cat > "$TMPDIR/tasks.av" << 'EOF'
{
  build: "echo BUILD_OK",
  test: {cmd: "echo TEST_OK", deps: ["build"], desc: "Run tests"},
  clean: "echo CLEAN_OK"
}
EOF

cat > "$TMPDIR/env_tasks.av" << 'EOF'
{
  greet: {
    cmd: "echo Hello $NAME from $PLACE",
    env: {NAME: "Avon", PLACE: "here"}
  },
  tag: {
    cmd: "echo ${APP}-v${VER}",
    env: {APP: "myapp", VER: "1.0"}
  }
}
EOF

cat > "$TMPDIR/cycle.av" << 'EOF'
{
  a: {cmd: "echo a", deps: ["b"]},
  b: {cmd: "echo b", deps: ["a"]}
}
EOF

# ── SECURITY: --git is blocked ───────────────────────────
echo "--- Security: --git blocked ---"
run_test_contains "git blocked: error message" \
    "Error: --git is not allowed with 'do' mode" \
    $AVON do build --git user/repo/tasks.av

run_test_contains "git blocked: security reason" \
    "security risk" \
    $AVON do build --git user/repo/tasks.av

run_test_contains "git blocked: safe alternative" \
    "avon eval --git user/repo/tasks.av > tasks.av" \
    $AVON do build --git user/repo/tasks.av

run_exit_code_test "git blocked: exit code 1" 1 \
    $AVON do build --git user/repo/tasks.av

# ── SECURITY: --stdin is blocked ─────────────────────────
echo ""
echo "--- Security: --stdin blocked ---"
result=$(echo '{build: "echo pwned"}' | $AVON do build --stdin 2>&1) || true
if echo "$result" | grep -qF "Error: --stdin is not allowed with 'do' mode"; then
    echo "✓ stdin blocked: error message"
    ((PASSED++))
else
    echo "✗ stdin blocked: error message"
    echo "  Expected: Error: --stdin is not allowed with 'do' mode"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

if echo "$result" | grep -qF "security risk"; then
    echo "✓ stdin blocked: security reason"
    ((PASSED++))
else
    echo "✗ stdin blocked: security reason"
    ((FAILED++))
fi

echo '{build: "echo pwned"}' | $AVON do build --stdin > /dev/null 2>&1
stdin_exit=$?
if [ $stdin_exit -ne 0 ]; then
    echo "✓ stdin blocked: exit code non-zero ($stdin_exit)"
    ((PASSED++))
else
    echo "✗ stdin blocked: should have failed (exit=0)"
    ((FAILED++))
fi

# ── SECURITY: local files still work ─────────────────────
echo ""
echo "--- Security: local files work ---"
run_test_contains "local file execution works" \
    "BUILD_OK" \
    $AVON do build "$TMPDIR/tasks.av"

run_exit_code_test "local file exit code 0" 0 \
    $AVON do build "$TMPDIR/tasks.av"

# ── FILE RESOLUTION: explicit file argument ──────────────
echo ""
echo "--- File Resolution ---"
run_test_contains "explicit file argument" \
    "BUILD_OK" \
    $AVON do build "$TMPDIR/tasks.av"

# ── FILE RESOLUTION: auto-discovery ──────────────────────
echo ""
echo "--- Auto-Discovery ---"
AUTODIR="$TMPDIR/autodir"
mkdir -p "$AUTODIR"
cat > "$AUTODIR/Avon.av" << 'EOF'
{
  hello: "echo AUTO_HELLO"
}
EOF

# Run from autodir to test auto-discovery
result=$(cd "$AUTODIR" && $AVON do hello 2>&1) || true
if echo "$result" | grep -qF "AUTO_HELLO"; then
    echo "✓ auto-discovery: finds Avon.av"
    ((PASSED++))
else
    echo "✗ auto-discovery: finds Avon.av"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

# --list with auto-discovery
result=$(cd "$AUTODIR" && $AVON do --list 2>&1) || true
if echo "$result" | grep -qF "hello"; then
    echo "✓ auto-discovery: --list works"
    ((PASSED++))
else
    echo "✗ auto-discovery: --list works"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

# --info with auto-discovery
result=$(cd "$AUTODIR" && $AVON do --info hello 2>&1) || true
if echo "$result" | grep -qF "Task: hello"; then
    echo "✓ auto-discovery: --info works"
    ((PASSED++))
else
    echo "✗ auto-discovery: --info works"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

# No Avon.av should error
NOFILE_DIR="$TMPDIR/nofile"
mkdir -p "$NOFILE_DIR"
result=$(cd "$NOFILE_DIR" && $AVON do build 2>&1)
nofile_exit=$?
if [ $nofile_exit -ne 0 ]; then
    echo "✓ auto-discovery: errors when no Avon.av"
    ((PASSED++))
else
    echo "✗ auto-discovery: should error when no Avon.av"
    ((FAILED++))
fi

# ── --dry-run ────────────────────────────────────────────
echo ""
echo "--- Dry Run ---"
run_test_contains "dry-run shows Execution Plan" \
    "Execution Plan" \
    $AVON do --dry-run test "$TMPDIR/tasks.av"

run_test_contains "dry-run shows task names" \
    "build" \
    $AVON do --dry-run test "$TMPDIR/tasks.av"

run_test_not_contains "dry-run does NOT execute" \
    "Running:" \
    $AVON do --dry-run test "$TMPDIR/tasks.av"

# ── --list ───────────────────────────────────────────────
echo ""
echo "--- List Tasks ---"
run_test_contains "list shows Available Tasks" \
    "Available Tasks" \
    $AVON do --list "$TMPDIR/tasks.av"

run_test_contains "list shows task names" \
    "build" \
    $AVON do --list "$TMPDIR/tasks.av"

run_test_contains "list shows descriptions" \
    "Run tests" \
    $AVON do --list "$TMPDIR/tasks.av"

# ── --info ───────────────────────────────────────────────
echo ""
echo "--- Task Info ---"
run_test_contains "info shows Task name" \
    "Task: test" \
    $AVON do --info test "$TMPDIR/tasks.av"

run_test_contains "info shows Description" \
    "Run tests" \
    $AVON do --info test "$TMPDIR/tasks.av"

run_test_contains "info shows Dependencies" \
    "build" \
    $AVON do --info test "$TMPDIR/tasks.av"

# ── Environment Variables ────────────────────────────────
echo ""
echo "--- Environment Variables ---"
run_test_contains "env: \$VAR expansion" \
    "Hello Avon from here" \
    $AVON do greet "$TMPDIR/env_tasks.av"

run_test_contains "env: \${VAR} expansion" \
    "myapp-v1.0" \
    $AVON do tag "$TMPDIR/env_tasks.av"

# ── Error Messages ───────────────────────────────────────
echo ""
echo "--- Error Messages ---"

# No task name
result=$($AVON do 2>&1) || true
if echo "$result" | grep -qF "Error: 'do' command requires a task name"; then
    echo "✓ no task name: correct error"
    ((PASSED++))
else
    echo "✗ no task name: correct error"
    echo "  Got: $(echo "$result" | head -3)"
    ((FAILED++))
fi

# Nonexistent task
run_test_contains "nonexistent task: error mentions task" \
    "not found" \
    $AVON do nonexistent "$TMPDIR/tasks.av"

# Typo suggestion
run_test_contains "typo suggestion for similar name" \
    "Did you mean" \
    $AVON do bild "$TMPDIR/tasks.av"

# Cyclic dependency
run_test_contains "cycle detection works" \
    "Cyclic" \
    $AVON do a "$TMPDIR/cycle.av"

run_exit_code_test "cycle detection: exit code non-zero" 1 \
    $AVON do a "$TMPDIR/cycle.av"

# ── CLI Help ─────────────────────────────────────────────
echo ""
echo "--- CLI Help ---"
run_test_contains "help mentions do command" \
    "do" \
    $AVON help

run_test_contains "help says --git is eval/deploy only" \
    "eval/deploy only" \
    $AVON help

run_test_contains "help do shows Security section" \
    "Security" \
    $AVON help do

run_test_contains "help do mentions --git blocked" \
    "--git" \
    $AVON help do

run_test_contains "help do mentions --stdin blocked" \
    "--stdin" \
    $AVON help do

# ── Task Execution Output Format ─────────────────────────
echo ""
echo "--- Execution Output Format ---"
run_test_contains "shows Running: prefix" \
    "Running: build" \
    $AVON do build "$TMPDIR/tasks.av"

run_test_contains "shows completion message" \
    "completed successfully" \
    $AVON do build "$TMPDIR/tasks.av"

# ── Failing Task Stops Downstream ────────────────────────
echo ""
echo "--- Failure Propagation ---"
cat > "$TMPDIR/fail.av" << 'EOF'
{
  fail: "exit 1",
  after: {cmd: "echo SHOULD_NOT_RUN", deps: ["fail"]}
}
EOF

run_exit_code_test "failing task returns non-zero" 1 \
    $AVON do fail "$TMPDIR/fail.av"

result=$($AVON do after "$TMPDIR/fail.av" 2>&1) || true
if echo "$result" | grep -qF "SHOULD_NOT_RUN"; then
    echo "✗ failing dep should stop downstream"
    ((FAILED++))
else
    echo "✓ failing dep stops downstream"
    ((PASSED++))
fi

# ── Structured Task Fields ───────────────────────────────
echo ""
echo "--- Structured Task Validation ---"
cat > "$TMPDIR/no_cmd.av" << 'EOF'
{
  bad: {desc: "No cmd field"}
}
EOF

run_exit_code_test "missing cmd field errors" 1 \
    $AVON do bad "$TMPDIR/no_cmd.av"

run_test_contains "missing cmd: mentions cmd" \
    "cmd" \
    $AVON do bad "$TMPDIR/no_cmd.av"

# ── Cleanup ──────────────────────────────────────────────
rm -rf "$TMPDIR"

echo ""
echo "========================================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✓ All do mode doc verification tests passed!"
    exit 0
else
    echo "✗ Some doc verification tests failed"
    exit 1
fi
