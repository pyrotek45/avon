#!/bin/bash
# Integration Tests Entry Point
# Runs end-to-end integration tests combining multiple components

TESTING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT_ROOT="$(cd "$TESTING_DIR/.." && pwd)"
INTEGRATION_TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

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

print_section "Running Integration Tests"

# 1. Full integration pipeline
print_section "Full Integration Pipeline"
run_test "Integration Tests" "$INTEGRATION_TEST_DIR/test_integration.sh"

# 2. Example deployment tests
print_section "Deployment Tests"
run_test "Example Outputs" "$INTEGRATION_TEST_DIR/test_example_outputs.sh"

# 3. Backup and recovery tests
print_section "Backup & Recovery"
run_test "Backup Tests" "$INTEGRATION_TEST_DIR/test_backup.sh"

# 4. Atomic deployment
print_section "Atomic Deployment"
run_test "Atomic Deployment" "$INTEGRATION_TEST_DIR/test_atomic_deployment.sh"

# 5. Bulletproof tests
print_section "Resilience Tests"
run_test "Bulletproof Tests" "$INTEGRATION_TEST_DIR/test_bulletproof.sh"

# Summary
print_section "Integration Tests Summary"
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
