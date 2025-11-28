#!/usr/bin/env bash
# Test that example files produce expected output patterns
# This catches issues like wrong function names, incorrect formatting, etc.

set -euo pipefail

echo "Building avon..."
cargo build --quiet 2>&1 || { echo "Build failed"; exit 1; }

echo ""
echo "Testing example outputs for expected patterns..."
echo "================================================"
echo ""

FAILED=()
PASSED=()

# Helper function to test output contains expected pattern
test_output_contains() {
    local file=$1
    local pattern=$2
    local description=$3
    
    echo -n "Testing $file - $description... "
    
    if output=$(cargo run --quiet -- eval "$file" 2>&1); then
        if echo "$output" | grep -q "$pattern"; then
            echo "✓ PASS"
            PASSED+=("$file: $description")
            return 0
        else
            echo "✗ FAIL (pattern not found)"
            echo "  Expected pattern: $pattern"
            echo "  Actual output (first 200 chars):"
            echo "$output" | head -c 200 | sed 's/^/    /'
            echo ""
            FAILED+=("$file: $description - pattern not found: $pattern")
            return 1
        fi
    else
        echo "✗ FAIL (execution error)"
        echo "  Error:"
        echo "$output" | sed 's/^/    /'
        FAILED+=("$file: $description - execution error")
        return 1
    fi
}

# Helper function to test output does NOT contain error patterns
test_no_error() {
    local file=$1
    local description=$2
    
    echo -n "Testing $file - $description... "
    
    if output=$(cargo run --quiet -- eval "$file" 2>&1); then
        # Check for common error patterns (but not config keywords like "error_log")
        if echo "$output" | grep -E "^(error|Error|ERROR):|unknown symbol|undefined|failed to|Fatal" >/dev/null 2>&1; then
            echo "✗ FAIL (contains error)"
            echo "  Output:"
            echo "$output" | sed 's/^/    /'
            FAILED+=("$file: $description - output contains error")
            return 1
        else
            echo "✓ PASS"
            PASSED+=("$file: $description")
            return 0
        fi
    else
        echo "✗ FAIL (execution error)"
        echo "  Error:"
        echo "$output" | sed 's/^/    /'
        FAILED+=("$file: $description - execution error")
        return 1
    fi
}

# Test specific examples for expected output patterns

# Formatting functions - comprehensive tests
test_output_contains "examples/formatting_demo.av" "Hexadecimal:     ff" "hex formatting"
test_output_contains "examples/formatting_demo.av" "Octal:           377" "octal formatting"
test_output_contains "examples/formatting_demo.av" "Binary:          11111111" "binary formatting"
test_output_contains "examples/formatting_demo.av" "1.23e7" "scientific notation"
test_output_contains "examples/formatting_demo.av" "1.46 MB" "bytes formatting"
test_output_contains "examples/formatting_demo.av" '\$19.99' "currency formatting"
test_output_contains "examples/formatting_demo.av" "85.60%" "percent formatting"
test_output_contains "examples/formatting_demo.av" "Yes" "boolean formatting (yes/no)"
test_output_contains "examples/formatting_demo.av" "Truncated..." "truncate function"
test_output_contains "examples/formatting_demo.av" "Centered" "center function"

# String functions - comprehensive tests
test_no_error "examples/string_functions.av" "string functions work"
test_output_contains "examples/string_functions.av" "HELLO" "uppercase"
test_output_contains "examples/string_functions.av" "hello" "lowercase"
test_output_contains "examples/string_functions.av" "Hello World" "title case"
test_output_contains "examples/string_functions.av" "Hello Avon" "replace result"

# String predicates
test_no_error "examples/string_predicates.av" "string predicates work"
test_output_contains "examples/string_predicates.av" "is_digit" "is_digit predicate"
test_output_contains "examples/string_predicates.av" "is_alpha" "is_alpha predicate"
test_output_contains "examples/string_predicates.av" "is_empty" "is_empty predicate"

# Map operations
test_output_contains "examples/map_example.av" "a-" "map with append"
test_output_contains "examples/map_example.av" "\[a-, b-, c-\]" "map result list"

# Map operations (map data structure)
test_no_error "examples/map_operations.av" "map operations work"
test_output_contains "examples/map_operations.av" "Map Operations Demo" "map operations header"
test_output_contains "examples/map_operations.av" "All keys:" "map keys output"
test_output_contains "examples/map_operations.av" "All values:" "map values output"
test_output_contains "examples/map_operations.av" "has 'name': true" "has_key check"

