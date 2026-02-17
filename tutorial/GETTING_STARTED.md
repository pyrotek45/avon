# Getting Started with Avon

A hands-on guide to learning Avon step by step. Each lesson builds on the previous one, with examples you can run immediately.

> **Prerequisites:** Install Avon and make sure `avon` is in your PATH. Test with `avon version`.

---

## Table of Contents

1. [Lesson 1: Hello, Avon!](#lesson-1-hello-avon) — Your first expressions
2. [Lesson 2: Variables with Let](#lesson-2-variables-with-let) — Naming values
3. [Lesson 3: Making Decisions](#lesson-3-making-decisions) — Conditionals
4. [Lesson 4: Functions](#lesson-4-functions) — Reusable logic
5. [Lesson 5: Lists](#lesson-5-lists) — Working with collections
6. [Lesson 6: Transform, Filter, Reduce](#lesson-6-transform-filter-reduce) — map, filter, fold
7. [Lesson 7: Pipes](#lesson-7-pipes) — Readable data flow
8. [Lesson 8: Dictionaries](#lesson-8-dictionaries) — Key-value data
9. [Lesson 9: Templates](#lesson-9-templates) — Text with interpolation
10. [Lesson 10: Generating Files](#lesson-10-generating-files) — FileTemplates
11. [Lesson 11: Multiple Files](#lesson-11-multiple-files) — Generating many files at once
12. [Lesson 12: CLI Arguments](#lesson-12-cli-arguments) — Reusable programs
13. [Lesson 13: Working with JSON](#lesson-13-working-with-json) — Double-brace templates
14. [Lesson 14: Importing & Sharing](#lesson-14-importing--sharing) — Reuse code from files and GitHub
15. [Lesson 15: Built-in Functions](#lesson-15-built-in-functions) — Exploring the standard library
16. [Lesson 16: Putting It All Together](#lesson-16-putting-it-all-together) — A real-world project
17. [Where to Go Next](#where-to-go-next)

---

## Lesson 1: Hello, Avon!

Avon is an expression-based language. Everything evaluates to a value. Let's start with the simplest expressions.

### Running Avon

The quickest way to try Avon is the `run` command, which evaluates a code string directly:

```bash
avon run '"Hello, world!"'
# Output: Hello, world!
```

You can also do math:

```bash
avon run '2 + 3'
# Output: 5

avon run '10 - 4'
# Output: 6

avon run '3 * 7'
# Output: 21

avon run '10 / 3'
# Output: 3.3333333333333335

avon run '10 // 3'
# Output: 3

avon run '10 % 3'
# Output: 1

avon run '2 ** 8'
# Output: 256
```

Quick reference:
- `/` — division (always returns a float)
- `//` — integer division (rounds down)
- `%` — remainder
- `**` — power

### Booleans and Comparisons

```bash
avon run '5 > 3'
# Output: true

avon run '5 == 5'
# Output: true

avon run 'true && false'
# Output: false

avon run 'true || false'
# Output: true

avon run 'not true'
# Output: false
```

### String Concatenation

Use `+` to join strings together:

```bash
avon run '"hello" + " world"'
# Output: hello world
```

### Comments

Lines starting with `#` are comments and are ignored:

```bash
avon run '# this is a comment
42'
# Output: 42
```

### Try It Yourself

Open the Avon REPL for interactive exploration:

```bash
avon repl
```

```
avon> 2 + 3
5 : Number

avon> "hello" + " world"
hello world : String

avon> 100 / 7
14.285714285714286 : Number

avon> :exit
```

The REPL shows the type of each result after the `:`, which is helpful for learning.

---

## Lesson 2: Variables with Let

Use `let ... in` to name values. The variable exists only within the `in` expression.

```bash
avon run 'let x = 10 in x * 2'
# Output: 20
```

### Cascading Lets

Chain multiple `let` bindings:

```bash
avon run 'let name = "Alice" in let greeting = "Hello" in greeting + ", " + name + "!"'
# Output: Hello, Alice!
```

In a file, this is more readable with each binding on its own line:

```avon
# greeting.av
let name = "Alice" in
let greeting = "Hello" in
greeting + ", " + name + "!"
```

```bash
avon eval greeting.av
# Output: Hello, Alice!
```

### Key Rule: No Shadowing

You cannot reuse a variable name in the same scope:

```avon
# ✗ This will error
let x = 5 in
let x = 10 in  # Error: x already exists
x
```

```avon
# ✓ Use different names
let x = 5 in
let y = 10 in
x + y
```

### Try It Yourself

```bash
avon run 'let width = 10 in let height = 5 in width * height'
# Output: 50
```

---

## Lesson 3: Making Decisions

Use `if ... then ... else` for conditionals. Both branches are always required.

```bash
avon run 'if true then "yes" else "no"'
# Output: yes

avon run 'if 5 > 3 then "bigger" else "smaller"'
# Output: bigger
```

### With Variables

```bash
avon run 'let age = 20 in if age >= 18 then "adult" else "minor"'
# Output: adult
```

### Conditionals Are Expressions

Since `if` is an expression, you can use it anywhere a value is expected:

```bash
avon run 'let env = "prod" in "debug: " + (if env == "prod" then "false" else "true")'
# Output: debug: false
```

### Try It Yourself

```bash
avon run 'let score = 85 in if score >= 90 then "A" else if score >= 80 then "B" else "C"'
# Output: B
```

---

## Lesson 4: Functions

Functions are defined with `\param body` (backslash followed by parameter name). They are values, just like numbers and strings.

### Single Parameter

```bash
avon run 'let double = \x x * 2 in double 5'
# Output: 10
```

### Multiple Parameters (Currying)

Each `\param` takes one argument. For multiple parameters, chain them:

```bash
avon run 'let add = \a \b a + b in add 3 7'
# Output: 10
```

### Functions as Arguments

Functions can be passed to other functions:

```bash
avon run 'let apply = \f \x f x in let double = \x x * 2 in apply double 5'
# Output: 10
```

### Default Parameters

Use `?` to give a parameter a default value:

```bash
avon run 'let greet = \name ? "World" {"Hello, {name}!"} in greet'
# Output: Hello, World!

avon run 'let greet = \name ? "World" {"Hello, {name}!"} in greet "Alice"'
# Output: Hello, Alice!
```

### Built-in Functions as Values

Built-in functions like `upper` can be passed around just like your own functions:

```bash
avon run 'map upper ["hello", "world"]'
# Output: [HELLO, WORLD]
```

### Try It Yourself

```bash
avon run 'let square = \x x * x in square 9'
# Output: 81

avon run 'let make_greeting = \greeting \name greeting + ", " + name + "!" in make_greeting "Hi" "Bob"'
# Output: Hi, Bob!
```

---

## Lesson 5: Lists

Lists are ordered collections. Create them with square brackets:

```bash
avon run '[1, 2, 3, 4, 5]'
# Output: [1, 2, 3, 4, 5]
```

### Ranges

Generate sequences with range syntax:

```bash
avon run '[1..5]'
# Output: [1, 2, 3, 4, 5]

avon run '[0, 2..10]'
# Output: [0, 2, 4, 6, 8, 10]
```

### List Operations

```bash
avon run 'length [10, 20, 30]'
# Output: 3

avon run 'head [10, 20, 30]'
# Output: 10

avon run 'tail [10, 20, 30]'
# Output: [20, 30]

avon run 'last [10, 20, 30]'
# Output: 30

avon run 'reverse [1, 2, 3]'
# Output: [3, 2, 1]
```

### Combining Lists

```bash
avon run '[1, 2] + [3, 4]'
# Output: [1, 2, 3, 4]
```

### More List Functions

```bash
avon run 'sort [3, 1, 4, 1, 5, 9, 2, 6]'
# Output: [1, 1, 2, 3, 4, 5, 6, 9]

avon run 'unique [1, 2, 2, 3, 3, 3]'
# Output: [1, 2, 3]

avon run 'take 3 [1, 2, 3, 4, 5]'
# Output: [1, 2, 3]

avon run 'drop 2 [1, 2, 3, 4, 5]'
# Output: [3, 4, 5]

avon run 'flatten [[1, 2], [3, 4], [5]]'
# Output: [1, 2, 3, 4, 5]

avon run 'zip [1, 2, 3] ["a", "b", "c"]'
# Output: [[1, a], [2, b], [3, c]]
```

### Joining Lists into Strings

```bash
avon run 'join ["apple", "banana", "cherry"] ", "'
# Output: apple, banana, cherry
```

### Aggregate Functions

```bash
avon run 'sum [1, 2, 3, 4, 5]'
# Output: 15

avon run 'min [3, 1, 4, 1, 5]'
# Output: 1

avon run 'max [3, 1, 4, 1, 5]'
# Output: 5
```

### Try It Yourself

```bash
avon run 'let nums = [1..10] in sum nums'
# Output: 55

avon run 'join (reverse ["c", "b", "a"]) "-"'
# Output: a-b-c
```

---

## Lesson 6: Transform, Filter, Reduce

These three operations are the heart of working with lists in Avon.

### Map — Transform Every Item

`map` applies a function to each item and returns a new list:

```bash
avon run 'map (\x x * 2) [1, 2, 3, 4, 5]'
# Output: [2, 4, 6, 8, 10]

avon run 'map upper ["hello", "world"]'
# Output: [HELLO, WORLD]
```

### Filter — Keep What Matches

`filter` keeps only items where the predicate returns `true`:

```bash
avon run 'filter (\x x > 3) [1, 2, 3, 4, 5]'
# Output: [4, 5]
```

### Fold — Reduce to One Value

`fold` combines all items into a single value using an accumulator:

```bash
avon run 'fold (\acc \x acc + x) 0 [1, 2, 3, 4, 5]'
# Output: 15
```

Here's how it works step by step:
- Start with `acc = 0`
- `acc = 0 + 1 = 1`
- `acc = 1 + 2 = 3`
- `acc = 3 + 3 = 6`
- `acc = 6 + 4 = 10`
- `acc = 10 + 5 = 15`

### Combining Them

```bash
avon run '[1, 2, 3, 4, 5] -> filter (\x x > 2) -> map (\x x * 10)'
# Output: [30, 40, 50]
```

This takes `[1, 2, 3, 4, 5]`, keeps items greater than 2 (`[3, 4, 5]`), then multiplies each by 10.

### Try It Yourself

```bash
# Get the sum of squares of even numbers from 1 to 10
avon run '[1..10] -> filter (\x x % 2 == 0) -> map (\x x * x) -> sum'
# Output: 220
```

---

## Lesson 7: Pipes

The pipe operator `->` passes the result of the left side as the **last argument** to the function on the right. It turns deeply nested calls into readable left-to-right chains.

### Without Pipes (Nested)

```bash
avon run 'length (filter (\x x > 2) [1, 2, 3, 4, 5])'
# Output: 3
```

### With Pipes (Linear)

```bash
avon run '[1, 2, 3, 4, 5] -> filter (\x x > 2) -> length'
# Output: 3
```

Both produce the same result, but pipes read naturally: "take this list, filter it, get the length."

### Pipes with Custom Functions

```bash
avon run 'let double = \x x * 2 in let add_one = \x x + 1 in 5 -> double -> add_one'
# Output: 11
```

This evaluates as: `5` → `double` (gives `10`) → `add_one` (gives `11`).

### Multi-Step Data Pipelines

Pipes really shine with multi-step transformations:

```bash
avon run '["  HELLO  ", "  world  ", "  AVON  "] -> map trim -> map lower'
# Output: [hello, world, avon]
```

> **Tip:** Some functions like `join` take the list as the *first* argument, so they
> don't work at the end of a pipe (since `->` feeds into the *last* argument).
> Use a `let` binding instead:
>
> ```bash
> avon run 'let result = ["a", "b", "c"] -> reverse in join result ", "'
> # Output: c, b, a
> ```

### Try It Yourself

```bash
avon run '[1..20] -> filter (\x x % 3 == 0) -> map (\x x * x) -> sum'
# Output: 819
```

---

## Lesson 8: Dictionaries

Dictionaries are key-value maps. Create them with curly braces:

```bash
avon run 'let config = {host:"localhost", port:8080} in config.host'
# Output: localhost
```

> **Note:** When using dicts on the command line, don't put spaces after colons (bash interprets `{a: 1}` as brace expansion). In `.av` files, spaces are fine.

### Accessing Values

Use dot notation:

```bash
avon run 'let config = {host:"localhost", port:8080} in config.port'
# Output: 8080
```

Or the `get` function (useful when the key is a variable):

```bash
avon run 'let config = {host:"localhost", port:8080} in get config "host"'
# Output: localhost
```

### Dict Operations

```bash
avon run 'let config = {host:"localhost", port:8080} in keys config'
# Output: [port, host]

avon run 'let config = {host:"localhost", port:8080} in has_key config "host"'
# Output: true

avon run 'let config = {host:"localhost", port:8080} in set config "debug" true'
# Output: {port: 8080, host: "localhost", debug: true}
```

### Merging Dictionaries

`dict_merge` combines two dicts. The second dict's values win on conflicts:

```bash
avon run 'dict_merge {a:1, b:2} {b:99, c:3}'
# Output: {b: 99, a: 1, c: 3}
```

### Handling Missing Keys with `default`

When a key doesn't exist, `get` returns `None`. Use `default` to provide a fallback:

```bash
avon run 'let config = {host:"localhost", port:8080} in get config "timeout" -> default 30'
# Output: 30
```

### Try It Yourself

```bash
avon run 'let person = {name:"Alice", age:30} in {"Name: {person.name}, Age: {person.age}"}'
# Output: Name: Alice, Age: 30
```

---

## Lesson 9: Templates

Templates are one of Avon's most powerful features. They look like `{"..."}` and support **interpolation** — embedding expressions inside `{...}`.

### Strings vs Templates

| | Strings `"..."` | Templates `{"..."}` |
|---|---|---|
| Interpolation | ✗ No — braces are literal | ✓ Yes — `{expr}` evaluates |
| Escape sequences | ✓ `\n`, `\t` work | ✗ Literal text only |

```bash
# String: braces are literal
avon run '"Hello, {name}!"'
# Output: Hello, {name}!

# Template: braces interpolate
avon run 'let name = "Alice" in {"Hello, {name}!"}'
# Output: Hello, Alice!
```

### Expressions in Templates

You can put any expression inside `{...}`:

```bash
avon run 'let x = 10 in {"x is {x} and double is {x * 2}"}'
# Output: x is 10 and double is 20
```

### Lists in Templates

When you interpolate a list, each item goes on its own line:

```bash
avon run 'let items = ["apple", "banana", "cherry"] in {"Items:\n{items}"}'
# Output:
# Items:
# apple
# banana
# cherry
```

Use `join` to put items on one line:

```bash
avon run 'let items = ["apple", "banana", "cherry"] in {"Items: {join items ", "}"}'
# Output: Items: apple, banana, cherry
```

### Try It Yourself

```bash
avon run 'let lang = "Avon" in let year = 2024 in {"{lang} was created in {year}. It has {194} built-in functions."}'
# Output: Avon was created in 2024. It has 194 built-in functions.
```

---

## Lesson 10: Generating Files

This is where Avon really shines. Use `@path {"content"}` to create a **FileTemplate** — a declaration that says "this content belongs at this path."

### Your First File

Create a file called `hello.av`:

```avon
# hello.av
@hello.txt {"Hello, world!"}
```

Preview it with `eval` (does NOT write anything):

```bash
avon eval hello.av
# Output:
# --- hello.txt ---
# Hello, world!
```

Deploy it with `deploy` (actually writes the file):

```bash
avon deploy hello.av --root ./output --force
# Output: Wrote ./output/hello.txt
```

### Multi-Line Templates with Dedent

Avon automatically removes leading whitespace (dedent), so you can indent templates in your source code without affecting the output:

```avon
# greeting.av
let name = "Alice" in
let role = "Developer" in
@greeting.txt {"
    Hello, {name}!
    Your role is: {role}
    Welcome to the team.
"}
```

```bash
avon eval greeting.av
# Output:
# --- greeting.txt ---
# Hello, Alice!
# Your role is: Developer
# Welcome to the team.
```

The 4-space indent is stripped automatically. This keeps your source code readable.

### Key Concepts

- `eval` — preview what would be generated (read-only, safe)
- `deploy` — actually write files to disk
- `--root ./dir` — confine all output to a directory (always use this!)
- `--force` — overwrite existing files

### Try It Yourself

Create a file `readme.av`:

```avon
let project = "my-app" in
let version = "1.0.0" in
@README.md {"
    # {project}

    Version: {version}

    ## Getting Started

    Run `{project} --help` for usage information.
"}
```

```bash
avon eval readme.av
```

---

## Lesson 11: Multiple Files

Return a list of FileTemplates to generate multiple files at once.

### Using Map

```avon
# multi.av
let environments = ["dev", "staging", "prod"] in
let make_config = \env
    @config-{env}.yml {"
        environment: {env}
        debug: {if env == "prod" then "false" else "true"}
        port: {if env == "prod" then "443" else "8080"}
    "}
in
map make_config environments
```

```bash
avon eval multi.av
# Output:
# --- config-dev.yml ---
# environment: dev
# debug: true
# port: 8080
# --- config-staging.yml ---
# environment: staging
# debug: true
# port: 8080
# --- config-prod.yml ---
# environment: prod
# debug: false
# port: 443
```

One command generates three files, each customized by environment.

### Dynamic Paths

The file path in `@path` supports interpolation too — `@config-{env}.yml` creates different filenames based on the variable.

### Deploying Multiple Files

```bash
avon deploy multi.av --root ./configs --force
# Output:
# Wrote ./configs/config-dev.yml
# Wrote ./configs/config-staging.yml
# Wrote ./configs/config-prod.yml
```

### Try It Yourself

```avon
# users.av
let users = ["alice", "bob", "charlie"] in
map (\user @users/{user}.txt {"
    Username: {user}
    Home: /home/{user}
"}) users
```

---

## Lesson 12: CLI Arguments

Make your Avon programs reusable by accepting arguments from the command line.

### How It Works

When a file evaluates to a **function**, Avon passes CLI arguments to it:

```avon
# greet.av
\name @greeting.txt {"
    Hello, {name}!
    Welcome to Avon.
"}
```

```bash
avon eval greet.av -name "Bob"
# Output:
# --- greeting.txt ---
# Hello, Bob!
# Welcome to Avon.
```

### Named vs Positional Arguments

```bash
# Named (recommended)
avon eval greet.av -name "Bob"

# Positional
avon eval greet.av "Bob"
```

### Default Values

Use `?` so arguments are optional:

```avon
# config.av
\env ? "dev" \port ? "8080" @config.yml {"
    environment: {env}
    port: {port}
"}
```

```bash
# Use all defaults
avon eval config.av

# Override some
avon eval config.av -env prod

# Override all
avon eval config.av -env prod -port 443
```

### Important: Arguments Are Strings

All CLI arguments arrive as strings. Convert them if needed:

```avon
# math.av
\x \y to_int x + to_int y
```

```bash
avon eval math.av -x 5 -y 3
# Output: 8
```

### Single-Dash vs Double-Dash

- `-name`, `-env`, `-port` → function parameters (your arguments)
- `--root`, `--force`, `--backup` → CLI flags (Avon's options)

No conflicts — they use different prefix conventions.

### Try It Yourself

```avon
# service.av
\name \replicas ? "3" @service-{name}.yml {"
    service: {name}
    replicas: {replicas}
"}
```

```bash
avon eval service.av -name webapp
avon eval service.av -name api -replicas 5
```

---

## Lesson 13: Working with JSON

JSON has literal braces `{` and `}`, which conflict with single-brace templates. Use **double-brace templates** `{{" "}}` to solve this.

### The Rule

| Template | Literal Braces | Interpolation |
|----------|----------------|---------------|
| `{" "}` | None | `{expr}` |
| `{{" "}}` | `{` and `}` | `{{expr}}` |
| `{{{" "}}}` | `{` and `{{` | `{{{expr}}}` |

### Example: Generating JSON

```avon
# config.av
let app_name = "my-app" in
let port = 8080 in
@config.json {{"
    {
      "name": "{{app_name}}",
      "port": {{port}},
      "debug": false
    }
"}}
```

```bash
avon eval config.av
# Output:
# --- config.json ---
# {
#   "name": "my-app",
#   "port": 8080,
#   "debug": false
# }
```

Inside `{{" "}}`:
- Single braces `{` and `}` are **literal** (perfect for JSON)
- Double braces `{{expr}}` are **interpolation**

### When to Use Which

| Output Format | Template | Why |
|---------------|----------|-----|
| YAML, plain text, Markdown | `{" "}` | No braces needed |
| JSON, Terraform, Nginx, CSS | `{{" "}}` | Single braces are literal |
| GitHub Actions, Mustache | `{{{" "}}}` | Double braces are literal |

### Try It Yourself

```avon
# package.av
\name \version ? "1.0.0"
@package.json {{"
    {
      "name": "{{name}}",
      "version": "{{version}}",
      "scripts": {
        "start": "node index.js"
      }
    }
"}}
```

```bash
avon eval package.av -name my-project
```

---

## Lesson 14: Importing & Sharing

Avon has a built-in module system. You can import code from local files or directly from GitHub — no package manager needed.

### Local Imports

Use `import` to load another Avon file. The file is evaluated and its result is returned:

```avon
# utils.av — a small library
{
    double: \x x * 2,
    greet: \name {"Hello, {name}!"},
    shout: \msg upper msg
}
```

```avon
# main.av — use the library
let utils = import "utils.av" in
utils.double 21
```

```bash
avon run 'let utils = import "utils.av" in utils.double 21'
# Output: 42

avon run 'let utils = import "utils.av" in utils.greet "Alice"'
# Output: Hello, Alice!

avon run 'let utils = import "utils.av" in utils.shout "wow"'
# Output: WOW
```

The pattern is simple: make a file that evaluates to a **dict of functions**, then import it and use dot notation to call them. This is how you build reusable libraries in Avon.

### Importing from GitHub

This is where it gets powerful. `import_git` lets you pull Avon code directly from any public GitHub repository:

```avon
let config = import_git
    "owner/repo/path/to/file.av"
    "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
in
config
```

It takes two arguments:
1. **Repository path** — `"owner/repo/path/to/file.av"`
2. **Commit hash** — a full 40-character SHA-1 commit hash

### Why a Commit Hash?

Avon deliberately requires a specific commit hash (not a branch name) for three important reasons:

- **Reproducibility** — You always get the exact same code
- **Security** — You control exactly what runs on your machine
- **No surprises** — Upstream changes won't silently break your project

### Real-World Example

The Avon project itself hosts shareable configs. Here's importing a Helix editor config from GitHub:

```avon
# Pull a helix editor config from GitHub
let helix_config = import_git
    "pyrotek45/config/helix.av"
    "f75d99c0b6803495a86bb0e4ec0ef014a5c57263"
in
helix_config
```

```bash
avon eval helix_config.av
# Output:
# --- .config/helix/config.toml ---
# theme = "dark_plus"
#
# [editor.cursor-shape]
# insert = "bar"
# normal = "block"
# select = "underline"
# ...
```

The imported file returns a FileTemplate, so you can preview it with `eval` and write it with `deploy` — just like any local Avon file.

### How to Get a Commit Hash

1. On GitHub, navigate to the file you want
2. Click **History** to see the commit list
3. Click the commit and copy the full 40-character hash from the URL or page
4. Or use git from the command line:

```bash
git log --format="%H" -n 1
```

### Sharing Your Own Templates

To share Avon files with others, just push them to a public GitHub repo. Anyone can then import them:

```avon
# Someone else can import your templates:
let my_template = import_git
    "your-username/your-repo/templates/setup.av"
    "abc123...full40chars..."
in
my_template
```

There's no registry, no publishing step — just push a `.av` file and share the repo path and commit hash.

### Use Cases

| Pattern | Description |
|---------|-------------|
| **Shared configs** | Team shares editor configs, linter settings, or dotfiles |
| **Template libraries** | Reusable templates for Dockerfiles, CI pipelines, K8s manifests |
| **Utility functions** | Common string/list/math helpers shared across projects |
| **Starter kits** | Import a project scaffold and deploy it locally |

### Deploying Directly from GitHub

You can skip the `import_git` step entirely and deploy files directly from GitHub with a single command using the `--git` flag:

```bash
# Evaluate (preview) a file from GitHub
avon eval --git "owner/repo/path/to/file.av"

# Deploy directly to disk
avon deploy --git "owner/repo/path/to/file.av" --root ./output --force
```

**Why is this powerful?**

- **One-liner setup** — No need to clone the repo or write an `.av` wrapper
- **Instant deployment** — Pull someone's template and deploy it immediately
- **Reproducible infrastructure** — Pin a commit hash and redeploy the exact same config anytime
- **Onboarding** — New team members can deploy a shared setup with a single command

**Example: Deploy a Helix config from GitHub**

```bash
# Just evaluate it first to see what it does
avon eval --git "pyrotek45/config/helix.av"

# Then deploy it to your home directory
avon deploy --git "pyrotek45/config/helix.av" --root ~/.config --force
```

This creates `~/.config/.config/helix/config.toml` with the Helix editor settings from GitHub.

**With arguments (if the template accepts parameters):**

```bash
avon deploy --git "owner/repo/template.av" --root ./output -theme dark -debug true
```

The `--git` flag also works with `--backup`, `--append`, and `--if-not-exists` for safety.

### Try It Yourself

Try importing the real Helix config from GitHub:

```avon
# try_import.av
let config = import_git
    "pyrotek45/config/helix.av"
    "f75d99c0b6803495a86bb0e4ec0ef014a5c57263"
in
config
```

```bash
avon eval try_import.av
# Pulls the config from GitHub and shows you the generated file
```

Or deploy it directly in one command:

```bash
avon deploy --git "pyrotek45/config/helix.av" --root ./my-helix-config --force
```

---

## Lesson 15: Built-in Functions

Avon has 194 built-in functions. Here are the most commonly used ones.

### Discovering Functions

Use `avon doc` to explore:

```bash
# Browse all functions
avon doc

# Look up a specific function
avon doc map
avon doc filter
avon doc join

# Browse by category
avon doc string
avon doc list
avon doc math
```

### String Functions

```bash
avon run 'upper "hello"'
# Output: HELLO

avon run 'lower "HELLO"'
# Output: hello

avon run 'trim "  hello  "'
# Output: hello

avon run 'split "a,b,c" ","'
# Output: [a, b, c]

avon run 'replace "hello world" "world" "Avon"'
# Output: hello Avon

avon run 'contains "hello world" "world"'
# Output: true

avon run 'length "hello"'
# Output: 5
```

### Type Functions

```bash
avon run 'typeof 42'
# Output: Number

avon run 'typeof "hello"'
# Output: String

avon run 'typeof [1, 2]'
# Output: List

avon run 'typeof true'
# Output: Bool

avon run 'to_string 42'
# Output: 42

avon run 'to_int "99"'
# Output: 99
```

### Environment Variables

Read environment variables safely:

```bash
avon run 'env_var_or "HOME" "/unknown"'
# Output: /home/yourusername  (or your actual home path)

avon run 'env_var_or "NONEXISTENT" "fallback"'
# Output: fallback
```

### The `default` Function

`default` provides a fallback when a value is `None`:

```bash
avon run 'head [] -> default "empty"'
# Output: empty

avon run 'head [1, 2, 3] -> default "empty"'
# Output: 1
```

### Try It Yourself

```bash
# Explore a category
avon doc dict

# Try a function you find interesting
avon run 'repeat "ha" 3'
avon run 'format_bytes 1048576'
```

---

## Lesson 16: Putting It All Together

Let's build a real project: a **multi-environment service configuration generator**.

Create a file called `deploy.av`:

```avon
# deploy.av — Generate service configs for multiple environments
\service ? "api" \port ? "8080"

let environments = ["dev", "staging", "prod"] in

let make_config = \env
    let is_prod = env == "prod" in
    @config/{env}/{service}.yml {"
        service: {service}
        environment: {env}
        port: {if is_prod then "443" else port}
        debug: {if is_prod then "false" else "true"}
        replicas: {if is_prod then "3" else "1"}
        log_level: {if is_prod then "warn" else "debug"}
    "}
in

map make_config environments
```

### Preview

```bash
avon eval deploy.av -service auth -port 9090
```

This shows all three config files without writing anything.

### Deploy

```bash
avon deploy deploy.av -service auth -port 9090 --root ./output --force
```

This writes:
- `./output/config/dev/auth.yml`
- `./output/config/staging/auth.yml`
- `./output/config/prod/auth.yml`

### What We Used

This single program combines nearly everything we learned:

| Concept | Where It's Used |
|---------|----------------|
| Functions & defaults | `\service ? "api" \port ? "8080"` |
| Let bindings | `let environments = ...` |
| Lists | `["dev", "staging", "prod"]` |
| Map | `map make_config environments` |
| Conditionals | `if is_prod then ... else ...` |
| Templates & interpolation | `{"service: {service}"}` |
| FileTemplates | `@config/{env}/{service}.yml {...}` |
| Dynamic paths | `{env}` and `{service}` in the path |
| CLI arguments | `-service auth -port 9090` |
| Deploy flags | `--root ./output --force` |

---

## Where to Go Next

You now know the fundamentals of Avon. Here's where to continue:

### Documentation

| Resource | Description |
|----------|-------------|
| [Full Tutorial](./TUTORIAL.md) | Comprehensive 6000+ line reference covering every feature |
| [Built-in Functions Reference](./BUILTIN_FUNCTIONS.md) | All 194 functions with signatures and examples |
| `avon doc <function>` | Look up any function from the command line |

### Example Files

The `examples/` directory contains dozens of working programs:

```bash
# Browse available examples
ls examples/

# Try some out
avon eval examples/site_generator_minimal.av
avon eval examples/docker_compose_gen.av
avon eval examples/kubernetes_gen.av
```

### Key Topics to Explore

Once you're comfortable with the basics, dive into these topics in the [Full Tutorial](./TUTORIAL.md):

1. **Data format conversion** — Parse and generate JSON, YAML, TOML, CSV, XML, and more
2. **Parallel operations** — `pmap`, `pfilter`, `pfold` for large datasets
3. **Advanced template techniques** — Multi-brace delimiters, complex interpolations
4. **REPL power features** — `:let`, `:vars`, `:deploy`, `:benchmark`, `:save-session`
5. **Embedding Avon** — Pipe stdin/stdout, exit codes, embedding in shell/Python/Node.js scripts
6. **Safety & security** — Atomic deployments, path validation, secrets management

> **Tip:** You already learned the fundamentals of importing in Lesson 14. The full tutorial covers edge cases, error handling, and advanced patterns.

### Quick Reference Card

```
# Values
42                       Number
3.14                     Float
"hello"                  String (escape sequences, no interpolation)
{"hello {x}"}           Template (interpolation, no escape sequences)
true / false             Boolean
[1, 2, 3]               List
{key: "value"}          Dictionary
\x x + 1                Function
none                     None (absence of value)

# Operators
+  -  *  /  //  %  **   Arithmetic
==  !=  >  <  >=  <=    Comparison
&&  ||  not              Logic
->                       Pipe
+                        String/List concatenation

# Core Syntax
let x = 10 in expr      Variable binding
if cond then a else b    Conditional
\x body                  Function (lambda)
\x ? default body        Function with default
@path {"content"}       FileTemplate

# Essential Commands
avon run 'expr'          Evaluate inline expression
avon eval file.av        Preview file output
avon deploy file.av      Write files to disk
avon repl                Interactive shell
avon doc <name>          Look up function docs
```

---

Happy generating! 🚀
