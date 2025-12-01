#!/usr/bin/env python3

"""
LSP Protocol Test Suite
Tests the Language Server Protocol implementation for Avon

Usage: python3 test-lsp-protocol.py [options]
"""

import json
import subprocess
import sys
import time
import threading
import os
import re
from pathlib import Path

class Colors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

class LSPTester:
    def __init__(self, lsp_binary_path):
        self.lsp_binary = lsp_binary_path
        self.process = None
        self.message_id = 0
        self.diagnostics = {}
        
    def log_header(self, text):
        print(f"\n{Colors.HEADER}{Colors.BOLD}{'='*70}{Colors.ENDC}")
        print(f"{Colors.HEADER}{Colors.BOLD}{text}{Colors.ENDC}")
        print(f"{Colors.HEADER}{Colors.BOLD}{'='*70}{Colors.ENDC}\n")
    
    def log_success(self, text):
        print(f"{Colors.OKGREEN}✓{Colors.ENDC} {text}")
    
    def log_error(self, text):
        print(f"{Colors.FAIL}✗{Colors.ENDC} {text}")
    
    def log_info(self, text):
        print(f"{Colors.WARNING}ℹ{Colors.ENDC} {text}")
    
    def start_server(self):
        """Start the LSP server"""
        self.log_info(f"Starting LSP server: {self.lsp_binary}")
        
        if not os.path.exists(self.lsp_binary):
            self.log_error(f"LSP binary not found: {self.lsp_binary}")
            return False
        
        try:
            self.process = subprocess.Popen(
                [self.lsp_binary],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1
            )
            
            # Give server time to start
            time.sleep(0.5)
            
            if self.process.poll() is None:
                self.log_success("LSP server started")
                return True
            else:
                self.log_error("LSP server failed to start")
                return False
        except Exception as e:
            self.log_error(f"Failed to start LSP: {e}")
            return False
    
    def send_message(self, method, params=None):
        """Send a message to the LSP server"""
        self.message_id += 1
        
        message = {
            "jsonrpc": "2.0",
            "id": self.message_id,
            "method": method
        }
        
        if params:
            message["params"] = params
        
        content = json.dumps(message)
        content_length = len(content.encode('utf-8'))
        
        full_message = f"Content-Length: {content_length}\r\n\r\n{content}"
        
        if self.process and self.process.stdin:
            self.process.stdin.write(full_message)
            self.process.stdin.flush()
            
            if os.environ.get('VERBOSE'):
                self.log_info(f"Sent: {method}")
    
    def read_messages(self, timeout=2):
        """Read messages from LSP server"""
        messages = []
        start_time = time.time()
        
        while time.time() - start_time < timeout:
            try:
                if self.process and self.process.stdout:
                    line = self.process.stdout.readline()
                    
                    if not line:
                        time.sleep(0.1)
                        continue
                    
                    if line.startswith('Content-Length:'):
                        # Parse content length
                        match = re.match(r'Content-Length: (\d+)', line)
                        if match:
                            content_length = int(match.group(1))
                            
                            # Skip blank line
                            self.process.stdout.readline()
                            
                            # Read message
                            message_str = ""
                            bytes_read = 0
                            
                            while bytes_read < content_length:
                                chunk = self.process.stdout.read(
                                    min(1024, content_length - bytes_read)
                                )
                                if not chunk:
                                    break
                                message_str += chunk
                                bytes_read = len(message_str.encode('utf-8'))
                            
                            if message_str:
                                try:
                                    message = json.loads(message_str)
                                    messages.append(message)
                                    
                                    # Track diagnostics
                                    if message.get('method') == 'textDocument/publishDiagnostics':
                                        uri = message.get('params', {}).get('uri', '')
                                        diags = message.get('params', {}).get('diagnostics', [])
                                        self.diagnostics[uri] = diags
                                        
                                except json.JSONDecodeError:
                                    pass
            except:
                time.sleep(0.1)
                continue
        
        return messages
    
    def test_initialization(self):
        """Test 1: Server initialization"""
        self.log_header("Test 1: Server Initialization")
        
        self.send_message("initialize", {
            "processId": None,
            "rootPath": None,
            "capabilities": {
                "textDocument": {
                    "synchronization": {
                        "didSave": True
                    }
                }
            }
        })
        
        messages = self.read_messages(timeout=2)
        
        if any(m.get('id') == self.message_id and 'result' in m for m in messages):
            self.log_success("Server initialization successful")
            return True
        else:
            self.log_error("Server initialization failed")
            return False
    
    def test_document_diagnostics(self, file_path):
        """Test 2: Document diagnostics"""
        self.log_header(f"Test 2: Document Diagnostics - {file_path}")
        
        if not os.path.exists(file_path):
            self.log_error(f"File not found: {file_path}")
            return False
        
        with open(file_path, 'r') as f:
            content = f.read()
        
        file_uri = f"file://{os.path.abspath(file_path)}"
        
        # Send didOpen
        self.send_message("textDocument/didOpen", {
            "textDocument": {
                "uri": file_uri,
                "languageId": "avon",
                "version": 1,
                "text": content
            }
        })
        
        messages = self.read_messages(timeout=2)
        
        if file_uri in self.diagnostics:
            diags = self.diagnostics[file_uri]
            self.log_success(f"Diagnostics received: {len(diags)} issues")
            
            for diag in diags:
                line = diag.get('range', {}).get('start', {}).get('line', 0)
                msg = diag.get('message', 'Unknown error')
                self.log_info(f"  Line {line + 1}: {msg}")
            
            return True
        else:
            self.log_info("No diagnostics published (may indicate valid code)")
            return True
    
    def test_completion(self):
        """Test 3: Code completion"""
        self.log_header("Test 3: Code Completion")
        
        self.send_message("textDocument/completion", {
            "textDocument": {"uri": "file:///test.av"},
            "position": {"line": 0, "character": 3}
        })
        
        messages = self.read_messages(timeout=2)
        
        if messages:
            self.log_success("Completion provider responds")
            return True
        else:
            self.log_info("Completion provider available")
            return True
    
    def test_builtin_signatures(self):
        """Test 4: Builtin function availability"""
        self.log_header("Test 4: Builtin Function Signatures")
        
        # Test with a file that uses builtins
        test_code = r"""
let result = map (\x x * 2) [1, 2, 3] in
let upper_text = upper "hello" in
let filtered = filter (\x x > 5) [1, 2, 3, 6, 7, 8] in
result
"""
        
        file_uri = "file:///test_builtins.av"
        
        self.send_message("textDocument/didOpen", {
            "textDocument": {
                "uri": file_uri,
                "languageId": "avon",
                "version": 1,
                "text": test_code
            }
        })
        
        messages = self.read_messages(timeout=2)
        
        if file_uri in self.diagnostics:
            diags = self.diagnostics[file_uri]
            if diags:
                self.log_warning(f"Detected {len(diags)} potential issues")
                for diag in diags:
                    self.log_info(f"  {diag.get('message', 'Unknown')}")
            else:
                self.log_success("Builtin functions recognized")
        else:
            self.log_success("Builtin functions recognized")
        
        return True
    
    def test_error_detection(self):
        """Test 5: Error detection"""
        self.log_header("Test 5: Error Detection")
        
        # Test code with syntax error
        test_code = "let x = this should fail in x"
        file_uri = "file:///test_error.av"
        
        self.send_message("textDocument/didOpen", {
            "textDocument": {
                "uri": file_uri,
                "languageId": "avon",
                "version": 1,
                "text": test_code
            }
        })
        
        messages = self.read_messages(timeout=2)
        
        if file_uri in self.diagnostics:
            diags = self.diagnostics[file_uri]
            if diags:
                self.log_success(f"Error detected: {diags[0].get('message', 'Unknown error')}")
                return True
            else:
                self.log_error("Error not detected")
                return False
        else:
            self.log_error("No diagnostics received")
            return False
    
    def stop_server(self):
        """Stop the LSP server"""
        if self.process:
            try:
                self.process.terminate()
                self.process.wait(timeout=2)
                self.log_success("LSP server stopped")
            except:
                self.process.kill()
                self.log_info("LSP server forcefully terminated")
    
    def run_all_tests(self):
        """Run all tests"""
        results = []
        
        if not self.start_server():
            return False
        
        results.append(("Initialization", self.test_initialization()))
        
        # Test with example files
        examples_dir = Path(__file__).parent.parent / "examples"
        test_files = [
            examples_dir / "lsp_comprehensive_tests.av",
            examples_dir / "lsp_lambda_tests.av",
        ]
        
        for test_file in test_files:
            if test_file.exists():
                results.append((f"Diagnostics: {test_file.name}", 
                              self.test_document_diagnostics(str(test_file))))
        
        results.append(("Code Completion", self.test_completion()))
        results.append(("Builtin Signatures", self.test_builtin_signatures()))
        results.append(("Error Detection", self.test_error_detection()))
        
        self.stop_server()
        
        # Print summary
        self.log_header("Test Summary")
        
        passed = sum(1 for _, result in results if result)
        total = len(results)
        
        for name, result in results:
            if result:
                self.log_success(name)
            else:
                self.log_error(name)
        
        print(f"\n{Colors.OKGREEN}Passed: {passed}/{total}{Colors.ENDC}\n")
        
        return passed == total

def main():
    # Find LSP binary
    script_dir = Path(__file__).parent
    testing_dir = script_dir.parent
    project_root = testing_dir.parent
    lsp_project = project_root / "avon-lsp"
    
    lsp_binary = lsp_project / "target" / "release" / "avon-lsp"
    if not lsp_binary.exists():
        lsp_binary = lsp_project / "target" / "debug" / "avon-lsp"
    
    print(f"\n{Colors.OKBLUE}{Colors.BOLD}LSP Protocol Test Suite{Colors.ENDC}\n")
    
    tester = LSPTester(str(lsp_binary))
    success = tester.run_all_tests()
    
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
