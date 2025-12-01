#!/bin/bash
# Test all examples from tutorial/TEMPLATE_SYNTAX.md
# This ensures all documentation examples are accurate

AVON="./target/debug/avon"
FAILED=0
PASSED=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test helper function
test_example() {
    local name="$1"
    local code="$2"
    local expected="$3"
    
    # Run the code and capture output
    local actual
    actual=$($AVON run "$code" 2>&1) || true
    
    if [ "$actual" = "$expected" ]; then
        echo -e "${GREEN}✓${NC} $name"
        ((PASSED++))
    else
        echo -e "${RED}✗${NC} $name"
        echo "  Code: $code"
        echo "  Expected: $expected"
        echo "  Got: $actual"
        ((FAILED++))
    fi
}

# Test helper for file-based tests (multiline)
test_file() {
    local name="$1"
    local file="$2"
    local expected="$3"
    
    local actual
    actual=$($AVON "$file" 2>&1) || true
    
    if [ "$actual" = "$expected" ]; then
        echo -e "${GREEN}✓${NC} $name"
        ((PASSED++))
    else
        echo -e "${RED}✗${NC} $name"
        echo "  File: $file"
        echo "  Expected:"
        echo "$expected" | sed 's/^/    /'
        echo "  Got:"
        echo "$actual" | sed 's/^/    /'
        ((FAILED++))
    fi
}

echo "========================================"
echo "Testing TEMPLATE_SYNTAX.md Examples"
echo "========================================"
echo

# Create temp directory for test files
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# ========================================
# Basic Template Syntax
# ========================================
echo -e "${YELLOW}Basic Template Syntax${NC}"

test_example "Basic template" \
    '{"Hello, world!"}' \
    "Hello, world!"

test_example "Basic interpolation" \
    'let name = "Alice" in {"Hello, {name}!"}' \
    "Hello, Alice!"

test_example "Expression interpolation" \
    'let x = 5 in {"The value is {x} and doubled is {x * 2}"}' \
    "The value is 5 and doubled is 10"

echo

# ========================================
# Multi-Brace Delimiter System
# ========================================
echo -e "${YELLOW}Multi-Brace Delimiter System${NC}"

test_example "Level 1 - single brace" \
    'let x = 5 in {"Value: {x}"}' \
    "Value: 5"

test_example "Level 2 - double brace with literal" \
    'let x = 5 in {{"Value: {{x}} and literal {braces}"}}' \
    "Value: 5 and literal {braces}"

test_example "Level 3 - triple brace with literals" \
    'let x = 5 in {{{"Value: {{{x}}} and {{literal}} and {also literal}"}}}' \
    "Value: 5 and {{literal}} and {also literal}"

echo

# ========================================
# Handling Literal Braces - JSON
# ========================================
echo -e "${YELLOW}Handling Literal Braces${NC}"

