# Avon Language Grammar

This document defines the formal grammar for the Avon language in Extended Backus-Naur Form (EBNF).

## Lexical Structure

### Tokens

```
INTEGER     ::= ['-'] DIGIT+
FLOAT       ::= ['-'] DIGIT+ '.' DIGIT+
STRING      ::= '"' (ESCAPE | [^"])* '"'
IDENTIFIER  ::= LETTER (LETTER | DIGIT | '_')*
BOOLEAN     ::= 'true' | 'false'
COMMENT     ::= '#' [^\n]*

ESCAPE      ::= '\\' ('n' | 't' | 'r' | '\\' | '"')
LETTER      ::= 'a'..'z' | 'A'..'Z'
DIGIT       ::= '0'..'9'
```

### Operators and Punctuation

```
OPERATORS   ::= '+' | '-' | '*' | '/' | '%' | '==' | '!=' | '>' | '<' | '>=' | '<=' | '&&' | '||' | '->' | '..'
PUNCTUATION ::= '(' | ')' | '[' | ']' | '{' | '}' | ',' | ':' | '?' | '@' | '\' | '|'
```

## Grammar Rules

### Program

```
Program ::= Expression
```

A program is a single expression (Avon's "one expression per file" model).

### Expression

```
Expression ::= OrExpr
OrExpr     ::= AndExpr ('||' AndExpr)*
AndExpr    ::= ComparisonExpr ('&&' ComparisonExpr)*
ComparisonExpr ::= RelExpr (('==' | '!=' | '>' | '<' | '>=' | '<=') RelExpr)*
RelExpr    ::= TermExpr (('+' | '-') TermExpr)*
TermExpr   ::= FactorExpr (('*' | '/' | '%') FactorExpr)*
FactorExpr ::= UnaryExpr | ApplicationExpr
```

### Unary Expressions

```
UnaryExpr ::= '-' AtomExpr
           | AtomExpr
```

### Application

```
ApplicationExpr ::= AtomExpr+
```

Function application is left-associative: `f x y` is `((f x) y)`.

### Atomic Expressions

```
AtomExpr ::= Literal
          | Identifier
          | LetBinding
          | Function
          | Conditional
          | List
          | Range
          | Dictionary
          | Template
          | Path
          | FileTemplate
          | MemberAccess
          | PipeExpr
          | '(' Expression ')'
```

### Literals

```
Literal ::= INTEGER
         | FLOAT
         | STRING
         | BOOLEAN
```

Note: `None` is not a literal but a value returned by certain functions (e.g., `head []`).

### Let Binding

```
LetBinding ::= 'let' Identifier '=' Expression 'in' Expression
```

### Function

```
Function ::= '\' Identifier ('?' Expression)? Expression
```

The optional `? Expression` is the default parameter value.

### Conditional

```
Conditional ::= 'if' Expression 'then' Expression 'else' Expression
```

### List

```
List ::= '[' ']'
       | '[' Expression (',' Expression)* ']'
```

### Range

```
Range ::= '[' Expression '..' Expression ']'
        | '[' Expression ',' Expression '..' Expression ']'
```

The first form generates integers from start to end (inclusive) with step 1.
The second form uses the middle expression as the step.

### Dictionary

```
Dictionary ::= '{' '}'
             | '{' DictEntry (',' DictEntry)* '}'
DictEntry  ::= Identifier ':' Expression
```

### Template

```
Template ::= '{' '"' TemplateContent '"' '}'
TemplateContent ::= (StringChunk | ExprChunk)*
StringChunk ::= [^{"]+
ExprChunk  ::= '{' Expression '}'
```

Templates can use variable brace delimiters:
- Single brace: `{"text {expr}"}`
- Double brace: `{{"text {{expr}}"}}`
- Triple brace: `{{{"text {{{expr}}}"}}}`

### Path

```
Path ::= '@' PathContent
PathContent ::= ('/' | [^/{])+ (PathInterpolation)*
PathInterpolation ::= '{' Expression '}'
```

### FileTemplate

```
FileTemplate ::= Path Template
```

A path followed by a template creates a FileTemplate value.

### Member Access

```
MemberAccess ::= Expression '.' Identifier
```

### Pipe Expression

```
PipeExpr ::= Expression '->' Expression
```

The pipe operator is right-associative: `x -> f -> g` is `x -> (f -> g)`.

### Identifier

```
Identifier ::= LETTER (LETTER | DIGIT | '_')*
```

Identifiers must not be keywords: `let`, `in`, `if`, `then`, `else`, `true`, `false`, `None`.

## Operator Precedence

From highest to lowest:

1. **Member Access** (`.`)
2. **Application** (space-separated)
3. **Unary** (`-`)
4. **Multiplicative** (`*`, `/`)
5. **Additive** (`+`, `-`)
6. **Comparison** (`==`, `!=`, `>`, `<`, `>=`, `<=`)
7. **Logical AND** (`&&`)
8. **Logical OR** (`||`)
9. **Pipe** (`->`)

## Associativity

- **Left-associative**: `+`, `-`, `*`, `/`, `==`, `!=`, `>`, `<`, `>=`, `<=`, `&&`, `||`, application
- **Right-associative**: `->`
- **Non-associative**: `.` (member access)

## Examples

### Simple Expression
```
42
```

### Let Binding
```
let x = 10 in x + 5
```

### Function
```
\x x + 1
```

### Curried Function
```
\x \y x + y
```

### Function with Default
```
\name ? "Guest" "Hello, " + name
```

### Conditional
```
if x > 0 then x else -x
```

### List
```
[1, 2, 3]
```

### Range
```
[1 .. 10]
[0, 2 .. 20]
```

### Dictionary
```
{name: "Alice", age: 30}
```

### Template
```
{"Hello, {name}!"}
```

### Path
```
@/path/to/file
@config/{env}/app.yml
```

### FileTemplate
```
@/output.txt {"Content: {value}"}
```

### Member Access
```
config.host
```

### Pipe
```
"hello" -> upper -> length
```

### Complex Expression
```
let add = \x \y x + y in
let numbers = [1, 2, 3] in
map (\x add x 1) numbers
```

## Why Recursion Is Not Supported

Avon intentionally does **not** support recursive functions (functions that call themselves). This design decision provides several benefits:

### Design Rationale

1. **Simplicity**: The language implementation is simpler without recursion support
2. **Performance**: No overhead from recursion tracking or depth limits
3. **Safety**: No risk of infinite recursion or stack overflow
4. **Clarity**: Error messages are clearer (unknown symbol vs infinite recursion)

### How It Works

When a function is defined, it captures its environment (closure), but it does **not** add itself to that environment. If a function tries to call itself, it will get an "unknown symbol" error:

```avon
let factorial = \n
  if n <= 1 then 1 else n * (factorial (n - 1)) in
factorial 5
# Error: unknown symbol: factorial
```

### Alternatives: Use Iteration

Instead of recursion, use Avon's built-in iteration functions:

```avon
# Factorial using fold
let factorial = \n
  fold (\acc \x acc * x) 1 [1 .. n] in
factorial 5

# Sum using fold
let sum_list = \list
  fold (\acc \x acc + x) 0 list in
sum_list [1, 2, 3, 4, 5]

# Countdown using range
let countdown = \n
  reverse [1 .. n] in
countdown 5
```

These iterative solutions are:
- More efficient (no function call overhead)
- More readable (clear intent)
- Safer (no risk of infinite loops)
- More idiomatic in Avon

## Notes

1. **Comments**: Lines starting with `#` are ignored.
2. **Whitespace**: Significant for separating tokens, but not for structure (except in templates).
3. **Negative Numbers**: Parsed as unary minus on number literals: `-42` is a single token.
4. **Range Syntax**: Requires spaces around `..`: `[1 .. 5]` not `[1..5]`.
5. **Template Indentation**: Templates automatically dedent based on the first non-whitespace line.
6. **No Shadowing**: Variables cannot be shadowed in nested scopes (except `_`).
7. **Immutability**: All bindings are immutable; `let` creates new scopes, doesn't mutate.

