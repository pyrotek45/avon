#!/bin/bash
# Extended coverage tests for builtins found in source NAMES arrays
# but previously missing from all test files.
#
# Covers: math (abs, ceil, floor, gcd, lcm, log, log10, pow, round, sqrt,
#         random_int, random_float, uuid),
#         string (lines, words, unlines, unwords, base64_encode, base64_decode,
#                 hash_md5, hash_sha256),
#         list (chunks, combinations, last, permutations, transpose, windows),
#         file_io (abspath, glob, relpath),
#         env (env_var, env_vars),
#         types (is_dict)

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

###############################################################################
#  MATH BUILTINS
###############################################################################
echo "── Math Builtins ──"

# abs
assert_eq "abs positive" "$($AVON run 'abs 5')" "5"
assert_eq "abs negative" "$($AVON run 'abs (-5)')" "5"
assert_eq "abs zero" "$($AVON run 'abs 0')" "0"
assert_eq "abs float neg" "$($AVON run 'abs (-3.14)')" "3.14"

# ceil
assert_eq "ceil 3.2" "$($AVON run 'ceil 3.2')" "4"
assert_eq "ceil 3.9" "$($AVON run 'ceil 3.9')" "4"
assert_eq "ceil neg" "$($AVON run 'ceil (-2.3)')" "-2"
assert_eq "ceil whole" "$($AVON run 'ceil 5.0')" "5"

# floor
assert_eq "floor 3.8" "$($AVON run 'floor 3.8')" "3"
assert_eq "floor 3.1" "$($AVON run 'floor 3.1')" "3"
assert_eq "floor neg" "$($AVON run 'floor (-2.3)')" "-3"
assert_eq "floor whole" "$($AVON run 'floor 5.0')" "5"

# round
assert_eq "round 3.5" "$($AVON run 'round 3.5')" "4"
assert_eq "round 3.4" "$($AVON run 'round 3.4')" "3"
assert_eq "round 3.6" "$($AVON run 'round 3.6')" "4"
assert_eq "round neg" "$($AVON run 'round (-2.7)')" "-3"
assert_eq "round whole" "$($AVON run 'round 7.0')" "7"

# sqrt
assert_eq "sqrt 16" "$($AVON run 'sqrt 16')" "4"
assert_eq "sqrt 0" "$($AVON run 'sqrt 0')" "0"
assert_eq "sqrt 1" "$($AVON run 'sqrt 1')" "1"
assert_eq "sqrt 2 approx" "$($AVON run 'let x = sqrt 2 in let y = round (x * 1000) in y / 1000')" "1.414"

# pow
assert_eq "pow 2 10" "$($AVON run 'pow 2 10')" "1024"
assert_eq "pow 3 0" "$($AVON run 'pow 3 0')" "1"
assert_eq "pow 5 1" "$($AVON run 'pow 5 1')" "5"
assert_eq "pow 10 3" "$($AVON run 'pow 10 3')" "1000"

# log (natural log)
assert_eq "log 1" "$($AVON run 'log 1')" "0"

# log10
assert_eq "log10 100" "$($AVON run 'log10 100')" "2"
assert_eq "log10 1000" "$($AVON run 'log10 1000')" "3"
assert_eq "log10 1" "$($AVON run 'log10 1')" "0"

# gcd
assert_eq "gcd 12 8" "$($AVON run 'gcd 12 8')" "4"
assert_eq "gcd 7 13" "$($AVON run 'gcd 7 13')" "1"
assert_eq "gcd 100 25" "$($AVON run 'gcd 100 25')" "25"
assert_eq "gcd same" "$($AVON run 'gcd 42 42')" "42"

# lcm
assert_eq "lcm 4 6" "$($AVON run 'lcm 4 6')" "12"
assert_eq "lcm 3 5" "$($AVON run 'lcm 3 5')" "15"
assert_eq "lcm same" "$($AVON run 'lcm 7 7')" "7"
assert_eq "lcm 1 n" "$($AVON run 'lcm 1 99')" "99"

# random_int (type and range check)
assert_eq "random_int type" "$($AVON run 'typeof (random_int 1 100)')" "Number"
assert_eq "random_int is_number" "$($AVON run 'is_number (random_int 1 100)')" "true"

# random_float (type check)
assert_eq "random_float type" "$($AVON run 'typeof (random_float 0.0 1.0)')" "Number"
assert_eq "random_float is_number" "$($AVON run 'is_number (random_float 0.0 1.0)')" "true"

# uuid
assert_eq "uuid type" "$($AVON run 'typeof (uuid)')" "String"
assert_eq "uuid length" "$($AVON run 'length (uuid)')" "36"
# uuid contains hyphens in standard format
assert_contains "uuid format" "$($AVON run 'uuid')" "-"

