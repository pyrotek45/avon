#!/bin/bash
# Deep coverage tests addressing gaps from TEST_COVERAGE_REPORT.md
#
# Targets the report's HIGH and MEDIUM priority areas:
#   1. DateTime edge cases (leap years, units, format strings, errors)
#   2. Regex with complex patterns (capture groups, anchors, errors)
#   3. HTML/Markdown builtins (html_tag, html_attr, html_escape, md_*)
#   4. Formatting edge cases (scientific, bool variants, empty input)
#   5. Parser/Lexer edge cases (precedence, nesting, delimiter matching)
#   6. File I/O parsers (xml_parse, html_parse, opml_parse + string variants)
#   7. Error path testing (type mismatches, missing files, invalid input)
#   8. Miscellaneous low-coverage builtins (neg, to_char, tap, enumerate, etc.)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

PASS=0
FAIL=0

assert_eq() {
    local test_name="$1"
    local actual="$2"
    local expected="$3"
    if [ "$actual" = "$expected" ]; then
        ((PASS++))
    else
        echo "FAIL: $test_name"
        echo "  expected: $expected"
        echo "  actual:   $actual"
        ((FAIL++))
    fi
}

assert_contains() {
    local test_name="$1"
    local actual="$2"
    local pattern="$3"
    if echo "$actual" | grep -q "$pattern"; then
        ((PASS++))
    else
        echo "FAIL: $test_name"
        echo "  output:  $actual"
        echo "  expected to contain: $pattern"
        ((FAIL++))
    fi
}

assert_success() {
    local test_name="$1"
    shift
    if "$@" > /dev/null 2>&1; then
        ((PASS++))
    else
        echo "FAIL: $test_name (command failed)"
        ((FAIL++))
    fi
}

assert_failure() {
    local test_name="$1"
    shift
    if "$@" > /dev/null 2>&1; then
        echo "FAIL: $test_name (expected failure but succeeded)"
        ((FAIL++))
    else
        ((PASS++))
    fi
}

# For tests that need .av file syntax
run_av() {
    local code="$1"
    local file="/tmp/avon_deep_test_$$.av"
    echo "$code" > "$file"
    $AVON eval "$file" 2>&1
    local rc=$?
    rm -f "$file"
    return $rc
}

###############################################################################
#  1. DATETIME DEPTH
###############################################################################
echo "── DateTime ──"

# now / timestamp / timezone types
assert_eq "now type" "$($AVON run 'typeof (now)')" "String"
assert_eq "timestamp type" "$($AVON run 'typeof (timestamp)')" "Number"
assert_eq "timezone type" "$($AVON run 'typeof (timezone)')" "String"

# date_parse round-trip
assert_eq "date_parse Y-m-d" "$($AVON run 'date_format (date_parse "2024-06-15" "%Y-%m-%d") "%Y-%m-%d"')" "2024-06-15"

# date_format various format strings
assert_eq "date_format HMS" "$($AVON run 'date_format "2024-06-15T10:30:45+00:00" "%H:%M:%S"')" "10:30:45"
assert_eq "date_format year" "$($AVON run 'date_format "2024-06-15T10:30:45+00:00" "%Y"')" "2024"
assert_eq "date_format month" "$($AVON run 'date_format "2024-06-15T10:30:45+00:00" "%m"')" "06"

# date_add: various units
assert_eq "date_add 1 day" "$($AVON run 'date_add "2024-01-15T00:00:00+00:00" "1 day"')" "2024-01-16T00:00:00+00:00"
assert_eq "date_add 1 week" "$($AVON run 'date_add "2024-01-01T00:00:00+00:00" "1 week"')" "2024-01-08T00:00:00+00:00"
assert_eq "date_add 3 hours" "$($AVON run 'date_add "2024-01-01T12:00:00+00:00" "3 hours"')" "2024-01-01T15:00:00+00:00"
assert_eq "date_add 2 days" "$($AVON run 'date_add "2024-02-28T00:00:00+00:00" "2 days"')" "2024-03-01T00:00:00+00:00"

# date_add: leap year
assert_eq "leap year feb29" "$($AVON run 'date_add "2024-02-28T00:00:00+00:00" "1 day"')" "2024-02-29T00:00:00+00:00"

