#!/bin/bash
# Code Structure Verification Tests
# Ensures key code patterns and files exist
# Migrated from tests/integration_tests.rs

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

PASSED=0
FAILED=0

echo "Testing Code Structure..."
echo "========================="
echo ""

# Test: Avon.av file exists at project root
if [ -f "$PROJECT_ROOT/Avon.av" ]; then
    echo "✓ Avon.av exists at project root"
    ((PASSED++))
else
    echo "✗ Avon.av exists at project root"
    ((FAILED++))
fi

# Test: expand_env_vars function exists in task_runner
if grep -q "expand_env_vars" "$PROJECT_ROOT/src/cli/task_runner.rs"; then
    echo "✓ expand_env_vars function exists in task_runner.rs"
    ((PASSED++))
else
    echo "✗ expand_env_vars function exists in task_runner.rs"
    ((FAILED++))
fi

# Test: typo suggestions exist in task_runner
if grep -q "suggestions" "$PROJECT_ROOT/src/cli/task_runner.rs"; then
    echo "✓ typo suggestions exist in task_runner.rs"
    ((PASSED++))
else
    echo "✗ typo suggestions exist in task_runner.rs"
    ((FAILED++))
fi

# Test: TaskError::ParseError is used in commands
if grep -q "TaskError::ParseError" "$PROJECT_ROOT/src/cli/commands.rs"; then
    echo "✓ TaskError::ParseError used in commands.rs"
    ((PASSED++))
else
    echo "✗ TaskError::ParseError used in commands.rs"
    ((FAILED++))
fi

# Test: build_execution_plan does NOT contain order.reverse()
plan_fn=$(sed -n '/fn build_execution_plan/,/^    }/p' "$PROJECT_ROOT/src/cli/task_runner.rs")
if echo "$plan_fn" | grep -q "reverse()"; then
    echo "✗ build_execution_plan should not contain reverse()"
    ((FAILED++))
else
    echo "✓ build_execution_plan does not contain reverse()"
    ((PASSED++))
fi

# Test: binary compiles without errors
if cargo check --quiet 2>&1; then
    echo "✓ cargo check passes (no compilation errors)"
    ((PASSED++))
else
    echo "✗ cargo check passes (no compilation errors)"
    ((FAILED++))
fi

echo ""
echo "========================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✓ All code structure tests passed!"
    exit 0
else
    echo "✗ Some code structure tests failed"
    exit 1
fi
