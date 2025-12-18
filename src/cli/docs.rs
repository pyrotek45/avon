// CLI documentation functions

/// Get documentation for a category of functions
pub fn get_category_doc(category: &str) -> Option<String> {
    let cat = category.to_lowercase();
    match cat.as_str() {
        "string" | "strings" | "str" | "text" => Some(format!(
            "String Functions:\n\
             ─────────────────\n\
             Text manipulation and inspection functions.\n\n\
             Basic Operations:\n\
             {:<16} Concatenate two strings\n\
             {:<16} Convert to uppercase\n\
             {:<16} Convert to lowercase\n\
             {:<16} Remove leading/trailing whitespace\n\
             {:<16} Get length of string\n\
             {:<16} Repeat string n times\n\n\
             Searching:\n\
             {:<16} Check if string contains substring\n\
             {:<16} Check if string starts with prefix\n\
             {:<16} Check if string ends with suffix\n\n\
             Splitting/Joining:\n\
             {:<16} Split string by delimiter\n\
             {:<16} Join list of strings with separator\n\
             {:<16} Split string into words (by whitespace)\n\
             {:<16} Join words with single space\n\
             {:<16} Split string into lines\n\
             {:<16} Join lines with newlines\n\n\
             Modification:\n\
             {:<16} Replace all occurrences\n\
             {:<16} Indent each line\n\
             {:<16} Pad string on left\n\
             {:<16} Pad string on right\n\n\
             Character Operations:\n\
             {:<16} Get character at index\n\
             {:<16} Convert string to list of chars\n\
             {:<16} Extract substring\n\n\
             Predicates:\n\
             {:<16} Check if all chars are digits\n\
             {:<16} Check if all chars are alphabetic\n\
             {:<16} Check if all chars are alphanumeric\n\
             {:<16} Check if all chars are whitespace\n\
             {:<16} Check if all chars are uppercase\n\
             {:<16} Check if all chars are lowercase\n\
             {:<16} Check if string is empty\n\n\
             Use :doc <function> for detailed documentation.",
            "concat",
            "upper",
            "lower",
            "trim",
            "length",
            "repeat",
            "contains",
            "starts_with",
            "ends_with",
            "split",
            "join",
            "words",
            "unwords",
            "lines",
            "unlines",
            "replace",
            "indent",
            "pad_left",
            "pad_right",
            "char_at",
            "chars",
            "slice",
            "is_digit",
            "is_alpha",
            "is_alphanumeric",
            "is_whitespace",
            "is_uppercase",
            "is_lowercase",
            "is_empty"
        )),
        "list" | "lists" | "array" | "arrays" => Some(format!(
            "List Functions:\n\
             ───────────────\n\
             Operations for working with lists (arrays).\n\n\
             Higher-Order Functions:\n\
             {:<16} Transform each item\n\
             {:<16} Keep items matching predicate\n\
             {:<16} Reduce list to single value\n\
             {:<16} Map then flatten one level\n\
             {:<16} Flatten nested list one level\n\n\
             Parallel Processing:\n\
             {:<16} Parallel map (multi-threaded)\n\
             {:<16} Parallel filter (multi-threaded)\n\
             {:<16} Parallel fold (multi-threaded)\n\n\
             Accessing Elements:\n\
             {:<16} Get first item\n\
             {:<16} Get last item\n\
             {:<16} Get all items except first\n\
             {:<16} Get item at index\n\
             {:<16} Get length of list\n\n\
             Searching:\n\
             {:<16} Find first matching item\n\
             {:<16} Find index of first match\n\n\
             Slicing:\n\
             {:<16} Take first n items\n\
             {:<16} Drop first n items\n\
             {:<16} Extract sublist\n\
             {:<16} Split list at index\n\
             {:<16} Split into chunks of size n\n\
             {:<16} Create sliding windows\n\n\
             Combining:\n\
             {:<16} Combine two lists into pairs\n\
             {:<16} Combine two lists with a function\n\
             {:<16} Split list of pairs into two lists\n\
             {:<16} Add index to each element\n\
             {:<16} Insert separator between elements\n\n\
             Reordering:\n\
             {:<16} Reverse list order\n\
             {:<16} Sort in ascending order\n\
             {:<16} Sort by function result\n\
             {:<16} Remove duplicates\n\
             {:<16} Randomly shuffle list\n\n\
             Random Selection:\n\
             {:<16} Pick random element\n\
             {:<16} Pick n random elements\n\n\
             Grouping:\n\
             {:<16} Group by key function\n\
             {:<16} Split by predicate\n\n\
             Aggregation:\n\
             {:<16} Sum all numbers\n\
             {:<16} Multiply all numbers\n\
             {:<16} Find minimum value\n\
             {:<16} Find maximum value\n\
             {:<16} Count matching items\n\n\
             Predicates:\n\
             {:<16} Check if all match predicate\n\
             {:<16} Check if any match predicate\n\n\
             Advanced:\n\
             {:<16} Generate all permutations\n\
             {:<16} Generate all combinations\n\
             {:<16} Transpose 2D list\n\n\
             Use :doc <function> for detailed documentation.",
            "map",
            "filter",
            "fold",
            "flatmap",
            "flatten",
            "pmap",
            "pfilter",
            "pfold",
            "head",
            "last",
            "tail",
            "nth",
            "length",
            "find",
            "find_index",
            "take",
            "drop",
            "slice",
            "split_at",
            "chunks",
            "windows",
            "zip",
            "zip_with",
            "unzip",
            "enumerate",
            "intersperse",
            "reverse",
            "sort",
            "sort_by",
            "unique",
            "shuffle",
            "choice",
            "sample",
            "group_by",
            "partition",
            "sum",
            "product",
            "min",
            "max",
            "count",
            "all",
            "any",
            "permutations",
            "combinations",
            "transpose"
        )),
        "math" | "number" | "numbers" | "numeric" => Some(format!(
            "Math Functions:\n\
             ───────────────\n\
             Mathematical operations on numbers.\n\n\
             Operators:\n\
             {:<16} Power (right-associative: 2**3**2 = 512)\n\
             {:<16} Float division (always returns float)\n\
             {:<16} Integer division (floor toward -∞)\n\n\
             Basic:\n\
             {:<16} Absolute value\n\
             {:<16} Negate a number\n\n\
             Powers & Roots:\n\
             {:<16} Square root\n\
             {:<16} Raise to power (function form)\n\n\
             Rounding:\n\
             {:<16} Round down (toward -infinity)\n\
             {:<16} Round up (toward +infinity)\n\
             {:<16} Round to nearest integer\n\n\
             Logarithms:\n\
             {:<16} Natural logarithm (base e)\n\
             {:<16} Base-10 logarithm\n\n\
             Number Theory:\n\
             {:<16} Greatest common divisor\n\
             {:<16} Least common multiple\n\n\
             Aggregation (on lists):\n\
             {:<16} Sum all numbers in list\n\
             {:<16} Multiply all numbers in list\n\
             {:<16} Find minimum in list\n\
             {:<16} Find maximum in list\n\n\
             Use :doc <function> for detailed documentation.",
            "a ** b",
            "a / b",
            "a // b",
            "abs",
            "neg",
            "sqrt",
            "pow",
            "floor",
            "ceil",
            "round",
            "log",
            "log10",
            "gcd",
            "lcm",
            "sum",
            "product",
            "min",
            "max"
        )),
        "dict" | "dicts" | "dictionary" | "dictionaries" | "object" => Some(format!(
            "Dictionary Functions:\n\
             ─────────────────────\n\
             Operations for key-value dictionaries.\n\n\
             Access:\n\
             {:<16} Get value by key (or None)\n\
             {:<16} Set/update key-value pair\n\
             {:<16} Check if key exists\n\n\
             Inspection:\n\
             {:<16} Get list of all keys\n\
             {:<16} Get list of all values\n\
             {:<16} Check if dictionary is empty\n\n\
             Combining:\n\
             {:<16} Merge two dictionaries\n\n\
             Note: Dict syntax uses braces: {{name: \"Alice\", age: 30}}\n\
             Access fields with dot notation: person.name\n\n\
             Use :doc <function> for detailed documentation.",
            "get", "set", "has_key", "keys", "values", "is_empty", "dict_merge"
        )),
        "file" | "files" | "io" | "filesystem" | "fs" => Some(format!(
            "File & Path Functions:\n\
             ──────────────────────\n\
             Reading files and working with paths.\n\n\
             Reading:\n\
             {:<16} Read entire file as string\n\
             {:<16} Read file as list of lines\n\
             {:<16} Fill template file with values\n\n\
             Path Info:\n\
             {:<16} Check if file/directory exists\n\
             {:<16} Get filename from path\n\
             {:<16} Get directory from path\n\
             {:<16} Convert to absolute path\n\
             {:<16} Convert to relative path\n\n\
             Directory:\n\
             {:<16} List files recursively\n\
             {:<16} Find files matching pattern\n\n\
             Import:\n\
             {:<16} Import and evaluate Avon file\n\n\
             Use :doc <function> for detailed documentation.",
            "readfile",
            "readlines",
            "fill_template",
            "exists",
            "basename",
            "dirname",
            "abspath",
            "relpath",
            "walkdir",
            "glob",
            "import"
        )),
        "type" | "types" | "typecheck" | "checking" => Some(format!(
            "Type Checking Functions:\n\
             ────────────────────────\n\
             Inspect and check value types.\n\n\
             Type Inspection:\n\
             {:<16} Get type name as string\n\n\
             Type Predicates:\n\
             {:<16} Check if value is string\n\
             {:<16} Check if value is number\n\
             {:<16} Check if value is integer\n\
             {:<16} Check if value is float\n\
             {:<16} Check if value is list\n\
             {:<16} Check if value is boolean\n\
             {:<16} Check if value is function\n\
             {:<16} Check if value is dictionary\n\
             {:<16} Check if value is None\n\n\
             Logic:\n\
             {:<16} Logical negation\n\n\
             Use :doc <function> for detailed documentation.",
            "typeof",
            "is_string",
            "is_number",
            "is_int",
            "is_float",
            "is_list",
            "is_bool",
            "is_function",
            "is_dict",
            "is_none",
            "not"
        )),
        "convert" | "conversion" | "cast" | "transform" => Some(format!(
            "Type Conversion Functions:\n\
             ──────────────────────────\n\
             Convert between types.\n\n\
             To String:\n\
             {:<16} Convert any value to string\n\n\
             To Number:\n\
             {:<16} Convert string to integer\n\
             {:<16} Convert string to float\n\n\
             To Boolean:\n\
             {:<16} Convert value to boolean\n\n\
             Character:\n\
             {:<16} Convert codepoint to character\n\
             {:<16} Convert string to char list\n\n\
             Use :doc <function> for detailed documentation.",
            "to_string", "to_int", "to_float", "to_bool", "to_char", "to_list"
        )),
        "format" | "formatting" => Some(format!(
            "Formatting Functions:\n\
             ─────────────────────\n\
             Format values for display.\n\n\
             Numbers:\n\
             {:<16} Zero-padded integer\n\
             {:<16} Float with decimal precision\n\
             {:<16} Hexadecimal\n\
             {:<16} Octal\n\
             {:<16} Binary\n\
             {:<16} Scientific notation\n\
             {:<16} Human-readable bytes\n\
             {:<16} Currency format\n\
             {:<16} Percentage format\n\n\
             Text:\n\
             {:<16} Truncate with ellipsis\n\
             {:<16} Center-align text\n\
             {:<16} Format boolean\n\n\
             Data:\n\
             {:<16} Join list with separator\n\
             {:<16} Format as 2D table\n\
             {:<16} Format as JSON\n\n\
             Use :doc <function> for detailed documentation.",
            "format_int",
            "format_float",
            "format_hex",
            "format_octal",
            "format_binary",
            "format_scientific",
            "format_bytes",
            "format_currency",
            "format_percent",
            "truncate",
            "center",
            "format_bool",
            "format_list",
            "format_table",
            "format_json"
        )),
        "regex" | "pattern" | "patterns" => Some(format!(
            "Regex Functions:\n\
             ────────────────\n\
             Regular expression operations.\n\n\
             Matching:\n\
             {:<16} Check if text matches pattern\n\
             {:<16} Find all matches in text\n\n\
             Transformation:\n\
             {:<16} Replace all matches\n\
             {:<16} Split by pattern\n\n\
             Note: Use Rust regex syntax.\n\
             Common patterns: \\d (digit), \\w (word char), \\s (whitespace)\n\
             Quantifiers: * (0+), + (1+), ? (0-1), {{n}} (exactly n)\n\n\
             Use :doc <function> for detailed documentation.",
            "regex_match", "scan", "regex_replace", "regex_split"
        )),
        "date" | "dates" | "time" | "datetime" => Some(format!(
            "Date/Time Functions:\n\
             ────────────────────\n\
             Working with dates and times.\n\n\
             Current Time:\n\
             {:<16} Get current date/time (ISO 8601)\n\
             {:<16} Get Unix timestamp (seconds)\n\
             {:<16} Get timezone offset\n\n\
             Formatting:\n\
             {:<16} Format date with pattern\n\
             {:<16} Parse date string\n\n\
             Arithmetic:\n\
             {:<16} Add duration to date\n\
             {:<16} Calculate difference in seconds\n\n\
             Duration format: number + unit\n\
             Units: s (seconds), m (minutes), h (hours),\n\
                    d (days), w (weeks), y (years)\n\n\
             Use :doc <function> for detailed documentation.",
            "now", "timestamp", "timezone", "date_format", "date_parse", "date_add", "date_diff"
        )),
        "html" | "markup" => Some(format!(
            "HTML Functions:\n\
             ───────────────\n\
             Generate HTML markup.\n\n\
             Escaping:\n\
             {:<16} Escape special HTML characters\n\n\
             Elements:\n\
             {:<16} Create HTML tag with content\n\
             {:<16} Create attribute string\n\n\
             Markdown:\n\
             {:<16} Convert markdown to HTML\n\n\
             Use :doc <function> for detailed documentation.",
            "html_escape", "html_tag", "html_attr", "markdown_to_html"
        )),
        "markdown" | "md" => Some(format!(
            "Markdown Functions:\n\
             ───────────────────\n\
             Generate Markdown text.\n\n\
             Structure:\n\
             {:<16} Create heading (level 1-6)\n\
             {:<16} Create bulleted list\n\n\
             Inline:\n\
             {:<16} Create link\n\
             {:<16} Create inline code\n\n\
             Conversion:\n\
             {:<16} Convert markdown to HTML\n\n\
             Use :doc <function> for detailed documentation.",
            "md_heading", "md_list", "md_link", "md_code", "markdown_to_html"
        )),
        "debug" | "debugging" | "assert" | "test" | "testing" => Some(format!(
            "Debug & Assert Functions:\n\
             ─────────────────────────\n\
             Debugging, tracing, and validation for complex pipelines.\n\n\
             Tracing (for pipelines):\n\
             {:<16} Print label and value, return value unchanged\n\
             {:<16} Auto-numbered quick debug (no label needed)\n\
             {:<16} Pretty-print internal structure with label\n\
             {:<16} Run function for side effects, return original value\n\n\
             Validation:\n\
             {:<16} Assert condition or error\n\
             {:<16} Throw custom error\n\n\
             Pipeline example:\n\
               data -> spy -> map f -> trace \"after map\" -> filter g -> spy\n\n\
             Use :doc <function> for detailed documentation.",
            "trace", "spy", "debug", "tap", "assert", "error"
        )),
        "parse" | "parsing" | "data" | "json" | "yaml" | "toml" | "csv" => Some(format!(
            "Data Parsing Functions:\n\
             ───────────────────────\n\
             Parse structured data formats.\n\n\
             Parsing:\n\
             {:<16} Parse JSON string\n\
             {:<16} Parse YAML string\n\
             {:<16} Parse TOML string\n\
             {:<16} Parse CSV file\n\n\
             Note: Returns Dict for objects, List for arrays.\n\
             Access parsed data with .key or get dict \"key\"\n\n\
             Use :doc <function> for detailed documentation.",
            "json_parse", "yaml_parse", "toml_parse", "csv_parse"
        )),
        "system" | "sys" | "env" | "environment" => Some(format!(
            "System Functions:\n\
             ─────────────────\n\
             System and environment access.\n\n\
             OS Info:\n\
             {:<16} Get OS name (linux/macos/windows)\n\n\
             Environment:\n\
             {:<16} Read env var (fail if missing)\n\
             {:<16} Read env var with default\n\n\
             Use :doc <function> for detailed documentation.",
            "os", "env_var", "env_var_or"
        )),
        _ => None,
    }
}

