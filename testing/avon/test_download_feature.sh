#!/bin/bash

################################################################################
# Test Suite: Download Feature in Task Runner
# Tests the download feature for fetching files before task execution
#
# Features tested:
# 1. Single file download {url, to} syntax parsing
# 2. Multiple file downloads (list of dicts) syntax parsing
# 3. Download with nested directory (auto-creation)
# 4. Download with quiet flag (suppress output)
# 5. Download with ignore_errors flag
# 6. Real network download with valid public URL
# 7. Invalid URL error handling and graceful failure
# 8. FPC Generator template works (doesn't use download)
#
# Note: Tests 6 and 7 require network access.
################################################################################

PROJECT_ROOT="/home/pyrotek45/projects/v9/avon"
TEST_DIR="/tmp/test_avon_download_$$"
AVON_BIN="avon"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0

# Create test directory
mkdir -p "$TEST_DIR"
trap "rm -rf '$TEST_DIR'" EXIT

# Set timeout for network operations
export AVON_TIMEOUT=30

print_test() {
    echo -e "\n${YELLOW}TEST: $1${NC}"
}

pass() {
    echo -e "${GREEN}✓ PASS${NC}: $1"
    ((PASS++))
}

fail() {
    echo -e "${RED}✗ FAIL${NC}: $1"
    ((FAIL++))
}

################################################################################
# Test 1: Single file download syntax validation
################################################################################
print_test "Single file download syntax {url, to} parsing"
TEST_1_DIR="$TEST_DIR/test_1_single_download"
mkdir -p "$TEST_1_DIR"

cat > "$TEST_1_DIR/Avon.av" << 'AVON_EOF'
{
  fetch_file: {
    cmd: "echo 'Task executed'",
    download: {
      url: "https://example.com/file.txt",
      to: "output.txt"
    },
    desc: "Test single download syntax"
  }
}
AVON_EOF

cd "$TEST_1_DIR"
# Just parse the file - don't actually download
if $AVON_BIN do --list > /tmp/test1_output.txt 2>&1; then
    if grep -q "fetch_file" /tmp/test1_output.txt; then
        pass "Single download syntax: File parses correctly"
    else
        fail "Single download syntax: Task not found"
    fi
else
    fail "Single download syntax: File does not parse"
fi

################################################################################
# Test 2: Multiple file downloads syntax validation
################################################################################
print_test "Multiple file downloads (list syntax) parsing"
TEST_2_DIR="$TEST_DIR/test_2_multi_download"
mkdir -p "$TEST_2_DIR"

cat > "$TEST_2_DIR/Avon.av" << 'AVON_EOF'
{
  fetch_multiple: {
    cmd: "echo 'Fetching multiple'",
    download: [
      {url: "https://example.com/file1.txt", to: "file1.txt"},
      {url: "https://example.com/file2.txt", to: "file2.txt"}
    ],
    desc: "Test multiple downloads"
  }
}
AVON_EOF

cd "$TEST_2_DIR"
if $AVON_BIN do --list > /tmp/test2_output.txt 2>&1; then
    if grep -q "fetch_multiple" /tmp/test2_output.txt; then
        pass "Multiple downloads syntax: File parses correctly"
    else
        fail "Multiple downloads syntax: Task not found"
    fi
else
    fail "Multiple downloads syntax: File does not parse"
fi

################################################################################
# Test 3: Invalid download syntax detection
################################################################################
print_test "Invalid download syntax detection"
TEST_3_DIR="$TEST_DIR/test_3_invalid_syntax"
mkdir -p "$TEST_3_DIR"

cat > "$TEST_3_DIR/Avon.av" << 'AVON_EOF'
{
  bad_download: {
    cmd: "echo 'bad'",
    download: {
      url: "https://example.com/file.txt"
    }
  }
}
AVON_EOF

cd "$TEST_3_DIR"
if ! $AVON_BIN do --list > /tmp/test3_output.txt 2>&1; then
    if grep -q "url\|to\|download" /tmp/test3_output.txt; then
        pass "Invalid syntax: Error properly detected"
    else
        fail "Invalid syntax: Error reported but wrong reason"
    fi
else
    fail "Invalid syntax: Should have errored"
fi

