# LSP Testing Guide

**Language Server Protocol Testing for Avon**

This directory contains comprehensive tests for the Avon Language Server Protocol (LSP) implementation.

---

## Quick Start

### Build and Test LSP
```bash
# Build the LSP project
cd /workspaces/avon/avon-lsp
cargo build

# Run integration tests
cd /workspaces/avon
bash scripts/test-lsp-integration.sh --verbose --build

# Run protocol tests
python3 scripts/test-lsp-protocol.py
```

---

## Test Scripts

### 1. test-lsp-integration.sh
**Purpose:** Integration testing of LSP with Avon compiler

**Features:**
- Validates LSP binary existence and type
- Tests compilation of example files
- Verifies builtin functions are available
- Performance benchmarking
- Comprehensive status reporting

**Usage:**
```bash
# Run with defaults
bash scripts/test-lsp-integration.sh

# Build LSP and run tests
bash scripts/test-lsp-integration.sh --build

# Verbose output
bash scripts/test-lsp-integration.sh --verbose

# Quick test (essential only)
bash scripts/test-lsp-integration.sh --quick

# Comprehensive test suite
bash scripts/test-lsp-integration.sh --all --verbose
```

**What it Tests:**
1. ✓ LSP Binary Validation
2. ✓ Diagnostics for Valid Code
3. ✓ Type Mismatch Detection
4. ✓ Lambda Expression Parsing
5. ✓ Function Currying Support
6. ✓ Code Completion Features
7. ✓ Example Files Validation
8. ✓ Performance Benchmarking

---

### 2. test-lsp-protocol.py
**Purpose:** Protocol-level testing using LSP JSON-RPC

**Features:**
- Direct LSP server communication
- Initialization and capabilities testing
- Document diagnostics publishing
- Error detection validation
- Builtin function signature checking
- Code completion testing

**Usage:**
```bash
# Run protocol tests
python3 scripts/test-lsp-protocol.py

# Verbose output
VERBOSE=1 python3 scripts/test-lsp-protocol.py
```

**What it Tests:**
1. ✓ Server Initialization
2. ✓ Document Diagnostics Publishing
3. ✓ Code Completion Provider
4. ✓ Builtin Function Signatures
5. ✓ Error Detection and Reporting

---

## LSP Example Files

Test files in `examples/` directory:

### lsp_comprehensive_tests.av
**Purpose:** Comprehensive validation of language features

**Tests:**
- String operations (split, upper, lower, concat, etc.)
- List operations (map, filter, fold, sort, etc.)
- Dictionary operations (get, has_key, keys, values)
- Type conversion (to_string, to_int, to_float, to_bool)
- Pipe operators
- Range syntax
- Path and template interpolation

**Expected Result:** All features compile and work correctly

---

### lsp_lambda_tests.av
**Purpose:** Lambda expression and function syntax

**Tests:**
- Lambda definition and usage
- Higher-order functions
- Function composition
- Parameter binding

**Expected Result:** All lambdas parse and execute correctly

---

### lsp_currying_tests.av
**Purpose:** Function currying support

**Tests:**
- Partial application
- Curried function chains
- Default parameters

**Expected Result:** Currying works as expected

---

### lsp_type_mismatch_tests.av
**Purpose:** Type checking and error detection

**Tests:**
- Type mismatches
- Argument count errors
- Invalid operations

**Expected Result:** LSP detects and reports all errors

---

### lsp_completion_demo.av
**Purpose:** Code completion demonstration

**Tests:**
- Builtin function names
- Variable completion
- Keyword completion

**Expected Result:** Completion suggestions provided accurately

---

## Test Results

Test results are saved to: `test-results/`

Files generated:
- `build.log` - LSP build output
- `test-summary.txt` - Summary of all test results
- Individual test logs

---

## Running LSP Server Manually

### Start LSP Server
```bash
cd /workspaces/avon/avon-lsp
cargo run
```

The LSP will listen on stdin/stdout and be ready to receive JSON-RPC messages.

### Connect with VS Code
1. Build the extension: `cd vscode-extension && npm install && npm run compile`
2. Install the extension in VS Code
3. Open an Avon file (.av)
4. LSP will provide diagnostics and completions

---

## LSP Features Tested

### Diagnostics
✓ Syntax error detection
✓ Type checking errors
✓ Undefined variable detection
✓ Function signature validation
✓ File template validation

### Code Completion
✓ Builtin function names (111+ functions)
✓ Local variable names
✓ Keywords (let, in, if, then, else)
✓ Proper filtering based on partial input

### Capabilities
✓ Text document synchronization
✓ Full document sync
✓ Completion provider
✓ Diagnostic publishing

---

## Known Limitations

### Current Limitations
1. **Argument Counting:** LSP may overcount arguments when splits on whitespace
   - Workaround: Complex expressions in parentheses may trigger false positives
   
2. **List Literals:** May be counted as multiple arguments
   - Example: `map f [1, 2, 3]` might count as 3 arguments instead of 2

### Planned Improvements
- [ ] More accurate argument counting
- [ ] Better handling of complex expressions
- [ ] Hover information for functions
- [ ] Go-to-definition support
- [ ] Semantic highlighting

---

## Continuous Integration

LSP tests run in CI/CD pipeline:
- On every commit
- Before release
- Pre-push validation

See `.github/workflows/lsp-testing.yml` for CI configuration.

---

## Troubleshooting

### LSP Server Won't Start
```bash
# Check if binary exists
ls -la /workspaces/avon/avon-lsp/target/debug/avon-lsp

# Rebuild
cd /workspaces/avon/avon-lsp
cargo clean
cargo build
```

### Tests Fail to Run
```bash
# Check permissions
chmod +x /workspaces/avon/scripts/test-lsp-*.sh

# Run with Python3
python3 --version  # Should be 3.6+

# Check dependencies
cd /workspaces/avon
cargo build
```

### Diagnostics Not Publishing
1. Check LSP starts: `cargo run` in avon-lsp/
2. Verify file syntax: `cargo run -- eval test.av`
3. Check LSP logs for errors

---

## Contributing

When adding new language features:
1. Create a test example in `examples/lsp_*.av`
2. Add test in one of the test scripts
3. Run full test suite: `test-lsp-integration.sh --all`
4. Verify diagnostics and completion work

---

## References

- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [Avon Language Guide](../tutorial/)
- [LSP Implementation](../avon-lsp/src/main.rs)

---

**Last Updated:** December 1, 2025  
**Status:** ✅ Tests Complete and Passing
