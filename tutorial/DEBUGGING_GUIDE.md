# Debugging in Avon

## Overview

Avon provides multiple debugging strategies to understand and troubleshoot your programs. This guide explains the complete debugging toolkit and how to use each tool effectively.

## The Debugging Philosophy

Avon uses a **layered debugging approach**:

1. **Error messages** tell you **where and what** failed with line numbers and context
2. **Debugging tools** show you **what values** are flowing through
3. **Type checking** lets you **validate assumptions** early
4. **CLI flags** show you **how the compiler** processes your code

This separation of concerns makes debugging **focused and efficient**.

## Error Messages

Avon provides direct error messages that name the failing functions/operators and include the line number and source context where the error occurred.

### Anatomy of an Error

Errors in Avon often form a "chain" showing the call stack at the moment of failure.

```
function_name: type_error on line 15
15 |    map (\x x + "string") [1, 2, 3]
```

Examples:
- `concat: expected String, found Number on line 10`
- `map: add_one: +: expected String, found Number on line 25`

**Reading the Chain:**
In the example `map: add_one: +: ...`, read from right to left (inner to outer):
1. `+` failed (the actual operation)
2. inside `add_one` (the function containing the operation)
3. inside `map` (the builtin calling the function)

**Advantages:**
- **Precise location** - The line number and source context help you find the error quickly
- **Direct causality** - The error chain shows exactly which function/operator failed
- **Actionable** - You know *where* and *what* the problem is

---

## Tool 1: Runtime Debugging with `trace` and `debug`

These are the **primary debugging tools** for understanding program behavior at runtime.

### The `trace` Function

`trace` is your go-to debugging function. It prints a labeled value to stderr and returns the value unchanged, allowing you to inspect intermediate values without disrupting your program.

**Signature:** `trace :: String -> a -> a`

**How it works:**
- Takes a **label** (String) and a **value** (any type)  
- Prints `[TRACE] label: value` to stderr
- Returns the value unchanged
- Can be freely inserted into expressions

**Basic example:**

```avon
let x = 42 in
let y = trace "the value of x" x in
y + 10
```

When you run this, stderr shows:
```
[TRACE] the value of x: 42
```

And stdout shows:
```
52
```

The trace doesn't affect the computation - it just lets you see what's happening.

**Debugging function arguments:**

```avon
let add_one = \x
  let _ = trace "input to add_one" x in
  x + 1
in

map add_one [1, 2, 3]
```

Stderr output:
```
[TRACE] input to add_one: 1
[TRACE] input to add_one: 2
[TRACE] input to add_one: 3
```

This shows exactly which values are being processed.

**Debugging intermediate computations:**

```avon
let pipeline = \x
  let step1 = trace "after doubling" (x * 2) in
  let step2 = trace "after adding 10" (step1 + 10) in
  step2
in

pipeline 5
```

Stderr:
```
[TRACE] after doubling: 10
[TRACE] after adding 10: 20
```

You can see the value at each transformation step.

### The `debug` Function

`debug` shows the **internal structure** of values using Rust's Debug format.

**Signature:** `debug :: a -> a`

**How it works:**
- Takes any value
- Prints its internal structure to stderr
- Returns the value unchanged

**When to use:** When you need to see the exact shape of complex data.

**Example:**

```avon
let config = {host: "localhost", port: "8080", debug: "true"} in

debug config
```

Stderr:
```
[DEBUG] Dict({"host": String("localhost"), "port": String("8080"), "debug": String("true")})
```

This shows the exact structure and types in the dict.

**Debugging after transformations:**

```avon
let numbers = [1, 2, 3] in
let doubled = map (\x x * 2) numbers in
debug doubled
```

Stderr:
```
[DEBUG] List([Number(2), Number(4), Number(6)])
```

Shows you exactly what `map` produced.

---

## Tool 2: Type Checking & Validation

Prevent errors before they happen by validating types early in your code.

**Type introspection functions:**

