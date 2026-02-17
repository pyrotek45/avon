#!/bin/bash
# Builtin Function Integration Tests
# Tests DateTime, Regex, File I/O, Formatting, Aggregate, Type Conversion,
# String, List, Dict, HTML/Markdown helper, and system functions.
# Covers gaps identified in TEST_COVERAGE_REPORT.md

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

PASSED=0
FAILED=0

run_test() {
    local name="$1"
    local expr="$2"
    local expected="$3"

    result=$($AVON run "$expr" 2>&1) || true
    if [ "$result" = "$expected" ]; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected: $expected"
        echo "  Got:      $result"
        ((FAILED++))
    fi
}

run_test_contains() {
    local name="$1"
    local expr="$2"
    local expected="$3"

    result=$($AVON run "$expr" 2>&1) || true
    if echo "$result" | grep -qF "$expected"; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected substring: $expected"
        echo "  Got: $result"
        ((FAILED++))
    fi
}

# Eval helper for pipe/dot syntax that only works in .av files
run_test_eval() {
    local name="$1"
    local code="$2"
    local expected="$3"
    local file="/tmp/avon_builtin_test_$$.av"

    echo "$code" > "$file"
    result=$($AVON eval "$file" 2>&1) || true
    rm -f "$file"
    if [ "$result" = "$expected" ]; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected: $expected"
        echo "  Got:      $result"
        ((FAILED++))
    fi
}

echo "Testing Builtin Functions..."
echo "============================="
echo ""

# ── DateTime ──────────────────────────────────────────────
echo "--- DateTime ---"
run_test "now returns String"        'typeof (now)'        "String"
run_test "timestamp returns Number"  'typeof (timestamp)'  "Number"
run_test "timestamp is positive"     'timestamp > 0'       "true"
run_test_contains "now has ISO T"    'now'                 "T"
run_test_contains "date_format %Y"   'date_format (now) "%Y"'  "202"
run_test_contains "date_add 1d"      'date_add (now) "1d"'     "T"
run_test "date_diff type"            'typeof (date_diff (now) (date_add (now) "1d"))' "Number"
run_test "date_diff 1d"              'date_diff (now) (date_add (now) "1d")' "-86400"
run_test "timezone type"             'typeof (timezone)'   "String"
run_test "date_parse type"           'typeof (date_parse "2024-01-15" "%Y-%m-%d")' "String"

# ── Regex ─────────────────────────────────────────────────
echo ""
echo "--- Regex ---"
run_test "regex_match positive"      'regex_match "[0-9]+" "hello123"'       "true"
run_test "regex_match negative"      'regex_match "[0-9]+" "hello"'          "false"
run_test "regex_match anchored ^"    'regex_match "^hello" "hello world"'    "true"
run_test 'regex_match anchored $'    'regex_match "world$" "hello world"'    "true"
run_test "regex_match digits"        'regex_match "^\d+$" "12345"'           "true"
run_test "regex_match digits fail"   'regex_match "^\d+$" "123a5"'           "false"
run_test "regex_replace digits"      'regex_replace "\d" "#" "a1b2c3"'       "a#b#c#"
run_test "regex_replace spaces"      'regex_replace "\s+" " " "hello    world"' "hello world"
run_test "regex_replace vowels"      'regex_replace "[aeiou]" "*" "hello"'   "h*ll*"
run_test_eval "regex_split len"      'regex_split "\s+" "a b  c" -> length'  "3"
run_test_eval "regex_split delims"   'regex_split "[,;]" "a,b;c" -> length'  "3"
run_test_eval "scan finds all"       'scan "\d+" "a12b34c56" -> length'      "3"
run_test_eval "scan first match"     'head (scan "\d+" "a12b34c56")'         "12"

# ── File I/O ──────────────────────────────────────────────
echo ""
echo "--- File I/O ---"
TEST_DIR="/tmp/avon_builtin_fio_$$"
mkdir -p "$TEST_DIR"
echo -n "test content" > "$TEST_DIR/read_test.txt"
printf "line1\nline2\nline3\n" > "$TEST_DIR/lines_test.txt"

run_test "readfile content"          "readfile \"$TEST_DIR/read_test.txt\""   "test content"
run_test "readfile type"             "typeof (readfile \"$TEST_DIR/read_test.txt\")" "String"
run_test_eval "readlines count"      "readlines \"$TEST_DIR/lines_test.txt\" -> length" "3"
run_test "readlines type"            "typeof (readlines \"$TEST_DIR/lines_test.txt\")" "List"
run_test "exists true"               "exists \"$TEST_DIR/read_test.txt\""    "true"
run_test "exists false"              "exists \"$TEST_DIR/nope.txt\""         "false"
run_test "basename"                  "basename \"$TEST_DIR/read_test.txt\""  "read_test.txt"
run_test "dirname"                   "dirname \"$TEST_DIR/read_test.txt\""   "$TEST_DIR"
rm -rf "$TEST_DIR"

