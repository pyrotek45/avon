use std::collections::HashMap;
use crate::common::{Value, EvalError};
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::eval::{eval, initial_builtins, apply_function, collect_file_templates, fetch_git_raw};

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

    // Type Conversion
    println!("Type Conversion:");
    println!("----------------");
    println!("  to_string    :: a -> String");
    println!("  to_int       :: String -> Int");
    println!("  to_float     :: String -> Float");
    println!("  to_bool      :: a -> Bool");
    println!("  format_int   :: Int -> Int -> String");
    println!("  format_float :: Float -> Int -> String");
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
    println!("  readfile     :: String -> String");
    println!("  readlines    :: String -> [String]");
    println!("  exists       :: String -> Bool");
    println!("  basename     :: String -> String");
    println!("  dirname      :: String -> String");
    println!("  walkdir      :: String -> [String]");
    println!();

    // Data Utilities
    println!("Data Utilities:");
    println!("---------------");
    println!("  json_parse   :: String -> Value");
    println!("  import       :: String -> Value");
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

pub fn run_cli(args: Vec<String>) {
    let mut root_opt: Option<String> = None;
    let mut git_opt: Option<String> = None;
    let mut debug = false;
    let mut cli_args: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                if i + 1 < args.len() {
                    root_opt = Some(args[i + 1].clone());
                    i += 2;
                    continue;
                } else {
                    eprintln!("--root requires a directory argument");
                    return;
                }
            }
            "--git" => {
                if i + 1 < args.len() {
                    git_opt = Some(args[i + 1].clone());
                    i += 2;
                    continue;
                } else {
                    eprintln!("--git requires a repo/file argument");
                    return;
                }
            }
            "--debug" => {
                debug = true;
                i += 1;
                continue;
            }
            _ => {}
        }
        cli_args.push(args[i].clone());
        i += 1;
    }

    if cli_args.len() > 0 {
        // concise help shown for -h/--help (keep short and readable)
        let help = r#"avon — evaluate and generate file templates

Usage: avon [options] <command> [args]

Commands:
  eval <file>        Evaluate a file and print the result
  --deploy           Deploy generated templates (provide args after --deploy)

Options:
  --force            Overwrite files during deploy
  --root <dir>       Prepend <dir> to generated file paths
  --git <repo/file>  Fetch source from git raw URL
  --debug            Enable detailed debug output (lexer/parser/eval)
  --doc              Show all builtin functions (with Haskell-style types)
  -h, --help         Show this brief help
"#;

        if cli_args[0] == "--help" || cli_args[0] == "-h" {
            println!("{}", help);
            return;
        }

        if cli_args[0] == "--doc" {
            print_builtin_docs();
            return;
        }

        if cli_args[0] == "eval" {
            run_eval(cli_args, git_opt, debug);
            return;
        }

        run_deploy_or_eval(cli_args, root_opt, git_opt, debug);
        return;
    }

    // short usage when no args supplied; keep it minimal
    println!("Usage: avon <command> — run 'avon --help' for brief help");
}

