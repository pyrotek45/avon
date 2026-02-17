#!/bin/bash
# Advanced Builtin Function Tests
# Covers builtins that were previously untested:
#   List/Functional: char_at, chars, choice, find_index, flatmap, fold,
#     group_by, intersperse, partition, sample, shuffle, slice, sort_by,
#     split_at, unzip, zip_with
#   Type Checking: is_alphanumeric, is_float, is_int, is_lowercase,
#     is_uppercase, is_whitespace
#   Formatting: format_bool, format_list, format_percent, format_scientific,
#     format_toml, format_html, format_opml, html_attr, md_list, neg, to_char
#   Debug/Trace: trace, debug, spy, tap, error
#   Dict: dict_merge
#   Data Parsers: json_parse_string, yaml_parse_string, toml_parse_string,
#     csv_parse_string, ini_parse_string, xml_parse_string, html_parse_string,
#     opml_parse_string, json_parse, yaml_parse, toml_parse
#   Other: fill_template, walkdir

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

# Eval helper for pipes and multi-line code
run_test_eval() {
    local name="$1"
    local code="$2"
    local expected="$3"
    local file="/tmp/avon_adv_test_$$.av"

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

run_test_eval_contains() {
    local name="$1"
    local code="$2"
    local expected="$3"
    local file="/tmp/avon_adv_test_$$.av"

    echo "$code" > "$file"
    result=$($AVON eval "$file" 2>&1) || true
    rm -f "$file"
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

# Test that a command exits with non-zero
run_test_error() {
    local name="$1"
    local expr="$2"
    local expected_substr="$3"

    result=$($AVON run "$expr" 2>&1)
    exit_code=$?
    if [ $exit_code -ne 0 ] && echo "$result" | grep -qiF "$expected_substr"; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected error containing: $expected_substr"
        echo "  Got (exit=$exit_code): $result"
        ((FAILED++))
    fi
}

# Stderr helper: test that the return value (stdout) matches expected
# while ignoring trace/debug output on stderr
run_test_eval_stdout() {
    local name="$1"
    local code="$2"
    local expected="$3"
    local file="/tmp/avon_adv_test_$$.av"

    echo "$code" > "$file"
    result=$($AVON eval "$file" 2>/dev/null) || true
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

echo "Testing Advanced Builtins..."
echo "============================="
echo ""

# ══════════════════════════════════════════════════════════
# List / Functional Builtins
# ══════════════════════════════════════════════════════════

echo "--- char_at ---"
run_test "char_at first"           'char_at "hello" 0'       "h"
run_test "char_at middle"          'char_at "hello" 2'       "l"
run_test "char_at last"            'char_at "hello" 4'       "o"
run_test "char_at second"          'char_at "abcdef" 1'      "b"

echo ""
echo "--- chars ---"
run_test "chars basic"             'chars "abc"'             "[a, b, c]"
run_test "chars single"            'chars "x"'               "[x]"
run_test "chars empty"             'chars ""'                "[]"
run_test_eval "chars length"       'chars "hello" -> length' "5"

echo ""
echo "--- choice ---"
# choice picks one random element; test that the return type is correct
run_test "choice number type"      'is_number (choice [1, 2, 3])'     "true"
run_test "choice string type"      'is_string (choice ["a", "b"])'    "true"
run_test "choice bool type"        'is_bool (choice [true, false])'   "true"

echo ""
echo "--- find_index ---"
run_test_eval "find_index found"   'find_index (\x x > 3) [1, 2, 3, 4, 5]'   "3"
run_test_eval "find_index first"   'find_index (\x x > 0) [1, 2, 3]'          "0"
run_test_eval "find_index last"    'find_index (\x x > 4) [1, 2, 3, 4, 5]'    "4"

echo ""
echo "--- flatmap ---"
run_test_eval "flatmap duplicate"  'flatmap (\x [x, x * 2]) [1, 2, 3]'       "[1, 2, 2, 4, 3, 6]"
run_test_eval "flatmap identity"   'flatmap (\x [x]) [1, 2, 3]'              "[1, 2, 3]"
run_test_eval "flatmap empty"      'flatmap (\x []) [1, 2, 3]'               "[]"
run_test_eval "flatmap sum"        'flatmap (\x [x, x * 2]) [1, 2, 3] -> sum' "18"

echo ""
echo "--- fold ---"
run_test_eval "fold sum"           'fold (\acc \x acc + x) 0 [1, 2, 3, 4, 5]' "15"
run_test_eval "fold product"       'fold (\acc \x acc * x) 1 [1, 2, 3, 4]'    "24"
run_test_eval "fold string"        'fold (\acc \x concat acc (to_string x)) "" [1, 2, 3]' "123"
run_test_eval "fold empty"         'fold (\acc \x acc + x) 42 []'             "42"

echo ""
echo "--- group_by ---"
run_test_eval "group_by parity keys"  'group_by (\x x % 2) [1, 2, 3, 4, 5] -> keys -> sort'  "[0, 1]"
run_test_eval "group_by parity type"  'typeof (group_by (\x x % 2) [1, 2, 3, 4, 5])'          "Dict"
run_test_eval "group_by evens"        'get (group_by (\x x % 2) [1, 2, 3, 4, 5]) "0"'         "[2, 4]"
run_test_eval "group_by odds"         'get (group_by (\x x % 2) [1, 2, 3, 4, 5]) "1"'          "[1, 3, 5]"

echo ""
echo "--- intersperse ---"
run_test "intersperse numbers"     'intersperse 0 [1, 2, 3]'          "[1, 0, 2, 0, 3]"
run_test_eval "intersperse strings" 'intersperse "-" ["a", "b", "c"]' "[a, -, b, -, c]"
run_test "intersperse single"      'intersperse 0 [1]'                "[1]"
run_test "intersperse empty"       'intersperse 0 []'                 "[]"

echo ""
echo "--- partition ---"
run_test_eval "partition basic head"   'partition (\x x > 3) [1, 2, 3, 4, 5] -> head'    "[4, 5]"
run_test_eval "partition basic tail"   'partition (\x x > 3) [1, 2, 3, 4, 5] -> nth 1'   "[1, 2, 3]"
run_test_eval "partition all pass"     'partition (\x x > 0) [1, 2, 3] -> head'           "[1, 2, 3]"
run_test_eval "partition none pass"    'partition (\x x > 10) [1, 2, 3] -> head'          "[]"
run_test_eval "partition length"       'partition (\x x > 3) [1, 2, 3, 4, 5] -> length'  "2"

echo ""
echo "--- sample ---"
# sample takes a count and returns a list of that many random elements
run_test "sample type"             'typeof (sample 2 [1, 2, 3])'    "List"
run_test_eval "sample length"      'sample 2 [1, 2, 3, 4, 5] -> length'  "2"
run_test_eval "sample zero"        'sample 0 [1, 2, 3] -> length'        "0"
run_test_eval "sample all"         'sample 3 [1, 2, 3] -> length'        "3"

echo ""
echo "--- shuffle ---"
run_test_eval "shuffle preserves length"  'length (shuffle [1, 2, 3, 4, 5])'  "5"
run_test_eval "shuffle type"              'typeof (shuffle [1, 2, 3])'         "List"
run_test_eval "shuffle empty"             'length (shuffle [])'                "0"

echo ""
echo "--- slice ---"
run_test "slice list"              'slice [10, 20, 30, 40, 50] 1 3'     "[20, 30]"
run_test "slice from start"        'slice [10, 20, 30, 40, 50] 0 2'     "[10, 20]"
run_test "slice to end"            'slice [10, 20, 30, 40, 50] 3 5'     "[40, 50]"
run_test "slice single"            'slice [10, 20, 30] 1 2'             "[20]"

echo ""
echo "--- sort_by ---"
run_test_eval "sort_by length"     'sort_by (\x length x) ["hi", "hello", "a"]'   "[a, hi, hello]"
run_test_eval "sort_by numeric"    'sort_by (\x neg x) [3, 1, 2]'                 "[3, 2, 1]"
run_test_eval "sort_by already"    'sort_by (\x x) [1, 2, 3]'                     "[1, 2, 3]"

echo ""
echo "--- split_at ---"
run_test "split_at basic"          'split_at 2 [1, 2, 3, 4, 5]'   "[[1, 2], [3, 4, 5]]"
run_test "split_at zero"           'split_at 0 [1, 2, 3]'          "[[], [1, 2, 3]]"
run_test "split_at all"            'split_at 3 [1, 2, 3]'          "[[1, 2, 3], []]"

echo ""
echo "--- unzip ---"
run_test "unzip basic"             'unzip [[1, "a"], [2, "b"]]'          '[[1, 2], [a, b]]'
run_test "unzip single"            'unzip [[1, "x"]]'                     '[[1], [x]]'
run_test_eval "unzip first"        'head (unzip [[10, 20], [30, 40]])'   "[10, 30]"

echo ""
echo "--- zip_with ---"
run_test_eval "zip_with add"       'zip_with (\x \y x + y) [1, 2, 3] [10, 20, 30]'  "[11, 22, 33]"
run_test_eval "zip_with mul"       'zip_with (\x \y x * y) [2, 3] [4, 5]'            "[8, 15]"
run_test_eval "zip_with concat"    'zip_with (\x \y concat x y) ["a", "b"] ["1", "2"]' "[a1, b2]"

# ══════════════════════════════════════════════════════════
# Type Checking Builtins
# ══════════════════════════════════════════════════════════

echo ""
echo "--- Type Checking (extended) ---"
run_test "is_alphanumeric true"    'is_alphanumeric "abc123"'     "true"
run_test "is_alphanumeric false"   'is_alphanumeric "abc 123"'    "false"
run_test "is_alphanumeric empty"   'is_alphanumeric ""'           "false"
run_test "is_lowercase true"       'is_lowercase "hello"'         "true"
run_test "is_lowercase false"      'is_lowercase "Hello"'         "false"
run_test "is_uppercase true"       'is_uppercase "HELLO"'         "true"
run_test "is_uppercase false"      'is_uppercase "Hello"'         "false"
run_test "is_whitespace true"      'is_whitespace "   "'          "true"
run_test "is_whitespace false"     'is_whitespace " a "'          "false"
run_test "is_float true"           'is_float 3.14'                "true"
run_test "is_float false"          'is_float 42'                  "false"
run_test "is_int true"             'is_int 42'                    "true"
run_test "is_int false"            'is_int 3.14'                  "false"

# ══════════════════════════════════════════════════════════
# Formatting Builtins
# ══════════════════════════════════════════════════════════

echo ""
echo "--- Formatting (extended) ---"
run_test "format_bool true yes/no"    'format_bool true "yes/no"'       "Yes"
run_test "format_bool false yes/no"   'format_bool false "yes/no"'      "No"
run_test "format_list comma"          'format_list [1, 2, 3] ", "'      "1, 2, 3"
run_test "format_list dash"           'format_list ["a", "b"] " - "'    "a - b"
run_test "format_list single"         'format_list [42] ", "'           "42"
run_test "format_percent basic"       'format_percent 0.75 2'           "75.00%"
run_test "format_percent zero"        'format_percent 0.0 1'            "0.0%"
run_test "format_percent whole"       'format_percent 1.0 0'            "100%"
run_test "format_scientific basic"    'format_scientific 12345.678 2'   "1.23e4"
run_test "format_scientific small"    'format_scientific 0.001 2'       "1.00e-3"
run_test_contains "format_toml kv"    'format_toml {key: "value"}'     'key = "value"'
run_test "format_toml type"           'typeof (format_toml {key: "value"})' "String"
run_test "format_html type"           'typeof (format_html {html: {body: "text"}})'  "String"
run_test "format_opml type"           'typeof (format_opml {head: {title: "test"}})'  "String"
run_test "html_attr class"            'html_attr "class" "bold"'        'class="bold"'
run_test "html_attr id"               'html_attr "id" "main"'           'id="main"'
run_test "html_attr data"             'html_attr "data-x" "42"'         'data-x="42"'
run_test "neg positive"               'neg 42'                          "-42"
run_test "neg negative"               'neg (-5)'                        "5"
run_test "neg zero"                    'neg 0'                           "0"
run_test "to_char uppercase A"        'to_char 65'                      "A"
run_test "to_char lowercase a"        'to_char 97'                      "a"
run_test "to_char zero"               'to_char 48'                      "0"

echo ""
echo "--- md_list ---"
run_test "md_list basic"              'md_list ["item1", "item2", "item3"]' "- item1
- item2
- item3"
run_test "md_list single"             'md_list ["only"]'                    "- only"

# ══════════════════════════════════════════════════════════
# Debug / Trace Builtins
# ══════════════════════════════════════════════════════════

echo ""
echo "--- Debug & Trace ---"
# trace prints to stderr but returns the value unchanged
run_test_eval_stdout "trace returns value"    'trace "label" 42'               "42"
run_test_eval_stdout "trace returns string"   'trace "msg" "hello"'            "hello"
run_test_eval_stdout "trace returns list"     'trace "data" [1, 2, 3]'         "[1, 2, 3]"
# debug prints to stderr but returns the value unchanged
run_test_eval_stdout "debug returns value"    'debug "lbl" 42'                 "42"
run_test_eval_stdout "debug returns string"   'debug "lbl" "test"'             "test"
# spy prints to stderr but returns the value unchanged
run_test_eval_stdout "spy returns value"       'spy 42'                         "42"
run_test_eval_stdout "spy returns string"      'spy "hello"'                    "hello"
# trace output goes to stderr
run_test_eval_contains "trace stderr output"   'trace "result" 42'              "[TRACE] result: 42"
run_test_eval_contains "debug stderr output"   'debug "lbl" 42'                 "[DEBUG] lbl:"
run_test_eval_contains "spy stderr output"     'spy 42'                          "[SPY:"

echo ""
echo "--- error ---"
# error should exit with non-zero and include the message
run_test_error "error exits non-zero"   'error "custom failure"'   "custom failure"
run_test_error "error with number msg"  'error "code 404"'         "code 404"

# ══════════════════════════════════════════════════════════
# Dict Builtins
# ══════════════════════════════════════════════════════════

echo ""
echo "--- Dict (extended) ---"
run_test_eval "dict_merge basic keys"  'dict_merge {a: 1} {b: 2} -> keys -> sort' "[a, b]"
run_test_eval "dict_merge basic get a" 'get (dict_merge {a: 1} {b: 2}) "a"'       "1"
run_test_eval "dict_merge basic get b" 'get (dict_merge {a: 1} {b: 2}) "b"'       "2"
# Dict key ordering is nondeterministic, so check via keys and value access
run_test_eval "dict_merge override val" 'get (dict_merge {a: 1, b: 2} {b: 99, c: 3}) "b"' "99"
run_test_eval "dict_merge override keys" 'dict_merge {a: 1, b: 2} {b: 99, c: 3} -> keys -> sort' "[a, b, c]"
run_test_eval "dict_merge empty left keys"  'dict_merge {} {a: 1} -> keys'         "[a]"
run_test_eval "dict_merge empty right keys" 'dict_merge {a: 1} {} -> keys'         "[a]"
run_test_eval "dict_merge keys count"  'dict_merge {a: 1, b: 2} {c: 3} -> keys -> length' "3"

# ══════════════════════════════════════════════════════════
# Data Format Parsers
# ══════════════════════════════════════════════════════════

echo ""
echo "--- Data Parsers (string-based) ---"

# json_parse_string
run_test "json_parse_string type"      'typeof (json_parse_string "{}")'         "Dict"
run_test_contains "json_parse_string"  'json_parse_string "{\"a\": 1}"'          "a: 1"

# yaml_parse_string
run_test "yaml_parse_string type"      'typeof (yaml_parse_string "name: test")'  "Dict"
run_test_contains "yaml_parse_string"  'yaml_parse_string "name: test"'           "name"

# toml_parse_string
run_test "toml_parse_string type"      'typeof (toml_parse_string "key = \"val\"")'  "Dict"

# csv_parse_string
run_test "csv_parse_string type"       'typeof (csv_parse_string "a,b\n1,2")'   "List"
run_test_eval "csv_parse_string len"   'csv_parse_string "a,b\n1,2\n3,4" -> length' "2"

# ini_parse_string
run_test "ini_parse_string type"       'typeof (ini_parse_string "[s]\nk=v")'    "Dict"

# xml_parse_string
run_test "xml_parse_string type"       'typeof (xml_parse_string "<root><c>t</c></root>")'  "Dict"

# html_parse_string
run_test "html_parse_string type"      'typeof (html_parse_string "<html><body>t</body></html>")'  "Dict"

# opml_parse_string
run_test "opml_parse_string type"      'typeof (opml_parse_string "<opml><head><title>t</title></head></opml>")'  "Dict"

echo ""
echo "--- Data Parsers (file-based) ---"
# Create temporary data files
PARSE_DIR="/tmp/avon_parse_test_$$"
mkdir -p "$PARSE_DIR"

echo '{"name": "test", "value": 42}' > "$PARSE_DIR/test.json"
printf 'name: test\nvalue: 42\n' > "$PARSE_DIR/test.yaml"
printf 'key = "value"\n' > "$PARSE_DIR/test.toml"
printf 'a,b\n1,2\n3,4\n' > "$PARSE_DIR/test.csv"
printf '[section]\nkey = value\n' > "$PARSE_DIR/test.ini"

run_test_eval "json_parse file type"    "typeof (json_parse \"$PARSE_DIR/test.json\")"   "Dict"
run_test_eval "json_parse file get"     "get (json_parse \"$PARSE_DIR/test.json\") \"name\"" "test"
run_test_eval "yaml_parse file type"    "typeof (yaml_parse \"$PARSE_DIR/test.yaml\")"   "Dict"
run_test_eval "yaml_parse file get"     "get (yaml_parse \"$PARSE_DIR/test.yaml\") \"name\"" "test"
run_test_eval "toml_parse file type"    "typeof (toml_parse \"$PARSE_DIR/test.toml\")"   "Dict"
run_test_eval "csv_parse file type"     "typeof (csv_parse \"$PARSE_DIR/test.csv\")"     "List"
run_test_eval "csv_parse file len"      "csv_parse \"$PARSE_DIR/test.csv\" -> length"    "2"
run_test_eval "ini_parse file type"     "typeof (ini_parse \"$PARSE_DIR/test.ini\")"     "Dict"

rm -rf "$PARSE_DIR"

# ══════════════════════════════════════════════════════════
# fill_template & walkdir
# ══════════════════════════════════════════════════════════

echo ""
echo "--- fill_template ---"
TMPL_DIR="/tmp/avon_tmpl_test_$$"
mkdir -p "$TMPL_DIR"

echo 'Hello {name}! Age: {age}.' > "$TMPL_DIR/greeting.txt"
run_test_eval "fill_template basic" \
    "fill_template \"$TMPL_DIR/greeting.txt\" {name: \"World\", age: 30}" \
    "Hello World! Age: 30."

echo '{title} by {author}' > "$TMPL_DIR/book.txt"
run_test_eval "fill_template book" \
    "fill_template \"$TMPL_DIR/book.txt\" {title: \"Avon\", author: \"PyroTek\"}" \
    "Avon by PyroTek"

rm -rf "$TMPL_DIR"

echo ""
echo "--- walkdir ---"
WALK_DIR="/tmp/avon_walk_test_$$"
mkdir -p "$WALK_DIR/sub"
echo "a" > "$WALK_DIR/file1.txt"
echo "b" > "$WALK_DIR/sub/file2.txt"

run_test_eval "walkdir type"         "typeof (walkdir \"$WALK_DIR\")"       "List"
# walkdir lists files AND directories recursively
run_test_eval "walkdir finds all"    "walkdir \"$WALK_DIR\" -> length"      "3"

rm -rf "$WALK_DIR"

# ══════════════════════════════════════════════════════════
# Pipe Chains & Currying Patterns
# ══════════════════════════════════════════════════════════

echo ""
echo "--- Pipe Chains ---"
run_test_eval "pipe: filter then sum"   '[1, 2, 3, 4, 5] -> filter (\x x > 2) -> sum'    "12"
run_test_eval "pipe: map then sort"     '[3, 1, 2] -> map (\x x * 10) -> sort'            "[10, 20, 30]"
run_test_eval "pipe: flatmap then len"  '[1, 2, 3] -> flatmap (\x [x, x]) -> length'      "6"
run_test_eval "pipe: chars then rev"    'reverse (chars "abc") -> (\lst join lst "")'   "cba"
run_test_eval "pipe: split then map"    'split "1,2,3" "," -> map to_int -> sum'           "6"
run_test_eval "pipe: range then fold"   'range 1 5 -> fold (\acc \x acc + x) 0'            "15"
run_test_eval "pipe: zip then map"      'zip [1, 2] [10, 20] -> map head -> sum'           "3"
run_test_eval "pipe: group then keys"   'group_by (\x x % 3) [1, 2, 3, 4, 5, 6] -> keys -> sort' "[0, 1, 2]"
run_test_eval "pipe: partition head"    '[1, 2, 3, 4, 5] -> partition (\x x > 3) -> head -> sum'  "9"
run_test_eval "pipe: chain 5 ops"       'range 1 10 -> filter (\x x % 2 == 0) -> map (\x x * x) -> sort -> sum' "220"

echo ""
echo "--- Currying Patterns ---"
run_test_eval "curried map"        'let double = map (\x x * 2) in
double [1, 2, 3]'                  "[2, 4, 6]"
run_test_eval "curried filter"     'let positives = filter (\x x > 0) in
positives [(-1), 0, 1, 2]'        "[1, 2]"
run_test_eval "curried fold"       'let sum_list = fold (\acc \x acc + x) 0 in
sum_list [10, 20, 30]'            "60"

# ══════════════════════════════════════════════════════════
# Summary
# ══════════════════════════════════════════════════════════

echo ""
echo "============================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✓ All advanced builtin tests passed!"
    exit 0
else
    echo "✗ Some advanced builtin tests failed"
    exit 1
fi
