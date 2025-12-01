#!/bin/bash
set -e

# Integration tests for atomic deployment claims
# These tests verify the actual CLI behavior matches the documented safety guarantees

AVON="./target/debug/avon"
TEST_DIR="/tmp/avon_atomic_test"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"

echo "Testing atomic deployment safety claims..."

# Test 1: Evaluation failure -> no files written
echo ""
echo "Test 1: Evaluation failure prevents file writing"
cat > "$TEST_DIR/bad_program.av" << 'EOF'
let x = "hello" in
x + 42  # Type error: string + number
EOF

$AVON deploy "$TEST_DIR/bad_program.av" --root "$TEST_DIR/output" 2>&1 | grep -q "expected.*found\|type mismatch" || {
    echo "FAIL: Should have reported type error"
    exit 1
}

if [ -d "$TEST_DIR/output" ]; then
    if [ "$(ls -A $TEST_DIR/output 2>/dev/null)" ]; then
        echo "FAIL: Files were written despite evaluation failure"
        exit 1
    fi
fi
echo "PASS: No files written on evaluation failure"

# Test 2: Missing env_var -> no files written
echo ""
echo "Test 2: Missing env_var prevents deployment"
cat > "$TEST_DIR/secret_program.av" << 'EOF'
let secret = env_var "MISSING_SECRET" in
@config.yml {"key: {secret}"}
EOF

$AVON deploy "$TEST_DIR/secret_program.av" --root "$TEST_DIR/output2" 2>&1 | grep -q "is not set" || {
    echo "FAIL: Should have reported missing env var"
    exit 1
}

if [ -d "$TEST_DIR/output2" ]; then
    if [ "$(ls -A $TEST_DIR/output2 2>/dev/null)" ]; then
        echo "FAIL: Files were written despite missing secret"
        exit 1
    fi
fi
echo "PASS: No files written when env_var is missing"

# Test 3: Non-deployable result -> no files written
echo ""
echo "Test 3: Non-deployable result prevents deployment"
cat > "$TEST_DIR/string_program.av" << 'EOF'
"just a string, not a FileTemplate"
EOF

$AVON deploy "$TEST_DIR/string_program.av" --root "$TEST_DIR/output3" 2>&1 | grep -q "not deployable\|expected filetemplate" || {
    echo "FAIL: Should have reported non-deployable result"
    exit 1
}

if [ -d "$TEST_DIR/output3" ]; then
    if [ "$(ls -A $TEST_DIR/output3 2>/dev/null)" ]; then
        echo "FAIL: Files were written despite non-deployable result"
        exit 1
    fi
fi
echo "PASS: No files written for non-deployable result"

# Test 4: Valid deployment -> files are written
echo ""
echo "Test 4: Valid deployment writes files correctly"
cat > "$TEST_DIR/valid_program.av" << 'EOF'
let files = [@test1.txt {"content1"}, @test2.txt {"content2"}] in files
EOF

$AVON deploy "$TEST_DIR/valid_program.av" --root "$TEST_DIR/output4" --force

if [ ! -f "$TEST_DIR/output4/test1.txt" ] || [ ! -f "$TEST_DIR/output4/test2.txt" ]; then
    echo "FAIL: Valid files were not written"
    exit 1
fi

if [ "$(cat $TEST_DIR/output4/test1.txt)" != "content1" ] || [ "$(cat $TEST_DIR/output4/test2.txt)" != "content2" ]; then
    echo "FAIL: File contents are incorrect"
    exit 1
fi
echo "PASS: Valid deployment writes files correctly"

# Test 5: Directory creation failure -> no files written
echo ""
echo "Test 5: Directory creation failure prevents deployment"
# Create a read-only parent directory to simulate permission failure
mkdir -p "$TEST_DIR/readonly_parent"
chmod 555 "$TEST_DIR/readonly_parent"  # Read-only, no write permission

cat > "$TEST_DIR/nested_program.av" << 'EOF'
@nested/deep/file.txt {"content"}
EOF

$AVON deploy "$TEST_DIR/nested_program.av" --root "$TEST_DIR/readonly_parent/output5" 2>&1 | grep -q "Failed to create directory\|Deployment aborted" || {
    echo "FAIL: Should have reported directory creation failure"
    chmod 755 "$TEST_DIR/readonly_parent"
    exit 1
}

# Check that no files were written in the readonly directory
if [ -d "$TEST_DIR/readonly_parent/output5" ]; then
    if [ "$(ls -A $TEST_DIR/readonly_parent/output5 2>/dev/null)" ]; then
        echo "FAIL: Files were written despite directory creation failure"
        chmod 755 "$TEST_DIR/readonly_parent"
        exit 1
    fi
fi

chmod 755 "$TEST_DIR/readonly_parent"  # Restore permissions for cleanup
echo "PASS: No files written when directory creation fails"

# Test 6: Multiple files - if one fails, deployment stops immediately
echo ""
echo "Test 6: Write failure stops deployment immediately (atomicity)"
# Test that if one file can't be written, no files are written
# Create a read-only file to simulate write failure
mkdir -p "$TEST_DIR/output6"
touch "$TEST_DIR/output6/b.txt"
chmod 444 "$TEST_DIR/output6/b.txt"  # Read-only

cat > "$TEST_DIR/multi_program.av" << 'EOF'
[@a.txt {"a"}, @b.txt {"b"}, @c.txt {"c"}]
EOF

# Try to deploy - should fail because b.txt is read-only
$AVON deploy "$TEST_DIR/multi_program.av" --root "$TEST_DIR/output6" --force 2>&1 | grep -q "Cannot write\|Deployment aborted" || {
    echo "FAIL: Should have reported write failure"
    chmod 644 "$TEST_DIR/output6/b.txt"
    exit 1
}

# Verify NO files were written (atomicity)
if [ -f "$TEST_DIR/output6/a.txt" ] || [ -f "$TEST_DIR/output6/c.txt" ]; then
    echo "FAIL: Some files were written despite write failure (not atomic!)"
    chmod 644 "$TEST_DIR/output6/b.txt"
    exit 1
fi

chmod 644 "$TEST_DIR/output6/b.txt"  # Restore permissions
echo "PASS: No files written when one file write fails (atomic deployment)"

# Test 6b: All files written when deployment succeeds
echo ""
echo "Test 6b: All files written in successful deployment"
rm -rf "$TEST_DIR/output6"
mkdir -p "$TEST_DIR/output6"

$AVON deploy "$TEST_DIR/multi_program.av" --root "$TEST_DIR/output6" --force

if [ ! -f "$TEST_DIR/output6/a.txt" ] || [ ! -f "$TEST_DIR/output6/b.txt" ] || [ ! -f "$TEST_DIR/output6/c.txt" ]; then
    echo "FAIL: Not all files were written"
    exit 1
fi
echo "PASS: All files written in successful deployment"

# Cleanup
rm -rf "$TEST_DIR"

echo ""
echo "All atomic deployment tests passed!"

