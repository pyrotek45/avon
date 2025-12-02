# New Avon Builtins for AoC Challenges

Based on analysis of the BUILTIN_IMPROVEMENTS.md report and common AoC patterns, here are the most impactful builtins to add next. These are prioritized by:
1. **Frequency** - How often they appear across AoC challenges
2. **Impact** - How much they simplify code
3. **Effort** - Estimated implementation complexity (Low/Medium/High)

---

## TIER 1: High Frequency + Low Effort (IMPLEMENT FIRST)

### 1. `min(list)` and `max(list)` - Find minimum/maximum values
**Arity:** 1 each  
**Priority:** CRITICAL - Used in almost every AoC challenge

**Examples:**
```avon
min [3, 1, 4, 1, 5, 9]           # → 1
max [3, 1, 4, 1, 5, 9]           # → 9
min [0, -5, 3, -2]               # → -5
```

**Current workaround:** Complex fold chains
```avon
fold (\a \x if x < a then x else a) 999999 list
```

**Why important:**
- Finding max/min is the #1 operation in AoC
- Day 1: Find max values in groups
- Day 4: Find min/max of signal ranges
- Day 8: Find tallest trees in grids

**Rust impl hint:** Use `.min()` and `.max()` from iterator trait

---

### 2. `sum(list)` - Sum all numbers in list
**Arity:** 1  
**Priority:** CRITICAL - Used constantly

**Examples:**
```avon
sum [1, 2, 3, 4, 5]              # → 15
sum []                           # → 0
sum [10]                         # → 10
```

**Current workaround:**
```avon
fold (\a \x a + x) 0 list
```

**Why important:**
- Second most common operation after min/max
- Day 1: Sum all values
- Day 3: Sum partial values
- Day 5: Sum distances

**Rust impl hint:** Use `.sum::<i64>()`

---

### 3. `all(predicate, list)` and `any(predicate, list)` - Check all/any elements
**Arity:** 2 each  
**Priority:** HIGH - Common in validation/checking

**Examples:**
```avon
all (\x x > 0) [1, 2, 3, 4]      # → true
all (\x x > 0) [1, -2, 3, 4]     # → false
any (\x x < 0) [1, 2, -3, 4]     # → true
any (\x x < 0) [1, 2, 3, 4]      # → false
```

**Current workaround:**
```avon
let check_all = \fn \list
  fold (\a \x a && fn x) true list
in
check_all (\x x > 0) [1, 2, 3]
```

**Why important:**
- Validation is critical in AoC
- Day 2: Check if list satisfies conditions
- Day 6: Check if all values in range
- Day 10: Validate grid properties

---

### 4. `count(predicate, list)` - Count matching elements
**Arity:** 2  
**Priority:** HIGH - Very common counting operation

**Examples:**
```avon
count (\x x > 5) [1, 6, 3, 8, 5]  # → 2
count (\x x == "a") ["a","b","a"] # → 2
count (\_ true) [1, 2, 3]         # → 3 (same as length)
```

**Current workaround:**
```avon
length (filter (\x x > 5) list)
```

**Why important:**
- Counting frequencies is core to many challenges
- Day 2: Count invalid ranges
- Day 3: Count overlaps
- Day 7: Count valid combinations

---

### 5. `find(predicate, list)` - Find first matching element
**Arity:** 2  
**Priority:** MEDIUM-HIGH - Common search operation

**Examples:**
```avon
find (\x x > 5) [1, 2, 6, 3, 8]   # → 6
find (\x x == "x") ["a", "b", "c"] # → none
```

**Current workaround:**
```avon
head (filter (\x x > 5) list)     # Crashes if none match
```

**Why important:**
- Searching is fundamental
- Day 1: Find matching pairs
- Day 4: Find first occurrence
- Day 8: Find path from start

---

## TIER 2: Medium Frequency + Medium Effort (IMPLEMENT SECOND)

### 6. `group_by(key_fn, list)` - Group elements by key
**Arity:** 2  
**Priority:** HIGH - Appears in ~30% of AoC challenges

