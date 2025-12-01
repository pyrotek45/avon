# Avon Language Server Protocol (LSP)

A standalone Language Server Protocol implementation for the Avon template language, providing real-time diagnostics and code completion in VS Code and other LSP-compatible editors.

## Features

- **Real-time Diagnostics** - Instant error reporting as you type
- **Code Completion** - Autocomplete for all 111+ built-in functions
- **Accurate Validation** - Uses the same lexer, parser, and evaluator as the Avon compiler for perfect accuracy
- **Zero False Positives** - No duplicate parsing logic, just direct reuse of compiler validation

## Installation

### Prerequisites

- Rust 1.56 or later
- Cargo

### Building

From this directory (`avon-lsp/`):

```bash
cargo build --release
```

The compiled binary will be at `target/release/avon-lsp`.

## Running the LSP

### Standalone

```bash
cargo run
```

### Via VS Code Extension

The LSP is automatically invoked by the Avon VS Code extension. See `../vscode-extension/` for extension setup.

## How It Works

The LSP provides accurate diagnostics by reusing the Avon compiler's validation pipeline:

1. **Tokenization** - Uses `avon::lexer::tokenize()` to lex the source code
2. **Parsing** - Uses `avon::parser::parse_with_error()` to build the AST
3. **Type Checking** - Uses `avon::eval::eval()` with empty symbol table to catch type and runtime errors
4. **Template Validation** - Uses `avon::eval::collect_file_templates()` to validate template rendering
5. **Diagnostics** - Converts compiler errors to LSP diagnostics and publishes them

This approach ensures the LSP and compiler always agree on what is valid Avon code.

## Development

The LSP reuses the avon library as a dependency. To rebuild after changes to the main compiler:

```bash
cargo build
```

Cargo will automatically rebuild the avon library dependency.

## Supported Capabilities

- Text document synchronization (full sync)
- Completion requests (builtin function names)
- Diagnostics publishing

## Dependencies

- `tower-lsp` - LSP server framework
- `tokio` - Async runtime
- `avon` - Core compiler and library

## License

MIT License - See LICENSE in the repository root
