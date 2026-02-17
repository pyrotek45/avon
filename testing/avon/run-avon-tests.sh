#!/bin/bash
# Avon Language Test Runner
# Runs all test scripts for the Avon template language.
# Called by: testing/run-all.sh (main entry point)

TESTING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT_ROOT="$(cd "$TESTING_DIR/.." && pwd)"
AVON_TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

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

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Avon Language Test Runner${NC}"
echo -e "${BLUE}========================================${NC}"

# ── Core Language Tests ──────────────────────────────────
print_section "Core Language"
run_test "Grammar"                "$AVON_TEST_DIR/test_grammar.sh"
run_test "Template Syntax"        "$AVON_TEST_DIR/test_template_syntax.sh"
run_test "Arithmetic & Overflow"  "$AVON_TEST_DIR/test_arithmetic.sh"
run_test "Scoping Rules"          "$AVON_TEST_DIR/test_scoping_rules.sh"
run_test "None Handling"          "$AVON_TEST_DIR/test_none_handling.sh"
run_test "Claims"                 "$AVON_TEST_DIR/test_claims.sh"

# ── Parser & Lexer ───────────────────────────────────────
print_section "Parser & Lexer"
run_test "Parser & Lexer"         "$AVON_TEST_DIR/test_parser_lexer.sh"

# ── Builtin Functions ────────────────────────────────────
print_section "Builtin Functions"
run_test "Builtin Functions"      "$AVON_TEST_DIR/test_builtin_functions.sh"
run_test "Advanced Builtins"      "$AVON_TEST_DIR/test_advanced_builtins.sh"
run_test "Extended Coverage"      "$AVON_TEST_DIR/test_extended_coverage.sh"
run_test "Deep Coverage"          "$AVON_TEST_DIR/test_deep_coverage.sh"

# ── Error Handling ───────────────────────────────────────
print_section "Error Handling"
run_test "Error Handling"         "$AVON_TEST_DIR/test_error_handling.sh"

# ── File Paths & Security ────────────────────────────────
print_section "File Paths & Security"
run_test "Path Literal Blocking"  "$AVON_TEST_DIR/test_path_literal_block.sh"
run_test "Path Traversal"         "$AVON_TEST_DIR/test_path_traversal.sh"
run_test "Security Comprehensive" "$AVON_TEST_DIR/test_security_comprehensive.sh"
run_test "Root Relative Paths"    "$AVON_TEST_DIR/test_root_relative_paths.sh"

# ── Example Files ────────────────────────────────────────
print_section "Example Validation"
run_test "All Examples"           "$AVON_TEST_DIR/test_all_examples.sh"
run_test "Tutorial Snippets"      "$AVON_TEST_DIR/test_tutorial_snippets.sh"
run_test "Markdown"               "$AVON_TEST_DIR/test_markdown.sh"

# ── REPL & Parallel ─────────────────────────────────────
print_section "REPL & Parallel"
run_test "REPL"                   "$AVON_TEST_DIR/test_repl.sh"
run_test "Parallel Functions"     "$AVON_TEST_DIR/test_parallel_functions.sh"

# ── Summary ──────────────────────────────────────────────
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Avon Language Tests Summary${NC}"
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
    echo -e "\n${GREEN}All Avon language tests passed!${NC}"
    exit 0
fi
