#![allow(clippy::result_large_err)] // EvalError is large but fundamental to the architecture

//! Avon Evaluation Engine
//!
//! This module contains:
//! - Core evaluation logic (`eval`, `eval_with_depth`)
//! - Function application (`apply_function`)
//! - Builtin execution (`execute_builtin`)
//! - Template rendering
//! - Helper utilities
//!
//! Submodules:
//! - `builtins`: Registry of builtin functions (names, arities, initialization)

mod builtins;

// Re-export registry functions for external use
pub use builtins::{get_builtin_arity, initial_builtins, is_builtin_name};

use crate::common::{Chunk, EvalError, Expr, Number, Token, Value};
use crate::lexer::tokenize;
use crate::parser::parse;
use std::collections::{HashMap, HashSet};

impl Value {
    pub fn to_string(&self, source: &str) -> String {
        self.to_string_with_depth(source, 0)
    }

    fn to_string_with_depth(&self, source: &str, depth: usize) -> String {
        const MAX_DEPTH: usize = 200;
        if depth > MAX_DEPTH {
            return format!("<recursion limit exceeded (depth > {})>", MAX_DEPTH);
        }

        match self {
            Value::None => "None".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(Number::Int(v)) => v.to_string(),
            Value::Number(Number::Float(v)) => v.to_string(),
            Value::String(s) => s.clone(),
            Value::Template(chunks, symbols) => {
                let raw = render_chunks_to_string_with_depth(chunks, symbols, source, depth)
                    .unwrap_or_else(|e| format!("<eval error: {}>", e));
                dedent(&raw)
            }
            Value::Path(chunks, symbols) => {
                let raw = render_chunks_to_string_with_depth(chunks, symbols, source, depth)
                    .unwrap_or_else(|e| format!("<eval error: {}>", e));
                dedent(&raw)
            }
            Value::FileTemplate {
                path: _p,
                template: t,
            } => {
                let raw = render_chunks_to_string_with_depth(&t.0, &t.1, source, depth)
                    .unwrap_or_else(|e| format!("<eval error: {}>", e));
                dedent(&raw)
            }
            Value::List(items) => {
                let inner: Vec<String> = items
                    .iter()
                    .map(|v| v.to_string_with_depth(source, depth + 1))
                    .collect();
                format!("[{}]", inner.join(", "))
            }
            Value::Function { .. } => "<function>".to_string(),
            Value::Builtin(name, _collected) => format!("<builtin:{}>", name),
            Value::Dict(map) => {
                const MAX_DICT_ENTRIES: usize = 100;
                if map.len() > MAX_DICT_ENTRIES {
                    return format!(
                        "<dict with {} entries (max {} for display)>",
                        map.len(),
                        MAX_DICT_ENTRIES
                    );
                }

                let mut entries: Vec<String> = Vec::new();
                for (k, v) in map.iter() {
                    let val_str = match v {
                        Value::String(s) => format!("\"{}\"", s),
                        _ => v.to_string_with_depth(source, depth + 1),
                    };
                    entries.push(format!("{}: {}", k, val_str));
                }
                format!("{{{}}}", entries.join(", "))
            }
        }
    }
}

// Security: Validate path to prevent directory traversal attacks
#[allow(clippy::result_large_err)]
fn validate_path(path: &str) -> Result<(), EvalError> {
    // Check for ".." components which could escape the intended directory
    if path.contains("..") {
        return Err(EvalError::new(
            format!("Path traversal not allowed: {}", path),
            None,
            None,
            0,
        ));
    }

    // Note: Absolute paths are blocked at the lexer level (@/ is a syntax error).
    // Paths in Avon are always relative by design. Use --root flag for deployment
    // to specify the absolute base directory.

    Ok(())
}

// Helper function to extract a file path from either a String or Path value
#[allow(clippy::result_large_err)]
pub fn value_to_path_string(val: &Value, source: &str) -> Result<String, EvalError> {
    let path_str = match val {
        Value::String(s) => s.clone(),
        Value::Path(chunks, symbols) => render_chunks_to_string(chunks, symbols, source)?,
        _ => {
            return Err(EvalError::type_mismatch(
                "string or path",
                val.to_string(source),
                0,
            ))
        }
    };

    // Validate the path for security issues
    validate_path(&path_str)?;

    Ok(path_str)
}

/// Extract all variable references from template chunks
/// This finds all identifiers used in expression chunks
fn extract_template_variables(chunks: &[Chunk]) -> HashSet<String> {
    let mut vars = HashSet::new();

    for chunk in chunks {
        if let Chunk::Expr(expr_str, _) = chunk {
            // Parse the expression to find variable references
            if let Ok(tokens) = tokenize(expr_str.clone()) {
                let ast = parse(tokens);
                extract_vars_from_expr(&ast.program, &mut vars);
            }
        }
    }

    vars
}

/// Recursively extract variable identifiers from an expression
fn extract_vars_from_expr(expr: &Expr, vars: &mut HashSet<String>) {
    match expr {
        Expr::Ident(name, _) => {
            vars.insert(name.clone());
        }
        Expr::Let { value, expr, .. } => {
            extract_vars_from_expr(value, vars);
            extract_vars_from_expr(expr, vars);
        }
        Expr::Function { default, expr, .. } => {
            if let Some(def) = default {
                extract_vars_from_expr(def, vars);
            }
            extract_vars_from_expr(expr, vars);
        }
        Expr::Application { lhs, rhs, .. } => {
            extract_vars_from_expr(lhs, vars);
            extract_vars_from_expr(rhs, vars);
        }
        Expr::Template(chunks, _) => {
            for chunk in chunks {
                if let Chunk::Expr(expr_str, _) = chunk {
                    if let Ok(tokens) = tokenize(expr_str.clone()) {
                        let ast = parse(tokens);
                        extract_vars_from_expr(&ast.program, vars);
                    }
                }
            }
        }
        Expr::Path(chunks, _) => {
            for chunk in chunks {
                if let Chunk::Expr(expr_str, _) = chunk {
                    if let Ok(tokens) = tokenize(expr_str.clone()) {
                        let ast = parse(tokens);
                        extract_vars_from_expr(&ast.program, vars);
                    }
                }
            }
        }
        Expr::FileTemplate { path, template, .. } => {
            for chunk in path {
                if let Chunk::Expr(expr_str, _) = chunk {
                    if let Ok(tokens) = tokenize(expr_str.clone()) {
                        let ast = parse(tokens);
                        extract_vars_from_expr(&ast.program, vars);
                    }
                }
            }
            for chunk in template {
                if let Chunk::Expr(expr_str, _) = chunk {
                    if let Ok(tokens) = tokenize(expr_str.clone()) {
                        let ast = parse(tokens);
                        extract_vars_from_expr(&ast.program, vars);
                    }
                }
            }
        }
        Expr::List(items, _) => {
            for item in items {
                extract_vars_from_expr(item, vars);
            }
        }
        Expr::Range {
            start, step, end, ..
        } => {
            extract_vars_from_expr(start, vars);
            if let Some(s) = step {
                extract_vars_from_expr(s, vars);
            }
            extract_vars_from_expr(end, vars);
        }
        Expr::Dict(pairs, _) => {
            for (_, expr) in pairs {
                extract_vars_from_expr(expr, vars);
            }
        }
        Expr::If { cond, t, f, .. } => {
            extract_vars_from_expr(cond, vars);
            extract_vars_from_expr(t, vars);
            extract_vars_from_expr(f, vars);
        }
        Expr::Binary { lhs, rhs, .. } => {
            extract_vars_from_expr(lhs, vars);
            extract_vars_from_expr(rhs, vars);
        }
        Expr::Member { object, .. } => {
            extract_vars_from_expr(object, vars);
        }
        Expr::Pipe { lhs, rhs, .. } => {
            extract_vars_from_expr(lhs, vars);
            extract_vars_from_expr(rhs, vars);
        }
        // These don't contain variable references
        Expr::None(_)
        | Expr::Bool(_, _)
        | Expr::Number(_, _)
        | Expr::String(_, _)
        | Expr::Builtin(_, _, _) => {}
    }
}

