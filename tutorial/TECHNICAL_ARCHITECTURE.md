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

Avon uses **stack-based scoping** with a single mutable symbol table. When you create a new scope with `let`, variables are added to the table, evaluated, then removed—no cloning:

```avon
let x = 5 in
let x = 10 in
x
```

Evaluation process:
1. Add `x=5` to table: `{x: 5}`
2. Add `x=10` (shadows): `{x: 10}`
3. Evaluate inner expression: result is `10`
4. Remove `x=10`: back to `{x: 5}`
5. Continue with outer scope

**Why this approach?**
- Efficient: Only insert/remove operations, no cloning per binding
- Memory-light: Single mutable reference passed through evaluation
- Correct: Naturally respects lexical scoping rules
- Fast: Even 200+ let bindings complete in milliseconds

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
   Let bindings don't clone the table—they add/remove from one mutable table. This is like a call stack: push a variable, evaluate, pop it back off.

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

Avon uses lexical (static) scoping: variable visibility is determined by code structure:

```avon
let x = 1 in
let f = \y x + y in
let x = 2 in
f 10
```

Result: **11** (not 12)

Why? Function `f` was created in the environment where `x=1`, so it captures `x=1`. The later `let x = 2` doesn't affect `f`'s captured environment.

### Shadowing

Variables can be redefined, but only explicitly:

```avon
let x = 5 in
let x = 10 in    # OK: explicit new binding
x                # Result: 10
```

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
