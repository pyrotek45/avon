use crate::common::Value;
use crate::eval::{
    apply_function, collect_file_templates, eval, fetch_git_raw, initial_builtins,
};
use crate::lexer::tokenize;
use crate::parser::parse;
use std::collections::HashMap;
use rustyline::error::ReadlineError;
use rustyline::Editor;

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
    println!("  format_table      :: ([[a]]|Dict) -> String -> String (2D table, also accepts dict)");
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
                    return Err("--git requires a URL argument (format: user/repo/path/to/file.av)".to_string());
                }
            }
            s if s.starts_with("-") => {
                let key = s.trim_start_matches('-').to_string();
                if i + 1 < args.len() {
                    opts.named_args.insert(key, args[i + 1].clone());
                    i += 2;
                } else {
                    return Err(format!("Named argument '{}' requires a value. Use: -{} <value>", key, key));
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
        return Err("Missing required file argument. Use: avon <command> <file> [options]".to_string());
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
                 },
                 Err(e) => {
                     eprintln!("Error: {}", e);
                     1
                 }
            }
        },
        "repl" => {
            execute_repl()
        },
        "doc" | "docs" => {
            print_builtin_docs();
            0
        },
        "version" | "--version" | "-v" => {
            println!("avon 0.1.0");
            0
        },
        "help" | "--help" | "-h" => {
            print_help();
            0
        },
        // Legacy / Convenience
        _ => {
            // If starts with --, it's likely a legacy flag command
            if cmd.starts_with("--") {
                match cmd.as_str() {
                    "--git" => {
                        // Legacy --git implies deploy
                         let mut legacy_args = vec!["--git".to_string()];
                         legacy_args.extend_from_slice(rest);
                         match parse_args(&legacy_args, true) { // require_file=true satisfied by --git
                            Ok(opts) => execute_deploy(opts),
                            Err(e) => { eprintln!("Error: {}", e); 1 }
                         }
                    },
                    "--git-eval" => {
                         let mut legacy_args = vec!["--git".to_string()]; // map to git opt
                         legacy_args.extend_from_slice(rest);
                         match parse_args(&legacy_args, true) {
                            Ok(opts) => execute_eval(opts),
                            Err(e) => { eprintln!("Error: {}", e); 1 }
                         }
                    },
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
                            },
                            Err(e) => { eprintln!("Error: {}", e); 1 }
                        }
                    },
                    "--doc" => { print_builtin_docs(); 0 },
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
                let filtered_rest: Vec<String> = rest.iter().filter(|s| *s != "--deploy").cloned().collect();
                
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
                    },
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
                    eprintln!("  Tip: Use 'avon eval {}' to test if the file is valid", file);
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