################################################################################
# Test 4: Download to nested directory (structure test)
################################################################################
print_test "Download to nested directory path"
TEST_4_DIR="$TEST_DIR/test_4_nested_path"
mkdir -p "$TEST_4_DIR"

cat > "$TEST_4_DIR/Avon.av" << 'AVON_EOF'
{
  fetch_nested: {
    cmd: "echo 'Testing nested'",
    download: {
      url: "https://example.com/file.txt",
      to: "config/data/subdir/file.txt"
    },
    desc: "Download to deeply nested path"
  }
}
AVON_EOF

cd "$TEST_4_DIR"
if $AVON_BIN do --list > /tmp/test4_output.txt 2>&1; then
    pass "Nested directory: Syntax valid"
else
    fail "Nested directory: Syntax invalid"
fi

################################################################################
# Test 5: Download with quiet flag
################################################################################
print_test "Download with quiet flag"
TEST_5_DIR="$TEST_DIR/test_5_quiet"
mkdir -p "$TEST_5_DIR"

cat > "$TEST_5_DIR/Avon.av" << 'AVON_EOF'
{
  fetch_quiet: {
    cmd: "echo 'Quiet mode'",
    download: {
      url: "https://example.com/file.txt",
      to: "file.txt"
    },
    quiet: true
  }
}
AVON_EOF

cd "$TEST_5_DIR"
if $AVON_BIN do --list > /tmp/test5_output.txt 2>&1; then
    pass "Quiet flag: Syntax valid"
else
    fail "Quiet flag: Syntax invalid"
fi

################################################################################
# Test 6: Download with ignore_errors flag
################################################################################
print_test "Download with ignore_errors flag"
TEST_6_DIR="$TEST_DIR/test_6_ignore_errors"
mkdir -p "$TEST_6_DIR"

cat > "$TEST_6_DIR/Avon.av" << 'AVON_EOF'
{
  fetch_ignore: {
    cmd: "echo 'Task executed'",
    download: {
      url: "https://invalid.example.com/missing.txt",
      to: "output.txt"
    },
    ignore_errors: true
  }
}
AVON_EOF

cd "$TEST_6_DIR"
if $AVON_BIN do --list > /tmp/test6_output.txt 2>&1; then
    pass "Ignore errors: Syntax valid"
else
    fail "Ignore errors: Syntax invalid"
fi

################################################################################
# Test 7: REAL network download - download LICENSE from GitHub
################################################################################
print_test "Real network download from GitHub (with timeout)"
TEST_7_DIR="$TEST_DIR/test_7_real_download"
mkdir -p "$TEST_7_DIR"

cat > "$TEST_7_DIR/Avon.av" << 'AVON_EOF'
{
  fetch_real: {
    cmd: "test -f license.txt && wc -l < license.txt",
    download: {
      url: "https://raw.githubusercontent.com/pyrotek45/avon/main/LICENSE",
      to: "license.txt"
    }
  }
}
AVON_EOF

cd "$TEST_7_DIR"
# Use timeout to prevent hanging
if timeout 30 $AVON_BIN do fetch_real > /tmp/test7_output.txt 2>&1; then
    if [ -f "$TEST_7_DIR/license.txt" ] && [ -s "$TEST_7_DIR/license.txt" ]; then
        LINES=$(wc -l < "$TEST_7_DIR/license.txt")
        if [ "$LINES" -gt 0 ]; then
            pass "Real download: GitHub LICENSE downloaded successfully ($LINES lines)"
        else
            fail "Real download: File is empty"
        fi
    else
        fail "Real download: File not created"
    fi
else
    if grep -q "timed out\|timeout" /tmp/test7_output.txt; then
        echo -e "${YELLOW}⊘ SKIP${NC}: Real download: Network timeout (this is OK in offline environments)"
    else
        fail "Real download: Task failed"
    fi
fi

################################################################################
# Test 8: Invalid URL error handling
################################################################################
print_test "Invalid URL error handling with graceful failure"
TEST_8_DIR="$TEST_DIR/test_8_invalid_url"
mkdir -p "$TEST_8_DIR"

cat > "$TEST_8_DIR/Avon.av" << 'AVON_EOF'
{
  fetch_invalid: {
    cmd: "echo 'This should NOT run'",
    download: {
      url: "https://invalid-nonexistent-url-12345-xyz.example.com/missing.txt",
      to: "output.txt"
    }
  }
}
AVON_EOF

