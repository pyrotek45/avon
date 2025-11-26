# Avon — The Modern Template Language for Developers

Welcome to **Avon**. You're about to give your configuration workflow superpowers.

Avon is designed for developers who are tired of copy-pasting. Whether you're building Kubernetes manifests, setting up CI/CD pipelines, or generating boilerplate code, Avon turns repetitive tasks into elegant, maintainable code.

**Pro tip:** Throughout this guide, look at the `examples/` directory for real-world use cases. Each example demonstrates practical Avon patterns you can adapt for your own projects.

---

## Table of Contents

1. **[Quick Start](#quick-start)** — Get up and running in 60 seconds
2. **[Core Concepts](#core-concepts)** — Values, types, and the Avon model
3. **[Language Essentials](#language-essentials)** — Syntax, expressions, and operators
4. **[Functions & Variables](#functions--variables)** — Defining and using functions, let bindings
5. **[Collections](#collections)** — Lists, dictionaries, and the powerful `map`, `filter`, `fold` operations
6. **[Templates](#templates)** — The heart of Avon: generating text output
7. **[File Templates & Deployment](#file-templates--deployment)** — Multi-file generation
8. **[Builtin Functions](#builtin-functions)** — String, list, file, and JSON helpers
9. **[CLI Usage](#cli-usage)** — Running, deploying, and fetching from GitHub
10. **[Real-World Examples](#real-world-examples)** — Docker, Kubernetes, GitHub Actions, site generation
11. **[Best Practices](#best-practices)** — Tips for clean, maintainable Avon code
12. **[Troubleshooting](#troubleshooting)** — Common issues and solutions

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
\name @/greeting.txt {"
    Hello, {name}!
    Welcome to Avon.
"}
```

Deploy it:

```bash
avon deploy examples/greet.av -name Alice --root /tmp/output --force
```

This creates `/tmp/output/greeting.txt` with the content:
```
Hello, Alice!
Welcome to Avon.
```

**What happened?**
- `\name` defines a function parameter
- `@/greeting.txt` specifies the output file path
- `{"..."}` is a template that interpolates the `{name}` variable

### Generate Multiple Files

Here's where Avon shines. Let's generate a config file for each environment:

```avon
let environments = ["dev", "staging", "prod"] in
map (\env @/config-{env}.yml {"
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

---

## Core Concepts

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
| **FileTemplate** | `@/path/file {"content"}` | File generation targets |

When evaluation is complete, `avon` either:
1. **Prints the result** (for `eval` command)
2. **Materializes files** (for `deploy` command)

Error messages stay concise, noting which function/operator failed but omitting line numbers or file references—rely on the debugger helpers for additional context.

---

## Language Essentials

### Syntax Overview

Avon is a small, elegant language optimized for readability and powerful file generation. Here's the complete syntax you need to know:

#### Literals

```avon
"hello"                    # String (escape sequences: \n, \t, \\, \")
42                         # Integer
3.14                       # Float
true false                 # Booleans
[1, 2, 3]                  # List
{host: "localhost", port: 8080}  # Dictionary (key: value syntax)
```

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

```avon
a + b                      # Addition (numbers), concatenation (strings/lists)
a - b                      # Subtraction (numbers only)
a * b                      # Multiplication (numbers only)
a / b                      # Division (numbers only)
a == b                     # Equality
a != b                     # Inequality
a > b                      # Greater than
a < b                      # Less than
a >= b                     # Greater or equal
a <= b                     # Less or equal
```

**Operator overloading:** The `+` operator adapts to its operands:
- `"hello" + " world"` -> `"hello world"` (strings concatenate)
- `[1,2] + [3,4]` -> `[1,2,3,4]` (lists concatenate)
- `5 + 3` -> `8` (numbers add)
- `{"Hello "} + {"World!"}` -> `"Hello World!"` (templates concatenate)
- `@/home/user + @/projects` -> `/home/user//projects` (paths join with `/` separator)

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
let base = @/home in
let user = @/alice in
base + user                       # "/home//alice"

# Practical example
let env = "prod" in
let config_dir = @/etc/{env} in
let app_config = @/app.conf in
config_dir + app_config           # "/etc/prod//app.conf"
```

#### Conditionals

```avon
if condition then true_expr else false_expr
```

Example:
```avon
if age > 18 then "adult" else "minor"
```

---

---

## Functions & Variables

### Let Bindings

Use `let` to define variables and intermediate values:

```avon
let greeting = "Hello" in
let name = "Alice" in
greeting + ", " + name
# Evaluates to: "Hello, Alice"
```

You can cascade multiple `let` bindings:

```avon
let a = 10 in
let b = 20 in
let sum = a + b in
sum * 2
# Evaluates to: 60
```

**Important:** Always include the `in` keyword! `let` bindings require an `in` to specify the expression where the binding is visible.

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

### Default Parameters

When deploying, you can provide default values for parameters using `?`:

```avon
\name ? "Guest" @/welcome.txt {"
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
\app ? "service" \env ? "dev" @/config-{app}-{env}.yml {"
    app: {app}
    environment: {env}
"}
```

### Practical Example: Configuration Generator

```avon
let make_config = \env \debug ? false @/config-{env}.yml {"
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
@/names.txt {"
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

**Query dict operations:**

```avon
let config = {host: "localhost", port: 8080, debug: true} in

dict_keys config      # ["host", "port", "debug"]
dict_values config    # ["localhost", 8080, true]
dict_size config      # 3
has_key config "host" # true
has_key config "user" # false
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

@/service.yml {"
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
let make_config = \env @/config-{env}.yml {"
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
map (\e @/prod-{e}.yml {"prod config"}) prod_envs
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
@/names.txt {"{join_names}"}
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
@/config.yml {"
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
@/shopping.txt {"
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

```avon
"hello\n"                  # String: \n is escape sequence
{"hello\n"}               # Template: literally contains the text "\n" (not a newline)
```

To get a newline in a template, press Enter in the source:

```avon
{"
hello
"}
```

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
@/app.yml {"
app:
  name: myapp
  debug: { debug_mode }
"}
```

#### Double-Brace Templates

Use when your output has **many literal braces** (JSON, HCL, Terraform, Lua dicts, etc.):

```avon
@/config.lua {{"
local config = {
  name = "{{ app_name }}",
  debug = {{ if dev then "true" else "false" }}
}
"}}
```

**Rule:** In double-brace templates, single braces are literal (no escaping needed):

```avon
@/output.json {{"
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
@/config.lua {"
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
@/config.lua {{"
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
@/path/to/file.txt {"
    File content goes here
"}
```

This is a `FileTemplate` value. When you evaluate and deploy a program that returns this, Avon writes the file.

### Deploying Single Files

Create `greet.av`:

```avon
\name @/greeting.txt {"
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
    @/docker-compose.yml {"
        docker-compose: {name}
    "},
    @/README.md {"
        # {name}
    "},
    @/.gitignore {"
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
map (\env @/config-{env}.yml {"
    environment: {env}
"}) configs
```

This generates `config-dev.yml` and `config-prod.yml`.

### Important Deploy Flags

- `--root <dir>` — prepend this directory to all generated paths (recommended!)
- `--force` — overwrite existing files without warning
- `--append` — append to existing files instead of overwriting (useful for logs or accumulating data)
- `--if-not-exists` — only create file if it doesn't exist (useful for initialization files)
- **Default behavior**: If a file exists and none of the above flags are used, Avon will skip it and show a clear warning

---

## Builtin Functions

Avon comes with a toolkit of built-in functions for common tasks. All builtins are curried, so you can partially apply them.

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

| Function | Description | Example |
|----------|-------------|---------|
| `map f list` | Apply function to each item | `map (\x x + 1) [1,2,3]` → `[2,3,4]` |
| `filter pred list` | Keep items where predicate is true | `filter (\x x > 2) [1,2,3,4]` → `[3,4]` |
| `fold f init list` | Reduce list to single value | `fold (\a \x a + x) 0 [1,2,3]` → `6` |
| `join list sep` | Join items with separator | `join ["a","b","c"] ", "` → `"a, b, c"` |
| `length list` | Get number of items | `length [1,2,3]` → `3` |

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

### Example: String & List Combination

Generate a configuration file with multiple items:

```avon
let items = ["api", "worker", "scheduler"] in
let formatted = map (\item concat "service: " item) items in
@/services.conf {"
    Services:
    {formatted}
"}
```

---

## CLI Usage

### Basic Commands

```bash
# Evaluate a program and print result
avon eval examples/map_example.av

# Deploy: evaluate program and generate files
avon deploy examples/site_generator.av --root ./output --force

# Deploy with named arguments
avon deploy examples/greet.av -name Alice -age 30 --root ./gen --force
```

### Passing Arguments

Avon allows you to pass values into your program from the command line. This is essential for reusing templates across different environments or configurations.

#### 1. Named Arguments

If your main expression is a function with parameters, you can pass values using `-parameter_name value`.

**Program (`greet.av`):**
```avon
# Function with two parameters
\name \role @/greeting.txt {"
    Hello, {name}!
    Role: {role}
"}
```

**Command:**
```bash
avon deploy greet.av -name "Alice" -role "Admin"
```

**How it works:**
- The CLI looks for `-name` and passes "Alice" to the `name` parameter.
- It looks for `-role` and passes "Admin" to the `role` parameter.
- Arguments are type-checked at runtime (e.g. if your function uses the argument as a number, passing "hello" will cause an error).

#### 2. Positional Arguments

You can also pass arguments positionally, without the parameter names. This maps command line arguments to function parameters in order.

**Command:**
```bash
# Maps "Alice" to name, "Admin" to role
avon deploy greet.av "Alice" "Admin"
```

**Recommendation:** Use named arguments for clarity, especially when you have multiple parameters or default values.

#### 3. Default Values

Parameters can have default values using the `?` syntax.

**Program (`config.av`):**
```avon
\env ? "dev" \port ? 8080 @/config.yml {"
    env: {env}
    port: {port}
"}
```

**Usage:**
- **Use defaults:** `avon deploy config.av` (env="dev", port=8080)
- **Override some:** `avon deploy config.av -env prod` (env="prod", port=8080)
- **Override all:** `avon deploy config.av -env prod -port 9090` (env="prod", port=9090)

#### 4. Mixing Named and Positional

While possible, mixing named and positional arguments can be confusing. Avon prioritizes named arguments first, then fills remaining parameters with positional arguments in order.

**Best Practice:** Stick to either all named or all positional arguments for a single command invocation.

### Command-Line Flags

| Flag | Purpose | Example |
|------|---------|---------|
| `eval` | Evaluate program and print result | `avon eval program.av` |
| `deploy` | Generate files using result | `avon deploy program.av ...` |
| `run` | Evaluate code string directly | `avon run '1 + 1'` |
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

---

## Error handling and debugging

- Runtime errors produce `EvalError` with a clear message that names the failing function/operator and includes the source line number and context code to help you locate the issue.
- Lexing / parsing errors also report line numbers and context to help you fix syntax issues quickly.
- If deployment panics during file materialization, `avon` catches the panic and reports `Deployment panicked` rather than aborting your entire process. This protects you from half-written states. Use `--force` and test locally.

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
@/config.yml {"
    database:
      host: localhost
      port: 5432
      name: myapp
"}
```

### Return Lists for Multiple Files

When generating multiple files, return a list of file templates:

```avon
let make_file = \name @/{name}.txt {"{name}"} in
map make_file ["a", "b", "c"]
# Returns three file templates
```

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
@/f.txt {"name: {"}    # Tries to interpolate {, expects closing }

# Correct (use {{ to escape)
@/f.txt {"name: {{"}   # Outputs: name: {
```

**Problem:** I have lots of braces and it's getting confusing.

**Solution:** Use a double-brace template to reduce brace nesting:

```avon
# Single-brace (awkward with 3+ braces)
@/f.txt {"obj: {{{x}}}}"}   # Hard to count!

# Double-brace (clearer)
@/f.txt {{"obj: {{{x}}}}"}}  # Easier to read
```

**Problem:** Interpolation not working as expected.

**Solution:** Verify you're using the correct brace count:
- Single-brace template: use `{ expr }` for interpolation
- Double-brace template: use `{{ expr }}` for interpolation

```avon
# Single-brace template
@/f.txt {"Result: { 5 + 5 }"}     # Works
@/f.txt {"Result: {{ 5 + 5 }}"}   # No interpolation, just literals

# Double-brace template
@/f.txt {{"Result: { 5 + 5 }"}}   # No interpolation
@/f.txt {{"Result: {{ 5 + 5 }}"}} # Works
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
   avon run '@/t.txt {"{{ {{{{ }}}}"}' 
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

### Let bindings and cascading lets

The language uses `let <ident> = <expr> in <expr>` for local bindings. You can nest `let` bindings to build up intermediate values. Example:

```avon
let a = "A" in
let b = "B" in
let combined = concat a b in
@/out/{combined}.txt {"
    Combined: {combined}
"}
```

This demonstrates cascading `let` expressions: each `let` introduces a symbol visible in the following `in` expression. Always remember to include the `in` keyword.
