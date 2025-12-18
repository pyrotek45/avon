# Avon Language Improvement Proposals

This report analyzes gotchas and workarounds discovered during documentation, identifying which ones represent genuine language design issues that could be fixed to improve developer experience.

---

## ✅ Implemented Features

The following proposals have been implemented:

1. **Short-circuit `&&`/`||`** - Now properly short-circuits (P0)
2. **Power operator `**`** - Added with right-associative precedence (P2)
3. **Float division `/`** - Now always returns float (P2)
4. **Integer division `//`** - New operator for floor division (P2)
5. **`contains` for lists** - Now works for element membership (Quick Win)

---

## Priority Levels

- **P0 (Critical)**: Breaks expected behavior, causes confusion
- **P1 (High)**: Common pain point, requires awkward workarounds
- **P2 (Medium)**: Inconvenient but manageable
- **P3 (Low)**: Minor improvement, nice-to-have

---

## P0: Critical Issues

### 1. ✅ IMPLEMENTED: `&&` and `||` Now Short-Circuit

**Status:** Fixed! Logical operators now properly short-circuit.

**Previous Behavior:**
```avon
false && (1 / 0 > 0)  # ERROR: division by zero
true || (1 / 0 > 0)   # ERROR: division by zero
```

**Expected Behavior:** Short-circuit evaluation (standard in virtually all languages)

**Impact:** 
- Forces users to use `if-then-else` for safety checks
- Breaks common patterns like `x != none && x.field`
- Surprising to anyone from JS, Python, Rust, etc.

**Proposed Fix:** Implement lazy evaluation for `&&` and `||` in `src/eval/mod.rs`:
```rust
// Current (lines 580-586):
let l_eval = eval_with_depth(*lhs.clone(), symbols, source, depth + 1)?;
let r_eval = eval_with_depth(*rhs.clone(), symbols, source, depth + 1)?;

// Proposed:
Token::And(_) => {
    let l_eval = eval_with_depth(*lhs.clone(), symbols, source, depth + 1)?;
    match l_eval {
        Value::Bool(false) => Ok(Value::Bool(false)),  // Short-circuit
        Value::Bool(true) => {
            let r_eval = eval_with_depth(*rhs.clone(), symbols, source, depth + 1)?;
            match r_eval {
                Value::Bool(rb) => Ok(Value::Bool(rb)),
                _ => Err(...)
            }
        }
        _ => Err(...)
    }
}
```

**Effort:** Low (localized change in eval)

---

### 2. Operator Precedence Issues with Function Calls

**Current Behavior:**
```avon
# These fail or parse incorrectly:
let indices = range 0 (length items - 1) in ...  # Parses wrong
find (\k k == key) (keys dict) != none           # Needs extra parens
```

**Impact:**
- Requires excessive parentheses
- Easy to write code that parses differently than intended
- The workaround is ugly: `let len = length items in range 0 (len - 1)`

**Proposed Fix:** Review operator precedence rules. Function application should bind tighter than arithmetic.

**Effort:** Medium (parser changes)

---

## P1: High Priority Issues

### 3. No Descending Range Support

**Current Behavior:**
```avon
range 5 1    # => [] (empty!)
[5..1]       # => [] (empty!)
```

**Expected:** `[5, 4, 3, 2, 1]` or at least an error

**Impact:**
- Silent failure (returns empty list)
- Workaround requires `range 1 5 -> reverse`

**Proposed Fixes (choose one):**

A) **Auto-detect direction:**
```avon
range 5 1  # => [5, 4, 3, 2, 1]
```

B) **Add step parameter:**
```avon
range 5 1 -1  # => [5, 4, 3, 2, 1]
```

C) **Add `range_down` function:**
```avon
range_down 5 1  # => [5, 4, 3, 2, 1]
```

**Recommendation:** Option A (auto-detect) - most intuitive

**Effort:** Low (modify range builtin)

---

### 4. ✅ IMPLEMENTED: `contains` Now Works on Lists

**Status:** Fixed! `contains` is now overloaded for both strings and lists.

**Previous Behavior:**
```avon
contains 3 [1, 2, 3]  # ERROR: expected string
```

**New Behavior:**
```avon
contains 3 [1, 2, 3]          # => true (list membership)
contains "wor" "hello world"  # => true (substring)
contains "apple" ["banana"]   # => false
```

---

### 5. `min`/`max` Only Take Lists, Not Two Arguments

**Current Behavior:**
```avon
min 3 7  # ERROR: expected list
```

**Expected:** `min 3 7  # => 3`

**Impact:**
- Common pattern `min a b` doesn't work
- Must use `min [a, b]` which is awkward

**Proposed Fix:** Overload to accept either:
```avon
min 3 7           # => 3
min [3, 7, 1, 5]  # => 1
```

**Effort:** Low (modify min/max builtins)

---

### 6. No Negative Indexing

**Current Behavior:**
```avon
nth (neg 1) [1, 2, 3]  # => None
```

**Expected:** `3` (last element)

**Impact:**
- Common pattern from Python/Ruby doesn't work
- Must use `last` or `nth (length list - 1) list`

