# Avon — Add Superpowers to Any File

Avon brings variables, functions, and utilities to any text format—from dotfiles to infrastructure configs.

Whether you're managing a single `.vimrc`, sharing dotfiles with friends, or generating hundreds of Kubernetes manifests, Avon is the workflow layer that makes any file more powerful, maintainable, and shareable. It's language agnostic, works with any text format, and brings the power of a functional language to your files.

Stop copy-pasting. Start generating. (Your future self will thank you.)

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)

## Documentation

Jump straight to what you need:

| Doc | What's inside |
|-----|---------------|
| [Tutorial](./tutorial/TUTORIAL.md) | Learn Avon from scratch (start here!) |
| [Features Reference](./tutorial/FEATURES.md) | Complete language reference |
| [Template Syntax](./tutorial/TEMPLATE_SYNTAX.md) | Multi-brace delimiters, literal braces, tips & tricks |
| [Simple Configs](./tutorial/SIMPLE_CONFIGS.md) | Quick examples for common configs |
| [Style Guide](./tutorial/STYLE_GUIDE.md) | Best practices and conventions |
| [Debugging Guide](./tutorial/DEBUGGING_GUIDE.md) | When things go wrong |
| [REPL Usage](./tutorial/REPL_USAGE.md) | Interactive development |
| [Security](./tutorial/SECURITY.md) | Security model and sandboxing |

Or just run `avon doc` for built-in help on all 111 functions.

---

## Installation

```bash
git clone https://github.com/pyrotek45/avon
cd avon
cargo build --release
```

That's it. No package.json. No 200MB of node_modules. Just one binary.

Optionally, add to your PATH:
```bash
sudo cp target/release/avon /usr/local/bin/
# Or: export PATH="$PATH:$(pwd)/target/release"
```

## Quick Start: Your First Avon Program

Create a file called `hello.av`:

```avon
# hello.av - Our first Avon program
# Variables are defined with `let ... in` syntax

let name = "World" in
let greeting = "Hello" in

# This creates a FileTemplate - Avon's deployment unit
# @path is a Path, {"..."} is a Template, combined they make a FileTemplate
@hello.txt {"
    {greeting}, {name}!
    Welcome to Avon.
"}
```

The three key types:
- `@hello.txt` — A Path (file destination). Paths start with `@` (relative only) and can include `{variables}`.
- `{"..."}` — A Template (multiline text content). Use `{name}` for variable interpolation.
- `@path {"..."}` — A FileTemplate (Path + Template). This is what Avon actually deploys.

What's happening here:
- `let name = "World" in` — Defines a variable. The `in` keyword means "use this in what follows"
- `@hello.txt {"..."}` — Creates a FileTemplate: the path `hello.txt` with that content
- `{name}` inside `{"..."}` — Gets replaced with the variable's value

Test it (preview without writing files):
```bash
avon eval hello.av
```

Deploy it (actually write files to disk):
```bash
avon deploy hello.av --root ./output
# Creates: ./output/hello.txt with "Hello, World!\nWelcome to Avon."
```

That's the core pattern: Define variables → use them in Templates → attach to Paths → deploy. Pretty simple, right?

## How Deployment Works

When you run `avon deploy program.av`, Avon evaluates your program and then:

1. If the result is a Function — Avon applies CLI arguments to it. So if your program returns `\env \port ...`, running `avon deploy program.av prod 8080` passes `prod` and `8080` as arguments.

2. If the result is a FileTemplate — Avon writes it to disk. One `@path {"content"}` = one file.

3. If the result is a List of FileTemplates — Avon writes them all. This is how you generate 50 configs from one program.

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

