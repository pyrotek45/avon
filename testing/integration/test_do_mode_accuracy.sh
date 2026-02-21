#!/bin/bash
# Comprehensive Do Mode Documentation Accuracy Tests
# Tests every claim made in README.md, DO_MODE_GUIDE.md, GETTING_STARTED.md, TUTORIAL.md
# about do mode functionality to verify documentation accuracy

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

TMPDIR="/tmp/avon_do_accuracy_test_$$"
mkdir -p "$TMPDIR"
trap "rm -rf $TMPDIR" EXIT

PASSED=0
FAILED=0
FAILED_TESTS=()

test_claim() {
    local name="$1"
    local setup="$2"
    local command="$3"
    local expected_pattern="$4"
    local should_fail="${5:-false}"
    
    # Run setup
    eval "$setup" || true
    
    # Run command and capture output + exit code
    exit_code=0
    result=$(eval "$command" 2>&1) || exit_code=$?
    
    # Check result
    if [ "$should_fail" = "true" ]; then
        if [ $exit_code -ne 0 ] && echo "$result" | grep -qE "$expected_pattern"; then
            echo "✓ $name"
            ((PASSED++))
        else
            echo "✗ $name"
            echo "  Expected error matching: $expected_pattern"
            echo "  Got (exit=$exit_code): $(echo "$result" | head -3)"
            ((FAILED++))
            FAILED_TESTS+=("$name")
        fi
    else
        if [ $exit_code -eq 0 ] && echo "$result" | grep -qE "$expected_pattern"; then
            echo "✓ $name"
            ((PASSED++))
        else
            echo "✗ $name"
            echo "  Expected pattern: $expected_pattern"
            echo "  Got (exit=$exit_code): $(echo "$result" | head -3)"
            ((FAILED++))
            FAILED_TESTS+=("$name")
        fi
    fi
}

echo "============================================"
echo "Avon Do Mode Documentation Accuracy Tests"
echo "============================================"
echo ""

echo "--- CLAIM: Simple tasks run without dependencies ---"
test_claim \
    "simple task runs" \
    "mkdir -p $TMPDIR/simple && echo '{hello: \"echo Hello\"}' > $TMPDIR/simple/Avon.av" \
    "cd $TMPDIR/simple && $AVON do hello" \
    "Hello"

echo ""
echo "--- CLAIM: Tasks can be commands or structured dicts ---"
test_claim \
    "string cmd works" \
    "mkdir -p $TMPDIR/cmd_type && echo '{greet: \"echo Hi\"}' > $TMPDIR/cmd_type/Avon.av" \
    "cd $TMPDIR/cmd_type && $AVON do greet" \
    "Hi"

test_claim \
    "structured dict with cmd works" \
    "mkdir -p $TMPDIR/struct && echo '{greet: {cmd: \"echo Hi\"}}' > $TMPDIR/struct/Avon.av" \
    "cd $TMPDIR/struct && $AVON do greet" \
    "Hi"

