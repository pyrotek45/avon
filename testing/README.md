# Avon Test Suite

Comprehensive testing framework for the Avon language.

## Quick Start

Run all tests:

```bash
bash run-all.sh
```

## Test Categories

### Avon Language Tests (`avon/`)

Tests for the Avon language implementation:

```bash
bash avon/test-all-avon.sh
```

**Includes:**
- Grammar and syntax validation
- Type checking
- Scoping rules
- Template processing
- Security features
- All example files

### LSP Server Tests (`lsp/`)

Tests for the Language Server Protocol implementation:

```bash
bash lsp/run-lsp-tests.sh
```

**Includes:**
- LSP build verification
- Integration testing with language features
- Protocol compliance
- Example file validation
- Builtin function detection
- Performance benchmarks

### Integration Tests (`integration/`)

End-to-end integration tests:

```bash
bash integration/run-integration-tests.sh
```

**Includes:**
- Full deployment pipelines
- Backup and recovery
- Atomic operations
- Security comprehensive tests
- Example outputs

### Utilities (`utils/`)

Shared test utilities and helper scripts.

## Test Results

After running tests, results are stored in:

```
test-results/
├── lsp/          - LSP test logs and results
└── [other test outputs]
```

## Individual Test Files

| Test | Location | Purpose |
|------|----------|---------|
| Grammar | `avon/test_grammar.sh` | Validate language grammar |
| Examples | `avon/test_all_examples.sh` | Test all example files |
| Security | `avon/test_security_comprehensive.sh` | Security feature testing |
| LSP Integration | `lsp/test-lsp-integration.sh` | LSP feature integration |
| Deployment | `integration/test_atomic_deployment.sh` | Deployment validation |
| Backup | `integration/test_backup.sh` | Backup/recovery testing |

## Troubleshooting

**Tests hanging?**
- Use `timeout` command: `timeout 120 bash run-all.sh`
- Check for LSP server issues: `bash lsp/run-lsp-tests.sh`

**Build failures?**
- Ensure Rust is installed: `rustup update`
- Clean build: `cargo clean && cargo build`

**Path issues?**
- All tests use relative paths from project root
- Run from `/workspaces/avon` directory

## Writing New Tests

1. Create script in appropriate subdirectory
2. Use color codes from existing tests
3. Follow naming convention: `test_*.sh`
4. Add to master test runner (`test-all-avon.sh`, etc.)

## CI/CD Integration

The test suite is designed to run in CI/CD pipelines:

```bash
bash testing/run-all.sh
```

Exit code 0 = all tests passed
Exit code 1 = test failures detected
