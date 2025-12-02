# Avon — Features Reference

A quick reference to all Avon language features, builtins, and examples demonstrating each feature.

## Key Feature: Share Templates with `--git`

One of Avon's most powerful features is the `--git` flag, which enables easy sharing and deployment of templates directly from GitHub. This makes it incredibly easy to:

- Share your templates: Put templates in GitHub, others can deploy with custom values
- Use others' templates: Deploy templates from GitHub repositories with your own settings
- Centralized management: One template in git, many customized deployments
- Easy updates: When templates are updated, everyone can redeploy with latest changes

Format: `avon deploy --git user/repo/path/to/file.av --root <destination> [arguments]`

Example:
```bash
# Deploy a vim config from GitHub with custom settings
avon deploy --git user/repo/vimrc.av --root ~ -username alice -theme gruvbox

# Deploy the same template with different settings
avon deploy --git user/repo/vimrc.av --root ~ -username bob -theme solarized
```

This workflow is perfect for dotfiles, team configs, infrastructure templates, and any shared configuration. See [SIMPLE_CONFIGS.md](./SIMPLE_CONFIGS.md) for detailed examples.

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
| None | Keyword | `none` | Absence of value (empty list head, missing key) |
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
# Result: "HelloAlice"
```

Examples: `examples/nested_let.av`, `examples/let_cascade.av`

### Functions
Define reusable logic with parameters:

```avon
let double = \x x * 2 in
map double [1, 2, 3]
# Result: [2, 4, 6]
```

Currying: Functions are automatically curried:
```avon
let add = \x \y x + y in
let add5 = add 5 in     # Partially applied
add5 3                  # Result: 8
```

**No Shadowing:** Neither `let` bindings nor lambda parameters can shadow builtin function names:

```avon
let map = 5 in map           # Error: 'map' is already defined
let f = \filter filter in f  # Error: cannot use 'filter' as parameter name
```

This prevents accidental bugs where a builtin is accidentally overwritten.

### Default Parameters
Provide fallback values using `?` after the parameter name:

```avon
# Single parameter with default
let greet = \name ? "Guest" {"Hello {name}!"} in
greet              # Result: "Hello Guest!"
greet "Alice"      # Result: "Hello Alice!"

# Multiple parameters with defaults
let config = \host ? "localhost" \port ? 8080 {"{host}:{port}"} in
config                     # Result: "localhost:8080"
config "example.com"       # Result: "example.com:8080"
config "example.com" 443   # Result: "example.com:443"

