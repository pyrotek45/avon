# Avon — The Modern Template Language for Developers

Welcome to **Avon**. You're about to give your configuration workflow superpowers.

Avon is designed for developers who are tired of copy-pasting. Whether you're building Kubernetes manifests, setting up CI/CD pipelines, or generating boilerplate code, Avon turns repetitive tasks into elegant, maintainable code.

But Avon isn't just for complex infrastructure projects. It's a **powerful workflow layer** that makes any file more maintainable and shareable—even if you're just managing a single config or sharing dotfiles. Avon brings variables, functions, and 80+ built-in utilities to **any text format**, making it perfect for developers, non-developers, and hobbyists alike.

**Pro tip:** Throughout this guide, look at the `examples/` directory for real-world use cases. Each example demonstrates practical Avon patterns you can adapt for your own projects.

---

## Table of Contents

1. **[Quick Start](#quick-start)**
   - Your First Program
   - Generate a Single File
   - Generate Multiple Files
   - Avon for Single Files and Dotfiles

2. **[Core Concepts](#core-concepts)**
   - Simple File Model (one expression per file)
   - The Avon Runtime Model (values and types)

3. **[Language Essentials](#language-essentials)**
   - Syntax Overview
     - Literals (strings, numbers including negatives, booleans, lists, dicts)
     - Identifiers and Variables
     - Function Literals
     - Function Application
   - Operators
     - Arithmetic (`+`, `-`, `*`, `/`, unary `-` for negative numbers)
     - Comparison (`==`, `!=`, `>`, `<`, `>=`, `<=`)
     - Logical (`&&`, `||`)
     - Pipe Operator (`->`)
   - Path Values (file system paths)
   - Conditionals (`if then else`)

4. **[Functions & Variables](#functions--variables)**
   - Let Bindings
     - How Scoping Works (lexical scoping)
     - Variable Visibility
     - Cascading Lets
     - Nested Scopes
     - No Variable Shadowing
     - Template Variable Capture (closures)
     - Function Closures
     - Scope Isolation
   - Defining Functions
   - Why Recursion Is Not Supported
   - Default Parameters (`?` syntax)
   - Named Deploy Arguments
   - Practical Example: Configuration Generator

5. **[Collections](#collections)**
   - Lists (creation, interpolation, concatenation)
   - Range Syntax (`[start..end]`, `[start, step..end]`)
     - How ranges work
     - What ranges return
     - Functions that work with ranges
     - Ranges in interpolation
   - Dictionaries (key-value maps, dot notation, operations)
   - Map (transform every item)
   - Filter (keep what you need)
   - Fold (reduce to a single value)
   - Builtins for Lists (comprehensive list operations)

6. **[Templates](#templates)**
   - Basic Template Syntax
   - Multi-line Templates
   - Indentation and Dedent
   - Interpolating Lists
   - String vs Template (escape sequences)
   - Template Escape Hatch (variable brace delimiters)
     - Single-brace templates
     - Double-brace templates
     - Triple-brace templates
   - Complex Interpolations

7. **[File Templates & Deployment](#file-templates--deployment)**
   - Basic FileTemplate
   - Deploying Single Files
   - Deploying Multiple Files
   - Dynamic File Paths
   - Important Deploy Flags
     - `--root` (required for safety)
     - `--force` (overwrite)
     - `--backup` (safe overwrite)
     - `--append` (additive)
     - `--if-not-exists` (initialization)
     - `--git` (fetch from GitHub)
     - `--debug` (detailed output)

8. **[Builtin Functions](#builtin-functions)**
   - String Operations (`concat`, `upper`, `lower`, `split`, `replace`, etc.)
   - List Operations (`map`, `filter`, `fold`, `join`, `length`, `zip`, `unzip`, `take`, `drop`, `split_at`, `partition`, `reverse`, `head`, `tail`)
   - File & Filesystem (`readfile`, `readlines`, `exists`, `basename`, `dirname`)
   - HTML Generation Helpers (`html_escape`, `html_tag`, `html_attr`)
   - Markdown Generation Helpers (`md_heading`, `md_link`, `md_code`, `md_list`)
   - Type Conversion (`to_string`, `to_int`, `to_float`, `to_bool`)
   - Advanced List Operations (`flatmap`, `flatten`)
   - Data & Utilities (`import`, `json_parse`, `os`)

9. **[CLI Usage](#cli-usage)**
   - Basic Commands
     - `eval` (evaluate and print)
     - `deploy` (generate files)
     - `run` (evaluate code string)
     - `repl` (interactive shell)
     - `doc` (builtin documentation)
     - `version` (version info)
     - `help` (usage information)
   - Passing Arguments
     - How It Works (function evaluation flow)
     - Named Arguments (`-param value`)
     - Positional Arguments
     - Default Values
     - Mixing Named and Positional
     - Argument Types (all arguments are strings)
     - Complete Examples
   - Interactive REPL
     - Why Use the REPL
     - Starting the REPL
     - Basic Usage
     - REPL Commands (`:help`, `:let`, `:vars`, `:inspect`, `:unlet`, `:read`, `:run`, `:eval`, `:preview`, `:deploy`, `:deploy-expr`, `:write`, `:history`, `:save-session`, `:load-session`, `:doc`, `:type`, `:clear`, `:exit`)
     - Multi-line Input
     - Error Handling
     - Best Practices
   - Command-Line Flags
   - Real-World Examples
   - Single File in Git, Many Deployments

10. **[Error handling and debugging](#error-handling-and-debugging)**
    - Runtime Type Safety
      - How type checking works
      - Error message format
      - Lexing and parsing errors
      - Deployment errors
      - Error recovery
    - Debugging Tools
      - `trace` (labeled values to stderr)
      - `debug` (pretty-print structures)
      - `assert` (validate conditions)
      - `--debug` flag (detailed output)

11. **[Best Practices](#best-practices)**
    - Write Clear, Composable Code
    - Test Before Deploying
    - Use Named Arguments
    - Always Use `--root`
    - Keep Templates Readable
    - Return Lists for Multiple Files

12. **[Safety & Security](#safety--security)**
    - Secrets Management
      - `env_var` (fail-safe)
      - `env_var_or` (with defaults)
    - Deployment Safety
      - No Partial Writes (atomic deployment)
      - Directory Creation Checks
      - Write Error Handling
    - Preventing Accidental Overwrites
      - Default behavior (skip existing)
      - Backup Feature (`--backup`)
    - Best Practices for Safety

13. **[Real-World Examples](#real-world-examples)**
    - Example 1: Site Generator
    - Example 2: Neovim Configuration
    - Example 3: Emacs Configuration
    - Example 4: Docker Compose Generator
    - Example 5: Kubernetes Manifests
    - Example 6: GitHub Actions Workflow
    - Example 7: Package.json Generator
    - Example 8: Escape Hatch Demonstration

14. **[Troubleshooting](#troubleshooting)**
    - Common Errors
      - "expected '\"' after opening braces"
      - "unexpected EOF"
      - "undefined identifier"
    - Escape Hatch Troubleshooting
      - Literal braces not showing
      - Lots of braces getting confusing
      - Interpolation not working
    - Debugging Tips

---

## Quick Start

### Your First Program

Let's start with the simplest possible Avon program. Create a file called `hello.av`:

```avon
"Hello, world!"
```

Run it:

```bash
avon eval examples/hello.av
# Output: Hello, world!
```

Congratulations! You've just run your first Avon program.

### Generate a Single File

Now let's generate an actual file. Create `greet.av`:

```avon
\name @greeting.txt {"
    Hello, {name}!
    Welcome to Avon.
"}
```

Deploy it:

```bash
avon deploy examples/greet.av -name Alice --root ./output --force
```

This creates `./output/greeting.txt` with the content:
```
Hello, Alice!
Welcome to Avon.
```

**What happened?**
- `\name` defines a function parameter
- `@greeting.txt` specifies the output file path (relative to `--root`)
- `{"..."}` is a template that interpolates the `{name}` variable
- `--root ./output` ensures files are written to `./output/greeting.txt`

### Generate Multiple Files

Here's where Avon shines. Let's generate a config file for each environment:

```avon
let environments = ["dev", "staging", "prod"] in
map (\env @config-{env}.yml {"
    environment: {env}
    debug: {if env == "prod" then false else true}
"}) environments
```

Deploy it:

```bash
avon deploy examples/gen_configs.av --root ./configs --force
```

This creates three files: `config-dev.yml`, `config-staging.yml`, and `config-prod.yml` — each with appropriate settings.

**Key insight:** Return a list of file templates and Avon generates them all in one go!

### Avon for Single Files and Dotfiles

Avon isn't just for generating hundreds of files. It's a powerful workflow layer that makes **any file** more maintainable and shareable, even if you're just managing a single config.

**Perfect for:**
- **Dotfiles** — Easy way to download and deploy configs to your system
- **Sharing configs** — One file in git, many customized deployments
- **Single files with variables** — Make any file more generic and maintainable
- **Long, repetitive files** — Use list interpolation to eliminate copy-paste
- **Non-developers** — Simple way to manage and share personal configs

**Example: Dotfile with Variables**
```avon
\username ? "developer" @.vimrc {"
  " Vim configuration for {username}
  set number
  set expandtab
  set tabstop=4
  colorscheme {if username == "developer" then "solarized" else "default"}
"}
```

**Deploy:**
```bash
avon deploy vimrc.av --root ~ -username alice
```

**Share:** Keep one `.vimrc.av` in git. Each person deploys their customized version. No more maintaining separate dotfiles for each machine.

**Example: Long Config with List Interpolation**
```avon
let plugins = ["vim-fugitive", "vim-surround", "vim-commentary", "vim-repeat"] in
@.vimrc {"
  " Plugin configuration
  {plugins}
"}
```

When you interpolate a list in a template, each item appears on its own line. This eliminates copy-paste even in a single file.

**Language Agnostic:** Avon works with **any text format**—YAML, JSON, shell scripts, code, configs, documentation, or dotfiles. It brings variables, functions, and 80+ built-in utilities to any file, making even single files more powerful.

**Runtime Type Safety:** Avon doesn't deploy if there's a type error. No static types needed—if a type error occurs, deployment simply doesn't happen. This flexible approach brings type safety to any file without the complexity of compile-time type systems.

**Built-in Utilities:** Avon comes with 80+ built-in functions for string operations, list operations, formatting, JSON manipulation, file I/O, and HTML/Markdown helpers. These utilities make any file more powerful, even if you're just managing a single config.

**Debugging Tools:** Use `trace`, `debug`, `assert`, and the `--debug` flag to troubleshoot quickly, whether you're working with complex infrastructure or a simple config file.

---

## Core Concepts

### Simple File Model

**Each Avon file contains exactly one expression.** This keeps Avon simple and predictable. When you run an Avon file, it evaluates that single expression to a value.

This simplicity enables powerful modularity: the `import` function allows any file to return any Avon type (a string, number, list, dict, function, FileTemplate, or any other value). Files can be libraries that export functions, data files that return dictionaries, or generators that return FileTemplates—all using the same simple model.

**Example: Library file (`math.av`):**
```avon
{double: \x x * 2, triple: \x x * 3, square: \x x * x}
```

**Example: Data file (`config.av`):**
```avon
{host: "localhost", port: 8080, debug: true}
```

**Example: Generator file (`deploy.av`):**
```avon
@config.yml {"host: localhost"}
```

All three are valid Avon files. The `import` function evaluates the file and returns whatever value it produces, making Avon naturally modular.

### The Avon Runtime Model

When you run an Avon program, it evaluates to a **Value**. Here are the types of values you'll encounter:

| Type | Example | Use Case |
|------|---------|----------|
| **String** | `"hello"` | Text and file content |
| **Number** | `42`, `3.14` | Configuration, counts, versions |
| **Bool** | `true`, `false` | Conditional logic |
| **List** | `[1, 2, 3]` | Collections (files, items, lines) |
| **Dictionary** | `{host: "localhost", port: 8080}` | Structured data with named fields |
| **Function** | `\x x + 1` | Reusable logic and transformations |
| **Template** | `{"Hello {name}"}` | Text generation with interpolation |
| **FileTemplate** | `@path/file {"content"}` | File generation targets (relative to `--root`) |

When evaluation is complete, `avon` either:
1. **Prints the result** (for `eval` command) - Shows the value as a string representation
2. **Materializes files** (for `deploy` command) - Writes FileTemplate values to disk

**How `eval` works:**
- Evaluates the expression in the file
- Converts the result to a string representation
- Prints it to stdout
- If the result is a FileTemplate or list of FileTemplates, it shows the paths and content that would be generated (but doesn't write them)
- Exit code: 0 on success, 1 on error

**How `deploy` works:**
- Evaluates the expression in the file
- If the result is a FileTemplate, writes it to disk
- If the result is a list containing FileTemplates, writes all of them
- If the result is not a FileTemplate or list of FileTemplates, reports an error
- Validates all paths and creates directories before writing (atomic deployment)
- Exit code: 0 on success, 1 on error

**Error messages:**
Avon provides detailed error messages that include:
- The function/operator that failed
- The expected types vs. actual types
- The line number where the error occurred
- Source code context around the error
- Actionable suggestions when possible

Use `--debug` flag for even more detailed information about the evaluation process.

---

## Language Essentials

### Syntax Overview

Avon is a small, elegant language optimized for readability and powerful file generation. Here's the complete syntax you need to know:

#### Literals

```avon
"hello"                    # String (escape sequences: \n, \t, \\, \")
42                         # Integer
-42                        # Negative integer
3.14                       # Float
-3.14                      # Negative float
true false                 # Booleans
[1, 2, 3]                  # List
{host: "localhost", port: 8080}  # Dictionary (key: value syntax)
```

**Negative Numbers:** You can write negative numbers directly using the `-` prefix:
```avon
-5                         # Negative integer
-3.14                      # Negative float
[-5, -4, -3]               # List with negative numbers
[10, -1 .. 0]              # Range with negative step
```

**Note:** For variables, use the `neg` function: `let x = 5 in -x` (uses `neg` function internally).

**Strings support escape sequences:** `"\n"` is a newline, `"\t"` is a tab, `"\\"` is a backslash, `"\""` is a quote.

**Dictionary syntax:** Keys are identifiers (unquoted), values can be any type:
```avon
{a: 1, b: 2}               # Simple dict
{host: "localhost", port: 8080, debug: true}  # Mixed types
{nested: {x: 1, y: 2}}     # Nested dicts
```

#### Identifiers and Variables

```avon
name                       # Variable reference
some_identifier            # Letters, digits, underscores
_private                   # Underscores are valid
```

#### Function Literals

```avon
\x x + 1                   # Function of one parameter
\x \y x + y                # Curried function of two parameters
\a \b \c (a + b) * c       # Curried function of three parameters
```

Functions are automatically curried, so `\x \y x + y` is equivalent to `\x (\y x + y)`.

#### Function Application (Calling)

```avon
f x                        # Apply f to x
f x y                      # Apply f to x, then apply result to y
map (\n n + 1) [1,2,3]    # Pass a function and a list to map
```

Application is **left-associative**, so `f a b` means `(f a) b`.

#### Operators

Avon supports these binary operators:

**Arithmetic Operators:**
```avon
a + b                      # Addition (numbers), concatenation (strings/lists/templates/paths)
a - b                      # Subtraction (numbers only)
a * b                      # Multiplication (numbers only)
a / b                      # Division (numbers only)
a % b                      # Modulo/Remainder (numbers only)
```

**Comparison Operators:**
```avon
a == b                     # Equality (works for all types)
a != b                     # Inequality (works for all types)
a > b                      # Greater than (numbers only)
a < b                      # Less than (numbers only)
a >= b                     # Greater or equal (numbers only)
a <= b                     # Less or equal (numbers only)
```

**Logical Operators:**
```avon
a && b                     # Logical AND (both must be true)
a || b                     # Logical OR (at least one must be true)
```

**Pipe Operator:**
```avon
a -> b                     # Pipe: pass a as first argument to b
```

The pipe operator `->` chains expressions, passing the left-hand side as the first argument to the right-hand side. This eliminates nested parentheses and makes code more readable.

**Basic pipe:**
```avon
[1, 2, 3] -> length        # Equivalent to: length [1, 2, 3]
"hello" -> upper           # Equivalent to: upper "hello"
```

**Chained pipes:**
```avon
"hello world" -> upper -> length
# Equivalent to: length (upper "hello world")
# Result: 11
```

**Pipe with curried functions:**
```avon
[1, 2, 3, 4, 5] -> filter (\x x > 2) -> length
# Equivalent to: length (filter (\x x > 2) [1, 2, 3, 4, 5])
# Result: 3
```

**Pipe with lambdas:**
```avon
10 -> \x x * 2             # Equivalent to: (\x x * 2) 10
# Result: 20
```

**Pipe with path literals:**
```avon
@config.json -> exists     # Equivalent to: exists @config.json
```

**Why use pipes?** Pipes make code more readable by showing the flow of data from left to right, rather than nested function calls. Compare:

```avon
# Without pipe (nested)
length (filter (\x x > 2) (map (\x x * 2) [1, 2, 3, 4, 5]))

# With pipe (linear)
[1, 2, 3, 4, 5] -> map (\x x * 2) -> filter (\x x > 2) -> length
```

The pipe version reads naturally: "take the list, double each item, filter values greater than 2, then get the length."

**Operator overloading:** The `+` operator adapts to its operands:
- `"hello" + " world"` -> `"hello world"` (strings concatenate)
- `[1,2] + [3,4]` -> `[1,2,3,4]` (lists concatenate)
- `5 + 3` -> `8` (numbers add)
- `{"Hello "} + {"World!"}` -> `"Hello World!"` (templates concatenate)
- `@home/user + @projects` -> `/home/user//projects` (paths join with `/` separator)

**Template Concatenation:**
Templates can be combined with `+` to merge content:
```avon
let greeting = {"Hello, "} in
let name = "Alice" in
let punct = {"!"} in
greeting + {"World"} + punct     # "Hello, World!"

# With interpolation
let t1 = {"User: {name}"} in
let t2 = {" (verified)"} in
t1 + t2                           # "User: Alice (verified)"
```

**Path Concatenation:**
Paths can be combined with `+` to join path segments:
```avon
let base = @home in
let user = @alice in
base + user                       # "home/alice"

# Practical example
let env = "prod" in
let config_dir = @config/{env} in
let app_config = @app.conf in
config_dir + app_config           # "config/prod/app.conf"
```

#### Path Values

Path values are first-class values in Avon. They represent file system paths and can be stored in variables, passed to functions, and used with file operations.

**Path Literal Syntax:**
```avon
@path/to/file                  # Relative path (use with --root flag)
@relative/path                 # Relative path
@config/{env}/app.yml          # Path with interpolation
```

**Using Path Values:**
```avon
# Store a path in a variable
let config_path = @config/production.json in

# Use with file operations
let content = readfile config_path in
let exists = exists config_path in
let lines = readlines config_path in

# Path manipulation
let filename = basename config_path in
let directory = dirname config_path in

# Dynamic path construction
let env = "staging" in
let app = "myapp" in
let dynamic_path = @config/{env}/{app}.yml in
```

**Path Interpolation:**
You can interpolate variables into paths:
```avon
let username = "alice" in
let home_dir = @users/{username} in
let config_file = @users/{username}/.config/app.yml in
```

**Path Concatenation:**
Paths can be concatenated with `+`:
```avon
let base = @config in
let app = @myapp in
let config = @config.yml in
base + app + config              # "config/myapp/config.yml"
```

**Paths with File Operations:**
All file operations accept path values:
- `readfile path` - Read file content
- `readlines path` - Read file as list of lines
- `exists path` - Check if file exists
- `basename path` - Get filename
- `dirname path` - Get directory
- `import path` - Import Avon file
- `fill_template path dict` - Fill template with substitutions

**Paths vs Strings:**
Paths are distinct from strings. They're type-safe and provide better error messages:
```avon
let p = @config.yml in
readfile p                       # Works: path value

let s = "config.yml" in
readfile s                       # Type error: expected Path, got String
```

**FileTemplate Syntax:**
FileTemplates combine a path with a template:
```avon
@path/to/file.txt {"
    File content here
"}
```

The `@` prefix creates a path value, and the following `{...}` is a template. Together, they form a `FileTemplate` value that can be deployed.

#### Conditionals

```avon
if condition then true_expr else false_expr
```

The `if` expression evaluates the condition. If it's `true`, it returns `true_expr`; otherwise, it returns `false_expr`. Both branches must be present (`then` and `else`).

**Examples:**
```avon
if age > 18 then "adult" else "minor"

if x == 0 then 1 else x

if debug then "verbose" else "quiet"
```

**Nested conditionals:**
```avon
if x > 0 then "positive" else (if x < 0 then "negative" else "zero")
```

**Important:** The condition must evaluate to a boolean (`true` or `false`). Type errors occur if you use a non-boolean value.

#### Logical Operators

Avon provides `&&` (AND) and `||` (OR) for combining boolean expressions:

```avon
a && b                     # Returns true only if both a and b are true
a || b                     # Returns true if at least one of a or b is true
```

**Examples:**
```avon
if (age >= 18) && (has_license) then "can drive" else "cannot drive"

if (x > 0) || (y > 0) then "at least one positive" else "both non-positive"
```

**Important:** Both operands must be booleans. Type errors occur if you use non-boolean values.

**Precedence:** Logical operators have lower precedence than comparison operators, so parentheses are often needed:
```avon
# Correct
if (x > 0) && (y > 0) then "both positive" else "not both positive"

# This would be parsed incorrectly without parentheses
# if x > 0 && y > 0 then ...  # Wrong! Parsed as: if x > (0 && y) > 0
```

**Short-circuit evaluation:** Both `&&` and `||` use short-circuit evaluation:
- `a && b`: If `a` is `false`, `b` is not evaluated
- `a || b`: If `a` is `true`, `b` is not evaluated

---

## Functions & Variables

### Let Bindings

Use `let` to define variables and intermediate values. The syntax is `let <identifier> = <expression> in <expression>`.

**Basic let binding:**
```avon
let greeting = "Hello" in
let name = "Alice" in
greeting + ", " + name
# Evaluates to: "Hello, Alice"
```

**Cascading let bindings:**
You can cascade multiple `let` bindings to build up complex computations:

```avon
let a = 10 in
let b = 20 in
let sum = a + b in
sum * 2
# Evaluates to: 60
```

**How Scoping Works:**

Avon uses **lexical scoping** (also called static scoping), which means variable visibility is determined by the structure of your code, not by when code executes. Here's how it works:

1. **Each `let` creates a new scope:** When you write `let x = value in expr`, Avon:
   - Evaluates `value` in the current scope
   - Creates a **new scope** (a copy of the current symbol table)
   - Adds `x` to this new scope
   - Evaluates `expr` in the new scope
   - The original scope is **never mutated**—this is functional programming

2. **Variable visibility:** Variables are visible in the expression following `in`, but not outside:
```avon
let x = 10 in
x * 2  # x is visible here
# x is NOT visible here (outside the 'in' expression)
```

3. **Nested scopes:** Inner `let` bindings create nested scopes. Each scope can see variables from outer scopes, but outer scopes cannot see variables from inner scopes:
```avon
let x = 10 in           # Outer scope: x = 10
let y = 20 in           # Middle scope: x = 10, y = 20
let temp = x + y in     # Inner scope: x = 10, y = 20, temp = 30
temp * 2                # Middle scope: can see temp
# Outer scope: cannot see temp or y
```

4. **No mutations:** Because each `let` creates a new scope (not a mutation), you can safely reason about your code. Variables never change after they're defined.

5. **The final expression:** The final expression (after all `in` keywords) is what gets evaluated and returned.

**Example with nested scopes:**
```avon
let x = 10 in
let y = 20 in
let result = let temp = x + y in temp * 2 in
result
# Evaluates to: 60
# Note: 'temp' is only visible within the inner 'let' expression
```

**Detailed Scoping Rules:**

### 1. Variable Visibility

Variables are visible in the expression following `in`. They are not visible before their definition or outside their scope.

**Example:**
```avon
let x = 10 in
x * 2  # ✓ x is visible here
# x is NOT visible here (outside the 'in' expression)
```

**Forward reference error:**
```avon
let result = x + 1 in  # Error: x is not defined yet
let x = 10 in
result
```

### 2. Cascading Lets (Sequential Scoping)

Each `let` binding makes the variable available to all subsequent expressions in the same scope. This is called "cascading" because each binding builds on the previous ones:

```avon
let a = "A" in
let b = "B" in
concat a b  # Both a and b are visible here
```

**How it works:**
1. First `let` creates scope 1: `{a: "A"}`
2. Second `let` creates scope 2: `{a: "A", b: "B"}` (includes a from scope 1)
3. Expression `concat a b` evaluates in scope 2 (can see both a and b)

### 3. Nested Scopes (Lexical Scoping)

Inner `let` bindings create nested scopes. Variables defined in inner scopes are not visible outside, but inner scopes can see variables from outer scopes:

```avon
let x = 10 in                    # Scope 1: {x: 10}
let result = let temp = x + 5 in # Scope 2: {x: 10, result: ...}
  temp * 2                       # Scope 3: {x: 10, temp: 15} - temp visible here
in                               # Back to Scope 2: {x: 10, result: 30}
result                           # Scope 2: temp is NOT visible here, only result
```

**Breaking it down:**
- **Scope 1** (`let x = 10 in`): Contains `{x: 10}`
- **Scope 2** (`let result = ... in result`): Contains `{x: 10, result: 30}` (inherits x from Scope 1)
- **Scope 3** (`let temp = x + 5 in temp * 2`): Contains `{x: 10, temp: 15}` (inherits x from Scope 2, adds temp)
- After Scope 3 completes, we're back in Scope 2, where `temp` no longer exists

**Verification:** Trying to use `temp` in the outer scope results in an error:
```avon
let x = 10 in
let result = let temp = x + 5 in temp * 2 in
temp  # Error: unknown symbol: temp
```

**Scope hierarchy:**
- Outer scope: `{x: 10}`
- Inner scope: `{x: 10, temp: 15}` (inherits x, adds temp)
- After inner scope: back to `{x: 10, result: 30}` (temp is gone)

### 4. No Variable Shadowing

Avon is a functional language and does not allow variable shadowing. If you try to define a variable with the same name as an existing variable in the same scope, you'll get an error:

```avon
let x = 1 in
let x = 2 in  # Error: variable 'x' is already defined in this scope
x
```

**Why no shadowing?**
- Prevents confusion and makes code more predictable
- Each variable name is unique within its scope
- Easier to reason about code—you always know which variable you're referring to
- Aligns with functional programming principles (immutability)

**Exception:** The variable `_` (underscore) can be reused. This is a special case for ignoring values:
```avon
let _ = trace "step 1" value1 in
let _ = trace "step 2" value2 in  # OK: _ can be reused
result
```

**Solution when you need similar names:**
```avon
let x = 1 in
let y = 2 in
let inner = let temp_x = 10 in temp_x + y in
let outer = x + y in
[inner, outer]
# Result: [12, 3]
# inner uses temp_x=10, outer uses x=1 (no shadowing, clear scoping)
```

### 5. Template Variable Capture (Closures)

Templates capture variables from their surrounding scope at the time they are created. This is called a "closure" because the template "closes over" the variables:

```avon
let name = "Alice" in
let template = {"Hello, {name}"} in
template
# Result: "Hello, Alice"
# Template captured "Alice" from the surrounding scope when created
```

**Important:** The template captures the value at creation time, not evaluation time:
```avon
let name = "Alice" in
let template = {"Hello, {name}"} in
# Even if 'name' were redefined here (which we can't do), 
# the template would still use "Alice"
template
```

### 6. Function Closures

Functions also capture their environment (closure) when they are created. This allows functions to "remember" variables from their surrounding scope:

```avon
let x = 10 in
let add_x = \y x + y in
add_x 5
# Result: 15
# Function captured x=10 from when it was created
```

**How closures work:**
1. When `add_x` is created, it captures the current scope: `{x: 10}`
2. This captured scope is stored with the function
3. When `add_x 5` is called, it uses the captured `x=10`, not any later definition

**Practical example:**
```avon
let make_adder = \base
  let offset = 5 in
  \x base + offset + x in

let add10 = make_adder 10 in
add10 3
# Result: 18
# add10 captured: base=10, offset=5
# When called with x=3: 10 + 5 + 3 = 18
```

### 7. Scope Isolation

Each `let` binding creates an isolated scope. This means:
- Variables in one scope cannot affect variables in another scope
- No mutations—variables never change after definition
- Predictable behavior—you can reason about code by looking at its structure

**Example demonstrating isolation:**
```avon
let x = 1 in
let y = 2 in
let inner = let temp = x + y in temp * 2 in
let outer = x + y in
[inner, outer]
# Result: [6, 3]
# inner: temp = 1 + 2 = 3, then 3 * 2 = 6
# outer: x + y = 1 + 2 = 3
# Both calculations are independent and predictable
```

**Best Practices:**
- Use unique variable names to avoid shadowing errors
- Remember that templates and functions capture their environment at creation time
- Each `let` binding creates a new scope, so variables are isolated and predictable
- Use descriptive names that reflect the variable's purpose
- The `_` variable can be reused when you want to ignore a value

**Important:** Always include the `in` keyword! `let` bindings require an `in` to specify the expression where the binding is visible. Without `in`, the parser will report an error.

**Common mistake:**
```avon
# Wrong - missing 'in'
let x = 10
x * 2

# Correct
let x = 10 in
x * 2
```

### Defining Functions

Functions are values too. You can bind functions to variables and call them:

```avon
let add = \x \y x + y in
add 5 3
# Evaluates to: 8
```

Functions are **curried** by default, meaning you can partially apply them:

```avon
let add = \x \y x + y in
let add5 = add 5 in      # Partially apply: waiting for second argument
add5 3                   # Apply remaining argument: 5 + 3 = 8
```

### Why Recursion Is Not Supported

Avon does **not** support recursive functions (functions that call themselves). This is an intentional design decision for several important reasons:

**1. Simplicity**
- Recursion adds complexity to the language implementation
- Without recursion, the evaluator is simpler and easier to understand
- Error messages are clearer (unknown symbol vs infinite recursion)

**2. Performance**
- Recursion tracking requires overhead (depth counters, stack management)
- Iterative solutions using `fold`, `map`, and `filter` are often more efficient
- No risk of stack overflow from deep recursion

**3. Encourages Better Patterns**
- Avon's built-in functions (`fold`, `map`, `filter`) are designed for iteration
- These functions are more idiomatic and readable than recursive solutions
- They handle edge cases (empty lists, etc.) automatically

**4. Safety**
- No risk of infinite recursion bugs
- No need for recursion depth limits
- Predictable execution behavior

**How to achieve recursive-like behavior:**

Instead of recursion, use Avon's built-in iteration functions:

```avon
# Instead of recursive factorial, use fold:
let factorial = \n
  fold (\acc \x acc * x) 1 [1 .. n] in
factorial 5
# Result: 120

# Instead of recursive sum, use fold:
let sum_list = \list
  fold (\acc \x acc + x) 0 list in
sum_list [1, 2, 3, 4, 5]
# Result: 15

# Instead of recursive countdown, use range:
let countdown = \n
  reverse [1 .. n] in
countdown 5
# Result: [5, 4, 3, 2, 1]
```

**If you try to use recursion:**
```avon
let factorial = \n
  if n <= 1 then 1 else n * (factorial (n - 1)) in
factorial 5
# Error: unknown symbol: factorial
```

The function cannot reference itself because it's not added to its own environment. This ensures the design goals above are met.

### Default Parameters

When deploying, you can provide default values for parameters using `?`:

```avon
\name ? "Guest" @welcome.txt {"
    Welcome, {name}!
"}
```

When you deploy this without a named argument, `name` defaults to `"Guest"`.

```bash
avon deploy examples/greet.av
# Uses default: "Guest"

avon deploy examples/greet.av -name Alice
# Uses provided value: "Alice"
```

### Named Deploy Arguments

When using the `deploy` command, you can pass named arguments with `-param value`:

```bash
avon deploy examples/config.av -env prod -debug false
```

This is especially useful for functions with multiple parameters:

```avon
\app ? "service" \env ? "dev" @config-{app}-{env}.yml {"
    app: {app}
    environment: {env}
"}
```

### Practical Example: Configuration Generator

```avon
let make_config = \env \debug ? false @config-{env}.yml {"
    environment: {env}
    debug: {debug}
"} in

let environments = ["dev", "staging", "prod"] in
[
  make_config "dev" true,
  make_config "staging" false,
  make_config "prod" false
]
```

This demonstrates:
- Defining a reusable function with defaults
- Calling the function multiple times with different arguments
- Returning multiple file templates as a list

---

## Collections

### Lists

Lists are the workhorse of Avon. They're written with square brackets:

```avon
[1, 2, 3]
["alice", "bob", "charlie"]
[]                          # Empty list
```

When a list is interpolated into a template, each item appears on its own line:

```avon
let names = ["Alice", "Bob", "Charlie"] in
@names.txt {"
    Names:
    {names}
"}
```

Produces:
```
Names:
Alice
Bob
Charlie
```

You can concatenate lists with `+`:

```avon
[1, 2] + [3, 4]           # Result: [1, 2, 3, 4]
```

### Range Syntax

Avon provides a convenient syntax for generating sequences of numbers using ranges:

**Simple Range:**
```avon
[1 .. 5]                   # Result: [1, 2, 3, 4, 5]
[10 .. 15]                 # Result: [10, 11, 12, 13, 14, 15]
```

**Range with Step:**
```avon
[0, 5 .. 20]               # Result: [0, 5, 10, 15, 20] (step of 5)
[1, 3 .. 10]               # Result: [1, 4, 7, 10] (step of 3)
[10, -1 .. 0]              # Result: [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0] (negative step)
```

**Syntax:**
- `[start .. end]` - Generates integers from `start` to `end` (inclusive), step of 1
- `[start, step .. end]` - Generates integers from `start` to `end` (inclusive), incrementing by `step`

**Important:** The `..` operator requires spaces around it: `[1 .. 5]` (not `[1..5]`).

**What ranges return:**
Ranges evaluate to a `List` of integers. You can use all list operations on ranges:

```avon
let ports = [8080 .. 8085] in
length ports                # 6
map (\p "port-" + (to_string p)) ports  # ["port-8080", "port-8081", ...]
filter (\p p > 8082) ports  # [8083, 8084, 8085]
```

**Ranges in interpolation:**
When a range is interpolated into a template, each number appears on its own line (like any list):

```avon
let ports = [8080 .. 8083] in
@ports.txt {"
  Ports:
  {ports}
"}
```

Produces:
```
Ports:
8080
8081
8082
8083
```

**Practical examples:**
```avon
# Generate configs for multiple ports
let ports = [8080 .. 8085] in
map (\p @service-{p}.yml {"
  port: {p}
  name: service-{p}
"}) ports

# Generate even numbers
let evens = [0, 2 .. 20] in  # [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
evens

# Countdown
let countdown = [10, -1 .. 0] in  # [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
countdown
```

**Functions that work with ranges:**
Since ranges return lists, all list functions work:
- `map`, `filter`, `fold` - Transform, filter, reduce
- `take`, `drop`, `split_at` - Extract portions
- `zip`, `unzip` - Combine with other lists
- `reverse`, `partition` - Reorder or split
- `head`, `tail` - Get first element or rest
- `length` - Get count
- All other list operations

**See also:** `examples/range_syntax.av` for more examples.

### Dictionaries (Key-Value Maps)

For structured data with named fields, use **dictionaries** instead of lists of pairs. Dictionaries provide fast key lookup and clean dot notation access.

**Dictionary syntax** uses curly braces with colon notation:

```avon
let config = {host: "localhost", port: 8080, debug: true} in
config
# Result: {host: "localhost", port: 8080, debug: true}
```

**Access values** using dot notation (modern) or the `get` function (compatible with lists of pairs):

```avon
let config = {host: "localhost", port: 8080} in
let host = config.host in        # Dot notation: "localhost"
let port = get config "port" in  # get function: 8080
port
```

**Dictionary query operations:**

```avon
let config = {host: "localhost", port: 8080, debug: true} in

keys config               # ["host", "port", "debug"]
values config             # ["localhost", 8080, true]
length (keys config)      # 3
has_key config "host"     # true
has_key config "missing"  # false
```

**Update a dictionary** using `set`:

```avon
let config = {host: "localhost", port: 8080} in
let updated = set config "debug" true in
updated
# Result: {host: "localhost", port: 8080, debug: true}
```

**Merge dictionaries:**

```avon
let db_config = {host: "db.local", port: 5432} in
let app_config = {timeout: 30, retries: 3} in
let merged = dict_merge db_config app_config in
merged
# Result: {host: "db.local", port: 5432, timeout: 30, retries: 3}
```

**Nested dictionaries:**

```avon
let config = {
  database: {host: "db.local", port: 5432},
  app: {name: "myapp", debug: true}
} in
let db_host = (get config "database").host in
db_host
# Result: "db.local"
```

**Why use dicts instead of list of pairs?**

- **Clearer intent** - `{host: "localhost"}` is more readable than `[["host", "localhost"]]`
- **Dot notation** - Access fields naturally: `config.host` instead of `get config "host"`
- **Faster lookups** - Hash map instead of linear search through pairs
- **Type-safe** - Errors when accessing non-existent keys are clear
- **Better for JSON** - JSON objects naturally parse to dicts
- **Backward compatible** - `get`, `set`, `has_key`, `keys`, `values` work with both dicts and list-of-pairs

**When to use each:**

- **Use dictionaries when:**
  - You need fast key lookup
  - You want dot notation access
  - Keys are unique
  - You're modeling structured configuration or data objects

- **Use list of pairs when:**
  - You need to maintain order
  - You want to iterate sequentially  
  - You might have duplicate keys
  - You're building data dynamically

**Real-world example: Configuration with dicts**

Instead of building a list of pairs and converting later:

```avon
# Modern approach with dict
let service_config = {
  name: "api-service",
  port: 8080,
  replicas: 3,
  health_check: {interval: 30, timeout: 5}
} in

@service.yml {"
  name: {service_config.name}
  port: {service_config.port}
  replicas: {service_config.replicas}
  health_check:
    interval: {service_config.health_check.interval}
    timeout: {service_config.health_check.timeout}
"}
```

**See also:** `examples/dict_operations.av`, `examples/dict_vs_pairs.av`, `examples/dict_config_system.av` for more examples.

### Map — Transform Every Item

`map` applies a function to each item in a list:

```avon
let double = \x x * 2 in
map double [1, 2, 3]
# Result: [2, 4, 6]
```

Real-world example: generate configuration for each environment:

```avon
let environments = ["dev", "staging", "prod"] in
let make_config = \env @config-{env}.yml {"
    environment: {env}
    debug: {env != "prod"}
"} in
map make_config environments
```

### Filter — Keep What You Need

`filter` keeps only items where a condition is true:

```avon
let numbers = [1, 2, 3, 4, 5] in
filter (\n n > 2) numbers
# Result: [3, 4, 5]
```

Example: generate files only for production and staging:

```avon
let all_envs = ["dev", "staging", "prod"] in
let prod_envs = filter (\e e != "dev") all_envs in
map (\e @prod-{e}.yml {"prod config"}) prod_envs
```

### Fold — Reduce to a Single Value

`fold` accumulates a result by applying a function to each item:

```avon
let numbers = [1, 2, 3, 4, 5] in
let sum = fold (\acc \n acc + n) 0 numbers in
sum
# Result: 15
```

The accumulator starts at `0`, and for each number `n`, we update the accumulator: `acc + n`.

Example: join list items into a comma-separated string:

```avon
let names = ["Alice", "Bob", "Charlie"] in
let join_names = fold (\acc \n (concat acc (concat ", " n))) "" names in
@names.txt {"{join_names}"}
```

(Note: Avon has a `join` builtin to make this easier!)

### Builtins for Lists

| Function | Description |
|----------|-------------|
| `map f list` | Apply `f` to each item |
| `filter pred list` | Keep items where `pred` is truthy |
| `fold f init list` | Reduce with accumulator |
| `join list sep` | Join list items with separator |
| `length list` | Get number of items in list |

---

## Templates

Templates are how you generate text output in Avon. They combine literal content with embedded expressions.

### Basic Template Syntax

```avon
{"Hello, World"}           # Simple template
{"Name: {name}"}           # Interpolate a variable
{"1 + 1 = {1 + 1}"}       # Interpolate an expression
```

The `{...}` syntax embeds an expression to be evaluated and inserted into the output.

### Multi-line Templates

Templates preserve newlines exactly as you write them:

```avon
let name = "Alice" in {"
    Hello, {name}!
    Welcome to Avon.
"}
```

Output:
```
Hello, Alice!
Welcome to Avon.
```

### Indentation and Dedent

Templates automatically dedent based on the **first line with content** (baseline indentation). You can indent templates naturally in your source code without padding the output:

```avon
let item = "widget" in
@config.yml {"
    item: {item}
    description: "A useful widget"
    version: 1.0
"}
```

The first line (`item: {item}`) has 4 spaces—that becomes the baseline. Dedent removes 4 spaces from every line, producing clean output:
```
item: widget
description: "A useful widget"
version: 1.0
```

Relative indentation is preserved, so nested structures work perfectly. This is crucial for readability in your Avon code!

### Interpolating Lists

When you interpolate a list, its items appear on separate lines:

```avon
let items = ["apple", "banana", "cherry"] in
@shopping.txt {"
    Items:
    {items}
"}
```

Output:
```
Items:
apple
banana
cherry
```

This is perfect for generating config files, markdown lists, and more.

### String vs Template

**Important distinction:**
- **Strings** (`"..."`) are values with escape sequence support. Use them for single-line content and data.
- **Templates** (`{...}`) generate multi-line output with interpolation. Use them for file content.

**String escape sequences:**
```avon
"hello\n"                  # String: \n is a newline character
"hello\tworld"             # String: \t is a tab character
"path\\to\\file"           # String: \\ is a backslash
"quote: \""                # String: \" is a quote character
```

**Template literal text:**
```avon
{"hello\n"}                # Template: literally contains the text "\n" (not a newline)
{"hello\tworld"}           # Template: literally contains "\t" (not a tab)
```

**To get a newline in a template, press Enter in the source:**
```avon
{"
hello
world
"}
# Output:
# hello
# world
```

**When to use each:**
- **Use strings for:** Data values, single-line content, escape sequences needed
- **Use templates for:** File content, multi-line output, variable interpolation needed

**Template variable capture:**
Templates capture variables from their surrounding scope at the time the template is created. This means:

```avon
let name = "Alice" in
let template = {"Hello, {name}"} in
let name = "Bob" in
template
# Still evaluates to: "Hello, Alice"
# The template captured "Alice" when it was created
```

This is important for closures and function returns—templates remember the values from when they were defined, not when they're evaluated.

### Template Escape Hatch: Variable Brace Delimiters

Avon templates use a **variable-brace delimiter system** that lets you choose how many opening braces to use. This powerful feature lets you generate code and config files cleanly, even when they contain many curly braces.

#### Why Multiple Brace Levels?

When generating code that uses braces (Lua, JSON, Terraform, HCL, Python, etc.), you need to distinguish:
- **Literal braces in the output** (e.g., `{` for a Lua table or JSON object)
- **Interpolation braces** (e.g., `{variable}` to substitute values)

Avon solves this by letting you choose how many braces delimit the template:

```avon
{" ... "}        # Single-brace: interpolate with { }, escape literals with {{
{{"  ... "}}     # Double-brace: interpolate with {{ }}, single braces are literal
{{{" ... "}}}    # Triple-brace: interpolate with {{{ }}}, double braces are literal
```

This way, you choose the delimiter that matches your content's brace density, minimizing escaping.

#### How the System Works

**Interpolation** uses exactly the same number of braces as the template opener:

```avon
{"Value: { 1 + 2 }"}              # Single-brace interpolation: { }
{{"Value: {{ 1 + 2 }}"}           # Double-brace interpolation: {{ }}
{{{" Value: {{{ 1 + 2 }}} "}}}     # Triple-brace interpolation: {{{ }}}
```

**Literal braces** are created by using more braces than the delimiter requires. The output has (k - open_count) braces:

| Delimiter | To output `{` | To output `{{` | To output `}` | To output `}}` |
|-----------|---------------|----------------|---------------|----------------|
| `{" "}` | `{{` | `{{{{` | `}}` | `}}}}` |
| `{{"  "}}` | `{{{` | `{{{{` | `}}}` | `}}}}` |
| `{{{" "}}}` | `{{{{` | `{{{{{` | `}}}}` | `}}}}}` |

#### Single-Brace Templates

Use when your output has **few or no literal braces**:

```avon
{"Value: { 1 + 2 }"}       # Output: Value: 3
{"Literal open: {{"}       # Output: Literal open: {
{"Literal close: }}"}      # Output: Literal close: }
```

**Example: Simple YAML config**
```avon
@app.yml {"
app:
  name: myapp
  debug: { debug_mode }
"}
```

#### Double-Brace Templates

Use when your output has **many literal braces** (JSON, HCL, Terraform, Lua dicts, etc.):

```avon
@config.lua {{"
local config = {
  name = "{{ app_name }}",
  debug = {{ if dev then "true" else "false" }}
}
"}}
```

**Rule:** In double-brace templates, single braces are literal (no escaping needed):

```avon
@output.json {{"
{
  "app": "{{ app_name }}",
  "nested": {
    "value": {{ port }}
  }
}
"}}
```

#### Example: Generating Lua Code

With single-brace, you must escape braces:

```avon
@config.lua {"
local config = {{
  name = "myapp",
  debug = true
}}

function init()
  return config
end
"}
```

With double-brace, braces are literal:

```avon
@config.lua {{"
local config = {
  name = "{{ app_name }}",
  debug = true
}

function init()
  return config
end
"}}
```

#### Strategic Choice: Brace Density

Choose your template delimiter based on how many braces are in your output:

| Output Type | Delimiter | Reason |
|-------------|-----------|--------|
| YAML, INI, simple configs | `{" "}` | Few braces, no escaping needed |
| Lua, shell scripts | `{" "}` | Occasional braces, light escaping |
| JSON, HCL, Terraform | `{{"  "}}` | Many braces, double-brace is cleaner |
| Python code | `{{"  "}}` or higher | Dict literals and f-strings require clean syntax |
| Extreme cases | `{{{" "}}}` | Custom DSLs with heavy brace syntax (rare) |

The key insight: **choose the delimiter that lets your template stay readable**.

See `examples/escape_hatch.av` for comprehensive demonstrations of all delimiter levels.

### Complex Interpolations

You can embed any expression in a template:

```avon
let x = 10 in
let y = 20 in
let items = ["apple", "banana", "cherry"] in {"
    Sum: {x + y}
    Max: {if x > y then x else y}
    Items: {join items ", "}
"}
```

---

## File Templates & Deployment

The real power of Avon is **file templates**: combining a file path with a template to generate files.

### Basic FileTemplate

```avon
@path/to/file.txt {"
    File content goes here
"}
```

This is a `FileTemplate` value. When you evaluate and deploy a program that returns this, Avon writes the file.

### Deploying Single Files

Create `greet.av`:

```avon
\name @greeting.txt {"
    Hello, {name}!
"}
```

Deploy:

```bash
avon deploy examples/greet.av -name Alice --root ./out --force
```

Creates: `./out/greeting.txt`

### Deploying Multiple Files

Return a list of file templates:

```avon
let name = "my-app" in
[
    @docker-compose.yml {"
        docker-compose: {name}
    "},
    @README.md {"
        # {name}
    "},
    @.gitignore {"
        __pycache__/
        node_modules/
    "}
]
```

Deploy it:

```bash
avon deploy examples/gen_files.av --root ./project --force
```

Creates all three files.

### Dynamic File Paths

Use variables in file paths:

```avon
let configs = ["dev", "prod"] in
map (\env @config-{env}.yml {"
    environment: {env}
"}) configs
```

This generates `config-dev.yml` and `config-prod.yml`.

### Important Deploy Flags

**`--root <dir>`** — Prepend this directory to all generated paths
- **Default behavior:** If `--root` is not specified, files are written relative to the current working directory where `avon` is executed
- **Required for safety:** Prevents accidental writes to system directories
- All file paths are resolved relative to this directory (or current directory if not specified)
- Example: `--root ./output` means `@config.yml` becomes `./output/config.yml`
- Example: Without `--root`, running `avon deploy config.av` from `/home/user/project/` writes `@config.yml` to `/home/user/project/config.yml`
- **Always use this flag** to keep your deployments contained and predictable

**`--force`** — Overwrite existing files without warning
- **Destructive:** Permanently replaces existing files
- No backup is created
- Use with caution, especially for production configs
- Overrides the default behavior of skipping existing files

**`--backup`** — Create a backup before overwriting
- **Safe overwrite:** Copies existing file to `filename.bak` before writing
- If backup fails (e.g., permissions), deployment aborts
- Original file remains untouched if backup fails
- Best practice for updating critical configurations

**`--append`** — Append to existing files instead of overwriting
- **Additive:** Adds new content to the end of existing files
- Useful for logs, accumulating data, or building files incrementally
- If file doesn't exist, creates it (same as normal write)

**`--if-not-exists`** — Only create file if it doesn't already exist
- **Initialization mode:** Skips files that already exist
- Useful for setup scripts that should only run once
- No warning is shown for skipped files (they're silently ignored)

**`--git <url>`** — Fetch source from a git raw URL
- Format: `user/repo/path/to/file.av`
- Fetches the file from GitHub's raw content API
- Useful for sharing templates across teams
- Example: `--git pyrotek45/avon/examples/config.av`

**`--debug`** — Show detailed debug output
- Shows lexer tokens, parser AST, and evaluator steps
- Useful for troubleshooting complex programs
- Output can be verbose—use only when needed

**Safety Guardrails:**
- By default, Avon **will not overwrite** existing files. It skips them and prints a warning.
- `--force` overrides this safety check (destructive).
- `--backup` allows overwriting but preserves the old file as `filename.bak` (safe).
- `--root` ensures you don't accidentally write to system directories (required for safety).
- If any error occurs during deployment preparation or validation, **zero files are written** (truly atomic deployment). All files are validated before any writes occur.

---

## Builtin Functions

Avon comes with a toolkit of **80+ built-in functions** for common tasks. All builtins are curried, so you can partially apply them.

These utilities make any file more powerful—whether you're generating hundreds of config files or just managing a single dotfile. You can leverage functions like `upper`, `lower`, `format_table`, `json_parse`, `html_escape`, and many more to add superpowers to any text format, even if it's just one file.

### String Operations

| Function | Example | Result |
|----------|---------|--------|
| `concat a b` | `concat "hello" " world"` | `"hello world"` |
| `upper s` | `upper "hello"` | `"HELLO"` |
| `lower s` | `lower "WORLD"` | `"world"` |
| `contains s substr` | `contains "hello" "ell"` | `true` |
| `starts_with s prefix` | `starts_with "hello" "he"` | `true` |
| `ends_with s suffix` | `ends_with "hello" "lo"` | `true` |
| `split s sep` | `split "a,b,c" ","` | `["a", "b", "c"]` |
| `replace s old new` | `replace "hello" "l" "L"` | `"heLLo"` |
| `trim s` | `trim "  hello  "` | `"hello"` |
| `length s` | `length "hello"` | `5` |
| `repeat s n` | `repeat "ab" 3` | `"ababab"` |
| `pad_left s width char` | `pad_left "7" 3 "0"` | `"007"` |
| `pad_right s width char` | `pad_right "hi" 5 " "` | `"hi   "` |
| `indent s spaces` | `indent "code" 4` | `"    code"` |

### List Operations

Lists are the heart of Avon, and Avon provides comprehensive list operations:

**Basic Operations:**
| Function | Description | Example |
|----------|-------------|---------|
| `map f list` | Apply function to each item | `map (\x x + 1) [1,2,3]` → `[2,3,4]` |
| `filter pred list` | Keep items where predicate is true | `filter (\x x > 2) [1,2,3,4]` → `[3,4]` |
| `fold f init list` | Reduce list to single value | `fold (\a \x a + x) 0 [1,2,3]` → `6` |
| `join list sep` | Join items with separator | `join ["a","b","c"] ", "` → `"a, b, c"` |
| `length list` | Get number of items | `length [1,2,3]` → `3` |

**Advanced List Operations:**
| Function | Description | Example |
|----------|-------------|---------|
| `zip list1 list2` | Pair elements from two lists | `zip [1,2,3] ["a","b","c"]` → `[[1,"a"], [2,"b"], [3,"c"]]` |
| `unzip pairs` | Split pairs into two lists | `unzip [[1,"a"], [2,"b"]]` → `[[1,2], ["a","b"]]` |
| `take n list` | Get first n elements | `take 3 [1,2,3,4,5]` → `[1,2,3]` |
| `drop n list` | Skip first n elements | `drop 2 [1,2,3,4,5]` → `[3,4,5]` |
| `split_at n list` | Split list at index | `split_at 2 [1,2,3,4,5]` → `[[1,2], [3,4,5]]` |
| `partition pred list` | Split by predicate | `partition (\x x > 2) [1,2,3,4,5]` → `[[3,4,5], [1,2]]` |
| `reverse list` | Reverse the list | `reverse [1,2,3]` → `[3,2,1]` |
| `head list` | Get first element (or `None` if empty) | `head [1,2,3]` → `1` |
| `tail list` | Get all but first element | `tail [1,2,3,4]` → `[2,3,4]` |

**Examples:**
```avon
# Zip two lists together
let numbers = [1, 2, 3] in
let letters = ["a", "b", "c"] in
zip numbers letters
# Result: [[1, "a"], [2, "b"], [3, "c"]]

# Take first 3 items
take 3 [1, 2, 3, 4, 5]      # [1, 2, 3]

# Drop first 2 items
drop 2 [1, 2, 3, 4, 5]      # [3, 4, 5]

# Split list in half
split_at 2 [1, 2, 3, 4, 5]  # [[1, 2], [3, 4, 5]]

# Partition by condition
partition (\x x > 2) [1, 2, 3, 4, 5]  # [[3, 4, 5], [1, 2]]

# Reverse a list
reverse [1, 2, 3]           # [3, 2, 1]

# Get first element
head [1, 2, 3]              # 1
head []                      # None

# Get rest of list
tail [1, 2, 3, 4]           # [2, 3, 4]
```

**Combining operations:**
```avon
let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] in
let evens = filter (\x x % 2 == 0) numbers in
let first_three_evens = take 3 evens in
reverse first_three_evens
# Result: [6, 4, 2]
```

**See also:** `examples/list_operations.av` for comprehensive examples.

### File & Filesystem

| Function | Description |
|----------|-------------|
| `readfile path` | Read entire file as string |
| `readlines path` | Read file lines as list |
| `exists path` | Check if file exists (true/false) |
| `basename path` | Get filename from path |
| `dirname path` | Get directory from path |

### HTML Generation Helpers

| Function | Description | Example |
|----------|-------------|---------|
| `html_escape s` | Escape HTML special characters | `html_escape "<div>"` → `"&lt;div&gt;"` |
| `html_tag tag content` | Wrap content in HTML tag | `html_tag "p" "text"` → `"<p>text</p>"` |
| `html_attr name value` | Create HTML attribute | `html_attr "class" "btn"` → `"class=\"btn\""` |

### Markdown Generation Helpers

| Function | Description | Example |
|----------|-------------|---------|
| `md_heading level text` | Create markdown heading | `md_heading 1 "Title"` → `"# Title"` |
| `md_link text url` | Create markdown link | `md_link "Click" "/home"` → `"[Click](/home)"` |
| `md_code code` | Wrap in inline code | `md_code "x = 1"` → `` "`x = 1`" `` |
| `md_list items` | Convert list to markdown list | `md_list ["a","b"]` → `"- a\n- b"` |

### Type Conversion & Casting

| Function | Description | Example | Result |
|----------|-------------|---------|--------|
| `to_string val` | Convert any value to string | `to_string 42` | `"42"` |
| `to_int val` | Convert to integer | `to_int "42"` | `42` |
| `to_int val` | Float to int (truncates) | `to_int 3.7` | `3` |
| `to_float val` | Convert to float | `to_float "3.14"` | `3.14` |
| `to_bool val` | Convert to boolean | `to_bool "yes"` | `true` |
| `to_bool val` | Number to bool (0=false) | `to_bool 0` | `false` |
| `format_int num width` | Format integer with zero-padding | `format_int 7 3` | `"007"` |
| `format_float num prec` | Format float with precision | `format_float 3.14159 2` | `"3.14"` |

**String to bool conversions:** `"true"`, `"yes"`, `"1"`, `"on"` -> `true`; `"false"`, `"no"`, `"0"`, `"off"`, `""` -> `false`

### Advanced List Operations

| Function | Description | Example |
|----------|-------------|---------|
| `flatmap f list` | Map then flatten | `flatmap (\x [x, x]) [1,2]` → `[1,1,2,2]` |
| `flatten list` | Flatten one level | `flatten [[1,2],[3,4]]` → `[1,2,3,4]` |

### Data & Utilities

| Function | Description | Example |
|----------|-------------|---------|
| `import path` | Load and evaluate another `.av` file | `import "lib.av"` |
| `json_parse json_str` | Parse JSON string | `json_parse "{\\"x\\": 1}"` |
| `os` | Get OS string | `os` → `"linux"`, `"macos"`, `"windows"` |

**The `import` Function and Modularity:**

Avon's simplicity enables powerful modularity. Since each file contains exactly one expression, the `import` function evaluates that expression and returns whatever value it produces. This means **any file can return any Avon type**:

- **Library files** return dictionaries of functions:
  ```avon
  # math.av
  {double: \x x * 2, triple: \x x * 3, square: \x x * x}
  ```
  ```avon
  # main.av
  let math = import "math.av" in
  math.double 21  # Returns 42
  ```

- **Data files** return dictionaries or lists:
  ```avon
  # config.av
  {host: "localhost", port: 8080, debug: true}
  ```
  ```avon
  # main.av
  let config = import "config.av" in
  config.host  # Returns "localhost"
  ```

- **Generator files** return FileTemplates or lists of FileTemplates:
  ```avon
  # deploy.av
  @config.yml {"host: localhost"}
  ```

- **Any other type** works too—strings, numbers, functions, etc.

This simple model makes Avon naturally modular: organize code into reusable files, each returning the value that makes sense for its purpose.

### Example: String & List Combination

Generate a configuration file with multiple items:

```avon
let items = ["api", "worker", "scheduler"] in
let formatted = map (\item concat "service: " item) items in
@services.conf {"
    Services:
    {formatted}
"}
```

---

## CLI Usage

### Basic Commands

**`eval` - Evaluate and Print:**
```bash
avon eval examples/map_example.av
avon eval examples/greet.av -name Alice  # Can pass arguments!
```
- Evaluates the Avon program
- **Accepts arguments** - if the file evaluates to a function, you can pass arguments
- Prints the result to stdout
- **Does NOT write any files** - this is read-only
- Use this to test your program before deploying
- If the result is a FileTemplate or list of FileTemplates, it shows the paths and content that would be generated

**`deploy` - Generate Files:**
```bash
avon deploy examples/site_generator.av --root ./output --force
avon deploy examples/greet.av -name Alice --root ./out --force  # Can pass arguments!
```
- Evaluates the Avon program
- **Accepts arguments** - if the file evaluates to a function, you can pass arguments
- If the result is a FileTemplate or list of FileTemplates, writes them to disk
- Requires `--root` to specify where files should be written (safety feature)
- By default, skips existing files (use `--force` or `--backup` to overwrite)

**`run` - Evaluate Code String:**
```bash
avon run 'map (\x x * 2) [1, 2, 3]'
```
- Evaluates a code string directly without a file
- Useful for quick one-off calculations
- Prints the result (does not deploy files)
- Code must be quoted to prevent shell interpretation

**`repl` - Interactive REPL:**
```bash
avon repl
```
- Starts an interactive shell for exploring Avon
- See the [Interactive REPL](#interactive-repl) section for details

**`doc` - Builtin Documentation:**
```bash
avon doc
```
- Shows documentation for all builtin functions
- Lists function signatures and descriptions

**`version` - Version Information:**
```bash
avon version
```
- Shows the Avon version number

**`help` - Help Message:**
```bash
avon help
# or
avon --help
```
- Shows usage information and available commands

### Passing Arguments

Avon allows you to pass values into your program from the command line. This works with **both `eval` and `deploy` commands**. When your Avon file evaluates to a function, the CLI automatically applies the arguments you provide.

**Important distinction:** 
- **Single-dash arguments** (`-name`, `-env`, `-x`, `-c`) are **function parameters** - passed to your Avon function
- **Double-dash arguments** (`--force`, `--root`, `--debug`) are **CLI options** - control how Avon behaves

This means you can use any name for your function parameters (including single letters like `-a`, `-b`, `-c`) without conflicts with CLI flags. Only `--` (double-dash) arguments are reserved for CLI options.

#### How It Works

**When a file evaluates to a function:**
1. Avon evaluates the file's expression
2. If the result is a function, Avon checks for arguments you provided
3. Arguments are applied to the function (named arguments first, then positional)
4. If the function still needs more arguments, default values are used (if available)
5. The process continues until the function is fully applied or no more arguments are available
6. The final result is then used:
   - **With `eval`**: The result is printed
   - **With `deploy`**: If the result is a FileTemplate or list of FileTemplates, files are written

**Example: Simple function**
```avon
# math.av
\x \y x + y
```

**Using `eval` with arguments:**
```bash
avon eval math.av 5 3
# Output: 8

avon eval math.av -x 5 -y 3
# Output: 8
```

**Using `deploy` with arguments:**
```bash
# This won't work - the result is a number, not a FileTemplate
avon deploy math.av 5 3
# Error: Expected FileTemplate or list of FileTemplates
```

**Example: Function that returns a FileTemplate**
```avon
# greet.av
\name @greeting.txt {"
    Hello, {name}!
"}
```

**Using `eval` to preview:**
```bash
avon eval greet.av -name Alice
# Output:
# --- /greeting.txt ---
# Hello, Alice
```

**Using `deploy` to write files:**
```bash
avon deploy greet.av -name Alice --root ./output --force
# Creates: ./output/greeting.txt with "Hello, Alice"
```

#### 1. Named Arguments

If your main expression is a function with parameters, you can pass values using `-parameter_name value`.

**Program (`greet.av`):**
```avon
# Function with two parameters
\name \role @greeting.txt {"
    Hello, {name}!
    Role: {role}
"}
```

**With `eval` (preview):**
```bash
avon eval greet.av -name "Alice" -role "Admin"
# Shows what would be generated
```

**With `deploy` (write files):**
```bash
avon deploy greet.av -name "Alice" -role "Admin" --root ./out --force
# Actually writes the file
```

**How it works:**
- The CLI looks for `-name` and passes "Alice" to the `name` parameter
- It looks for `-role` and passes "Admin" to the `role` parameter
- Arguments are received as **strings** (see "Argument Types" below)
- Arguments are type-checked at runtime when used in your function

#### 2. Positional Arguments

You can also pass arguments positionally, without the parameter names. This maps command line arguments to function parameters in order.

**Command:**
```bash
# Maps "Alice" to name, "Admin" to role
avon eval greet.av "Alice" "Admin"
avon deploy greet.av "Alice" "Admin" --root ./out --force
```

**How positional arguments work:**
- Arguments are applied in the order they appear
- Named arguments are applied first, then positional arguments fill remaining parameters
- If you mix named and positional, named arguments take priority

**Example:**
```avon
# config.av
\env \port \debug @config.yml {"
    env: {env}
    port: {port}
    debug: {debug}
"}
```

```bash
# All positional
avon eval config.av "prod" "8080" "true"

# Mix named and positional (named takes priority)
avon eval config.av -env dev "9090" "false"
# env=dev (from named), port="9090" (positional), debug="false" (positional)
```

**Recommendation:** Use named arguments for clarity, especially when you have multiple parameters or default values.

#### 3. Default Values

Parameters can have default values using the `?` syntax. If an argument is not provided, the default is used.

**Program (`config.av`):**
```avon
\env ? "dev" \port ? 8080 @config.yml {"
    env: {env}
    port: {port}
"}
```

**Usage with `eval`:**
```bash
# Use defaults
avon eval config.av
# env="dev", port=8080

# Override some
avon eval config.av -env prod
# env="prod", port=8080

# Override all
avon eval config.av -env prod -port 9090
# env="prod", port=9090
```

**Usage with `deploy`:**
```bash
# Same syntax works for deploy
avon deploy config.av --root ./out --force
avon deploy config.av -env prod --root ./out --force
avon deploy config.av -env prod -port 9090 --root ./out --force
```

**How defaults work:**
- If a named argument is provided, it's used
- If a positional argument is available, it's used
- If neither is provided, the default value is used
- If no default exists and no argument is provided, an error is shown

#### 4. Mixing Named and Positional

While possible, mixing named and positional arguments can be confusing. Avon prioritizes named arguments first, then fills remaining parameters with positional arguments in order.

**Example:**
```avon
# multi.av
\a \b \c \d [a, b, c, d]
```

```bash
# Mixing named and positional
avon eval multi.av -a 1 -c 3 2 4
# a=1 (named), b=2 (positional, first unused), c=3 (named), d=4 (positional, second unused)
# Result: [1, 2, 3, 4]
```

**Important:** Single-dash arguments (like `-a`, `-c`) are always treated as named function parameters, not CLI flags. Only double-dash arguments (like `--force`, `--root`) are CLI options. This means you can use any single-letter or short name for your function parameters without conflicts.

**Best Practice:** Stick to either all named or all positional arguments for a single command invocation to avoid confusion.

#### 5. Argument Types

> **Important:** All command-line arguments passed to your Avon program are received as **strings**—even if you intend to use them as numbers or booleans, you must explicitly convert them inside your program.

**Example:**
```bash
avon eval math.av -x 5 -y 40
```

Both `x` and `y` are provided as strings: `"5"` and `"40"`. You should convert them as needed:

```avon
# math.av
\x \y to_int x + to_int y
```

**Why?** This ensures correct type handling and prevents subtle bugs when performing arithmetic or boolean logic. The CLI doesn't know what types your function expects, so it passes everything as strings for maximum flexibility.

**Type conversion examples:**
```avon
# Convert to number
\port to_int port

# Convert to boolean
\debug to_bool debug

# Convert to float
\ratio to_float ratio
```

#### 6. Complete Examples

**Example 1: Preview with `eval`, then deploy**
```avon
# app.av
\name \env ? "dev" @config-{env}.yml {"
    app_name: {name}
    environment: {env}
"}
```

```bash
# Step 1: Preview what will be generated
avon eval app.av -name "myapp" -env prod
# Output shows the FileTemplate that would be created

# Step 2: Actually deploy it
avon deploy app.av -name "myapp" -env prod --root ./configs --force
```

**Example 2: Function that needs multiple arguments**
```avon
# deploy.av
\app \env \version @deploy-{app}-{env}.yml {"
    app: {app}
    environment: {env}
    version: {version}
"}
```

```bash
# All named arguments
avon deploy deploy.av -app api -env prod -version 1.2.3 --root ./out --force

# All positional
avon deploy deploy.av api prod 1.2.3 --root ./out --force

# Mix (not recommended but works)
avon deploy deploy.av -app api prod 1.2.3 --root ./out --force
```

**Example 3: Function with defaults**
```avon
# service.av
\name \replicas ? 3 \port ? 8080 @service-{name}.yml {"
    name: {name}
    replicas: {replicas}
    port: {port}
"}
```

```bash
# Use all defaults (but name is required)
avon eval service.av -name webapp
# replicas=3, port=8080

# Override some
avon eval service.av -name webapp -replicas 5
# replicas=5, port=8080

# Override all
avon eval service.av -name webapp -replicas 5 -port 9090
# replicas=5, port=9090
```

**Example 4: Non-function files**
```avon
# data.av
{host: "localhost", port: 8080}
```

```bash
# No arguments needed - file doesn't evaluate to a function
avon eval data.av
# Output: {host: "localhost", port: 8080}

# Arguments are ignored if result is not a function
avon eval data.av -x 5
# Still outputs: {host: "localhost", port: 8080}
# (The -x argument is ignored)
```

#### Summary

- **Both `eval` and `deploy` accept arguments** - use `eval` to preview, `deploy` to write files
- **When a file evaluates to a function**, arguments are automatically applied
- **Named arguments** use `-param value` syntax
- **Positional arguments** are passed in order without names
- **Default values** are used when arguments aren't provided
- **All arguments are strings** - convert them in your code with `to_int`, `to_bool`, etc.
- **Arguments work the same** for both `eval` and `deploy` - the only difference is what happens with the final result

### Interactive REPL

The REPL (Read-Eval-Print Loop) is an interactive shell for exploring Avon. It's perfect for learning the language, testing expressions, and debugging. The REPL maintains a persistent symbol table, so variables you define persist across expressions, making it ideal for building up complex computations step by step.

**Why Use the REPL?**

The REPL is an essential tool for Avon development:

- **Learning**: Explore Avon syntax and builtins interactively without creating files
- **Prototyping**: Test expressions quickly before adding them to your programs
- **Debugging**: Isolate problematic code and test fixes immediately
- **Exploration**: Discover how functions work with different inputs
- **Quick Calculations**: Perform one-off computations or transformations
- **Type Checking**: Verify types of expressions before using them in files

**Starting the REPL:**
```bash
avon repl
```

You'll see:
```
Avon REPL - Interactive Avon Shell
Type ':help' for commands, ':exit' to quit

avon> 
```

**Basic Usage:**
```avon
avon> 1 + 2
3 : Number

avon> let x = 42 in x * 2
84 : Number

avon> map (\x x * 2) [1, 2, 3]
[2, 4, 6] : List

avon> typeof "hello"
String : String
```

**REPL Commands:**
- `:help` or `:h` - Show help and available commands
- `:let <name> = <expr>` - Store a value in the REPL (persists across commands)
- `:vars` - List all user-defined variables
- `:inspect <name>` - Show detailed information about a variable
- `:unlet <name>` - Remove a user-defined variable
- `:read <file>` - Read and display file contents (any path allowed)
- `:run <file> [--debug]` - Evaluate file and display result (doesn't modify REPL state)
- `:eval <file>` - Evaluate file and merge Dict keys into REPL (if result is a Dict)
- `:preview <file> [--debug]` - Preview what would be deployed without writing files
- `:deploy <file> [flags...]` - Deploy a file (supports same flags as CLI: `--root <dir>`, `--force`, `--backup`, `--append`, `--if-not-exists`, `--debug`, `-param value`)
- `:deploy-expr <expr> [--root <dir>]` - Deploy the result of an expression
- `:write <file> <expr>` - Write expression result to file
- `:history` - Show command history (last 50 entries)
- `:save-session <file>` - Save REPL state (variables) to file
- `:load-session <file>` - Load REPL state from file
- `:assert <expr>` - Assert that expression evaluates to true
- `:test <expr> <expected>` - Test that expression equals expected value
- `:benchmark <expr>` - Measure evaluation time for an expression
- `:benchmark-file <file>` - Measure evaluation time for a file
- `:watch <name>` - Watch a variable and show when it changes (works with :let and expressions)
- `:unwatch <name>` - Stop watching a variable
- `:pwd` - Show current working directory
- `:list [dir]` - List directory contents (shows current directory path)
- `:cd <dir>` - Change working directory
- `:sh <command>` - Execute shell command
- `:doc` - Show all available builtin functions and REPL commands
- `:doc <name>` - Show detailed documentation for a builtin function or REPL command
  - Example: `:doc map` - Shows documentation for the `map` builtin function
  - Example: `:doc pwd` - Shows documentation for the `:pwd` REPL command
  - Example: `:doc read` - Shows documentation for the `:read` REPL command
- `:type <expr>` - Show the type of an expression
- `:clear` - Clear all user-defined variables (resets to initial state)
- `:exit` or `:quit` or `:q` - Exit the REPL

**Keyboard Shortcuts:**
- `↑` / `↓` - Navigate command history (in-memory only, no file saved)
- `Tab` - Tab completion for REPL commands and filenames
- `Ctrl+A` - Move to beginning of line
- `Ctrl+E` - Move to end of line
- `Ctrl+K` - Delete from cursor to end of line
- `Ctrl+U` - Delete from cursor to beginning of line
- `Ctrl+F` - Move forward one character
- `Ctrl+B` - Move backward one character
- `Ctrl+W` - Delete word backward
- `Ctrl+L` - Clear screen

**Example 1: Using Persistent Variables**

The REPL supports persistent variables that persist across commands:

```avon
avon> :let double = \x x * 2
Stored: double : Function

avon> :let numbers = [1, 2, 3, 4, 5]
Stored: numbers : List

avon> map double numbers
[2, 4, 6, 8, 10] : List

avon> :vars
User-defined variables:
  double : Function
  numbers : List = [5 items]

avon> :inspect numbers
Variable: numbers
  Type: List
  Length: 5
  Items:
    [0]: 1
    [1]: 2
    [2]: 3
    [3]: 4
    [4]: 5

avon> :doc
Available builtin functions (use :doc <name> for details):
  assert          concat          contains        debug
  filter          flatten         flatmap         fold
  map             ...

Available REPL commands (use :doc <command> for details):
  :help            :exit            :clear           :vars            :let             :inspect       
  :unlet           :read            :run             :eval            :preview         :deploy        
  :deploy-expr     :write           :history         :save-session     :load-session    :assert        
  :test            :benchmark       :benchmark-file  :watch           :unwatch         :pwd             :list            :cd            
  :doc             :type            :sh            

Tip: Use :doc <name> to see detailed documentation for any builtin function or REPL command.

avon> :doc pwd
:pwd
  Show the current working directory.
  Example: :pwd

avon> :doc map
map :: (a -> b) -> [a] -> [b]
  Transform each item in list.
  Example: map (\x x * 2) [1, 2, 3] -> [2, 4, 6]
```

**Example 2: File Operations and Deployment**

The REPL supports reading, evaluating, and deploying files:

```avon
avon> :read config.av
let port = 8080 in
@config.yml {"port: {port}"}

avon> :preview config.av
Would deploy 1 file(s):
  Path: config.yml
  Content:
port: 8080

avon> :deploy config.av --root ./output --backup
Deployment completed successfully

avon> :let env = "prod"
Stored: env : String

avon> :deploy-expr @config-{env}.yml {"env: {env}"} --root ./output
Deployed: ./output/config-prod.yml
Deployment completed successfully
```

**Example 3: Testing File Templates**

You can test file generation without actually writing files:

```avon
avon> @test.txt {"Hello, {os}"}
FileTemplate:
  Path: /test.txt
  Content:
Hello, linux

avon> let name = "Alice" in @greeting.txt {"
...>   Hello, {name}!
...>   Welcome to Avon.
...> "}
FileTemplate:
  Path: /greeting.txt
  Content:
  Hello, Alice!
  Welcome to Avon.
```

**Example 4: Debugging with trace and debug**

Use built-in debugging tools interactively:

```avon
avon> trace "intermediate" (1 + 2)
[TRACE] intermediate: 3
3 : Number

avon> let result = map (\x trace "doubling" (x * 2)) [1, 2, 3] in result
[TRACE] doubling: 2
[TRACE] doubling: 4
[TRACE] doubling: 6
[2, 4, 6] : List

avon> debug [1, 2, 3]
[DEBUG] List([Number(1.0), Number(2.0), Number(3.0)])
[1, 2, 3] : List
```

**Example 4: Type Checking**

Verify types before using expressions in your files:

```avon
avon> :type [1, 2, 3]
Type: List

avon> :type "hello"
Type: String

avon> :type map (\x x * 2)
Type: Function

avon> let config = {host: "localhost", port: 8080} in :type config
Type: Dict
```

**Example 5: Testing Function Compositions**

Build and test function pipelines interactively:

```avon
avon> let add_one = \x x + 1 in add_one
Function : Function

avon> let double = \x x * 2 in double
Function : Function

avon> let pipeline = \x double (add_one x) in pipeline 5
12 : Number

avon> map pipeline [1, 2, 3]
[4, 6, 8] : List
```

**Example 6: Working with Dictionaries**

Test dictionary operations interactively:

```avon
avon> let config = {host: "localhost", port: 8080, debug: true} in config
{host: "localhost", port: 8080, debug: true} : Dict

avon> config.host
localhost : String

avon> keys config
["host", "port", "debug"] : List

avon> has_key config "port"
true : Bool

avon> let updated = set config "timeout" 30 in updated
{host: "localhost", port: 8080, debug: true, timeout: 30} : Dict
```

**Multi-line Input:**

The REPL automatically detects incomplete expressions and waits for completion. This is especially useful for `let` expressions, `if` statements, and complex structures:

```avon
avon> let config = {
    >   host: "localhost",
    >   port: 8080,
    >   debug: true
    > } in config.host
localhost : String

avon> if true
    > then "yes"
    > else "no"
yes : String

avon> let x = 10 in
    > let y = 20 in
    > x + y
30 : Number
```

Notice how the continuation prompt (`    >`) aligns with the initial prompt, making it easy to see the structure of your multi-line expressions.

**Error Handling:**

The REPL provides clear error messages and continues running after errors:

```avon
avon> 1 + "hello"
Error: +: type mismatch
  Expected: Number, Number
  Got: Number, String
  At line 1

avon> let x = 42 in x
42 : Number
```

After an error, you can continue working—the REPL doesn't crash.

**Best Practices:**

1. **Test before writing files**: Use the REPL to verify expressions work before adding them to your `.av` files
2. **Build incrementally**: Define functions and variables step by step, checking each one
3. **Use `:doc` to explore builtins and commands**: See all available functions and REPL commands with `:doc`, or get details with `:doc <name>` (works for both builtin functions and REPL commands like `:doc pwd` or `:doc map`)
4. **Use `:type` for verification**: Check types of complex expressions
5. **Clear when needed**: Use `:clear` to reset if you make mistakes

**When to Use REPL vs Files:**

- **Use REPL for**: Quick tests, learning, debugging, one-off calculations
- **Use files for**: Production code, reusable programs, version-controlled configs

The REPL is your interactive playground—use it liberally to explore and experiment!


### Command-Line Flags

| Flag | Purpose | Example |
|------|---------|---------|
| `eval` | Evaluate program and print result | `avon eval program.av` |
| `deploy` | Generate files using result | `avon deploy program.av ...` |
| `run` | Evaluate code string directly | `avon run '1 + 1'` |
| `repl` | Start interactive REPL | `avon repl` |
| `-param value` | Named argument for deploy | `deploy ... -name alice` |
| `--root <dir>` | Prepend directory to all generated paths | `--root ./output` |
| `--force` | Overwrite existing files without warning | `--force` |
| `--append` | Append to existing files instead of overwriting | `--append` |
| `--if-not-exists` | Only write file if it doesn't already exist | `--if-not-exists` |
| `--git <url>` | Fetch and use git raw URL as source | `avon deploy --git user/repo/file.av` |

### Real-World Examples

Generate site with custom name:

```bash
avon deploy examples/site_generator.av -name "My Site" --root ./website --force
```

Generate config files for all environments:

```bash
avon deploy examples/config_gen.av --root ./configs --force
```

Fetch and deploy a program from GitHub:

```bash
avon deploy --git pyrotek45/avon/examples/site_generator.av --root ./site
```

Fetch and evaluate a program from GitHub:

```bash
avon eval --git pyrotek45/avon/examples/string_functions.av
```

### Single File in Git, Many Deployments

A powerful pattern with Avon is to keep **one template file in git** and let each environment, developer, or user deploy customized configs via CLI arguments. This is especially useful for **dotfiles** and shared configurations.

**Example: Dotfile Template (`vimrc.av` in git):**
```avon
\username ? "developer" \theme ? "solarized" @.vimrc {"
  " Vim configuration for {username}
  set number
  set expandtab
  set tabstop=4
  colorscheme {theme}
  
  " User-specific settings
  let mapleader = " "
"}
```

**Usage:**
```bash
# Developer laptop
avon deploy --git user/repo/vimrc.av --root ~ -username alice -theme solarized

# Server
avon deploy --git user/repo/vimrc.av --root ~ -username admin -theme default
```

**Example: App Config (`config.av` in git):**
```avon
\env ? "dev" \user ? "developer" @config-{env}.yml {"
    user: {user}
    env: {env}
"}
```

**Usage:**
```bash
# Developer machine
avon deploy --git user/repo/config.av --root ~/.config/myapp -env dev -user alice

# Production server
avon deploy --git user/repo/config.av --root /etc/myapp -env prod -user service
```

You keep a single, versioned Avon program as the source of truth, and use a combination of **default parameters** and **CLI arguments** to adapt it to each machine or environment. This makes sharing dotfiles and configs incredibly easy—just share one file in git, and everyone can deploy their customized version.

---

## Error handling and debugging

### Runtime Type Safety

Avon uses **runtime type checking** rather than static compile-time types. This flexible approach brings type safety to any file without the complexity of compile-time type systems.

**Key behavior:** Avon **does not deploy** if there's a type error. If a type error occurs during evaluation, deployment simply doesn't happen. This protects you from bad or improperly typed configurations being written to disk.

**How type checking works:**
- Types are checked at runtime when operations are performed
- If you try to add a string to a number, Avon immediately reports a type error
- If you try to call a non-function value, Avon reports a type error
- Type errors prevent evaluation from completing, so no files are written

**Error message format:**
Avon provides detailed error messages that help you fix issues quickly:

```
Error: +: type mismatch
  Expected: Number, Number
  Got: Number, String
  At line 5
```

This tells you:
- **What failed:** The `+` operator
- **What was expected:** Two numbers
- **What you provided:** A number and a string
- **Where it failed:** Line 5

**Lexing and parsing errors:**
Syntax errors are caught during parsing and include:
- Line number where the error occurred
- Context around the error
- What was expected vs. what was found

Example:
```
Parse error: expected 'in' after let binding
  At line 3
  let x = 10
         ^
```

**Deployment errors:**
If an error occurs during file materialization (writing files), Avon:
- Stops immediately with zero files written (truly atomic deployment - all files validated before any writes)
- Reports exactly what failed
- Shows how many files were written before the error
- Does not leave partial deployments

**Error recovery:**
- After an error, you can fix the issue and try again
- No files are left in an inconsistent state
- Use `--debug` flag for more detailed error information

### Debugging Tools

Avon provides comprehensive debugging tools that work for both complex infrastructure projects and simple single-file configs:

**`trace "label" value`** — Print labeled values to stderr while the program runs, returning the value unchanged so evaluation can continue. Perfect for inspecting intermediate values in your computation pipeline.

**`debug value`** — Pretty-print the value structure to stderr, also returning the value unchanged. Useful for inspecting complex structures like lists, dicts, or nested data.

**`assert condition value`** — Validate conditions early. For example:
- `assert (is_string x) x` — Assert x is a string
- `assert (x > 0) x` — Assert x is positive
- `assert (length xs > 0) xs` — Assert list is not empty

**`--debug` flag** — Use with `avon eval` to show detailed lexer, parser, and evaluator debug output. This provides deep insight into the execution process when you need to troubleshoot complex issues.

These tools ensure that whether you're debugging a simple type mismatch in a single config file or inspecting complex list structures in a multi-file generator, you have the necessary feedback to streamline your workflow.

---

## Best Practices

### Write Clear, Composable Code

Break complex logic into smaller functions:

```avon
let capitalize = \s upper s in
let add_prefix = \prefix \s concat prefix s in
let format_item = add_prefix "Item: " in
let items = ["apple", "banana", "cherry"] in
map format_item items
```

This is more readable than nesting everything in one expression.

### Test Before Deploying

Always test your program with `eval` first:

```bash
# Test the logic
avon eval examples/gen_config.av

# Once satisfied, deploy
avon deploy examples/gen_config.av --root ./out --force
```

### Use Named Arguments

For functions with multiple parameters, use named arguments for clarity:

```bash
# Good: clear what each argument means
avon deploy program.av -app myservice -env prod -version 1.0

# Less clear: position-dependent
avon deploy program.av myservice prod 1.0
```

### Always Use `--root`

Avoid accidentally writing to system directories:

```bash
# Good: files go to ./generated/
avon deploy program.av --root ./generated --force

# Risky: files go to absolute paths
avon deploy program.av --force
```

### Keep Templates Readable

Indent templates nicely in your source code. Avon's dedent feature handles the indentation:

```avon
@config.yml {"
    database:
      host: localhost
      port: 5432
      name: myapp
"}
```

### Return Lists for Multiple Files

When generating multiple files, return a list of file templates:

```avon
let make_file = \name @{name}.txt {"{name}"} in
map make_file ["a", "b", "c"]
# Returns three file templates
```

---

## Safety & Security

Avon is built with safety as a priority. It includes robust guardrails to prevent accidental data loss and secure mechanisms for handling sensitive information.

### Secrets Management

**Security Rule #1:** Never hardcode secrets (API keys, passwords, tokens) in your Avon source files.

Use the `env_var` function to read secrets from the environment at runtime.

```avon
let db_password = env_var "DB_PASSWORD" in
@config.yml {"
  database:
    host: localhost
    password: {db_password}
"}
```

**How it works:**
1. Export the variable in your shell: `export DB_PASSWORD="my-secret-pass"`
2. Run Avon: `avon deploy config.av`

**Fail-Safe Behavior:**
If the environment variable `DB_PASSWORD` is missing, `env_var` will **fail immediately** with an error. This prevents you from accidentally deploying a configuration with empty or missing secrets.

For optional variables, use `env_var_or`:
```avon
let port = env_var_or "PORT" "8080" in
# Uses "8080" if PORT env var is not set
```

### Deployment Safety

Avon's deployment process is designed to be **truly atomic** and fail-safe. When deploying a list of FileTemplates, if any file cannot be written, **zero files are written**.

**1. Three-Phase Atomic Deployment**

Avon uses a three-phase approach to ensure atomicity:

**Phase 1: Preparation & Validation**
- All paths are validated for security (no path traversal)
- All parent directories are created
- No type errors occurred during evaluation

If **any** error occurs during this phase, Avon aborts immediately. **Zero files are written.**

**Phase 2: Write Validation**
Before writing any files, Avon validates that **all** files can be written:
- For existing files: Verifies they can be opened for writing (checks permissions)
- For backup operations: Verifies backup location is writable
- For new files: Verifies parent directories are writable

If **any** file fails validation, Avon aborts immediately. **Zero files are written.**

This ensures that when deploying a list like `[@/a.txt {...}, @/b.txt {...}, @/c.txt {...}]`, if `b.txt` cannot be written (e.g., read-only file), then `a.txt` and `c.txt` are also not written. The deployment is truly atomic.

**Phase 3: Writing**
Only after all files pass validation does Avon proceed to write them. Files are written sequentially, but since all have been validated, write failures are extremely rare (e.g., disk full during write).

**2. Directory Creation Checks**
If creating a directory fails (e.g., due to permissions), deployment aborts before any files are written.

**3. Write Error Handling**
If a file write fails during Phase 3 (e.g., disk full), Avon stops immediately and reports exactly what happened. However, this is rare since all files are validated in Phase 2.

### Preventing Accidental Overwrites

By default, `avon deploy` is **conservative**. It will **skip** any file that already exists on disk and print a warning.

To change this behavior, you must explicitly opt-in:

| Flag | Behavior | Safety Level |
|------|----------|--------------|
| (none) | Skip existing files | **Safest** |
| `--backup` | Backup existing file to `.bak`, then overwrite | **Safe** |
| `--force` | Overwrite existing files immediately | **Destructive** |
| `--append` | Append to existing files | **Additive** |

**The Backup Feature (`--backup`)**
Use `--backup` when you want to update files but keep a safety copy of the old version.

```bash
avon deploy config.av --backup
```

If `config.yml` exists, Avon will:
1. Copy `config.yml` to `config.yml.bak`
2. Write the new content to `config.yml`

If the backup fails (e.g., permissions), the deployment aborts and the original file is untouched.

### Security Features

**Path Traversal Protection:**
- All file operations (`readfile`, `import`, `fill_template`) block paths containing `..` (parent directory)
- When using `--root`, paths are validated to ensure they stay within the root directory
- Absolute paths without `--root` are blocked if they contain `..`

**File Size Limits:**
- `readfile`: Maximum 10MB per file (prevents DoS from large files)
- `import`: Maximum 1MB per source file (prevents DoS from large source files)
- `fill_template`: Maximum 10MB per template file

**Recursion Depth Limits:**
- Default limit: 10,000 function calls (prevents stack overflow and infinite loops)
- Configurable via `--recursion-limit <N>` flag
- Protects against accidental infinite recursion and malicious code

**Input Validation:**
- All file paths are validated before access
- Template interpolation is sandboxed (no arbitrary code execution)
- Type checking prevents many classes of errors

### Best Practices for Safety

1. **Always use `--root`** to confine deployment to a specific directory:
   ```bash
   avon deploy site.av --root ./build
   ```
   This prevents accidental writes to system directories like `/etc` or `~`.

2. **Use `env_var`** for all credentials.

3. **Prefer `--backup` over `--force`** when updating critical configurations.

4. **Test with `avon eval` first** to inspect the output before deploying.

5. **Recursion is not supported** - use iterative solutions with `fold`, `map`, and `filter` instead

---

## Real-World Examples

### Example 1: Site Generator

See `examples/site_generator.av`. This generates a full website with multiple HTML pages, including:
- Shared CSS styling
- Navigation links between pages
- Dynamic content interpolation

```bash
avon deploy examples/site_generator.av --root ./website --force
```

### Example 2: Neovim Configuration

See `examples/neovim_init.av`. Advanced example featuring:
- Conditional plugin loading
- LSP and Treesitter configuration
- Complex keybinding generation
- Concatenation of plugin lists

### Example 3: Emacs Configuration

See `examples/emacs_init.av`. Demonstrates:
- Complex conditional logic
- String operations (concat, conditional expressions)
- Multi-category package management
- Feature toggles (LSP, Org Mode, etc.)

### Example 4: Docker Compose Generator

See `examples/docker_compose_gen.av`. Shows:
- Multi-service configuration
- Environment variable interpolation
- Volume and network definitions
- Health check configuration

### Example 5: Kubernetes Manifests

See `examples/kubernetes_gen.av`. Comprehensive example with:
- Multiple Kubernetes resource types
- ConfigMaps and Secrets
- Deployment with probes and resource limits
- Ingress, Service, and HPA configurations

### Example 6: GitHub Actions Workflow

See `examples/github_actions_gen.av`. Demonstrates:
- Conditional job configuration
- Matrix builds and multi-file generation
- Secrets and environment variable handling
- Complex nested YAML structures

### Example 7: Package.json Generator

See `examples/package_json_gen.av`. Shows:
- JSON generation from Avon code
- Conditional dependency lists
- NPM script generation
- Dynamic package configuration

### Example 8: Escape Hatch Demonstration

See `examples/escape_hatch.av`. Comprehensive example of the template escape hatch:
- Single-brace templates: `{" ... "}` with `{{ }}` for literal braces
- Double-brace templates: `{{" ... "}}` with `{{{ }}}` for literal braces
- Interpolation and literal brace sequences side-by-side
- Use this as a reference when generating code with lots of braces

---

## Troubleshooting

### Common Errors

**"expected '\"' after opening braces"**  
This means a template isn't properly quoted. Templates require the syntax `{...}` with literal content or quotes if you need special formatting.

**"unexpected EOF"**  
You have an unclosed expression, list, or template. Check your brackets and braces.

**"undefined identifier"**  
You referenced a variable that doesn't exist. Check spelling and make sure it's in scope (within a `let` binding or function parameter).

### Escape Hatch Troubleshooting

**Problem:** My literal braces aren't showing up.

**Solution:** Remember the rule: use one MORE brace than the template's opening count to get a literal brace.

```avon
# Wrong (in a single-brace template, { starts interpolation)
@f.txt {"name: {"}    # Tries to interpolate {, expects closing }

# Correct (use {{ to escape)
@f.txt {"name: {{"}   # Outputs: name: {
```

**Problem:** I have lots of braces and it's getting confusing.

**Solution:** Use a double-brace template to reduce brace nesting:

```avon
# Single-brace (awkward with 3+ braces)
@f.txt {"obj: {{{x}}}}"}   # Hard to count!

# Double-brace (clearer)
@f.txt {{"obj: {{{x}}}}"}}  # Easier to read
```

**Problem:** Interpolation not working as expected.

**Solution:** Verify you're using the correct brace count:
- Single-brace template: use `{ expr }` for interpolation
- Double-brace template: use `{{ expr }}` for interpolation

```avon
# Single-brace template
@f.txt {"Result: { 5 + 5 }"}     # Works
@f.txt {"Result: {{ 5 + 5 }}"}   # No interpolation, just literals

# Double-brace template
@f.txt {{"Result: { 5 + 5 }"}}   # No interpolation
@f.txt {{"Result: {{ 5 + 5 }}"}} # Works
```

### Debugging Tips

1. **Break it into pieces:** Use `let` to name intermediate values:
   ```avon
   let step1 = map f list in
   let step2 = filter pred step1 in
   step2
   ```

2. **Print intermediate values:** Use `eval` to see what expressions evaluate to:
   ```bash
   avon run 'let x = [1,2,3] in map (\n n * 2) x' 
   ```

3. **Check file generation:** Before using `deploy`, check if files will be generated where you expect:
   ```bash
   avon eval program.av  # Shows what will be generated
   ```

4. **Isolate escape hatch issues:** Test brace escaping independently:
   ```bash
   avon run '@t.txt {"{{ {{{{ }}}}"}' 
   # Outputs: { {{{ }}}
   ```

---

## Next Steps

Ready to build something? Here are some ideas:

1. **Configuration Generator:** Generate config for multiple services (Redis, PostgreSQL, etc.)
2. **Project Scaffolder:** Create project structure for a new Node/Python/Rust project
3. **CI/CD Automation:** Generate GitHub Actions, GitLab CI, or other CI workflows
4. **Infrastructure as Code:** Generate Terraform, Ansible, or CloudFormation templates
5. **Documentation:** Auto-generate README files, API docs, or changelog templates

---

If you have questions or want to contribute examples, the Avon project welcomes contributions! Check out the repository for more details.

Happy generating!

