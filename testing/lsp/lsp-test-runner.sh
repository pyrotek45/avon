#!/bin/bash
# LSP Test Runner - Sends a test file to the LSP and captures diagnostics
# Usage: lsp-test-runner.sh <test_file>

TEST_FILE="$1"

if [ -z "$TEST_FILE" ]; then
    echo '{"errors": [], "message": "No test file provided"}'
    exit 1
fi

if [ ! -f "$TEST_FILE" ]; then
    echo '{"errors": [], "message": "File not found"}'
    exit 1
fi

# Read file content
FILE_CONTENT=$(cat "$TEST_FILE")

# Create temporary script to communicate with LSP
TEMP_SCRIPT=$(mktemp)
trap "rm -f $TEMP_SCRIPT" EXIT

cat > "$TEMP_SCRIPT" << 'EOF'
#!/usr/bin/env python3
import sys
import json
import subprocess
import time
import threading
import re

test_file = sys.argv[1]
with open(test_file, 'r') as f:
    file_content = f.read()

file_uri = f"file://{test_file}"

# LSP requests
init_request = {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
        "processId": None,
        "rootPath": None,
        "capabilities": {}
    }
}

did_open_request = {
    "jsonrpc": "2.0",
    "id": 2,
    "method": "textDocument/didOpen",
    "params": {
        "textDocument": {
            "uri": file_uri,
            "languageId": "avon",
            "version": 1,
            "text": file_content
        }
    }
}

errors_found = []
lsp_ready = False
diagnostics_received = False
timeout_count = 0

def read_lsp_output():
    global lsp_ready, diagnostics_received, timeout_count
    try:
        process = subprocess.Popen(['/usr/local/bin/avon-lsp'], 
                                 stdin=subprocess.PIPE, 
                                 stdout=subprocess.PIPE, 
                                 stderr=subprocess.PIPE,
                                 text=True,
                                 bufsize=1)
        
        # Send initialize
        msg = json.dumps(init_request)
        content_length = len(msg.encode('utf-8'))
        full_msg = f"Content-Length: {content_length}\r\n\r\n{msg}"
        process.stdin.write(full_msg)
        process.stdin.flush()
        
        # Send did_open
        msg = json.dumps(did_open_request)
        content_length = len(msg.encode('utf-8'))
        full_msg = f"Content-Length: {content_length}\r\n\r\n{msg}"
        process.stdin.write(full_msg)
        process.stdin.flush()
        
        # Read responses (with timeout)
        start_time = time.time()
        while time.time() - start_time < 3:
            try:
                line = process.stdout.readline()
                if not line:
                    time.sleep(0.1)
                    continue
                    
                if line.startswith('Content-Length:'):
                    # Read the actual message
                    content_length_match = re.match(r'Content-Length: (\d+)', line)
                    if content_length_match:
                        content_length = int(content_length_match.group(1))
                        # Skip blank line
                        process.stdout.readline()
                        # Read message
                        message = process.stdout.read(content_length)
                        if message:
                            data = json.loads(message)
                            # Look for publishDiagnostics
                            if 'method' in data and data['method'] == 'textDocument/publishDiagnostics':
                                diagnostics = data.get('params', {}).get('diagnostics', [])
                                for diag in diagnostics:
                                    errors_found.append({
                                        'line': diag['range']['start']['line'],
                                        'column': diag['range']['start']['character'],
                                        'message': diag['message'],
                                        'severity': diag.get('severity', 1)
                                    })
                                diagnostics_received = True
                                lsp_ready = True
            except Exception as e:
                pass
        
        # Terminate process
        try:
            process.terminate()
            process.wait(timeout=1)
        except:
            process.kill()
            
    except Exception as e:
        pass

# Run LSP in thread
thread = threading.Thread(target=read_lsp_output, daemon=True)
thread.start()
thread.join(timeout=4)

# Output results
result = {
    "errors": errors_found,
    "count": len(errors_found),
    "file": test_file
}

print(json.dumps(result))
EOF

# Run the Python test runner
python3 "$TEMP_SCRIPT" "$TEST_FILE"
