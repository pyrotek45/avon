# Avon — Add Superpowers to Any File

**Avon** brings variables, functions, and utilities to **any text format**—from dotfiles to infrastructure configs.

Whether you're managing a single `.vimrc`, sharing dotfiles with friends, or generating hundreds of Kubernetes manifests, Avon is the workflow layer that makes any file more powerful, maintainable, and shareable. It's language agnostic, works with any text format, and brings the power of a functional language to your files.

**From one dotfile to a thousand configs: variables, functions, and utilities for any file.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)

## Quick Start (2 Minutes)

```bash
# 1. Build
cargo build --release

# 2. Create hello.av
echo 'let name = "World" in @/hello.txt "Hello, {name}!"' > hello.av

# 3. Deploy
./target/release/avon deploy hello.av --root ./output

# Result: ./output/hello.txt created with "Hello, World!"
```

**That's it.** You just wrote a program that generates and deploys files.

## The Problem

Stop copy-pasting configs. Stop maintaining 50 nearly-identical YAML files. Stop forgetting to update that one config when you change a port.

**Traditional approach:** Generate -> Save -> Copy -> Repeat 100 times  
**Avon:** Write once -> `avon deploy` -> 100 files appear

**But Avon isn't just for infrastructure teams.** Whether you're a developer managing dotfiles, a hobbyist sharing configs, or someone who wants to add variables and utilities to a single file—Avon brings the power of a functional language to any text format, making any file more generic, maintainable, and shareable.

## What Makes Avon Different

**Two integrated systems:**

1. **Functional Language** — Variables, functions, lists, conditionals, runtime type checking
2. **Deployment System** — `@/path/to/file.yml {"content"}` syntax writes files directly

**Result:** One command generates and deploys everything. No intermediate steps.

**Language Agnostic:** Avon works with **any text format**—YAML, JSON, TOML, shell scripts, code, configs, documentation, or dotfiles. It brings variables, functions, and 89+ built-in utilities to any file, making even single files more powerful and maintainable.

**Share Templates with `--git`:** One of Avon's key features is the `--git` flag, which lets you fetch and deploy templates directly from GitHub. Keep one template in git, deploy customized versions everywhere with a single command. Perfect for sharing dotfiles, team configs, and infrastructure templates.

**Perfect for Everyone:**
- **Developers:** Generate 10-1000 config files from one program
- **Non-developers:** Easy way to download and deploy dotfiles, share configs, or manage personal settings
- **Hobbyists:** Add superpowers to any file with variables, interpolation, and built-in functions
- **Teams:** Share one template file in git, deploy customized variants via CLI arguments

### Avon vs Alternatives

| Tool | Approach | Multi-file Deploy | Type Checking | Learning Curve |
|------|----------|-------------------|---------------|----------------|
| **Avon** | Language + Deploy | Built-in | Runtime checks | Low |
| Jsonnet | Pure language | Manual | Limited | Medium |
| Dhall | Typed language | Manual | Strong types | High |
| CUE | Data validation | Via scripts | Constraints | Medium |
| Jinja2 | Template only | Manual | None | Low |

**Avon's advantage:** Combines the ease of use of Jinja2, the power of functional languages, and built-in deployment capabilities that make it perfect for any task.

## Handles Everything

Avon is a powerful, general-purpose tool that excels at generating hundreds of files, but it's equally powerful for single files. It's a comprehensive workflow layer that makes **any file** more maintainable, whether you're managing one config or building complex multi-file systems.

**Example: Dotfiles with Variables**
```avon
\username ? "developer" @/.vimrc {"
  " Vim configuration for {username}
  set number
  set expandtab
  set tabstop=4
  colorscheme {if username == "developer" then "solarized" else "default"}
"}
```

**Deploy:**
```bash
avon deploy vimrc.av --root ~ -username alice
```

**Share:** Keep one `.vimrc.av` in git. Each developer deploys their customized version. No more maintaining separate dotfiles for each machine.

**Example: Long Config with Repetition**
```avon
let plugins = ["vim-fugitive", "vim-surround", "vim-commentary"] in
@/.vimrc {"
  " Plugin configuration
  {plugins}
"}
```