/// Create a minimal symbol table containing only the variables referenced in a template
fn create_minimal_symbol_table(
    chunks: &[Chunk],
    symbols: &HashMap<String, Value>,
) -> HashMap<String, Value> {
    let referenced_vars = extract_template_variables(chunks);
    let mut minimal = HashMap::new();

    // Only copy variables that are actually referenced
    for var_name in referenced_vars {
        if let Some(value) = symbols.get(&var_name) {
            minimal.insert(var_name, value.clone());
        }
    }

    minimal
}

pub fn render_chunks_to_string(
    chunks: &[Chunk],
    symbols: &HashMap<String, Value>,
    source: &str,
) -> Result<String, EvalError> {
    render_chunks_to_string_with_depth(chunks, symbols, source, 0)
}

fn render_chunks_to_string_with_depth(
    chunks: &[Chunk],
    symbols: &HashMap<String, Value>,
    source: &str,
    depth: usize,
) -> Result<String, EvalError> {
    const MAX_DEPTH: usize = 200;
    const MAX_CHUNKS: usize = 100000; // Increased - slow is OK
    const MAX_ITERATIONS: usize = 1000000; // Increased to 1 million - catches infinite loops, not slow evaluation

    // Note: Removed MAX_RENDER_TIME_MS timeout - slow template rendering is OK
    // The iteration counter and depth limit catch infinite loops

    if depth > MAX_DEPTH {
        return Err(EvalError::new(
            format!("template recursion limit exceeded (depth > {})", MAX_DEPTH),
            None,
            None,
            0,
        ));
    }

    if chunks.len() > MAX_CHUNKS {
        return Err(EvalError::new(
            format!(
                "template too large ({} chunks, max {})",
                chunks.len(),
                MAX_CHUNKS
            ),
            None,
            None,
            0,
        ));
    }

    let mut out = String::new();
    let mut iteration_count = 0;
    for c in chunks.iter() {
        // Check iteration limit - this catches infinite loops, not slow evaluation
        iteration_count += 1;
        if iteration_count > MAX_ITERATIONS {
            return Err(EvalError::new(
                format!(
                    "template rendering iteration limit exceeded (>{}) - possible infinite loop",
                    MAX_ITERATIONS
                ),
                None,
                None,
                0,
            ));
        }
        match c {
            Chunk::String(s) => out.push_str(s),
            Chunk::Expr(e, line) => {
                // Limit symbol table size to prevent performance issues
                if symbols.len() > 1000 {
                    return Err(EvalError::new(
                        format!(
                            "template symbol table too large ({} symbols, max 1000)",
                            symbols.len()
                        ),
                        None,
                        None,
                        *line,
                    ));
                }

                // Limit symbol table size to prevent memory exhaustion (not performance)
                // This catches unbounded growth, not slow evaluation
                const MAX_TEMPLATE_SYMBOLS: usize = 100000; // 100k symbols
                if symbols.len() > MAX_TEMPLATE_SYMBOLS {
                    return Err(EvalError::new(
                        format!("template symbol table too large ({} symbols, max {}) - possible infinite loop", symbols.len(), MAX_TEMPLATE_SYMBOLS),
                        None,
                        None,
                        *line,
                    ));
                }

                let tokens = tokenize(e.to_string()).map_err(|mut err| {
                    // Parser error line is 1-indexed; add chunk line (also 1-indexed) and subtract 1
                    if err.line == 1 {
                        err.line = *line;
                    } else {
                        // Multi-line expression: err.line is relative to expression, need to add offset
                        err.line = line.saturating_add(err.line.saturating_sub(1));
                    }
                    err
                })?;
                let ast = parse(tokens);
                let mut env = symbols.clone();
                // Use eval_with_depth to prevent infinite recursion during template rendering
                let v = eval_with_depth(ast.program, &mut env, source, depth + 1).map_err(
                    |mut err| {
                        // Error line from eval might be relative to expression or absolute
                        // If it's 0 or 1, use chunk line; otherwise add offset
                        if err.line <= 1 {
                            err.line = *line;
                        } else {
                            err.line = line.saturating_add(err.line.saturating_sub(1));
                        }
                        err
                    },
                )?;
                match v {
                    Value::List(ref items) => {
                        let items_str: Vec<String> = items
                            .iter()
                            .map(|it| it.to_string_with_depth(source, depth + 1))
                            .collect();
                        let indent = out.rsplit('\n').next().unwrap_or("");
                        let indent_prefix: String = indent
                            .chars()
                            .take_while(|c| *c == ' ' || *c == '\t')
                            .collect();

                        let mut first_item = true;
                        for item_s in items_str.iter() {
                            let lines: Vec<&str> = item_s.lines().collect();
                            if !first_item {
                                out.push('\n');
                                out.push_str(&indent_prefix);
                            }
                            if !lines.is_empty() {
                                out.push_str(lines[0]);
                                for ln in &lines[1..] {
                                    out.push('\n');
                                    out.push_str(&indent_prefix);
                                    out.push_str(ln);
                                }
                            }
                            first_item = false;
                        }
                    }
                    _ => out.push_str(&v.to_string_with_depth(source, depth + 1)),
                }
            }
        }
    }
    Ok(out)
}

