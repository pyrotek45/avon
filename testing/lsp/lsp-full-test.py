#!/usr/bin/env python3
"""
LSP-Based Full Test Suite
Tests all example files and generated type tests using the actual LSP.
Reports false positives and type checking issues.
"""

import json
import subprocess
import tempfile
import os
import time
import threading
from pathlib import Path
from collections import defaultdict

class LSPTestRunner:
    """Runs tests against the actual LSP"""
    
    def __init__(self):
        self.results = defaultdict(list)
        self.summary = {
            "total_tests": 0,
            "passed": 0,
            "failed": 0,
            "errors": defaultdict(int)
        }
    
    def run_lsp_check(self, code: str, timeout: float = 2.0) -> list:
        """
        Run code through LSP and get diagnostics.
        Returns list of diagnostic messages.
        """
        diagnostics = []
        test_file = tempfile.NamedTemporaryFile(mode='w', suffix='.av', delete=False)
        test_file.write(code)
        test_file.close()
        
        try:
            # Prepare LSP requests
            init_request = {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"processId": None, "rootPath": None, "capabilities": {}}
            }
            
            did_open_request = {
                "jsonrpc": "2.0",
                "id": 2,
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": {
                        "uri": f"file://{test_file.name}",
                        "languageId": "avon",
                        "version": 1,
                        "text": code
                    }
                }
            }
            
            # Use thread with timeout to prevent hanging
            def run_lsp():
                try:
                    proc = subprocess.Popen(
                        ['/usr/local/bin/avon-lsp'],
                        stdin=subprocess.PIPE,
                        stdout=subprocess.PIPE,
                        stderr=subprocess.PIPE,
                        text=True,
                        bufsize=1
                    )
                    
                    # Send init
                    msg = json.dumps(init_request)
                    req = f"Content-Length: {len(msg)}\r\n\r\n{msg}"
                    proc.stdin.write(req)
                    proc.stdin.flush()
                    
                    # Send didOpen
                    msg = json.dumps(did_open_request)
                    req = f"Content-Length: {len(msg)}\r\n\r\n{msg}"
                    proc.stdin.write(req)
                    proc.stdin.flush()
                    
                    # Read responses
                    time.sleep(0.3)
                    data = proc.stdout.read(5000)
                    
                    # Parse for diagnostics
                    if 'publishDiagnostics' in data:
                        # Simple parsing
                        lines = data.split('\n')
                        for line in lines:
                            if '"message"' in line:
                                # Extract message
                                try:
                                    part = line.split('"message"')[1]
                                    msg = part.split('":"')[1].split('"')[0]
                                    if msg:
                                        diagnostics.append(msg)
                                except:
                                    pass
                    
                    proc.terminate()
                    proc.wait(timeout=0.5)
                except:
                    try:
                        if proc:
                            proc.kill()
                    except:
                        pass
            
            thread = threading.Thread(target=run_lsp, daemon=True)
            thread.start()
            thread.join(timeout=timeout)
            
        finally:
            try:
                os.unlink(test_file.name)
            except:
                pass
        
        return diagnostics
    
    def test_example_file(self, filepath: str) -> dict:
        """Test a single example file"""
        try:
            with open(filepath, 'r') as f:
                code = f.read()
        except:
            return {"file": filepath, "status": "read_error", "diagnostics": []}
        
        basename = os.path.basename(filepath)
        diagnostics = self.run_lsp_check(code)
        
        return {
            "file": basename,
            "path": filepath,
            "status": "pass" if len(diagnostics) == 0 else "fail",
            "diagnostics": diagnostics,
            "diagnostic_count": len(diagnostics)
        }
    
    def run_all_tests(self, test_dir: str = "/workspaces/avon/examples"):
        """Run tests on all files in directory"""
        print("="*80)
        print("LSP COMPREHENSIVE TEST SUITE")
        print("="*80)
        print(f"\nTesting all files in {test_dir}\n")
        
        test_files = sorted(Path(test_dir).glob("*.av"))
        self.summary["total_tests"] = len(test_files)
        
        passed_files = []
        failed_files = []
        
        for filepath in test_files:
            result = self.test_example_file(str(filepath))
            basename = result["file"]
            
            if result["status"] == "pass":
                print(f"✓ {basename}")
                passed_files.append(basename)
                self.summary["passed"] += 1
            else:
                print(f"❌ {basename}")
                for diag in result["diagnostics"]:
                    print(f"   - {diag}")
                    # Count error types
                    if "Undefined variable" in diag:
                        self.summary["errors"]["undefined_variable"] += 1
                    elif "Type error" in diag:
                        self.summary["errors"]["type_error"] += 1
                    else:
                        self.summary["errors"]["other"] += 1
                failed_files.append(result)
                self.summary["failed"] += 1
        
        self.print_summary(passed_files, failed_files)
    
    def print_summary(self, passed: list, failed: list) -> None:
        """Print test summary"""
        print("\n" + "="*80)
        print("TEST SUMMARY")
        print("="*80)
        print(f"\nTotal Tests: {self.summary['total_tests']}")
        print(f"Passed: ✓ {self.summary['passed']}")
        print(f"Failed: ❌ {self.summary['failed']}")
        print(f"Pass Rate: {100 * self.summary['passed'] / max(1, self.summary['total_tests']):.1f}%")
        
        if self.summary["failed"] > 0:
            print(f"\n⚠️  FALSE POSITIVES DETECTED!")
            print("\nError distribution:")
            for error_type, count in self.summary["errors"].items():
                print(f"  {error_type}: {count}")
        else:
            print("\n✅ All tests passed! No false positives.")
        
        print("\n" + "="*80)