List interpolation automatically expands `{plugins}` into separate lines, eliminating copy-paste even in a single file.

## Real Example: Multi-File Generation

10 services x 3 environments = 30 Kubernetes manifests. **One Avon program:**

```avon
let services = ["auth", "api", "frontend", "worker", "cache"] in
let environments = ["dev", "staging", "prod"] in

let make_k8s = \svc \env @/k8s/{env}/{svc}-deployment.yaml {"
  apiVersion: apps/v1
  kind: Deployment
  metadata:
    name: {svc}
    namespace: {env}
  spec:
    replicas: {if env == "prod" then "3" else "1"}
    containers:
    - name: {svc}
      image: myapp/{svc}:latest
      env:
      - name: LOG_LEVEL
        value: {if env == "prod" then "warn" else "debug"}
"} in

flatmap (\env map (\svc make_k8s svc env) services) environments
```

**Run:** `avon deploy k8s.av --root ./manifests`  
**Result:** 15 files created instantly. Change one line, redeploy all.

**The Workflow:**
```
Write k8s.av -> Test with `avon eval` -> Deploy to staging -> Deploy to prod
     |              |                        |                    |
  1 file       Verify output          ./staging/k8s/*      ./prod/k8s/*
```

## Your First Real Program

**Create `docker-configs.av`:**
```avon
let services = ["web", "api", "db"] in
let make_compose = \svc @/docker-compose-{svc}.yml {"
  version: '3.8'
  services:
    {svc}:
      image: myapp/{svc}:latest
      ports:
        - {if svc == "web" then "80:80" else "8080:8080"}
"} in
map make_compose services
```

**Test first:**
```bash
avon eval docker-configs.av  # See what will be generated
```

**Deploy:**
```bash
avon deploy docker-configs.av --root ./generated
```

**Result:** 3 files created in `./generated/`. Scale this to 100 services just as easily.

**Now modify:** Change `services` list to add 10 more services. Redeploy. Done.

## Use Cases

**Multi-environment configs:**
```avon
let envs = ["dev", "staging", "prod"] in
map (\env @/.env.{env} {"
  DATABASE_URL=postgres://db-{env}.company.com
  API_KEY={if env == "prod" then "secret" else "dev-key"}
"}) envs
```

**CI/CD pipelines:**
```avon
let repos = ["frontend", "backend", "mobile"] in
map (\repo @/.github/workflows/{repo}-ci.yml {"
  name: {repo} CI
  on: [push, pull_request]
  jobs:
    test:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v2
        - run: npm test
"}) repos
```

**From JSON data:**
```avon
let data = json_parse "services.json" in
let services = data.services in
map (\svc 
  let name = svc.name in
  @/nginx-{name}.conf {"
    server {{
      listen 80;
      server_name {name}.example.com;
      }}
  "}) services
```

## Key Features

**`@` Deployment Syntax** — The game changer
```avon
@/path/to/file.yml {"content goes here"}
```
Files know where they belong. `avon deploy` writes everything at once.

**Dictionaries with Dot Notation** — First-class hash maps
```avon
let config = {host: "localhost", port: 8080} in
config.host  # Access with dots!
```

**Pipe Operator** — Chain expressions without nested parentheses
```avon
[1, 2, 3, 4, 5] -> filter (\x x > 2) -> length  # Clean and readable!
# Instead of: length (filter (\x x > 2) [1, 2, 3, 4, 5])
```

**Simple & Modular** — Each file contains one expression, keeping Avon simple. The `import` function makes it modular: any file can return any Avon type (string, dict, function, FileTemplate, etc.), allowing files to be libraries, data sources, or generators.
```avon
# math_lib.av (see examples/math_lib.av): {double: \x x * 2, triple: \x x * 3}
# config.av: {host: "localhost", port: 8080}
# deploy.av: @/config.yml {"content"}

let math = import "math_lib.av" in
let config = import "config.av" in
math.double 21  # Returns 42
config.host     # Returns "localhost"
```

**Functional Programming** — Variables, functions, map/filter/fold, conditionals

**Runtime Type Safety** — Avon doesn't deploy if there's a type error. No static types needed—if a type error occurs, deployment simply doesn't happen. This flexible approach brings type safety to any file without the complexity of compile-time type systems.

