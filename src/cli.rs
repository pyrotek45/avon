use crate::common::Value;
use crate::eval::{apply_function, collect_file_templates, eval, fetch_git_raw, initial_builtins};
use crate::lexer::tokenize;
use crate::parser::parse;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::collections::HashMap;

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

                                // Step 2: Write all files (if any write fails, deployment is aborted)
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

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Error: Failed to initialize REPL: {}", e);
            return 1;
        }
    };

    let mut symbols = initial_builtins();
    let mut input_buffer = String::new();

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

                // Handle REPL commands
                if trimmed.starts_with(':') {
                    let cmd = trimmed.trim_start_matches(':');
                    match cmd {
                        "help" | "h" => {
                            println!("REPL Commands:");
                            println!("  :help, :h       Show this help");
                            println!("  :doc <name>     Show documentation for a builtin function");
                            println!("  :type <expr>    Show the type of an expression");
                            println!("  :vars           Show all defined variables");
                            println!("  :clear          Clear all user-defined variables");
                            println!("  :exit, :quit    Exit the REPL");
                            println!();
                            println!("Examples:");
                            println!("  map (\\x x * 2) [1, 2, 3]");
                            println!("  let x = 42 in x + 1");
                            println!("  trace \"result\" (1 + 2)");
                            println!("  typeof [1, 2, 3]");
                            continue;
                        }
                        "exit" | "quit" | "q" => {
                            println!("Goodbye!");
                            break;
                        }
                        "clear" => {
                            symbols = initial_builtins();
                            input_buffer.clear();
                            println!("Cleared all user-defined variables");
                            continue;
                        }
                        "vars" => {
                            println!(
                                "Note: In Avon, `let` bindings are scoped to their expression."
                            );
                            println!("      Once a `let ... in` expression completes, bindings are gone.");
                            println!("      The REPL maintains a symbol table, but `let` doesn't add to it.");
                            println!();
                            println!("To persist values in the REPL, evaluate expressions that return values:");
                            println!(
                                "  Example: let x = 42 in x  # Returns 42, but x is not stored"
                            );
                            println!();
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
                            println!("Tip: Use :doc <function> to see detailed documentation for any builtin.");
                            continue;
                        }
                        cmd if cmd.starts_with("doc ") => {
                            let func_name = cmd.trim_start_matches("doc ").trim();
                            if func_name.is_empty() {
                                println!("Usage: :doc <function_name>");
                                println!("  Example: :doc map");
                                println!("  Example: :doc assert");
                                continue;
                            }

                            // Check if it's a builtin
                            let builtins = initial_builtins();
                            if builtins.contains_key(func_name) {
                                if let Some(doc) = get_builtin_doc(func_name) {
                                    println!("{}", doc);
                                } else {
                                    println!("Function: {}", func_name);
                                    println!("  This is a builtin function.");
                                    println!("  Use 'avon doc' to see all builtin documentation.");
                                }
                            } else if symbols.contains_key(func_name) {
                                // User-defined variable/function
                                let val = &symbols[func_name];
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
                                println!("Variable: {}", func_name);
                                println!("  Type: {}", type_info);
                                println!("  Note: This is a user-defined variable in the current REPL session.");
                            } else {
                                println!("Unknown function or variable: {}", func_name);
                                println!("  Use :vars to see available builtins");
                                println!("  Use 'avon doc' to see all builtin documentation");
                            }
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
                            continue;
                        }
                        _ => {
                            println!(
                                "Unknown command: {}. Type :help for available commands",
                                cmd
                            );
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
                                input_buffer.clear();
                            }
                            Err(e) => {
                                eprintln!(
                                    "Error: {}",
                                    e.pretty_with_file(&input_buffer, Some("<repl>"))
                                );
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
