// CLI Options and argument parsing

use std::collections::HashMap;

#[derive(Debug)]
pub struct CliOptions {
    pub root: Option<String>,
    pub force: bool,
    pub append: bool,
    pub if_not_exists: bool,
    pub backup: bool,
    pub debug: bool,
    pub read_stdin: bool,
    pub git_url: Option<String>,
    pub named_args: HashMap<String, String>,
    pub pos_args: Vec<String>,
    pub file: Option<String>,
    pub code: Option<String>,
}

impl CliOptions {
    pub fn new() -> Self {
        Self {
            root: None,
            force: false,
            append: false,
            if_not_exists: false,
            backup: false,
            debug: false,
            read_stdin: false,
            git_url: None,
            named_args: HashMap::new(),
            pos_args: Vec::new(),
            file: None,
            code: None,
        }
    }
}

impl Default for CliOptions {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_args(args: &[String], require_file: bool) -> Result<CliOptions, String> {
    let mut opts = CliOptions::new();
    let mut i = 0;

    // First arg might be file if not flag
    if require_file && i < args.len() && !args[i].starts_with('-') {
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
            "--stdin" => {
                opts.read_stdin = true;
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
            "-" => {
                // Treat "-" as a file argument (stdin) or positional arg
                if require_file && opts.file.is_none() && opts.git_url.is_none() {
                    opts.file = Some("-".to_string());
                } else {
                    opts.pos_args.push("-".to_string());
                }
                i += 1;
            }
            s if s.starts_with('-') => {
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

    if require_file && opts.file.is_none() && opts.git_url.is_none() && !opts.read_stdin {
        return Err(
            "Missing required file argument. Use: avon <command> <file> [options]".to_string(),
        );
    }

    Ok(opts)
}
