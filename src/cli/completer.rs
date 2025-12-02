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

        // Complete boolean literals
        for &literal in &["true", "false"] {
            if literal.starts_with(search) {
                matches.push(Pair {
                    display: literal.to_string(),
                    replacement: literal.to_string(),
                });
            }
        }

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
