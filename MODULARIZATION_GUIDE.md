# Avon Eval Module Modularization Guide

## Overview

The `src/eval.rs` file (previously ~4750 lines) has been modularized into a cleaner directory structure. The registry of builtin functions is now in a separate module, making it much easier to add new builtins.

## Current Directory Structure

```
src/
├── eval/
│   ├── mod.rs              # Main eval module - core eval logic (~4000 lines)
│   │                       # Contains: eval(), apply_function(), execute_builtin()
│   │                       # Template rendering, Value impl, helpers
│   │
│   └── builtins/
│       ├── mod.rs          # Re-exports from registry (10 lines)
│       └── registry.rs     # Builtin registry (~400 lines)
│                           # Contains: is_builtin_name(), initial_builtins(), get_builtin_arity()
└── ...
```

## Benefits Achieved

1. **Centralized registry** - All builtin metadata in one place (registry.rs)
2. **Easy to add new functions** - Just update registry.rs + execute_builtin in mod.rs
3. **Single source of truth** - Arity and names defined once
4. **Unit tests** - Registry has tests to verify consistency
5. **Reduced from 4750 to 3998 lines** in main file (752 lines moved to registry)

## Adding a New Builtin Function

### Step 1: Add to Registry (`src/eval/builtins/registry.rs`)

1. Add the function name to `is_builtin_name()` (alphabetically sorted by category):
```rust
pub fn is_builtin_name(name: &str) -> bool {
    matches!(
        name,
        // ... existing functions ...
            | "your_new_func"  // Add to appropriate category
            | ...
    )
}
```

2. Add arity in `get_builtin_arity()`:
```rust
pub fn get_builtin_arity(name: &str) -> Option<usize> {
    Some(match name {
        // Arity 1
        "existing" | "your_new_func" => 1,  // Or appropriate arity
        // ...
    })
}
```

3. Add to `initial_builtins()`:
```rust
pub fn initial_builtins() -> HashMap<String, Value> {
    // ... 
    add_builtin!("your_new_func");
    // ...
}
```

### Step 2: Implement in execute_builtin (`src/eval/mod.rs`)

Find the `execute_builtin` function and add your implementation:
```rust
pub fn execute_builtin(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    match name {
        // ... existing cases ...
        "your_new_func" => {
            // Your implementation here
            let arg1 = &args[0];
            // ...
            Ok(Value::String(result))
        }
        // ...
    }
}
```

### Step 3: Update CLI Documentation (`src/cli.rs`)

Add documentation in `get_builtin_doc()`:
```rust
("your_new_func", "your_new_func :: Type -> Type -> Type\n  Description.\n  Example: your_new_func arg1 arg2 -> result"),
```

### Step 4: Update VSCode Extension

**File:** `vscode/syntaxes/avon.tmLanguage.json`

Add to the builtin regex pattern (line ~38):
```json
"match": "(?<!\\.)\\b(existing|your_new_func|more)\\b(?=\\s|\\(|\\[|\\||->|\\{)"
```

### Step 5: Add Tests

Create tests for your new function or add to existing test files.

## Function Categories in Registry

The registry organizes functions by category:
- **Aggregate functions**: all, any, count, max, min, sum
- **Assertion/Debug**: assert, debug, error, not, trace
- **Date/Time**: date_add, date_diff, date_format, date_parse, now, timestamp, timezone
- **Dictionary**: dict_merge, get, has_key, keys, set, values
- **Environment**: env_var, env_var_or, os
- **File I/O**: basename, dirname, exists, fill_template, import, json_parse, readfile, readlines, walkdir
- **Formatting**: center, format_*, truncate
- **HTML**: html_attr, html_escape, html_tag
- **List operations**: drop, enumerate, filter, flatmap, flatten, fold, head, map, partition, range, reverse, sort, sort_by, split_at, tail, take, unique, unzip, zip
- **Markdown**: markdown_to_html, md_*
- **Math**: neg
- **String operations**: char_at, chars, concat, contains, ends_with, indent, is_*, join, length, lower, pad_*, repeat, replace, slice, split, starts_with, trim, upper
- **Type checking**: is_bool, is_dict, is_float, is_function, is_int, is_list, is_none, is_number, is_string, typeof
- **Type conversion**: to_bool, to_char, to_float, to_int, to_list, to_string

## Registry Tests

The registry includes unit tests to ensure consistency:
- `test_all_builtins_have_arity` - Every builtin has an arity defined
- `test_all_builtins_are_registered` - Every builtin in initial_builtins is in is_builtin_name

Run tests with: `cargo test`

## Future Improvements

If the codebase grows further, consider:
1. Splitting `execute_builtin` into category-specific files (string.rs, list.rs, etc.)
2. Using a trait-based approach for builtins
3. Auto-generating registry from implementations using macros
