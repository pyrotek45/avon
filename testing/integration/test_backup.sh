#!/bin/bash
set -e

# Build
cargo build --release

AVON="./target/release/avon"
OUT_DIR="/tmp/avon_backup_test"

# Cleanup
rm -rf $OUT_DIR
mkdir -p $OUT_DIR

# Create a test avon file
echo '@test.txt {"version 1"}' > test_gen.av

# 1. First deploy
echo "Deploying version 1..."
$AVON deploy test_gen.av --root $OUT_DIR
cat $OUT_DIR/test.txt
if ! grep -q "version 1" $OUT_DIR/test.txt; then
    echo "FAIL: version 1 not deployed"
    exit 1
fi

# 2. Update avon file
echo '@test.txt {"version 2"}' > test_gen.av

# 3. Deploy with backup
echo "Deploying version 2 with --backup..."
$AVON deploy test_gen.av --root $OUT_DIR --backup

# Check files
echo "Checking files..."
cat $OUT_DIR/test.txt
if ! grep -q "version 2" $OUT_DIR/test.txt; then
    echo "FAIL: version 2 not deployed"
    exit 1
fi

if [ ! -f "$OUT_DIR/test.txt.bak" ]; then
    echo "FAIL: backup file not created"
    exit 1
fi

if ! grep -q "version 1" "$OUT_DIR/test.txt.bak"; then
    echo "FAIL: backup file content incorrect"
    exit 1
fi

echo "SUCCESS: Backup test passed"

# Cleanup
rm test_gen.av
rm -rf $OUT_DIR