pub fn dedent(s: &str) -> String {
    let mut lines: Vec<&str> = s.lines().collect();

    // Remove leading empty lines
    while let Some(first) = lines.first() {
        if first.trim().is_empty() {
            lines.remove(0);
        } else {
            break;
        }
    }

    // Remove trailing empty lines
    while let Some(last) = lines.last() {
        if last.trim().is_empty() {
            lines.pop();
        } else {
            break;
        }
    }

    if lines.is_empty() {
        return String::new();
    }

    // Find the column position of the first non-whitespace character
    // This becomes our baseline for dedentation
    let baseline_indent = lines
        .iter()
        .find_map(|line| {
            let leading_spaces = line.chars().take_while(|c| c.is_whitespace()).count();
            if leading_spaces < line.len() {
                // This line has non-whitespace content
                Some(leading_spaces)
            } else {
                // This line is all whitespace, skip it
                None
            }
        })
        .unwrap_or(0);

    let out_lines: Vec<String> = lines
        .into_iter()
        .map(|l| {
            let trimmed_len = l.trim_start().len();

            // Count leading whitespace
            let leading_spaces = l.chars().take_while(|c| c.is_whitespace()).count();

            // If line is empty/whitespace-only, keep it empty
            if trimmed_len == 0 {
                String::new()
            } else if leading_spaces >= baseline_indent {
                // Remove baseline_indent spaces
                l.chars().skip(baseline_indent).collect()
            } else {
                // Line has fewer spaces than baseline, keep as-is
                l.to_string()
            }
        })
        .collect();

    out_lines.join("\n")
}