fn run_eval(cli_args: Vec<String>, git_opt: Option<String>, debug: bool) {
    if cli_args.len() < 2 {
        eprintln!("eval requires a file path");
        return;
    }
    let filepath = &cli_args[1];
    let mut eval_pos: Vec<String> = vec![];
    let mut eval_named: HashMap<String, String> = HashMap::new();
    if cli_args.len() > 2 {
        let mut it = cli_args.iter().skip(2);
        while let Some(tok) = it.next() {
            if tok.starts_with('-') {
                let key = tok.trim_start_matches('-').to_string();
                if let Some(val) = it.next() {
                    eval_named.insert(key, val.clone());
                } else {
                    eprintln!("named argument {} missing value", key);
                }
            } else {
                eval_pos.push(tok.clone());
            }
        }
    }
    let file_result = if let Some(gspec) = git_opt.as_ref() {
        fetch_git_raw(gspec)
    } else {
        std::fs::read_to_string(filepath).map_err(|e| EvalError::new(format!("failed to read {}: {}", filepath, e), None, None, 0))
    };
    match file_result {
        Ok(file) => {
            if debug {
                eprintln!("[DEBUG] Starting lexer...");
            }
            match tokenize(file.clone()) {
                Ok(tokens) => {
                    if debug {
                        eprintln!("[DEBUG] Lexer produced {} tokens", tokens.len());
                        for (i, tok) in tokens.iter().enumerate() {
                            eprintln!("[DEBUG]   Token {}: {:?}", i, tok);
                        }
                        eprintln!("[DEBUG] Starting parser...");
                    }
                    let ast = parse(tokens);
                    if debug {
                        eprintln!("[DEBUG] Parser produced AST: {:?}", ast);
                        eprintln!("[DEBUG] Starting evaluator...");
                    }
                    let mut symbols = initial_builtins();
                    match eval(ast.program, &mut symbols, &file) {
                        Ok(mut v) => {
                            if !eval_pos.is_empty() || !eval_named.is_empty() {
                                let mut pos_idx: usize = 0;
                                loop {
                                    match &v {
                                        Value::Function { ident, default, .. } => {
                                            if let Some(named_val) = eval_named.remove(ident) {
                                                match apply_function(&v, Value::String(named_val), &file) {
                                                    Ok(nv) => { v = nv; continue; }
                                                    Err(e) => { eprintln!("{}", e.pretty(&file)); break; }
                                                }
                                            } else if pos_idx < eval_pos.len() {
                                                match apply_function(&v, Value::String(eval_pos[pos_idx].clone()), &file) {
                                                    Ok(nv) => { v = nv; pos_idx += 1; continue; }
                                                    Err(e) => { eprintln!("{}", e.pretty(&file)); break; }
                                                }
                                            } else if let Some(def_box) = default {
                                                match apply_function(&v, (**def_box).clone(), &file) {
                                                    Ok(nv) => { v = nv; continue; }
                                                    Err(e) => { eprintln!("{}", e.pretty(&file)); break; }
                                                }
                                            } else {
                                                eprintln!("missing argument for {}", ident);
                                                break;
                                            }
                                        }
                                        Value::Builtin(_, _) => {
                                            if pos_idx < eval_pos.len() {
                                                match apply_function(&v, Value::String(eval_pos[pos_idx].clone()), &file) {
                                                    Ok(nv) => { v = nv; pos_idx += 1; continue; }
                                                    Err(e) => { eprintln!("{}", e.pretty(&file)); break; }
                                                }
                                            } else {
                                                break;
                                            }
                                        }
                                        _ => break,
                                    }
                                }
                            }

                            match collect_file_templates(&v, &file) {
                                Ok(files) => {
                                    for (path, content) in files {
                                        println!("--- {} ---", path);
                                        println!("{}", content);
                                    }
                                }
                                Err(_) => println!("{}", v.to_string(&file)),
                            }
                        }
                        Err(e) => eprintln!("{}", e.pretty(&file)),
                    }
                }
                Err(e) => eprintln!("{}", e.pretty(&file)),
            }
        }
        Err(e) => eprintln!("{}", e.pretty("") ),
    }
}

