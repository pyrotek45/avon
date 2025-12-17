#!/bin/bash
# Common utilities for Avon test scripts
# Source this file at the start of any test script

# Find the project root
if [ -z "$PROJECT_ROOT" ]; then
    PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fi

# Find the Avon binary - prefer release, fall back to debug
find_avon_binary() {
    local release_bin="$PROJECT_ROOT/target/release/avon"
    local debug_bin="$PROJECT_ROOT/target/debug/avon"
    
    if [ -x "$release_bin" ]; then
        echo "$release_bin"
    elif [ -x "$debug_bin" ]; then
        echo "$debug_bin"
    else
        echo ""
    fi
}

# Find the LSP binary - prefer release, fall back to debug
find_lsp_binary() {
    local release_bin="$PROJECT_ROOT/avon-lsp/target/release/avon-lsp"
    local debug_bin="$PROJECT_ROOT/avon-lsp/target/debug/avon-lsp"
    
    if [ -x "$release_bin" ]; then
        echo "$release_bin"
    elif [ -x "$debug_bin" ]; then
        echo "$debug_bin"
    else
        echo ""
    fi
}

# Set AVON variable if not already set
if [ -z "$AVON" ]; then
    AVON=$(find_avon_binary)
    if [ -z "$AVON" ]; then
        echo "ERROR: No Avon binary found. Run 'cargo build' or 'cargo build --release' first." >&2
        exit 1
    fi
fi

# Set LSP_BIN variable if not already set
if [ -z "$LSP_BIN" ]; then
    LSP_BIN=$(find_lsp_binary)
    # Don't error here - not all tests need the LSP
fi

# Export for child scripts
export AVON
export LSP_BIN
export PROJECT_ROOT

# Colors for output (if not already defined)
RED=${RED:-'\033[0;31m'}
GREEN=${GREEN:-'\033[0;32m'}
YELLOW=${YELLOW:-'\033[1;33m'}
BLUE=${BLUE:-'\033[0;34m'}
NC=${NC:-'\033[0m'}