fn process_source(
    source: String,
    source_name: String,
    opts: CliOptions,
    deploy_mode: bool,
) -> i32 {
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
                                     match apply_function(&v, Value::String(named_val.clone()), &source, 0) {
                                         Ok(nv) => { v = nv; continue; },
                                         Err(e) => { eprintln!("{}", e.pretty_with_file(&source, Some(&source_name))); return 1; }
                                     }
                                 } else if pos_idx < opts.pos_args.len() {
                                     match apply_function(&v, Value::String(opts.pos_args[pos_idx].clone()), &source, 0) {
                                         Ok(nv) => { v = nv; pos_idx += 1; continue; },
                                         Err(e) => { eprintln!("{}", e.pretty_with_file(&source, Some(&source_name))); return 1; }
                                     }
                                 } else if let Some(def_box) = default {
                                     match apply_function(&v, (**def_box).clone(), &source, 0) {
                                         Ok(nv) => { v = nv; continue; },
                                         Err(e) => { eprintln!("{}", e.pretty_with_file(&source, Some(&source_name))); return 1; }
                                     }
                                 } else {
                                     eprintln!("Error: Missing required argument: {}", ident);
                                     eprintln!("  The program expects an argument named '{}'", ident);
                                     if !opts.named_args.is_empty() || !opts.pos_args.is_empty() {
                                         eprintln!("  Provided arguments: {:?}", opts.named_args.keys().collect::<Vec<_>>());
                                         if !opts.pos_args.is_empty() {
                                             eprintln!("  Positional arguments: {:?}", opts.pos_args);
                                         }
                                     }
                                     eprintln!("  Usage: avon deploy {} -{} <value>", source_name, ident);
                                     eprintln!("  Example: avon deploy {} -{} myvalue", source_name, ident);
                                     return 1;
                                 }
                             },
                             Value::Builtin(_, _) => {
                                 if pos_idx < opts.pos_args.len() {
                                     match apply_function(&v, Value::String(opts.pos_args[pos_idx].clone()), &source, 0) {
                                         Ok(nv) => { v = nv; pos_idx += 1; continue; },
                                         Err(e) => { eprintln!("{}", e.pretty_with_file(&source, Some(&source_name))); return 1; }
                                     }
                                 } else {
                                     break;
                                 }
                             },
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
                                let mut prepared_files: Vec<(std::path::PathBuf, String, bool, bool)> = Vec::new();
                                for (path, content) in &files {
                                    let write_path = if let Some(root) = &opts.root {
                                        let rel = path.trim_start_matches('/');
                                        // SECURITY: Normalize path to prevent directory traversal attacks
                                        let normalized = std::path::Path::new(rel)
                                            .components()
                                            .filter(|c| match c {
                                                std::path::Component::ParentDir => false, // Block ".."
                                                std::path::Component::RootDir => false,    // Block absolute paths
                                                _ => true,
                                            })
                                            .collect::<std::path::PathBuf>();
                                        let full_path = std::path::Path::new(root).join(normalized);
                                        // SECURITY: Ensure the resolved path is still within the root directory
                                        let root_path = std::path::Path::new(root).canonicalize()
                                            .unwrap_or_else(|_| std::path::Path::new(root).to_path_buf());
                                        let resolved = full_path.canonicalize()
                                            .unwrap_or_else(|_| full_path.clone());
                                        if !resolved.starts_with(&root_path) {
                                            eprintln!("Error: Path traversal detected: {}", path);
                                            eprintln!("  Attempted path would escape --root directory");
                                            eprintln!("  Deployment aborted.");
                                            return 1;
                                        }
                                        resolved
                                    } else {
                                        // SECURITY: Without --root, validate absolute paths don't contain ".."
                                        let path_buf = std::path::Path::new(&path).to_path_buf();
                                        if path_buf.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
                                            eprintln!("Error: Path contains '..' which is not allowed without --root");
                                            eprintln!("  Use --root to safely contain file operations");
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
                                             eprintln!("Error: Failed to create directory: {}", parent.display());
                                             eprintln!("  Reason: {}", e);
                                             if e.kind() == std::io::ErrorKind::PermissionDenied {
                                                 eprintln!("  Tip: Check directory permissions");
                                                 eprintln!("  Tip: Try using a different --root directory");
                                             } else if e.kind() == std::io::ErrorKind::NotFound {
                                                 eprintln!("  Tip: Check that the parent path exists");
                                             }
                                             eprintln!("Deployment aborted. No files were written.");
                                             return 1;
                                         }
                                     }
                                    
                                    prepared_files.push((write_path, content.clone(), exists, should_backup));
                                }
                                
                                // Step 2: Write all files (if any write fails, deployment is aborted)
                                let mut written_files = Vec::new();
                                for (write_path, content, exists, should_backup) in prepared_files {
                                    // Perform backup if needed
                                    if should_backup {
                                        let mut backup_name = write_path.file_name().unwrap().to_os_string();
                                        backup_name.push(".bak");
                                        let backup_path = write_path.with_file_name(backup_name);
                                        
                                        if let Err(e) = std::fs::copy(&write_path, &backup_path) {
                                            eprintln!("Error: Failed to create backup: {}", backup_path.display());
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
                                        match std::fs::OpenOptions::new().append(true).open(&write_path) {
                                            Ok(mut f) => {
                                                if let Err(e) = f.write_all(content.as_bytes()) {
                                                    eprintln!("Error: Failed to append to file: {}", write_path.display());
                                                    eprintln!("  Reason: {}", e);
                                                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                                                        eprintln!("  Tip: Check file permissions");
                                                    } else if e.kind() == std::io::ErrorKind::OutOfMemory {
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
                                            },
                                            Err(e) => {
                                                eprintln!("Error: Failed to open file for append: {}", write_path.display());
                                                eprintln!("  Reason: {}", e);
                                                if e.kind() == std::io::ErrorKind::PermissionDenied {
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
                                            eprintln!("Error: Failed to write file: {}", write_path.display());
                                            eprintln!("  Reason: {}", e);
                                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                                eprintln!("  Tip: Check file permissions");
                                                eprintln!("  Tip: Try using a different --root directory");
                                            } else if e.kind() == std::io::ErrorKind::NotFound {
                                                eprintln!("  Tip: Check that the parent directory exists");
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
                             },
                             Err(e) => {
                                 // In deploy mode, if the result isn't deployable (not FileTemplate or list), error out
                                 eprintln!("Error: Deployment failed - result is not deployable");
                                 eprintln!("  The program evaluated successfully, but the result cannot be deployed.");
                                 eprintln!("  Expected: FileTemplate or list of FileTemplates");
                                 eprintln!("  Got: {}", v.to_string(&source));
                                 eprintln!("  Details: {}", e.message);
                                 eprintln!("");
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
                            },
                            Err(_) => {
                                println!("{}", v.to_string(&source));
                            }
                        }
                    }
                    0
                },
                Err(e) => {
                    eprintln!("{}", e.pretty_with_file(&source, Some(&source_name)));
                    1
                }
            }
        },
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

    let mut rl = match Editor::<()>::new() {
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
            "avon> ".to_string()  // 6 chars: "avon" (4) + ">" (1) + " " (1)
        } else {
            "    > ".to_string()  // 6 chars: 4 spaces + ">" + " " = matches "avon> " exactly, aligns "let"
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
                            let user_vars: Vec<_> = symbols
                                .iter()
                                .filter(|(k, _)| {
                                    !matches!(k.as_str(), 
                                        "concat" | "map" | "filter" | "fold" | "import" |
                                        "readfile" | "exists" | "basename" | "dirname" | "readlines" |
                                        "upper" | "lower" | "trim" | "split" | "join" | "replace" |
                                        "contains" | "starts_with" | "ends_with" | "length" | "repeat" |
                                        "pad_left" | "pad_right" | "indent" | "is_digit" | "is_alpha" |
                                        "is_alphanumeric" | "is_whitespace" | "is_uppercase" | "is_lowercase" |
                                        "is_empty" | "html_escape" | "html_tag" | "html_attr" |
                                        "md_heading" | "md_link" | "md_code" | "md_list" |
                                        "dict_get" | "dict_set" | "dict_has_key" | "to_string" |
                                        "to_int" | "to_float" | "to_bool" | "neg" | "format_int" |
                                        "format_float" | "format_hex" | "format_octal" | "format_binary" |
                                        "format_scientific" | "format_bytes" | "format_list" | "format_table" |
                                        "format_json" | "format_currency" | "format_percent" | "format_bool" |
                                        "truncate" | "center" | "flatmap" | "flatten" | "get" | "set" |
                                        "keys" | "values" | "has_key" | "typeof" | "is_string" | "is_number" |
                                        "is_int" | "is_float" | "is_list" | "is_bool" | "is_function" | "is_dict" |
                                        "assert" | "error" | "trace" | "debug" | "os" | "env_var" | "env_var_or" |
                                        "json_parse" | "fill_template" | "walkdir")
                                })
                                .collect();
                            
                            if user_vars.is_empty() {
                                println!("No user-defined variables");
                            } else {
                                println!("User-defined variables:");
                                for (name, val) in user_vars {
                                    let type_info = match val {
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
                                    println!("  {} : {}", name, type_info);
                                }
                            }
                            continue;
                        }
                        cmd if cmd.starts_with("doc ") => {
                            let func_name = cmd.trim_start_matches("doc ").trim();
                            // Show builtin doc for specific function
                            let builtins = initial_builtins();
                            if builtins.contains_key(func_name) {
                                println!("Function: {}", func_name);
                                // Try to get type info from print_builtin_docs logic
                                // For now, just indicate it exists
                                println!("  This is a builtin function. Use it in an expression to see its behavior.");
                                println!("  Example: {} <args>", func_name);
                            } else {
                                println!("Unknown function: {}", func_name);
                                println!("  Use :vars to see available variables");
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
                                    eprintln!("Parse error: {}", e.pretty_with_file(expr_str, Some("<input>")));
                                }
                            }
                            continue;
                        }
                        _ => {
                            println!("Unknown command: {}. Type :help for available commands", cmd);
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
                                    Value::List(items) if items.iter().any(|v| matches!(v, Value::FileTemplate { .. })) => {
                                        match collect_file_templates(&val, &input_buffer) {
                                            Ok(files) => {
                                                println!("List of FileTemplates ({}):", files.len());
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
                                        println!("{} : {}", val.to_string(&input_buffer), type_name);
                                    }
                                }
                                input_buffer.clear();
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e.pretty_with_file(&input_buffer, Some("<repl>")));
                                input_buffer.clear();
                            }
                        }
                    }
                    Err(e) => {
                        // Check if it's an incomplete expression
                        let error_msg = e.pretty_with_file(&input_buffer, Some("<repl>"));
                        // If it looks like incomplete input, continue collecting
                        if error_msg.contains("unexpected") || error_msg.contains("EOF") || 
                           (error_msg.contains("expected") && (error_msg.contains("in") || error_msg.contains("then") || error_msg.contains("else"))) {
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
            if remaining.starts_with("let") && (i == 0 || !chars[i - 1].is_alphanumeric()) {
                if i + 3 >= chars.len() || !chars[i + 3].is_alphanumeric() {
                    let_count += 1;
                    i += 3;
                    continue;
                }
            }
            
            // Check for "in" keyword
            if remaining.starts_with("in") && (i == 0 || !chars[i - 1].is_alphanumeric()) {
                if i + 2 >= chars.len() || !chars[i + 2].is_alphanumeric() {
                    in_count += 1;
                    i += 2;
                    continue;
                }
            }
            
            // Check for "if" keyword
            if remaining.starts_with("if") && (i == 0 || !chars[i - 1].is_alphanumeric()) {
                if i + 2 >= chars.len() || !chars[i + 2].is_alphanumeric() {
                    if_count += 1;
                    i += 2;
                    continue;
                }
            }
            
            // Check for "then" keyword
            if remaining.starts_with("then") && (i == 0 || !chars[i - 1].is_alphanumeric()) {
                if i + 4 >= chars.len() || !chars[i + 4].is_alphanumeric() {
                    then_count += 1;
                    i += 4;
                    continue;
                }
            }
            
            // Check for "else" keyword
            if remaining.starts_with("else") && (i == 0 || !chars[i - 1].is_alphanumeric()) {
                if i + 4 >= chars.len() || !chars[i + 4].is_alphanumeric() {
                    else_count += 1;
                    i += 4;
                    continue;
                }
            }
        }
        
        i += 1;
    }
    
    // Expression is complete if:
    // - All let statements have matching in
    // - All if statements have matching then and else
    // - All brackets/parens/braces are balanced
    // - Not in the middle of a string or template
    let_count == in_count &&
    if_count == then_count && if_count == else_count &&
    paren_depth == 0 &&
    bracket_depth == 0 &&
    brace_depth == 0 &&
    !in_string &&
    !in_template
}