fn run_deploy_or_eval(cli_args: Vec<String>, root_opt: Option<String>, git_opt: Option<String>, debug: bool) {
    let filepath = &cli_args[0];
    let force = cli_args.iter().any(|s| s == "--force");
    let deploy_idx = cli_args.iter().position(|s| s == "--deploy");
    let deploy_mode = deploy_idx.is_some();
    let mut deploy_pos: Vec<String> = vec![];
    let mut deploy_named: HashMap<String, String> = HashMap::new();
    if let Some(idx) = deploy_idx {
        let mut it = cli_args.iter().skip(idx + 1).filter(|s| !s.starts_with("--"));
        while let Some(tok) = it.next() {
            if tok.starts_with('-') {
                let key = tok.trim_start_matches('-').to_string();
                if let Some(val) = it.next() {
                    deploy_named.insert(key, val.clone());
                } else {
                    eprintln!("named argument {} missing value", key);
                }
            } else {
                deploy_pos.push(tok.clone());
            }
        }
    }

    let file_result = if let Some(gspec) = git_opt.as_ref() {
        fetch_git_raw(gspec)
    } else {
        std::fs::read_to_string(filepath).map_err(|e| EvalError::new(format!("failed to read {}: {}", filepath, e), None, None, 0))
    };

    match file_result {
        Ok(file) => {
            if debug {
                eprintln!("[DEBUG] Starting lexer...");
            }
            match tokenize(file.clone()) {
                Ok(tokens) => {
                    if debug {
                        eprintln!("[DEBUG] Lexer produced {} tokens", tokens.len());
                        for (i, tok) in tokens.iter().enumerate() {
                            eprintln!("[DEBUG]   Token {}: {:?}", i, tok);
                        }
                        eprintln!("[DEBUG] Starting parser...");
                    }
                    let ast = parse(tokens);
                    if debug {
                        eprintln!("[DEBUG] Parser produced AST: {:?}", ast);
                        eprintln!("[DEBUG] Starting evaluator...");
                    }
                    let mut symbols = initial_builtins();

                    match eval(ast.program, &mut symbols, &file) {
                        Ok(v) => {
                            if deploy_mode {
                                let root = root_opt.clone();
                                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    let mut current = v;
                                    let mut pos_idx: usize = 0;
                                    loop {
                                        match &current {
                                            Value::Function { ident, default, .. } => {
                                                if let Some(named_val) = deploy_named.remove(ident) {
                                                    current = apply_function(&current, Value::String(named_val), &file)?;
                                                    continue;
                                                } else if pos_idx < deploy_pos.len() {
                                                    current = apply_function(&current, Value::String(deploy_pos[pos_idx].clone()), &file)?;
                                                    pos_idx += 1;
                                                    continue;
                                                } else if let Some(def_box) = default {
                                                    current = apply_function(&current, (**def_box).clone(), &file)?;
                                                    continue;
                                                } else {
                                                    return Err(EvalError::new(format!("missing argument: {}", ident), None, None, 0));
                                                }
                                            }
                                            Value::Builtin(_, _) => {
                                                if pos_idx < deploy_pos.len() {
                                                    current = apply_function(&current, Value::String(deploy_pos[pos_idx].clone()), &file)?;
                                                    pos_idx += 1;
                                                    continue;
                                                } else {
                                                    break;
                                                }
                                            }
                                            _ => break,
                                        }
                                    }
                                    let files = collect_file_templates(&current, &file)?;
                                    for (path, content) in files {
                                        let write_path = if let Some(rootdir) = root.as_ref() {
                                            let rel = if path.starts_with('/') { path.trim_start_matches('/') } else { &path };
                                            std::path::Path::new(rootdir).join(rel)
                                        } else {
                                            std::path::Path::new(&path).to_path_buf()
                                        };

                                        if write_path.exists() && !force {
                                            return Err(EvalError::new(format!("refusing to overwrite {} (use --force)", write_path.display()), None, None, 0));
                                        }
                                        if let Some(parent) = write_path.parent() {
                                            std::fs::create_dir_all(parent).ok();
                                        }
                                        std::fs::write(&write_path, content)
                                            .map_err(|e| EvalError::new(format!("failed to write file: {}", e), None, None, 0))?;
                                        println!("Wrote {}", write_path.display());
                                    }
                                    Ok::<(), EvalError>(())
                                }));

                                match res {
                                    Ok(Ok(())) => {}
                                    Ok(Err(e)) => eprintln!("Deployment error: {}", e.pretty(&file)),
                                    Err(_) => eprintln!("Deployment panicked"),
                                }
                            } else {
                                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| v.to_string(&file))) {
                                    Ok(s) => println!("{}", s),
                                    Err(_) => eprintln!("Printing result panicked"),
                                }
                            }
                        }
                        Err(err) => eprintln!("{}", err.pretty(&file)),
                    }
                }
                Err(e) => eprintln!("{}", e.pretty(&file)),
            }
        }
        Err(err) => eprintln!("{}", err.pretty("") ),
    }
}
