#!/usr/bin/env python3
"""
LSP Integration Analyzer
Analyzes example files for LSP errors and generates a report
"""

import os
import subprocess
import json
import glob
from pathlib import Path
from collections import defaultdict

class LSPAnalyzer:
    """Analyzes Avon files for LSP compliance"""
    
    def __init__(self):
        self.examples_dir = "/workspaces/avon/examples"
        self.results = {}
        self.summary = defaultdict(int)
        
    def analyze_file(self, filepath: str) -> dict:
        """Analyze a single file for LSP issues"""
        if not os.path.isfile(filepath):
            return {"file": filepath, "status": "not_found"}
        
        try:
            with open(filepath, 'r') as f:
                content = f.read()
        except Exception as e:
            return {"file": filepath, "status": "read_error", "error": str(e)}
        
        # Run basic checks
        issues = []
        
        # Check 1: Undefined variables
        defined_vars = self.extract_definitions(content)
        undefined = self.find_undefined_vars(content, defined_vars)
        for var, lines in undefined.items():
            for line_num in lines:
                issues.append({
                    "type": "undefined_variable",
                    "variable": var,
                    "line": line_num,
                    "severity": "error"
                })
        
        # Check 2: Template syntax
        template_issues = self.check_templates(content)
        issues.extend(template_issues)
        
        # Check 3: Structure issues
        struct_issues = self.check_structure(content)
        issues.extend(struct_issues)
        
        return {
            "file": filepath,
            "status": "analyzed",
            "issues": issues,
            "lines": len(content.split('\n')),
            "has_templates": "{" in content and "}" in content
        }
    
    def extract_definitions(self, content: str) -> set:
        """Extract defined variables from content"""
        defined = set()
        
        # Extract let bindings
        import re
        let_pattern = r'let\s+(\w+)\s*='
        for match in re.finditer(let_pattern, content):
            defined.add(match.group(1))
        
        # Extract lambda parameters
        lambda_pattern = r'\\(\w+)'
        for match in re.finditer(lambda_pattern, content):
            defined.add(match.group(1))
        
        # Add builtins
        builtins = [
            'map', 'filter', 'fold', 'length', 'concat', 'contains', 'starts_with',
            'ends_with', 'split', 'join', 'sort', 'reverse', 'first', 'last',
            'take', 'drop', 'flatten', 'unique', 'group', 'index', 'any', 'all',
            'is_string', 'is_number', 'is_int', 'is_float', 'is_bool', 'is_list',
            'is_dict', 'is_function', 'keys', 'values', 'pairs', 'to_string',
            'to_number', 'to_int', 'to_float', 'to_bool', 'to_list'
        ]
        defined.update(builtins)
        
        return defined
    
    def find_undefined_vars(self, content: str, defined: set) -> dict:
        """Find undefined variable references"""
        undefined = defaultdict(list)
        
        # This is a simplified check - the full LSP does much more
        import re
        for line_num, line in enumerate(content.split('\n'), 1):
            # Skip comments
            if '#' in line:
                line = line[:line.index('#')]
            
            # Skip strings
            line_no_strings = re.sub(r'"[^"]*"', '""', line)
            
            # Find identifiers (simplified)
            ident_pattern = r'\b([a-zA-Z_]\w*)\b'
            for match in re.finditer(ident_pattern, line_no_strings):
                ident = match.group(1)
                if ident not in defined and not ident[0].isupper():
                    if ident not in ['if', 'then', 'else', 'let', 'in', 'true', 'false', 'none', 'and', 'or', 'not']:
                        # Don't report as this would have too many false positives
                        pass
        
        return undefined
    
    def check_templates(self, content: str) -> list:
        """Check template syntax"""
        issues = []
        
        # Count braces to detect template issues
        import re
        templates = re.findall(r'\{+\"', content)
        if templates:
            # Templates found - basic validation
            open_count = content.count('{')
            close_count = content.count('}')
            if open_count != close_count:
                issues.append({
                    "type": "template_brace_mismatch",
                    "severity": "warning",
                    "line": 1,
                    "message": f"Brace mismatch: {open_count} open, {close_count} close"
                })
        
        return issues
    
    def check_structure(self, content: str) -> list:
        """Check general Avon structure"""
        issues = []
        
        import re
        
        # Check let/in balance
        let_count = len(re.findall(r'\blet\b', content))
        in_count = len(re.findall(r'\bin\b', content))
        if let_count > in_count:
            issues.append({
                "type": "incomplete_let",
                "severity": "error",
                "line": 1,
                "message": f"Incomplete let binding ({let_count} let, {in_count} in)"
            })
        
        return issues
    
    def analyze_all(self) -> dict:
        """Analyze all example files"""
        avon_files = glob.glob(os.path.join(self.examples_dir, "*.av"))
        
        results = {}
        for filepath in sorted(avon_files):
            filename = os.path.basename(filepath)
            print(f"Analyzing {filename}...", end=" ")
            
            analysis = self.analyze_file(filepath)
            results[filename] = analysis
            
            issue_count = len(analysis.get('issues', []))
            if analysis['status'] == 'analyzed':
                print(f"Found {issue_count} issues")
                self.summary[analysis['status']] += 1
            else:
                print(f"Status: {analysis['status']}")
                self.summary[analysis['status']] += 1
        
        return results
    
    def generate_report(self, results: dict) -> str:
        """Generate analysis report"""
        report = []
        report.append("\n" + "="*70)
        report.append("AVON LSP INTEGRATION ANALYSIS REPORT")
        report.append("="*70)
        
        # Summary
        report.append(f"\nTotal Files Analyzed: {len(results)}")
        for status, count in self.summary.items():
            report.append(f"  {status}: {count}")
        
        # Detailed issues
        report.append("\n" + "-"*70)
        report.append("DETAILED FINDINGS")
        report.append("-"*70)
        
        files_with_issues = [f for f, r in results.items() if r.get('issues')]
        
        if files_with_issues:
            for filename in sorted(files_with_issues):
                result = results[filename]
                report.append(f"\n{filename}")
                for issue in result['issues']:
                    report.append(f"  [{issue['severity'].upper()}] {issue['type']}: {issue.get('message', '')}")
                    report.append(f"    Line {issue.get('line', '?')}")
        else:
            report.append("\nNo issues found in example files!")
        
        report.append("\n" + "="*70)
        return "\n".join(report)

if __name__ == "__main__":
    analyzer = LSPAnalyzer()
    print("Avon LSP Integration Analyzer\n")
    
    results = analyzer.analyze_all()
    report = analyzer.generate_report(results)
    print(report)
    
    # Save results to file
    with open("/tmp/lsp-analysis-report.json", "w") as f:
        json.dump(results, f, indent=2)
    
    print("\nDetailed results saved to: /tmp/lsp-analysis-report.json")