// Global counter to track total evaluation steps (prevents infinite loops)
thread_local! {
    static EVAL_COUNTER: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub fn eval(
    expr: Expr,
    symbols: &mut HashMap<String, Value>,
    source: &str,
) -> Result<Value, EvalError> {
    EVAL_COUNTER.with(|counter| {
        counter.set(0);
    });
    eval_with_depth(expr, symbols, source, 0)
}

fn eval_with_depth(
    expr: Expr,
    symbols: &mut HashMap<String, Value>,
    source: &str,
    depth: usize,
) -> Result<Value, EvalError> {
    const MAX_EVAL_DEPTH: usize = 200;
    const MAX_EVAL_STEPS: usize = 1000000; // Increased to 1 million - catches infinite loops, not slow evaluation

    // Note: We removed the timeout check - slow evaluation is OK, we only need to catch infinite loops
    // The step counter and depth limit are sufficient to catch infinite loops

    // Check global step counter - this prevents infinite loops
    EVAL_COUNTER.with(|counter| {
        let steps = counter.get();
        if steps > MAX_EVAL_STEPS {
            return Err(EvalError::new(
                format!(
                    "evaluation step limit exceeded (>{}) - possible infinite loop",
                    MAX_EVAL_STEPS
                ),
                None,
                None,
                expr.line(),
            ));
        }
        counter.set(steps + 1);
        Ok(())
    })?;

    if depth > MAX_EVAL_DEPTH {
        return Err(EvalError::new(
            format!(
                "evaluation depth limit exceeded (depth > {})",
                MAX_EVAL_DEPTH
            ),
            None,
            None,
            expr.line(),
        ));
    }

    // Limit symbol table size to prevent memory exhaustion (not performance)
    // This catches cases where symbol table grows unbounded (infinite loop indicator)
    // Set high enough to allow legitimate large programs
    const MAX_SYMBOL_TABLE_SIZE: usize = 100000; // 100k symbols - catches unbounded growth
    if symbols.len() > MAX_SYMBOL_TABLE_SIZE {
        return Err(EvalError::new(
            format!("symbol table too large ({} symbols, max {}) - possible infinite loop causing unbounded growth", symbols.len(), MAX_SYMBOL_TABLE_SIZE),
            None,
            None,
            expr.line(),
        ));
    }

    let _line = expr.line();
    match expr {
        Expr::Number(value, _) => Ok(Value::Number(value)),
        Expr::String(value, _) => Ok(Value::String(value)),
        Expr::Binary { lhs, op, rhs, line } => {
            let l_eval = eval_with_depth(*lhs.clone(), symbols, source, depth + 1)?;
            let r_eval = eval_with_depth(*rhs.clone(), symbols, source, depth + 1)?;

            match op {
                // handle logical operators
                Token::And(_) => match (l_eval.clone(), r_eval.clone()) {
                    (Value::Bool(lb), Value::Bool(rb)) => Ok(Value::Bool(lb && rb)),
                    (a, b) => {
                        let l_type = match a {
                            Value::Number(_) => "number",
                            Value::String(_) => "string",
                            Value::List(_) => "list",
                            Value::Bool(_) => "bool",
                            Value::Function { .. } => "function",
                            Value::Template(_, _) => "template",
                            Value::Path(_, _) => "path",
                            Value::Dict(_) => "dict",
                            Value::None => "none",
                            _ => "unknown",
                        };
                        let r_type = match b {
                            Value::Number(_) => "number",
                            Value::String(_) => "string",
                            Value::List(_) => "list",
                            Value::Bool(_) => "bool",
                            Value::Function { .. } => "function",
                            Value::Template(_, _) => "template",
                            Value::Path(_, _) => "path",
                            Value::Dict(_) => "dict",
                            Value::None => "none",
                            _ => "unknown",
                        };

                        Err(EvalError::new(
                            format!("cannot use && with {} and {}", l_type, r_type),
                            None,
                            None,
                            line,
                        ))
                    }
                },
                Token::Or(_) => match (l_eval.clone(), r_eval.clone()) {
                    (Value::Bool(lb), Value::Bool(rb)) => Ok(Value::Bool(lb || rb)),
                    (a, b) => {
                        let l_type = match a {
                            Value::Number(_) => "number",
                            Value::String(_) => "string",
                            Value::List(_) => "list",
                            Value::Bool(_) => "bool",
                            Value::Function { .. } => "function",
                            Value::Template(_, _) => "template",
                            Value::Path(_, _) => "path",
                            Value::Dict(_) => "dict",
                            Value::None => "none",
                            _ => "unknown",
                        };
                        let r_type = match b {
                            Value::Number(_) => "number",
                            Value::String(_) => "string",
                            Value::List(_) => "list",
                            Value::Bool(_) => "bool",
                            Value::Function { .. } => "function",
                            Value::Template(_, _) => "template",
                            Value::Path(_, _) => "path",
                            Value::Dict(_) => "dict",
                            Value::None => "none",
                            _ => "unknown",
                        };

                        Err(EvalError::new(
                            format!("cannot use || with {} and {}", l_type, r_type),
                            None,
                            None,
                            line,
                        ))
                    }
                },
                Token::Add(_) => match (l_eval.clone(), r_eval.clone()) {
                    (Value::Number(ln), Value::Number(rn)) => Ok(Value::Number(ln + rn)),
                    (Value::String(ls), Value::String(rs)) => {
                        let mut out = ls.clone();
                        out.push_str(&rs);
                        Ok(Value::String(out))
                    }
                    (Value::List(mut la), Value::List(lb)) => {
                        la.extend(lb);
                        Ok(Value::List(la))
                    }
                    (Value::Template(lchunks, lsyms), Value::Template(rchunks, rsyms)) => {
                        // Limit total chunks to prevent memory issues
                        const MAX_TOTAL_CHUNKS: usize = 100000; // Increased - slow is OK
                        if lchunks.len() + rchunks.len() > MAX_TOTAL_CHUNKS {
                            return Err(EvalError::new(
                                format!("template concatenation would exceed chunk limit ({} + {} > {})", 
                                    lchunks.len(), rchunks.len(), MAX_TOTAL_CHUNKS),
                                None,
                                None,
                                line,
                            ));
                        }

                        // Merge symbol tables - with minimal symbol tables, this is much faster
                        // Both templates already only contain referenced variables, so merging is efficient
                        let mut combined_symbols = lsyms.clone();
                        combined_symbols.extend(rsyms.clone());

                        // Limit symbol table size to prevent memory exhaustion (not performance)
                        // This catches unbounded growth from infinite loops, not slow evaluation
                        const MAX_SYMBOLS: usize = 100000; // 100k symbols - catches unbounded growth
                        if combined_symbols.len() > MAX_SYMBOLS {
                            return Err(EvalError::new(
                                format!("template concatenation would exceed symbol table limit ({} > {}) - possible infinite loop", 
                                    combined_symbols.len(), MAX_SYMBOLS),
                                None,
                                None,
                                line,
                            ));
                        }

                        // Concatenate template chunks
                        let mut combined_chunks = lchunks.clone();
                        combined_chunks.extend(rchunks.clone());
                        Ok(Value::Template(combined_chunks, combined_symbols))
                    }
                    (Value::Path(lchunks, lsyms), Value::Path(rchunks, rsyms)) => {
                        // Smart path concatenation with correct slash handling and error modes
                        // Rules:
                        // - If rhs starts with a slash, don't insert an extra separator
                        // - If lhs already ends with a slash, don't add another
                        // - If both sides are absolute (lhs starts with '/' and rhs starts with '/'), error
                        // - Otherwise join with a single '/'

                        fn last_string_suffix(chunks: &[Chunk]) -> Option<&str> {
                            for c in chunks.iter().rev() {
                                if let Chunk::String(s) = c {
                                    return Some(s.as_str());
                                }
                            }
                            None
                        }
                        fn first_string_prefix(chunks: &[Chunk]) -> Option<&str> {
                            for c in chunks {
                                if let Chunk::String(s) = c {
                                    return Some(s.as_str());
                                }
                            }
                            None
                        }

                        // Since all paths are relative (lexer blocks absolute paths),
                        // we just need to join them with appropriate separator logic
                        let lhs_ends_with_slash = last_string_suffix(&lchunks)
                            .map(|s| s.ends_with('/'))
                            .unwrap_or(false);
                        let rhs_starts_with_slash = first_string_prefix(&rchunks)
                            .map(|s| s.starts_with('/'))
                            .unwrap_or(false);

                        let mut combined_chunks = lchunks.clone();
                        if !(lhs_ends_with_slash || rhs_starts_with_slash) {
                            combined_chunks.push(Chunk::String("/".to_string()));
                        }
                        combined_chunks.extend(rchunks.clone());

                        // Merge symbol tables
                        let mut combined_symbols = lsyms.clone();
                        combined_symbols.extend(rsyms.clone());
                        Ok(Value::Path(combined_chunks, combined_symbols))
                    }
                    (a, b) => {
                        let l_type = match a {
                            Value::Number(_) => "Number",
                            Value::String(_) => "String",
                            Value::List(_) => "List",
                            Value::Bool(_) => "Bool",
                            Value::Function { .. } => "Function",
                            Value::Template(_, _) => "Template",
                            Value::Path(_, _) => "Path",
                            _ => "unknown type",
                        };
                        let r_type = match b {
                            Value::Number(_) => "Number",
                            Value::String(_) => "String",
                            Value::List(_) => "List",
                            Value::Bool(_) => "Bool",
                            Value::Function { .. } => "Function",
                            Value::Template(_, _) => "Template",
                            Value::Path(_, _) => "Path",
                            _ => "unknown type",
                        };

                        Err(EvalError::new(
                            format!("cannot add {} and {}", l_type, r_type),
                            None,
                            None,
                            line,
                        ))
                    }
                },
                Token::Mul(_) | Token::Div(_) | Token::Sub(_) | Token::Mod(_) => {
                    let lnumber = match l_eval {
                        Value::Number(n) => n,
                        other => {
                            let op_name = match op {
                                Token::Mul(_) => "*",
                                Token::Div(_) => "/",
                                Token::Sub(_) => "-",
                                Token::Mod(_) => "%",
                                _ => "unknown",
                            };
                            return Err(EvalError::type_mismatch(
                                "number",
                                match other {
                                    Value::String(_) => "string",
                                    Value::List(_) => "list",
                                    Value::Bool(_) => "bool",
                                    Value::Function { .. } => "function",
                                    _ => "unknown",
                                },
                                line,
                            )
                            .with_context(op_name));
                        }
                    };

                    let rnumber = match r_eval {
                        Value::Number(n) => n,
                        other => {
                            let op_name = match op {
                                Token::Mul(_) => "*",
                                Token::Div(_) => "/",
                                Token::Sub(_) => "-",
                                Token::Mod(_) => "%",
                                _ => "unknown",
                            };
                            return Err(EvalError::type_mismatch(
                                "number",
                                match other {
                                    Value::String(_) => "string",
                                    Value::List(_) => "list",
                                    Value::Bool(_) => "bool",
                                    Value::Function { .. } => "function",
                                    _ => "unknown",
                                },
                                line,
                            )
                            .with_context(op_name));
                        }
                    };

                    let res = match op {
                        Token::Mul(_) => Value::Number(lnumber * rnumber),
                        Token::Div(_) => Value::Number(lnumber / rnumber),
                        Token::Sub(_) => Value::Number(lnumber - rnumber),
                        Token::Mod(_) => Value::Number(lnumber % rnumber),
                        _ => unreachable!(),
                    };
                    Ok(res)
                }
                Token::DoubleEqual(_)
                | Token::NotEqual(_)
                | Token::Greater(_)
                | Token::Less(_)
                | Token::GreaterEqual(_)
                | Token::LessEqual(_) => {
                    let eq = match (&l_eval, &r_eval) {
                        (Value::Number(ln), Value::Number(rn)) => {
                            let lval = match ln {
                                Number::Int(i) => *i as f64,
                                Number::Float(f) => *f,
                            };
                            let rval = match rn {
                                Number::Int(i) => *i as f64,
                                Number::Float(f) => *f,
                            };
                            match op {
                                Token::DoubleEqual(_) => lval == rval,
                                Token::NotEqual(_) => lval != rval,
                                Token::Greater(_) => lval > rval,
                                Token::Less(_) => lval < rval,
                                Token::GreaterEqual(_) => lval >= rval,
                                Token::LessEqual(_) => lval <= rval,
                                _ => false,
                            }
                        }
                        (Value::String(ls), Value::String(rs)) => match op {
                            Token::DoubleEqual(_) => ls == rs,
                            Token::NotEqual(_) => ls != rs,
                            Token::Greater(_) => ls > rs,
                            Token::Less(_) => ls < rs,
                            Token::GreaterEqual(_) => ls >= rs,
                            Token::LessEqual(_) => ls <= rs,
                            _ => false,
                        },
                        (Value::Bool(lb), Value::Bool(rb)) => match op {
                            Token::DoubleEqual(_) => lb == rb,
                            Token::NotEqual(_) => lb != rb,
                            _ => {
                                return Err(EvalError::new(
                                    "invalid comparison for bool",
                                    None,
                                    None,
                                    line,
                                ))
                            }
                        },
                        (Value::List(la), Value::List(lb)) => {
                            // Compare lists element by element
                            let lists_equal = la.len() == lb.len()
                                && la
                                    .iter()
                                    .zip(lb.iter())
                                    .all(|(a, b)| a.to_string(source) == b.to_string(source));
                            match op {
                                Token::DoubleEqual(_) => lists_equal,
                                Token::NotEqual(_) => !lists_equal,
                                _ => {
                                    return Err(EvalError::new(
                                        "lists only support == and != comparison",
                                        None,
                                        None,
                                        line,
                                    ))
                                }
                            }
                        }
                        (Value::Dict(da), Value::Dict(db)) => {
                            // Compare dicts key by key
                            let dicts_equal = da.len() == db.len()
                                && da.iter().all(|(k, v)| {
                                    db.get(k)
                                        .map(|v2| v.to_string(source) == v2.to_string(source))
                                        .unwrap_or(false)
                                });
                            match op {
                                Token::DoubleEqual(_) => dicts_equal,
                                Token::NotEqual(_) => !dicts_equal,
                                _ => {
                                    return Err(EvalError::new(
                                        "dicts only support == and != comparison",
                                        None,
                                        None,
                                        line,
                                    ))
                                }
                            }
                        }
                        (Value::None, Value::None) => match op {
                            Token::DoubleEqual(_) => true,
                            Token::NotEqual(_) => false,
                            _ => {
                                return Err(EvalError::new(
                                    "none only supports == and != comparison",
                                    None,
                                    None,
                                    line,
                                ))
                            }
                        },
                        (Value::None, _) | (_, Value::None) => match op {
                            Token::DoubleEqual(_) => false,
                            Token::NotEqual(_) => true,
                            _ => {
                                return Err(EvalError::new(
                                    "none only supports == and != comparison",
                                    None,
                                    None,
                                    line,
                                ))
                            }
                        },
                        (a, b) => {
                            // Type mismatch - different types cannot be compared
                            let type_a = match a {
                                Value::None => "None",
                                Value::Bool(_) => "Bool",
                                Value::Number(_) => "Number",
                                Value::String(_) => "String",
                                Value::Function { .. } => "Function",
                                Value::Builtin(_, _) => "Builtin",
                                Value::Template(_, _) => "Template",
                                Value::Path(_, _) => "Path",
                                Value::FileTemplate { .. } => "FileTemplate",
                                Value::List(_) => "List",
                                Value::Dict(_) => "Dict",
                            };
                            let type_b = match b {
                                Value::None => "None",
                                Value::Bool(_) => "Bool",
                                Value::Number(_) => "Number",
                                Value::String(_) => "String",
                                Value::Function { .. } => "Function",
                                Value::Builtin(_, _) => "Builtin",
                                Value::Template(_, _) => "Template",
                                Value::Path(_, _) => "Path",
                                Value::FileTemplate { .. } => "FileTemplate",
                                Value::List(_) => "List",
                                Value::Dict(_) => "Dict",
                            };
                            return Err(EvalError::new(
                                format!(
                                    "comparison type mismatch: cannot compare {} with {}",
                                    type_a, type_b
                                ),
                                Some("operands must be the same type".to_string()),
                                None,
                                line,
                            ));
                        }
                    };
                    Ok(Value::Bool(eq))
                }
                value => Err(EvalError::new(
                    format!("Not a valid operation: {:?}", value),
                    None,
                    None,
                    line,
                )),
            }
        }
        Expr::Ident(ident, line) => {
            // Underscore cannot be used as a variable - it's only for discarding values
            if ident == "_" {
                return Err(EvalError::new(
                    "underscore '_' cannot be used as a variable - it's only for discarding values in let bindings".to_string(),
                    Some("use a named variable".to_string()),
                    Some("_".to_string()),
                    line,
                ));
            }
            if let Some(value) = symbols.get(&ident) {
                // Automatically execute zero-arity builtins when looked up
                match value {
                    Value::Builtin(name, args) if args.is_empty() => {
                        // Check if this is a zero-arity builtin that should be executed
                        let arity = match name.as_str() {
                            "now" | "timestamp" | "timezone" => 0,
                            _ => 1, // Default arity for safety
                        };
                        if arity == 0 {
                            execute_builtin(name, &[], source, line)
                        } else {
                            Ok(value.clone())
                        }
                    }
                    _ => Ok(value.clone()),
                }
            } else {
                Err(EvalError::unknown_symbol(ident.clone(), line))
            }
        }
        Expr::Let {
            ident,
            value,
            expr,
            line,
        } => {
            // Check if variable already exists in current scope (prevent shadowing)
            // Exception: allow '_' to be reused (common pattern for ignoring values)
            if ident != "_" && symbols.contains_key(&ident) {
                return Err(EvalError::new(
                    format!("variable '{}' is already defined in this scope", ident),
                    Some("new variable name".to_string()),
                    Some("existing variable".to_string()),
                    line,
                ));
            }

            // Evaluate the value in the current scope
            let mut evalue = eval_with_depth(*value, symbols, source, depth + 1)?;
            if let Value::Function { ref mut name, .. } = evalue {
                *name = Some(ident.clone());
            }

            // Add binding to current scope, evaluate expression, then remove (stack-based scoping)
            symbols.insert(ident.clone(), evalue);
            let result = eval_with_depth(*expr, symbols, source, depth + 1);
            symbols.remove(&ident); // Restore previous state
            result
        }
        Expr::Function {
            ident,
            default,
            expr,
            line: _,
        } => {
            let default_val = if let Some(def_expr_box) = default {
                Some(Box::new(eval_with_depth(
                    *def_expr_box,
                    symbols,
                    source,
                    depth + 1,
                )?))
            } else {
                None
            };
            Ok(Value::Function {
                name: None,
                ident,
                default: default_val,
                expr,
                env: std::rc::Rc::new(symbols.clone()), // Rc wraps a snapshot of the current environment
            })
        }
        Expr::Application { lhs, rhs, line } => {
            let lhs_eval = eval_with_depth(*lhs, symbols, source, depth + 1)?;
            let arg_val = eval_with_depth(*rhs, symbols, source, depth + 1)?;
            match lhs_eval {
                Value::Function { .. } => apply_function(&lhs_eval, arg_val, source, line),
                builtin @ Value::Builtin(_, _) => apply_function(&builtin, arg_val, source, line),
                other => {
                    let type_name = match other {
                        Value::String(_) => "string",
                        Value::Number(_) => "number",
                        Value::List(_) => "list",
                        Value::Bool(_) => "bool",
                        _ => "unknown",
                    };
                    Err(EvalError::new(
                        "call",
                        Some("function".to_string()),
                        Some(type_name.to_string()),
                        line,
                    ))
                }
            }
        }
        Expr::None(_) => Ok(Value::None),
        Expr::Template(ref chunks, line) => {
            // Validate that template expressions can be evaluated
            // Templates are not lazy - they should fail immediately if they contain errors
            for chunk in chunks {
                if let Chunk::Expr(expr_str, chunk_line) = chunk {
                    // Try to parse and evaluate the expression to catch errors early
                    if let Ok(tokens) = tokenize(expr_str.clone()) {
                        let ast = parse(tokens);
                        // Evaluate the expression to check for errors
                        // We don't use the result, just check that it evaluates without error
                        eval_with_depth(ast.program, symbols, source, depth + 1).map_err(
                            |mut err| {
                                // Always use the chunk_line since expressions inside templates
                                // are parsed out of context and will have default line numbers
                                err.line = *chunk_line;
                                err
                            },
                        )?;
                    }
                }
            }

            // Only capture variables that are actually referenced in the template
            let minimal_symbols = create_minimal_symbol_table(chunks, symbols);

            // Limit symbol table size to prevent memory exhaustion (not performance)
            const MAX_TEMPLATE_CREATE_SYMBOLS: usize = 100000;
            if minimal_symbols.len() > MAX_TEMPLATE_CREATE_SYMBOLS {
                return Err(EvalError::new(
                    format!(
                        "symbol table too large ({} symbols, max {}) when creating template - possible infinite loop",
                        minimal_symbols.len(), MAX_TEMPLATE_CREATE_SYMBOLS
                    ),
                    None,
                    None,
                    line,
                ));
            }
            Ok(Value::Template(chunks.clone(), minimal_symbols))
        }
        Expr::Builtin(function, args, line) => match function.as_str() {
            "concat" => {
                let arg1 = symbols
                    .get(&args[0])
                    .cloned()
                    .ok_or_else(|| EvalError::unknown_symbol(args[0].clone(), line))?;

                let arg2 = symbols
                    .get(&args[1])
                    .cloned()
                    .ok_or_else(|| EvalError::unknown_symbol(args[1].clone(), line))?;

                if let Value::String(_) = arg1 {
                } else {
                    return Err(EvalError::type_mismatch(
                        "string",
                        arg1.to_string(source),
                        line,
                    ));
                }
                if let Value::String(_) = arg2 {
                } else {
                    return Err(EvalError::type_mismatch(
                        "string",
                        arg2.to_string(source),
                        line,
                    ));
                }

                match (arg1, arg2) {
                    (Value::String(mut lhs), Value::String(rhs)) => {
                        lhs.push_str(rhs.as_str());
                        Ok(Value::String(lhs))
                    }
                    (a, b) => Err(EvalError::type_mismatch(
                        "string",
                        format!("{}, {}", a.to_string(source), b.to_string(source)),
                        line,
                    )),
                }
            }
            _ => Err(EvalError::new("unimplemented builtin", None, None, line)),
        },
        Expr::Bool(value, _) => Ok(Value::Bool(value)),
        Expr::If { cond, t, f, line } => {
            let cond_eval = eval_with_depth(*cond, symbols, source, depth + 1)?;
            if let Value::Bool(cond_value) = cond_eval {
                if cond_value {
                    eval_with_depth(*t, symbols, source, depth + 1)
                } else {
                    eval_with_depth(*f, symbols, source, depth + 1)
                }
            } else {
                Err(EvalError::type_mismatch(
                    "bool",
                    cond_eval.to_string(source),
                    line,
                ))
            }
        }
        Expr::Path(ref chunks, _) => {
            // Only capture variables that are actually referenced in the path
            let minimal_symbols = create_minimal_symbol_table(chunks, symbols);

            // Limit symbol table size to prevent memory exhaustion (not performance)
            // This catches unbounded growth, not slow evaluation
            const MAX_PATH_CREATE_SYMBOLS: usize = 100000; // 100k symbols
            if minimal_symbols.len() > MAX_PATH_CREATE_SYMBOLS {
                return Err(EvalError::new(
                    format!(
                        "symbol table too large ({} symbols, max {}) when creating path - possible infinite loop",
                        minimal_symbols.len(), MAX_PATH_CREATE_SYMBOLS
                    ),
                    None,
                    None,
                    expr.line(),
                ));
            }
            Ok(Value::Path(chunks.clone(), minimal_symbols))
        }
        Expr::Range {
            start,
            step,
            end,
            line,
        } => {
            let start_val = eval_with_depth(*start, symbols, source, depth + 1)?;
            let end_val = eval_with_depth(*end, symbols, source, depth + 1)?;
            let step_val = if let Some(step_expr) = step {
                Some(eval_with_depth(*step_expr, symbols, source, depth + 1)?)
            } else {
                None
            };

            // Extract numeric values
            let start_num = match start_val {
                Value::Number(Number::Int(n)) => n,
                Value::Number(Number::Float(f)) => f as i64,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        start_val.to_string(source),
                        line,
                    ))
                }
            };
            let end_num = match end_val {
                Value::Number(Number::Int(n)) => n,
                Value::Number(Number::Float(f)) => f as i64,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        end_val.to_string(source),
                        line,
                    ))
                }
            };
            let step_num = if let Some(sv) = step_val {
                match sv {
                    Value::Number(Number::Int(n)) => n,
                    Value::Number(Number::Float(f)) => f as i64,
                    _ => {
                        return Err(EvalError::type_mismatch(
                            "number",
                            sv.to_string(source),
                            line,
                        ))
                    }
                }
            } else {
                1
            };

            // Generate range
            let mut result = Vec::new();
            if step_num > 0 {
                let mut current = start_num;
                while current <= end_num {
                    result.push(Value::Number(Number::Int(current)));
                    current += step_num;
                }
            } else if step_num < 0 {
                let mut current = start_num;
                while current >= end_num {
                    result.push(Value::Number(Number::Int(current)));
                    current += step_num;
                }
            } else {
                return Err(EvalError::new(
                    "range step cannot be zero",
                    None,
                    None,
                    line,
                ));
            }

            Ok(Value::List(result))
        }
        Expr::List(items, _) => {
            let mut evaluated = Vec::new();
            for item in items {
                evaluated.push(eval_with_depth(item, symbols, source, depth + 1)?);
            }
            Ok(Value::List(evaluated))
        }
        Expr::Dict(pairs, _) => {
            let mut map = HashMap::new();
            for (key, value_expr) in pairs {
                let value = eval_with_depth(value_expr, symbols, source, depth + 1)?;
                map.insert(key.clone(), value);
            }
            Ok(Value::Dict(map))
        }
        Expr::Member {
            object,
            field,
            line,
        } => {
            let obj_val = eval_with_depth(*object, symbols, source, depth + 1)?;
            match obj_val {
                Value::Dict(map) => map.get(&field).cloned().ok_or_else(|| {
                    EvalError::new(
                        ".",
                        Some(format!("key '{}'", field)),
                        Some("missing".to_string()),
                        line,
                    )
                }),
                other => Err(EvalError::type_mismatch(
                    "dict",
                    match other {
                        Value::String(_) => "string",
                        Value::Number(_) => "number",
                        Value::List(_) => "list",
                        Value::Bool(_) => "bool",
                        Value::Function { .. } => "function",
                        _ => "unknown",
                    },
                    line,
                )),
            }
        }
        Expr::FileTemplate {
            ref path,
            ref template,
            line,
        } => {
            // Validate that path and template expressions can be evaluated
            // FileTemplates are not lazy - they should fail immediately if they contain errors
            for chunk in path {
                if let Chunk::Expr(expr_str, chunk_line) = chunk {
                    if let Ok(tokens) = tokenize(expr_str.clone()) {
                        let ast = parse(tokens);
                        eval_with_depth(ast.program, symbols, source, depth + 1).map_err(
                            |mut err| {
                                // Always use the chunk_line since expressions inside templates
                                // are parsed out of context and will have default line numbers
                                err.line = *chunk_line;
                                err
                            },
                        )?;
                    }
                }
            }
            for chunk in template {
                if let Chunk::Expr(expr_str, chunk_line) = chunk {
                    if let Ok(tokens) = tokenize(expr_str.clone()) {
                        let ast = parse(tokens);
                        eval_with_depth(ast.program, symbols, source, depth + 1).map_err(
                            |mut err| {
                                // Always use the chunk_line since expressions inside templates
                                // are parsed out of context and will have default line numbers
                                err.line = *chunk_line;
                                err
                            },
                        )?;
                    }
                }
            }

            // Only capture variables that are actually referenced in the path and template
            let path_vars = extract_template_variables(path);
            let template_vars = extract_template_variables(template);
            let mut all_vars = path_vars;
            all_vars.extend(template_vars);

            let mut minimal_symbols = HashMap::new();
            for var_name in all_vars {
                if let Some(value) = symbols.get(&var_name) {
                    minimal_symbols.insert(var_name, value.clone());
                }
            }

            // Limit symbol table size to prevent memory exhaustion (not performance)
            const MAX_FILETEMPLATE_CREATE_SYMBOLS: usize = 100000;
            if minimal_symbols.len() > MAX_FILETEMPLATE_CREATE_SYMBOLS {
                return Err(EvalError::new(
                    format!(
                        "symbol table too large ({} symbols, max {}) when creating file template - possible infinite loop",
                        minimal_symbols.len(), MAX_FILETEMPLATE_CREATE_SYMBOLS
                    ),
                    None,
                    None,
                    line,
                ));
            }
            Ok(Value::FileTemplate {
                path: (path.clone(), minimal_symbols.clone()),
                template: (template.clone(), minimal_symbols),
            })
        }
        Expr::Pipe { lhs, rhs, line } => {
            let lhs_val = eval_with_depth(*lhs, symbols, source, depth + 1)?;
            let rhs_fn = eval_with_depth(*rhs, symbols, source, depth + 1)?;
            apply_function(&rhs_fn, lhs_val, source, line)
        }
    }
}

