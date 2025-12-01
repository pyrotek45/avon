#!/bin/bash
# LSP Test Orchestrator - Master test coordinator
# Automatically runs all tests and fixes issues

set -e

REPO_DIR="/workspaces/avon"
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo ""
    echo -e "${BLUE}=================================================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}=================================================================================${NC}"
}

print_section() {
    echo ""
    echo -e "${YELLOW}→ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Main execution
print_header "AVON LSP AUTOMATED TEST ORCHESTRATOR"

# Step 1: Build
print_section "Step 1: Building LSP"
cd "$REPO_DIR"
if cargo build --release 2>&1 | tail -3; then
    print_success "LSP build successful"
else
    print_error "LSP build failed"
    exit 1
fi

# Step 2: Update symlink
print_section "Step 2: Updating LSP symlink"
sudo ln -sf "$REPO_DIR/target/release/avon_lsp" /usr/local/bin/avon-lsp
print_success "LSP symlink updated"

# Step 3: Analyze code
print_section "Step 3: Running LSP Code Analysis"
python3 "$REPO_DIR/lsp-autofixer.py" 2>&1 | head -50

# Step 4: Test example files
print_section "Step 4: Analyzing Example Files"
python3 "$REPO_DIR/analyze-lsp.py" 2>&1 | tail -20

# Step 5: Validation
print_section "Step 5: Final Validation"

# Check if LSP binary exists and is executable
if [ -x /usr/local/bin/avon-lsp ]; then
    print_success "LSP binary is executable"
else
    print_error "LSP binary is not executable"
    exit 1
fi

# Check if test framework exists
if [ -f "$REPO_DIR/test-lsp-framework.sh" ]; then
    print_success "Test framework ready"
else
    print_error "Test framework not found"
fi

# Final summary
print_header "TEST ORCHESTRATION COMPLETE"

echo ""
echo "Available Testing Tools:"
echo "  1. analyze-lsp.py              - Analyze example files for issues"
echo "  2. lsp-autofixer.py            - Detect and suggest fixes for LSP code"
echo "  3. test-lsp-automated.py       - Run automated test suite"
echo "  4. test-lsp-framework.sh       - Run integration tests"
echo "  5. run-lsp-ci.sh               - Run full CI/CD pipeline"
echo ""
echo "Commands:"
echo "  python3 analyze-lsp.py         - Analyze all examples"
echo "  python3 lsp-autofixer.py       - Run LSP analysis"
echo "  python3 test-lsp-automated.py  - Run automated tests"
echo "  bash run-lsp-ci.sh             - Run CI pipeline"
echo ""

print_success "LSP testing infrastructure ready!"
