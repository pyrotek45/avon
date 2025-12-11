#!/bin/bash
# Test runner for file import functionality
# Tests glob, json_parse, import, and related functions

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEST_DIR="$PROJECT_ROOT/testing/imports"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PASSED=0
FAILED=0
FAILED_TESTS=()

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}IMPORT AND FILE I/O TESTS${NC}"
echo -e "${BLUE}========================================${NC}\n"

cd "$TEST_DIR" || exit 1

# Run each test file
for test_file in test_*.av; do
    test_name=$(basename "$test_file" .av)
    
    if avon eval "$test_file" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ $test_name${NC}"
        ((PASSED++))
    else
        echo -e "${RED}✗ $test_name${NC}"
        ((FAILED++))
        FAILED_TESTS+=("$test_name")
    fi
done

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "Results: ${GREEN}$PASSED passed${NC}, ${RED}$FAILED failed${NC}"

if [ $FAILED -gt 0 ]; then
    echo -e "\n${RED}Failed tests:${NC}"
    for test in "${FAILED_TESTS[@]}"; do
        echo -e "  ${RED}✗ $test${NC}"
    done
    exit 1
else
    echo -e "\n${GREEN}All import tests passed!${NC}"
    exit 0
fi
