use crate::common::{Number, Value};
use crate::eval::{apply_function, collect_file_templates, eval, fetch_git_raw, initial_builtins};
use crate::lexer::tokenize;
use crate::parser::parse;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, Context, EditMode, Editor};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Get documentation for a REPL command
fn get_repl_command_doc(cmd_name: &str) -> Option<String> {
    let docs: std::collections::HashMap<&str, &str> = [
        ("help", ":help, :h\n  Show this help message with all available REPL commands."),
        ("h", ":help, :h\n  Show this help message with all available REPL commands."),
        ("exit", ":exit, :quit, :q\n  Exit the REPL."),
        ("quit", ":exit, :quit, :q\n  Exit the REPL."),
        ("q", ":exit, :quit, :q\n  Exit the REPL."),
        ("clear", ":clear\n  Clear all user-defined variables. Builtin functions are preserved."),
        ("vars", ":vars\n  List all user-defined variables (excluding builtin functions).\n  Example: :vars"),
        ("let", ":let <name> = <expr>\n  Store a value in a variable for later use.\n  Example: :let x = 42\n  Example: :let config = {host: \"localhost\", port: 8080}"),
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

// Get documentation for a specific builtin function
fn get_builtin_doc(func_name: &str) -> Option<String> {
    let docs: std::collections::HashMap<&str, &str> = [
        // String Operations
        ("concat", "concat :: String -> String -> String\n  Concatenate two strings.\n  Example: concat \"hello\" \" world\" -> \"hello world\""),
        ("upper", "upper :: String -> String\n  Convert string to uppercase.\n  Example: upper \"hello\" -> \"HELLO\""),
        ("lower", "lower :: String -> String\n  Convert string to lowercase.\n  Example: lower \"HELLO\" -> \"hello\""),
        ("trim", "trim :: String -> String\n  Remove leading and trailing whitespace.\n  Example: trim \"  hello  \" -> \"hello\""),
        ("split", "split :: String -> String -> [String]\n  Split string by delimiter.\n  Example: split \"a,b,c\" \",\" -> [\"a\", \"b\", \"c\"]"),
        ("join", "join :: [String] -> String -> String\n  Join list of strings with separator.\n  Example: join [\"a\", \"b\"] \", \" -> \"a, b\""),
        ("replace", "replace :: String -> String -> String -> String\n  Replace all occurrences of substring.\n  Example: replace \"hello\" \"l\" \"L\" -> \"heLLo\""),
        ("contains", "contains :: String -> String -> Bool\n  Check if string contains substring.\n  Example: contains \"hello\" \"ell\" -> true"),
        ("starts_with", "starts_with :: String -> String -> Bool\n  Check if string starts with prefix.\n  Example: starts_with \"hello\" \"he\" -> true"),
        ("ends_with", "ends_with :: String -> String -> Bool\n  Check if string ends with suffix.\n  Example: ends_with \"hello\" \"lo\" -> true"),
        ("length", "length :: (String|List) -> Int\n  Get length of string or list.\n  Example: length \"hello\" -> 5, length [1,2,3] -> 3"),
        ("repeat", "repeat :: String -> Int -> String\n  Repeat string n times.\n  Example: repeat \"x\" 3 -> \"xxx\""),
        ("pad_left", "pad_left :: String -> Int -> String -> String\n  Pad string on left to specified length.\n  Example: pad_left \"7\" 3 \"0\" -> \"007\""),
        ("pad_right", "pad_right :: String -> Int -> String -> String\n  Pad string on right to specified length.\n  Example: pad_right \"hi\" 5 \" \" -> \"hi   \""),
        ("indent", "indent :: String -> Int -> String\n  Indent each line by n spaces.\n  Example: indent \"code\" 4 -> \"    code\""),
        ("is_digit", "is_digit :: String -> Bool\n  Check if all characters are digits.\n  Example: is_digit \"123\" -> true"),
        ("is_alpha", "is_alpha :: String -> Bool\n  Check if all characters are alphabetic.\n  Example: is_alpha \"abc\" -> true"),
        ("is_alphanumeric", "is_alphanumeric :: String -> Bool\n  Check if all characters are alphanumeric.\n  Example: is_alphanumeric \"abc123\" -> true"),
        ("is_whitespace", "is_whitespace :: String -> Bool\n  Check if all characters are whitespace.\n  Example: is_whitespace \"  \" -> true"),
        ("is_uppercase", "is_uppercase :: String -> Bool\n  Check if all characters are uppercase.\n  Example: is_uppercase \"ABC\" -> true"),
        ("is_lowercase", "is_lowercase :: String -> Bool\n  Check if all characters are lowercase.\n  Example: is_lowercase \"abc\" -> true"),
        ("is_empty", "is_empty :: (String|List) -> Bool\n  Check if string or list is empty.\n  Example: is_empty \"\" -> true, is_empty [] -> true"),

        // List Operations
        ("map", "map :: (a -> b) -> [a] -> [b]\n  Transform each item in list.\n  Example: map (\\x x * 2) [1, 2, 3] -> [2, 4, 6]"),
        ("filter", "filter :: (a -> Bool) -> [a] -> [a]\n  Keep items matching predicate.\n  Example: filter (\\x x > 2) [1, 2, 3, 4] -> [3, 4]"),
        ("fold", "fold :: (acc -> a -> acc) -> acc -> [a] -> acc\n  Reduce list to single value.\n  Example: fold (\\a \\x a + x) 0 [1, 2, 3] -> 6"),
        ("flatmap", "flatmap :: (a -> [b]) -> [a] -> [b]\n  Map then flatten one level.\n  Example: flatmap (\\x [x, x]) [1, 2] -> [1, 1, 2, 2]"),
        ("flatten", "flatten :: [[a]] -> [a]\n  Flatten nested list one level.\n  Example: flatten [[1,2], [3]] -> [1, 2, 3]"),
        ("zip", "zip :: [a] -> [b] -> [(a, b)]\n  Combine two lists into pairs.\n  Example: zip [1, 2] [\"a\", \"b\"] -> [(1, \"a\"), (2, \"b\")]"),
        ("unzip", "unzip :: [(a, b)] -> ([a], [b])\n  Split list of pairs into two lists.\n  Example: unzip [(1, \"a\"), (2, \"b\")] -> ([1, 2], [\"a\", \"b\"])"),
        ("take", "take :: Int -> [a] -> [a]\n  Take first n items.\n  Example: take 2 [1, 2, 3, 4] -> [1, 2]"),
        ("drop", "drop :: Int -> [a] -> [a]\n  Drop first n items.\n  Example: drop 2 [1, 2, 3, 4] -> [3, 4]"),
        ("split_at", "split_at :: Int -> [a] -> ([a], [a])\n  Split list at index.\n  Example: split_at 2 [1, 2, 3, 4] -> ([1, 2], [3, 4])"),
        ("partition", "partition :: (a -> Bool) -> [a] -> ([a], [a])\n  Split list into matching and non-matching.\n  Example: partition (\\x x > 2) [1, 2, 3, 4] -> ([3, 4], [1, 2])"),
        ("reverse", "reverse :: [a] -> [a]\n  Reverse list order.\n  Example: reverse [1, 2, 3] -> [3, 2, 1]"),
        ("head", "head :: [a] -> a | None\n  Get first item or None.\n  Example: head [1, 2, 3] -> 1"),
        ("tail", "tail :: [a] -> [a]\n  Get all items except first.\n  Example: tail [1, 2, 3] -> [2, 3]"),

        // Dict Operations
        ("get", "get :: (Dict|Pairs) -> String -> a | None\n  Get value by key.\n  Example: get {a: 1} \"a\" -> 1"),
        ("set", "set :: (Dict|Pairs) -> String -> a -> (Dict|Pairs)\n  Update or add key-value pair.\n  Example: set {a: 1} \"b\" 2 -> {a: 1, b: 2}"),
        ("keys", "keys :: (Dict|Pairs) -> [String]\n  Get all keys.\n  Example: keys {a: 1, b: 2} -> [\"a\", \"b\"]"),
        ("values", "values :: (Dict|Pairs) -> [a]\n  Get all values.\n  Example: values {a: 1, b: 2} -> [1, 2]"),
        ("has_key", "has_key :: (Dict|Pairs) -> String -> Bool\n  Check if key exists.\n  Example: has_key {a: 1} \"a\" -> true"),

        // Type Conversion
        ("to_string", "to_string :: a -> String\n  Convert value to string.\n  Example: to_string 42 -> \"42\""),
        ("to_int", "to_int :: String -> Int\n  Convert string to integer.\n  Example: to_int \"42\" -> 42"),
        ("to_float", "to_float :: String -> Float\n  Convert string to float.\n  Example: to_float \"3.14\" -> 3.14"),
        ("to_bool", "to_bool :: a -> Bool\n  Convert value to boolean.\n  Example: to_bool \"yes\" -> true"),
        ("neg", "neg :: Number -> Number\n  Negate a number.\n  Example: neg 5 -> -5"),

        // Formatting
        ("format_int", "format_int :: Number -> Int -> String\n  Format integer with zero-padding.\n  Example: format_int 7 3 -> \"007\""),
        ("format_float", "format_float :: Number -> Int -> String\n  Format float with decimal precision.\n  Example: format_float 3.14159 2 -> \"3.14\""),
        ("format_hex", "format_hex :: Number -> String\n  Format number as hexadecimal.\n  Example: format_hex 255 -> \"ff\""),
        ("format_octal", "format_octal :: Number -> String\n  Format number as octal.\n  Example: format_octal 64 -> \"100\""),
        ("format_binary", "format_binary :: Number -> String\n  Format number as binary.\n  Example: format_binary 15 -> \"1111\""),
        ("format_scientific", "format_scientific :: Number -> Int -> String\n  Format number in scientific notation.\n  Example: format_scientific 12345 2 -> \"1.23e4\""),
        ("format_bytes", "format_bytes :: Number -> String\n  Format bytes as human-readable size.\n  Example: format_bytes 1536000 -> \"1.46 MB\""),
        ("format_list", "format_list :: [a] -> String -> String\n  Join list with separator.\n  Example: format_list [\"a\", \"b\"] \", \" -> \"a, b\""),
        ("format_table", "format_table :: ([[a]]|Dict) -> String -> String\n  Format as 2D table.\n  Example: format_table [[\"A\", \"B\"], [\"1\", \"2\"]] \" | \" -> \"A | B\\n1 | 2\""),
        ("format_json", "format_json :: a -> String\n  Format value as JSON.\n  Example: format_json [1, 2, 3] -> \"[1, 2, 3]\""),
        ("format_currency", "format_currency :: Number -> String -> String\n  Format number as currency.\n  Example: format_currency 19.99 \"$\" -> \"$19.99\""),
        ("format_percent", "format_percent :: Number -> Int -> String\n  Format number as percentage.\n  Example: format_percent 0.856 2 -> \"85.60%\""),
        ("format_bool", "format_bool :: Bool -> String -> String\n  Format boolean with custom text.\n  Example: format_bool true \"yes/no\" -> \"Yes\""),
        ("truncate", "truncate :: String -> Int -> String\n  Truncate string with ellipsis.\n  Example: truncate \"Long text\" 8 -> \"Long ...\""),
        ("center", "center :: String -> Int -> String\n  Center-align text.\n  Example: center \"Hi\" 10 -> \"    Hi    \""),

        // HTML
        ("html_escape", "html_escape :: String -> String\n  Escape HTML special characters.\n  Example: html_escape \"<div>\" -> \"&lt;div&gt;\""),
        ("html_tag", "html_tag :: String -> String -> String\n  Create HTML tag.\n  Example: html_tag \"p\" \"text\" -> \"<p>text</p>\""),
        ("html_attr", "html_attr :: String -> String -> String\n  Create HTML attribute.\n  Example: html_attr \"class\" \"btn\" -> \"class=\\\"btn\\\"\""),

        // Markdown
        ("md_heading", "md_heading :: Int -> String -> String\n  Create markdown heading.\n  Example: md_heading 1 \"Title\" -> \"# Title\""),
        ("md_link", "md_link :: String -> String -> String\n  Create markdown link.\n  Example: md_link \"text\" \"url\" -> \"[text](url)\""),
        ("md_code", "md_code :: String -> String\n  Create inline code.\n  Example: md_code \"x = 1\" -> \"`x = 1`\""),
        ("md_list", "md_list :: [String] -> String\n  Create markdown list.\n  Example: md_list [\"a\", \"b\"] -> \"- a\\n- b\""),

        // File Operations
        ("readfile", "readfile :: String|Path -> String\n  Read entire file as string.\n  Example: readfile \"config.json\" -> file contents\n  Note: Use relative paths only. Absolute paths blocked for security."),
        ("readlines", "readlines :: String|Path -> [String]\n  Read file as list of lines.\n  Example: readlines \"file.txt\" -> [\"line1\", \"line2\"]"),
        ("fill_template", "fill_template :: String|Path -> (Dict|[[String, String]]) -> String\n  Read file and fill {{placeholders}} with values.\n  Example: fill_template \"template.txt\" {name: \"Alice\"} -> filled template"),
        ("exists", "exists :: String|Path -> Bool\n  Check if file exists.\n  Example: exists \"config.json\" -> true"),
        ("basename", "basename :: String|Path -> String\n  Get filename from path.\n  Example: basename @config/app.yml -> \"app.yml\""),
        ("dirname", "dirname :: String|Path -> String\n  Get directory from path.\n  Example: dirname @config/app.yml -> \"config\""),
        ("walkdir", "walkdir :: String|Path -> [String]\n  List all files in directory recursively.\n  Example: walkdir \"src\" -> [\"src/file1.txt\", \"src/file2.txt\"]"),

        // Data Utilities
        ("json_parse", "json_parse :: String -> a\n  Parse JSON string (objects -> dicts, arrays -> lists).\n  Example: json_parse '{\"a\": 1}' -> {a: 1}"),
        ("import", "import :: String|Path -> Value\n  Import and evaluate another Avon file.\n  Example: import \"lib.av\" -> value from lib.av\n  Note: Use relative paths only. Absolute paths blocked for security."),

        // Type Checking
        ("typeof", "typeof :: a -> String\n  Get type name as string.\n  Example: typeof 42 -> \"Number\""),
        ("is_string", "is_string :: a -> Bool\n  Check if value is string.\n  Example: is_string \"hello\" -> true"),
        ("is_number", "is_number :: a -> Bool\n  Check if value is number.\n  Example: is_number 42 -> true"),
        ("is_int", "is_int :: a -> Bool\n  Check if value is integer.\n  Example: is_int 42 -> true"),
        ("is_float", "is_float :: a -> Bool\n  Check if value is float.\n  Example: is_float 3.14 -> true"),
        ("is_list", "is_list :: a -> Bool\n  Check if value is list.\n  Example: is_list [1, 2] -> true"),
        ("is_bool", "is_bool :: a -> Bool\n  Check if value is boolean.\n  Example: is_bool true -> true"),
        ("is_function", "is_function :: a -> Bool\n  Check if value is function.\n  Example: is_function (\\x x) -> true"),
        ("is_dict", "is_dict :: a -> Bool\n  Check if value is dictionary.\n  Example: is_dict {a: 1} -> true"),

        // Assert & Debug
        ("assert", "assert :: Bool -> a -> a\n  Assert condition, return value or error with debug info.\n  Example: assert (is_number x) x\n  Example: assert (x > 0) x\n  Use for input validation and type checking."),
        ("trace", "trace :: String -> a -> a\n  Print label and value to stderr, return value unchanged.\n  Example: trace \"x\" 42 -> prints \"[TRACE] x: 42\" to stderr, returns 42"),
        ("debug", "debug :: a -> a\n  Pretty-print value structure to stderr, return value unchanged.\n  Example: debug [1, 2, 3] -> prints structure, returns [1, 2, 3]"),
        ("error", "error :: String -> a\n  Throw custom error with message.\n  Example: error \"Invalid input\" -> throws error"),

        // System
        ("os", "os :: String\n  Get operating system name.\n  Returns: \"linux\", \"macos\", or \"windows\""),
        ("env_var", "env_var :: String -> String\n  Read environment variable, fail if missing.\n  Example: env_var \"HOME\" -> \"/home/user\"\n  Fails if variable not set (fail-safe behavior)."),
        ("env_var_or", "env_var_or :: String -> String -> String\n  Read environment variable with default.\n  Example: env_var_or \"PORT\" \"8080\" -> env value or \"8080\""),
    ]
    .iter()
    .cloned()
    .collect();

    docs.get(func_name).map(|s| s.to_string())
}

// Custom completer for tab completion
struct AvonCompleter {
    file_completer: FilenameCompleter,
    symbols: Rc<RefCell<HashMap<String, Value>>>,
}

impl Completer for AvonCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        // If line starts with ':', complete REPL commands
        if line.trim_start().starts_with(':') {
            let commands = vec![
                "help",
                "h",
                "exit",
                "quit",
                "q",
                "clear",
                "vars",
                "let",
                "inspect",
                "unlet",
                "read",
                "run",
                "eval",
                "preview",
                "deploy",
                "deploy-expr",
                "write",
                "history",
                "save-session",
                "load-session",
                "assert",
                "test",
                "benchmark",
                "benchmark-file",
                "watch",
                "unwatch",
                "pwd",
                "list",
                "cd",
                "doc",
                "type",
                "sh",
            ];

            let prefix = line[..pos].trim_start();
            let start = prefix.rfind(':').unwrap_or(0);
            let search = &prefix[start + 1..];

            let matches: Vec<Pair> = commands
                .iter()
                .filter(|cmd| cmd.starts_with(search))
                .map(|cmd| Pair {
                    display: cmd.to_string(),
                    replacement: format!(":{}", cmd),
                })
                .collect();

            if !matches.is_empty() {
                return Ok((start, matches));
            }
        }

        // Get current word being completed
        let word_start = line[..pos]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != ':')
            .map(|i| i + 1)
            .unwrap_or(0);

        let search = &line[word_start..pos];

        // Complete builtin functions
        let builtins = initial_builtins();
        let builtin_names: Vec<&str> = builtins.keys().map(|s| s.as_str()).collect();

        let mut matches: Vec<Pair> = builtin_names
            .iter()
            .filter(|name| name.starts_with(search))
            .map(|name| Pair {
                display: name.to_string(),
                replacement: name.to_string(),
            })
            .collect();

        // Complete user-defined variables (excluding builtins)
        if !search.is_empty() && word_start < pos {
            let symbols = self.symbols.borrow();
            let builtins = initial_builtins();
            let var_names: Vec<String> = symbols
                .keys()
                .filter(|name| !builtins.contains_key(*name) && name.starts_with(search))
                .cloned()
                .collect();

            for var_name in var_names {
                matches.push(Pair {
                    display: var_name.clone(),
                    replacement: var_name,
                });
            }
        }

        if !matches.is_empty() {
            return Ok((word_start, matches));
        }

        // Fall back to filename completion
        self.file_completer.complete(line, pos, ctx)
    }
}