# date_diff
assert_eq "date_diff 1 day" "$($AVON run 'date_diff "2024-01-01T00:00:00+00:00" "2024-01-02T00:00:00+00:00"')" "-86400"
assert_eq "date_diff same" "$($AVON run 'date_diff "2024-01-01T00:00:00+00:00" "2024-01-01T00:00:00+00:00"')" "0"

# date_parse error
assert_failure "date_parse invalid" $AVON run 'date_parse "not-a-date" "%Y-%m-%d"'

# now returns non-empty string
NOW_VAL=$($AVON run 'now')
assert_contains "now not empty" "$NOW_VAL" "T"

# timestamp is a number
assert_eq "timestamp is_number" "$($AVON run 'is_number (timestamp)')" "true"

###############################################################################
#  2. REGEX DEPTH
###############################################################################
echo "── Regex ──"

# regex_match: basic
assert_eq "regex digits match" "$($AVON run 'regex_match "\\d+" "abc123"')" "true"
assert_eq "regex digits no match" "$($AVON run 'regex_match "\\d+" "abcdef"')" "false"

# regex_match: anchored
assert_eq "regex anchored match" "$($AVON run 'regex_match "^\\d{3}-\\d{4}$" "555-1234"')" "true"
assert_eq "regex anchored no match" "$($AVON run 'regex_match "^\\d{3}-\\d{4}$" "55-1234"')" "false"

# regex_match: character classes
assert_eq "regex word chars" "$($AVON run 'regex_match "^\\w+$" "hello_world"')" "true"
assert_eq "regex word chars fail" "$($AVON run 'regex_match "^\\w+$" "hello world"')" "false"

# regex_split
assert_eq "regex_split spaces" "$($AVON run 'regex_split "\\s+" "hello   world   foo"')" "[hello, world, foo]"
assert_eq "regex_split comma" "$($AVON run 'regex_split ",\\s*" "a, b,c,  d"')" "[a, b, c, d]"

# regex_replace
assert_eq "regex_replace digits" "$($AVON run 'regex_replace "\\d+" "X" "abc123def456"')" "abcXdefX"
assert_eq "regex_replace capture" "$($AVON run 'regex_replace "(\\w+)" "[$1]" "hello world"')" "[hello] [world]"

# scan: find all matches
assert_eq "scan digits" "$($AVON run 'scan "\\d+" "a1b22c333"')" "[1, 22, 333]"
assert_eq "scan words" "$($AVON run 'scan "[a-z]+" "Hello World 123"')" "[ello, orld]"
assert_eq "scan no match" "$($AVON run 'scan "\\d+" "no digits"')" "[]"

# regex error: invalid pattern
assert_failure "regex invalid pattern" $AVON run 'regex_match "[" "test"'
assert_failure "regex_split invalid" $AVON run 'regex_split "[" "test"'

# regex in pipeline
assert_eq "pipe regex_split" "$($AVON run '"a1b2c3" -> scan "\\d+" -> length')" "3"

###############################################################################
#  3. HTML & MARKDOWN BUILTINS
###############################################################################
echo "── HTML & Markdown ──"

# html_escape
assert_eq "html_escape tags" "$($AVON run 'html_escape "<b>bold</b>"')" "&lt;b&gt;bold&lt;/b&gt;"
assert_eq "html_escape amp" "$($AVON run 'html_escape "a & b"')" "a &amp; b"
assert_eq "html_escape quotes" "$($AVON run 'html_escape "say \"hi\""')" 'say &quot;hi&quot;'
assert_eq "html_escape clean" "$($AVON run 'html_escape "clean text"')" "clean text"

# html_tag
assert_eq "html_tag div" "$($AVON run 'html_tag "div" "content"')" "<div>content</div>"
assert_eq "html_tag p" "$($AVON run 'html_tag "p" "paragraph"')" "<p>paragraph</p>"
assert_eq "html_tag span" "$($AVON run 'html_tag "span" "inline"')" "<span>inline</span>"
assert_eq "html_tag empty" "$($AVON run 'html_tag "br" ""')" "<br></br>"