# Filter operations
test_output_contains "examples/filter_example.av" "\[a, b, c\]" "filtered list"

# JSON operations - thorough validation
test_output_contains "examples/json_map_demo.av" "test-app" "json parsing and map access"
test_output_contains "examples/json_map_demo.av" "1.2.3" "json version field"
test_output_contains "examples/json_map_demo.av" "Alice" "json author field"
test_output_contains "examples/json_map_demo.av" "Port: 3000" "json nested object access"
test_output_contains "examples/json_map_demo.av" "Debug: true" "json boolean value"
test_output_contains "examples/json_map_demo.av" "express" "json array access"
test_output_contains "examples/json_map_demo.av" "lodash" "json array contains lodash"
test_output_contains "examples/json_map_demo.av" "axios" "json array contains axios"

# List operations
test_output_contains "examples/plus_strings.av" "hello world" "string concatenation"
test_output_contains "examples/plus_strings.av" "hello -  world - hello" "repeated concatenation"

# Split and join
test_no_error "examples/split_join.av" "split and join work"
test_output_contains "examples/split_join.av" "one|two|three" "split/join result"

test_no_error "examples/join_replace.av" "join and replace work"

# Contains/starts/ends
test_no_error "examples/contains_starts_ends.av" "string predicates example"

# New functions demo - comprehensive validation
test_no_error "examples/new_functions_demo.av" "no unknown symbol errors"
test_output_contains "examples/new_functions_demo.av" "String length: 11" "length function"
test_output_contains "examples/new_functions_demo.av" "List length: 5" "length on list"
test_output_contains "examples/new_functions_demo.av" "00042" "padded number"
test_output_contains "examples/new_functions_demo.av" "Repeated chars:" "repeat function result"
test_output_contains "examples/new_functions_demo.av" "    host: localhost" "indent function result"
test_output_contains "examples/new_functions_demo.av" "&lt;script&gt;" "html escape XSS protection"
test_output_contains "examples/new_functions_demo.av" "alert" "html escape contains alert"
test_output_contains "examples/new_functions_demo.av" "<p>Hello, World!</p>" "html tag paragraph"
test_output_contains "examples/new_functions_demo.av" '<li><a>Home</a></li>' "html nested tags"
test_output_contains "examples/new_functions_demo.av" "# My Document" "markdown h1 heading"
test_output_contains "examples/new_functions_demo.av" "## Introduction" "markdown h2 heading"
test_output_contains "examples/new_functions_demo.av" "\[Back to Home\]" "markdown link text"
test_output_contains "examples/new_functions_demo.av" "\`const x = 1\`" "markdown inline code"
test_output_contains "examples/new_functions_demo.av" "Write code" "markdown list item"

# Nginx config - comprehensive validation
test_no_error "examples/nginx_gen.av" "no errors"
test_output_contains "examples/nginx_gen.av" "worker_processes 4" "worker processes config"
test_output_contains "examples/nginx_gen.av" "api.example.com" "server name"
test_output_contains "examples/nginx_gen.av" "listen 443 ssl" "ssl listen directive"
test_output_contains "examples/nginx_gen.av" "ssl_protocols TLSv1.2 TLSv1.3" "ssl protocols"
test_output_contains "examples/nginx_gen.av" "localhost:8080" "upstream server 1"
test_output_contains "examples/nginx_gen.av" "localhost:8081" "upstream server 2"
test_output_contains "examples/nginx_gen.av" "localhost:8082" "upstream server 3"
test_output_contains "examples/nginx_gen.av" "proxy_cache_path" "cache configuration"
test_output_contains "examples/nginx_gen.av" "proxy_pass http://backend" "proxy pass directive"
test_output_contains "examples/nginx_gen.av" "gzip on" "gzip enabled"

# Import functionality
test_output_contains "examples/import_example.av" "\[one, two\]" "import works"

# Fold operations
test_output_contains "examples/fold_example.av" "abc" "fold concatenation"

# Path operations
test_output_contains "examples/simple_path_test.av" "Hello, {name}!" "path value read"

# Template operations
test_output_contains "examples/features_demo.av" "Foo Bar" "features demo runs"

# Conditional templates
test_output_contains "examples/conditionals_template.av" "Hello stranger" "conditional in template"

# Practical formatting
test_output_contains "examples/formatting_practical.av" "SERVER CONFIGURATION REPORT" "practical formatting header"
test_output_contains "examples/formatting_practical.av" "8.00 GB" "memory formatting"