fn print_builtin_docs() {
    println!("Avon Builtin Functions Reference");
    println!("=================================\n");

    // String Operations
    println!("String Operations:");
    println!("------------------");
    println!("  concat       :: String -> String -> String");
    println!("  upper        :: String -> String");
    println!("  lower        :: String -> String");
    println!("  trim         :: String -> String");
    println!("  split        :: String -> String -> [String]");
    println!("  join         :: [String] -> String -> String");
    println!("  replace      :: String -> String -> String -> String");
    println!("  contains     :: String -> String -> Bool");
    println!("  starts_with  :: String -> String -> Bool");
    println!("  ends_with    :: String -> String -> Bool");
    println!("  length       :: String -> Int  (also works on lists)");
    println!("  repeat       :: String -> Int -> String");
    println!("  pad_left     :: String -> Int -> String -> String");
    println!("  pad_right    :: String -> Int -> String -> String");
    println!("  indent       :: String -> Int -> String");
    println!("  is_digit     :: String -> Bool");
    println!("  is_alpha     :: String -> Bool");
    println!("  is_alphanumeric :: String -> Bool");
    println!("  is_whitespace :: String -> Bool");
    println!("  is_uppercase :: String -> Bool");
    println!("  is_lowercase :: String -> Bool");
    println!("  is_empty     :: String -> Bool  (also works on lists)");
    println!();

    // List Operations
    println!("List Operations:");
    println!("----------------");
    println!("  map          :: (a -> b) -> [a] -> [b]");
    println!("  filter       :: (a -> Bool) -> [a] -> [a]");
    println!("  fold         :: (acc -> a -> acc) -> acc -> [a] -> acc");
    println!("  flatmap      :: (a -> [b]) -> [a] -> [b]");
    println!("  flatten      :: [[a]] -> [a]");
    println!("  length       :: [a] -> Int  (also works on strings)");
    println!("  zip          :: [a] -> [b] -> [(a, b)]");
    println!("  unzip        :: [(a, b)] -> ([a], [b])");
    println!("  take         :: Int -> [a] -> [a]");
    println!("  drop         :: Int -> [a] -> [a]");
    println!("  split_at     :: Int -> [a] -> ([a], [a])");
    println!("  partition    :: (a -> Bool) -> [a] -> ([a], [a])");
    println!("  reverse      :: [a] -> [a]");
    println!("  head         :: [a] -> a | None");
    println!("  tail         :: [a] -> [a]");
    println!();

    // Map/Dictionary Operations
    println!("Map/Dictionary Operations:");
    println!("--------------------------");
    println!("  dict_get     :: Dict -> String -> a | None       (get value by key - deprecated, use dot notation)");
    println!("  get          :: (Dict|Pairs) -> String -> a | None");
    println!("  set          :: (Dict|Pairs) -> String -> a -> (Dict|Pairs)");
    println!(
        "  keys         :: (Dict|Pairs) -> [String]         (works with both dict and list pairs)"
    );
    println!(
        "  values       :: (Dict|Pairs) -> [a]              (works with both dict and list pairs)"
    );
    println!("  has_key      :: (Dict|Pairs) -> String -> Bool");
    println!();
    println!("  Modern syntax: let config = {{host: \"localhost\", port: 8080}} in");
    println!("                 config.host  # Access with dot notation!");
    println!("  Tip: Use keys/values/length with dicts and pairs for generic dict operations");
    println!("  Legacy: get/set/keys/values/has_key work with both dicts and [[k,v]] pairs");
    println!();

    // Type Conversion
    println!("Type Conversion:");
    println!("----------------");
    println!("  to_string    :: a -> String");
    println!("  to_int       :: String -> Int");
    println!("  to_float     :: String -> Float");
    println!("  to_bool      :: a -> Bool");
    println!("  neg          :: Number -> Number                      (negate a number)");
    println!();

    // Formatting Functions
    println!("Formatting Functions:");
    println!("---------------------");
    println!("  format_int        :: Number -> Int -> String          (zero-padded integers)");
    println!("  format_float      :: Number -> Int -> String          (decimal precision)");
    println!("  format_hex        :: Number -> String                 (hexadecimal)");
    println!("  format_octal      :: Number -> String                 (octal)");
    println!("  format_binary     :: Number -> String                 (binary)");
    println!("  format_scientific :: Number -> Int -> String          (scientific notation)");
    println!("  format_bytes      :: Number -> String                 (human-readable bytes)");
    println!("  format_list       :: [a] -> String -> String          (join with separator)");
    println!(
        "  format_table      :: ([[a]]|Dict) -> String -> String (2D table, also accepts dict)"
    );
    println!("  format_json       :: a -> String                      (JSON representation)");
    println!("  format_currency   :: Number -> String -> String       (currency with symbol)");
    println!("  format_percent    :: Number -> Int -> String          (percentage)");
    println!("  format_bool       :: Bool -> String -> String         (custom bool formatting)");
    println!("  truncate          :: String -> Int -> String          (truncate with ...)");
    println!("  center            :: String -> Int -> String          (center-align text)");
    println!();

    // HTML Helpers
    println!("HTML Helpers:");
    println!("-------------");
    println!("  html_escape  :: String -> String");
    println!("  html_tag     :: String -> String -> String");
    println!("  html_attr    :: String -> String -> String");
    println!();

    // Markdown Helpers
    println!("Markdown Helpers:");
    println!("-----------------");
    println!("  md_heading   :: Int -> String -> String");
    println!("  md_link      :: String -> String -> String");
    println!("  md_code      :: String -> String");
    println!("  md_list      :: [String] -> String");
    println!();

    // File Operations
    println!("File Operations:");
    println!("----------------");
    println!("  readfile        :: String|Path -> String");
    println!("  readlines       :: String|Path -> [String]");
    println!("  fill_template   :: String|Path -> (Dict|[[String, String]]) -> String");
    println!("                     (reads file and fills {{placeholders}} with values)");
    println!("  exists          :: String|Path -> Bool");
    println!("  basename        :: String|Path -> String");
    println!("  dirname         :: String|Path -> String");
    println!("  walkdir         :: String|Path -> [String]");
    println!();
    println!("  Note: Path values are created with @ syntax: @config/{{env}}.yml");
    println!("        Paths can be stored in variables and passed to file functions.");
    println!();

    // Data Utilities
    println!("Data Utilities:");
    println!("---------------");
    println!("  json_parse   :: String -> a                       (JSON arrays → lists, objects → dicts)");
    println!("  import       :: String|Path -> Value");
    println!();

    // Type Checking & Introspection
    println!("Type Checking & Introspection:");
    println!("-------------------------------");
    println!("  typeof       :: a -> String                       (returns type name: \"String\", \"Number\", \"List\", etc.)");
    println!("  is_string    :: a -> Bool");
    println!("  is_number    :: a -> Bool");
    println!("  is_int       :: a -> Bool");
    println!("  is_float     :: a -> Bool");
    println!("  is_list      :: a -> Bool");
    println!("  is_bool      :: a -> Bool");
    println!("  is_function  :: a -> Bool");
    println!();
    println!("  assert       :: Bool -> a -> a                   (returns value if condition true, errors with debug info otherwise)");
    println!();
    println!("  Usage Examples:");
    println!("    assert (is_number x) x              # Assert x is a number, return x");
    println!("    assert (x > 0) x                    # Assert x is positive, return x");
    println!("    assert (is_string s) s              # Assert s is a string, return s");
    println!();

    // Debug & Error Handling
    println!("Debug & Error Handling:");
    println!("-----------------------");
    println!("  trace        :: String -> a -> a                 (prints \"label: value\" to stderr, returns value)");
    println!("  debug        :: a -> a                           (pretty-prints value structure, returns value)");
    println!(
        "  error        :: String -> a                      (throws custom error with message)"
    );
    println!();
    println!("  Examples:");
    println!("    trace \"x\" 42                        # Prints \"x: 42\" to stderr, returns 42");
    println!(
        "    debug [1, 2, 3]                     # Prints pretty list structure, returns [1, 2, 3]"
    );
    println!("    if (x < 0) then error \"negative\"    # Throws error if x is negative");
    println!();

    // System
    println!("System:");
    println!("-------");
    println!("  os           :: String  (returns \"linux\", \"macos\", or \"windows\")");
    println!("  env_var      :: String -> String  (reads env var, fails if missing)");
    println!("  env_var_or   :: String -> String -> String  (reads env var with default)");
    println!();

    println!("Notes:");
    println!("------");
    println!("  • All functions are curried and support partial application");
    println!("  • Type variables (a, b, acc) represent any type");
    println!("  • Functions use space-separated arguments: f x y, not f(x, y)");
    println!();
    println!("For more examples and tutorials, see: https://github.com/pyrotek45/avon");
}

