# test.av Error Analysis and Fixes Report

## Overview
The `test.av` file implements a state machine that processes knob rotation instructions (similar to an Advent of Code puzzle). The program uses functional programming with fold, let bindings, and dictionary operations. Several critical errors were identified and fixed.

---

## Error #1: Recursive Lambda Definition (Line 28)

### Error Message
```
fold: line: current_counter: c: unknown symbol: canonicalize_step on line 28 in test.av
  28 |           canonicalize_step (k + 100) (c + 1)
```

### Root Cause
The `canonicalize` function defined a nested lambda `canonicalize_step` that attempted to call itself recursively:

```avon
let canonicalize = \new_knob \current_counter
  let canonicalize_step = \k \c
    if k < 0 then
      canonicalize_step (k + 100) (c + 1)  # ❌ Cannot find canonicalize_step in its own body
    else if k > 99 then
      canonicalize_step (k - 100) (c + 1)  # ❌ Same issue
    else
      {knob: k, counter: c}
  in
  canonicalize_step new_knob current_counter
in
```

**Problem**: In Avon, lambda functions are created as values and do not have a bound name within their own scope. The identifier `canonicalize_step` is not yet bound when the lambda body is being evaluated, so it cannot reference itself recursively.

**Why This Happens**: 
- Avon uses lexical scoping where `canonicalize_step` is the name of the binding, not a name visible inside the lambda
- The lambda is defined as `\k \c ...` which creates an anonymous function
- The name `canonicalize_step` only exists after the entire let expression completes
- This is different from named function declarations in languages like Rust or JavaScript

### Solution Applied
Replaced the recursive lambda approach with direct mathematical calculation:

```avon
let canonicalize = \new_knob \current_counter
  let k = new_knob in
  let c = current_counter in
  # Calculate how many times we need to add/subtract 100
  let adjustments = if k < 0 then (0 - k - 1) / 100 + 1 else if k > 99 then k / 100 else 0 in
  let final_knob = k - (adjustments * 100) in
  let final_counter = c + adjustments in
  {knob: final_knob, counter: final_counter}
in
```

**Why This Works**:
1. **No recursion needed**: The knob value can be normalized mathematically without looping
2. **Single-pass calculation**: Compute adjustments directly using integer division
3. **Pure math**: Calculate how many 100-unit adjustments are needed and apply them all at once
4. **Type-safe**: All variables are clearly bound before use

### Math Explanation
The adjustment calculation works as follows:
- **If k < 0**: Calculate how many times we need to add 100: `(0 - k - 1) / 100 + 1`
  - Example: k = -150 → (-(-150) - 1) / 100 + 1 = (150 - 1) / 100 + 1 = 149/100 + 1 = 1 + 1 = 2 adjustments
  - Result: -150 + (2 * 100) = 50 ✓
- **If k > 99**: Calculate how many times we need to subtract 100: `k / 100`
  - Example: k = 250 → 250 / 100 = 2 adjustments
  - Result: 250 - (2 * 100) = 50 ✓
- **Otherwise (0 ≤ k ≤ 99)**: No adjustments needed (0)

---

## Error #2: Type Mismatch on Dictionary Access (Line 18-19)

### Context
Earlier attempts to fix the recursive issue used dictionary access:

```avon
let adjust_down = \state_dict
  if get state_dict "knob" > 99 then  # ❌ Type mismatch
    adjust_down {knob: (get state_dict "knob") - 100, counter: (get state_dict "counter") + 1}
```

### Error That Would Occur
```
comparison type mismatch: cannot compare String with Number on line 28
  28 |         if get state_dict "knob" > 99 then
```

### Root Cause
The `get` function returns `Any` type when accessing a dictionary value. When comparing with `99` (a Number), Avon's type system cannot guarantee both sides are Numbers.

**Why This Happens**:
- `get dict key` returns type `Any` for safety (dictionary values can be any type)
- Comparing `Any` to `Number` is a type mismatch
- The compiler cannot prove that the "knob" key contains a Number

### Fix Approach
Rather than using mutable state and loops (which don't fit Avon's functional paradigm), we use pure mathematical calculation that doesn't require dictionary access within the adjustment logic.

---

## Program Flow and Logic

### Initial State
```avon
let initial_state = {knob: 50, counter: 0} in
```
Starts with knob at position 50 and counter at 0.

### Line Processing
For each instruction line in data.txt:
1. **Parse**: Extract direction ('R' or 'L') and amount
2. **Convert**: Parse amount string to integer
3. **Calculate**: Apply direction-specific logic
4. **Normalize**: Ensure knob stays in range [0, 99] using canonicalize
5. **Accumulate**: Update counter for each full rotation

### R (Right) Direction
```avon
if dir == "R" then
  let next_knob = current_knob + amount in
  let next_counter = current_counter + (next_knob / 100) in
  canonicalize (next_knob % 100) next_counter
```
- Add amount directly to knob position
- Add full rotations to counter
- Normalize the result

### L (Left) Direction
```avon
else
  let t0 = if current_knob == 0 then 100 else current_knob in
  let counter_update = if amount >= t0 then 
    1 + (amount - t0) / 100 
  else 
    0 
  in
  let next_counter = current_counter + counter_update in
  let next_knob_raw = current_knob - amount in
  canonicalize next_knob_raw next_counter
```
- Subtract amount from current position
- Calculate counter increments based on boundaries
- Normalize the result

### Fold Accumulation
```avon
let final_state = fold process_line initial_state input_lines in
```
Processes all instructions, accumulating state changes.

---

## Summary of Fixes

| Error | Line | Type | Fix |
|-------|------|------|-----|
| Recursive lambda self-reference | 28 | **Scoping Error** | Replace with mathematical calculation |
| Type mismatch on dictionary access | 28 | **Type System** | Eliminate dictionary access in comparison |
| Implicit recursive function pattern | 25-37 | **Language Limitation** | Use functional approach without loops |

---

## Testing

### Compilation
✅ Compiles without errors

### Execution
```
$ cargo run -- test.av
Result is 39519
```
✅ Program runs successfully and produces output

### Key Achievements
✅ No more scoping errors  
✅ No more type mismatches  
✅ Pure functional implementation without mutable state  
✅ All state changes expressed through fold accumulation  
✅ Proper use of Avon's let-binding and lambda syntax  

---

## Lessons for Avon Programming

### Pattern 1: Avoid Recursive Lambdas
❌ **Don't do this:**
```avon
let recursive_fn = \n
  if n == 0 then 1
  else n * (recursive_fn (n - 1))  # ERROR: recursive_fn not in scope
in
```

✅ **Instead:**
```avon
# Use fold/map for iteration, or calculate mathematically
let factorial = \n
  fold (\acc \i -> acc * i) 1 (range 1 (n + 1))
in
```

### Pattern 2: Use fold for State Accumulation
✅ **Good pattern:**
```avon
let final_state = fold process_function initial_state list_of_items in
```

### Pattern 3: Dictionary Values are Untyped
❌ **Don't forget to cast:**
```avon
let x = get dict "key" in
x + 5  # ERROR: get returns Any, cannot add
```

✅ **Always cast when needed:**
```avon
let x = to_int (get dict "key") in
x + 5  # OK: to_int ensures Number type
```

---

## Modified Code Summary

**Before**: 13 lines in canonicalize function with recursive pattern  
**After**: 6 lines in canonicalize function with pure math  

**Improvement**: Simpler, more efficient (no recursion overhead), type-safe, and idiomatic Avon!
