#!/bin/bash

# Testing suite for the publish builtin function
# Tests dynamic FileTemplate creation for various use cases

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

TMPDIR=$(mktemp -d)
cleanup() {
    rm -rf "$TMPDIR"
}
trap cleanup EXIT

# Path to avon binary
AVON_BIN="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/target/release/avon"

if [ ! -f "$AVON_BIN" ]; then
    echo "Error: Avon binary not found at $AVON_BIN"
    echo "Please build with: cargo build --release"
    exit 1
fi

passed=0
failed=0

test_file() {
    local name=$1
    local file=$2
    local expected=$3

    if [ ! -f "$file" ]; then
        echo -e "${RED}✗${NC} $name (file not created)"
        ((failed++))
        return
    fi

    local result=$(cat "$file")
    if [ "$result" = "$expected" ]; then
        echo -e "${GREEN}✓${NC} $name"
        ((passed++))
    else
        echo -e "${RED}✗${NC} $name"
        echo "  Expected: $expected"
        echo "  Got: $result"
        ((failed++))
    fi
}

test_no_error() {
    local name=$1
    local file=$2

    if "$AVON_BIN" eval "$file" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} $name"
        ((passed++))
    else
        echo -e "${RED}✗${NC} $name (eval error)"
        ((failed++))
    fi
}

# ====== Tests ======

echo -e "${YELLOW}Testing publish builtin function${NC}"
echo

# Test 1: Basic publish with string literals
cat > "$TMPDIR/basic_publish.av" << 'EOF'
publish "output.txt" "Hello, World!"
EOF

test_no_error "publish with string literals" "$TMPDIR/basic_publish.av"

# Test 2: publish with variable path
cat > "$TMPDIR/variable_path.av" << 'EOF'
let filename = "greeting.txt" in
publish filename "Hello!"
EOF

test_no_error "publish with variable path" "$TMPDIR/variable_path.av"

# Test 3: publish with template
cat > "$TMPDIR/with_template.av" << 'EOF'
let name = "Alice" in
publish "hello.txt" {"Hello, {name}!"}
EOF

test_no_error "publish with template" "$TMPDIR/with_template.av"

# Test 4: publish with computed path
cat > "$TMPDIR/computed_path.av" << 'EOF'
let env = "prod" in
publish ("config/" + env + ".yml") "port: 443"
EOF

test_no_error "publish with computed path" "$TMPDIR/computed_path.av"

# Test 5: publish in map to generate multiple files
cat > "$TMPDIR/publish_in_map.av" << 'EOF'
["one", "two", "three"] -> map (\name publish (name + ".txt") "content")
EOF

test_no_error "publish in map" "$TMPDIR/publish_in_map.av"

# Test 6: publish in function
cat > "$TMPDIR/publish_in_function.av" << 'EOF'
let make_file = \name \content
  publish ("output/" + name) content
in
make_file "test.txt" "test content"
EOF

test_no_error "publish in function" "$TMPDIR/publish_in_function.av"

# Test 7: publish with path value
cat > "$TMPDIR/publish_with_path.av" << 'EOF'
publish @src/main.rs "fn main() {}"
EOF

test_no_error "publish with path value" "$TMPDIR/publish_with_path.av"

# Test 8: deploy publish to actually create files
cat > "$TMPDIR/deploy_publish.av" << 'EOF'
let filename = "test_file.txt" in
publish filename "This is test content"
EOF

# Deploy and check file was created
"$AVON_BIN" deploy "$TMPDIR/deploy_publish.av" --root "$TMPDIR/deploy_output" --force > /dev/null 2>&1
test_file "deploy publish creates file" "$TMPDIR/deploy_output/test_file.txt" "This is test content"

# Test 9: deploy multiple files from map
cat > "$TMPDIR/deploy_multiple.av" << 'EOF'
["a", "b", "c"] -> 
map (\name publish (name + ".txt") ("Content of " + name))
EOF

"$AVON_BIN" deploy "$TMPDIR/deploy_multiple.av" --root "$TMPDIR/deploy_multi" --force > /dev/null 2>&1
test_file "deploy multiple files - a" "$TMPDIR/deploy_multi/a.txt" "Content of a"
test_file "deploy multiple files - b" "$TMPDIR/deploy_multi/b.txt" "Content of b"
test_file "deploy multiple files - c" "$TMPDIR/deploy_multi/c.txt" "Content of c"

# Test 10: Comparison of publish vs @path {{}} syntax
cat > "$TMPDIR/compare_syntax1.av" << 'EOF'
@test.txt {"hello"}
EOF

cat > "$TMPDIR/compare_syntax2.av" << 'EOF'
publish "test.txt" "hello"
EOF

test_no_error "traditional @path {{}} syntax" "$TMPDIR/compare_syntax1.av"
test_no_error "publish syntax" "$TMPDIR/compare_syntax2.av"

# Test 11: publish with multiline content
cat > "$TMPDIR/multiline.av" << 'EOF'
publish "README.md" {"# Project
This is a README
With multiple lines"}
EOF

"$AVON_BIN" deploy "$TMPDIR/multiline.av" --root "$TMPDIR/multiline_out" --force > /dev/null 2>&1
test_file "publish multiline content" "$TMPDIR/multiline_out/README.md" "# Project
This is a README
With multiple lines"

# Test 12: publish with nested path construction
cat > "$TMPDIR/nested_paths.av" << 'EOF'
let base = "src" in
let subdir = "utils" in
let filename = "helper.rs" in
publish (base + "/" + subdir + "/" + filename) "pub fn help() {}"
EOF

test_no_error "publish with nested path construction" "$TMPDIR/nested_paths.av"

# Test 13: publish in filter
cat > "$TMPDIR/publish_with_filter.av" << 'EOF'
["file1.txt", "file2.rs", "file3.txt"] ->
filter (\f ends_with f ".txt") ->
map (\f publish f "text file content")
EOF

test_no_error "publish with filter" "$TMPDIR/publish_with_filter.av"

# Test 14: Using publish to generate Avon code
cat > "$TMPDIR/gen_avon.av" << 'EOF'
let version = "0.1.0" in
publish "version.av" {"let VERSION = \"{version}\" in VERSION"}
EOF

"$AVON_BIN" deploy "$TMPDIR/gen_avon.av" --root "$TMPDIR/gen_avon_out" --force > /dev/null 2>&1
test_file "publish can generate Avon code" "$TMPDIR/gen_avon_out/version.av" 'let VERSION = \"0.1.0\" in VERSION'

# Test 15: Real-world example: generating config files for multiple environments
cat > "$TMPDIR/multi_env_config.av" << 'EOF'
let environments = ["dev", "staging", "prod"] in
let make_config = \env \port
  publish ("config-{env}.yml") {"environment: {env}\nport: {port}"}
in
[
  make_config "dev" "8080",
  make_config "staging" "8443",
  make_config "prod" "443"
]
EOF

test_no_error "real-world: multi-environment configs" "$TMPDIR/multi_env_config.av"

echo
echo -e "${GREEN}Passed: $passed${NC}"
if [ $failed -gt 0 ]; then
    echo -e "${RED}Failed: $failed${NC}"
    exit 1
else
    echo "All tests passed!"
fi
