# Avon Test Suite

Comprehensive testing framework for the Avon language, compiler, CLI, LSP, and tooling.

## Main Entry Point

**To run all tests**, use the main entry point:

```bash
bash testing/run-all.sh              # Full clean build + all tests
bash testing/run-all.sh --no-clean   # Skip rebuild, run tests only
```

This is the **only command you need to remember**. It handles building, linting, and delegating to every test runner.

## How It Works

The test suite is organized in a **two-level hierarchy**:

```
testing/run-all.sh                         ← MAIN ENTRY POINT
  │
  ├── cargo test --all                     (Rust unit + integration tests)
  ├── cargo clippy                         (lint check)
  ├── cargo fmt --check                    (format check)
  │
  ├── avon/run-avon-tests.sh              ← RUNNER: Language tests
  │     ├── test_grammar.sh                  Grammar & syntax
  │     ├── test_template_syntax.sh          Template string parsing
  │     ├── test_arithmetic.sh               Arithmetic & overflow
  │     ├── test_scoping_rules.sh            Variable scoping
  │     ├── test_none_handling.sh            None value handling
  │     ├── test_claims.sh                   Claim assertions
  │     ├── test_parser_lexer.sh             Parser & lexer (95 tests)
  │     ├── test_builtin_functions.sh        Core builtins (150 tests)
  │     ├── test_advanced_builtins.sh        Advanced builtins (152 tests)
  │     ├── test_extended_coverage.sh        Extended coverage (145 tests)
  │     ├── test_deep_coverage.sh            Deep coverage (208 tests)
  │     ├── test_error_handling.sh           Error & edge cases (61 tests)
  │     ├── test_path_literal_block.sh       Absolute path blocking
  │     ├── test_path_traversal.sh           Path traversal protection
  │     ├── test_security_comprehensive.sh   Security sandbox
  │     ├── test_root_relative_paths.sh      Relative --root paths
  │     ├── test_all_examples.sh             All example files compile
  │     ├── test_tutorial_snippets.sh        Tutorial code validation
  │     ├── test_markdown.sh                 Markdown processing
  │     ├── test_repl.sh                     REPL basics
  │     ├── test_parallel_functions.sh       Parallel map/filter/fold
  │     └── test_do_mode.sh                  Do mode task runner (66 tests)
  │
  ├── integration/run-integration-tests.sh ← RUNNER: Integration tests
  │     ├── test_cli_integration.sh          CLI commands (27 tests)
  │     ├── test_example_outputs.sh          Example output validation
  │     ├── test_backup.sh                   Backup & recovery
  │     ├── test_atomic_deployment.sh        Atomic deploy
  │     ├── test_bulletproof.sh              Resilience tests
  │     ├── test_do_mode.sh                  Do mode integration (32 tests)
  │     ├── test_do_mode_docs.sh             Do mode doc verification (41 tests)
  │     ├── repl/test-multiline.sh           REPL multiline (69 tests)
  │     └── repl/test-error-history.sh       REPL error history
  │
  ├── imports/run-import-tests.sh          ← RUNNER: Import & file I/O
  │     └── test_01..test_11.av              11 Avon import test files
  │
  └── lsp/run-lsp-tests.sh                ← RUNNER: LSP server
        ├── Build verification
        ├── Protocol compliance
        ├── Integration tests
        └── Startup performance
```

## Running Individual Test Runners

You can run any runner script independently:

```bash
# Language tests only
bash testing/avon/run-avon-tests.sh

# Integration tests only
bash testing/integration/run-integration-tests.sh

# Import tests only
bash testing/imports/run-import-tests.sh

# LSP tests only
bash testing/lsp/run-lsp-tests.sh
```

## Running Individual Test Scripts

Every test script can also be run standalone:

```bash
# Run just the builtin function tests
bash testing/avon/test_builtin_functions.sh

# Run just the CLI integration tests
bash testing/integration/test_cli_integration.sh

# Run just the REPL multiline tests
bash testing/repl/test-multiline.sh
```

## Test Categories

### Language Tests (`avon/`)

Tests for the Avon language implementation — parsing, evaluation, builtins, templates, security:

| Test Script | Tests | What It Covers |
|---|---|---|
| `test_grammar.sh` | — | Full grammar validation |
| `test_template_syntax.sh` | — | Template `{"...{expr}..."}` parsing |
| `test_arithmetic.sh` | — | Math operators, overflow, precision |
| `test_scoping_rules.sh` | — | `let`/`in` variable scoping |
| `test_none_handling.sh` | — | None propagation, `is_none`, `default` |
| `test_claims.sh` | — | `claim` assertions |
| `test_parser_lexer.sh` | 95 | Literals, operators, pipes, lambdas, errors |
| `test_builtin_functions.sh` | 150 | Core builtins: math, string, list, dict, regex, datetime, file I/O, formatting |
| `test_advanced_builtins.sh` | 152 | Advanced builtins: fold, flatmap, group_by, partition, data parsers, debug/trace, currying, pipe chains |
| `test_extended_coverage.sh` | 145 | Extended coverage: math (abs, ceil, floor, gcd, lcm, log, pow, round, sqrt, uuid), string (lines, words, unlines, unwords, base64, hashes), list (chunks, combinations, last, permutations, transpose, windows), file I/O (abspath, glob, relpath), env (env_var, env_vars), types (is_dict) |
| `test_deep_coverage.sh` | 208 | Deep coverage: DateTime (leap year, units, formats, errors), Regex (anchors, captures, scan, errors), HTML/Markdown (all helpers), Formatting (scientific, bool, edge cases), Parser/Lexer (precedence, nesting, errors), File parsers (xml, html, opml + string variants), Error paths, Misc builtins (neg, to_char, tap, enumerate, pad, env_var_or, walkdir, has_key, all/any/count) |
| `test_error_handling.sh` | 61 | Type errors, division by zero, boundary cases, none edge cases |
| `test_path_literal_block.sh` | — | Blocks absolute path literals in source |
| `test_path_traversal.sh` | — | Prevents `../` path traversal |
| `test_security_comprehensive.sh` | — | Full security sandbox tests |
| `test_root_relative_paths.sh` | — | `--root` flag with relative paths |
| `test_all_examples.sh` | — | Every `examples/*.av` file compiles |
| `test_tutorial_snippets.sh` | — | Tutorial code snippets work |
| `test_markdown.sh` | — | Markdown-to-HTML conversion |
| `test_repl.sh` | — | Basic REPL functionality |
| `test_parallel_functions.sh` | — | `pmap`, `pfilter`, `pfold` |
| `test_do_mode.sh` | 66 | Do mode: task dict eval, example files, execution, flags, errors, auto-discovery, security, diamond deps, env vars, CLI help |

### Integration Tests (`integration/`)

End-to-end tests for the CLI, deploy system, and REPL:

| Test Script | Tests | What It Covers |
|---|---|---|
| `test_cli_integration.sh` | 27 | `avon run`, `eval`, `deploy`, `doc` commands |
| `test_example_outputs.sh` | — | Example file output patterns |
| `test_backup.sh` | — | `--backup` flag for deploy |
| `test_atomic_deployment.sh` | — | Atomic file deployment |
| `test_bulletproof.sh` | — | Resilience under edge cases |
| `test_do_mode.sh` | 32 | Do mode: simple/structured tasks, deps, dry-run, list, info, env vars, errors, typos, cycles |
| `test_do_mode_docs.sh` | 41 | Do mode: doc verification — security blocks, file resolution, auto-discovery, CLI help accuracy |

### REPL Tests (`repl/`)

Interactive REPL tests (run via the integration runner):

| Test Script | Tests | What It Covers |
|---|---|---|
| `test-multiline.sh` | 69 | Multiline `:let`, lambdas, pipes |
| `test-error-history.sh` | 6 | Error recovery and history |

### Import Tests (`imports/`)

File import and module system tests using `.av` test files.

### LSP Tests (`lsp/`)

Language Server Protocol tests — build, protocol compliance, completion, diagnostics.

## Shared Utilities

- **`common.sh`** — Shared by all test scripts. Finds the `avon` and `avon-lsp` binaries (prefers release, falls back to debug), exports `$AVON`, `$LSP_BIN`, `$PROJECT_ROOT`, and color codes.
- **`utils/`** — Helper scripts (fuzzer, example runner, LSP wrapper). Not test scripts themselves.

## Writing New Tests

### 1. Create the test script

Place it in the appropriate directory:
- `avon/` — Language features, parsing, builtins
- `integration/` — CLI commands, deploy, end-to-end workflows
- `repl/` — REPL-specific behavior
- `imports/` — File import functionality (`.av` files)
- `lsp/` — LSP server features

### 2. Follow the template

```bash
#!/bin/bash
# Description of what this tests

# Source common utilities for $AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"

PASSED=0
FAILED=0

run_test() {
    local name="$1"
    local expr="$2"
    local expected="$3"

    result=$($AVON run "$expr" 2>&1) || true
    if [ "$result" = "$expected" ]; then
        echo "✓ $name"
        ((PASSED++))
    else
        echo "✗ $name"
        echo "  Expected: $expected"
        echo "  Got:      $result"
        ((FAILED++))
    fi
}

echo "Testing Feature X..."
echo "===================="

run_test "test name" 'expression' "expected output"

echo ""
echo "Results: $PASSED passed, $FAILED failed"
[ $FAILED -eq 0 ] && exit 0 || exit 1
```

### 3. Register it in the runner

Add a `run_test` line to the appropriate runner script:
- `avon/run-avon-tests.sh` for language tests
- `integration/run-integration-tests.sh` for integration tests

### 4. Naming convention

- Shell test scripts: `test_*.sh`
- Avon test files: `test_*.av`
- Runners: `run-*-tests.sh`

## Troubleshooting

**Tests hanging?**
```bash
timeout 120 bash testing/run-all.sh --no-clean
```

**Binary not found?**
```bash
cargo build --release    # Build the avon binary
```

**Want to skip the full rebuild?**
```bash
bash testing/run-all.sh --no-clean
```

**Run just one failing test with verbose output?**
```bash
bash testing/avon/test_builtin_functions.sh
```

## CI/CD

```bash
bash testing/run-all.sh
# Exit code 0 = all tests passed
# Exit code 1 = failures detected
```