###############################################################################
#  STRING BUILTINS
###############################################################################
echo "── String Builtins ──"

# lines
assert_eq "lines basic" "$($AVON run 'lines "a\nb\nc"')" "[a, b, c]"
assert_eq "lines single" "$($AVON run 'lines "hello"')" "[hello]"
assert_eq "lines empty" "$($AVON run 'lines ""')" "[]"

# words
assert_eq "words basic" "$($AVON run 'words "hello world foo"')" "[hello, world, foo]"
assert_eq "words single" "$($AVON run 'words "hello"')" "[hello]"
assert_eq "words extra spaces" "$($AVON run 'words "  a  b  "')" "[a, b]"

# unwords
assert_eq "unwords basic" "$($AVON run 'unwords ["hello", "world"]')" "hello world"
assert_eq "unwords single" "$($AVON run 'unwords ["hello"]')" "hello"
assert_eq "unwords empty" "$($AVON run 'unwords []')" ""

# unlines - produces actual newlines
UNLINES_OUT=$($AVON run 'unlines ["a", "b", "c"]')
UNLINES_LINES=$(echo "$UNLINES_OUT" | wc -l | tr -d ' ')
assert_eq "unlines produces newlines" "$UNLINES_LINES" "3"
assert_eq "unlines first line" "$(echo "$UNLINES_OUT" | head -1)" "a"

# roundtrip: words/unwords
assert_eq "words/unwords roundtrip" "$($AVON run 'unwords (words "hello world")')" "hello world"

# base64_encode
assert_eq "base64 encode hello" "$($AVON run 'base64_encode "hello"')" "aGVsbG8="
assert_eq "base64 encode empty" "$($AVON run 'base64_encode ""')" ""

# base64_decode
assert_eq "base64 decode hello" "$($AVON run 'base64_decode "aGVsbG8="')" "hello"
assert_eq "base64 decode empty" "$($AVON run 'base64_decode ""')" ""

# roundtrip: base64
assert_eq "base64 roundtrip" "$($AVON run 'base64_decode (base64_encode "avon rocks!")')" "avon rocks!"

# hash_md5
assert_eq "hash_md5 test" "$($AVON run 'hash_md5 "test"')" "098f6bcd4621d373cade4e832627b4f6"
assert_eq "hash_md5 empty" "$($AVON run 'hash_md5 ""')" "d41d8cd98f00b204e9800998ecf8427e"
assert_eq "hash_md5 length" "$($AVON run 'length (hash_md5 "x")')" "32"

# hash_sha256
assert_eq "hash_sha256 test" "$($AVON run 'hash_sha256 "test"')" "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
assert_eq "hash_sha256 empty" "$($AVON run 'hash_sha256 ""')" "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
assert_eq "hash_sha256 length" "$($AVON run 'length (hash_sha256 "x")')" "64"

# determinism: same input → same hash
assert_eq "hash_md5 deterministic" "$($AVON run 'let a = hash_md5 "abc" in let b = hash_md5 "abc" in a == b')" "true"
assert_eq "hash_sha256 deterministic" "$($AVON run 'let a = hash_sha256 "abc" in let b = hash_sha256 "abc" in a == b')" "true"

###############################################################################
#  LIST BUILTINS
###############################################################################
echo "── List Builtins ──"

# last
assert_eq "last basic" "$($AVON run 'last [1, 2, 3]')" "3"
assert_eq "last single" "$($AVON run 'last [42]')" "42"
assert_eq "last empty" "$($AVON run 'last []')" "None"
assert_eq "last strings" "$($AVON run 'last ["a", "b", "c"]')" "c"

# chunks
assert_eq "chunks 2 of 5" "$($AVON run 'chunks 2 [1, 2, 3, 4, 5]')" "[[1, 2], [3, 4], [5]]"
assert_eq "chunks 3 of 6" "$($AVON run 'chunks 3 [1, 2, 3, 4, 5, 6]')" "[[1, 2, 3], [4, 5, 6]]"
assert_eq "chunks 1" "$($AVON run 'chunks 1 [1, 2, 3]')" "[[1], [2], [3]]"
assert_eq "chunks larger" "$($AVON run 'chunks 5 [1, 2]')" "[[1, 2]]"
assert_eq "chunks empty" "$($AVON run 'chunks 2 []')" "[]"

