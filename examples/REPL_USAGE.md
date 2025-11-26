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

avon> :vars
User-defined variables:
  double : Function
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
avon> @/test.txt {"Hello, {os}"}
FileTemplate:
  Path: /test.txt
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
- `:vars` - List user-defined variables
- `:type <expr>` - Show type of expression
- `:doc <name>` - Show builtin function info
- `:clear` - Clear all user-defined variables
- `:exit` - Exit the REPL

## Use Cases

1. **Learning Avon**: Explore syntax and builtins interactively
2. **Quick Testing**: Test expressions before adding to files
3. **Debugging**: Isolate problematic code
4. **Prototyping**: Build functions step by step
5. **Exploration**: Discover how functions work