# ── Formatting ────────────────────────────────────────────
echo ""
echo "--- Formatting ---"
run_test "format_bytes B"            'format_bytes 512'          "512 B"
run_test "format_bytes KB"           'format_bytes 1024'         "1.00 KB"
run_test "format_bytes MB"           'format_bytes 1048576'      "1.00 MB"
run_test "format_bytes GB"           'format_bytes 1073741824'   "1.00 GB"
run_test "format_hex 255"            'format_hex 255'            "ff"
run_test "format_hex 0"              'format_hex 0'              "0"
run_test "format_hex 16"             'format_hex 16'             "10"
run_test "format_binary 10"          'format_binary 10'          "1010"
run_test "format_octal 8"            'format_octal 8'            "10"
run_test "format_float precision"    'format_float 3.14159 2'    "3.14"
run_test "format_int zero-pad"       'format_int 42 5'           "00042"
run_test_contains "format_currency"  'format_currency 1234.56 "$"'  "1234"
run_test_contains "format_json"      'format_json {a: 1}'        '"a"'
run_test_contains "format_yaml"      'format_yaml {name: "test"}'  "name"
run_test_contains "format_xml"       'format_xml {root: {child: "text"}}' "root"
run_test_contains "format_ini"       'format_ini {section: {key: "value"}}' "key"
run_test_contains "format_csv"       'format_csv [[1, 2], [3, 4]]' "1"
run_test_contains "format_table"     'format_table [["Name", "Age"], ["Alice", "30"]] " | "' "Name"
run_test "truncate long"             'truncate "hello world this is long" 10' "hello w..."
run_test "center text"               'center "hi" 6'            "  hi  "
run_test "indent text"               'indent "hello" 2'         "  hello"

# ── Aggregates ────────────────────────────────────────────
echo ""
echo "--- Aggregates ---"
run_test "sum ints"           'sum [1, 2, 3, 4, 5]'           "15"
run_test "sum floats"         'sum [1.5, 2.5, 3.0]'           "7"
run_test "sum empty"          'sum []'                          "0"
run_test "product ints"       'product [1, 2, 3, 4]'          "24"
run_test "product empty"      'product []'                      "1"
run_test "min numbers"        'min [3, 1, 4, 1, 5]'           "1"
run_test "min strings"        'min ["zebra", "apple", "banana"]' "apple"
run_test "min empty"          'min []'                          "None"
run_test "max numbers"        'max [3, 1, 4, 1, 5]'           "5"
run_test "max strings"        'max ["zebra", "apple", "banana"]' "zebra"
run_test "max empty"          'max []'                          "None"
run_test "all true"           'all (\x x > 0) [1, 2, 3]'      "true"
run_test "all false"          'all (\x x > 0) [1, -2, 3]'     "false"
run_test "all empty"          'all (\x x > 0) []'              "true"
run_test "any true"           'any (\x x < 0) [1, 2, -3]'     "true"
run_test "any false"          'any (\x x < 0) [1, 2, 3]'      "false"
run_test "any empty"          'any (\x x < 0) []'              "false"
run_test "count match"        'count (\x x > 5) [1, 6, 3, 8, 5]' "2"
run_test "count none"         'count (\x x > 10) [1, 2, 3]'   "0"

# ── List Functions ────────────────────────────────────────
echo ""
echo "--- Lists ---"
run_test "head"               'head [10, 20, 30]'              "10"
run_test "head empty"         'head []'                         "None"
run_test "tail"               'tail [10, 20, 30]'              "[20, 30]"
run_test "nth valid"          'nth 1 [10, 20, 30]'             "20"
run_test "nth oob"            'nth 10 [10, 20, 30]'            "None"
run_test "reverse"            'reverse [1, 2, 3]'              "[3, 2, 1]"
run_test "sort numbers"       'sort [3, 1, 4, 1, 5]'          "[1, 1, 3, 4, 5]"
run_test "sort strings"       'sort ["banana", "apple", "cherry"]' "[apple, banana, cherry]"
run_test "unique"             'unique [1, 2, 2, 3, 3, 3]'     "[1, 2, 3]"
run_test "range inclusive"    'range 1 5'                       "[1, 2, 3, 4, 5]"
run_test "zip"                'zip [1, 2] ["a", "b"]'          "[[1, a], [2, b]]"
run_test_eval "enumerate len" 'enumerate ["a", "b", "c"] -> length'  "3"
run_test_eval "flatten"       'flatten [[1, 2], [3, 4]] -> length'   "4"
run_test_eval "take"          'take 2 [1, 2, 3, 4] -> length'       "2"
run_test_eval "drop"          'drop 2 [1, 2, 3, 4] -> length'       "2"
run_test_eval "filter"        'filter (\x x > 2) [1, 2, 3, 4, 5] -> length' "3"
run_test_eval "map"           'map (\x x * 2) [1, 2, 3] -> sum'     "12"
run_test_eval "find"          'find (\x x > 3) [1, 2, 3, 4, 5]'     "4"
run_test "find none"          'find (\x x > 10) [1, 2, 3]'    "None"

