# Avon

> Give your files superpowers

Avon is a functional language for generating and deploying any text file. Add variables, functions, and logic to YAML, JSON, configs, scripts, or any text format. Whether you're managing a single dotfile or generating thousands of Kubernetes manifests, Avon gives you the power to automate what you never thought possible.

**What makes Avon powerful:**
- **Language agnostic** — Transform any text format
- **Functional programming** — Variables, functions, map/filter/fold, type safety
- **Built-in deployment** — Files know where they belong
- **Git integration** — Share templates, deploy anywhere
- **Extensible** — Combine primitives in creative ways

Avon is designed to be powerful and flexible. We're excited to see how creative developers use it in ways we never imagined.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)

---

## Documentation

| Resource | Description |
|----------|-------------|
| [**Tutorial**](./tutorial/TUTORIAL.md) | Complete guide from basics to advanced patterns. Start here for language essentials, templates, CLI usage, style guide, best practices, error handling, and debugging. |
| [**Function Reference**](./tutorial/BUILTIN_FUNCTIONS.md) | Reference for all built-in functions with signatures, descriptions, and examples organized by category. |
| **Command Line** | Run `avon doc` for built-in help on any function |
| **Examples** | See `examples/` directory for 90+ real-world examples |

---

## Installation

**Build from source:**

```bash
git clone https://github.com/pyrotek45/avon
cd avon
cargo build --release
```

The binary will be at `target/release/avon`.

**Add to PATH (optional):**

```bash
sudo cp target/release/avon /usr/local/bin/
# Or add to PATH: export PATH="$PATH:$(pwd)/target/release"
```

---

## Quick Start

**Create your first Avon program (`hello.av`):**

```avon
# Variables are defined with let...in syntax
let name = "World" in
let greeting = "Hello" in

# This creates a FileTemplate - Avon's deployment unit
@hello.txt {"
    {greeting}, {name}!
    Welcome to Avon.
"}
```

**The three key types:**

| Type | Syntax | Description |
|------|--------|-------------|
| Path | `@hello.txt` | File destination (relative only). Can include `{variables}`. |
| Template | `{"..."}` | Multiline text content. Use `{name}` for variable interpolation. |
| FileTemplate | `@path {"..."}` | Path + Template. This is what Avon deploys. |

**Test without writing files:**

```bash
avon eval hello.av
```

**Deploy to disk:**

```bash
avon deploy hello.av --root ./output
```

Creates `./output/hello.txt` with the generated content.

---

## How Deployment Works

When you run `avon deploy program.av`, Avon evaluates your program and:

1. **Function result** — Applies CLI arguments to it
   - Program returns `\env \port ...`
   - Run `avon deploy program.av prod 8080`
   - Passes `prod` and `8080` as arguments

2. **FileTemplate result** — Writes it to disk
   - One `@path {"content"}` produces one file

3. **List of FileTemplates** — Writes them all
   - Generate multiple files from one program

**Example outputs:**

```avon
# Returns a function - CLI args are applied
\env @config-{env}.yml {"environment: {env}"}

# Returns a FileTemplate - deployed directly  
@config.yml {"port: 8080"}

# Returns a list of FileTemplates - all are deployed
[
  @config.yml {"port: 8080"},
  @settings.yml {"debug: true"}
]
```

**Core commands:**

```bash
avon eval program.av              # Preview output (no files written)
avon deploy program.av --root ./  # Write files to disk
avon run 'expr'                   # Evaluate expression directly
avon repl                         # Interactive exploration
avon doc                          # Built-in function reference
```

---

## Git Integration

Fetch and deploy templates directly from GitHub. Keep one Avon file in version control, deploy customized versions anywhere.

**Template in your repository (`configs.av`):**

```avon
\env ? "dev" \user ? "developer"

@config.yml {"
    user: {user}
    environment: {env}
    debug: {if env == "dev" then "true" else "false"}
"}
```

**Deploy with different settings on different machines:**

```bash
# Development laptop
avon deploy --git user/repo/configs.av --root ~/.config/myapp -env dev -user alice

# Production server  
avon deploy --git user/repo/configs.av --root /etc/myapp -env prod -user service

# Coworker's machine
avon deploy --git user/repo/configs.av --root ~/.config/myapp -env dev -user bob
```

One source of truth, infinite variations. Each deployment customized via CLI arguments.

---

## FileTemplates

Avon has first-class types for file paths and templates. The `@path {"content"}` syntax creates a FileTemplate—the unit Avon uses for deployment.

**Paths are values:**

```avon
let config_path = @config.yml in
config_path  # Returns: config.yml
```

**Templates are values:**

```avon
let content = {"
  name: myapp
  port: 8080
"} in
content  # Returns the template string
```

**Dynamic paths with interpolation:**

```avon
let env = "prod" in
@configs/{env}.yml {"port: 8080"}  # Path becomes configs/prod.yml
```

