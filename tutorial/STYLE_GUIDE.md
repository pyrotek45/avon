# Avon Style Guide

This guide describes the recommended formatting conventions for writing Avon code.

## Core Principles

1. **Readability First** — Code should be easy to read and understand
2. **Consistent Formatting** — Follow these conventions across all Avon programs
3. **Leverage Dedent** — Avon automatically removes common leading whitespace from templates

---

## Template Formatting

### ✅ Template on Same Line as `in`

**Preferred:**
```avon
let name = "Alice" in @/greeting.txt {"
  Hello, {name}!
  Welcome to Avon.
"}
```

**Avoid:**
```avon
let name = "Alice" in
@/greeting.txt {"
Hello, {name}!
"}
```

**Reason:** The template opening `{"` must appear on the same line as the path `@/file`. This is a syntax requirement in Avon.

### ✅ Indent Template Content

**Preferred:**
```avon
@/config.yml {"
  server:
    host: localhost
    port: 8080
  database:
    name: myapp
"}
```

**Avoid:**
```avon
@/config.yml {"
server:
  host: localhost
  port: 8080
database:
  name: myapp
"}
```

**Reason:** Indenting template content improves readability in your Avon source code. Avon automatically dedents the output, so the generated file will have correct formatting.

### Indentation Amount

- **Avon code:** 2 spaces per indentation level
- **Template content:** Match the target format's conventions
  - YAML: 2 spaces
  - JSON: 2 spaces
  - Code (Lua, JS, etc.): 2-4 spaces (match target style)
  - Configs: Match existing style

**Example:**
```avon
let make_config = \service @/config/{service}.yml {"
  # Avon uses 2-space indent
  service: {service}
  settings:
    # YAML content uses 2-space indent (YAML standard)
    enabled: true
    timeout: 30
"}
```

---

## Escaping Braces in Templates

When generating code or configs that contain literal `{` and `}` characters (like Nginx, Lua, HCL), use more curly braces around your templates.

```avon
@/nginx.conf {{"
  server {
    listen 80;
    server_name example.com;
    location / {
      proxy_pass http://localhost:8080;
    }
  }
"}}
```

**Output:**
```
server {
  listen 80;
  server_name example.com;
  location / {
    proxy_pass http://localhost:8080;
  }
}
```

### Double-Brace Templates (For Heavy Brace Usage)

When you have many literal braces, use double-brace templates `{{"..."}}`:

```avon
@/terraform.tf {{"
  resource "aws_instance" "web" {
    ami = "{{ "ami_id" }}"
    tags = {
      Name = "{{ "hello" }}"
    }
  }
"}}
```

---

## Let Bindings

### ✅ One Binding Per Line

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

### ✅ Cascading Lets for Readability

Break complex logic into named steps:

**Preferred:**
```avon
let services = ["api", "web", "worker"] in
let make_config = \svc @/config-{svc}.yml {"
  service: {svc}
"} in
map make_config services
```

**Avoid:**
```avon
map (\svc @/config-{svc}.yml {"service: {svc}"}) ["api", "web", "worker"]
```

---

## Function Definitions

### ✅ Lambda Functions

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

### ✅ Default Parameters

Use `?` for parameters with defaults:

```avon
let greet = \name ? "Guest" @/greeting.txt {"
  Hello, {name}!
"} in
greet "Alice"
```

### ✅ Function Naming

Use `snake_case` for function names:

```avon
let make_kubernetes_manifest = \service \env @/k8s/{env}/{service}.yaml {"
  ...
"} in
```

---

## Lists and Collections

### ✅ Short Lists on One Line

```avon
let colors = ["red", "green", "blue"] in
```

### ✅ Long Lists on Multiple Lines

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

### ✅ List Operations

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
let environments = ["dev", "staging", "prod"] in

# Create a config file for each environment
let make_config = \env @/config-{env}.yml {"
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
let make_k8s_manifest = \service \env @/k8s/{env}/{service}-deployment.yaml {"
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
- [ ] Template content is indented for readability
- [ ] Braces are escaped (`{{` `}}`) in configs with literal braces
- [ ] Let bindings are on separate lines
- [ ] Functions use `snake_case` naming
- [ ] Complex logic is broken into named steps
- [ ] Comments explain the "why", not the "what"
- [ ] Code is formatted consistently

---

## Additional Resources

- **[FEATURES.md](./FEATURES.md)** — Complete language reference
- **[TUTORIAL.md](./TUTORIAL.md)** — Learn Avon step-by-step
- **[examples/](../examples/)** — 77 real-world examples

Happy coding! 🚀
