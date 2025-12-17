#!/bin/bash
# Test parallel functions (pmap, pfilter, pfold) for correctness

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"
AVON_BIN="$AVON"

TEST_FILE="/tmp/test_parallel_functions.av"

cat > "$TEST_FILE" << 'EOF'
# Test suite for parallel list functions: pmap, pfilter, pfold
# These should produce identical results to their sequential counterparts
# (except pfold which requires associative combiners)

# Test 1: pmap basic - identical to map
let test_pmap_basic = (
    let sequential = map (\x x * 2) [1, 2, 3, 4, 5] in
    let parallel = pmap (\x x * 2) [1, 2, 3, 4, 5] in
    sequential == parallel
) in

# Test 2: pmap preserves order
let test_pmap_order = (
    let result = pmap (\x x) [1, 2, 3, 4, 5] in
    result == [1, 2, 3, 4, 5]
) in

# Test 3: pmap with empty list
let test_pmap_empty = (
    let result = pmap (\x x * 2) [] in
    result == []
) in

# Test 4: pmap with single element
let test_pmap_single = (
    let result = pmap (\x x * 10) [42] in
    result == [420]
) in

# Test 5: pmap with string transformation
let test_pmap_strings = (
    let sequential = map upper ["hello", "world"] in
    let parallel = pmap upper ["hello", "world"] in
    sequential == parallel
) in

# Test 6: pfilter basic - identical to filter
let test_pfilter_basic = (
    let sequential = filter (\x x > 2) [1, 2, 3, 4, 5] in
    let parallel = pfilter (\x x > 2) [1, 2, 3, 4, 5] in
    sequential == parallel
) in

# Test 7: pfilter preserves order
let test_pfilter_order = (
    let result = pfilter (\x x % 2 == 0) [1, 2, 3, 4, 5, 6, 7, 8] in
    result == [2, 4, 6, 8]
) in

# Test 8: pfilter with all pass
let test_pfilter_all_pass = (
    let result = pfilter (\x x > 0) [1, 2, 3, 4, 5] in
    result == [1, 2, 3, 4, 5]
) in

# Test 9: pfilter with none pass
let test_pfilter_none_pass = (
    let result = pfilter (\x x > 100) [1, 2, 3, 4, 5] in
    result == []
) in

# Test 10: pfilter with empty list
let test_pfilter_empty = (
    let result = pfilter (\x x > 0) [] in
    result == []
) in

# Test 11: pfold with addition (associative - should work correctly)
let test_pfold_add = (
    let sequential = fold (\acc \x acc + x) 0 [1, 2, 3, 4, 5] in
    let parallel = pfold (\acc \x acc + x) 0 [1, 2, 3, 4, 5] in
    sequential == parallel && parallel == 15
) in

# Test 12: pfold with multiplication (associative - should work correctly)
let test_pfold_multiply = (
    let sequential = fold (\acc \x acc * x) 1 [1, 2, 3, 4, 5] in
    let parallel = pfold (\acc \x acc * x) 1 [1, 2, 3, 4, 5] in
    sequential == parallel && parallel == 120
) in

# Test 13: pfold with empty list
let test_pfold_empty = (
    let result = pfold (\acc \x acc + x) 0 [] in
    result == 0
) in

# Test 14: pfold with single element
let test_pfold_single = (
    let result = pfold (\acc \x acc + x) 0 [42] in
    result == 42
) in

# Test 15: pfold with max (associative)
let test_pfold_max = (
    let sequential = fold (\acc \x if x > acc then x else acc) 0 [3, 1, 4, 1, 5, 9, 2, 6] in
    let parallel = pfold (\acc \x if x > acc then x else acc) 0 [3, 1, 4, 1, 5, 9, 2, 6] in
    sequential == parallel && parallel == 9
) in

# Test 16: pmap with closure capturing variable
let test_pmap_closure = (
    let multiplier = 10 in
    let result = pmap (\x x * multiplier) [1, 2, 3] in
    result == [10, 20, 30]
) in

# Test 17: pfilter with closure
let test_pfilter_closure = (
    let threshold = 3 in
    let result = pfilter (\x x > threshold) [1, 2, 3, 4, 5] in
    result == [4, 5]
) in

# Test 18: pfold with closure (using associative operation)
# Note: The combiner must be associative for pfold to work correctly
let test_pfold_closure = (
    let offset = 100 in
    let result = pfold (\acc \x if x > acc then x else acc) 0 [3, 1, 4, 1, 5] in
    result == 5
) in

# Test 19: pmap on larger list (verify correctness at scale)
let test_pmap_large = (
    let input = range 1 100 in
    let sequential = map (\x x * x) input in
    let parallel = pmap (\x x * x) input in
    sequential == parallel
) in

# Test 20: pfilter on larger list
let test_pfilter_large = (
    let input = range 1 100 in
    let sequential = filter (\x x % 3 == 0) input in
    let parallel = pfilter (\x x % 3 == 0) input in
    sequential == parallel
) in

# Run all tests and return summary
let results = [
    test_pmap_basic,
    test_pmap_order,
    test_pmap_empty,
    test_pmap_single,
    test_pmap_strings,
    test_pfilter_basic,
    test_pfilter_order,
    test_pfilter_all_pass,
    test_pfilter_none_pass,
    test_pfilter_empty,
    test_pfold_add,
    test_pfold_multiply,
    test_pfold_empty,
    test_pfold_single,
    test_pfold_max,
    test_pmap_closure,
    test_pfilter_closure,
    test_pfold_closure,
    test_pmap_large,
    test_pfilter_large
] in

# Count passes
let pass_count = fold (\acc \v if v then acc + 1 else acc) 0 results in
let fail_count = fold (\acc \v if v then acc else acc + 1) 0 results in
let total = length results in

# Report results
if fail_count == 0 then
    trace "PASS" (concat "All " (concat (to_string total) " parallel function tests passed"))
else
    trace "FAIL" (concat "Tests failed: " (to_string fail_count))
EOF

# Run the test
OUTPUT=$($AVON_BIN "$TEST_FILE" 2>&1)
EXIT_CODE=$?

# Clean up
rm "$TEST_FILE"

# Check results
if [ $EXIT_CODE -ne 0 ]; then
    echo "Avon exited with error code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

if echo "$OUTPUT" | grep -q "FAIL"; then
    echo "Parallel Function Tests failed:"
    echo "$OUTPUT"
    exit 1
fi

# Check for PASS
if echo "$OUTPUT" | grep -q "PASS"; then
    echo "✓ All parallel function tests passed"
    exit 0
else
    echo "Parallel Function Tests - No pass found"
    echo "$OUTPUT"
    exit 1
fi