/// Get documentation for a REPL command
pub fn get_repl_command_doc(cmd_name: &str) -> Option<String> {
    let docs: std::collections::HashMap<&str, &str> = [
        ("help", ":help, :h\n  Show this help message with all available REPL commands."),
        ("h", ":help, :h\n  Show this help message with all available REPL commands."),
        ("exit", ":exit, :quit, :q\n  Exit the REPL."),
        ("quit", ":exit, :quit, :q\n  Exit the REPL."),
        ("q", ":exit, :quit, :q\n  Exit the REPL."),
        ("cancel", ":cancel, :c, :reset\n  Cancel multi-line input and return to the REPL prompt.\n  Use when you're in multi-line mode and want to start over.\n  You can also press Enter 3 times on empty lines to cancel."),
        ("c", ":cancel, :c, :reset\n  Cancel multi-line input and return to the REPL prompt.\n  Use when you're in multi-line mode and want to start over.\n  You can also press Enter 3 times on empty lines to cancel."),
        ("reset", ":cancel, :c, :reset\n  Cancel multi-line input and return to the REPL prompt.\n  Use when you're in multi-line mode and want to start over.\n  You can also press Enter 3 times on empty lines to cancel."),
        ("clear", ":clear\n  Clear all user-defined variables. Builtin functions are preserved."),
        ("vars", ":vars\n  List all user-defined variables (excluding builtin functions).\n  Example: :vars"),
        ("let", ":let <name> = <expr>\n  Store a value in a variable for later use.\n  Supports multi-line input for lambdas, dicts, lists, and nested let/in.\n  Example: :let x = 42\n  Example: :let config = {host: \"localhost\", port: 8080}\n  Example: :let add = \\x \\y\n             x + y\n  To cancel multi-line input, use :cancel or press Enter 3 times."),
        ("inspect", ":inspect <name>\n  Show detailed information about a variable.\n  Example: :inspect x"),
        ("unlet", ":unlet <name>\n  Remove a user-defined variable.\n  Example: :unlet x\n  Note: Cannot remove builtin functions."),
        ("read", ":read <file>\n  Read and display the contents of a file.\n  Example: :read config.av\n  Note: REPL allows any path (absolute or relative) for interactive use."),
        ("run", ":run <file>\n  Evaluate a file and display the result without modifying REPL state.\n  Example: :run config.av\n  Note: REPL allows any path (absolute or relative) for interactive use."),
        ("eval", ":eval <file>\n  Evaluate a file and merge Dict keys into REPL state.\n  Example: :eval config.av\n  If the file evaluates to a Dict, its keys are added as variables.\n  Note: REPL allows any path (absolute or relative) for interactive use."),
        ("preview", ":preview <file> [flags...]\n  Preview what would be deployed without writing files.\n  Example: :preview template.av\n  Example: :preview template.av --debug\n  Supports CLI flags: --root, --force, --backup, --append, --if-not-exists, --debug, -param value"),
        ("deploy", ":deploy <file> [flags...]\n  Deploy a file template to disk.\n  Example: :deploy config.av --root /tmp\n  Supports CLI flags: --root, --force, --backup, --append, --if-not-exists, --debug, -param value"),
        ("deploy-expr", ":deploy-expr <expr> [flags...]\n  Deploy the result of an expression.\n  Example: :deploy-expr @test.txt {{\"Hello\"}} --root /tmp\n  Supports CLI flags: --root, --force, --backup, --append, --if-not-exists, --debug, -param value"),
        ("write", ":write <file> <expr>\n  Write the result of an expression to a file.\n  Example: :write output.txt \"Hello, World!\"\n  Note: REPL allows any path (absolute or relative) for interactive use."),
        ("history", ":history [N]\n  Show command history. If N is provided, show last N entries.\n  Example: :history\n  Example: :history 10"),
        ("save-session", ":save-session <file>\n  Save all user-defined variables to a file.\n  Example: :save-session my_session.avon\n  The file can be loaded later with :load-session."),
        ("load-session", ":load-session <file>\n  Load variables from a saved session file.\n  Example: :load-session my_session.avon\n  Merges loaded variables into current REPL state."),
        ("assert", ":assert <expr>\n  Assert that an expression evaluates to true.\n  Example: :assert (x > 0)\n  Example: :assert (length list == 5)"),
        ("test", ":test <expr> <expected>\n  Test that an expression equals an expected value.\n  Example: :test (1 + 1) 2\n  Example: :test (upper \"hello\") \"HELLO\""),
        ("benchmark", ":benchmark <expr>\n  Measure the evaluation time of an expression.\n  Example: :benchmark (map (\\x x * 2) [1..1000])\n  Example: :benchmark map double [1..1000]"),
        ("benchmark-file", ":benchmark-file <file>\n  Measure the evaluation time of a file.\n  Example: :benchmark-file config.av\n  Example: :benchmark-file large_program.av"),
        ("watch", ":watch <name>\n  Watch a variable and show when it changes (via :let or expressions).\n  Example: :watch x\n  Use :watch with no argument to list watched variables.\n  Use :unwatch <name> to stop watching."),
        ("unwatch", ":unwatch <name>\n  Stop watching a variable.\n  Example: :unwatch x"),
        ("pwd", ":pwd\n  Show the current working directory.\n  Example: :pwd"),
        ("list", ":list [dir]\n  List directory contents. Shows current directory if no argument.\n  Example: :list\n  Example: :list ./examples\n  Note: REPL allows any path (absolute or relative) for interactive use."),
        ("cd", ":cd <dir>\n  Change the current working directory.\n  Example: :cd ./examples\n  Example: :cd /tmp\n  Note: REPL allows any path (absolute or relative) for interactive use."),
        ("doc", ":doc [name]\n  Show documentation. Without argument, lists all builtin functions.\n  With argument, shows documentation for a builtin function or REPL command.\n  Example: :doc\n  Example: :doc map\n  Example: :doc pwd"),
        ("type", ":type <expr>\n  Show the type of an expression.\n  Example: :type [1, 2, 3]\n  Example: :type \"hello\""),
        ("sh", ":sh <command>\n  Execute a shell command.\n  Example: :sh ls -la\n  Example: :sh echo hello\n  Note: This executes a single shell command. For interactive shell, exit REPL and use your terminal."),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(cmd_name).map(|s| s.to_string())
}

/// Get documentation for a specific builtin function
pub fn get_builtin_doc(func_name: &str) -> Option<String> {
    let docs: std::collections::HashMap<&str, &str> = [
        // String Operations
        ("concat", "concat :: (String|Template) -> (String|Template) -> String\n  Concatenate (join) two strings together.\n  \n  Arguments:\n    1. First string\n    2. Second string\n  \n  Example: concat \"hello\" \" world\" -> \"hello world\"\n  Example: concat \"file_\" \"2024\" -> \"file_2024\"\n  \n  Tip: For multiple strings, use join instead\n  Note: Both arguments accept strings and templates (templates auto-convert to strings)"),
        ("upper", "upper :: (String|Template) -> String\n  Convert all letters in string to UPPERCASE.\n  \n  Arguments:\n    1. String to convert\n  \n  Example: upper \"hello\" -> \"HELLO\"\n  Example: upper \"Hello World\" -> \"HELLO WORLD\"\n  \n  Use case: Normalize strings for comparison, generate constants\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("lower", "lower :: (String|Template) -> String\n  Convert all letters in string to lowercase.\n  \n  Arguments:\n    1. String to convert\n  \n  Example: lower \"HELLO\" -> \"hello\"\n  Example: lower \"Hello World\" -> \"hello world\"\n  \n  Use case: Normalize strings for comparison, generate filenames\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("trim", "trim :: (String|Template) -> String\n  Remove spaces, tabs, and newlines from beginning and end of string.\n  \n  Arguments:\n    1. String to trim\n  \n  Example: trim \"  hello  \" -> \"hello\"\n  Example: trim \"\\n\\tworld\\n\" -> \"world\"\n  \n  Use case: Clean up user input, remove extra whitespace from file contents\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("split", "split :: (String|Template) -> (String|Template) -> [String]\n  Split a string into a list of parts using a delimiter.\n  \n  Arguments:\n    1. String to split\n    2. Delimiter (what to split by)\n  \n  Example: split \"a,b,c\" \",\" -> [\"a\", \"b\", \"c\"]\n           Split by comma\n  \n  Example: split \"one-two-three\" \"-\" -> [\"one\", \"two\", \"three\"]\n           Split by dash\n  \n  Example: split \"line1\\nline2\\nline3\" \"\\n\" -> [\"line1\", \"line2\", \"line3\"]\n           Split by newline (or use 'lines' function)\n  \n  Tip: Use with map to process each part\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("join", "join :: [String] -> (String|Template) -> String\n  Join a list of strings into one string, with a separator between each.\n  \n  Arguments:\n    1. List of strings to join\n    2. Separator to put between strings\n  \n  Example: join [\"a\", \"b\", \"c\"] \", \" -> \"a, b, c\"\n           Join with comma and space\n  \n  Example: join [\"file\", \"txt\"] \".\" -> \"file.txt\"\n           Join with dot\n  \n  Example: join [\"usr\", \"local\", \"bin\"] \"/\" -> \"usr/local/bin\"\n           Build a path\n  \n  Tip: Opposite of split\n  Note: Separator accepts both strings and templates (templates auto-convert to strings)"),
        ("replace", "replace :: (String|Template) -> (String|Template) -> (String|Template) -> String\n  Replace ALL occurrences of a substring with another string.\n  \n  Arguments:\n    1. Original string\n    2. Substring to find\n    3. Replacement string\n  \n  Example: replace \"hello\" \"l\" \"L\" -> \"heLLo\"\n           Replace all 'l' with 'L'\n  \n  Example: replace \"foo_bar_baz\" \"_\" \"-\" -> \"foo-bar-baz\"\n           Replace underscores with dashes\n  \n  Example: replace \"v1.0.0\" \"1\" \"2\" -> \"v2.0.0\"\n           Bump version number\n  \n  Tip: Replaces ALL matches, not just the first one\n  Note: All arguments accept both strings and templates (templates auto-convert to strings)"),
        ("contains", "contains :: (String|Template) -> (String|Template|List) -> Bool\n  Check if a string contains a substring, or if a list contains an element.\n  \n  For strings:\n    Arguments:\n      1. String to search in\n      2. Substring to search for\n  \n    Example: contains \"hello world\" \"world\" -> true\n             \"world\" is in \"hello world\"\n  \n    Example: contains \"hello\" \"xyz\" -> false\n             \"xyz\" is not found\n  \n  For lists (element membership):\n    Arguments:\n      1. Element to search for\n      2. List to search in\n  \n    Example: contains 3 [1, 2, 3, 4] -> true\n             3 is in the list\n  \n    Example: contains \"apple\" [\"banana\", \"cherry\"] -> false\n             \"apple\" is not in the list\n  \n    Example: filter (\\x contains x valid_values) input_list\n             Keep only valid items\n  \n  Use case: Search text, validate content, check membership\n  Note: For strings, both arguments accept strings and templates"),
        ("starts_with", "starts_with :: (String|Template) -> (String|Template) -> Bool\n  Check if a string starts with a specific prefix.\n  \n  Arguments:\n    1. String to check\n    2. Prefix to look for\n  \n  Example: starts_with \"hello\" \"he\" -> true\n           Starts with \"he\"\n  \n  Example: starts_with \"hello\" \"lo\" -> false\n           Doesn't start with \"lo\"\n  \n  Example: starts_with \"test_file.av\" \"test\" -> true\n           Check if filename starts with \"test\"\n  \n  Example: filter (\\line starts_with line \"ERROR\") log_lines\n           Find error lines in logs\n  \n  Use case: Filter by prefix, validate format, categorize strings\n  Note: Both arguments accept strings and templates (templates auto-convert to strings)"),
        ("ends_with", "ends_with :: (String|Template) -> (String|Template) -> Bool\n  Check if a string ends with a specific suffix.\n  \n  Arguments:\n    1. String to check\n    2. Suffix to look for\n  \n  Example: ends_with \"hello\" \"lo\" -> true\n           Ends with \"lo\"\n  \n  Example: ends_with \"hello\" \"he\" -> false\n           Doesn't end with \"he\"\n  \n  Example: ends_with \"file.txt\" \".txt\" -> true\n           Check file extension\n  \n  Example: filter (\\file ends_with file \".av\") file_list\n           Get only .av files\n  \n  Use case: Check file extensions, validate suffixes, filter by ending\n  Note: Both arguments accept strings and templates (templates auto-convert to strings)"),
        ("length", "length :: (String|Template|List) -> Int\n  Get the number of characters in a string or items in a list.\n  \n  Arguments:\n    1. String or list to measure\n  \n  Example: length \"hello\" -> 5\n           Count characters\n  \n  Example: length [1, 2, 3] -> 3\n           Count list items\n  \n  Example: length \"\" -> 0\n           Empty string has length 0\n  \n  Example: if length input > 10 then \"too long\" else input\n           Validate input length\n  \n  Use case: Validation, counting, conditional logic\n  Note: Templates are converted to strings before measuring length"),
        ("repeat", "repeat :: (String|Template) -> Int -> String\n  Repeat a string multiple times.\n  \n  Arguments:\n    1. String to repeat\n    2. Number of times to repeat\n  \n  Example: repeat \"x\" 3 -> \"xxx\"\n  Example: repeat \"=\" 50 -> \"==================================================\"\n           Create a line separator\n  \n  Example: repeat \"  \" 4 -> \"        \"\n           Create 4 levels of indentation (8 spaces)\n  \n  Use case: Create separators, padding, decorative elements\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("pad_left", "pad_left :: (String|Template) -> Int -> (String|Template) -> String\n  Add characters to the LEFT of a string until it reaches a target length.\n  \n  Arguments:\n    1. String to pad\n    2. Target total length\n    3. Character to pad with\n  \n  Example: pad_left \"7\" 3 \"0\" -> \"007\"\n           Make \"7\" into 3 digits with leading zeros\n  \n  Example: pad_left \"42\" 5 \"0\" -> \"00042\"\n           Zero-pad to 5 digits\n  \n  Example: pad_left \"hi\" 10 \" \" -> \"        hi\"\n           Right-align with spaces\n  \n  Example: pad_left (to_string num) 4 \"0\"\n           Format number with leading zeros\n  \n  Use case: Format numbers, align text, create fixed-width fields\n  Note: String and pad char accept both strings and templates (templates auto-convert to strings)"),
        ("pad_right", "pad_right :: (String|Template) -> Int -> (String|Template) -> String\n  Add characters to the RIGHT of a string until it reaches a target length.\n  \n  Arguments:\n    1. String to pad\n    2. Target total length\n    3. Character to pad with\n  \n  Example: pad_right \"hi\" 5 \" \" -> \"hi   \"\n           Add spaces to the right\n  \n  Example: pad_right \"Name\" 20 \".\" -> \"Name................\"\n           Create dotted line\n  \n  Example: pad_right \"Alice\" 10 \" \" -> \"Alice     \"\n           Left-align in fixed width\n  \n  Use case: Format tables, align text, create fixed-width columns\n  Note: String and pad char accept both strings and templates (templates auto-convert to strings)"),
        ("indent", "indent :: (String|Template) -> Int -> String\n  Add spaces to the beginning of each line in a string.\n  Useful for formatting code or nested content.\n  \n  Arguments:\n    1. String to indent (can be multi-line)\n    2. Number of spaces to add\n  \n  Example: indent \"code\" 4 -> \"    code\"\n           Add 4 spaces\n  \n  Example: indent \"line1\\nline2\" 2 -> \"  line1\\n  line2\"\n           Indent both lines\n  \n  Example: indent code_block 8\n           Indent code block by 8 spaces\n  \n  Use case: Format nested code, create indented blocks, structure output\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_digit", "is_digit :: (String|Template) -> Bool\n  Check if ALL characters in a string are digits (0-9).\n  \n  Arguments:\n    1. String to check\n  \n  Example: is_digit \"123\" -> true\n           All digits\n  \n  Example: is_digit \"12.3\" -> false\n           Contains a dot (not a digit)\n  \n  Example: is_digit \"\" -> false\n           Empty string is false\n  \n  Example: if is_digit input then to_int input else 0\n           Safely convert to number\n  \n  Use case: Validate numeric input, check before parsing\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_alpha", "is_alpha :: (String|Template) -> Bool\n  Check if ALL characters in a string are letters (a-z, A-Z).\n  \n  Arguments:\n    1. String to check\n  \n  Example: is_alpha \"abc\" -> true\n           All letters\n  \n  Example: is_alpha \"abc123\" -> false\n           Contains numbers\n  \n  Example: is_alpha \"hello world\" -> false\n           Contains space (not a letter)\n  \n  Example: filter is_alpha word_list\n           Keep only words with letters\n  \n  Use case: Validate names, filter text, check input format\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_alphanumeric", "is_alphanumeric :: (String|Template) -> Bool\n  Check if ALL characters in a string are letters or digits.\n  \n  Arguments:\n    1. String to check\n  \n  Example: is_alphanumeric \"abc123\" -> true\n           Letters and numbers\n  \n  Example: is_alphanumeric \"hello\" -> true\n           Just letters (still alphanumeric)\n  \n  Example: is_alphanumeric \"user_123\" -> false\n           Underscore is not alphanumeric\n  \n  Use case: Validate usernames, check identifiers, filter strings\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_whitespace", "is_whitespace :: (String|Template) -> Bool\n  Check if ALL characters in a string are whitespace (spaces, tabs, newlines).\n  \n  Arguments:\n    1. String to check\n  \n  Example: is_whitespace \"  \" -> true\n           All spaces\n  \n  Example: is_whitespace \"\\t\\n\" -> true\n           Tab and newline\n  \n  Example: is_whitespace \"  a\" -> false\n           Contains 'a'\n  \n  Example: filter (\\s not (is_whitespace s)) lines\n           Remove blank lines\n  \n  Use case: Detect empty lines, validate input, clean data\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_uppercase", "is_uppercase :: (String|Template) -> Bool\n  Check if ALL letter characters in a string are uppercase.\n  Non-letter characters are ignored.\n  \n  Arguments:\n    1. String to check\n  \n  Example: is_uppercase \"ABC\" -> true\n           All uppercase letters\n  \n  Example: is_uppercase \"ABC123\" -> true\n           Numbers don't affect result\n  \n  Example: is_uppercase \"Abc\" -> false\n           Contains lowercase 'b' and 'c'\n  \n  Use case: Validate constants, check formatting\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_lowercase", "is_lowercase :: (String|Template) -> Bool\n  Check if ALL letter characters in a string are lowercase.\n  Non-letter characters are ignored.\n  \n  Arguments:\n    1. String to check\n  \n  Example: is_lowercase \"abc\" -> true\n           All lowercase letters\n  \n  Example: is_lowercase \"abc123\" -> true\n           Numbers don't affect result\n  \n  Example: is_lowercase \"Hello\" -> false\n           Contains uppercase 'H'\n  \n  Use case: Validate formatting, check style conventions\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_empty", "is_empty :: (String|Template|List|Dict) -> Bool\n  Check if a string, list, or dictionary has no elements.\n  \n  Arguments:\n    1. String, list, or dict to check\n  \n  Example: is_empty \"\" -> true\n           Empty string\n  \n  Example: is_empty [] -> true\n           Empty list\n  \n  Example: is_empty {} -> true\n           Empty dictionary\n  \n  Example: is_empty \"hello\" -> false\n           Has content\n  \n  Example: if is_empty user_input then \"Please enter something\" else user_input\n           Validate non-empty input\n  \n  Use case: Validate input, check for data, conditional logic\n  Note: Templates are converted to strings before checking"),

        // List Operations
        ("map", "map :: (a -> b) -> [a] -> [b]\n  Transform each item in a list by applying a function.\n  \n  Arguments:\n    1. Function to apply to each element\n    2. List to transform\n  \n  Example: map (\\x x * 2) [1, 2, 3] -> [2, 4, 6]\n           Double each number in the list\n  \n  Example: map upper [\"hello\", \"world\"] -> [\"HELLO\", \"WORLD\"]\n           Convert each string to uppercase\n  \n  Tip: Use with partially applied functions:\n       let double = map (\\x x * 2) in\n       double [1, 2, 3]"),
        ("filter", "filter :: (a -> Bool) -> [a] -> [a]\n  Keep only items that match a condition (predicate returns true).\n  \n  Arguments:\n    1. Predicate function (returns true/false)\n    2. List to filter\n  \n  Example: filter (\\x x > 2) [1, 2, 3, 4] -> [3, 4]\n           Keep only numbers greater than 2\n  \n  Example: filter (\\s starts_with s \"test\") [\"test1.av\", \"demo.av\", \"test2.av\"]\n           -> [\"test1.av\", \"test2.av\"]\n           Keep only files starting with \"test\"\n  \n  Tip: Combine with map for powerful data pipelines:\n       [1, 2, 3, 4, 5] -> filter (\\x x > 2) -> map (\\x x * 2)"),
        ("fold", "fold :: (acc -> a -> acc) -> acc -> [a] -> acc\n  Reduce a list to a single value by combining elements.\n  Also known as 'reduce' in other languages.\n  \n  Arguments:\n    1. Combiner function (takes accumulator and current item, returns new accumulator)\n    2. Initial accumulator value\n    3. List to process\n  \n  Example: fold (\\acc \\x acc + x) 0 [1, 2, 3] -> 6\n           Sum all numbers (start with 0, add each number)\n  \n  Example: fold (\\acc \\x concat acc x) \"\" [\"Hello\", \" \", \"World\"] -> \"Hello World\"\n           Concatenate all strings\n  \n  Example: fold (\\acc \\x if x > acc then x else acc) 0 [3, 7, 2, 9, 1] -> 9\n           Find maximum value\n  \n  Tip: First parameter is always the accumulator, second is the current item"),

        // Parallel List Operations
        ("pmap", "pmap :: (a -> b) -> [a] -> [b]\n  Parallel map - transform each item using multiple threads.\n  Same as 'map' but processes elements concurrently for better performance on large lists.\n  \n  Arguments:\n    1. Function to apply to each element\n    2. List to transform\n  \n  Example: pmap (\\x x * 2) [1, 2, 3, 4, 5] -> [2, 4, 6, 8, 10]\n           Double each number in parallel\n  \n  Example: pmap expensive_compute large_dataset\n           Process data concurrently\n  \n  When to use:\n    - Large lists (1000+ elements)\n    - CPU-intensive operations per element\n    - Independent computations (no shared state)\n  \n  When NOT to use:\n    - Small lists (overhead exceeds benefit)\n    - Simple operations (x + 1)\n    - Operations with side effects\n  \n  Note: Results are in the same order as input\n  Note: Uses all available CPU cores via Rayon"),
        ("pfilter", "pfilter :: (a -> Bool) -> [a] -> [a]\n  Parallel filter - keep matching items using multiple threads.\n  Same as 'filter' but evaluates predicates concurrently.\n  \n  Arguments:\n    1. Predicate function (returns true/false)\n    2. List to filter\n  \n  Example: pfilter (\\x x > 3) [1, 2, 3, 4, 5, 6] -> [4, 5, 6]\n           Keep numbers > 3 in parallel\n  \n  Example: pfilter expensive_check large_list\n           Filter with costly predicate\n  \n  When to use:\n    - Large lists with expensive predicates\n    - CPU-intensive condition checking\n  \n  When NOT to use:\n    - Small lists\n    - Simple predicates (x > 0)\n  \n  Note: Results preserve original order\n  Note: Uses all available CPU cores via Rayon"),
        ("pfold", "pfold :: (acc -> a -> acc) -> acc -> [a] -> acc\n  Parallel fold - reduce list using multiple threads.\n  Divides list into chunks, folds each chunk in parallel, then combines results.\n  \n  Arguments:\n    1. Combiner function (must be associative for correct results)\n    2. Initial accumulator value (identity element)\n    3. List to process\n  \n  Example: pfold (\\a \\b a + b) 0 [1, 2, 3, 4, 5] -> 15\n           Sum all numbers in parallel\n  \n  Example: pfold (\\a \\b a * b) 1 [1, 2, 3, 4, 5] -> 120\n           Product in parallel\n  \n  IMPORTANT: The combiner function must be ASSOCIATIVE:\n    (a op b) op c == a op (b op c)\n  \n  Good operations (associative):\n    - Addition: a + b\n    - Multiplication: a * b\n    - Max/Min: max a b\n    - String concat (for collecting)\n  \n  Bad operations (non-associative):\n    - Subtraction: a - b\n    - Division: a / b\n  \n  Note: Uses all available CPU cores via Rayon"),

        ("flatmap", "flatmap :: (a -> [b]) -> [a] -> [b]\n  Apply a function to each item (like map), then flatten the results one level.\n  Useful when your function returns a list for each input.\n  \n  Arguments:\n    1. Function that takes an item and returns a list\n    2. List to process\n  \n  Example: flatmap (\\x [x, x]) [1, 2] -> [1, 1, 2, 2]\n           Duplicate each number\n  \n  Example: flatmap (\\s split s \",\") [\"a,b\", \"c,d\"] -> [\"a\", \"b\", \"c\", \"d\"]\n           Split each string then combine all results\n  \n  Example: flatmap (\\n range 1 n) [2, 3] -> [1, 2, 1, 2, 3]\n           Generate ranges for each number, then flatten\n  \n  Use case: Process items that produce multiple results\n  Tip: Same as: map f list -> flatten"),
        ("flatten", "flatten :: [[a]] -> [a]\n  Flatten a nested list by one level (removes one layer of brackets).\n  \n  Arguments:\n    1. List of lists to flatten\n  \n  Example: flatten [[1, 2], [3]] -> [1, 2, 3]\n           Combine all sublists\n  \n  Example: flatten [[\"a\", \"b\"], [\"c\"], [\"d\", \"e\"]] -> [\"a\", \"b\", \"c\", \"d\", \"e\"]\n           Merge multiple string lists\n  \n  Example: flatten [] -> []\n           Empty list stays empty\n  \n  Use case: Combine results from multiple operations, merge grouped data\n  Note: Only flattens one level (not recursive)"),
        ("zip", "zip :: [a] -> [b] -> [(a, b)]\n  Combine two lists into pairs (tuples).\n  Pairs up items at the same position.\n  \n  Arguments:\n    1. First list\n    2. Second list\n  \n  Example: zip [1, 2] [\"a\", \"b\"] -> [(1, \"a\"), (2, \"b\")]\n           Pair numbers with letters\n  \n  Example: zip [\"Alice\", \"Bob\"] [30, 25] -> [(\"Alice\", 30), (\"Bob\", 25)]\n           Pair names with ages\n  \n  Example: zip [1, 2, 3] [\"a\", \"b\"] -> [(1, \"a\"), (2, \"b\")]\n           Stops when shortest list ends\n  \n  Use case: Combine related data, create key-value pairs\n  Tip: Opposite of unzip"),
        ("unzip", "unzip :: [(a, b)] -> ([a], [b])\n  Split a list of pairs (tuples) into two separate lists.\n  \n  Arguments:\n    1. List of pairs to split\n  \n  Example: unzip [(1, \"a\"), (2, \"b\")] -> ([1, 2], [\"a\", \"b\"])\n           Separate numbers from letters\n  \n  Example: unzip [(\"Alice\", 30), (\"Bob\", 25)] -> ([\"Alice\", \"Bob\"], [30, 25])\n           Separate names from ages\n  \n  Example: unzip [] -> ([], [])\n           Empty list produces two empty lists\n  \n  Use case: Separate paired data, split coordinates\n  Tip: Opposite of zip"),
        ("take", "take :: Int -> [a] -> [a]\n  Take the first n items from a list.\n  \n  Arguments:\n    1. Number of items to take\n    2. List to take from\n  \n  Example: take 2 [1, 2, 3, 4] -> [1, 2]\n           Take first 2 items\n  \n  Example: take 0 [1, 2, 3] -> []\n           Take 0 items gives empty list\n  \n  Example: take 10 [1, 2, 3] -> [1, 2, 3]\n           Taking more than list length returns entire list\n  \n  Example: take 3 (sort scores) \n           Get top 3 sorted items\n  \n  Use case: Pagination, top-N queries, limiting results\n  Tip: Combine with drop for pagination: drop (page * size) -> take size"),
        ("drop", "drop :: Int -> [a] -> [a]\n  Skip (drop) the first n items from a list.\n  \n  Arguments:\n    1. Number of items to skip\n    2. List to drop from\n  \n  Example: drop 2 [1, 2, 3, 4] -> [3, 4]\n           Skip first 2 items\n  \n  Example: drop 0 [1, 2, 3] -> [1, 2, 3]\n           Drop 0 items returns entire list\n  \n  Example: drop 10 [1, 2, 3] -> []\n           Dropping more than list length returns empty list\n  \n  Example: drop 1 lines\n           Skip header line from CSV data\n  \n  Use case: Skip headers, pagination, remove prefix elements\n  Tip: Combine with take for pagination: drop (page * size) -> take size"),
        ("slice", "slice :: (String|[a]) -> Int -> Int -> (String|[a])\n  Extract a portion of a string or list from start index to end index.\n  Start is inclusive, end is exclusive (like [start, end)).\n  \n  Arguments:\n    1. String or list to slice\n    2. Start index (included, 0-based)\n    3. End index (excluded, 0-based)\n  \n  Example: slice \"hello\" 1 4 -> \"ell\"\n           Get characters at positions 1, 2, 3 (not 4)\n  \n  Example: slice \"hello\" 0 2 -> \"he\"\n           Get first 2 characters\n  \n  Example: slice [1, 2, 3, 4, 5] 1 4 -> [2, 3, 4]\n           Get list items from index 1 to 3\n  \n  Example: slice \"abcdef\" 2 10 -> \"cdef\"\n           End beyond length is okay\n  \n  Use case: Extract substrings, get list portions, pagination"),
        ("char_at", "char_at :: String -> Int -> String | None\n  Get the character at a specific position (index) in a string.\n  \n  Arguments:\n    1. String to index\n    2. Position (0-based: 0 is first character)\n  \n  Example: char_at \"hello\" 0 -> \"h\"\n           First character\n  \n  Example: char_at \"hello\" 2 -> \"l\"\n           Third character (index 2)\n  \n  Example: char_at \"hello\" 10 -> None\n           Index out of bounds returns None\n  \n  Use case: Access specific character, validate positions\n  Tip: Check with is_none if unsure about string length"),
        ("chars", "chars :: String -> [String]\n  Convert a string into a list of individual characters.\n  Each character becomes a separate string in the list.\n  \n  Arguments:\n    1. String to split into characters\n  \n  Example: chars \"hello\" -> [\"h\", \"e\", \"l\", \"l\", \"o\"]\n           Split into individual letters\n  \n  Example: chars \"ab\" -> [\"a\", \"b\"]\n           Two characters\n  \n  Example: chars \"\" -> []\n           Empty string gives empty list\n  \n  Example: chars \"hello\" -> map upper -> join \"\"\n           Process each character then rejoin\n  \n  Use case: Character-by-character processing, counting specific chars\n  Tip: Use with map to transform each character"),
        ("split_at", "split_at :: Int -> [a] -> ([a], [a])\n  Split a list into two parts at a specific index.\n  \n  Arguments:\n    1. Index to split at\n    2. List to split\n  \n  Example: split_at 2 [1, 2, 3, 4] -> ([1, 2], [3, 4])\n           Split after 2nd item\n  \n  Example: split_at 0 [1, 2, 3] -> ([], [1, 2, 3])\n           Split at start gives empty first list\n  \n  Example: split_at 10 [1, 2] -> ([1, 2], [])\n           Index beyond length gives all in first list\n  \n  Use case: Separate header/body, divide data, pagination"),
        ("partition", "partition :: (a -> Bool) -> [a] -> ([a], [a])\n  Split a list into two groups: items that match condition and items that don't.\n  \n  Arguments:\n    1. Condition function (returns true/false)\n    2. List to partition\n  \n  Example: partition (\\x x > 2) [1, 2, 3, 4] -> ([3, 4], [1, 2])\n           First list: items > 2, Second list: items <= 2\n  \n  Example: partition (\\s starts_with s \"test\") [\"test.av\", \"demo.av\", \"test2.av\"]\n           -> ([\"test.av\", \"test2.av\"], [\"demo.av\"])\n           Separate test files from other files\n  \n  Example: partition is_empty [\"a\", \"\", \"b\", \"\"] -> ([\"\", \"\"], [\"a\", \"b\"])\n           Separate empty from non-empty strings\n  \n  Use case: Classify data, separate valid/invalid items\n  Tip: Like filter but keeps both matching and non-matching items"),
        ("reverse", "reverse :: [a] -> [a]\n  Reverse the order of items in a list.\n  First becomes last, last becomes first.\n  \n  Arguments:\n    1. List to reverse\n  \n  Example: reverse [1, 2, 3] -> [3, 2, 1]\n           Flip number order\n  \n  Example: reverse [\"a\", \"b\", \"c\"] -> [\"c\", \"b\", \"a\"]\n           Flip string order\n  \n  Example: reverse [] -> []\n           Empty list stays empty\n  \n  Use case: Process in reverse order, undo operations, rotate data"),
        ("sort", "sort :: [a] -> [a]\n  Sort a list in ascending order.\n  Numbers sorted numerically, strings alphabetically.\n  \n  Arguments:\n    1. List to sort\n  \n  Example: sort [3, 1, 2] -> [1, 2, 3]\n           Numbers: smallest to largest\n  \n  Example: sort [\"c\", \"a\", \"b\"] -> [\"a\", \"b\", \"c\"]\n           Strings: alphabetically\n  \n  Use case: Organize data, find rankings, alphabetize\n  Tip: For custom sorting, use sort_by"),
        ("sort_by", "sort_by :: (a -> b) -> [a] -> [a]\n  Sort a list using a custom key function.\n  \n  Arguments:\n    1. Key function (extracts value to sort by)\n    2. List to sort\n  \n  Example: sort_by (\\x x.age) users\n           Sort users by age field\n  \n  Example: sort_by lower [\"Bob\", \"alice\", \"Charlie\"] -> [\"alice\", \"Bob\", \"Charlie\"]\n           Case-insensitive sort\n  \n  Example: sort_by length [\"cc\", \"a\", \"bbb\"] -> [\"a\", \"cc\", \"bbb\"]\n           Sort strings by length\n  \n  Use case: Sort objects by property, custom ordering"),
        ("unique", "unique :: [a] -> [a]\n  Remove duplicate elements from a list, keeping only first occurrence.\n  \n  Arguments:\n    1. List to deduplicate\n  \n  Example: unique [1, 2, 1, 3, 2] -> [1, 2, 3]\n           Remove duplicate numbers\n  \n  Example: unique [\"a\", \"b\", \"a\"] -> [\"a\", \"b\"]\n           Remove duplicate strings\n  \n  Example: unique [] -> []\n           Empty list stays empty\n  \n  Use case: Remove duplicates, get distinct values\n  Tip: Order is preserved (first occurrence kept)"),
        ("range", "range :: Int -> Int -> [Int]\n  Generate a list of consecutive integers from start to end (both inclusive).\n  \n  Arguments:\n    1. Start number (included)\n    2. End number (included)\n  \n  Example: range 1 5 -> [1, 2, 3, 4, 5]\n           Numbers 1 through 5\n  \n  Example: range 0 2 -> [0, 1, 2]\n           Start from 0\n  \n  Example: range 10 10 -> [10]\n           Single number\n  \n  Example: range 1 100 -> map (\\x x * 2)\n           Create range then transform\n  \n  Use case: Generate sequences, loops, test data\n  Note: Returns empty list [] if start > end"),
        ("enumerate", "enumerate :: [a] -> [[Int, a]]\n  Add an index to each element, returning a list of [index, value] pairs.\n  Indices start at 0.\n  \n  Arguments:\n    1. List to enumerate\n  \n  Example: enumerate [\"a\", \"b\", \"c\"] -> [[0, \"a\"], [1, \"b\"], [2, \"c\"]]\n           Add indices to strings\n  \n  Example: enumerate [10, 20, 30] -> [[0, 10], [1, 20], [2, 30]]\n           Add indices to numbers\n  \n  Example: enumerate items -> filter (\\pair (head pair) % 2 == 0)\n           Keep only even-indexed items\n  \n  Example: enumerate lines -> map (\\pair concat (to_string (head pair)) \": \" (last pair))\n           Add line numbers to text\n  \n  Use case: Number items, track positions, create indexed data\n  Tip: Access index with head and value with last on each pair"),
        ("sum", "sum :: [Number] -> Number\n  Calculate the sum of all numbers in a list.\n  Returns 0 for empty lists.\n  \n  Arguments:\n    1. List of numbers to sum\n  \n  Example: sum [1, 2, 3, 4, 5] -> 15\n           1+2+3+4+5 = 15\n  \n  Example: sum [] -> 0\n           Empty list sums to 0 (identity element)\n  \n  Example: sum [1.5, 2.5, 3.0] -> 7.0\n           Works with floats\n  \n  Example: sum (map length strings)\n           Sum the lengths of all strings\n  \n  Example: let total = sum prices in\n           Calculate total price\n  \n  Use case: Totals, aggregations, calculating averages (sum/length)\n  Note: Returns Int if all ints, Float if any floats"),
        ("product", "product :: [Number] -> Number\n  Multiply all numbers in a list together.\n  Returns 1 for empty lists.\n  \n  Arguments:\n    1. List of numbers to multiply\n  \n  Example: product [1, 2, 3, 4] -> 24\n           1*2*3*4 = 24\n  \n  Example: product [] -> 1\n           Empty list products to 1 (identity element)\n  \n  Example: product [2, 2, 2] -> 8\n           2^3 = 8\n  \n  Example: product [0.5, 2, 4] -> 4.0\n           Works with floats\n  \n  Example: let factorial = \\n product (range 1 n)\n           Calculate factorial of n\n  \n  Use case: Factorials, compound calculations, probability\n  Note: Returns Int if all ints, Float if any floats"),
        ("min", "min :: [a] -> a | None\n  Find the minimum (smallest) value in a list.\n  Works with numbers (including mixed int/float) and strings.\n  Returns None for empty lists.\n  \n  Arguments:\n    1. List to find minimum in\n  \n  Example: min [3, 1, 4, 1, 5] -> 1\n           Smallest number\n  \n  Example: min [3.14, 2.71, 1.41] -> 1.41\n           Works with floats\n  \n  Example: min [\"zebra\", \"apple\", \"banana\"] -> \"apple\"\n           Alphabetically first string\n  \n  Example: min [] -> None\n           Empty list returns None\n  \n  Example: let lowest_price = min prices in\n           if is_none lowest_price then 0 else lowest_price\n  \n  Use case: Find smallest value, rankings, bounds checking\n  Tip: Check for None before using result from potentially empty lists"),
        ("max", "max :: [a] -> a | None\n  Find the maximum (largest) value in a list.\n  Works with numbers (including mixed int/float) and strings.\n  Returns None for empty lists.\n  \n  Arguments:\n    1. List to find maximum in\n  \n  Example: max [3, 1, 4, 1, 5] -> 5\n           Largest number\n  \n  Example: max [3.14, 2.71, 1.41] -> 3.14\n           Works with floats\n  \n  Example: max [\"zebra\", \"apple\", \"banana\"] -> \"zebra\"\n           Alphabetically last string\n  \n  Example: max [] -> None\n           Empty list returns None\n  \n  Example: let highest_score = max scores in\n           if is_none highest_score then 0 else highest_score\n  \n  Use case: Find largest value, rankings, bounds checking\n  Tip: Check for None before using result from potentially empty lists"),
        ("all", "all :: (a -> Bool) -> [a] -> Bool\n  Check if ALL elements in a list satisfy a predicate.\n  Returns true only if every element passes the test.\n  \n  Arguments:\n    1. Predicate function (returns true/false)\n    2. List to check\n  \n  Example: all (\\x x > 0) [1, 2, 3] -> true\n           All numbers are positive\n  \n  Example: all (\\x x > 0) [1, -2, 3] -> false\n           -2 fails the test\n  \n  Example: all (\\x x > 0) [] -> true\n           Empty list returns true (vacuous truth)\n  \n  Example: all is_string [\"a\", \"b\", \"c\"] -> true\n           All are strings\n  \n  Example: if all (\\f exists f) files then \"all exist\" else \"some missing\"\n           Check if all files exist\n  \n  Use case: Validation, preconditions, ensuring data quality\n  Note: Empty list returns true (vacuous truth - there's no element that fails)"),
        ("any", "any :: (a -> Bool) -> [a] -> Bool\n  Check if ANY element in a list satisfies a predicate.\n  Returns true if at least one element passes the test.\n  \n  Arguments:\n    1. Predicate function (returns true/false)\n    2. List to check\n  \n  Example: any (\\x x < 0) [1, 2, -3] -> true\n           At least one negative number\n  \n  Example: any (\\x x < 0) [1, 2, 3] -> false\n           No negative numbers\n  \n  Example: any (\\x x < 0) [] -> false\n           Empty list returns false\n  \n  Example: any (\\s contains s \"error\") log_lines -> true\n           Check if any log line contains \"error\"\n  \n  Example: if any is_none values then \"has nulls\" else \"all valid\"\n           Check for None values in data\n  \n  Use case: Search, validation, early detection\n  Note: Empty list returns false (no element can satisfy the predicate)"),
        ("count", "count :: (a -> Bool) -> [a] -> Int\n  Count how many elements in a list satisfy a predicate.\n  \n  Arguments:\n    1. Predicate function (returns true/false)\n    2. List to count in\n  \n  Example: count (\\x x > 5) [1, 6, 3, 8, 5] -> 2\n           Two numbers greater than 5 (6 and 8)\n  \n  Example: count (\\x x == \"a\") [\"a\", \"b\", \"a\", \"c\", \"a\"] -> 3\n           Three occurrences of \"a\"\n  \n  Example: count is_empty [\"\", \"a\", \"\", \"b\"] -> 2\n           Two empty strings\n  \n  Example: count (\\line starts_with line \"ERROR\") log_lines\n           Count error lines in logs\n  \n  Example: let passing = count (\\score score >= 60) scores in\n           Count passing grades\n  \n  Use case: Statistics, counting occurrences, tallying\n  Tip: count f list == length (filter f list), but more efficient"),
        ("head", "head :: [a] -> a | None\n  Get the first element of a list, or None if the list is empty.\n  \n  Arguments:\n    1. List to get first element from\n  \n  Example: head [1, 2, 3] -> 1\n           First element\n  \n  Example: head [\"apple\", \"banana\"] -> \"apple\"\n           First string\n  \n  Example: head [] -> None\n           Empty list returns None\n  \n  Example: let first = head items in\n           if is_none first then \"no items\" else first\n           Safe access pattern\n  \n  Example: head (sort scores)\n           Get lowest score (after sorting ascending)\n  \n  Use case: Access first element, peek at data, get default\n  Tip: Always check for None when list might be empty"),
        ("last", "last :: [a] -> a | None\n  Get the last element of a list, or None if the list is empty.\n  \n  Arguments:\n    1. List to get last element from\n  \n  Example: last [1, 2, 3] -> 3\n           Last element\n  \n  Example: last [\"apple\", \"banana\"] -> \"banana\"\n           Last string\n  \n  Example: last [] -> None\n           Empty list returns None\n  \n  Example: last (sort scores)\n           Get highest score (after sorting ascending)\n  \n  Example: let extension = last (split filename \".\") in\n           Get file extension\n  \n  Use case: Access last element, get file extension, most recent item\n  Tip: Always check for None when list might be empty"),
        ("nth", "nth :: Int -> [a] -> a | None\n  Get the element at a specific index (0-based), or None if out of bounds.\n  \n  Arguments:\n    1. Index (0 = first element)\n    2. List to access\n  \n  Example: nth 0 [1, 2, 3] -> 1\n           First element (index 0)\n  \n  Example: nth 2 [1, 2, 3] -> 3\n           Third element (index 2)\n  \n  Example: nth 5 [1, 2, 3] -> None\n           Index out of bounds returns None\n  \n  Example: nth -1 [1, 2, 3] -> None\n           Negative index returns None\n  \n  Example: let second_word = nth 1 (words sentence) in\n           Get second word from sentence\n  \n  Use case: Random access, specific position lookup\n  Tip: Always check for None when index might be out of bounds"),
        ("tail", "tail :: [a] -> [a]\n  Get all elements except the first one.\n  Returns empty list if input is empty or has one element.\n  \n  Arguments:\n    1. List to get tail of\n  \n  Example: tail [1, 2, 3] -> [2, 3]\n           Everything except first\n  \n  Example: tail [\"a\"] -> []\n           Single element list gives empty tail\n  \n  Example: tail [] -> []\n           Empty list gives empty tail\n  \n  Example: tail (lines file_content)\n           Skip header line from file\n  \n  Example: let rest = tail args in\n           Get all arguments except the first\n  \n  Use case: Skip first element, process rest of list, recursive patterns\n  Tip: tail is like drop 1"),

        // New Python-like list functions
        ("choice", "choice :: [a] -> a\n  Select a random element from a list.\n  \n  Arguments:\n    1. Non-empty list to select from\n  \n  Example: choice [1, 2, 3, 4, 5] -> 3\n           Random element (varies each call)\n  \n  Example: choice [\"red\", \"green\", \"blue\"] -> \"green\"\n           Random color\n  \n  Example: let winner = choice participants in ...\n           Pick a random winner\n  \n  Note: Errors on empty list (use sample 1 if you need None)"),
        ("shuffle", "shuffle :: [a] -> [a]\n  Randomly reorder all elements in a list.\n  \n  Arguments:\n    1. List to shuffle\n  \n  Example: shuffle [1, 2, 3, 4, 5] -> [3, 1, 5, 2, 4]\n           Random order (varies each call)\n  \n  Example: shuffle [\"a\", \"b\", \"c\"] -> [\"c\", \"a\", \"b\"]\n           Shuffle strings\n  \n  Example: let deck = shuffle [1..52] in take 5 deck\n           Shuffle deck and draw 5 cards\n  \n  Note: Empty list returns empty list"),
        ("sample", "sample :: Int -> [a] -> [a]\n  Select n unique random elements from a list (no duplicates).\n  \n  Arguments:\n    1. Number of elements to sample\n    2. List to sample from\n  \n  Example: sample 3 [1, 2, 3, 4, 5] -> [2, 5, 1]\n           3 unique random elements\n  \n  Example: sample 2 [\"a\", \"b\", \"c\", \"d\"] -> [\"d\", \"a\"]\n           2 random strings\n  \n  Example: sample 0 [1, 2, 3] -> []\n           Zero samples gives empty list\n  \n  Note: Errors if n > list length (can't sample more than available)"),
        ("find", "find :: (a -> Bool) -> [a] -> a | None\n  Find the first element that matches a predicate.\n  \n  Arguments:\n    1. Predicate function (returns true/false)\n    2. List to search\n  \n  Example: find (\\x x > 5) [1, 3, 7, 2, 9] -> 7\n           First number greater than 5\n  \n  Example: find (\\x x > 100) [1, 2, 3] -> None\n           No match returns None\n  \n  Example: find (\\s starts_with s \"test\") files -> None | String\n           Find first test file\n  \n  Use case: Search for first match, early termination\n  Tip: Unlike filter, stops at first match"),
        ("find_index", "find_index :: (a -> Bool) -> [a] -> Int | None\n  Find the index of the first element that matches a predicate.\n  \n  Arguments:\n    1. Predicate function (returns true/false)\n    2. List to search\n  \n  Example: find_index (\\x x > 5) [1, 3, 7, 2, 9] -> 2\n           Index of 7 (first > 5)\n  \n  Example: find_index (\\x x > 100) [1, 2, 3] -> None\n           No match returns None\n  \n  Example: find_index (\\x x == \"target\") items\n           Find position of target\n  \n  Use case: Get position instead of value"),
        ("group_by", "group_by :: (a -> k) -> [a] -> Dict[k, [a]]\n  Group list elements by the result of a key function.\n  \n  Arguments:\n    1. Key function (extracts grouping key from each element)\n    2. List to group\n  \n  Example: group_by (\\x x % 2) [1, 2, 3, 4, 5, 6]\n           -> {0: [2, 4, 6], 1: [1, 3, 5]}\n           Group by even/odd\n  \n  Example: group_by length [\"a\", \"bb\", \"c\", \"ddd\", \"ee\"]\n           -> {1: [\"a\", \"c\"], 2: [\"bb\", \"ee\"], 3: [\"ddd\"]}\n           Group strings by length\n  \n  Example: group_by (\\p p.category) products\n           Group products by category field\n  \n  Use case: Categorize data, create buckets, aggregate by key"),
        ("zip_with", "zip_with :: (a -> b -> c) -> [a] -> [b] -> [c]\n  Combine two lists using a function (generalized zip).\n  \n  Arguments:\n    1. Combining function (takes element from each list)\n    2. First list\n    3. Second list\n  \n  Example: zip_with (\\a \\b a + b) [1, 2, 3] [10, 20, 30] -> [11, 22, 33]\n           Add corresponding elements\n  \n  Example: zip_with (\\a \\b a * b) [2, 3, 4] [5, 6, 7] -> [10, 18, 28]\n           Multiply corresponding elements\n  \n  Example: zip_with (\\x \\y concat x y) [\"a\", \"b\"] [\"1\", \"2\"] -> [\"a1\", \"b2\"]\n           Concatenate corresponding strings\n  \n  Note: Stops at shorter list length\n  Tip: More flexible than zip - directly applies function"),
        ("intersperse", "intersperse :: a -> [a] -> [a]\n  Insert a separator value between each element of a list.\n  \n  Arguments:\n    1. Separator value to insert\n    2. List to process\n  \n  Example: intersperse 0 [1, 2, 3] -> [1, 0, 2, 0, 3]\n           Insert 0 between numbers\n  \n  Example: intersperse \", \" [\"a\", \"b\", \"c\"] -> [\"a\", \", \", \"b\", \", \", \"c\"]\n           Insert separator strings\n  \n  Example: intersperse \"--\" items -> join \"\" ...\n           Build separated string\n  \n  Note: Empty list returns empty, single element returns unchanged\n  Tip: Different from join - returns list, doesn't concatenate"),

        // Regex Functions
        ("regex_match", "regex_match :: String -> String -> Bool\n  Check if text matches a regular expression pattern.\n  \n  Arguments:\n    1. Regex pattern\n    2. Text to check\n  \n  Example: regex_match \"^\\\\d+$\" \"123\" -> true\n           String is all digits\n  \n  Example: regex_match \"^\\\\d+$\" \"12a\" -> false\n           Contains non-digit\n  \n  Example: regex_match \"hello\" \"say hello world\" -> true\n           Pattern found anywhere in text\n  \n  Example: regex_match \"^hello\" \"hello world\" -> true\n           Starts with 'hello'\n  \n  Example: filter (\\s regex_match \"@.*\\\\.com$\" s) emails\n           Filter emails ending in .com\n  \n  Common patterns:\n    ^       Start of string\n    $       End of string\n    \\\\d     Any digit (0-9)\n    \\\\w     Word character (letter, digit, _)\n    \\\\s     Whitespace\n    .       Any character\n    *       Zero or more\n    +       One or more\n    ?       Zero or one\n  \n  Use case: Validation, filtering, pattern matching"),
        ("regex_replace", "regex_replace :: String -> String -> String -> String\n  Replace all matches of a regex pattern with a replacement string.\n  \n  Arguments:\n    1. Regex pattern to find\n    2. Replacement string\n    3. Text to process\n  \n  Example: regex_replace \"\\\\d\" \"#\" \"a1b2c3\" -> \"a#b#c#\"\n           Replace all digits with #\n  \n  Example: regex_replace \"\\\\s+\" \" \" \"hello    world\" -> \"hello world\"\n           Collapse multiple spaces\n  \n  Example: regex_replace \"[aeiou]\" \"*\" \"hello\" -> \"h*ll*\"\n           Replace vowels\n  \n  Example: regex_replace \"^\\\\s+|\\\\s+$\" \"\" text\n           Trim whitespace (regex trim)\n  \n  Capture groups:\n    Use $1, $2 etc. to reference captured groups\n    Example: regex_replace \"(\\\\w+)@(\\\\w+)\" \"$1 AT $2\" \"user@domain\"\n             -> \"user AT domain\"\n  \n  Use case: Text cleanup, sanitization, formatting, find-and-replace"),
        ("regex_split", "regex_split :: String -> String -> [String]\n  Split text by a regex pattern (like split but with regex).\n  \n  Arguments:\n    1. Regex pattern (delimiter)\n    2. Text to split\n  \n  Example: regex_split \"\\\\s+\" \"a b  c\" -> [\"a\", \"b\", \"c\"]\n           Split by any whitespace\n  \n  Example: regex_split \"[,;]\" \"a,b;c\" -> [\"a\", \"b\", \"c\"]\n           Split by comma or semicolon\n  \n  Example: regex_split \"\\\\d+\" \"a1b22c333d\" -> [\"a\", \"b\", \"c\", \"d\"]\n           Split by numbers\n  \n  Example: regex_split \"\\\\s*,\\\\s*\" \"a, b , c\" -> [\"a\", \"b\", \"c\"]\n           Split by comma with optional whitespace\n  \n  Use case: Parse complex formats, flexible tokenization, data extraction"),
        ("scan", "scan :: String -> String -> [String|[String]]\n  Find all matches of a regex pattern in text.\n  \n  Arguments:\n    1. Regex pattern\n    2. Text to search\n  \n  Without capture groups - returns list of matches:\n    Example: scan \"\\\\d+\" \"a12b34c56\" -> [\"12\", \"34\", \"56\"]\n             Find all numbers\n  \n    Example: scan \"\\\\w+@\\\\w+\\\\.\\\\w+\" text -> all email addresses\n             Extract email addresses\n  \n  With capture groups - returns list of lists:\n    Example: scan \"(\\\\w+)=(\\\\d+)\" \"x=1,y=2\" -> [[\"x\", \"1\"], [\"y\", \"2\"]]\n             Capture key-value pairs\n  \n    Example: scan \"<(\\\\w+)>\" \"<div><span>\" -> [[\"div\"], [\"span\"]]\n             Extract tag names\n  \n  Use case: Data extraction, log parsing, pattern finding\n  Tip: Use without groups for simple matching, with groups to extract parts"),

        // Dict Operations
        ("get", "get :: (Dict|[[String, a]]) -> String -> a | None\n  Get a value from a dictionary by key.\n  Returns None if the key doesn't exist.\n  \n  Arguments:\n    1. Dictionary or list of pairs\n    2. Key to look up\n  \n  Example: get {name: \"Alice\", age: 30} \"name\" -> \"Alice\"\n           Get value by key\n  \n  Example: get {a: 1, b: 2} \"c\" -> None\n           Missing key returns None\n  \n  Example: get [[\"a\", 1], [\"b\", 2]] \"a\" -> 1\n           Works with list of pairs too\n  \n  Example: let value = get config \"port\" in\n           if is_none value then 8080 else value\n           Get with default fallback\n  \n  Example: get (get nested \"outer\") \"inner\"\n           Nested dictionary access\n  \n  Use case: Dictionary lookup, config access, data extraction\n  Tip: For dict literals, you can also use dot notation: config.port"),
        ("set", "set :: (Dict|[[String, a]]) -> String -> a -> (Dict|[[String, a]])\n  Create a new dictionary with a key set to a value.\n  If key exists, it's updated; if not, it's added.\n  \n  Arguments:\n    1. Dictionary or list of pairs\n    2. Key to set\n    3. Value to assign\n  \n  Example: set {a: 1} \"b\" 2 -> {a: 1, b: 2}\n           Add new key\n  \n  Example: set {a: 1} \"a\" 99 -> {a: 99}\n           Update existing key\n  \n  Example: set {} \"first\" \"value\" -> {first: \"value\"}\n           Add to empty dict\n  \n  Example: set [[\"a\", 1]] \"b\" 2 -> [[\"a\", 1], [\"b\", 2]]\n           Works with list of pairs\n  \n  Example: config -> set \"port\" 3000 -> set \"host\" \"0.0.0.0\"\n           Chain multiple updates\n  \n  Use case: Building dicts, updating config, immutable updates\n  Note: Returns new dict (original unchanged - immutable)"),
        ("keys", "keys :: (Dict|[[String, a]]) -> [String]\n  Get all keys from a dictionary as a list.\n  \n  Arguments:\n    1. Dictionary or list of pairs\n  \n  Example: keys {name: \"Alice\", age: 30} -> [\"name\", \"age\"]\n           Get all keys\n  \n  Example: keys {} -> []\n           Empty dict has no keys\n  \n  Example: keys [[\"a\", 1], [\"b\", 2]] -> [\"a\", \"b\"]\n           Works with list of pairs\n  \n  Example: length (keys config)\n           Count number of config options\n  \n  Example: keys env_vars -> filter (\\k starts_with k \"MY_APP\")\n           Find all keys with prefix\n  \n  Use case: Iterate over dict, check available keys, inspect structure\n  Note: Order may not be preserved in all cases"),
        ("values", "values :: (Dict|[[String, a]]) -> [a]\n  Get all values from a dictionary as a list.\n  \n  Arguments:\n    1. Dictionary or list of pairs\n  \n  Example: values {a: 1, b: 2, c: 3} -> [1, 2, 3]\n           Get all values\n  \n  Example: values {} -> []\n           Empty dict has no values\n  \n  Example: values [[\"x\", 10], [\"y\", 20]] -> [10, 20]\n           Works with list of pairs\n  \n  Example: sum (values scores)\n           Sum all values in a dict\n  \n  Example: any is_none (values record)\n           Check if any value is None\n  \n  Use case: Aggregate values, validate data, extract for processing\n  Note: Order corresponds to keys order"),
        ("has_key", "has_key :: (Dict|[[String, a]]) -> String -> Bool\n  Check if a key exists in a dictionary.\n  \n  Arguments:\n    1. Dictionary or list of pairs\n    2. Key to check for\n  \n  Example: has_key {name: \"Alice\"} \"name\" -> true\n           Key exists\n  \n  Example: has_key {name: \"Alice\"} \"age\" -> false\n           Key doesn't exist\n  \n  Example: has_key {} \"anything\" -> false\n           Empty dict has no keys\n  \n  Example: has_key [[\"a\", 1], [\"b\", 2]] \"a\" -> true\n           Works with list of pairs\n  \n  Example: if has_key config \"debug\" then get config \"debug\" else false\n           Check before getting\n  \n  Use case: Check key existence, conditional logic, validation\n  Tip: Safer than checking if get result is_none (handles None values)"),
        ("dict_merge", "dict_merge :: Dict -> Dict -> Dict\n  Merge two dictionaries together.\n  Keys from the second dict override keys from the first.\n  \n  Arguments:\n    1. Base dictionary\n    2. Dictionary to merge in (takes precedence)\n  \n  Example: dict_merge {a: 1, b: 2} {c: 3} -> {a: 1, b: 2, c: 3}\n           Add new keys\n  \n  Example: dict_merge {a: 1, b: 2} {b: 99} -> {a: 1, b: 99}\n           Override existing key\n  \n  Example: dict_merge {} {a: 1} -> {a: 1}\n           Merge into empty dict\n  \n  Example: dict_merge defaults user_config\n           Apply user config over defaults\n  \n  Example: fold dict_merge {} list_of_dicts\n           Merge multiple dicts together\n  \n  Use case: Config layering, combining data, applying defaults\n  Note: Only works with Dict type, not list of pairs"),

        // Type Conversion
        ("to_string", "to_string :: a -> String\n  Convert any value to its string representation.\n  \n  Arguments:\n    1. Value to convert (any type)\n  \n  Example: to_string 42 -> \"42\"\n           Number to string\n  \n  Example: to_string 3.14 -> \"3.14\"\n           Float to string\n  \n  Example: to_string true -> \"true\"\n           Boolean to string\n  \n  Example: to_string [1, 2, 3] -> \"[1, 2, 3]\"\n           List to string\n  \n  Example: to_string {name: \"Alice\"} -> \"{name: \\\"Alice\\\"}\"\n           Dict to string\n  \n  Example: map to_string [1, 2, 3] -> [\"1\", \"2\", \"3\"]\n           Convert list of numbers to strings\n  \n  Use case: Display values, concatenation, logging, formatting output\n  Tip: Required when joining numbers with strings"),
        ("to_int", "to_int :: String -> Int\n  Parse a string as an integer number.\n  \n  Arguments:\n    1. String containing an integer\n  \n  Example: to_int \"42\" -> 42\n           Parse positive integer\n  \n  Example: to_int \"-17\" -> -17\n           Parse negative integer\n  \n  Example: to_int \"0\" -> 0\n           Parse zero\n  \n  Example: map to_int [\"1\", \"2\", \"3\"] -> [1, 2, 3]\n           Parse list of number strings\n  \n  Example: to_int (env_var_or \"PORT\" \"8080\")\n           Parse environment variable as integer\n  \n  Use case: Parse user input, read config values, process CSV data\n  Note: Errors if string is not a valid integer (use is_digit to check first)"),
        ("to_float", "to_float :: String -> Float\n  Parse a string as a floating-point number.\n  \n  Arguments:\n    1. String containing a number\n  \n  Example: to_float \"3.14\" -> 3.14\n           Parse decimal number\n  \n  Example: to_float \"42\" -> 42.0\n           Integers become floats\n  \n  Example: to_float \"-2.5\" -> -2.5\n           Parse negative float\n  \n  Example: to_float \"1.5e10\" -> 15000000000.0\n           Scientific notation\n  \n  Example: map to_float [\"1.1\", \"2.2\", \"3.3\"] -> [1.1, 2.2, 3.3]\n           Parse list of float strings\n  \n  Use case: Parse measurements, read scientific data, process CSV\n  Note: Errors if string is not a valid number"),
        ("to_bool", "to_bool :: a -> Bool\n  Convert a value to a boolean.\n  \n  Arguments:\n    1. Value to convert\n  \n  Truthy values (become true):\n    - Non-empty strings (\"yes\", \"true\", \"1\", any text)\n    - Non-zero numbers (1, -1, 3.14)\n    - Non-empty lists ([1], [\"a\"])\n    - Non-empty dicts ({a: 1})\n    - true\n  \n  Falsy values (become false):\n    - Empty string \"\"\n    - Zero (0, 0.0)\n    - Empty list []\n    - Empty dict {}\n    - false\n    - None\n  \n  Example: to_bool \"yes\" -> true\n  Example: to_bool \"\" -> false\n  Example: to_bool 0 -> false\n  Example: to_bool [1] -> true\n  \n  Use case: Normalize boolean flags, check for empty/non-empty"),
        ("to_char", "to_char :: Int -> String\n  Convert a Unicode codepoint (number) to its character.\n  \n  Arguments:\n    1. Unicode codepoint (integer)\n  \n  Example: to_char 72 -> \"H\"\n           ASCII 72 is 'H'\n  \n  Example: to_char 65 -> \"A\"\n           ASCII 65 is 'A'\n  \n  Example: to_char 48 -> \"0\"\n           ASCII 48 is digit '0'\n  \n  Example: to_char 128512 -> \"😀\"\n           Unicode emoji\n  \n  Example: map to_char [72, 105] -> [\"H\", \"i\"]\n           Convert codepoints to characters\n  \n  Example: map to_char (range 65 90) -> [\"A\"..\"Z\"]\n           Generate uppercase alphabet\n  \n  Use case: Generate characters, work with Unicode, ASCII art"),
        ("to_list", "to_list :: String -> [String]\n  Convert a string to a list of individual characters.\n  Each character becomes a separate single-character string.\n  \n  Arguments:\n    1. String to convert\n  \n  Example: to_list \"Hi\" -> [\"H\", \"i\"]\n           Split into characters\n  \n  Example: to_list \"hello\" -> [\"h\", \"e\", \"l\", \"l\", \"o\"]\n           Each char is a string\n  \n  Example: to_list \"\" -> []\n           Empty string gives empty list\n  \n  Example: to_list \"café\" -> [\"c\", \"a\", \"f\", \"é\"]\n           Works with Unicode\n  \n  Example: to_list word -> reverse -> join \"\"\n           Reverse a word character by character\n  \n  Use case: Character manipulation, anagram checking, string analysis\n  Note: Same as 'chars' function"),
        ("neg", "neg :: Number -> Number\n  Negate a number (flip its sign).\n  Positive becomes negative, negative becomes positive.\n  \n  Arguments:\n    1. Number to negate\n  \n  Example: neg 5 -> -5\n           Positive to negative\n  \n  Example: neg -3 -> 3\n           Negative to positive\n  \n  Example: neg 0 -> 0\n           Zero stays zero\n  \n  Example: neg 3.14 -> -3.14\n           Works with floats\n  \n  Example: map neg [1, -2, 3] -> [-1, 2, -3]\n           Negate all numbers in list\n  \n  Use case: Flip signs, calculate differences, invert values\n  Tip: neg x is equivalent to 0 - x or -1 * x"),

        // Formatting
        ("format_int", "format_int :: Number -> Int -> String\n  Format an integer with zero-padding to a minimum width.\n  \n  Arguments:\n    1. Number to format\n    2. Minimum width (pads with leading zeros)\n  \n  Example: format_int 7 3 -> \"007\"\n           Pad to 3 digits\n  \n  Example: format_int 42 5 -> \"00042\"\n           Pad to 5 digits\n  \n  Example: format_int 12345 3 -> \"12345\"\n           Number wider than width is unchanged\n  \n  Example: format_int 1 2 -> \"01\"\n           Two-digit format (months, days)\n  \n  Example: map (\\n format_int n 4) [1, 23, 456] -> [\"0001\", \"0023\", \"0456\"]\n           Format sequence numbers\n  \n  Use case: File naming, sequence IDs, time formatting, fixed-width output"),
        ("format_float", "format_float :: Number -> Int -> String\n  Format a floating-point number with specified decimal places.\n  \n  Arguments:\n    1. Number to format\n    2. Number of decimal places\n  \n  Example: format_float 3.14159 2 -> \"3.14\"\n           Two decimal places\n  \n  Example: format_float 3.14159 4 -> \"3.1416\"\n           Four decimal places (rounds)\n  \n  Example: format_float 42 2 -> \"42.00\"\n           Integer with decimals\n  \n  Example: format_float 0.1 3 -> \"0.100\"\n           Pad with trailing zeros\n  \n  Example: map (\\n format_float n 2) prices\n           Format all prices to 2 decimals\n  \n  Use case: Currency, measurements, scientific data, consistent formatting"),
        ("format_hex", "format_hex :: Number -> String\n  Format an integer as lowercase hexadecimal (base 16).\n  \n  Arguments:\n    1. Integer to format\n  \n  Example: format_hex 255 -> \"ff\"\n           255 in hex\n  \n  Example: format_hex 16 -> \"10\"\n           16 in hex\n  \n  Example: format_hex 0 -> \"0\"\n           Zero\n  \n  Example: format_hex 16777215 -> \"ffffff\"\n           White color in hex\n  \n  Example: map format_hex [255, 128, 64] -> [\"ff\", \"80\", \"40\"]\n           RGB values to hex\n  \n  Use case: Color codes, memory addresses, byte values, IDs\n  Tip: Use pad_left to get fixed width: pad_left (format_hex n) 2 \"0\""),
        ("format_octal", "format_octal :: Number -> String\n  Format an integer as octal (base 8).\n  \n  Arguments:\n    1. Integer to format\n  \n  Example: format_octal 64 -> \"100\"\n           64 in octal\n  \n  Example: format_octal 8 -> \"10\"\n           8 in octal\n  \n  Example: format_octal 511 -> \"777\"\n           Unix permission 777\n  \n  Example: format_octal 420 -> \"644\"\n           Unix permission 644\n  \n  Use case: File permissions, Unix utilities, legacy systems"),
        ("format_binary", "format_binary :: Number -> String\n  Format an integer as binary (base 2).\n  \n  Arguments:\n    1. Integer to format\n  \n  Example: format_binary 15 -> \"1111\"\n           15 in binary (all bits set)\n  \n  Example: format_binary 8 -> \"1000\"\n           8 in binary\n  \n  Example: format_binary 255 -> \"11111111\"\n           One byte, all ones\n  \n  Example: format_binary 0 -> \"0\"\n           Zero\n  \n  Use case: Bit manipulation, flags visualization, low-level debugging\n  Tip: Use pad_left for fixed width: pad_left (format_binary n) 8 \"0\""),
        ("format_scientific", "format_scientific :: Number -> Int -> String\n  Format a number in scientific notation (exponential form).\n  \n  Arguments:\n    1. Number to format\n    2. Decimal places in mantissa\n  \n  Example: format_scientific 12345 2 -> \"1.23e4\"\n           Two decimal places\n  \n  Example: format_scientific 0.00042 2 -> \"4.20e-4\"\n           Small numbers\n  \n  Example: format_scientific 1000000 1 -> \"1.0e6\"\n           One million\n  \n  Example: format_scientific 6.022e23 3 -> \"6.022e23\"\n           Avogadro's number\n  \n  Use case: Scientific data, very large/small numbers, compact display"),
        ("format_bytes", "format_bytes :: Number -> String\n  Format a byte count as human-readable size (KB, MB, GB, etc.).\n  \n  Arguments:\n    1. Number of bytes\n  \n  Example: format_bytes 1024 -> \"1.00 KB\"\n           One kilobyte\n  \n  Example: format_bytes 1536000 -> \"1.46 MB\"\n           About 1.5 megabytes\n  \n  Example: format_bytes 1073741824 -> \"1.00 GB\"\n           One gigabyte\n  \n  Example: format_bytes 500 -> \"500 B\"\n           Under 1KB shows bytes\n  \n  Example: map format_bytes file_sizes\n           Format all file sizes\n  \n  Use case: File sizes, disk usage, bandwidth, storage quotas\n  Note: Uses binary units (1 KB = 1024 bytes)"),
        ("format_csv", "format_csv :: ([Dict]|[[a]]) -> String\n  Format data as CSV (Comma-Separated Values).\n  Accepts list of Dicts (with headers) or list of lists (no headers).\n  \n  List of Dicts (with headers):\n    Example: format_csv [{name: \"Alice\", age: 30}, {name: \"Bob\", age: 25}]\n    Output: \"name,age\\nAlice,30\\nBob,25\\n\"\n  \n  List of Lists (no headers):\n    Example: format_csv [[\"Alice\", 30], [\"Bob\", 25]]\n    Output: \"Alice,30\\nBob,25\\n\"\n  \n  Example: format_csv [{id: 1, value: \"a,b\"}]\n           -> \"id,value\\n1,\\\"a,b\\\"\\n\"\n           Values with commas are quoted\n  \n  Use case: Export data, generate reports, data interchange\n  Tip: Use with File deployment to create CSV files"),
        ("format_list", "format_list :: [a] -> String -> String\n  Join list elements into a string with a separator.\n  Similar to 'join' but auto-converts elements to strings.\n  \n  Arguments:\n    1. List to format\n    2. Separator string\n  \n  Example: format_list [\"a\", \"b\", \"c\"] \", \" -> \"a, b, c\"\n           Join with comma-space\n  \n  Example: format_list [1, 2, 3] \" | \" -> \"1 | 2 | 3\"\n           Join numbers (auto-converts)\n  \n  Example: format_list [true, false] \"/\" -> \"true/false\"\n           Join booleans\n  \n  Example: format_list items \"\\n\"\n           One item per line\n  \n  Use case: Display lists, create delimited strings, output formatting\n  Tip: Unlike 'join', this auto-converts non-strings to strings"),
        ("format_table", "format_table :: ([[a]]|Dict) -> String -> String\n  Format data as a simple 2D text table with custom separator.\n  \n  Arguments:\n    1. 2D list (rows) or Dict (key-value pairs)\n    2. Column separator string\n  \n  Example: format_table [[\"Name\", \"Age\"], [\"Alice\", \"30\"]] \" | \"\n           -> \"Name | Age\\nAlice | 30\"\n  \n  Example: format_table {name: \"Alice\", age: 30} \": \"\n           -> \"name: Alice\\nage: 30\"\n  \n  Example: format_table [[\"A\", \"B\"], [\"1\", \"2\"], [\"3\", \"4\"]] \"\\t\"\n           Tab-separated table\n  \n  Use case: Display tabular data, simple reports, debug output\n  Note: For aligned columns, use pad_right on each cell first"),
        ("format_json", "format_json :: a -> String\n  Convert any value to its JSON representation.\n  \n  Arguments:\n    1. Value to convert to JSON\n  \n  Example: format_json [1, 2, 3] -> \"[1, 2, 3]\"\n           List to JSON array\n  \n  Example: format_json {name: \"Alice\", age: 30}\n           -> \"{\\\"name\\\":\\\"Alice\\\",\\\"age\\\":30}\"\n  \n  Example: format_json true -> \"true\"\n           Boolean to JSON\n  \n  Example: format_json \"hello\" -> \"\\\"hello\\\"\"\n           String to JSON (with quotes)\n  \n  Use case: API responses, config files, data export, serialization\n  Note: Output is compact (no pretty-printing)"),
        ("format_currency", "format_currency :: Number -> String -> String\n  Format a number as currency with a symbol.\n  \n  Arguments:\n    1. Amount (number)\n    2. Currency symbol\n  \n  Example: format_currency 19.99 \"$\" -> \"$19.99\"\n           US dollars\n  \n  Example: format_currency 1234.5 \"€\" -> \"€1234.50\"\n           Euros (pads to 2 decimals)\n  \n  Example: format_currency 99 \"£\" -> \"£99.00\"\n           British pounds\n  \n  Example: map (\\p format_currency p \"$\") prices\n           Format all prices\n  \n  Use case: Invoices, reports, e-commerce, financial data\n  Note: Always shows 2 decimal places"),
        ("format_percent", "format_percent :: Number -> Int -> String\n  Format a decimal number as a percentage.\n  \n  Arguments:\n    1. Decimal value (0.5 = 50%)\n    2. Decimal places to show\n  \n  Example: format_percent 0.856 2 -> \"85.60%\"\n           Two decimal places\n  \n  Example: format_percent 0.5 0 -> \"50%\"\n           No decimals\n  \n  Example: format_percent 1.0 1 -> \"100.0%\"\n           100 percent\n  \n  Example: format_percent 0.333 1 -> \"33.3%\"\n           One-third\n  \n  Use case: Statistics, progress indicators, ratios, analytics\n  Note: Multiplies by 100 and adds % symbol"),
        ("format_bool", "format_bool :: Bool -> String -> String\n  Format a boolean with custom true/false text.\n  \n  Arguments:\n    1. Boolean value\n    2. Format string: \"trueText/falseText\"\n  \n  Example: format_bool true \"yes/no\" -> \"Yes\"\n           Custom yes/no\n  \n  Example: format_bool false \"yes/no\" -> \"No\"\n           Returns capitalized version\n  \n  Example: format_bool true \"enabled/disabled\" -> \"Enabled\"\n           Feature flags\n  \n  Example: format_bool active \"✓/✗\" -> \"✓\" or \"✗\"\n           Unicode symbols\n  \n  Use case: User-friendly boolean display, reports, status indicators\n  Note: First word is for true, second for false, separated by /"),
        ("truncate", "truncate :: String -> Int -> String\n  Truncate a string to a maximum length, adding ellipsis if needed.\n  \n  Arguments:\n    1. String to truncate\n    2. Maximum length (including ellipsis)\n  \n  Example: truncate \"Hello World\" 8 -> \"Hello...\"\n           Truncate to 8 chars total\n  \n  Example: truncate \"Hi\" 10 -> \"Hi\"\n           Short string unchanged\n  \n  Example: truncate \"Long text here\" 7 -> \"Long...\"\n           Truncate to 7 chars\n  \n  Example: map (\\s truncate s 20) descriptions\n           Truncate all descriptions\n  \n  Use case: UI display, previews, table columns, tooltips\n  Note: Ellipsis (...) counts toward the max length"),
        ("center", "center :: String -> Int -> String\n  Center-align a string within a given width using spaces.\n  \n  Arguments:\n    1. String to center\n    2. Total width\n  \n  Example: center \"Hi\" 10 -> \"    Hi    \"\n           Center in 10 chars\n  \n  Example: center \"Title\" 20 -> \"       Title        \"\n           Center a title\n  \n  Example: center \"ABC\" 3 -> \"ABC\"\n           String same as width\n  \n  Example: center \"X\" 5 -> \"  X  \"\n           Single character centered\n  \n  Use case: Headers, titles, formatted output, ASCII art\n  Note: If padding is odd, extra space goes on right"),

        // HTML
        ("html_escape", "html_escape :: String -> String\n  Escape special HTML characters to prevent XSS and display issues.\n  \n  Arguments:\n    1. String to escape\n  \n  Characters escaped:\n    < -> &lt;\n    > -> &gt;\n    & -> &amp;\n    \" -> &quot;\n    ' -> &#39;\n  \n  Example: html_escape \"<div>\" -> \"&lt;div&gt;\"\n           Escape tags\n  \n  Example: html_escape \"Tom & Jerry\" -> \"Tom &amp; Jerry\"\n           Escape ampersand\n  \n  Example: html_escape \"She said \\\"Hi\\\"\" -> \"She said &quot;Hi&quot;\"\n           Escape quotes\n  \n  Example: html_escape user_input\n           Always escape user input in HTML!\n  \n  Use case: Prevent XSS, safe HTML generation, display raw code\n  Important: Always use when inserting user data into HTML"),
        ("html_tag", "html_tag :: String -> String -> String\n  Create an HTML element with a tag name and content.\n  \n  Arguments:\n    1. Tag name (e.g., \"p\", \"div\", \"span\")\n    2. Content (inner HTML)\n  \n  Example: html_tag \"p\" \"Hello\" -> \"<p>Hello</p>\"\n           Paragraph\n  \n  Example: html_tag \"strong\" \"Important\" -> \"<strong>Important</strong>\"\n           Bold text\n  \n  Example: html_tag \"li\" item\n           List item\n  \n  Example: html_tag \"div\" (html_tag \"p\" \"nested\")\n           -> \"<div><p>nested</p></div>\"\n           Nested elements\n  \n  Example: map (\\item html_tag \"li\" item) items -> join \"\\n\"\n           Generate list items\n  \n  Use case: HTML generation, template building, markup creation\n  Note: For tags with attributes, build string manually or use templates"),
        ("html_attr", "html_attr :: String -> String -> String\n  Create an HTML attribute string (properly quoted).\n  \n  Arguments:\n    1. Attribute name\n    2. Attribute value\n  \n  Example: html_attr \"class\" \"btn\" -> \"class=\\\"btn\\\"\"\n           Class attribute\n  \n  Example: html_attr \"href\" \"https://example.com\" \n           -> \"href=\\\"https://example.com\\\"\"\n           Link attribute\n  \n  Example: html_attr \"id\" \"main-content\" -> \"id=\\\"main-content\\\"\"\n           ID attribute\n  \n  Example: html_attr \"data-value\" \"42\" -> \"data-value=\\\"42\\\"\"\n           Data attribute\n  \n  Use case: Building HTML attributes safely, dynamic attribute generation\n  Note: Values are properly quoted; use html_escape for user values"),

        // Markdown
        ("md_heading", "md_heading :: Int -> String -> String\n  Create a markdown heading of specified level (1-6).\n  \n  Arguments:\n    1. Heading level (1 = h1, 2 = h2, etc.)\n    2. Heading text\n  \n  Example: md_heading 1 \"Title\" -> \"# Title\"\n           Level 1 heading (h1)\n  \n  Example: md_heading 2 \"Section\" -> \"## Section\"\n           Level 2 heading (h2)\n  \n  Example: md_heading 3 \"Subsection\" -> \"### Subsection\"\n           Level 3 heading (h3)\n  \n  Example: map (\\s md_heading 2 s) sections -> join \"\\n\\n\"\n           Generate section headings\n  \n  Use case: Generate README files, documentation, markdown reports"),
        ("md_link", "md_link :: String -> String -> String\n  Create a markdown link.\n  \n  Arguments:\n    1. Link text (displayed)\n    2. URL (destination)\n  \n  Example: md_link \"Click here\" \"https://example.com\"\n           -> \"[Click here](https://example.com)\"\n  \n  Example: md_link \"GitHub\" \"https://github.com\"\n           -> \"[GitHub](https://github.com)\"\n  \n  Example: md_link file (concat \"./docs/\" file)\n           Create relative link\n  \n  Example: map (\\item md_link item.name item.url) links\n           Generate multiple links\n  \n  Use case: Navigation, references, table of contents"),
        ("md_code", "md_code :: String -> String\n  Wrap text in inline code backticks.\n  \n  Arguments:\n    1. Code text\n  \n  Example: md_code \"x = 1\" -> \"`x = 1`\"\n           Inline code\n  \n  Example: md_code \"println!\" -> \"`println!`\"\n           Function name\n  \n  Example: md_code \"config.yml\" -> \"`config.yml`\"\n           Filename\n  \n  Example: concat \"Use \" (md_code \"map\") \" to transform lists\"\n           -> \"Use `map` to transform lists\"\n           Inline code in text\n  \n  Use case: Highlight code, commands, filenames, technical terms"),
        ("md_list", "md_list :: [String] -> String\n  Create a markdown bulleted list from a list of strings.\n  \n  Arguments:\n    1. List of items\n  \n  Example: md_list [\"First\", \"Second\", \"Third\"]\n           -> \"- First\\n- Second\\n- Third\"\n  \n  Example: md_list items\n           Generate bullet points\n  \n  Example: md_list (map basename files)\n           List of filenames\n  \n  Example: md_list [\"Item 1\", \"Item 2\"] ++ \"\\n\\n\" ++ md_list [\"Other\"]\n           Multiple lists\n  \n  Use case: Generate lists, README features, documentation\n  Note: Creates unordered (bulleted) list with - prefix"),
        ("markdown_to_html", "markdown_to_html :: (String|Template) -> String\n  Convert markdown text to HTML.\n  \n  Arguments:\n    1. Markdown string\n  \n  Supported syntax:\n    # Heading      -> <h1>Heading</h1>\n    ## Heading     -> <h2>Heading</h2>\n    **bold**       -> <strong>bold</strong>\n    *italic*       -> <em>italic</em>\n    `code`         -> <code>code</code>\n    Paragraphs     -> <p>...</p>\n  \n  Example: markdown_to_html \"# Hello\\nWorld\" \n           -> \"<h1>Hello</h1>\\n<p>World</p>\"\n  \n  Example: markdown_to_html \"**bold** and *italic*\"\n           -> \"<p><strong>bold</strong> and <em>italic</em></p>\"\n  \n  Example: markdown_to_html readme_content\n           Convert README to HTML\n  \n  Use case: Blog posts, documentation sites, email formatting\n  Note: Basic markdown support; for full markdown, use external tools"),

        // File Operations
        ("readfile", "readfile :: String|Path -> String\n  Read the entire contents of a file as a single string.\n  \n  Arguments:\n    1. File path (string or @path)\n  \n  Example: readfile \"config.json\" -> \"{\\\"port\\\": 8080}\"\n           Read a JSON file\n  \n  Example: readfile @templates/header.html -> \"<header>...</header>\"\n           Read using path literal\n  \n  Example: let content = readfile \"README.md\" in\n           upper content\n           Read and convert to uppercase\n  \n  Use case: Load configuration files, read templates, process file contents\n  Note: Strings can be absolute (safe for reading). Path literals (@...) must be relative."),
        ("readlines", "readlines :: String|Path -> [String]\n  Read a file and return each line as a separate string in a list.\n  \n  Arguments:\n    1. File path (string or @path)\n  \n  Example: readlines \"todos.txt\" -> [\"Buy milk\", \"Write code\", \"Deploy\"]\n           Read each line\n  \n  Example: readlines \"data.csv\" -> [\"name,age\", \"Alice,30\", \"Bob,25\"]\n           Read CSV file (then use csv_parse to parse it properly)\n  \n  Example: readlines \"log.txt\" -> length -> to_string\n           Count number of lines\n  \n  Tip: Use with map/filter to process lines:\n       readlines \"file.txt\" -> filter (\\line starts_with line \"ERROR\")"),
        ("fill_template", "fill_template :: String|Path -> (Dict|[[String, String]]) -> String\n  Read a file and replace {{placeholder}} syntax with actual values.\n  \n  Arguments:\n    1. Template file path\n    2. Dictionary or list of [key, value] pairs with values to fill in\n  \n  Template file (email.txt):\n    Hello {{name}}!\n    Your order #{{order_id}} is ready.\n  \n  Example: fill_template \"email.txt\" {name: \"Alice\", order_id: \"12345\"}\n           -> \"Hello Alice!\\nYour order #12345 is ready.\"\n  \n  Example: fill_template @templates/config.yml {env: \"prod\", port: \"8080\"}\n           Use path literal and fill variables\n  \n  Use case: Generate personalized emails, configs, reports\n  Tip: Placeholders must match dictionary keys exactly"),
        ("exists", "exists :: String|Path -> Bool\n  Check if a file or directory exists on disk.\n  \n  Arguments:\n    1. File or directory path\n  \n  Example: exists \"config.json\" -> true\n           File exists\n  \n  Example: exists \"missing.txt\" -> false\n           File doesn't exist\n  \n  Example: if exists \"config.json\"\n           then readfile \"config.json\"\n           else \"{}\"\n           Conditional file reading\n  \n  Use case: Check before reading, conditional logic, validation"),
        ("basename", "basename :: String|Path -> String\n  Extract just the filename (last part) from a file path.\n  \n  Arguments:\n    1. File path (string or @path)\n  \n  Example: basename @config/app.yml -> \"app.yml\"\n           Get just the filename\n  \n  Example: basename \"/usr/local/bin/avon\" -> \"avon\"\n           Get filename from absolute path\n  \n  Example: basename \"src/main.av\" -> \"main.av\"\n           Get filename from relative path\n  \n  Example: map basename file_paths\n           Get all filenames from list of paths\n  \n  Use case: Extract filenames, display names, process file lists"),
        ("dirname", "dirname :: String|Path -> String\n  Extract the directory (folder path) from a file path.\n  \n  Arguments:\n    1. File path (string or @path)\n  \n  Example: dirname @config/app.yml -> \"config\"\n           Get directory part\n  \n  Example: dirname \"/usr/local/bin/avon\" -> \"/usr/local/bin\"\n           Get directory from absolute path\n  \n  Example: dirname \"src/utils/helper.av\" -> \"src/utils\"\n           Get parent directory\n  \n  Example: map dirname file_paths -> unique\n           Get all unique directories\n  \n  Use case: Find parent directories, organize files, build paths"),
        ("walkdir", "walkdir :: String|Path -> [String]\n  List ALL files in a directory and all its subdirectories (recursive).\n  Returns full paths to every file found.\n  \n  Arguments:\n    1. Directory path to scan\n  \n  Example: walkdir \"src\" -> [\"src/main.av\", \"src/lib.av\", \"src/utils/helper.av\"]\n           Find all files in src/ and subdirectories\n  \n  Example: walkdir \"examples\" -> filter (\\f ends_with f \".av\")\n           Find all .av files recursively\n  \n  Example: walkdir \".\" -> length\n           Count total files in current directory tree\n  \n  Use case: Find all files, search directories, batch processing\n  Tip: Use with filter to find specific file types"),

        // Data Utilities
        ("json_parse", "json_parse :: String -> (Dict|List|a)\n  Parse a JSON file and return its contents as Avon values.\n  \n  Arguments:\n    1. Path to JSON file\n  \n  Returns:\n    - Dict for JSON objects: {\"key\": \"value\"} -> {key: \"value\"}\n    - List for JSON arrays: [1, 2, 3] -> [1, 2, 3]\n    - String/Number/Bool for primitives\n    - None for null values\n  \n  Example: json_parse \"config.json\" -> {port: 8080, host: \"localhost\"}\n           Parse config file\n  \n  Example: let config = json_parse \"settings.json\" in config.theme\n           Access parsed field\n  \n  Example: json_parse \"data.json\" -> map (\\item item.name)\n           Process array items\n  \n  Example: let users = json_parse \"users.json\" in\n           filter (\\u u.active) users\n           Filter parsed data\n  \n  Use case: Read config files, load data, API response files\n  Note: Reads from FILE path, not JSON string. Use for file parsing only."),
        ("yaml_parse", "yaml_parse :: String -> (Dict|List|a)\n  Parse a YAML file and return its contents as Avon values.\n  \n  Arguments:\n    1. Path to YAML file\n  \n  Returns:\n    - Dict for YAML mappings\n    - List for YAML sequences\n    - String/Number/Bool for scalars\n  \n  Example: yaml_parse \"config.yml\" -> {database: {host: \"localhost\"}}\n           Parse YAML config\n  \n  Example: yaml_parse \"docker-compose.yml\" -> services\n           Parse Docker Compose\n  \n  Example: let config = yaml_parse \"settings.yaml\" in\n           get config \"environment\"\n           Access nested config\n  \n  Use case: Kubernetes configs, Docker Compose, CI/CD pipelines\n  Note: Reads from FILE path, not YAML string. Use for file parsing only."),
        ("toml_parse", "toml_parse :: String -> (Dict|List|a)\n  Parse a TOML file and return its contents as Avon values.\n  \n  Arguments:\n    1. Path to TOML file\n  \n  Returns:\n    - Dict for TOML tables\n    - List for TOML arrays\n    - String/Number/Bool for values\n  \n  Example: toml_parse \"Cargo.toml\" -> {package: {name: \"myapp\", version: \"1.0\"}}\n           Parse Cargo.toml\n  \n  Example: toml_parse \"pyproject.toml\" -> project\n           Parse Python project config\n  \n  Example: let cargo = toml_parse \"Cargo.toml\" in\n           cargo.package.name\n           Get package name\n  \n  Use case: Rust Cargo files, Python pyproject, config files\n  Note: Reads from FILE path, not TOML string. Use for file parsing only."),
        ("csv_parse", "csv_parse :: String -> [Dict|[String]]\n  Parse a CSV file and return its contents as a list.\n  \n  Arguments:\n    1. Path to CSV file\n  \n  Returns:\n    - With headers: List of Dicts [{name: \"Alice\", age: \"30\"}, ...]\n    - Without headers: List of lists [[\"Alice\", \"30\"], ...]\n  \n  Example: csv_parse \"users.csv\" -> [{name: \"Alice\", age: \"30\"}, {name: \"Bob\", age: \"25\"}]\n           Parse CSV with headers\n  \n  Example: csv_parse \"data.csv\" -> map (\\row row.value)\n           Extract column from parsed CSV\n  \n  Example: let data = csv_parse \"sales.csv\" in\n           filter (\\row to_int row.amount > 100) data\n           Filter CSV rows\n  \n  Example: csv_parse \"export.csv\" -> length\n           Count rows in CSV\n  \n  Use case: Data import, spreadsheet processing, reports\n  Note: Reads from FILE path. All values are strings (use to_int/to_float to convert).\n  Note: First row is treated as headers if it looks like headers."),
        ("import", "import :: String|Path -> Value\n  Import and evaluate another Avon file, returning its result.\n  \n  Arguments:\n    1. Path to Avon file (string or @path)\n  \n  Example: import \"lib.av\" -> value from lib.av\n           Import a library file\n  \n  Example: import @utils/helpers.av -> {helper functions}\n           Import using path literal\n  \n  Example: let utils = import \"utils.av\" in\n           utils.format_date (now)\n           Use imported functions\n  \n  Example: let config = import \"config.av\" in\n           deploy with config values\n           Import config for deployment\n  \n  Use case: Code reuse, libraries, config separation, modular templates\n  Note: Imported file is fully evaluated; its final value is returned"),
        ("import_git", "import_git :: String -> String -> Value\n  Import and evaluate an Avon file directly from GitHub.\n  \n  Arguments:\n    1. Repository path: \"owner/repo/path/to/file.av\"\n    2. Git commit hash (full 40-character SHA-1)\n  \n  Example: import_git \"user/avon-libs/utils.av\" \"a1b2c3d4e5f6...\" \n           Import from GitHub\n  \n  Why commit hash?\n    - Reproducibility: Same code every time\n    - Security: You control exactly what runs\n    - No surprises: Code won't change unexpectedly\n  \n  How to get commit hash:\n    1. On GitHub, click file -> History -> click commit\n    2. Copy 40-char hash from URL or page\n    3. Or run: git log --format=\"%H\" -n 1\n  \n  Example:\n    let http = import_git \"pyrotek45/avon-libs/http.av\" \n                          \"abc123...full40chars...\" in\n    http.get \"https://api.example.com\"\n  \n  Use case: Share libraries, version-pinned dependencies, remote configs\n  Note: Requires internet. Downloads from raw.githubusercontent.com"),

        // Type Checking
        ("typeof", "typeof :: a -> String\n  Get the type of a value as a string.\n  \n  Arguments:\n    1. Value to inspect\n  \n  Returns one of: \"String\", \"Number\", \"Bool\", \"List\", \"Dict\", \"Function\", \"None\", \"Path\"\n  \n  Example: typeof 42 -> \"Number\"\n           Integer is a Number\n  \n  Example: typeof 3.14 -> \"Number\"\n           Float is also a Number\n  \n  Example: typeof \"hello\" -> \"String\"\n  Example: typeof [1, 2, 3] -> \"List\"\n  Example: typeof {a: 1} -> \"Dict\"\n  Example: typeof true -> \"Bool\"\n  Example: typeof (\\x x) -> \"Function\"\n  Example: typeof (head []) -> \"None\"\n  \n  Example: if typeof x == \"List\" then length x else 1\n           Conditional based on type\n  \n  Use case: Type inspection, debugging, polymorphic functions"),
        ("is_string", "is_string :: a -> Bool\n  Check if a value is a string.\n  \n  Arguments:\n    1. Value to check\n  \n  Example: is_string \"hello\" -> true\n           String literal\n  \n  Example: is_string 42 -> false\n           Number is not a string\n  \n  Example: is_string \"\" -> true\n           Empty string is still a string\n  \n  Example: filter is_string mixed_list\n           Keep only strings from mixed list\n  \n  Example: if is_string x then upper x else to_string x\n           Handle strings differently\n  \n  Use case: Type checking, validation, filtering mixed data"),
        ("is_number", "is_number :: a -> Bool\n  Check if a value is a number (integer or float).\n  \n  Arguments:\n    1. Value to check\n  \n  Example: is_number 42 -> true\n           Integer\n  \n  Example: is_number 3.14 -> true\n           Float\n  \n  Example: is_number \"42\" -> false\n           String is not a number\n  \n  Example: filter is_number mixed_list\n           Keep only numbers from mixed list\n  \n  Example: all is_number values\n           Check if all values are numbers\n  \n  Use case: Validation, filtering, arithmetic safety"),
        ("is_int", "is_int :: a -> Bool\n  Check if a value is an integer (whole number).\n  \n  Arguments:\n    1. Value to check\n  \n  Example: is_int 42 -> true\n           Integer\n  \n  Example: is_int 3.14 -> false\n           Float is not an integer\n  \n  Example: is_int 3.0 -> false\n           Float type, even if whole\n  \n  Example: is_int \"42\" -> false\n           String is not an integer\n  \n  Example: if is_int n then n else floor n\n           Round non-integers\n  \n  Use case: Validate integer input, type checking, floor/ceil decisions"),
        ("is_float", "is_float :: a -> Bool\n  Check if a value is a floating-point number.\n  \n  Arguments:\n    1. Value to check\n  \n  Example: is_float 3.14 -> true\n           Float literal\n  \n  Example: is_float 42 -> false\n           Integer is not a float\n  \n  Example: is_float 3.0 -> true\n           Float even if whole number\n  \n  Example: any is_float numbers\n           Check if list contains any floats\n  \n  Use case: Type checking, format decisions, precision handling"),
        ("is_list", "is_list :: a -> Bool\n  Check if a value is a list.\n  \n  Arguments:\n    1. Value to check\n  \n  Example: is_list [1, 2, 3] -> true\n           List of numbers\n  \n  Example: is_list [] -> true\n           Empty list is still a list\n  \n  Example: is_list \"hello\" -> false\n           String is not a list\n  \n  Example: is_list [[1], [2]] -> true\n           Nested list is a list\n  \n  Example: if is_list x then map f x else [f x]\n           Handle both single values and lists\n  \n  Use case: Polymorphic functions, recursive processing, validation"),
        ("is_bool", "is_bool :: a -> Bool\n  Check if a value is a boolean (true or false).\n  \n  Arguments:\n    1. Value to check\n  \n  Example: is_bool true -> true\n  Example: is_bool false -> true\n  Example: is_bool 1 -> false\n           1 is a number, not a boolean\n  \n  Example: is_bool \"true\" -> false\n           String is not a boolean\n  \n  Example: filter is_bool values\n           Extract boolean values\n  \n  Use case: Type checking, config validation, strict typing"),
        ("is_function", "is_function :: a -> Bool\n  Check if a value is a function (lambda).\n  \n  Arguments:\n    1. Value to check\n  \n  Example: is_function (\\x x + 1) -> true\n           Lambda function\n  \n  Example: is_function map -> true\n           Builtin function\n  \n  Example: is_function 42 -> false\n           Number is not a function\n  \n  Example: is_function \"map\" -> false\n           String is not a function\n  \n  Example: if is_function f then f x else f\n           Apply if function, otherwise return as-is\n  \n  Use case: Higher-order functions, dynamic dispatch, callbacks"),
        ("is_dict", "is_dict :: a -> Bool\n  Check if a value is a dictionary.\n  \n  Arguments:\n    1. Value to check\n  \n  Example: is_dict {name: \"Alice\"} -> true\n           Dictionary literal\n  \n  Example: is_dict {} -> true\n           Empty dict is still a dict\n  \n  Example: is_dict [[\"a\", 1]] -> false\n           List of pairs is NOT a dict\n  \n  Example: is_dict [1, 2, 3] -> false\n           List is not a dict\n  \n  Example: if is_dict x then keys x else []\n           Get keys if dict\n  \n  Use case: Type checking, data structure validation, polymorphic handling"),
        ("is_none", "is_none :: a -> Bool\n  Check if a value is None.\n  None is returned by functions when there's no valid result.\n  \n  Arguments:\n    1. Value to check\n  \n  Sources of None:\n    - head [] (head of empty list)\n    - last [] (last of empty list)\n    - get dict \"missing_key\"\n    - nth 99 [1,2,3] (out of bounds)\n    - find (\\x false) list (no match)\n    - JSON null values\n  \n  Example: is_none none -> true\n           The none literal\n  \n  Example: is_none (head []) -> true\n           Empty list head\n  \n  Example: is_none (get {a: 1} \"b\") -> true\n           Missing key\n  \n  Example: is_none (find (\\x x > 100) [1,2,3]) -> true\n           No match found\n  \n  Pattern:\n    let value = get config \"key\" in\n    if is_none value then default else value\n  \n  Use case: Safe access, default values, error handling"),
        ("not", "not :: Bool -> Bool\n  Logical negation - flip true to false and vice versa.\n  \n  Arguments:\n    1. Boolean value\n  \n  Example: not true -> false\n  Example: not false -> true\n  \n  Example: not (1 == 2) -> true\n           Negate comparison\n  \n  Example: not (is_empty list) -> true\n           Check if list is NOT empty\n  \n  Example: filter (\\x not (is_empty x)) strings\n           Keep non-empty strings\n  \n  Example: filter (\\x not (starts_with x \".\")) files\n           Filter out hidden files\n  \n  Use case: Invert conditions, negative filtering, logical operations"),

        // Assert & Debug
        ("assert", "assert :: Bool -> a -> a\n  Validate a condition; return value if true, error if false.\n  \n  Arguments:\n    1. Condition that must be true\n    2. Value to return (also used in error message)\n  \n  Example: assert (x > 0) x\n           Ensure x is positive\n  \n  Example: assert (is_string name) name\n           Type check\n  \n  Example: assert (length list > 0) list\n           Ensure non-empty list\n  \n  Example: let validated = assert (is_number x) x in\n           validated * 2\n           Validate before use\n  \n  Pipeline pattern:\n    data -> assert (\\d length d > 0) -> process\n  \n  Use case: Input validation, preconditions, defensive programming\n  Note: On failure, shows condition, value, and value's type"),
        ("trace", "trace :: String -> a -> a\n  Print a labeled debug message to stderr and return the value unchanged.\n  Perfect for debugging pipelines without changing results.\n  \n  Arguments:\n    1. Label string (description of what you're tracing)\n    2. Value to trace (will be printed and returned)\n  \n  Example: trace \"result\" 42 \n           Prints: [TRACE] result: 42\n           Returns: 42\n  \n  Example: trace \"input\" [1, 2, 3]\n           Prints: [TRACE] input: [1, 2, 3]\n  \n  Pipeline debugging:\n    data \n    -> trace \"start\" \n    -> map double \n    -> trace \"after map\" \n    -> filter even \n    -> trace \"final\"\n  \n  Use case: Debug pipelines, track values, understand data flow\n  Note: Output goes to stderr, so it doesn't affect stdout results"),
        ("debug", "debug :: String -> a -> a\n  Print the internal structure of a value with a label to stderr.\n  Shows more detail than trace (useful for complex nested data).\n  \n  Arguments:\n    1. Label string\n    2. Value to debug\n  \n  Example: debug \"config\" {a: 1, b: [2, 3]}\n           Prints: [DEBUG] config: Dict({\"a\": Number(1), \"b\": List([...])})\n  \n  Example: debug \"parsed\" (json_parse \"data.json\")\n           See internal representation of parsed data\n  \n  Example: debug \"lambda\" (\\x x + 1)\n           Inspect function structure\n  \n  Use case: Inspect complex data, understand types, deep debugging\n  Note: Shows internal Rust representation, more verbose than trace"),
        ("spy", "spy :: a -> a\n  Quick auto-numbered debug output - print value and return it.\n  No label needed; automatically numbers each spy call.\n  \n  Arguments:\n    1. Value to spy on\n  \n  Example: spy 42\n           Prints: [SPY:1] 42\n           Returns: 42\n  \n  Pipeline debugging (quick and easy):\n    data -> spy -> map f -> spy -> filter g -> spy\n    Prints:\n      [SPY:1] original data\n      [SPY:2] after map\n      [SPY:3] after filter\n  \n  Example: [1, 2, 3] -> spy -> map (\\x x * 2) -> spy -> sum -> spy\n           Track each transformation step\n  \n  Use case: Quick debugging, exploring pipelines, understanding flow\n  Note: Counter resets when program restarts"),
        ("tap", "tap :: (a -> b) -> a -> a\n  Run a function for side effects, then return the original value.\n  The function's result is discarded.\n  \n  Arguments:\n    1. Function to run (for side effects)\n    2. Value to pass and return\n  \n  Example: tap (\\x trace \"value\" x) data\n           Trace inside a tap\n  \n  Example: data -> tap (\\x assert (length x > 0) x) -> process\n           Validate in middle of pipeline\n  \n  Example: tap (\\_ trace \"checkpoint\" \"reached\") value\n           Log a checkpoint\n  \n  Example: result -> tap (\\r debug \"final\" r)\n           Debug final result while still returning it\n  \n  Use case: Side effects in pipelines, logging, validation\n  Note: Function's return value is ignored; original value passes through"),
        ("error", "error :: String -> a\n  Throw a custom error with a message.\n  Immediately stops evaluation with the provided message.\n  \n  Arguments:\n    1. Error message string\n  \n  Example: error \"Invalid input\"\n           Throw error with message\n  \n  Example: if x < 0 then error \"x must be positive\" else x\n           Conditional error\n  \n  Example: let value = get config \"required_key\" in\n           if is_none value \n           then error \"Missing required config key\" \n           else value\n           Error on missing config\n  \n  Example: match item.type\n           | \"valid\" => process item\n           | _ => error (concat \"Unknown type: \" item.type)\n           Error in match\n  \n  Use case: Input validation, error handling, fail-fast logic\n  Note: Program stops immediately; use for unrecoverable errors"),

        // Date/Time Operations
        ("now", "now :: String\n  Get current date and time in ISO 8601 format.\n  Example: now -> \"2024-03-15T14:30:45+00:00\"\n  Note: Returns RFC 3339 formatted string with timezone offset."),
        ("date_format", "date_format :: String -> String -> String\n  Format a date string with a custom format using strftime codes.\n  \n  Arguments:\n    1. ISO 8601 date string (from 'now', 'date_parse', or literal)\n    2. Format string with % codes\n  \n  Common format codes:\n    %Y - 4-digit year (2024)\n    %y - 2-digit year (24)\n    %m - Month number 01-12\n    %B - Full month name (December)\n    %b - Short month name (Dec)\n    %d - Day of month 01-31\n    %H - Hour 00-23 (24-hour)\n    %I - Hour 01-12 (12-hour)\n    %M - Minute 00-59\n    %S - Second 00-59\n    %p - AM/PM\n    %A - Full weekday (Wednesday)\n    %a - Short weekday (Wed)\n  \n  Examples:\n    date_format (now) \"%Y-%m-%d\" -> \"2024-12-11\"\n    date_format (now) \"%B %d, %Y\" -> \"December 11, 2024\"\n    date_format (now) \"%I:%M %p\" -> \"03:30 PM\"\n    date_format (now) \"%A, %B %d\" -> \"Wednesday, December 11\"\n  \n  Note: Invalid format codes will cause an error.\n        Use only standard strftime codes."),
        ("date_parse", "date_parse :: String -> String -> String\n  Parse a date string and return ISO 8601 format.\n  First arg: date string to parse\n  Second arg: strftime format string matching the input\n  Example: date_parse \"2024-03-15 14:30\" \"%Y-%m-%d %H:%M\" -> ISO 8601 string\n  Example: date_parse \"15/03/2024\" \"%d/%m/%Y\" -> ISO 8601 string\n  Returns: RFC 3339 formatted string for use with other date functions."),
        ("date_add", "date_add :: String -> String -> String\n  Add duration to a date.\n  First arg: ISO 8601 date string\n  Second arg: duration string (number + unit)\n  Units: s (seconds), m (minutes), h (hours), d (days), w (weeks), y (years)\n  Example: date_add (now) \"1d\" -> date 1 day from now\n  Example: date_add (now) \"2h\" -> date 2 hours from now\n  Example: date_add (now) \"30m\" -> date 30 minutes from now\n  Example: date_add (now) \"1w\" -> date 1 week from now"),
        ("date_diff", "date_diff :: String -> String -> Int\n  Calculate difference between two dates in seconds.\n  First arg: later date (ISO 8601)\n  Second arg: earlier date (ISO 8601)\n  Example: date_diff date1 date2 -> seconds difference\n  Returns: Positive number if first date is after second, negative if before."),
        ("timestamp", "timestamp :: Int\n  Get current Unix timestamp (seconds since epoch).\n  Example: timestamp -> 1710509445\n  Note: Useful for unique filenames and sortable timestamps."),
        ("timezone", "timezone :: String\n  Get current timezone offset.\n  Example: timezone -> \"+00:00\" or \"-05:00\"\n  Note: Returns offset from UTC in ±HH:MM format."),

        // System
        ("os", "os :: String\n  Get operating system name.\n  Returns: \"linux\", \"macos\", or \"windows\""),
        ("args", "args :: [String]\n  Get command-line arguments passed after the script file.\n  \n  Example: avon eval script.av file1.txt file2.txt\n           args -> [\"file1.txt\", \"file2.txt\"]\n  \n  Example: args -> filter (\\f (exists f)) -> head\n           Get first existing file from arguments\n  \n  Example: args -> filter (\\f (f |> ends_with \".txt\"))\n           Get all .txt files from arguments\n  \n  Use case: Build CLI tools, process multiple files, integrate with other programs\n  Note: In REPL mode, args is an empty list []"),
        ("env_var", "env_var :: String -> String\n  Read environment variable, fail if missing.\n  Example: env_var \"HOME\" -> \"/home/user\"\n  Fails if variable not set (fail-safe behavior)."),
        ("env_var_or", "env_var_or :: String -> String -> String\n  Read environment variable with default.\n  Example: env_var_or \"PORT\" \"8080\" -> env value or \"8080\""),
        ("env_vars", "env_vars :: () -> Dict\n  Get all environment variables as a dictionary.\n  \n  Example: env_vars -> {HOME: \"/home/user\", PATH: \"/usr/bin:...\", ...}\n  \n  Example: let vars = env_vars in\n           get vars \"HOME\" -> \"/home/user\"\n  \n  Example: env_vars -> keys -> filter (\\k starts_with k \"MY_APP\")\n           Get all environment variables starting with \"MY_APP\"\n  \n  Use case: Inspect environment, configuration management, debugging\n  Note: Returns all variables as string keys with string values"),

        // Math Functions
        ("abs", "abs :: Number -> Number\n  Get the absolute (non-negative) value of a number.\n  Removes the sign, keeping just the magnitude.\n  \n  Arguments:\n    1. Number (integer or float)\n  \n  Example: abs -5 -> 5\n           Negative becomes positive\n  \n  Example: abs 5 -> 5\n           Positive stays positive\n  \n  Example: abs -3.14 -> 3.14\n           Works with floats\n  \n  Example: abs 0 -> 0\n           Zero stays zero\n  \n  Example: map abs [-3, 2, -1, 5] -> [3, 2, 1, 5]\n           Get magnitudes of all numbers\n  \n  Use case: Distance calculations, comparing magnitudes, ensuring positive values"),
        ("sqrt", "sqrt :: Number -> Float\n  Calculate the square root of a number.\n  \n  Arguments:\n    1. Non-negative number\n  \n  Example: sqrt 16 -> 4.0\n           Perfect square\n  \n  Example: sqrt 2 -> 1.4142135623730951\n           Irrational result\n  \n  Example: sqrt 0 -> 0.0\n           Square root of zero\n  \n  Example: sqrt 0.25 -> 0.5\n           Works with floats\n  \n  Example: let distance = sqrt ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1))\n           Euclidean distance formula\n  \n  Use case: Geometry, statistics, physics calculations\n  Note: Returns NaN for negative numbers"),
        ("pow", "pow :: Number -> Number -> Number\n  Raise a number to a power (exponentiation).\n  \n  Arguments:\n    1. Base number\n    2. Exponent (power)\n  \n  Example: pow 2 3 -> 8\n           2^3 = 2 * 2 * 2\n  \n  Example: pow 10 2 -> 100\n           10 squared\n  \n  Example: pow 2 -1 -> 0.5\n           Negative exponent = 1/2\n  \n  Example: pow 2 0.5 -> 1.4142135623730951\n           Fractional exponent = square root\n  \n  Example: pow 2 0 -> 1\n           Anything to the power of 0 is 1\n  \n  Example: pow 10 (length digits) -> get magnitude\n           Calculate order of magnitude\n  \n  Use case: Exponential growth, scientific notation, compound interest"),
        ("floor", "floor :: Number -> Int\n  Round down to the nearest integer (toward negative infinity).\n  \n  Arguments:\n    1. Number to round\n  \n  Example: floor 3.7 -> 3\n           Round 3.7 down to 3\n  \n  Example: floor 3.2 -> 3\n           Round 3.2 down to 3\n  \n  Example: floor -2.3 -> -3\n           Negative: -2.3 rounds DOWN to -3\n  \n  Example: floor 5 -> 5\n           Integers unchanged\n  \n  Example: floor (price / 100)\n           Get dollar amount from cents\n  \n  Use case: Pagination (items per page), integer division, currency\n  Note: floor(-2.1) = -3 (rounds toward -infinity, not toward 0)"),
        ("ceil", "ceil :: Number -> Int\n  Round up to the nearest integer (toward positive infinity).\n  \n  Arguments:\n    1. Number to round\n  \n  Example: ceil 3.2 -> 4\n           Round 3.2 up to 4\n  \n  Example: ceil 3.0 -> 3\n           Exact integer stays same\n  \n  Example: ceil -2.7 -> -2\n           Negative: -2.7 rounds UP to -2\n  \n  Example: ceil 5 -> 5\n           Integers unchanged\n  \n  Example: ceil (items / page_size)\n           Calculate number of pages needed\n  \n  Use case: Pagination (total pages), capacity planning, rounding up\n  Note: ceil(-2.9) = -2 (rounds toward +infinity)"),
        ("round", "round :: Number -> Int\n  Round to the nearest integer (half away from zero).\n  \n  Arguments:\n    1. Number to round\n  \n  Example: round 3.5 -> 4\n           Half rounds up (away from zero)\n  \n  Example: round 3.4 -> 3\n           Less than half rounds down\n  \n  Example: round 3.6 -> 4\n           More than half rounds up\n  \n  Example: round -2.5 -> -3\n           Negative half rounds away from zero\n  \n  Example: round (score * 100) / 100\n           Round to 2 decimal places (multiply, round, divide)\n  \n  Use case: Display-friendly numbers, statistics, grading\n  Note: Uses \"round half away from zero\" rule"),
        ("log", "log :: Number -> Float\n  Calculate the natural logarithm (base e ≈ 2.718) of a number.\n  \n  Arguments:\n    1. Positive number\n  \n  Example: log 2.718281828 -> 1.0\n           ln(e) = 1\n  \n  Example: log 1 -> 0.0\n           ln(1) = 0\n  \n  Example: log 10 -> 2.302585...\n           Natural log of 10\n  \n  Example: log (pow 2.718281828 5) -> 5.0\n           ln(e^5) = 5\n  \n  Use case: Exponential decay, statistics, scientific calculations\n  Note: Returns NaN for negative numbers, -Infinity for 0\n  Tip: For base-10 log, use log10"),
        ("log10", "log10 :: Number -> Float\n  Calculate the base-10 (common) logarithm of a number.\n  \n  Arguments:\n    1. Positive number\n  \n  Example: log10 100 -> 2.0\n           10^2 = 100\n  \n  Example: log10 1000 -> 3.0\n           10^3 = 1000\n  \n  Example: log10 1 -> 0.0\n           10^0 = 1\n  \n  Example: log10 50 -> 1.698...\n           Between 10^1 and 10^2\n  \n  Example: ceil (log10 (n + 1))\n           Get number of digits in n\n  \n  Use case: Order of magnitude, decibels, pH calculations\n  Note: Returns NaN for negative numbers, -Infinity for 0"),
        ("gcd", "gcd :: Int -> Int -> Int\n  Calculate the greatest common divisor (GCD) of two integers.\n  The largest number that divides both evenly.\n  \n  Arguments:\n    1. First integer\n    2. Second integer\n  \n  Example: gcd 12 8 -> 4\n           12 = 4*3, 8 = 4*2, so GCD is 4\n  \n  Example: gcd 17 5 -> 1\n           Coprime numbers (no common factors)\n  \n  Example: gcd 100 25 -> 25\n           25 divides both evenly\n  \n  Example: gcd 0 5 -> 5\n           GCD with zero returns the other number\n  \n  Example: let g = gcd numerator denominator in\n           (numerator / g, denominator / g)\n           Simplify a fraction\n  \n  Use case: Simplifying fractions, finding common factors, cryptography"),
        ("lcm", "lcm :: Int -> Int -> Int\n  Calculate the least common multiple (LCM) of two integers.\n  The smallest number that both divide into evenly.\n  \n  Arguments:\n    1. First integer\n    2. Second integer\n  \n  Example: lcm 4 6 -> 12\n           4 divides 12, 6 divides 12\n  \n  Example: lcm 3 5 -> 15\n           Coprime: LCM = product\n  \n  Example: lcm 12 18 -> 36\n           12 and 18 both divide 36\n  \n  Example: lcm 7 7 -> 7\n           LCM of same number is itself\n  \n  Use case: Finding common denominators, scheduling, cycle detection\n  Tip: LCM(a, b) * GCD(a, b) = a * b"),
        ("uuid", "uuid :: () -> String\n  Generate a random UUID version 4 string.\n  \n  Example: uuid -> \"550e8400-e29b-41d4-a716-446655440000\"\n           (unique each call)\n  \n  Example: let id = uuid in concat \"user_\" id\n           -> \"user_550e8400-e29b-41d4-a716-446655440000\"\n  \n  Example: range 1 5 -> map (\\_ uuid)\n           Generate 5 unique UUIDs\n  \n  Use case: Generate unique identifiers, correlation IDs, temporary filenames"),
        ("random_int", "random_int :: Int -> Int -> Int\n  Generate a random integer in range [min, max] (inclusive).\n  \n  Arguments:\n    1. Minimum value (inclusive)\n    2. Maximum value (inclusive)\n  \n  Example: random_int 1 10 -> 7\n           Random number between 1 and 10\n  \n  Example: random_int 0 1 -> 0 or 1\n           Random binary value (coin flip)\n  \n  Example: range 1 5 -> map (\\_ random_int 1 100)\n           Generate 5 random numbers 1-100\n  \n  Note: Both bounds are inclusive"),
        ("random_float", "random_float :: Float -> Float -> Float\n  Generate a random float in range [min, max].\n  \n  Arguments:\n    1. Minimum value (inclusive)\n    2. Maximum value (exclusive)\n  \n  Example: random_float 0.0 1.0 -> 0.7324...\n           Random float between 0 and 1\n  \n  Example: random_float -10.0 10.0 -> -3.456...\n           Random float in range\n  \n  Use case: Probability simulations, random sampling, testing"),

        // New String Functions
        ("words", "words :: String -> [String]\n  Split a string into a list of words by whitespace.\n  Handles multiple spaces, tabs, and newlines.\n  \n  Arguments:\n    1. String to split\n  \n  Example: words \"hello world\" -> [\"hello\", \"world\"]\n           Basic word split\n  \n  Example: words \"  multiple   spaces  \" -> [\"multiple\", \"spaces\"]\n           Handles extra whitespace\n  \n  Example: words \"line1\\nline2\\ttab\" -> [\"line1\", \"line2\", \"tab\"]\n           Handles all whitespace types\n  \n  Example: words \"\" -> []\n           Empty string gives empty list\n  \n  Example: words input -> length\n           Count words in text\n  \n  Example: words sentence -> head\n           Get first word\n  \n  Use case: Word counting, text processing, tokenization\n  Tip: Opposite of unwords"),
        ("unwords", "unwords :: [String] -> String\n  Join a list of strings with single spaces between them.\n  \n  Arguments:\n    1. List of strings to join\n  \n  Example: unwords [\"hello\", \"world\"] -> \"hello world\"\n           Join with spaces\n  \n  Example: unwords [\"one\", \"two\", \"three\"] -> \"one two three\"\n           Multiple words\n  \n  Example: unwords [] -> \"\"\n           Empty list gives empty string\n  \n  Example: unwords [\"single\"] -> \"single\"\n           Single item unchanged\n  \n  Example: words text -> map upper -> unwords\n           Uppercase all words\n  \n  Use case: Rebuild sentences, join tokens, reconstruct text\n  Tip: Opposite of words"),
        ("lines", "lines :: String -> [String]\n  Split a string into a list of lines.\n  Handles both Unix (\\n) and Windows (\\r\\n) line endings.\n  \n  Arguments:\n    1. String to split into lines\n  \n  Example: lines \"hello\\nworld\" -> [\"hello\", \"world\"]\n           Unix line endings\n  \n  Example: lines \"a\\r\\nb\" -> [\"a\", \"b\"]\n           Windows line endings\n  \n  Example: lines \"one line\" -> [\"one line\"]\n           No newlines gives single-element list\n  \n  Example: lines (readfile \"data.txt\") -> length\n           Count lines in file\n  \n  Example: lines content -> filter (\\l not (is_empty l))\n           Remove blank lines\n  \n  Use case: Process files line-by-line, parse text data\n  Tip: Opposite of unlines"),
        ("unlines", "unlines :: [String] -> String\n  Join a list of strings with newlines between them.\n  \n  Arguments:\n    1. List of strings to join\n  \n  Example: unlines [\"hello\", \"world\"] -> \"hello\\nworld\"\n           Join with newlines\n  \n  Example: unlines [\"line1\", \"line2\", \"line3\"] -> \"line1\\nline2\\nline3\"\n           Multiple lines\n  \n  Example: unlines [] -> \"\"\n           Empty list gives empty string\n  \n  Example: lines file -> map process -> unlines\n           Process and reassemble file\n  \n  Example: unlines (map (\\n to_string n) [1, 2, 3])\n           Numbers on separate lines\n  \n  Use case: Build multiline strings, reconstruct files, format output\n  Tip: Opposite of lines"),

        // List Advanced Functions
        ("permutations", "permutations :: [a] -> [[a]]\n  Generate all possible orderings (permutations) of a list.\n  \n  Arguments:\n    1. List to permute\n  \n  Example: permutations [1, 2] -> [[1, 2], [2, 1]]\n           Two orderings\n  \n  Example: permutations [1, 2, 3] -> [[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]\n           Six orderings (3! = 6)\n  \n  Example: permutations [\"a\"] -> [[\"a\"]]\n           Single element has one permutation\n  \n  Example: permutations [] -> [[]]\n           Empty list has one permutation (itself)\n  \n  Example: length (permutations [1,2,3,4]) -> 24\n           4! = 24 permutations\n  \n  Use case: Generate all arrangements, puzzles, brute-force search\n  Note: Returns n! results for list of n elements (grows very fast!)"),
        ("combinations", "combinations :: Int -> [a] -> [[a]]\n  Generate all combinations of k elements from a list.\n  Order doesn't matter (unlike permutations).\n  \n  Arguments:\n    1. Number of elements to choose (k)\n    2. List to choose from\n  \n  Example: combinations 2 [1, 2, 3] -> [[1, 2], [1, 3], [2, 3]]\n           Choose 2 from 3 (3 ways)\n  \n  Example: combinations 1 [\"a\", \"b\", \"c\"] -> [[\"a\"], [\"b\"], [\"c\"]]\n           Choose 1 from 3 (3 ways)\n  \n  Example: combinations 3 [1, 2, 3] -> [[1, 2, 3]]\n           Choose 3 from 3 (1 way)\n  \n  Example: combinations 0 [1, 2, 3] -> [[]]\n           Choose 0 gives one empty combination\n  \n  Example: length (combinations 2 [1..10]) -> 45\n           10 choose 2 = 45\n  \n  Use case: Lottery combinations, team selections, subsets\n  Note: Returns C(n,k) = n!/(k!(n-k)!) combinations"),
        ("chunks", "chunks :: Int -> [a] -> [[a]]\n  Split a list into chunks of a specified size.\n  Last chunk may be smaller if list doesn't divide evenly.\n  \n  Arguments:\n    1. Chunk size\n    2. List to split\n  \n  Example: chunks 2 [1, 2, 3, 4, 5] -> [[1, 2], [3, 4], [5]]\n           Size 2 chunks (last is smaller)\n  \n  Example: chunks 3 [1, 2, 3, 4, 5, 6] -> [[1, 2, 3], [4, 5, 6]]\n           Even split\n  \n  Example: chunks 10 [1, 2, 3] -> [[1, 2, 3]]\n           Chunk larger than list\n  \n  Example: chunks 1 [1, 2, 3] -> [[1], [2], [3]]\n           Size 1 chunks\n  \n  Example: chunks 100 data -> map process_batch\n           Batch processing\n  \n  Use case: Pagination, batch processing, splitting data for parallel work"),
        ("windows", "windows :: Int -> [a] -> [[a]]\n  Create sliding windows of a specified size over a list.\n  Each window overlaps with the next.\n  \n  Arguments:\n    1. Window size\n    2. List to window over\n  \n  Example: windows 2 [1, 2, 3, 4] -> [[1, 2], [2, 3], [3, 4]]\n           Size 2 windows\n  \n  Example: windows 3 [1, 2, 3, 4, 5] -> [[1, 2, 3], [2, 3, 4], [3, 4, 5]]\n           Size 3 windows\n  \n  Example: windows 2 [1, 2] -> [[1, 2]]\n           Window same size as list\n  \n  Example: windows 5 [1, 2, 3] -> []\n           Window larger than list gives empty\n  \n  Example: windows 2 prices -> map (\\w (last w) - (head w))\n           Calculate price changes\n  \n  Use case: Moving averages, sequence analysis, pattern detection\n  Note: Returns (length - window_size + 1) windows"),
        ("transpose", "transpose :: [[a]] -> [[a]]\n  Transpose a 2D list (swap rows and columns).\n  Like rotating a matrix 90 degrees.\n  \n  Arguments:\n    1. 2D list (list of lists)\n  \n  Example: transpose [[1, 2], [3, 4]] -> [[1, 3], [2, 4]]\n           Swap rows/columns\n  \n  Example: transpose [[1, 2, 3], [4, 5, 6]] -> [[1, 4], [2, 5], [3, 6]]\n           2x3 becomes 3x2\n  \n  Example: transpose [[1], [2], [3]] -> [[1, 2, 3]]\n           Column to row\n  \n  Example: transpose [names, ages, cities]\n           Combine parallel lists into records\n  \n  Example: transpose (zip data1 data2)\n           Unzip alternative\n  \n  Use case: Matrix operations, parallel list processing, data reshaping\n  Note: Inner lists should have equal length for predictable results"),

        // Path Functions
        ("glob", "glob :: String -> [String]\n  Find all files matching a glob pattern.\n  Returns list of matching file paths.\n  \n  Arguments:\n    1. Glob pattern\n  \n  Pattern syntax:\n    *        Any characters (not crossing directories)\n    **       Any directories (recursive)\n    ?        Single character\n    [abc]    Character class (a, b, or c)\n    [!abc]   Negated class (not a, b, or c)\n  \n  Example: glob \"*.av\" -> [\"example.av\", \"test.av\"]\n           All .av files in current directory\n  \n  Example: glob \"src/**/*.rs\" -> [\"src/main.rs\", \"src/lib/utils.rs\"]\n           All .rs files under src/ recursively\n  \n  Example: glob \"docs/*.md\" -> [\"docs/README.md\", \"docs/API.md\"]\n           All .md files in docs/\n  \n  Example: glob \"config.{json,yaml,toml}\"\n           Match multiple extensions\n  \n  Example: glob \"*.txt\" -> filter (\\f not (starts_with (basename f) \".\"))\n           Find text files, exclude hidden\n  \n  Use case: Find files, batch processing, file discovery"),
        ("abspath", "abspath :: String -> String\n  Convert a relative path to an absolute path.\n  Resolves . and .. components.\n  \n  Arguments:\n    1. Relative or absolute path\n  \n  Example: abspath \"./file.txt\" -> \"/home/user/project/file.txt\"\n           Resolve current directory\n  \n  Example: abspath \"../other.txt\" -> \"/home/user/other.txt\"\n           Resolve parent directory\n  \n  Example: abspath \"file.txt\" -> \"/home/user/project/file.txt\"\n           Simple filename\n  \n  Example: abspath \"/already/absolute\" -> \"/already/absolute\"\n           Absolute path unchanged\n  \n  Example: map abspath (glob \"*.av\")\n           Get absolute paths for all matches\n  \n  Use case: Normalize paths, create portable references, logging\n  Note: Uses current working directory as base"),
        ("relpath", "relpath :: String -> String\n  Convert an absolute path to a relative path from the current directory.\n  \n  Arguments:\n    1. Absolute path to convert\n  \n  Example: relpath \"/home/user/project/file.txt\" -> \"file.txt\"\n           Same directory\n  \n  Example: relpath \"/home/user/other/file.txt\" -> \"../other/file.txt\"\n           Different directory\n  \n  Example: relpath \"/home/user/project/src/main.rs\" -> \"src/main.rs\"\n           Subdirectory\n  \n  Example: map relpath absolute_paths\n           Convert all paths to relative\n  \n  Use case: Display paths, create portable references, shorten paths\n  Note: Uses current working directory as reference point"),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(func_name).map(|s| s.to_string())
}

#[allow(clippy::print_literal)]
pub fn print_builtin_docs() {
    println!("Avon Builtin Functions Reference");
    println!("=================================\n");

    // String Operations
    println!("String Operations:");
    println!("------------------");
    println!("  Note: All string functions accept both String and Template types.");
    println!("        Templates are automatically converted to strings before processing.");
    println!();
    println!(
        "  {:<18} :: {}",
        "concat", "(String|Template) -> (String|Template) -> String"
    );
    println!("  {:<18} :: {}", "upper", "(String|Template) -> String");
    println!("  {:<18} :: {}", "lower", "(String|Template) -> String");
    println!("  {:<18} :: {}", "trim", "(String|Template) -> String");
    println!(
        "  {:<18} :: {}",
        "split", "(String|Template) -> (String|Template) -> [String]"
    );
    println!(
        "  {:<18} :: {}",
        "join", "[String] -> (String|Template) -> String"
    );
    println!(
        "  {:<18} :: {}",
        "replace", "(String|Template) -> (String|Template) -> (String|Template) -> String"
    );
    println!(
        "  {:<18} :: {}",
        "contains", "(String|Template) -> (String|Template) -> Bool"
    );
    println!(
        "  {:<18} :: {}",
        "starts_with", "(String|Template) -> (String|Template) -> Bool"
    );
    println!(
        "  {:<18} :: {}",
        "ends_with", "(String|Template) -> (String|Template) -> Bool"
    );
    println!("  {:<18} :: {}", "length", "(String|Template|List) -> Int");
    println!(
        "  {:<18} :: {}",
        "repeat", "(String|Template) -> Int -> String"
    );
    println!(
        "  {:<18} :: {}",
        "pad_left", "(String|Template) -> Int -> (String|Template) -> String"
    );
    println!(
        "  {:<18} :: {}",
        "pad_right", "(String|Template) -> Int -> (String|Template) -> String"
    );
    println!(
        "  {:<18} :: {}",
        "indent", "(String|Template) -> Int -> String"
    );
    println!("  {:<18} :: {}", "is_digit", "(String|Template) -> Bool");
    println!("  {:<18} :: {}", "is_alpha", "(String|Template) -> Bool");
    println!(
        "  {:<18} :: {}",
        "is_alphanumeric", "(String|Template) -> Bool"
    );
    println!(
        "  {:<18} :: {}",
        "is_whitespace", "(String|Template) -> Bool"
    );
    println!(
        "  {:<18} :: {}",
        "is_uppercase", "(String|Template) -> Bool"
    );
    println!(
        "  {:<18} :: {}",
        "is_lowercase", "(String|Template) -> Bool"
    );
    println!(
        "  {:<18} :: {}",
        "is_empty", "(String|Template|List|Dict) -> Bool"
    );
    println!();

    // List Operations
    println!("List Operations:");
    println!("----------------");
    println!("  {:<18} :: {}", "map", "(a -> b) -> [a] -> [b]");
    println!("  {:<18} :: {}", "filter", "(a -> Bool) -> [a] -> [a]");
    println!(
        "  {:<18} :: {}",
        "fold", "(acc -> a -> acc) -> acc -> [a] -> acc"
    );
    println!("  {:<18} :: {}", "flatmap", "(a -> [b]) -> [a] -> [b]");
    println!("  {:<18} :: {}", "flatten", "[[a]] -> [a]");
    println!(
        "  {:<18} :: {}",
        "length", "[a] -> Int  (also works on strings)"
    );
    println!("  {:<18} :: {}", "zip", "[a] -> [b] -> [(a, b)]");
    println!("  {:<18} :: {}", "unzip", "[(a, b)] -> ([a], [b])");
    println!("  {:<18} :: {}", "take", "Int -> [a] -> [a]");
    println!("  {:<18} :: {}", "drop", "Int -> [a] -> [a]");
    println!(
        "  {:<18} :: {}",
        "slice", "(String|[a]) -> Int -> Int -> (String|[a])"
    );
    println!(
        "  {:<18} :: {}",
        "char_at", "String -> Int -> String | None"
    );
    println!("  {:<18} :: {}", "chars", "String -> [String]");
    println!("  {:<18} :: {}", "split_at", "Int -> [a] -> ([a], [a])");
    println!(
        "  {:<18} :: {}",
        "partition", "(a -> Bool) -> [a] -> ([a], [a])"
    );
    println!("  {:<18} :: {}", "reverse", "[a] -> [a]");
    println!("  {:<18} :: {}", "sort", "[a] -> [a]");
    println!("  {:<18} :: {}", "sort_by", "(a -> b) -> [a] -> [a]");
    println!("  {:<18} :: {}", "unique", "[a] -> [a]");
    println!(
        "  {:<18} :: {}",
        "range", "Int -> Int -> [Int]  (inclusive on both ends!)"
    );
    println!("  {:<18} :: {}", "enumerate", "[a] -> [[Int, a]]");
    println!("  {:<18} :: {}", "sum", "[Number] -> Number");
    println!("  {:<18} :: {}", "product", "[Number] -> Number");
    println!("  {:<18} :: {}", "min", "[a] -> a | None");
    println!("  {:<18} :: {}", "max", "[a] -> a | None");
    println!("  {:<18} :: {}", "all", "(a -> Bool) -> [a] -> Bool");
    println!("  {:<18} :: {}", "any", "(a -> Bool) -> [a] -> Bool");
    println!("  {:<18} :: {}", "count", "(a -> Bool) -> [a] -> Int");
    println!("  {:<18} :: {}", "head", "[a] -> a | None");
    println!("  {:<18} :: {}", "nth", "Int -> [a] -> a | None");
    println!("  {:<18} :: {}", "tail", "[a] -> [a]");
    println!();

    // Random/Search List Operations
    println!("Random & Search List Operations:");
    println!("--------------------------------");
    println!("  {:<18} :: {}", "choice", "[a] -> a");
    println!("  {:<18} :: {}", "shuffle", "[a] -> [a]");
    println!("  {:<18} :: {}", "sample", "Int -> [a] -> [a]");
    println!("  {:<18} :: {}", "find", "(a -> Bool) -> [a] -> a | None");
    println!(
        "  {:<18} :: {}",
        "find_index", "(a -> Bool) -> [a] -> Int | None"
    );
    println!(
        "  {:<18} :: {}",
        "group_by", "(a -> k) -> [a] -> Dict[k, [a]]"
    );
    println!(
        "  {:<18} :: {}",
        "zip_with", "(a -> b -> c) -> [a] -> [b] -> [c]"
    );
    println!("  {:<18} :: {}", "intersperse", "a -> [a] -> [a]");
    println!();

    // Regex Functions
    println!("Regex Functions:");
    println!("----------------");
    println!("  {:<18} :: {}", "regex_match", "String -> String -> Bool");
    println!(
        "  {:<18} :: {}",
        "regex_replace", "String -> String -> String -> String"
    );
    println!(
        "  {:<18} :: {}",
        "regex_split", "String -> String -> [String]"
    );
    println!(
        "  {:<18} :: {}",
        "scan", "String -> String -> [String|[String]]"
    );
    println!();

    // Map/Dictionary Operations
    println!("Map/Dictionary Operations:");
    println!("--------------------------");
    println!(
        "  {:<18} :: {}",
        "get", "(Dict|[[String, a]]) -> String -> a | None"
    );
    println!(
        "  {:<18} :: {}",
        "set", "(Dict|[[String, a]]) -> String -> a -> (Dict|[[String, a]])"
    );
    println!("  {:<18} :: {}", "keys", "(Dict|[[String, a]]) -> [String]");
    println!("  {:<18} :: {}", "values", "(Dict|[[String, a]]) -> [a]");
    println!(
        "  {:<18} :: {}",
        "has_key", "(Dict|[[String, a]]) -> String -> Bool"
    );
    println!("  {:<18} :: {}", "dict_merge", "Dict -> Dict -> Dict");
    println!();
    println!(
        "  Note: 'Pairs' is not a type - it's a list of 2-element lists: [[\"key\", value], ...]"
    );
    println!("  Modern: let config = {{host: \"localhost\", port: 8080}} in config.host");
    println!(
        "  Legacy: let pairs = [[\"host\", \"localhost\"], [\"port\", 8080]] in get pairs \"host\""
    );
    println!();

    // Type Conversion
    println!("Type Conversion:");
    println!("----------------");
    println!("  {:<18} :: {}", "to_string", "a -> String");
    println!("  {:<18} :: {}", "to_int", "String -> Int");
    println!("  {:<18} :: {}", "to_float", "String -> Float");
    println!("  {:<18} :: {}", "to_bool", "a -> Bool");
    println!(
        "  {:<18} :: {}",
        "to_char", "Int -> String  (codepoint to character)"
    );
    println!(
        "  {:<18} :: {}",
        "to_list", "String -> [String]  (string to char list)"
    );
    println!(
        "  {:<18} :: {}",
        "neg", "Number -> Number  (negate a number)"
    );
    println!();

    // Formatting Functions
    println!("Formatting Functions:");
    println!("---------------------");
    println!(
        "  {:<18} :: {}",
        "format_int", "Number -> Int -> String  (zero-padded integers)"
    );
    println!(
        "  {:<18} :: {}",
        "format_float", "Number -> Int -> String  (decimal precision)"
    );
    println!(
        "  {:<18} :: {}",
        "format_hex", "Number -> String  (hexadecimal)"
    );
    println!(
        "  {:<18} :: {}",
        "format_octal", "Number -> String  (octal)"
    );
    println!(
        "  {:<18} :: {}",
        "format_binary", "Number -> String  (binary)"
    );
    println!(
        "  {:<18} :: {}",
        "format_scientific", "Number -> Int -> String  (scientific notation)"
    );
    println!(
        "  {:<18} :: {}",
        "format_bytes", "Number -> String  (human-readable bytes)"
    );
    println!(
        "  {:<18} :: {}",
        "format_list", "[a] -> String -> String  (join with separator)"
    );
    println!(
        "  {:<18} :: {}",
        "format_table", "([[a]]|Dict) -> String -> String  (2D table)"
    );
    println!(
        "  {:<18} :: {}",
        "format_json", "a -> String  (JSON representation)"
    );
    println!(
        "  {:<18} :: {}",
        "format_currency", "Number -> String -> String  (currency)"
    );
    println!(
        "  {:<18} :: {}",
        "format_percent", "Number -> Int -> String  (percentage)"
    );
    println!(
        "  {:<18} :: {}",
        "format_bool", "Bool -> String -> String  (custom formatting)"
    );
    println!(
        "  {:<18} :: {}",
        "truncate", "String -> Int -> String  (truncate with ...)"
    );
    println!(
        "  {:<18} :: {}",
        "center", "String -> Int -> String  (center-align text)"
    );
    println!();

    // HTML Helpers
    println!("HTML Helpers:");
    println!("-------------");
    println!("  {:<18} :: {}", "html_escape", "String -> String");
    println!("  {:<18} :: {}", "html_tag", "String -> String -> String");
    println!("  {:<18} :: {}", "html_attr", "String -> String -> String");
    println!();

    // Markdown Helpers
    println!("Markdown Helpers:");
    println!("-----------------");
    println!("  {:<18} :: {}", "md_heading", "Int -> String -> String");
    println!("  {:<18} :: {}", "md_link", "String -> String -> String");
    println!("  {:<18} :: {}", "md_code", "String -> String");
    println!("  {:<18} :: {}", "md_list", "[String] -> String");
    println!(
        "  {:<18} :: {}",
        "markdown_to_html", "(String|Template) -> String"
    );
    println!();

    // File Operations
    println!("File Operations:");
    println!("----------------");
    println!("  {:<18} :: {}", "readfile", "String|Path -> String");
    println!("  {:<18} :: {}", "readlines", "String|Path -> [String]");
    println!(
        "  {:<18} :: {}",
        "fill_template", "String|Path -> (Dict|[[String, String]]) -> String"
    );
    println!("                     (reads file and fills {{placeholders}} with values)");
    println!("  {:<18} :: {}", "exists", "String|Path -> Bool");
    println!("  {:<18} :: {}", "basename", "String|Path -> String");
    println!("  {:<18} :: {}", "dirname", "String|Path -> String");
    println!("  {:<18} :: {}", "walkdir", "String|Path -> [String]");
    println!();
    println!("  Note: Path values are created with @ syntax: @config/{{env}}.yml");
    println!("        Paths can be stored in variables and passed to file functions.");
    println!();

    // Data Utilities
    println!("Data Utilities:");
    println!("---------------");
    println!("  {:<18} :: {}", "json_parse", "String -> (Dict|List|a)");
    println!("                     (Reads from file, returns Dict for objects, List for arrays)");
    println!("  {:<18} :: {}", "yaml_parse", "String -> (Dict|List|a)");
    println!(
        "                     (Reads from file, returns Dict for mappings, List for sequences)"
    );
    println!("  {:<18} :: {}", "toml_parse", "String -> (Dict|List|a)");
    println!("                     (Reads from file, returns Dict for tables, List for arrays)");
    println!("  {:<18} :: {}", "csv_parse", "String -> [Dict|[String]]");
    println!("                     (Reads from file, returns list of Dicts if headers, else list of lists)");
    println!("  {:<18} :: {}", "import", "String|Path -> Value");
    println!();
    println!("  Note: Parse functions (json_parse, yaml_parse, toml_parse, csv_parse) only read from files.");
    println!("        They do not parse strings directly. Pass a file path, not file content.");
    println!();

    // Type Checking & Introspection
    println!("Type Checking & Introspection:");
    println!("-------------------------------");
    println!(
        "  {:<18} :: {}",
        "typeof", "a -> String  (returns type name)"
    );
    println!("  {:<18} :: {}", "is_string", "a -> Bool");
    println!("  {:<18} :: {}", "is_number", "a -> Bool");
    println!("  {:<18} :: {}", "is_int", "a -> Bool");
    println!("  {:<18} :: {}", "is_float", "a -> Bool");
    println!("  {:<18} :: {}", "is_list", "a -> Bool");
    println!("  {:<18} :: {}", "is_bool", "a -> Bool");
    println!("  {:<18} :: {}", "is_function", "a -> Bool");
    println!(
        "  {:<18} :: {}",
        "is_none", "a -> Bool  (None from head [], get missing key, JSON null)"
    );
    println!("  {:<18} :: {}", "not", "Bool -> Bool  (logical negation)");
    println!(
        "  {:<18} :: {}",
        "assert", "Bool -> a -> a  (returns value if true, errors otherwise)"
    );
    println!();

    // Debug & Error Handling
    println!("Debug & Error Handling:");
    println!("-----------------------");
    println!(
        "  {:<18} :: {}",
        "trace", "String -> a -> a  (prints label: value to stderr)"
    );
    println!(
        "  {:<18} :: {}",
        "debug", "a -> a  (pretty-prints value structure)"
    );
    println!(
        "  {:<18} :: {}",
        "error", "String -> a  (throws custom error)"
    );
    println!();

    // System
    println!("Date/Time Operations:");
    println!("---------------------");
    println!("  {:<18} :: {}", "now", "String");
    println!(
        "  {:<18} :: {}",
        "date_format", "String -> String -> String"
    );
    println!("  {:<18} :: {}", "date_parse", "String -> String -> String");
    println!("  {:<18} :: {}", "date_add", "String -> String -> String");
    println!("  {:<18} :: {}", "date_diff", "String -> String -> Int");
    println!("  {:<18} :: {}", "timestamp", "Int");
    println!("  {:<18} :: {}", "timezone", "String");
    println!();
    println!("  Note: Date strings use ISO 8601 (RFC 3339) format: \"2024-03-15T14:30:00+00:00\"");
    println!("        Duration format: number + unit (s/m/h/d/w/y), e.g., \"1d\", \"2h\", \"30m\"");
    println!("        Format strings use strftime codes: %Y (year), %m (month), %d (day), etc.");
    println!();

    println!("System:");
    println!("-------");
    println!(
        "  {:<18} :: {}",
        "os", "String  (returns \"linux\", \"macos\", or \"windows\")"
    );
    println!(
        "  {:<18} :: {}",
        "env_var", "String -> String  (reads env var, fails if missing)"
    );
    println!(
        "  {:<18} :: {}",
        "env_var_or", "String -> String -> String  (reads env var with default)"
    );
    println!();

    println!("Notes:");
    println!("------");
    println!("  • All functions are curried and support partial application");
    println!("  • Type variables (a, b, acc) represent any type");
    println!("  • Functions use space-separated arguments: f x y, not f(x, y)");
    println!();
    println!("For more examples and tutorials, see: https://github.com/pyrotek45/avon");
}

pub fn print_help() {
    let help = r#"avon — evaluate and generate file templates

Usage: avon <command> [args]

Commands:
  eval <file>        Evaluate a file and print the result (no files written)
  deploy <file>      Deploy generated templates to disk
  run <code>         Evaluate code string directly
  repl               Start interactive REPL (Read-Eval-Print Loop)
  doc [name]         Show builtin function reference
  version            Show version information
  help               Show this help message

Documentation:
  avon doc                   List all builtin functions
  avon doc <function>        Show help for a specific function (e.g., avon doc map)
  
  In the REPL, use :doc with categories to browse functions by type:
    :doc string    Text manipulation (concat, upper, lower, split, join...)
    :doc list      List operations (map, filter, fold, head, tail, zip...)
    :doc math      Math functions (sqrt, pow, floor, ceil, abs, sum...)
    :doc dict      Dictionary operations (keys, values, get, set, merge...)
    :doc file      File system (read_file, write_file, glob, abspath...)
    :doc type      Type checking (is_string, is_number, is_list...)
    :doc format    Formatting (format_hex, format_json, format_table...)
    :doc date      Date/time (now, format_date, parse_date...)
    :doc regex     Regular expressions (regex_match, regex_replace...)

Note: You can omit 'eval' - 'avon <file>' is equivalent to 'avon eval <file>'
      Example: 'avon config.av' works the same as 'avon eval config.av'

Options:
  --root <dir>       Prepend <dir> to generated file paths (deploy only)
                     Default: If not specified, files are written relative to current working directory
                     Recommended: Always use --root to avoid writing to system directories
  
  --force            Overwrite existing files without warning (deploy only)
                     Use with caution: This will overwrite files without backup
  
  --append           Append to existing files instead of overwriting (deploy only)
                     Useful for logs or accumulating data
  
  --if-not-exists    Only write file if it doesn't exist (deploy only)
                     Useful for initialization files
  
  --backup           Create backup (.bak) of existing files before overwriting (deploy only)
                     Safest option: Preserves old files while allowing updates
  
  --git <url>        Use git raw URL as source file (for eval/deploy)
                     Format: user/repo/path/to/file.av
  
  --debug            Enable detailed debug output (lexer/parser/eval)
                     Useful for troubleshooting syntax or evaluation issues
  
  -param value       Pass named arguments to main function
                     Example: -env prod -version 1.0

Safety:
  By default, Avon will NOT overwrite existing files. It skips them and warns you.
  Use --force, --append, or --backup to explicitly allow file modifications.
  Always use --root to confine deployment to a specific directory.

Examples:
  # Evaluate a file (see what it produces) - these are equivalent:
  avon eval config.av
  avon config.av
  
  # Deploy to a specific directory
  avon deploy config.av --root ./output
  
  # Deploy with backup (safest)
  avon deploy config.av --root ./output --backup
  
  # Deploy with arguments
  avon deploy config.av --root ./output -env prod -version 1.0
  
  # Evaluate code directly
  avon run 'map (\x x*2) [1,2,3]'
  
  # Fetch and deploy from GitHub
  avon deploy --git user/repo/file.av --root ./out
  
  # Start interactive REPL
  avon repl
  
  # Debug a problematic file
  avon eval config.av --debug

For more information, see: https://github.com/pyrotek45/avon
"#;
    println!("{}", help);
}