```avon
typeof value           # Returns type as string: "string", "number", "list", etc.
is_string value        # Boolean: true if string
is_number value        # Boolean: true if number  
is_int value           # Boolean: true if integer
is_float value         # Boolean: true if float
is_list value          # Boolean: true if list
is_bool value          # Boolean: true if boolean
is_function value      # Boolean: true if function
```

**General assertion function:**

```avon
assert (test) value    # If test is true, returns value; otherwise errors with debug info
```

**Common assertion patterns:**

```avon
assert (is_string x) x     # Assert x is a string
assert (is_number x) x     # Assert x is a number
assert (is_int x) x        # Assert x is an integer
assert (is_list x) x       # Assert x is a list
assert (is_bool x) x       # Assert x is a boolean
assert (x > 0) x           # Assert x is positive
assert (length xs > 0) xs  # Assert list is not empty
```

**Example: Defensive filtering**

```avon
let process_numbers = \data
  # Check input type first
  let validated = assert (is_list data) data in
  
  # Map with type checks
  map (\item
    let num = assert (is_number item) item in
    num * 2
  ) validated
in

process_numbers [1, 2, 3]    # Works
process_numbers ["1", "2"]   # Error: assertion failed
```

Assertions give you **early failure** with **clear debug information**, showing both the failing test and the actual value.

---

## Tool 3: Compiler-Level Debugging with `--debug` Flag

For understanding how the compiler processes your code, use the `--debug` command-line flag.

**Usage:**

```bash
avon eval program.av --debug
```

**Output:** Shows three compilation phases:

```
[DEBUG] Starting lexer...
[DEBUG] Lexer produced 42 tokens
[DEBUG]   Token 0: Keyword("let")
[DEBUG]   Token 1: Ident("x")
[DEBUG] Starting parser...
[DEBUG] Parser produced AST: ...
[DEBUG] Starting evaluator...
```

**When to use:**
- Suspecting syntax errors
- Learning how Avon parses your code
- Understanding evaluation order
- When file-level debugging isn't enough

**Not needed for most debugging** - `trace` and `debug` are better for runtime issues.

---

## Debugging Workflow

### Step 1: Run and Get an Error

```bash
avon eval program.av
```

You get an error message with line number:

```
map: add_one: +: expected String, found Number on line 25
25 |    x + 1
```

### Step 2: Understand the Error

The error tells you:
- **`map`** - error is in the map builtin
- **`add_one`** - called within add_one function  
- **`+`** - in the + operator
- **`expected String, found Number`** - type mismatch
- **`on line 25`** - exact location

**Action:** The error chain shows the problem is in `add_one` receiving the wrong type.

### Step 3: Inspect the Input

Add a `trace` to see what's being passed to `map`:

```avon
let add_one = \x
  trace "add_one input" x
  x + 1
in

let numbers = trace "numbers list" [1, 2, 3] in
map add_one numbers
```

Re-run to see:

```
[TRACE] numbers list: [1, 2, 3]
[TRACE] add_one input: 1
[TRACE] add_one input: 2
[TRACE] add_one input: 3
```

Now you know the values being passed are correct numbers.

### Step 4: Inspect the Data Structure

If you're unsure about the structure, use `debug`:

```avon
let numbers = [1, 2, 3] in
debug numbers
```

Output:

```
[DEBUG] List([Number(1), Number(2), Number(3)])
```

This shows exactly what types are in your data.

### Step 5: Isolate the Problem

If the inputs look right, the problem is in the function logic. Fix `add_one`:

```avon
# Wrong: tries to add string and number
let add_one = \x
  x + 1  # If x is a string, this fails
in

# Correct: ensure x is a number
let add_one = \x
  let num = assert (is_number x) x in
  num + 1
in
```

### Step 6: Verify the Fix

Re-run to confirm:

```bash
avon eval program.av
```

---

## Common Pitfalls & Solutions

### 1. Missing `in` keyword

**Error:** `unexpected token: ...` or parser errors.

