// CLI command execution

use crate::common::Value;
use crate::eval::{apply_function, collect_file_templates, eval, fetch_git_raw, initial_builtins};
use crate::lexer::tokenize;
use crate::parser::parse;

use super::docs::{get_builtin_doc, get_category_doc, print_builtin_docs, print_help};
use super::options::{parse_args, CliOptions};
use super::repl::execute_repl;

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
            if rest.is_empty() {
                // No arguments - show all docs
                print_builtin_docs();
            } else {
                // Got a function name or category
                let name = rest[0].as_str();

                // Check for category documentation first
                if let Some(doc) = get_category_doc(name) {
                    println!("{}", doc);
                    return 0;
                }

                // Check for builtin function documentation
                let builtins = initial_builtins();
                if builtins.contains_key(name) {
                    if let Some(doc) = get_builtin_doc(name) {
                        println!("{}", doc);
                    } else {
                        println!("Function: {}", name);
                        println!("  This is a builtin function.");
                        println!("  Use 'avon doc' to see all builtin documentation.");
                    }
                    return 0;
                }

                // Unknown name
                eprintln!("Unknown function or category: {}", name);
                eprintln!();
                eprintln!("Available categories:");
                eprintln!("  string, list, dict, math, type, logic, io, template");
                eprintln!();
                eprintln!("Example usage:");
                eprintln!("  avon doc map        # Show documentation for 'map'");
                eprintln!("  avon doc string     # Show all string functions");
                eprintln!("  avon doc            # Show all documentation");
                return 1;
            }
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

pub fn get_source(opts: &CliOptions) -> Result<(String, String), i32> {
    if opts.read_stdin {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map(|_| (buffer, "<stdin>".to_string()))
            .map_err(|e| {
                eprintln!("Error: Failed to read from stdin: {}", e);
                1
            })
    } else if let Some(url) = &opts.git_url {
        fetch_git_raw(url).map(|s| (s, url.clone())).map_err(|e| {
            eprintln!("Error: Failed to fetch from git URL: {}", e.message);
            eprintln!("  URL: {}", url);
            eprintln!("  Tip: Make sure the URL format is: user/repo/path/to/file.av");
            eprintln!("  Example: avon deploy --git pyrotek45/avon/examples/config.av");
            1
        })
    } else if let Some(file) = &opts.file {
        if file == "-" {
            use std::io::Read;
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .map(|_| (buffer, "<stdin>".to_string()))
                .map_err(|e| {
                    eprintln!("Error: Failed to read from stdin: {}", e);
                    1
                })
        } else {
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
        }
    } else {
        eprintln!("Error: No source file provided");
        eprintln!("  Usage: avon <command> <file> [options]");
        eprintln!("  Example: avon eval config.av");
        eprintln!("  Example: avon deploy config.av --root ./output");
        eprintln!("  Use 'avon help' for more information");
        Err(1)
    }
}

pub fn process_source(
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
                        deploy_files(&v, &source, &source_name, &opts)
                    } else {
                        // Eval mode
                        match collect_file_templates(&v, &source) {
                            Ok(files) if !files.is_empty() => {
                                // Only print file templates if there are any
                                for (path, content) in files {
                                    println!("--- {} ---", path);
                                    // Don't highlight output - it's the result, not Avon source code
                                    println!("{}", content);
                                }
                            }
                            Ok(_) | Err(_) => {
                                // If no file templates found (empty list) or collection errors,
                                // print the value as-is (no highlighting)
                                println!("{}", v.to_string(&source));
                            }
                        }
                        0
                    }
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