# In deploy files
\name ? "Guest" @welcome.txt {"
    Welcome, {name}!
"}
```

When deployed without a named argument, `name` defaults to `"Guest"`.

**Examples:** `examples/function_defaults.av`, `examples/deploy_list.av`

### Conditionals
Choose between alternatives:

```avon
if age > 18 then "adult" else "minor"
# Result (if age = 25): "adult"

# In templates:
let count = 5 in
@output.txt {"
    Status: {if count > 0 then "has items" else "empty"}
"}
# Result: Status: has items
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
- **Indentation is automatically dedented** (based on first non-whitespace character)
- Any expression can be interpolated with `{expr}`
- Lists expand to newline-separated items
- Multi-brace delimiters for literal braces (see below)

**Indentation:** The column position of the first non-whitespace character in a template becomes the baseline. All lines are dedented by that amount. This lets you indent templates naturally to match your code structure:

```avon
# Indented by 2 spaces - first content at column 2
let config = \name {"
  Name: {name}
  Port: 8080
"}
in

# Output has no leading indent, internal structure preserved:
# Name: MyApp
# Port: 8080
```

This works with any nesting level and preserves relative indentation (useful for Python/YAML code generation).

**Examples:** `examples/list_insert.av`, `examples/complex_template.av`, `examples/baseline_indentation_demo.av`

#### Multi-Brace Delimiters for Literal Braces

Avon templates use a **variable-brace delimiter system** that lets you choose how many opening braces to use. This is a powerful feature that adapts to your content's needs:

```
@file.txt {" ... "}      # single-brace template (open_count = 1)
@file.txt {{" ... "}}    # double-brace template (open_count = 2)
@file.txt {{{" ... "}}}  # triple-brace template (open_count = 3)
```

**Why this design?**

When your template output contains curly braces (JSON, CSS, Lua, Nginx, etc.), you need a way to distinguish:
- **Literal braces in the output** (e.g., `{` for Lua tables, JSON objects, CSS rules)
- **Interpolation braces** (e.g., `{variable_name}` to substitute values)

By choosing a higher delimiter level, braces with fewer than that count become literal. No escaping needed.

**How it works:**

Interpolation uses exactly the same number of braces as the template opener:

```
@single.txt {"Value: { 1 + 2 }"}        # interpolation with { }
@double.txt {{"Value: {{ 1 + 2 }}"}}    # interpolation with {{ }}
@triple.txt {{{" Value: {{{ 1 + 2 }}} "}}}  # interpolation with {{{ }}}
```

In higher-level templates, fewer braces are literal:

| Template | Interpolate with | Literal braces | Use case |
|----------|-----------------|----------------|----------|
| `{" "}` | `{expr}` | (none) | Plain text, YAML |
| `{{" "}}` | `{{expr}}` | `{` `}` | JSON, CSS, Lua, Nginx |
| `{{{" "}}}` | `{{{expr}}}` | `{` `{{` `}` `}}` | GitHub Actions, Mustache |
| `{{`            | `{{ expr }}` | `{{{` -> `{` | `}}}` -> `}` | Many braces in output (JSON, HCL, Terraform) |
| `{{{`           | `{{{ expr }}}` | `{{{{` -> `{` | `}}}}` -> `}` | Output full of triple-braces (rare) |

**General rule:** A run of k consecutive braces outputs (k - open_count) literal braces when k > open_count.
- Single-brace: `{{` (2 braces) -> 1 literal `{`, `{{{{` (4 braces) -> 3 literals `{{{`
- Double-brace: `{{{` (3 braces) -> 1 literal `{`, `{{{{` (4 braces) -> 2 literals `{{`
- Triple-brace: `{{{{` (4 braces) -> 1 literal `{`, `{{{{{` (5 braces) -> 2 literals `{{`

**Choosing the right delimiter:**

```avon
# Few braces? Single-brace is fine
@simple.txt {"
  Config: { value }
"}

# Many braces? Use double-brace to avoid escaping
@config.json {{"
  {
    "key": "{{ value }}",
    "nested": {
      "setting": true
    }
  }
"}}

# Very brace-heavy? Use triple-brace (rare)
@complex.txt {{{" 
  {{ outer }} and {{{ interpolation }}} mixed
"}}}
```

#### Practical Examples

**Lua configuration** (single-brace template with brace escaping):
```avon
let dev = true in
@config.lua {"
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
  debug = true
}
```

Note: In single-brace templates `{" "}`, literal braces must be escaped as `{{` and `}}`. The interpolation `{expr}` requires single braces.

**Nginx server block** (single-brace template with brace escaping):
```avon
let domain = "example.com" in
@nginx.conf {"
  server {{
    listen 80;
    server_name { domain };
  }}
"}
```
Produces:
```nginx
server {
  listen 80;
  server_name example.com;
}
```

Note: The outer `{{` and `}}` are literal braces, while the `{ domain }` is interpolation.

**Terraform HCL** (double-brace template - cleaner, no escaping for output braces):

With double braces `{{"..."}}`, single braces in the output are literal and don't need escaping. Only `{{ }}` is used for interpolation:

```avon
let ami_id = "ami-0c55b159cbfafe1f0" in
let instance_name = "web-server" in
@main.tf {{"
  resource "aws_instance" "web" {
    ami = "{{ ami_id }}"
    tags = {
      Name = "{{ instance_name }}"
    }
  }
"}}
```
Produces:
```hcl
resource "aws_instance" "web" {
  ami = "ami-0c55b159cbfafe1f0"
  tags = {
    Name = "web-server"
  }
}
```

**Strategic choice: Which delimiter for your use case?**

The key insight is choosing your template delimiter based on brace density in your output:

| Use Case | Delimiter | Reason | Example |
|----------|-----------|--------|---------|
| Simple config, few braces | `{" "}` | No escaping needed | YAML, INI files |
| Configuration language | `{{" "}}` | Few escapes needed | Lua, shell scripts |
| Data formats | `{{" "}}` | Cleaner than escaping | JSON, HCL, TOML |
| Code with many braces | `{{" "}}` or higher | Minimize escaping burden | Python, JavaScript, Go |
| Extreme cases (rare) | `{{{" "}}}` or more | Only when absolutely necessary | Custom DSLs with brace syntax |

<!-- If you find yourself using quadruple braces, take a step back. Go outside. Touch grass. Reconsider your life choices. Then come back and refactor. -->

**Examples by brace density:**

```avon
# YAML config (few braces) - single-brace is fine:
@app.yml {"
app:
  name: myapp
  debug: { debug_mode }
"}

# JSON (lots of braces) - double-brace is much cleaner:
@config.json {{"
{
  "database": {
    "host": "{{ db_host }}",
    "port": {{ db_port }}
  }
}
"}}

# Python code (many braces for dict literals):
@config.py {{{
def get_config():
    return {
        "database": "{{ db_name }}",
        "settings": {{ "{} nested dict with triple brace" }}
    }
}}}
```

With this system, your templates stay readable even in brace-heavy contexts—you simply choose the delimiter that fits your content.

See `tutorial/TEMPLATE_SYNTAX.md` for comprehensive documentation of the multi-brace delimiter system.

### Path Values
Paths are first-class values that can be stored in variables, passed to functions, interpolated with variables, and combined using `+`.

**Syntax:** Use `@` prefix to create a path value:
```avon
# Store a path in a variable
let config_path = @config/production.json in

# Use with file operations
let content = readfile config_path in
let exists = exists config_path in

# Path interpolation (single braces only)
let env = "staging" in
let app = "myapp" in
let dynamic_path = @config/{env}/{app}.yml in

# Path concatenation with +
let base = @config in
let file = @app.json in
let full = base + file in  # Results in @config/app.json

# Use paths with any file function
let lines = readlines dynamic_path in
let base_name = basename dynamic_path in
let dir = dirname dynamic_path in
```

**Important Rules:**
- **Path literals must be relative:** `@config/app.json` ✅, `@/etc/passwd` ❌
- Absolute paths are blocked at syntax level to prevent unsafe deployment
- For reading files with absolute paths, use **strings**: `readfile "/absolute/path"`
- For deployment to absolute locations, use `--root` flag: `avon deploy app.av --root /opt`
- Path concatenation with `+` automatically adds `/` separator as needed
- **VS Code highlighting:** Only relative paths (not starting with `/`) will be highlighted as paths

**Paths in Lists:**

Paths are first-class values and can be used directly in lists, making it easy to work with collections of files:

```avon
# Simple list of paths
[@config/app.yml, @config/database.yml, @config/cache.yml]
# Result: [app.yml, database.yml, cache.yml]

# Paths in function results
let environments = ["dev", "staging", "prod"] in
map (\env @config/{env}.yml) environments
# Result: [@config/dev.yml, @config/staging.yml, @config/prod.yml]

# Nested lists with paths
[
  [@logs/app.log, @logs/error.log],
  [@metrics/cpu.txt, @metrics/memory.txt]
]

# Generate multiple files from a list of paths
let files = [@readme.md, @license.txt, @changelog.md] in
let content = \file readfile file in
map content files  # Read all files
```

When used in templates, paths expand naturally:

```avon
let config_files = [@app.yml, @database.yml] in
@index.txt {"
Config files:
{config_files}
"}
```

**Benefits:**
- **Safety:** Syntax prevents accidental absolute path deployment
- **Reusability:** Define a path once, use it multiple times
- **Composition:** Pass paths as function arguments and combine with `+`
- **Type safety:** Paths are distinct from strings
- **Interpolation:** Dynamic path construction with variables (single `{}` only)
- **Collections:** Work with multiple paths easily using lists and iteration

**Supported Functions:** All file operations accept path values or strings:
- `readfile`, `readlines`, `import` - Accept Path or String
- `fill_template`, `walkdir` - Accept Path or String
- `exists`, `basename`, `dirname` - Accept Path or String
- **Note:** Strings can be absolute for reading; Path literals must be relative

**Examples:** `examples/path_value_demo.av`, `examples/simple_path_test.av`, `examples/path_interpolation_test.av`, `examples/fill_with_path.av`

### File Templates
Combine paths with templates for file generation:

```avon
@config.yml {"
    environment: prod
    debug: false
"}
```

Paths can be used in functions and with file operations, but file template syntax requires the `@` prefix at the point of template declaration:
```avon
@tmp/report.txt {"Generated report content"}
```

To reuse paths, pass them to functions or use them with file operations like `readfile`:
```avon
let output_file = @tmp/report.txt in
let content = readfile output_file in
content
```

**Examples:** `examples/site_generator.av`, `examples/named_args.av`, `examples/large_program.av`

---

## Builtins by Category

### String Operations

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
| `slice` | 3 | `slice "hello" 1 4` | `"ell"` |
| `char_at` | 2 | `char_at "hello" 2` | `"l"` |
| `chars` | 1 | `chars "hi"` | `["h", "i"]` |

**Examples:** `examples/string_functions.av`, `examples/split_join.av`, `examples/new_functions_demo.av`

### String Predicates

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `is_digit` | 1 | Check if all chars are digits | `is_digit "123"` | `true` |
| `is_alpha` | 1 | Check if all chars are alphabetic | `is_alpha "abc"` | `true` |
| `is_alphanumeric` | 1 | Check if all chars are alphanumeric | `is_alphanumeric "abc123"` | `true` |
| `is_whitespace` | 1 | Check if all chars are whitespace | `is_whitespace "  "` | `true` |
| `is_uppercase` | 1 | Check if all chars are uppercase | `is_uppercase "ABC"` | `true` |
| `is_lowercase` | 1 | Check if all chars are lowercase | `is_lowercase "abc"` | `true` |
| `is_empty` | 1 | Check if string or list is empty | `is_empty ""` | `true` |

### List Operations

| Function | Arity | Purpose | Example |
|----------|-------|---------|---------|
| `map` | 2 | Transform each item | `map (\x x + 1) [1,2,3]` -> `[2,3,4]` |
| `filter` | 2 | Keep matching items | `filter (\x x > 2) [1,2,3]` -> `[3]` |
| `fold` | 3 | Reduce to value | `fold (\a \x a + x) 0 [1,2,3]` -> `6` |
| `flatmap` | 2 | Map then flatten | `flatmap (\x [x,x]) [1,2]` -> `[1,1,2,2]` |
| `flatten` | 1 | Flatten one level | `flatten [[1,2],[3]]` -> `[1,2,3]` |
| `length` | 1 | Count items | `length [1,2,3]` -> `3` |
| `sort` | 1 | Sort list (numbers numerically, others lexically) | `sort [3,1,4,1,5]` -> `[1,1,3,4,5]` |
| `sort_by` | 2 | Sort by key function | `sort_by (\x neg x) [1,2,3]` -> `[3,2,1]` |
| `unique` | 1 | Remove duplicates (preserve order) | `unique [1,2,2,3,1]` -> `[1,2,3]` |
| `range` | 2 | Generate integer range (inclusive) | `range 1 5` -> `[1,2,3,4,5]` |
| `slice` | 3 | Extract sublist | `slice [1,2,3,4,5] 1 3` -> `[2,3]` |
| `enumerate` | 1 | Add indices | `enumerate ["a","b","c"]` -> `[[0,"a"],[1,"b"],[2,"c"]]` |

**Sorting Examples:**
```avon
# Sort numbers
sort [3, 1, 4, 1, 5, 9, 2, 6]  # [1, 1, 2, 3, 4, 5, 6, 9]

# Sort strings
sort ["zebra", "apple", "banana"]  # ["apple", "banana", "zebra"]

# Reverse sort using sort_by
sort_by (\x neg x) [5, 2, 8, 1]  # [8, 5, 2, 1]

# Sort by string length
sort_by (\x length x) ["aaa", "a", "aa"]  # ["a", "aa", "aaa"]

# Remove duplicates
unique [1, 2, 2, 3, 1, 4, 3, 5]  # [1, 2, 3, 4, 5]

# Generate ranges
range 1 10  # [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
range 5 8   # [5, 6, 7, 8]

# Slice sublists
slice [1,2,3,4,5] 1 3       # [2, 3]
slice [10,20,30,40] 0 2     # [10, 20]

# Enumerate for index tracking
enumerate ["apple", "banana", "cherry"]
# [[0, "apple"], [1, "banana"], [2, "cherry"]]

# Practical: Sort and enumerate
let items = ["zebra", "apple", "banana"] in
let sorted = sort items in
enumerate sorted
# [[0, "apple"], [1, "banana"], [2, "zebra"]]
```

**String Operations Examples:**
```avon
# Slicing strings
slice "hello" 1 4            # "ell"
slice "avon" 0 2             # "av"

# Access individual characters
char_at "hello" 0            # "h"
char_at "hello" 4            # "o"

# Convert string to character list
chars "hi"                   # ["h", "i"]
chars "abc"                  # ["a", "b", "c"]

# Practical: Process each character
let text = "hello" in
let chars_list = chars text in
map (\c upper c) chars_list  # ["H", "E", "L", "L", "O"]
```

**Examples:** `examples/map_example.av`, `examples/filter_example.av`, `examples/fold_example.av`, `examples/map_filter_fold.av`, `examples/list_operations.av`

### Map/Dictionary Operations

Avon provides a first-class Dict type for structured data with key-value pairs:

| Function | Arity | Purpose | Example |
|----------|-------|---------|---------|
| `get` | 2 | Get value by key | `get {name: "Alice"} "name"` -> `"Alice"` |
| `set` | 3 | Update or add key | `set {a: 1} "b" 2` -> `{a: 1, b: 2}` |
| `keys` | 1 | Extract all keys | `keys {a: 1, b: 2}` -> `["a", "b"]` |
| `values` | 1 | Extract all values | `values {a: 1, b: 2}` -> `[1, 2]` |
| `has_key` | 2 | Check if key exists | `has_key {a: 1} "a"` -> `true` |

Note: These functions work with both dictionaries and lists of pairs (list of 2-element lists). "Pairs" is not a separate type—it's a list of pairs: `[["key", value], ...]`. For example, `get [[\"a\", 1], [\"b\", 2]] \"a\"` works the same as `get {a: 1, b: 2} \"a\"`.

**Dict Syntax:**  
Dictionaries use curly braces with colon notation:

```avon
# Create a dict
let config = {host: "localhost", port: 8080, debug: true} in

# Access with dot notation
let host = config.host in          # Result: "localhost"
let port = config.port in          # Result: 8080

# Query the dict
let all_keys = keys config in      # Result: ["debug", "host", "port"]
let all_values = values config in  # Result: [true, "localhost", 8080]
```

**JSON Integration:**  
JSON objects are automatically parsed as dicts:

```avon
# config.json: {"app": "myapp", "version": "1.0.0", "debug": true}
let data = json_parse (readfile @config.json) in
let app_name = data.app in         # Result: "myapp"
let version = get data "version" in # Result: "1.0.0"
let all_keys = keys data in        # Result: ["app", "debug", "version"]
has_key data "version"             # Result: true
```

**Examples:** `examples/dict_operations.av`, `examples/json_map_demo.av`

### File & Filesystem

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
let subs = {name: "Alice", email: "alice@example.com"} in
fill_template @template.txt subs
# Result: "Hello, Alice! Email: alice@example.com"
```

### HTML Generation

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `html_escape` | 1 | Escape HTML chars | `html_escape "<div>"` | `"&lt;div&gt;"` |
| `html_tag` | 2 | Create HTML tag | `html_tag "p" "text"` | `"<p>text</p>"` |
| `html_attr` | 2 | Create attribute | `html_attr "class" "btn"` | `"class=\"btn\""` |

**Examples:** `examples/html_page_gen.av`, `examples/site_generator.av`

### Markdown Generation

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `md_heading` | 2 | Create heading | `md_heading 1 "Title"` | `"# Title"` |
| `md_link` | 2 | Create link | `md_link "text" "url"` | `"[text](url)"` |
| `md_code` | 1 | Inline code | `md_code "x = 1"` | `` "`x = 1`" `` |
| `md_list` | 1 | Create list | `md_list ["a","b"]` | `"- a\n- b"` |

**Examples:** `examples/markdown_readme_gen.av`

### Type Checking & Validation

Avon provides comprehensive type introspection and validation utilities:

#### Type Introspection

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `typeof` | 1 | Get type name | `typeof 42` | `"Number"` |
| `is_string` | 1 | Check if string | `is_string "hello"` | `true` |
| `is_number` | 1 | Check if number | `is_number 42` | `true` |
| `is_int` | 1 | Check if integer | `is_int 42` | `true` |
| `is_float` | 1 | Check if float | `is_float 3.14` | `true` |
| `is_list` | 1 | Check if list | `is_list [1,2,3]` | `true` |
| `is_bool` | 1 | Check if boolean | `is_bool true` | `true` |
| `is_function` | 1 | Check if function | `is_function (\x x)` | `true` |
| `is_none` | 1 | Check if None | `is_none (head [])` | `true` |

#### Working with None

`None` represents the absence of a value. Functions that might not find what they're looking for return `none`:

```avon
head []                     # Result: none (empty list)
get {a: 1} "b"              # Result: none (missing key)
```

Check for None values with `is_none`:

```avon
let x = head [] in
if is_none x then "empty" else x
# Result: "empty"

let config = {port: 8080} in
let val = get config "missing_key" in
if is_none val then "default" else val
# Result: "default"
```

#### Assertions & Error Handling

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `assert` | 2 | Assert condition, return value or error | `assert (x > 0) x` | `x` (or error with debug info) |
| `trace` | 2 | Print label + value to stderr | `trace "x" 42` | `42` (prints `[TRACE] x: 42`) |
| `debug` | 1 | Pretty-print structure to stderr | `debug [1,2,3]` | `[1,2,3]` (prints `[DEBUG] List([...])`) |
| `error` | 1 | Throw custom error | `error "Invalid port"` | Throws error with message |

<!-- Easter egg: If you're reading the raw markdown, congrats! Run `avon run '6 * 9'` and ponder the universe. -->

**Common assertion patterns:**

```avon
assert (is_string x) x      # Assert x is a string
assert (is_number x) x      # Assert x is a number
assert (is_int x) x         # Assert x is an integer
assert (is_list xs) xs      # Assert xs is a list
assert (x > 0) x            # Assert x is positive
assert (length xs > 0) xs   # Assert list is not empty
```

**Practical Examples:**

```avon
# Type checking before operations
let process_config = \cfg
  let port_val = get cfg "port" in
  let host_val = get cfg "host" in
  let port = assert (is_number port_val) port_val in
  let host = assert (is_string host_val) host_val in
  {"Config: {host}:{port}"}
in

# Validation with helpful errors
let validate_port = \p
  if is_number p then
    if p > 0 && p < 65536 then p
    else error "Port must be between 1 and 65535"
  else error "Port must be a number"
in

# Debugging computation pipeline
let compute = \x
  let doubled = trace "doubled" (x * 2) in
  let added = trace "added 10" (doubled + 10) in
  added
in

# Inspecting complex structures
let analyze = \data
  let _ = debug data in
  typeof data
in

# Type-safe wrapper
let safe_divide = \a \b
  let num = assert (is_number a) a in
  let denom = assert (is_number b) b in
  if denom == 0 then
    error "Division by zero"
  else
    num / denom
in
```

**Use Cases:**
- **Validation:** Ensure configuration values have correct types
- **Debugging:** Trace intermediate values in computations
- **Error Messages:** Provide clear custom error messages
- **Type Safety:** Add runtime type checks to critical functions

**Examples:** `examples/type_checking_demo.av`

### Type Conversion

| Function | Arity | Purpose | Example |
|----------|-------|---------|---------|
| `to_string` | 1 | Convert to string | `to_string 42` -> `"42"` |
| `to_int` | 1 | Convert to integer | `to_int "42"` -> `42` |
| `to_float` | 1 | Convert to float | `to_float "3.14"` -> `3.14` |
| `to_bool` | 1 | Convert to boolean | `to_bool "yes"` -> `true` |
| `to_char` | 1 | Codepoint to character | `to_char 72` -> `"H"` |
| `to_list` | 1 | String to char list | `to_list "Hi"` -> `["H", "i"]` |

Type conversions are explicit. None of that "let me guess what type you meant" sorcery.

### Formatting Functions

Avon provides a comprehensive suite of 15 formatting functions for various data types:

#### Number Formatting

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `format_int` | 2 | Zero-padded integers | `format_int 7 3` | `"007"` |
| `format_float` | 2 | Decimal precision | `format_float 3.14159 2` | `"3.14"` |
| `format_hex` | 1 | Hexadecimal | `format_hex 255` | `"ff"` |
| `format_octal` | 1 | Octal | `format_octal 64` | `"100"` |
| `format_binary` | 1 | Binary | `format_binary 15` | `"1111"` |
| `format_scientific` | 2 | Scientific notation | `format_scientific 12345 2` | `"1.23e4"` |
| `format_bytes` | 1 | Human-readable bytes | `format_bytes 1536000` | `"1.46 MB"` |
| `format_currency` | 2 | Currency with symbol | `format_currency 19.99 "$"` | `"$19.99"` |
| `format_percent` | 2 | Percentage | `format_percent 0.856 2` | `"85.60%"` |

#### Collection Formatting

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `format_list` | 2 | Join with separator | `format_list ["a","b","c"] ", "` | `"a, b, c"` |
| `format_table` | 2 | 2D table | `format_table [["A","B"],["1","2"]] " \| "` | `"A \| B\n1 \| 2"` |
| | | | Also accepts dicts: `format_table {a: 1, b: 2} " \| "` | `"a \| b\n1 \| 2"` |
| `format_json` | 1 | JSON representation | `format_json [1,2,3]` | `"[1, 2, 3]"` |

#### Text Formatting

| Function | Arity | Purpose | Example | Result |
|----------|-------|---------|---------|--------|
| `format_bool` | 2 | Custom bool text | `format_bool (1==1) "yes/no"` | `"Yes"` |
| `truncate` | 2 | Truncate with ... | `truncate "Long text" 8` | `"Long ..."` |
| `center` | 2 | Center-align | `center "Hi" 10` | `"    Hi    "` |

**Boolean Format Styles:**
- `"yes/no"` -> Yes/No
- `"on/off"` -> On/Off
- `"enabled"` -> Enabled/Disabled
- `"active"` -> Active/Inactive
- `"success"` -> Success/Failure
- `"1/0"` -> 1/0

Examples:
```avon
# Number bases
format_hex 255           # "ff"
format_binary 15         # "1111"

# Human-readable
format_bytes 1048576     # "1.00 MB"
format_currency 99.95 "$" # "$99.95"
format_percent 0.75 1    # "75.0%"

# Collections
format_list ["apple", "banana", "cherry"] ", "
# "apple, banana, cherry"

format_table [["Name", "Age"], ["Alice", "30"], ["Bob", "25"]] " | "
# "Name | Age\nAlice | 30\nBob | 25"

# format_table also works directly with dicts:
let data = {name: "Alice", age: 30, city: "NYC"} in
format_table data " | "
# Or with dict literal directly:
format_table {a: 1, b: 2} " | "
# "age | city | name\n30 | NYC | Alice"
# Note: Key order in dicts is not guaranteed

# For more advanced dict-to-table patterns, see examples/dict_to_table.av

# Text formatting
format_bool (age > 18) "yes/no"  # "Yes" or "No"
truncate "Very long text here" 10  # "Very lo..."
center "Title" 20                  # "       Title        "
```

**Full Demo:** `examples/formatting_demo.av`

### Data & Utilities

| Function | Arity | Purpose | Example |
|----------|-------|---------|---------|
| `import` | 1 | Load another `.av` file | `import "lib.av"` |
| `json_parse` | 1 | Parse JSON (objects -> dicts, arrays -> lists) | `json_parse "{\"x\": 1}"` -> `{x: 1}` |
| `os` | 0 | Get operating system | `os` -> `"linux"`, `"windows"`, `"macos"` |

Note: `json_parse` converts JSON objects to Dict types (e.g., `{"a": 1}` -> `{a: 1}`), which support dot notation access like `data.a` and functions like `keys`, `values`, `has_key`, etc.

**Examples:** `examples/import_example.av`, `examples/json_map_demo.av`

---

## Operators

### Arithmetic
- `+` Addition (numbers)
- `-` Subtraction
- `*` Multiplication
- `/` Division

### String Concatenation
```avon
"hello" + " " + "world"    # Result: "hello world"
```

### List Concatenation
```avon
[1, 2] + [3, 4]            # Result: [1, 2, 3, 4]
```

### Template Concatenation
Templates can be combined with the `+` operator:
```avon
let greeting = {"Hello "} in
let name_part = {"World!"} in
greeting + name_part       # Result: "Hello World!"

# With interpolation
let name = "Alice" in
let t1 = {"Hello, {name}"} in
let t2 = {"!"} in
t1 + t2                    # Result: "Hello, Alice!"
```

### Path Concatenation
Paths can be combined with the `+` operator to build composite paths:

```avon
# Basic path concatenation
@config + @app.json              # → "config/app.json"
@home/user + @projects           # → "home/user/projects"

# With variables
let base = @config in
let file = @app.json in
base + file                       # → "config/app.json"

# Path interpolation + concatenation
let env = "prod" in
let path1 = @config/{env} in
let path2 = @app.conf in
path1 + path2                     # → "config/prod/app.conf"
```

**Automatic separator handling:**
- Adds `/` separator automatically: `@a + @b` → `"a/b"`
- No double slashes: `@a/ + @b` → `"a/b"` (not `"a//b"`)

**Important restrictions:**
- **Path literals must be relative:** `@config/file` ✅, `@/etc/file` ❌ (syntax error)
- Absolute paths are blocked to prevent unsafe deployment
- For absolute paths in reading operations, use strings: `readfile "/absolute/path"`
- Use `--root` flag for deployment to absolute locations

**Alternative: Path interpolation**
```avon
# Instead of concatenation, you can use interpolation
let user = "alice" in
@home/{user}/projects             # ✓ "home/alice/projects"

# Or combine string pieces
let base = "home/alice" in
let leaf = "projects" in
@{base}/{leaf}                    # ✓ "home/alice/projects"
```

**Examples:** `examples/template_path_concat.av`, `examples/neovim_config.av`
- `==` Equal
- `!=` Not equal
- `>` Greater than
- `<` Less than
- `>=` Greater or equal
- `<=` Less or equal

### Pipe Operator
The pipe operator `->` allows you to chain expressions without nested parentheses. It passes the result of the left-hand side as the **first argument** to the function on the right-hand side.

```avon
# Simple chaining
[1, 2, 3] -> length                    # Result: 3

# Chaining multiple operations
"hello" -> upper -> length             # Result: 5

# Chaining with curried functions (filter takes 2 args, here it gets the second from the pipe)
[1, 2, 3, 4, 5] -> filter (\x x > 2) -> length  # Result: 3
```

**Supported Patterns:**
1. **Standard functions:** `value -> func` (equivalent to `func value`)
2. **Curried functions:** `value -> func arg1` (equivalent to `func arg1 value`)
3. **Lambda expressions:** `value -> \x x * 2` (equivalent to `(\x x * 2) value`)
4. **Path literals:** `@path -> exists` (equivalent to `exists @path`)

Examples:
```avon
# Lambda on RHS
10 -> \x x * 2  # Result: 20

# Path on LHS
@config.json -> exists  # Result: true (if file exists)
```

---

## CLI Commands

### Evaluate a Program
```bash
avon eval examples/map_example.av
```
Runs the program and prints the result.

### Deploy Files
```bash
avon deploy examples/site_generator.av --root ./website --force
```

Deploy the program, generate files in the specified directory.

**Flags:**
- `--root <dir>` — Prepend to all generated paths
- `--force` — Overwrite existing files without warning (use with caution, or reckless abandon)
- `--append` — Append to existing files instead of overwriting
- `--if-not-exists` — Only write file if it doesn't already exist
- `-param value` — Named argument (e.g., `-name Alice`)
- **Default**: Files are NOT overwritten; a clear warning is shown instead

### Quick Eval from Command Line
```bash
avon run 'map (\x x * 2) [1, 2, 3]'
avon run 'typeof 42'
```

Evaluate code directly without writing a file.

### Debugging & Documentation
```bash
# Show debug output from lexer, parser, and evaluator
avon eval program.av --debug

# Show all builtin function documentation
avon doc
```

The `--debug` flag shows detailed tokenization, parsing, and evaluation steps, helpful for troubleshooting complex programs.

### Fetch from GitHub

**Deploy from GitHub** (automatic deployment):
```bash
avon deploy --git pyrotek45/avon/examples/site_generator.av --root ./out
```

**Evaluate from GitHub** (just run and print):
```bash
avon eval --git pyrotek45/avon/examples/string_functions.av
```

Fetch and run programs directly from GitHub's raw content CDN.

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
@config.yml {"
    app: {app}
    debug: false
"}
```

### Generate Files for Multiple Items
```avon
let items = ["dev", "staging", "prod"] in
map (\item @config-{item}.yml {"{item}"}) items
```

### Complex Configuration with Conditionals
```avon
let env = "prod" in
@.env {"
    DEBUG={if env == "prod" then "false" else "true"}
    CACHE_ENABLED={if env == "dev" then "false" else "true"}
"}
```

### Transform and Filter Data
```avon
let names = ["Alice", "Bob", "Charlie"] in
let formatted = map (\n upper n) names in
let long_names = filter (\n (length n) > 3) formatted in
join long_names ", "
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
Always test with `eval` before deploying. Trust, but verify. Actually, just verify. Trust is for people who don't write tests.
```bash
avon eval program.av             # Check output first
avon deploy program.av --force   # Then generate files
```

### 3. Template Indentation for Readability
Use indentation in templates — Avon's dedent removes it based on the first line's indentation:

```avon
@config.yml {"
    server:
      host: localhost    # There's no place like 127.0.0.1
      port: 8080
"}
```

The 4 spaces in the first line become the baseline and are removed from all lines.

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

For most use cases (100s of files), Avon runs in milliseconds. For pathological cases (1000s of files), it still runs in milliseconds. We tested. We have no life.

---

## See Also

- [TUTORIAL.md](./TUTORIAL.md) — Learn Avon from scratch (start here!)
- [TEMPLATE_SYNTAX.md](./TEMPLATE_SYNTAX.md) — Multi-brace delimiters and literal braces
- [SIMPLE_CONFIGS.md](./SIMPLE_CONFIGS.md) — Quick examples for common configs
- [STYLE_GUIDE.md](./STYLE_GUIDE.md) — Best practices and conventions
- [DEBUGGING_GUIDE.md](./DEBUGGING_GUIDE.md) — When things go wrong
- [REPL_USAGE.md](./REPL_USAGE.md) — Interactive development
- [SECURITY.md](./SECURITY.md) — Security model and sandboxing

Have fun generating!