**89+ Builtins** — String ops, list ops (map, filter, fold, sort, unique, range, enumerate), formatting (15 functions), date/time operations, JSON, file I/O, HTML/Markdown helpers. These utilities make any file more powerful, even if you're just managing a single config.

**List Interpolation** — Perfect for long, repetitive files. Write one small program that generates extensive content using `map`, `filter`, and list interpolation features.

**Any Text Format** — YAML, JSON, TOML, HCL, shell scripts, code, configs, docs, dotfiles. Language agnostic and format agnostic.

**Debugging Tools** — `trace`, `debug`, `assert`, and `--debug` flag help you troubleshoot quickly, even for simple files.

Run `avon doc` for complete function reference.

## Commands

**Core Commands:**
```bash
# Evaluate and print result (no files written)
avon eval program.av

# Deploy files to disk
avon deploy program.av --root ./output

# Evaluate code directly
avon run 'map (\x x * 2) [1, 2, 3]'

# Start interactive REPL
avon repl

# Fetch and run from git (single template, many machines)
avon eval --git user/repo/program.av
avon deploy --git user/repo/program.av --root ./output
```

### Single Template, Many Deployments (The `--git` Workflow)

**This is one of Avon's most powerful features.** The `--git` flag lets you fetch and deploy templates directly from GitHub, enabling a powerful workflow: keep **one Avon file in git** and let each developer or environment deploy their own variant using CLI arguments.

**Example (`configs.av` in git):**
```avon
\env ? "dev" \user ? "developer" @/config-{env}.yml {"
    user: {user}
    env: {env}
"}
```

**Usage:**
```bash
# On a laptop
avon deploy --git user/repo/configs.av --root ~/.config/myapp -env dev -user alice

# On a server
avon deploy --git user/repo/configs.av --root /etc/myapp -env prod -user service
```

Everyone shares the same source file in git, but customizes the deployed config via command-line arguments and default parameters.

**Deployment Options:**
```bash
# Overwrite existing files
avon deploy program.av --force

# Append to existing files
avon deploy program.av --append

# Only write if file doesn't exist
avon deploy program.av --if-not-exists

# Prepend directory to all paths
avon deploy program.av --root ./dist
```

**Pass Arguments to Functions:**
```bash
# Named arguments (uses function parameter names)
avon deploy program.av -env prod -region us-east-1

# Positional arguments (passed to top-level function)
avon deploy program.av staging
```

**Debugging:**
```bash
# Show lexer, parser, and evaluator debug output
avon eval program.av --debug

# Start interactive REPL for exploration and debugging
avon repl

# Get all builtin function documentation
avon doc
```

**Workflow:** Write -> Test with `eval` -> Deploy to test dir -> Deploy to production

## Examples

**92 working examples in `examples/` directory:**
- Docker Compose, Kubernetes, Terraform, GitHub Actions
- Nginx configs, environment files, CI/CD pipelines
- Neovim/Emacs configs, static sites, markdown docs

```bash
ls examples/                    # See all examples
avon eval examples/nginx_gen.av # Try one
```

## Learn Avon in 5 Minutes

```avon
# Variables
let port = 8080 in
let host = "localhost" in

# Functions
let make_url = \svc \p {"http://{svc}:{p}"} in

# Dictionaries (hash maps with dot notation)
let config = {host: host, port: port} in
config.port  # Access with dots!

# Lists and map
let services = ["auth", "api", "web"] in
map (\s make_url s config.port) services

# Conditionals
let env = "prod" in
if env == "prod" then "3 replicas" else "1 replica"

# Module system - each file is one expression, import returns that value
let math = import "math_lib.av" in  # math_lib.av returns a dict (see examples/math_lib.av)
math.double 21  # Use functions from imported dict
# Any file can return any type: dict, string, FileTemplate, etc.

# Generate files with @ syntax
@/config.yml {"
  port: {port}
  debug: {if env == "prod" then "false" else "true"}
"}
```

See [TUTORIAL.md](./tutorial/TUTORIAL.md) for complete guide or run `avon doc`.

## Documentation

