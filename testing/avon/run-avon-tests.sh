#!/bin/bash
# Avon Language Tests Entry Point
# Runs all tests for the Avon template language compiler

TESTING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT_ROOT="$(cd "$TESTING_DIR/.." && pwd)"
AVON_TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

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
    
    if bash "$script" > /tmp/avon_test_output.log 2>&1; then
        echo -e "${GREEN}✓ $name${NC}"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}✗ $name${NC}"
        tail -20 /tmp/avon_test_output.log
        ((TESTS_FAILED++))
        FAILED_TESTS+=("$name")
    fi
}

print_section "Running Avon Language Tests"

# 1. Cargo unit tests
print_section "Unit Tests"
cd "$PROJECT_ROOT"
if cargo test --lib 2>&1 | tail -5; then
    ((TESTS_PASSED++))
    echo -e "${GREEN}✓ Unit Tests${NC}"
else
    ((TESTS_FAILED++))
    FAILED_TESTS+=("Unit Tests")
    echo -e "${RED}✗ Unit Tests${NC}"
fi

# 2. Integration tests
print_section "Integration Tests"
if cargo test --test '*' 2>&1 | tail -5; then
    ((TESTS_PASSED++))
    echo -e "${GREEN}✓ Integration Tests${NC}"
else
    ((TESTS_FAILED++))
    FAILED_TESTS+=("Integration Tests")
    echo -e "${RED}✗ Integration Tests${NC}"
fi

# 3. Example validation
print_section "Example Files Validation"
run_test "Example Files" "$AVON_TEST_DIR/test_all_examples.sh"

# 4. Language feature tests
print_section "Language Features"
run_test "Template Syntax" "$AVON_TEST_DIR/test_template_syntax.sh"
run_test "Path Handling" "$AVON_TEST_DIR/test_path_literal_block.sh"
run_test "Grammar" "$AVON_TEST_DIR/test_grammar.sh"
run_test "Tutorial Snippets" "$AVON_TEST_DIR/test_tutorial_snippets.sh"

# 5. Security and sandboxing
print_section "Security & Sandboxing"
run_test "Path Traversal Protection" "$AVON_TEST_DIR/test_path_traversal.sh"
run_test "Security Comprehensive" "$AVON_TEST_DIR/test_security_comprehensive.sh"

# 6. Specific feature tests
print_section "Feature Tests"
run_test "None Handling" "$AVON_TEST_DIR/test_none_handling.sh"
run_test "Scoping Rules" "$AVON_TEST_DIR/test_scoping_rules.sh"
run_test "Claims" "$AVON_TEST_DIR/test_claims.sh"
run_test "REPL" "$AVON_TEST_DIR/test_repl.sh"

# Summary
print_section "Avon Tests Summary"
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Failed: $TESTS_FAILED${NC}"

if [ $TESTS_FAILED -gt 0 ]; then
    echo -e "\n${RED}Failed tests:${NC}"
    for test in "${FAILED_TESTS[@]}"; do
        echo -e "  ${RED}✗${NC} $test"
    done
    exit 1
else
    echo -e "\n${GREEN}All Avon language tests passed!${NC}"
    exit 0
fi