# JSON example (multiline)
cat > "$TMPDIR/json_test.av" << 'EOF'
let name = "Alice" in
let age = 30 in
{{"
{
  "name": "{{name}}",
  "age": {{age}}
}
"}}
EOF

test_file "JSON generation" "$TMPDIR/json_test.av" '{
  "name": "Alice",
  "age": 30
}'

# JavaScript class example
cat > "$TMPDIR/js_class.av" << 'EOF'
let className = "MyClass" in
let fieldName = "value" in
{{"
class {{className}} {
  constructor() {
    this.{{fieldName}} = 0;
  }
}
"}}
EOF

test_file "JavaScript class generation" "$TMPDIR/js_class.av" 'class MyClass {
  constructor() {
    this.value = 0;
  }
}'

echo

# ========================================
# Meta-Templates
# ========================================
echo -e "${YELLOW}Meta-Templates${NC}"

cat > "$TMPDIR/meta_template.av" << 'EOF'
let varName = "username" in
let placeholder = "{" + varName + "}" in
{{{"
# This Avon template uses {{{varName}}}:
{"Hello, {{{placeholder}}}!"}
"}}}
EOF

test_file "Meta-template generation" "$TMPDIR/meta_template.av" '# This Avon template uses username:
{"Hello, {username}!"}'

cat > "$TMPDIR/static_meta.av" << 'EOF'
{{{"
# A static Avon template example:
{"Hello, {name}!"}
"}}}
EOF

test_file "Static meta-template" "$TMPDIR/static_meta.av" '# A static Avon template example:
{"Hello, {name}!"}'

echo

# ========================================
# Common Use Cases
# ========================================
echo -e "${YELLOW}Common Use Cases${NC}"

# Docker Compose
cat > "$TMPDIR/docker.av" << 'EOF'
let service = "web" in
let port = 8080 in
{{"
services:
  {{service}}:
    build: .
    ports:
      - "{{port}}:{{port}}"
    environment:
      NODE_ENV: production
"}}
EOF

test_file "Docker Compose" "$TMPDIR/docker.av" 'services:
  web:
    build: .
    ports:
      - "8080:8080"
    environment:
      NODE_ENV: production'

# Kubernetes ConfigMap
cat > "$TMPDIR/k8s.av" << 'EOF'
let appName = "myapp" in
let configData = "key=value" in
{{"
apiVersion: v1
kind: ConfigMap
metadata:
  name: {{appName}}-config
data:
  config.properties: |
    {{configData}}
"}}
EOF

test_file "Kubernetes ConfigMap" "$TMPDIR/k8s.av" 'apiVersion: v1
kind: ConfigMap
metadata:
  name: myapp-config
data:
  config.properties: |
    key=value'

# GitHub Actions
cat > "$TMPDIR/github.av" << 'EOF'
let jobName = "build" in
{{"
jobs:
  {{jobName}}:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm install
"}}
EOF

test_file "GitHub Actions" "$TMPDIR/github.av" 'jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm install'

# React/JSX
cat > "$TMPDIR/jsx.av" << 'EOF'
let componentName = "Greeting" in
let propName = "name" in
let jsx = \v "{" + v + "}" in

{{" 
function {{componentName}}({{jsx propName}}) {
  return <div>Hello, {{jsx propName}}!</div>;
}
"}}
EOF

test_file "React/JSX component" "$TMPDIR/jsx.av" 'function Greeting({name}) {
  return <div>Hello, {name}!</div>;
}'

echo

# ========================================
# Quick Reference Examples
# ========================================
echo -e "${YELLOW}Quick Reference${NC}"

test_example "Level 1 plain text" \
    'let x = 5 in {"Value: {x}"}' \
    "Value: 5"

test_example "Level 2 JSON" \
    'let x = 5 in {{"{"value": {{x}}}"}}' \
    '{"value": 5}'

test_example "Level 3 meta-template" \
    'let x = 5 in {{{"Template: {{{x}}} and {{literal}}"}}}' \
    "Template: 5 and {{literal}}"

echo

# ========================================
# Tips and Tricks
# ========================================
echo -e "${YELLOW}Tips and Tricks${NC}"

# String concatenation for braces
test_example "String concat braces" \
    'let x = "value" in "{" + x + "}"' \
    "{value}"

# Template-based wrapper with spaces
test_example "Template wrapper (spaced)" \
    'let brace = \s {{"{ {{s}} }"}} in brace "name"' \
    "{ name }"

# Plus operator wrapper (no spaces)
test_example "Plus operator wrapper" \
    'let brace = \s "{" + s + "}" in brace "name"' \
    "{name}"

# Mustache wrapper
test_example "Mustache wrapper" \
    'let mustache = \s "{{" + s + "}}" in mustache "value"' \
    "{{value}}"

# GitHub expression wrapper
test_example "GitHub expression wrapper" \
    'let gh_expr = \s "${{" + s + "}}" in gh_expr "github.ref"' \
    '${{github.ref}}'

# Shell variable wrapper
test_example "Shell variable wrapper" \
    'let shell_var = \v "$" + "{" + v + "}" in shell_var "PATH"' \
    '${PATH}'

# Shell variable in echo
test_example "Shell var in echo" \
    'let shell_var = \v "$" + "{" + v + "}" in let var = "PATH" in "echo " + (shell_var var)' \
    "echo \${PATH}"

# Multiple brace styles
test_example "Multiple brace styles" \
    'let single = \s "{" + s + "}" in let double = \s "{{" + s + "}}" in let field1 = "name" in let field2 = "age" in "Single: " + (single field1) + "\nDouble: " + (double field2)' \
    "Single: {name}
Double: {{age}}"

echo

# ========================================
# Escape Sequences
# ========================================
echo -e "${YELLOW}Escape Sequences${NC}"

test_example "String escape newline" \
    '"Line 1\nLine 2"' \
    "Line 1
Line 2"

test_example "Template literal backslash" \
    '{"Line 1\nLine 2"}' \
    'Line 1\nLine 2'

echo

# ========================================
# Type Conversions
# ========================================
echo -e "${YELLOW}Type Conversions${NC}"

test_example "to_string for numbers" \
    'let count = 42 in "{" + (to_string count) + "}"' \
    "{42}"

echo

# ========================================
# Mixing Template Levels
# ========================================
echo -e "${YELLOW}Mixing Template Levels${NC}"

cat > "$TMPDIR/mixing.av" << 'EOF'
let name = "user" in
let mustache = \s "{{" + s + "}}" in
let placeholder = mustache "data" in

{{" {"name": "{{name}}", "template": "{{placeholder}}"}"}}
EOF

test_file "Mixing levels (JSON + Mustache)" "$TMPDIR/mixing.av" '{"name": "user", "template": "{{data}}"}'

echo

# ========================================
# What Gets Interpolated
# ========================================
echo -e "${YELLOW}What Gets Interpolated${NC}"

test_example "List interpolation" \
    'let list = [1, 2, 3] in {"items: {list}"}' \
    "items: 1
2
3"

test_example "Dict interpolation" \
    'let d = { name: "Alice" } in {"data: {d}"}' \
    'data: {name: "Alice"}'

test_example "Newline from string variable" \
    'let nl = "\n" in {"line1{nl}line2"}' \
    "line1
line2"

echo

# ========================================
# Common Patterns Library
# ========================================
echo -e "${YELLOW}Common Patterns Library${NC}"

test_example "brace helper" \
    'let brace = \s "{" + s + "}" in brace "name"' \
    "{name}"

test_example "mustache helper" \
    'let mustache = \s "{{" + s + "}}" in mustache "value"' \
    "{{value}}"

test_example "mustache_raw helper" \
    'let mustache_raw = \s "{{{" + s + "}}}" in mustache_raw "html"' \
    "{{{html}}}"

test_example "shell_var helper" \
    'let shell_var = \v "$" + "{" + v + "}" in shell_var "HOME"' \
    '${HOME}'

test_example "gh_expr helper" \
    'let gh_expr = \s "${{" + s + "}}" in gh_expr "secrets.TOKEN"' \
    '${{secrets.TOKEN}}'

test_example "jinja helper" \
    'let jinja = \s "{{ " + s + " }}" in jinja "variable"' \
    "{{ variable }}"

test_example "erb helper" \
    'let erb = \s "<%= " + s + " %>" in erb "code"' \
    "<%= code %>"

echo

# ========================================
# Template-based wrappers
# ========================================
echo -e "${YELLOW}Template-based Wrappers${NC}"

test_example "brace_spaced" \
    'let brace_spaced = \s {{"{ {{s}} }"}} in brace_spaced "name"' \
    "{ name }"

test_example "mustache_spaced" \
    'let mustache_spaced = \s {{{"{{ {{{s}}} }}"}}} in mustache_spaced "value"' \
    "{{ value }}"

echo

# ========================================
# Summary
# ========================================
echo "========================================"
echo "Results"
echo "========================================"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"

if [ $FAILED -gt 0 ]; then
    echo
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
else
    echo
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi
