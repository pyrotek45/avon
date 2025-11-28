# Avon Style Guide

This guide describes the recommended formatting conventions for writing Avon code. You don't have to follow these, but your teammates (and future you) will appreciate it if you do.

> Tip: Sharing Templates:** If you plan to share your templates via the `--git` flag (highly recommended!), use function parameters with defaults (`\param ? "default"`) instead of top-level `let` bindings. This makes templates flexible and easy to customize when deployed from GitHub. See [SIMPLE_CONFIGS.md](./SIMPLE_CONFIGS.md) for examples.

## Core Principles

1. **Readability First** — Code should be easy to read and understand
2. **Consistent Formatting** — Follow these conventions across all Avon programs
3. **Leverage Dedent** — Avon automatically dedents templates based on the first line's indentation (baseline)

---

## Template Formatting

### Template on Same Line as `in`

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

**Reason:** Keeping the template on the same line as `in` makes the relationship clear, though the parser does allow them on separate lines.

### Indent Template Content (Leverage Automatic Dedent)

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

Why this matters:** 
- Avon's **automatic dedent** removes the common leading whitespace from all template lines
- This means you can indent your template content in source code for readability without affecting the generated output
- The generated file will have correct formatting: no extra leading spaces
- This makes nested templates and indented Avon code much more readable

### Indentation Amount

Avon's automatic dedent means you have flexibility in how you indent source code—just maintain consistency within your project.

- **Avon code:** 2 spaces per indentation level (recommended)
- **Template content indentation in source:** Match your Avon code indentation
- **Output format:** Match the target file's conventions
  - YAML: 2 spaces
  - JSON: 2 spaces
  - Code (Lua, JS, etc.): 2-4 spaces (match target style)
  - Configs: Match existing style

How Dedent Works:**
1. Avon strips leading and trailing blank lines
2. Finds the first line with non-whitespace content—that line's indentation becomes the **baseline**
3. Removes that baseline amount of whitespace from every line
4. Lines with less indentation than baseline are kept as-is
5. Blank lines become empty

This lets you indent your template in source code without padding your output.

Example:
```avon
# Avon code is indented 2 spaces inside the function
let make_config = \service @config/{service}.yml {"
  # This line has 2 spaces: it becomes the baseline
  # Dedent will remove 2 spaces from every line
  service: {service}
  settings:
    # Relative indentation is preserved: this line still has 4 more spaces
    enabled: true
    timeout: 30
"}
```

**Output** (after dedent removes 2 common spaces—the baseline):
```yaml
service: auth
settings:
  enabled: true
  timeout: 30
```

**Pro tip:** The first non-whitespace character determines your baseline. Indent as much as you need in your source code—dedent always uses the first content line's indentation as the reference point.

---

## Leveraging Automatic Dedent

Avon's automatic dedent is a powerful feature that lets you write readable, well-indented source code without worrying about padding your generated output.

### How It Works

Avon finds the **first line with non-whitespace content** and uses its indentation level as the baseline. Every line has that baseline amount removed from its leading whitespace. Blank lines and lines with less indentation than the baseline are preserved as-is.

### Key Dedent Behaviors

1. **Baseline = first non-whitespace line's indentation**
2. **Strips leading and trailing blank lines** automatically
3. **Removes baseline indentation from all lines** (if they have it)
4. **Preserves relative indentation** between lines
5. **Works with mixed tabs and spaces** (though spaces are recommended—we have standards here)

### Best Practices

**Indent for code clarity:**
```avon
let make_yaml = \name @config.yml {"
  app:
    name: {name}
    database:
      host: localhost
      port: 5432
"}
```

Output has no leading spaces:
```yaml
app:
  name: myapp
  database:
    host: localhost
    port: 5432
```

**Deeply nested templates stay readable:**
```avon
let environments = ["dev", "staging"] in
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

Even though the template is 4+ levels deep in the source, the generated files are properly formatted with zero leading spaces.

**Use blank lines for readability (they're automatically stripped):**
```avon
@config.json {{"

  {
    "database": {
      "host": "localhost"
    }
  }

"}}
```

Output (blank lines removed, properly formatted):
```json
{
  "database": {
    "host": "localhost"
  }
}
```

**Don't worry about indentation affecting output—it won't:**
```avon
# This produces identical output to the examples above
@config.yml {"
                  server: localhost
                  port: 8080
"}
```

The first line `server: localhost` (with 18 spaces) becomes the baseline, so 18 spaces are removed from every line, producing zero leading spaces in the output.

**Practical example showing baseline selection:**
```avon
# 8-space indent in source, first line has 8 spaces = baseline
@file.txt {"
        line1
        line2
            line3
        line4
"}
```

Output (8 spaces removed from every line):
```
line1
line2
    line3
line4
```

Notice that `line3` keeps its extra 4-space indentation (12 - 8 = 4 spaces remain).

---

## Template Delimiter Strategy

When generating code or configs that contain curly braces, Avon's variable-brace delimiter system lets you choose the right level for your content. The key is matching your template delimiter to your brace density.

### Single-Brace Templates (Minimal Escaping Needed)

Use `{" ... "}` when your output has **few or no literal braces**:

```avon
@app.yml {"
app:
  name: myapp
  port: { port }
"}
```

**Use for:** YAML, INI, plain text, simple configs
- No escaping needed for most content
- Interpolation uses `{ expr }`

### Double-Brace Templates (Heavy Brace Usage)

Use `{{"  ... "}}` when your output has **many literal braces** (JSON, HCL, Terraform, Lua, etc.):

```avon
@nginx.conf {{"
server {
  listen 80;
  server_name {{ domain }};
  location / {
    proxy_pass http://{{ upstream }};
  }
}
"}}
```

**Advantages:**
- Single braces `{` and `}` are literal (no escaping!)
- Interpolation uses `{{ expr }}`
- Much cleaner for brace-heavy formats

**Use for:** JSON, HCL, Terraform, Lua, Nginx, shell scripts, Python code

**Example: JSON Config**
```avon
let db_host = "localhost" in
let db_port = 5432 in
@config.json {{"
{
  "database": {
    "host": "{{ db_host }}",
    "port": {{ db_port }}
  }
}
"}}
```

**Example: Terraform**
```avon
let ami_id = "ami-123456" in
@main.tf {{"
resource "aws_instance" "web" {
  ami = "{{ ami_id }}"
  tags = {
    Name = "web-server"
  }
}
"}}
```

### Decision Table: When to Use Each

| Output Format | Delimiter | Reason |
|-------------|-----------|--------|
| YAML, INI files | `{" "}` | Few braces, no escaping |
| Nginx, shell | `{{"  "}}` | Multiple blocks/braces |
| JSON, HCL, Terraform | `{{"  "}}` | Many structured braces |
| Python code | `{{"  "}}` | Dict literals and f-strings |
| Extreme brace density | `{{{" "}}}` | Triple-brace (rare) |

**Pro tip:** Choose the delimiter that keeps your template readable. More braces in the output? Use more braces in the delimiter!

---

## Let Bindings

### One Binding Per Line

**Preferred:**
```avon
let port = 8080 in
let host = "localhost" in
let url = {"http://{host}:{port}"} in
url
```

**Avoid:**
```avon
let port = 8080 in let host = "localhost" in let url = {"http://{host}:{port}"} in url
```

### Cascading Lets for Readability

Break complex logic into named steps:

**Preferred:**
```avon
let services = ["api", "web", "worker"] in
let make_config = \svc @config-{svc}.yml {"
  service: {svc}
"} in
map make_config services
```

**Avoid:**
```avon
map (\svc @config-{svc}.yml {"service: {svc}"}) ["api", "web", "worker"]
```

---

## Function Definitions

### Lambda Functions

**Single Parameter:**
```avon
let double = \x x * 2 in
map double [1, 2, 3]
```

**Multiple Parameters (Curried):**
```avon
let make_url = \protocol \host \port {"{protocol}://{host}:{port}"} in
make_url "https" "example.com" "443"
```

### Default Parameters

Use `?` for parameters with defaults:

```avon
let greet = \name ? "Guest" @greeting.txt {"
  Hello, {name}!
"} in
greet "Alice"
```

### Function Naming

Use `snake_case` for function names:

```avon
let make_kubernetes_manifest = \service \env @k8s/{env}/{service}.yaml {"
  ...
"} in
```

---

## Lists and Collections

### Short Lists on One Line

```avon
let colors = ["red", "green", "blue"] in
```

### Long Lists on Multiple Lines

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

### List Operations

```avon
let numbers = [1, 2, 3, 4, 5] in
let doubled = map (\x x * 2) numbers in
let evens = filter (\x (x % 2) == 0) doubled in
evens
```

---

## Comments

Use `#` for comments. Place them above the code they describe:

```avon
# Generate configuration for each environment
# (because someone thought 3 environments wasn't enough)
let environments = ["dev", "staging", "prod"] in

# Create a config file for each environment
let make_config = \env @config-{env}.yml {"
  environment: {env}
  debug: {if env == "prod" then "false" else "true"}
"} in

map make_config environments
```

---

## Complete Example

This example demonstrates all style guidelines:

```avon
# Multi-environment Kubernetes deployment generator
# Generates deployment manifests for multiple services across environments

let services = [
  "auth",
  "api",
  "frontend"
] in

let environments = ["dev", "staging", "prod"] in

# Create a Kubernetes deployment manifest
let make_k8s_manifest = \service \env @k8s/{env}/{service}-deployment.yaml {"
  apiVersion: apps/v1
  kind: Deployment
  metadata:
    name: {service}
    namespace: {env}
    labels:
      app: {service}
      environment: {env}
  spec:
    replicas: {if env == "prod" then "3" else "1"}
    selector:
      matchLabels:
        app: {service}
    template:
      metadata:
        labels:
          app: {service}
      spec:
        containers:
        - name: {service}
          image: mycompany/{service}:latest
          ports:
          - containerPort: 8080
          env:
          - name: ENVIRONMENT
            value: {env}
          - name: LOG_LEVEL
            value: {if env == "prod" then "warn" else "debug"}
          resources:
            requests:
              memory: "{if env == "prod" then "512Mi" else "256Mi"}"
              cpu: "{if env == "prod" then "500m" else "250m"}"
            limits:
              memory: "{if env == "prod" then "1Gi" else "512Mi"}"
              cpu: "{if env == "prod" then "1000m" else "500m"}"
"} in

# Generate all manifests: service x environment combinations
flatmap (\env map (\svc make_k8s_manifest svc env) services) environments
```

---

## Quick Checklist

Before committing your Avon code, verify:

- [ ] Templates start with `{"` on same line as path
- [ ] Template content is indented for source code readability (dedent will clean output)
- [ ] No manual indentation compensation needed—rely on automatic dedent
- [ ] Let bindings are on separate lines
- [ ] Functions use `snake_case` naming
- [ ] Complex logic is broken into named steps
- [ ] Comments explain the "why", not the "what"
- [ ] Code is formatted consistently (2-space indentation recommended)

---

## Template Auto-Conversion

Avon automatically converts templates to strings when used with string functions. This means you don't need to call `to_string` explicitly:

**Preferred:**
```avon
let template = "Hello <!-- title -->" in
let title = {"My Site"} in
let html = replace template "<!-- title -->" title in  # Template auto-converts
html
```

**Avoid:**
```avon
let template = "Hello <!-- title -->" in
let title = {"My Site"} in
let html = replace template "<!-- title -->" (to_string title) in  # Unnecessary - replace auto-converts templates
html
```

**String functions that accept templates (auto-convert to string):**
- `concat` - concatenates two strings/templates (takes exactly 2 arguments)
- `upper`, `lower`, `trim` - case and whitespace operations
- `split`, `join`, `replace` - string manipulation
- `contains`, `starts_with`, `ends_with` - string predicates
- `repeat`, `pad_left`, `pad_right`, `indent` - formatting
- All string predicate functions (`is_digit`, `is_alpha`, `is_alphanumeric`, etc.)

**Example showing auto-conversion:**
```avon
let title = {"My Site"} in
let header = concat "<h1>" title in  # title auto-converts, no to_string needed
let full = concat header "</h1>" in  # chain concat calls for multiple parts
let upper_title = upper title in     # Also auto-converts
full
```

## Pipe Operator

Use the pipe operator `->` for chaining operations:

**Preferred:**
```avon
let result = [1, 2, 3, 4, 5]
  -> filter (\x x > 2)
  -> map (\x x * 2)
  -> fold (\acc \x acc + x) 0
in
result
```

**Avoid:**
```avon
fold (\acc \x acc + x) 0 (map (\x x * 2) (filter (\x x > 2) [1, 2, 3, 4, 5]))
```

Note: Only `->` is a valid pipe operator. The single `|` character is not a pipe operator in Avon.

## Error Handling Patterns

### Use `assert` for Validation

**Preferred:**
```avon
let port = to_int port_str in
assert (port > 0 && port < 65536) port
```

### Use `env_var_or` for Safe Defaults

**Preferred:**
```avon
let log_level = env_var_or "LOG_LEVEL" "info" in
```

### Validate File Existence

**Preferred:**
```avon
let config_path = "config.yml" in
assert (exists config_path) config_path in
readfile config_path
```

## Best Practices Summary

1. **Use templates for all string content** - They support interpolation and auto-convert in string functions
2. **Leverage automatic dedent** - Indent templates in source code for readability
3. **Choose the right template delimiter** - Single braces for YAML, double braces for JSON/Terraform
4. **Break complex logic into named steps** - Use cascading `let` bindings
5. **Use pipe operator for chaining** - Makes data transformations readable
6. **Validate inputs with `assert`** - Catch errors early
7. **Use default parameters** - Make functions flexible with `?` syntax
8. **Comment the "why"** - Explain design decisions, not obvious code. Future you doesn't remember what past you was thinking. Past you was probably caffeinated anyway.

## Additional Resources

- **[FEATURES.md](./FEATURES.md)** — Complete language reference
- **[TUTORIAL.md](./TUTORIAL.md)** — Learn Avon step-by-step
- **[DEBUGGING_GUIDE.md](./DEBUGGING_GUIDE.md)** — Debugging tools and techniques
- **[examples/](../examples/)** — 92+ real-world examples

Happy coding!

<!-- 
Style is subjective. These are guidelines, not laws. 
If your code works, ships, and doesn't wake anyone up at night, you're doing fine.
If it does wake people up at night, maybe read this guide again.
-->
