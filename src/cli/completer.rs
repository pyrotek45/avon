// Tab completion for REPL

use crate::common::Value;
use crate::eval::initial_builtins;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::Context;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Custom completer for tab completion
pub struct AvonCompleter {
    pub file_completer: FilenameCompleter,
    pub symbols: Rc<RefCell<HashMap<String, Value>>>,
}

// REPL commands list
const REPL_COMMANDS: &[&str] = &[
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
    "source",
    "edit",
    "clear-history",
    "report",
    "time",
];

// Commands that expect file paths
const FILE_COMMANDS: &[&str] = &[
    "read",
    "run",
    "eval",
    "preview",
    "deploy",
    "write",
    "save-session",
    "load-session",
    "benchmark-file",
    "source",
    "edit",
    "cd",
    "list",
];

// Commands that expect variable names
const VAR_COMMANDS: &[&str] = &["inspect", "unlet", "watch", "unwatch"];

// Doc categories
const DOC_CATEGORIES: &[&str] = &[
    "string",
    "list",
    "math",
    "dict",
    "type",
    "format",
    "file",
    "date",
    "regex",
    "debug",
    "html",
    "markdown",
    "parse",
    "system",
    "convert",
];

impl Completer for AvonCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let trimmed = line.trim_start();

        // If line starts with ':', we're in command mode
        if trimmed.starts_with(':') {
            // Check if we're completing the command itself or its arguments
            let after_colon = &trimmed[1..];
            let parts: Vec<&str> = after_colon.split_whitespace().collect();

            if parts.is_empty() || (parts.len() == 1 && !after_colon.ends_with(' ')) {
                // Completing the command name itself
                let prefix = line[..pos].trim_start();
                let start = prefix.rfind(':').unwrap_or(0);
                let search = &prefix[start + 1..];

                let matches: Vec<Pair> = REPL_COMMANDS
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
            } else {
                // Completing command arguments - context-aware completion
                let cmd = parts[0];

                // Get the word being completed
                let word_start = line[..pos]
                    .rfind(|c: char| c.is_whitespace())
                    .map(|i| i + 1)
                    .unwrap_or(0);
                let search = &line[word_start..pos];

                // :doc command - complete with categories and builtin names
                if cmd == "doc" {
                    let mut matches: Vec<Pair> = Vec::new();

                    // Add categories first
                    for cat in DOC_CATEGORIES {
                        if cat.starts_with(search) {
                            matches.push(Pair {
                                display: format!("{} (category)", cat),
                                replacement: cat.to_string(),
                            });
                        }
                    }

                    // Add builtin function names
                    let builtins = initial_builtins();
                    for name in builtins.keys() {
                        if name.starts_with(search) {
                            matches.push(Pair {
                                display: name.to_string(),
                                replacement: name.to_string(),
                            });
                        }
                    }

                    // Add REPL command names (without colon)
                    for repl_cmd in REPL_COMMANDS {
                        if repl_cmd.starts_with(search) {
                            matches.push(Pair {
                                display: format!("{} (command)", repl_cmd),
                                replacement: repl_cmd.to_string(),
                            });
                        }
                    }

                    if !matches.is_empty() {
                        return Ok((word_start, matches));
                    }
                }

                // File-based commands - complete with file paths
                if FILE_COMMANDS.contains(&cmd) {
                    return self.file_completer.complete(line, pos, ctx);
                }

                // Variable-based commands - complete with user variable names
                if VAR_COMMANDS.contains(&cmd) {
                    let symbols = self.symbols.borrow();
                    let builtins = initial_builtins();
                    let matches: Vec<Pair> = symbols
                        .keys()
                        .filter(|name| !builtins.contains_key(*name) && name.starts_with(search))
                        .map(|name| Pair {
                            display: name.clone(),
                            replacement: name.clone(),
                        })
                        .collect();

                    if !matches.is_empty() {
                        return Ok((word_start, matches));
                    }
                }

                // :type, :assert, :test, :benchmark, :let, :deploy-expr - complete with builtins + variables
                if matches!(
                    cmd,
                    "type" | "assert" | "test" | "benchmark" | "let" | "deploy-expr" | "time"
                ) {
                    return self.complete_expression(line, pos, search);
                }
            }

            // No matches for command arguments
            return Ok((pos, vec![]));
        }

        // Regular expression mode - complete with builtins, variables, and keywords
        let word_start = line[..pos]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);

        let search = &line[word_start..pos];

        self.complete_expression(line, pos, search)
    }
}

impl AvonCompleter {
    /// Complete an expression with builtins, variables, and keywords
    fn complete_expression(
        &self,
        _line: &str,
        pos: usize,
        search: &str,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let word_start = pos - search.len();

        let builtins = initial_builtins();
        let mut matches: Vec<Pair> = builtins
            .keys()
            .filter(|name| name.starts_with(search))
            .map(|name| Pair {
                display: name.to_string(),
                replacement: name.to_string(),
            })
            .collect();

        // Complete boolean literals and keywords
        for &keyword in &["true", "false", "let", "in", "if", "then", "else", "import"] {
            if keyword.starts_with(search) {
                matches.push(Pair {
                    display: keyword.to_string(),
                    replacement: keyword.to_string(),
                });
            }
        }

        // Complete user-defined variables (excluding builtins)
        let symbols = self.symbols.borrow();
        for name in symbols.keys() {
            if !builtins.contains_key(name) && name.starts_with(search) {
                matches.push(Pair {
                    display: format!("{} (var)", name),
                    replacement: name.clone(),
                });
            }
        }

        Ok((word_start, matches))
    }
}
