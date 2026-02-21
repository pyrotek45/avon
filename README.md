# Avon

> Give your files superpowers

Avon is a functional language for generating, deploying, and automating any text file. Add variables, functions, and logic to YAML, JSON, configs, scripts, or any text format. Whether you're managing a single dotfile, generating thousands of Kubernetes manifests, or running build tasks, Avon gives you the power to automate what you never thought possible.

**What makes Avon powerful:**
- **Language agnostic** — Transform any text format
- **Functional programming** — Variables, functions, map/filter/fold, type safety
- **Built-in deployment** — Files know where they belong
- **Built-in task runner** — Define and run shell tasks with dependency resolution
- **Atomic deployment** — All-or-nothing, no partial failures
- **Git integration** — Share templates, deploy anywhere
- **Extensible** — Combine primitives in creative ways

Avon is designed to be powerful and flexible. I'm excited to see how you use it in ways not even mentioned here.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)

---

## Documentation

| Resource | Description |
|----------|-------------|
| [**Getting Started**](./tutorial/GETTING_STARTED.md) | New to Avon? Start here. A step-by-step guide with 15 hands-on lessons that build on each other. |
| [**Tutorial**](./tutorial/TUTORIAL.md) | Complete guide from basics to advanced patterns. Covers language essentials, templates, CLI usage, style guide, best practices, error handling, and debugging. |
| [**Function Reference**](./tutorial/BUILTIN_FUNCTIONS.md) | Reference for all built-in functions with signatures, descriptions, and examples organized by category. |
| [**Do Mode Guide**](./tutorial/DO_MODE_GUIDE.md) | Built-in task runner guide. Define and run shell tasks with dependencies, env vars, and auto-discovery. |
| **Command Line** | Run `avon doc` for built-in help on any function, `avon help do` for task runner help |
| **Examples** | See `examples/` directory for 160+ real-world examples |

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

## Built-in Documentation

Avon includes comprehensive built-in documentation for all of its functions. The `avon doc` command is one of its most powerful features, enabling developers to quickly learn the tool without leaving the terminal.

**Look up any function:**

```bash
avon doc map
```

Output:
```
map :: (a -> b) -> [a] -> [b]
  Transform each item in a list by applying a function.
  
  Arguments:
    1. Function to apply to each element
    2. List to transform
  
  Example: map (\x x * 2) [1, 2, 3] -> [2, 4, 6]
           Double each number in the list
  
  Example: map upper ["hello", "world"] -> ["HELLO", "WORLD"]
           Convert each string to uppercase
```

**Browse functions by category:**

```bash
avon doc string      # All string functions
avon doc list        # All list functions  
avon doc dict        # All dictionary functions
avon doc math        # All math functions
avon doc io          # All I/O functions
avon doc template    # All template functions
```

Example output for `avon doc string`:
```
String Functions:
─────────────────
Basic Operations:
concat           Concatenate two strings
upper            Convert to uppercase
lower            Convert to lowercase
trim             Remove leading/trailing whitespace
...
```

**Show all available documentation:**

```bash
avon doc             # Complete function reference
```

**Quick workflow:**

```bash
# 1. Browse category
avon doc list

# 2. Look up specific function
avon doc filter

# 3. Test it immediately
avon run 'filter (\x x > 2) [1, 2, 3, 4, 5]'
```

This tight feedback loop makes learning Avon fast and intuitive. No need to switch to a browser or search through docs—everything you need is in the terminal.

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

### Atomic Deployment

Deployment is atomic — if any error occurs during evaluation or validation, **no files are written**.

**Three-phase process:**
1. **Evaluate** — Run your program and collect FileTemplates
2. **Validate** — Check all paths, permissions, and directories
3. **Write** — Only if phases 1 & 2 succeed, write all files

If evaluation fails (type errors, undefined variables), validation fails (permissions, path issues), or the result isn't deployable, Avon aborts with zero files written. This prevents partial deployments that leave your system in an inconsistent state

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
avon do build                     # Run a task from Avon.av
avon run 'expr'                   # Evaluate expression directly
avon repl                         # Interactive exploration
avon doc [function|category]      # Built-in function reference (see above)
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

## Do Mode — Built-in Task Runner

Avon includes a task runner that replaces Make, Just, and npm scripts. Define tasks in an `Avon.av` file and run them with `avon do`.

**Basic tasks:**