pub fn apply_function(
    func: &Value,
    arg: Value,
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    match func {
        Value::Function {
            name,
            ident,
            expr,
            env,
            ..
        } => {
            // Check if parameter name shadows a builtin
            if is_builtin_name(ident) {
                return Err(EvalError::new(
                    format!(
                        "cannot use '{}' as parameter name: shadows builtin function",
                        ident
                    ),
                    None,
                    None,
                    line,
                ));
            }
            // Create a new scope based on the captured environment (Rc allows cheap sharing)
            let mut new_env = (**env).clone(); // Dereference Rc and clone only once for this application
            new_env.insert(ident.clone(), arg);
            // NOTE: Recursive functions are not supported in Avon.
            // Functions cannot call themselves because they are not added to their own environment.
            // This design choice ensures:
            // 1. Simpler implementation (no need to track recursion depth)
            // 2. Better performance (no overhead from recursion tracking)
            // 3. Clearer error messages (unknown symbol vs infinite recursion)
            // 4. Encourages iterative solutions using fold/map/filter
            // If a function tries to call itself, it will get an "unknown symbol" error.
            let func_name = name.as_ref().unwrap_or(ident).clone();
            eval_with_depth(*expr.clone(), &mut new_env, source, 0).map_err(|mut err| {
                if !err.message.starts_with(&format!("{}:", func_name)) {
                    err.message = format!("{}: {}", func_name, err.message);
                }
                err
            })
        }
        Value::Builtin(name, collected) => {
            let mut new_collected = collected.clone();
            new_collected.push(arg);

            // Get arity from the registry - defaults to 1 if not found
            let arity = get_builtin_arity(name).unwrap_or(1);

            if new_collected.len() < arity {
                return Ok(Value::Builtin(name.clone(), new_collected));
            }

            execute_builtin(name, &new_collected, source, line).map_err(|mut err| {
                if !err.message.starts_with(&format!("{}:", name)) {
                    err.message = format!("{}: {}", name, err.message);
                }
                err
            })
        }
        other => {
            let type_name = match other {
                Value::String(_) => "string",
                Value::Number(_) => "number",
                Value::List(_) => "list",
                Value::Bool(_) => "bool",
                _ => "unknown",
            };
            Err(EvalError::new(
                "call",
                Some("function".to_string()),
                Some(type_name.to_string()),
                line,
            ))
        }
    }
}

