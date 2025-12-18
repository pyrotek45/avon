# Avon Gotchas and Tips Report

This report documents experimentally verified behaviors, gotchas, and tips for Avon developers. All findings have been tested with `avon run`.

**Note:** The key findings from this report have been incorporated into `tutorial/TUTORIAL.md`:
- Gotchas 19-28 added to the Gotchas section
- New "Tips and Tricks" section added
- Fixed incorrect claim about `&&`/`||` short-circuit evaluation

---

## Table of Contents

1. [Gotchas (Surprising Behaviors)](#gotchas-surprising-behaviors)
2. [Tips and Tricks](#tips-and-tricks)
3. [Common Patterns](#common-patterns)
4. [Edge Cases](#edge-cases)

---

## Gotchas (Surprising Behaviors)

### Gotcha: Boolean Strictness - No Truthy/Falsy Values

Unlike JavaScript or Python, Avon does **not** treat 0, empty strings, or empty lists as falsy:

```avon
# These ALL produce errors!
if 0 then "yes" else "no"       # ERROR: expected bool, found 0
if "" then "yes" else "no"      # ERROR: expected bool
if [] then "yes" else "no"      # ERROR: expected bool

# You must use explicit boolean expressions
if length [] == 0 then "empty" else "not empty"  # ✓ Works
```

### Gotcha: String Length Counts Bytes, Not Characters

The `length` function counts UTF-8 bytes, not visible characters:

```avon
length "hello"   # => 5 (as expected)
length "café"    # => 5 (not 4! - é is 2 bytes)
length "👋"      # => 4 (emoji is 4 bytes)
```

**Tip:** Use `chars` then `length` for character count:
```avon
length (chars "café")  # => 4 (correct character count)
```

### Gotcha: Variable Shadowing is Forbidden in Same Scope

Avon does not allow re-binding variables in the same scope:

```avon
# ERROR: variable 'x' is already defined in this scope
let x = 5 in let x = 10 in x

# Instead, use different names or nested scopes
let x = 5 in let y = x + 5 in y  # ✓ Works
```

### Gotcha: Range with Start > End Returns Empty List

Descending ranges don't work automatically (applies to both `range` function and `[a..b]` shorthand):

```avon
range 1 5    # => [1, 2, 3, 4, 5]
[1..5]       # => [1, 2, 3, 4, 5] (shorthand syntax)
range 5 1    # => [] (empty, not [5, 4, 3, 2, 1]!)
[5..1]       # => [] (shorthand also returns empty)
range 5 5    # => [5] (single element works)
```

**Tip:** For descending ranges, generate ascending then reverse:
```avon
range 1 5 -> reverse  # => [5, 4, 3, 2, 1]
[1..5] -> reverse     # => [5, 4, 3, 2, 1]
```

### Note: Division Semantics

Division now consistently returns floats, with a separate operator for integer division:

```avon
10 / 3      # => 3.333... (always returns float)
10 / 5      # => 2.0 (still a float, even for exact division)
10 // 3     # => 3 (integer/floor division, toward -∞)
-7 // 2     # => -4 (floors toward negative infinity)
```

**Tip:** Use `//` for integer division when you need whole numbers.

### Gotcha: Floating Point Precision

Standard IEEE 754 floating-point precision issues apply:

```avon
0.1 + 0.2        # => 0.30000000000000004
0.1 + 0.2 == 0.3 # => false!
```

**Tip:** For financial or precise calculations, work with integers (cents instead of dollars).

### Gotcha: Negative Modulo Follows Sign of Dividend

```avon
7 % 3     # => 1
-7 % 3    # => -1 (not 2!)
7 % -3    # => 1
```

### Gotcha: Comparison Type Strictness

Types must match for comparison:

```avon
1 == "1"    # ERROR: cannot compare Number with String
[1,2] < [1,3]  # ERROR: lists only support == and !=
```

### Gotcha: List Functions Return None on Missing Elements

Functions gracefully return `None` instead of erroring:

```avon
head []         # => None (not an error)
nth 10 [1,2,3]  # => None (out of bounds)
nth (neg 1) [1,2,3]  # => None (negative index doesn't wrap)
```

**Tip:** Use `last` for the last element instead of negative indexing.

### Gotcha: `get` Function Argument Order

The `get` function takes arguments in an unexpected order for piping:

```avon
# This doesn't work with pipe:
{a: 1} -> get "a"  # ERROR

# Use dot notation instead:
let d = {a: 1} in d.a  # => 1

# Or set takes dict first:
set {a: 1} "b" 2  # => {a: 1, b: 2}
```

### Gotcha: `repeat` Takes String First, Count Second

```avon
repeat 3 "x"    # ERROR!
repeat "x" 3    # => "xxx" ✓
```

### Gotcha: `min`/`max` Operate on Lists, Not Two Arguments

```avon
min 3 7           # ERROR: expected list
min [3, 7, 1, 5]  # => 1 ✓
max [3, 7, 1, 5]  # => 7 ✓
```

### Note: `contains` Works for Strings AND Lists

The `contains` function now works for both substring checks and list membership:

```avon
# String usage:
contains "hello world" "world"  # => true (substring check)
"hello world" -> contains "world"  # Works with pipe

# List membership:
contains 3 [1, 2, 3, 4]  # => true (element is in list)
contains "apple" ["banana", "cherry"]  # => false

# You can also use `any` for more complex conditions:
any (\x x > 3) [1, 2, 3, 4]  # => true
```

### Gotcha: `zip` Truncates to Shorter List

```avon
zip [1, 2, 3] [4, 5]  # => [[1, 4], [2, 5]] (3 is dropped)
zip_with (\a \b a + b) [1, 2, 3] [10, 20]  # => [11, 22]
```

### Gotcha: `all` on Empty List Returns True

```avon
all (\x x > 0) []  # => true (vacuously true)
any (\x x > 0) []  # => false
```

### Gotcha: `pfold` Requires Associative Operations

Parallel fold may split work differently:

```avon
# Sequential fold: left-to-right
fold (\acc \x acc - x) 10 [1, 2, 3]  # => 4

# Parallel fold: non-deterministic order!
pfold (\acc \x acc - x) 10 [1, 2, 3]  # => unpredictable!

# Only use pfold with associative operations (addition, multiplication, max, min)
pfold (\acc \x acc + x) 0 [1, 2, 3]  # => 6 ✓
```

### Gotcha: `split` Returns Separator Characters on Empty String

```avon
"" -> split ","     # May return unexpected results
"" -> chars         # => [] (empty list)
"abc" -> split ""   # => [] (empty separator)
```

### Gotcha: Large Integer Overflow

Very large integers may overflow:

```avon
999999999999999999999  # => 0 (overflow!)
pow 2 62               # => 4611686018427387904 (works)
pow 2 63               # May overflow to negative
```

### Note: Power Operator `**` (Not `^`)

The power operator is `**`, not `^`:

```avon
2 ** 8       # => 256 (power operator)
2 ** 3 ** 2  # => 512 (right-associative: 2 ** (3 ** 2))
pow 2 8      # => 256 (function form also works)
2 ^ 8        # ERROR: ^ is not recognized
```

### Note: Logical Operators Now Short-Circuit

`&&` and `||` properly short-circuit evaluation:

```avon
false && (1 / 0 > 0)  # => false (right side NOT evaluated)
true || (1 / 0 > 0)   # => true (right side NOT evaluated)

# Safe guard pattern:
x != 0 && (y / x > 1)  # Safe - won't divide by zero if x == 0
```

---

## Tips and Tricks

### Tip: Use Pipe Chains for Readable Data Transformation

```avon
[1, 2, 3, 4, 5]
  -> map (\x x * 2)
  -> filter (\x x > 5)
  -> fold (\a \b a + b) 0
# => 18
```

### Tip: Dot Notation for Dict Access

```avon
let config = {
  server: {
    host: "localhost",
    port: 8080
  }
} in config.server.port  # => 8080
```

### Tip: Use `chars` for String Character Operations

```avon
# Reverse a string
"hello" -> chars -> reverse -> fold (\a \b a ++ b) ""

# Get character at index
nth 0 (chars "hello")  # => "h"
```

### Tip: Check List Membership with `any`

```avon
let has_item = \item \list any (\x x == item) list
in has_item 3 [1, 2, 3, 4]  # => true
```

### Tip: Safe Dict Key Check

```avon
let check = \key \dict (find (\k k == key) (keys dict)) != none
in check "a" {a: 1, b: 2}  # => true
```

### Tip: Function Composition via Pipes

```avon
let double = \x x * 2
let inc = \x x + 1
in 5 -> double -> inc  # => 11
```

### Tip: Use `typeof` for Runtime Type Checking

```avon
typeof 42        # => "Number"
typeof "hello"   # => "String"
typeof [1, 2]    # => "List"
typeof {a: 1}    # => "Dict"
typeof none      # => "None"
typeof true      # => "Bool"
typeof (\x x)    # => "Function"
```

### Tip: Use `is_*` Functions for Type Guards

```avon
is_list [1, 2, 3]   # => true
is_dict {a: 1}      # => true
is_string "hello"   # => true
is_number 42        # => true
is_none none        # => true
is_bool true        # => true
```

### Tip: Use `flatten` to Concatenate Lists

```avon
flatten [[1, 2], [3, 4], [5]]  # => [1, 2, 3, 4, 5]
```

### Tip: Group and Partition Data

```avon
# Partition into matching/non-matching
partition (\x x > 3) [1, 2, 3, 4, 5]  # => [[4, 5], [1, 2, 3]]

# Group by key function
group_by (\x x % 2) [1, 2, 3, 4, 5]   # => {0: [2, 4], 1: [1, 3, 5]}
```

### Tip: Use `neg` for Negative Numbers in Expressions

```avon
take (neg 1) [1, 2, 3]  # Doesn't work as expected, but...
0 - 5                   # => -5
neg 5                   # => -5
```

### Tip: Use `last` Instead of Negative Indexing

```avon
last [1, 2, 3, 4, 5]  # => 5
nth (neg 1) [1, 2, 3] # => None (doesn't work!)
```

### Tip: `take` with Negative Returns Full List

```avon
[1, 2, 3] -> take (neg 1)  # => [1, 2, 3] (returns everything)
take 0 [1, 2, 3]           # => []
take 10 [1, 2, 3]          # => [1, 2, 3] (safe, returns what's available)
```

### Tip: Unicode Characters Work in `chars`

```avon
chars "αβγ"   # => ["α", "β", "γ"]
chars "👋🌍"  # => ["👋", "🌍"] (emoji handled correctly)
```

---

## Common Patterns

### Pattern: Safe Division

```avon
let safe_div = \a \b if b == 0 then none else a / b
in safe_div 10 0  # => None instead of error
```

### Pattern: Default Value for None

```avon
let with_default = \default \value if value == none then default else value
let result = find (\x x > 100) [1, 2, 3]
in with_default 0 result  # => 0
```

### Pattern: Map with Index

```avon
let items = ["a", "b", "c"]
let len = length items
in zip (range 0 (len - 1)) items  # => [[0, "a"], [1, "b"], [2, "c"]]
```

### Pattern: Dict from Lists

```avon
let keys_list = ["a", "b", "c"]
let vals_list = [1, 2, 3]
let pairs = zip keys_list vals_list
in fold (\d \p set d (head p) (last p)) {} pairs
```

### Pattern: Extract Nested Data

```avon
let users = [
  {name: "Alice", age: 30},
  {name: "Bob", age: 25}
]
in users -> map (\u u.name)  # => ["Alice", "Bob"]
```

---

## Edge Cases

### Empty Collections

```avon
head []          # => None
tail []          # => []
length []        # => 0
sum []           # => 0
max []           # => None
keys {}          # => []
values {}        # => []
"" -> chars      # => []
```

### Single Element

```avon
head [42]        # => 42
tail [42]        # => []
range 5 5        # => [5]
```

### Type Conversion

```avon
to_int 3.7       # => 3 (truncates toward zero)
to_int "42"      # => 42
to_int "3.7"     # => ERROR
to_float 42      # => 42.0
to_string 42     # => "42"
to_string none   # => "None"
```

---

## Summary of Key Points

1. **No truthy/falsy** - Use explicit boolean expressions
2. **String length = bytes** - Use `chars` for character count
3. **No variable shadowing** - Use unique names
4. **Range is ascending only** - Reverse for descending
5. **Division `/` always returns float** - Use `//` for integer division
6. **Comparison types must match** - No cross-type comparison
7. **`&&`/`||` do short-circuit** - Safe to use guards like `x != 0 && (y / x > 1)`
8. **pfold needs associative ops** - Subtraction/division won't work correctly
9. **`contains` works for strings AND lists** - `contains 3 [1,2,3]` returns true
10. **Use dot notation for dicts** - `d.key` is cleaner than `get`
11. **Power operator `**` is right-associative** - `2 ** 3 ** 2` = 512
