# Avon Technical Architecture

This document explains how Avon works under the hood—the design decisions, data structures, and implementation details that make the language work.

## Table of Contents

1. [Symbol Table & Environment](#symbol-table--environment)
2. [Evaluation Model](#evaluation-model)
3. [Function Closures](#function-closures)
4. [Type System](#type-system)
5. [Performance Optimizations](#performance-optimizations)
6. [Error Handling](#error-handling)

## Symbol Table & Environment

### What is the Symbol Table?

The symbol table is a `HashMap<String, Value>` that stores all variables currently in scope. When you write:

```avon
let x = 10 in
let y = 20 in
x + y
```

The interpreter maintains a symbol table that tracks:
1. After `let x = 10`: `{x: 10}`
2. After `let y = 20`: `{x: 10, y: 20}`
3. During `x + y`: Both x and y are looked up in the table

### Stack-Based Scoping

Avon uses **stack-based scoping** with a single mutable symbol table. When you create a new scope with `let`, variables are added to the table, the expression after `in` is evaluated, then the variable is removed—no cloning:

```avon
let x = 5 in
let y = 10 in
x + y
```

Evaluation process:
1. Insert `x=5` into table: `{x: 5}`
2. Evaluate `let y = 10 in x + y`:
   - Insert `y=10` into table: `{x: 5, y: 10}`
   - Evaluate `x + y` (both available): result is `15`
   - Remove `y` from table (after `x + y` is evaluated): `{x: 5}`
   - Return `15`
3. Remove `x` from table (after inner let expression is evaluated): `{}`
4. Return result `15`

**Important: No Variable Shadowing**

Avon does **not** allow variable shadowing. If you try to redefine a variable in the same scope, you'll get an error:

```avon
let x = 5 in
let x = 10 in  # Error: variable 'x' is already defined in this scope
x
```

**Exception:** The underscore `_` can be reused (common pattern for ignoring values):
```avon
let _ = compute_value() in
let _ = another_value() in  # OK: underscore can be reused
result
```

**Why this approach?**
- Efficient: Only insert/remove operations, no cloning per binding
- Memory-light: Single mutable reference passed through evaluation
- Correct: Naturally respects lexical scoping rules
- Fast: Even 200+ let bindings complete in milliseconds
- Clear: No shadowing means no confusion about which variable is being used

### HashMap Performance

The symbol table uses Rust's `HashMap` for O(1) average lookup:

```rust
let value = symbols.get("variable_name");  // O(1) lookup
symbols.insert("name", value);              // O(1) insert
symbols.remove("name");                     // O(1) remove
```

With proper implementation, even 200+ let bindings complete in milliseconds.

## Evaluation Model

### Recursive Descent Evaluation

Avon uses recursive descent evaluation. For an expression like `1 + 2 * 3`:

```
eval(BinOp(+, 1, BinOp(*, 2, 3)))
  ├─ eval(1) → 1
  ├─ eval(BinOp(*, 2, 3))
  │   ├─ eval(2) → 2
  │   ├─ eval(3) → 3
  │   └─ 2 * 3 → 6
  └─ 1 + 6 → 7
```

Each `eval` call takes the current symbol table by mutable reference:

```rust
fn eval(expr: Expr, symbols: &mut HashMap<String, Value>, source: &str) -> Result<Value>
```

### Evaluation Order

Avon evaluates expressions strictly (eager evaluation):
- All arguments are evaluated before applying functions
- No lazy evaluation of conditionals (though `if` short-circuits the non-taken branch)

## Function Closures

### Environment Capture with Rc

When a function is created, it captures the current environment using `Rc`:

```rust
// When evaluating: \x x + factor (where factor=10)
Function {
    ident: "x",
    expr: BinOp(+, Ident("x"), Ident("factor")),
    env: Rc::new(HashMap {factor: 10, ...})
}
```

### Why Rc?

Rc (Reference Counted) allows:
- **Cheap cloning**: Cloning the Rc just increments a counter, not the HashMap
- **Shared ownership**: Multiple functions can share the same captured environment
- **Immutability**: The captured environment can't change (preventing bugs)
- **Single-threaded efficiency**: Rc is more efficient than Arc for single-threaded use cases

### Function Application

When you call a function:

```avon
let f = \x x * 2 in f 5
```

Avon:
1. Looks up `f` in symbol table → gets the Function value with captured env
2. Creates new scope from captured environment + argument binding
3. Evaluates function body with new scope
4. Returns result

The efficiency comes from Rc: we dereference the Rc and clone only the captured snapshot (one HashMap clone per function call), not the entire evaluation state.

## Recursion: By Design

**Avon uses iterative functions instead of recursion.**

This is a powerful design choice that enables Avon to handle everything from simple configs to complex data transformations. Here's why:

### Why Iteration Works Better

Avon is designed to handle **any task**—from template generation and data transformation to complex computational workflows. Iterative functions provide superior capabilities:

1. **No Infinite Loops**: Iterative functions naturally handle any dataset size without risk of hanging
2. **Memory Efficiency**: Iteration uses constant stack space, handling large datasets reliably
3. **Clarity**: Iterative patterns are easier to understand and reason about in any context
4. **Performance**: Avon's iteration functions are optimized for efficiency and predictable performance

### Powerful Iteration Functions

Avon provides **higher-order functions** (map, filter, fold) that handle any iteration pattern:

```avon
# Don't use recursion
let sumlist = \xs ...recursive call... in

# Use fold instead
let sum = fold (\acc \x acc + x) 0 numbers in

# Example: transform config list
let servers = ["web1", "web2", "web3"] in
let config = map (\name {hostname: name, port: 8080}) servers in
# This naturally handles any list size without recursion
```

### Benefits of This Design

- **Safe**: Can't accidentally infinite loop
- **Simple**: Easier to understand data transformations
- **Predictable**: No surprise stack overflows
- **Focused**: Encourages functional composition over recursive algorithms
- **Efficient**: map/filter/fold are optimized in Rust, not user code

If you need complex recursive algorithms, Avon isn't the right tool. But for templates and config generation, you don't need recursion—you need map, filter, and fold.

## Type System

### Dynamic Types

Avon uses runtime type checking. Every value has a tag:

```rust
enum Value {
    None,
    Bool(bool),
    Number(Number),
    String(String),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function { ident, expr, env },
    Template(...),
    Path(...),
}
```

### Type Errors at Runtime

Type errors are caught during evaluation, not compile time:

```avon
"hello" + 5  # Runtime error: can't concat string and number
```

Error: `+: type mismatch: expected string/list/dict/path, found number`

### Type Checking Benefits

- **Flexibility**: Can build generic functions
- **Clear errors**: Error messages show exactly what went wrong
- **Late binding**: Functions can work with multiple types (e.g., `+` for strings and numbers)

## Performance Optimizations

### The Problem: Deep Nesting Slowdown

When you write deeply nested let bindings:

```avon
let a = 1 in
let b = a + 1 in
let c = b + 1 in
... (200+ levels) ...
let z = ... in z
```

A naive implementation would clone the symbol table at each level, causing O(n²) performance. With just 200 bindings, this would timeout.

### The Solution: Rc + Stack-Based Scoping + Minimal Symbol Table Capture

Instead of cloning, Avon uses three techniques:

1. **Rc Wrapping for Closures**
   When a function captures its environment, it wraps it in Rc (reference counting). This lets multiple functions share the same environment without deep copying.

2. **Stack-Based Scope Management**
   Let bindings don't clone the table—they add/remove from one mutable table. This is like a call stack: push a variable, evaluate the expression after `in`, then pop it back off. Each variable is removed immediately after its expression (the part after `in`) is **fully evaluated**—the variable remains available throughout the entire evaluation of that expression. Note: Avon does not allow variable shadowing (except `_` can be reused in let bindings, but cannot be used as a variable in expressions), so each variable name must be unique within its scope.
   
   **Key point:** Variables are removed after their expression is fully evaluated, not during evaluation. This means expressions like `x + x` work correctly—both sides are evaluated while `x` is still in the symbol table, and `x` is only removed after the entire `x + x` expression completes. In nested lets, inner variables are removed before outer ones.
   
   **Example 1: Nested lets in value position**
   ```avon
   let x = let y = 29 in y in x
   ```
   Evaluation process:
   1. Evaluate `let y = 29 in y`:
      - Insert `y = 29` into symbol table
      - Evaluate `y` (returns `29`)
      - Remove `y` from symbol table (after `y` is evaluated)
      - Return `29`
   2. Insert `x = 29` into symbol table
   3. Evaluate `x` (returns `29`, only `x` is available, `y` is already removed)
   4. Remove `x` from symbol table (after `x` is evaluated)
   5. Return result `29`
   
   **Example 2: Sequential lets**
   ```avon
   let x = 5 in
   let y = 10 in
   x + y
   ```
   Evaluation process:
   1. Insert `x = 5` into symbol table
   2. Evaluate `let y = 10 in x + y`:
      - Insert `y = 10` into symbol table
      - Evaluate `x + y` (both `x` and `y` are available throughout this evaluation)
      - Remove `y` from symbol table (after `x + y` is fully evaluated)
      - Return `15`
   3. Remove `x` from symbol table (after inner let expression is evaluated)
   4. Return result `15`
   
   **Example 3: Variable used multiple times**
   ```avon
   let x = 5 in x + x
   ```
   Evaluation process:
   1. Insert `x = 5` into symbol table
   2. Evaluate `x + x`:
      - Evaluate left `x` (looks up `x` in symbol table, gets `5`)
      - Evaluate right `x` (looks up `x` in symbol table, gets `5`)
      - Add them: `5 + 5 = 10`
      - Return `10`
   3. Remove `x` from symbol table (after `x + x` is fully evaluated)
   4. Return result `10`
   
   Note: Both sides of `x + x` are evaluated while `x` is still in the symbol table. The variable is only removed after the entire expression completes.

3. **Minimal Symbol Table Capture for Templates** 
   Templates only capture the variables they actually reference, not the entire symbol table. This eliminates exponential symbol table growth during template concatenation.

### Real-World Impact

With 200+ let bindings:
- **Without optimization**: ~10MB memory, 60+ seconds (timeout)
- **With optimization**: ~100KB memory, <1 second
- **With minimal symbol tables**: Even faster, templates only capture 1-10 variables instead of 200+

The same technique is used in production Rust code—it's proven, efficient, and correct.

### Understanding the Tradeoff

The stack-based approach has one constraint: **you can't store a reference to a modified scope**. This is fine for Avon because:
- Functions capture their environment at creation time (immutable)
- You can't pass mutable state between functions
- Everything is functional/immutable

This constraint actually makes Avon code easier to reason about—there's no hidden mutable state being passed around.

## Error Handling

### Error Propagation

Errors propagate up the call stack with context:

```avon
let nums = [1, 2, 3] in
map (\x x + "hello") nums
```

Error chain:
```
map: (lambda): +: type mismatch: expected number, found string
     ^^^^^^^^  ^^  ^^^^^^^^^^^^^^
     function  op  specific error
```

### Line Numbers

All errors include line numbers for precise debugging:

```
Error on line 42: filter: is_positive: expected function, found number
```

### Error Types

Three main categories:

1. **Parse Errors**: Malformed syntax
   ```
   Error: Unexpected token ')' on line 5
   ```

2. **Type Errors**: Wrong types for operation
   ```
   Error: +: type mismatch: expected number, found string
   ```

3. **Name Errors**: Unknown variable/function
   ```
   Error: Undefined variable 'x'
   ```

## Operator Precedence & Associativity

### Precedence (Low to High)

```
Logical OR (||)
Logical AND (&&)
Comparison (==, !=, >, <, >=, <=)
Addition/Subtraction (+, -)
Multiplication/Division/Modulo (*, /, %)
Unary Minus (-)
Function Application (highest)
```

### Associativity

- **Left-associative**: `a + b + c` = `(a + b) + c`
- **Right-associative**: `\x \y x + y` = `\x (\y x + y)`

### Parsing Example

```avon
1 + 2 * 3 - 4
```

Parsed as:
```
((1 + (2 * 3)) - 4)
    ↑ higher
precedence
```

## Lexical Scoping

### How Scoping Works

Avon uses lexical (static) scoping: variable visibility is determined by code structure. Functions capture variables from their creation environment:

```avon
let x = 1 in
let f = \y x + y in
f 10
```

Result: **11**

Why? Function `f` was created in the environment where `x=1`, so it captures `x=1` in its closure. When `f 10` is called, it uses the captured `x=1`, resulting in `1 + 10 = 11`.

**Note:** Avon does not allow variable shadowing, so you cannot redefine `x` in the same scope. The function captures the variable value from when it was created, not from when it's called.

### No Variable Shadowing

Avon does **not** allow variable shadowing. Once a variable is defined in a scope, it cannot be redefined in that same scope:

```avon
let x = 5 in
let x = 10 in    # Error: variable 'x' is already defined in this scope
x
```

**Exception:** The underscore `_` can be reused in `let` bindings (common pattern for ignoring values), but it **cannot be used as a variable** in expressions:
```avon
let _ = compute_value() in
let _ = another_value() in  # OK: underscore can be reused in let bindings
result  # OK: using result, not _
```

**Not allowed:**
```avon
let _ = 5 in _  # Error: underscore cannot be used as a variable
let _ = 5 in _ + 1  # Error: underscore cannot be used as a variable
```

The underscore is **only** for discarding values in `let` bindings, not for referencing values.

**Why no shadowing?**
- Prevents confusion about which variable is being used
- Makes code easier to reason about
- Encourages clear, descriptive variable names
- Aligns with functional programming principles

## Built-in Functions vs Language Features

### Built-ins (Map, Filter, Fold)

These are language primitives, not user-defined functions:

```avon
map f [1, 2, 3]
```

Evaluated as:
```rust
eval(Builtin("map", [f, [1, 2, 3]]))
```

### Why Built-in?

- Can't be implemented in user code (no recursion)
- Fundamental to functional programming
- Highly optimized in Rust

## Next Steps: Reading the Code

To understand implementation details:

1. **`src/common.rs`**: Data structure definitions (Expr, Value, Token)
2. **`src/lexer.rs`**: Tokenization (string → tokens)
3. **`src/parser.rs`**: Parsing (tokens → AST)
4. **`src/eval.rs`**: Evaluation (AST → Value)
5. **`src/main.rs`**: CLI and REPL

Each file is well-commented with examples.

## Further Reading

- **Performance**: See `tutorial/OPTIMIZATION.md` for Rc implementation details
- **Security**: See `tutorial/SECURITY.md` for path traversal prevention
- **Grammar**: See `tutorial/GRAMMAR.md` for formal language specification
- **Examples**: See `examples/` directory for real-world usage patterns
- **Sharing Templates**: The `--git` flag enables fetching templates directly from GitHub, making it easy to share and deploy templates. This is implemented using GitHub's raw content API and supports the format `user/repo/path/to/file.av`. See [SIMPLE_CONFIGS.md](./SIMPLE_CONFIGS.md) for examples.