# html_attr
assert_eq "html_attr class" "$($AVON run 'html_attr "class" "main"')" 'class="main"'
assert_eq "html_attr id" "$($AVON run 'html_attr "id" "header"')" 'id="header"'

# md_heading
assert_eq "md_heading h1" "$($AVON run 'md_heading 1 "Title"')" "# Title"
assert_eq "md_heading h2" "$($AVON run 'md_heading 2 "Subtitle"')" "## Subtitle"
assert_eq "md_heading h3" "$($AVON run 'md_heading 3 "Section"')" "### Section"

# md_link
assert_eq "md_link basic" "$($AVON run 'md_link "Click" "https://example.com"')" "[Click](https://example.com)"

# md_code (arity 1: wraps in backticks)
assert_eq "md_code inline" "$($AVON run 'md_code "let x = 42"')" '`let x = 42`'

# md_list
MD_LIST_OUT=$($AVON run 'md_list ["first", "second", "third"]')
assert_contains "md_list has items" "$MD_LIST_OUT" "first"
assert_contains "md_list has second" "$MD_LIST_OUT" "second"
assert_contains "md_list has third" "$MD_LIST_OUT" "third"

# markdown_to_html
assert_contains "md_to_html heading" "$($AVON run 'markdown_to_html "# Hello"')" "<h1>"
assert_contains "md_to_html bold" "$($AVON run 'markdown_to_html "**bold**"')" "<strong>"
assert_contains "md_to_html para" "$($AVON run 'markdown_to_html "text"')" "<p>"

###############################################################################
#  4. FORMATTING EDGE CASES
###############################################################################
echo "── Formatting Edge Cases ──"

# format_scientific
assert_eq "format_sci large" "$($AVON run 'format_scientific 12345.678 3')" "1.235e4"
assert_eq "format_sci small" "$($AVON run 'format_scientific 0.001 2')" "1.00e-3"
assert_eq "format_sci zero" "$($AVON run 'format_scientific 0.0 2')" "0.00e0"

# format_bool variants
assert_eq "format_bool on/off true" "$($AVON run 'format_bool true "on/off"')" "On"
assert_eq "format_bool on/off false" "$($AVON run 'format_bool false "on/off"')" "Off"
assert_eq "format_bool true/false" "$($AVON run 'format_bool false "true/false"')" "False"

# format_float edge cases
assert_eq "format_float 4 dec" "$($AVON run 'format_float 3.14159 4')" "3.1416"
assert_eq "format_float 0 dec" "$($AVON run 'format_float 3.14159 0')" "3"
assert_eq "format_float neg" "$($AVON run 'format_float (-1.5) 1')" "-1.5"

# format_int edge cases
assert_eq "format_int pad3" "$($AVON run 'format_int 7 3')" "007"
assert_eq "format_int pad1" "$($AVON run 'format_int 42 1')" "42"
assert_eq "format_int pad0" "$($AVON run 'format_int 100 0')" "100"

# truncate edge cases
assert_eq "truncate short" "$($AVON run 'truncate "short" 100')" "short"
assert_eq "truncate exact" "$($AVON run 'truncate "hello" 5')" "hello"

# center edge cases
assert_eq "center narrow" "$($AVON run 'center "x" 5')" "  x  "
assert_eq "center wider" "$($AVON run 'center "hi" 6')" "  hi  "
assert_eq "center no pad" "$($AVON run 'center "hello" 3')" "hello"

# format_toml
assert_contains "format_toml kv" "$($AVON run 'format_toml {name: "test", version: "1.0"}')" 'name = "test"'

# format_percent
assert_eq "format_percent 50%" "$($AVON run 'format_percent 0.5 0')" "50%"
assert_eq "format_percent 100%" "$($AVON run 'format_percent 1.0 0')" "100%"

# format_bytes edge
assert_eq "format_bytes 0" "$($AVON run 'format_bytes 0')" "0 B"
assert_eq "format_bytes TB" "$($AVON run 'format_bytes 1099511627776')" "1.00 TB"

###############################################################################
#  5. PARSER & LEXER EDGE CASES
###############################################################################
echo "── Parser & Lexer Edge Cases ──"

# operator precedence
assert_eq "mul before add" "$($AVON run '2 + 3 * 4')" "14"
assert_eq "parens override" "$($AVON run '(2 + 3) * 4')" "20"

