#!/bin/bash
# Unified Test Suite for Avon
# Runs all tests to verify the interpreter works correctly
# This script combines all individual test scripts into one comprehensive test suite

set -e  # Exit on first error

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
SKIPPED=0

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TESTING_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$TESTING_DIR/.." && pwd)"
AVON_TESTS_DIR="$SCRIPT_DIR"

# Change to project root
cd "$PROJECT_ROOT"

# Test wrapper
run_test() {
    local test_name="$1"
    local test_script="$2"
    
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Running: $test_name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    if [ ! -f "$test_script" ]; then
        echo -e "${YELLOW}⚠ SKIPPED${NC}: $test_name (script not found: $test_script)"
        ((SKIPPED++)) || true
        return 0
    fi
    
    if bash "$test_script" > /tmp/test_output.txt 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}: $test_name"
        ((PASSED++)) || true
    else
        echo -e "${RED}✗ FAILED${NC}: $test_name"
        echo "Error output:"
        cat /tmp/test_output.txt | head -50
        if [ $(wc -l < /tmp/test_output.txt) -gt 50 ]; then
            echo "... (output truncated)"
        fi
        ((FAILED++)) || true
    fi
}

echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║          Avon Unified Test Suite                               ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Check for shell.nix and load environment if it exists
if [ -f "shell.nix" ]; then
    echo -e "${BLUE}ℹ${NC} Found shell.nix, loading nix-shell environment..."
    if command -v nix-shell >/dev/null 2>&1; then
        echo "  Running: nix-shell --run 'bash $0'"
        echo "  (This will reload the script in the nix environment)"
        exec nix-shell --run "bash $0"
    else
        echo -e "${YELLOW}⚠${NC} nix-shell not found, continuing without nix environment"
    fi
fi

# Build the project
echo ""
echo "Building project..."
if cargo build --quiet 2>&1; then
    echo -e "${GREEN}✓${NC} Build successful"
else
    echo -e "${RED}✗${NC} Build failed"
    exit 1
fi

# Run Cargo unit tests
echo ""
echo "Running Cargo unit tests..."
if cargo test --quiet --release 2>&1; then
    echo -e "${GREEN}✓${NC} Cargo tests passed"
    ((PASSED++)) || true
else
    echo -e "${RED}✗${NC} Cargo tests failed"
    ((FAILED++)) || true
fi

# Run test suites in logical order
echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║          Running Test Suites                                  ║"
echo "╚════════════════════════════════════════════════════════════════╝"

# Core language tests
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Core Language Tests${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
run_test "Grammar Comprehensive Test" "$AVON_TESTS_DIR/test_grammar.sh"
run_test "Bulletproof Comprehensive Test" "$AVON_TESTS_DIR/test_bulletproof.sh"
run_test "Scoping Rules Test" "$AVON_TESTS_DIR/test_scoping_rules.sh"
run_test "Template Syntax Test" "$AVON_TESTS_DIR/test_template_syntax.sh"
run_test "Tutorial Snippets Test" "$AVON_TESTS_DIR/test_tutorial_snippets.sh"
run_test "None Handling Test" "$AVON_TESTS_DIR/test_none_handling.sh"

# Example tests
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Example Tests${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
run_test "All Examples Test" "$AVON_TESTS_DIR/test_all_examples.sh"
run_test "Example Outputs Test" "$AVON_TESTS_DIR/test_example_outputs.sh"
run_test "Run Examples (Deployment) Test" "$AVON_TESTS_DIR/run_examples.sh"

# Deployment and integration tests
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Deployment & Integration Tests${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
run_test "Atomic Deployment Test" "$AVON_TESTS_DIR/test_atomic_deployment.sh"
run_test "Backup Test" "$AVON_TESTS_DIR/test_backup.sh"
run_test "Markdown to HTML Test" "$AVON_TESTS_DIR/test_markdown.sh"

# Security tests
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Security Tests${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
run_test "Path Traversal Security Test" "$AVON_TESTS_DIR/test_path_traversal.sh"
run_test "Fuzz Security Test" "$AVON_TESTS_DIR/fuzz.sh"
run_test "Comprehensive Security Test" "$AVON_TESTS_DIR/test_security_comprehensive.sh"

# Lexer path literal blocking
run_test "Absolute Path Literal Blocking Test" "$AVON_TESTS_DIR/test_path_literal_block.sh"

# REPL test
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}REPL Test${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
run_test "REPL Test" "$AVON_TESTS_DIR/test_repl.sh"

# Summary
echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║          Test Summary                                          ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo -e "  ${GREEN}Passed: $PASSED${NC}"
echo -e "  ${RED}Failed: $FAILED${NC}"
if [ $SKIPPED -gt 0 ]; then
    echo -e "  ${YELLOW}Skipped: $SKIPPED${NC}"
fi
echo ""

if [ $FAILED -eq 0 ]; then
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║          🎉 All Tests Passed! 🎉                              ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    exit 0
else
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║          ❌ Some Tests Failed ❌                              ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    exit 1
fi

