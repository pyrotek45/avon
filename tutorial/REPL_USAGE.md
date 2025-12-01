# Example: Using the REPL for Development

This example demonstrates how to use Avon's REPL for interactive development and debugging. Think of it as a playground where mistakes are cheap and experiments are encouraged.

> **Tip:** The REPL now features **syntax highlighting** for Avon code, making it easier to read and write expressions. Keywords, builtins, strings, numbers, and comments are all color-coded!

> **Tip:** You can use the `:read` command in the REPL to load templates from GitHub using the `--git` flag format, or use `:run` to evaluate templates fetched from GitHub. This makes it easy to experiment with shared templates before deploying them.

## Starting the REPL

```bash
avon repl
```

## Syntax Highlighting

The REPL automatically highlights your input as you type:
- **Keywords** (`let`, `in`, `if`, `then`, `else`, `match`, `fn`) — Magenta
- **Builtins** (`map`, `filter`, `fold`, `print`, etc.) — Blue
- **Strings** and Templates — Green
- **Numbers** — Cyan
- **Comments** — Gray
- **Operators** — Yellow
- **Booleans** (`true`, `false`, `none`) — Cyan

When using `:read` on `.av` files, the output is also syntax highlighted for easy reading.

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

### 2. Using Persistent Variables

```avon
avon> :let double = \x x * 2
Stored: double : Function

avon> double 21
42 : Number

avon> :let config = {host: "localhost", port: 8080}
Stored: config : Dict

avon> config.host
localhost : String

avon> :vars
User-defined variables:
  config : Dict = { 2 keys }
  double : Function

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
  map             markdown_to_html md_code         md_heading      md_link
  md_list         neg             os              pad_left
  pad_right       readfile        readlines       replace
  reverse         set             split           starts_with
  tail            take            to_bool         to_float
  to_int          to_string       trace           trim
  truncate        typeof          upper           values
  walkdir

Available REPL commands (use :doc <command> for details):
  :help            :exit            :clear           :vars            :let             :inspect       
  :unlet           :read            :run             :eval            :preview         :deploy        
  :deploy-expr     :write           :history         :save-session     :load-session    :assert        
  :test            :benchmark       :benchmark-file  :watch           :unwatch         :pwd             :list            :cd            
  :doc             :type            :sh            

Tip: Use :doc <name> to see detailed documentation for any builtin function or REPL command.

avon> :doc pwd
:pwd
  Show the current working directory.
  Example: :pwd

avon> :doc read
:read <file>
  Read and display the contents of a file.
  Example: :read config.av
  Note: REPL allows any path (absolute or relative) for interactive use.

avon> :doc map
map :: (a -> b) -> [a] -> [b]
  Transform each item in list.
  Example: map (\x x * 2) [1, 2, 3] -> [2, 4, 6]
```

### 3. Type Checking

```avon
avon> :type [1, 2, 3]
Type: List

avon> typeof "hello"
String : String
```

### 4. Debugging with trace

Pro tip: `trace` is like `console.log` but you don't have to feel guilty about leaving it in.

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

No `npm install` required. No `node_modules` folder eating your disk. Just commands.

### Basic Commands
- `:help` or `:h` - Show all available commands
- `:exit` or `:quit` or `:q` - Exit the REPL
- `:clear` - Clear all user-defined variables

### Variable Management
- `:let <name> = <expr>` - Store a value in the REPL (persists across commands)
- `:vars` - List all user-defined variables
- `:inspect <name>` - Show detailed information about a variable
- `:unlet <name>` - Remove a user-defined variable

