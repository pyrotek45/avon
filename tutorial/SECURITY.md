# Avon Security Guide

## Overview

Avon is designed with security as a core principle. This document explains the security measures implemented and how to use Avon safely. Because nobody wants to be "that person" who accidentally deployed credentials to production.

> Tip: Using `--git` Safely:** The `--git` flag fetches templates from GitHub's raw content API. Only use `--git` with repositories you trust. Review the template code before deploying, especially when using templates from unknown sources. The `--git` flag is a powerful feature for sharing templates, but always verify the source.

## Security Principles

1. **Path Isolation**: Prevent access to files outside the deployment root
2. **Input Validation**: Reject malformed or dangerous input
3. **Type Safety**: Enforce type checking at runtime
4. **No Arbitrary Code Execution**: Avon is a data transformation language, not a shell
5. **Resource Limits**: Protect against resource exhaustion

## Known Security Features

### 1. Path Traversal Prevention & Path Type Safety

**Problem**: Untrusted input could access sensitive files or escape deployment boundaries
```avon
readfile "../../etc/passwd"          # Should be blocked - path traversal
@/etc/config.txt {"data"}             # Should be blocked - absolute deployment path
```

**Solution**: Two-layer security model

**Layer 1: Syntax-level protection (Lexer)**
- Path literals (`@path`) cannot start with `/` (absolute paths blocked)
- This prevents accidental absolute path deployment
- Error: `Absolute paths are not allowed in Avon syntax`
- Use relative paths with `--root` flag for deployment

**Layer 2: Runtime validation**
```rust
fn validate_path(path: &str) -> Result<()> {
    if path.contains("..") {
        return Err("Path traversal not allowed");
    }
    // Note: Absolute paths in strings are allowed for safe read operations
    Ok(())
}
```

**Safe patterns:**
```avon
# ✅ SAFE: Reading with absolute path (string)
readfile "/home/user/config.json"     # OK - strings allow absolute for reading

# ✅ SAFE: Path literals are always relative, used with + for composition
let p = @config + @app.json            # OK - results in "config/app.json"

# ✅ SAFE: Deployment with relative paths + --root
@config/app.json {"settings"}          # OK - relative path
# Deploy: avon deploy app.av --root /opt/myapp

# ❌ BLOCKED: Path traversal
readfile "../../etc/passwd"            # Error - traversal blocked

# ❌ BLOCKED: Absolute path literals
@/etc/config.txt {"data"}              # Error - lexer blocks @/
```

**Affected Functions**:
- `readfile()` - Accepts String (absolute OK) or Path (relative only); blocks `..`
- `import()` - Same as readfile
- `fill_template()` - Same as readfile
- `readlines()` - Same as readfile
- `exists()` - Same as readfile
- `basename()`, `dirname()` - Accept Path or String values
- **Path concatenation** (`+`) - Combines relative paths: `@a + @b` → `"a/b"`
- **Deployment** - Path literals must be relative; use `--root` for absolute base directory

**Key insight**: 
- **Reading files** is safe with absolute paths → use strings: `readfile "/absolute/path"`
- **Writing files** (deployment) uses path literals (`@...`) → always relative + `--root`

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
- Logical (`&&`, `||`, `not`): Operands must be booleans

No implicit coercion. `"5" + 3` is an error, not `"53"` or `8` depending on the phase of the moon.

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
- Your code will always terminate (unlike some meetings)
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
# Reading files: strings can be absolute (safe for reading)
readfile "config.json"                    # Safe (relative string)
readfile "/tmp/data.txt"                  # Safe (absolute string for reading)
readfile "../../secrets.env"              # Error: traversal not allowed

# Path literals (@...): syntactically must be relative (for deployment)
@config/app.json {"content"}              # Safe (relative Path)
@/etc/passwd {"hack"}                     # Syntax error: absolute paths not allowed

# For absolute deployment, use --root flag
@config/app.json {"content"}              # Relative path
# Deploy: avon deploy app.av --root /var/www
# Result: /var/www/config/app.json
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
# Note: Without --root, absolute paths are rejected; traversal is always rejected
```

### ❌ Unsafe Patterns

These will get you a 3am phone call. Or worse, a Slack message from your CTO.

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

## Security Capabilities

### ✓ What Avon Protects Against

- Path traversal attacks (via validation)
- Type confusion errors (via runtime checking)
- Malformed syntax execution (via parsing)
- Recursive infinite loops (by design)
- Injection attacks (via template safety)

### Understanding Security Boundaries

Avon provides comprehensive security features for template generation and file operations. Like any tool, it operates within the constraints of the operating system and user permissions:

- **Logic errors in user code**: Avon provides powerful debugging tools (`trace`, `debug`, `assert`) to help you catch mistakes
- **Configuration mistakes**: Avon's type system and runtime validation help prevent many common errors
- **File permission issues**: Avon respects OS permissions, ensuring your deployments work within system security boundaries
- **Large computations**: Avon's efficient evaluation engine handles large datasets, and you can optimize with `fold`, `map`, and `filter` for best performance

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

- [ ] All paths avoid traversal (`..`)
- [ ] Path literals (@...) are relative only (syntactic restriction)
- [ ] Deployment uses `--root` flag for absolute base directory
- [ ] Reading files with absolute paths (strings) is acceptable
- [ ] `--root` flag used for all deployments
- [ ] All user input validated with `assert`
- [ ] Type checking with `is_string`, `is_number`, etc. before use
- [ ] No user input passed directly to file operations
- [ ] Input validation on all CLI parameters
- [ ] Type safety verified (no type errors in output)
- [ ] No recursion attempts in code
- [ ] File permissions set correctly
- [ ] Security tests pass: `bash scripts/test_security_comprehensive.sh`
- [ ] Code reviewed by another person (or at minimum, by yourself after coffee)
- [ ] Deployment rollout plan documented

## Further Reading

- **Path Security**: See implementation in `src/eval.rs` (validate_path function)
- **Type System**: See type checking in `src/eval.rs` (type_mismatch errors)
- **Parser Safety**: See parser validation in `src/parser.rs`
- **Design Decisions**: See `tutorial/WHY_NO_RECURSION.md` for recursion rationale

---

## See Also

- [TUTORIAL.md](./TUTORIAL.md) — Learn Avon from scratch
- [BUILTIN_FUNCTIONS.md](./BUILTIN_FUNCTIONS.md) — Complete reference of built-in functions
- [WHY_NO_RECURSION.md](./WHY_NO_RECURSION.md) — Design decisions

## Version Info

This document applies to Avon v0.2+

Last updated: November 2025

<!-- 
If you're reading this, you either really care about security (good) or you're 
looking for vulnerabilities (less good, but at least you're reading the docs).
Either way, thanks for being thorough.
-->