# deeply nested parens
assert_eq "deep parens" "$($AVON run '((((((1))))))')" "1"

# empty containers
assert_eq "empty list" "$($AVON run '[]')" "[]"
assert_eq "list of lists" "$($AVON run '[[], [], []]')" "[[], [], []]"
assert_eq "empty string" "$($AVON run 'length ""')" "0"

# chained let bindings
assert_eq "chained let" "$($AVON run 'let a = 1 in let b = 2 in let c = 3 in a + b + c')" "6"

# lambda returning lambda
assert_eq "curried lambda" "$($AVON run 'let f = \x \y x + y in f 3 4')" "7"

# negative in list
assert_eq "neg list" "$($AVON run '[(-1), (-2), (-3)]')" "[-1, -2, -3]"

# nested if
assert_eq "nested if" "$($AVON run 'if true then (if false then 1 else 2) else 3')" "2"

# if in let
assert_eq "if in let" "$($AVON run 'let x = if true then 42 else 0 in x + 1')" "43"

# boolean logic
assert_eq "and logic" "$($AVON run '1 < 2 && 3 > 2')" "true"
assert_eq "or logic" "$($AVON run 'false || true')" "true"
assert_eq "not in if" "$($AVON run 'if (not false) then "yes" else "no"')" "yes"

# nested function calls
assert_eq "nested calls" "$($AVON run 'length (split "hello world" " ")')" "2"

# dict in let
assert_eq "dict in let" "$($AVON run 'let d = {x: 10, y: 20} in get d "x"')" "10"

# map + range pattern
assert_eq "map range squares" "$($AVON run 'map (\x x * x) (range 1 5)')" "[1, 4, 9, 16, 25]"

# pipe with multiple ops
assert_eq "multi pipe" "$($AVON run 'range 1 5 -> map (\x x * 2) -> filter (\x x > 4) -> length')" "3"

# comparison operators
assert_eq "lte" "$($AVON run '3 <= 3')" "true"
assert_eq "gte" "$($AVON run '3 >= 4')" "false"
assert_eq "neq" "$($AVON run '1 != 2')" "true"
assert_eq "eq" "$($AVON run '5 == 5')" "true"

# string equality
assert_eq "str eq" "$($AVON run '"abc" == "abc"')" "true"
assert_eq "str neq" "$($AVON run '"abc" != "def"')" "true"

# unicode
assert_eq "unicode len" "$($AVON run 'length "héllo"')" "6"

# parser error cases
# parser error output (these print errors but may not exit 1)
assert_contains "unmatched bracket err" "$($AVON run '[1, 2' 2>&1)" "Parse error"
assert_contains "bad lambda err" "$($AVON run '\ 42' 2>&1)" "Parse error"
assert_contains "let no in err" "$($AVON run 'let x =' 2>&1)" "Parse error"

###############################################################################
#  6. FILE I/O PARSERS
###############################################################################
echo "── File I/O Parsers ──"

TEST_TMPDIR=$(mktemp -d)

# Create test files
echo '{"name": "alice", "age": 30}' > "$TEST_TMPDIR/test.json"
echo -e "name: bob\nage: 25" > "$TEST_TMPDIR/test.yaml"
printf '[section]\nkey = value\nnum = 42\n' > "$TEST_TMPDIR/test.ini"
echo '<root><item id="1">hello</item><item id="2">world</item></root>' > "$TEST_TMPDIR/test.xml"
echo '<html><body><p>Hello World</p></body></html>' > "$TEST_TMPDIR/test.html"
echo '<?xml version="1.0"?><opml version="2.0"><head><title>Feed</title></head><body><outline text="Entry1"/></body></opml>' > "$TEST_TMPDIR/test.opml"
echo -e "name,age\nalice,30\nbob,25" > "$TEST_TMPDIR/test.csv"
echo -e "[package]\nname = \"myapp\"\nversion = \"1.0.0\"" > "$TEST_TMPDIR/test.toml"

# json_parse from file
assert_eq "json_parse name" "$($AVON run "get (json_parse \"$TEST_TMPDIR/test.json\") \"name\"")" "alice"
assert_eq "json_parse age" "$($AVON run "get (json_parse \"$TEST_TMPDIR/test.json\") \"age\"")" "30"