**Cause:** `let` bindings must always be followed by `in`.

**Wrong:**
```avon
let x = 1
x + 1
```

**Correct:**
```avon
let x = 1 in
x + 1
```

### 2. Template Quote Syntax

**Error:** `expected '"' after opening braces`

**Cause:** Templates must strictly follow the syntax `{"..."}` or `{{"..."}}`. You cannot use spaces between the brace and the quote.

**Wrong:**
```avon
{ "hello" }
```

**Correct:**
```avon
{"hello"}
```

### 3. Number vs String in Arithmetic

**Error:** `+: expected Number, found String`

**Cause:** Avon does not auto-convert strings to numbers in math operations.

**Wrong:**
```avon
"5" + 10
```

**Correct:**
```avon
to_int "5" + 10
```

### 4. Unmatched Braces in Templates

**Error:** `unexpected EOF inside template`

**Cause:** You opened a template with `{{"` but tried to close it with `}"` or used interpolation `{...}` inside a double-brace template without double braces.

**Fix:** Ensure your interpolation delimiters match your template definition (see `tutorial/TUTORIAL.md` for the full Escape Hatch guide).

---

## Common Debugging Patterns

### Pattern 1: Tracing Function Inputs and Outputs

```avon
let process = \data
  let _ = trace "input" data in
  let result = ...transformation... in
  let _ = trace "output" result in
  result
in
```

This shows you what comes in and what goes out. Note: `trace` returns a value, so you need to bind it with `let _` to discard it (or use its return value), then use `in` to continue.

### Pattern 2: Validating Before Use

```avon
let safe_divide = \a \b
  let a_num = assert (is_number a) a in
  let b_num = assert (is_number b) b in
  if b_num == 0 then error "Division by zero"
  else a_num / b_num
in
```

Assertions catch type errors early with clear debug information showing the failing value.

### Pattern 3: Debugging List Transformations

```avon
let pipeline = \data
  let step1 = trace "after step1" (map f data) in
  let step2 = trace "after step2" (filter pred step1) in
  let step3 = trace "after step3" (map g step2) in
  step3
in
```

This shows the data at each transformation.

### Pattern 4: Understanding Conditionals

```avon
let classify = \x
  let x_num = assert (is_number x) x in
  if x_num < 0 then trace "negative" "neg"
  else if x_num == 0 then trace "zero" "zero"
  else trace "positive" "pos"
in

classify (0 - 5)  # Pass a negative number: 0 - 5 = -5
```

Traces show which branch is taken.

### Pattern 5: Debugging Fold Accumulation

```avon
let sum = \acc \item
  trace "accumulator" acc
  trace "current item" item
  acc + item
in

fold sum 0 [1, 2, 3]
```

Shows each step of the accumulation:

```
[TRACE] accumulator: 0
[TRACE] current item: 1
[TRACE] accumulator: 1
[TRACE] current item: 2
[TRACE] accumulator: 3
[TRACE] current item: 3
```

---

## Troubleshooting Deployment Issues

### Path Errors

If your files aren't appearing where you expect:

1. **Check your paths**: Use `eval` to print the output without writing files.
   ```bash
   avon eval program.av
   ```
   This will show the full paths of files that *would* be created.

2. **Check `--root`**: The `--root` flag prepends a directory to all paths.
   ```avon
   @config.yml {"..."}
   ```
   - **With `--root ./out`**: This writes to `./out/config.yml`
   - **Without `--root`**: Files are written relative to the current working directory where `avon` is executed
     - Example: Running `avon deploy config.av` from `/home/user/project/` writes `@config.yml` to `/home/user/project/config.yml`
     - **Note:** Always use `--root` for predictable, contained deployments

