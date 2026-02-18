# Avon Task Runner - Quick Start Guide

The Avon Task Runner lets you define and execute tasks with automatic dependency resolution.

## Basic Concept

Define tasks as a dictionary, where each task can be:
- A simple command string: `build: "cargo build"`
- A structured definition with dependencies:
  ```
  test: {
    cmd: "cargo test",
    deps: ["build"],
    desc: "Run tests"
  }
  ```

## Command Syntax

```bash
avon do <task_name> [avonfile]
```

- `<task_name>`: Name of the task to execute (required)
- `[avonfile]`: Path to your task definition file (optional, defaults to `Avonfile.av`)

## Examples

### Example 1: Simple Tasks

**File: `tasks.av`**
```avon
{
  build: "cargo build",
  test: "cargo test",
  clean: "cargo clean"
}
```

**Commands:**
```bash
avon do build tasks.av      # Runs: cargo build
avon do test tasks.av       # Runs: cargo test
avon do clean tasks.av      # Runs: cargo clean
```

### Example 2: Tasks with Dependencies

**File: `Avonfile.av`**
```avon
{
  check: "cargo check",
  build: {
    cmd: "cargo build",
    deps: ["check"]
  },
  test: {
    cmd: "cargo test",
    deps: ["build"]
  },
  release: {
    cmd: "cargo build --release",
    deps: ["test"]
  }
}
```

**Execution:**
```bash
avon do release              # Automatically runs: check → build → test → release
```

The task runner automatically determines the execution order based on dependencies!

### Example 3: Complex Workflows

```avon
{
  fmt: "cargo fmt",
  lint: "cargo clippy",
  build: {
    cmd: "cargo build",
    deps: ["fmt", "lint"]
  },
  test: {
    cmd: "cargo test",
    deps: ["build"]
  },
  doc: {
    cmd: "cargo doc --no-deps",
    deps: ["build"]
  },
  ci: {
    cmd: "echo All checks passed!",
    deps: ["test", "doc"]
  }
}
```

This creates a comprehensive CI pipeline:
```
fmt         lint         (run in parallel logically)
  \         /
   build
   /       \
  test    doc
   \       /
     ci
```

## Task Definition Formats

### Simple Format
```avon
{
  taskname: "shell command here"
}
```

### Structured Format
```avon
{
  taskname: {
    cmd: "shell command",           # Required: the actual command to run
    deps: ["other_task"],           # Optional: list of task dependencies
    desc: "Human readable description"  # Optional: description (for future --list)
  }
}
```

## Error Handling

### Task Not Found
```bash
$ avon do nonexistent Avonfile.av
Error running task 'nonexistent': Error: Task 'nonexistent' not found
```

### Undefined Dependency
```bash
$ avon do build Avonfile.av
Error initializing task runner: Error: Task 'build' depends on 'nonexistent' which does not exist
```

### Cyclic Dependency
```bash
$ avon do a Avonfile.av
Error initializing task runner: Error: Cyclic dependency detected
```

## Tips & Best Practices

1. **Keep task names simple** - Use lowercase with underscores: `build_release`, `run_tests`
2. **Document complex tasks** - Use the `desc` field to explain what a task does
3. **Use dependencies** - Don't manually call dependencies; let Avon handle the ordering
4. **One task per purpose** - Make tasks focused and composable
5. **Test your tasks** - Run `avon do taskname` before relying on them

## What's Coming Soon (Phase 2)

- `avon do --list [file]` - Show all available tasks
- `avon do --info taskname [file]` - Show task details
- `avon do --dry-run taskname [file]` - Show execution plan without running
- `Avonfile.av` auto-discovery - Automatically find default file
- Environment variable support in task commands

## Advanced Features (Phase 2+)

- Multiple task execution: `avon do task1 task2 task3`
- Task output filtering and logging
- Conditional task execution
- Task result inspection
- Integration with other Avon features

---

**Current Phase:** 1 (Core functionality)
**Status:** Fully functional for basic and intermediate use cases
