# Avon Security Guide

## Overview

Avon is designed with security as a core principle. This document explains the security measures implemented and how to use Avon safely.

## Security Principles

1. **Path Isolation**: Prevent access to files outside the deployment root
2. **Input Validation**: Reject malformed or dangerous input
3. **Type Safety**: Enforce type checking at runtime
4. **No Arbitrary Code Execution**: Avon is a data transformation language, not a shell
5. **Resource Limits**: Protect against resource exhaustion

## Known Security Features

### 1. Path Traversal Prevention

**Problem**: Untrusted input could access sensitive files
```avon
readfile "../../etc/passwd"  # Should be blocked
```

**Solution**: Path validation blocks `..` and absolute paths
```rust
fn validate_path(path: &str) -> Result<()> {
    if path.contains("..") {
        return Err("Path traversal not allowed");
    }
    if path.starts_with("/") {
        return Err("Absolute paths not allowed");
    }
    Ok(())
}
```

**Affected Functions**:
- `readfile()` - Read file content
- `import()` - Import another Avon file
- `fill_template()` - Read template file

**Test**: Run `bash scripts/test_security_comprehensive.sh`

### 2. Type Safety

**Problem**: Type errors at runtime could cause unexpected behavior
```avon
"hello" + 5  # Type mismatch
```

**Solution**: Runtime type checking with clear error messages
```
Error: +: type mismatch: expected number/string/list, found number
```

**Type Rules**:
- Arithmetic ops (`+`, `-`, `*`, `/`, `%`): Both operands must be numbers
- String concat (`+`): Both operands must be strings
- List concat (`+`): Both operands must be lists
- Comparison (`==`, `!=`, `>`, `<`, `>=`, `<=`): Operands must be same type
- Logical (`&&`, `||`): Both operands must be booleans

### 3. Malformed Syntax Rejection

**Problem**: Incomplete or malformed code could be silently accepted
```avon
let x = in  # Incomplete
))))))      # Unmatched
```

**Solution**: Parser validates syntax and rejects malformed input
```
Error: Expected expression after '=', got 'in' on line 1
Error: Unexpected token ')' on line 5
```

### 4. Function Application Limits

**Problem**: Malicious code could cause infinite loops or stack overflow
```avon
let f = \x f x in f 1  # Would infinite loop (if recursion allowed)
```

**Solution**: Recursion is intentionally not supported
- No recursive function calls
- No circular dependencies
- See `tutorial/WHY_NO_RECURSION.md` for rationale

### 5. Template Injection Prevention

**Problem**: User input could break out of templates
```avon
let user = "Alice\" more code" in
{"Hello, {user}!"}
```

**Solution**: Template expressions are evaluated, not concatenated
- Variables are interpolated safely
- String escaping works correctly
- Template boundaries are enforced

## Testing Security

### Run Comprehensive Security Tests

```bash
# Full security test suite
bash scripts/test_security_comprehensive.sh

# Specific test categories
bash scripts/test_security_comprehensive.sh | grep "Path Traversal"
bash scripts/test_security_comprehensive.sh | grep "Injection"
bash scripts/test_security_comprehensive.sh | grep "Resource"
```

### Test Coverage

The security test suite covers:

1. **Path Traversal** (9 tests)
   - readfile attacks
   - import attacks
   - fill_template attacks

2. **Injection Attacks** (6 tests)
   - Template expression injection
   - Code injection
   - Dynamic path injection

3. **Malformed Input** (10 tests)
   - Incomplete expressions
   - Bracket mismatches
   - Invalid operators

4. **Resource Exhaustion** (6 tests)
   - Large strings
   - Large lists
   - Deep nesting

5. **Type Safety** (5 tests)
   - Type mismatches
   - Invalid function calls
   - Wrong operand types

6. **Environment Manipulation** (3 tests)
   - Variable shadowing
   - Closure isolation
   - Scope boundaries

7. **Special Characters** (4 tests)
   - Null bytes
   - Unicode handling
   - Escape sequences

8. **Recursion Prevention** (2 tests)
   - Intentional non-support
   - Clear error messages

9. **File System Boundary** (4 tests)
   - Multiple traversals
   - Prefix tricks
   - URL-style paths
   - Windows paths

## Safe Usage Patterns

### ✅ Safe Path Handling

```avon
# Use relative paths only (absolute paths blocked for security)
readfile "config.json"                    # Safe
readfile "templates/app.json"             # Safe
readfile @config/app.json                 # Safe (Path type)

# Don't use absolute paths or traversal
readfile "/etc/passwd"                    # Error: absolute not allowed
readfile "../../secrets.env"              # Error: traversal not allowed
readfile @etc/passwd                      # Error: absolute path blocked

# For deployment, always use --root flag
@config/app.json {"content"}              # Relative path
# Deploy with: avon deploy app.av --root ./output
```

### ✅ Safe User Input