# combinations
assert_eq "combinations 2 of 3" "$($AVON run 'combinations 2 [1, 2, 3]')" "[[1, 2], [1, 3], [2, 3]]"
assert_eq "combinations 1" "$($AVON run 'combinations 1 [1, 2, 3]')" "[[1], [2], [3]]"
assert_eq "combinations all" "$($AVON run 'combinations 3 [1, 2, 3]')" "[[1, 2, 3]]"
assert_eq "combinations 0" "$($AVON run 'combinations 0 [1, 2, 3]')" "[[]]"
assert_eq "combinations count" "$($AVON run 'length (combinations 2 [1, 2, 3, 4])')" "6"

# permutations
assert_eq "permutations 2 of 3" "$($AVON run 'permutations 2 [1, 2, 3]')" "[[1, 2], [1, 3], [2, 1], [2, 3], [3, 1], [3, 2]]"
assert_eq "permutations 1" "$($AVON run 'permutations 1 [1, 2]')" "[[1], [2]]"
assert_eq "permutations count" "$($AVON run 'length (permutations 2 [1, 2, 3])')" "6"

# windows
assert_eq "windows 3 of 5" "$($AVON run 'windows 3 [1, 2, 3, 4, 5]')" "[[1, 2, 3], [2, 3, 4], [3, 4, 5]]"
assert_eq "windows 2" "$($AVON run 'windows 2 [1, 2, 3]')" "[[1, 2], [2, 3]]"
assert_eq "windows full" "$($AVON run 'windows 3 [1, 2, 3]')" "[[1, 2, 3]]"
assert_eq "windows too large" "$($AVON run 'windows 5 [1, 2, 3]')" "[]"
assert_eq "windows empty" "$($AVON run 'windows 2 []')" "[]"

# transpose
assert_eq "transpose 3x2" "$($AVON run 'transpose [[1, 2], [3, 4], [5, 6]]')" "[[1, 3, 5], [2, 4, 6]]"
assert_eq "transpose 2x3" "$($AVON run 'transpose [[1, 2, 3], [4, 5, 6]]')" "[[1, 4], [2, 5], [3, 6]]"
assert_eq "transpose empty" "$($AVON run 'transpose []')" "[]"
assert_eq "transpose 1x1" "$($AVON run 'transpose [[42]]')" "[[42]]"
# non-rectangular matrix should error
assert_failure "transpose non-rect" $AVON run 'transpose [[1], [2, 3]]'
# double transpose is identity
assert_eq "transpose roundtrip" "$($AVON run 'transpose (transpose [[1, 2], [3, 4]])')" "[[1, 2], [3, 4]]"

###############################################################################
#  FILE I/O BUILTINS
###############################################################################
echo "── File I/O Builtins ──"

# Create temp directory for file tests
TEST_TMPDIR=$(mktemp -d)
echo "test content" > "$TEST_TMPDIR/test.txt"
mkdir -p "$TEST_TMPDIR/sub"
echo "sub content" > "$TEST_TMPDIR/sub/file.txt"

# abspath
ABSPATH_RESULT=$($AVON run "abspath \"$TEST_TMPDIR/test.txt\"")
assert_eq "abspath returns string" "$($AVON run "typeof (abspath \"$TEST_TMPDIR/test.txt\")")" "String"
assert_contains "abspath contains path" "$ABSPATH_RESULT" "$TEST_TMPDIR"

# glob
assert_eq "glob returns list" "$($AVON run "typeof (glob \"$TEST_TMPDIR/*.txt\")")" "List"
assert_eq "glob finds file" "$($AVON run "length (glob \"$TEST_TMPDIR/*.txt\")")" "1"
assert_eq "glob no match" "$($AVON run "glob \"$TEST_TMPDIR/*.xyz\"")" "[]"

# relpath (base, target)
assert_eq "relpath basic" "$($AVON run "relpath \"/home\" \"/home/user/file.txt\"")" "user/file.txt"
assert_eq "relpath same dir" "$($AVON run "relpath \"/home/user\" \"/home/user\"")" ""
assert_eq "relpath up" "$($AVON run "relpath \"/home/user/a\" \"/home/user/b\"")" "../b"

# Clean up
rm -rf "$TEST_TMPDIR"

###############################################################################
#  ENVIRONMENT BUILTINS
###############################################################################
echo "── Environment Builtins ──"

# env_var
assert_eq "env_var HOME type" "$($AVON run 'typeof (env_var "HOME")')" "String"
assert_contains "env_var HOME" "$($AVON run 'env_var "HOME"')" "/"
# env_var of unset var should error
assert_failure "env_var unset" $AVON run 'env_var "AVON_NONEXISTENT_VAR_12345"'

