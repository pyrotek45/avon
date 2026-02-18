#!/bin/bash
# Cargo Test Runner — wraps cargo unit and integration tests
# Called by: testing/run-all.sh (or run standalone)
# This runs all Rust-level tests: unit tests in src/ and integration tests in tests/

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

PASSED=0
FAILED=0

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Cargo Test Runner${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

cd "$PROJECT_ROOT"

# Build first to avoid test timeouts
echo "Building project..."
if ! cargo build 2>&1 | tail -3; then
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi
echo ""

# Run library unit tests
echo "--- Library unit tests ---"
if cargo test --lib 2>&1 | tail -5 | grep -q "test result: ok"; then
    count=$(cargo test --lib 2>&1 | grep "test result" | grep -oP '\d+ passed')
    echo -e "${GREEN}✓ Library unit tests ($count)${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ Library unit tests${NC}"
    cargo test --lib 2>&1 | tail -10
    ((FAILED++))
fi

# Run binary unit tests (includes task_runner tests)
echo ""
echo "--- Binary unit tests ---"
if cargo test --bin avon 2>&1 | tail -5 | grep -q "test result: ok"; then
    count=$(cargo test --bin avon 2>&1 | grep "test result" | grep -oP '\d+ passed')
    echo -e "${GREEN}✓ Binary unit tests ($count)${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ Binary unit tests${NC}"
    cargo test --bin avon 2>&1 | tail -10
    ((FAILED++))
fi

# Run integration tests
echo ""
echo "--- Integration tests (tests/) ---"
if cargo test --test integration_tests 2>&1 | tail -5 | grep -q "test result: ok"; then
    count=$(cargo test --test integration_tests 2>&1 | grep "test result" | grep -oP '\d+ passed')
    echo -e "${GREEN}✓ Integration tests ($count)${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ Integration tests${NC}"
    cargo test --test integration_tests 2>&1 | tail -10
    ((FAILED++))
fi

# Summary
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Cargo Test Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"

if [ $FAILED -eq 0 ]; then
    echo -e "\n${GREEN}All cargo tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}Some cargo tests failed${NC}"
    exit 1
fi
