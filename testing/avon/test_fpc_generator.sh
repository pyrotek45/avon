#!/bin/bash
# Comprehensive tests for FPC project generator
# Tests multiple project types, configurations, and build tasks

set -e

# Determine script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
AVON_BIN="${AVON_BIN:-$PROJECT_ROOT/target/debug/avon}"
TEST_DIR="/tmp/fpc_gen_tests"
TEMPLATE="$PROJECT_ROOT/examples/fpc_project_gen.av"
PASSED=0
FAILED=0

# Cleanup before starting
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"

# Test counter
TEST_NUM=0

# ANSI colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

function test_fpc() {
    local test_name="$1"
    local project_type="$2"
    local name="$3"
    local use_classes="$4"
    local use_testing="$5"
    local test_build="${6:-true}"
    
    TEST_NUM=$((TEST_NUM + 1))
    
    local test_label="${test_name} (${project_type}, classes=${use_classes}, testing=${use_testing})"
    echo -n "Test $TEST_NUM: $test_label... "
    
    local test_root="$TEST_DIR/test_${TEST_NUM}_$(echo $name | tr '/' '_')"
    mkdir -p "$test_root"
    
    # Deploy project
    if ! $AVON_BIN deploy "$TEMPLATE" --root "$test_root" --force \
        -project_type "$project_type" \
        -name "$name" \
        -use_classes "$use_classes" \
        -use_testing "$use_testing" > /dev/null 2>&1; then
        echo -e "${RED}FAIL${NC} (deployment failed)"
        FAILED=$((FAILED + 1))
        return 1
    fi
    
    # Check that expected files exist
    local expected_files=("src/${name}.pas" "Avon.av" ".gitignore" "README.md" "bin/.gitkeep")
    
    if [ "$use_classes" = "true" ]; then
        expected_files+=("src/${name}Core.pas")
    fi
    
    if [ "$use_testing" = "true" ]; then
        expected_files+=("tests/test_runner.pas")
    fi
    
    for file in "${expected_files[@]}"; do
        if [ ! -f "$test_root/$file" ]; then
            echo -e "${RED}FAIL${NC} (missing file: $file)"
            FAILED=$((FAILED + 1))
            return 1
        fi
    done
    
    # Verify Avon.av syntax
    if ! $AVON_BIN do --list --root "$test_root" > /dev/null 2>&1; then
        echo -e "${RED}FAIL${NC} (invalid Avon.av syntax)"
        FAILED=$((FAILED + 1))
        return 1
    fi
    
    # Test build task (if FPC is available and test_build is true)
    if [ "$test_build" = "true" ] && command -v fpc &> /dev/null; then
        if ! (cd "$test_root" && $AVON_BIN do build > /dev/null 2>&1); then
            echo -e "${RED}FAIL${NC} (build failed)"
            FAILED=$((FAILED + 1))
            return 1
        fi
        
        # Verify binary was created
        if [ ! -f "$test_root/bin/$name" ]; then
            echo -e "${RED}FAIL${NC} (binary not created)"
            FAILED=$((FAILED + 1))
            return 1
        fi
        
        # Test run task
        if [ "$project_type" = "library" ]; then
            echo -e "${GREEN}PASS${NC} (skipped run for library)"
        else
            if ! (cd "$test_root" && $AVON_BIN do run > /dev/null 2>&1); then
                echo -e "${RED}FAIL${NC} (run failed)"
                FAILED=$((FAILED + 1))
                return 1
            fi
        fi
        
        # Test test task if enabled
        if [ "$use_testing" = "true" ]; then
            if ! (cd "$test_root" && $AVON_BIN do test > /dev/null 2>&1); then
                echo -e "${RED}FAIL${NC} (test task failed)"
                FAILED=$((FAILED + 1))
                return 1
            fi
        fi
        
        # Test check task
        if ! (cd "$test_root" && $AVON_BIN do check > /dev/null 2>&1); then
            echo -e "${RED}FAIL${NC} (check task failed)"
            FAILED=$((FAILED + 1))
            return 1
        fi
        
        # Test clean task
        if ! (cd "$test_root" && $AVON_BIN do clean > /dev/null 2>&1); then
            echo -e "${RED}FAIL${NC} (clean task failed)"
            FAILED=$((FAILED + 1))
            return 1
        fi
        
        echo -e "${GREEN}PASS${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${GREEN}PASS${NC} (FPC check skipped)"
        PASSED=$((PASSED + 1))
    fi
}

echo "=========================================="
echo "FPC Project Generator Comprehensive Tests"
echo "=========================================="
echo ""

# Test Matrix: Different configurations
echo "Category: Console Projects"
test_fpc "Console basic" "console" "myapp" "false" "false"
test_fpc "Console with classes" "console" "myapp2" "true" "false"
test_fpc "Console with testing" "console" "myapp3" "false" "true"
test_fpc "Console full featured" "console" "myapp4" "true" "true"

echo ""
echo "Category: GUI Projects"
test_fpc "GUI basic" "gui" "mygui" "false" "false" false
test_fpc "GUI with classes" "gui" "mygui2" "true" "false" false
test_fpc "GUI with testing" "gui" "mygui3" "false" "true" false
test_fpc "GUI full featured" "gui" "mygui4" "true" "true" false

echo ""
echo "Category: Library Projects"
test_fpc "Library basic" "library" "mylib" "false" "false" false
test_fpc "Library with classes" "library" "mylib2" "true" "false" false
test_fpc "Library with testing" "library" "mylib3" "false" "true" false
test_fpc "Library full featured" "library" "mylib4" "true" "true" false

echo ""
echo "Category: Special Names"
test_fpc "Project with underscore" "console" "my_project" "true" "true"
test_fpc "Project with numbers" "console" "app2024" "true" "true"

echo ""
echo "=========================================="
echo "Test Results"
echo "=========================================="
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
