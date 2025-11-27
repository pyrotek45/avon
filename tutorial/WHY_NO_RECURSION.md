# Why Avon Does Not Support Recursion

## Overview

Avon intentionally does **not** support recursive functions (functions that call themselves). This document explains the design rationale and provides guidance on achieving recursive-like behavior using Avon's built-in iteration functions.

## Design Rationale

### 1. Simplicity

Recursion adds significant complexity to language implementation:
- Requires tracking recursion depth
- Needs stack management
- Requires base case detection
- Adds complexity to error messages

By removing recursion, Avon's evaluator is simpler, easier to understand, and easier to maintain.

### 2. Performance

Recursion has inherent performance costs:
- Function call overhead for each recursive call
- Stack frame allocation
- Recursion depth tracking overhead
- Potential for stack overflow

Iterative solutions using `fold`, `map`, and `filter` are:
- More efficient (no function call overhead per iteration)
- More predictable (no stack growth)
- Better optimized by the evaluator

### 3. Safety

Recursion introduces safety concerns:
- Risk of infinite recursion bugs
- Stack overflow from deep recursion
- Difficult to detect problematic patterns statically

Without recursion:
- No risk of infinite loops from recursion
- No stack overflow concerns
- Predictable execution behavior
- Easier to reason about program behavior

### 4. Clarity

Recursion can make code harder to understand:
- Requires mental stack tracking
- Base cases can be subtle
- Error messages for infinite recursion are confusing

Iterative solutions are:
- More explicit about the iteration pattern
- Easier to understand at a glance
- Clearer error messages (unknown symbol vs infinite recursion)

## How It Works

When a function is defined in Avon, it captures its environment (closure) but does **not** add itself to that environment. This means:

```avon
let factorial = \n
  if n <= 1 then 1 else n * (factorial (n - 1)) in
factorial 5
# Error: unknown symbol: factorial
```

The function `factorial` cannot reference itself because it's not in its own scope. This is by design.

## Alternatives: Using Iteration

Avon provides powerful built-in functions for iteration that can replace most recursive patterns:

### Factorial

**Recursive (not supported):**
```avon
let factorial = \n
  if n <= 1 then 1 else n * (factorial (n - 1))
```

**Iterative (supported):**
```avon
let factorial = \n
  fold (\acc \x acc * x) 1 [1 .. n]
```

### Sum List

**Recursive (not supported):**
```avon
let sum_list = \list
  if (length list) == 0 then 0
  else (head list) + (sum_list (tail list))
```

**Iterative (supported):**
```avon
let sum_list = \list
  fold (\acc \x acc + x) 0 list
```

### Countdown

**Recursive (not supported):**
```avon
let countdown = \n
  if n <= 0 then [] else [n] + (countdown (n - 1))
```

**Iterative (supported):**
```avon
let countdown = \n
  reverse [1 .. n]
```

### Filtering and Mapping

**Recursive (not supported):**
```avon
let filter_positive = \list
  if (length list) == 0 then []
  else if (head list) > 0 then [head list] + (filter_positive (tail list))
  else filter_positive (tail list)
```

**Iterative (supported):**
```avon
let filter_positive = \list
  filter (\x x > 0) list
```

## Benefits of This Approach

1. **Performance**: Iterative solutions are faster
2. **Readability**: Intent is clearer with `fold`, `map`, `filter`
3. **Safety**: No risk of infinite recursion
4. **Simplicity**: Language implementation is simpler
5. **Consistency**: All iteration uses the same patterns

## Conclusion

While recursion is a powerful tool in some languages, Avon's design philosophy prioritizes simplicity, performance, and safety. The built-in iteration functions (`fold`, `map`, `filter`, etc.) provide all the power needed for most use cases while being more efficient, safer, and easier to understand.

If you find yourself wanting recursion, consider:
1. Can this be solved with `fold`?
2. Can this be solved with `map` or `filter`?
3. Can this be solved with ranges and list operations?

In most cases, the answer is yes, and the iterative solution will be better.
