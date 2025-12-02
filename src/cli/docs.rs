// CLI documentation functions

/// Get documentation for a REPL command
pub fn get_repl_command_doc(cmd_name: &str) -> Option<String> {
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

/// Get documentation for a specific builtin function
pub fn get_builtin_doc(func_name: &str) -> Option<String> {
    let docs: std::collections::HashMap<&str, &str> = [
        // String Operations
        ("concat", "concat :: (String|Template) -> (String|Template) -> String\n  Concatenate two strings.\n  Example: concat \"hello\" \" world\" -> \"hello world\"\n  Note: Both arguments accept both strings and templates (templates auto-convert to strings)"),
        ("upper", "upper :: (String|Template) -> String\n  Convert string to uppercase.\n  Example: upper \"hello\" -> \"HELLO\"\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("lower", "lower :: (String|Template) -> String\n  Convert string to lowercase.\n  Example: lower \"HELLO\" -> \"hello\"\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("trim", "trim :: (String|Template) -> String\n  Remove leading and trailing whitespace.\n  Example: trim \"  hello  \" -> \"hello\"\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("split", "split :: (String|Template) -> (String|Template) -> [String]\n  Split string by delimiter.\n  Example: split \"a,b,c\" \",\" -> [\"a\", \"b\", \"c\"]\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("join", "join :: [String] -> (String|Template) -> String\n  Join list of strings with separator.\n  Example: join [\"a\", \"b\"] \", \" -> \"a, b\"\n  Note: Separator accepts both strings and templates (templates auto-convert to strings)"),
        ("replace", "replace :: (String|Template) -> (String|Template) -> (String|Template) -> String\n  Replace all occurrences of substring.\n  Example: replace \"hello\" \"l\" \"L\" -> \"heLLo\"\n  Note: All arguments accept both strings and templates (templates auto-convert to strings)"),
        ("contains", "contains :: (String|Template) -> (String|Template) -> Bool\n  Check if string contains substring.\n  Example: contains \"hello\" \"ell\" -> true\n  Note: Both arguments accept strings and templates (templates auto-convert to strings)"),
        ("starts_with", "starts_with :: (String|Template) -> (String|Template) -> Bool\n  Check if string starts with prefix.\n  Example: starts_with \"hello\" \"he\" -> true\n  Note: Both arguments accept strings and templates (templates auto-convert to strings)"),
        ("ends_with", "ends_with :: (String|Template) -> (String|Template) -> Bool\n  Check if string ends with suffix.\n  Example: ends_with \"hello\" \"lo\" -> true\n  Note: Both arguments accept strings and templates (templates auto-convert to strings)"),
        ("length", "length :: (String|Template|List) -> Int\n  Get length of string, template, or list.\n  Example: length \"hello\" -> 5, length [1,2,3] -> 3\n  Note: Templates are converted to strings before measuring length"),
        ("repeat", "repeat :: (String|Template) -> Int -> String\n  Repeat string n times.\n  Example: repeat \"x\" 3 -> \"xxx\"\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("pad_left", "pad_left :: (String|Template) -> Int -> (String|Template) -> String\n  Pad string on left to specified length.\n  Example: pad_left \"7\" 3 \"0\" -> \"007\"\n  Note: String and pad char accept both strings and templates (templates auto-convert to strings)"),
        ("pad_right", "pad_right :: (String|Template) -> Int -> (String|Template) -> String\n  Pad string on right to specified length.\n  Example: pad_right \"hi\" 5 \" \" -> \"hi   \"\n  Note: String and pad char accept both strings and templates (templates auto-convert to strings)"),
        ("indent", "indent :: (String|Template) -> Int -> String\n  Indent each line by n spaces.\n  Example: indent \"code\" 4 -> \"    code\"\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_digit", "is_digit :: (String|Template) -> Bool\n  Check if all characters are digits.\n  Example: is_digit \"123\" -> true\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_alpha", "is_alpha :: (String|Template) -> Bool\n  Check if all characters are alphabetic.\n  Example: is_alpha \"abc\" -> true\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_alphanumeric", "is_alphanumeric :: (String|Template) -> Bool\n  Check if all characters are alphanumeric.\n  Example: is_alphanumeric \"abc123\" -> true\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_whitespace", "is_whitespace :: (String|Template) -> Bool\n  Check if all characters are whitespace.\n  Example: is_whitespace \"  \" -> true\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_uppercase", "is_uppercase :: (String|Template) -> Bool\n  Check if all characters are uppercase.\n  Example: is_uppercase \"ABC\" -> true\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_lowercase", "is_lowercase :: (String|Template) -> Bool\n  Check if all characters are lowercase.\n  Example: is_lowercase \"abc\" -> true\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),
        ("is_empty", "is_empty :: (String|Template|List|Dict) -> Bool\n  Check if string, template, list, or dict is empty.\n  Example: is_empty \"\" -> true, is_empty [] -> true, is_empty {} -> true\n  Note: Templates are converted to strings before checking"),

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
        ("slice", "slice :: (String|[a]) -> Int -> Int -> (String|[a])\n  Extract substring or sublist from start (inclusive) to end (exclusive).\n  Example: slice \"hello\" 1 4 -> \"ell\"\n  Example: slice [1, 2, 3, 4, 5] 1 4 -> [2, 3, 4]"),
        ("char_at", "char_at :: String -> Int -> String | None\n  Get character at index (0-based).\n  Example: char_at \"hello\" 2 -> \"l\"\n  Example: char_at \"hello\" 10 -> None"),
        ("chars", "chars :: String -> [String]\n  Convert string to list of single-character strings.\n  Example: chars \"hello\" -> [\"h\", \"e\", \"l\", \"l\", \"o\"]"),
        ("split_at", "split_at :: Int -> [a] -> ([a], [a])\n  Split list at index.\n  Example: split_at 2 [1, 2, 3, 4] -> ([1, 2], [3, 4])"),
        ("partition", "partition :: (a -> Bool) -> [a] -> ([a], [a])\n  Split list into matching and non-matching.\n  Example: partition (\\x x > 2) [1, 2, 3, 4] -> ([3, 4], [1, 2])"),
        ("reverse", "reverse :: [a] -> [a]\n  Reverse list order.\n  Example: reverse [1, 2, 3] -> [3, 2, 1]"),
        ("sort", "sort :: [a] -> [a]\n  Sort list in ascending order (numbers numerically, others lexically).\n  Example: sort [3, 1, 2] -> [1, 2, 3]\n  Example: sort [\"c\", \"a\", \"b\"] -> [\"a\", \"b\", \"c\"]"),
        ("sort_by", "sort_by :: (a -> b) -> [a] -> [a]\n  Sort list by applying function to each element.\n  Example: sort_by (\\x x.age) users -> users sorted by age\n  Example: sort_by lower [\"Bob\", \"alice\", \"Charlie\"] -> case-insensitive sort"),
        ("unique", "unique :: [a] -> [a]\n  Remove duplicate elements (keeps first occurrence).\n  Example: unique [1, 2, 1, 3, 2] -> [1, 2, 3]\n  Example: unique [\"a\", \"b\", \"a\"] -> [\"a\", \"b\"]"),
        ("range", "range :: Int -> Int -> [Int]\n  Generate range of integers from start to end (inclusive).\n  Example: range 1 5 -> [1, 2, 3, 4, 5]\n  Note: Returns empty list if start > end"),
        ("enumerate", "enumerate :: [a] -> [[Int, a]]\n  Add index to each element (returns list of [index, value] pairs).\n  Example: enumerate [\"a\", \"b\", \"c\"] -> [[0, \"a\"], [1, \"b\"], [2, \"c\"]]"),
        ("sum", "sum :: [Number] -> Number\n  Sum all numbers in list.\n  Example: sum [1, 2, 3, 4, 5] -> 15\n  Example: sum [] -> 0\n  Note: Returns Int if all ints, Float if any floats"),
        ("product", "product :: [Number] -> Number\n  Multiply all numbers in list.\n  Example: product [1, 2, 3, 4] -> 24\n  Example: product [] -> 1\n  Note: Returns Int if all ints, Float if any floats"),
        ("min", "min :: [a] -> a | None\n  Find minimum value in list.\n  Works with numbers (mixed int/float) and strings.\n  Example: min [3, 1, 4, 1, 5] -> 1\n  Example: min [\"zebra\", \"apple\", \"banana\"] -> \"apple\"\n  Returns None for empty lists."),
        ("max", "max :: [a] -> a | None\n  Find maximum value in list.\n  Works with numbers (mixed int/float) and strings.\n  Example: max [3, 1, 4, 1, 5] -> 5\n  Example: max [\"zebra\", \"apple\", \"banana\"] -> \"zebra\"\n  Returns None for empty lists."),
        ("all", "all :: (a -> Bool) -> [a] -> Bool\n  Check if all elements satisfy predicate.\n  Returns true if all pass, false if any fail.\n  Example: all (\\x x > 0) [1, 2, 3] -> true\n  Example: all (\\x x > 0) [1, -2, 3] -> false\n  Example: all (\\x x > 0) [] -> true (vacuous truth)"),
        ("any", "any :: (a -> Bool) -> [a] -> Bool\n  Check if any element satisfies predicate.\n  Returns true if any pass, false if all fail.\n  Example: any (\\x x < 0) [1, 2, -3] -> true\n  Example: any (\\x x < 0) [1, 2, 3] -> false\n  Example: any (\\x x < 0) [] -> false"),
        ("count", "count :: (a -> Bool) -> [a] -> Int\n  Count elements matching predicate.\n  Example: count (\\x x > 5) [1, 6, 3, 8, 5] -> 2\n  Example: count (\\x x == \"a\") [\"a\", \"b\", \"a\"] -> 2"),
        ("head", "head :: [a] -> a | None\n  Get first item or None.\n  Example: head [1, 2, 3] -> 1"),
        ("nth", "nth :: Int -> [a] -> a | None\n  Get item at index (0-based) or None if out of bounds.\n  Example: nth 0 [1, 2, 3] -> 1\n  Example: nth 2 [1, 2, 3] -> 3\n  Example: nth 5 [1, 2, 3] -> None"),
        ("tail", "tail :: [a] -> [a]\n  Get all items except first.\n  Example: tail [1, 2, 3] -> [2, 3]"),

        // Regex Functions
        ("regex_match", "regex_match :: String -> String -> Bool\n  Check if text matches regex pattern.\n  First arg: regex pattern\n  Second arg: text to check\n  Example: regex_match \"^\\d+$\" \"123\" -> true"),
        ("regex_replace", "regex_replace :: String -> String -> String -> String\n  Replace all regex matches.\n  First arg: regex pattern\n  Second arg: replacement string\n  Third arg: text to process\n  Example: regex_replace \"\\d\" \"#\" \"a1b2\" -> \"a#b#\""),
        ("regex_split", "regex_split :: String -> String -> [String]\n  Split text by regex pattern.\n  First arg: regex pattern\n  Second arg: text to split\n  Example: regex_split \"\\s+\" \"a b  c\" -> [\"a\", \"b\", \"c\"]"),
        ("scan", "scan :: String -> String -> [String|[String]]\n  Find all regex matches.\n  First arg: regex pattern\n  Second arg: text to scan\n  Example: scan \"\\d+\" \"a12b34\" -> [\"12\", \"34\"]\n  Note: If pattern has capture groups, returns list of lists of groups."),

        // Dict Operations
        ("get", "get :: (Dict|[[String, a]]) -> String -> a | None\n  Get value by key.\n  Works with both dicts and list of pairs (list of 2-element lists).\n  Example: get {a: 1} \"a\" -> 1\n  Example: get [[\"a\", 1], [\"b\", 2]] \"a\" -> 1\n  Note: 'Pairs' is not a type - it's a list of pairs: [[\"key\", value], ...]"),
        ("set", "set :: (Dict|[[String, a]]) -> String -> a -> (Dict|[[String, a]])\n  Update or add key-value pair.\n  Works with both dicts and list of pairs.\n  Example: set {a: 1} \"b\" 2 -> {a: 1, b: 2}\n  Note: 'Pairs' is not a type - it's a list of pairs: [[\"key\", value], ...]"),
        ("keys", "keys :: (Dict|[[String, a]]) -> [String]\n  Get all keys.\n  Works with both dicts and list of pairs.\n  Example: keys {a: 1, b: 2} -> [\"a\", \"b\"]\n  Note: 'Pairs' is not a type - it's a list of pairs: [[\"key\", value], ...]"),
        ("values", "values :: (Dict|[[String, a]]) -> [a]\n  Get all values.\n  Works with both dicts and list of pairs.\n  Example: values {a: 1, b: 2} -> [1, 2]\n  Note: 'Pairs' is not a type - it's a list of pairs: [[\"key\", value], ...]"),
        ("has_key", "has_key :: (Dict|[[String, a]]) -> String -> Bool\n  Check if key exists.\n  Works with both dicts and list of pairs.\n  Example: has_key {a: 1} \"a\" -> true\n  Note: 'Pairs' is not a type - it's a list of pairs: [[\"key\", value], ...]"),
        ("dict_merge", "dict_merge :: Dict -> Dict -> Dict\n  Merge two dictionaries, with second dict values overriding first.\n  Example: dict_merge {a: 1} {b: 2} -> {a: 1, b: 2}\n  Example: dict_merge {a: 1} {a: 2} -> {a: 2}"),

        // Type Conversion
        ("to_string", "to_string :: a -> String\n  Convert value to string.\n  Example: to_string 42 -> \"42\""),
        ("to_int", "to_int :: String -> Int\n  Convert string to integer.\n  Example: to_int \"42\" -> 42"),
        ("to_float", "to_float :: String -> Float\n  Convert string to float.\n  Example: to_float \"3.14\" -> 3.14"),
        ("to_bool", "to_bool :: a -> Bool\n  Convert value to boolean.\n  Example: to_bool \"yes\" -> true"),
        ("to_char", "to_char :: Int -> String\n  Convert Unicode codepoint to character.\n  Example: to_char 72 -> \"H\""),
        ("to_list", "to_list :: String -> [String]\n  Convert string to list of characters.\n  Example: to_list \"Hi\" -> [\"H\", \"i\"]"),
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
        ("markdown_to_html", "markdown_to_html :: (String|Template) -> String\n  Convert markdown text to HTML.\n  Supports headings (# through ######), bold (**text**), italic (*text*), inline code (`code`), paragraphs, and line breaks.\n  Example: markdown_to_html \"# Hello\\nWorld\" -> \"<h1>Hello</h1>\\n<p>World</p>\"\n  Note: Accepts both strings and templates (templates auto-convert to strings)"),

        // File Operations
        ("readfile", "readfile :: String|Path -> String\n  Read entire file as string.\n  Example: readfile \"config.json\" -> file contents\n  Note: Strings can be absolute (safe for reading). Path literals (@...) must be relative."),
        ("readlines", "readlines :: String|Path -> [String]\n  Read file as list of lines.\n  Example: readlines \"file.txt\" -> [\"line1\", \"line2\"]"),
        ("fill_template", "fill_template :: String|Path -> (Dict|[[String, String]]) -> String\n  Read file and fill {{placeholders}} with values.\n  Example: fill_template \"template.txt\" {name: \"Alice\"} -> filled template"),
        ("exists", "exists :: String|Path -> Bool\n  Check if file exists.\n  Example: exists \"config.json\" -> true"),
        ("basename", "basename :: String|Path -> String\n  Get filename from path.\n  Example: basename @config/app.yml -> \"app.yml\""),
        ("dirname", "dirname :: String|Path -> String\n  Get directory from path.\n  Example: dirname @config/app.yml -> \"config\""),
        ("walkdir", "walkdir :: String|Path -> [String]\n  List all files in directory recursively.\n  Example: walkdir \"src\" -> [\"src/file1.txt\", \"src/file2.txt\"]"),

        // Data Utilities
        ("json_parse", "json_parse :: String -> (Dict|List|a)\n  Parse JSON string.\n  Returns Dict for objects, List for arrays, or primitive values.\n  Example: json_parse '{\"a\": 1}' -> {a: 1}\n  Example: json_parse '[1, 2]' -> [1, 2]"),
        ("yaml_parse", "yaml_parse :: String -> (Dict|List|a)\n  Parse YAML string.\n  Returns Dict for mappings, List for sequences, or primitive values.\n  Example: yaml_parse \"a: 1\" -> {a: 1}"),
        ("toml_parse", "toml_parse :: String -> (Dict|List|a)\n  Parse TOML string.\n  Returns Dict for tables, List for arrays, or primitive values.\n  Example: toml_parse \"a = 1\" -> {a: 1}"),
        ("csv_parse", "csv_parse :: String -> [Dict|[String]]\n  Parse CSV file path.\n  Returns list of Dicts (if headers exist) or list of lists (if no headers).\n  Example: csv_parse \"data.csv\" -> [{name: \"Alice\", age: \"30\"}, ...]"),
        ("import", "import :: String|Path -> Value\n  Import and evaluate another Avon file.\n  Example: import \"lib.av\" -> value from lib.av\n  Note: Strings can be absolute (safe for reading). Path literals (@...) must be relative."),

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
        ("is_none", "is_none :: a -> Bool\n  Check if value is None.\n  None is returned by: head on empty list, get on missing key, JSON null values.\n  Example: is_none none -> true\n  Example: is_none (head []) -> true\n  Example: is_none (get {a: 1} \"b\") -> true\n  Pattern: let x = get config \"key\" in if is_none x then default else x"),
        ("not", "not :: Bool -> Bool\n  Logical negation - returns true if false, false if true.\n  Example: not true -> false\n  Example: not false -> true\n  Example: not (1 == 2) -> true\n  Pattern: filter (\\x not (is_empty x)) list"),

        // Assert & Debug
        ("assert", "assert :: Bool -> a -> a\n  Assert condition, return value or error with debug info.\n  Example: assert (is_number x) x\n  Example: assert (x > 0) x\n  Use for input validation and type checking."),
        ("trace", "trace :: String -> a -> a\n  Print label and value to stderr, return value unchanged.\n  Example: trace \"x\" 42 -> prints \"[TRACE] x: 42\" to stderr, returns 42"),
        ("debug", "debug :: a -> a\n  Pretty-print value structure to stderr, return value unchanged.\n  Example: debug [1, 2, 3] -> prints structure, returns [1, 2, 3]"),
        ("error", "error :: String -> a\n  Throw custom error with message.\n  Example: error \"Invalid input\" -> throws error"),

        // Date/Time Operations
        ("now", "now :: String\n  Get current date and time in ISO 8601 format.\n  Example: now -> \"2024-03-15T14:30:45+00:00\"\n  Note: Returns RFC 3339 formatted string with timezone offset."),
        ("date_format", "date_format :: String -> String -> String\n  Format a date string with a custom format.\n  First arg: ISO 8601 date string (from 'now' or 'date_parse')\n  Second arg: strftime format string\n  Example: date_format (now) \"%Y-%m-%d\" -> \"2024-03-15\"\n  Example: date_format (now) \"%B %d, %Y\" -> \"March 15, 2024\"\n  Common formats: %Y (year), %m (month), %d (day), %H (hour), %M (minute), %S (second)\n  See strftime documentation for all format codes."),
        ("date_parse", "date_parse :: String -> String -> String\n  Parse a date string and return ISO 8601 format.\n  First arg: date string to parse\n  Second arg: strftime format string matching the input\n  Example: date_parse \"2024-03-15 14:30\" \"%Y-%m-%d %H:%M\" -> ISO 8601 string\n  Example: date_parse \"15/03/2024\" \"%d/%m/%Y\" -> ISO 8601 string\n  Returns: RFC 3339 formatted string for use with other date functions."),
        ("date_add", "date_add :: String -> String -> String\n  Add duration to a date.\n  First arg: ISO 8601 date string\n  Second arg: duration string (number + unit)\n  Units: s (seconds), m (minutes), h (hours), d (days), w (weeks), y (years)\n  Example: date_add (now) \"1d\" -> date 1 day from now\n  Example: date_add (now) \"2h\" -> date 2 hours from now\n  Example: date_add (now) \"30m\" -> date 30 minutes from now\n  Example: date_add (now) \"1w\" -> date 1 week from now"),
        ("date_diff", "date_diff :: String -> String -> Int\n  Calculate difference between two dates in seconds.\n  First arg: later date (ISO 8601)\n  Second arg: earlier date (ISO 8601)\n  Example: date_diff date1 date2 -> seconds difference\n  Returns: Positive number if first date is after second, negative if before."),
        ("timestamp", "timestamp :: Int\n  Get current Unix timestamp (seconds since epoch).\n  Example: timestamp -> 1710509445\n  Note: Useful for unique filenames and sortable timestamps."),
        ("timezone", "timezone :: String\n  Get current timezone offset.\n  Example: timezone -> \"+00:00\" or \"-05:00\"\n  Note: Returns offset from UTC in ±HH:MM format."),

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

    // Regex Functions
    println!("Regex Functions:");
    println!("----------------");
    println!(
        "  {:<18} :: {}",
        "regex_match", "String -> String -> Bool"
    );
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
    println!(
        "  {:<18} :: {}",
        "json_parse", "String -> (Dict|List|a)"
    );
    println!("                     (Returns Dict for objects, List for arrays)");
    println!(
        "  {:<18} :: {}",
        "yaml_parse", "String -> (Dict|List|a)"
    );
    println!("                     (Returns Dict for mappings, List for sequences)");
    println!(
        "  {:<18} :: {}",
        "toml_parse", "String -> (Dict|List|a)"
    );
    println!("                     (Returns Dict for tables, List for arrays)");
    println!(
        "  {:<18} :: {}",
        "csv_parse", "String -> [Dict|[String]]"
    );
    println!("                     (Returns list of Dicts if headers exist, else list of lists)");
    println!("  {:<18} :: {}", "import", "String|Path -> Value");
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
  doc                Show builtin function reference
  version            Show version information
  help               Show this help message

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
