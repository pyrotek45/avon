#!/bin/bash
# Integration Test Runner
# Runs end-to-end integration tests: CLI, deploy, REPL, backup, examples.
# Called by: testing/run-all.sh (main entry point)

TESTING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT_ROOT="$(cd "$TESTING_DIR/.." && pwd)"
INTEGRATION_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPL_DIR="$TESTING_DIR/repl"

# Source common utilities
source "$TESTING_DIR/common.sh"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

TESTS_PASSED=0
TESTS_FAILED=0
FAILED_TESTS=()

print_section() {
    echo -e "\n${BLUE}>>> $1${NC}\n"
}

run_test() {
    local name=$1
    local script=$2
    
    if [ ! -f "$script" ]; then
        echo -e "${YELLOW}⊘ Skipping $name (script not found)${NC}"
        return 0
    fi
    
    if bash "$script" > /tmp/integration_test_output.log 2>&1; then
        echo -e "${GREEN}✓ $name${NC}"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}✗ $name${NC}"
        tail -20 /tmp/integration_test_output.log
        ((TESTS_FAILED++))
        FAILED_TESTS+=("$name")
    fi
}

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Integration Test Runner${NC}"
echo -e "${BLUE}========================================${NC}"

# ── CLI Integration ──────────────────────────────────────
print_section "CLI Integration"
run_test "CLI Integration"        "$INTEGRATION_DIR/test_cli_integration.sh"

# ── Example Outputs ──────────────────────────────────────
print_section "Example Outputs"
run_test "Example Outputs"        "$INTEGRATION_DIR/test_example_outputs.sh"

# ── Deploy & Backup ──────────────────────────────────────
print_section "Deploy & Backup"
run_test "Backup Tests"           "$INTEGRATION_DIR/test_backup.sh"
run_test "Atomic Deployment"      "$INTEGRATION_DIR/test_atomic_deployment.sh"
run_test "Bulletproof Tests"      "$INTEGRATION_DIR/test_bulletproof.sh"

# ── REPL Tests ───────────────────────────────────────────
print_section "REPL Tests"
run_test "REPL Multiline"         "$REPL_DIR/test-multiline.sh"
run_test "REPL Error History"     "$REPL_DIR/test-error-history.sh"

# ── Summary ──────────────────────────────────────────────
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Integration Tests Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Failed: $TESTS_FAILED${NC}"

if [ $TESTS_FAILED -gt 0 ]; then
    echo -e "\n${RED}Failed tests:${NC}"
    for test in "${FAILED_TESTS[@]}"; do
        echo -e "  ${RED}✗${NC} $test"
    done
    exit 1
else
    echo -e "\n${GREEN}All integration tests passed!${NC}"
    exit 0
fi