- **[Tutorial](./tutorial/TUTORIAL.md)** | **[Reference](./tutorial/FEATURES.md)** | **[Style Guide](./tutorial/STYLE_GUIDE.md)** | **[Debug](./tutorial/DEBUGGING_GUIDE.md)**
- **[REPL Usage](./tutorial/REPL_USAGE.md)** | **[Simple Configs](./tutorial/SIMPLE_CONFIGS.md)** | **[Site Generator](./tutorial/SITE_GENERATOR.md)**
- **92 working examples** in `./examples/`
- **`avon doc`** for built-in help

## Why Avon?

**Avon can handle everything:**
- Generating multiple config files (Docker, K8s, CI/CD, etc.)
- Multi-environment deployments (dev/staging/prod)
- **Managing dotfiles** — Easy way to download and deploy configs to your system
- **Sharing configs** — One file in git, many customized deployments
- **Single files with variables** — Make any file more generic and maintainable
- **Long, repetitive files** — Use list interpolation to eliminate copy-paste
- Runtime type checking and validation in configs
- One source of truth for infrastructure
- Copy-paste-modify workflows
- **Adding superpowers to any file** — Variables, functions, and utilities for any text format
- **Static site generation** — Build complete websites with markdown, templates, and HTML
- **Data transformation** — Process JSON, YAML, and any text format with powerful functions
- **Complex workflows** — Chain operations, import modules, and build sophisticated pipelines

**Avon is a powerful, general-purpose tool that can handle everything from simple config files to complex multi-file generators. Whether you need to generate 10-1000 files from one program or just add variables to a single file, Avon provides the flexibility and power you need.**

## Quality

- **500+ tests passing** (157 unit tests + 93 working examples + integration tests)
- **Simple, clear error messages** showing function/operator name and type information
- Type-safe, single binary, no dependencies
- Production-ready error handling (no panics)

## Error Messages & Debugging

Avon provides simple, direct error messages that show exactly what went wrong and where:

```bash
# Type mismatch in operator
$ avon eval examples/test.av
concat: type mismatch: expected String, found Number on line 10
10 |    concat "Port: " 8080

# Type mismatch in function (example - create your own test file)
$ avon eval test_map.av
map: add_one: +: expected String, found Number on line 5
   5 |    x + " suffix"

# Nested function error chain (example - create your own test file)
$ avon eval test_fold.av
fold: x: +: expected Number, found String on line 15
  15 |    acc + item
```

Each error shows the function/operator name, the types involved, and the exact line number with source code context. **Actionable errors help you fix problems faster.**

**Debugging Tools:**
- `trace "label" value` — Print labeled values to stderr, returns value unchanged
- `debug value` — Pretty-print value structure to stderr, returns value unchanged
- `assert condition value` — Validate conditions early (e.g., `assert (is_string x) x`)
- `--debug` flag — See lexer/parser/evaluator debug output

These tools make debugging simple, whether you're working with complex infrastructure or a single config file. Avon's runtime type checking ensures reliability—if a type error occurs, deployment simply doesn't happen, protecting you from bad configurations.

See [tutorial/DEBUGGING_GUIDE.md](tutorial/DEBUGGING_GUIDE.md) for the complete debugging guide.

## Installation

### From Source
```bash
git clone https://github.com/pyrotek45/avon
cd avon
cargo build --release
./target/release/avon version
```

### Add to PATH (optional)
```bash
# Linux/macOS
sudo cp target/release/avon /usr/local/bin/

# Or add to your shell profile
export PATH="$PATH:/path/to/avon/target/release"
```

## Community & Contributing

- **Issues:** Report bugs or request features on GitHub
- **Examples:** Share your Avon programs in discussions
- **Contributing:** PRs welcome! See examples/ for coding style

## License

MIT — Use it however you want.

---

## Get Started Now

```bash
# 1. Build
cargo build --release

# 2. Try an example
./target/release/avon eval examples/docker_compose_gen.av

# 3. Generate your first configs
./target/release/avon deploy examples/docker_compose_gen.av --root ./my-configs
```

**Stop maintaining 50 config files. Maintain 1 Avon program.**

---

**Questions?** Check the [Tutorial](./tutorial/TUTORIAL.md) or run `avon doc` for built-in help.
