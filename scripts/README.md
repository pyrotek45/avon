# Test Scripts

This directory contains various test scripts for the Avon project. These scripts help verify that all examples compile and run correctly with the simplified error handling system.

## Available Scripts

### `test_integration.sh`
**Purpose:** Run the complete integration test suite to verify all features work correctly.

**Usage:**
```bash
./scripts/test_integration.sh
```

**What it does:**
- Builds the project
- Runs all major test suites in sequence:
  - Grammar comprehensive test
  - Bulletproof comprehensive test
  - Tutorial snippets test
  - Scoping rules test
  - All examples test (105+ files)
  - Atomic deployment test
  - Fuzz security test (path traversal, malformed syntax, edge cases)
  - Comprehensive security test (49 security validations)
- Provides a summary of all test results
- Exits with success only if all tests pass

### `test_security_comprehensive.sh`
**Purpose:** Comprehensive security vulnerability testing across 9 categories (49 tests).

**Usage:**
```bash
./scripts/test_security_comprehensive.sh
```

**Security Tests (9 Categories):**

1. **Path Traversal** (9 tests) - readfile, import, fill_template protection
2. **Injection Attacks** (6 tests) - Template and code injection prevention
3. **Malformed Input** (10 tests) - Syntax validation and error handling
4. **Resource Exhaustion** (6 tests) - Large strings, lists, deep nesting
5. **Type Safety** (5 tests) - Type mismatch detection
6. **Environment Manipulation** (3 tests) - Scope and closure isolation
7. **Special Characters** (4 tests) - Null bytes, Unicode, escaping
8. **Recursion Prevention** (2 tests) - By-design non-support
9. **File System Boundary** (4 tests) - Multiple traversal attempts

**What it does:**
- Tests 49 security-related code patterns
- Verifies path traversal blocking
- Validates injection attack prevention
- Checks malformed input handling
- Tests resource limits
- Ensures type safety
- Validates scope isolation
- Tests special character handling
- Confirms recursion prevention
- Tests file system boundaries

**Example Output:**
```
SECTION 1: Path Traversal Vulnerabilities
  ✓ readfile with ..
  ✓ readfile with absolute path
  ✓ import with ../
  ✗ CRITICAL: injection possible
  ...
✅ All security tests passed!
```

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

1. **`test_integration.sh`** - Complete test suite ensuring all features work together
2. **`test_all_examples.sh`** - Quick smoke test to ensure nothing is broken
3. **`test_example_outputs.sh`** - Validates correctness of output
4. **`run_examples.sh`** - Full integration testing with deployments

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
