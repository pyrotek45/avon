#!/bin/bash
# Comprehensive path traversal security tests
# Ensures files cannot escape the --root directory

# Don't use set -e here - we need to check exit codes manually

AVON="./target/debug/avon"
TEST_DIR="/tmp/avon_path_traversal_test"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"

echo "Testing path traversal security..."
echo "=================================="
echo ""

FAILED=0
PASSED=0

# Test function
test_traversal_blocked() {
    local test_name="$1"
    local avon_code="$2"
    local expected_error="$3"
    
    local test_file="$TEST_DIR/test_${RANDOM}.av"
    echo "$avon_code" > "$test_file"
    
    # Run command and capture both output and exit code
    # Use || true to prevent script from exiting, but capture the actual exit code
    local output
    local exit_code
    output=$($AVON deploy "$test_file" --root "$TEST_DIR/root" 2>&1) || exit_code=$?
    # If command succeeded, exit_code will be unset, so set it to 0
    exit_code=${exit_code:-0}
    
    # Check if deployment failed (which is expected for traversal attempts)
    # Exit code should be non-zero (1) for failures
    if [ $exit_code -eq 0 ]; then
        echo "✗ FAIL: $test_name (deployment succeeded, should have failed)"
        echo "  Code: $avon_code"
        echo "  Output: $output"
        echo "  Exit code: $exit_code"
        ((FAILED++))
        return 1
    fi
    
    # Check if the error message matches expected pattern
    if echo "$output" | grep -qiE "$expected_error"; then
        echo "✓ PASS: $test_name"
        ((PASSED++))
        return 0
    else
        echo "✗ FAIL: $test_name (wrong error message)"
        echo "  Code: $avon_code"
        echo "  Expected pattern: $expected_error"
        echo "  Got: $output"
        echo "  Exit code: $exit_code"
        ((FAILED++))
        return 1
    fi
}

# Test 1: Basic parent directory traversal
echo "Test 1: Basic parent directory traversal"
test_traversal_blocked \
    "Basic .. traversal" \
    '@../../etc/passwd {"test"}' \
    "Path traversal detected|contains.*\.\."

# Test 2: Multiple parent directories
echo ""
echo "Test 2: Multiple parent directories"
test_traversal_blocked \
    "Multiple .. traversal" \
    '@../../../etc/passwd {"test"}' \
    "Path traversal detected|contains.*\.\."

# Test 3: Parent directory in middle of path
echo ""
echo "Test 3: Parent directory in middle of path"
test_traversal_blocked \
    ".. in middle" \
    '@safe/../../etc/passwd {"test"}' \
    "Path traversal detected|contains.*\.\."

# Test 4: Encoded parent directory (should be filtered by component parsing)
echo ""
echo "Test 4: Path with .. component"
test_traversal_blocked \
    "Path with .. component" \
    '@subdir/../etc/passwd {"test"}' \
    "Path traversal detected|contains.*\.\."

# Test 5: Absolute-looking path (should be allowed within root)
echo ""
echo "Test 5: Absolute-looking path within root (should succeed)"
mkdir -p "$TEST_DIR/root"
cat > "$TEST_DIR/absolute.av" << 'EOF'
@etc/passwd {"test content"}
EOF

if $AVON deploy "$TEST_DIR/absolute.av" --root "$TEST_DIR/root" --force > /dev/null 2>&1; then
    if [ -f "$TEST_DIR/root/etc/passwd" ]; then
        # Verify it's actually within root, not the real /etc/passwd
        if [ -f "/etc/passwd" ]; then
            if ! diff -q "$TEST_DIR/root/etc/passwd" "/etc/passwd" > /dev/null 2>&1; then
                echo "✓ PASS: Absolute-looking path works within root (safe)"
                ((PASSED++))
            else
                echo "✗ FAIL: Absolute-looking path wrote to real /etc/passwd"
                ((FAILED++))
            fi
        else
            echo "✓ PASS: Absolute-looking path works within root (safe)"
            ((PASSED++))
        fi
    else
        echo "✗ FAIL: Absolute-looking path - file not created"
        ((FAILED++))
    fi
else
    echo "✗ FAIL: Absolute-looking path - deployment failed"
    ((FAILED++))
fi

# Test 6: Symlink traversal (if symlinks exist)
echo ""
echo "Test 6: Symlink traversal protection"
mkdir -p "$TEST_DIR/root/safe"
mkdir -p "$TEST_DIR/root/escape_target"
# Create a symlink inside root that points outside
ln -sf "$TEST_DIR/escape_target" "$TEST_DIR/root/safe/link" 2>/dev/null || true