**Functions that return FileTemplates:**

```avon
# Function that returns a FileTemplate
let make_config = \env \port 
  @configs/{env}.yml {"
    environment: {env}
    port: {port}
  "} 
in

# Generate multiple FileTemplates with map
map (\env make_config env 8080) ["dev", "staging", "prod"]
# Result: 3 FileTemplates, one for each environment
```

FileTemplates are first-class values that can be stored in variables, returned from functions, collected in lists, and transformed with map/filter/fold.

---

## Key Features

### Deployment Syntax

Files know where they belong with `@` path syntax:

```avon
@path/to/file.yml {"content goes here"}
```

### Dictionaries with Dot Notation

First-class hash maps with convenient access:

```avon
let config = {host: "localhost", port: 8080} in
config.host  # Access with dots
```

### Pipe Operator

Chain expressions without nested parentheses:

```avon
[1, 2, 3, 4, 5] -> filter (\x x > 2) -> length
# Instead of: length (filter (\x x > 2) [1, 2, 3, 4, 5])
```

### Functional Programming

Variables, functions, map/filter/fold, conditionals, currying.

### Rich Standard Library

Built-in functions for string operations, list operations, formatting, date/time, JSON, file I/O, HTML/Markdown helpers, and more.

### Runtime Type Safety

Avon won't deploy if there's a type error. Catch issues before deployment.

### Any Text Format

Works with YAML, JSON, TOML, HCL, shell scripts, code, configs, docs, and dotfiles.

### Simple & Modular

Each file contains one expression. Import returns any Avon type:

```avon
let math = import "math_lib.av" in
let config = import "config.av" in
math.double 21  # Returns 42
```

### Git Integration

Fetch and deploy templates directly from GitHub:

```bash
avon deploy --git user/repo/config.av --root ~/.config -env prod
```

Run `avon doc` for the complete function reference.

---

## What Makes Avon Different

Avon integrates two systems that are usually separate:

1. **Functional Language** — Variables, functions, lists, conditionals, runtime type checking
2. **Deployment System** — `@path/to/file.yml {"content"}` syntax writes files directly

One command generates and deploys everything. No intermediate steps or glue scripts needed.

### Comparison with Alternatives

| Tool | Approach | Multi-file Deploy | Type Checking | Learning Curve |
|------|----------|-------------------|---------------|----------------|
| Avon | Language + Deploy | Built-in | Runtime checks | Low |
| Jsonnet | Pure language | Manual | Limited | Medium |
| Dhall | Typed language | Manual | Strong types | High |
| CUE | Data validation | Via scripts | Constraints | Medium |
| Jinja2 | Template only | Manual | None | Low |

---

## Syntax Reference

### Multiline Templates

Avon uses `{"..."}` for multiline strings. Everything between braces is literal text, and `{variable}` placeholders get replaced:

```avon
let message = {"
    This is a multiline
    template string
"} in message
```

### The `let...in` Pattern

Every `let` must be followed by `in` and an expression that uses it:

```avon
let x = 10 in
let y = 20 in
x + y  # Final expression - equals 30
```

Think of it as: "let x equal 10, then in the following expression, use x"

### Functions

Functions use `\parameter expression` syntax (inspired by lambda calculus):

```avon
let double = \x x * 2 in
let add = \a \b a + b in
double 5  # Returns 10
```

### Function Application

Functions are called by placing arguments after the function name, separated by spaces:

```avon
let add = \a \b a + b in
add 3 5  # Returns 8

# Parentheses for grouping
(add 3) 5
add (1 + 2) (2 + 3)
```

### Currying

All functions are curried—applying fewer arguments returns a new function:

```avon
let add = \a \b a + b in
let add10 = add 10 in  # Partial application
add10 5                 # Returns 15
```

Powerful for mapping:

```avon
let add = \a \b a + b in
map (add 10) [1, 2, 3]  # Returns [11, 12, 13]
```

---

## Real-World Examples

### Kubernetes Multi-Environment Configs

Generate config files for multiple environments:

```avon
let envs = ["dev", "prod"] in
let replicas = {dev: 1, prod: 3} in

let make_config = \env 
  @k8s/{env}-deployment.yaml {"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: app
  namespace: {env}
spec:
  replicas: {get replicas env}
"}
in

map make_config envs
```

Result: 2 deployment files, one for each environment.

### Environment-Specific Config

```avon
\env ? "dev"

let config = {
  dev:  {host: "localhost", debug: "true"},
  prod: {host: "db.prod", debug: "false"}
} in

let c = get config env in

@config.env {"
HOST={c.host}
DEBUG={c.debug}
"}
```

### CI/CD Pipelines

Generate CI configs for multiple repositories:

