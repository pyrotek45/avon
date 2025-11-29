# Template Syntax Guide

This document provides a comprehensive guide to Avon's template syntax, including the multi-brace delimiter system for handling literal braces.

## Table of Contents

1. [Basic Template Syntax](#basic-template-syntax)
2. [Automatic Dedent](#automatic-dedent)
3. [Interpolation](#interpolation)
4. [The Multi-Brace Delimiter System](#the-multi-brace-delimiter-system)
5. [Handling Literal Braces](#handling-literal-braces)
6. [Common Use Cases](#common-use-cases)
7. [Best Practices](#best-practices)
8. [Quick Reference](#quick-reference)
9. [Tips and Tricks](#tips-and-tricks)

---

## Basic Template Syntax

Templates in Avon are enclosed in `{" "}`:

```avon
{"Hello, world!"}
# Output: Hello, world!
```

The opening delimiter is `{"` and the closing delimiter is `"}`.

**Tip:** Keep templates on the same line as `in` for clarity:

```avon
# Preferred
let name = "Alice" in @greeting.txt {"
  Hello, {name}!
"}

# Works, but less clear
let name = "Alice" in
@greeting.txt {"
Hello, {name}!
"}
```

---

## Automatic Dedent

Avon automatically removes leading whitespace from templates based on the **first line's indentation** (the baseline). This lets you indent templates in source code for readability without affecting the output.

### How It Works

1. Avon strips leading and trailing blank lines
2. Finds the first line with content—that line's indentation becomes the **baseline**
3. Removes that baseline amount of whitespace from every line
4. Relative indentation between lines is preserved

### Example

```avon
let make_config = \service @config/{service}.yml {"
  service: {service}
  settings:
    enabled: true
    timeout: 30
"}
```

The first content line (`service: {service}`) has 2 spaces, so 2 spaces are removed from every line.

**Output** (no leading spaces):
```yaml
service: myapp
settings:
  enabled: true
  timeout: 30
```

### Indent for Readability

You can indent templates as deep as you want in your source code:

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

### Blank Lines Are Stripped

Leading and trailing blank lines inside templates are automatically removed:

```avon
@config.json {{"

  {
    "key": "value"
  }

"}}
```

Output (blank lines removed):
```json
{
  "key": "value"
}
```

---

## Interpolation

Use `{expression}` inside a template to interpolate values:

```avon
let name = "Alice" in
{"Hello, {name}!"}
# Output: Hello, Alice!
```

Any valid Avon expression can be interpolated:

```avon
let x = 5 in
{"The value is {x} and doubled is {x * 2}"}
# Output: The value is 5 and doubled is 10
```

---

## The Multi-Brace Delimiter System

Avon uses a clever system for handling literal braces: **increase the number of braces in the delimiter to make fewer braces literal inside**.

### Single-Brace Delimiter: `{" "}`

- Opening: `{"`
- Closing: `"}`
- Interpolation: `{expr}`
- **Single `{` starts interpolation** (requires matching `}`)

```avon
let x = 5 in {"Value: {x}"}
# Output: Value: 5
```

### Double-Brace Delimiter: `{{" "}}`

- Opening: `{{"`
- Closing: `"}}`
- Interpolation: `{{expr}}`
- **Single `{` and `}` are literal**

```avon
let x = 5 in {{"Value: {{x}} and literal {braces}"}}
# Output: Value: 5 and literal {braces}
```

### Triple-Brace Delimiter: `{{{" "}}}`

- Opening: `{{{"`
- Closing: `"}}}`
- Interpolation: `{{{expr}}}`
- **Single `{` and `{{` are literal**

```avon
let x = 5 in {{{"Value: {{{x}}} and {{literal}} and {also literal}"}}}
# Output: Value: 5 and {{literal}} and {also literal}
```

### The Pattern

| Delimiter | Interpolation | Literal |
|-----------|---------------|---------|
| `{" "}` | `{x}` | (none without escaping) |
| `{{" "}}` | `{{x}}` | `{` and `}` |
| `{{{" "}}}` | `{{{x}}}` | `{`, `{{`, `}`, `}}` |
| `{{{{" "}}}}` | `{{{{x}}}}` | `{`, `{{`, `{{{`, `}`, `}}`, `}}}` |

**Rule:** With N braces in the delimiter, any sequence of fewer than N braces is literal.

---

## Handling Literal Braces

### Generating JSON

For JSON output, use the double-brace delimiter:

```avon
let name = "Alice" in
let age = 30 in
{{"
{
  "name": "{{name}}",
  "age": {{age}}
}
"}}
```

Output:
```json
{
  "name": "Alice",
  "age": 30
}
```

### Generating Code with Braces

For languages like JavaScript, C, or Rust that use braces:

```avon
let className = "MyClass" in
let fieldName = "value" in
{{"
class {{className}} {
  constructor() {
    this.{{fieldName}} = 0;
  }
}
"}}
```

Output:
```javascript
class MyClass {
  constructor() {
    this.value = 0;
  }
}
```

### Generating Templates (Meta-Templates)

When generating Avon templates or other template languages, use triple-brace templates so single and double braces are literal:

```avon
let varName = "username" in
let placeholder = "{" + varName + "}" in
{{{"
# This Avon template uses {{{varName}}}:
{"Hello, {{{placeholder}}}!"}
"}}}
```

Output:
```avon
# This Avon template uses username:
{"Hello, {username}!"}
```

**How it works:**
- `{{{varName}}}` interpolates the variable → `username`
- `{{{placeholder}}}` interpolates the string `"{username}"` → `{username}`
- `{"Hello, ... !"}` stays literal (single braces < triple)

**Simpler case** - when you just need literal template syntax without dynamic placeholders:

```avon
{{{"
# A static Avon template example:
{"Hello, {name}!"}
"}}}
```

Output:
```avon
# A static Avon template example:
{"Hello, {name}!"}
```

---

## Common Use Cases

### Docker Compose

```avon
let service = "web" in
let port = 8080 in
{{"
services:
  {{service}}:
    build: .
    ports:
      - "{{port}}:{{port}}"
    environment:
      NODE_ENV: production
"}}
```

### Kubernetes ConfigMap

```avon
let appName = "myapp" in
let configData = "key=value" in
{{"
apiVersion: v1
kind: ConfigMap
metadata:
  name: {{appName}}-config
data:
  config.properties: |
    {{configData}}
"}}
```

### GitHub Actions

```avon
let jobName = "build" in
{{"
jobs:
  {{jobName}}:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm install
"}}
```

### React/JSX Components

JSX uses single braces for expressions. Use a helper function to wrap interpolated values:

```avon
let componentName = "Greeting" in
let propName = "name" in
let jsx = \v "{" + v + "}" in

{{" 
function {{componentName}}({{jsx propName}}) {
  return <div>Hello, {{jsx propName}}!</div>;
}
"}}
# Output:
# function Greeting({name}) {
#   return <div>Hello, {name}!</div>;
# }
```

The `jsx` helper wraps values in single braces. Level 2 templates keep single braces literal for the parameter destructuring syntax `({ name })`.

---

## Best Practices

### 1. Choose the Right Delimiter Level

- **No braces in output?** Use single: `{" "}`
- **Single braces in output?** Use double: `{{" "}}`
- **Double braces in output?** Use triple: `{{{" "}}}`

### 2. Match Your Content

Think about what braces appear in your target format:

| Format | Recommended Delimiter |
|--------|----------------------|
| Plain text | `{" "}` |
| JSON, YAML with braces | `{{" "}}` |
| JavaScript, C, Rust code | `{{" "}}` |
| Lua, Nginx configs | `{{" "}}` |
| GitHub Actions YAML | `{{{" "}}}` |
| Avon meta-templates | `{{{" "}}}` |
| Mustache/Handlebars templates | `{{{" "}}}` |

### 3. Keep Templates Readable

For complex templates, use multiline format:

```avon
let config = {
  host: "localhost",
  port: 8080
} in
{{"
server {
  host = {{config.host}}
  port = {{config.port}}
}
"}}
```

---

## Quick Reference

### Delimiter Levels

| Level | Open | Close | Interpolate | Literal |
|-------|------|-------|-------------|---------|
| 1 | `{"` | `"}` | `{x}` | - |
| 2 | `{{"` | `"}}` | `{{x}}` | `{` `}` |
| 3 | `{{{"` | `"}}}` | `{{{x}}}` | `{` `{{` `}` `}}` |
| N | `{...{"` | `"}...}` | `{...{x}...}` | fewer than N braces |

### Examples at Each Level

**Level 1 - Plain text:**
```avon
let x = 5 in {"Value: {x}"}
# → Value: 5
```

**Level 2 - JSON/Code:**
```avon
let x = 5 in {{"{"value": {{x}}}"}}
# → {"value": 5}
```

**Level 3 - Meta-templates:**
```avon
let x = 5 in {{{"Template: {{{x}}} and {{literal}}"}}}
# → Template: 5 and {{literal}}
```

---

## Tips and Tricks

### Wrapping Interpolated Values in Braces

Sometimes you need the output to contain braces around an interpolated value, like `{value}`. Since braces at the same level as your delimiter are used for interpolation, use string concatenation or helper functions:

**Using String Concatenation:**
```avon
let x = "value" in
"{" + x + "}"
# Output: {value}
```

**Using a Template-Based Wrapper:**
```avon
let brace = \s {{"{ {{s}} }"}} in
brace "name"
# Output: { name }
```

**Using the `+` Operator (no spaces):**
```avon
let brace = \s "{" + s + "}" in
brace "name"
# Output: {name}
```

This pattern is useful for:
- Generating format strings: `{0}`, `{name}`
- Creating placeholder syntax: `{PLACEHOLDER}`
- Building URI templates: `/users/{id}`

**Create Reusable Wrapper Functions:**
```avon
# Wrapper for single braces (using + for no extra spaces)
let brace = \s "{" + s + "}" in

# Wrapper for double braces (Mustache/Handlebars)
let mustache = \s "{{" + s + "}}" in

# Wrapper for GitHub Actions expressions
let gh_expr = \s "${{" + s + "}}" in

# Wrapper for shell/bash variables
let shell_var = \v "$" + "{" + v + "}" in

# Usage
brace "name"        # {name}
mustache "value"    # {{value}}
gh_expr "github.ref"  # ${{github.ref}}
shell_var "PATH"    # ${PATH}
```

### Generating Shell/Bash Variables

For shell variable syntax like `${PATH}`:

```avon
let shell_var = \v "$" + "{" + v + "}" in
let var = "PATH" in
"echo " + (shell_var var)
# Output: echo ${PATH}
```

Note: Parentheses around `(shell_var var)` are needed because `+` binds tighter than function application.

### Generating Multiple Brace Styles

For complex output requiring different brace counts:

```avon
let single = \s "{" + s + "}" in
let double = \s "{{" + s + "}}" in

let field1 = "name" in
let field2 = "age" in
{{"
Single: "}} + (single field1) + {{"
Double: "}} + (double field2)
```

### Escape Sequences in Templates vs Strings

Regular strings process escape sequences like `\n` (newline), `\\` (backslash), and `\"` (quote):

```avon
"Line 1\nLine 2"
# Output:
# Line 1
# Line 2
```

Templates treat text literally—backslashes and other characters are preserved as-is:

```avon
{"Line 1\nLine 2"}
# Output: Line 1\nLine 2
```

If you need actual newlines in template output, use multiline templates:

```avon
{"Line 1
Line 2"}
# Output:
# Line 1
# Line 2
```

### Converting Non-String Values

When using `concat` or `+` for string operations, values must be strings. Use `to_string` to convert numbers:

```avon
let count = 42 in
"{" + (to_string count) + "}"
# Output: {42}

# This would fail:
# "{" + count + "}"  # Error: type mismatch
```

### Mixing Template Levels

When you need different brace styles in one output, use helper functions and multi-brace templates together:

```avon
# Create a JSON template with a Mustache placeholder
let name = "user" in
let mustache = \s "{{" + s + "}}" in
let placeholder = mustache "data" in

# Use level 2 for the JSON structure, interpolate the placeholder
{{" {"name": "{{name}}", "template": "{{placeholder}}"} "}}
# Output: {"name": "user", "template": "{{data}}"}
```

The key insight: level 2 templates treat single braces `{}` as literal, so JSON is easy. The `mustache` helper builds `{{...}}` strings, which are then interpolated into the level 2 template.

### What Gets Interpolated

Most Avon values can be interpolated in templates:

| Value Type | Interpolation Result |
|------------|---------------------|
| String | Content as-is |
| Number | String representation |
| Boolean | `true` or `false` |
| List | Elements on separate lines |
| Dict | Dictionary notation |
| Template | Evaluated template content |
| None | `None` |
| Function | `<function>` |

Examples:

```avon
let list = [1, 2, 3] in {"items: {list}"}
# Output:
# items: 1
# 2
# 3

let d = { name: "Alice" } in {"data: {d}"}
# Output: data: {name: "Alice"}

let nl = "\n" in {"line1{nl}line2"}
# Output:
# line1
# line2
```

### Common Patterns Library

Here's a collection of helper functions for common output patterns:

```avon
# Single braces (compact)
let brace = \s "{" + s + "}" in

# Double braces (Mustache/Handlebars)
let mustache = \s "{{" + s + "}}" in

# Triple braces (raw Mustache)
let mustache_raw = \s "{{{" + s + "}}}" in

# Shell/Bash variables
let shell_var = \v "$" + "{" + v + "}" in

# GitHub Actions expressions
let gh_expr = \s "${{" + s + "}}" in

# Jinja2 expressions (with spaces)
let jinja = \s "{{ " + s + " }}" in

# ERB/Ruby
let erb = \s "<%= " + s + " %>" in

# Usage
brace "name"           # {name}
mustache "value"       # {{value}}
shell_var "PATH"       # ${PATH}
gh_expr "github.ref"   # ${{github.ref}}
jinja "variable"       # {{ variable }}
```

**Template-based wrappers (when you want the visual spacing):**
```avon
# These include spaces around the interpolated value
let brace_spaced = \s {{"{ {{s}} }"}} in
let mustache_spaced = \s {{{"{{ {{{s}}} }}"}}} in

brace_spaced "name"      # { name }
mustache_spaced "value"  # {{ value }}
```

---

## See Also

- [TUTORIAL.md](./TUTORIAL.md) — Learn Avon from scratch
- [FEATURES.md](./FEATURES.md) — Complete language reference
- [BUILDING_CONTENTS.md](./BUILDING_CONTENTS.md) — Building file contents
- [STYLE_GUIDE.md](./STYLE_GUIDE.md) — Best practices and conventions
- [DEBUGGING_GUIDE.md](./DEBUGGING_GUIDE.md) — When things go wrong
