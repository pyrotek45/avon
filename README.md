# Avon — The Modern Template Language

**Avon** is a lightweight, elegant templating and file generation language built for developers. Write templates once, generate multi-file projects instantly.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

## Why Avon?

- **🎯 Purpose-built:** Designed specifically for generating code, configs, and infrastructure
- **⚡ Fast & Small:** Single-binary tool; no dependencies on Python, Node, or Ruby
- **🔄 Functional:** Pure functional programming model; composable, testable code
- **📝 Readable:** Clean syntax inspired by functional languages; easy to learn
- **🛠️ Powerful:** Generate multiple files in one go with full control over paths and content
- **🧑‍💻 Developer-friendly:** Escape hatch for literal braces; supports any text format

## Quick Start

### Install

```bash
cargo build --release
./target/release/avon --version
```

### Your First Program

Create `hello.av`:

```avon
"Hello, Avon!"
```

Run it:

```bash
cargo run -- eval hello.av
# Output: Hello, Avon!
```

### Generate Files

Create `greet.av`:

```avon
\name @/greeting.txt {"
Hello, {name}!
Welcome to Avon.
"}
```

Deploy:

```bash
cargo run -- greet.av --deploy -name Alice --root ./output --force
```

Creates `./output/greeting.txt` with personalized content.

### Generate Multiple Files

```avon
let environments = ["dev", "staging", "prod"] in
map (\env @/config-{env}.yml {"
environment: {env}
debug: {env != "prod"}
"}) environments
```

Deploy this and get three config files automatically!

## Core Features

### Template Syntax

Avon templates support interpolation with the `{...}` syntax:

```avon
{"Name: {name}, Age: {age}"}
```

Multi-line templates preserve formatting:

```avon
@/config.yml {"
app: myapp
port: 8080
debug: {debug_mode}
"}
```

### Escape Hatch for Literal Braces

When generating code with braces (Lua, Nginx, JSON), use the escape hatch:

```avon
@/config.lua {"
local config = {{
  name = "myapp"
}}
"}
```

Or for complex cases, use double-brace templates:

```avon
@/output.txt {{"
Interpolate: {{ 10 + 20 }}
Literal open: {{{
"}}
```

### Functional Programming

- **Functions:** `\x x + 1` (automatic currying)
- **Let bindings:** `let x = 10 in x + 5`
- **List operations:** `map`, `filter`, `fold`
- **Operators:** `+` (overloaded), `-`, `*`, `/`, `==`, `!=`, `>`, `<`, `>=`, `<=`

### Builtin Functions

**Strings:** `concat`, `upper`, `lower`, `split`, `join`, `replace`, `contains`, `starts_with`, `ends_with`

**Lists:** `map`, `filter`, `fold`, `length`

**Files:** `readfile`, `readlines`, `exists`, `basename`, `dirname`

**Data:** `import`, `json_parse`

**System:** `os` (returns "linux", "macos", or "windows")

## Command-Line Usage

```bash
# Evaluate and print result
cargo run -- eval program.av

# Deploy files with arguments
cargo run -- program.av --deploy -name value --root ./output --force

# Fetch and run from GitHub
cargo run -- --git owner/repo/path/to/file.av --deploy --root ./output
```

**Flags:**

- `eval` — Evaluate program and print result
- `--deploy` — Generate files
- `-param value` — Pass named arguments
- `--root <dir>` — Prepend directory to all file paths
- `--force` — Allow overwriting existing files
- `--git owner/repo/path` — Fetch from GitHub raw URL

## Examples

Avon comes with 20+ examples in `examples/`:

- **site_generator.av** — Generate a multi-page website
- **docker_compose_gen.av** — Generate Docker Compose configs
- **kubernetes_gen.av** — Generate Kubernetes manifests
- **github_actions_gen.av** — Generate CI/CD workflows
- **neovim_init.av** — Generate Neovim configuration
- **escape_hatch.av** — Demonstrate template escape hatch
- And more!

Run any example:

```bash
cargo run -- eval examples/site_generator.av
cargo run -- examples/docker_compose_gen.av --deploy --root ./gen --force
```

## Language Highlights

### String Interpolation

```avon
let name = "Alice" in
let age = 30 in
@/profile.txt {"
Name: {name}
Age: {age}
"}
```

### List Operations

```avon
let items = ["apple", "banana", "cherry"] in
map (\item concat "- " item) items
```

### Conditionals

```avon
if age > 18 then "adult" else "minor"
```

### Multi-file Generation

Return a list of FileTemplates to generate multiple files:

```avon
let make_file = \name @/{name}.txt {"{name}"} in
map make_file ["a", "b", "c"]
```

## Documentation

- **[Full Tutorial](./tutorial/TUTORIAL.md)** — Comprehensive guide to Avon syntax and features
- **[Features Reference](./tutorial/FEATURES.md)** — Quick reference for language features

## Development

```bash
# Build
cargo build
cargo build --release

# Run tests
bash scripts/run_examples.sh

# Check for warnings
RUSTFLAGS="-D warnings" cargo build
```

## Project Structure

```
src/
  main.rs       — CLI entry point
  cli.rs        — Command-line argument parsing
  lexer.rs      — Tokenization and template parsing
  parser.rs     — AST construction
  eval.rs       — Runtime evaluation
  common.rs     — Shared types and utilities

examples/       — 20+ example programs
tutorial/       — User documentation
scripts/        — Build and test scripts
```

## Tested Examples

All examples pass automated tests:

```
✅ site_generator.av
✅ docker_compose_gen.av
✅ kubernetes_gen.av
✅ github_actions_gen.av
✅ neovim_init.av
✅ emacs_init.av
✅ escape_hatch.av
... and 15+ more
```

## Design Philosophy

Avon is built on these principles:

1. **Simplicity first** — Small, focused language
2. **Functional** — Pure expressions; no side effects
3. **Practical** — Solves real file-generation problems
4. **Zero runtime dependency** — Single binary; works anywhere

## License

Avon is dual-licensed under MIT and Apache 2.0. You can choose whichever license works best for your use case:

- **MIT License** — Simple, permissive, business-friendly
- **Apache 2.0 License** — Includes explicit patent protection

See [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) for full details.

## Contributing

Contributions are welcome! Whether you're improving documentation, adding examples, or enhancing the language, we'd love your input.

## Future Ideas

- [ ] Additional examples (migrations, API specs, infra scaffolding)
- [ ] Performance optimizations
- [ ] Extended standard library
- [ ] Editor support (syntax highlighting)

---

**Ready to generate?** Start with the [Quick Start](#quick-start) or dive into the [Full Tutorial](./tutorial/TUTORIAL.md).