# Casting demo - type conversion validation
test_output_contains "examples/casting_demo.av" "TYPE CONVERSION" "casting demo header"
test_output_contains "examples/casting_demo.av" "to_string examples:" "to_string section"
test_output_contains "examples/casting_demo.av" "to_int examples:" "to_int section"
test_output_contains "examples/casting_demo.av" "to_float examples:" "to_float section"
test_output_contains "examples/casting_demo.av" "to_bool examples:" "to_bool section"
test_output_contains "examples/casting_demo.av" "00001, 00002" "padded numbers"
test_output_contains "examples/casting_demo.av" "flatmap" "flatmap operation"

# Builtin currying - partial application
test_output_contains "examples/builtin_currying.av" "Mr. Smith" "curry prefix"
test_output_contains "examples/builtin_currying.av" "Mr. Jones" "curry multiple items"
test_output_contains "examples/builtin_currying.av" "the cat" "curry with 'the'"
test_output_contains "examples/builtin_currying.av" "is valid" "curry validation"

# Pipe operator
test_no_error "examples/pipe_operator.av" "basic pipe operator"
test_output_contains "examples/pipe_operator.av" "2" "pipe result"

test_no_error "examples/pipe_operator_demo.av" "pipe operator demo"
# Expect list of results: [5, 3, 11, 2, [2, 4, 6], false, 50] (config.json doesn't exist)
test_output_contains "examples/pipe_operator_demo.av" "\[5, 3, 11, 2, \[2, 4, 6\], false, 50\]" "pipe demo results"

# Complex usage examples
test_no_error "examples/complex_usage_1.av" "complex usage 1 runs"
test_no_error "examples/complex_usage_2.av" "complex usage 2 runs"
test_no_error "examples/complex_usage_3.av" "complex usage 3 runs"
test_no_error "examples/complex_usage_4.av" "complex usage 4 runs"
test_no_error "examples/complex_usage_5.av" "complex usage 5 runs"

# Generator examples - config file generation
test_output_contains "examples/docker_compose_gen.av" "version:" "docker compose version"
test_output_contains "examples/docker_compose_gen.av" "services:" "docker compose services"
test_output_contains "examples/docker_compose_gen.av" "image:" "docker compose image"
test_output_contains "examples/docker_compose_gen.av" "ports:" "docker compose ports"

test_output_contains "examples/github_actions_gen.av" "name: CI" "github actions CI name"
test_output_contains "examples/github_actions_gen.av" "name: Release" "github actions release name"
test_output_contains "examples/github_actions_gen.av" "actions/checkout" "github actions checkout"
test_output_contains "examples/github_actions_gen.av" "ubuntu-latest" "github actions runner"
test_output_contains "examples/github_actions_gen.av" "createComment" "github actions JS object syntax"

test_output_contains "examples/package_json_gen.av" '"name": "awesome-app"' "package.json name"
test_output_contains "examples/package_json_gen.av" '"version"' "package.json version"
test_output_contains "examples/package_json_gen.av" '"scripts"' "package.json scripts"
test_output_contains "examples/package_json_gen.av" '"dependencies"' "package.json dependencies"

test_no_error "examples/kubernetes_gen.av" "kubernetes config generation"
test_output_contains "examples/kubernetes_gen.av" "apiVersion:" "kubernetes apiVersion"
test_output_contains "examples/kubernetes_gen.av" "kind: Deployment" "kubernetes deployment"

test_no_error "examples/terraform_gen.av" "terraform config generation"
test_output_contains "examples/terraform_gen.av" "production" "terraform environment var"
test_output_contains "examples/terraform_gen.av" "us-east-1" "terraform region"

test_no_error "examples/ci_pipeline.av" "CI pipeline generation"
test_output_contains "examples/ci_pipeline.av" "name: CI" "CI pipeline name"

# Curly brace tests - regression tests for template syntax
test_no_error "examples/curly_test_1_simple_json.av" "curly test 1 - simple json"
test_output_contains "examples/curly_test_1_simple_json.av" '"name":' "simple json has name field"

test_no_error "examples/curly_test_2_nested_json.av" "curly test 2 - nested json"
test_output_contains "examples/curly_test_2_nested_json.av" '"outer":' "nested json has outer key"

test_no_error "examples/curly_test_3_mixed.av" "curly test 3 - mixed syntax"
test_no_error "examples/curly_test_4_array.av" "curly test 4 - array syntax"
test_no_error "examples/curly_test_5_code.av" "curly test 5 - code blocks"

