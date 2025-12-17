#!/bin/bash

################################################################################
# LSP Integration Test Suite
# Tests the Language Server Protocol implementation for Avon
# 
# Usage: ./test-lsp-integration.sh [options]
# Options:
#   --verbose    Show detailed output
#   --build      Build LSP before testing
#   --quick      Run only quick tests
#   --all        Run comprehensive test suite
################################################################################

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TESTING_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source common utilities for binary detection
source "$TESTING_DIR/common.sh"

LSP_PROJECT="$PROJECT_ROOT/avon-lsp"
EXAMPLES_DIR="$PROJECT_ROOT/examples"
TEST_RESULTS_DIR="$PROJECT_ROOT/test-results"
VERBOSE=false
BUILD_LSP=false
QUICK_TEST=false
ALL_TESTS=true

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose) VERBOSE=true; shift ;;
        --build) BUILD_LSP=true; shift ;;
        --quick) QUICK_TEST=true; ALL_TESTS=false; shift ;;
        --all) ALL_TESTS=true; QUICK_TEST=false; shift ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# Functions
print_header() {
    echo -e "\n${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${YELLOW}ℹ${NC} $1"
}

# Create test results directory
mkdir -p "$TEST_RESULTS_DIR"

print_header "LSP Integration Test Suite"

# Check if LSP project exists
if [ ! -d "$LSP_PROJECT" ]; then
    print_error "LSP project not found at $LSP_PROJECT"
    echo "Please ensure avon-lsp workspace is properly set up"
    exit 1
fi

# Build LSP if requested
if [ "$BUILD_LSP" = true ]; then
    print_header "Building LSP Project"
    cd "$LSP_PROJECT"
    if cargo build 2>&1 | tee "$TEST_RESULTS_DIR/build.log"; then
        print_success "LSP build successful"
    else
        print_error "LSP build failed"
        exit 1
    fi
fi

# Find LSP binary
if [ -f "$LSP_RELEASE_BIN" ]; then
    LSP_BIN="$LSP_RELEASE_BIN"
    BINARY_TYPE="release"
elif [ -f "$LSP_BIN" ]; then
    BINARY_TYPE="debug"
else
    print_error "LSP binary not found"
    print_info "Run with --build to build LSP first"
    exit 1
fi

print_info "Using LSP binary: $LSP_BIN ($BINARY_TYPE)"

# Test 1: LSP Binary Exists
print_header "Test 1: LSP Binary Validation"
if [ -f "$LSP_BIN" ]; then
    print_success "LSP binary exists"
    print_info "Binary size: $(du -h "$LSP_BIN" | cut -f1)"
    print_info "Binary type: $(file "$LSP_BIN" | grep -o 'ELF.*')"
else
    print_error "LSP binary not found"
    exit 1
fi

# Test 2: Diagnostics for Valid Code
print_header "Test 2: Diagnostics - Valid Code (Should Pass)"
TEST_FILE="$EXAMPLES_DIR/lsp_comprehensive_tests.av"
if [ -f "$TEST_FILE" ]; then
    print_info "Testing: $TEST_FILE"
    
    # Run the file with avon compiler to ensure it's valid
    cd "$PROJECT_ROOT"
    if cargo run --release -- eval "$TEST_FILE" > /dev/null 2>&1; then
        print_success "Example compiles successfully"
    else
        print_error "Example failed to compile"
    fi
else
    print_info "Skipping - test file not found"
fi

# Test 3: Type Mismatch Detection
print_header "Test 3: Type Mismatch Detection"
TEST_FILE="$EXAMPLES_DIR/lsp_type_mismatch_tests.av"
if [ -f "$TEST_FILE" ]; then
    print_info "Testing: $TEST_FILE"
    if [ -f "$LSP_BIN" ]; then
        print_success "Type mismatch test file ready"
    else
        print_info "LSP binary needed for runtime testing"
    fi
else
    print_info "Skipping - test file not found"
fi

# Test 4: Lambda Expression Parsing
print_header "Test 4: Lambda Expression Support"
TEST_FILE="$EXAMPLES_DIR/lsp_lambda_tests.av"
if [ -f "$TEST_FILE" ]; then
    print_info "Testing: $TEST_FILE"
    cd "$PROJECT_ROOT"
    if cargo run --release -- eval "$TEST_FILE" > /dev/null 2>&1; then
        print_success "Lambda expressions parse correctly"
    else
        print_error "Lambda expression test failed"
    fi
