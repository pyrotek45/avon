#!/bin/bash
# Unified test runner for the Avon project
# This is the single entry point for all testing

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TESTING_DIR="$PROJECT_ROOT/testing"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results tracking
TESTS_PASSED=0
TESTS_FAILED=0
FAILED_TESTS=()

print_header() {
    echo -e "\n${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
    ((TESTS_PASSED++))
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
    ((TESTS_FAILED++))
    FAILED_TESTS+=("$1")
}

run_test_suite() {
    local name=$1
    local script=$2
    
    if [ ! -f "$script" ]; then
        print_error "$name - script not found: $script"
        return 1
    fi
    
    print_header "Running: $name"
    
    if bash "$script"; then
        print_success "$name"
        return 0
    else
        print_error "$name"
        return 1
    fi
}

# Main testing
print_header "AVON PROJECT - UNIFIED TEST SUITE"
echo -e "Project Root: ${YELLOW}$PROJECT_ROOT${NC}"
echo -e "Testing Directory: ${YELLOW}$TESTING_DIR${NC}"

# 1. Run Cargo tests (unit and integration tests)
print_header "Cargo Tests"
if cd "$PROJECT_ROOT" && cargo test --all 2>&1 | tail -20; then
    print_success "Cargo unit and integration tests"
else
    print_error "Cargo unit and integration tests"
fi

# 2. Run clippy
print_header "Code Quality - Clippy"
if cd "$PROJECT_ROOT" && cargo clippy --all-targets --all-features 2>&1 | tail -5; then
    print_success "Clippy linting"
else
    print_error "Clippy linting"
fi

# 3. Check formatting
print_header "Code Quality - Format Check"
if cd "$PROJECT_ROOT" && cargo fmt --all -- --check 2>&1 | tail -5; then
    print_success "Code formatting"
else
    print_error "Code formatting"
fi

# 4. Run Avon language tests
print_header "Avon Language Tests"
if [ -f "$TESTING_DIR/avon/run-avon-tests.sh" ]; then
    run_test_suite "Avon Language Tests" "$TESTING_DIR/avon/run-avon-tests.sh"
else
    echo -e "${YELLOW}Note: Avon language test suite not yet created${NC}"
fi

# 5. Run LSP tests
print_header "LSP Server Tests"
if [ -f "$TESTING_DIR/lsp/run-lsp-tests.sh" ]; then
    run_test_suite "LSP Tests" "$TESTING_DIR/lsp/run-lsp-tests.sh"
else
    echo -e "${YELLOW}Note: LSP test suite not yet created${NC}"
fi

# 6. Run integration tests
print_header "Integration Tests"
if [ -f "$TESTING_DIR/integration/run-integration-tests.sh" ]; then
    run_test_suite "Integration Tests" "$TESTING_DIR/integration/run-integration-tests.sh"
else
    echo -e "${YELLOW}Note: Integration test suite not yet created${NC}"
fi

# 7. Run import and file I/O tests
print_header "Import and File I/O Tests"
if [ -f "$TESTING_DIR/imports/run-import-tests.sh" ]; then
    run_test_suite "Import and File I/O Tests" "$TESTING_DIR/imports/run-import-tests.sh"
else
    echo -e "${YELLOW}Note: Import test suite not yet created${NC}"
fi

# Summary
print_header "TEST SUMMARY"
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Failed: $TESTS_FAILED${NC}"

if [ $TESTS_FAILED -gt 0 ]; then
    echo -e "\n${RED}Failed tests:${NC}"
    for test in "${FAILED_TESTS[@]}"; do
        echo -e "  ${RED}✗${NC} $test"
    done
    exit 1
else
    echo -e "\n${GREEN}All tests passed!${NC}"
    exit 0
fi