# Test 6: Symlink traversal (if symlinks exist)
echo ""
echo "Test 6: Symlink traversal protection"
mkdir -p "$TEST_DIR/root/safe"
mkdir -p "$TEST_DIR/root/escape_target"
# Create a symlink inside root that points outside
ln -sf "$TEST_DIR/escape_target" "$TEST_DIR/root/safe/link" 2>/dev/null || true

# For symlink tests, any error that prevents writing is acceptable
test_file="$TEST_DIR/test_symlink_${RANDOM}.av"
echo '@safe/link/file.txt {"test"}' > "$test_file"

output=$($AVON deploy "$test_file" --root "$TEST_DIR/root" 2>&1) || exit_code=$?
exit_code=${exit_code:-0}

if [ $exit_code -ne 0 ]; then
    # Any error that prevents writing is acceptable for symlink protection
    echo "✓ PASS: Symlink traversal (blocked by error)"
    ((PASSED++))
else
    echo "✗ FAIL: Symlink traversal (deployment succeeded)"
    echo "  Output: $output"
    ((FAILED++))
fi

# Test 7: Normal path should work
echo ""
echo "Test 7: Normal path (should succeed)"
mkdir -p "$TEST_DIR/root"
cat > "$TEST_DIR/normal.av" << 'EOF'
@safe/file.txt {"content"}
EOF

if $AVON deploy "$TEST_DIR/normal.av" --root "$TEST_DIR/root" --force > /dev/null 2>&1; then
    if [ -f "$TEST_DIR/root/safe/file.txt" ]; then
        echo "✓ PASS: Normal path works correctly"
        ((PASSED++))
    else
        echo "✗ FAIL: Normal path - file not created"
        ((FAILED++))
    fi
else
    echo "✗ FAIL: Normal path - deployment failed"
    ((FAILED++))
fi

# Test 8: Nested paths should work
echo ""
echo "Test 8: Nested paths (should succeed)"
cat > "$TEST_DIR/nested.av" << 'EOF'
@deeply/nested/path/file.txt {"content"}
EOF

if $AVON deploy "$TEST_DIR/nested.av" --root "$TEST_DIR/root" --force > /dev/null 2>&1; then
    if [ -f "$TEST_DIR/root/deeply/nested/path/file.txt" ]; then
        echo "✓ PASS: Nested paths work correctly"
        ((PASSED++))
    else
        echo "✗ FAIL: Nested paths - file not created"
        ((FAILED++))
    fi
else
    echo "✗ FAIL: Nested paths - deployment failed"
    ((FAILED++))
fi

# Test 9: Path with variables (interpolation)
echo ""
echo "Test 9: Path with variables"
test_traversal_blocked \
    "Variable path with .." \
    'let evil = "../../etc" in @{evil}/passwd {"test"}' \
    "Path traversal detected|contains.*\.\."

# Test 10: Multiple files - one with traversal
echo ""
echo "Test 10: Multiple files with one traversal"
cat > "$TEST_DIR/mixed.av" << 'EOF'
[@safe/file1.txt {"ok"}, @../../etc/passwd {"evil"}, @safe/file2.txt {"ok"}]
EOF

output=$($AVON deploy "$TEST_DIR/mixed.av" --root "$TEST_DIR/root" 2>&1)
if echo "$output" | grep -qiE "Path traversal detected|contains.*\.\."; then
    # Check that NO files were written (atomic deployment)
    if [ ! -f "$TEST_DIR/root/safe/file1.txt" ] && [ ! -f "$TEST_DIR/root/safe/file2.txt" ]; then
        echo "✓ PASS: Atomic deployment - no files written when traversal detected"
        ((PASSED++))
    else
        echo "✗ FAIL: Files were written despite traversal detection"
        ((FAILED++))
    fi
else
    echo "✗ FAIL: Traversal not detected in mixed file list"
    ((FAILED++))
fi

# Summary
echo ""
echo "=================================="
echo "Path Traversal Security Test Summary:"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo ""

# Cleanup
rm -rf "$TEST_DIR"

if [ $FAILED -eq 0 ]; then
    echo "✅ All path traversal security tests passed!"
    exit 0
else
    echo "❌ Some path traversal security tests failed!"
    exit 1
fi

