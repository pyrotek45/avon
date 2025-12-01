#!/usr/bin/env python3
"""
LSP Auto-Fixer
Automatically detects LSP issues and generates fixes for the LSP code
"""

import re
import json
from pathlib import Path
from datetime import datetime

class LSPAutoFixer:
    """Automatically fixes LSP issues"""
    
    def __init__(self, lsp_file: str = "/workspaces/avon/src/bin/avon_lsp.rs"):
        self.lsp_file = lsp_file
        self.issues_found = []
        self.fixes_applied = []
        
    def read_lsp_code(self) -> str:
        """Read LSP source code"""
        with open(self.lsp_file, 'r') as f:
            return f.read()
    
    def write_lsp_code(self, content: str):
        """Write LSP source code with backup"""
        backup_file = f"{self.lsp_file}.backup.{datetime.now().strftime('%Y%m%d_%H%M%S')}"
        Path(self.lsp_file).backup = open(backup_file, 'w').write(Path(self.lsp_file).read_text())
        Path(self.lsp_file).write_text(content)
        print(f"✓ Changes saved (backup: {backup_file})")
    
    def detect_issues(self) -> list:
        """Detect common LSP issues"""
        code = self.read_lsp_code()
        issues = []
        
        # Issue 1: Check for TODO comments (unfinished work)
        todos = re.finditer(r'// TODO:.*', code)
        for match in todos:
            issues.append({
                'type': 'unfinished_work',
                'pattern': match.group(0),
                'message': f"Unfinished work found: {match.group(0)}"
            })
        
        # Issue 2: Check for panic! calls
        panics = re.finditer(r'panic!\(', code)
        for match in panics:
            issues.append({
                'type': 'panic_call',
                'message': "panic!() call found - should return error instead"
            })
        
        # Issue 3: Check for unwrap() calls
        unwraps = re.finditer(r'\.unwrap\(\)', code)
        unwrap_count = sum(1 for _ in unwraps)
        if unwrap_count > 0:
            issues.append({
                'type': 'unwrap_usage',
                'count': unwrap_count,
                'message': f"Found {unwrap_count} .unwrap() calls - could cause panics"
            })
        
        # Issue 4: Check for potential infinite loops
        while_loops = re.finditer(r'while (?:true|i < chars\.len\(\))', code)
        for match in while_loops:
            issues.append({
                'type': 'potential_infinite_loop',
                'message': f"Potential infinite loop: {match.group(0)}"
            })
        
        # Issue 5: Check for unused variables
        let_vars = re.findall(r'let (\w+)\s*=', code)
        var_usage = {}
        for var in let_vars:
            if var not in ['_', 'self']:
                count = len(re.findall(rf'\b{var}\b', code))
                if count == 1:
                    var_usage[var] = count
        
        if var_usage:
            issues.append({
                'type': 'unused_variables',
                'count': len(var_usage),
                'message': f"Found {len(var_usage)} potentially unused variables"
            })
        
        # Issue 6: Check for proper error handling
        if code.count('?') < code.count('unwrap'):
            issues.append({
                'type': 'error_handling',
                'message': "May need more ? operator usage for proper error propagation"
            })
        
        return issues
    
    def suggest_fixes(self) -> list:
        """Suggest fixes for detected issues"""
        code = self.read_lsp_code()
        suggestions = []
        
        # Suggestion 1: Add missing error types
        if 'EvalError' not in code and 'Result<' in code:
            suggestions.append({
                'type': 'missing_error_type',
                'action': 'Check if error types are properly defined'
            })
        
        # Suggestion 2: Add timeout for template parsing
        if 'template' in code.lower() and 'timeout' not in code.lower():
            suggestions.append({
                'type': 'missing_timeout',
                'action': 'Consider adding timeout for template parsing to prevent hangs',
                'code_location': 'check_template_interpolations function'
            })
        
        # Suggestion 3: Improve variable tracking
        if 'HashSet' in code and 'HashMap' not in code:
            suggestions.append({
                'type': 'data_structure',
                'action': 'Consider using HashMap for variable scope tracking',
                'benefit': 'Better scope management and context awareness'
            })
        
        # Suggestion 4: Add comprehensive tests
        if 'test' not in code:
            suggestions.append({
                'type': 'missing_tests',
                'action': 'Add unit tests for core LSP functions',
                'examples': ['check_undefined_variables', 'check_template_interpolations']
            })
        
        return suggestions
    
    def generate_report(self) -> str:
        """Generate detailed report"""
        issues = self.detect_issues()
        suggestions = self.suggest_fixes()
        
        report = []
        report.append("\n" + "="*70)
        report.append("LSP AUTO-FIXER ANALYSIS REPORT")
        report.append("="*70)
        
        report.append(f"\nFile: {self.lsp_file}")
        
        # Issues
        if issues:
            report.append(f"\n⚠️  ISSUES FOUND: {len(issues)}")
            for issue in issues:
                report.append(f"\n  • {issue['type'].upper()}")
                report.append(f"    {issue.get('message', '')}")
        else:
            report.append("\n✓ No critical issues found")
        
        # Suggestions
        if suggestions:
            report.append(f"\n💡 SUGGESTIONS: {len(suggestions)}")
            for sugg in suggestions:
                report.append(f"\n  • {sugg['type'].upper()}")
                report.append(f"    Action: {sugg.get('action', '')}")
                if 'code_location' in sugg:
                    report.append(f"    Location: {sugg['code_location']}")
                if 'benefit' in sugg:
                    report.append(f"    Benefit: {sugg['benefit']}")
        
        report.append("\n" + "="*70)
        return "\n".join(report)
    
    def run_analysis(self) -> bool:
        """Run complete analysis"""
        print("\nLSP Auto-Fixer Analysis")
        print("="*70)
        
        issues = self.detect_issues()
        suggestions = self.suggest_fixes()
        
        print(f"\nIssues Found: {len(issues)}")
        for issue in issues:
            print(f"  - {issue['type']}: {issue.get('message', '')}")
        
        print(f"\nSuggestions: {len(suggestions)}")
        for sugg in suggestions:
            print(f"  - {sugg['type']}: {sugg.get('action', '')}")
        
        report = self.generate_report()
        print(report)
        
        # Save report
        report_file = "/tmp/lsp-autofixer-report.txt"
        with open(report_file, 'w') as f:
            f.write(report)
        print(f"\nReport saved to: {report_file}")
        
        return len(issues) == 0

if __name__ == "__main__":
    fixer = LSPAutoFixer()
    success = fixer.run_analysis()
    
    import sys
    sys.exit(0 if success else 1)
