# Example: Using the REPL for Development

This example demonstrates how to use Avon's REPL for interactive development and debugging.

## Starting the REPL

```bash
avon repl
```

## Common REPL Workflows

### 1. Testing Expressions

```avon
avon> 1 + 2
3 : Number

avon> map (\x x * 2) [1, 2, 3]
[2, 4, 6] : List
```

### 2. Defining and Testing Functions

```avon
avon> let double = \x x * 2 in double 21
42 : Number

avon> :doc
Available builtin functions (use :doc <name> for details):
  assert          concat          contains        debug
  filter          flatten         flatmap         fold
  format_binary   format_bool     format_bytes    format_currency
  format_float    format_hex      format_int      format_json
  format_list     format_octal    format_percent  format_scientific
  format_table    get             has_key         html_attr
  html_escape     html_tag        import          indent
  is_alpha        is_alphanumeric is_bool         is_dict
  is_digit        is_empty        is_float        is_function
  is_int          is_list         is_lowercase    is_number
  is_string       is_uppercase    is_whitespace   join
  json_parse      keys            length          lower
  map             md_code         md_heading      md_link
  md_list         neg             os              pad_left
  pad_right       readfile        readlines       replace
  reverse         set             split           starts_with
  tail            take            to_bool         to_float
  to_int          to_string       trace           trim
  truncate        typeof          upper           values
  walkdir

Tip: Use :doc <function> to see detailed documentation for any builtin.
```

### 3. Type Checking

```avon
avon> :type [1, 2, 3]
Type: List

avon> typeof "hello"
String : String
```

### 4. Debugging with trace

```avon
avon> trace "intermediate" (1 + 2)
[TRACE] intermediate: 3
3 : Number

avon> let result = trace "final" (map (\x x * 2) [1, 2, 3]) in result
[TRACE] final: [2, 4, 6]
[2, 4, 6] : List
```

### 5. Testing FileTemplates

```avon
avon> @test.txt {"Hello, {os}"}
FileTemplate:
  Path: test.txt
  Content:
Hello, linux
```

### 6. Multi-line Expressions

```avon
avon> let config = {
...>   host: "localhost",
...>   port: 8080,
...>   debug: true
...> } in config.host
localhost : String
```

## REPL Commands

- `:help` - Show all available commands
- `:doc` - Show all available builtin functions
- `:doc <name>` - Show detailed documentation for a builtin function
- `:type <expr>` - Show type of expression
- `:clear` - Clear all user-defined variables
- `:exit` - Exit the REPL

## Keyboard Shortcuts

The REPL supports command history navigation and emacs-style editing shortcuts:

**History Navigation:**
- `↑` / `↓` - Navigate through previous commands
- History is kept in-memory only (no file is created)

**Emacs-Style Shortcuts:**
- `Ctrl+A` - Move to beginning of line
- `Ctrl+E` - Move to end of line
- `Ctrl+K` - Delete from cursor to end of line
- `Ctrl+U` - Delete from cursor to beginning of line
- `Ctrl+F` - Move forward one character
- `Ctrl+B` - Move backward one character
- `Ctrl+W` - Delete word backward
- `Ctrl+L` - Clear screen

## Use Cases

1. **Learning Avon**: Explore syntax and builtins interactively
2. **Quick Testing**: Test expressions before adding to files
3. **Debugging**: Isolate problematic code
4. **Prototyping**: Build functions step by step
5. **Exploration**: Discover how functions work

