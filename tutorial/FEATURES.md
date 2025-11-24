# Avon — Features Reference

A quick reference to all Avon language features, builtins, and examples demonstrating each feature.

## Language Basics

### File Format
- Extension: `.av`
- Plain text files containing Avon code
- Comments: use `#` for single-line comments

### Primitive Types

| Type | Syntax | Example | Use |
|------|--------|---------|-----|
| Number | Decimal or float | `42`, `3.14` | Calculations, versions, counts |
| String | Double-quoted | `"hello"` | Text data, file names |
| Boolean | Keywords | `true`, `false` | Conditionals and flags |
| List | Bracketed | `[1, 2, 3]` | Collections, multiple items |

### Identifiers
Variable and function names use letters, digits, and underscores:
```avon
name
my_variable
_private_impl
func1
```

---

## Language Features

### Let Bindings
Introduce variables and intermediate values:

```avon
let greeting = "Hello" in
let name = "Alice" in
concat greeting name
```

**Examples:** `examples/nested_let.av`, `examples/let_cascade.av`

### Functions
Define reusable logic with parameters:

```avon
let double = \x x * 2 in
map double [1, 2, 3]
```

**Currying:** Functions are automatically curried:
```avon
let add = \x \y x + y in
let add5 = add 5 in     # Partially applied
add5 3                  # Result: 8
```

### Default Parameters
Provide fallback values:

```avon
\name ? "Guest" @/welcome.txt {"
Welcome, {name}!
"}
```

When deployed without a named argument, `name` defaults to `"Guest"`.

**Examples:** `examples/function_defaults.av`, `examples/deploy_list.av`

### Conditionals
Choose between alternatives:

```avon
if age > 18 then "adult" else "minor"

# In templates:
@/output.txt {"
Status: {if count > 0 then "has items" else "empty"}
"}
```

**Examples:** `examples/conditionals_template.av`

### Templates
Generate text with embedded expressions:

```avon
{"
Hello, {name}!
Count: {length items}
"}
```

**Key features:**
- Newlines are preserved
- Indentation is dedented (automatically)
- Any expression can be interpolated with `{expr}`
- Lists expand to newline-separated items
- Escape hatch for literal braces (see below)

**Examples:** `examples/list_insert.av`, `examples/complex_template.av`

#### Template Escape Hatch

Templates start with one or more opening braces followed immediately by a quote:

```
@/file.txt {" ... "}      # single-brace template (open_count = 1)
@/file.txt {{" ... "}}    # double-brace template (open_count = 2)
```

Interpolation uses exactly the same number of leading braces as the template opening:

```
@/single.txt {"Value: { 1 + 2 }"}
@/double.txt {{"Value: {{ 1 + 2 }}"}}
```

To produce literal braces without starting interpolation, use one more brace than the opener:

| Template opener | Interpolation | Literal `{` | Literal `}` | Example |
|-----------------|--------------|-------------|-------------|---------|
| `{`             | `{ expr }`   | `{{` → `{`  | `}}` → `}`  | `@/f.txt {"x: {{y: { z }}}"}` |
| `{{`            | `{{ expr }}` | `{{{` → `{` | `}}}` → `}` | `@/f.txt {{"obj: {{{ x: {{ y }} }}}"}}`  |

**General rule:** A run of k consecutive braces outputs (k - open_count) literal braces when k > open_count.
- Single-brace: `{{` (2) → 1 literal, `{{{{` (4) → 3 literals
- Double-brace: `{{{` (3) → 1 literal, `{{{{` (4) → 2 literals

#### Practical Examples

**Lua configuration** (single-brace template):
```avon
@/config.lua {"
local settings = {{
  name = "app",
  debug = { if dev then "true" else "false" }
}}
"}
```
Produces:
```lua
local settings = {
  name = "app",
  debug = false
}
```

**Nginx server block** (single-brace template):
```avon
@/nginx.conf {"
server {{
  listen 80;
  server_name { domain };
}}
"}
```

**Terraform HCL** (double-brace template with `{{ }}` for interpolation):
```avon
@/main.tf {{"
resource "aws_instance" "web" {{
  ami = "{{ ami_id }}"
  tags = {{
    Name = "{{ instance_name }}"
  }}
}}
"}}
```

