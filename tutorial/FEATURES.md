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

### Path Values
Paths are first-class values that can be stored in variables, passed to functions, and interpolated with variables.

**Syntax:** Use `@` prefix to create a path value:
```avon
# Store a path in a variable
let config_path = @config/production.json in

# Use with file operations
let content = readfile config_path in
let exists = exists config_path in

# Path interpolation
let env = "staging" in
let app = "myapp" in
let dynamic_path = @config/{env}/{app}.yml in

# Use paths with any file function
let lines = readlines dynamic_path in
let base = basename dynamic_path in
let dir = dirname dynamic_path in
```

**Benefits:**
- **Reusability:** Define a path once, use it multiple times
- **Composition:** Pass paths as function arguments
- **Type safety:** Paths are distinct from strings
- **Interpolation:** Dynamic path construction with variables

**Supported Functions:** All file operations accept path values:
- `readfile`, `readlines`, `import`
- `fill_template`, `walkdir`
- `exists`, `basename`, `dirname`

**Examples:** `examples/path_value_demo.av`, `examples/simple_path_test.av`, `examples/path_interpolation_test.av`, `examples/fill_with_path.av`

### File Templates
Combine paths with templates for file generation:

```avon
@/config.yml {"
    environment: prod
    debug: false
"}
```

Paths in file templates can also be stored in variables:
```avon
let output_file = @/tmp/report.txt in
output_file {"Generated report content"}
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
| `repeat` | 2 | `repeat "x" 3` | `"xxx"` |
| `pad_left` | 3 | `pad_left "7" 3 "0"` | `"007"` |
| `pad_right` | 3 | `pad_right "hi" 5 " "` | `"hi   "` |
| `indent` | 2 | `indent "code" 4` | `"    code"` |
| `str` | 1 | `str 42` | `"42"` |

**Examples:** `examples/string_functions.av`, `examples/split_join.av`, `examples/new_functions_demo.av`

### 🔍 String Predicates

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `is_digit` | 1 | Check if all chars are digits | `is_digit "123"` | `true` |
| `is_alpha` | 1 | Check if all chars are alphabetic | `is_alpha "abc"` | `true` |
| `is_alphanumeric` | 1 | Check if all chars are alphanumeric | `is_alphanumeric "abc123"` | `true` |
| `is_whitespace` | 1 | Check if all chars are whitespace | `is_whitespace "  "` | `true` |
| `is_uppercase` | 1 | Check if all chars are uppercase | `is_uppercase "ABC"` | `true` |
| `is_lowercase` | 1 | Check if all chars are lowercase | `is_lowercase "abc"` | `true` |
| `is_empty` | 1 | Check if string or list is empty | `is_empty ""` | `true` |

### 📊 List Operations

| Function | Arity | Purpose | Example |
|----------|-------|---------|---------|
| `map` | 2 | Transform each item | `map (\x x + 1) [1,2,3]` → `[2,3,4]` |
| `filter` | 2 | Keep matching items | `filter (\x x > 2) [1,2,3]` → `[3]` |
| `fold` | 3 | Reduce to value | `fold (\a \x a + x) 0 [1,2,3]` → `6` |
| `flatmap` | 2 | Map then flatten | `flatmap (\x [x,x]) [1,2]` → `[1,1,2,2]` |
| `flatten` | 1 | Flatten one level | `flatten [[1,2],[3]]` → `[1,2,3]` |
| `length` | 1 | Count items | `length [1,2,3]` → `3` |

**Examples:** `examples/map_example.av`, `examples/filter_example.av`, `examples/fold_example.av`, `examples/map_filter_fold.av`

### 📁 File & Filesystem

| Function | Arity | Purpose |
|----------|-------|---------|
| `readfile` | 1 | Read entire file as string |
| `readlines` | 1 | Read file lines as list |
| `fill_template` | 2 | Read file and fill `{placeholders}` with values |
| `exists` | 1 | Check if file exists |
| `basename` | 1 | Extract filename from path |
| `dirname` | 1 | Extract directory from path |
| `walkdir` | 1 | List all files in directory recursively |

**Example - fill_template:**
```avon
# template.txt contains: "Hello, {name}! Email: {email}"
let subs = [["name", "Alice"], ["email", "alice@example.com"]] in
fill_template "template.txt" subs
# Result: "Hello, Alice! Email: alice@example.com"
```

### 🌐 HTML Generation

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `html_escape` | 1 | Escape HTML chars | `html_escape "<div>"` | `"&lt;div&gt;"` |
| `html_tag` | 2 | Create HTML tag | `html_tag "p" "text"` | `"<p>text</p>"` |
| `html_attr` | 2 | Create attribute | `html_attr "class" "btn"` | `"class=\"btn\""` |

**Examples:** `examples/html_page_gen.av`, `examples/site_generator.av`

### 📝 Markdown Generation

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `md_heading` | 2 | Create heading | `md_heading 1 "Title"` | `"# Title"` |
| `md_link` | 2 | Create link | `md_link "text" "url"` | `"[text](url)"` |
| `md_code` | 1 | Inline code | `md_code "x = 1"` | `` "`x = 1`" `` |
| `md_list` | 1 | Create list | `md_list ["a","b"]` | `"- a\n- b"` |

**Examples:** `examples/markdown_readme_gen.av`

### � Type Conversion & Formatting

| Function | Arity | Purpose | Example |
|----------|-------|---------|---------|
| `to_string` | 1 | Convert to string | `to_string 42` → `"42"` |
| `to_int` | 1 | Convert to integer | `to_int "42"` → `42` |
| `to_float` | 1 | Convert to float | `to_float "3.14"` → `3.14` |
| `to_bool` | 1 | Convert to boolean | `to_bool "yes"` → `true` |
| `format_int` | 2 | Format int with padding | `format_int 7 3` → `"007"` |
| `format_float` | 2 | Format float with precision | `format_float 3.14159 2` → `"3.14"` |

**Examples:** `examples/casting_demo.av`

### �📦 Data & Utilities

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
- `--force` — Overwrite existing files without warning
- `--append` — Append to existing files instead of overwriting
- `--if-not-exists` — Only write file if it doesn't already exist
- **Default**: Files are NOT overwritten; a clear warning is shown instead

### Fetch from GitHub

**Deploy from GitHub** (automatic deployment):
```bash
avon --git pyrotek45/avon/examples/site_generator.av --root ./out
```

**Evaluate from GitHub** (just run and print):
```bash
avon --git-eval pyrotek45/avon/examples/string_functions.av
```

Fetch and run programs directly from GitHub's raw content CDN. The `--git` flag automatically deploys, while `--git-eval` evaluates and prints the result.

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