3. **Absolute vs Relative**: 
   - **Without `--root`**: 
     - Absolute paths (starting with `/`, e.g. `@/etc/config`) are **blocked** for security
     - Paths containing `..` are **blocked** for security
     - Only relative paths are allowed, written to the current working directory
   - **With `--root`**: 
     - Paths starting with `/` have the leading `/` stripped and are appended to the root directory
     - Example: `@/etc/config` with `--root ./out` writes to `./out/etc/config`
     - Example: `@config.yml` with `--root ./out` writes to `./out/config.yml`
   - **Always use `--root`** when deploying to ensure files are written to a controlled directory

### Permission Errors

If you get "permission denied":

1. **Directory creation**: Avon attempts to create parent directories. Ensure you have write permissions to the target directory.
2. **Existing files**: If a file exists and you didn't use `--force`, Avon will warn you. If you don't have write permission to the existing file, `--force` will fail.

---

## Best Practices

### Do This

- **Use `trace` liberally** during development - label clearly ("input", "after step1", etc.)
- **Chain traces** to see the flow of data through transformations
- **Use `debug` for complex structures** - when you need to see exact types and nesting
- **Validate early** with `assert` - catch errors at the source with clear debug output
- **Use type predicates** with assert: `assert (is_number x) x`, `assert (x > 0) x`
- **Keep `trace` calls in code** - they're cheap and help future debugging
- **Test with `eval` first** - always verify logic before deploying with `--deploy`

### Don't Do This

- **Ignore error messages** - they point directly to the failing code with line numbers
- **Expect pretty printing** - Avon shows simple, focused errors on purpose
- **Assume data types** - use type checks to verify assumptions
- **Nest traces too deeply** - use intermediate `let` bindings to keep code clear
- **Deploy without testing** - always `eval` first to catch errors early

---

## Complete Example: Real-World Debugging

### Problem

Parse environment configuration from a dict but the dict structure is wrong:

```
map: process_env: .: key not found: "timeout" on line 42
42 |    let timeout = env_dict.timeout in
```

### Investigation

Original code:

```avon
let process_env = \env_dict
  let timeout = env_dict.timeout in
  let retries = env_dict.retries in
  {timeout: timeout, retries: retries}
in

let envs = [
  {host: "localhost", port: 8080},
  {host: "prod.example.com", port: 443}
] in
map process_env envs
```

### Add Debugging

```avon
let process_env = \env_dict
  let _ = trace "processing dict" env_dict in
  let timeout = env_dict.timeout in
  let retries = env_dict.retries in
  {timeout: timeout, retries: retries}
in

let envs = [
  {host: "localhost", port: 8080},
  {host: "prod.example.com", port: 443}
] in
let _ = trace "all envs" envs in
map process_env envs
```

Run to see:

```
[TRACE] all envs: [{host: "localhost", port: 8080}, {host: "prod.example.com", port: 443}]
[TRACE] processing dict: {host: "localhost", port: 8080}
map: process_env: .: expected key 'timeout', found missing on line 42
42 |    let timeout = env_dict.timeout in
```

Ah! The dicts have `host` and `port`, but the code expects `timeout` and `retries`. The data structure doesn't match expectations.

### Fix

Option 1: Update the dict structure:

```avon
let envs = [
  {timeout: 30, retries: 3},
  {timeout: 60, retries: 5}
] in
map process_env envs
```

Option 2: Validate before processing with `has_key`:

```avon
let process_env = \env_dict
  if has_key env_dict "timeout" then
    let timeout = env_dict.timeout in
    let retries = env_dict.retries in
    {timeout: timeout, retries: retries}
  else
    error "dict missing required keys"
in
```

Option 3: Use defensive checking with debug output:

```avon
let process_env = \env_dict
  let _ = debug env_dict in
  let timeout = env_dict.timeout in
  let retries = env_dict.retries in
  {timeout: timeout, retries: retries}
in

let envs = [
  {timeout: 30, retries: 3},
  {timeout: 60, retries: 5}
] in
let _ = trace "all envs" envs in
map process_env envs
```

This shows that **tracing the dict before processing** reveals the structure mismatch immediately.

### Verify

Re-run:

```bash
avon eval program.av
```

Now it works correctly!
