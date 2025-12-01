#!/bin/bash
# Test that absolute path literals (@/) are blocked by the lexer in both run and deploy

AVON="./target/debug/avon"
TMP_FILE="/tmp/avon_path_literal_block.av"

set -e

echo '@/abs.txt {"x"}' > "$TMP_FILE"

# run should fail with lexer error
if $AVON run "@/abs.txt {\"x\"}" 2>&1 | grep -qi 'absolute paths are not allowed'; then
  echo "✓ PASS: Lexer blocks absolute path literal in run"
else
  echo "✗ FAIL: Lexer did not block absolute path literal in run"
  exit 1
fi

# deploy should also fail with lexer error
if $AVON deploy "$TMP_FILE" --root "/tmp/avon_out" 2>&1 | grep -qi 'absolute paths are not allowed'; then
  echo "✓ PASS: Lexer blocks absolute path literal in deploy"
else
  echo "✗ FAIL: Lexer did not block absolute path literal in deploy"
  exit 1
fi

rm -f "$TMP_FILE"
