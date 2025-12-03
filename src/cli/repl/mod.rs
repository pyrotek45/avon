// REPL module

mod commands;
mod state;

use crate::common::Value;
use crate::eval::{collect_file_templates, eval, initial_builtins};
use crate::lexer::tokenize;
use crate::parser::parse_with_error;

use super::completer::AvonCompleter;
use super::helpers::{is_expression_complete_impl, is_let_expr_complete};
use commands::handle_command;
use state::ReplState;

use avon::syntax::AvonHighlighter;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, Context, EditMode, Editor, Helper};
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AvonHelper {
    pub completer: AvonCompleter,
    pub highlighter: AvonHighlighter,
}

impl Helper for AvonHelper {}

impl Completer for AvonHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Highlighter for AvonHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        self.highlighter.highlight_prompt(prompt, default)
    }

    fn highlight_char(&self, line: &str, pos: usize, _forced: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Hinter for AvonHelper {
    type Hint = String;
}

impl Validator for AvonHelper {}

pub fn execute_repl() -> i32 {
    println!("Avon REPL - Interactive Avon Shell");
    println!("Type ':help' for commands, ':exit' to quit");
    println!();

    // Configure rustyline with tab completion
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let symbols = initial_builtins();
    let symbols_rc = Rc::new(RefCell::new(symbols.clone()));

    let helper = AvonHelper {
        completer: AvonCompleter {
            file_completer: FilenameCompleter::new(),
            symbols: symbols_rc.clone(),
        },
        highlighter: AvonHighlighter::new(),
    };

    let mut rl = match Editor::<AvonHelper, DefaultHistory>::with_config(config) {
        Ok(mut editor) => {
            editor.set_helper(Some(helper));
            editor
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize REPL: {}", e);
            return 1;
        }
    };

    let mut state = ReplState::new(symbols, symbols_rc);

    loop {
        let prompt = if state.pending_let.is_some() || !state.input_buffer.is_empty() {
            "    > ".to_string()
        } else {
            "avon> ".to_string()
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                // Handle continuation of pending :let command FIRST (before empty check)
                if let Some(ref mut pending) = state.pending_let.take() {
                    // Allow escape from continuation mode with special commands
                    if trimmed == ":exit" || trimmed == ":quit" || trimmed == ":q" {
                        println!("Cancelled pending input.");
                        break;
                    }
                    // :cancel, :c, or :reset to cancel and return to REPL
                    if trimmed == ":cancel" || trimmed == ":c" || trimmed == ":reset" {
                        println!("Cancelled pending input.");
                        state.consecutive_empty_lines = 0;
                        continue;
                    }
                    
                    // Track consecutive empty lines to allow cancel (3 empty lines)
                    if trimmed.is_empty() {
                        state.consecutive_empty_lines += 1;
                        if state.consecutive_empty_lines >= 3 {
                            println!("Cancelled pending input (3 empty lines).");
                            state.consecutive_empty_lines = 0;
                            continue;
                        }
                        pending.expr_buffer.push('\n');
                        state.pending_let = Some(pending.clone());
                        continue;
                    } else {
                        state.consecutive_empty_lines = 0;
                    }
                    
                    pending.expr_buffer.push('\n');
                    pending.expr_buffer.push_str(trimmed);
                    
                    // Check if the expression is now complete
                    if is_let_expr_complete(&pending.expr_buffer) {
                        // Try to evaluate
                        execute_pending_let(&pending.var_name, &pending.expr_buffer, &mut state);
                    } else {
                        // Still incomplete, keep buffering
                        state.pending_let = Some(pending.clone());
                    }
                    continue;
                }

                // Handle empty input (when not in continuation mode)
                if trimmed.is_empty() {
                    if !state.input_buffer.is_empty() {
                        state.input_buffer.push('\n');
                    }
                    continue;
                }

                // Handle REPL commands
                if trimmed.starts_with(':') {
                    let cmd = trimmed.trim_start_matches(':');
                    if let Some(should_exit) = handle_command(cmd, &mut state, &mut rl) {
                        if should_exit {
                            break;
                        }
                    }
                    let _ = rl.add_history_entry(trimmed);
                    continue;
                }

                // Add to input buffer
                if state.input_buffer.is_empty() {
                    state.input_buffer = trimmed.to_string();
                } else {
                    state.input_buffer.push('\n');
                    state.input_buffer.push_str(trimmed);
                }

                // Check if expression is complete before trying to parse
                if !is_expression_complete_impl(&state.input_buffer) {
                    continue;
                }

                // Try to parse and evaluate
                match tokenize(state.input_buffer.clone()) {
                    Ok(tokens) => {
                        match parse_with_error(tokens) {
                            Ok(ast) => {
                                match eval(ast.program, &mut state.symbols, &state.input_buffer) {
                                    Ok(val) => {
                                        // Check watched variables for changes
                                        let mut changed_vars: Vec<(String, Value)> = Vec::new();
                                        for (name, old_val) in &state.watched_vars {
                                            if let Some(new_val) = state.symbols.get(name) {
                                                let old_str = old_val.to_string("");
                                                let new_str = new_val.to_string("");
                                                if old_str != new_str {
                                                    println!(
                                                        "[WATCH] {} changed: {} -> {}",
                                                        name, old_str, new_str
                                                    );
                                                    changed_vars.push((name.clone(), new_val.clone()));
                                                }
                                            }
                                        }
                                        for (name, val) in changed_vars {
                                            state.watched_vars.insert(name, val);
                                        }

                                        // Display result nicely
                                        display_value(&val, &state.input_buffer);

                                        // Add complete expression to history
                                        let _ = rl.add_history_entry(&state.input_buffer);
                                        state.input_buffer.clear();
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Error: {}",
                                            e.pretty_with_file(&state.input_buffer, Some("<repl>"))
                                        );
                                        state.input_buffer.clear();
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                    "Parse error: {}",
                                    e.pretty_with_file(&state.input_buffer, Some("<repl>"))
                                );
                                state.input_buffer.clear();
                            }
                        }
                    }
                    Err(e) => {
                        let error_msg = e.pretty_with_file(&state.input_buffer, Some("<repl>"));
                        if error_msg.contains("unexpected")
                            || error_msg.contains("EOF")
                            || (error_msg.contains("expected")
                                && (error_msg.contains("in")
                                    || error_msg.contains("then")
                                    || error_msg.contains("else")))
                        {
                            continue;
                        } else {
                            eprintln!("Lexer error: {}", error_msg);
                            state.input_buffer.clear();
                            continue;
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                state.input_buffer.clear();
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

fn display_value(val: &Value, input_buffer: &str) {
    match val {
        Value::FileTemplate { .. } => match collect_file_templates(val, input_buffer) {
            Ok(files) => {
                println!("FileTemplate:");
                for (path, content) in files {
                    println!("  Path: {}", path);
                    println!("  Content:\n{}", content);
                }
            }
            Err(_) => {
                println!("{}", val.to_string(input_buffer));
            }
        },
        Value::List(items)
            if items
                .iter()
                .any(|v| matches!(v, Value::FileTemplate { .. })) =>
        {
            match collect_file_templates(val, input_buffer) {
                Ok(files) => {
                    println!("List of FileTemplates ({}):", files.len());
                    for (path, content) in files {
                        println!("  Path: {}", path);
                        println!("  Content:\n{}", content);
                    }
                }
                Err(_) => {
                    println!("{}", val.to_string(input_buffer));
                }
            }
        }
        _ => {
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
            println!("{} : {}", val.to_string(input_buffer), type_name);
        }
    }
}

/// Execute a pending :let command with a complete expression
fn execute_pending_let(var_name: &str, expr_str: &str, state: &mut ReplState) {
    match tokenize(expr_str.to_string()) {
        Ok(tokens) => {
            match parse_with_error(tokens) {
                Ok(ast) => {
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
                                        println!("[WATCH] {} changed: {} -> {}", var_name, old_str, new_str);
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
                            eprintln!("Error: {}", e.pretty_with_file(expr_str, Some("<repl>")));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Parse error: {}", e.pretty_with_file(expr_str, Some("<repl>")));
                }
            }
        }
        Err(e) => {
            eprintln!("Lexer error: {}", e.pretty_with_file(expr_str, Some("<repl>")));
        }
    }
}