See `examples/escape_hatch.av` for comprehensive single and double-brace demonstrations with both interpolation and literal brace sequences.

### File Templates
Combine paths with templates for file generation:

```avon
@/config.yml {"
environment: prod
debug: false
"}
```

When deployed, this writes the template content to the specified file.

**Examples:** `examples/site_generator.av`, `examples/named_args.av`, `examples/large_program.av`

---

## Builtins by Category

### 📝 String Operations

| Function | Arity | Example | Result |
|----------|-------|---------|--------|
| `concat` | 2 | `concat "hello" " world"` | `"hello world"` |
| `upper` | 1 | `upper "hello"` | `"HELLO"` |
| `lower` | 1 | `lower "WORLD"` | `"world"` |
| `length` | 1 | `length "hello"` | `5` |
| `contains` | 2 | `contains "hello" "ell"` | `true` |
| `starts_with` | 2 | `starts_with "hello" "he"` | `true` |
| `ends_with` | 2 | `ends_with "hello" "lo"` | `true` |
| `split` | 2 | `split "a,b,c" ","` | `["a", "b", "c"]` |
| `join` | 2 | `join ["a","b"] ", "` | `"a, b"` |
| `replace` | 3 | `replace "hello" "l" "L"` | `"heLLo"` |
| `trim` | 1 | `trim "  hello  "` | `"hello"` |

**Examples:** `examples/string_functions.av`, `examples/split_join.av`

### 📊 List Operations

| Function | Arity | Purpose | Example |
|----------|-------|---------|---------|
| `map` | 2 | Transform each item | `map (\x x + 1) [1,2,3]` → `[2,3,4]` |
| `filter` | 2 | Keep matching items | `filter (\x x > 2) [1,2,3]` → `[3]` |
| `fold` | 3 | Reduce to value | `fold (\a \x a + x) 0 [1,2,3]` → `6` |
| `length` | 1 | Count items | `length [1,2,3]` → `3` |

**Examples:** `examples/map_example.av`, `examples/filter_example.av`, `examples/fold_example.av`, `examples/map_filter_fold.av`

### 📁 File & Filesystem

| Function | Arity | Purpose |
|----------|-------|---------|
| `readfile` | 1 | Read entire file as string |
| `readlines` | 1 | Read file lines as list |
| `exists` | 1 | Check if file exists |
| `basename` | 1 | Extract filename from path |
| `dirname` | 1 | Extract directory from path |

### 📦 Data & Utilities

| Function | Arity | Purpose | Example |
|----------|-------|---------|---------|
| `import` | 1 | Load another `.av` file | `import "lib.av"` |
| `json_parse` | 1 | Parse JSON string | `json_parse "{\"x\": 1}"` |
| `os` | 0 | Get operating system | `os` → `"linux"`, `"windows"`, `"macos"` |

**Examples:** `examples/import_example.av`

---

## Operators

### Arithmetic
- `+` Addition (numbers)
- `-` Subtraction
- `*` Multiplication
- `/` Division

### String Concatenation
```avon
"hello" + " " + "world"    # "hello world"
```

### List Concatenation
```avon
[1, 2] + [3, 4]            # [1, 2, 3, 4]
```

### Comparison
- `==` Equal
- `!=` Not equal
- `>` Greater than
- `<` Less than
- `>=` Greater or equal
- `<=` Less or equal

---

## CLI Commands

### Evaluate a Program
```bash
avon eval examples/map_example.av
```
Runs the program and prints the result.

### Deploy Files
```bash
avon examples/site_generator.av --deploy --root ./website --force
```

Deploy the program, generate files in the specified directory.

**Flags:**
- `--deploy` — Generate files from file templates
- `-param value` — Named argument (e.g., `-name Alice`)
- `--root <dir>` — Prepend to all generated paths
- `--force` — Overwrite existing files

### Fetch from GitHub
```bash
avon --git owner/repo/examples/gen.av --deploy --root ./out
```

Fetch and run a program from GitHub's raw content CDN.

---

## Real-World Examples by Use Case

### 🌐 Web Development

**Site Generator:** `examples/site_generator.av`
- Generate HTML pages with shared layouts
- Dynamic navigation and content
- CSS styling

