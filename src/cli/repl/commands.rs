use crate::cli::commands::process_source;
use crate::cli::docs::{get_builtin_doc, get_repl_command_doc};
use crate::cli::helpers::print_file_content;
use crate::cli::options::CliOptions;
use crate::cli::repl::state::ReplState;
use crate::common::{Number, Value};
use crate::eval::{apply_function, collect_file_templates, eval, fetch_git_raw, initial_builtins};
use crate::lexer::tokenize;
use crate::parser::parse;
use rustyline::history::{DefaultHistory, History};
use rustyline::Editor;
use std::collections::HashMap;

use super::AvonHelper;

pub fn handle_command(
    cmd: &str,
    state: &mut ReplState,
    rl: &mut Editor<AvonHelper, DefaultHistory>,
) -> Option<bool> {
    match cmd {
        "help" | "h" => {
            println!("REPL Commands:");
            println!("  :help, :h       Show this help");
            println!("  :let <name> = <expr>  Store a value");
            println!("  :vars           List all stored variables");
            println!("  :inspect <name> Show detailed variable info");
            println!("  :unlet <name>   Remove a variable");
            println!("  :read <file>    Read and display file contents (supports --git)");
            println!(
                "  :run <file>     Evaluate file and display result (supports --git, --debug)"
            );
            println!(
                "  :eval <file>    Evaluate file and merge Dict keys into REPL (supports --git)"
            );
            println!("  :preview <file> [flags...]  Preview deployment (supports --git, --debug, -param)");
            println!("  :deploy <file> [flags...]   Deploy a file (supports --git, --root, --force, etc.)");
            println!("  :deploy-expr <expr> [flags...]  Deploy expression result");
            println!("  :edit [file]    Open file in $EDITOR (or last edited file)");
            println!("  :write <file> <expr>  Write expression result to file");
            println!("  :source <file>  Execute commands from a file");
            println!("  :history [N]    Show command history (last N entries)");
            println!("  :clear-history  Clear command history");
            println!("  :save-session <file>  Save REPL state to file");
            println!("  :load-session <file>  Load REPL state from file");
            println!("  :assert <expr>  Assert that expression is true");
            println!("  :test <expr> <expected>  Test that expression equals expected");
            println!("  :time <expr>    Measure expression evaluation time");
            println!("  :benchmark <expr>        Alias for :time");
            println!("  :benchmark-file <file>   Measure file evaluation time");
            println!("  :watch <name>   Watch a variable for changes");
            println!("  :unwatch <name> Stop watching a variable");
            println!("  :report         Show session report (commands, variables, etc.)");
            println!("  :pwd            Show current working directory");
            println!("  :list [dir]     List directory contents");
            println!("  :cd <dir>       Change working directory");
            println!("  :sh <command>   Execute shell command");
            println!("  :doc            Show all builtin functions and REPL commands");
            println!("  :doc <name>     Show documentation for a builtin or command");
            println!("  :type <expr>    Show the type of an expression");
            println!("  :clear          Clear all user-defined variables");
            println!("  :exit, :quit    Exit the REPL");
            println!();
            println!("Common Flags (for :read, :run, :eval, :preview, :deploy):");
            println!("  --git <url>     Fetch from GitHub (format: user/repo/path/file.av)");
            println!("  --debug         Enable debug output");
            println!("  --root <dir>    Set output directory (deploy only)");
            println!("  --force         Overwrite existing files (deploy only)");
            println!("  --backup        Create .bak backup (deploy only)");
            println!("  -param value    Pass named parameter");
            println!();
            println!("Navigation:");
            println!("  ↑/↓             Navigate command history");
            println!("  Tab             Auto-complete commands, functions, files");
            println!("  Ctrl+A          Move to beginning of line");
            println!("  Ctrl+E          Move to end of line");
            println!("  Ctrl+K          Delete from cursor to end of line");
            println!("  Ctrl+U          Delete from cursor to beginning of line");
            println!("  Ctrl+W          Delete word backward");
            println!("  Ctrl+L          Clear screen");
            println!();
            println!("Features:");
            println!("  • Syntax highlighting (colors for keywords, builtins, strings)");
            println!("  • Multi-line input (incomplete expressions continue on next line)");
            println!("  • Tab completion for builtins, variables, and files");
            println!();
            println!("Examples:");
            println!("  map (\\x x * 2) [1, 2, 3]");
            println!("  let x = 42 in x + 1");
            println!("  :run --git user/repo/example.av");
            println!("  :deploy config.av --root ./out --backup");
            Some(false)
        }
        "exit" | "quit" | "q" => {
            println!("Goodbye!");
            Some(true)
        }
        "clear" => {
            state.symbols = initial_builtins();
            state.sync_symbols();
            state.input_buffer.clear();
            println!("Cleared all user-defined variables");
            Some(false)
        }
        "vars" => {
            let builtins = initial_builtins();
            let user_vars: Vec<(&String, &Value)> = state
                .symbols
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
                        Value::List(l) => {
                            println!("  {} : {} = [{} items]", name, type_name, l.len())
                        }
                        Value::Dict(d) => {
                            println!("  {} : {} = {{ {} keys }}", name, type_name, d.len())
                        }
                        _ => println!("  {} : {}", name, type_name),
                    }
                }
            }
            Some(false)
        }
        "let" => {
            eprintln!("Error: Missing variable name and expression");
            eprintln!("Usage: :let <name> = <expression>");
            eprintln!("  Example: :let x = 42");
            eprintln!("  Example: :let config = {{host: \"localhost\", port: 8080}}");
            eprintln!(
                "  Note: You must include an '=' sign between the variable name and expression"
            );
            Some(false)
        }
        cmd if cmd.starts_with("let ") => {
            let rest = cmd.trim_start_matches("let ").trim();
            if rest.is_empty() {
                eprintln!("Error: Missing variable name and expression");
                eprintln!("Usage: :let <name> = <expression>");
                eprintln!("  Example: :let x = 42");
                eprintln!("  Example: :let config = {{host: \"localhost\", port: 8080}}");
                eprintln!(
                    "  Note: You must include an '=' sign between the variable name and expression"
                );
                return Some(false);
            }
            if let Some(equals_pos) = rest.find('=') {
                let var_name = rest[..equals_pos].trim();
                let expr_str = rest[equals_pos + 1..].trim();

                if var_name.is_empty() {
                    eprintln!("Error: Variable name cannot be empty");
                    eprintln!("Usage: :let <name> = <expression>");
                    eprintln!("  Example: :let x = 42");
                    eprintln!("  Example: :let config = {{host: \"localhost\", port: 8080}}");
                    return Some(false);
                }
                if expr_str.is_empty() {
                    eprintln!("Error: Expression cannot be empty");
                    eprintln!("Usage: :let <name> = <expression>");
                    eprintln!("  Example: :let x = 42");
                    eprintln!("  Example: :let config = {{host: \"localhost\", port: 8080}}");
                    return Some(false);
                }

                match tokenize(expr_str.to_string()) {
                    Ok(tokens) => {
                        let ast = parse(tokens);
                        match eval(ast.program, &mut state.symbols, expr_str) {
                            Ok(val) => {
                                let was_watched = state.watched_vars.contains_key(var_name);
                                let old_watched_val = state.watched_vars.get(var_name).cloned();

                                state.symbols.insert(var_name.to_string(), val.clone());
                                state.sync_symbols();

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
                                    state.watched_vars.insert(var_name.to_string(), val.clone());
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
                eprintln!("Error: Missing '=' sign between variable name and expression");
                eprintln!("Usage: :let <name> = <expression>");
                eprintln!("  Example: :let x = 42");
                eprintln!("  Example: :let config = {{host: \"localhost\", port: 8080}}");
                eprintln!(
                    "  Note: The '=' sign is required to separate the variable name from its value"
                );
            }
            Some(false)
        }
        "doc" => {
            println!("Available builtin functions (use :doc <name> for details):");
            let builtins = initial_builtins();
            let builtin_names: Vec<&String> = builtins.keys().collect();
            let mut sorted_names: Vec<&str> = builtin_names.iter().map(|s| s.as_str()).collect();
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
            Some(false)
        }
        "inspect" => {
            eprintln!("Error: Missing variable name");
            eprintln!("Usage: :inspect <variable_name>");
            eprintln!("  Example: :inspect x");
            eprintln!("  Note: This shows detailed information about a variable");
            Some(false)
        }
        cmd if cmd.starts_with("inspect ") => {
            let var_name = cmd.trim_start_matches("inspect ").trim();
            if var_name.is_empty() {
                eprintln!("Error: Missing variable name");
                eprintln!("Usage: :inspect <variable_name>");
                eprintln!("  Example: :inspect x");
                eprintln!("  Note: This shows detailed information about a variable");
                return Some(false);
            }

            if let Some(val) = state.symbols.get(var_name) {
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
            Some(false)
        }
        "unlet" => {
            eprintln!("Error: Missing variable name");
            eprintln!("Usage: :unlet <variable_name>");
            eprintln!("  Example: :unlet x");
            eprintln!("  Note: This removes a user-defined variable (cannot remove builtins)");
            Some(false)
        }
        cmd if cmd.starts_with("unlet ") => {
            let var_name = cmd.trim_start_matches("unlet ").trim();
            if var_name.is_empty() {
                eprintln!("Error: Missing variable name");
                eprintln!("Usage: :unlet <variable_name>");
                eprintln!("  Example: :unlet x");
                eprintln!("  Note: This removes a user-defined variable (cannot remove builtins)");
                return Some(false);
            }

            let builtins = initial_builtins();
            if builtins.contains_key(var_name) {
                eprintln!("Error: Cannot remove builtin function '{}'", var_name);
                return Some(false);
            }

            if state.symbols.remove(var_name).is_some() {
                state.sync_symbols();
                println!("Removed variable: {}", var_name);
            } else {
                eprintln!("Variable '{}' not found", var_name);
                eprintln!("  Use :vars to see available variables");
            }
            Some(false)
        }
        cmd if cmd.starts_with("doc ") => {
            let name = cmd.trim_start_matches("doc ").trim();
            if name.is_empty() {
                println!("Usage: :doc <name>");
                println!("  Example: :doc map");
                println!("  Example: :doc pwd");
                println!("  Example: :doc read");
                return Some(false);
            }

            if let Some(doc) = get_repl_command_doc(name) {
                println!("{}", doc);
                return Some(false);
            }

            let builtins = initial_builtins();
            if builtins.contains_key(name) {
                if let Some(doc) = get_builtin_doc(name) {
                    println!("{}", doc);
                } else {
                    println!("Function: {}", name);
                    println!("  This is a builtin function.");
                    println!("  Use 'avon doc' to see all builtin documentation.");
                }
            } else if state.symbols.contains_key(name) {
                let val = &state.symbols[name];
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
            Some(false)
        }
        "type" => {
            eprintln!("Error: Missing expression");
            eprintln!("Usage: :type <expression>");
            eprintln!("  Example: :type [1, 2, 3]");
            eprintln!("  Example: :type \"hello\"");
            eprintln!("  Note: This shows the type of an expression");
            Some(false)
        }
        cmd if cmd.starts_with("type ") => {
            let expr_str = cmd.trim_start_matches("type ").trim();
            if expr_str.is_empty() {
                eprintln!("Error: Missing expression");
                eprintln!("Usage: :type <expression>");
                eprintln!("  Example: :type [1, 2, 3]");
                eprintln!("  Example: :type \"hello\"");
                eprintln!("  Note: This shows the type of an expression");
                return Some(false);
            }
            match tokenize(expr_str.to_string()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    let mut temp_symbols = state.symbols.clone();
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
            Some(false)
        }
        "read" => {
            eprintln!("Error: Missing file path");
            eprintln!("Usage: :read <file_path> [--git]");
            eprintln!("  Example: :read config.av");
            eprintln!("  Example: :read --git user/repo/path/file.av");
            eprintln!("  Note: This displays the contents of the specified file");
            Some(false)
        }
        cmd if cmd.starts_with("read ") => {
            let rest = cmd.trim_start_matches("read ").trim();
            if rest.is_empty() {
                eprintln!("Error: Missing file path");
                eprintln!("Usage: :read <file_path> [--git]");
                eprintln!("  Example: :read config.av");
                eprintln!("  Example: :read --git user/repo/path/file.av");
                eprintln!("  Note: This displays the contents of the specified file");
                return Some(false);
            }

            let (file_path, use_git) = if rest.starts_with("--git ") {
                (rest.trim_start_matches("--git ").trim(), true)
            } else if rest.ends_with(" --git") {
                (rest.trim_end_matches(" --git").trim(), true)
            } else {
                (rest, false)
            };

            let content = if use_git {
                match fetch_git_raw(file_path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error fetching from git '{}': {}", file_path, e.message);
                        eprintln!("  Tip: URL format should be: user/repo/path/to/file.av");
                        return Some(false);
                    }
                }
            } else {
                match std::fs::read_to_string(file_path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error reading file '{}': {}", file_path, e);
                        if e.kind() == std::io::ErrorKind::NotFound {
                            eprintln!("  Tip: Check that the file exists and the path is correct");
                        } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                            eprintln!("  Tip: Check file permissions");
                        }
                        return Some(false);
                    }
                }
            };

            print_file_content(file_path, &content);
            Some(false)
        }
        "run" => {
            eprintln!("Error: Missing file path");
            eprintln!("Usage: :run <file_path> [--git] [--debug]");
            eprintln!("  Example: :run config.av");
            eprintln!("  Example: :run --git user/repo/path/file.av");
            eprintln!("  Note: This evaluates a file without modifying REPL state");
            Some(false)
        }
        cmd if cmd.starts_with("run ") => {
            let rest = cmd.trim_start_matches("run ").trim();
            if rest.is_empty() {
                eprintln!("Error: Missing file path");
                eprintln!("Usage: :run <file_path> [--git] [--debug]");
                eprintln!("  Example: :run config.av");
                eprintln!("  Example: :run --git user/repo/path/file.av");
                eprintln!("  Note: This evaluates a file without modifying REPL state");
                return Some(false);
            }

            let parts: Vec<&str> = rest.split_whitespace().collect();
            let mut use_git = false;
            let mut _debug = false;
            let mut file_parts: Vec<&str> = Vec::new();

            for part in parts {
                match part {
                    "--git" => use_git = true,
                    "--debug" => _debug = true,
                    _ => file_parts.push(part),
                }
            }

            let file_path = file_parts.join(" ");
            if file_path.is_empty() {
                eprintln!("Error: Missing file path");
                return Some(false);
            }

            let (source, source_name) = if use_git {
                match fetch_git_raw(&file_path) {
                    Ok(s) => (s, file_path.clone()),
                    Err(e) => {
                        eprintln!("Error fetching from git '{}': {}", file_path, e.message);
                        eprintln!("  Tip: URL format should be: user/repo/path/to/file.av");
                        return Some(false);
                    }
                }
            } else {
                match std::fs::read_to_string(&file_path) {
                    Ok(s) => (s, file_path.clone()),
                    Err(e) => {
                        eprintln!("Error reading file '{}': {}", file_path, e);
                        if e.kind() == std::io::ErrorKind::NotFound {
                            eprintln!("  Tip: Check that the file exists and the path is correct");
                        } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                            eprintln!("  Tip: Check file permissions");
                        }
                        return Some(false);
                    }
                }
            };

            match tokenize(source.clone()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    let mut temp_symbols = initial_builtins();
                    match eval(ast.program, &mut temp_symbols, &source) {
                        Ok(val) => match &val {
                            Value::FileTemplate { .. } => {
                                match collect_file_templates(&val, &source) {
                                    Ok(files) => {
                                        println!("FileTemplate:");
                                        for (path, content) in files {
                                            println!("  Path: {}", path);
                                            println!("  Content:\n{}", content);
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error: {}", e.message);
                                    }
                                }
                            }
                            Value::List(l) => {
                                let mut all_are_file_templates = true;
                                for item in l {
                                    if !matches!(item, Value::FileTemplate { .. }) {
                                        all_are_file_templates = false;
                                        break;
                                    }
                                }
                                if all_are_file_templates {
                                    match collect_file_templates(&val, &source) {
                                        Ok(files) => {
                                            println!("FileTemplates ({} files):", files.len());
                                            for (path, content) in files {
                                                println!("  Path: {}", path);
                                                println!("  Content:\n{}", content);
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Error: {}", e.message);
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
                        },
                        Err(e) => {
                            eprintln!("Error: {}", e.pretty_with_file(&source, Some(&source_name)));
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Parse error: {}",
                        e.pretty_with_file(&source, Some(&source_name))
                    );
                }
            }
            Some(false)
        }
        "eval" => {
            eprintln!("Error: Missing file path");
            eprintln!("Usage: :eval <file_path> [--git] [-arg value] [positional...]");
            eprintln!("  Example: :eval config.av");
            eprintln!("  Example: :eval config.av -env prod -port 8080");
            eprintln!("  Example: :eval --git user/repo/path/file.av -env dev");
            eprintln!(
                "  Note: If the file evaluates to a Dict, its keys will be added to REPL state"
            );
            eprintln!("  Note: If the file evaluates to a Function, arguments are applied");
            Some(false)
        }
        cmd if cmd.starts_with("eval ") => {
            let rest = cmd.trim_start_matches("eval ").trim();
            if rest.is_empty() {
                eprintln!("Error: Missing file path");
                eprintln!("Usage: :eval <file_path> [--git] [-arg value] [positional...]");
                eprintln!("  Example: :eval config.av");
                eprintln!("  Example: :eval config.av -env prod -port 8080");
                eprintln!("  Example: :eval --git user/repo/path/file.av -env dev");
                eprintln!(
                    "  Note: If the file evaluates to a Dict, its keys will be added to REPL state"
                );
                eprintln!("  Note: If the file evaluates to a Function, arguments are applied");
                return Some(false);
            }

            let parts: Vec<&str> = rest.split_whitespace().collect();
            let mut use_git = false;
            let mut file_path_parts: Vec<&str> = Vec::new();
            let mut named_args: HashMap<String, String> = HashMap::new();
            let mut pos_args: Vec<String> = Vec::new();

            let mut i = 0;
            while i < parts.len() {
                match parts[i] {
                    "--git" => {
                        use_git = true;
                        i += 1;
                    }
                    s if s.starts_with("-") && !s.starts_with("--") => {
                        let key = s.trim_start_matches('-').to_string();
                        if i + 1 < parts.len() {
                            named_args.insert(key, parts[i + 1].to_string());
                            i += 2;
                        } else {
                            eprintln!("Error: Named argument '{}' requires a value", s);
                            return Some(false);
                        }
                    }
                    s => {
                        if file_path_parts.is_empty() {
                            file_path_parts.push(s);
                        } else {
                            pos_args.push(s.to_string());
                        }
                        i += 1;
                    }
                }
            }

            let file_path = file_path_parts.join(" ");
            if file_path.is_empty() {
                eprintln!("Error: Missing file path");
                return Some(false);
            }

            let (source, source_name) = if use_git {
                match fetch_git_raw(&file_path) {
                    Ok(s) => (s, file_path.clone()),
                    Err(e) => {
                        eprintln!("Error fetching from git '{}': {}", file_path, e.message);
                        eprintln!("  Tip: URL format should be: user/repo/path/to/file.av");
                        return Some(false);
                    }
                }
            } else {
                match std::fs::read_to_string(&file_path) {
                    Ok(s) => (s, file_path.clone()),
                    Err(e) => {
                        eprintln!("Error reading file '{}': {}", file_path, e);
                        if e.kind() == std::io::ErrorKind::NotFound {
                            eprintln!("  Tip: Check that the file exists and the path is correct");
                        } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                            eprintln!("  Tip: Check file permissions");
                        }
                        return Some(false);
                    }
                }
            };

            match tokenize(source.clone()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    let mut temp_symbols = initial_builtins();
                    match eval(ast.program, &mut temp_symbols, &source) {
                        Ok(mut val) => {
                            let mut pos_idx = 0;
                            loop {
                                match &val {
                                    Value::Function { ident, default, .. } => {
                                        if let Some(named_val) = named_args.get(ident) {
                                            match apply_function(
                                                &val,
                                                Value::String(named_val.clone()),
                                                &source,
                                                0,
                                            ) {
                                                Ok(nv) => {
                                                    val = nv;
                                                    continue;
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error: {}",
                                                        e.pretty_with_file(
                                                            &source,
                                                            Some(&source_name)
                                                        )
                                                    );
                                                    break;
                                                }
                                            }
                                        } else if pos_idx < pos_args.len() {
                                            match apply_function(
                                                &val,
                                                Value::String(pos_args[pos_idx].clone()),
                                                &source,
                                                0,
                                            ) {
                                                Ok(nv) => {
                                                    val = nv;
                                                    pos_idx += 1;
                                                    continue;
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error: {}",
                                                        e.pretty_with_file(
                                                            &source,
                                                            Some(&source_name)
                                                        )
                                                    );
                                                    break;
                                                }
                                            }
                                        } else if let Some(def_box) = default {
                                            match apply_function(
                                                &val,
                                                (**def_box).clone(),
                                                &source,
                                                0,
                                            ) {
                                                Ok(nv) => {
                                                    val = nv;
                                                    continue;
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error: {}",
                                                        e.pretty_with_file(
                                                            &source,
                                                            Some(&source_name)
                                                        )
                                                    );
                                                    break;
                                                }
                                            }
                                        } else {
                                            eprintln!(
                                                "Error: Missing required argument: {}",
                                                ident
                                            );
                                            eprintln!(
                                                "  Usage: :eval {} -{} <value>",
                                                source_name, ident
                                            );
                                            break;
                                        }
                                    }
                                    Value::Builtin(_, _) => {
                                        if pos_idx < pos_args.len() {
                                            match apply_function(
                                                &val,
                                                Value::String(pos_args[pos_idx].clone()),
                                                &source,
                                                0,
                                            ) {
                                                Ok(nv) => {
                                                    val = nv;
                                                    pos_idx += 1;
                                                    continue;
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error: {}",
                                                        e.pretty_with_file(
                                                            &source,
                                                            Some(&source_name)
                                                        )
                                                    );
                                                    break;
                                                }
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                    _ => break,
                                }
                            }

                            if let Value::Dict(d) = &val {
                                let mut added = 0;
                                for (key, value) in d.iter() {
                                    state.symbols.insert(key.clone(), value.clone());
                                    added += 1;
                                }
                                state.sync_symbols();
                                println!(
                                    "Evaluated '{}': {} keys added to REPL",
                                    source_name, added
                                );
                            } else {
                                let val_str = val.to_string("");
                                println!("Result: {}", val_str);
                                if !use_git {
                                    println!(
                                        "  Note: Use :let <name> = import \"{}\" to store this value",
                                        source_name
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e.pretty_with_file(&source, Some(&source_name)));
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Parse error: {}",
                        e.pretty_with_file(&source, Some(&source_name))
                    );
                }
            }
            Some(false)
        }
        "preview" => {
            eprintln!("Error: Missing file path");
            eprintln!("Usage: :preview <file_path> [flags...]");
            eprintln!("  Example: :preview config.av");
            eprintln!("  Example: :preview --git user/repo/config.av");
            eprintln!("  Example: :preview config.av --debug");
            eprintln!("  Note: This shows what would be deployed without writing files");
            Some(false)
        }
        cmd if cmd.starts_with("preview ") => {
            let rest = cmd.trim_start_matches("preview ").trim();
            if rest.is_empty() {
                eprintln!("Error: Missing file path");
                eprintln!("Usage: :preview <file_path> [flags...]");
                eprintln!("  Example: :preview config.av");
                eprintln!("  Example: :preview --git user/repo/config.av");
                return Some(false);
            }

            let parts: Vec<&str> = rest.split_whitespace().collect();
            let mut use_git = false;
            let mut file_parts: Vec<&str> = Vec::new();

            let mut i = 0;
            while i < parts.len() {
                match parts[i] {
                    "--git" => {
                        use_git = true;
                        i += 1;
                    }
                    "--debug" => {
                        i += 1;
                    }
                    _ => {
                        file_parts.push(parts[i]);
                        i += 1;
                    }
                }
            }

            let file_path = file_parts.join(" ");
            if file_path.is_empty() {
                eprintln!("Error: Missing file path");
                return Some(false);
            }

            let (source, source_name) = if use_git {
                match fetch_git_raw(&file_path) {
                    Ok(s) => (s, file_path.clone()),
                    Err(e) => {
                        eprintln!("Error fetching from git '{}': {}", file_path, e.message);
                        eprintln!("  Tip: URL format should be: user/repo/path/to/file.av");
                        return Some(false);
                    }
                }
            } else {
                match std::fs::read_to_string(&file_path) {
                    Ok(s) => (s, file_path.clone()),
                    Err(e) => {
                        eprintln!("Error reading file '{}': {}", file_path, e);
                        return Some(false);
                    }
                }
            };

            match tokenize(source.clone()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    let mut temp_symbols = initial_builtins();
                    match eval(ast.program, &mut temp_symbols, &source) {
                        Ok(val) => match collect_file_templates(&val, &source) {
                            Ok(files) => {
                                println!("Would deploy {} file(s):", files.len());
                                for (path, content) in files {
                                    println!("--- {} ---", path);
                                    println!("{}", content);
                                }
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e.message);
                                eprintln!(
                                    "  Result is not a FileTemplate or list of FileTemplates"
                                );
                            }
                        },
                        Err(e) => {
                            eprintln!("Error: {}", e.pretty_with_file(&source, Some(&source_name)));
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Parse error: {}",
                        e.pretty_with_file(&source, Some(&source_name))
                    );
                }
            }
            Some(false)
        }
        "deploy" => {
            eprintln!("Error: Missing file path");
            eprintln!("Usage: :deploy <file_path> [flags...]");
            eprintln!("  Flags: --git, --root <dir>, --force, --backup, --append, --if-not-exists, --debug");
            eprintln!("  Example: :deploy config.av --root ./output --backup");
            eprintln!("  Example: :deploy --git user/repo/config.av --root ./out");
            eprintln!("  Note: This deploys FileTemplates from the file to disk");
            Some(false)
        }
        cmd if cmd.starts_with("deploy ") => {
            let rest = cmd.trim_start_matches("deploy ").trim();
            let parts: Vec<&str> = rest.split_whitespace().collect();

            if parts.is_empty() {
                eprintln!("Error: Missing file path");
                eprintln!("Usage: :deploy <file_path> [flags...]");
                eprintln!("  Flags: --git, --root <dir>, --force, --backup, --append, --if-not-exists, --debug");
                eprintln!("  Example: :deploy config.av --root ./output --backup");
                eprintln!("  Example: :deploy --git user/repo/config.av --root ./out");
                eprintln!("  Note: This deploys FileTemplates from the file to disk");
                return Some(false);
            }

            let mut deploy_opts = CliOptions::new();
            let mut use_git = false;
            let mut file_path_parts: Vec<&str> = Vec::new();

            let mut i = 0;
            while i < parts.len() {
                match parts[i] {
                    "--git" => {
                        use_git = true;
                        i += 1;
                    }
                    "--root" => {
                        if i + 1 < parts.len() {
                            deploy_opts.root = Some(parts[i + 1].to_string());
                            i += 2;
                        } else {
                            eprintln!("Error: --root requires a directory argument");
                            return Some(false);
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
                        let key = s.trim_start_matches('-').to_string();
                        if i + 1 < parts.len() {
                            deploy_opts.named_args.insert(key, parts[i + 1].to_string());
                            i += 2;
                        } else {
                            eprintln!("Error: Named argument '{}' requires a value", s);
                            return Some(false);
                        }
                    }
                    s => {
                        if deploy_opts.file.is_none() {
                            file_path_parts.push(s);
                        } else {
                            deploy_opts.pos_args.push(s.to_string());
                        }
                        i += 1;
                    }
                }
            }

            let file_path = file_path_parts.join(" ");
            if file_path.is_empty() {
                eprintln!("Error: Missing file path");
                return Some(false);
            }
            deploy_opts.file = Some(file_path.clone());

            let (source, source_name) = if use_git {
                match fetch_git_raw(&file_path) {
                    Ok(s) => (s, file_path.clone()),
                    Err(e) => {
                        eprintln!("Error fetching from git '{}': {}", file_path, e.message);
                        return Some(false);
                    }
                }
            } else {
                match std::fs::read_to_string(&file_path) {
                    Ok(s) => (s, file_path.clone()),
                    Err(e) => {
                        eprintln!("Error reading file '{}': {}", file_path, e);
                        return Some(false);
                    }
                }
            };

            let result = process_source(source, source_name, deploy_opts, true);

            if result == 0 {
                println!("Deployment completed successfully");
            }
            Some(false)
        }
        "deploy-expr" => {
            eprintln!("Error: Missing expression");
            eprintln!("Usage: :deploy-expr <expression> [--root <dir>]");
            eprintln!("  Example: :deploy-expr @test.txt {{\"Hello\"}}");
            eprintln!("  Example: :deploy-expr config --root ./output");
            eprintln!(
                "  Note: The expression must evaluate to a FileTemplate or list of FileTemplates"
            );
            eprintln!("  Note: --root is required for deployment");
            Some(false)
        }
        cmd if cmd.starts_with("deploy-expr ") => {
            let rest = cmd.trim_start_matches("deploy-expr ").trim();

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
                eprintln!("Error: Missing expression");
                eprintln!("Usage: :deploy-expr <expression> [--root <dir>]");
                eprintln!("  Example: :deploy-expr @test.txt {{\"Hello\"}}");
                eprintln!("  Example: :deploy-expr config --root ./output");
                eprintln!("  Note: The expression must evaluate to a FileTemplate or list of FileTemplates");
                eprintln!("  Note: --root is required for deployment");
                return Some(false);
            }

            match tokenize(expr_str.to_string()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    match eval(ast.program, &mut state.symbols, expr_str) {
                        Ok(val) => match collect_file_templates(&val, expr_str) {
                            Ok(files) => {
                                if files.is_empty() {
                                    eprintln!(
                                        "Error: Expression does not produce any FileTemplates"
                                    );
                                    return Some(false);
                                }

                                let root_path = if let Some(root_str) = root_dir {
                                    std::path::Path::new(&root_str).to_path_buf()
                                } else {
                                    eprintln!("Error: --root is required for :deploy-expr");
                                    eprintln!("  Usage: :deploy-expr <expr> --root <dir>");
                                    return Some(false);
                                };

                                if let Err(e) = std::fs::create_dir_all(&root_path) {
                                    eprintln!("Error: Failed to create root directory: {}", e);
                                    return Some(false);
                                }

                                let mut success = true;
                                for (path, content) in &files {
                                    let rel = path.trim_start_matches('/');
                                    let full_path = root_path.join(rel);

                                    if let Some(parent) = full_path.parent() {
                                        if let Err(e) = std::fs::create_dir_all(parent) {
                                            eprintln!("Error: Failed to create directory: {}", e);
                                            success = false;
                                            break;
                                        }
                                    }

                                    if let Err(e) = std::fs::write(&full_path, content) {
                                        eprintln!(
                                            "Error: Failed to write {}: {}",
                                            full_path.display(),
                                            e
                                        );
                                        success = false;
                                        break;
                                    }
                                    println!("Deployed: {}", full_path.display());
                                }

                                if success {
                                    println!("Deployment completed successfully");
                                } else {
                                    eprintln!(
                                        "Deployment failed - some files may have been written"
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e.message);
                                eprintln!("  Expression result is not a FileTemplate or list of FileTemplates");
                            }
                        },
                        Err(e) => {
                            eprintln!("Error: {}", e.pretty_with_file(expr_str, Some("<repl>")));
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
            Some(false)
        }
        "history" => {
            let history = rl.history();
            let entries: Vec<String> = history.iter().map(|s| s.to_string()).collect();
            let count = entries.len();
            if count == 0 {
                println!("No command history yet.");
            } else {
                println!("Command history ({} entries):", count);
                let start = count.saturating_sub(50);
                for (i, entry) in entries.iter().enumerate().skip(start) {
                    println!("  {}: {}", i + 1, entry);
                }
            }
            Some(false)
        }
        "write" => {
            eprintln!("Error: Missing file path and expression");
            eprintln!("Usage: :write <file_path> <expression>");
            eprintln!("  Example: :write output.txt \"Hello, world!\"");
            eprintln!("  Example: :write result.json (json_parse config)");
            eprintln!("  Note: You must provide both a file path and an expression to write");
            Some(false)
        }
        cmd if cmd.starts_with("write ") => {
            let rest = cmd.trim_start_matches("write ").trim();
            if rest.is_empty() {
                eprintln!("Error: Missing file path and expression");
                eprintln!("Usage: :write <file_path> <expression>");
                eprintln!("  Example: :write output.txt \"Hello, world!\"");
                eprintln!("  Example: :write result.json (json_parse config)");
                eprintln!("  Note: You must provide both a file path and an expression to write");
                return Some(false);
            }

            let parts: Vec<&str> = rest.splitn(2, char::is_whitespace).collect();
            if parts.len() < 2 {
                eprintln!("Error: Missing expression after file path");
                eprintln!("Usage: :write <file_path> <expression>");
                eprintln!("  Example: :write output.txt \"Hello, world!\"");
                eprintln!("  Example: :write result.json (json_parse config)");
                eprintln!("  Note: The expression will be evaluated and written to the file");
                return Some(false);
            }

            let file_path = parts[0];
            let expr_str = parts[1];

            match tokenize(expr_str.to_string()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    match eval(ast.program, &mut state.symbols, expr_str) {
                        Ok(val) => {
                            let content = val.to_string("");
                            match std::fs::write(file_path, content) {
                                Ok(_) => {
                                    println!("Written to: {}", file_path);
                                }
                                Err(e) => {
                                    eprintln!("Error writing to '{}': {}", file_path, e);
                                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                                        eprintln!("  Tip: Check file permissions");
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e.pretty_with_file(expr_str, Some("<repl>")));
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
            Some(false)
        }
        "save-session" => {
            eprintln!("Error: Missing file path");
            eprintln!("Usage: :save-session <file_path>");
            eprintln!("  Example: :save-session my_session.avon");
            eprintln!("  Note: This will save all user-defined variables to the specified file");
            eprintln!("  Use :load-session to restore variables from a saved file");
            Some(false)
        }
        cmd if cmd.starts_with("save-session ") => {
            let file_path = cmd.trim_start_matches("save-session ").trim();
            if file_path.is_empty() {
                eprintln!("Error: Missing file path");
                eprintln!("Usage: :save-session <file_path>");
                eprintln!("  Example: :save-session my_session.avon");
                eprintln!(
                    "  Note: This will save all user-defined variables to the specified file"
                );
                eprintln!("  Use :load-session to restore variables from a saved file");
                return Some(false);
            }

            let builtins = initial_builtins();
            let user_vars: Vec<(&String, &Value)> = state
                .symbols
                .iter()
                .filter(|(name, _)| !builtins.contains_key(*name))
                .collect();

            let var_count = user_vars.len();
            if var_count == 0 {
                eprintln!("No user-defined variables to save.");
                return Some(false);
            }

            let mut session_code = String::from("# Avon REPL Session\n");
            session_code.push_str("# Saved variables:\n\n");
            let mut saved_count = 0;
            let mut dict_entries = Vec::new();

            for (name, val) in &user_vars {
                let val_str = match val {
                    Value::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
                    Value::Number(Number::Int(i)) => i.to_string(),
                    Value::Number(Number::Float(f)) => f.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::List(_) => val.to_string(""),
                    Value::Dict(_) => val.to_string(""),
                    Value::Function { .. } => {
                        eprintln!(
                            "Warning: Cannot save function '{}' (functions cannot be serialized)",
                            name
                        );
                        continue;
                    }
                    _ => {
                        eprintln!(
                            "Warning: Cannot save variable '{}' (type not serializable)",
                            name
                        );
                        continue;
                    }
                };
                session_code.push_str(&format!("let {} = {} in\n", name, val_str));
                dict_entries.push(format!("{}: {}", name, name));
                saved_count += 1;
            }

            if saved_count > 0 {
                session_code.push_str(&format!("{{{}}}\n", dict_entries.join(", ")));
            }

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
            Some(false)
        }
        "load-session" => {
            eprintln!("Error: Missing file path");
            eprintln!("Usage: :load-session <file_path>");
            eprintln!("  Example: :load-session my_session.avon");
            eprintln!("  Note: This will load variables from a file saved with :save-session");
            Some(false)
        }
        cmd if cmd.starts_with("load-session ") => {
            let file_path = cmd.trim_start_matches("load-session ").trim();
            if file_path.is_empty() {
                eprintln!("Error: Missing file path");
                eprintln!("Usage: :load-session <file_path>");
                eprintln!("  Example: :load-session my_session.avon");
                eprintln!("  Note: This will load variables from a file saved with :save-session");
                return Some(false);
            }

            match std::fs::read_to_string(file_path) {
                Ok(source) => match tokenize(source.clone()) {
                    Ok(tokens) => {
                        let ast = parse(tokens);
                        let mut temp_symbols = initial_builtins();
                        match eval(ast.program, &mut temp_symbols, &source) {
                            Ok(val) => {
                                let builtins = initial_builtins();
                                let mut loaded = 0;

                                if let Value::Dict(d) = val {
                                    for (name, value) in d.iter() {
                                        if !builtins.contains_key(name) {
                                            state.symbols.insert(name.clone(), value.clone());
                                            loaded += 1;
                                        }
                                    }
                                } else {
                                    for (name, val) in temp_symbols.iter() {
                                        if !builtins.contains_key(name) {
                                            state.symbols.insert(name.clone(), val.clone());
                                            loaded += 1;
                                        }
                                    }
                                }

                                state.sync_symbols();
                                println!(
                                    "Session loaded from: {} ({} variables restored)",
                                    file_path, loaded
                                );
                            }
                            Err(e) => {
                                eprintln!(
                                    "Error evaluating session file: {}",
                                    e.pretty_with_file(&source, Some(file_path))
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
                },
                Err(e) => {
                    eprintln!("Error reading session file '{}': {}", file_path, e);
                }
            }
            Some(false)
        }
        "assert" => {
            eprintln!("Error: Missing expression");
            eprintln!("Usage: :assert <expression>");
            eprintln!("  Example: :assert (x > 0)");
            eprintln!("  Example: :assert true");
            eprintln!("  Note: The expression must evaluate to a boolean (true or false)");
            Some(false)
        }
        cmd if cmd.starts_with("assert ") => {
            let expr_str = cmd.trim_start_matches("assert ").trim();
            if expr_str.is_empty() {
                eprintln!("Error: Missing expression");
                eprintln!("Usage: :assert <expression>");
                eprintln!("  Example: :assert (x > 0)");
                eprintln!("  Example: :assert true");
                eprintln!("  Note: The expression must evaluate to a boolean (true or false)");
                return Some(false);
            }

            match tokenize(expr_str.to_string()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    match eval(ast.program, &mut state.symbols, expr_str) {
                        Ok(val) => match val {
                            Value::Bool(true) => {
                                println!("✓ PASS: Assertion passed");
                            }
                            Value::Bool(false) => {
                                eprintln!(
                                    "✗ FAIL: Assertion failed (expression evaluated to false)"
                                );
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
                            eprintln!("✗ FAIL: {}", e.pretty_with_file(expr_str, Some("<repl>")));
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
            Some(false)
        }
        "benchmark" => {
            eprintln!("Error: Missing expression");
            eprintln!("Usage: :benchmark <expression>");
            eprintln!("  Example: :benchmark map (\\x x * 2) [1..1000]");
            eprintln!("  Example: :benchmark map double [1..1000]");
            eprintln!("  Note: This command measures the evaluation time of an expression");
            Some(false)
        }
        cmd if cmd.starts_with("benchmark ") => {
            let rest = cmd.trim_start_matches("benchmark ").trim();
            if rest.is_empty() {
                eprintln!("Error: Missing expression");
                eprintln!("Usage: :benchmark <expression>");
                eprintln!("  Example: :benchmark map (\\x x * 2) [1..1000]");
                eprintln!("  Example: :benchmark map double [1..1000]");
                eprintln!("  Note: This command measures the evaluation time of an expression");
                return Some(false);
            }

            let start = std::time::Instant::now();
            match tokenize(rest.to_string()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    match eval(ast.program, &mut state.symbols, rest) {
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
                            eprintln!("Error: {}", e.pretty_with_file(rest, Some("<repl>")));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Parse error: {}", e.pretty_with_file(rest, Some("<repl>")));
                }
            }
            Some(false)
        }
        "benchmark-file" => {
            eprintln!("Error: Missing file path");
            eprintln!("Usage: :benchmark-file <file_path>");
            eprintln!("  Example: :benchmark-file config.av");
            eprintln!("  Example: :benchmark-file large_program.av");
            eprintln!("  Note: This command measures the evaluation time of a file");
            Some(false)
        }
        cmd if cmd.starts_with("benchmark-file ") => {
            let rest = cmd.trim_start_matches("benchmark-file ").trim();
            if rest.is_empty() {
                eprintln!("Usage: :benchmark-file <file>");
                eprintln!("  Example: :benchmark-file config.av");
                eprintln!("  Example: :benchmark-file large_program.av");
                return Some(false);
            }

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
                                    eprintln!("Error: {}", e.pretty_with_file(&source, Some(rest)));
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Parse error: {}", e.pretty_with_file(&source, Some(rest)));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", rest, e);
                }
            }
            Some(false)
        }
        "test" => {
            eprintln!("Error: Missing expression and expected value");
            eprintln!("Usage: :test <expression> <expected_value>");
            eprintln!("  Example: :test (double 21) 42");
            eprintln!("  Example: :test (length [1,2,3]) 3");
            eprintln!("  Note: This compares the expression result with the expected value");
            Some(false)
        }
        cmd if cmd.starts_with("test ") => {
            let rest = cmd.trim_start_matches("test ").trim();
            if rest.is_empty() {
                eprintln!("Error: Missing expression and expected value");
                eprintln!("Usage: :test <expression> <expected_value>");
                eprintln!("  Example: :test (double 21) 42");
                eprintln!("  Example: :test (length [1,2,3]) 3");
                eprintln!("  Note: This compares the expression result with the expected value");
                return Some(false);
            }

            let parts: Vec<&str> = rest.rsplitn(2, char::is_whitespace).collect();
            if parts.len() < 2 {
                eprintln!("Error: Missing expected value after expression");
                eprintln!("Usage: :test <expression> <expected_value>");
                eprintln!("  Example: :test (double 21) 42");
                eprintln!("  Example: :test (length [1,2,3]) 3");
                eprintln!("  Note: Separate the expression and expected value with a space");
                return Some(false);
            }

            let expected_str = parts[0];
            let expr_str = parts[1];

            let expr_result = match tokenize(expr_str.to_string()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    eval(ast.program, &mut state.symbols, expr_str)
                }
                Err(e) => {
                    eprintln!(
                        "Parse error in expression: {}",
                        e.pretty_with_file(expr_str, Some("<repl>"))
                    );
                    return Some(false);
                }
            };

            let expected_result = match tokenize(expected_str.to_string()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    eval(ast.program, &mut state.symbols, expected_str)
                }
                Err(e) => {
                    eprintln!(
                        "Parse error in expected value: {}",
                        e.pretty_with_file(expected_str, Some("<repl>"))
                    );
                    return Some(false);
                }
            };

            match (expr_result, expected_result) {
                (Ok(expr_val), Ok(expected_val)) => {
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
                                ll.iter().zip(rl.iter()).all(|(l, r)| match (l, r) {
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
                                    _ => l.to_string("") == r.to_string(""),
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
            Some(false)
        }
        cmd if cmd.starts_with("list ") => {
            let dir_path = cmd.trim_start_matches("list ").trim();
            let dir = if dir_path.is_empty() { "." } else { dir_path };

            match std::fs::read_dir(dir) {
                Ok(entries) => {
                    let mut files: Vec<String> = Vec::new();
                    for entry in entries {
                        match entry {
                            Ok(e) => {
                                let name = e.file_name().to_string_lossy().to_string();
                                let path = e.path();
                                let file_type = if path.is_dir() { "/" } else { "" };
                                files.push(format!("{}{}", name, file_type));
                            }
                            Err(e) => {
                                eprintln!("Error reading directory entry: {}", e);
                            }
                        }
                    }
                    files.sort();
                    let current_dir =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    println!("Directory '{}' (current: {}):", dir, current_dir.display());
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
            Some(false)
        }
        "list" => {
            let current_dir =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            match std::fs::read_dir(&current_dir) {
                Ok(entries) => {
                    let mut files: Vec<String> = Vec::new();
                    for entry in entries {
                        match entry {
                            Ok(e) => {
                                let name = e.file_name().to_string_lossy().to_string();
                                let path = e.path();
                                let file_type = if path.is_dir() { "/" } else { "" };
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
            Some(false)
        }
        "cd" => {
            eprintln!("Error: Missing directory path");
            eprintln!("Usage: :cd <directory>");
            eprintln!("  Example: :cd ./examples");
            eprintln!("  Example: :cd /tmp");
            eprintln!("  Note: This changes the current working directory");
            Some(false)
        }
        cmd if cmd.starts_with("cd ") => {
            let dir_path = cmd.trim_start_matches("cd ").trim();
            if dir_path.is_empty() {
                eprintln!("Error: Missing directory path");
                eprintln!("Usage: :cd <directory>");
                eprintln!("  Example: :cd ./examples");
                eprintln!("  Example: :cd /tmp");
                eprintln!("  Note: This changes the current working directory");
                return Some(false);
            }

            match std::env::set_current_dir(dir_path) {
                Ok(_) => {
                    let current_dir =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    println!("Changed directory to: {}", current_dir.display());
                }
                Err(e) => {
                    eprintln!("Error changing directory to '{}': {}", dir_path, e);
                }
            }
            Some(false)
        }
        "pwd" => {
            match std::env::current_dir() {
                Ok(dir) => {
                    println!("{}", dir.display());
                }
                Err(e) => {
                    eprintln!("Error getting current directory: {}", e);
                }
            }
            Some(false)
        }
        cmd if cmd.starts_with("sh ") => {
            let shell_cmd = cmd.trim_start_matches("sh ").trim();
            if shell_cmd.is_empty() {
                eprintln!("Error: Missing shell command");
                eprintln!("Usage: :sh <command>");
                eprintln!("  Example: :sh ls -la");
                eprintln!("  Example: :sh echo hello");
                eprintln!("  Note: This executes a single shell command");
                return Some(false);
            }

            match std::process::Command::new("sh")
                .arg("-c")
                .arg(shell_cmd)
                .status()
            {
                Ok(status) => {
                    if status.success() {
                        println!("\x1b[1;32m✓\x1b[0m Shell command completed successfully");
                    } else {
                        match status.code() {
                            Some(code) => {
                                eprintln!(
                                    "\x1b[1;31m✗\x1b[0m Shell command failed with exit code: {}",
                                    code
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => {
                                eprintln!("\x1b[1;31m✗\x1b[0m Shell command terminated by signal");
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("\x1b[1;31m✗\x1b[0m Error executing command: {}", e);
                }
            }
            Some(false)
        }
        "sh" => {
            eprintln!("Usage: :sh <command>");
            eprintln!("  Example: :sh ls -la");
            eprintln!("  Example: :sh echo hello");
            eprintln!("  Note: This executes a single shell command. For interactive shell, exit REPL and use your terminal.");
            Some(false)
        }
        cmd if cmd.starts_with("watch ") => {
            let var_name = cmd.trim_start_matches("watch ").trim();
            if var_name.is_empty() {
                eprintln!("Error: Missing variable name");
                eprintln!("Usage: :watch <variable_name>");
                eprintln!("  Example: :watch x");
                eprintln!("  Note: Use :watch with no argument to list watched variables");
                eprintln!("  Note: This will show a message when the variable's value changes");
                return Some(false);
            }

            if let Some(val) = state.symbols.get(var_name) {
                state.watched_vars.insert(var_name.to_string(), val.clone());
                println!("Watching: {} = {}", var_name, val.to_string(""));
            } else {
                eprintln!("Variable '{}' not found", var_name);
                eprintln!("  Use :vars to see available variables");
            }
            Some(false)
        }
        "watch" => {
            if state.watched_vars.is_empty() {
                println!("No variables being watched.");
                println!("  Use :watch <name> to watch a variable");
                println!("  Use :unwatch <name> to stop watching a variable");
            } else {
                println!("Watched variables:");
                for name in state.watched_vars.keys() {
                    if let Some(val) = state.symbols.get(name) {
                        println!("  {} = {}", name, val.to_string(""));
                    } else {
                        println!("  {} (variable no longer exists)", name);
                    }
                }
                println!("  Use :unwatch <name> to stop watching a variable");
            }
            Some(false)
        }
        cmd if cmd.starts_with("unwatch ") => {
            let var_name = cmd.trim_start_matches("unwatch ").trim();
            if var_name.is_empty() {
                eprintln!("Error: Missing variable name");
                eprintln!("Usage: :unwatch <variable_name>");
                eprintln!("  Example: :unwatch x");
                eprintln!("  Note: This stops watching a variable for changes");
                eprintln!("  Note: Use :watch with no argument to list watched variables");
                return Some(false);
            }

            if state.watched_vars.remove(var_name).is_some() {
                println!("Stopped watching: {}", var_name);
            } else {
                eprintln!("Variable '{}' is not being watched", var_name);
                eprintln!("  Use :watch to see watched variables");
            }
            Some(false)
        }
        "edit" => {
            eprintln!("Usage: :edit <file>");
            eprintln!("  Opens file in $EDITOR or $VISUAL");
            eprintln!("  Example: :edit config.av");
            Some(false)
        }
        cmd if cmd.starts_with("edit ") => {
            let file_path = cmd.trim_start_matches("edit ").trim();
            if file_path.is_empty() {
                eprintln!("Usage: :edit <file>");
                return Some(false);
            }

            let editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| "vi".to_string());

            match std::process::Command::new(&editor).arg(file_path).status() {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("Editor exited with code: {:?}", status.code());
                    }
                }
                Err(e) => {
                    eprintln!("Error launching editor '{}': {}", editor, e);
                    eprintln!("  Set $EDITOR or $VISUAL environment variable");
                }
            }
            Some(false)
        }
        "source" => {
            eprintln!("Usage: :source <file>");
            eprintln!("  Execute REPL commands from a file");
            eprintln!("  Example: :source setup.repl");
            Some(false)
        }
        cmd if cmd.starts_with("source ") => {
            let file_path = cmd.trim_start_matches("source ").trim();
            if file_path.is_empty() {
                eprintln!("Usage: :source <file>");
                return Some(false);
            }

            match std::fs::read_to_string(file_path) {
                Ok(content) => {
                    let mut executed = 0;
                    let mut errors = 0;
                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }
                        match tokenize(line.to_string()) {
                            Ok(tokens) => {
                                let ast = parse(tokens);
                                match eval(ast.program, &mut state.symbols, line) {
                                    Ok(_) => executed += 1,
                                    Err(e) => {
                                        eprintln!("Error in line '{}': {}", line, e.message);
                                        errors += 1;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Parse error in '{}': {}", line, e.message);
                                errors += 1;
                            }
                        }
                    }
                    state.sync_symbols();
                    println!(
                        "Sourced {}: {} executed, {} errors",
                        file_path, executed, errors
                    );
                }
                Err(e) => {
                    eprintln!("Error reading '{}': {}", file_path, e);
                }
            }
            Some(false)
        }
        "clear-history" => {
            rl.clear_history().ok();
            println!("Command history cleared");
            Some(false)
        }
        "time" => {
            eprintln!("Usage: :time <expression>");
            eprintln!("  Measure evaluation time of an expression");
            eprintln!("  Example: :time map (\\x x * 2) [1..1000]");
            Some(false)
        }
        cmd if cmd.starts_with("time ") => {
            let expr_str = cmd.trim_start_matches("time ").trim();
            if expr_str.is_empty() {
                eprintln!("Usage: :time <expression>");
                return Some(false);
            }

            let start = std::time::Instant::now();
            match tokenize(expr_str.to_string()) {
                Ok(tokens) => {
                    let ast = parse(tokens);
                    match eval(ast.program, &mut state.symbols, expr_str) {
                        Ok(val) => {
                            let duration = start.elapsed();
                            let val_str = val.to_string("");
                            println!("Result: {}", val_str);
                            println!(
                                "Time: {:?} ({:.3}ms)",
                                duration,
                                duration.as_secs_f64() * 1000.0
                            );
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e.pretty_with_file(expr_str, Some("<repl>")));
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
            Some(false)
        }
        "report" => {
            let builtins = initial_builtins();
            let user_vars: Vec<&String> = state
                .symbols
                .keys()
                .filter(|name| !builtins.contains_key(*name))
                .collect();

            let history_count = rl.history().len();

            println!("═══════════════════════════════════════════");
            println!("           AVON REPL SESSION REPORT        ");
            println!("═══════════════════════════════════════════");
            println!();
            println!("Session Statistics:");
            println!("  Commands in history: {}", history_count);
            println!("  User variables: {}", user_vars.len());
            println!("  Watched variables: {}", state.watched_vars.len());
            println!("  Builtin functions: {}", builtins.len());
            println!();

            if !user_vars.is_empty() {
                println!("User Variables:");
                let mut sorted_vars = user_vars.clone();
                sorted_vars.sort();
                for name in sorted_vars.iter().take(20) {
                    if let Some(val) = state.symbols.get(*name) {
                        let type_name = match val {
                            Value::String(_) => "String",
                            Value::Number(_) => "Number",
                            Value::Bool(_) => "Bool",
                            Value::List(l) => {
                                println!("  {} : List[{}]", name, l.len());
                                continue;
                            }
                            Value::Dict(d) => {
                                println!("  {} : Dict{{{} keys}}", name, d.len());
                                continue;
                            }
                            Value::Function { .. } => "Function",
                            _ => "Other",
                        };
                        println!("  {} : {}", name, type_name);
                    }
                }
                if user_vars.len() > 20 {
                    println!("  ... and {} more", user_vars.len() - 20);
                }
                println!();
            }

            if !state.watched_vars.is_empty() {
                println!("Watched Variables:");
                for name in state.watched_vars.keys() {
                    println!("  {}", name);
                }
                println!();
            }

            println!(
                "Current Directory: {}",
                std::env::current_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| "unknown".to_string())
            );
            println!();
            println!("═══════════════════════════════════════════");
            Some(false)
        }
        _ => {
            println!(
                "Unknown command: {}. Type :help for available commands",
                cmd
            );
            Some(false)
        }
    }
}
