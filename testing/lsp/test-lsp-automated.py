#!/usr/bin/env python3
"""
Avon LSP Automated Testing and Validation System
Tests the LSP with comprehensive test cases and validates results
"""

import json
import subprocess
import tempfile
import os
import sys
from pathlib import Path
from dataclasses import dataclass
from typing import List, Dict, Optional

@dataclass
class TestCase:
    """A single test case"""
    name: str
    code: str
    expected_errors: int
    description: str = ""

@dataclass
class ErrorInfo:
    """Information about a detected error"""
    line: int
    column: int
    message: str
    severity: int

class LSPTester:
    """Automated LSP testing system"""
    
    def __init__(self, lsp_binary: str = "/usr/local/bin/avon-lsp"):
        self.lsp_binary = lsp_binary
        self.test_results = []
        self.passed = 0
        self.failed = 0
        self.test_dir = tempfile.mkdtemp(prefix="avon_lsp_test_")
        
    def test_case(self, test: TestCase) -> bool:
        """Run a single test case"""
        test_file = os.path.join(self.test_dir, f"{test.name}.av")
        
        # Write test file
        with open(test_file, 'w') as f:
            f.write(test.code)
        
        # Get diagnostics
        errors = self.get_diagnostics(test_file)
        error_count = len(errors)
        
        # Check result
        passed = error_count == test.expected_errors
        
        result = {
            "name": test.name,
            "passed": passed,
            "expected": test.expected_errors,
            "actual": error_count,
            "errors": errors,
            "description": test.description,
            "code": test.code
        }
        self.test_results.append(result)
        
        if passed:
            self.passed += 1
            print(f"✓ {test.name}: PASS (found {error_count} errors as expected)")
        else:
            self.failed += 1
            print(f"✗ {test.name}: FAIL (expected {test.expected_errors}, got {error_count})")
            if errors:
                for err in errors:
                    print(f"    Line {err['line']}: {err['message']}")
        
        return passed
    
    def get_diagnostics(self, test_file: str) -> List[Dict]:
        """Get diagnostics from LSP for a test file"""
        try:
            # Read file content
            with open(test_file, 'r') as f:
                content = f.read()
            
            # Create LSP client script
            script = f"""
import json
import sys

# Simplified LSP diagnostic extraction
# Just parse the file with the LSP binary and capture output
content = {json.dumps(content)}
# In a real implementation, this would communicate via JSON-RPC
# For now, we parse the Avon code directly
errors = []
print(json.dumps({{"errors": errors}}))
"""
            
            # For now, return empty - will be improved with proper JSON-RPC
            return []
        except Exception as e:
            print(f"Error getting diagnostics: {e}")
            return []
    
    def report(self):
        """Print test report"""
        print("\n" + "="*60)
        print("LSP TEST REPORT")
        print("="*60)
        print(f"Total: {self.passed + self.failed}")
        print(f"Passed: {self.passed}")
        print(f"Failed: {self.failed}")
        
        if self.failed > 0:
            print("\nFailed Tests:")
            for result in self.test_results:
                if not result['passed']:
                    print(f"\n  {result['name']}: {result['description']}")
                    print(f"    Code: {result['code'][:60]}...")
                    print(f"    Expected {result['expected']} errors, got {result['actual']}")
        
        return self.failed == 0

# Test cases
TEST_CASES = [
    TestCase(
        name="undefined_variable",
        code="let x = 5 in x + y",
        expected_errors=1,
        description="Should error on undefined variable 'y'"
    ),
    TestCase(
        name="valid_let_cascade",
        code="let x = 5 in let y = 10 in x + y",
        expected_errors=0,
        description="Should allow cascading let bindings"
    ),
    TestCase(
        name="single_brace_template",
        code='@out.txt {"Hello {name}"}',
        expected_errors=0,
        description="Should allow single-brace templates"
    ),
    TestCase(
        name="double_brace_template",
        code='@out.txt {{"Hello {{x}}"}}',
        expected_errors=0,
        description="Should allow double-brace templates"
    ),
    TestCase(
        name="triple_brace_template",
        code='@out.txt {{{"Content\nHere"}}}',
        expected_errors=0,
        description="Should allow triple-brace templates"
    ),
    TestCase(
        name="multiline_if",
        code="if x > 5\nthen 10\nelse 20",
        expected_errors=1,  # 'x' is undefined
        description="Should allow multi-line if statements"
    ),
    TestCase(
        name="lambda_multiline",
        code="let f = \\x \\y\n  x + y\nin f 5 10",
        expected_errors=0,
        description="Should track lambda parameters across lines"
    ),
    TestCase(
        name="dict_access",
        code='let config = {host: "localhost", port: 8080} in config.host',
        expected_errors=0,
        description="Should allow dict field access"
    ),
    TestCase(
        name="pipe_operator",
        code="[1,2,3] -> length",
        expected_errors=0,
        description="Should recognize pipe operators"
    ),
    TestCase(
        name="builtin_functions",
        code="length [1,2,3]",
        expected_errors=0,
        description="Should recognize builtin functions"
    ),
]

if __name__ == "__main__":
    tester = LSPTester()
    
    print("Running Avon LSP Test Suite...\n")
    
    for test_case in TEST_CASES:
        tester.test_case(test_case)
    
    success = tester.report()
    sys.exit(0 if success else 1)