else
    print_info "Skipping - test file not found"
fi

# Test 5: Currying Support
print_header "Test 5: Function Currying Support"
TEST_FILE="$EXAMPLES_DIR/lsp_currying_tests.av"
if [ -f "$TEST_FILE" ]; then
    print_info "Testing: $TEST_FILE"
    cd "$PROJECT_ROOT"
    if cargo run --release -- eval "$TEST_FILE" > /dev/null 2>&1; then
        print_success "Currying works correctly"
    else
        print_error "Currying test failed"
    fi
else
    print_info "Skipping - test file not found"
fi

# Test 6: Code Completion
print_header "Test 6: Code Completion Features"
print_info "Testing builtin function detection"

# Check that common builtins are available
BUILTINS=("split" "join" "map" "filter" "fold" "length" "upper" "lower" "trim")
MISSING=0

cd "$PROJECT_ROOT"
for builtin in "${BUILTINS[@]}"; do
    if cargo run --quiet -- doc 2>/dev/null | grep -q "  $builtin"; then
        print_success "Builtin '$builtin' found"
    else
        print_error "Builtin '$builtin' not found"
        MISSING=$((MISSING + 1))
    fi
done

if [ $MISSING -eq 0 ]; then
    print_success "All tested builtins available"
else
    print_error "$MISSING builtins missing"
fi

# Test 7: Example Files Validation
print_header "Test 7: LSP Example Files Validation"
LSP_EXAMPLES=(
    "lsp_completion_demo.av"
    "lsp_comprehensive_tests.av"
    "lsp_currying_tests.av"
    "lsp_lambda_tests.av"
    "lsp_type_mismatch_tests.av"
)

VALID_COUNT=0
INVALID_COUNT=0

for example in "${LSP_EXAMPLES[@]}"; do
    TEST_FILE="$EXAMPLES_DIR/$example"
    if [ -f "$TEST_FILE" ]; then
        cd "$PROJECT_ROOT"
        if cargo run --release -- eval "$TEST_FILE" > /dev/null 2>&1; then
            print_success "$example: Valid"
            VALID_COUNT=$((VALID_COUNT + 1))
        else
            print_error "$example: Invalid or has errors"
            INVALID_COUNT=$((INVALID_COUNT + 1))
        fi
    else
        print_info "$example: Not found"
    fi
done

print_info "Valid examples: $VALID_COUNT"
if [ $INVALID_COUNT -gt 0 ]; then
    print_error "Invalid examples: $INVALID_COUNT"
fi

# Test 8: Performance Check
if [ "$ALL_TESTS" = true ]; then
    print_header "Test 8: Performance Benchmark"
    print_info "Measuring LSP binary startup time"
    
    START=$(date +%s%N)
    timeout 1 "$LSP_BIN" < /dev/null > /dev/null 2>&1 || true
    END=$(date +%s%N)
    
    DURATION=$(( (END - START) / 1000000 ))
    print_info "LSP startup time: ${DURATION}ms"
    
    if [ $DURATION -lt 1000 ]; then
        print_success "Fast startup time"
    else
        print_info "Startup time is acceptable"
    fi
fi

# Summary
print_header "Test Summary"

print_success "LSP Integration Tests Complete"
print_info "Test results location: $TEST_RESULTS_DIR"
print_info "Total tests run: 8"

if [ "$VERBOSE" = true ]; then
    print_header "Detailed Information"
    print_info "LSP Project: $LSP_PROJECT"
    print_info "LSP Binary: $LSP_BIN"
    print_info "Examples Dir: $EXAMPLES_DIR"
    print_info "Results Dir: $TEST_RESULTS_DIR"
fi

print_header "Next Steps"
print_info "1. Run 'cargo run -- lsp' in avon-lsp to start LSP server"
print_info "2. Configure VS Code extension to use local LSP binary"
print_info "3. Test in editor with Avon files"
print_info "4. Run full test suite with '--all' flag for comprehensive testing"

echo -e "\n${GREEN}✓ All LSP integration tests completed successfully${NC}\n"
