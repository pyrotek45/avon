# Avon Do Mode — Task Runner Guide

Avon's **do mode** is a built-in task runner that lets you define, organize, and execute
shell commands with dependency resolution, environment variables, and helpful error messages.
Think of it as a lightweight alternative to Make, Just, or npm scripts — powered by Avon's
native data format.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Avon vs Make vs Just](#avon-vs-make-vs-just)
3. [Task File Format](#task-file-format)
4. [Simple Tasks](#simple-tasks)
5. [Structured Tasks](#structured-tasks)
6. [Multiple Commands](#multiple-commands)
7. [String Manipulation & Generated Commands](#string-manipulation--generated-commands)
8. [Dependencies](#dependencies)
9. [Environment Variables](#environment-variables)
10. [CLI Flags](#cli-flags)
11. [Auto-Discovery (Avon.av)](#auto-discovery)
12. [Error Messages & Typo Suggestions](#error-messages)
13. [Gotchas & Edge Cases](#gotchas)
14. [Advanced Examples](#advanced-examples)
15. [Real-World Examples](#real-world-examples)

---

## Quick Start

Create a file called `Avon.av` in your project root:

```
{
  build: "cargo build --release",
  test: "cargo test",
  clean: "cargo clean"
}
```

Then run a task:

```sh
avon do build              # runs 'cargo build --release'
avon do test               # runs 'cargo test'
avon do --list             # shows all available tasks
```

That's it! Avon automatically finds `Avon.av` in the current directory.

---

## Avon vs Make vs Just

Avon's do mode brings the power of a full programming language to task running.
Here's how it compares:

### Simple Task Definition

**Problem:** You have a handful of shell commands you run frequently. You want them in one place with clear names.

**Make:**
```makefile
build:
	cargo build --release

test:
	cargo test

clean:
	cargo clean
```

**Just:**
```just
build:
	cargo build --release

test:
	cargo test

clean:
	cargo clean
```

**Avon:**
```avon
{
  build: "cargo build --release",
  test: "cargo test",
  clean: "cargo clean"
}
```

✅ **Avon advantages:**
- Cleaner, more readable syntax
- Valid Avon/JSON data structure (no special parsing rules)
- Can be manipulated programmatically
- Integrates with other Avon programs

---

### Tasks with Dependencies

**Problem:** You have tasks that must run in a specific order. Task B needs Task A to finish first. What if you forget to run A, or define them in the wrong order?

**Make:**
```makefile
clean:
	rm -rf target

build: clean
	cargo build --release

test: build
	cargo test

deploy: test
	./deploy.sh
```

**Just:**
```just
clean:
	rm -rf target

build: clean
	cargo build --release

test: build
	cargo test

deploy: test
	./deploy.sh
```

**Avon:**
```avon
{
  clean: "rm -rf target",
  build: {cmd: "cargo build --release", deps: ["clean"]},
  test: {cmd: "cargo test", deps: ["build"]},
  deploy: {cmd: "./deploy.sh", deps: ["test"]}
}
```

✅ **Avon advantages:**
- Dependencies are explicit data, not implicit ordering
- Works correctly even if tasks are defined out of order
- `avon do --dry-run` shows the full execution plan before running
- Handles complex dependency graphs (diamond dependencies)
- Each task runs exactly once, even if multiple tasks depend on it

---

### Tasks with Descriptions

**Problem:** You want users to understand what each task does without reading the file. `make --help` doesn't show task descriptions.

**Make:**
```makefile
# This comment is for developers
build:
	cargo build --release

# Actually just runs tests
test:
	cargo test
```

No standard way to show task descriptions to users.

**Just:**
```just
# Build the project
build:
	cargo build --release

# Run tests
test:
	cargo test
```

Comments work, but `just --list` doesn't show them.

**Avon:**
```avon
{
  build: {
    cmd: "cargo build --release",
    desc: "Build the project in release mode"
  },
  test: {
    cmd: "cargo test",
    desc: "Run all tests"
  }
}
```

Run `avon do --list` to see descriptions without reading the file.

✅ **Avon advantages:**
- Descriptions are first-class data, not comments
- `avon do --list` automatically shows all tasks with descriptions
- Descriptions are optional but recommended (can be queried programmatically)
- Team members and CI scripts can query task information programmatically

---

### Tasks with Environment Variables

**Problem:** You want tasks to use environment variables without cluttering the command line. How do you document which env vars a task needs? What are the defaults?

**Make:**
```makefile
build:
	RUST_LOG=debug cargo build --release

test:
	RUST_ENV=testing cargo test

deploy:
	DEPLOY_ENV=prod DEPLOY_VERSION=1.0.0 ./deploy.sh
```

Env vars are embedded in shell syntax. Hard to extract or document. Defaults are hidden in the code.

**Just:**
```just
build:
	RUST_LOG=debug cargo build --release

test:
	RUST_ENV=testing cargo test

deploy:
	DEPLOY_ENV=prod DEPLOY_VERSION=1.0.0 ./deploy.sh
```

Same problem as Make.

**Avon:**
```avon
{
  build: {
    cmd: "cargo build",
    env: {RUST_LOG: "debug"}
  },
  test: {
    cmd: "cargo test",
    env: {RUST_ENV: "testing"}
  },
  deploy: {
    cmd: "./deploy.sh",
    env: {
      DEPLOY_ENV: "prod",
      DEPLOY_VERSION: "1.0.0"
    }
  }
}
```

✅ **Avon advantages:**
- Environment variables are structured data (a dict), not embedded in shell strings
- `avon do --info build` shows what env vars the task will use
- Defaults are explicit and visible
- Easy to override from command line: `RUST_LOG=warn avon do build`
- Cleaner to read and maintain
- Less chance of shell injection bugs

---

### Diamond Dependencies (Shared Prerequisites)

**Problem:** Multiple tasks depend on the same shared task. Will it run multiple times (wasting time) or just once?

**Make:**
```makefile
setup:
	echo "Setting up..."

frontend: setup
	echo "Building frontend"

backend: setup
	echo "Building backend"

deploy: frontend backend
	echo "Deploying"
```

When you run `make deploy`, `setup` runs ONCE (automatically), then `frontend` and `backend`, then `deploy`. But this isn't guaranteed for complex graphs.

**Just:**
```just
setup:
	echo "Setting up..."

frontend: setup
	echo "Building frontend"

backend: setup
	echo "Building backend"

deploy: frontend backend
	echo "Deploying"
```

Same as Make.

**Avon:**
```avon
{
  setup: "echo Setting up...",
  frontend: {cmd: "echo Building frontend", deps: ["setup"]},
  backend: {cmd: "echo Building backend", deps: ["setup"]},
  deploy: {cmd: "echo Deploying", deps: ["frontend", "backend"]}
}
```

`avon do deploy` runs: setup → frontend + backend → deploy

✅ **Avon advantages:**
- Topological sort is guaranteed by design
- You can preview the plan with `avon do --dry-run deploy`
- Each task runs exactly once, regardless of how many tasks depend on it
- Works perfectly for complex dependency graphs (not just simple chains)

---

## Task File Format

A task file is a standard Avon dictionary where each key is a task name
and each value is either a command string or a structured definition.

```
{
  task_name: "shell command",
  other_task: {cmd: "shell command", deps: ["task_name"]}
}
```

Task files are regular `.av` files, so you can use all of Avon's features
(variables, functions, templates) to generate your task definitions dynamically.

---

## Simple Tasks

The simplest task is just a name mapped to a shell command string:

```
{
  greet: "echo 'Hello, world!'",
  build: "cargo build",
  test: "cargo test"
}
```

Run with:

```sh
avon do greet myfile.av
```

---

## Structured Tasks

For more control, use a dictionary with these fields:

| Field  | Type              | Required | Description                    |
|--------|-------------------|----------|--------------------------------|
| `cmd`  | string or list    | yes      | Shell command(s) to execute    |
| `deps` | list of strings   | no       | Tasks that must run first      |
| `desc` | string            | no       | Human-readable description     |
| `env`  | dict of strings   | no       | Environment variables to set   |

**Note:** `cmd` can be a single string or a list of strings. When a list is provided,
commands are joined with `&&` and execute sequentially, stopping on the first failure.

```
{
  build: {
    cmd: "cargo build --release",
    desc: "Build the project in release mode",
    deps: ["lint", "test"],
    env: {RUST_LOG: "info"}
  },
  lint: "cargo clippy",
  test: "cargo test"
}
```

---

## Multiple Commands

You can run multiple commands sequentially in a single task by using a list for the `cmd` field.
Commands are joined with `&&`, so if any command fails, the task stops immediately.

### Basic Usage

```
{
  build_and_test: {
    cmd: [
      "cargo build",
      "cargo test",
      "cargo clippy"
    ]
  }
}
```

This is equivalent to running:
```bash
cargo build && cargo test && cargo clippy
```

### With Dependencies and Descriptions

```
{
  ci: {
    cmd: [
      "cargo fmt --check",
      "cargo clippy -- -D warnings",
      "cargo test --all"
    ],
    deps: ["clean"],
    desc: "Full CI pipeline: format, lint, and test",
    env: {RUST_LOG: "debug"}
  }
}
```

### When to Use Multiple Commands

Use `cmd` lists when:
- Steps are tightly coupled (failure of one means skip the rest)
- You want the sequence visible in one task
- The combined output matters

Use separate tasks with `deps` when:
- You need different descriptions for each step
- Individual steps are reusable
- You want finer-grained control over which steps to run

---

## String Manipulation & Generated Commands

One of Avon's superpowers in do mode is **generating commands dynamically** using string manipulation.
Since task files are Avon programs, you can concatenate strings, use conditionals, and apply functions
to build command strings that adapt to your environment.

### Problem: Hardcoded Commands Don't Scale

In Make/Just, when you need variants of a command, you have three bad options:

**Option 1: Copy-paste duplication**
```makefile
build_dev:
	cargo build --profile dev

build_release:
	cargo build --profile release

build_custom:
	cargo build --profile custom
```

Problem: If you need to change the base command, you have to edit it in 3 places.

**Option 2: Shell variables (confusing syntax)**
```makefile
PROFILE ?= dev
build:
	cargo build --profile $(PROFILE)
```

Problem: Variables are weakly typed, mixed with shell syntax, hard to compose.

**Option 3: External scripts (maintenance nightmare)**
```makefile
build:
	@bash scripts/build.sh
```

Problem: You now have to maintain separate shell scripts. Logic is scattered.

**Avon's solution: String manipulation as first-class feature**

### Basic String Concatenation

Build commands by combining strings with the `+` operator:

```avon
let version = env_var_or "VERSION" "1.0.0" in
let registry = "ghcr.io/mycompany" in
let image_tag = registry + "/myapp:" + version in

{
  docker_build: {
    cmd: "docker build -t " + image_tag + " .",
    desc: "Build Docker image " + image_tag
  }
}
```

✅ **Advantages:**
- One source of truth (the `image_tag` variable)
- Reusable across multiple tasks
- Description matches the actual command (both use `image_tag`)

### Conditional Command Strings

Use `if/then/else` to adjust commands based on environment:

```avon
let is_release = (env_var_or "PROFILE" "debug") == "release" in
let flags = if is_release then "--release --locked" else "" in

{
  build: {
    cmd: "cargo build " + flags,
    desc: if is_release then "Build release binary" else "Build debug binary"
  }
}
```

✅ **Advantages:**
- No code duplication (one build task handles both profiles)
- Conditions are type-checked by Avon (not shell quoting issues)
- Easy to understand intent (the `is_release` variable is self-documenting)

### Reusable Command Templates with Functions

Define functions that generate command strings:

```avon
let cargo_test = \target "cargo test --lib " + target in
let docker_build = \tag \path "docker build -t " + tag + " " + path in

{
  test_api: {cmd: cargo_test "-p api", desc: "Test api crate"},
  test_web: {cmd: cargo_test "-p web", desc: "Test web crate"},
  
  docker_api: {cmd: docker_build "api:v1" "./services/api"},
  docker_web: {cmd: docker_build "web:v1" "./services/web"}
}
```

✅ **Advantages:**
- Define the command pattern once
- Apply it to multiple tasks
- Consistent commands across your project
- Easy to refactor (change the function, all tasks update)

### Complete Example: Configuration-Driven Tasks

Here's a realistic example that shows all these techniques together:

```avon
# Configuration
let version = env_var_or "VERSION" "1.0.0" in
let env = env_var_or "ENV" "dev" in
let is_prod = (env == "prod") in

let registry = "ghcr.io/mycompany" in
let image = registry + "/myapp:" + version in
let log_level = if is_prod then "warn" else "debug" in
let profile = if is_prod then "release" else "debug" in

# Command templates
let cargo_cmd = \action \opts "cargo " + action + " --profile " + profile + " " + opts in
let deploy_cmd = \app \ver "sh scripts/deploy.sh " + app + " " + ver + " " + env in

let tasks = {
  build: {
    cmd: cargo_cmd "build" "--locked",
    desc: "Build for " + env + " (profile: " + profile + ")"
  },
  
  test: {
    cmd: cargo_cmd "test" "--all",
    desc: "Run tests with RUST_LOG=" + log_level,
    deps: ["build"],
    env: {RUST_LOG: log_level}
  },
  
  docker: {
    cmd: "docker build -t " + image + " .",
    desc: "Build Docker image " + image,
    deps: ["build"]
  },
  
  deploy: {
    cmd: deploy_cmd "myapp" version,
    desc: "Deploy version " + version + " to " + env,
    deps: ["test", "docker"],
    env: {IMAGE: image, VERSION: version}
  }
} in

tasks
```

Usage:
```sh
avon do build Avon.av                           # debug profile
ENV=prod avon do deploy Avon.av                 # release profile, production env
VERSION=2.0.0 ENV=prod avon do build Avon.av    # custom version, production
```

### Real-World Problem: Docker Image Tagging

**Without string manipulation (Make):**
```makefile
# Fragile—must update in multiple places
DOCKER_REGISTRY = ghcr.io/mycompany
APP_NAME = myapp
VERSION = 1.0.0

build:
	cargo build --release

docker_build:
	docker build -t $(DOCKER_REGISTRY)/$(APP_NAME):$(VERSION) .

docker_push:
	docker push $(DOCKER_REGISTRY)/$(APP_NAME):$(VERSION)

docker_build_latest:
	docker build -t $(DOCKER_REGISTRY)/$(APP_NAME):latest .
```

Problem: Image tag is repeated 3 times. If `VERSION` changes, you must remember to rebuild all tasks.

**With Avon string manipulation:**
```avon
let registry = "ghcr.io/mycompany" in
let app = "myapp" in
let version = env_var_or "VERSION" "1.0.0" in
let image_tag = registry + "/" + app + ":" + version in
let latest_tag = registry + "/" + app + ":latest" in

{
  build: "cargo build --release",
  
  docker_build: {
    cmd: "docker build -t " + image_tag + " .",
    deps: ["build"],
    desc: "Build Docker image: " + image_tag
  },
  
  docker_push: {
    cmd: "docker push " + image_tag,
    deps: ["docker_build"]
  },
  
  docker_tag_latest: {
    cmd: "docker tag " + image_tag + " " + latest_tag,
    deps: ["docker_build"]
  }
}
```

✅ **Avon advantages:**
- One source of truth for `image_tag` (changed once, used everywhere)
- No duplication = fewer bugs
- Version is configurable via environment (`VERSION=2.0.0 avon do ...`)
- Description automatically shows the computed image tag
- Logic is in the task file (no separate shell scripts)

### Why This Matters

**Make/Just can't do this:**
```makefile
# Make - no string manipulation, must use external scripts
build:
	@bash scripts/build.sh $(PROFILE)
```

You're forced into one of two camps:
1. Duplicate code in multiple task definitions
2. Push logic into external shell scripts (maintenance nightmare)

**Avon lets you have it both ways:**
```avon
# Avon - full string manipulation right in the task file
let profile = if is_prod then "release" else "debug" in
{
  build: {cmd: "cargo build --profile " + profile}
}
```

✅ **Avon advantages over Make/Just:**
- No separate shell scripts needed
- Type-safe string building (Avon syntax is validated)
- Full language power (functions, conditionals, pattern matching, recursion)
- Commands are computed, not hardcoded
- Easy to test and debug
- Single file, easy to understand the full task system
- Automatic description generation from computed values

---

## Dependencies

Tasks can depend on other tasks. Avon resolves the full dependency graph
using topological sort, so each task runs exactly once in the correct order.

```
{
  clean: "rm -rf target",
  build: {cmd: "cargo build", deps: ["clean"]},
  test: {cmd: "cargo test", deps: ["build"]},
  deploy: {cmd: "./deploy.sh", deps: ["test"]}
}
```

Running `avon do deploy` will execute: `clean` → `build` → `test` → `deploy`.

### Diamond Dependencies

Avon handles shared dependencies correctly. If two tasks depend on the same
prerequisite, it only runs once:

```
{
  setup: "echo setup",
  frontend: {cmd: "echo frontend", deps: ["setup"]},
  backend: {cmd: "echo backend", deps: ["setup"]},
  deploy: {cmd: "echo deploy", deps: ["frontend", "backend"]}
}
```

Running `avon do deploy` executes `setup` only once, then both `frontend`
and `backend`, then `deploy`.

### Cycle Detection

Avon detects circular dependencies and reports a clear error:

```
{
  a: {cmd: "echo a", deps: ["b"]},
  b: {cmd: "echo b", deps: ["a"]}
}
```

```
Error initializing task runner: Error: Cyclic dependency detected: a
```

> **Note:** The specific task name reported in a cycle error may vary between runs
> (e.g., `a` or `b`). What matters is that the cycle is detected and reported.

---

## Environment Variables

Tasks can define environment variables that are set during execution.
Avon also expands `$VAR` and `${VAR}` references in the command string.

### Basic Usage

```
{
  greet: {
    cmd: "echo Hello, $NAME!",
    env: {NAME: "Developer"}
  }
}
```

Output: `Hello, Developer!`

### Braces Syntax

Use `${VAR}` when the variable is adjacent to other text:

```
{
  tag: {
    cmd: "echo ${APP}-v${VERSION}",
    env: {APP: "myapp", VERSION: "1.0.0"}
  }
}
```

Output: `myapp-v1.0.0`

### System Fallback

If a variable isn't defined in `env`, Avon falls back to your system
environment variables:

```
{
  whoami: {cmd: "echo Running as $USER in $HOME"}
}
```

### Priority

Task-level `env` values take priority over system environment variables.

---

## CLI Flags

### `--dry-run` — Preview Execution Plan

See what would run without executing anything:

```sh
avon do --dry-run deploy Avon.av
```

```
Execution Plan:
================
1. clean (cmd: rm -rf target)
2. build (cmd: cargo build)
   deps: clean
3. test (cmd: cargo test)
   deps: build
4. deploy (cmd: ./deploy.sh)
   deps: test
```

### `--list` — Show All Tasks

List every task with its description and command:

```sh
avon do --list Avon.av
```

```
Available Tasks:
================
build
  Description: Build the project
  Command: cargo build
  Dependencies: clean

clean
  Command: rm -rf target

test
  Command: cargo test
  Dependencies: build
```

**Note:** Tasks without descriptions (like `clean` above) are still listed.
The "Description:" line is simply omitted if `desc` is not provided.
This way, you can see all available tasks even if some lack descriptions.

### `--info` — Task Details

Get detailed information about a specific task:

```sh
avon do --info build Avon.av
```

```
Task: build
Command: cargo build --release
Description: Build the project in release mode
Dependencies: lint, test
Environment Variables:
  RUST_LOG: info
```

---

## How Avon Finds Your Task File

When you run an `avon do` command, Avon resolves the source file using
this priority order:

1. **Explicit file argument** — `avon do build tasks.av` reads `tasks.av`
2. **Auto-discovery** — If no file is given, Avon looks for `Avon.av`
   in the current directory

If none of these succeed, Avon prints an error with usage hints.

> **Security:** The `--git` and `--stdin` flags are **blocked** for `do` mode.
> Running shell commands from a remote URL or piped input is a security risk
> (remote code execution). If you want to use a remote task file, download
> and review it first:
> ```sh
> avon eval --git user/repo/tasks.av > tasks.av
> cat tasks.av          # review the file
> avon do build tasks.av
> ```

### Auto-Discovery

If you don't specify a file, Avon looks for `Avon.av` in the current
directory:

```sh
avon do build          # same as: avon do build Avon.av
avon do --list         # same as: avon do --list Avon.av
avon do --info build   # same as: avon do --info build Avon.av
avon do --dry-run test # same as: avon do --dry-run test Avon.av
```

This makes it easy to set up project-level task definitions that your
whole team can use without remembering file paths.

### Do Mode vs Eval vs Deploy

Avon has three ways to process an `.av` file. Each expects different content:

| Mode     | Command                | What It Does                              | File Must Produce         |
|----------|------------------------|-------------------------------------------|---------------------------|
| `eval`   | `avon eval file.av`    | Evaluate and print the result             | Any valid Avon expression |
| `deploy` | `avon deploy file.av`  | Write generated files to disk             | FileTemplate(s)           |
| `do`     | `avon do build file.av`| Run shell tasks with dependency resolution| A dict of task definitions|

All three modes share the same Avon evaluation pipeline: lex → parse → eval.
The difference is what happens **after** evaluation:

- **eval** — Prints the result to stdout. Nothing is written to disk.
- **deploy** — Expects the result to be a `FileTemplate` (or list of them)
  and writes them to disk. If the result isn't deployable, it errors.
- **do** — Expects the result to be a dictionary where each key is a task
  name and each value is a command string or structured task definition.
  It extracts the tasks and runs the requested one (with dependencies).

---

## Error Messages

Avon provides clear, actionable error messages with typo suggestions.

### Task Not Found (with suggestion)

```sh
avon do bild
```

```
Error running task 'bild': Error: Task 'bild' not found. Did you mean 'build'?
```

### Undefined Dependency (with suggestion)

If a task references a dependency that doesn't exist but is close to an
existing task name:

```
Error initializing task runner: Error: Task 'test' depends on 'bild' which does not exist. Did you mean 'build'?
```

### No Suggestion

When the typo is too far from any known task, no suggestion is shown:

```
Error running task 'xyzzy': Error: Task 'xyzzy' not found
```

### Nonexistent Task with `--info`

```sh
avon do --info nonexistent Avon.av
```

```
Error: Task 'nonexistent' not found
Available tasks: build, test, clean
```

---

## Gotchas & Edge Cases

### `cmd` is Required for Structured Tasks

If you use a dict for a task definition, the `cmd` field is required.
A task with only `desc` or `deps` will produce an error:

```
{
  info: {desc: "Just a description"}
}
```

```
Error: Task 'info' has invalid format: missing required 'cmd' field
```

### Dry-Run Shows Unexpanded Variables

The `--dry-run` flag shows commands **before** environment variable expansion.
You'll see `${APP}:${TAG}` in the plan, but the actual execution will
substitute the values.

### Undefined Environment Variables

If a `$VAR` reference isn't found in the task's `env` dict, Avon falls back
to system environment variables. If the variable isn't defined anywhere,
the shell handles it — typically expanding to an empty string.

### Cycle Detection is Non-Deterministic

When a cyclic dependency is found, the specific task name reported in the
error may vary between runs. The cycle is always detected, but the entry
point into the cycle depends on internal iteration order.

### Task Execution Output

When a task runs, Avon prints a `Running: <task_name>` header (plus the
description if one exists), then the command's output, then a success
message. If a task in a dependency chain fails, execution stops immediately.

### No Task Name Given

Running `avon do` without a task name or flag produces:

```
Error: 'do' command requires a task name
  Usage: avon do <task_name> [file]
  Example: avon do build
```

### Invalid File Content

If the file doesn't evaluate to a dictionary of tasks, you get:

```
Error: Parse error: Expected a dictionary of tasks
  Make sure your Avon.av contains task definitions
  Example task format: {build: "cargo build", test: "cargo test"}
```

### `--git` and `--stdin` Are Blocked

For security, `do` mode only accepts local files. Attempting to use
`--git` or `--stdin` produces an error:

```
Error: --git is not allowed with 'do' mode
  Running shell commands from a remote source is a security risk.
  Download the file first, review it, then run locally:
    avon eval --git user/repo/tasks.av > tasks.av
    avon do build tasks.av
```

This prevents remote code execution attacks. The `eval` and `deploy` modes
still support `--git` and `--stdin` since they don't execute shell commands.

---

## Advanced Examples

Avon's true power comes from combining do mode with Avon's full programming language.
Since task files are Avon programs, you can use variables, functions, conditionals,
and string interpolation to generate task definitions dynamically.

### Example 1: Parameterized Tasks with Environment

**Problem:** Different environments (dev/staging/prod) need different commands and variables.

**Make/Just Solution:** Use environment variables or `.env` files (external system).

**Avon Solution:**

```avon
let env = env_var_or "ENV" "dev" in
let is_debug = (env == "dev") in
let mode = if is_debug then "debug" else "release" in

let tasks = {
  build: {
    cmd: "echo Building in " + mode + " mode for " + env,
    desc: "Build for dev or prod"
  },
  test: {
    cmd: "cargo test",
    deps: ["build"]
  },
  deploy: {
    cmd: "sh scripts/deploy.sh " + env,
    desc: "Deploy to environment",
    deps: ["test"],
    env: {DEPLOY_ENV: env}
  }
} in

tasks
```

Usage:
```bash
avon do build                  # Builds in debug mode
ENV=prod avon do deploy        # Builds release, then deploys to prod
```

✅ Avon advantage: Logic is in the task file, not scattered across scripts. Full control over which commands run for which environments.

---

### Example 2: Generated Tasks from Lists

**Problem:** Run the same task for multiple inputs (e.g., test multiple services).

**Make/Just Solution:** Write repetitive rules or use a generator script.

**Avon Solution:**

```avon
let services = ["api", "web", "worker"] in

let test_service = \svc {
  cmd: "echo Testing " + svc + " service",
  desc: "Test " + svc + " service"
} in

let docker_service = \svc {
  cmd: "echo Building Docker image for " + svc,
  desc: "Build Docker image for " + svc,
  deps: ["test_" + svc]
} in

let tasks = {
  # Generate test tasks
  test_api: test_service "api",
  test_web: test_service "web",
  test_worker: test_service "worker",
  
  # Generate docker tasks
  docker_api: docker_service "api",
  docker_web: docker_service "web",
  docker_worker: docker_service "worker",
  
  # Aggregate task
  build_all: {
    cmd: "echo All services built",
    deps: ["docker_api", "docker_web", "docker_worker"]
  }
} in

tasks
```

Run `avon do --list` to see all generated tasks:
```
test_api       Test api service
test_web       Test web service
test_worker    Test worker service
docker_api     Build Docker image for api
docker_web     Build Docker image for web
docker_worker  Build Docker image for worker
build_all      ...
```

✅ Avon advantage: No duplication. Tasks are generated from data. Add a service? Update the list once.

---

### Example 3: Conditional Task Chains

**Problem:** Some tasks should only run in certain conditions (CI vs local, or based on changed files).

**Avon Solution:**

```avon
let is_ci = ((env_var_or "CI" "false") == "true") in
let run_slow_tests = ((env_var_or "FULL_TEST" "false") == "true") in

let tasks = {
  # Always run
  fmt: {
    cmd: "cargo fmt --check",
    desc: "Check formatting"
  },
  
  lint: {
    cmd: "cargo clippy -- -D warnings",
    desc: "Lint with clippy",
    deps: ["fmt"]
  },
  
  # Quick tests
  test_quick: {
    cmd: "cargo test --lib",
    desc: "Quick unit tests",
    deps: ["lint"]
  },
  
  # Slow tests only in CI or on demand
  test_integration: if run_slow_tests then {
    cmd: "cargo test --test '*'",
    desc: "Integration tests",
    deps: ["test_quick"]
  } else {
    cmd: "echo Skipping integration tests (set FULL_TEST=true to run)",
    deps: ["test_quick"]
  },
  
  # In CI, also build docs
  ci: if is_ci then {
    cmd: "echo CI pipeline complete",
    desc: "Full CI pipeline",
    deps: ["test_integration", "build_docs"]
  } else {
    cmd: "echo Local build (not full CI)",
    deps: ["test_integration"]
  },
  
  build_docs: {
    cmd: "cargo doc --no-deps",
    desc: "Build documentation"
  }
} in

tasks
```

Usage:
```bash
avon do test_quick              # Just quick tests
FULL_TEST=true avon do ci       # Full suite locally
CI=true avon do ci              # Full suite in CI (with docs)
```

✅ Avon advantage: Task definitions adapt to conditions. Same file, different behavior. No environment pollution.

---

### Example 4: Shared Configuration & DRY

**Problem:** Same configuration (paths, versions, flags) repeated across many tasks.

**Avon Solution:**

```avon
# Define configuration once
let config = {
  rust_version: "1.70.0",
  target_dir: "target",
  docker_registry: "ghcr.io/myorg",
  docker_repo: "myapp",
  tag: env_var_or "VERSION" "latest"
} in

# Build docker tag from config
let docker_tag = config.docker_registry + "/" + config.docker_repo + ":" + config.tag in

let tasks = {
  check: {
    cmd: "cargo check --all-targets",
    desc: "Check all targets"
  },
  
  test: {
    cmd: "cargo test --all",
    desc: "Run all tests",
    deps: ["check"]
  },
  
  build: {
    cmd: "cargo build --release",
    desc: "Build release binary",
    deps: ["test"]
  },
  
  docker_build: {
    cmd: "docker build -t " + docker_tag + " .",
    desc: "Build Docker image",
    deps: ["build"]
  },
  
  docker_push: {
    cmd: "docker push " + docker_tag,
    desc: "Push Docker image",
    deps: ["docker_build"]
  },
  
  release: {
    cmd: "gh release create v" + config.tag + " --title Release " + config.tag,
    desc: "Create GitHub release",
    deps: ["docker_push"]
  }
} in

tasks
```

Usage:
```bash
avon do docker_build             # Uses tag=latest
VERSION=1.2.0 avon do release   # Updates tag everywhere: docker and GitHub
```

✅ Avon advantage: Single source of truth. Change the version once, all tasks update automatically.

---

### Example 5: Complex Multi-Step Workflows

**Problem:** Workflows with multiple parallel and sequential stages.

**Avon Solution:**

```avon
let profile = env_var_or "PROFILE" "debug" in
let num_jobs = env_var_or "JOBS" "4" in

let tasks = {
  # Stage 1: Validation
  fmt: {cmd: "cargo fmt --check", desc: "Format check"},
  lint: {cmd: "cargo clippy -- -D warnings", desc: "Lint check"},
  
  # Stage 2: Build (depends on validation)
  build: {
    cmd: "cargo build --profile " + profile + " -j " + num_jobs,
    desc: "Build in " + profile + " mode",
    deps: ["fmt", "lint"]
  },
  
  # Stage 3: Test (parallel)
  test_unit: {
    cmd: "cargo test --lib",
    deps: ["build"]
  },
  test_integration: {
    cmd: "cargo test --test '*'",
    deps: ["build"]
  },
  test_doc: {
    cmd: "cargo test --doc",
    deps: ["build"]
  },
  
  # Stage 4: Finalize
  coverage: {
    cmd: "cargo tarpaulin --out Lcov",
    desc: "Generate coverage report",
    deps: ["test_unit", "test_integration", "test_doc"]
  },
  
  # Pipeline
  ci: {
    cmd: "echo All checks passed!",
    desc: "Full CI pipeline",
    deps: ["coverage"]
  }
} in

tasks
```

Run:
```bash
avon do --dry-run ci    # See the full execution plan
avon do ci              # Execute everything in order
```

Output:
```
Execution Plan:
================
1. fmt (cmd: cargo fmt --check)
2. lint (cmd: cargo clippy -- -D warnings)
3. build (cmd: cargo build --profile debug -j 4)
   deps: fmt, lint
4. test_unit (cmd: cargo test --lib)
   deps: build
5. test_integration (cmd: cargo test --test '*')
   deps: build
6. test_doc (cmd: cargo test --doc)
   deps: build
7. coverage (cmd: cargo tarpaulin --out Lcov)
   deps: test_unit, test_integration, test_doc
8. ci (cmd: echo All checks passed!)
   deps: coverage
```

✅ Avon advantage: Complex workflows are readable. Parallel tasks implicit in dependency graph. Can preview before running.

---

## Real-World Examples

### Rust Project

```
{
  fmt: "cargo fmt",
  lint: "cargo clippy -- -D warnings",
  test: {cmd: "cargo test", deps: ["fmt", "lint"]},
  build: {cmd: "cargo build --release", deps: ["test"]},
  clean: "cargo clean"
}
```

### Node.js Project

```
{
  install: "npm ci",
  lint: {cmd: "npx eslint src/", deps: ["install"]},
  test: {cmd: "npx jest", deps: ["install"]},
  build: {cmd: "npx vite build", deps: ["lint", "test"]},
  dev: {cmd: "npx vite", deps: ["install"]}
}
```

### Docker Deployment

```
{
  build_image: {
    cmd: "docker build -t ${APP}:${TAG} .",
    desc: "Build Docker image",
    env: {APP: "myservice", TAG: "latest"}
  },
  push: {
    cmd: "docker push ${REGISTRY}/${APP}:${TAG}",
    desc: "Push image to registry",
    deps: ["build_image"],
    env: {REGISTRY: "ghcr.io/myorg", APP: "myservice", TAG: "latest"}
  },
  deploy: {
    cmd: "kubectl apply -f k8s/",
    desc: "Deploy to Kubernetes",
    deps: ["push"]
  }
}
```

### Multi-Stage CI Pipeline

See `examples/tasks_pipeline.av` for a complete 8-stage pipeline with
environment variables and deep dependency chains.

---

## Summary

| Feature                | Syntax                                    |
|------------------------|-------------------------------------------|
| Simple task            | `name: "command"`                         |
| Task with deps         | `name: {cmd: "...", deps: [...]}`         |
| Description            | `name: {cmd: "...", desc: "..."}`         |
| Environment vars       | `name: {cmd: "echo $VAR", env: {VAR: "val"}}` |
| Run a task             | `avon do <task> [file]`                   |
| Dry-run                | `avon do --dry-run <task> [file]`         |
| List tasks             | `avon do --list [file]`                   |
| Task info              | `avon do --info <task> [file]`            |
| Auto-discovery         | `avon do <task>` (uses Avon.av)       |