**Package.json Generator:** `examples/package_json_gen.av`
- Configure Node.js project
- Conditional dependencies
- Custom scripts

### ⚙️ Configuration & Infrastructure

**Docker Compose:** `examples/docker_compose_gen.av`
- Multi-service setup
- Environment variables
- Volumes and networks

**Kubernetes Manifests:** `examples/kubernetes_gen.av`
- Deployment, Service, Ingress
- ConfigMaps and Secrets
- Resource limits and probes

**GitHub Actions:** `examples/github_actions_gen.av`
- CI/CD workflows
- Conditional jobs
- Multi-file generation

### 🛠️ Tool Configuration

**Neovim Config:** `examples/neovim_init.av`
- Plugin management
- LSP configuration
- Custom keybindings

**Emacs Config:** `examples/emacs_init.av`
- Package organization by category
- Conditional features (LSP, Org Mode)
- Theme and UI settings

---

## Examples Quick Reference

| Example | Features Shown | Complexity |
|---------|----------------|------------|
| `test.av` | Basics, lists | ⭐ |
| `nested_let.av` | Let bindings | ⭐ |
| `list_insert.av` | Lists, templates | ⭐⭐ |
| `map_example.av` | Map, filter | ⭐⭐ |
| `fold_example.av` | Fold operation | ⭐⭐ |
| `function_defaults.av` | Functions, defaults | ⭐⭐ |
| `string_functions.av` | String builtins | ⭐⭐ |
| `conditionals_template.av` | If/then/else in templates | ⭐⭐ |
| `site_generator.av` | Multi-file generation | ⭐⭐⭐ |
| `neovim_init.av` | Complex config, conditionals | ⭐⭐⭐ |
| `emacs_init.av` | Feature toggles, filtering | ⭐⭐⭐ |
| `docker_compose_gen.av` | Multi-service templates | ⭐⭐⭐ |
| `kubernetes_gen.av` | Complex multi-file output | ⭐⭐⭐ |
| `github_actions_gen.av` | Conditional YAML generation | ⭐⭐⭐ |

---

## Quick Start for Common Tasks

### Generate a Single Configuration File
```avon
let app = "myapp" in
@/config.yml {"
app: {app}
debug: false
"}
```

### Generate Files for Multiple Items
```avon
let items = ["dev", "staging", "prod"] in
map (\item @/config-{item}.yml {"{item}"}) items
```

### Complex Configuration with Conditionals
```avon
let env = "prod" in
@/.env {"
DEBUG={if env == "prod" then "false" else "true"}
CACHE_ENABLED={if env == "dev" then "false" else "true"}
"}
```

### Transform and Filter Data
```avon
let names = ["Alice", "Bob", "Charlie"] in
let formatted = map (\n upper n) names in
let long_names = filter (\n (length n) > 3) formatted in
{join long_names ", "}
```

---

## Testing Examples

Run all examples to verify they work:

```bash
bash scripts/run_examples.sh
```

This script:
1. Builds the Avon compiler
2. Runs each example in a temporary directory
3. Verifies expected files are created
4. Reports pass/fail for each example

All 21+ examples pass, including complex multi-file generation and conditional logic.

---

## Tips & Tricks

### 1. Use Let for Clarity
Instead of deeply nested expressions, use `let` to name intermediate values.

### 2. Debug with Eval
Always test with `eval` before deploying:
```bash
avon program.av eval        # Check output first
avon program.av --deploy    # Then generate files
```

### 3. Template Indentation for Readability
Use indentation in templates — Avon's dedent removes it:

```avon
@/config.yml {"
	server:
	  host: localhost
	  port: 8080
"}
```

### 4. Partial Application for Reuse
Create helper functions by partially applying builtins:

```avon
let upper_all = map upper in
upper_all ["hello", "world"]
```

### 5. Comments for Documentation
```avon
# Generate configuration for each environment
let envs = ["dev", "staging", "prod"] in
```

---

## Performance Characteristics

- **Parsing:** O(n) with single pass
- **Evaluation:** O(1) for primitives, O(n) for lists (one-time cost)
- **File I/O:** Linear in number of files generated

For most use cases (100s of files), Avon runs in milliseconds.

---

Have fun generating! 🚀 For more details, see `tutorial/TUTORIAL.md`.
