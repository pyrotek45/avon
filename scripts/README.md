# Test Scripts

This directory contains various test scripts for the Avon project. These scripts help verify that all examples compile and run correctly with the simplified error handling system.

## Available Scripts

### `test_all_examples.sh`
**Purpose:** Quick pass/fail check for all example files.

**Usage:**
```bash
# Normal mode - shows pass/fail status
./scripts/test_all_examples.sh

# Verbose mode - shows full output of each example
./scripts/test_all_examples.sh --show-output
./scripts/test_all_examples.sh -v
```

**What it does:**
- Compiles avon
- Runs every `.av` file in the `examples/` directory
- Reports which ones pass and which ones fail
- In verbose mode, displays the full output of each example

### `test_example_outputs.sh`
**Purpose:** Validate that examples produce expected output patterns.

**Usage:**
```bash
./scripts/test_example_outputs.sh
```

**What it does:**
- Tests specific examples for expected output patterns
- Catches issues like:
  - Wrong function names (e.g., `str` instead of `to_string`)
  - Incorrect formatting
  - Missing features
  - Unexpected errors
- Tests 53 specific output patterns across key examples

**Examples of tests:**
- Formatting functions produce correct hex/binary/currency output
- JSON parsing works with map operations
- HTML/Markdown generation functions work correctly
- Config generators produce valid syntax
- No "unknown symbol" errors in examples

This script is particularly useful for catching regressions and ensuring examples stay up-to-date with the language.

### `run_examples.sh`
**Purpose:** Comprehensive integration tests with deployment testing.

**Usage:**
```bash
./scripts/run_examples.sh
```

**What it does:**
- Full integration test suite
- Tests both `eval` and `--deploy` modes
- Validates file creation and content
- Tests overwrite protection
- Runs 76 total tests including deployment scenarios

## Test Philosophy

1. **`test_all_examples.sh`** - Quick smoke test to ensure nothing is broken
2. **`test_example_outputs.sh`** - Validates correctness of output
3. **`run_examples.sh`** - Full integration testing with deployments

## Error Handling

As of the latest refactoring, the Avon error system has been simplified to provide direct, clear error messages:

- **Simple Format**: `function_name: type_error` (e.g., `concat: type mismatch: expected string, found 8080, /api`)
- **No Visual Artifacts**: Clean single-line output without pretty-printing
- **Complete Call Chains**: Nested function errors show the full call chain (e.g., `map: add_one: +: expected String, found Number`)

See [ERROR_SIMPLIFICATION.md](../ERROR_SIMPLIFICATION.md) for detailed information about the error system.

## Adding New Tests

### To add a new example to output validation:

Edit `test_example_outputs.sh` and add:

```bash
# Your new example
test_output_contains "examples/your_example.av" "expected pattern" "description"
```

Or to just verify it runs without errors:

```bash
test_no_error "examples/your_example.av" "description"
```

The other scripts will automatically pick up new `.av` files in the `examples/` directory.