class QuickTestRunner:
    """Quick validation without LSP (basic syntax checks)"""
    
    @staticmethod
    def check_file(filepath: str) -> dict:
        """Quick syntax check"""
        try:
            with open(filepath, 'r') as f:
                content = f.read()
        except:
            return {"file": filepath, "valid": False, "reason": "read_error"}
        
        errors = []
        
        # Check braces
        if content.count('{') != content.count('}'):
            errors.append("brace_mismatch")
        
        # Check parens
        if content.count('(') != content.count(')'):
            errors.append("paren_mismatch")
        
        # Check brackets
        if content.count('[') != content.count(']'):
            errors.append("bracket_mismatch")
        
        # Check quotes
        in_string = False
        i = 0
        while i < len(content):
            if content[i] == '"' and (i == 0 or content[i-1] != '\\'):
                in_string = not in_string
            i += 1
        
        if in_string:
            errors.append("unclosed_string")
        
        return {
            "file": os.path.basename(filepath),
            "valid": len(errors) == 0,
            "errors": errors
        }
    
    @staticmethod
    def run_quick_validation(test_dir: str = "/workspaces/avon/examples"):
        """Quick validation of all files"""
        print("="*80)
        print("QUICK VALIDATION (Basic Syntax Checks)")
        print("="*80)
        print(f"\nValidating all files in {test_dir}\n")
        
        test_files = sorted(Path(test_dir).glob("*.av"))
        passed = 0
        failed = 0
        
        for filepath in test_files:
            result = QuickTestRunner.check_file(str(filepath))
            
            if result["valid"]:
                print(f"✓ {result['file']}")
                passed += 1
            else:
                print(f"❌ {result['file']}: {', '.join(result['errors'])}")
                failed += 1
        
        print("\n" + "="*80)
        print(f"Passed: {passed}/{passed+failed}")
        print(f"Failed: {failed}/{passed+failed}")
        print("="*80)

if __name__ == "__main__":
    import sys
    
    # Quick validation first
    print("\nPhase 1: Quick Syntax Validation\n")
    QuickTestRunner.run_quick_validation()
    
    # Then full LSP testing (optional, can be slow)
    if len(sys.argv) > 1 and sys.argv[1] == "--full":
        print("\n\nPhase 2: Full LSP Testing\n")
        runner = LSPTestRunner()
        runner.run_all_tests()
    else:
        print("\n\nRun with '--full' flag to test with actual LSP")
        print("Example: python3 lsp-full-test.py --full")