// Helper function to convert Value to String, handling both String and Template
pub fn value_to_string_auto(val: &Value, source: &str, line: usize) -> Result<String, EvalError> {
    match val {
        Value::String(s) => Ok(s.clone()),
        Value::Template(chunks, symbols) => render_chunks_to_string(chunks, symbols, source)
            .map_err(|e| EvalError::new(format!("template render error: {}", e), None, None, line)),
        _ => Err(EvalError::type_mismatch(
            "string or template",
            val.to_string(source),
            line,
        )),
    }
}


pub fn execute_builtin(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    // Delegate to category modules based on the function name
    if builtins::aggregate::is_builtin(name) {
        return builtins::aggregate::execute(name, args, source, line);
    }
    if builtins::datetime::is_builtin(name) {
        return builtins::datetime::execute(name, args, source, line);
    }
    if builtins::debug::is_builtin(name) {
        return builtins::debug::execute(name, args, source, line);
    }
    if builtins::dict::is_builtin(name) {
        return builtins::dict::execute(name, args, source, line);
    }
    if builtins::env::is_builtin(name) {
        return builtins::env::execute(name, args, source, line);
    }
    if builtins::file_io::is_builtin(name) {
        return builtins::file_io::execute(name, args, source, line);
    }
    if builtins::formatting::is_builtin(name) {
        return builtins::formatting::execute(name, args, source, line);
    }
    if builtins::html::is_builtin(name) {
        return builtins::html::execute(name, args, source, line);
    }
    if builtins::list::is_builtin(name) {
        return builtins::list::execute(name, args, source, line);
    }
    if builtins::markdown::is_builtin(name) {
        return builtins::markdown::execute(name, args, source, line);
    }
    if builtins::math::is_builtin(name) {
        return builtins::math::execute(name, args, source, line);
    }
    if builtins::string::is_builtin(name) {
        return builtins::string::execute(name, args, source, line);
    }
    if builtins::types::is_builtin(name) {
        return builtins::types::execute(name, args, source, line);
    }
    
    // Fallback for unrecognized builtins
    Err(EvalError::new(
        format!("unimplemented builtin {}", name),
        None,
        None,
        line,
    ))
}


