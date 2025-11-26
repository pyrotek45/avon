use crate::common::Value;
use crate::eval::{
    apply_function, collect_file_templates, eval, fetch_git_raw, initial_builtins,
};
use crate::lexer::tokenize;
use crate::parser::parse;
use std::collections::HashMap;

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
    println!();

    println!("Notes:");
    println!("------");
    println!("  • All functions are curried and support partial application");
    println!("  • Type variables (a, b, acc) represent any type");
    println!("  • Functions use space-separated arguments: f x y, not f(x, y)");
    println!();
    println!("For more examples and tutorials, see: tutorial/TUTORIAL.md");
}

fn print_help() {
    let help = r#"avon — evaluate and generate file templates

Usage: avon <command> [args]

Commands:
  eval <file>        Evaluate a file and print the result
  deploy <file>      Deploy generated templates to disk
  run <code>         Evaluate code string directly
  doc                Show builtin function reference
  version            Show version information

Options:
  --root <dir>       Prepend <dir> to generated file paths (deploy only)
  --force            Overwrite existing files (deploy only)
  --append           Append to existing files instead of overwriting (deploy only)
  --if-not-exists    Only write file if it doesn't exist (deploy only)
  --git <url>        Use git raw URL as source file (for eval/deploy)
  --debug            Enable detailed debug output (lexer/parser/eval)
  -param value       Pass named arguments to main function

Examples:
  avon eval config.av
  avon deploy config.av --root ./output
  avon run 'map (\x x*2) [1,2,3]'
  avon deploy --git user/repo/file.av --root ./out
"#;
    println!("{}", help);
}

#[derive(Debug)]
struct CliOptions {
    root: Option<String>,
    force: bool,
    append: bool,
    if_not_exists: bool,
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
            "--debug" => {
                opts.debug = true;
                i += 1;
            }
            "--git" => {
                if i + 1 < args.len() {
                    opts.git_url = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("--git requires a repo/file argument".to_string());
                }
            }
            s if s.starts_with("-") => {
                let key = s.trim_start_matches('-').to_string();
                if i + 1 < args.len() {
                    opts.named_args.insert(key, args[i + 1].clone());
                    i += 2;
                } else {
                    return Err(format!("named argument {} missing value", key));
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
        return Err("missing file argument (or --git)".to_string());
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
                1
            }
        },
        "deploy" => match parse_args(rest, true) {
            Ok(opts) => execute_deploy(opts),
            Err(e) => {
                eprintln!("Error: {}", e);
                1
            }
        },
        "run" => {
            if rest.is_empty() {
                eprintln!("Error: run requires code string argument");
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
            eprintln!("Failed to fetch git url: {}", e.message);
            1
        })
    } else if let Some(file) = &opts.file {
        std::fs::read_to_string(file)
            .map(|s| (s, file.clone()))
            .map_err(|e| {
                eprintln!("Failed to read file {}: {}", file, e);
                1
            })
    } else {
        eprintln!("No source file provided");
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
                                     eprintln!("missing argument for {}", ident);
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
                                 for (path, content) in files {
                                     let write_path = if let Some(root) = &opts.root {
                                         let rel = path.trim_start_matches('/');
                                         std::path::Path::new(root).join(rel)
                                     } else {
                                         std::path::Path::new(&path).to_path_buf()
                                     };
                                     
                                     let exists = write_path.exists();
                                     
                                     if opts.if_not_exists && exists {
                                         println!("Skipped {} (exists)", write_path.display());
                                         continue;
                                     }
                                     
                                     if exists && !opts.force && !opts.append {
                                         eprintln!("WARNING: File {} exists. Use --force to overwrite or --append to append.", write_path.display());
                                         continue;
                                     }
                                     
                                     if let Some(parent) = write_path.parent() {
                                         std::fs::create_dir_all(parent).ok();
                                     }
                                     
                                     if opts.append && exists {
                                          use std::io::Write;
                                          match std::fs::OpenOptions::new().append(true).open(&write_path) {
                                              Ok(mut f) => {
                                                  if let Err(e) = f.write_all(content.as_bytes()) {
                                                      eprintln!("Failed to append to {}: {}", write_path.display(), e);
                                                      return 1;
                                                  }
                                                  println!("Appended to {}", write_path.display());
                                              },
                                              Err(e) => {
                                                  eprintln!("Failed to open {} for append: {}", write_path.display(), e);
                                                  return 1;
                                              }
                                          }
                                     } else {
                                         if let Err(e) = std::fs::write(&write_path, content) {
                                             eprintln!("Failed to write {}: {}", write_path.display(), e);
                                             return 1;
                                         }
                                         if exists {
                                             println!("Overwrote {}", write_path.display());
                                         } else {
                                             println!("Wrote {}", write_path.display());
                                         }
                                     }
                                 }
                             },
                             Err(_) => {
                                 // If deploy mode but result isn't files, just print string?
                                 // Or error? Original code printed string if error collecting templates
                                 // But collecting templates only errors if value is not list/filetemplate
                                 println!("{}", v.to_string(&source)); 
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
