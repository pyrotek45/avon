# Avon — The Modern Template Language for Developers

Welcome to Avon! A template language for people who have better things to do than copy-paste.

Avon is designed for developers who are tired of copy-pasting. Whether you're building Kubernetes manifests, setting up CI/CD pipelines, or generating boilerplate code, Avon turns repetitive tasks into elegant, maintainable code. Life's too short to manually update 47 YAML files.

Avon is a general-purpose tool that handles everything from complex infrastructure projects to simple single-file configs. It's a workflow layer that makes any file more maintainable and shareable. Avon brings variables, functions, and 137 built-in utilities to any text format.

> Tip: Throughout this guide, look at the `examples/` directory for real-world use cases. Each example demonstrates practical Avon patterns you can adapt for your own projects.

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
     - Logical (`&&`, `||`, `not`)
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
   - File I/O and Globbing
     - The File I/O Pipeline
     - Core File Functions
     - Key Behavior: json_parse
     - Practical Glob Examples
     - Practical File I/O Patterns
     - Edge Cases and Error Handling
   - Builtins for Lists (comprehensive list operations)

6. **[Templates](#templates)**
   - Basic Template Syntax
   - Multi-line Templates
   - Indentation and Dedent
   - Interpolating Lists
   - String vs Template (escape sequences)
   - Multi-Brace Delimiters for Literal Braces
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
     - `--root` (recommended for safety)
     - `--force` (overwrite)
     - `--backup` (safe overwrite)
     - `--append` (additive)
     - `--if-not-exists` (initialization)
     - `--git` (fetch from GitHub)
     - `--debug` (detailed output)

8. **[Builtin Functions](#builtin-functions)**
   - String Operations (`concat`, `upper`, `lower`, `split`, `replace`, etc.)
   - List Operations (`map`, `filter`, `fold`, `join`, `length`, `zip`, `unzip`, `take`, `drop`, `split_at`, `partition`, `reverse`, `head`, `tail`, `sort`, `sort_by`, `unique`, `range`, `enumerate`)
   - Aggregate Functions (`sum`, `product`, `min`, `max`, `all`, `any`, `count`)
   - Regex Functions (`regex_match`, `regex_replace`, `regex_split`, `scan`)
   - File & Filesystem (`readfile`, `readlines`, `exists`, `basename`, `dirname`)
   - HTML Generation Helpers (`html_escape`, `html_tag`, `html_attr`)
   - Markdown Generation Helpers (`md_heading`, `md_link`, `md_code`, `md_list`)
   - Type Conversion (`to_string`, `to_int`, `to_float`, `to_bool`)
   - Advanced List Operations (`flatmap`, `flatten`)
   - Data & Utilities (`import`, `json_parse`, `os`)

9. **[Importing Files from Folders](#importing-files-from-folders)**
   - Core Pattern: Glob → Map/Filter → Fold
   - Common Patterns
     - Load JSON Folder as Dictionary
     - Import Avon Modules
     - Filter Files by Extension
     - Combine Multiple JSON Files
   - Important: Dict Literal Syntax in Fold
   - Path Operations (`basename`, `dirname`, `exists`)
   - Advanced Example: Config Override System
   - Comprehensive Importing Methods
     - Method 1: Single File Import
     - Method 2: Glob with Loop (fold)
     - Method 3: Filter Then Map
     - Method 4: Import Multiple Modules
     - Method 5: Import from GitHub (import_git)
     - Method 6: Merge Multiple Configs
     - Method 7: Multi-Folder Import
   - Real-World Importing Scenarios
     - Scenario 1: Multi-Environment Configuration
     - Scenario 2: Dynamic Function Library
     - Scenario 3: Data Pipeline
     - Scenario 4: Kubernetes Manifest Generator
   - When to Use Importing

10. **[CLI Usage](#cli-usage)**
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

11. **[Error handling and debugging](#error-handling-and-debugging)**
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

12. **[Security Best Practices](#security-best-practices)**
    - Input Validation & Sanitization
    - Template Safety Patterns
    - File Deployment Safety
    - Production Checklist
    - Path Security

13. **[Real-World Examples](#real-world-examples)**
    - Example 1: Site Generator
    - Example 2: Neovim Configuration
    - Example 3: Emacs Configuration
    - Example 4: Docker Compose Generator
    - Example 5: Kubernetes Manifests
    - Example 6: GitHub Actions Workflow
    - Example 7: Package.json Generator
    - Example 8: Multi-Brace Template Demo

14. **[Troubleshooting](#troubleshooting)**
    - Common Errors
      - "expected '\"' after opening braces"
      - "unexpected EOF"
      - "undefined identifier"
    - Template Brace Troubleshooting
      - Literal braces not showing
      - Lots of braces getting confusing
      - Interpolation not working
    - Debugging Tips

15. **[Gotchas and Common Pitfalls](#gotchas-and-common-pitfalls)**
    - Function Parameters Are CLI Arguments
    - Variables Don't Shadow – They Nest
    - Functions with All Defaults Still Return Functions
    - No Recursion – Use `fold` Instead
    - Template Braces Can Be Confusing
    - `json_parse` Only Reads from Files — Use `json_parse_string` for Strings
    - Lists in Templates Expand to Multiple Lines
    - `glob` Returns Paths, Not Contents
    - Import Evaluates the Entire File
    - Avon is Single-Pass and Simple
    - `pfold` Requires an Associative Combiner
    - Division Always Returns Float (Use `//` for Integer Division)
    - Range with Start > End Returns Empty List
    - Power Operator `**` is Right-Associative
    - `zip` Truncates to Shorter List
    - And more...

16. **[Tips and Tricks](#tips-and-tricks)**
    - Check List Membership with `any`
    - Safe Division with Default Value
    - Type Checking with `typeof` and `is_*`
    - Function Composition via Pipes
    - Working with Characters in Strings
    - And more...

17. **[Piping, Stdin, Stdout, and Embedding Avon](#piping-stdin-stdout-and-embedding-avon)**
    - Piping Avon Source Code into the CLI
    - Piping Data into an Avon Program
    - Capturing Avon Output
    - Exit Codes
    - Debug Output Goes to Stderr
    - Embedding Avon in Other Programs
    - Real-World Integration: File Collection Scripts
    - Summary: Stdin/Stdout Modes

18. **[Next Steps](#next-steps)**

---

## Quick Start

### Your First Program

Let's start with the simplest possible Avon program. Create a file called `hello.av` (or see `examples/` for working examples):

```avon
"Hello, world!"
```

Run it:

```bash
avon eval examples/hello.av
# Output: Hello, world!
```

Congratulations! You've just run your first Avon program. Not quite "Hello, World" in C, but at least you didn't have to include stdio.h. 

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

What happened?
- `\name` defines a function parameter (that backslash is the lambda—more on that later)
- `@greeting.txt` specifies the output file path (relative to `--root`)
- `{"..."}` is a template that interpolates the `{name}` variable
- `--root ./output` ensures files are written to `./output/greeting.txt`

### Generate Multiple Files

Here's where Avon gets fun. Let's generate a config file for each environment:

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

This creates three files: `config-dev.yml`, `config-staging.yml`, and `config-prod.yml`—each with appropriate settings.

Key insight: Return a list of file templates and Avon generates them all in one go! No loops, no scripts, no glue code. No "wait, which file did I forget to update?"

### Avon for Single Files and Dotfiles

Avon excels at generating hundreds of files, but it's equally powerful for single files. It's a comprehensive workflow layer that makes any file more maintainable and shareable, whether you're managing a single config or building complex multi-file systems.

Perfect for:
- Dotfiles — Easy way to download and deploy configs to your system
- Sharing configs — One file in git, many customized deployments
- Single files with variables — Make any file more generic and maintainable
- Long, repetitive files — Use list interpolation to eliminate copy-paste
- Non-developers — Simple way to manage and share personal configs

Example: Dotfile with Variables
```avon
\username ? "developer" @.vimrc {"
  " Vim configuration for {username}
  set number
  set expandtab
  set tabstop=4
  colorscheme {if username == "developer" then "solarized" else "default"}
"}
```

Deploy:
```bash
avon deploy vimrc.av --root ~ -username alice
```

Share: Keep one `.vimrc.av` in git. Each person deploys their customized version. No more maintaining separate dotfiles for each machine. No more "which version of my vimrc is this again?" No more archaeological expeditions through your dotfiles repo.

Example: Long Config with List Interpolation
```avon
let plugins = ["vim-fugitive", "vim-surround", "vim-commentary", "vim-repeat"] in
@.vimrc {"
  " Plugin configuration
  {plugins}
"}
```

When you interpolate a list in a template, each item appears on its own line. This eliminates copy-paste even in a single file.

Language Agnostic: Avon works with any text format—YAML, JSON, shell scripts, code, configs, documentation, or dotfiles. It brings variables, functions, and 137 built-in utilities to any file.

Runtime Type Safety: Avon won't deploy if there's a type error. No static types needed—if a type error occurs, deployment simply doesn't happen. Sleep soundly knowing your configs are valid. (Unlike that bash script you wrote at 2am.)

Built-in Utilities: Avon comes with a comprehensive set of built-in functions for string operations, list operations (map, filter, fold, sort, unique, range), formatting, date/time operations, JSON manipulation, file I/O, and HTML/Markdown helpers.

Debugging Tools: Use `trace`, `debug`, `assert`, and the `--debug` flag to troubleshoot quickly.

---

## Core Concepts

### Simple File Model

Each Avon file contains exactly one expression. This keeps Avon simple and predictable. When you run an Avon file, it evaluates that single expression to a value.

This simplicity enables modularity: the `import` function allows any file to return any Avon type (a string, number, list, dict, function, FileTemplate, or any other value). Files can be libraries that export functions, data files that return dictionaries, or generators that return FileTemplates—all using the same simple model.

Example: Library file (`math.av`):
```avon
{double: \x x * 2, triple: \x x * 3, square: \x x * x}
```

Example: Data file (`config.av`):
```avon
{host: "localhost", port: 8080, debug: true}
```

Example: Generator file (`deploy.av`):
```avon
@config.yml {"host: localhost"}
```

All three are valid Avon files. The `import` function evaluates the file and returns whatever value it produces, making Avon naturally modular.

### The Avon Runtime Model

When you run an Avon program, it evaluates to a Value. Here are the types of values you'll encounter:

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

How `eval` works:**
- Evaluates the expression in the file
- Converts the result to a string representation
- Prints it to stdout
- If the result is a FileTemplate or list of FileTemplates, it shows the paths and content that would be generated (but doesn't write them)
- Exit code: 0 on success, 1 on error

How `deploy` works:**
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
none                       # None (absence of value)
[1, 2, 3]                  # List
{host: "localhost", port: 8080}  # Dictionary (key: value syntax)
```

None: The `none` literal represents the absence of a value. It's returned by:
- `head` on an empty list: `head []` returns `none`
- `get` on a missing key: `get {a: 1} "b"` returns `none`
- JSON null values when parsing JSON

Use `is_none` to check for None values:
```avon
let x = head [] in
if is_none x then "list was empty" else x

let val = get config "optional_key" in
if is_none val then "default" else val
```

Negative Numbers: You can write negative numbers directly using the `-` prefix:
```avon
-5                         # Negative integer
-3.14                      # Negative float
[-5, -4, -3]               # List with negative numbers
[10, -1 .. 0]              # Range with negative step
```

Note: For variables, use the `neg` function: `let x = 5 in -x` (uses `neg` function internally).

Strings support escape sequences: `"\n"` is a newline, `"\t"` is a tab, `"\\"` is a backslash, `"\""` is a quote.

Dictionary syntax: Keys are identifiers (unquoted), values can be any type:
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

#### Comments

```avon
# This is a comment - everything after # is ignored
let x = 42 in              # Inline comments work too
x + 1
```

Comments start with `#` and continue to the end of the line. Use them to explain your code.

#### Function Literals

```avon
\x x + 1                   # Function of one parameter
\x \y x + y                # Curried function of two parameters
\a \b \c (a + b) * c       # Curried function of three parameters
```

Functions are automatically curried, so `\x \y x + y` is equivalent to `\x (\y x + y)`. If you've never seen currying before, don't panic—it just means multi-argument functions are really chains of single-argument functions. You can call them the same way either way. If you've used Haskell, welcome home. If not, you'll be fine.

#### Function Application (Calling)

```avon
f x                        # Apply f to x
f x y                      # Apply f to x, then apply result to y
map (\n n + 1) [1,2,3]    # Pass a function and a list to map
```

Application is left-associative, so `f a b` means `(f a) b`.

#### Operators

Avon supports these binary operators:

Arithmetic Operators:
```avon
a + b                      # Addition (numbers), concatenation (strings/lists/templates/paths)
a - b                      # Subtraction (numbers only)
a * b                      # Multiplication (numbers only)
a / b                      # Division - always returns float (like Python 3)
a // b                     # Integer division (floor division, toward -∞)
a % b                      # Modulo/Remainder (numbers only)
a ** b                     # Exponentiation (power, right-associative)
```

**Division Examples:**
```avon
10 / 3     # => 3.333... (always float)
10 // 3    # => 3 (integer division)
-7 // 2    # => -4 (floors toward negative infinity)
7 // -3    # => -3 (floors toward negative infinity)
2 ** 8     # => 256
2 ** 0.5   # => 1.414... (square root)
4 ** 0.5   # => 2 (exact integer result)
```

**Modulo Examples:**
```avon
10 % 3     # => 1
-7 % 3     # => -1 (sign follows dividend)
7 % -3     # => 1
```

**Arithmetic Edge Cases & Error Handling:**

Avon handles arithmetic edge cases gracefully without crashing:

| Operation | Result | Notes |
|-----------|--------|-------|
| `10 / 0` | Error: "division by zero" | Runtime error |
| `10 // 0` | Error: "integer division by zero" | Runtime error |
| `10 % 0` | Error: "modulo by zero" | Runtime error |
| `0 ** 0` | `1` | Mathematical convention |
| `0 ** -1` | `inf` | Infinity |
| `(-1) ** 0.5` | `NaN` | Not a real number |
| `2 ** 1000` | Large float | Very large numbers become floats |
| `MAX_I64 + 1` | Wraps to `MIN_I64` | Integer overflow wraps |
| `MIN_I64 - 1` | Wraps to `MAX_I64` | Integer underflow wraps |
| `MIN_I64 // -1` | `MIN_I64` | Overflow wraps (returns MIN) |
| `MIN_I64 % -1` | `0` | Mathematically correct |

> **Note:** Integer arithmetic uses wrapping semantics for overflow/underflow. This is intentional and consistent across all integer operations.

<!-- In Avon, "5" + 3 is a type error, not "53". You're welcome. -->

Comparison Operators:
```avon
a == b                     # Equality (works for all types)
a != b                     # Inequality (works for all types)
a > b                      # Greater than (numbers only)
a < b                      # Less than (numbers only)
a >= b                     # Greater or equal (numbers only)
a <= b                     # Less or equal (numbers only)
```

Logical Operators:
```avon
a && b                     # Logical AND (short-circuits: b not evaluated if a is false)
a || b                     # Logical OR (short-circuits: b not evaluated if a is true)
not a                      # Logical NOT (returns true if false, false if true)
```

These work on actual booleans, not "truthy" values. `0`, `""`, and `[]` aren't secretly `false` here.

**Operator Precedence Table:**

From highest to lowest precedence:

| Precedence | Operators/Operations | Associativity | Description |
|------------|---------------------|---------------|-------------|
| 1 (highest) | `()` `[]` `{}` | - | Grouping, lists, dicts |
| 2 | `-x` (unary) | - | Unary minus |
| 3 | `**` | Right | Exponentiation |
| 4 | `*` `/` `//` `%` | Left | Multiplication, division |
| 5 | `+` `-` | Left | Addition, subtraction |
| 6 | `==` `!=` `>` `<` `>=` `<=` | Left | Comparison |
| 7 | function application | Left | `f x`, `map (\x x) list` |
| 8 | `&&` | Left | Logical AND |
| 9 | `not` | - | Logical NOT |
| 10 | `\|\|` | Left | Logical OR |
| 11 (lowest) | `->` | Left | Pipe |

**Key precedence rules:**
- Unary minus binds tighter than `**`: `-2 ** 2` → `(-2) ** 2` = `4`
- `**` binds tighter than `*`: `2 * 3 ** 2` → `2 * (3 ** 2)` = `18`
- `**` is right-associative: `2 ** 3 ** 2` → `2 ** (3 ** 2)` = `512`
- `&&` binds tighter than `||`: `true || false && false` → `true || (false && false)` = `true`
- Arithmetic binds tighter than comparison: `1 + 2 < 5` → `(1 + 2) < 5` = `true`
- Pipe has lowest precedence: use parens for `([1,2,3] -> length) + 5`

Pipe Operator:
```avon
a -> b                     # Pipe: pass a as first argument to b
```

The pipe operator `->` (not `|`) chains expressions, passing the left-hand side as the first argument to the right-hand side. This eliminates nested parentheses and makes code more readable.

Note: Only `->` is a valid pipe operator. The single `|` character is not a pipe operator in Avon.
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

Why use pipes?** Pipes make code more readable by showing the flow of data from left to right, rather than nested function calls. Compare:

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

Examples:
```avon
if age > 18 then "adult" else "minor"

if x == 0 then 1 else x

if debug then "verbose" else "quiet"
```

**Nested conditionals:**
```avon
if x > 0 then "positive" else (if x < 0 then "negative" else "zero")
```

Important: The condition must evaluate to a boolean (`true` or `false`). Type errors occur if you use a non-boolean value.

#### Logical Operators

Avon provides `&&` (AND), `||` (OR), and `not` (NOT) for combining boolean expressions:

```avon
a && b                     # Returns true only if both a and b are true
a || b                     # Returns true if at least one of a or b is true
not a                      # Returns true if a is false, false if a is true
```

Examples:
```avon
if (age >= 18) && (has_license) then "can drive" else "cannot drive"

if (x > 0) || (y > 0) then "at least one positive" else "both non-positive"

if not (is_empty list) then head list else "no items"
```

Important: Both operands must be booleans. Type errors occur if you use non-boolean values.

**Precedence:** Logical operators have lower precedence than comparison operators, so parentheses are often needed:
```avon
# Correct
if (x > 0) && (y > 0) then "both positive" else "not both positive"

# This would be parsed incorrectly without parentheses
# if x > 0 && y > 0 then ...  # Wrong! Parsed as: if x > (0 && y) > 0
```

**Short-circuit evaluation:** Avon's `&&` and `||` operators properly short-circuit:
```avon
false && (1 / 0 > 0)  # => false (right side is NOT evaluated!)
true || (1 / 0 > 0)   # => true (right side is NOT evaluated!)
```

This means you can safely write guard conditions:
```avon
let x = none in
(x != none) && (x > 5)  # Safe! If x is none, the comparison isn't evaluated
```

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

How Scoping Works:**

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

Example:
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

How it works:**
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

Why no shadowing?**
- Prevents confusion and makes code more predictable (no more "which x is this?")
- Each variable name is unique within its scope
- Easier to reason about code—you always know which variable you're referring to
- Aligns with functional programming principles (immutability)
- Saves you from yourself at 2am
- Unlike certain languages, `var` won't suddenly hoist your variable to the shadow realm

**Exception:** The variable `_` (underscore) can be reused. This is a special case for ignoring values (the "I don't care" variable):
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

Important: The template captures the value at creation time, not evaluation time:
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

How closures work:**
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

Important: Always include the `in` keyword! `let` bindings require an `in` to specify the expression where the binding is visible. Without `in`, the parser will report an error.

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
- Your laptop won't sound like a jet engine taking off

**3. Encourages Better Patterns**
- Avon's built-in functions (`fold`, `map`, `filter`) are designed for iteration
- These functions are more idiomatic and readable than recursive solutions
- They handle edge cases (empty lists, etc.) automatically

**4. Safety**
- No risk of infinite recursion bugs
- No need for recursion depth limits
- Predictable execution behavior

How to achieve recursive-like behavior:**

Instead of recursion, use Avon's built-in iteration functions:

```avon
# Instead of recursive factorial, use fold:
# (Yes, every programming tutorial must include factorial. It's the law.)
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

You can provide default values for lambda parameters using `?`:

```avon
let greet = \name ? "Guest" {"Hello {name}!"} in greet
# Returns: "Hello Guest!"

let greet = \name ? "Guest" {"Hello {name}!"} in greet "Alice"
# Returns: "Hello Alice!"
```

**Syntax:** `\param ? default_value expression`

**Multiple parameters with defaults:**
```avon
let config = \host ? "localhost" \port ? 8080 {"{host}:{port}"} in

config                     # "localhost:8080" (both defaults)
config "example.com"       # "example.com:8080" (first overridden)
config "example.com" 443   # "example.com:443" (both overridden)
```

**Mixed required and optional parameters:**
```avon
let make_url = \scheme \host ? "localhost" \port ? 80 {"{scheme}://{host}:{port}"} in

make_url "https"                   # "https://localhost:80"
make_url "https" "api.example.com" # "https://api.example.com:80"
make_url "https" "api.example.com" 443  # "https://api.example.com:443"
```

**Default values are evaluated at function definition time:**
```avon
let x = 10 in
let f = \y ? x y + 1 in
f         # 11 (uses x=10 as default)
f 5       # 6 (uses provided value)
```

This works for any lambda, whether used directly or in a deploy file. When deploying, the default is used if no argument is provided:

```avon
\name ? "Guest" @welcome.txt {"
    Welcome, {name}!
"}
```

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

### Important: Functions with All Defaults

When a function has **all parameters with defaults**, Avon has special behavior:

**When you reference the function at the top level, it auto-evaluates:**

```avon
let add = \x ? 5 \y ? 10 x + y in
add
# Result: 15 (auto-evaluated with defaults)
```

**But the function is still a function value:**

```avon
let add = \x ? 5 \y ? 10 x + y in
typeof add
# Result: "Function"

is_function add
# Result: true
```

**Context matters - in data structures, it stays as a function:**

```avon
let add = \x ? 5 \y ? 10 x + y in
{my_func: add}
# Result: {my_func: <function>}  (NOT auto-evaluated)
```

**You can still override defaults or use the function value:**

```avon
let add = \x ? 5 \y ? 10 x + y in
add 3 4
# Result: 7 (with custom arguments)

let add = \x ? 5 \y ? 10 x + y in
let adder = {fn: add} in
adder.fn  # Still a function value
```

**Key Takeaway:** Functions remain functions, but top-level auto-evaluation is a convenience feature. If you need the function value itself, store it in a data structure or use a variable.

---

## Collections

### Lists

Lists are the workhorse of Avon. They're written with square brackets:

```avon
[1, 2, 3]
["alice", "bob", "charlie"]  # A list, not a database of users who owe you money
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

<!-- Easter egg: In some languages, [1, 2] + [3, 4] equals 10. Or "1,23,4". Or throws. We just concatenate lists like sane people. -->

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

Spaces around `..` are optional: both `[1..5]` and `[1 .. 5]` work.

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

Why use dicts instead of list of pairs?**

- **Clearer intent** - `{host: "localhost"}` is more readable than `[["host", "localhost"]]`
- **Dot notation** - Access fields naturally: `config.host` instead of `get config "host"`
- **Faster lookups** - Hash map instead of linear search through pairs
- **Type-safe** - Errors when accessing non-existent keys are clear
- **Better for JSON** - JSON objects naturally parse to dicts
- **Backward compatible** - `get`, `set`, `has_key`, `keys`, `values` work with both dicts and list-of-pairs

When to use each:**

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
  port: 8080,           # Not 80, because we're not savages running as root
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

(Note: Avon has a `join` builtin to make this easier! We just wanted to show you the hard way first so you'd appreciate it.)

### Builtins for Lists

| Function | Description |
|----------|-------------|
| `map f list` | Apply `f` to each item |
| `filter pred list` | Keep items where `pred` is truthy |
| `fold f init list` | Reduce with accumulator |
| `join list sep` | Join list items with separator |
| `length list` | Get number of items in list |
| `sort list` | Sort list (numbers numerically, others lexically) |
| `sort_by f list` | Sort by applying key function to each item |
| `unique list` | Remove duplicates (preserves order) |
| `range start end` | Generate inclusive integer range |
| `enumerate list` | Convert to `[[index, item], ...]` pairs |

**Sorting and deduplication examples:**
```avon
# Sort numbers
sort [3, 1, 4, 1, 5, 9, 2, 6]  # [1, 1, 2, 3, 4, 5, 6, 9]

# Sort strings
sort ["zebra", "apple", "banana"]  # ["apple", "banana", "zebra"]

# Reverse sort with sort_by
sort_by (\x neg x) [5, 2, 8, 1]  # [8, 5, 2, 1]

# Sort by string length
sort_by (\x length x) ["aaa", "a", "aa"]  # ["a", "aa", "aaa"]

# Remove duplicates
unique [1, 2, 2, 3, 1, 4, 3, 5]  # [1, 2, 3, 4, 5]

# Generate ranges (alternative to [1..10] syntax)
range 1 10  # [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Enumerate for index tracking
enumerate ["apple", "banana", "cherry"]
# [[0, "apple"], [1, "banana"], [2, "cherry"]]
```

### File I/O and Globbing

Avon provides powerful functions for working with files and folders. These are commonly used with list operations like `map`, `filter`, and `fold` to build data pipelines.

#### The File I/O Pipeline

When working with files, you typically:

1. **Discover** files with `glob` (returns file paths)
2. **Read** file contents with `readfile` or implicit reading in parse functions
3. **Parse** content with `json_parse`, `yaml_parse`, or `import`
4. **Transform** results with `map`, `filter`, `fold`

**Example: Step by step**
```avon
# Step 1: Discover files (returns paths as strings)
let files = glob "config/*.json" in

# Step 2: Read and parse each file
let configs = map (\f json_parse f) files in

# Step 3: Aggregate into a single dict (optional)
let merged = fold
  (\acc \f set acc (basename f) (json_parse f))
  {}
  files
in
merged
```

#### Core File Functions

| Function | Input | Returns | Notes |
|----------|-------|---------|-------|
| `glob pattern` | String pattern | List[String] | File paths matching pattern (e.g., `"*.json"` or `"config/**/*.json"`) |
| `readfile path` | String or Path | String | Raw file contents as a string |
| `readlines path` | String or Path | List[String] | File split into lines |
| `exists path` | String or Path | Bool | Check if file exists |
| `basename path` | String or Path | String | Filename from path (e.g., `"config/app.json"` → `"app.json"`) |
| `dirname path` | String or Path | String | Directory from path (e.g., `"config/app.json"` → `"config"`) |
| `json_parse file_path` | String | Dict/List | Parse JSON from file |
| `yaml_parse file_path` | String | Dict/List | Parse YAML from file |
| `toml_parse file_path` | String | Dict | Parse TOML from file |
| `csv_parse file_path` | String | List[Dict]/List[List] | Parse CSV from file |
| `xml_parse file_path` | String | Dict | Parse XML from file (tag, attrs, children, text) |
| `html_parse file_path` | String | Dict | Parse HTML from file (tag, attrs, children, text). Uses HTML5 parser. |
| `opml_parse file_path` | String | Dict | Parse OPML from file (version, head, outlines) |
| `ini_parse file_path` | String | Dict | Parse INI from file (section Dicts, global keys under "global") |
| `import path` | String | Any | Evaluate Avon file and return result |

#### Key Behavior: All Parsers Only Read from Files

`json_parse` (and `yaml_parse`, `toml_parse`, `csv_parse`, `xml_parse`, `html_parse`, `opml_parse`, `ini_parse`) only read from file paths, not from strings:

```avon
# Pass a file path - json_parse reads and parses it
json_parse "config.json"              # Success: reads file, parses JSON

# Pass JSON content directly - json_parse tries to read as file (errors)
json_parse "{\"key\": \"value\"}"     # Error: tries to read file named '{"key": "value"}'

# Glob returns paths, which json_parse handles correctly
let files = glob "*.json" in
map (\f json_parse f) files           # Works: json_parse reads each file
```

**The Golden Rule:** `glob` returns **paths**, not content. Use `json_parse`, `readfile`, or `import` to get the actual content.

```
Discover (glob)
    ↓ returns: ["file1.json", "file2.json", ...]
Read & Parse (json_parse, readfile, etc.)
    ↓ returns: {data: ...}, "content", etc.
Transform (map, filter, fold, etc.)
    ↓ returns: organized result
```

#### Practical Glob Examples

```avon
glob "*.json"                  # All JSON files in current dir
glob "config/*.json"           # All JSON in config subdir
glob "**/*.json"               # Recursive (any depth)
glob "src/**/*.av"             # All Avon files in src (recursive)
glob "data/users/*.yaml"       # YAML files in users dir
```

#### Practical File I/O Patterns

**Pattern: Load and filter files**
```avon
let all_files = glob "data/*" in
let json_only = filter (\f ends_with f ".json") all_files in
let configs = map (\f json_parse f) json_only in
configs
```

**Pattern: Load configs with defaults**
```avon
let defaults = {debug: false, timeout: 30} in
let user_config = json_parse "config.json" in
let merged = dict_merge defaults user_config in
merged
```

**Pattern: Load multiple configs into named dict**
```avon
fold
  (\acc \f
    set acc (basename f) (json_parse f))
  {}
  (glob "config/*.json")
# Result: {app.json: {...}, db.json: {...}, ...}
```

**Pattern: Load files excluding certain names**
```avon
let active = filter
  (\f not (starts_with (basename f) "_"))
  (glob "config/*.json")
in
map (\f json_parse f) active
```

#### Empty Globs and Missing Files

```avon
# Glob with no matches returns empty list (not an error)
glob "nonexistent/*.json"      # [] (empty list)

# Reading missing file is an error
readfile "missing.txt"         # Error: No such file or directory
json_parse "missing.json"      # Error: No such file or directory
```

To handle missing files gracefully:
```avon
let config = if exists "custom.json" then json_parse "custom.json" else {} in
config
```

#### Data Format Conversion

Avon supports 8 structured data formats with paired parsers and formatters:

| Format | File Parser | String Parser | Formatter |
|--------|-------------|---------------|-----------|
| JSON | `json_parse` | `json_parse_string` | `format_json` |
| YAML | `yaml_parse` | `yaml_parse_string` | `format_yaml` |
| TOML | `toml_parse` | `toml_parse_string` | `format_toml` |
| CSV | `csv_parse` | `csv_parse_string` | `format_csv` |
| XML | `xml_parse` | `xml_parse_string` | `format_xml` |
| HTML | `html_parse` | `html_parse_string` | `format_html` |
| OPML | `opml_parse` | `opml_parse_string` | `format_opml` |
| INI | `ini_parse` | `ini_parse_string` | `format_ini` |

**File parsers** (`*_parse`) read from file paths. **String parsers** (`*_parse_string`) parse raw strings directly — useful for inline data, API responses, or dynamically built content:

```avon
# File parsing — reads from disk
let config = json_parse "config.json" in config.port

# String parsing — parses a raw string
let data = json_parse_string "{\"port\": 8080}" in data.port
```

**The universal value type.** The reason cross-format conversion works is that every parser produces the same Avon value types — Dict, List, String, Number, Bool. There is no "JSON object" or "YAML mapping." Once parsed, data is just a Dict or List, and all Avon builtins work on it:

```avon
# Parse any format — the result is always Dict or List
let from_json = json_parse "config.json" in     # Dict
let from_yaml = yaml_parse "config.yml" in       # Dict
let from_csv  = csv_parse "users.csv" in         # List of Dicts

# All standard operations work on parsed data:
typeof from_json                    # "Dict"
keys from_json                     # list of keys
get from_json "host"               # value for "host"
has_key from_json "port"           # true or false
from_json.host                     # dot notation access

# List operations work on parsed lists:
map (\u u.name) from_csv           # extract a field from each row
filter (\u u.age > 30) from_csv    # filter rows
fold (\acc \u acc + 1) 0 from_csv  # count rows
```

**Cross-format conversion** is just piping one parser's output into another formatter:

```avon
# Convert between any formats
json_parse "data.json" -> format_yaml      # JSON → YAML
yaml_parse "config.yml" -> format_toml     # YAML → TOML
csv_parse "data.csv" -> format_json        # CSV → JSON
xml_parse "feed.xml" -> format_json        # XML → JSON
html_parse "page.html" -> format_json      # HTML → JSON
ini_parse "config.ini" -> format_json      # INI → JSON
```

**Transform, then convert** — parse, use map/filter/fold to reshape, then output in any format:

```avon
# Read CSV, keep only active users, output as JSON
let users = csv_parse "users.csv" in
let active = filter (\u u.status == "active") users in
format_json active
```

For complete reference and all supported conversions, see [BUILTIN_FUNCTIONS.md — Data Format Conversion](./BUILTIN_FUNCTIONS.md#data-format-conversion).

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

### Nested Template Indentation

When templates are interpolated into other templates, the indentation is handled intelligently. Templates maintain a **shared baseline** at the outermost level:

```avon
let config_line = {"    host: localhost"} in
let full_config = {"
[database]
{config_line}
    port: 5432
"} in
full_config
```

Output:
```
[database]
    host: localhost
    port: 5432
```

The inner template's indentation (4 spaces) aligns properly with the outer template's baseline. This means:
- Nested templates automatically maintain consistent indentation
- You don't need to worry about indentation misalignment when interpolating
- Multi-level template composition works intuitively

**Complex nested example:**
```avon
let render_values = {"
        value1
        value2
"} in
let render_section = \name {"
    section: {name}
    values: {render_values}
"} in
render_section "config"
```

Output:
```

    section: config
    values: 
        value1
        value2
```

Each level maintains proper relative indentation, creating clean, readable output.

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

When to use each:**
- **Use strings for:** Data values, single-line content, escape sequences needed
- **Use templates for:** File content, multi-line output, variable interpolation needed

**Template variable capture:**
Templates capture variables from their surrounding scope at the time the template is created. This means:

```avon
let name = "Alice" in
let template = {"Hello, {name}"} in
let greeting = template in
greeting
# Evaluates to: "Hello, Alice"
# The template captured "Alice" when it was created
```

This is important for closures and function returns—templates remember the values from when they were defined, not when they're evaluated. The template's interpolation happens at template creation time, not when the template is used.

### Template Auto-Conversion in String Functions

**Important UX feature:** Any function that accepts a string argument will automatically accept a template as well. The template is converted to a string before the function processes it. This makes templates much easier to use throughout your code.

**Functions that auto-convert templates:**
- All string manipulation functions: `upper`, `lower`, `trim`, `split`, `join`, `replace`, `contains`, `starts_with`, `ends_with`, `length`, `repeat`, `pad_left`, `pad_right`, `indent`
- All string predicate functions: `is_digit`, `is_alpha`, `is_alphanumeric`, `is_whitespace`, `is_uppercase`, `is_lowercase`, `is_empty`
- HTML functions: `html_escape`, `html_tag`, `html_attr`
- Markdown functions: `md_heading`, `md_link`, `md_code`, `markdown_to_html`
- String operations: `concat`

Example:
```avon
let name = "world" in
let template = {"hello {name}"} in
upper template
# Result: "HELLO WORLD"
# No need to call to_string first!
# (Fun fact: this example has been written 10 million times in every language tutorial ever)
```


**After (automatic conversion):**
```avon
let t = {"hello {name}"} in
upper t
```

This makes templates much more powerful and convenient to use. You can pass templates directly to any string function without explicit conversion.

### Multi-Brace Delimiters for Literal Braces

Avon templates use a **variable-brace delimiter system** that lets you choose how many opening braces to use. This powerful feature lets you generate code and config files cleanly, even when they contain many curly braces.

#### Why Multiple Brace Levels?

When generating code that uses braces (Lua, JSON, Terraform, HCL, Python, etc.), you need to distinguish:
- **Literal braces in the output** (e.g., `{` for a Lua table or JSON object)
- **Interpolation braces** (e.g., `{variable}` to substitute values)

Avon solves this by letting you choose how many braces delimit the template:

```avon
{" ... "}        # Single-brace: interpolate with { }
{{"  ... "}}     # Double-brace: interpolate with {{ }}, single braces are literal
{{{" ... "}}}    # Triple-brace: interpolate with {{{ }}}, single and double braces are literal
```

This way, you choose the delimiter that matches your content's brace density, avoiding escaping entirely.

#### How the System Works

**Interpolation** uses exactly the same number of braces as the template opener:

```avon
{"Value: { 1 + 2 }"}              # Single-brace interpolation: { }
{{"Value: {{ 1 + 2 }}"}}          # Double-brace interpolation: {{ }}
{{{"Value: {{{ 1 + 2 }}}"}}}      # Triple-brace interpolation: {{{ }}}
```

**Literal braces** are any braces with fewer than the delimiter count:

| Delimiter | Interpolate with | Literal braces |
|-----------|------------------|----------------|
| `{" "}` | `{x}` | (none) |
| `{{" "}}` | `{{x}}` | `{` `}` |
| `{{{" "}}}` | `{{{x}}}` | `{` `{{` `}` `}}` |

#### Single-Brace Templates

Use when your output has **no literal braces**:

```avon
{"Value: { 1 + 2 }"}       # Output: Value: 3
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

Use when your output has **single braces** (JSON, CSS, Lua, Nginx, etc.):

```avon
@config.lua {{"
    local config = {
      name = "{{ app_name }}",
      debug = {{ if dev then "true" else "false" }}
    }
"}}
```

**Rule:** In double-brace templates, single braces are literal:

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

#### Triple-Brace Templates

Use when your output has **double braces** (GitHub Actions, Mustache templates, etc.):

```avon
@workflow.yml {{{"
    name: CI
    env:
      VAR: ${{ github.repository }}
    jobs:
      build:
        steps:
          - run: echo "Value is {{{value}}}"
"}}}
```

#### Example: Generating Lua Code

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

#### Strategic Choice: Match Your Content

Choose your template delimiter based on what braces appear in your output:

| Output Type | Delimiter | Why |
|-------------|-----------|-----|
| Plain text, YAML | `{" "}` | No braces needed |
| JSON, CSS, Lua, Nginx | `{{" "}}` | Single braces are literal |
| GitHub Actions, Mustache | `{{{" "}}}` | Double braces are literal |

The key insight: **level up your delimiter to make lower-level braces literal**.

#### Using Functions to Wrap Variables in Braces

Sometimes you need to generate literal braces **around interpolated values**. For example, when generating Terraform variables or template placeholders:

**Problem:** You want output like `{variable_name}` where `variable_name` comes from a variable.

**Solution:** Create a simple wrapping function using string concatenation:

```avon
# Define a wrap function
let wrap = \x "{" + x + "}" in

# Use it to generate braced placeholders
let vars = ["name", "email", "age"] in
{"
Variables:
{map wrap vars}
"}
```

Output:
```
Variables:
{name}
{email}
{age}
```

**Practical Example: Generating Terraform Variables**

```avon
let wrap = \x "{" + x + "}" in
let vars = ["project_id", "region", "instance_type"] in

@variables.tf {"
variable "config" {
  type = map(string)
  default = {
{join (map (\v "    " + v + " = " + (wrap v)) vars) ",\n"}
  }
}
"}
```

Output:
```terraform
variable "config" {
  type = map(string)
  default = {
    project_id = {project_id},
    region = {region},
    instance_type = {instance_type}
  }
}
```

**JSON Generation with Placeholders:**

```avon
let wrap = \x "{" + x + "}" in
let fields = ["name", "email", "age"] in

@template.json {"
{
{join (map (\v "  \"" + v + "\": " + (wrap v)) fields) ",\n"}
}
"}
```

Output:
```json
{
  "name": {name},
  "email": {email},
  "age": {age}
}
```

**Key Points:**
- Use single `+` operator for string concatenation
- Wrap function: `\x "{" + x + "}"`
- Double wrap for double braces: `\x "{{" + x + "}}"`
- Triple wrap for triple braces: `\x "{{{" + x + "}}}"`
- This works because the function returns a string, which is then interpolated into the template

#### Variable Brace Count (Advanced)

The template delimiter can use **any number of braces**, not just 1-3:

```avon
# 4 braces
let x = "value" in {{{{"Text: {{{{x}}}} and {{{literal}}} and {{double}}}}}}

# 5 braces
let y = "data" in {{{{{"Result: {{{{{y}}}}} and {{{{quad}}}} and {{{three}}}"}}}}}
```

The rule remains consistent: **interpolation uses the same number of braces as the template delimiter**, and all lower brace counts are treated as literals.

See `tutorial/TEMPLATE_SYNTAX.md` for comprehensive documentation of the template system.

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
- **Recommended for safety:** Prevents accidental writes to system directories
- All file paths are resolved relative to this directory (or current directory if not specified)
- Example: `--root ./output` means `@config.yml` becomes `./output/config.yml`
- Example: Without `--root`, running `avon deploy config.av` from `/home/user/project/` writes `@config.yml` to `/home/user/project/config.yml`
- **Use this flag** to keep your deployments contained and predictable

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
- `--root` confines all writes to a specific directory (recommended for safety).
- If any error occurs during deployment preparation or validation, **zero files are written** (truly atomic deployment). All files are validated before any writes occur.

---

## Builtin Functions

Avon comes with a toolkit of **137 built-in functions** for common tasks. All builtins are curried, so you can partially apply them.

> **Quick Reference:** Use `avon doc` to look up any function instantly in your terminal. See [CLI Usage](#cli-usage) section for detailed documentation command examples.
> 
> **Full Reference:** For a complete list of all built-in functions, see [BUILTIN_FUNCTIONS.md](./BUILTIN_FUNCTIONS.md).

These utilities work with any file format—whether you're generating hundreds of config files or just managing a single dotfile. Functions like `upper`, `lower`, `format_table`, `json_parse`, and `html_escape` help you manipulate text however you need.

**Examples of using the doc command:**

```bash
# Look up any function
avon doc map
avon doc filter
avon doc join

# Browse by category
avon doc string    # All string manipulation functions
avon doc list      # All list operations
avon doc dict      # All dictionary functions
```

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

### Aggregate Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sum list` | Sum of numbers | `sum [1, 2, 3]` → `6` |
| `product list` | Product of numbers | `product [2, 3, 4]` → `24` |
| `min list` | Minimum value | `min [3, 1, 4]` → `1` |
| `max list` | Maximum value | `max [3, 1, 4]` → `4` |
| `all pred list` | True if all items match | `all (\x x > 0) [1, 2]` → `true` |
| `any pred list` | True if any item matches | `any (\x x > 5) [1, 6]` → `true` |
| `count pred list` | Count matching items | `count (\x x % 2 == 0) [1, 2, 3, 4]` → `2` |

### Math Functions

| Function | Description | Example |
|----------|-------------|---------|
| `abs n` | Absolute value | `abs -5` → `5` |
| `sqrt n` | Square root | `sqrt 16` → `4.0` |
| `floor n` | Round down (toward -∞) | `floor 3.7` → `3`, `floor (-2.5)` → `-3` |
| `ceil n` | Round up (toward +∞) | `ceil 3.2` → `4`, `ceil (-2.5)` → `-2` |
| `round n` | Round to nearest | `round 3.5` → `4` |
| `pow base exp` | Power (also `**`) | `pow 2 3` → `8` |
| `log n` | Natural logarithm | `log 2.718` → `1.0` |
| `log10 n` | Base-10 logarithm | `log10 100` → `2.0` |
| `gcd a b` | Greatest common divisor | `gcd 12 8` → `4` |
| `lcm a b` | Least common multiple | `lcm 4 6` → `12` |
| `random_int min max` | Random integer (inclusive) | `random_int 1 10` → `7` (varies) |
| `random_float min max` | Random float in range | `random_float 0.0 1.0` → `0.42` (varies) |
| `uuid` | Generate UUID v4 | `uuid` → `"550e8400-..."` |

**Math Function Edge Cases:**

| Expression | Result | Notes |
|------------|--------|-------|
| `sqrt (-1)` | `NaN` | Square root of negative |
| `log 0` | `-inf` | Logarithm of zero |
| `log (-1)` | `NaN` | Logarithm of negative |
| `gcd 0 0` | `0` | GCD with both zeros |
| `gcd 0 n` | `n` | GCD with one zero |

### Date/Time Functions

| Function | Description | Example |
|----------|-------------|---------|
| `now` | Current ISO 8601 datetime | `now` → `"2024-12-10T15:30:00Z"` |
| `timestamp` | Current Unix timestamp | `timestamp` → `1733850600` |
| `timezone` | Current timezone offset | `timezone` → `"+00:00"` |
| `date_format dt fmt` | Format datetime | `date_format "2024-12-10T15:30:00Z" "%Y-%m-%d"` → `"2024-12-10"` |
| `date_parse str fmt` | Parse datetime string | `date_parse "2024-12-10" "%Y-%m-%d"` → `"2024-12-10T00:00:00Z"` |
| `date_add dt offset` | Add to datetime | `date_add "2024-12-10T00:00:00Z" "1d"` → `"2024-12-11T00:00:00Z"` |
| `date_diff dt1 dt2` | Days between dates | `date_diff "2024-12-01" "2024-12-10"` → `9` |

**Date format codes:** `%Y` year, `%m` month, `%d` day, `%H` hour, `%M` minute, `%S` second

**Offset units:** `Nd` = N days, `Nh` = N hours, `Nm` = N minutes, `Ns` = N seconds

### Regex Functions

| Function | Description | Example |
|----------|-------------|---------|
| `regex_match pattern text` | Check if text matches pattern | `regex_match "^\\d+$" "123"` → `true` |
| `regex_replace pattern repl text` | Replace matches | `regex_replace "\\d" "#" "a1b2"` → `"a#b#"` |
| `regex_split pattern text` | Split by pattern | `regex_split "\\s+" "a b  c"` → `["a", "b", "c"]` |
| `scan pattern text` | Find all matches | `scan "\\d+" "a12b34"` → `["12", "34"]` |

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
| `slice list start end` | Extract portion (0-based, end exclusive) | `slice [1,2,3,4,5] 1 4` → `[2,3,4]` |
| `split_at n list` | Split list at index | `split_at 2 [1,2,3,4,5]` → `[[1,2], [3,4,5]]` |
| `chunks n list` | Split into chunks of size n | `chunks 2 [1,2,3,4,5]` → `[[1,2],[3,4],[5]]` |
| `windows n list` | Sliding windows of size n | `windows 3 [1,2,3,4,5]` → `[[1,2,3],[2,3,4],[3,4,5]]` |
| `partition pred list` | Split by predicate | `partition (\x x > 2) [1,2,3,4,5]` → `[[3,4,5], [1,2]]` |
| `reverse list` | Reverse the list | `reverse [1,2,3]` → `[3,2,1]` |
| `head list` | Get first element (or `None` if empty) | `head [1,2,3]` → `1` |
| `nth n list` | Get element at index n (0-based, or `None` if out of bounds) | `nth 1 [1,2,3]` → `2` |
| `tail list` | Get all but first element | `tail [1,2,3,4]` → `[2,3,4]` |
| `last list` | Get last element (or `None` if empty) | `last [1,2,3]` → `3` |
| `take n list` | Get first n elements | `take 2 [1,2,3,4]` → `[1,2]` |
| `drop n list` | Remove first n elements | `drop 2 [1,2,3,4]` → `[3,4]` |
| `split_at n list` | Split list at index n | `split_at 2 [1,2,3,4,5]` → `[[1,2], [3,4,5]]` |
| `find pred list` | Find first item matching predicate (or `None`) | `find (\x x > 2) [1,2,3,4]` → `3` |
| `find_index pred list` | Find index of first match (or `None`) | `find_index (\x x > 2) [1,2,3,4]` → `2` |
| `intersperse sep list` | Insert separator between elements | `intersperse 0 [1,2,3]` → `[1,0,2,0,3]` |
| `transpose matrix` | Transpose rows/columns of nested lists | `transpose [[1,2],[3,4]]` → `[[1,3],[2,4]]` |
| `zip_with f l1 l2` | Combine two lists with a function | `zip_with (\a \b a+b) [1,2] [10,20]` → `[11,22]` |
| `shuffle list` | Randomly reorder elements | `shuffle [1,2,3]` → `[2,3,1]` (varies) |
| `sample n list` | Get n unique random elements | `sample 2 [1,2,3,4,5]` → `[3,1]` (varies) |
| `choice list` | Get one random element | `choice ["a","b","c"]` → `"b"` (varies) |
| `combinations k list` | All k-element combinations | `combinations 2 [1,2,3]` → `[[1,2],[1,3],[2,3]]` |
| `permutations k list` | All k-element permutations | `permutations 2 [1,2,3]` → `[[1,2],[1,3],[2,1],...]` |

Examples:
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
| `markdown_to_html md` | Convert markdown text to HTML | `markdown_to_html "# Hello\nWorld"` → `"<h1>Hello</h1>\n<p>World</p>"` |

**Markdown to HTML conversion:**
The `markdown_to_html` function converts markdown text to HTML. It supports:
- Headings: `#` through `######` → `<h1>` through `<h6>`
- Bold: `**text**` → `<strong>text</strong>`
- Italic: `*text*` → `<em>text</em>`
- Inline code: `` `code` `` → `<code>code</code>`
- Paragraphs: Regular text → `<p>text</p>`
- Empty lines → `<br>`

Example:
```avon
let md = {"# Welcome
This is **bold** and *italic* text.
"} in
markdown_to_html md
# Result: "<h1>Welcome</h1>\n<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"
```

Note: `markdown_to_html` accepts both strings and templates, automatically converting templates to strings before processing.

### Type Conversion & Casting

| Function | Description | Example | Result |
|----------|-------------|---------|--------|
| `to_string val` | Convert any value to string | `to_string 42` | `"42"` |
| `to_int val` | Convert to integer | `to_int "42"` | `42` |
| `to_int val` | Float to int (truncates) | `to_int 3.7` | `3` |
| `to_float val` | Convert to float | `to_float "3.14"` | `3.14` |
| `to_bool val` | Convert to boolean | `to_bool "yes"` | `true` |
| `to_bool val` | Number to bool (0=false) | `to_bool 0` | `false` |
| `to_char code` | Unicode codepoint to char | `to_char 72` | `"H"` |
| `to_list str` | String to list of chars | `to_list "Hi"` | `["H", "i"]` |
| `format_int num width` | Format integer with zero-padding | `format_int 7 3` | `"007"` |
| `format_float num prec` | Format float with precision | `format_float 3.14159 2` | `"3.14"` |

**String to bool conversions:** `"true"`, `"yes"`, `"1"`, `"on"` -> `true`; `"false"`, `"no"`, `"0"`, `"off"`, `""` -> `false`

### Type Checking Functions

| Function | Description | Example |
|----------|-------------|---------|
| `typeof val` | Get type name as string | `typeof 42` → `"Number"` |
| `is_string val` | Check if string | `is_string "hi"` → `true` |
| `is_number val` | Check if number (int or float) | `is_number 3.14` → `true` |
| `is_int val` | Check if integer | `is_int 42` → `true` |
| `is_float val` | Check if float | `is_float 3.14` → `true` |
| `is_bool val` | Check if boolean | `is_bool true` → `true` |
| `is_list val` | Check if list | `is_list [1,2,3]` → `true` |
| `is_function val` | Check if function | `is_function length` → `true` |
| `is_none val` | Check if None | `is_none (head [])` → `true` |
| `is_empty val` | Check if empty (string/list/dict) | `is_empty []` → `true` |

### Formatting Functions

| Function | Description | Example |
|----------|-------------|---------|
| `format_json val` | Convert to JSON string | `format_json {a: 1}` → `"{\"a\": 1}"` |
| `format_table data sep` | Format as 2D table | `format_table [["a","b"],["1","2"]] " \| "` |
| `format_hex n` | Number to hexadecimal | `format_hex 255` → `"ff"` |
| `format_binary n` | Number to binary | `format_binary 5` → `"101"` |
| `format_octal n` | Number to octal | `format_octal 8` → `"10"` |
| `format_bytes n` | Human-readable bytes | `format_bytes 1048576` → `"1 MB"` |
| `format_currency n sym` | Format as currency | `format_currency 1234.5 "$"` → `"$1,234.50"` |
| `format_percent n prec` | Format as percentage | `format_percent 0.756 1` → `"75.6%"` |
| `truncate s maxlen` | Truncate with "..." | `truncate "hello world" 8` → `"hello..."` |
| `center s width` | Center-align text | `center "hi" 6` → `"  hi  "` |

<!-- Fun fact: The number of ways to convert a string to a boolean is inversely proportional to the number of production incidents it will cause. -->

### Advanced List Operations

| Function | Description | Example |
|----------|-------------|---------|
| `flatmap f list` | Map then flatten | `flatmap (\x [x, x]) [1,2]` → `[1,1,2,2]` |
| `flatten list` | Flatten one level | `flatten [[1,2],[3,4]]` → `[1,2,3,4]` |

### Data & Utilities

| Function | Description | Example |
|----------|-------------|---------|
| `import path` | Load and evaluate another `.av` file | `import "lib.av"` |
| `json_parse file_path` | Parse JSON from a file | `json_parse "config.json"` |
| `os` | Get OS string | `os` → `"linux"`, `"macos"`, `"windows"` |

**Important note on `json_parse`:** This function only accepts file paths, not JSON strings. It reads the file at the given path and parses its contents as JSON. If you have JSON content in a string variable, you'll need to write it to a file first before using `json_parse`.

**The `import` Function and Modularity:**

Avon's simplicity enables powerful modularity. Since each file contains exactly one expression, the `import` function evaluates that expression and returns whatever value it produces. This means **any file can return any Avon type**:

- **Library files** return dictionaries of functions:
  ```avon
  # math.av - The answer to life, the universe, and everything
  {double: \x x * 2, triple: \x x * 3, square: \x x * x}
  ```
  ```avon
  # main.av
  let math = import "math.av" in
  math.double 21  # Returns 42 (of course)
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

## Importing Files from Folders

Avon's file I/O and list processing capabilities make it powerful for working with entire folders of files. You can load JSON configs, import Avon modules, filter files, and aggregate data—all in a clean, declarative way.

### Core Pattern: Glob → Map/Filter → Fold

The fundamental pattern for working with folders is:

1. **Discover files**: Use `glob` to find files matching a pattern
2. **Transform or filter**: Use `map` or `filter` to select and transform files
3. **Accumulate results**: Use `fold` or `flatten` to combine into a single result

**Simple example: Load all JSON configs into a single dictionary**

```avon
fold
  (\acc \f set acc (basename f) (json_parse f))
  {}
  (glob "config/*.json")
```

This pattern:
1. `glob "config/*.json"` - Find all JSON files in config folder
2. For each file, `json_parse f` reads and parses the JSON (no explicit `readfile` needed)
3. Use `set acc (basename f) ...` to store in dictionary under the filename
4. Start with empty dict `{}` as accumulator
5. Result: One dictionary with all configs merged together

### Common Patterns

#### Pattern 1: Load JSON Folder as Dictionary

Load all JSON files in a folder and merge them into a single dictionary:

```avon
let config_files = glob "config/*.json" in
fold
  (\acc \f
    let data = json_parse f in
    dict_merge acc data)
  {}
  config_files
```

Note: `json_parse` reads the file directly from the path, so you don't need to call `readfile` first. It only accepts file paths, not JSON strings.

This is useful for:
- Loading environment-specific configs that override defaults
- Merging multiple configuration sources into a single dict
- Building configuration registries

**Real-world use case:**
```avon
let base_config = {host: "localhost", port: 8080, debug: true} in
let env_overrides = json_parse "config/prod.json" in
let final_config = dict_merge base_config env_overrides in
final_config
```

#### Pattern 2: Import Avon Modules

Load and combine functions from multiple Avon files:

```avon
let modules = fold
  (\acc \f set acc (basename f) (import f))
  {}
  (glob "lib/*.av")
in
modules.math.double 21
```

This creates a dictionary of module functions. Each file in `lib/` becomes a key in the dictionary.

**Example library files:**
```avon
# lib/math.av
{double: \x x * 2, triple: \x x * 3}

# lib/strings.av  
{upper: upper, lower: lower}
```

Then import them:
```avon
let libs = fold
  (\acc \f set acc (basename f) (import f))
  {}
  (glob "lib/*.av")
in
[libs.math.double 5, libs.strings.upper "hello"]
# Result: [10, "HELLO"]
```

#### Pattern 3: Filter Files by Extension

Select only files with a specific extension:

```avon
let json_files = filter (\f ends_with f ".json") (glob "data/*") in
map (\f json_parse f) json_files
```

This is useful for:
- Processing only JSON files, ignoring other files
- Handling mixed directory contents
- Selective data loading

**Practical example: Load only active configurations**
```avon
let active_configs = filter
  (\f (not (starts_with (basename f) "_")))
  (glob "config/*.json")
in
fold
  (\acc \f set acc (basename f) (json_parse f))
  {}
  active_configs
```

#### Pattern 4: Combine Multiple JSON Files

Load data from multiple JSON files and combine into a single list:

```avon
flatten
  (map
    (\f json_parse f)
    (glob "data/*.json"))
```

If each JSON file contains an array, this creates one combined array. Useful for:
- Aggregating data from multiple sources
- Building datasets by combining files
- Processing batch data

**Real-world use case:**
```avon
let all_users = flatten
  (map
    (\f json_parse f)
    (glob "users/*.json"))
in
length all_users
# Count total users across all files
```

### Important: Dict Literal Syntax in Fold

There's one syntax gotcha when using dicts in fold operations. When accumulating dict values, you **must use `set`** to update the accumulator—you can't use dict literals directly.

**❌ WRONG - Dict literal doesn't auto-accumulate:**
```avon
fold
  (\acc \f {data: f})
  {}
  (glob "*.txt")
# Result: Only the last file in {data: ...}
# The accumulator is NOT updated!
```

**✅ RIGHT - Use `set` to update accumulator:**
```avon
fold
  (\acc \f set acc (basename f) (readfile f))
  {}
  (glob "*.txt")
# Result: Accumulator updated with each file
# {file1.txt: "content1", file2.txt: "content2", ...}
```

The issue: In the wrong version, the expression `{data: f}` creates a new dict each iteration, but it's never merged back into the accumulator. The `set` function properly updates the accumulator.

### Path Operations

Avon provides helper functions for path manipulation:

- `basename path` - Extract filename from path
- `dirname path` - Extract directory from path
- `exists path` - Check if file exists (returns true/false)

**Example: Filter files that exist**
```avon
let required_files = ["config.json", "schema.json", "data.json"] in
let existing = filter (\f exists @{f}) required_files in
existing
```

**Example: Group files by directory**
```avon
let files = glob "src/**/*.av" in
let grouped = fold
  (\acc \f
    let dir = dirname f in
    set acc dir (head (get acc dir) || []) + [basename f])
  {}
  files
in
grouped
```

### Advanced Example: Config Override System

Build a layered configuration system where files in later directories override earlier ones:

```avon
let load_dir = \dir
  fold
    (\acc \f set acc (basename f) (json_parse f))
    {}
    (glob dir "/*.json") in

let base = load_dir "config/base" in
let env_override = load_dir "config/prod" in
let instance_override = load_dir "config/prod/instance-1" in

let final_config = dict_merge base env_override in
let final_config = dict_merge final_config instance_override in

final_config
```

This creates a configuration precedence chain where later files override earlier ones—perfect for environment-specific deployments.

### Comprehensive Importing Methods

#### Method 1: Single File Import

The simplest approach: import one file directly.

```avon
let config = json_parse "app.json" in
let version = config.version in
@output.txt {"Version: {version}"}
```

**Use cases:** Loading a single config, reading a template, importing a library.

#### Method 2: Glob with Loop (fold)

Import all files matching a pattern and aggregate them.

```avon
let config_files = glob "config/*.json" in
let configs = fold
  (\acc \f
    set acc (basename f) (json_parse f))
  {}
  config_files
in
configs
```

This creates a dictionary where keys are filenames and values are parsed contents:
```
{
  app.json: {name: "MyApp", version: "1.0.0"},
  database.json: {host: "localhost", port: 5432},
  cache.json: {ttl: 3600, redis_host: "cache.local"}
}
```

#### Method 3: Filter Then Map

Filter files first, then transform them.

```avon
let json_files = filter (\f ends_with f ".json") (glob "config/*") in
let parsed = map (\f json_parse f) json_files in
parsed
```

**Use cases:** Load only JSON files, skip certain directories, conditional loading.

#### Method 4: Import Multiple Modules

Load Avon modules from a folder.

```avon
let libraries = fold
  (\acc \f
    set acc (basename f) (import f))
  {}
  (glob "lib/*.av")
in
libraries
```

Now `libraries` is a dict where each key is a module filename and each value is what that module returns:
```
{
  math.av: {add: <function>, multiply: <function>},
  strings.av: {upper: <function>, lower: <function>},
  lists.av: {concat: <function>, reverse: <function>}
}
```

#### Method 5: Import from GitHub (import_git)

Download and evaluate Avon files directly from GitHub repositories. This is useful for sharing reusable modules, configurations, and templates.

⚠️ **Safety First**: `import_git` requires a **commit hash** (not a branch name like "main") to prevent accidental changes to your code if the remote file is updated.

```avon
let helix_config = import_git "pyrotek45/config/helix.av" "f75d99c0b6803495a86bb0e4ec0ef014a5c57263" in
@helix_config.toml {helix_config}
```

**Format:**
- First argument: `"owner/repo/path/to/file.av"` (relative path within repo)
- Second argument: full commit hash from GitHub (use `git log --format="%H" path/to/file.av` to get it)

**Use cases:**
- Sharing Avon libraries across projects
- Importing configuration templates from public repositories
- Building deployable Avon modules from GitHub
- Composing multi-file applications from versioned modules

**Error handling** — Clear error messages if something goes wrong:
- **File not found (404)**: Shows owner, repo, file path, and commit hash for debugging
- **Wrong argument type**: Must be two strings, not numbers or other types
- **Parse error**: If downloaded file has syntax errors, shows the error with context
- **Network error**: Connection problems are reported clearly

**Note:** Avon does not have try/catch syntax. If `import_git` encounters an error (404, network issue, parse error), it will fail immediately with a descriptive error message. For safe imports, ensure:
1. The commit hash is correct and exists in the repository
2. The file path is valid relative to the repository root
3. The file has valid Avon syntax
4. You have network connectivity to GitHub

See `examples/import_git_error_handling.av` for examples of error scenarios.

#### Method 6: Merge Multiple Configs

Combine multiple JSON files into a single configuration.

```avon
let config_files = glob "config/*.json" in
fold
  (\acc \f
    let data = json_parse f in
    dict_merge acc data)
  {}
  config_files
```

This flattens all JSON objects into one.

**Input** (3 separate files):
- `app.json`: `{version: "1.0.0", name: "MyApp", debug: false, port: 8080}`
- `database.json`: `{host: "localhost", port: 5432, user: "admin", password: "secret"}`
- `cache.json`: `{redis_host: "cache.local", redis_port: 6379, ttl: 3600}`

**Output**:
```
{
  version: "1.0.0",
  name: "MyApp",
  debug: false,
  port: 8080,
  host: "localhost",
  user: "admin",
  password: "secret",
  redis_host: "cache.local",
  redis_port: 6379,
  ttl: 3600
}
```

#### Method 7: Multi-Folder Import

Load files from multiple directories.

```avon
let lib_files = glob "lib/**/*.av" in
let data_files = glob "data/**/*.json" in
let libs = map (\f import f) lib_files in
let data = map (\f json_parse f) data_files in
[libs, data]
```

### Real-World Importing Scenarios

#### Scenario 1: Multi-Environment Configuration

Generate environment-specific configs from templates and data.

```avon
let env = "prod" in
let defaults = json_parse "config/defaults.json" in
let env_config = json_parse "config/{env}.json" in
let merged = dict_merge defaults env_config in

@output/{env}.conf {"{
  \"name\": \"{merged.name}\",
  \"port\": {merged.port},
  \"debug\": {merged.debug}
}"}
```

#### Scenario 2: Dynamic Function Library

Build a function library by importing multiple modules.

```avon
let stdlib = fold
  (\acc \f
    let module_name = str_replace ".av" "" (basename f) in
    set acc module_name (import f))
  {}
  (glob "lib/*.av")
in

# Now use: stdlib.math.add, stdlib.strings.upper, etc.
let result = stdlib.math.add 5 3 in
@output.txt {"Result: {result}"}
```

#### Scenario 3: Data Pipeline

Load multiple data files, transform, and export.

```avon
let users = map (\f json_parse f) (glob "data/users/*.json") in
let products = map (\f json_parse f) (glob "data/products/*.json") in

let user_count = length users in
let product_count = length products in

@report.md {"{# Data Report
- Total Users: {user_count}
- Total Products: {product_count}
"}
```

#### Scenario 4: Kubernetes Manifest Generator

Generate K8s manifests from configuration and template files.

```avon
let bases = fold
  (\acc \f set acc (basename f) (json_parse f))
  {} (glob "k8s/base/*.json") in
let services = fold
  (\acc \f set acc (basename f) (json_parse f))
  {} (glob "k8s/services/*.json") in

@deployment.yaml {"{
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {bases.app.name}
  namespace: {services.namespace}
spec:
  replicas: {bases.app.replicas}
  labels:
    app: {bases.app.name}
}"}
```

### When to Use Importing

✅ **Great for:**
- Loading configuration files from directories
- Building registries of functions or modules
- Aggregating data from multiple sources
- Deploying customized versions of templates
- Sharing reusable code libraries
- Fetching external templates from GitHub

❌ **Not ideal for:**
- Real-time file watching (Avon files are evaluated once at deploy time)
- Large binary files (Avon works with text)
- Files that change frequently during execution (evaluation is one-time)

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

**`doc` - Built-in Documentation:**

The `doc` command is one of Avon's most powerful features for rapid learning. It provides comprehensive, searchable documentation for all 40+ built-in functions without leaving your terminal.

```bash
# Show all available documentation
avon doc

# Look up a specific function
avon doc map
avon doc filter
avon doc join

# Browse functions by category
avon doc string      # All string functions
avon doc list        # All list functions  
avon doc dict        # All dictionary functions
avon doc math        # All math functions
avon doc io          # All I/O functions
avon doc template    # All template functions
avon doc type        # All type functions
avon doc logic       # All logic functions
```

**Example: Looking up `map`**

```bash
$ avon doc map
```

Output:
```
map :: (a -> b) -> [a] -> [b]
  Transform each item in a list by applying a function.
  
  Arguments:
    1. Function to apply to each element
    2. List to transform
  
  Example: map (\x x * 2) [1, 2, 3] -> [2, 4, 6]
           Double each number in the list
  
  Example: map upper ["hello", "world"] -> ["HELLO", "WORLD"]
           Convert each string to uppercase
  
  Tip: Use with partially applied functions:
       let double = map (\x x * 2) in
       double [1, 2, 3]
```

**Example: Browsing by category**

```bash
$ avon doc string
```

Output:
```
String Functions:
─────────────────
Basic Operations:
concat           Concatenate two strings
upper            Convert to uppercase
lower            Convert to lowercase
trim             Remove leading/trailing whitespace
length           Get length of string
repeat           Repeat string n times

Searching:
contains         Check if string contains substring
starts_with      Check if string starts with prefix
ends_with        Check if string ends with suffix
...
```

**Quick learning workflow:**

```bash
# 1. Browse a category to discover functions
avon doc list

# 2. Look up specific function documentation
avon doc filter

# 3. Test it immediately with 'run'
avon run 'filter (\x x > 2) [1, 2, 3, 4, 5]'

# 4. Use it in your program
# (edit your .av file with the function you learned)

# 5. Preview with eval
avon eval myprogram.av

# 6. Deploy when ready
avon deploy myprogram.av --root ./output --force
```

This tight feedback loop makes learning Avon intuitive and fast. Each function includes:
- **Type signature** - Shows parameter and return types
- **Description** - What the function does
- **Arguments** - Detailed parameter descriptions
- **Examples** - Real code showing common usage patterns
- **Tips** - Best practices and advanced usage

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

When a file evaluates to a function:**
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

How it works:**
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

How positional arguments work:**
- Arguments are applied in the order they appear
- Named arguments are applied first, then positional arguments fill remaining parameters
- If you mix named and positional, named arguments take priority

Example:
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

How defaults work:**
- If a named argument is provided, it's used
- If a positional argument is available, it's used
- If neither is provided, the default value is used
- If no default exists and no argument is provided, an error is shown

#### 4. Mixing Named and Positional

While possible, mixing named and positional arguments can be confusing. Avon prioritizes named arguments first, then fills remaining parameters with positional arguments in order.

Example:
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

Important: Single-dash arguments (like `-a`, `-c`) are always treated as named function parameters, not CLI flags. Only double-dash arguments (like `--force`, `--root`) are CLI options. This means you can use any single-letter or short name for your function parameters without conflicts.

**Best Practice:** Stick to either all named or all positional arguments for a single command invocation to avoid confusion.

#### 5. Argument Types

> Important: All command-line arguments passed to your Avon program are received as **strings**—even if you intend to use them as numbers or booleans, you must explicitly convert them inside your program.

Example:
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
- When a file evaluates to a function**, arguments are automatically applied
- **Named arguments** use `-param value` syntax
- **Positional arguments** are passed in order without names
- **Default values** are used when arguments aren't provided
- **All arguments are strings** - convert them in your code with `to_int`, `to_bool`, etc.
- **Arguments work the same** for both `eval` and `deploy` - the only difference is what happens with the final result

### Interactive REPL

The REPL (Read-Eval-Print Loop) is an interactive shell for exploring Avon. It's perfect for learning the language, testing expressions, and debugging. The REPL maintains a persistent symbol table, so variables you define persist across expressions, making it ideal for building up complex computations step by step.

Why Use the REPL?**

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

When to Use REPL vs Files:**

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

### Single File in Git, Many Deployments (The `--git` Workflow)

**This is one of Avon's most powerful and key features.** The `--git` flag enables a powerful pattern: keep **one template file in git** and let each environment, developer, or user deploy customized configs via CLI arguments. This is especially useful for **dotfiles**, shared configurations, team templates, and infrastructure code.

**Key Benefits:**
- **Easy sharing**: Put templates in GitHub, anyone can deploy with custom values
- **Centralized management**: One template, many customized deployments
- **Automatic updates**: When templates are updated, everyone can redeploy
- **No copying**: No need to clone repos or copy files between machines
- **Version control**: All templates are versioned in git

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

How type checking works:**
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
- No files are left in an inconsistent state (unlike your git history)
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

Avoid accidentally writing to system directories. Your `/etc` will thank you:

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

### Style and Formatting Guide

This section consolidates best practices for writing readable, maintainable Avon code.

#### Core Principles

1. **Readability First** — Code should be easy to read and understand
2. **Consistent Formatting** — Follow these conventions across your projects
3. **Leverage Dedent** — Avon automatically dedents templates based on first-line indentation

#### Template on Same Line as `in`

**Preferred:**
```avon
let name = "Alice" in @greeting.txt {"
  Hello, {name}!
  Welcome to Avon.
"}
```

**Avoid:**
```avon
let name = "Alice" in
@greeting.txt {"
Hello, {name}!
"}
```

**Reason:** Keeping the template on the same line as `in` makes the relationship clear and improves code readability.

#### Indent Template Content (Leverage Automatic Dedent)

**Preferred:**
```avon
@config.yml {"
  server:
    host: localhost
    port: 8080
  database:
    name: myapp
"}
```

**Avoid:**
```avon
@config.yml {"
server:
  host: localhost
  port: 8080
database:
  name: myapp
"}
```

**Why:** Avon's automatic dedent removes the common leading whitespace from all template lines. This means you can indent your template content in source code for readability without affecting the generated output. The generated file will have correct formatting with no extra leading spaces. This makes nested templates and indented Avon code much more readable.

#### How Dedent Works

1. Avon strips leading and trailing blank lines
2. Finds the first line with content—that line's indentation becomes the **baseline**
3. Removes that baseline amount of whitespace from every line
4. Lines with less indentation than baseline are kept as-is
5. Blank lines become empty

**Example showing baseline selection:**
```avon
let make_config = \service @config/{service}.yml {"
  service: {service}
  settings:
    enabled: true
    timeout: 30
"}
```

The first content line (`service: {service}`) has 2 spaces, so 2 spaces are removed from every line.

**Output** (after dedent):
```yaml
service: myapp
settings:
  enabled: true
  timeout: 30
```

**Pro tip:** You can indent templates as deep as you want in your source code:

```avon
let environments = ["dev", "staging", "prod"] in
let make_deploy = \env
  let replicas = if env == "prod" then "3" else "1" in
  @deploy-{env}.yaml {"
    apiVersion: apps/v1
    kind: Deployment
    spec:
      replicas: {replicas}
  "}
in
map make_deploy environments
```

Even though the template is 4+ levels deep, the generated files have zero leading spaces.

#### Let Bindings

**One binding per line:**
```avon
let port = 8080 in
let host = "localhost" in
let url = {"http://{host}:{port}"} in
url
```

**Cascading lets for readability** break complex logic into named steps:
```avon
let services = ["api", "web", "worker"] in
let make_config = \svc @config-{svc}.yml {"
  service: {svc}
"} in
map make_config services
```

#### Function Definitions

**Single parameter:**
```avon
let double = \x x * 2 in
map double [1, 2, 3]
```

**Multiple parameters (curried):**
```avon
let make_url = \protocol \host \port {"{protocol}://{host}:{port}"} in
make_url "https" "example.com" "443"
```

**Use snake_case for function names:**
```avon
let make_kubernetes_manifest = \service \env @k8s/{env}/{service}.yaml {"
  ...
"} in
```

#### Template Delimiter Strategy

When generating code with many braces, use multi-brace delimiters to keep templates clean:

| Output Format | Delimiter | Why |
|-------------|-----------|-----|
| YAML, plain text | `{" "}` | Few braces, no escaping |
| JSON, HCL, Terraform, Lua, Nginx | `{{" "}}` | Single braces are literal |
| GitHub Actions, Mustache | `{{{" "}}}` | Double braces are literal |

**Examples:**
```avon
# YAML: use single braces
@app.yml {"
app:
  name: myapp
  debug: { debug_mode }
"}

# JSON: use double braces
@config.json {{"
{
  "app": "{{ app_name }}",
  "port": {{ port }}
}
"}}

# Terraform: use double braces  
@main.tf {{"
resource "aws_instance" "web" {
  ami = "{{ ami_id }}"
  tags = {
    Name = "web-server"
  }
}
"}}
```

**Decision rule:** Choose the delimiter that keeps your template readable. More braces in output? Use more braces in the delimiter!

#### Handling Missing Data with `default`

Avon distinguishes between **data absence** (returns `None`) and **errors** (file I/O, etc.). The `default` builtin function provides a clean, composable way to handle missing values.

**Understanding None vs Errors:**

Many functions return `None` when data is absent:
- `head []` returns `None` (empty list)
- `get dict "missing_key"` returns `None` (key doesn't exist)
- `find (\x false) list` returns `None` (no match found)
- `nth 10 [1,2,3]` returns `None` (index out of bounds)
- JSON null values parse to `None`

File I/O functions return **errors** for missing files:
- `readfile "missing.txt"` → Error
- `json_parse "missing.json"` → Error
- These are exceptional conditions that halt execution

**The Problem: Verbose None Handling**

Without `default`, you must write verbose conditionals for every missing value:

```avon
let timeout = get config "timeout" in
if is_none timeout then 30 else timeout
```

This pattern repeats throughout your code, making it verbose and hard to read.

**The Solution: `default`**

The `default` function provides a fallback value when the second argument is `None`. It has the signature:

```
default :: a -> a -> a
```

Where the first argument is the fallback value, the second is the value to check.

**Simple usage:**
```avon
get config "timeout" -> default 30
# If timeout key exists, returns its value
# If timeout key is missing, returns 30
```

**Chaining with other functions:**
```avon
# Empty list handling
head [] -> default "no items"
nth 10 [1,2,3] -> default 0

# Find with fallback
find (\x x > 100) items -> default 0

# Nested access with defaults
get config "database" -> default {} -> (get "host" -> default "localhost")
```

**Key gotcha: Only None is replaced**

The `default` function ONLY checks for `None`. Falsy values like `false`, `0`, and `""` are NOT replaced:

```avon
false -> default true       # Returns false (not replaced!)
0 -> default 10             # Returns 0 (not replaced!)
"" -> default "default"     # Returns "" (not replaced!)
none -> default "fallback"  # Returns "fallback" (DOES replace)
```

This is intentional—these values are valid data, not missing data.

**Real-world example: Configuration with defaults**

```avon
let config = json_parse "app.json" in

let app_name = get config "name" -> default "MyApp" in
let port = get config "port" -> default 8080 in
let debug = get config "debug" -> default false in
let timeout = get config "timeout" -> default 30 in

@config.yml {"
name: {app_name}
port: {port}
debug: {debug}
timeout: {timeout}
"}
```

This elegantly handles optional configuration keys, using defaults only when keys are missing.

**Multi-level defaults with dict merging**

```avon
# Load environment-specific config, fall back to defaults
let defaults = {host: "localhost", port: 8080, timeout: 30} in
let user_config = json_parse "config.json" in
let final = dict_merge defaults user_config in

@app.conf {"
host: {final.host}
port: {final.port}
timeout: {final.timeout}
"}
```

**Lists with defaults:**
```avon
let items = [1, 2, 3] in
head items -> default 0           # 1
head [] -> default 0              # 0

let filtered = filter (\x x > 100) items in
head filtered -> default "none found"   # "none found"
```

**Best practice pattern:**

When your code interacts with optional data:
1. Access the data (e.g., `get config "key"`)
2. Pipe to `default` with a sensible fallback (e.g., `-> default "default_value"`)
3. Use the result

This makes optional data handling idiomatic and composable:

```avon
let config = json_parse "app.json" in

let make_service = \name
  @service-{name}.yaml {"
    name: {name}
    replicas: {get config "replicas" -> default 3}
    port: {get config "port" -> default 8080}
  "} in

map make_service ["api", "web", "worker"]
```

**See also:** `examples/default_builtin_before_after.av` for more comprehensive examples of the `default` function and how it simplifies code.

#### Lists and Collections

**Short lists on one line:**
```avon
let colors = ["red", "green", "blue"] in
```

**Long lists on multiple lines:**
```avon
let services = [
  "auth",
  "api", 
  "frontend",
  "worker",
  "cache",
  "db"
] in
```

#### Pipe Operator for Chaining

**Preferred:**
```avon
let result = [1, 2, 3, 4, 5]
  -> filter (\x x > 2)
  -> map (\x x * 2)
  -> fold (\acc \x acc + x) 0
in
result
```

**Avoid (deeply nested):**
```avon
fold (\acc \x acc + x) 0 (map (\x x * 2) (filter (\x x > 2) [1, 2, 3, 4, 5]))
```

Note: Only `->` is a valid pipe operator. The single `|` is not a pipe operator in Avon.

#### Comments

Use `#` for comments. Place them above the code they describe:

```avon
# Generate configuration for each environment
let environments = ["dev", "staging", "prod"] in

# Create a config file for each environment
let make_config = \env @config-{env}.yml {"
  environment: {env}
  debug: {if env == "prod" then "false" else "true"}
"} in

map make_config environments
```

#### Complete Style Example

This example demonstrates all style guidelines:

```avon
# Multi-environment Kubernetes deployment generator
let services = ["auth", "api", "frontend"] in
let environments = ["dev", "staging", "prod"] in

# Create a Kubernetes deployment manifest
let make_k8s_manifest = \service \env @k8s/{env}/{service}-deployment.yaml {"
  apiVersion: apps/v1
  kind: Deployment
  metadata:
    name: {service}
    namespace: {env}
  spec:
    replicas: {if env == "prod" then "3" else "1"}
    selector:
      matchLabels:
        app: {service}
"} in

# Generate all manifests
flatmap (\env map (\svc make_k8s_manifest svc env) services) environments
```

#### Style Checklist

Before committing your Avon code, verify:

- [ ] Templates start with `{"` on same line as path
- [ ] Template content is indented for source readability (dedent cleans output)
- [ ] Let bindings are on separate lines
- [ ] Functions use `snake_case` naming
- [ ] Complex logic is broken into named steps
- [ ] Comments explain the "why", not the "what"
- [ ] Code is formatted consistently (2-space indentation recommended)
- [ ] Pipe operator used for chaining operations
- [ ] Appropriate template delimiter chosen for brace density

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

How it works:**
1. Export the variable in your shell: `export DB_PASSWORD="my-secret-pass"`
2. Run Avon: `avon deploy config.av`

**Fail-Safe Behavior:**
If the environment variable `DB_PASSWORD` is missing, `env_var` will **fail immediately** with an error. This prevents you from accidentally deploying a configuration with empty or missing secrets. Unlike that one time in production. You know the one.

For optional variables, use `env_var_or`:
```avon
let port = env_var_or "PORT" "8080" in
# Uses "8080" if PORT env var is not set
```

## Security Best Practices

Avon prioritizes safety in code generation. This section covers security considerations for both developers and production deployments.

### Input Validation & Sanitization

Always validate and sanitize user-provided values before using them in templates:

```avon
# ❌ UNSAFE: Direct interpolation of user input
@config.json {"command": {user_input}}

# ✅ SAFE: Validate against whitelist
let commands = ["start", "stop", "restart"] in
let cmd = if contains commands input then input else "start" in
@script.sh {"command": {cmd}}
```

Whitelist validation prevents command injection and template escaping attacks.

### Template Safety Patterns

Avoid dangerous patterns when working with templates:

```avon
# ❌ DON'T: Treat user input as code
@exec.sh {exec_command}  # Dangerous if exec_command is user input

# ✅ DO: Treat everything as data
let safe_value = if starts_with value "safe_" then value else "" in
@config.json {"value": safe_value}
```

Avon doesn't execute arbitrary code, so all user input is treated as literal text. Keep it that way.

### File Deployment Safety

The deployment process is designed to be **atomic and fail-safe**:

1. **Three-Phase Deployment:**
   - **Phase 1: Validation** - All paths checked, no writes occur
   - **Phase 2: Permission Check** - Verify all files can be written before starting
   - **Phase 3: Writing** - Only after all validation passes do files get written
   
   If any error occurs, **zero files are written**.

2. **Safety Flags:**

```bash
# PREVIEW first (does NOT write files)
avon preview config.av --root ./output

# Deploy with safety
avon deploy config.av --root ./output --backup
```

| Flag | Behavior | Use When |
|------|----------|----------|
| `--root <dir>` | Confine all writes to directory | Always use this |
| `--backup` | Backup existing files to `.bak` | Updating critical files |
| `--if-not-exists` | Skip existing files | First-time setup |
| `--force` | Overwrite immediately | You're certain it's safe |

3. **Default Behavior:**
By default, `avon deploy` **skips** existing files and prints a warning. This is conservative by design.

### Production Checklist

Before deploying Avon in production:

- [ ] Always use `--root` flag to confine output
- [ ] Preview with `avon preview` before deploying
- [ ] Use `--backup` when updating existing configurations
- [ ] Validate all user input in templates
- [ ] Store templates in version control (Git)
- [ ] Review generated code before deployment
- [ ] Restrict file permissions on deployed files
- [ ] Keep Avon and dependencies up to date

### Path Security

Path validation in **Avon code** prevents directory traversal attacks:

```bash
# This is fine - normal file system navigation in the CLI
cd /some/deep/folder && avon deploy ../config.av --root ./output
```

However, within Avon code (`@` path literals), directory traversal is blocked:

```avon
# ❌ BLOCKED: Can't use .. in Avon code to escape --root
@../escape.json {"hack"}

# ✅ ALLOWED: Paths relative to --root stay within root
@config.json {"setting"}
@app/config/settings.json {"debug": true}
```

This distinction is important:
- **CLI file paths**: Use them however you want (normal file system rules)
- **Avon code paths** (`@file.txt`): Must be relative, no `..` allowed (security boundary)

```avon
# Paths are relative to --root (or current dir)
@config.json {"setting": "value"}

# Works with nested paths
@app/config/settings.json {"debug": true}

# Use forward slashes for cross-platform compatibility
@subdir/file.txt {"content": "data"}
```



---

## Real-World Examples

### Deployment Safety

Avon's deployment process is designed to be **truly atomic** and fail-safe. When deploying a list of FileTemplates, if any file cannot be written, **zero files are written**.

The process uses a three-phase approach to ensure atomicity:

**Phase 1: Preparation & Validation**
- All paths are validated for security (no path traversal)
- All parent directories are created
- No type errors occurred during evaluation

**Phase 2: Write Validation**
Before writing any files, Avon validates that **all** files can be written:
- For existing files: Verifies they can be opened for writing (checks permissions)
- For backup operations: Verifies backup location is writable
- For new files: Verifies parent directories are writable

**Phase 3: Writing**
Only after all files pass validation does Avon proceed to write them. Files are written sequentially, but since all have been validated, write failures are extremely rare.

This ensures truly atomic deployments—either all files write or none do.

### Preventing Accidental Overwrites

By default, `avon deploy` is **conservative**. It will **skip** any file that already exists on disk and print a warning.

To change this behavior, you must explicitly opt-in with `--backup`, `--force`, or `--append`.

### Example 1: Static Site Generator

Avon can be used as a static site generator, similar to Jekyll or Hugo, but with no dependencies and full functional programming support.

#### Quick Start: Minimal Example

The simplest example shows the core pattern in just 26 lines:

```bash
avon deploy examples/site_generator_minimal.av --root ./site --force
```

**The Pattern:**
1. Define markdown content as a template
2. Create HTML template with comment placeholders
3. Convert markdown to HTML using `markdown_to_html`
4. Replace placeholders with `replace`
5. Generate the file

```avon
let title = "My Site" in
let markdown = {"# Hello World
This is **markdown** content.
"} in
let html_template = {"
<!DOCTYPE html>
<html>
<head><title><!-- expand-title --></title></head>
<body>
    <h1><!-- expand-title --></h1>
    <!-- expand-body -->
</body>
</html>
"} in

# Convert markdown to HTML using built-in function
let html_body = markdown_to_html markdown in

# Replace placeholders (templates auto-convert to strings in string functions)
let html = replace (replace html_template "<!-- expand-title -->" title) "<!-- expand-body -->" html_body in

@index.html {"{html}"}
```

**Key Features:**
- **Templates for content**: Both HTML and markdown use Avon's template syntax `{"..."}` which allows interpolation
- **Built-in markdown conversion**: The `markdown_to_html` function handles headings, bold, italic, inline code, paragraphs, and line breaks
- **Template auto-conversion**: When you use `replace` (or any string function), templates automatically convert to strings
- **Comment placeholders**: HTML comments act as placeholders that get replaced with actual content

#### Multiple Pages Example

For more complex sites with multiple pages:

```avon
let posts = [
    {
        title: "Getting Started",
        author: "Alice",
        date: "2024-01-01",
        slug: "getting-started",
        content: {"# Getting Started\nWelcome to my blog!"}
    },
    {
        title: "Static Sites",
        author: "Bob",
        date: "2024-01-15",
        slug: "static-sites",
        content: {"# Static Sites\nStatic sites are great!"}
    }
] in

let generate_post = \post
    let html_body = markdown_to_html post.content in
    let html = replace (replace (replace html_template "<!-- expand-title -->" post.title) "<!-- expand-body -->" html_body) "<!-- expand-date -->" post.date in
    @posts/{post.slug}.html {"{html}"}
in

map generate_post posts
```

#### Advanced Features

You can extend site generators to support:
- **Links and images**: Use `replace` and `split` to process markdown-like syntax
- **Code blocks** with syntax highlighting
- **Tables** (parse pipe-separated lines)
- **External markdown files**: Use `readfile "posts/getting-started.md"`
- **RSS feeds**: Generate XML from post data
- **Multi-page layouts**: Create templates for about, contact, archives pages

#### Why Avon?

| Feature | Avon | Jekyll | Hugo |
|---------|------|--------|------|
| Template System | ✅ | ✅ | ✅ |
| Markdown Support | ✅ (built-in) | ✅ | ✅ |
| Functions | ✅ | ❌ | ❌ |
| Multi-file Output | ✅ | ✅ | ✅ |
| No Dependencies | ✅ | ❌ | ❌ |
| Functional Programming | ✅ | ❌ | ❌ |
| Type Safety | ✅ | ❌ | ❌ |

**Tip:** Use the `--git` flag to share your site generator templates! Put your `.av` file in GitHub, and others can deploy it with custom content: `avon deploy --git user/repo/site_gen.av --root ./site`

See `examples/site_generator_minimal.av`, `examples/site_generator_simple.av`, and `examples/site_generator_advanced.av` for complete working examples.

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
- ConfigMaps and Secrets (please don't actually put secrets here)
- Deployment with probes and resource limits
- Ingress, Service, and HPA configurations

### Example 6: GitHub Actions Workflow

See `examples/github_actions_gen.av`. Demonstrates:
- Conditional job configuration
- Matrix builds and multi-file generation
- Secrets and environment variable handling
- Complex nested YAML structures using triple-brace templates

### Example 7: Package.json Generator

See `examples/package_json_gen.av`. Shows:
- JSON generation from Avon code using double-brace templates
- Conditional dependency lists
- NPM script generation
- Dynamic package configuration

### Example 8: Multi-Brace Template Demo

See `examples/nginx_gen.av` or `examples/neovim_config.av`. These demonstrate:
- Double-brace templates `{{" "}}` for configs with literal braces
- Single braces are literal (no escaping needed)
- `{{expr}}` for interpolation within double-brace templates
- Use this pattern when generating JSON, Lua, Nginx, CSS, etc.

---

## Troubleshooting

Don't panic. Everyone hits these at some point.

### Common Errors

**"expected '\"' after opening braces"**  
This means a template isn't properly quoted. Templates require the syntax `{...}` with literal content or quotes if you need special formatting.

**"unexpected EOF"**  
You have an unclosed expression, list, or template. Check your brackets and braces. Count them. Count them again.

**"undefined identifier"**  
You referenced a variable that doesn't exist. Check spelling and make sure it's in scope (within a `let` binding or function parameter).

### Template Brace Troubleshooting

**Problem:** My literal braces aren't showing up.

**Solution:** Level up your delimiter. Use double-brace `{{" "}}` instead of single-brace `{" "}` to make single braces literal.

```avon
# Wrong (in a single-brace template, { starts interpolation)
@f.txt {"name: {"}    # Tries to interpolate {, expects closing }

# Correct (use double-brace template)
@f.txt {{"name: {"}}  # Outputs: name: {
```

**Problem:** I have lots of braces and it's getting confusing.

**Solution:** Use double or triple-brace templates so fewer braces require special handling:

```avon
# Single-brace (can't have literal braces easily)
@f.txt {"value: {x}"}

# Double-brace (single braces are literal)
@f.txt {{"obj: {value: {{x}}}}"}}  # Much clearer
```

**Problem:** Interpolation not working as expected.

**Solution:** Verify you're using the correct brace count. Interpolation uses the same number of braces as the delimiter:
- Single-brace template `{" "}`: use `{ expr }` for interpolation
- Double-brace template `{{" "}}`: use `{{ expr }}` for interpolation
- Triple-brace template `{{{" "}}}`: use `{{{ expr }}}` for interpolation

```avon
# Single-brace template
@f.txt {"Result: { 5 + 5 }"}     # Works, outputs: Result: 10

# Double-brace template  
@f.txt {{"Result: {{ 5 + 5 }}"}} # Works, outputs: Result: 10
@f.txt {{"Result: { 5 + 5 }"}}   # Literal braces, outputs: Result: { 5 + 5 }
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

4. **Test template braces:** When in doubt, test your template syntax:
   ```bash
   avon run '{{"Value: {{5}} and {literal}"}}' 
   # Outputs: Value: 5 and {literal}
   ```

---

## Gotchas and Common Pitfalls

Avon is designed to be simple, but like any language, it has edge cases. Here are the things that trip up developers:

### Gotcha 1: Function Parameters Are CLI Arguments

When you define a function with parameters, those parameter names become CLI arguments that the `deploy` command expects:

```avon
\host \port @config.txt {"Server: {host}:{port}"}
```

If you try to deploy without arguments, you'll get an error:
```
Error: Missing required argument: host
```

**Solution:** Either provide the arguments:
```bash
avon deploy config.av localhost 8080
```

Or give parameters defaults:
```avon
\host ? "localhost" \port ? 8080 @config.txt {"Server: {host}:{port}"}
```

Then deploy without arguments:
```bash
avon deploy config.av
```

### Gotcha 2: Variables Don't Shadow – They Nest

Avon doesn't allow variable shadowing. If you try to bind a variable that's already in scope, it's an error:

```avon
let x = 5 in
let x = 10 in  # Error: x already exists
x
```

**Solution:** Use different names for different scopes:
```avon
let x = 5 in
let y = 10 in
x + y
```

This might seem restrictive, but it actually prevents bugs where you accidentally reuse a variable name.

### Gotcha 3: Functions with All Defaults Still Return Functions

A common misconception: if a function has all defaults, you might think calling it returns the result. But Avon is functionally pure—functions are values:

```avon
let greet = \name ? "World" "Hello, {name}!" in
greet        # This is the result (auto-evaluated at top-level)
typeof greet # But typeof still shows "Function"
is_function greet # true
```

**The key:** Top-level auto-evaluation is a convenience, but if you store the function in a data structure, it stays a function:

```avon
let greet = \name ? "World" "Hello, {name}!" in
{fn: greet}  # greet is NOT auto-evaluated here
```

**Solution:** If you need the result, evaluate it first:
```avon
let greet = \name ? "World" "Hello, {name}!" in
let result = greet in
{greeting: result}  # Now it's the evaluated result
```

### Gotcha 4: No Recursion – Use `fold` Instead

Avon doesn't support recursion. If you try to call a function from within itself, it won't work:

```avon
let countdown = \n
  if n <= 0 then "Done"
  else "{n}, {countdown (n - 1)}"  # Won't work!
in countdown 5
```

**Why?** Avon is designed for data generation and transformation, not control flow. Recursion can lead to infinite loops and confusion.

**Solution:** Use `fold`, `map`, `filter`, and similar higher-order functions instead:

```avon
# Instead of recursion, use range + map
let numbers = [5, 4, 3, 2, 1] in
join (map to_string numbers) ", "
# Output: 5, 4, 3, 2, 1
```

Or use `fold` for reduction:
```avon
let sum = fold (\acc \n acc + n) 0 [1, 2, 3, 4, 5] in
sum  # 15
```

### Gotcha 5: Template Braces Can Be Confusing

When you mix templates with different brace counts, it's easy to get confused:

```avon
# This tries to interpolate { as a variable (wrong)
@f.txt {"value: {"}

# This outputs literal { (correct)
@f.txt {{"value: {"}}
```

**Solution:** Remember the rule: interpolation uses the **same number of braces** as the template delimiter.

- Single-brace `{" "}`: interpolate with `{ expr }`
- Double-brace `{{" "}}`: interpolate with `{{ expr }}`
- Triple-brace `{{{" "}}}`: interpolate with `{{{ expr }}}`

```avon
# Single-brace template with interpolation
@f.txt {"Result: { 1 + 2 }"}          # Outputs: Result: 3

# Double-brace template (easier if you need literal braces)
@f.txt {{"Object: { key: {{ val }} }"}}  # Outputs: Object: { key: val }
```

**Special case: Wrapping variables in braces**

If you need to generate braces **around** interpolated values (e.g., `{variable_name}` where the variable holds the name), use a wrapping function:

```avon
# Problem: You want output like {name}, {email}, {age}
# where the field names come from a list

# Solution: Create a wrap function
let wrap = \x "{" + x + "}" in
let fields = ["name", "email", "age"] in

@template.json {"
{
{join (map (\f "  \"" + f + "\": " + (wrap f)) fields) ",\n"}
}
"}
```

Output:
```json
{
  "name": {name},
  "email": {email},
  "age": {age}
}
```

This works because:
1. The wrap function returns a string: `"{" + x + "}"`
2. String concatenation uses single `+` operator 
3. The resulting string is then interpolated into the template
4. This technique generates template placeholders dynamically from data

### Gotcha 6: `json_parse` Only Reads from Files — Use `json_parse_string` for Strings

`json_parse` (and all `*_parse` functions) read from files, not from strings directly:

```avon
json_parse "settings.json"     # ✓ Reads file and parses JSON
json_parse "{\"name\": \"Alice\"}"  # ✗ Tries to read file "{\"name\": \"Alice\"}" (errors)
```

**Solution:** Use the companion `*_parse_string` function to parse a raw string directly:

```avon
json_parse_string "{\"name\": \"Alice\"}"   # ✓ Parses the JSON string
yaml_parse_string "name: Alice\nage: 30"    # ✓ Parses the YAML string
toml_parse_string "[server]\nport = 8080"   # ✓ Parses the TOML string
csv_parse_string "name,age\nAlice,30"       # ✓ Parses the CSV string
xml_parse_string "<root><item/></root>"     # ✓ Parses the XML string
html_parse_string "<div>Hello</div>"        # ✓ Parses the HTML string
ini_parse_string "[db]\nhost=localhost"     # ✓ Parses the INI string
```

Every file parser has a `*_parse_string` counterpart. This separation is intentional — it keeps the intent explicit: file parsers read files, string parsers parse strings. No ambiguity.

### Gotcha 7: Lists in Templates Expand to Multiple Lines

When you interpolate a list in a template, each item goes on its own line:

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

This is **intentional and useful** for generating lists, but it can surprise you if you're expecting inline output.

**Solution:** Use `join` if you want items on one line:
```avon
let items = ["apple", "banana", "cherry"] in
@shopping.txt {"Items: {join items ", "}"}
```

Output:
```
Items: apple, banana, cherry
```

### Gotcha 8: `glob` Returns Paths, Not Contents

`glob` finds files matching a pattern but returns their **paths**, not contents:

```avon
glob "config/*.json"
# Returns: ["config/app.json", "config/db.json"]  (paths only)
```

If you want contents, you need to `readfile`:
```avon
let files = glob "config/*.json" in
map readfile files
# Returns: contents of each file
```

**Solution:** Remember the pipeline:
1. `glob` → find files (returns paths)
2. `readfile` → read file contents
3. `json_parse` → parse JSON

### Gotcha 9: Import Evaluates the Entire File

When you use `import`, the entire file is evaluated as an expression. The result is what the file returns:

```avon
# math.av
{double: \x x * 2, square: \x x * x}

# main.av
let math = import "math.av" in
math.double 5  # 10
```

This means the imported file could return anything—a dictionary, a function, a number, a FileTemplate, whatever. This is powerful but means you need to know what each imported file returns.

**Solution:** Make it clear in your file what it returns. Consider a convention like:
- `lib_*.av` files return function libraries (dicts of functions)
- `config_*.av` files return configuration dicts
- `generate_*.av` files return FileTemplates or lists of FileTemplates

### Gotcha 10: Strings vs Templates – Two Different Types

Avon has **two distinct types** for text data, and understanding the difference is crucial:

#### Strings: `"..."` – Literal text with escape sequences

```avon
"Hello {name}!"       # Literal string: Hello {name}!
"Line 1\nLine 2"      # Escape sequences work: prints on two lines
typeof "text"         # String
```

**Strings:**
- Use double quotes: `"..."`
- Support escape sequences: `\n`, `\t`, `\\`, `\"`
- Braces are literal (no interpolation)
- Can be concatenated with `+`

#### Templates: `{"..."}` – Interpolated expressions

```avon
let name = "Alice" in
{"Hello {name}!"}     # Interpolates: Hello Alice!

let x = 10 in
{"x = {x}, double = {x * 2}"}  # x = 10, double = 20

typeof {"text"}       # Template
```

**Templates:**
- Use curly braces: `{"..."}`
- Support interpolation: `{expr}` evaluates and inserts the result
- Escape sequences are literal (no `\n` or `\t` processing)
- Can be concatenated with `+`: `{"hello"} + {" world"}`

#### Common Mistake: Using the Wrong Type

```avon
# ✗ Wrong: String doesn't interpolate
let name = "Bob" in
"Hello {name}!"  # Returns: Hello {name}!  (literal braces)

# ✓ Correct: Template interpolates
let name = "Bob" in
{"Hello {name}!"}  # Returns: Hello Bob!
```

#### When to Use Each

**Use Strings when:**
- You need escape sequences (`\n`, `\t`)
- You need string concatenation
- You're building paths or simple text
- You don't need variable interpolation

```avon
"Line 1\nLine 2\n"          # Escape sequences
"Hello " + name + "!"        # Concatenation
"/path/to/file.txt"          # Literal paths
```

**Use Templates when:**
- You need to interpolate variables or expressions
- You're generating code or config with dynamic values
- You're using deployment templates `@file {...}`

```avon
{"Server: {host}:{port}"}                    # Interpolation
{"Sum: {x + y}, Product: {x * y}"}          # Expressions
@config.json {{"port": {port}}}             # Deployment templates
```

#### Type Coercion: Templates Auto-Convert for Functions

Here's an important convenience feature: **templates are automatically coerced to strings** when passed to string functions:

```avon
# String functions accept templates:
length {"hello"}                    # 5
trim {"  spaces  "}                 # "spaces"
split {"a,b,c"} ","                 # ["a", "b", "c"]
concat {"template"} " string"       # "template string"

# With interpolation:
let x = 5 in
length {"Value: {x}"}               # 8 (counts "Value: 5")
```

**But the `+` operator is strict** - no coercion:

```avon
{"Hello"} + {" world"}  # ✓ Template + Template works
"Hello" + " world"      # ✓ String + String works

{"Hello"} + " world"    # ✗ Error: cannot add Template and String
"Hello" + {" world"}    # ✗ Error: cannot add String and Template
```

**Why the difference?**
- Functions use **implicit coercion** for convenience (templates → strings)
- Operators use **strict typing** to prevent subtle bugs
- Use `concat` if you need to mix types: `concat {"template"} " string"`

**Key Takeaway:** If you need interpolation, use templates `{"..."}`. If you need escape sequences, use strings `"..."`. String functions accept both and will coerce templates automatically.

### Gotcha 11: Dict Literals Need No Spaces in Shell

When using dicts at the command line, don't put spaces after colons:

```bash
# ✗ Wrong (causes timeout/hang due to shell brace expansion)
avon run 'keys {a: 1, b: 2}'

# ✓ Correct (no spaces after colons)
avon run 'keys {a:1,b:2}'
```

**Why?** Bash performs brace expansion on `{a: 1}` before Avon sees it. Without spaces, bash treats it as a single token.

**Inside .av files, spaces are fine:**
```avon
# In a .av file, this works perfectly:
let config = {
  host: "localhost",
  port: 8080,
  debug: true
} in
config
```

**Solution:** Only worry about this when passing dicts directly on the command line. In files, format however you like.

### Gotcha 12: Operators Can't Be Used as Prefix Functions

Unlike some functional languages, you can't use operators in prefix position:

```avon
(+ 1 2)     # ✗ Error: expected function, found unknown
map (+ 1) [1, 2, 3]  # ✗ Won't work
```

Operators are **infix only** in Avon:

```avon
1 + 2       # ✓ Works
```

**Solution:** Use lambdas to wrap operators:
```avon
map (\x x + 1) [1, 2, 3]  # ✓ Returns [2, 3, 4]
```

### Gotcha 13: String Length Counts UTF-8 Bytes, Not Characters

The `length` function returns UTF-8 byte count, not visual character count:

```avon
length "hello"    # 5 (ASCII, 1 byte per char)
length "🌍"       # 4 (emoji is 4 bytes in UTF-8)
length "世界"     # 6 (3 bytes per character)
```

**But `chars` works correctly:**
```avon
chars "🌍世界"  # ["🌍", "世", "界"] - 3 characters!
```

**Solution:** Use `length (chars str)` if you need character count:
```avon
let str = "Hello 🌍!" in
length (chars str)  # 8 characters (including emoji)
```

### Gotcha 14: The `+` Operator Is Generic (But Type-Safe)

The `+` operator works with multiple types for different operations:

```avon
# Numbers: addition
5 + 10                    # 15

# Strings: concatenation
"hello" + " world"        # "hello world"

# Templates: concatenation
{"hello"} + {" world"}    # "hello world"

# Lists: concatenation
[1, 2] + [3, 4]          # [1, 2, 3, 4]
```

**But both operands must be the same type:**

```avon
"hello" + {"world"}      # ✗ Error: cannot add String and Template
[1, 2] + "text"          # ✗ Error: cannot add List and String
```

**The `concat` function** is specific to strings and templates:

```avon
concat "hello" " world"  # Works with strings
concat {"hello"} {" world"}  # Works with templates
concat [1, 2] [3, 4]     # ✗ Error: expected string or template
```

**Practical tip:** Use `+` for most concatenation. Use `concat` when you need it as a function (e.g., with `fold` or `map`):

### Gotcha 15: No Type Coercion (Be Explicit)

Avon has strict typing with no implicit conversions:

```avon
"5" == 5        # ✗ Error: cannot compare String with Number
"hello" + 5     # ✗ Error: cannot add String and Number
if 1 then "yes" # ✗ Error: expected bool, found 1
```

**Why?** Type errors catch bugs early. If `if 1` worked, what about `if 0`? Explicit is better.

**Solution:** Convert types explicitly:
```avon
to_string 5              # "5"
to_int "42"              # 42
to_string 1 + " item"    # "1 item"

# For boolean conditions, use comparisons:
if (length items) > 0 then "has items" else "empty"
```

### Gotcha 16: `readfile` Requires UTF-8

Binary files will error when read with `readfile`:

```avon
readfile "image.png"  # ✗ Error: stream did not contain valid UTF-8
```

**Why?** Avon works with text, not arbitrary binary data.

**Solution:** Avon is designed for text-based file generation. If you need binary file paths, use `glob` to get paths, not contents:
```avon
glob "images/*.png"  # ✓ Returns list of paths
```

### Gotcha 17: Avon is Single-Pass and Simple

Avon intentionally doesn't have advanced features like:
- Recursion (use `fold` instead)
- Mutable state (use functional transformations)
- Complex type system (rely on runtime errors for type safety)
- Module system beyond `import` (keep dependencies simple)

**Why?** This simplicity makes Avon:
- Fast (no complex analysis needed)
- Predictable (what you see is what you get)
- Easy to learn (fewer concepts)
- Safe (runtime type safety prevents deployment errors)

**Solution:** Embrace functional programming patterns. They're more powerful than they first appear.

### Gotcha 18: `pfold` Requires an Associative Combiner

The parallel functions `pmap`, `pfilter`, and `pfold` use multiple CPU cores for better performance on large lists. While `pmap` and `pfilter` always produce identical results to their sequential counterparts, **`pfold` requires the combiner function to be associative**.

```avon
# ✓ Associative operations work correctly with pfold
fold (\acc \x acc + x) 0 [1, 2, 3, 4, 5]    # 15
pfold (\acc \x acc + x) 0 [1, 2, 3, 4, 5]   # 15 (same result!)

fold (\acc \x acc * x) 1 [1, 2, 3, 4, 5]    # 120
pfold (\acc \x acc * x) 1 [1, 2, 3, 4, 5]   # 120 (same result!)

# ✗ Non-associative operations give DIFFERENT results
fold (\acc \x acc - x) 0 [1, 2, 3, 4, 5]    # -15
pfold (\acc \x acc - x) 0 [1, 2, 3, 4, 5]   # 15 (WRONG! Different result!)
```

**Why?** Parallel fold splits the list into chunks, folds each chunk in parallel, then combines the results. For non-associative operations like subtraction or division, the order of operations matters:
- Sequential: `((((0 - 1) - 2) - 3) - 4) - 5 = -15`
- Parallel: Chunks may combine as `(0 - 1 - 2) - (3 - 4 - 5) = 3` or other orders

**When to use parallel functions:**
- `pmap`/`pfilter`: Safe to use anytime; ideal for large lists (1000+ elements) with CPU-intensive functions
- `pfold`: Only use with **associative** combiners: `+`, `*`, `max`, `min`, `and`, `or`, `++` (list concat)

**When NOT to use parallel functions:**
- Small lists (under ~100 elements): The overhead of parallelism outweighs the benefit
- Simple operations: `map (\x x + 1)` on a small list is faster sequentially
- `pfold` with subtraction, division, or order-dependent accumulation

### Gotcha 19: Division Always Returns Float

Division `/` always returns a float, even for integer operands:

```avon
10 / 3       # => 3.3333... (NOT 3!)
6 / 2        # => 3.0 (float, not int)
-7 / 2       # => -3.5
```

**Solution:** Use `//` for integer (floor) division:
```avon
10 // 3      # => 3 (integer floor division)
6 // 2       # => 3 (integer)
-7 // 2      # => -4 (floors toward negative infinity)
```

### Gotcha 20: Range with Start > End Returns Empty List

Avon's `range` function (and shorthand `[a..b]` syntax) only works for ascending sequences:

```avon
range 1 5    # => [1, 2, 3, 4, 5]
[1..5]       # => [1, 2, 3, 4, 5] (shorthand syntax)
range 5 1    # => [] (NOT [5, 4, 3, 2, 1]!)
[5..1]       # => [] (shorthand also returns empty)
range 5 5    # => [5] (single element works)
```

**Solution:** Use step syntax or reverse:
```avon
[10, -1 .. 0]         # => [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0] (step of -1)
[10, -2 .. 0]         # => [10, 8, 6, 4, 2, 0] (step of -2)
range 1 5 -> reverse  # => [5, 4, 3, 2, 1]
[1..5] -> reverse     # => [5, 4, 3, 2, 1]
```

### Gotcha 21: Power Operator is Right-Associative

The `**` operator associates to the right, unlike most other operators:

```avon
2 ** 3 ** 2   # => 512 (evaluates as 2 ** (3 ** 2) = 2 ** 9)
(2 ** 3) ** 2 # => 64 (explicit left grouping)
```

This matches mathematical convention where $2^{3^2} = 2^9 = 512$.

### Gotcha 22: Floating Point Precision Quirks

Standard IEEE 754 floating-point precision issues apply:

```avon
0.1 + 0.2        # => 0.30000000000000004
0.1 + 0.2 == 0.3 # => false!
```

**Solution:** For financial calculations, work with integers (cents instead of dollars).

### Gotcha 23: `all` on Empty List Returns True

Vacuous truth applies to `all`:

```avon
all (\x x > 0) []  # => true (vacuously true!)
any (\x x > 0) []  # => false
```

This is mathematically correct (universal quantification over empty set is true), but can be surprising.

### Gotcha 24: `zip` Truncates to Shorter List

When zipping lists of different lengths, extra elements are dropped:

```avon
zip [1, 2, 3] [4, 5]           # => [[1, 4], [2, 5]] (3 is dropped!)
zip_with (\a \b a + b) [1, 2, 3] [10, 20]  # => [11, 22]
```

### Gotcha 25: Caret `^` is Not a Power Operator

The caret `^` is not recognized in Avon. Use `**` for exponentiation:

```avon
2 ^ 8    # ERROR: expected function, found number
2 ** 8   # => 256 ✓ (use ** for power)
pow 2 8  # => 256 ✓ (function form also works)
```

Note: `**` is right-associative (see Gotcha 21).

### Gotcha 26: `min` and `max` Take Lists, Not Two Arguments

Unlike some languages, `min`/`max` operate on lists:

```avon
min 3 7           # ERROR: expected list
min [3, 7, 1, 5]  # => 1 ✓
max [3, 7, 1, 5]  # => 7 ✓
```

### Gotcha 27: List Element Access Returns `None` for Invalid Indices

Invalid indices don't error—they return `None`:

```avon
head []              # => None
nth 10 [1, 2, 3]     # => None (out of bounds)
nth (neg 1) [1,2,3]  # => None (negative indices don't wrap)
```

**Tip:** Use `last` for the last element instead of negative indexing.

### Gotcha 28: `contains` Has Different Semantics for Strings vs Lists

The `contains` function is overloaded for both strings and lists:

```avon
# For strings: contains "haystack" "needle" => checks if haystack contains needle
contains "hello world" "wor"    # => true

# For lists: contains elem list => checks if list contains elem
contains 3 [1, 2, 3, 4]         # => true
contains "apple" ["apple", "banana"]  # => true
```

Note the argument order difference:
- Strings: `contains haystack needle`
- Lists: `contains element list`

---

## Tips and Tricks

This section contains useful patterns and idioms for effective Avon programming.

### Tip: Check List Membership with `contains` or `any`

Use `contains` for simple membership checks, or `any` for complex predicates:

```avon
# Simple membership with contains
contains 3 [1, 2, 3, 4]  # => true

# Complex predicates with any
any (\x x > 5) [1, 2, 3, 10]  # => true (checks if any element > 5)
```

### Tip: Safe Division with Default Value

Avoid division by zero errors with a guard:

```avon
let safe_div = \a \b if b == 0 then none else a / b
in safe_div 10 0  # => None instead of error
```

### Tip: Default Value for None Results

Handle `None` results gracefully:

```avon
let with_default = \default \value if value == none then default else value
let result = find (\x x > 100) [1, 2, 3]
in with_default 0 result  # => 0
```

### Tip: Check if Dict Has a Key

Use `find` on `keys` to check for key existence:

```avon
let check = \key \dict (find (\k k == key) (keys dict)) != none
in check "a" {a: 1, b: 2}  # => true
```

### Tip: Use `typeof` for Runtime Type Checking

Determine the type of any value at runtime:

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

Type-specific boolean checks:

```avon
is_list [1, 2, 3]   # => true
is_dict {a: 1}      # => true
is_string "hello"   # => true
is_number 42        # => true
is_none none        # => true
is_bool true        # => true
```

### Tip: Function Composition via Pipes

Chain transformations in a readable way:

```avon
let double = \x x * 2
let inc = \x x + 1
in 5 -> double -> inc  # => 11

# Or inline for data pipelines
[1, 2, 3, 4, 5]
  -> map (\x x * 2)
  -> filter (\x x > 5)
  -> fold (\a \b a + b) 0
# => 18
```

### Tip: Use `flatten` to Concatenate Lists of Lists

```avon
flatten [[1, 2], [3, 4], [5]]  # => [1, 2, 3, 4, 5]
```

### Tip: Group and Partition Data

```avon
# Partition into matching/non-matching groups
partition (\x x > 3) [1, 2, 3, 4, 5]  # => [[4, 5], [1, 2, 3]]

# Group by key function
group_by (\x x % 2) [1, 2, 3, 4, 5]   # => {0: [2, 4], 1: [1, 3, 5]}
```

### Tip: Use `chars` for String Character Operations

Since `nth` doesn't work on strings directly:

```avon
# Get character at index
nth 0 (chars "hello")  # => "h"

# Count actual characters (not bytes)
length (chars "café")  # => 4 (not 5)

# Iterate over characters
chars "hello" -> map upper  # => ["H", "E", "L", "L", "O"]
```

### Tip: Unicode Characters Work Correctly in `chars`

```avon
chars "αβγ"   # => ["α", "β", "γ"]
chars "👋🌍"  # => ["👋", "🌍"]
```

### Tip: Use `neg` for Negative Numbers in Expressions

When you need a negative number as an argument:

```avon
abs (neg 5)  # => 5
0 - 5        # Also works: => -5
```

### Tip: `take` with Large Count is Safe

Requesting more elements than available doesn't error:

```avon
take 100 [1, 2, 3]  # => [1, 2, 3] (returns what's available)
take 0 [1, 2, 3]    # => []
```

### Tip: Map with Index Using `zip`

```avon
let items = ["a", "b", "c"]
let len = length items
in zip (range 0 (len - 1)) items  # => [[0, "a"], [1, "b"], [2, "c"]]
```

### Tip: Extract Nested Data from Lists of Dicts

```avon
let users = [
  {name: "Alice", age: 30},
  {name: "Bob", age: 25}
]
in users -> map (\u u.name)  # => ["Alice", "Bob"]
```

---

## Piping, Stdin, Stdout, and Embedding Avon

Avon works great with shell pipes and can be embedded in other programs. Here's how:

### Piping Avon Source Code into the CLI

You can pipe Avon **source code** to the CLI using `--stdin` or `-` as the filename:

```bash
# Using --stdin flag
echo '1 + 2' | avon eval --stdin
# Output: 3

# Using '-' as filename (same effect)
echo 'map (\x x * 2) [1, 2, 3]' | avon eval -
# Output: [2, 4, 6]
```

This is useful when generating Avon programs dynamically or fetching them from other sources.

### Piping Data into an Avon Program

If you want your Avon program to **read piped data** (not source code), use `avon run` with a code argument and read from `/dev/stdin`:

```bash
# Pipe data into an Avon program
echo -e 'hello\nworld\ntest' | avon run 'readfile "/dev/stdin"'
# Output:
# hello
# world
# test

# Process piped data line by line
echo -e 'alice\nbob\ncharlie' | avon run 'map upper (lines (readfile "/dev/stdin"))'
# Output: [ALICE, BOB, CHARLIE]
```

**Key difference:**
- `--stdin` or `-`: Avon reads stdin as **program source**
- `readfile "/dev/stdin"`: Avon program reads stdin as **data**

When you use `avon run 'code'`, the code is passed as an argument, so stdin remains available for the program to read via `/dev/stdin`.

**Note:** This works on Linux/macOS. On Windows, `/dev/stdin` is not available.

### Capturing Avon Output

Avon prints results to stdout, making it easy to capture in shell scripts or other programs:

```bash
# Capture output in a variable
result=$(avon run '[1, 2, 3] -> map (\x x * 10) -> fold (\a \b a + b) 0')
echo "Captured result: $result"
# Output: Captured result: 60
```

### Exit Codes

Avon returns proper exit codes for scripting:
- **0**: Success
- **1**: Error (syntax, runtime, or missing arguments)

```bash
# Success case
avon run '42 * 2'
echo "Exit code: $?"
# Output:
# 84
# Exit code: 0

# Error case
avon run 'unknown_function 1'
echo "Exit code: $?"
# Output:
# unknown symbol: unknown_function on line 1 in <input>
# Exit code: 1
```

### Debug Output Goes to Stderr

The `trace` and `spy` builtins print to stderr, keeping stdout clean for results:

```bash
# trace output goes to stderr, result to stdout
avon run 'let _ = trace "debug" 42 in 100'
# stderr: [TRACE] debug: 42
# stdout: 100

# Suppress debug output, keep only result
avon run 'let _ = trace "debug" 42 in 100' 2>/dev/null
# Output: 100
```

This is useful in pipelines where you want debug info but need clean output:

```bash
# Debug goes to terminal, clean result goes to next command
avon run 'let _ = trace "step1" [1,2,3] in map (\x x*2) [1,2,3]' 2>&1 | head -1
```

### Embedding Avon in Other Programs

Any program can spawn Avon and capture results. Here's the pattern:

**Bash:**
```bash
#!/bin/bash
result=$(avon run 'map (\x x * 2) [1, 2, 3]')
if [ $? -eq 0 ]; then
    echo "Success: $result"
else
    echo "Avon failed"
fi
```

**Python:**
```python
import subprocess

result = subprocess.run(
    ["avon", "run", "1 + 2 + 3"],
    capture_output=True,
    text=True
)

if result.returncode == 0:
    print(f"Result: {result.stdout.strip()}")  # "6"
else:
    print(f"Error: {result.stderr}")
```

**Node.js:**
```javascript
const { execSync } = require('child_process');

try {
    const result = execSync('avon run "join [1,2,3] \\"-\\""').toString().trim();
    console.log(`Result: ${result}`);  // "1-2-3"
} catch (e) {
    console.error(`Error: ${e.stderr}`);
}
```

### Real-World Integration: File Collection Scripts

A common use case is integrating Avon into build tools or utilities that need to know which files to process. For example, a bundler might need to collect source files, or a deployment tool might need to identify which files to ship.

**The scenario:** Your tool runs `mytool build src/main.rs src/lib.rs` and needs to extract the file paths from those arguments.

#### Using the `args` Builtin (Recommended)

The `args` builtin provides all positional command-line arguments as a list. This is the cleanest approach:

**A file-collector script (`.collect-files`):**

```avon
# .collect-files - extract files from command args using args builtin
# When user runs: mytool build src/main.rs src/lib.rs
# Your tool calls: avon eval .collect-files build src/main.rs src/lib.rs
# Returns: src/main.rs (first existing file)

# args contains all positional arguments: ["build", "src/main.rs", "src/lib.rs"]
# Filter to only files that exist
args -> filter (\f (exists f)) -> head
```

**Calling it from your tool:**

```bash
# Pass the original command args to avon
avon eval .collect-files build src/main.rs src/lib.rs
# Output: src/main.rs
```

**Handling multiple files with `args`:**

```avon
# .collect-files - extract multiple files from args
# args gives us ALL command line arguments as a list - no limit!

# Filter to only files that exist (flags like --verbose are filtered out)
let files = args -> filter (\f (exists f)) in

# Return one file per line
join files "\n"
```

```bash
avon eval .collect-files build src/main.rs src/lib.rs tests/test.rs
# Output:
# src/main.rs
# src/lib.rs
# tests/test.rs
```

**Why `args` is powerful:**
- No limit on number of arguments (unlike `\arg1 ? ""` which requires predefined parameters)
- Automatically available in every script
- Works with `filter`, `map`, `fold`, and all list operations
- Clean one-liner for common cases

#### Alternative: Named Parameters

For simpler cases with known argument positions, you can also use named parameters with defaults:

```avon
# .collect-files - using named parameters
\cmd ? ""
\file ? ""

# Return the file if it exists
if exists file then file else ""
```

This works but is limited to the number of parameters you define.

**Using glob patterns instead:**

If you want the user to specify a pattern rather than individual files:

```avon
# .collect-files - glob-based file collection
\pattern ? "src/*.rs"

join (glob pattern) "\n"
```

```bash
avon eval .collect-files "src/*.rs"
# Output: all .rs files in src/
```

**Parsing in Rust:**

```rust
use std::process::Command;

fn get_files_from_avon(config_file: &str, args: &[String]) -> Result<Vec<String>, String> {
    // Call avon with the config file and pass through all args
    let mut cmd_args = vec!["eval".to_string(), config_file.to_string()];
    cmd_args.extend(args.iter().cloned());
    
    let output = Command::new("avon")
        .args(&cmd_args)
        .output()
        .map_err(|e| format!("Failed to run avon: {}", e))?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse newline-separated output
    let files: Vec<String> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect();
    
    Ok(files)
}

// Usage in your tool:
// let user_args = vec!["build", "src/main.rs", "src/lib.rs"];
// let files = get_files_from_avon(".collect-files", &user_args)?;
// for file in files {
//     bundle.add_file(&file)?;
// }
```

**Why this pattern works well:**
- Users write simple config files to customize which files to collect
- Your tool passes command args directly to `avon eval <config> <args...>`
- The config file uses `exists` to verify files are real
- Exit code 0 = success, 1 = error (check stderr for details)
- Output is newline-separated for easy parsing
- The `glob` builtin handles all the pattern matching
- The `args` builtin gives access to all command-line arguments

### Summary: Stdin/Stdout Modes

| Mode | Command | Stdin Used For | Output |
|------|---------|----------------|--------|
| Source from stdin | `cat prog.av \| avon eval --stdin` | Program source | Result to stdout |
| Source from stdin | `echo '1+2' \| avon eval -` | Program source | Result to stdout |
| Data from stdin | `cat data \| avon run 'readfile "/dev/stdin"'` | Data for program | Result to stdout |
| Normal file | `avon eval prog.av` | Not used | Result to stdout |
| Inline code | `avon run '1+2'` | Available for `/dev/stdin` | Result to stdout |

---

Ready to build something? Here are some ideas:

1. **Configuration Generator:** Generate config for multiple services (Redis, PostgreSQL, etc.)
2. **Project Scaffolder:** Create project structure for a new Node/Python/Rust project
3. **CI/CD Automation:** Generate GitHub Actions, GitLab CI, or other CI workflows
4. **Infrastructure as Code:** Generate Terraform, Ansible, or CloudFormation templates
5. **Documentation:** Auto-generate README files, API docs, or changelog templates

---

If you have questions or want to contribute examples, the Avon project welcomes contributions! Check out the repository for more details.

Happy generating!

<!-- 
Secret: Run `avon run 'join (map to_char [72, 101, 108, 108, 111]) ""'` for a surprise.
If you found this, you're the kind of person who reads source code. We respect that.
-->
