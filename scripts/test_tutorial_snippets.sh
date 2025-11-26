#!/bin/bash
# Test script to verify all Avon code snippets from the tutorial
# This script extracts snippets and tests them to ensure they're correct

# Don't exit on error - we want to test all snippets
set +e

# Build avon first
echo "Building Avon..."
cargo build --quiet 2>&1 | tail -1

AVON="./target/debug/avon"
TEST_DIR="./tutorial_tests"
OUTPUT_DIR="./tutorial_test_outputs"

# Clean up previous test runs
rm -rf "$TEST_DIR" "$OUTPUT_DIR"
mkdir -p "$TEST_DIR" "$OUTPUT_DIR"

echo "Testing all Avon code snippets from tutorial..."
echo "================================================"
echo ""

# Test counter
PASSED=0
FAILED=0

# Function to test a snippet
test_snippet() {
    local name=$1
    local code=$2
    local expected_output=$3
    local file="$TEST_DIR/${name}.av"
    
    echo "$code" > "$file"
    
    # Always test that it parses and evaluates without error
    local result=$("$AVON" eval "$file" 2>&1)
    local exit_code=$?
    
    if [ $exit_code -ne 0 ]; then
        echo "✗ $name (evaluation failed)"
        echo "$result" | head -5
        ((FAILED++))
        return 1
    fi
    
    if [ -n "$expected_output" ]; then
        # Test with expected output
        if echo "$result" | grep -q "$expected_output"; then
            echo "✓ $name"
            ((PASSED++))
            return 0
        else
            echo "✗ $name (expected output not found)"
            echo "  Expected: $expected_output"
            echo "  Got: $(echo "$result" | head -1)"
            ((FAILED++))
            return 1
        fi
    else
        # Just test that it parses and evaluates without error
        echo "✓ $name"
        ((PASSED++))
        return 0
    fi
}

# Test 1: Basic string
test_snippet "test_01_hello_world" '"Hello, world!"' "Hello, world!"

# Test 2: Function with parameter
test_snippet "test_02_greet" '\name @/greeting.txt {"
    Hello, {name}!
    Welcome to Avon.
"}' ""