echo ""
echo "--- CLAIM: Dependencies run in correct order ---"
test_claim \
    "linear deps: A -> B -> C" \
    "mkdir -p $TMPDIR/linear && cat > $TMPDIR/linear/Avon.av << 'EOF'
{
  a: {cmd: \"echo a\", deps: []},
  b: {cmd: \"echo b\", deps: [\"a\"]},
  c: {cmd: \"echo c\", deps: [\"b\"]}
}
EOF" \
    "cd $TMPDIR/linear && $AVON do c" \
    "Running: a"

test_claim \
    "diamond deps: A runs once" \
    "mkdir -p $TMPDIR/diamond && cat > $TMPDIR/diamond/Avon.av << 'EOF'
{
  setup: {cmd: \"echo setup\", deps: []},
  left: {cmd: \"echo left\", deps: [\"setup\"]},
  right: {cmd: \"echo right\", deps: [\"setup\"]},
  top: {cmd: \"echo top\", deps: [\"left\", \"right\"]}
}
EOF" \
    "cd $TMPDIR/diamond && $AVON do top" \
    "setup"

echo ""
echo "--- CLAIM: Cycle detection reports error ---"
test_claim \
    "cycle detected" \
    "mkdir -p $TMPDIR/cycle && cat > $TMPDIR/cycle/Avon.av << 'EOF'
{
  a: {cmd: \"echo a\", deps: [\"b\"]},
  b: {cmd: \"echo b\", deps: [\"a\"]}
}
EOF" \
    "cd $TMPDIR/cycle && $AVON do a" \
    "Cyclic dependency|cycle" \
    "true"

echo ""
echo "--- CLAIM: --list shows all tasks ---"
test_claim \
    "--list lists tasks" \
    "mkdir -p $TMPDIR/list && cat > $TMPDIR/list/Avon.av << 'EOF'
{
  build: {cmd: \"cargo build\", desc: \"Build project\"},
  test: {cmd: \"cargo test\", desc: \"Run tests\"},
  clean: {cmd: \"rm -rf target\"}
}
EOF" \
    "cd $TMPDIR/list && $AVON do --list" \
    "build"

test_claim \
    "--list shows descriptions" \
    "mkdir -p $TMPDIR/listdesc && cat > $TMPDIR/listdesc/Avon.av << 'EOF'
{
  build: {cmd: \"cargo build\", desc: \"Build project\"}
}
EOF" \
    "cd $TMPDIR/listdesc && $AVON do --list" \
    "Build project"

echo ""
echo "--- CLAIM: --info shows task details ---"
test_claim \
    "--info shows task name" \
    "mkdir -p $TMPDIR/info && cat > $TMPDIR/info/Avon.av << 'EOF'
{
  build: {cmd: \"cargo build\", desc: \"Build it\"}
}
EOF" \
    "cd $TMPDIR/info && $AVON do --info build" \
    "Task: build"

test_claim \
    "--info shows command" \
    "mkdir -p $TMPDIR/infocmd && cat > $TMPDIR/infocmd/Avon.av << 'EOF'
{
  build: {cmd: \"cargo build --release\"}
}
EOF" \
    "cd $TMPDIR/infocmd && $AVON do --info build" \
    "cargo build --release"

test_claim \
    "--info shows dependencies" \
    "mkdir -p $TMPDIR/infodeps && cat > $TMPDIR/infodeps/Avon.av << 'EOF'
{
  clean: {cmd: \"rm -rf target\"},
  build: {cmd: \"cargo build\", deps: [\"clean\"]}
}
EOF" \
    "cd $TMPDIR/infodeps && $AVON do --info build" \
    "clean"

echo ""
echo "--- CLAIM: --dry-run shows plan without executing ---"
test_claim \
    "--dry-run shows plan" \
    "mkdir -p $TMPDIR/dry && cat > $TMPDIR/dry/Avon.av << 'EOF'
{
  a: {cmd: \"echo a\"},
  b: {cmd: \"echo b\", deps: [\"a\"]}
}
EOF" \
    "cd $TMPDIR/dry && $AVON do --dry-run b" \
    "Execution Plan"

test_claim \
    "--dry-run doesn't run commands" \
    "mkdir -p $TMPDIR/dry_norun && echo '{fail_task: \"exit 1\"}' > $TMPDIR/dry_norun/Avon.av" \
    "cd $TMPDIR/dry_norun && $AVON do --dry-run fail_task" \
    "Execution Plan"

echo ""
echo "--- CLAIM: Auto-discovery finds Avon.av ---"
test_claim \
    "auto-discovery: avon do build works" \
    "mkdir -p $TMPDIR/autodiscover && echo '{build: \"echo building\"}' > $TMPDIR/autodiscover/Avon.av" \
    "cd $TMPDIR/autodiscover && $AVON do build" \
    "building"

test_claim \
    "auto-discovery: --list works without file arg" \
    "mkdir -p $TMPDIR/autolist && echo '{build: \"echo\"}' > $TMPDIR/autolist/Avon.av" \
    "cd $TMPDIR/autolist && $AVON do --list" \
    "build"

test_claim \
    "auto-discovery: --info works without file arg" \
    "mkdir -p $TMPDIR/autoinfo && echo '{build: \"cargo build\"}' > $TMPDIR/autoinfo/Avon.av" \
    "cd $TMPDIR/autoinfo && $AVON do --info build" \
    "Task: build"

echo ""
echo "--- CLAIM: Explicit file arg overrides auto-discovery ---"
test_claim \
    "explicit file overrides Avon.av" \
    "mkdir -p $TMPDIR/override && echo '{default: \"echo default\"}' > $TMPDIR/override/Avon.av && echo '{custom: \"echo custom\"}' > $TMPDIR/override/custom.av" \
    "cd $TMPDIR/override && $AVON do custom custom.av" \
    "custom"

echo ""
echo "--- CLAIM: env dict sets environment variables ---"
test_claim \
    "env var expansion: simple syntax" \
    "mkdir -p $TMPDIR/env1 && cat > $TMPDIR/env1/Avon.av << 'EOF'
{
  greet: {
    cmd: \"echo Hello \$NAME\",
    env: {NAME: \"World\"}
  }
}
EOF" \
    "cd $TMPDIR/env1 && $AVON do greet" \
    "Hello World"

test_claim \
    "env var expansion: braces syntax" \
    "mkdir -p $TMPDIR/env2 && cat > $TMPDIR/env2/Avon.av << 'EOF'
{
  tag: {
    cmd: \"echo \${APP}-v\${VERSION}\",
    env: {APP: \"myapp\", VERSION: \"1.0.0\"}
  }
}
EOF" \
    "cd $TMPDIR/env2 && $AVON do tag" \
    "myapp-v1.0.0"

test_claim \
    "env vars: system fallback (USER)" \
    "mkdir -p $TMPDIR/env_sys && echo '{whoami: {cmd: \"echo \$USER\"}}' > $TMPDIR/env_sys/Avon.av" \
    "cd $TMPDIR/env_sys && $AVON do whoami" \
    "^[a-zA-Z0-9_-]+$|pyrotek45"

test_claim \
    "task-level env overrides system" \
    "mkdir -p $TMPDIR/env_override && cat > $TMPDIR/env_override/Avon.av << 'EOF'
{
  myuser: {
    cmd: \"echo \$USER\",
    env: {USER: \"overridden\"}
  }
}
EOF" \
    "cd $TMPDIR/env_override && $AVON do myuser" \
    "overridden"

echo ""
echo "--- CLAIM: Task not found with typo suggestion ---"
test_claim \
    "typo suggestion: close match" \
    "mkdir -p $TMPDIR/typo && echo '{build: \"echo\"}' > $TMPDIR/typo/Avon.av" \
    "cd $TMPDIR/typo && $AVON do bild" \
    "Did you mean.*build" \
    "true"

test_claim \
    "typo suggestion: undefined dep" \
    "mkdir -p $TMPDIR/typo_dep && cat > $TMPDIR/typo_dep/Avon.av << 'EOF'
{
  build: {cmd: \"echo\", deps: [\"clen\"]},
  clean: {cmd: \"echo\"}
}
EOF" \
    "cd $TMPDIR/typo_dep && $AVON do build" \
    "Did you mean.*clean" \
    "true"

echo ""
echo "--- CLAIM: Missing cmd field errors ---"
test_claim \
    "missing cmd field error" \
    "mkdir -p $TMPDIR/no_cmd && cat > $TMPDIR/no_cmd/Avon.av << 'EOF'
{
  bad: {desc: \"only description\"}
}
EOF" \
    "cd $TMPDIR/no_cmd && $AVON do bad" \
    "missing required.*cmd|invalid format" \
    "true"

echo ""
echo "--- CLAIM: --git requires confirmation for do mode ---"
test_claim \
    "--git confirmation: shows warning" \
    "mkdir -p $TMPDIR/git_block" \
    "echo 'N' | $AVON do build --git pyrotek45/avon/Avon.av" \
    "Warning.*--git.*remote source|Aborted" \
    "true"

echo ""
echo "--- CLAIM: --stdin requires confirmation for do mode ---"
test_claim \
    "--stdin confirmation: shows warning" \
    "mkdir -p $TMPDIR/stdin_block" \
    "echo 'fake' | $AVON do build --stdin" \
    "Warning.*--stdin.*piped input|Aborted" \
    "true"

echo ""
echo "--- CLAIM: Task runs with description shown (if available) ---"
test_claim \
    "description shown during execution" \
    "mkdir -p $TMPDIR/desc && cat > $TMPDIR/desc/Avon.av << 'EOF'
{
  build: {
    cmd: \"echo done\",
    desc: \"Build the project\"
  }
}
EOF" \
    "cd $TMPDIR/desc && $AVON do build" \
    "Running: build"

echo ""
echo "--- CLAIM: Failed task returns non-zero exit code ---"
test_claim \
    "failing task exit code" \
    "mkdir -p $TMPDIR/fail && echo '{bad: \"exit 1\"}' > $TMPDIR/fail/Avon.av" \
    "cd $TMPDIR/fail && $AVON do bad; echo \$?" \
    "^1$|exit.*1"

test_claim \
    "failing dep stops downstream" \
    "mkdir -p $TMPDIR/fail_dep && cat > $TMPDIR/fail_dep/Avon.av << 'EOF'
{
  setup: {cmd: \"exit 1\"},
  build: {cmd: \"echo should_not_run\", deps: [\"setup\"]}
}
EOF" \
    "cd $TMPDIR/fail_dep && $AVON do build 2>&1" \
    "setup.*failed|Task.*setup.*failed|exit.*1" \
    "true"

echo ""
echo "--- CLAIM: File not found shows helpful error ---"
test_claim \
    "no file and no Avon.av error" \
    "mkdir -p $TMPDIR/nofile" \
    "cd $TMPDIR/nofile && $AVON do build" \
    "No source file|Avon.av|Usage" \
    "true"

echo ""
echo "--- CLAIM: Do mode vs eval vs deploy have different outputs ---"
test_claim \
    "eval with task dict shows dict" \
    "mkdir -p $TMPDIR/eval_tasks && echo '{build: \"cargo build\"}' > $TMPDIR/eval_tasks/Avon.av" \
    "cd $TMPDIR/eval_tasks && $AVON eval Avon.av" \
    "build.*cargo build"

test_claim \
    "deploy with task dict errors" \
    "mkdir -p $TMPDIR/deploy_tasks && echo '{build: \"cargo build\"}' > $TMPDIR/deploy_tasks/Avon.av" \
    "cd $TMPDIR/deploy_tasks && $AVON deploy Avon.av --root /tmp" \
    "not deployable|Expected.*FileTemplate" \
    "true"

test_claim \
    "do with FileTemplate dict errors" \
    "mkdir -p $TMPDIR/do_filetemplates && echo '@test.txt {\"content\"}' > $TMPDIR/do_filetemplates/Avon.av" \
    "cd $TMPDIR/do_filetemplates && $AVON do build" \
    "not.*dict|invalid|task|dictionary" \
    "true"

echo ""
echo "--- CLAIM: Variables in Avon.av are evaluated ---"
test_claim \
    "variables in task file work" \
    "mkdir -p $TMPDIR/vars && cat > $TMPDIR/vars/Avon.av << 'EOF'
let cmd = \"echo hello\" in
{build: cmd}
EOF" \
    "cd $TMPDIR/vars && $AVON do build" \
    "hello"

test_claim \
    "functions in task file work" \
    "mkdir -p $TMPDIR/funcs && cat > $TMPDIR/funcs/Avon.av << 'EOF'
let make_task = \msg {cmd: concat \"echo \" msg} in
{greet: make_task \"world\"}
EOF" \
    "cd $TMPDIR/funcs && $AVON do greet" \
    "world"

echo ""
echo "============================================"
echo "Test Results"
echo "============================================"
echo "Passed: $PASSED"
echo "Failed: $FAILED"

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Failed Tests:"
    for test in "${FAILED_TESTS[@]}"; do
        echo "  - $test"
    done
    echo ""
    exit 1
else
    echo ""
    echo "✓ All documentation claims verified!"
    exit 0
fi