# Escape hatch - literal brace handling
test_output_contains "examples/escape_hatch.av" "One brace escape: {" "escape hatch single brace literal"
test_output_contains "examples/escape_hatch.av" "Three literal opens: {{{" "escape hatch triple brace"
test_output_contains "examples/escape_hatch.av" "Computed in double braces: 30" "escape hatch interpolation"

# Neovim/Vim configuration examples
test_no_error "examples/vim_simple.av" "vim config generation"
test_output_contains "examples/vim_simple.av" "set number" "vim set number"
test_output_contains "examples/vim_simple.av" ".vimrc" "vim config path"

test_no_error "examples/neovim_simple.av" "neovim config generation"
test_output_contains "examples/neovim_simple.av" "set number" "neovim set number"
test_output_contains "examples/neovim_simple.av" "init.vim" "neovim config path"

test_no_error "examples/neovim_config_gen.av" "neovim advanced config"
test_no_error "examples/neovim_lua_simple.av" "neovim lua config"
test_output_contains "examples/neovim_lua_simple.av" "lua" "neovim lua syntax"

test_no_error "examples/vim_plugins.av" "vim plugin config"
test_no_error "examples/emacs_init.av" "emacs config generation"
test_output_contains "examples/emacs_init.av" "require 'package" "emacs package require"

# Site and HTML generation
test_no_error "examples/site_generator.av" "static site generation"
test_no_error "examples/html_page_gen.av" "html page generation"
test_output_contains "examples/html_page_gen.av" "<html>" "html tag"
test_output_contains "examples/html_page_gen.av" "<head>" "head tag"

test_no_error "examples/markdown_readme_gen.av" "markdown readme generation"
test_output_contains "examples/markdown_readme_gen.av" "# " "markdown heading syntax"

# Path value examples
test_no_error "examples/path_simple_test.av" "path value simple test"
test_no_error "examples/path_value_demo.av" "path value demo"
test_no_error "examples/path_comprehensive.av" "path comprehensive test"
test_no_error "examples/fill_with_path.av" "fill template with path"
test_no_error "examples/path_fill_template.av" "path fill template"
test_no_error "examples/path_interpolation_test.av" "path interpolation"

# Template filling
test_no_error "examples/fill_template_demo.av" "fill template demo"
test_no_error "examples/pattern_template_fill.av" "pattern template fill"
test_no_error "examples/pattern_read_append.av" "pattern read append"

# Function features
test_no_error "examples/function_defaults.av" "function with defaults"
test_no_error "examples/named_args.av" "named arguments"

# Let bindings
test_no_error "examples/nested_let.av" "nested let bindings"

test_no_error "examples/let_cascade.av" "cascading let bindings"

# Date/Time operations
test_no_error "examples/date_time_demo.av" "date/time demo"
test_output_contains "examples/date_time_demo.av" "Date/Time Functions Demo" "date/time demo header"
test_output_contains "examples/date_time_demo.av" "Current date/time (ISO 8601):" "date/time current"
test_output_contains "examples/date_time_demo.av" "Formatted outputs:" "date/time formatting"
test_output_contains "examples/date_time_demo.av" "Future dates:" "date/time future dates"
test_output_contains "examples/date_time_demo.av" "Unix timestamp:" "date/time timestamp"
test_output_contains "examples/date_time_demo.av" "Timezone:" "date/time timezone"
test_output_contains "examples/date_time_demo.av" "Time difference:" "date/time diff"
test_output_contains "examples/date_time_demo.av" "86400 seconds" "date/time 1 day in seconds"

test_no_error "examples/backup_config_with_timestamp.av" "backup config with timestamp"
test_output_contains "examples/backup_config_with_timestamp.av" "Backup Configuration" "backup config header"
test_output_contains "examples/backup_config_with_timestamp.av" "application:" "backup config yaml structure"
test_output_contains "examples/backup_config_with_timestamp.av" "backup:" "backup config backup section"
test_output_contains "examples/backup_config_with_timestamp.av" "retention_days: 7" "backup config retention"
test_output_contains "examples/backup_config_with_timestamp.av" "created_at:" "backup config created timestamp"

echo ""
echo "================================================"
echo "Summary:"
echo "  Passed: ${#PASSED[@]}"
echo "  Failed: ${#FAILED[@]}"
echo ""

if [ ${#FAILED[@]} -gt 0 ]; then
    echo "❌ Failed tests:"
    for f in "${FAILED[@]}"; do
        echo "  - $f"
    done
    exit 1
else
    echo "🎉 All output tests passed! ✓"
    exit 0
fi