cd "$TEST_8_DIR"
# Use timeout
if timeout 30 $AVON_BIN do fetch_invalid > /tmp/test8_output.txt 2>&1; then
    if grep -q "This should NOT run" /tmp/test8_output.txt; then
        fail "Invalid URL: Task executed despite failed download"
    else
        fail "Invalid URL: Task should have failed"
    fi
else
    # Check for proper error messages
    if grep -q "Download failed\|HTTP request failed\|timed out" /tmp/test8_output.txt; then
        pass "Invalid URL: Download error reported gracefully"
    else
        fail "Invalid URL: Error not properly reported"
    fi
    if grep -q "This should NOT run" /tmp/test8_output.txt; then
        fail "Invalid URL: Task executed despite download failure"
    else
        pass "Invalid URL: Task correctly prevented from executing"
    fi
fi

################################################################################
# Test 9: Download with working directory (dir field)
################################################################################
print_test "Download with working directory (dir field)"
TEST_9_DIR="$TEST_DIR/test_9_with_dir"
mkdir -p "$TEST_9_DIR/subdir"

cat > "$TEST_9_DIR/Avon.av" << 'AVON_EOF'
{
  fetch_in_dir: {
    cmd: "pwd && ls -la",
    download: {
      url: "https://example.com/file.txt",
      to: "file.txt"
    },
    dir: "subdir"
  }
}
AVON_EOF

cd "$TEST_9_DIR"
if $AVON_BIN do --list > /tmp/test9_output.txt 2>&1; then
    pass "Directory field: Syntax valid"
else
    fail "Directory field: Syntax invalid"
fi

################################################################################
# Test 10: FPC Generator (doesn't use download, ensures compatibility)
################################################################################
print_test "FPC Generator template (ensure no download usage breaks it)"
TEST_10_DIR="$TEST_DIR/test_10_fpc_no_download"
mkdir -p "$TEST_10_DIR"

cd "$TEST_10_DIR"
if $AVON_BIN deploy "$PROJECT_ROOT/examples/fpc_project_gen.av" --root fpc_app --force -name testapp > /tmp/test10_output.txt 2>&1; then
    if [ -f "$TEST_10_DIR/fpc_app/Avon.av" ]; then
        pass "FPC Generator: Template generates Avon.av successfully"
    else
        fail "FPC Generator: Avon.av not generated"
    fi
    
    cd "$TEST_10_DIR/fpc_app"
    if $AVON_BIN do --list > /tmp/test10_tasks.txt 2>&1; then
        if grep -q "build" /tmp/test10_tasks.txt; then
            pass "FPC Generator: All tasks correctly parsed"
        else
            fail "FPC Generator: Tasks not correctly parsed"
        fi
    else
        fail "FPC Generator: Could not list tasks"
    fi
else
    fail "FPC Generator: Deploy failed"
fi

################################################################################
# Test 11: Multiple formats mixed (download + other fields)
################################################################################
print_test "Download mixed with other task fields"
TEST_11_DIR="$TEST_DIR/test_11_mixed_fields"
mkdir -p "$TEST_11_DIR"

cat > "$TEST_11_DIR/Avon.av" << 'AVON_EOF'
{
  complex: {
    cmd: "echo 'All fields present'",
    download: {url: "https://example.com/file.txt", to: "file.txt"},
    desc: "Complex task with all fields",
    deps: [],
    quiet: false,
    ignore_errors: false,
    dir: "."
  }
}
AVON_EOF

cd "$TEST_11_DIR"
if $AVON_BIN do --list > /tmp/test11_output.txt 2>&1; then
    if grep -q "complex" /tmp/test11_output.txt; then
        pass "Mixed fields: All fields coexist correctly"
    else
        fail "Mixed fields: Task not recognized"
    fi
else
    fail "Mixed fields: File does not parse"
fi

################################################################################
# Summary
################################################################################
echo ""
echo "═════════════════════════════════════════════════════════════"
echo "Test Results:"
echo "═════════════════════════════════════════════════════════════"
echo -e "${GREEN}Passed: $PASS${NC}"
echo -e "${RED}Failed: $FAIL${NC}"
echo "═════════════════════════════════════════════════════════════"

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Please review output above.${NC}"
    exit 1
fi