# env_vars
assert_eq "env_vars type" "$($AVON run 'typeof (env_vars)')" "Dict"
# env_vars should be a dict and we can get HOME from it
assert_eq "env_vars has HOME" "$($AVON run 'is_string (get (env_vars) "HOME")')" "true"

###############################################################################
#  TYPE BUILTINS
###############################################################################
echo "── Type Builtins ──"

# is_dict
assert_eq "is_dict true" "$($AVON run 'is_dict {a: 1}')" "true"
assert_eq "is_dict false num" "$($AVON run 'is_dict 42')" "false"
assert_eq "is_dict false str" "$($AVON run 'is_dict "hello"')" "false"
assert_eq "is_dict false list" "$($AVON run 'is_dict [1, 2]')" "false"
assert_eq "is_dict false bool" "$($AVON run 'is_dict true')" "false"
assert_eq "is_dict false none" "$($AVON run 'is_dict none')" "false"

###############################################################################
#  PIPE CHAINS & COMPOSITION  (exercising new builtins in pipelines)
###############################################################################
echo "── Pipe Chains & Composition ──"

# chain: words → map upper → unwords
assert_eq "words->upper->unwords" "$($AVON run 'words "hello world" -> map upper -> unwords')" "HELLO WORLD"

# chain: range → chunks → map length
assert_eq "range->chunks->lengths" "$($AVON run 'range 1 6 -> chunks 3 -> map length')" "[3, 3]"

# chain: combinations → map sum
assert_eq "combinations->sums" "$($AVON run 'combinations 2 [1, 2, 3] -> map (\pair fold (\a \b a + b) 0 pair)')" "[3, 4, 5]"

# chain: abs in pipe
assert_eq "pipe abs" "$($AVON run '(-7) -> abs')" "7"

# chain: base64 roundtrip in pipe
assert_eq "pipe base64 rt" "$($AVON run '"avon" -> base64_encode -> base64_decode')" "avon"

# chain: lines → length
assert_eq "pipe lines length" "$($AVON run '"a\nb\nc\nd" -> lines -> length')" "4"

# chain: hash in pipe
assert_eq "pipe hash_md5" "$($AVON run '"test" -> hash_md5 -> length')" "32"

# math pipe chain
assert_eq "pipe ceil" "$($AVON run '3.14 -> ceil')" "4"
assert_eq "pipe floor" "$($AVON run '3.99 -> floor')" "3"
assert_eq "pipe sqrt" "$($AVON run '25 -> sqrt')" "5"

# chunks → flatten roundtrip
assert_eq "chunks flatten rt" "$($AVON run '[1,2,3,4,5,6] -> chunks 2 -> flatten')" "[1, 2, 3, 4, 5, 6]"

# windows → map head
assert_eq "windows map head" "$($AVON run 'windows 2 [10, 20, 30, 40] -> map head')" "[10, 20, 30]"

# last in pipe
assert_eq "pipe last" "$($AVON run '[10, 20, 30] -> last')" "30"

# uuid uniqueness (two uuids should differ)
assert_eq "uuid unique" "$($AVON run 'uuid == uuid')" "false"

###############################################################################
#  EDGE CASES & ERROR HANDLING
###############################################################################
echo "── Edge Cases ──"

# math edge cases
assert_eq "pow 0 0" "$($AVON run 'pow 0 0')" "1"
assert_eq "gcd 0 5" "$($AVON run 'gcd 0 5')" "5"
assert_eq "abs large" "$($AVON run 'abs (-999999)')" "999999"

# list edge cases
assert_eq "chunks single elem" "$($AVON run 'chunks 1 [42]')" "[[42]]"
assert_eq "windows 1" "$($AVON run 'windows 1 [1, 2, 3]')" "[[1], [2], [3]]"
assert_eq "last nested" "$($AVON run 'last [[1, 2], [3, 4]]')" "[3, 4]"
assert_eq "combinations empty" "$($AVON run 'combinations 2 []')" "[]"

# string edge cases
assert_eq "words empty" "$($AVON run 'words ""')" "[]"
assert_eq "hash_md5 spaces" "$($AVON run 'hash_md5 " "')" "7215ee9c7d9dc229d2921a40e899ec5f"

# type error handling
assert_failure "abs on string" $AVON run 'abs "hello"'
assert_failure "ceil on string" $AVON run 'ceil "hello"'
assert_failure "sqrt on string" $AVON run 'sqrt "hello"'
assert_failure "chunks on string" $AVON run 'chunks 2 "hello"'
assert_failure "windows on string" $AVON run 'windows 2 "hello"'
assert_failure "transpose on num" $AVON run 'transpose 42'

###############################################################################
echo ""
echo "Extended Coverage Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