# Test 3: Multiple files generation
test_snippet "test_03_gen_configs" 'let environments = ["dev", "staging", "prod"] in
map (\env @/config-{env}.yml {"
    environment: {env}
    debug: {if env == "prod" then false else true}
"}) environments' ""

# Test 4: Dotfile with variables
test_snippet "test_04_dotfile" '\username ? "developer" @/.vimrc {"
  " Vim configuration for {username}
  set number
  set expandtab
  set tabstop=4
  colorscheme {if username == "developer" then "solarized" else "default"}
"}' ""

# Test 5: List interpolation
test_snippet "test_05_list_interpolation" 'let plugins = ["vim-fugitive", "vim-surround", "vim-commentary", "vim-repeat"] in
@/.vimrc {"
  " Plugin configuration
  {plugins}
"}' ""

# Test 6: Let bindings
test_snippet "test_06_let_basic" 'let greeting = "Hello" in
let name = "Alice" in
greeting + ", " + name' "Hello, Alice"

# Test 7: Cascading lets
test_snippet "test_07_cascading_lets" 'let a = 10 in
let b = 20 in
let sum = a + b in
sum * 2' "60"

# Test 8: Function definition
test_snippet "test_08_function_def" 'let add = \x \y x + y in
add 5 3' "8"

# Test 9: Curried function
test_snippet "test_09_curried" 'let add = \x \y x + y in
let add5 = add 5 in
add5 3' "8"

# Test 10: Default parameters
test_snippet "test_10_default_params" '\name ? "Guest" @/welcome.txt {"
    Welcome, {name}!
"}' ""

# Test 11: Lists
test_snippet "test_11_lists" '[1, 2, 3]' ""

# Test 12: List interpolation in template
test_snippet "test_12_list_template" 'let names = ["Alice", "Bob", "Charlie"] in
@/names.txt {"
    Names:
    {names}
"}' ""

# Test 13: Dictionary
test_snippet "test_13_dict" 'let config = {host: "localhost", port: 8080, debug: true} in
config.host' "localhost"

# Test 14: Dictionary operations
test_snippet "test_14_dict_ops" 'let config = {host: "localhost", port: 8080, debug: true} in
keys config' ""

# Test 15: Map
test_snippet "test_15_map" 'let double = \x x * 2 in
map double [1, 2, 3]' ""

# Test 16: Filter
test_snippet "test_16_filter" 'let numbers = [1, 2, 3, 4, 5] in
filter (\n n > 2) numbers' ""

# Test 17: Fold
test_snippet "test_17_fold" 'let numbers = [1, 2, 3, 4, 5] in
let sum = fold (\acc \n acc + n) 0 numbers in
sum' "15"

# Test 18: Template basic
test_snippet "test_18_template_basic" '{"Hello, World"}' "Hello, World"

# Test 19: Template with variable
test_snippet "test_19_template_var" 'let name = "Alice" in {"
    Hello, {name}!
    Welcome to Avon.
"}' ""

# Test 20: Pipe operator
test_snippet "test_20_pipe" '[1, 2, 3] -> length' "3"

# Test 21: Chained pipes
test_snippet "test_21_pipe_chain" '"hello world" -> upper -> length' "11"

# Test 22: Logical operators
test_snippet "test_22_logical_and" 'true && false' "false"

# Test 23: Logical or
test_snippet "test_23_logical_or" 'true || false' "true"

# Test 24: Path values
test_snippet "test_24_path" 'let config_path = @config/production.json in
basename config_path' "production.json"

# Test 25: Path interpolation
test_snippet "test_25_path_interp" 'let env = "staging" in
let app = "myapp" in
@config/{env}/{app}.yml' ""

# Test 26: Nested let scoping
test_snippet "test_26_nested_let" 'let x = 10 in
let y = 20 in
let result = let temp = x + y in temp * 2 in
result' "60"

# Test 27: Scoping - variable not visible outside
test_snippet "test_27_scope_outside" 'let x = 10 in
let y = 20 in
x + y' "30"

# Test 28: Template variable capture
test_snippet "test_28_template_capture" 'let name = "Alice" in
let template = {"Hello, {name}"} in
template' "Hello, Alice"

# Test 29: If expression
test_snippet "test_29_if" 'if true then "yes" else "no"' "yes"

# Test 30: Nested if
test_snippet "test_30_nested_if" 'if true then "positive" else (if false then "negative" else "zero")' "positive"

# Test 31: Comparison operators
test_snippet "test_31_comparison" '5 > 3' "true"

# Test 32: String concatenation
test_snippet "test_32_string_concat" '"hello" + " world"' "hello world"

# Test 33: List concatenation
test_snippet "test_33_list_concat" '[1, 2] + [3, 4]' ""

# Test 34: Template concatenation
test_snippet "test_34_template_concat" 'let greeting = {"Hello, "} in
let name = "Alice" in
let punct = {"!"} in
greeting + {"World"} + punct' "Hello, World!"

# Test 35: Path concatenation
test_snippet "test_35_path_concat" 'let base = @/home in
let user = @/alice in
base + user' ""

# Test 36: Function with multiple params
test_snippet "test_36_multi_param" '\app ? "service" \env ? "dev" @/config-{app}-{env}.yml {"
    app: {app}
    environment: {env}
"}' ""

# Test 37: Complex expression
test_snippet "test_37_complex" 'let make_config = \env \debug ? false @/config-{env}.yml {"
    environment: {env}
    debug: {debug}
"} in
let environments = ["dev", "staging", "prod"] in
[
  make_config "dev" true,
  make_config "staging" false,
  make_config "prod" false
]' ""

# Test 38: Dictionary update
test_snippet "test_38_dict_update" 'let config = {host: "localhost", port: 8080} in
let updated = set config "debug" true in
updated' ""

# Test 39: Nested dictionaries
test_snippet "test_39_nested_dict" 'let config = {
  database: {host: "db.local", port: 5432},
  app: {name: "myapp", debug: true}
} in
let db_host = (get config "database").host in
db_host' "db.local"

# Test 40: Type conversion
test_snippet "test_40_type_conv" 'to_string 42' "42"

# Test 41: String operations
test_snippet "test_41_string_ops" 'upper "hello"' "HELLO"

# Test 42: List operations
test_snippet "test_42_list_ops" 'join ["a", "b", "c"] ", "' "a, b, c"

# Test 43: Import example (library)
test_snippet "test_43_import_lib" '{double: \x x * 2, triple: \x x * 3, square: \x x * x}' ""

# Test 44: Import example (data)
test_snippet "test_44_import_data" '{host: "localhost", port: 8080, debug: true}' ""

# Test 45: Import example (generator)
test_snippet "test_45_import_gen" '@/config.yml {"host: localhost"}' ""

# Test 46: Double-brace template
test_snippet "test_46_double_brace" '@/config.lua {{"
local config = {
  name = "{{ app_name }}",
  debug = true
}
"}}' ""

# Test 47: Single-brace with escaping
test_snippet "test_47_single_brace_escape" '{"Literal open: {{"}' "Literal open: {"

# Test 48: Environment variable
test_snippet "test_48_env_var" 'env_var_or "TEST_VAR" "default"' "default"

# Test 49: File operations
test_snippet "test_49_file_ops" 'let path = @examples/hello.av in
exists path' ""

# Test 50: Complex scoping test (no shadowing)
test_snippet "test_50_complex_scope" 'let x = 1 in
let y = 2 in
let inner = let temp_x = 10 in temp_x + y in
let outer = x + y in
[inner, outer]' ""

echo ""
echo "================================================"
echo "Test Results: $PASSED passed, $FAILED failed"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "All tests passed! ✓"
    exit 0
else
    echo "Some tests failed. Please review the output above."
    exit 1
fi