pub fn collect_file_templates(v: &Value, source: &str) -> Result<Vec<(String, String)>, EvalError> {
    match v {
        Value::FileTemplate {
            path: (pchunks, penv),
            template: (tchunks, tenv),
        } => {
            let path = render_chunks_to_string(pchunks, penv, source)?;
            let raw = render_chunks_to_string(tchunks, tenv, source)?;
            let content = dedent(&raw);
            Ok(vec![(path, content)])
        }
        Value::List(items) => {
            let mut out = Vec::new();
            for item in items {
                // Collect FileTemplates and recurse into nested lists
                // Report errors for bare Templates/Paths (which have no file paths)
                // Silently skip data types (strings, numbers, dicts, etc.)
                match item {
                    Value::FileTemplate { .. } | Value::List(_) => {
                        let mut res = collect_file_templates(item, source)?;
                        out.append(&mut res);
                    }
                    Value::Template(_, _) | Value::Path(_, _) => {
                        // Bare Templates and Paths can't be deployed - they need file paths
                        // This is likely a user error (e.g., concatenating templates instead of file templates)
                        return Err(EvalError::new(
                            "cannot deploy bare template or path - use @file {{...}} syntax to create a FileTemplate",
                            None,
                            None,
                            0,
                        ));
                    }
                    // Silently skip other data types (strings, numbers, dicts, bools, etc.)
                    _ => {}
                }
            }
            Ok(out)
        }
        _ => Err(EvalError::new(
            "expected filetemplate or list of filetemplates",
            None,
            None,
            0,
        )),
    }
}

pub fn fetch_git_raw(spec: &str) -> Result<String, EvalError> {
    let parts: Vec<&str> = spec.split('/').collect();
    if parts.len() < 3 {
        return Err(EvalError::new(
            "invalid git spec (expected owner/repo/path)",
            None,
            None,
            0,
        ));
    }
    let owner = parts[0];
    let repo = parts[1];
    let path = parts[2..].join("/");
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/{}",
        owner, repo, path
    );
    let resp = ureq::get(&url)
        .call()
        .map_err(|e| EvalError::new(format!("failed to fetch {}: {}", url, e), None, None, 0))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(EvalError::new(
            format!("failed to fetch {}: status {}", url, status),
            None,
            None,
            0,
        ));
    }
    let text = resp
        .into_body()
        .read_to_string()
        .map_err(|e| EvalError::new(format!("failed to read response: {}", e), None, None, 0))?;
    Ok(text)
}