```avon
{
  build: "cargo build --release",
  test: {cmd: "cargo test", deps: ["build"]},
  clean: "cargo clean"
}
```

```bash
avon do build          # runs cargo build --release
avon do test           # runs build first (dependency), then test
avon do --list         # show all tasks
avon do --dry-run test # preview execution plan
```

### Why Not Just Use Make?

Because Avon task files are **programs**, not static text. You get variables, functions, conditionals, and string manipulation — things Make and Just can't do without external scripts.

**This is Avon's own `Avon.av` — it uses a function to wrap every command in nix-shell:**

```avon
let nix = \cmd "nix-shell --run '" + cmd + "'" in

{
  fmt: nix "cargo fmt",
  lint: nix "cargo clippy -- -D warnings",
  test: {cmd: nix "cargo test", deps: ["fmt", "lint"]},
  build: {cmd: nix "cargo build --release", deps: ["test"]},
  clean: nix "cargo clean"
}
```

One function. No repetition. Every command runs inside the Nix environment automatically.

**In Make, you'd have to repeat yourself:**

```makefile
fmt:
	nix-shell --run 'cargo fmt'
lint:
	nix-shell --run 'cargo clippy -- -D warnings'
test: fmt lint
	nix-shell --run 'cargo test'
build: test
	nix-shell --run 'cargo build --release'
clean:
	nix-shell --run 'cargo clean'
```

The `nix-shell --run '...'` prefix is duplicated in every single rule. Change it? Edit 5 lines. With Avon, change the `nix` function — one line.

**More examples of what Avon tasks can do that Make can't:**

```avon
# Environment-driven configuration
let env = env_var_or "ENV" "dev" in
let profile = if env == "prod" then "release" else "dev" in

# Reusable command templates
let cargo = \action "cargo " + action + " --profile " + profile in

{
  build: {cmd: cargo "build", desc: "Build for " + env},
  test:  {cmd: cargo "test",  desc: "Test in " + profile + " mode", deps: ["build"]},
  
  deploy: {
    cmd: "echo 'Deploying to " + env + "'",
    deps: ["test"],
    env: {DEPLOY_ENV: env}
  }
}
```

```bash
avon do build                    # builds with dev profile
ENV=prod avon do deploy          # builds release, runs tests, deploys to prod
```

Variables, conditionals, and functions — computed at evaluation time, not hardcoded. See the [Do Mode Guide](./tutorial/DO_MODE_GUIDE.md) for the full documentation.

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

### `publish()` — Flexible FileTemplate Creation

The `publish()` function provides an alternative way to create FileTemplates that's particularly useful when working with stored templates or dynamic data structures.

Both `@path {...}` and `publish()` support dynamic path interpolation. The key difference is that `publish()` accepts templates and paths as *stored values* (from variables, function returns, etc.), while `@path {...}` requires the template to be written inline.

**Basic usage:**

```avon
# Simple case
publish "hello.txt" "Hello, World!"

# With template interpolation
let env = "prod" in
publish ("config-{env}.yml") {"
  environment: {env}
  port: 443
"}

# Content from a variable (stored template)
let template = @template.txt in
publish "output.txt" template

# From function results
let load_template = \name
  readfile (name + ".template")
in
publish "output.txt" (load_template "mytemplate")
```

**Powerful combination: `publish()` with `format_*` functions**

Generate multiple configuration formats from a single data structure:

```avon
# Generate config in Avon, JSON, and YAML from the same data
let config = {
  name: "MyApp",
  port: 8080,
  debug: false,
  database: {host: "localhost", port: 5432}
}
in
[
  publish "config.avon" (format_avon config),
  publish "config.json" (format_json config),
  publish "config.yaml" (format_yaml config)
]

# Generate AVONFILE task definitions from data
let tasks = {
  build: {cmd: "cargo build --release"},
  test: {cmd: "cargo test --all"},
  clean: {cmd: "cargo clean"}
}
in
publish "AVONFILE" (format_avon tasks)

# Generate Docker Compose from structured data
let docker_services = {
  version: "3.8",
  services: {
    web: {image: "nginx:latest"},
    api: {image: "myapp:latest"}
  }
}
in
publish "docker-compose.yml" (format_yaml docker_services)
```

