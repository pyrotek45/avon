# Avon Builtin Function Improvements

## Status: Challenge Solved Despite Missing Builtins ✅

Successfully solved AoC 2024 Day 15 (Gift Shop Product ID Verification) using a **mathematical workaround** instead of string manipulation. The solution demonstrates how Avon's higher-order functions and expressive arithmetic enable creative problem-solving even without certain convenience builtins.

**Solution Approach:** Instead of using string slicing (`slice()`) to extract and compare patterns, we leveraged modular arithmetic:
- For a number like 6464 = 64 × (10² + 1)
- Check if `num % (10^half_len + 1) == 0`
- Computed 10^n using `fold (\acc \_ acc * 10) 1 (range ...)`
- Result: Correct answer 1,227,775,554 ✓

This validates Avon's functional programming strengths, but also highlights where convenience builtins would significantly improve developer experience.

## ⚠️ IMPORTANT: Remember to Update VSCode Extension When Adding New Builtins!

When adding new builtin functions to Avon, **always update the syntax highlighting** in the VSCode extension:

**File to update:** `vscode/syntaxes/avon.tmLanguage.json`

**Location:** Line ~38 in the "builtin" patterns section

**Action:** Add the new builtin names to the regex match pattern (keep alphabetical order):
```json
"match": "(?<!\\.)\\b(existing_builtins|NEW_BUILTIN_HERE|more_builtins)\\b(?=\\s|\\(|\\[|\\||->|\\{)"
```

**Example:** When slice, char_at, and chars were added:
- Location: Between "center" and "concat" (slice), after "basename" (char_at), after "chars"
- Result: Syntax highlighting now recognizes all three new functions

**Why:** Without this update:
- VSCode won't highlight the new builtins with special syntax coloring
- Users won't get autocomplete for new functions
- New functions look like regular identifiers instead of builtins

This is a quick fix (2 minutes) but makes a big UX difference!

## Issues Encountered While Solving AoC Challenge

While solving the challenge, several limitations were discovered that would be easily solved with additional builtin functions or documentation improvements.

### Challenge: Finding repeated patterns in strings (e.g., "6464" = "64" + "64")

**Problem:** Need to extract substrings and compare them to check if a string is made of a pattern repeated exactly twice.

**Current Workaround:** Extremely complex and unintuitive using `split("")`, `join()`, `take()`, and `drop()`.

**Missing Builtins:**

1. **`slice(str, start, end)` - String/List slicing**
   - Extracts a substring or sublist
   - Usage: `slice "hello" 1 3` → `"el"`
   - Usage: `slice [1,2,3,4,5] 1 3` → `[2,3]`
   - Would replace: `take (drop str start) (end - start)`
   - **Priority:** HIGH - Very common operation

2. **`char_at(str, index)` - Get character at index**
   - Returns the character at a specific position in a string
   - Usage: `char_at "hello" 2` → `"l"`
   - **Priority:** MEDIUM - Useful for character-level string manipulation

3. **`starts_with_pattern(str, pattern)` - Pattern matching helper**
   - Check if string starts with a pattern multiple times
   - Usage: `starts_with_pattern "123123" "123"` → `true`
   - **Priority:** LOW - Can be built from existing functions, but verbose

4. **`repeat_count(str)` - Find repeat count**
   - Returns how many times a pattern repeats (0 if not repeated, 1 if pattern+pattern, etc.)
   - Usage: `repeat_count "6464"` → `2`, `repeat_count "123456"` → `0`
   - **Priority:** LOW - Convenience function

### Other Issues Discovered:

**String Splitting Edge Cases:**
- `split "" ""` returns `["", content, ""]` with unwanted empty strings at edges
- Requires manual cleanup with `drop` and `take`
- **Suggestion:** Add optional parameter to `split` to control trimming behavior
- **Alternative:** Add `chars(str)` builtin that returns list of characters without empty strings

**Documentation Gaps:**
1. **Function reference incomplete** - Should have comprehensive list of all 111+ builtins with examples
   - Current state: Scattered across tutorial.md, making it hard to find specific functions
   - **Suggestion:** Create `BUILTINS.md` with organized reference (string, list, math, I/O, etc.)

2. **String functions not discoverable** - No systematic list showing:
   - Which functions work on strings (concat, upper, lower, trim, split, join, replace, contains, starts_with, ends_with, length, repeat, pad_left, pad_right, indent, etc.)
   - Return types and exact signatures

3. **List functions scattered** - Documented in tutorial but not systematically organized

### Proposed Solution: `BUILTINS.md` File Structure

```markdown
# Avon Builtins Reference

## String Functions
- `concat(a, b)` - Concatenate two strings
- `upper(s)` - Convert to uppercase
- `slice(s, start, end)` - Extract substring [NEW]
- `char_at(s, index)` - Get character at index [NEW]
- etc.

## List Functions
- `map(fn, list)` - Apply function to each element
- `filter(fn, list)` - Keep elements where fn(elem) is true
- `slice(list, start, end)` - Extract sublist [NEW]
- etc.

## Type Functions
- `is_string(x)` - Check if string
- `to_string(x)` - Convert to string
- etc.

## I/O Functions
- `readfile(path)` - Read file
- `readlines(path)` - Read file as lines
- etc.
```

### Recommended Priority for Implementation:

**TIER 1 (High Impact - Easy to Implement):**
- `slice(str_or_list, start, end)` - Most requested, used constantly
- `chars(str)` - Clean alternative to `split("")` hack

**TIER 2 (Medium Impact - Moderate Implementation):**
- `char_at(str, index)` - Useful for character manipulation
- Better `split()` with trim option

**TIER 3 (Nice to Have):**
- `repeat_count(str)` - Pattern matching convenience
- String indexing operator: `str[0]` syntax

### Current Workaround Example (Why We Need These):

```avon
# To get substring [0, 3) from "hello":
# Current way (UGLY):
let str = "hello" in
let chars = take (drop (split str "") 1) (length str) in
let result = join (take chars 3) "" in
result  # "hel"

# With slice builtin (CLEAN):
slice "hello" 0 3  # "hel"

# To check if string is pattern repeated twice:
# Current way (VERY COMPLEX):
let is_doubled = \s
  let len = length s in
  if len % 2 != 0 then false
  else
    let chars = take (drop (split s "") 1) len in
    let half = len / 2 in
    let first = join (take chars half) "" in
    let second = join (take (drop chars half) half) "" in
    first == second
in

# With better string functions (CLEAN):
let is_doubled = \s
  let len = length s in
  if len % 2 != 0 then false
  else
    let half = len / 2 in
    slice s 0 half == slice s half len
in
```

## Files Affected

- `src/builtins.rs` - Add new builtin functions
- `src/eval.rs` - Register new builtins
- `BUILTINS.md` - Document all builtins (NEW FILE)
- `tutorial/TUTORIAL.md` - Reference new builtins section

## Testing Required

- Unit tests for each new builtin
- Edge cases: empty strings/lists, out-of-bounds indices
- Integration tests with AoC challenges to verify they work well for real problems
