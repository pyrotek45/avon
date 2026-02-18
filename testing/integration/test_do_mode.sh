#!/bin/bash
# Do Mode Integration Tests
# Tests avon's task runner (do command) end-to-end
# Covers: task execution, dependencies, --dry-run, --list, --info,
#         env vars, typo suggestions, cycle detection, auto-discovery

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

# Force debug binary since do mode may not be in release yet
AVON="$PROJECT_ROOT/target/debug/avon"
if [ ! -x "$AVON" ]; then
    echo "ERROR: Debug binary not found at $AVON. Run 'cargo build' first." >&2
    exit 1
fi

PASSED=0
FAILED=0

TMPDIR="/tmp/avon_do_test_$$"
mkdir -p "$TMPDIR"

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

run_error_test_contains() {
    local name="$1"
    local expected="$2"
    shift 2

    result=$("$@" 2>&1)
    exit_code=$?
    if [ $exit_code -ne 0 ] && echo "$result" | grep -qF "$expected"; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected error containing: $expected"
        echo "  Exit code: $exit_code"
        echo "  Got: $(echo "$result" | head -3)"
        ((FAILED++))
    fi
}

echo "Testing Do Mode (Task Runner)..."
echo "================================="
echo ""

# ── Create fixture files ─────────────────────────────────
cat > "$TMPDIR/simple.av" << 'EOF'
{
  hello: "echo HELLO_OUTPUT",
  world: "echo WORLD_OUTPUT"
}
EOF

cat > "$TMPDIR/deps.av" << 'EOF'
{
  step1: "echo STEP1",
  step2: {cmd: "echo STEP2", deps: ["step1"]},
  step3: {cmd: "echo STEP3", deps: ["step2"]}
}
EOF

cat > "$TMPDIR/diamond.av" << 'EOF'
{
  base: "echo BASE",
  left: {cmd: "echo LEFT", deps: ["base"]},
  right: {cmd: "echo RIGHT", deps: ["base"]},
  top: {cmd: "echo TOP", deps: ["left", "right"]}
}
EOF

cat > "$TMPDIR/described.av" << 'EOF'
{
  build: {cmd: "echo building", desc: "Build the project"},
  test: {cmd: "echo testing", desc: "Run test suite", deps: ["build"]},
  clean: "echo cleaning"
}
EOF

cat > "$TMPDIR/env.av" << 'EOF'
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

cat > "$TMPDIR/undef.av" << 'EOF'
{
  main: {cmd: "echo main", deps: ["ghost"]}
}
EOF

cat > "$TMPDIR/typo_dep.av" << 'EOF'
{
  build: "echo build",
  test: {cmd: "echo test", deps: ["bild"]}
}
EOF

cat > "$TMPDIR/many_tasks.av" << 'EOF'
{
  build: "echo build",
  test: "echo test",
  deploy: "echo deploy",
  lint: "echo lint",
  clean: "echo clean"
}
EOF

cat > "$TMPDIR/failing.av" << 'EOF'
{
  fail: "exit 1",
  after: {cmd: "echo SHOULD_NOT_RUN", deps: ["fail"]}
}
EOF

cat > "$TMPDIR/empty.av" << 'EOF'
{}
EOF

cat > "$TMPDIR/info_env.av" << 'EOF'
{
  deploy: {cmd: "deploy.sh", desc: "Deploy app", env: {ENV: "prod", VER: "2.0"}, deps: ["build"]},
  build: "echo build"
}
EOF

# ── Simple Task Execution ────────────────────────────────
echo "--- Simple Execution ---"
run_test_contains "run simple task"               "HELLO_OUTPUT"         $AVON do hello "$TMPDIR/simple.av"
run_test_contains "run shows Running: prefix"     "Running: hello"       $AVON do hello "$TMPDIR/simple.av"
run_test_contains "run shows completion"          "completed successfully" $AVON do hello "$TMPDIR/simple.av"

# ── Dependency Resolution ────────────────────────────────
echo ""
echo "--- Dependencies ---"
run_test_contains "linear deps - step1 runs"     "STEP1"                $AVON do step3 "$TMPDIR/deps.av"
run_test_contains "linear deps - step2 runs"     "STEP2"                $AVON do step3 "$TMPDIR/deps.av"
run_test_contains "linear deps - step3 runs"     "STEP3"                $AVON do step3 "$TMPDIR/deps.av"
run_test_contains "diamond - base runs"          "BASE"                 $AVON do top "$TMPDIR/diamond.av"
run_test_contains "diamond - top runs"           "TOP"                  $AVON do top "$TMPDIR/diamond.av"