This pattern is particularly powerful for **code generation** and **cross-format configuration**:
- Generate multiple formats (JSON, YAML, TOML, Avon) from a single data structure
- Create task files (`AVONFILE`) programmatically
- Build Docker Compose or Kubernetes manifests from data
- Generate OpenAPI/Swagger specs from structured definitions

**When to use `publish()` instead of `@path {...}`:**

| Scenario | Use |
|----------|-----|
| Content is stored in a variable | `publish()` |
| Content comes from a function result | `publish()` |
| Content is a template from a different file | `publish()` |
| Content is from `format_avon()`, `format_json()`, etc. | `publish()` |
| Building FileTemplates in map/filter operations | `publish()` |
| Content is hardcoded in the expression | `@path {...}` (simpler) |

**Example: Generate files from stored templates**

```avon
# Store templates in variables
let web_template = @templates/web.yml in
let api_template = @templates/api.yml in

# Use publish to generate multiple files
[
  publish "config/web.yml" web_template,
  publish "config/api.yml" api_template
]

# Or programmatically with a function
let make_config = \name \template
  publish ("config/{name}.yml") template
in
[
  make_config "web" web_template,
  make_config "api" api_template
]
```

See `examples/publish_demo.av` and `examples/publish_with_formats.av` for more examples, and run `avon doc publish` for full documentation.

---

### Download Feature — Fetch Resources Before Tasks

Avon's task runner (do mode) can download files from the internet before executing tasks. This makes it perfect for workflows that depend on external resources, data pipelines, and distributed configurations.

**Basic download in a task:**

```avon
{
  process_data: {
    cmd: "process.sh data.json",
    desc: "Download and process remote data",
    download: {
      url: "https://api.example.com/data.json",
      to: "data.json"
    }
  }
}
```

**Multiple downloads:**

```avon
{
  setup: {
    cmd: "echo 'Templates downloaded'",
    desc: "Download multiple templates from GitHub",
    download: [
      {
        url: "https://raw.githubusercontent.com/org/repo/main/template1.yml",
        to: "configs/template1.yml"
      },
      {
        url: "https://raw.githubusercontent.com/org/repo/main/template2.yml",
        to: "configs/template2.yml"
      }
    ]
  }
}
```

**Download with error handling:**

```avon
{
  process: {
    cmd: "process.sh",
    download: {
      url: "https://example.com/data.json",
      to: "data.json"
    },
    ignore_errors: true  # Continue even if download fails
  }
}
```

**Download with quiet mode:**

```avon
{
  setup: {
    cmd: "echo done",
    download: {
      url: "https://example.com/file.txt",
      to: "file.txt"
    },
    quiet: true  # Suppress "Downloading:" messages
  }
}
```

**Real-world workflow: Data pipeline**

```avon
{
  fetch_data: {
    cmd: "echo 'Data fetched'",
    download: {
      url: "https://api.example.com/dataset.csv",
      to: "raw/dataset.csv"
    }
  },
  
  validate: {
    cmd: "validator raw/dataset.csv",
    deps: ["fetch_data"]
  },
  
  transform: {
    cmd: "transform.sh raw/dataset.csv > processed/data.json",
    deps: ["validate"]
  },
  
  report: {
    cmd: "python generate_report.py processed/data.json",
    deps: ["transform"]
  }
}
```

**Why downloads matter:**

- **One source of truth** — Keep master data/configs on a server, deploy to any machine
- **Dynamic workflows** — Workflows fetch what they need, no manual pre-staging
- **Data pipelines** — Download → validate → transform → report in one task chain
- **Configuration templates** — Download base configs from GitHub, customize locally
- **Build artifacts** — Fetch dependencies, templates, or data files as part of the build

**Download feature options:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | string | yes | HTTP(S) URL to download from |
| `to` | string | yes | Local file path to save to (creates directories as needed) |
| `quiet` | bool | no | Suppress "Downloading:" output messages |
| `ignore_errors` | bool | no | Continue task even if download fails |

See `examples/download_basic.av`, `examples/download_pipeline.av`, `examples/download_config_gen.av`, and `examples/download_swiss_army.av` for more examples.

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

### Atomic Deployment

All-or-nothing deployment. If any error occurs during evaluation or validation, no files are written. No partial deployments, no inconsistent state.

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

Avon integrates three systems that are usually separate:

1. **Functional Language** — Variables, functions, lists, conditionals, runtime type checking
2. **Deployment System** — `@path/to/file.yml {"content"}` syntax writes files directly
3. **Task Runner** — Define and run shell tasks with dependency resolution, environment variables, and computed commands

