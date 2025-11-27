# Performance Optimization

Avon's evaluator uses Arc (Atomic Reference Counting) for function environment capture and stack-based scoping for variable bindings.

## Function Environment Capture

Functions capture their creation environment using Arc pointers:

```rust
Value::Function {
    params: Vec<String>,
    body: Box<Expr>,
    env: Arc<HashMap<String, Value>>,
}
```

When a function is created, the current symbol table is wrapped in Arc. This allows:

- **Multiple functions share environment references** without cloning it repeatedly
- **Each function captures the environment at creation time** for correct closures
- **Lazy copying** - the environment is only cloned when needed during function application

## Stack-Based Variable Scoping

Let bindings modify the symbol table in place without cloning:

```rust
symbols.insert(ident, value);           // Add binding
let result = eval(body, symbols);       // Evaluate with binding available
symbols.remove(&ident);                 // Remove binding when done
```

This approach:

- Avoids cloning the symbol table for each let binding
- Maintains proper lexical scoping through add/remove sequencing
- Uses a single mutable reference to the symbol table during evaluation
- Properly restores scope when let bindings end

## Performance Characteristics

### Memory Efficiency

Deep nesting with many let bindings is efficient because:
- Symbol table mutations don't cause full clones
- Only environment snapshots are cloned (when functions capture their creation environment)
- Arc references avoid redundant copies

### Correctness

The design maintains correctness because:
- Functions capture a snapshot at creation time
- Closures receive their creation-time environment
- Variable shadowing is still prevented
- Scoping rules are maintained through proper add/remove sequencing

## When Cloning Happens

Cloning only happens:
1. When a function is created (wraps current environment in Arc)
2. When a function is applied (dereferences Arc and clones that snapshot)

All other operations (let bindings, variable lookups) avoid cloning.