fn print_help() {
    let help = r#"avon — evaluate and generate file templates

Usage: avon <command> [args]

Commands:
  eval <file>        Evaluate a file and print the result (no files written)
  deploy <file>      Deploy generated templates to disk
  run <code>         Evaluate code string directly
  repl               Start interactive REPL (Read-Eval-Print Loop)
  doc                Show builtin function reference
  version            Show version information
  help               Show this help message

Note: You can omit 'eval' - 'avon <file>' is equivalent to 'avon eval <file>'
      Example: 'avon config.av' works the same as 'avon eval config.av'

Options:
  --root <dir>       Prepend <dir> to generated file paths (deploy only)
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

#[derive(Debug)]
struct CliOptions {
    root: Option<String>,
    force: bool,
    append: bool,
    if_not_exists: bool,
    backup: bool,
    debug: bool,
    git_url: Option<String>,
    named_args: HashMap<String, String>,
    pos_args: Vec<String>,
    file: Option<String>,
    code: Option<String>,
}

impl CliOptions {
    fn new() -> Self {
        Self {
            root: None,
            force: false,
            append: false,
            if_not_exists: false,
            backup: false,
            debug: false,
            git_url: None,
            named_args: HashMap::new(),
            pos_args: Vec::new(),
            file: None,
            code: None,
        }
    }
}

fn parse_args(args: &[String], require_file: bool) -> Result<CliOptions, String> {
    let mut opts = CliOptions::new();
    let mut i = 0;

    // First arg might be file if not flag
    if require_file && i < args.len() && !args[i].starts_with("-") {
        opts.file = Some(args[i].clone());
        i += 1;
    }

    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                if i + 1 < args.len() {
                    opts.root = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("--root requires a directory argument".to_string());
                }
            }
            "--force" => {
                opts.force = true;
                i += 1;
            }
            "--append" => {
                opts.append = true;
                i += 1;
            }
            "--if-not-exists" => {
                opts.if_not_exists = true;
                i += 1;
            }
            "--backup" => {
                opts.backup = true;
                i += 1;
            }
            "--debug" => {
                opts.debug = true;
                i += 1;
            }
            "--git" => {
                if i + 1 < args.len() {
                    opts.git_url = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err(
                        "--git requires a URL argument (format: user/repo/path/to/file.av)"
                            .to_string(),
                    );
                }
            }
            s if s.starts_with("-") => {
                let key = s.trim_start_matches('-').to_string();
                if i + 1 < args.len() {
                    opts.named_args.insert(key, args[i + 1].clone());
                    i += 2;
                } else {
                    return Err(format!(
                        "Named argument '{}' requires a value. Use: -{} <value>",
                        key, key
                    ));
                }
            }
            s => {
                // If we didn't get a file yet and require one, treat first non-flag as file
                // This handles `avon eval --debug file.av` case
                if require_file && opts.file.is_none() && opts.git_url.is_none() {
                    opts.file = Some(s.to_string());
                } else {
                    opts.pos_args.push(s.to_string());
                }
                i += 1;
            }
        }
    }

    if require_file && opts.file.is_none() && opts.git_url.is_none() {
        return Err(
            "Missing required file argument. Use: avon <command> <file> [options]".to_string(),
        );
    }

    Ok(opts)
}

pub fn run_cli(args: Vec<String>) -> i32 {
    if args.len() < 2 {
        print_help();
        return 0;
    }

    let cmd = &args[1];
    let rest = if args.len() > 2 { &args[2..] } else { &[] };

    match cmd.as_str() {
        "eval" => match parse_args(rest, true) {
            Ok(opts) => execute_eval(opts),
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!("  Usage: avon eval <file> [options]");
                eprintln!("  Example: avon eval config.av");
                eprintln!("  Use 'avon help' for more information");
                1
            }
        },
        "deploy" => match parse_args(rest, true) {
            Ok(opts) => execute_deploy(opts),
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!("  Usage: avon deploy <file> [options]");
                eprintln!("  Example: avon deploy config.av --root ./output");
                eprintln!("  Use 'avon help' for more information");
                1
            }
        },
        "run" => {
            if rest.is_empty() {
                eprintln!("Error: 'run' command requires a code string argument");
                eprintln!("  Usage: avon run '<code>'");
                eprintln!("  Example: avon run 'map (\\x x*2) [1,2,3]'");
                eprintln!("  Note: Use quotes around the code string");
                return 1;
            }
            let code = rest[0].clone();
            // parse remaining flags like --debug
            match parse_args(&rest[1..], false) {
                Ok(mut opts) => {
                    opts.code = Some(code);
                    execute_run(opts)
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    1
                }
            }
        }
        "repl" => execute_repl(),
        "doc" | "docs" => {
            print_builtin_docs();
            0
        }
        "version" | "--version" | "-v" => {
            println!("avon 0.1.0");
            0
        }
        "help" | "--help" | "-h" => {
            print_help();
            0
        }
        // Legacy / Convenience
        _ => {
            // If starts with --, it's likely a legacy flag command
            if cmd.starts_with("--") {
                match cmd.as_str() {
                    "--git" => {
                        // Legacy --git implies deploy
                        let mut legacy_args = vec!["--git".to_string()];
                        legacy_args.extend_from_slice(rest);
                        match parse_args(&legacy_args, true) {
                            // require_file=true satisfied by --git
                            Ok(opts) => execute_deploy(opts),
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                1
                            }
                        }
                    }
                    "--git-eval" => {
                        let mut legacy_args = vec!["--git".to_string()]; // map to git opt
                        legacy_args.extend_from_slice(rest);
                        match parse_args(&legacy_args, true) {
                            Ok(opts) => execute_eval(opts),
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                1
                            }
                        }
                    }
                    "--eval-input" => {
                        if rest.is_empty() {
                            eprintln!("Error: --eval-input requires code string");
                            return 1;
                        }
                        let code = rest[0].clone();
                        match parse_args(&rest[1..], false) {
                            Ok(mut opts) => {
                                opts.code = Some(code);
                                execute_run(opts)
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                1
                            }
                        }
                    }
                    "--doc" => {
                        print_builtin_docs();
                        0
                    }
                    _ => {
                        eprintln!("Unknown flag: {}", cmd);
                        print_help();
                        1
                    }
                }
            } else {
                // Fallback: avon <file> [args]
                // Check if --deploy exists in args to decide mode
                let is_deploy = args.contains(&"--deploy".to_string());
                // Filter out --deploy from args for parsing
                let filtered_rest: Vec<String> =
                    rest.iter().filter(|s| *s != "--deploy").cloned().collect();

                // We treat the command as the file
                let mut eff_args = vec![cmd.clone()];
                eff_args.extend(filtered_rest);

                match parse_args(&eff_args, true) {
                    Ok(opts) => {
                        if is_deploy {
                            execute_deploy(opts)
                        } else {
                            execute_eval(opts)
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        1
                    }
                }
            }
        }
    }
}

fn get_source(opts: &CliOptions) -> Result<(String, String), i32> {
    if let Some(url) = &opts.git_url {
        fetch_git_raw(url).map(|s| (s, url.clone())).map_err(|e| {
            eprintln!("Error: Failed to fetch from git URL: {}", e.message);
            eprintln!("  URL: {}", url);
            eprintln!("  Tip: Make sure the URL format is: user/repo/path/to/file.av");
            eprintln!("  Example: avon deploy --git pyrotek45/avon/examples/config.av");
            1
        })
    } else if let Some(file) = &opts.file {
        std::fs::read_to_string(file)
            .map(|s| (s, file.clone()))
            .map_err(|e| {
                eprintln!("Error: Failed to read file: {}", file);
                eprintln!("  Reason: {}", e);
                if e.kind() == std::io::ErrorKind::NotFound {
                    eprintln!("  Tip: Check that the file exists and the path is correct");
                    eprintln!(
                        "  Tip: Use 'avon eval {}' to test if the file is valid",
                        file
                    );
                } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                    eprintln!("  Tip: Check file permissions");
                }
                1
            })
    } else {
        eprintln!("Error: No source file provided");
        eprintln!("  Usage: avon <command> <file> [options]");
        eprintln!("  Example: avon eval config.av");
        eprintln!("  Example: avon deploy config.av --root ./output");
        eprintln!("  Use 'avon help' for more information");
        Err(1)
    }
}

