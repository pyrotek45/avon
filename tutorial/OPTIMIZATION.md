# Performance Optimization

Avon's evaluator uses Rc (Reference Counting) for function environment capture and stack-based scoping for variable bindings.

## Function Environment Capture

Functions capture their creation environment using Rc pointers:

```rust
Value::Function {
    params: Vec<String>,
    body: Box<Expr>,
    env: Rc<HashMap<String, Value>>,
}
```

When a function is created, the current symbol table is wrapped in Rc. This allows:

- **Multiple functions share environment references** without cloning it repeatedly
- **Each function captures the environment at creation time** for correct closures
- **Lazy copying** - the environment is only cloned when needed during function application

## Stack-Based Variable Scoping

Let bindings modify the symbol table in place without cloning:

```rust
symbols.insert(ident, value);           // Add binding
let result = eval(body, symbols);       // Evaluate expression after 'in' with binding available
symbols.remove(&ident);                 // Remove binding after expression is evaluated
```

This approach:

- Avoids cloning the symbol table for each let binding
- Maintains proper lexical scoping through add/remove sequencing
- Uses a single mutable reference to the symbol table during evaluation
- Variables are removed immediately after their expression (after `in`) is fully evaluated, not after the entire file

**Key point:** In nested lets like `let x = let y = 29 in y in x`, the inner variable `y` is removed before the outer variable `x` is even used in the final expression. This ensures variables are only available within their proper scope.

## Performance Characteristics

### Memory Efficiency

Deep nesting with many let bindings is efficient because:
- Symbol table mutations don't cause full clones
- Only environment snapshots are cloned (when functions capture their creation environment)
- Rc references avoid redundant copies

### Correctness

The design maintains correctness because:
- Functions capture a snapshot at creation time
- Closures receive their creation-time environment
- Variable shadowing is still prevented
- Scoping rules are maintained through proper add/remove sequencing

## When Cloning Happens

Cloning only happens:
1. When a function is created (wraps current environment in Rc)
2. When a function is applied (dereferences Rc and clones that snapshot)

All other operations (let bindings, variable lookups) avoid cloning.