(Why "deploy"? Because Avon doesn't just generate text—it puts files where they belong. The `@path` syntax makes file destinations a first-class part of your program.)

Core commands:
```bash
avon eval program.av              # Preview output (no files written)
avon deploy program.av --root ./  # Write files to disk
avon run 'expr'                   # Evaluate expression directly
avon repl                         # Interactive exploration
avon doc                          # Built-in function reference
```

## The `--git` Workflow: Share One Template, Deploy Everywhere

Here's a game-changer: fetch and deploy templates directly from GitHub.

Keep one Avon file in git, deploy customized versions on every machine:

```avon
# configs.av (stored in your git repo)
\env ? "dev" \user ? "developer"

@config.yml {"
    user: {user}
    environment: {env}
    debug: {if env == "dev" then "true" else "false"}
"}
```

Deploy with different settings on different machines:
```bash
# On your laptop
avon deploy --git user/repo/configs.av --root ~/.config/myapp -env dev -user alice

# On a production server  
avon deploy --git user/repo/configs.av --root /etc/myapp -env prod -user service

# On a coworker's machine
avon deploy --git user/repo/configs.av --root ~/.config/myapp -env dev -user bob
```

Everyone shares the same source file in git, but each deployment is customized via CLI arguments. One source of truth, infinite variations. No more "hey can you send me your config?"

## FileTemplates: The Key Insight

Avon has first-class types for file paths and templates. The `@path {"content"}` syntax creates a FileTemplate—the unit Avon uses for deployment.

Paths are values:
```avon
let config_path = @config.yml in   # This is a Path value
config_path                          # Returns: config.yml
```

Templates are values:
```avon
let content = {"
  name: myapp
  port: 8080
"} in
content   # Returns the template string
```

Paths can include interpolation:
```avon
let env = "prod" in
@configs/{env}.yml {"port: 8080"}   # Path becomes configs/prod.yml
```

Dynamic file generation with functions:
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
# (that's 3 more than most startups actually need)
```

FileTemplates are first-class values that can be:
- Stored in variables
- Returned from functions
- Collected in lists
- Mapped, filtered, and transformed

When you run `avon deploy`, Avon evaluates your program, collects all FileTemplates, and writes each one to disk. That's the entire deployment model—your program returns FileTemplates, and Avon deploys them.

## Key Features

`@` Deployment Syntax — Files know where they belong (relative paths only)
```avon
@path/to/file.yml {"content goes here"}
```
One command deploys everything. No more manual file juggling.

Dictionaries with Dot Notation — First-class hash maps
```avon
let config = {host: "localhost", port: 8080} in
config.host  # Access with dots! Like a normal language!
```

Pipe Operator — Chain expressions without nested parentheses
```avon
[1, 2, 3, 4, 5] -> filter (\x x > 2) -> length  # Clean and readable!
# Instead of: length (filter (\x x > 2) [1, 2, 3, 4, 5])
```

Functional Programming — Variables, functions, map/filter/fold, conditionals, currying

111 Builtins — String ops, list ops, formatting, date/time, JSON, file I/O, HTML/Markdown helpers

Runtime Type Safety — Avon won't deploy if there's a type error. Sleep soundly. No "undefined is not a function" at 3am.

Any Text Format — YAML, JSON, TOML, HCL, shell scripts, code, configs, docs, dotfiles

Simple & Modular — Each file contains one expression. Import returns any Avon type.
```avon
let math = import "math_lib.av" in  # Returns a dict of functions
let config = import "config.av" in  # Returns a dict of settings
math.double 21  # Returns 42
```

Share Templates with `--git` — Fetch and deploy templates directly from GitHub
```bash
avon deploy --git user/repo/config.av --root ~/.config -env prod
```

Run `avon doc` for the complete function reference.

## What Makes Avon Different

Two integrated systems:

1. Functional Language — Variables, functions, lists, conditionals, runtime type checking
2. Deployment System — `@path/to/file.yml {"content"}` syntax writes files directly

Result: One command generates and deploys everything. No intermediate steps. No glue scripts. No 3am debugging sessions wondering why your sed command ate your config.

### Avon vs Alternatives

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

Avon uses `{"..."}` for multiline strings (called templates). Everything between the braces is literal text, and `{variable}` placeholders get replaced:

```avon
let message = {"
    This is a multiline
    template string
"} in message
```

### The `let ... in` Pattern

Every `let` must be followed by `in` and an expression that uses it:

```avon
let x = 10 in
let y = 20 in
x + y  # This is the final expression - it equals 30
```

Think of it as: "let x equal 10, then in the following expression, use x"

### Functions (Lambda Syntax)

Functions use `\parameter expression` syntax. Yes, the backslash looks weird at first. You'll get used to it. (It's inspired by lambda calculus—the `\` is a budget λ for those of us without Greek keyboards.)

```avon
let double = \x x * 2 in      # Function that doubles a number
let add = \a \b a + b in      # Function that takes two parameters
double 5                       # Returns 10
```

### Function Application

Functions are called by placing arguments after the function name, separated by spaces:

```avon
let add = \a \b a + b in
add 3 5      # Returns 8

# Parentheses can be used for grouping
(add 3) 5    # Same result - applies 3, then 5
add (1 + 2) (2 + 3)  # Arguments can be expressions
```

### Currying

All functions are curried—applying fewer arguments returns a new function:

```avon
let add = \a \b a + b in
let add10 = add 10 in   # Partial application: a=10, waiting for b
add10 5                  # Returns 15
```

No `bind()`. No `apply()`. No `undefined` slipping in because you forgot an argument.

This is surprisingly powerful for mapping:
```avon
let add = \a \b a + b in
map (add 10) [1, 2, 3]   # Returns [11, 12, 13]
```

---

## Real-World Examples

### Kubernetes Multi-Environment Configs

This is where Avon really shines—generating multiple related config files from a single source of truth:

```avon
# k8s_generator.av - Generate deployment configs for all environments
let services = ["auth", "api", "frontend", "worker", "cache"] in
let environments = ["dev", "staging", "prod"] in

let env_config = {
  dev:     { replicas: 1, log_level: "debug", resources: "256Mi" },
  staging: { replicas: 2, log_level: "info",  resources: "512Mi" },
  prod:    { replicas: 3, log_level: "warn",  resources: "1Gi" }
} in

let make_deployment = \svc \env 
  let cfg = get env_config env in
  @k8s/{env}/{svc}-deployment.yaml {"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {svc}
  namespace: {env}
spec:
  replicas: {cfg.replicas}
  template:
    spec:
      containers:
      - name: {svc}
        image: mycompany/{svc}:latest
        env:
        - name: LOG_LEVEL
          value: {cfg.log_level}
"}
in

flatmap (\env map (\svc make_deployment svc env) services) environments
```

Run: `avon deploy k8s_generator.av --root ./manifests`  
Result: 15 YAML files organized by environment. One command, fifteen files.

### Environment-Specific Secrets

```avon
\env ? "dev"

let config = {
  dev:  { db_host: "localhost", debug: "true" },
  prod: { db_host: "db.prod.internal", debug: "false" }
} in

let c = get config env in
let db_password = env_var_or "DB_PASSWORD" "dev-password" in

@.config/myapp/secrets.env {"
    DB_HOST={c.db_host}
    DB_PASSWORD={db_password}
    DEBUG={c.debug}
"}
```

### Template Replacement

```avon
let template = read "email_template.txt" in
let filled = fill_template template {
  name: "Alice",
  order_id: "12345", 
  status: "shipped"
} in
@emails/alice.txt {"{filled}"}
```

### CI/CD Pipelines

Generate CI configs for all your repos at once (because manually copy-pasting YAML is a crime against developer sanity):

```avon
let repos = ["frontend", "backend", "mobile"] in
map (\repo @.github/workflows/{repo}-ci.yml {"
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

### Platform-Specific Code Generation

Generate platform-specific configs using the `os` builtin (returns "linux", "macos", or "windows"):

```avon
# Auto-detect current platform with `os` builtin
let platform = os in

# Or override via CLI: avon deploy platform.av -platform windows
# \platform ? os   # Use os as default, allow override

let platform_config = {
  linux:   { shell: "/bin/bash",    home: "/home",       pathsep: ":" },
  macos:   { shell: "/bin/zsh",     home: "/Users",      pathsep: ":" },
  windows: { shell: "powershell",   home: "C:\\Users",   pathsep: ";" }
} in

let includes = {
  linux:   "#include <unistd.h>\n#include <pthread.h>",
  windows: "#include <windows.h>\n#include <process.h>",
  macos:   "#include <unistd.h>\n#include <mach/mach.h>"
} in

let cfg = get platform_config platform in

[
  @platform.h {"
    #ifndef PLATFORM_H
    #define PLATFORM_H

    /* Auto-generated for {platform} */
    {get includes platform}

    #define PLATFORM_NAME \"{platform}\"
    #define DEFAULT_SHELL \"{cfg.shell}\"

    #endif
  "},

  @build_config.env {"
    # Build configuration for {platform}
    PLATFORM={platform}
    SHELL={cfg.shell}
    HOME_BASE={cfg.home}
    PATH_SEPARATOR={cfg.pathsep}
  "}
]
```

**Deploy for current OS:** `avon deploy platform.av --root ./build`  
**Deploy for specific OS:** `avon deploy platform.av --root ./build -platform windows`

---

## Commands Reference

```bash
# Evaluate and print result (preview - no files written)
avon eval program.av

# Deploy files to disk
avon deploy program.av --root ./output

# Deployment options
avon deploy program.av --force          # Overwrite existing
avon deploy program.av --append         # Append to existing
avon deploy program.av --if-not-exists  # Skip if exists

# Pass arguments
avon deploy program.av -env prod -region us-east-1  # Named args
avon deploy program.av staging                       # Positional args

# Fetch from git
avon deploy --git user/repo/config.av --root ./

# Other commands
avon run 'map (\x x * 2) [1, 2, 3]'  # Evaluate expression
avon repl                            # Interactive REPL
avon doc                             # Function documentation
avon eval program.av --debug         # Debug output
```

---

## Examples

92 working examples in `examples/` directory:
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

# Functions (yes, the backslash is the lambda)
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

# Import modules
let math = import "math_lib.av" in
math.double 21  # Returns 42

# Generate files
@config.yml {"
  port: {port}
  debug: {if env == "prod" then "false" else "true"}
"}
```

See [TUTORIAL.md](./tutorial/TUTORIAL.md) for the complete guide.

## Error Messages & Debugging

Avon tells you exactly what went wrong and where:

```bash
$ avon eval test.av
concat: type mismatch: expected String, found Number on line 10
10 |    concat "Port: " 8080
```

Debugging tools:
- `trace "label" value` — Print labeled values to stderr
- `debug value` — Pretty-print value structure
- `assert condition value` — Validate conditions early
- `--debug` flag — See lexer/parser/evaluator output

## Quality

- 500+ tests passing (157 unit tests + 93 working examples + integration tests)
- Clear error messages with line numbers and context (because "undefined is not a function" is not a personality)
- Type-safe, single binary, no dependencies
- Production-ready error handling (no panics in sight)

## Community & Contributing

- Issues: Report bugs or request features on GitHub
- Examples: Share your Avon programs in discussions
- Contributing: PRs welcome! See examples/ for coding style

## License

MIT — Use it however you want. We're not lawyers, and this isn't legal advice. But also, it's MIT, so go nuts.

---

## Get Started Now

```bash
# Try an example
avon eval examples/docker_compose_gen.av

# Generate your first configs
avon deploy examples/docker_compose_gen.av --root ./my-configs

# Explore
ls examples/
```

Stop maintaining 50 config files. Maintain 1 Avon program. (Your keyboard will thank you for the reduced wear.)

---

Questions? Check the [Tutorial](./tutorial/TUTORIAL.md) or run `avon doc` for built-in help.

<!-- P.S. No configs were harmed in the making of this tool. The same cannot be said for the developer's sanity. -->