```avon
let repos = ["frontend", "backend"] in

map (\repo @{repo}-ci.yml {"
name: {repo} CI
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - run: npm test
"}) repos
```

### Platform-Specific Config

Generate platform-specific configs using the `os` builtin:

```avon
let platform = os in

let config = {
  linux: {shell: "/bin/bash", pathsep: ":"},
  macos: {shell: "/bin/zsh", pathsep: ":"},
  windows: {shell: "powershell", pathsep: ";"}
} in

let c = get config platform in

@platform-config.env {"
PLATFORM={platform}
SHELL={c.shell}
PATH_SEP={c.pathsep}
"}
```

---

## Command Reference

### Basic Commands

```bash
# Evaluate and preview (no files written)
avon eval program.av

# Deploy files to disk
avon deploy program.av --root ./output

# Evaluate expression directly
avon run 'map (\x x * 2) [1, 2, 3]'

# Interactive REPL
avon repl

# Function documentation
avon doc
```

### Deployment Options

```bash
# Overwrite existing files
avon deploy program.av --force

# Append to existing files
avon deploy program.av --append

# Skip if files exist
avon deploy program.av --if-not-exists
```

### Passing Arguments

```bash
# Named arguments
avon deploy program.av -env prod -region us-east-1

# Positional arguments
avon deploy program.av staging
```

### Git Integration

```bash
# Fetch and deploy from GitHub
avon deploy --git user/repo/config.av --root ./
```

### Debugging

```bash
# Show debug output
avon eval program.av --debug
```

---

## Examples

The `examples/` directory contains 90+ working examples:

**Infrastructure:**
- Docker Compose, Kubernetes, Terraform
- GitHub Actions, CI/CD pipelines

**Configuration:**
- Nginx configs, environment files
- Neovim/Emacs configs

**Content:**
- Static sites, markdown documentation

**Try an example:**

```bash
ls examples/
avon eval examples/docker_compose_gen.av
```

---

## Language Basics

**Variables and functions:**

```avon
let port = 8080 in
let make_url = \svc \p {"http://{svc}:{p}"} in
make_url "api" port  # Returns: http://api:8080
```

**Dictionaries with dot notation:**

```avon
let config = {host: "localhost", port: 8080} in
config.port  # Returns: 8080
```

**Lists and map:**

```avon
let services = ["auth", "api", "web"] in
map (\s {"http://{s}:8080"}) services
```

**Conditionals:**

```avon
let env = "prod" in
if env == "prod" then "3 replicas" else "1 replica"
```

**Import modules:**

```avon
let math = import "math_lib.av" in
math.double 21  # Returns: 42
```

**Generate files:**

```avon
@config.yml {"
port: 8080
debug: true
"}
```

See the [Tutorial](./tutorial/TUTORIAL.md) for the complete guide.

---

## Error Messages & Debugging

Avon provides clear error messages with line numbers:

```bash
$ avon eval test.av
concat: type mismatch: expected String, found Number on line 10
10 |    concat "Port: " 8080
```

**Debugging tools:**

| Tool | Purpose |
|------|---------|
| `trace "label" value` | Print labeled values to stderr |
| `debug value` | Pretty-print value structure |
| `assert condition value` | Validate conditions early |
| `--debug` flag | Show lexer/parser/evaluator output |

## Getting Started

**Try an example:**

```bash
avon eval examples/docker_compose_gen.av
```

**Generate your first configs:**

```bash
avon deploy examples/docker_compose_gen.av --root ./my-configs
```

**Explore examples:**

```bash
ls examples/
```

---

## What Will You Build?

Avon is a powerful, composable language. The examples here barely scratch the surface of what's possible. We've seen it used for:

- Generating infrastructure configs across cloud providers
- Building custom static site generators
- Automating development environment setup
- Creating code generators and scaffolding tools
- Managing complex multi-environment deployments

**But the real power comes from you.** Avon gives you functional programming primitives, file generation, and deployment—how you combine them is limited only by your creativity.

We're genuinely excited to see what you build. If you create something interesting, share it in GitHub Discussions. We love seeing Avon used in ways we never imagined.

---

## Quality & Testing

- **500+ tests passing** — Unit tests, integration tests, and working examples
- **Clear error messages** — Line numbers and context for all errors
- **Type-safe** — Runtime type checking prevents deployment errors
- **Single binary** — No dependencies, easy deployment
- **Production-ready** — Comprehensive error handling

---

## Contributing

**Report issues or request features** on GitHub Issues

**Share your Avon programs** in GitHub Discussions — we love seeing creative uses!

**Submit pull requests** — See `examples/` directory for coding style

---

## Resources

- [Tutorial](./tutorial/TUTORIAL.md) — Complete language guide
- [Function Reference](./tutorial/BUILTIN_FUNCTIONS.md) — Built-in function documentation
- `avon doc` — Command-line help

---

## License

MIT License - See LICENSE file for details
