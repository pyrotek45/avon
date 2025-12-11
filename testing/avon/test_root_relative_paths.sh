#!/bin/bash
# Test that --root flag accepts relative paths and works correctly with nested file paths
# This is a regression test for the bug where relative --root paths would falsely detect
# path traversal when deploying files with nested paths like @config/file.json

AVON="./target/debug/avon"
TEST_DIR=$(mktemp -d)
trap "rm -rf $TEST_DIR" EXIT

echo "Testing --root with relative paths..."

# Test 1: Basic relative --root with simple file
TEST_FILE="$TEST_DIR/test1.av"
echo '@config.txt {"hello"}' > "$TEST_FILE"

OUTPUT_DIR="$TEST_DIR/output1"
if $AVON deploy "$TEST_FILE" --root ./output1 --force > /dev/null 2>&1; then
  if [ -f ./output1/config.txt ]; then
    echo "✓ PASS: Basic relative --root works"
  else
    echo "✗ FAIL: File not created with relative --root"
    exit 1
  fi
else
  echo "✗ FAIL: deploy failed with relative --root"
  exit 1
fi
rm -rf ./output1

# Test 2: Relative --root with nested file paths (regression test)
TEST_FILE="$TEST_DIR/test2.av"
echo '@config/app.json {"{}"}' > "$TEST_FILE"

if $AVON deploy "$TEST_FILE" --root ./output2 --force > /dev/null 2>&1; then
  if [ -f ./output2/config/app.json ]; then
    echo "✓ PASS: Relative --root with nested paths works (regression test)"
  else
    echo "✗ FAIL: Nested file not created with relative --root"
    exit 1
  fi
else
  echo "✗ FAIL: deploy failed with relative --root and nested paths"
  exit 1
fi
rm -rf ./output2

# Test 3: Deeply nested relative --root
TEST_FILE="$TEST_DIR/test3.av"
echo '@app/config/settings.yml {"debug: true"}' > "$TEST_FILE"

if $AVON deploy "$TEST_FILE" --root ./my/nested/output --force > /dev/null 2>&1; then
  if [ -f ./my/nested/output/app/config/settings.yml ]; then
    echo "✓ PASS: Deeply nested relative --root works"
  else
    echo "✗ FAIL: File not created in deeply nested relative --root"
    exit 1
  fi
else
  echo "✗ FAIL: deploy failed with deeply nested relative --root"
  exit 1
fi
rm -rf ./my

# Test 4: Path traversal still blocked (security check)
TEST_FILE="$TEST_DIR/test4.av"
echo '@../escape.txt {"hack"}' > "$TEST_FILE"

if $AVON deploy "$TEST_FILE" --root ./output4 --force 2>&1 | grep -q "Path traversal detected\|Path contains"; then
  echo "✓ PASS: Path traversal still blocked with relative --root"
else
  echo "✗ FAIL: Path traversal not blocked (security regression)"
  exit 1
fi
rm -rf ./output4

# Test 5: Absolute --root still works
TEST_FILE="$TEST_DIR/test5.av"
echo '@test.txt {"content"}' > "$TEST_FILE"
ABS_OUT="$TEST_DIR/abs_output"

if $AVON deploy "$TEST_FILE" --root "$ABS_OUT" --force > /dev/null 2>&1; then
  if [ -f "$ABS_OUT/test.txt" ]; then
    echo "✓ PASS: Absolute --root still works"
  else
    echo "✗ FAIL: File not created with absolute --root"
    exit 1
  fi
else
  echo "✗ FAIL: deploy failed with absolute --root"
  exit 1
fi

echo ""
echo "All relative --root tests passed!"