One tool for generating configs, deploying files, and running build tasks. No intermediate steps or glue scripts needed.

### Comparison with Alternatives

| Tool | Approach | File Generation | Task Runner | Type Checking | String Logic |
|------|----------|----------------|-------------|---------------|--------------|
| **Avon** | Language + Deploy + Tasks | Built-in | Built-in | Runtime checks | Full language |
| Make | Task runner only | No | Yes | No | Shell only |
| Just | Task runner only | No | Yes | No | Shell only |
| Jsonnet | Pure language | JSON only | No | Limited | Limited |
| Dhall | Typed language | JSON/YAML | No | Strong types | Limited |
| Jinja2 | Template only | Manual | No | None | Limited |

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

# Run a task from Avon.av
avon do build

# Evaluate expression directly
avon run 'map (\x x * 2) [1, 2, 3]'

# Interactive REPL
avon repl

# Function documentation
avon doc

# Task runner help
avon help do
```

### Task Runner (Do Mode)

```bash
# Run a task (auto-discovers Avon.av)
avon do build

# Run a task from a specific file
avon do test tasks.av

# List all tasks
avon do --list

# Preview execution plan
avon do --dry-run deploy

# Show task details
avon do --info build
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

### How Avon Reads Files

Avon resolves its source file in this priority order:

1. **`--stdin`** — Read source from standard input (eval/deploy only)
2. **`--git <url>`** — Fetch from GitHub (`user/repo/path/to/file.av`)
3. **`<file>` argument** — Read the named file from disk
4. **`Avon.av`** — Auto-discover in the current directory (`do` mode only)

If none of these succeed, Avon prints an error with usage hints.

> **Security:** `--git` and `--stdin` work with `do` mode but require confirmation
> because running shell commands from a remote or piped source is a security risk.
> Use `--force` to skip the prompt (e.g., in CI), or download and review the file first.

**When to use each mode:**

| Mode     | Purpose                                        | File Content               |
|----------|------------------------------------------------|----------------------------|
| `eval`   | Preview output — no files written to disk      | Any valid Avon expression  |
| `deploy` | Write generated FileTemplates to disk          | Must produce FileTemplates |
| `do`     | Run shell tasks with dependency resolution     | Must evaluate to a dict of task definitions |
| `run`    | Evaluate a code string directly (no file)      | N/A (code on command line) |

---

## Examples

The `examples/` directory contains 160+ working examples:

**Infrastructure:**
- Docker Compose, Kubernetes, Terraform
- GitHub Actions, CI/CD pipelines

**Configuration:**
- Nginx configs, environment files
- Neovim/Emacs configs

**Task Runner:**
- Build automation, CI/CD pipelines
- Multi-step workflows with dependencies

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

Avon is a powerful, composable language. The examples here barely scratch the surface of what's possible.

**Real-world usage:**
- **[pyrotek45.github.io](https://pyrotek45.github.io)** — This entire website is generated with Avon
- Infrastructure configs across cloud providers
- Custom static site generators
- Development environment automation
- Code generators and scaffolding tools
- Multi-environment deployment systems

**The real power comes from how you use it.** Avon gives you functional programming primitives, file generation, and deployment. What you build with them is limited only by your imagination.

Think beyond configuration files. Think beyond the examples shown here. Avon can transform any text, automate any workflow, generate any file structure you need.

**I'm genuinely curious to see what you build.** If you create something interesting, share it in GitHub Discussions. I love seeing Avon used in ways I never thought of.

---

## Quality & Testing

- **660+ tests passing** — Unit tests, integration tests, and working examples
- **Clear error messages** — Line numbers, context, and typo suggestions for all errors
- **Type-safe** — Runtime type checking prevents deployment errors
- **Single binary** — No dependencies, easy deployment
- **Production-ready** — Comprehensive error handling

---

## Resources

- [Tutorial](./tutorial/TUTORIAL.md) — Complete language guide
- [Getting Started](./tutorial/GETTING_STARTED.md) — Step-by-step lessons
- [Do Mode Guide](./tutorial/DO_MODE_GUIDE.md) — Task runner guide
- [Function Reference](./tutorial/BUILTIN_FUNCTIONS.md) — Built-in function documentation
- `avon doc` — Command-line help
- `avon help do` — Task runner CLI help
