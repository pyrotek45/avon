#!/bin/bash
# Wrapper script for Avon LSP
# This script handles stdio redirection to prevent hanging the terminal
# when used by VS Code Language Client

# The LSP binary is built in the workspace
LSP_BINARY="/workspaces/avon/target/release/avon_lsp"

# Check if binary exists
if [ ! -f "$LSP_BINARY" ]; then
    echo "Error: LSP binary not found at $LSP_BINARY" >&2
    echo "Please build the LSP: cd /workspaces/avon && cargo build --bin avon_lsp --release" >&2
    exit 1
fi

# Run the LSP binary with proper stdio handling
# VS Code Language Client sends JSON-RPC over stdin/stdout
# We pass through stdin/stdout directly as the LSP protocol requires
exec "$LSP_BINARY"