**Proposed Fix:** Support negative indices in `nth`:
```avon
nth -1 [1, 2, 3]  # => 3
nth -2 [1, 2, 3]  # => 2
```

**Effort:** Low (modify nth builtin)

---

### 7. No `has_key` / `contains_key` for Dicts

**Current Behavior:**
```avon
# Must use awkward workaround:
(find (\k k == "a") (keys dict)) != none
```

**Expected:** `has_key "a" dict  # => true`

**Impact:**
- Very common operation requires 3-4 function calls
- Easy to get wrong (precedence issues with `!= none`)

**Proposed Fix:** Add `has_key` builtin:
```avon
has_key "a" {a: 1, b: 2}  # => true
has_key "c" {a: 1, b: 2}  # => false
```

**Effort:** Very low (simple new builtin)

---

## P2: Medium Priority Issues

### 8. ✅ IMPLEMENTED: Division Now Returns Float

**Status:** Fixed! `/` now always returns float, `//` added for integer division.

**Previous Behavior:**
```avon
10 / 3  # => 3 (integer division)
```

**New Behavior:**
```avon
10 / 3   # => 3.333... (always float)
10 // 3  # => 3 (integer/floor division)
-7 // 2  # => -4 (floors toward -∞)
```

---

### 9. ✅ IMPLEMENTED: Power Operator Added

**Status:** Fixed! `**` operator added with right-associative precedence.

**Previous Behavior:**
```avon
2 ^ 8  # ERROR: expected function
```

**New Behavior:**
```avon
2 ** 8       # => 256
2 ** 3 ** 2  # => 512 (right-associative: 2 ** (3 ** 2))
4.0 ** 0.5   # => 2.0 (works with floats)
```

---

### 10. String `length` Counts Bytes, Not Characters

**Current Behavior:**
```avon
length "café"  # => 5 (not 4!)
length "👋"    # => 4 (not 1!)
```

**Impact:**
- Surprising for anyone working with Unicode
- Must use `length (chars str)` for character count

**Proposed Fixes:**

A) **Make `length` count characters** (breaking change)

B) **Add `byte_length` for current behavior, change `length`**

C) **Add `char_count` function, keep `length` as bytes** (document clearly)

**Recommendation:** Option C (non-breaking, explicit)

**Effort:** Very low (add new builtin)

---

### 11. Variable Shadowing Forbidden

**Current Behavior:**
```avon
let x = 5 in let x = 10 in x  # ERROR: variable already defined
```

**Impact:**
- Forces unique variable names
- Makes refactoring harder
- Unusual restriction (most functional languages allow shadowing)

**Discussion:** This might be intentional for clarity. Consider making it optional or allowing in nested scopes.

**Effort:** Medium (scope handling changes)

---

### 12. No `get_or` / `get_default` for Dicts

**Current Behavior:**
```avon
get "missing" dict  # => None or error
# No way to provide default
```

**Expected:**
```avon
get_or "key" "default" dict  # => value or "default"
```

**Proposed Fix:** Add `get_or` builtin

**Effort:** Very low

---

## P3: Low Priority (Nice-to-Have)

### 13. No Function Composition Operator

**Current:** Use pipes `5 -> f -> g`

**Nice to have:** `f >> g` or `compose f g`

**Effort:** Low

---

### 14. No `zip_longest` / `zip_with_default`

**Current:** `zip` truncates to shorter list

**Nice to have:** Option to fill with default value

**Effort:** Low

---

### 15. No `index_of` for Lists

**Current:** Must use filter/find patterns

**Nice to have:** `index_of 3 [1, 2, 3, 4]  # => 2`

**Effort:** Very low

---

## Summary: Recommended Priority Order

### ✅ Implemented (Complete)
1. ✅ **Short-circuit `&&`/`||`** - Now properly short-circuits
2. ✅ **Power operator `**`** - Right-associative, higher precedence than `*`
3. ✅ **Float division `/`** - Always returns float
4. ✅ **Integer division `//`** - Floor division toward -∞
5. ✅ **`contains` for lists** - Element membership check

### Quick Wins (Very Low Effort, High Impact)
1. Add `has_key` for dicts
2. ~~Overload `contains` for lists~~ ✅ Done
3. Overload `min`/`max` for two arguments
4. Add `get_or` for dicts with default
5. Add `char_count` function

### Important Fixes (Low-Medium Effort)
6. ~~**Implement short-circuit `&&`/`||`**~~ ✅ Done
7. Auto-detect descending ranges
8. Support negative indexing in `nth`

### Consider for Future
9. Review operator precedence
10. ~~Add power operator `^` or `**`~~ ✅ Done
11. Reconsider variable shadowing rules
12. Add `index_of` for lists

---

## Implementation Notes

Most of these fixes are localized to:
- `src/eval/builtins.rs` - for new/modified builtins
- `src/eval/mod.rs` - for short-circuit and operators
- `src/parser.rs` - for new operators like `**`
- `src/lexer.rs` - for tokenizing new operators
- `src/common.rs` - for token definitions

The implemented changes have comprehensive unit tests in `src/main.rs`.
