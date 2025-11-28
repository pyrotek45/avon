# Template Syntax Guide

This document provides a comprehensive guide to Avon's template syntax, including the multi-brace delimiter system for handling literal braces.

## Table of Contents

1. [Basic Template Syntax](#basic-template-syntax)
2. [Interpolation](#interpolation)
3. [The Multi-Brace Delimiter System](#the-multi-brace-delimiter-system)
4. [Handling Literal Braces](#handling-literal-braces)
5. [Common Use Cases](#common-use-cases)
6. [Best Practices](#best-practices)
7. [Quick Reference](#quick-reference)
8. [Tips and Tricks](#tips-and-tricks)

---

## Basic Template Syntax

Templates in Avon are enclosed in `{" "}`:

```avon
{"Hello, world!"}
# Output: Hello, world!
```

The opening delimiter is `{"` and the closing delimiter is `"}`.

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

When generating Avon templates or other template languages:

```avon
let varName = "username" in
{{{"
# This Avon template will interpolate {{varName}}:
{"Hello, {{{varName}}}!"}
"}}}
```

Output:
```avon
# This Avon template will interpolate username:
{"Hello, {username}!"}
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

For JSX with JavaScript expressions:

```avon
let componentName = "Greeting" in
let propName = "name" in
{{{"
function {{{componentName}}}({ {{{propName}}} }) {
  return <div>Hello, {{propName}}!</div>;
}
"}}}
```

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

When using `concat` or `+` for string operations, values must be strings. Use `str` to convert numbers:

```avon
let count = 42 in
"{" + str count + "}"
# Output: {42}

# This would fail:
# "{" + count + "}"  # Error: type mismatch
```

### Mixing Template Levels

When you need different brace styles in one output, combine templates with concatenation:

```avon
let name = "user" in
let value = "data" in

# JSON with a Mustache placeholder inside
{{"{"}} + {{"\"name\": \"{{name}}\", \"template\": \""}} + "{{" + value + "}}" + {{"\"}"}}
# Output: {"name": "user", "template": "{{data}}"}
```

A cleaner approach uses helper functions:

```avon
let mustache = \s "{{" + s + "}}" in
let name = "user" in
let value = "data" in

{{"{"name": "{{name}}", "template": ""}} + mustache value + {{"\"}}"}}
```

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

- [TUTORIAL.md](./TUTORIAL.md) - Main Avon tutorial
- [STYLE_GUIDE.md](./STYLE_GUIDE.md) - Code style recommendations
- [BUILDING_CONTENTS.md](./BUILDING_CONTENTS.md) - Building file contents