# yaml_parse from file
assert_eq "yaml_parse name" "$($AVON run "get (yaml_parse \"$TEST_TMPDIR/test.yaml\") \"name\"")" "bob"

# ini_parse from file
assert_eq "ini_parse section" "$($AVON run "typeof (ini_parse \"$TEST_TMPDIR/test.ini\")")" "Dict"
assert_eq "ini_parse value" "$($AVON run "get (get (ini_parse \"$TEST_TMPDIR/test.ini\") \"section\") \"key\"")" "value"

# xml_parse from file
assert_eq "xml_parse type" "$($AVON run "typeof (xml_parse \"$TEST_TMPDIR/test.xml\")")" "Dict"
assert_eq "xml_parse tag" "$($AVON run "get (xml_parse \"$TEST_TMPDIR/test.xml\") \"tag\"")" "root"

# html_parse from file
assert_eq "html_parse type" "$($AVON run "typeof (html_parse \"$TEST_TMPDIR/test.html\")")" "Dict"
assert_eq "html_parse tag" "$($AVON run "get (html_parse \"$TEST_TMPDIR/test.html\") \"tag\"")" "html"

# opml_parse from file
assert_eq "opml_parse type" "$($AVON run "typeof (opml_parse \"$TEST_TMPDIR/test.opml\")")" "Dict"
assert_eq "opml_parse version" "$($AVON run "get (opml_parse \"$TEST_TMPDIR/test.opml\") \"version\"")" "2.0"

# csv_parse from file
assert_eq "csv_parse type" "$($AVON run "typeof (csv_parse \"$TEST_TMPDIR/test.csv\")")" "List"

# toml_parse from file
assert_eq "toml_parse type" "$($AVON run "typeof (toml_parse \"$TEST_TMPDIR/test.toml\")")" "Dict"

# String-based parsers
assert_eq "json_parse_string" "$($AVON run 'get (json_parse_string "{\"x\": 99}") "x"')" "99"
assert_eq "yaml_parse_string" "$($AVON run 'get (yaml_parse_string "key: val") "key"')" "val"
assert_eq "toml_parse_string" "$($AVON run 'get (toml_parse_string "k = \"v\"") "k"')" "v"
assert_eq "ini_parse_string type" "$($AVON run 'typeof (ini_parse_string "[s]\na = b")')" "Dict"
assert_eq "xml_parse_string type" "$($AVON run 'typeof (xml_parse_string "<r><c>t</c></r>")')" "Dict"
assert_eq "html_parse_string type" "$($AVON run 'typeof (html_parse_string "<p>hi</p>")')" "Dict"
assert_eq "opml_parse_string type" "$($AVON run 'typeof (opml_parse_string "<?xml version=\"1.0\"?><opml version=\"2.0\"><head><title>T</title></head><body></body></opml>")')" "Dict"
assert_eq "csv_parse_string type" "$($AVON run 'typeof (csv_parse_string "a,b\n1,2")')" "List"

# File error handling
assert_failure "json_parse missing file" $AVON run 'json_parse "/nonexistent.json"'
assert_failure "yaml_parse missing file" $AVON run 'yaml_parse "/nonexistent.yaml"'
assert_failure "xml_parse missing file" $AVON run 'xml_parse "/nonexistent.xml"'
assert_failure "html_parse missing file" $AVON run 'html_parse "/nonexistent.html"'
assert_failure "opml_parse missing file" $AVON run 'opml_parse "/nonexistent.opml"'
assert_failure "readfile missing" $AVON run 'readfile "/nonexistent"'

# String parser error handling
assert_failure "json_parse_string invalid" $AVON run 'json_parse_string "not json"'

rm -rf "$TEST_TMPDIR"

###############################################################################
#  7. ERROR PATH TESTING (report says only 14 error tests for 597 total)
###############################################################################
echo "── Error Paths ──"

