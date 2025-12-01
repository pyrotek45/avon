# Avon VSCode Extension

Language support for Avon in Visual Studio Code.

## Installation

### From Source

1. Navigate to the VSCode extension directory:
   ```bash
   cd vscode
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the extension:
   ```bash
   npm run compile
   ```

4. Launch the extension in development mode:
   ```bash
   npm run watch
   ```

### In VSCode

1. Open VSCode
2. Go to Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "Avon"
4. Click Install

## Usage

Once installed, the extension provides:

- **Syntax Highlighting** - Avon code is highlighted with appropriate colors
- **Language Support** - Full Avon language support
- **LSP Integration** - Advanced features through Language Server Protocol
  - Code completion
  - Error diagnostics
  - Type checking
  - Jump to definition

## LSP Configuration

The extension uses the Avon Language Server for advanced features.

To use a local LSP binary:

1. Build the LSP in `avon-lsp/`:
   ```bash
   cd ../avon-lsp
   cargo build --release
   ```

2. Update VSCode settings to point to the local binary:
   ```json
   {
     "avon.lsp.path": "/path/to/avon-lsp/target/release/avon-lsp"
   }
   ```

## Development

- **Syntax**: See `syntaxes/` directory
- **Language Features**: See `language-configuration.json`
- **Extension Code**: See `src/` directory

For more information, see the main [Avon README](../README.md).
