# Avon — Generate and Deploy Configs with Code

**Avon** is a programming language AND deployment system for configuration files. Write one program, deploy hundreds of files.

**The language is only half the story—the `@` syntax deployment system is what makes Avon different.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

## Quick Start (2 Minutes)

```bash
# 1. Build
cargo build --release

# 2. Create hello.av
echo 'let name = "World" in @/hello.txt "Hello, {name}!"' > hello.av

# 3. Deploy
./target/release/avon hello.av --deploy --root ./output

# Result: ./output/hello.txt created with "Hello, World!"
```

**That's it.** You just wrote a program that generates and deploys files.

## The Problem

Stop copy-pasting configs. Stop maintaining 50 nearly-identical YAML files. Stop forgetting to update that one config when you change a port.

**Traditional approach:** Generate → Save → Copy → Repeat 100 times  
**Avon:** Write once → `avon --deploy` → 100 files appear

## What Makes Avon Different

**Two integrated systems:**

1. **Functional Language** — Variables, functions, lists, conditionals, type checking
2. **Deployment System** — `@/path/to/file.yml {"content"}` syntax writes files directly

**Result:** One command generates and deploys everything. No intermediate steps.

### Avon vs Alternatives

| Tool | Approach | Multi-file Deploy | Type Checking | Learning Curve |
|------|----------|-------------------|---------------|----------------|
| **Avon** | Language + Deploy | ✅ Built-in | ✅ Runtime checks | Low |
| Jsonnet | Pure language | ❌ Manual | ⚠️ Limited | Medium |
| Dhall | Typed language | ❌ Manual | ✅ Strong types | High |
| CUE | Data validation | ⚠️ Via scripts | ✅ Constraints | Medium |
| Jinja2 | Template only | ❌ Manual | ❌ None | Low |

**Avon's sweet spot:** Easier than Dhall, more powerful than Jinja2, deploys unlike Jsonnet.

## Real Example

10 services × 3 environments = 30 Kubernetes manifests. **One Avon program:**

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

**Run:** `avon k8s.av --deploy --root ./manifests`  
**Result:** 15 files created instantly. Change one line, redeploy all.

**The Workflow:**
```
Write k8s.av → Test with `avon eval` → Deploy to staging → Deploy to prod
     ↓              ↓                        ↓                    ↓
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
avon docker-configs.av --deploy --root ./generated
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
Files know where they belong. `avon --deploy` writes everything at once.

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

**Module System** — Import files, get their values, use dot notation
```avon
# math.av exports: {double: \x x * 2, triple: \x x * 3, ...}
let math = import "math.av" in
math.double 21  # Returns 42
```

**Functional Programming** — Variables, functions, map/filter/fold, conditionals

**Type Safety** — `typeof`, `is_*`, `assert_*` functions catch errors early

**80+ Builtins** — String ops, list ops, formatting, JSON, file I/O, HTML/Markdown helpers

**Any Text Format** — YAML, JSON, TOML, HCL, shell scripts, code, configs, docs

Run `avon --doc` for complete function reference.

## Commands

**Evaluation (No Files Written):**
```bash
# Evaluate and print result
avon eval program.av

# Evaluate code from command line
avon --eval-input 'map (\x x * 2) [1, 2, 3]'

# Evaluate from git
avon --git-eval pyrotek45/avon/examples/test.av
```

**Deployment:**
```bash
# Generate and deploy files
avon program.av --deploy --root ./output

# Overwrite existing files
avon program.av --deploy --root ./output --force

# Append to existing files instead of overwriting
avon program.av --deploy --root ./output --append

# Only write if file doesn't exist
avon program.av --deploy --root ./output --if-not-exists

# Deploy from git repository
avon --git pyrotek45/avon/examples/site_generator.av --root ./site
```

**Pass Arguments to Functions:**
```bash
# Named arguments (uses function parameter names)
avon program.av --deploy -env prod -region us-east-1

# Positional arguments
avon program.av --deploy staging
```

**Debugging:**
```bash
# Show lexer, parser, and evaluator debug output
avon program.av --debug

# Get all builtin function documentation
avon --doc
```

**Workflow:** Write → Test with `eval` → Deploy to test dir → Deploy to production

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

# Module system - import returns the file's value
let math = import "math.av" in  # math.av returns a dict
math.double 21  # Use functions from imported dict

# Generate files with @ syntax
@/config.yml {"
  port: {port}
  debug: {if env == "prod" then "false" else "true"}
"}
```

See [TUTORIAL.md](./tutorial/TUTORIAL.md) for complete guide or run `avon --doc`.

## Documentation

- **[Tutorial](./tutorial/TUTORIAL.md)** | **[Reference](./tutorial/FEATURES.md)** | **[Style Guide](./tutorial/STYLE_GUIDE.md)**
- **92 working examples** in `./examples/`
- **`avon --doc`** for built-in help

## Why Avon?

**When to use Avon:**
- ✅ Generating multiple config files (Docker, K8s, CI/CD, etc.)
- ✅ Multi-environment deployments (dev/staging/prod)
- ✅ Need type checking and validation in configs
- ✅ Want one source of truth for infrastructure
- ✅ Tired of copy-paste-modify workflows

**When NOT to use Avon:**
- ❌ Single static config file (just write YAML/JSON)
- ❌ Need strong compile-time types (use Dhall)
- ❌ Building web apps (use a web framework)
- ❌ Complex data validation logic (use CUE)

**Avon shines when you need to generate 10-1000 files from one program.**

## Quality

- **500+ tests passing** (157 unit tests + 93 working examples + integration tests)
- **Simple, clear error messages** showing function/operator name and type information
- Type-safe, single binary, no dependencies
- Production-ready error handling (no panics)

## Error Messages

Avon provides simple, direct error messages that show exactly what went wrong:

```bash
# Type mismatch in operator
$ avon test.av
concat: type mismatch: expected string, found 8080, /api

# Type mismatch in function
$ avon test_map.av
map: add_one: +: expected String, found Number

# Nested function error chain
$ avon test_fold.av
fold: x: +: expected Number, found String
```

Each error shows the function/operator name and the types involved, making debugging fast and easy. See [ERROR_SIMPLIFICATION.md](ERROR_SIMPLIFICATION.md) for detailed information about the error system.

## Installation

### From Source
```bash
git clone https://github.com/pyrotek45/avon
cd avon
cargo build --release
./target/release/avon --version
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

MIT OR Apache-2.0 — Use it however you want.

---

## Get Started Now

```bash
# 1. Build
cargo build --release

# 2. Try an example
./target/release/avon eval examples/docker_compose_gen.av

# 3. Generate your first configs
./target/release/avon examples/docker_compose_gen.av --deploy --root ./my-configs
```

**Stop maintaining 50 config files. Maintain 1 Avon program.**

---

**Questions?** Check the [Tutorial](./tutorial/TUTORIAL.md) or run `avon --doc` for built-in help.