**Examples:**
```avon
# Group by position
group_by (\x x % 3) [0,1,2,3,4,5]
# → {0: [0, 3], 1: [1, 4], 2: [2, 5]}

# Group by type
group_by (\x if x > 5 then "high" else "low") [1,6,3,8]
# → {"low": [1, 3], "high": [6, 8]}
```

**Why important:**
- Grouping is essential for many challenges
- Day 1: Group elves' food by elf
- Day 3: Group supplies by section
- Day 7: Group commands by directory

**Implementation:** More complex, returns Dict or List[List]

---

### 7. `product(list)` - Multiply all numbers
**Arity:** 1  
**Priority:** MEDIUM - Less common than sum but still frequent

**Examples:**
```avon
product [1, 2, 3, 4]             # → 24
product [5, 2]                   # → 10
product []                       # → 1
```

**Current workaround:**
```avon
fold (\a \x a * x) 1 list
```

**Why important:**
- Used in permutation/combination problems
- Day 1: Calculate totals
- Day 8: Calculate signal strength
- Day 9: Calculate rope physics

---

### 8. `abs_diff(a, b)` - Absolute difference
**Arity:** 2  
**Priority:** MEDIUM - Common distance metric

**Examples:**
```avon
abs_diff 5 2                     # → 3
abs_diff -5 2                    # → 7
abs_diff 100 100                 # → 0
```

**Current workaround:**
```avon
let a = 5 in let b = 2 in
if a > b then a - b else b - a
```

**Why important:**
- Distance calculations everywhere
- Day 2: Distance metrics
- Day 4: Range overlaps
- Day 9: Rope distance

---

### 9. `gcd(a, b)` - Greatest Common Divisor
**Arity:** 2  
**Priority:** MEDIUM - Appears in algorithmic challenges

**Examples:**
```avon
gcd 48 18                        # → 6
gcd 100 50                       # → 50
gcd 7 3                          # → 1
```

**Why important:**
- Used in modular arithmetic
- Day 1: Cycle detection
- Day 8: Finding LCM
- Day 11: Divisibility patterns

---

### 10. `lcm(a, b)` - Least Common Multiple
**Arity:** 2  
**Priority:** MEDIUM - Often paired with gcd

**Examples:**
```avon
lcm 4 6                          # → 12
lcm 12 18                        # → 36
lcm 7 5                          # → 35
```

**Use case:** Combine with gcd: `lcm a b = (a * b) / gcd a b`

---

## TIER 3: High-Value Utilities (IMPLEMENT THIRD)

### 11. `in_range(value, start, end)` - Check if value in range
**Arity:** 3  
**Priority:** MEDIUM - Common validation

**Examples:**
```avon
in_range 5 1 10                  # → true
in_range 0 1 10                  # → false
in_range 10 1 10                 # → true (inclusive end)
```

**Alternative:** `between(val, start, end)` - non-inclusive

---

### 12. `clamp(value, min_val, max_val)` - Constrain value to range
**Arity:** 3  
**Priority:** LOW-MEDIUM - Physics/simulation problems

**Examples:**
```avon
clamp 15 0 10                    # → 10
clamp -5 0 10                    # → 0
clamp 5 0 10                     # → 5
```

---

### 13. `swap(list, i, j)` - Swap elements at indices
**Arity:** 3  
**Priority:** MEDIUM - Sorting and rearrangement

**Examples:**
```avon
swap [1, 2, 3] 0 2               # → [3, 2, 1]
swap ["a", "b", "c"] 1 2         # → ["a", "c", "b"]
```

**Use case:** Implement sorting algorithms, permutations

---

### 14. `remove(list, index)` - Remove element at index
**Arity:** 2  
**Priority:** MEDIUM - List manipulation

**Examples:**
```avon
remove [1, 2, 3, 4] 1            # → [1, 3, 4]
remove ["a", "b", "c"] 0         # → ["b", "c"]
remove [1] 0                     # → []
```