fn process_source(source: String, source_name: String, opts: CliOptions, deploy_mode: bool) -> i32 {
    if opts.debug {
        eprintln!("[DEBUG] Starting lexer...");
    }

    match tokenize(source.clone()) {
        Ok(tokens) => {
            if opts.debug {
                eprintln!("[DEBUG] Lexer produced {} tokens", tokens.len());
                for (i, tok) in tokens.iter().enumerate() {
                    eprintln!("[DEBUG]   Token {}: {:?}", i, tok);
                }
                eprintln!("[DEBUG] Starting parser...");
            }

            let ast = parse(tokens);
            if opts.debug {
                eprintln!("[DEBUG] Parser produced AST: {:?}", ast);
                eprintln!("[DEBUG] Starting evaluator...");
            }

            let mut symbols = initial_builtins();
            match eval(ast.program, &mut symbols, &source) {
                Ok(mut v) => {
                    // Apply arguments logic
                    // ... (Function application logic from old code) ...
                    // We need to apply opts.named_args and opts.pos_args

                    // If v is a function, try to apply args
                    let mut pos_idx = 0;
                    loop {
                        match &v {
                            Value::Function { ident, default, .. } => {
                                if let Some(named_val) = opts.named_args.get(ident) {
                                    match apply_function(
                                        &v,
                                        Value::String(named_val.clone()),
                                        &source,
                                        0,
                                    ) {
                                        Ok(nv) => {
                                            v = nv;
                                            continue;
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "{}",
                                                e.pretty_with_file(&source, Some(&source_name))
                                            );
                                            return 1;
                                        }
                                    }
                                } else if pos_idx < opts.pos_args.len() {
                                    match apply_function(
                                        &v,
                                        Value::String(opts.pos_args[pos_idx].clone()),
                                        &source,
                                        0,
                                    ) {
                                        Ok(nv) => {
                                            v = nv;
                                            pos_idx += 1;
                                            continue;
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "{}",
                                                e.pretty_with_file(&source, Some(&source_name))
                                            );
                                            return 1;
                                        }
                                    }
                                } else if let Some(def_box) = default {
                                    match apply_function(&v, (**def_box).clone(), &source, 0) {
                                        Ok(nv) => {
                                            v = nv;
                                            continue;
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "{}",
                                                e.pretty_with_file(&source, Some(&source_name))
                                            );
                                            return 1;
                                        }
                                    }
                                } else {
                                    eprintln!("Error: Missing required argument: {}", ident);
                                    eprintln!(
                                        "  The program expects an argument named '{}'",
                                        ident
                                    );
                                    if !opts.named_args.is_empty() || !opts.pos_args.is_empty() {
                                        eprintln!(
                                            "  Provided arguments: {:?}",
                                            opts.named_args.keys().collect::<Vec<_>>()
                                        );
                                        if !opts.pos_args.is_empty() {
                                            eprintln!(
                                                "  Positional arguments: {:?}",
                                                opts.pos_args
                                            );
                                        }
                                    }
                                    eprintln!(
                                        "  Usage: avon deploy {} -{} <value>",
                                        source_name, ident
                                    );
                                    eprintln!(
                                        "  Example: avon deploy {} -{} myvalue",
                                        source_name, ident
                                    );
                                    return 1;
                                }
                            }
                            Value::Builtin(_, _) => {
                                if pos_idx < opts.pos_args.len() {
                                    match apply_function(
                                        &v,
                                        Value::String(opts.pos_args[pos_idx].clone()),
                                        &source,
                                        0,
                                    ) {
                                        Ok(nv) => {
                                            v = nv;
                                            pos_idx += 1;
                                            continue;
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "{}",
                                                e.pretty_with_file(&source, Some(&source_name))
                                            );
                                            return 1;
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }

                    // Result handling
                    if deploy_mode {
                        match collect_file_templates(&v, &source) {
                            Ok(files) => {
                                // SAFETY: Collect all files first, validate all paths, then write all files
                                // If any error occurs during collection or validation, no files are written

                                // Step 1: Prepare all file operations (validate paths, create dirs)
                                // SECURITY: Canonicalize root path once to prevent symlink attacks
                                let (root_path, canonical_root) = if let Some(root_str) = &opts.root
                                {
                                    let root = std::path::Path::new(root_str);
                                    match std::fs::canonicalize(root) {
                                        Ok(canon) => (root.to_path_buf(), Some(canon)),
                                        Err(_) => {
                                            // If root doesn't exist, create it and then canonicalize
                                            if let Err(e) = std::fs::create_dir_all(root) {
                                                eprintln!(
                                                    "Error: Failed to create root directory: {}",
                                                    root_str
                                                );
                                                eprintln!("  Reason: {}", e);
                                                eprintln!(
                                                    "Deployment aborted. No files were written."
                                                );
                                                return 1;
                                            }
                                            match std::fs::canonicalize(root) {
                                                Ok(canon) => (root.to_path_buf(), Some(canon)),
                                                Err(e) => {
                                                    eprintln!("Error: Failed to canonicalize root directory: {}", root_str);
                                                    eprintln!("  Reason: {}", e);
                                                    eprintln!("Deployment aborted. No files were written.");
                                                    return 1;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    (std::path::PathBuf::new(), None)
                                };

                                let mut prepared_files: Vec<(
                                    std::path::PathBuf,
                                    String,
                                    bool,
                                    bool,
                                )> = Vec::new();
                                for (path, content) in &files {
                                    let write_path = if let Some(canon_root) = &canonical_root {
                                        // SECURITY: Reject paths containing ".." before processing
                                        // This prevents directory traversal attacks
                                        if path.contains("..") {
                                            eprintln!("Error: Path traversal detected: {}", path);
                                            eprintln!("  Path contains '..' which is not allowed");
                                            eprintln!("  Deployment aborted.");
                                            return 1;
                                        }

                                        let rel = path.trim_start_matches('/');
                                        // SECURITY: Normalize path to prevent directory traversal attacks
                                        // Filter out ParentDir ("..") and RootDir components as additional safety
                                        let normalized = std::path::Path::new(rel)
                                            .components()
                                            .filter(|c| match c {
                                                std::path::Component::ParentDir => false, // Block ".."
                                                std::path::Component::RootDir => false, // Block absolute paths
                                                _ => true,
                                            })
                                            .collect::<std::path::PathBuf>();

                                        // Build the full path within the root
                                        let full_path = root_path.join(&normalized);

                                        // SECURITY: Canonicalize the full path to resolve symlinks
                                        // If the path doesn't exist yet, we need to check parent directories
                                        let resolved = if full_path.exists() {
                                            match std::fs::canonicalize(&full_path) {
                                                Ok(p) => p,
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error: Failed to resolve path: {}",
                                                        full_path.display()
                                                    );
                                                    eprintln!("  Reason: {}", e);
                                                    eprintln!("Deployment aborted. No files were written.");
                                                    return 1;
                                                }
                                            }
                                        } else {
                                            // Path doesn't exist yet - canonicalize parent and check it's within root
                                            if let Some(parent) = full_path.parent() {
                                                if parent.exists() {
                                                    match std::fs::canonicalize(parent) {
                                                        Ok(canon_parent) => {
                                                            // Ensure parent is within root
                                                            if !canon_parent.starts_with(canon_root)
                                                            {
                                                                eprintln!("Error: Path traversal detected: {}", path);
                                                                eprintln!("  Attempted path would escape --root directory");
                                                                eprintln!("  Deployment aborted.");
                                                                return 1;
                                                            }
                                                            // Build the final path from canonical parent + filename
                                                            canon_parent.join(
                                                                full_path
                                                                    .file_name()
                                                                    .unwrap_or_default(),
                                                            )
                                                        }
                                                        Err(e) => {
                                                            eprintln!("Error: Failed to resolve parent directory: {}", parent.display());
                                                            eprintln!("  Reason: {}", e);
                                                            eprintln!("Deployment aborted. No files were written.");
                                                            return 1;
                                                        }
                                                    }
                                                } else {
                                                    // Parent doesn't exist - will be created, but validate the path structure
                                                    // Check that all parent components are safe
                                                    let mut current = root_path.clone();
                                                    for component in normalized.components() {
                                                        current = current.join(component);
                                                        // This should never happen due to filtering above, but double-check
                                                        if let std::path::Component::ParentDir =
                                                            component
                                                        {
                                                            eprintln!("Error: Path traversal detected: {}", path);
                                                            eprintln!("  Attempted path would escape --root directory");
                                                            eprintln!("  Deployment aborted.");
                                                            return 1;
                                                        }
                                                    }
                                                    full_path
                                                }
                                            } else {
                                                // No parent - this is the root itself (shouldn't happen with file paths)
                                                full_path
                                            }
                                        };

                                        // SECURITY: Final check - ensure resolved path is within canonical root
                                        if !resolved.starts_with(canon_root) {
                                            eprintln!("Error: Path traversal detected: {}", path);
                                            eprintln!(
                                                "  Attempted path would escape --root directory"
                                            );
                                            eprintln!("  Resolved path: {}", resolved.display());
                                            eprintln!("  Root directory: {}", canon_root.display());
                                            eprintln!("  Deployment aborted.");
                                            return 1;
                                        }

                                        resolved
                                    } else {
                                        // SECURITY: Without --root, validate absolute paths don't contain ".."
                                        let path_buf = std::path::Path::new(&path).to_path_buf();
                                        if path_buf
                                            .components()
                                            .any(|c| matches!(c, std::path::Component::ParentDir))
                                        {
                                            eprintln!("Error: Path contains '..' which is not allowed without --root");
                                            eprintln!(
                                                "  Use --root to safely contain file operations"
                                            );
                                            eprintln!("  Deployment aborted.");
                                            return 1;
                                        }
                                        // Also block absolute paths without --root for security
                                        if path_buf.is_absolute() {
                                            eprintln!("Error: Absolute paths are not allowed without --root");
                                            eprintln!(
                                                "  Use --root to safely contain file operations"
                                            );
                                            eprintln!(
                                                "  Example: avon deploy program.av --root ./output"
                                            );
                                            eprintln!("  Deployment aborted.");
                                            return 1;
                                        }
                                        path_buf
                                    };

                                    let exists = write_path.exists();
                                    let mut should_backup = false;

                                    if exists {
                                        if opts.if_not_exists {
                                            println!("Skipped {} (exists)", write_path.display());
                                            continue;
                                        }

                                        if opts.backup {
                                            should_backup = true;
                                        } else if !opts.force && !opts.append {
                                            eprintln!("WARNING: File {} exists. Use --force to overwrite, --append to append, or --backup to backup and overwrite.", write_path.display());
                                            continue;
                                        }
                                    }

                                    // Create parent directories before writing
                                    if let Some(parent) = write_path.parent() {
                                        if let Err(e) = std::fs::create_dir_all(parent) {
                                            eprintln!(
                                                "Error: Failed to create directory: {}",
                                                parent.display()
                                            );
                                            eprintln!("  Reason: {}", e);
                                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                                eprintln!("  Tip: Check directory permissions");
                                                eprintln!(
                                                    "  Tip: Try using a different --root directory"
                                                );
                                            } else if e.kind() == std::io::ErrorKind::NotFound {
                                                eprintln!(
                                                    "  Tip: Check that the parent path exists"
                                                );
                                            }
                                            eprintln!("Deployment aborted. No files were written.");
                                            return 1;
                                        }
                                    }

                                    prepared_files.push((
                                        write_path,
                                        content.clone(),
                                        exists,
                                        should_backup,
                                    ));
                                }

                                // Step 2: Validate all files can be written BEFORE writing any
                                // This ensures true atomicity - if any file can't be written, none are written
                                for (write_path, _content, exists, should_backup) in &prepared_files
                                {
                                    // Check if we can write to the file location
                                    // For existing files, check write permissions
                                    if *exists {
                                        // Validate: Check if existing file can be opened for writing
                                        #[allow(clippy::suspicious_open_options)]
                                        match std::fs::OpenOptions::new()
                                            .write(true)
                                            .truncate(false)
                                            .open(write_path)
                                        {
                                            Ok(_) => {
                                                // File is writable, continue
                                            }
                                            Err(e) => {
                                                eprintln!(
                                                    "Error: Cannot write to existing file: {}",
                                                    write_path.display()
                                                );
                                                eprintln!("  Reason: {}", e);
                                                if e.kind() == std::io::ErrorKind::PermissionDenied
                                                {
                                                    eprintln!("  Tip: Check file permissions");
                                                }
                                                eprintln!(
                                                    "Deployment aborted. No files were written."
                                                );
                                                return 1;
                                            }
                                        }
                                    }

                                    // For backup operations, check if backup location is writable
                                    if *should_backup {
                                        let mut backup_name =
                                            write_path.file_name().unwrap().to_os_string();
                                        backup_name.push(".bak");
                                        let backup_path = write_path.with_file_name(backup_name);

                                        // Check if we can create the backup file
                                        #[allow(clippy::suspicious_open_options)]
                                        match std::fs::OpenOptions::new()
                                            .write(true)
                                            .create(true)
                                            .truncate(true)
                                            .open(&backup_path)
                                        {
                                            Ok(_) => {
                                                // Backup location is writable, continue
                                            }
                                            Err(e) => {
                                                eprintln!(
                                                    "Error: Cannot create backup file: {}",
                                                    backup_path.display()
                                                );
                                                eprintln!("  Reason: {}", e);
                                                if e.kind() == std::io::ErrorKind::PermissionDenied
                                                {
                                                    eprintln!("  Tip: Check write permissions for backup location");
                                                }
                                                eprintln!(
                                                    "Deployment aborted. No files were written."
                                                );
                                                return 1;
                                            }
                                        }
                                    }

                                    // For new files, check parent directory is writable
                                    if !*exists {
                                        if let Some(parent) = write_path.parent() {
                                            // Try to create a test file to verify write permissions
                                            let test_file = parent.join(".avon_write_test");
                                            match std::fs::File::create(&test_file) {
                                                Ok(_) => {
                                                    // Clean up test file immediately
                                                    let _ = std::fs::remove_file(&test_file);
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error: Cannot write to directory: {}",
                                                        parent.display()
                                                    );
                                                    eprintln!("  Reason: {}", e);
                                                    if e.kind()
                                                        == std::io::ErrorKind::PermissionDenied
                                                    {
                                                        eprintln!("  Tip: Check directory write permissions");
                                                    }
                                                    eprintln!("Deployment aborted. No files were written.");
                                                    return 1;
                                                }
                                            }
                                        }
                                    }
                                }

                                // Step 3: All files validated - now write them all
                                // Since we've validated all can be written, we can write them sequentially
                                let mut written_files = Vec::new();
                                for (write_path, content, exists, should_backup) in prepared_files {
                                    // Perform backup if needed
                                    if should_backup {
                                        let mut backup_name =
                                            write_path.file_name().unwrap().to_os_string();
                                        backup_name.push(".bak");
                                        let backup_path = write_path.with_file_name(backup_name);

                                        if let Err(e) = std::fs::copy(&write_path, &backup_path) {
                                            eprintln!(
                                                "Error: Failed to create backup: {}",
                                                backup_path.display()
                                            );
                                            eprintln!("  Reason: {}", e);
                                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                                eprintln!("  Tip: Check write permissions for the backup location");
                                            }
                                            if !written_files.is_empty() {
                                                eprintln!("  Note: {} file(s) were written before the error occurred.", written_files.len());
                                            }
                                            eprintln!("Deployment aborted.");
                                            return 1;
                                        }
                                        println!("Backed up to {}", backup_path.display());
                                    }

                                    if opts.append && exists {
                                        use std::io::Write;
                                        match std::fs::OpenOptions::new()
                                            .append(true)
                                            .open(&write_path)
                                        {
                                            Ok(mut f) => {
                                                if let Err(e) = f.write_all(content.as_bytes()) {
                                                    eprintln!(
                                                        "Error: Failed to append to file: {}",
                                                        write_path.display()
                                                    );
                                                    eprintln!("  Reason: {}", e);
                                                    if e.kind()
                                                        == std::io::ErrorKind::PermissionDenied
                                                    {
                                                        eprintln!("  Tip: Check file permissions");
                                                    } else if e.kind()
                                                        == std::io::ErrorKind::OutOfMemory
                                                    {
                                                        eprintln!("  Tip: File may be too large for available memory");
                                                    }
                                                    if !written_files.is_empty() {
                                                        eprintln!("  Note: {} file(s) were written before the error occurred.", written_files.len());
                                                    }
                                                    eprintln!("Deployment aborted.");
                                                    return 1;
                                                }
                                                println!("Appended to {}", write_path.display());
                                                written_files.push(write_path.clone());
                                            }
                                            Err(e) => {
                                                eprintln!(
                                                    "Error: Failed to open file for append: {}",
                                                    write_path.display()
                                                );
                                                eprintln!("  Reason: {}", e);
                                                if e.kind() == std::io::ErrorKind::PermissionDenied
                                                {
                                                    eprintln!("  Tip: Check file permissions");
                                                    eprintln!("  Tip: Try using --backup instead of --append");
                                                }
                                                if !written_files.is_empty() {
                                                    eprintln!("  Note: {} file(s) were written before the error occurred.", written_files.len());
                                                }
                                                eprintln!("Deployment aborted.");
                                                return 1;
                                            }
                                        }
                                    } else {
                                        if let Err(e) = std::fs::write(&write_path, content) {
                                            eprintln!(
                                                "Error: Failed to write file: {}",
                                                write_path.display()
                                            );
                                            eprintln!("  Reason: {}", e);
                                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                                eprintln!("  Tip: Check file permissions");
                                                eprintln!(
                                                    "  Tip: Try using a different --root directory"
                                                );
                                            } else if e.kind() == std::io::ErrorKind::NotFound {
                                                eprintln!(
                                                    "  Tip: Check that the parent directory exists"
                                                );
                                            } else if e.kind() == std::io::ErrorKind::OutOfMemory {
                                                eprintln!("  Tip: File may be too large for available memory");
                                            }
                                            if !written_files.is_empty() {
                                                eprintln!("  Note: {} file(s) were written before the error occurred.", written_files.len());
                                            }
                                            eprintln!("Deployment aborted.");
                                            return 1;
                                        }
                                        if exists {
                                            println!("Overwrote {}", write_path.display());
                                        } else {
                                            println!("Wrote {}", write_path.display());
                                        }
                                        written_files.push(write_path);
                                    }
                                }
                            }
                            Err(e) => {
                                // In deploy mode, if the result isn't deployable (not FileTemplate or list), error out
                                eprintln!("Error: Deployment failed - result is not deployable");
                                eprintln!("  The program evaluated successfully, but the result cannot be deployed.");
                                eprintln!("  Expected: FileTemplate or list of FileTemplates");
                                eprintln!("  Got: {}", v.to_string(&source));
                                eprintln!("  Details: {}", e.message);
                                eprintln!();
                                eprintln!("  Tip: Make sure your program returns a FileTemplate (using @/path {{...}})");
                                eprintln!("  Tip: Or return a list of FileTemplates: [@/file1.txt {{...}}, @/file2.txt {{...}}]");
                                eprintln!("  Tip: Use 'avon eval {}' to see what your program evaluates to", source_name);
                                eprintln!("  No files were written.");
                                return 1;
                            }
                        }
                    } else {
                        // Eval mode
                        match collect_file_templates(&v, &source) {
                            Ok(files) => {
                                for (path, content) in files {
                                    println!("--- {} ---", path);
                                    println!("{}", content);
                                }
                            }
                            Err(_) => {
                                println!("{}", v.to_string(&source));
                            }
                        }
                    }
                    0
                }
                Err(e) => {
                    eprintln!("{}", e.pretty_with_file(&source, Some(&source_name)));
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e.pretty_with_file(&source, Some(&source_name)));
            1
        }
    }
}

fn execute_eval(opts: CliOptions) -> i32 {
    match get_source(&opts) {
        Ok((source, name)) => process_source(source, name, opts, false),
        Err(c) => c,
    }
}

fn execute_deploy(opts: CliOptions) -> i32 {
    match get_source(&opts) {
        Ok((source, name)) => process_source(source, name, opts, true),
        Err(c) => c,
    }
}

fn execute_run(opts: CliOptions) -> i32 {
    if let Some(code) = opts.code.clone() {
        process_source(code, "<input>".to_string(), opts, false)
    } else {
        1
    }
}