# ── --dry-run ────────────────────────────────────────────
echo ""
echo "--- Dry Run ---"
run_test_contains "dry-run shows plan"           "Execution Plan"       $AVON do --dry-run step3 "$TMPDIR/deps.av"
run_test_contains "dry-run shows step order"     "step1"                $AVON do --dry-run step3 "$TMPDIR/deps.av"

# Verify dry-run does NOT actually execute
dry_output=$($AVON do --dry-run step3 "$TMPDIR/deps.av" 2>&1) || true
if echo "$dry_output" | grep -qF "Running:"; then
    echo "✗ dry-run should not execute tasks"
    ((FAILED++))
else
    echo "✓ dry-run does not execute tasks"
    ((PASSED++))
fi

# ── --list ───────────────────────────────────────────────
echo ""
echo "--- List Tasks ---"
run_test_contains "list shows Available Tasks"   "Available Tasks"      $AVON do --list "$TMPDIR/described.av"
run_test_contains "list shows build"             "build"                $AVON do --list "$TMPDIR/described.av"
run_test_contains "list shows description"       "Build the project"    $AVON do --list "$TMPDIR/described.av"
run_test_contains "list empty shows No tasks"    "No tasks found"       $AVON do --list "$TMPDIR/empty.av"

# ── --info ───────────────────────────────────────────────
echo ""
echo "--- Task Info ---"
run_test_contains "info shows Task name"         "Task: deploy"         $AVON do --info deploy "$TMPDIR/info_env.av"
run_test_contains "info shows Description"       "Deploy app"           $AVON do --info deploy "$TMPDIR/info_env.av"
run_test_contains "info shows Dependencies"      "build"                $AVON do --info deploy "$TMPDIR/info_env.av"
run_test_contains "info shows Environment"       "Environment Variables" $AVON do --info deploy "$TMPDIR/info_env.av"

# ── Environment Variables ────────────────────────────────
echo ""
echo "--- Environment Variables ---"
run_test_contains "env dollar syntax"            "Hello Avon from here"  $AVON do greet "$TMPDIR/env.av"
run_test_contains "env braces syntax"            "myapp-v1.0"            $AVON do tag "$TMPDIR/env.av"

# ── Error Handling ───────────────────────────────────────
echo ""
echo "--- Error Handling ---"
run_error_test "task not found fails"            $AVON do nonexistent "$TMPDIR/simple.av"
run_error_test_contains "typo suggests build"    "Did you mean 'build'" $AVON do bild "$TMPDIR/many_tasks.av"
run_error_test_contains "typo suggests test"     "Did you mean 'test'"  $AVON do tset "$TMPDIR/many_tasks.av"
run_error_test "cyclic dependency fails"         $AVON do a "$TMPDIR/cycle.av"
run_error_test_contains "cycle mentions Cyclic"  "Cyclic"               $AVON do a "$TMPDIR/cycle.av"
run_error_test "undefined dep fails"             $AVON do main "$TMPDIR/undef.av"
run_error_test_contains "undef dep mentions dep" "ghost"                $AVON do main "$TMPDIR/undef.av"
run_error_test_contains "dep typo suggestion"    "Did you mean 'build'" $AVON do test "$TMPDIR/typo_dep.av"

# ── Failing Tasks ────────────────────────────────────────
echo ""
echo "--- Failing Tasks ---"
run_error_test "failing task returns error"      $AVON do fail "$TMPDIR/failing.av"

# Verify failing dep stops downstream
fail_output=$($AVON do after "$TMPDIR/failing.av" 2>&1) || true
if echo "$fail_output" | grep -qF "SHOULD_NOT_RUN"; then
    echo "✗ failing dep should stop downstream"
    ((FAILED++))
else
    echo "✓ failing dep stops downstream"
    ((PASSED++))
fi

# ── Description displayed during run ─────────────────────
echo ""
echo "--- Descriptions ---"
run_test_contains "desc shown during run"        "Build the project"    $AVON do build "$TMPDIR/described.av"

# ── Cleanup ──────────────────────────────────────────────
rm -rf "$TMPDIR"

echo ""
echo "================================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✓ All do mode tests passed!"
    exit 0
else
    echo "✗ Some do mode tests failed"
    exit 1
fi
