#!/bin/bash
# Quick start guide for Avon LSP

echo "🚀 Avon Language Server - Quick Start"
echo "===================================="
echo ""

# Verify LSP is installed
if ! command -v avon-lsp &> /dev/null; then
    echo "❌ LSP not found in PATH"
    exit 1
fi

echo "✅ LSP installed at: $(which avon-lsp)"
echo "✅ LSP binary: $(file $(which avon-lsp) | grep -o 'ELF.*')"
echo ""

echo "📦 Extension location:"
echo "   /workspaces/avon/vscode-extension/"
echo ""

echo "🎯 To use the extension:"
echo "   1. Open VS Code with the extension folder:"
echo "      code /workspaces/avon/vscode-extension"
echo ""
echo "   2. Press F5 to launch Extension Development Host"
echo ""
echo "   3. Open any .av file to see LSP validation in action"
echo ""

echo "🧪 To test the LSP directly:"
echo "   python3 /workspaces/avon/test_lsp_direct.py"
echo ""

echo "📝 Demo file:"
echo "   /workspaces/avon/demo.av"
echo ""

echo "✨ Status: Ready to use!"