fn execute_repl() -> i32 {
    println!("Avon REPL - Interactive Avon Shell");
    println!("Type ':help' for commands, ':exit' to quit");
    println!();

    // Configure rustyline with tab completion
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let mut symbols = initial_builtins();
    let symbols_rc = Rc::new(RefCell::new(symbols.clone()));

    // Set up completer for tab completion using Helper trait
    use rustyline::highlight::Highlighter;
    use rustyline::hint::Hinter;
    use rustyline::validate::Validator;
    use rustyline::Helper;

    struct AvonHelper {
        completer: AvonCompleter,
    }
    impl Helper for AvonHelper {}
    impl Completer for AvonHelper {
        type Candidate = Pair;
        fn complete(
            &self,
            line: &str,
            pos: usize,
            ctx: &Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Pair>)> {
            self.completer.complete(line, pos, ctx)
        }
    }
    impl Highlighter for AvonHelper {}
    impl Hinter for AvonHelper {
        type Hint = String;
    }
    impl Validator for AvonHelper {}

    let helper = AvonHelper {
        completer: AvonCompleter {
            file_completer: FilenameCompleter::new(),
            symbols: symbols_rc.clone(),
        },
    };

    let mut rl = match Editor::with_config(config) {
        Ok(mut editor) => {
            editor.set_helper(Some(helper));
            editor
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize REPL: {}", e);
            return 1;
        }
    };

    // History navigation (↑/↓ arrows) and emacs shortcuts are enabled by default
    // History is kept in-memory only (no file is created or saved)
    let mut input_buffer = String::new();
    let mut watched_vars: std::collections::HashMap<String, Value> =
        std::collections::HashMap::new();

    loop {
        let prompt = if input_buffer.is_empty() {
            "avon> ".to_string() // 6 chars: "avon" (4) + ">" (1) + " " (1)
        } else {
            "    > ".to_string() // 6 chars: 4 spaces + ">" + " " = matches "avon> " exactly, aligns "let"
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                // Handle empty input
                if trimmed.is_empty() {
                    if !input_buffer.is_empty() {
                        // Continue multi-line input
                        input_buffer.push('\n');
                        continue;
                    }
                    continue;
                }

                // Note: History entries are added after successful evaluation
                // This ensures both single-line and multi-line expressions are properly recorded

                // Handle REPL commands
                if trimmed.starts_with(':') {
                    let cmd = trimmed.trim_start_matches(':');
                    match cmd {
                        "help" | "h" => {
                            println!("REPL Commands:");
                            println!("  :help, :h       Show this help");
                            println!("  :let <name> = <expr>  Store a value");
                            println!("  :vars           List all stored variables");
                            println!("  :inspect <name> Show detailed variable info");
                            println!("  :unlet <name>   Remove a variable");
                            println!("  :read <file>    Read and display file contents");
                            println!("  :run <file>     Evaluate file and display result");
                            println!(
                                "  :eval <file>    Evaluate file and merge Dict keys into REPL"
                            );
                            println!("  :preview <file> [--debug]  Preview what would be deployed");
                            println!("  :deploy <file> [flags...]  Deploy a file (supports --root, --force, --backup, --append, --if-not-exists, --debug, -param value)");
                            println!(
                                "  :deploy-expr <expr> [--root <dir>]  Deploy expression result"
                            );
                            println!("  :run <file> [--debug]  Evaluate file and display result");
                            println!("  :write <file> <expr>  Write expression result to file");
                            println!("  :history        Show command history");
                            println!("  :save-session <file>  Save REPL state to file");
                            println!("  :load-session <file>  Load REPL state from file");
                            println!("  :assert <expr>  Assert that expression is true");
                            println!(
                                "  :test <expr> <expected>  Test that expression equals expected"
                            );
                            println!(
                                "  :benchmark <expr>        Measure expression evaluation time"
                            );
                            println!("  :benchmark-file <file>    Measure file evaluation time");
                            println!("  :watch <name>   Watch a variable for changes");
                            println!("  :unwatch <name> Stop watching a variable");
                            println!("  :pwd            Show current working directory");
                            println!("  :list [dir]     List directory contents");
                            println!("  :cd <dir>       Change working directory");
                            println!("  :sh <command>   Execute shell command");
                            println!(
                                "  :doc            Show all builtin functions and REPL commands"
                            );
                            println!("  :doc <name>     Show documentation for a builtin function or REPL command");
                            println!("  :type <expr>    Show the type of an expression");
                            println!("  :clear          Clear all user-defined variables");
                            println!("  :exit, :quit    Exit the REPL");
                            println!();
                            println!("Navigation:");
                            println!("  ↑/↓             Navigate command history");
                            println!("  Ctrl+A          Move to beginning of line");
                            println!("  Ctrl+E          Move to end of line");
                            println!("  Ctrl+K          Delete from cursor to end of line");
                            println!("  Ctrl+U          Delete from cursor to beginning of line");
                            println!("  Ctrl+F          Move forward one character");
                            println!("  Ctrl+B          Move backward one character");
                            println!("  Ctrl+W          Delete word backward");
                            println!("  Ctrl+L          Clear screen");
                            println!();
                            println!("Examples:");
                            println!("  map (\\x x * 2) [1, 2, 3]");
                            println!("  let x = 42 in x + 1");
                            println!("  trace \"result\" (1 + 2)");
                            println!("  typeof [1, 2, 3]");
                            // Add REPL commands to history too
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        "exit" | "quit" | "q" => {
                            let _ = rl.add_history_entry(trimmed);
                            println!("Goodbye!");
                            break;
                        }
                        "clear" => {
                            symbols = initial_builtins();
                            *symbols_rc.borrow_mut() = symbols.clone();
                            input_buffer.clear();
                            println!("Cleared all user-defined variables");
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        "vars" => {
                            // List all stored variables (excluding builtins)
                            let builtins = initial_builtins();
                            let user_vars: Vec<(&String, &Value)> = symbols
                                .iter()
                                .filter(|(name, _)| !builtins.contains_key(*name))
                                .collect();

                            if user_vars.is_empty() {
                                println!("No user-defined variables.");
                                println!("Use :let <name> = <expr> to store a value.");
                            } else {
                                println!("User-defined variables:");
                                let mut sorted_vars: Vec<(&String, &Value)> = user_vars;
                                sorted_vars.sort_by_key(|(name, _)| *name);

                                for (name, val) in sorted_vars {
                                    let type_name = match val {
                                        Value::String(_) => "String",
                                        Value::Number(_) => "Number",
                                        Value::Bool(_) => "Bool",
                                        Value::List(_) => "List",
                                        Value::Dict(_) => "Dict",
                                        Value::Function { .. } => "Function",
                                        Value::Builtin(_, _) => "Builtin",
                                        Value::FileTemplate { .. } => "FileTemplate",
                                        Value::Template(_, _) => "Template",
                                        Value::Path(_, _) => "Path",
                                        Value::None => "None",
                                    };
                                    // Show value for simple types, just type for complex ones
                                    match val {
                                        Value::String(s) => {
                                            println!("  {} : {} = \"{}\"", name, type_name, s)
                                        }
                                        Value::Number(_) => {
                                            let val_str = val.to_string("");
                                            println!("  {} : {} = {}", name, type_name, val_str)
                                        }
                                        Value::Bool(b) => {
                                            println!("  {} : {} = {}", name, type_name, b)
                                        }
                                        Value::List(l) => println!(
                                            "  {} : {} = [{} items]",
                                            name,
                                            type_name,
                                            l.len()
                                        ),
                                        Value::Dict(d) => println!(
                                            "  {} : {} = {{ {} keys }}",
                                            name,
                                            type_name,
                                            d.len()
                                        ),
                                        _ => println!("  {} : {}", name, type_name),
                                    }
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("let ") => {
                            // Parse: :let <name> = <expr>
                            let rest = cmd.trim_start_matches("let ").trim();
                            if let Some(equals_pos) = rest.find('=') {
                                let var_name = rest[..equals_pos].trim();
                                let expr_str = rest[equals_pos + 1..].trim();

                                if var_name.is_empty() {
                                    eprintln!("Error: Variable name cannot be empty");
                                    eprintln!("Usage: :let <name> = <expr>");
                                    eprintln!("  Example: :let x = 42");
                                    eprintln!("  Example: :let double = \\x x * 2");
                                    continue;
                                }

                                if expr_str.is_empty() {
                                    eprintln!("Error: Expression cannot be empty");
                                    eprintln!("Usage: :let <name> = <expr>");
                                    continue;
                                }

                                // Evaluate the expression
                                match tokenize(expr_str.to_string()) {
                                    Ok(tokens) => {
                                        let ast = parse(tokens);
                                        match eval(ast.program, &mut symbols, expr_str) {
                                            Ok(val) => {
                                                // Check if this variable is being watched and has changed
                                                let was_watched =
                                                    watched_vars.contains_key(var_name);
                                                let old_watched_val =
                                                    watched_vars.get(var_name).cloned();

                                                symbols.insert(var_name.to_string(), val.clone());
                                                *symbols_rc.borrow_mut() = symbols.clone();

                                                // Check watched variables for changes
                                                if was_watched {
                                                    if let Some(watched_old) = old_watched_val {
                                                        let old_str = watched_old.to_string("");
                                                        let new_str = val.to_string("");
                                                        if old_str != new_str {
                                                            println!(
                                                                "[WATCH] {} changed: {} -> {}",
                                                                var_name, old_str, new_str
                                                            );
                                                        }
                                                    }
                                                    // Update watched variable
                                                    watched_vars
                                                        .insert(var_name.to_string(), val.clone());
                                                }

                                                let type_name = match val {
                                                    Value::String(_) => "String",
                                                    Value::Number(_) => "Number",
                                                    Value::Bool(_) => "Bool",
                                                    Value::List(_) => "List",
                                                    Value::Dict(_) => "Dict",
                                                    Value::Function { .. } => "Function",
                                                    Value::Builtin(_, _) => "Builtin",
                                                    Value::FileTemplate { .. } => "FileTemplate",
                                                    Value::Template(_, _) => "Template",
                                                    Value::Path(_, _) => "Path",
                                                    Value::None => "None",
                                                };
                                                println!("Stored: {} : {}", var_name, type_name);
                                            }
                                            Err(e) => {
                                                eprintln!(
                                                    "Error: {}",
                                                    e.pretty_with_file(expr_str, Some("<repl>"))
                                                );
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Parse error: {}",
                                            e.pretty_with_file(expr_str, Some("<repl>"))
                                        );
                                    }
                                }
                            } else {
                                eprintln!("Error: Missing '=' in :let command");
                                eprintln!("Usage: :let <name> = <expr>");
                                eprintln!("  Example: :let x = 42");
                                eprintln!("  Example: :let double = \\x x * 2");
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        "doc" => {
                            // Show all builtins and REPL commands when :doc is called without arguments
                            println!("Available builtin functions (use :doc <name> for details):");
                            let builtins = initial_builtins();
                            let builtin_names: Vec<&String> = builtins.keys().collect();
                            let mut sorted_names: Vec<&str> =
                                builtin_names.iter().map(|s| s.as_str()).collect();
                            sorted_names.sort();
                            for (i, name) in sorted_names.iter().enumerate() {
                                if i > 0 && i % 6 == 0 {
                                    println!();
                                }
                                print!("  {:<15}", name);
                            }
                            println!();
                            println!();
                            println!("Available REPL commands (use :doc <command> for details):");
                            println!("  :help            :exit            :clear           :vars            :let             :inspect");
                            println!("  :unlet           :read            :run             :eval            :preview         :deploy");
                            println!("  :deploy-expr     :write           :history         :save-session     :load-session    :assert");
                            println!("  :test            :benchmark       :benchmark-file  :watch           :unwatch         :pwd             :list            :cd");
                            println!("  :doc             :type            :sh");
                            println!();
                            println!("Tip: Use :doc <name> to see detailed documentation for any builtin function or REPL command.");
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("inspect ") => {
                            let var_name = cmd.trim_start_matches("inspect ").trim();
                            if var_name.is_empty() {
                                eprintln!("Usage: :inspect <variable_name>");
                                eprintln!("  Example: :inspect x");
                                continue;
                            }

                            if let Some(val) = symbols.get(var_name) {
                                let type_name = match val {
                                    Value::String(_) => "String",
                                    Value::Number(_) => "Number",
                                    Value::Bool(_) => "Bool",
                                    Value::List(_) => "List",
                                    Value::Dict(_) => "Dict",
                                    Value::Function { .. } => "Function",
                                    Value::Builtin(_, _) => "Builtin",
                                    Value::FileTemplate { .. } => "FileTemplate",
                                    Value::Template(_, _) => "Template",
                                    Value::Path(_, _) => "Path",
                                    Value::None => "None",
                                };

                                println!("Variable: {}", var_name);
                                println!("  Type: {}", type_name);

                                // Show detailed information based on type
                                match val {
                                    Value::String(s) => {
                                        println!("  Value: \"{}\"", s);
                                        println!("  Length: {}", s.len());
                                    }
                                    Value::Number(_) => {
                                        let val_str = val.to_string("");
                                        println!("  Value: {}", val_str);
                                    }
                                    Value::Bool(b) => {
                                        println!("  Value: {}", b);
                                    }
                                    Value::List(l) => {
                                        println!("  Length: {}", l.len());
                                        println!("  Items:");
                                        for (i, item) in l.iter().take(10).enumerate() {
                                            let item_str = item.to_string("");
                                            println!("    [{}]: {}", i, item_str);
                                        }
                                        if l.len() > 10 {
                                            println!("    ... ({} more items)", l.len() - 10);
                                        }
                                    }
                                    Value::Dict(d) => {
                                        println!("  Keys: {}", d.len());
                                        let mut keys: Vec<&String> = d.keys().collect();
                                        keys.sort();
                                        println!("  Key list:");
                                        for key in keys.iter().take(20) {
                                            println!("    - {}", key);
                                        }
                                        if d.len() > 20 {
                                            println!("    ... ({} more keys)", d.len() - 20);
                                        }
                                    }
                                    Value::Function { ident, .. } => {
                                        println!("  Parameter: {}", ident);
                                    }
                                    _ => {
                                        let val_str = val.to_string("");
                                        println!("  Value: {}", val_str);
                                    }
                                }
                            } else {
                                eprintln!("Variable '{}' not found", var_name);
                                eprintln!("  Use :vars to see available variables");
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("unlet ") => {
                            let var_name = cmd.trim_start_matches("unlet ").trim();
                            if var_name.is_empty() {
                                eprintln!("Usage: :unlet <variable_name>");
                                eprintln!("  Example: :unlet x");
                                continue;
                            }

                            let builtins = initial_builtins();
                            if builtins.contains_key(var_name) {
                                eprintln!("Error: Cannot remove builtin function '{}'", var_name);
                                continue;
                            }

                            if symbols.remove(var_name).is_some() {
                                *symbols_rc.borrow_mut() = symbols.clone();
                                println!("Removed variable: {}", var_name);
                            } else {
                                eprintln!("Variable '{}' not found", var_name);
                                eprintln!("  Use :vars to see available variables");
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("doc ") => {
                            let name = cmd.trim_start_matches("doc ").trim();
                            if name.is_empty() {
                                // This shouldn't happen, but handle it gracefully
                                println!("Usage: :doc <name>");
                                println!("  Example: :doc map");
                                println!("  Example: :doc pwd");
                                println!("  Example: :doc read");
                                continue;
                            }

                            // First check if it's a REPL command
                            if let Some(doc) = get_repl_command_doc(name) {
                                println!("{}", doc);
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            // Then check if it's a builtin function
                            let builtins = initial_builtins();
                            if builtins.contains_key(name) {
                                if let Some(doc) = get_builtin_doc(name) {
                                    println!("{}", doc);
                                } else {
                                    println!("Function: {}", name);
                                    println!("  This is a builtin function.");
                                    println!("  Use 'avon doc' to see all builtin documentation.");
                                }
                            } else if symbols.contains_key(name) {
                                // User-defined variable/function
                                let val = &symbols[name];
                                let type_info = match val {
                                    Value::String(_) => "String",
                                    Value::Number(_) => "Number",
                                    Value::Bool(_) => "Bool",
                                    Value::List(_) => "List",
                                    Value::Dict(_) => "Dict",
                                    Value::Function { .. } => "Function (user-defined)",
                                    Value::Builtin(_, _) => "Builtin",
                                    Value::FileTemplate { .. } => "FileTemplate",
                                    Value::Template(_, _) => "Template",
                                    Value::Path(_, _) => "Path",
                                    Value::None => "None",
                                };
                                println!("Variable: {}", name);
                                println!("  Type: {}", type_info);
                                println!("  Note: This is a user-defined variable in the current REPL session.");
                            } else {
                                println!("Unknown function or variable: {}", name);
                                println!("  Use :doc to see available builtin functions and REPL commands");
                                println!("  Use 'avon doc' to see all builtin documentation");
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("type ") => {
                            let expr_str = cmd.trim_start_matches("type ").trim();
                            if expr_str.is_empty() {
                                println!("Usage: :type <expression>");
                                continue;
                            }
                            // Evaluate expression to get type
                            match tokenize(expr_str.to_string()) {
                                Ok(tokens) => {
                                    let ast = parse(tokens);
                                    let mut temp_symbols = symbols.clone();
                                    match eval(ast.program, &mut temp_symbols, expr_str) {
                                        Ok(val) => {
                                            let type_name = match val {
                                                Value::String(_) => "String",
                                                Value::Number(_) => "Number",
                                                Value::Bool(_) => "Bool",
                                                Value::List(_) => "List",
                                                Value::Dict(_) => "Dict",
                                                Value::Function { .. } => "Function",
                                                Value::Builtin(_, _) => "Builtin",
                                                Value::FileTemplate { .. } => "FileTemplate",
                                                Value::Template(_, _) => "Template",
                                                Value::Path(_, _) => "Path",
                                                Value::None => "None",
                                            };
                                            println!("Type: {}", type_name);
                                        }
                                        Err(e) => {
                                            eprintln!("Error: {}", e.message);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Parse error: {}",
                                        e.pretty_with_file(expr_str, Some("<input>"))
                                    );
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("read ") => {
                            let file_path = cmd.trim_start_matches("read ").trim();
                            if file_path.is_empty() {
                                eprintln!("Usage: :read <file_path>");
                                eprintln!("  Example: :read config.av");
                                continue;
                            }

                            // REPL is a power tool - allow any path the developer wants to read
                            match std::fs::read_to_string(file_path) {
                                Ok(content) => {
                                    println!("{}", content);
                                }
                                Err(e) => {
                                    eprintln!("Error reading file '{}': {}", file_path, e);
                                    if e.kind() == std::io::ErrorKind::NotFound {
                                        eprintln!("  Tip: Check that the file exists and the path is correct");
                                    } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                                        eprintln!("  Tip: Check file permissions");
                                    }
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("run ") => {
                            let file_path = cmd.trim_start_matches("run ").trim();
                            if file_path.is_empty() {
                                eprintln!("Usage: :run <file_path>");
                                eprintln!("  Example: :run config.av");
                                continue;
                            }

                            // REPL is a power tool - allow any path the developer wants to run
                            match std::fs::read_to_string(file_path) {
                                Ok(source) => {
                                    match tokenize(source.clone()) {
                                        Ok(tokens) => {
                                            let ast = parse(tokens);
                                            let mut temp_symbols = initial_builtins();
                                            match eval(ast.program, &mut temp_symbols, &source) {
                                                Ok(val) => {
                                                    // Display result nicely (same as regular evaluation)
                                                    match &val {
                                                        Value::FileTemplate { .. } => {
                                                            match collect_file_templates(
                                                                &val, &source,
                                                            ) {
                                                                Ok(files) => {
                                                                    println!("FileTemplate:");
                                                                    for (path, content) in files {
                                                                        println!(
                                                                            "  Path: {}",
                                                                            path
                                                                        );
                                                                        println!(
                                                                            "  Content:\n{}",
                                                                            content
                                                                        );
                                                                    }
                                                                }
                                                                Err(e) => {
                                                                    eprintln!(
                                                                        "Error: {}",
                                                                        e.message
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        Value::List(l) => {
                                                            // Check if it's a list of FileTemplates
                                                            let mut all_are_file_templates = true;
                                                            for item in l {
                                                                if !matches!(
                                                                    item,
                                                                    Value::FileTemplate { .. }
                                                                ) {
                                                                    all_are_file_templates = false;
                                                                    break;
                                                                }
                                                            }
                                                            if all_are_file_templates {
                                                                match collect_file_templates(
                                                                    &val, &source,
                                                                ) {
                                                                    Ok(files) => {
                                                                        println!("FileTemplates ({} files):", files.len());
                                                                        for (path, content) in files
                                                                        {
                                                                            println!(
                                                                                "  Path: {}",
                                                                                path
                                                                            );
                                                                            println!(
                                                                                "  Content:\n{}",
                                                                                content
                                                                            );
                                                                        }
                                                                    }
                                                                    Err(e) => {
                                                                        eprintln!(
                                                                            "Error: {}",
                                                                            e.message
                                                                        );
                                                                    }
                                                                }
                                                            } else {
                                                                let val_str = val.to_string("");
                                                                println!("{}", val_str);
                                                            }
                                                        }
                                                        _ => {
                                                            let val_str = val.to_string("");
                                                            println!("{}", val_str);
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error: {}",
                                                        e.pretty_with_file(
                                                            &source,
                                                            Some(file_path)
                                                        )
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Parse error: {}",
                                                e.pretty_with_file(&source, Some(file_path))
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error reading file '{}': {}", file_path, e);
                                    if e.kind() == std::io::ErrorKind::NotFound {
                                        eprintln!("  Tip: Check that the file exists and the path is correct");
                                    } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                                        eprintln!("  Tip: Check file permissions");
                                    }
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("eval ") => {
                            let file_path = cmd.trim_start_matches("eval ").trim();
                            if file_path.is_empty() {
                                eprintln!("Usage: :eval <file_path>");
                                eprintln!("  Example: :eval config.av");
                                eprintln!("  Note: If the file evaluates to a Dict, its keys will be added to REPL");
                                continue;
                            }

                            // REPL is a power tool - allow any path the developer wants to evaluate
                            match std::fs::read_to_string(file_path) {
                                Ok(source) => {
                                    match tokenize(source.clone()) {
                                        Ok(tokens) => {
                                            let ast = parse(tokens);
                                            let mut temp_symbols = initial_builtins();
                                            match eval(ast.program, &mut temp_symbols, &source) {
                                                Ok(val) => {
                                                    // If result is a Dict, merge its keys into REPL symbols
                                                    if let Value::Dict(d) = &val {
                                                        let mut added = 0;
                                                        for (key, value) in d.iter() {
                                                            symbols
                                                                .insert(key.clone(), value.clone());
                                                            added += 1;
                                                        }
                                                        *symbols_rc.borrow_mut() = symbols.clone();
                                                        println!(
                                                            "Evaluated file '{}': {} keys added to REPL",
                                                            file_path, added
                                                        );
                                                    } else {
                                                        // For non-dict results, just show the result
                                                        let val_str = val.to_string("");
                                                        println!("Result: {}", val_str);
                                                        println!("  Note: Use :let <name> = import \"{}\" to store this value", file_path);
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error: {}",
                                                        e.pretty_with_file(
                                                            &source,
                                                            Some(file_path)
                                                        )
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Parse error: {}",
                                                e.pretty_with_file(&source, Some(file_path))
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error reading file '{}': {}", file_path, e);
                                    if e.kind() == std::io::ErrorKind::NotFound {
                                        eprintln!("  Tip: Check that the file exists and the path is correct");
                                    } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                                        eprintln!("  Tip: Check file permissions");
                                    }
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("preview ") => {
                            let file_path = cmd.trim_start_matches("preview ").trim();
                            if file_path.is_empty() {
                                eprintln!("Usage: :preview <file_path>");
                                eprintln!("  Example: :preview config.av");
                                continue;
                            }

                            // Note: REPL is interactive, so we allow relative paths and absolute paths
                            // The developer is responsible for what they read

                            match std::fs::read_to_string(file_path) {
                                Ok(source) => match tokenize(source.clone()) {
                                    Ok(tokens) => {
                                        let ast = parse(tokens);
                                        let mut temp_symbols = initial_builtins();
                                        match eval(ast.program, &mut temp_symbols, &source) {
                                            Ok(val) => {
                                                match collect_file_templates(&val, &source) {
                                                    Ok(files) => {
                                                        println!(
                                                            "Would deploy {} file(s):",
                                                            files.len()
                                                        );
                                                        for (path, content) in files {
                                                            println!("  Path: {}", path);
                                                            println!("  Content:\n{}", content);
                                                        }
                                                    }
                                                    Err(e) => {
                                                        eprintln!("Error: {}", e.message);
                                                        eprintln!("  Result is not a FileTemplate or list of FileTemplates");
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!(
                                                    "Error: {}",
                                                    e.pretty_with_file(&source, Some(file_path))
                                                );
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Parse error: {}",
                                            e.pretty_with_file(&source, Some(file_path))
                                        );
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Error reading file '{}': {}", file_path, e);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("deploy ") => {
                            // Parse: :deploy <file> [--root <dir>]
                            let rest = cmd.trim_start_matches("deploy ").trim();
                            let parts: Vec<&str> = rest.split_whitespace().collect();

                            if parts.is_empty() {
                                eprintln!("Usage: :deploy <file_path> [flags...]");
                                eprintln!("  Flags: --root <dir>, --force, --backup, --append, --if-not-exists, --debug");
                                eprintln!("  Example: :deploy config.av --root ./output --backup");
                                eprintln!(
                                    "  Example: :deploy config.av --root ./out --force -env prod"
                                );
                                continue;
                            }

                            let file_path = parts[0];

                            // Parse flags (same as CLI for consistency)
                            let mut deploy_opts = CliOptions::new();
                            deploy_opts.file = Some(file_path.to_string());

                            let mut i = 1;
                            while i < parts.len() {
                                match parts[i] {
                                    "--root" => {
                                        if i + 1 < parts.len() {
                                            deploy_opts.root = Some(parts[i + 1].to_string());
                                            i += 2;
                                        } else {
                                            eprintln!(
                                                "Error: --root requires a directory argument"
                                            );
                                            let _ = rl.add_history_entry(trimmed);
                                            continue;
                                        }
                                    }
                                    "--force" => {
                                        deploy_opts.force = true;
                                        i += 1;
                                    }
                                    "--backup" => {
                                        deploy_opts.backup = true;
                                        i += 1;
                                    }
                                    "--append" => {
                                        deploy_opts.append = true;
                                        i += 1;
                                    }
                                    "--if-not-exists" => {
                                        deploy_opts.if_not_exists = true;
                                        i += 1;
                                    }
                                    "--debug" => {
                                        deploy_opts.debug = true;
                                        i += 1;
                                    }
                                    s if s.starts_with("-") && !s.starts_with("--") => {
                                        // Named argument: -param value
                                        let key = s.trim_start_matches('-').to_string();
                                        if i + 1 < parts.len() {
                                            deploy_opts
                                                .named_args
                                                .insert(key, parts[i + 1].to_string());
                                            i += 2;
                                        } else {
                                            eprintln!(
                                                "Error: Named argument '{}' requires a value",
                                                s
                                            );
                                            let _ = rl.add_history_entry(trimmed);
                                            continue;
                                        }
                                    }
                                    s => {
                                        // Positional argument
                                        deploy_opts.pos_args.push(s.to_string());
                                        i += 1;
                                    }
                                }
                            }

                            // Use process_source with deploy_mode=true
                            let result = match std::fs::read_to_string(file_path) {
                                Ok(source) => {
                                    process_source(source, file_path.to_string(), deploy_opts, true)
                                }
                                Err(e) => {
                                    eprintln!("Error reading file '{}': {}", file_path, e);
                                    1
                                }
                            };

                            if result == 0 {
                                println!("Deployment completed successfully");
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("deploy-expr ") => {
                            // Parse: :deploy-expr <expr> [--root <dir>]
                            let rest = cmd.trim_start_matches("deploy-expr ").trim();

                            // Parse --root flag if present
                            let (expr_str, root_dir) = if let Some(root_pos) = rest.find("--root") {
                                let expr_part = rest[..root_pos].trim();
                                let root_part = rest[root_pos..].trim();
                                let root_parts: Vec<&str> = root_part.split_whitespace().collect();
                                if root_parts.len() >= 2 && root_parts[0] == "--root" {
                                    (expr_part, Some(root_parts[1].to_string()))
                                } else {
                                    (rest, None)
                                }
                            } else {
                                (rest, None)
                            };

                            if expr_str.is_empty() {
                                eprintln!("Usage: :deploy-expr <expression> [--root <dir>]");
                                eprintln!("  Example: :deploy-expr @test.txt {{\"Hello\"}}");
                                eprintln!("  Example: :deploy-expr config --root ./output");
                                continue;
                            }

                            // Evaluate the expression
                            match tokenize(expr_str.to_string()) {
                                Ok(tokens) => {
                                    let ast = parse(tokens);
                                    match eval(ast.program, &mut symbols, expr_str) {
                                        Ok(val) => {
                                            // Check if result is FileTemplate or list of FileTemplates
                                            match collect_file_templates(&val, expr_str) {
                                                Ok(files) => {
                                                    if files.is_empty() {
                                                        eprintln!("Error: Expression does not produce any FileTemplates");
                                                        let _ = rl.add_history_entry(trimmed);
                                                        continue;
                                                    }

                                                    // Create CliOptions for deployment
                                                    let deploy_opts = CliOptions {
                                                        file: None,
                                                        root: root_dir,
                                                        force: false,
                                                        backup: false,
                                                        append: false,
                                                        if_not_exists: false,
                                                        debug: false,
                                                        git_url: None,
                                                        code: Some(expr_str.to_string()),
                                                        named_args: std::collections::HashMap::new(
                                                        ),
                                                        pos_args: Vec::new(),
                                                    };

                                                    // Manually handle deployment since we have the files already
                                                    // We need to reuse the deployment logic from process_source
                                                    // For now, let's use a simplified approach
                                                    let root_path = if let Some(root_str) =
                                                        &deploy_opts.root
                                                    {
                                                        std::path::Path::new(root_str).to_path_buf()
                                                    } else {
                                                        eprintln!("Error: --root is required for :deploy-expr");
                                                        eprintln!("  Usage: :deploy-expr <expr> --root <dir>");
                                                        let _ = rl.add_history_entry(trimmed);
                                                        continue;
                                                    };

                                                    // Create root directory if needed
                                                    if let Err(e) =
                                                        std::fs::create_dir_all(&root_path)
                                                    {
                                                        eprintln!("Error: Failed to create root directory: {}", e);
                                                        let _ = rl.add_history_entry(trimmed);
                                                        continue;
                                                    }

                                                    // Deploy files (simplified - reuse logic from process_source would be better)
                                                    let mut success = true;
                                                    for (path, content) in &files {
                                                        // Note: Path validation is handled by --root requirement
                                                        // The developer controls what they deploy in the REPL
                                                        let rel = path.trim_start_matches('/');
                                                        let full_path = root_path.join(rel);

                                                        // Create parent directories
                                                        if let Some(parent) = full_path.parent() {
                                                            if let Err(e) =
                                                                std::fs::create_dir_all(parent)
                                                            {
                                                                eprintln!("Error: Failed to create directory: {}", e);
                                                                success = false;
                                                                break;
                                                            }
                                                        }

                                                        // Write file
                                                        if let Err(e) =
                                                            std::fs::write(&full_path, content)
                                                        {
                                                            eprintln!(
                                                                "Error: Failed to write {}: {}",
                                                                full_path.display(),
                                                                e
                                                            );
                                                            success = false;
                                                            break;
                                                        }
                                                        println!(
                                                            "Deployed: {}",
                                                            full_path.display()
                                                        );
                                                    }

                                                    if success {
                                                        println!(
                                                            "Deployment completed successfully"
                                                        );
                                                    } else {
                                                        eprintln!("Deployment failed - some files may have been written");
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!("Error: {}", e.message);
                                                    eprintln!("  Expression result is not a FileTemplate or list of FileTemplates");
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Error: {}",
                                                e.pretty_with_file(expr_str, Some("<repl>"))
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Parse error: {}",
                                        e.pretty_with_file(expr_str, Some("<repl>"))
                                    );
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        "history" => {
                            // Show command history
                            let history = rl.history();
                            let entries: Vec<String> =
                                history.iter().map(|s| s.to_string()).collect();
                            let count = entries.len();
                            if count == 0 {
                                println!("No command history yet.");
                            } else {
                                println!("Command history ({} entries):", count);
                                // Show last 50 entries
                                let start = count.saturating_sub(50);
                                for (i, entry) in entries.iter().enumerate().skip(start) {
                                    println!("  {}: {}", i + 1, entry);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("write ") => {
                            // Parse: :write <file> <expr>
                            let rest = cmd.trim_start_matches("write ").trim();

                            // Find the file path (first word) and expression (rest)
                            let parts: Vec<&str> = rest.splitn(2, char::is_whitespace).collect();
                            if parts.len() < 2 {
                                eprintln!("Usage: :write <file_path> <expression>");
                                eprintln!("  Example: :write output.txt \"Hello, world!\"");
                                eprintln!("  Example: :write result.json (json_parse config)");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            let file_path = parts[0];
                            let expr_str = parts[1];

                            // Evaluate the expression
                            match tokenize(expr_str.to_string()) {
                                Ok(tokens) => {
                                    let ast = parse(tokens);
                                    match eval(ast.program, &mut symbols, expr_str) {
                                        Ok(val) => {
                                            // Convert value to string
                                            let content = val.to_string("");

                                            // Write to file
                                            match std::fs::write(file_path, content) {
                                                Ok(_) => {
                                                    println!("Written to: {}", file_path);
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error writing to '{}': {}",
                                                        file_path, e
                                                    );
                                                    if e.kind()
                                                        == std::io::ErrorKind::PermissionDenied
                                                    {
                                                        eprintln!("  Tip: Check file permissions");
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Error: {}",
                                                e.pretty_with_file(expr_str, Some("<repl>"))
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Parse error: {}",
                                        e.pretty_with_file(expr_str, Some("<repl>"))
                                    );
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("save-session ") => {
                            // Parse: :save-session <file>
                            let file_path = cmd.trim_start_matches("save-session ").trim();
                            if file_path.is_empty() {
                                eprintln!("Usage: :save-session <file_path>");
                                eprintln!("  Example: :save-session my_session.avon");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            // Serialize current REPL state to Avon format
                            let builtins = initial_builtins();
                            let user_vars: Vec<(&String, &Value)> = symbols
                                .iter()
                                .filter(|(name, _)| !builtins.contains_key(*name))
                                .collect();

                            let var_count = user_vars.len();
                            if var_count == 0 {
                                eprintln!("No user-defined variables to save.");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            // Generate Avon code that recreates the variables
                            // We'll create a dict containing all variables so they persist after evaluation
                            let mut session_code = String::from("# Avon REPL Session\n");
                            session_code.push_str("# Saved variables:\n\n");
                            let mut saved_count = 0;
                            let mut dict_entries = Vec::new();

                            for (name, val) in &user_vars {
                                // Convert value to Avon syntax
                                let val_str = match val {
                                    Value::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
                                    Value::Number(Number::Int(i)) => i.to_string(),
                                    Value::Number(Number::Float(f)) => f.to_string(),
                                    Value::Bool(b) => b.to_string(),
                                    Value::List(_) => val.to_string(""),
                                    Value::Dict(_) => val.to_string(""),
                                    Value::Function { .. } => {
                                        eprintln!("Warning: Cannot save function '{}' (functions cannot be serialized)", name);
                                        continue;
                                    }
                                    _ => {
                                        eprintln!("Warning: Cannot save variable '{}' (type not serializable)", name);
                                        continue;
                                    }
                                };
                                session_code.push_str(&format!("let {} = {} in\n", name, val_str));
                                dict_entries.push(format!("{}: {}", name, name));
                                saved_count += 1;
                            }

                            // Add a final expression that creates a dict with all variables
                            // This ensures variables persist in the symbols map after evaluation
                            if saved_count > 0 {
                                session_code
                                    .push_str(&format!("{{{}}}\n", dict_entries.join(", ")));
                            }

                            // Write session file
                            match std::fs::write(file_path, session_code) {
                                Ok(_) => {
                                    println!(
                                        "Session saved to: {} ({} variables)",
                                        file_path, saved_count
                                    );
                                }
                                Err(e) => {
                                    eprintln!("Error saving session to '{}': {}", file_path, e);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("load-session ") => {
                            // Parse: :load-session <file>
                            let file_path = cmd.trim_start_matches("load-session ").trim();
                            if file_path.is_empty() {
                                eprintln!("Usage: :load-session <file_path>");
                                eprintln!("  Example: :load-session my_session.avon");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            // Read and evaluate session file
                            match std::fs::read_to_string(file_path) {
                                Ok(source) => {
                                    match tokenize(source.clone()) {
                                        Ok(tokens) => {
                                            let ast = parse(tokens);
                                            // Evaluate in a temporary environment
                                            // The session file evaluates to a dict containing all variables
                                            let mut temp_symbols = initial_builtins();
                                            match eval(ast.program, &mut temp_symbols, &source) {
                                                Ok(val) => {
                                                    // Extract variables from the dict result
                                                    let builtins = initial_builtins();
                                                    let mut loaded = 0;

                                                    if let Value::Dict(d) = val {
                                                        // Variables are in the dict
                                                        for (name, value) in d.iter() {
                                                            if !builtins.contains_key(name) {
                                                                symbols.insert(
                                                                    name.clone(),
                                                                    value.clone(),
                                                                );
                                                                loaded += 1;
                                                            }
                                                        }
                                                    } else {
                                                        // Fallback: try to extract from temp_symbols (for backwards compatibility)
                                                        for (name, val) in temp_symbols.iter() {
                                                            if !builtins.contains_key(name) {
                                                                symbols.insert(
                                                                    name.clone(),
                                                                    val.clone(),
                                                                );
                                                                loaded += 1;
                                                            }
                                                        }
                                                    }

                                                    *symbols_rc.borrow_mut() = symbols.clone();
                                                    println!("Session loaded from: {} ({} variables restored)", file_path, loaded);
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error evaluating session file: {}",
                                                        e.pretty_with_file(
                                                            &source,
                                                            Some(file_path)
                                                        )
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Parse error in session file: {}",
                                                e.pretty_with_file(&source, Some(file_path))
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error reading session file '{}': {}", file_path, e);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("assert ") => {
                            // Parse: :assert <expr>
                            let expr_str = cmd.trim_start_matches("assert ").trim();
                            if expr_str.is_empty() {
                                eprintln!("Usage: :assert <expression>");
                                eprintln!("  Example: :assert (x > 0)");
                                eprintln!("  Example: :assert true");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            // Evaluate the expression
                            match tokenize(expr_str.to_string()) {
                                Ok(tokens) => {
                                    let ast = parse(tokens);
                                    match eval(ast.program, &mut symbols, expr_str) {
                                        Ok(val) => match val {
                                            Value::Bool(true) => {
                                                println!("✓ PASS: Assertion passed");
                                            }
                                            Value::Bool(false) => {
                                                eprintln!("✗ FAIL: Assertion failed (expression evaluated to false)");
                                            }
                                            _ => {
                                                eprintln!("✗ FAIL: Assertion failed (expression must evaluate to a boolean, got: {})", 
                                                        match val {
                                                            Value::String(_) => "String",
                                                            Value::Number(_) => "Number",
                                                            Value::List(_) => "List",
                                                            Value::Dict(_) => "Dict",
                                                            Value::Function { .. } => "Function",
                                                            _ => "Other"
                                                        });
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!(
                                                "✗ FAIL: {}",
                                                e.pretty_with_file(expr_str, Some("<repl>"))
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Parse error: {}",
                                        e.pretty_with_file(expr_str, Some("<repl>"))
                                    );
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("benchmark ") => {
                            // Parse: :benchmark <expr>
                            let rest = cmd.trim_start_matches("benchmark ").trim();
                            if rest.is_empty() {
                                eprintln!("Usage: :benchmark <expression>");
                                eprintln!("  Example: :benchmark map (\\x x * 2) [1..1000]");
                                eprintln!("  Example: :benchmark map double [1..1000]");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            // Benchmark an expression
                            let start = std::time::Instant::now();
                            match tokenize(rest.to_string()) {
                                Ok(tokens) => {
                                    let ast = parse(tokens);
                                    match eval(ast.program, &mut symbols, rest) {
                                        Ok(val) => {
                                            let duration = start.elapsed();
                                            let val_str = val.to_string("");
                                            println!("Result: {}", val_str);
                                            println!(
                                                "Time: {:?} ({:.2}ms)",
                                                duration,
                                                duration.as_secs_f64() * 1000.0
                                            );
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Error: {}",
                                                e.pretty_with_file(rest, Some("<repl>"))
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Parse error: {}",
                                        e.pretty_with_file(rest, Some("<repl>"))
                                    );
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("benchmark-file ") => {
                            // Parse: :benchmark-file <file>
                            let rest = cmd.trim_start_matches("benchmark-file ").trim();
                            if rest.is_empty() {
                                eprintln!("Usage: :benchmark-file <file>");
                                eprintln!("  Example: :benchmark-file config.av");
                                eprintln!("  Example: :benchmark-file large_program.av");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            // Benchmark a file
                            match std::fs::read_to_string(rest) {
                                Ok(source) => {
                                    let start = std::time::Instant::now();
                                    match tokenize(source.clone()) {
                                        Ok(tokens) => {
                                            let ast = parse(tokens);
                                            let mut temp_symbols = initial_builtins();
                                            match eval(ast.program, &mut temp_symbols, &source) {
                                                Ok(val) => {
                                                    let duration = start.elapsed();
                                                    let val_str = val.to_string("");
                                                    println!("File: {}", rest);
                                                    println!("Result: {}", val_str);
                                                    println!(
                                                        "Time: {:?} ({:.2}ms)",
                                                        duration,
                                                        duration.as_secs_f64() * 1000.0
                                                    );
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error: {}",
                                                        e.pretty_with_file(&source, Some(rest))
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Parse error: {}",
                                                e.pretty_with_file(&source, Some(rest))
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error reading file '{}': {}", rest, e);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("test ") => {
                            // Parse: :test <expr> <expected>
                            let rest = cmd.trim_start_matches("test ").trim();

                            // Find the boundary between expr and expected (look for last space before expected)
                            // This is tricky - we'll try to split on the last space, but that might not work for complex expressions
                            // For now, require the expected to be a simple value or use a separator
                            let parts: Vec<&str> = rest.rsplitn(2, char::is_whitespace).collect();
                            if parts.len() < 2 {
                                eprintln!("Usage: :test <expression> <expected_value>");
                                eprintln!("  Example: :test (double 21) 42");
                                eprintln!("  Example: :test (length [1,2,3]) 3");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            let expected_str = parts[0];
                            let expr_str = parts[1];

                            // Evaluate both expressions
                            let expr_result = match tokenize(expr_str.to_string()) {
                                Ok(tokens) => {
                                    let ast = parse(tokens);
                                    eval(ast.program, &mut symbols, expr_str)
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Parse error in expression: {}",
                                        e.pretty_with_file(expr_str, Some("<repl>"))
                                    );
                                    let _ = rl.add_history_entry(trimmed);
                                    continue;
                                }
                            };

                            let expected_result = match tokenize(expected_str.to_string()) {
                                Ok(tokens) => {
                                    let ast = parse(tokens);
                                    eval(ast.program, &mut symbols, expected_str)
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Parse error in expected value: {}",
                                        e.pretty_with_file(expected_str, Some("<repl>"))
                                    );
                                    let _ = rl.add_history_entry(trimmed);
                                    continue;
                                }
                            };

                            match (expr_result, expected_result) {
                                (Ok(expr_val), Ok(expected_val)) => {
                                    // Compare values using the same logic as == operator in eval
                                    let are_equal = match (&expr_val, &expected_val) {
                                        (Value::Number(ln), Value::Number(rn)) => {
                                            let lval = match ln {
                                                Number::Int(i) => *i as f64,
                                                Number::Float(f) => *f,
                                            };
                                            let rval = match rn {
                                                Number::Int(i) => *i as f64,
                                                Number::Float(f) => *f,
                                            };
                                            (lval - rval).abs() < f64::EPSILON
                                        }
                                        (Value::String(ls), Value::String(rs)) => ls == rs,
                                        (Value::Bool(lb), Value::Bool(rb)) => lb == rb,
                                        (Value::List(ll), Value::List(rl)) => {
                                            if ll.len() != rl.len() {
                                                false
                                            } else {
                                                ll.iter().zip(rl.iter()).all(|(l, r)| {
                                                    match (l, r) {
                                                        (Value::Number(ln), Value::Number(rn)) => {
                                                            let lval = match ln {
                                                                Number::Int(i) => *i as f64,
                                                                Number::Float(f) => *f,
                                                            };
                                                            let rval = match rn {
                                                                Number::Int(i) => *i as f64,
                                                                Number::Float(f) => *f,
                                                            };
                                                            (lval - rval).abs() < f64::EPSILON
                                                        }
                                                        (Value::String(ls), Value::String(rs)) => {
                                                            ls == rs
                                                        }
                                                        (Value::Bool(lb), Value::Bool(rb)) => {
                                                            lb == rb
                                                        }
                                                        _ => l.to_string("") == r.to_string(""),
                                                    }
                                                })
                                            }
                                        }
                                        _ => expr_val.to_string("") == expected_val.to_string(""),
                                    };

                                    if are_equal {
                                        println!("✓ PASS: {} == {}", expr_str, expected_str);
                                    } else {
                                        eprintln!("✗ FAIL: {} != {}", expr_str, expected_str);
                                        eprintln!("  Got: {}", expr_val.to_string(""));
                                        eprintln!("  Expected: {}", expected_val.to_string(""));
                                    }
                                }
                                (Err(e), _) => {
                                    eprintln!(
                                        "✗ FAIL: Error evaluating expression: {}",
                                        e.pretty_with_file(expr_str, Some("<repl>"))
                                    );
                                }
                                (_, Err(e)) => {
                                    eprintln!(
                                        "✗ FAIL: Error evaluating expected value: {}",
                                        e.pretty_with_file(expected_str, Some("<repl>"))
                                    );
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("list ") => {
                            // Parse: :list <dir>
                            let dir_path = cmd.trim_start_matches("list ").trim();
                            let dir = if dir_path.is_empty() { "." } else { dir_path };

                            match std::fs::read_dir(dir) {
                                Ok(entries) => {
                                    let mut files: Vec<String> = Vec::new();
                                    for entry in entries {
                                        match entry {
                                            Ok(e) => {
                                                let name =
                                                    e.file_name().to_string_lossy().to_string();
                                                let path = e.path();
                                                let file_type =
                                                    if path.is_dir() { "/" } else { "" };
                                                files.push(format!("{}{}", name, file_type));
                                            }
                                            Err(e) => {
                                                eprintln!("Error reading directory entry: {}", e);
                                            }
                                        }
                                    }
                                    files.sort();
                                    let current_dir = std::env::current_dir()
                                        .unwrap_or_else(|_| std::path::PathBuf::from("."));
                                    println!(
                                        "Directory '{}' (current: {}):",
                                        dir,
                                        current_dir.display()
                                    );
                                    if files.is_empty() {
                                        println!("  (empty)");
                                    } else {
                                        for file in files {
                                            println!("  {}", file);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error reading directory '{}': {}", dir, e);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        "list" => {
                            // List current directory
                            let current_dir = std::env::current_dir()
                                .unwrap_or_else(|_| std::path::PathBuf::from("."));
                            match std::fs::read_dir(&current_dir) {
                                Ok(entries) => {
                                    let mut files: Vec<String> = Vec::new();
                                    for entry in entries {
                                        match entry {
                                            Ok(e) => {
                                                let name =
                                                    e.file_name().to_string_lossy().to_string();
                                                let path = e.path();
                                                let file_type =
                                                    if path.is_dir() { "/" } else { "" };
                                                files.push(format!("{}{}", name, file_type));
                                            }
                                            Err(e) => {
                                                eprintln!("Error reading directory entry: {}", e);
                                            }
                                        }
                                    }
                                    files.sort();
                                    println!("Current directory: {}", current_dir.display());
                                    if files.is_empty() {
                                        println!("  (empty)");
                                    } else {
                                        for file in files {
                                            println!("  {}", file);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error reading current directory: {}", e);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("cd ") => {
                            // Parse: :cd <dir>
                            let dir_path = cmd.trim_start_matches("cd ").trim();
                            if dir_path.is_empty() {
                                eprintln!("Usage: :cd <directory>");
                                eprintln!("  Example: :cd ./examples");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            match std::env::set_current_dir(dir_path) {
                                Ok(_) => {
                                    let current_dir = std::env::current_dir()
                                        .unwrap_or_else(|_| std::path::PathBuf::from("."));
                                    println!("Changed directory to: {}", current_dir.display());
                                }
                                Err(e) => {
                                    eprintln!("Error changing directory to '{}': {}", dir_path, e);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        "pwd" => {
                            // Show current working directory
                            match std::env::current_dir() {
                                Ok(dir) => {
                                    println!("{}", dir.display());
                                }
                                Err(e) => {
                                    eprintln!("Error getting current directory: {}", e);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("sh ") => {
                            // Execute shell command: :sh <command>
                            let shell_cmd = cmd.trim_start_matches("sh ").trim();
                            if shell_cmd.is_empty() {
                                eprintln!("Usage: :sh <command>");
                                eprintln!("  Example: :sh ls -la");
                                eprintln!("  Example: :sh echo hello");
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            match std::process::Command::new("sh")
                                .arg("-c")
                                .arg(shell_cmd)
                                .status()
                            {
                                Ok(status) => {
                                    if !status.success() {
                                        eprintln!("Command exited with code: {:?}", status.code());
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error executing command: {}", e);
                                }
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        "sh" => {
                            // Start interactive shell
                            eprintln!("Usage: :sh <command>");
                            eprintln!("  Example: :sh ls -la");
                            eprintln!("  Example: :sh echo hello");
                            eprintln!("  Note: This executes a single shell command. For interactive shell, exit REPL and use your terminal.");
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("watch ") => {
                            // Parse: :watch <name>
                            let var_name = cmd.trim_start_matches("watch ").trim();
                            if var_name.is_empty() {
                                eprintln!("Usage: :watch <variable_name>");
                                eprintln!("  Example: :watch x");
                                eprintln!(
                                    "  Use :watch with no argument to list watched variables"
                                );
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            if let Some(val) = symbols.get(var_name) {
                                watched_vars.insert(var_name.to_string(), val.clone());
                                println!("Watching: {} = {}", var_name, val.to_string(""));
                            } else {
                                eprintln!("Variable '{}' not found", var_name);
                                eprintln!("  Use :vars to see available variables");
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        "watch" => {
                            // List watched variables
                            if watched_vars.is_empty() {
                                println!("No variables being watched.");
                                println!("  Use :watch <name> to watch a variable");
                                println!("  Use :unwatch <name> to stop watching a variable");
                            } else {
                                println!("Watched variables:");
                                for name in watched_vars.keys() {
                                    if let Some(val) = symbols.get(name) {
                                        println!("  {} = {}", name, val.to_string(""));
                                    } else {
                                        println!("  {} (variable no longer exists)", name);
                                    }
                                }
                                println!("  Use :unwatch <name> to stop watching a variable");
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        cmd if cmd.starts_with("unwatch ") => {
                            // Parse: :unwatch <name>
                            let var_name = cmd.trim_start_matches("unwatch ").trim();
                            if var_name.is_empty() {
                                eprintln!("Usage: :unwatch <variable_name>");
                                eprintln!("  Example: :unwatch x");
                                eprintln!(
                                    "  Use :watch with no argument to list watched variables"
                                );
                                let _ = rl.add_history_entry(trimmed);
                                continue;
                            }

                            if watched_vars.remove(var_name).is_some() {
                                println!("Stopped watching: {}", var_name);
                            } else {
                                eprintln!("Variable '{}' is not being watched", var_name);
                                eprintln!("  Use :watch to see watched variables");
                            }
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                        _ => {
                            println!(
                                "Unknown command: {}. Type :help for available commands",
                                cmd
                            );
                            let _ = rl.add_history_entry(trimmed);
                            continue;
                        }
                    }
                }

                // Add to input buffer
                if input_buffer.is_empty() {
                    input_buffer = trimmed.to_string();
                } else {
                    input_buffer.push('\n');
                    input_buffer.push_str(trimmed);
                }

                // Check if expression is complete before trying to parse
                if !_is_expression_complete_impl(&input_buffer) {
                    // Expression is incomplete, continue collecting
                    continue;
                }

                // Try to parse and evaluate
                match tokenize(input_buffer.clone()) {
                    Ok(tokens) => {
                        let ast = parse(tokens);
                        match eval(ast.program, &mut symbols, &input_buffer) {
                            Ok(val) => {
                                // Check watched variables for changes (compare string representations)
                                let mut changed_vars: Vec<(String, Value)> = Vec::new();
                                for (name, old_val) in &watched_vars {
                                    if let Some(new_val) = symbols.get(name) {
                                        let old_str = old_val.to_string("");
                                        let new_str = new_val.to_string("");
                                        if old_str != new_str {
                                            println!(
                                                "[WATCH] {} changed: {} -> {}",
                                                name, old_str, new_str
                                            );
                                            changed_vars.push((name.clone(), new_val.clone()));
                                        }
                                    }
                                }
                                // Update watched variables after iteration
                                for (name, val) in changed_vars {
                                    watched_vars.insert(name, val);
                                }
                                // Display result nicely
                                match &val {
                                    Value::FileTemplate { .. } => {
                                        match collect_file_templates(&val, &input_buffer) {
                                            Ok(files) => {
                                                println!("FileTemplate:");
                                                for (path, content) in files {
                                                    println!("  Path: {}", path);
                                                    println!("  Content:\n{}", content);
                                                }
                                            }
                                            Err(_) => {
                                                println!("{}", val.to_string(&input_buffer));
                                            }
                                        }
                                    }
                                    Value::List(items)
                                        if items
                                            .iter()
                                            .any(|v| matches!(v, Value::FileTemplate { .. })) =>
                                    {
                                        match collect_file_templates(&val, &input_buffer) {
                                            Ok(files) => {
                                                println!(
                                                    "List of FileTemplates ({}):",
                                                    files.len()
                                                );
                                                for (path, content) in files {
                                                    println!("  Path: {}", path);
                                                    println!("  Content:\n{}", content);
                                                }
                                            }
                                            Err(_) => {
                                                println!("{}", val.to_string(&input_buffer));
                                            }
                                        }
                                    }
                                    _ => {
                                        let type_name = match &val {
                                            Value::String(_) => "String",
                                            Value::Number(_) => "Number",
                                            Value::Bool(_) => "Bool",
                                            Value::List(_) => "List",
                                            Value::Dict(_) => "Dict",
                                            Value::Function { .. } => "Function",
                                            Value::Builtin(_, _) => "Builtin",
                                            Value::FileTemplate { .. } => "FileTemplate",
                                            Value::Template(_, _) => "Template",
                                            Value::Path(_, _) => "Path",
                                            Value::None => "None",
                                        };
                                        println!(
                                            "{} : {}",
                                            val.to_string(&input_buffer),
                                            type_name
                                        );
                                    }
                                }
                                // Add complete expression to history (for up/down arrow navigation)
                                let _ = rl.add_history_entry(&input_buffer);
                                input_buffer.clear();
                            }
                            Err(e) => {
                                eprintln!(
                                    "Error: {}",
                                    e.pretty_with_file(&input_buffer, Some("<repl>"))
                                );
                                // Don't add failed expressions to history
                                input_buffer.clear();
                            }
                        }
                    }
                    Err(e) => {
                        // Check if it's an incomplete expression
                        let error_msg = e.pretty_with_file(&input_buffer, Some("<repl>"));
                        // If it looks like incomplete input, continue collecting
                        if error_msg.contains("unexpected")
                            || error_msg.contains("EOF")
                            || (error_msg.contains("expected")
                                && (error_msg.contains("in")
                                    || error_msg.contains("then")
                                    || error_msg.contains("else")))
                        {
                            // Continue multi-line input
                            continue;
                        } else {
                            eprintln!("Parse error: {}", error_msg);
                            input_buffer.clear();
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                input_buffer.clear();
            }
            Err(ReadlineError::Eof) => {
                println!("\nGoodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    0
}

// Helper function to check if an expression appears complete
#[cfg(test)]
pub fn is_expression_complete(input: &str) -> bool {
    _is_expression_complete_impl(input)
}

fn _is_expression_complete_impl(input: &str) -> bool {
    let mut let_count = 0;
    let mut in_count = 0;
    let mut if_count = 0;
    let mut then_count = 0;
    let mut else_count = 0;
    let mut paren_depth = 0;
    let mut bracket_depth = 0;
    let mut brace_depth = 0;
    let mut in_string = false;
    let mut in_template = false;
    let mut escape_next = false;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if escape_next {
            escape_next = false;
            i += 1;
            continue;
        }

        if in_string {
            if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if in_template {
            if c == '}' {
                // Check if it's closing a template brace
                let mut j = i;
                let mut brace_count = 1;
                while j > 0 && chars[j - 1] == '{' {
                    j -= 1;
                    brace_count += 1;
                }
                if brace_count >= 2 {
                    in_template = false;
                }
            }
            i += 1;
            continue;
        }

        match c {
            '"' => in_string = true,
            '{' => {
                // Check if it's a template start {{"
                if i + 2 < chars.len() && chars[i + 1] == '{' && chars[i + 2] == '"' {
                    in_template = true;
                    i += 3;
                    continue;
                } else {
                    brace_depth += 1;
                }
            }
            '}' => {
                if !in_template {
                    brace_depth -= 1;
                }
            }
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            '\\' => {
                escape_next = true;
                i += 1;
                continue;
            }
            _ => {}
        }

        // Check for keywords (only when not in string/template)
        if !in_string && !in_template && i + 2 < chars.len() {
            let remaining: String = chars[i..].iter().collect();

            // Check for "let" keyword (word boundary)
            if remaining.starts_with("let")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 3 >= chars.len() || !chars[i + 3].is_alphanumeric())
            {
                let_count += 1;
                i += 3;
                continue;
            }

            // Check for "in" keyword
            if remaining.starts_with("in")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 2 >= chars.len() || !chars[i + 2].is_alphanumeric())
            {
                in_count += 1;
                i += 2;
                continue;
            }

            // Check for "if" keyword
            if remaining.starts_with("if")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 2 >= chars.len() || !chars[i + 2].is_alphanumeric())
            {
                if_count += 1;
                i += 2;
                continue;
            }

            // Check for "then" keyword
            if remaining.starts_with("then")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 4 >= chars.len() || !chars[i + 4].is_alphanumeric())
            {
                then_count += 1;
                i += 4;
                continue;
            }

            // Check for "else" keyword
            if remaining.starts_with("else")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 4 >= chars.len() || !chars[i + 4].is_alphanumeric())
            {
                else_count += 1;
                i += 4;
                continue;
            }
        }

        i += 1;
    }

    // Expression is complete if:
    // - All let statements have matching in
    // - All if statements have matching then and else
    // - All brackets/parens/braces are balanced
    // - Not in the middle of a string or template
    let_count == in_count
        && if_count == then_count
        && if_count == else_count
        && paren_depth == 0
        && bracket_depth == 0
        && brace_depth == 0
        && !in_string
        && !in_template
}
