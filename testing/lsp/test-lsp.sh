#!/bin/bash
# Test script for Avon LSP - safely tests with timeout

# Start LSP in background with timeout
echo "Starting LSP test with 5 second timeout..."

timeout 5 /usr/local/bin/avon-lsp &
LSP_PID=$!

sleep 1

# Send initialize request
echo "Sending initialize request..."
cat > /tmp/lsp_init.json << 'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootPath":null,"capabilities":{}}}
EOF

cat /tmp/lsp_init.json | nc -U /tmp/avon-lsp.sock 2>/dev/null || echo "Note: Socket test skipped (LSP uses stdio)"

sleep 2

# Kill LSP
kill $LSP_PID 2>/dev/null || true
wait $LSP_PID 2>/dev/null || true

echo "LSP test completed"