**Current workaround:**
```avon
concat (take list index) (drop list (index + 1))
```

---

### 15. `contains_all(haystack, needle_list)` - Check if all needles in haystack
**Arity:** 2  
**Priority:** LOW-MEDIUM - Set operations

**Examples:**
```avon
contains_all ["a", "b", "c"] ["a", "c"]    # → true
contains_all ["a", "b", "c"] ["a", "d"]    # → false
```

---

## TIER 4: Advanced Algorithms (Lower Priority)

### 16. `cartesian_product(list1, list2)` - Generate all pairs
**Arity:** 2  
**Priority:** LOW - Specific use cases

**Examples:**
```avon
cartesian_product [1, 2] ["a", "b"]
# → [[1, "a"], [1, "b"], [2, "a"], [2, "b"]]
```

---

### 17. `permutations(list)` - Generate all permutations
**Arity:** 1  
**Priority:** LOW - Backtracking problems

**Examples:**
```avon
permutations [1, 2, 3]
# → [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
```

---

## Recommended Implementation Order

**Phase 1 (IMMEDIATE - ~2 hours):**
1. `sum()` - 5 min, huge impact
2. `min()` - 5 min, huge impact  
3. `max()` - 5 min, huge impact
4. `all()` - 10 min
5. `any()` - 10 min

**Phase 2 (NEXT SESSION - ~3 hours):**
6. `count()` - 10 min
7. `find()` - 15 min
8. `abs_diff()` - 5 min
9. `product()` - 5 min
10. `gcd()` - 15 min
11. `lcm()` - 5 min (depends on gcd)

**Phase 3 (FUTURE - ~4 hours):**
12. `group_by()` - 45 min (most complex)
13. `in_range()` - 10 min
14. `clamp()` - 10 min
15. `remove()` - 15 min
16. `swap()` - 15 min

---

## Impact Analysis

### Before (Current Avon)
```avon
# Sum numbers (verbose)
fold (\a \x a + x) 0 [1,2,3,4,5]

# Find minimum (very verbose)
fold (\a \x if x < a then x else a) 99999 [3,1,4,1,5]

# Check all positive (moderate)
fold (\a \x a && (x > 0)) true [1,2,3,4]
```

### After (With Tier 1 additions)
```avon
# Sum numbers (clean)
sum [1,2,3,4,5]

# Find minimum (clean)
min [3,1,4,1,5]

# Check all positive (clean)
all (\x x > 0) [1,2,3,4]
```

### Code Reduction
- **AoC solutions:** ~30-40% fewer lines
- **Readability:** 3-4x easier to understand
- **Performance:** Optimized implementations vs. fold chains

---

## Test Cases Needed

For each new builtin, create:
1. **Happy path:** `sum [1,2,3]` → `6`
2. **Edge cases:** Empty lists, single element, negatives, zeros
3. **Type combinations:** Lists of ints, floats, mixed
4. **Error cases:** Wrong types, invalid ranges

Example test file structure:
```bash
# testing/avon/test_new_builtins.sh
test_sum_basic() {
  result=$(cargo run --quiet -- -c 'sum [1,2,3,4,5]')
  assert_equal "$result" "15" "sum basic"
}

test_sum_empty() {
  result=$(cargo run --quiet -- -c 'sum []')
  assert_equal "$result" "0" "sum empty"
}
```

---

## VSCode Extension Update Required

When implementing Phase 1, remember to update:
- **File:** `vscode/syntaxes/avon.tmLanguage.json` line ~38
- **Add to builtin list:** `sum|min|max|all|any|count|find|product|abs_diff|gcd|lcm|...`
- **Keep alphabetical order**
- **Re-package extension** after syntax changes

---

## Success Criteria

After implementing Tier 1:
- ✅ verify_norm.av can be simplified to cleaner version
- ✅ AoC solutions drop ~20-30% in line count
- ✅ All edge cases covered by tests
- ✅ VSCode syntax highlighting works
- ✅ CLI help documents all new functions