fn deploy_files(v: &Value, source: &str, source_name: &str, opts: &CliOptions) -> i32 {
    match collect_file_templates(v, source) {
        Ok(files) => {
            // SAFETY: Collect all files first, validate all paths, then write all files
            // If any error occurs during collection or validation, no files are written

            // Step 1: Prepare all file operations (validate paths, create dirs)
            // SECURITY: Canonicalize root path once to prevent symlink attacks
            let (root_path, canonical_root) = if let Some(root_str) = &opts.root {
                let root = std::path::Path::new(root_str);
                match std::fs::canonicalize(root) {
                    Ok(canon) => (root.to_path_buf(), Some(canon)),
                    Err(_) => {
                        // If root doesn't exist, create it and then canonicalize
                        if let Err(e) = std::fs::create_dir_all(root) {
                            eprintln!("Error: Failed to create root directory: {}", root_str);
                            eprintln!("  Reason: {}", e);
                            eprintln!("Deployment aborted. No files were written.");
                            return 1;
                        }
                        match std::fs::canonicalize(root) {
                            Ok(canon) => (root.to_path_buf(), Some(canon)),
                            Err(e) => {
                                eprintln!(
                                    "Error: Failed to canonicalize root directory: {}",
                                    root_str
                                );
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

            let mut prepared_files: Vec<(std::path::PathBuf, String, bool, bool)> = Vec::new();
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
                            std::path::Component::RootDir => false,   // Block absolute paths
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
                                eprintln!("Error: Failed to resolve path: {}", full_path.display());
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
                                        if !canon_parent.starts_with(canon_root) {
                                            eprintln!("Error: Path traversal detected: {}", path);
                                            eprintln!(
                                                "  Attempted path would escape --root directory"
                                            );
                                            eprintln!("  Deployment aborted.");
                                            return 1;
                                        }
                                        // Build the final path from canonical parent + filename
                                        canon_parent.join(full_path.file_name().unwrap_or_default())
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Error: Failed to resolve parent directory: {}",
                                            parent.display()
                                        );
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
                                    if let std::path::Component::ParentDir = component {
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
                        eprintln!("  Attempted path would escape --root directory");
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
                        eprintln!("  Use --root to safely contain file operations");
                        eprintln!("  Deployment aborted.");
                        return 1;
                    }
                    // Also block absolute paths without --root for security
                    if path_buf.is_absolute() {
                        eprintln!("Error: Absolute paths are not allowed without --root");
                        eprintln!("  Use --root to safely contain file operations");
                        eprintln!("  Example: avon deploy program.av --root ./output");
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

            // Step 2: Validate all files can be written BEFORE writing any
            for (write_path, _content, exists, should_backup) in &prepared_files {
                if *exists {
                    #[allow(clippy::suspicious_open_options)]
                    match std::fs::OpenOptions::new()
                        .write(true)
                        .truncate(false)
                        .open(write_path)
                    {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(
                                "Error: Cannot write to existing file: {}",
                                write_path.display()
                            );
                            eprintln!("  Reason: {}", e);
                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                eprintln!("  Tip: Check file permissions");
                            }
                            eprintln!("Deployment aborted. No files were written.");
                            return 1;
                        }
                    }
                }

                if *should_backup {
                    let mut backup_name = write_path.file_name().unwrap().to_os_string();
                    backup_name.push(".bak");
                    let backup_path = write_path.with_file_name(backup_name);

                    #[allow(clippy::suspicious_open_options)]
                    match std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(&backup_path)
                    {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(
                                "Error: Cannot create backup file: {}",
                                backup_path.display()
                            );
                            eprintln!("  Reason: {}", e);
                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                eprintln!("  Tip: Check write permissions for backup location");
                            }
                            eprintln!("Deployment aborted. No files were written.");
                            return 1;
                        }
                    }
                }

                if !*exists {
                    if let Some(parent) = write_path.parent() {
                        let test_file = parent.join(".avon_write_test");
                        match std::fs::File::create(&test_file) {
                            Ok(_) => {
                                let _ = std::fs::remove_file(&test_file);
                            }
                            Err(e) => {
                                eprintln!("Error: Cannot write to directory: {}", parent.display());
                                eprintln!("  Reason: {}", e);
                                if e.kind() == std::io::ErrorKind::PermissionDenied {
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
            let mut written_files = Vec::new();
            for (write_path, content, exists, should_backup) in prepared_files {
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
                            eprintln!(
                                "  Note: {} file(s) were written before the error occurred.",
                                written_files.len()
                            );
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
                                eprintln!(
                                    "Error: Failed to append to file: {}",
                                    write_path.display()
                                );
                                eprintln!("  Reason: {}", e);
                                if e.kind() == std::io::ErrorKind::PermissionDenied {
                                    eprintln!("  Tip: Check file permissions");
                                } else if e.kind() == std::io::ErrorKind::OutOfMemory {
                                    eprintln!("  Tip: File may be too large for available memory");
                                }
                                if !written_files.is_empty() {
                                    eprintln!(
                                        "  Note: {} file(s) were written before the error occurred.",
                                        written_files.len()
                                    );
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
                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                eprintln!("  Tip: Check file permissions");
                                eprintln!("  Tip: Try using --backup instead of --append");
                            }
                            if !written_files.is_empty() {
                                eprintln!(
                                    "  Note: {} file(s) were written before the error occurred.",
                                    written_files.len()
                                );
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
                            eprintln!(
                                "  Note: {} file(s) were written before the error occurred.",
                                written_files.len()
                            );
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
            0
        }
        Err(e) => {
            // In deploy mode, if the result isn't deployable (not FileTemplate or list), error out
            eprintln!("Error: Deployment failed - result is not deployable");
            eprintln!("  The program evaluated successfully, but the result cannot be deployed.");
            eprintln!("  Expected: FileTemplate or list of FileTemplates");
            eprintln!("  Got: {}", v.to_string(source));
            eprintln!("  Details: {}", e.message);
            eprintln!();
            eprintln!("  Tip: Make sure your program returns a FileTemplate (using @path {{...}})");
            eprintln!(
                "  Tip: Or return a list of FileTemplates: [@file1.txt {{...}}, @file2.txt {{...}}]"
            );
            eprintln!(
                "  Tip: Use 'avon eval {}' to see what your program evaluates to",
                source_name
            );
            eprintln!("  No files were written.");
            1
        }
    }
}

pub fn execute_eval(opts: CliOptions) -> i32 {
    match get_source(&opts) {
        Ok((source, name)) => process_source(source, name, opts, false),
        Err(c) => c,
    }
}

pub fn execute_deploy(opts: CliOptions) -> i32 {
    match get_source(&opts) {
        Ok((source, name)) => process_source(source, name, opts, true),
        Err(c) => c,
    }
}

pub fn execute_run(opts: CliOptions) -> i32 {
    if let Some(code) = opts.code.clone() {
        process_source(code, "<input>".to_string(), opts, false)
    } else {
        1
    }
}
