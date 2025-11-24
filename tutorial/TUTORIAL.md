# Avon — The Modern Template Language for Developers

Welcome to Avon! This is a comprehensive guide to using Avon, a lightweight yet powerful templating and file generation language designed for developers who build infrastructure, configure tools, and generate code at scale.

Avon makes it easy to:
- **Generate multi-file projects** from templates with data
- **Manage configuration** for Docker, Kubernetes, CI/CD pipelines
- **Create boilerplate** for any tool or language
- **Automate repetitive tasks** with a clean, functional syntax

This tutorial covers everything from basic syntax to advanced patterns. We assume you have the `avon` binary and access to examples in the `examples/` directory.

**Pro tip:** Throughout this guide, look at the `examples/` directory for real-world use cases. Each example demonstrates practical Avon patterns you can adapt for your own projects.

---

## Table of Contents

1. **[Quick Start](#quick-start)** — Get up and running in 60 seconds
2. **[Core Concepts](#core-concepts)** — Values, types, and the Avon model
3. **[Language Essentials](#language-essentials)** — Syntax, expressions, and operators
4. **[Functions & Variables](#functions--variables)** — Defining and using functions, let bindings
5. **[Collections](#collections)** — Lists and the powerful `map`, `filter`, `fold` operations
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
avon examples/greet.av --deploy -name Alice --root /tmp/output --force
```

This creates `/tmp/output/greeting.txt` with the content:
```
Hello, Alice!
Welcome to Avon.
```

**What happened?**
- `\name` defines a function parameter
- `@/greeting.txt` specifies the output file path
- `{...}` is a template that interpolates the `{name}` variable

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
avon examples/gen_configs.av --deploy --root ./configs --force
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
| **Function** | `\x x + 1` | Reusable logic and transformations |
| **Template** | `{"Hello {name}"}` | Text generation with interpolation |
| **FileTemplate** | `@/path/file {"content"}` | File generation targets |

When evaluation is complete, `avon` either:
1. **Prints the result** (for `eval` command)
2. **Materializes files** (for `--deploy` command)

Error messages include helpful line numbers and context to guide you.

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
```

**Strings support escape sequences:** `"\n"` is a newline, `"\t"` is a tab, `"\\"` is a backslash, `"\""` is a quote.

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
- `"hello" + " world"` → `"hello world"` (strings concatenate)
- `[1,2] + [3,4]` → `[1,2,3,4]` (lists concatenate)
- `5 + 3` → `8` (numbers add)

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

When you deploy this without a `--deploy -name` argument, `name` defaults to `"Guest"`.

```bash
avon examples/greet.av --deploy
# Uses default: "Guest"

avon examples/greet.av --deploy -name Alice
# Uses provided value: "Alice"
```

### Named Deploy Arguments

When using the `--deploy` command, you can pass named arguments with `-param value`:

```bash
avon examples/config.av --deploy -env prod -debug false
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
} in

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
| `length list` | Get list length |

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
let name = "Alice" in
{"
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

Templates automatically strip common leading whitespace (dedent), so you can indent them nicely in your source:

```avon
let item = "widget" in
@/config.yml {"
	item: {item}
	description: "A useful widget"
	version: 1.0
"}
```

The leading tabs are removed, producing clean output:
```
item: widget
description: "A useful widget"
version: 1.0
```

This is crucial for readability in your Avon code!

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

### Template Escape Hatch: Literal Braces

When generating code (Lua, HCL, Nginx, etc.), you often need literal `{` and `}` characters. Avon's escape hatch lets you output literal braces without interpolation.

#### Single-Brace Templates

The default template syntax uses single braces for interpolation:

```avon
{"Value: { 1 + 2 }"}       # Output: Value: 3
{"Literal open: {{"}       # Output: Literal open: {
{"Literal close: }}"}      # Output: Literal close: }
```

**Rule:** Inside a single-brace template, a run of k consecutive braces outputs (k - 1) literal braces:
- `{{` → `{` (one literal)
- `{{{{` → `{{{` (three literals)
- `}}` → `}` (one literal)
- `}}}}` → `}}}` (three literals)

#### Example: Generating Lua Code

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

Output:
```lua
local config = {
  name = "myapp",
  debug = true
}

function init()
  return config
end
```

#### Double-Brace Templates

For cases requiring many literal braces, use double-brace templates:

```avon
@/output.txt {{"
Two-brace template: {{ 10 + 20 }}
Literal open: {{{
Literal pair: {{{{
"}}
```

**Rule:** Inside a double-brace template (`{{" ... "}}`), interpolation requires exactly two braces, and literal braces work by (k - 2):
- `{{{` → `{` (one literal, 3 - 2)
- `{{{{` → `{{` (two literals, 4 - 2)
- `}}}` → `}` (one literal)
- `}}}}` → `}}` (two literals)

#### Why Multiple Brace Levels?

The escape hatch is crucial for generating:
- **Lua configs:** Tables use `{ ... }`
- **Nginx configs:** Blocks use `{ ... }`
- **Terraform:** Maps use `{ ... }`
- **HCL:** Variable references use `${ ... }`
- **JSON:** Objects use `{ ... }`

Without the escape hatch, every `{` would trigger interpolation. With it, you can generate any brace-heavy syntax cleanly.

See `examples/escape_hatch.av` for a comprehensive demo of both single and double-brace templates.

### Complex Interpolations

You can embed any expression in a template:

```avon
let x = 10 in
let y = 20 in
{"
Sum: {x + y}
Max: {if x > y then x else y}
Count: {length [1,2,3]}
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
avon examples/greet.av --deploy -name Alice --root ./out --force
```

Creates: `./out/greeting.txt`

### Deploying Multiple Files

Return a list of file templates:

```avon
let name = "my-app" in
[
	@/docker-compose.yml {" docker-compose: {name} "},
	@/README.md {"# {name}"},
	@/.gitignore {"__pycache__/\nnode_modules/"}
]
```

Deploy it:

```bash
avon examples/gen_files.av --deploy --root ./project --force
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
- `--force` — allow overwriting existing files; without this, Avon refuses to overwrite

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
| `length s` | `length "hello"` | `5` |

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
avon examples/site_generator.av --deploy --root ./output --force

# Deploy with named arguments
avon examples/greet.av --deploy -name Alice -age 30 --root ./gen --force
```

### Command-Line Flags

| Flag | Purpose | Example |
|------|---------|---------|
| `eval` | Evaluate program and print result | `avon eval program.av` |
| `--deploy` | Generate files using result | `avon program.av --deploy ...` |
| `-param value` | Named argument for deploy | `--deploy -name alice` |
| `--root <dir>` | Prepend directory to all generated paths | `--root ./output` |
| `--force` | Allow overwriting existing files | `--force` |
| `--git owner/repo/path` | Fetch program from GitHub raw URL | `--git user/repo/examples/gen.av` |

### Real-World Examples

Generate site with custom name:

```bash
avon examples/site_generator.av --deploy -name "My Site" --root ./website --force
```

Generate config files for all environments:

```bash
avon examples/config_gen.av --deploy --root ./configs --force
```

Fetch and run a program from GitHub:

```bash
avon --git owner/avon-scripts/generators/site.av --deploy -title "Hello" --root ./site
```

---

## Error handling and debugging

- Runtime errors produce `EvalError` with message, and `pretty` printing attempts to show the source line and a caret indicating the approximate location. If you see an error, look at the program line printed and surrounding source for the likely cause.
- Lexing / parsing errors will also include best-effort line numbers.
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
avon examples/gen_config.av --deploy --root ./out --force
```

### Use Named Arguments

For functions with multiple parameters, use named arguments for clarity:

```bash
# Good: clear what each argument means
avon program.av --deploy -app myservice -env prod -version 1.0

# Less clear: position-dependent
avon program.av --deploy myservice prod 1.0
```

### Always Use `--root`

Avoid accidentally writing to system directories:

```bash
# Good: files go to ./generated/
avon program.av --deploy --root ./generated --force

# Risky: files go to absolute paths
avon program.av --deploy --force
```

### Keep Templates Readable

Indent templates nicely in your source code. Avon's dedent feature handles the indentation:

```avon
let config = {"
	database:
	  host: localhost
	  port: 5432
	  name: myapp
"} in
@/config.yml {config}
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
avon examples/site_generator.av --deploy --root ./website --force
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
# ❌ Wrong (in a single-brace template, { starts interpolation)
@/f.txt {"name: {"}    # Tries to interpolate {, expects closing }

# ✅ Correct (use {{ to escape)
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
@/f.txt {"Result: { 5 + 5 }"}     # ✅ Works
@/f.txt {"Result: {{ 5 + 5 }}"}   # ❌ No interpolation, just literals

# Double-brace template
@/f.txt {{"Result: { 5 + 5 }"}}   # ❌ No interpolation
@/f.txt {{"Result: {{ 5 + 5 }}"}} # ✅ Works
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
   avon -c 'let x = [1,2,3] in map (\n n * 2) x' eval
   ```

3. **Check file generation:** Before using `--deploy`, check if files will be generated where you expect:
   ```bash
   avon program.av eval  # Shows what will be generated
   ```

4. **Isolate escape hatch issues:** Test brace escaping independently:
   ```bash
   avon -c '@/t.txt {"{{ {{{{ }}}}"}' eval
   # Outputs: { {{
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

Happy generating! 🚀

### Let bindings and cascading lets

The language uses `let <ident> = <expr> in <expr>` for local bindings. You can nest `let` bindings to build up intermediate values. Example:

```
let a = "A" in
let b = "B" in
let combined = concat a b in
@/out/{combined}.txt {"
Combined: {combined}
"}
```

This demonstrates cascading `let` expressions: each `let` introduces a symbol visible in the following `in` expression. Always remember to include the `in` keyword — examples and previous docs that omitted it were incorrect and have been fixed.