```avon
# Validate input before use with assert
let port_str = "8080" in
let port_num = assert (is_string port_str) (to_int port_str) in
let validated_port = assert (port_num > 1024 && port_num < 65535) port_num in
{"Server running on port {validated_port}"}

# Validate type before use
let user_input = "O'Brien" in
let safe_input = assert (is_string user_input) user_input in
{"Name: {safe_input}"}  # Safely interpolated

# Validate with multiple checks
let config_value = env_var "PORT" in
let port = assert (is_string config_value) (to_int config_value) in
let valid_port = assert (port > 0 && port < 65536) port in
valid_port
```

### ✅ Safe File Operations

```avon
# Deploy to controlled paths only (use --root flag)
@config/app.json {"setting": "value"}
# Deploy with: avon deploy app.av --root ./output

# Don't construct paths from user input
let filename = "malicious" in
@{"{filename}"} {"content"}  # Don't do this! User input in path

# Instead, validate and use safe lists
let allowed = ["config", "data", "logs"] in
let safe_dir = assert (contains allowed "config") "config" in
@{safe_dir}/app.json {"setting": "value"}

# Always use --root flag for deployment
# avon deploy app.av --root ./output
```

### ❌ Unsafe Patterns

```avon
# Don't import user-specified files
import user_input  # DANGER

# Don't construct paths dynamically
let path = user_prefix + "/config" in
readfile path  # DANGER

# Don't eval dynamic code
# (Avon doesn't support eval, but in other languages...)
```

## Deployment Security

### Best Practices

1. **Validate Inputs**
   ```bash
   # Use CLI validation
   avon deploy app.av -username "$(validate_username "$1")"
   ```

2. **Use --root Flag for Deployment**
   ```bash
   # Always use --root to confine deployment to specific directory
   avon deploy app.av --root ./output
   # This ensures files cannot escape the specified directory
   ```

3. **Review Templates Before Deploy**
   ```bash
   # Preview output before writing
   avon eval app.av  # Check output
   avon deploy app.av  # Then deploy
   ```

4. **Use Read-Only Mode When Possible**
   ```bash
   # For CI/CD, use eval instead of deploy
   avon eval template.av > output.yaml
   # Then manually place file
   ```

5. **Audit Avon Programs**
   - Code review all `.av` files
   - Check for path operations
   - Verify no dangerous pattern usage

## Known Limitations

### ✓ What Avon Does Prevent

- Path traversal attacks (via validation)
- Type confusion errors (via runtime checking)
- Malformed syntax execution (via parsing)
- Recursive infinite loops (by design)
- Injection attacks (via template safety)

### ⚠️ What Avon Doesn't Prevent

- **Logic errors in user code**: Avon can't prevent mistakes in your templates
- **Configuration mistakes**: Wrong settings are still wrong
- **File permission issues**: Avon respects OS permissions
- **Timing attacks**: Avon doesn't prevent side-channel attacks
- **DOS via large computations**: Very long folds or maps could be slow

## Reporting Security Issues

If you find a security vulnerability:

1. **Do not** open a public issue
2. **Email** security details to the maintainers
3. **Include**:
   - Specific code that triggers issue
   - Expected vs actual behavior
   - Impact assessment

## Input Validation with assert

The `assert` function is essential for validating user input and ensuring type safety:

```avon
# Validate type
let user_input = env_var "PORT" in
let port = assert (is_string user_input) (to_int user_input) in
let valid_port = assert (port > 0 && port < 65536) port in
valid_port

# Validate before file operations
let filename = env_var "CONFIG_FILE" in
let safe_name = assert (is_string filename) filename in
let validated = assert (contains ["config.json", "app.json"] safe_name) safe_name in
readfile validated

# Validate dictionary structure
let config = json_parse config_json in
let host = assert (is_dict config && has_key config "host") (get config "host") in
let port = assert (is_number (get config "port")) (get config "port") in
{host: host, port: port}
```

**Best Practices:**
- Always validate user input with `assert` before use
- Check types with `is_string`, `is_number`, `is_list`, etc.
- Validate ranges and constraints (e.g., port numbers, array bounds)
- Use `assert` before file operations to ensure paths are safe
- Chain assertions for complex validation

## Security Checklist

Before deploying Avon templates in production:

- [ ] All paths use relative paths only (no absolute paths)
- [ ] `--root` flag used for all deployments
- [ ] All user input validated with `assert`
- [ ] Type checking with `is_string`, `is_number`, etc. before use
- [ ] No user input passed directly to file operations
- [ ] Input validation on all CLI parameters
- [ ] Type safety verified (no type errors in output)
- [ ] No recursion attempts in code
- [ ] File permissions set correctly
- [ ] Security tests pass: `bash scripts/test_security_comprehensive.sh`
- [ ] Code reviewed by another person
- [ ] Deployment rollout plan documented

## Further Reading

- **Path Security**: See implementation in `src/eval.rs` (validate_path function)
- **Type System**: See type checking in `src/eval.rs` (type_mismatch errors)
- **Parser Safety**: See parser validation in `src/parser.rs`
- **Design Decisions**: See `tutorial/WHY_NO_RECURSION.md` for recursion rationale

## Version Info

This document applies to Avon v0.2+

Last updated: November 2025