# Type mismatch errors
assert_failure "map non-func" $AVON run 'map 42 [1, 2]'
assert_failure "filter non-bool return" $AVON run 'filter (\x x) [1, 2]'
assert_failure "sort_by non-func" $AVON run 'sort_by 42 [1, 2]'
assert_failure "abs on string" $AVON run 'abs "hello"'
assert_failure "sqrt on string" $AVON run 'sqrt "hello"'
assert_failure "ceil on string" $AVON run 'ceil "hello"'
assert_failure "floor on list" $AVON run 'floor [1]'
assert_failure "not on string" $AVON run 'not "hello"'
assert_failure "not on number" $AVON run 'not 42'

# Conversion errors
assert_failure "to_int bad string" $AVON run 'to_int "xyz"'
assert_failure "to_float bad string" $AVON run 'to_float "xyz"'

# Arithmetic errors
assert_failure "div by zero" $AVON run '1 / 0'
assert_failure "mod by zero" $AVON run '5 % 0'

# Assertion error
assert_failure "assert false" $AVON run 'assert false "should fail"'

# Regex errors
assert_failure "regex invalid bracket" $AVON run 'regex_match "[" "test"'
assert_failure "regex_replace invalid" $AVON run 'regex_replace "[" "x" "test"'
assert_failure "regex_split invalid" $AVON run 'regex_split "[" "test"'

# Undefined variable
assert_failure "undefined var" $AVON run 'undefined_variable + 1'

# Wrong number of args / type mismatches on data structures
assert_failure "transpose non-rect" $AVON run 'transpose [[1], [2, 3]]'
assert_failure "chunks on string" $AVON run 'chunks 2 "hello"'

# Edge cases that return None/empty (not errors)
assert_eq "head empty none" "$($AVON run 'head []')" "None"
assert_eq "last empty none" "$($AVON run 'last []')" "None"
assert_eq "find no match" "$($AVON run 'find (\x x > 100) [1, 2, 3]')" "None"
assert_eq "find_index no match" "$($AVON run 'find_index (\x x > 100) [1, 2, 3]')" "None"
assert_eq "get missing key" "$($AVON run 'get {a: 1} "b"')" "None"
assert_eq "nth out of bounds" "$($AVON run 'nth 10 [1, 2, 3]')" "None"
assert_eq "tail empty" "$($AVON run 'tail []')" "[]"

###############################################################################
#  8. MISCELLANEOUS LOW-COVERAGE BUILTINS
###############################################################################
echo "── Misc Builtins ──"

# neg
assert_eq "neg positive" "$($AVON run 'neg 5')" "-5"
assert_eq "neg negative" "$($AVON run 'neg (-3)')" "3"
assert_eq "neg zero" "$($AVON run 'neg 0')" "0"
assert_eq "neg pipe" "$($AVON run '7 -> neg')" "-7"

# to_char
assert_eq "to_char 65" "$($AVON run 'to_char 65')" "A"
assert_eq "to_char 97" "$($AVON run 'to_char 97')" "a"
assert_eq "to_char 48" "$($AVON run 'to_char 48')" "0"

# tap (runs function for side effect, returns original value)
assert_eq "tap identity" "$($AVON run 'tap (\x x) 42')" "42"
assert_eq "tap pipe" "$($AVON run '99 -> tap (\x x)')" "99"

# enumerate
assert_eq "enumerate basic" "$($AVON run 'enumerate ["a", "b", "c"]')" "[[0, a], [1, b], [2, c]]"
assert_eq "enumerate empty" "$($AVON run 'enumerate []')" "[]"
assert_eq "enumerate single" "$($AVON run 'enumerate ["only"]')" "[[0, only]]"

# pad_left / pad_right (3 args: string, width, fill_char)
assert_eq "pad_left stars" "$($AVON run 'pad_left "hi" 8 "*"')" "******hi"
assert_eq "pad_right dots" "$($AVON run 'pad_right "hi" 8 "."')" "hi......"
assert_eq "pad_left no pad" "$($AVON run 'pad_left "hello" 3 " "')" "hello"
assert_eq "pad_right no pad" "$($AVON run 'pad_right "hello" 3 " "')" "hello"

# indent
INDENT_OUT=$($AVON run 'indent "hello" 4')
assert_eq "indent 4 spaces" "$INDENT_OUT" "    hello"

# os
assert_eq "os type" "$($AVON run 'typeof (os)')" "String"
assert_contains "os value" "$($AVON run 'os')" "linux"