### File Operations (all support `--git` flag)
- `:read <file> [--git]` - Read and display file contents (syntax highlighted for .av files)
- `:run <file> [--git] [--debug]` - Evaluate file and display result (doesn't modify REPL state)
- `:eval <file> [--git]` - Evaluate file and merge Dict keys into REPL (if result is a Dict)
- `:source <file>` - Alias for `:eval` - execute commands from a file
- `:preview <file> [--git] [--debug]` - Preview what would be deployed without writing files
- `:deploy <file> [--git] [flags...]` - Deploy a file with full CLI flag support:
  - `--git` - Fetch file from GitHub (format: `user/repo/path/file.av`)
  - `--root <dir>` - Deployment root directory
  - `--force` - Overwrite existing files
  - `--backup` - Backup before overwriting
  - `--append` - Append to existing files
  - `--if-not-exists` - Only write if file doesn't exist
  - `--debug` - Enable debug output
  - `-param value` - Pass named arguments to functions
- `:deploy-expr <expr> [--root <dir>]` - Deploy the result of an expression
- `:write <file> <expr>` - Write expression result to file
- `:edit <file>` - Open file in default editor (`$EDITOR` or `$VISUAL`, falls back to `vi`)

### Session Management
- `:history` - Show command history (last 50 entries)
- `:clear-history` - Clear all command history
- `:save-session <file>` - Save REPL state (variables) to file
- `:load-session <file>` - Load REPL state from file

### Testing & Debugging
- `:assert <expr>` - Assert that expression evaluates to true
- `:test <expr> <expected>` - Test that expression equals expected value
- `:benchmark <expr>` - Measure evaluation time for an expression
- `:benchmark-file <file>` - Measure evaluation time for a file
- `:time <expr>` - Time the evaluation of an expression (shows execution time in ms)
- `:watch <name>` - Watch a variable and show when it changes
- `:unwatch <name>` - Stop watching a variable

### Navigation & Shell
- `:pwd` - Show current working directory
- `:list [dir]` - List directory contents
- `:cd <dir>` - Change working directory
- `:sh <command>` - Execute shell command

### Documentation & Types
- `:doc` - Show all available builtin functions and REPL commands
- `:doc <name>` - Show detailed documentation for a builtin function or REPL command
- `:type <expr>` - Show type of expression

### Session Reports
- `:report` - Show a detailed session report including:
  - Commands executed count
  - Variables defined
  - Session duration
  - Error count
  - Most recent commands

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

---

## New REPL Features

### Syntax Highlighting

The REPL now provides syntax highlighting for both input and `.av` file output:

```avon
avon> let double = \x x * 2 in map double [1, 2, 3]
      ^^^         ^^          ^^^
      keyword     function    builtin (highlighted in color!)
```

When reading `.av` files, syntax highlighting is automatically applied:
```avon
avon> :read examples/hello.av
# Output is syntax highlighted for .av files
let name = "World" in
@hello.txt {"Hello, {name}!"}
```

### The `--git` Flag

All file commands now support fetching from GitHub:

```avon
# Read a file from GitHub
avon> :read --git pyrotek45/avon/examples/hello.av

# Run a file from GitHub (evaluate without modifying state)
avon> :run --git user/repo/path/config.av

# Evaluate and merge into REPL state
avon> :eval --git user/repo/path/lib.av

# Preview deployment from GitHub
avon> :preview --git user/repo/path/deploy.av

# Deploy from GitHub
avon> :deploy --git user/repo/path/deploy.av --root ./output
```

### Edit Files in Your Editor

Open files directly in your default editor:

```avon
avon> :edit config.av
# Opens config.av in $EDITOR or $VISUAL (falls back to vi)
# After saving and closing, you're back in the REPL
```

### Timing Expressions

Measure how long expressions take to evaluate:

```avon
avon> :time fold([1..1000], 0, \a \b a + b)
Result: 500500
Time: 1.234ms (1.234ms)

avon> :time map (\x x * x) [1..10000]
Result: [1, 4, 9, 16, 25, ...]
Time: 5.678ms (5.678ms)
```

### Session Reports

Get an overview of your REPL session:

```avon
avon> :report
═══════════════════════════════════════════════════════════
                     REPL SESSION REPORT
═══════════════════════════════════════════════════════════

Session Duration: 15m 32s
Commands Executed: 47
Errors Encountered: 2

User Variables (5):
  config : Dict{3 keys}
  double : Function
  items : List[10]
  name : String
  port : Number

Watched Variables:
  items

Current Directory: /home/user/project

═══════════════════════════════════════════════════════════
```

### Clear History

Clear your command history when needed:

```avon
avon> :history
Command history (5 entries):
  1: let x = 42
  2: x * 2
  3: :vars
  4: :history

avon> :clear-history
Command history cleared

avon> :history
Command history (0 entries):
```

---

### 7. File Operations

```avon
avon> :read config.av
let port = 8080 in
@config.yml {"port: {port}"}

avon> :run config.av
FileTemplate:
  Path: config.yml
  Content:
port: 8080

avon> :preview config.av
Would deploy 1 file(s):
  Path: config.yml
  Content:
port: 8080
```

### 8. Deployment from REPL

```avon
avon> :let env = "prod"
Stored: env : String

avon> :deploy config.av --root ./output --backup
Deployment completed successfully

avon> :deploy-expr @test.txt {"Hello"} --root ./output
Deployed: ./output/test.txt
Deployment completed successfully
```

### 9. Writing Files and Session Management

```avon
avon> :let result = "Hello, world!"
Stored: result : String

avon> :write output.txt result
Written to: output.txt

avon> :history
Command history (5 entries):
  1: :let result = "Hello, world!"
  2: result
  3: :write output.txt result
  4: :history

avon> :save-session my_session.avon
Session saved to: my_session.avon (1 variables)

avon> :clear
Cleared all user-defined variables

avon> :load-session my_session.avon
Session loaded from: my_session.avon (1 variables restored)

avon> :vars
User-defined variables:
  result : String = "Hello, world!"
```

#### Session File Format

Session files are plain Avon source code. When you save a session, Avon generates code that recreates your variables:

```avon
# Avon REPL Session
# Saved variables:

let result = "Hello, world!" in
let count = 42 in
let items = [1, 2, 3] in
let config = {name: "app", port: 8080} in
{result: result, count: count, items: items, config: config}
```

This means session files are human-readable and can be edited manually.

#### What Can Be Saved

| Value Type | Saveable? | Notes |
|------------|-----------|-------|
| String | ✓ | Quotes are escaped automatically |
| Number (Int/Float) | ✓ | Preserved exactly |
| Boolean | ✓ | `true` or `false` |
| List | ✓ | All elements must be saveable |
| Dict | ✓ | All values must be saveable |
| Function | ✗ | Cannot serialize captured environment |
| Template | ✗ | Use strings instead |
| FileTemplate | ✗ | Deploy files, don't save them |
| None | ✗ | No meaningful value to save |

**Example with mixed types:**

```avon
avon> :let name = "Alice"
Stored: name : String

avon> :let age = 30
Stored: age : Number

avon> :let greet = \x {"Hello, {x}!"}
Stored: greet : Function

avon> :save-session data.avon
Warning: Cannot save function 'greet' (functions cannot be serialized)
Session saved to: data.avon (2 variables)
```

Functions cannot be saved because they may capture runtime state that can't be represented as text.

#### Use Cases

**Preserving work across sessions:**
```avon
# End of day - save your work
avon> :save-session project_state.avon
Session saved to: project_state.avon (5 variables)
avon> :exit

# Next day - restore your work
$ avon repl
avon> :load-session project_state.avon
Session loaded from: project_state.avon (5 variables restored)
avon> :vars
```

**Sharing configurations:**
```avon
# Create reusable configuration
avon> :let db_host = "localhost"
avon> :let db_port = 5432
avon> :let db_name = "myapp"
avon> :save-session db_config.avon
Session saved to: db_config.avon (3 variables)

# Share db_config.avon with teammates
```

**Creating test fixtures:**
```avon
# Set up test data
avon> :let users = [{name: "Alice", id: 1}, {name: "Bob", id: 2}]
avon> :let settings = {debug: true, timeout: 30}
avon> :save-session test_fixtures.avon
```

### 10. Testing and Validation

```avon
avon> :let double = \x x * 2
Stored: double : Function

avon> :test (double 21) 42
✓ PASS: (double 21) == 42

avon> :assert (double 0) == 0
✓ PASS: Assertion passed

avon> :benchmark map double [1..1000]
Result: [2, 4, 6, 8, 10, ...]
Time: 16.75ms (16.75ms)

avon> :benchmark-file config.av
File: config.av
Result: {...}
Time: 0.11ms (0.11ms)
```

### 11. Variable Watching and File Navigation

```avon
avon> :let data = [1, 2, 3]
Stored: data : List

avon> :watch data
Watching: data = [1, 2, 3]

avon> :let data = map (\x x * 2) data
[WATCH] data changed: [1, 2, 3] -> [2, 4, 6]
Stored: data : List

avon> :let data = filter (\x x > 3) data
[WATCH] data changed: [2, 4, 6] -> [4, 6]
Stored: data : List

avon> :unwatch data
Stopped watching: data

avon> :pwd
/workspaces/avon

avon> :list
Current directory: /workspaces/avon
  examples/
  src/
  Cargo.toml
  README.md

avon> :cd examples
Changed directory to: /workspaces/avon/examples

avon> :pwd
/workspaces/avon/examples

avon> :list
Current directory: /workspaces/avon/examples
  hello.av
  config.av
  ...
```

### 12. Variable Watching with :unwatch

```avon
avon> :let x = 1
Stored: x : Number

avon> :watch x
Watching: x = 1

avon> :let x = 2
[WATCH] x changed: 1 -> 2
Stored: x : Number

avon> :let x = 3
[WATCH] x changed: 2 -> 3
Stored: x : Number

avon> :unwatch x
Stopped watching: x

avon> :let x = 4
Stored: x : Number
# No watch message since we stopped watching

avon> :watch
No variables being watched.
  Use :watch <name> to watch a variable
  Use :unwatch <name> to stop watching a variable
```

## Use Cases

1. **Learning Avon**: Explore syntax and builtins interactively
2. **Quick Testing**: Test expressions before adding to files
3. **Debugging**: Isolate problematic code
4. **Prototyping**: Build functions step by step
5. **Exploration**: Discover how functions work
6. **Interactive Development**: Build up configurations incrementally
7. **File Management**: Read, evaluate, and deploy files without leaving REPL
8. **Rapid Iteration**: Test and deploy in the same session
9. **Procrastination**: Look busy while technically doing nothing (we won't judge)

---

## See Also

- [TUTORIAL.md](./TUTORIAL.md) — Learn Avon from scratch
- [FEATURES.md](./FEATURES.md) — Complete language reference
- [DEBUGGING_GUIDE.md](./DEBUGGING_GUIDE.md) — Debugging tools and techniques
- [STYLE_GUIDE.md](./STYLE_GUIDE.md) — Best practices and conventions