# ── String Functions ──────────────────────────────────────
echo ""
echo "--- Strings ---"
run_test "upper"              'upper "hello"'                   "HELLO"
run_test "lower"              'lower "HELLO"'                   "hello"
run_test "trim"               'trim "  hello  "'               "hello"
run_test "split"              'split "a,b,c" ","'              "[a, b, c]"
run_test "join"               'join ["a", "b", "c"] ", "'      "a, b, c"
run_test "contains"           'contains "hello world" "world"' "true"
run_test "starts_with"        'starts_with "hello" "hel"'      "true"
run_test "ends_with"          'ends_with "hello" "llo"'        "true"
run_test "replace"            'replace "hello world" "world" "avon"' "hello avon"
run_test "repeat"             'repeat "ab" 3'                  "ababab"
run_test "length string"      'length "hello"'                 "5"
run_test "pad_left"           'pad_left "42" 5 "0"'            "00042"
run_test "pad_right"          'pad_right "hi" 5 "."'           "hi..."
run_test "is_digit true"      'is_digit "12345"'               "true"
run_test "is_digit false"     'is_digit "12a45"'               "false"
run_test "is_alpha true"      'is_alpha "hello"'               "true"
run_test "is_alpha false"     'is_alpha "hello1"'              "false"
run_test "is_empty string"    'is_empty ""'                    "true"
run_test "is_empty list"      'is_empty []'                    "true"
run_test "concat"             'concat "hello" " world"'        "hello world"

# ── Dict Functions ────────────────────────────────────────
echo ""
echo "--- Dicts ---"
run_test 'get existing'       'get {a: 1, b: 2} "b"'          "2"
run_test 'get missing'        'get {a: 1} "b"'                 "None"
run_test 'has_key true'       'has_key {a: 1} "a"'             "true"
run_test 'has_key false'      'has_key {a: 1} "b"'             "false"
run_test 'set adds key'       'has_key (set {a: 1} "b" 2) "b"' "true"
run_test_eval "keys count"    'keys {a: 1, b: 2} -> length'    "2"
run_test_eval "values count"  'values {a: 1, b: 2} -> length'  "2"

# ── Environment & System ─────────────────────────────────
echo ""
echo "--- Environment ---"
run_test "env_var_or default" 'env_var_or "NONEXISTENT_VAR_12345" "fallback"'  "fallback"
run_test "env_var_or type"    'typeof (env_var_or "PATH" "default")'           "String"
run_test "os type"            'typeof (os)'                    "String"
run_test "os value"           'os'                             "linux"

# ── Type Conversion ──────────────────────────────────────
echo ""
echo "--- Type Conversion ---"
run_test "to_string number"   'to_string 42'                   "42"
run_test "to_string bool"     'to_string true'                 "true"
run_test "to_int"             'to_int "42"'                    "42"
run_test "to_float"           'to_float "3.14"'                "3.14"
run_test "to_bool true"       'to_bool "true"'                 "true"
run_test "to_bool false"      'to_bool "false"'                "false"
run_test "to_list"            'to_list "abc"'                  "[a, b, c]"

# ── Type Checking ────────────────────────────────────────
echo ""
echo "--- Type Checking ---"
run_test "typeof String"      'typeof "hello"'                 "String"
run_test "typeof Number"      'typeof 42'                      "Number"
run_test "typeof Bool"        'typeof true'                    "Bool"
run_test "typeof List"        'typeof [1, 2]'                  "List"
run_test "typeof Dict"        'typeof {a: 1}'                  "Dict"
run_test "typeof None"        'typeof none'                    "None"
run_test "is_string"          'is_string "hello"'              "true"
run_test "is_number"          'is_number 42'                   "true"
run_test "is_bool"            'is_bool true'                   "true"
run_test "is_list"            'is_list [1]'                    "true"
run_test "is_function"        'is_function (\x x)'            "true"
run_test "is_none"            'is_none none'                   "true"

# ── HTML / Markdown ───────────────────────────────────────
echo ""
echo "--- HTML & Markdown ---"
run_test_contains "markdown_to_html" 'markdown_to_html "# Hello"'  "<h1>"
run_test_contains "html_tag"         'html_tag "p" "text"'          "<p>"
run_test "html_escape"               'html_escape "<b>test</b>"'    "&lt;b&gt;test&lt;/b&gt;"
run_test_contains "md_heading"       'md_heading 2 "Title"'         "## Title"
run_test_contains "md_link"          'md_link "text" "url"'         "[text](url)"
run_test "md_code"                   'md_code "x = 1"'             '`x = 1`'

# ── Default / Assert / Debug ─────────────────────────────
echo ""
echo "--- Default & Assert ---"
run_test "default with none"          'default 42 none'            "42"
run_test "default with value"         'default 42 7'               "7"
run_test "default with head empty"    'default 0 (head [])'        "0"
run_test "assert true"               'assert true 42'              "42"

echo ""
echo "============================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✓ All builtin function tests passed!"
    exit 0
else
    echo "✗ Some builtin function tests failed"
    exit 1
fi