# env_var_or (with default)
assert_eq "env_var_or exists" "$($AVON run 'typeof (env_var_or "HOME" "fallback")')" "String"
assert_eq "env_var_or fallback" "$($AVON run 'env_var_or "AVON_NONEXISTENT_12345" "default_val"')" "default_val"

# exists
assert_eq "exists true" "$($AVON run 'exists "Cargo.toml"')" "true"
assert_eq "exists false" "$($AVON run 'exists "/nonexistent_file_xyz"')" "false"

# basename / dirname
assert_eq "basename" "$($AVON run 'basename "/home/user/file.txt"')" "file.txt"
assert_eq "dirname" "$($AVON run 'dirname "/home/user/file.txt"')" "/home/user"

# readfile / readlines
echo "test line 1" > /tmp/avon_readtest.txt
echo "test line 2" >> /tmp/avon_readtest.txt
assert_contains "readfile content" "$($AVON run 'readfile "/tmp/avon_readtest.txt"')" "test line 1"
assert_eq "readlines count" "$($AVON run 'length (readlines "/tmp/avon_readtest.txt")')" "2"
rm -f /tmp/avon_readtest.txt

# walkdir
assert_eq "walkdir type" "$($AVON run 'typeof (walkdir "src")')" "List"
# walkdir should find Rust files
assert_eq "walkdir finds files" "$($AVON run 'let w = walkdir "src" in let n = length w in n > 0')" "true"

# has_key
assert_eq "has_key true" "$($AVON run 'has_key {a: 1} "a"')" "true"
assert_eq "has_key false" "$($AVON run 'has_key {a: 1} "b"')" "false"

# keys / values (sort to handle dict ordering)
assert_eq "keys sorted" "$($AVON run '{a: 1, b: 2} -> keys -> sort')" "[a, b]"
assert_eq "values sorted" "$($AVON run '{a: 1, b: 2} -> values -> sort')" "[1, 2]"

# set (dict)
assert_eq "dict set" "$($AVON run 'set {a: 1} "b" 2 -> keys -> sort')" "[a, b]"

# product
assert_eq "product basic" "$($AVON run 'product [1, 2, 3, 4]')" "24"
assert_eq "product empty" "$($AVON run 'product []')" "1"

# all / any
assert_eq "all true" "$($AVON run 'all (\x x > 0) [1, 2, 3]')" "true"
assert_eq "all false" "$($AVON run 'all (\x x > 2) [1, 2, 3]')" "false"
assert_eq "any true" "$($AVON run 'any (\x x > 2) [1, 2, 3]')" "true"
assert_eq "any false" "$($AVON run 'any (\x x > 10) [1, 2, 3]')" "false"

# count
assert_eq "count basic" "$($AVON run 'count (\x x > 2) [1, 2, 3, 4, 5]')" "3"
assert_eq "count none" "$($AVON run 'count (\x x > 100) [1, 2, 3]')" "0"

###############################################################################
#  9. PIPELINE & COMPOSITION PATTERNS
###############################################################################
echo "── Pipeline Patterns ──"

# Complex pipeline
assert_eq "complex pipe" "$($AVON run 'range 1 10 -> filter (\x x % 2 == 0) -> map (\x x * x) -> sum')" "220"

# Pipeline with string ops
assert_eq "str pipeline" "$($AVON run 'words "Hello World" -> map upper -> unwords')" "HELLO WORLD"

# Pipeline with type checking
assert_eq "type pipe" "$($AVON run '42 -> typeof')" "Number"
assert_eq "type pipe str" "$($AVON run '"hi" -> typeof')" "String"
assert_eq "type pipe list" "$($AVON run '[1] -> typeof')" "List"

# Nested map with pipes
assert_eq "nested map" "$($AVON run '[[1,2],[3,4]] -> map (\row map (\x x + 10) row)')" "[[11, 12], [13, 14]]"

# Pipeline with dict ops (using sort for deterministic output)
assert_eq "dict pipeline" "$($AVON run '{a: 1, b: 2, c: 3} -> values -> sort -> sum')" "6"

###############################################################################
echo ""
echo "Deep Coverage Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
