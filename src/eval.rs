#![allow(clippy::result_large_err)] // EvalError is large but fundamental to the architecture

use crate::common::{Chunk, EvalError, Expr, Number, Token, Value};
use crate::lexer::tokenize;
use crate::parser::parse;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use std::collections::{HashMap, HashSet};

/// Check if a name is a builtin function name
/// This list is derived from `avon doc` output
fn is_builtin_name(name: &str) -> bool {
    matches!(
        name,
        "assert"
            | "basename"
            | "center"
            | "concat"
            | "contains"
            | "date_add"
            | "date_diff"
            | "date_format"
            | "date_parse"
            | "debug"
            | "dict_merge"
            | "dirname"
            | "drop"
            | "ends_with"
            | "enumerate"
            | "env_var"
            | "env_var_or"
            | "error"
            | "exists"
            | "fill_template"
            | "filter"
            | "flatmap"
            | "flatten"
            | "fold"
            | "format_binary"
            | "format_bool"
            | "format_bytes"
            | "format_currency"
            | "format_float"
            | "format_hex"
            | "format_int"
            | "format_json"
            | "format_list"
            | "format_octal"
            | "format_percent"
            | "format_scientific"
            | "format_table"
            | "get"
            | "has_key"
            | "head"
            | "html_attr"
            | "html_escape"
            | "html_tag"
            | "import"
            | "indent"
            | "is_alpha"
            | "is_alphanumeric"
            | "is_bool"
            | "is_digit"
            | "is_empty"
            | "is_float"
            | "is_function"
            | "is_int"
            | "is_list"
            | "is_lowercase"
            | "is_none"
            | "is_number"
            | "is_string"
            | "is_uppercase"
            | "is_whitespace"
            | "join"
            | "json_parse"
            | "keys"
            | "length"
            | "lower"
            | "map"
            | "markdown_to_html"
            | "md_code"
            | "md_heading"
            | "md_link"
            | "md_list"
            | "neg"
            | "not"
            | "now"
            | "os"
            | "pad_left"
            | "pad_right"
            | "partition"
            | "range"
            | "readfile"
            | "readlines"
            | "repeat"
            | "replace"
            | "reverse"
            | "set"
            | "sort"
            | "sort_by"
            | "split"
            | "split_at"
            | "starts_with"
            | "tail"
            | "take"
            | "timestamp"
            | "timezone"
            | "to_bool"
            | "to_char"
            | "to_float"
            | "to_int"
            | "to_list"
            | "to_string"
            | "trace"
            | "trim"
            | "truncate"
            | "typeof"
            | "unique"
            | "unzip"
            | "upper"
            | "values"
            | "walkdir"
            | "zip"
    )
}

pub fn initial_builtins() -> HashMap<String, Value> {
    let mut m = HashMap::new();
    m.insert(
        "env_var".to_string(),
        Value::Builtin("env_var".to_string(), Vec::new()),
    );
    m.insert(
        "env_var_or".to_string(),
        Value::Builtin("env_var_or".to_string(), Vec::new()),
    );

    // Core operations
    m.insert(
        "concat".to_string(),
        Value::Builtin("concat".to_string(), Vec::new()),
    );
    m.insert(
        "map".to_string(),
        Value::Builtin("map".to_string(), Vec::new()),
    );
    m.insert(
        "filter".to_string(),
        Value::Builtin("filter".to_string(), Vec::new()),
    );
    m.insert(
        "fold".to_string(),
        Value::Builtin("fold".to_string(), Vec::new()),
    );
    m.insert(
        "import".to_string(),
        Value::Builtin("import".to_string(), Vec::new()),
    );

    // File operations
    m.insert(
        "readfile".to_string(),
        Value::Builtin("readfile".to_string(), Vec::new()),
    );
    m.insert(
        "exists".to_string(),
        Value::Builtin("exists".to_string(), Vec::new()),
    );
    m.insert(
        "basename".to_string(),
        Value::Builtin("basename".to_string(), Vec::new()),
    );
    m.insert(
        "dirname".to_string(),
        Value::Builtin("dirname".to_string(), Vec::new()),
    );
    m.insert(
        "readlines".to_string(),
        Value::Builtin("readlines".to_string(), Vec::new()),
    );
    m.insert(
        "walkdir".to_string(),
        Value::Builtin("walkdir".to_string(), Vec::new()),
    );
    m.insert(
        "json_parse".to_string(),
        Value::Builtin("json_parse".to_string(), Vec::new()),
    );
    m.insert(
        "fill_template".to_string(),
        Value::Builtin("fill_template".to_string(), Vec::new()),
    );

    // String operations
    m.insert(
        "upper".to_string(),
        Value::Builtin("upper".to_string(), Vec::new()),
    );
    m.insert(
        "lower".to_string(),
        Value::Builtin("lower".to_string(), Vec::new()),
    );
    m.insert(
        "trim".to_string(),
        Value::Builtin("trim".to_string(), Vec::new()),
    );
    m.insert(
        "split".to_string(),
        Value::Builtin("split".to_string(), Vec::new()),
    );
    m.insert(
        "join".to_string(),
        Value::Builtin("join".to_string(), Vec::new()),
    );
    m.insert(
        "replace".to_string(),
        Value::Builtin("replace".to_string(), Vec::new()),
    );
    m.insert(
        "contains".to_string(),
        Value::Builtin("contains".to_string(), Vec::new()),
    );
    m.insert(
        "starts_with".to_string(),
        Value::Builtin("starts_with".to_string(), Vec::new()),
    );
    m.insert(
        "ends_with".to_string(),
        Value::Builtin("ends_with".to_string(), Vec::new()),
    );
    m.insert(
        "length".to_string(),
        Value::Builtin("length".to_string(), Vec::new()),
    );
    m.insert(
        "repeat".to_string(),
        Value::Builtin("repeat".to_string(), Vec::new()),
    );
    m.insert(
        "pad_left".to_string(),
        Value::Builtin("pad_left".to_string(), Vec::new()),
    );
    m.insert(
        "pad_right".to_string(),
        Value::Builtin("pad_right".to_string(), Vec::new()),
    );
    m.insert(
        "indent".to_string(),
        Value::Builtin("indent".to_string(), Vec::new()),
    );

    // String predicates
    m.insert(
        "is_digit".to_string(),
        Value::Builtin("is_digit".to_string(), Vec::new()),
    );
    m.insert(
        "is_alpha".to_string(),
        Value::Builtin("is_alpha".to_string(), Vec::new()),
    );
    m.insert(
        "is_alphanumeric".to_string(),
        Value::Builtin("is_alphanumeric".to_string(), Vec::new()),
    );
    m.insert(
        "is_whitespace".to_string(),
        Value::Builtin("is_whitespace".to_string(), Vec::new()),
    );
    m.insert(
        "is_uppercase".to_string(),
        Value::Builtin("is_uppercase".to_string(), Vec::new()),
    );
    m.insert(
        "is_lowercase".to_string(),
        Value::Builtin("is_lowercase".to_string(), Vec::new()),
    );
    m.insert(
        "is_empty".to_string(),
        Value::Builtin("is_empty".to_string(), Vec::new()),
    );

    // Type conversion/casting
    m.insert(
        "to_string".to_string(),
        Value::Builtin("to_string".to_string(), Vec::new()),
    );
    m.insert(
        "to_int".to_string(),
        Value::Builtin("to_int".to_string(), Vec::new()),
    );
    m.insert(
        "to_float".to_string(),
        Value::Builtin("to_float".to_string(), Vec::new()),
    );
    m.insert(
        "to_bool".to_string(),
        Value::Builtin("to_bool".to_string(), Vec::new()),
    );
    m.insert(
        "to_char".to_string(),
        Value::Builtin("to_char".to_string(), Vec::new()),
    );
    m.insert(
        "to_list".to_string(),
        Value::Builtin("to_list".to_string(), Vec::new()),
    );
    m.insert(
        "neg".to_string(),
        Value::Builtin("neg".to_string(), Vec::new()),
    );
    m.insert(
        "format_int".to_string(),
        Value::Builtin("format_int".to_string(), Vec::new()),
    );
    m.insert(
        "format_float".to_string(),
        Value::Builtin("format_float".to_string(), Vec::new()),
    );

    // Formatting functions
    m.insert(
        "format_hex".to_string(),
        Value::Builtin("format_hex".to_string(), Vec::new()),
    );
    m.insert(
        "format_octal".to_string(),
        Value::Builtin("format_octal".to_string(), Vec::new()),
    );
    m.insert(
        "format_binary".to_string(),
        Value::Builtin("format_binary".to_string(), Vec::new()),
    );
    m.insert(
        "format_scientific".to_string(),
        Value::Builtin("format_scientific".to_string(), Vec::new()),
    );
    m.insert(
        "format_bytes".to_string(),
        Value::Builtin("format_bytes".to_string(), Vec::new()),
    );
    m.insert(
        "format_list".to_string(),
        Value::Builtin("format_list".to_string(), Vec::new()),
    );
    m.insert(
        "format_table".to_string(),
        Value::Builtin("format_table".to_string(), Vec::new()),
    );
    m.insert(
        "format_json".to_string(),
        Value::Builtin("format_json".to_string(), Vec::new()),
    );
    m.insert(
        "format_currency".to_string(),
        Value::Builtin("format_currency".to_string(), Vec::new()),
    );
    m.insert(
        "format_percent".to_string(),
        Value::Builtin("format_percent".to_string(), Vec::new()),
    );
    m.insert(
        "format_bool".to_string(),
        Value::Builtin("format_bool".to_string(), Vec::new()),
    );
    m.insert(
        "truncate".to_string(),
        Value::Builtin("truncate".to_string(), Vec::new()),
    );
    m.insert(
        "center".to_string(),
        Value::Builtin("center".to_string(), Vec::new()),
    );

    // List operations (advanced)
    m.insert(
        "flatmap".to_string(),
        Value::Builtin("flatmap".to_string(), Vec::new()),
    );
    m.insert(
        "flatten".to_string(),
        Value::Builtin("flatten".to_string(), Vec::new()),
    );
    m.insert(
        "head".to_string(),
        Value::Builtin("head".to_string(), Vec::new()),
    );
    m.insert(
        "tail".to_string(),
        Value::Builtin("tail".to_string(), Vec::new()),
    );
    m.insert(
        "take".to_string(),
        Value::Builtin("take".to_string(), Vec::new()),
    );
    m.insert(
        "drop".to_string(),
        Value::Builtin("drop".to_string(), Vec::new()),
    );
    m.insert(
        "zip".to_string(),
        Value::Builtin("zip".to_string(), Vec::new()),
    );
    m.insert(
        "unzip".to_string(),
        Value::Builtin("unzip".to_string(), Vec::new()),
    );
    m.insert(
        "split_at".to_string(),
        Value::Builtin("split_at".to_string(), Vec::new()),
    );
    m.insert(
        "partition".to_string(),
        Value::Builtin("partition".to_string(), Vec::new()),
    );
    m.insert(
        "reverse".to_string(),
        Value::Builtin("reverse".to_string(), Vec::new()),
    );
    m.insert(
        "sort".to_string(),
        Value::Builtin("sort".to_string(), Vec::new()),
    );
    m.insert(
        "sort_by".to_string(),
        Value::Builtin("sort_by".to_string(), Vec::new()),
    );
    m.insert(
        "unique".to_string(),
        Value::Builtin("unique".to_string(), Vec::new()),
    );
    m.insert(
        "range".to_string(),
        Value::Builtin("range".to_string(), Vec::new()),
    );
    m.insert(
        "enumerate".to_string(),
        Value::Builtin("enumerate".to_string(), Vec::new()),
    );

    // Map/Dictionary operations (using list of pairs)
    m.insert(
        "get".to_string(),
        Value::Builtin("get".to_string(), Vec::new()),
    );
    m.insert(
        "set".to_string(),
        Value::Builtin("set".to_string(), Vec::new()),
    );
    m.insert(
        "keys".to_string(),
        Value::Builtin("keys".to_string(), Vec::new()),
    );
    m.insert(
        "values".to_string(),
        Value::Builtin("values".to_string(), Vec::new()),
    );
    m.insert(
        "has_key".to_string(),
        Value::Builtin("has_key".to_string(), Vec::new()),
    );
    m.insert(
        "dict_merge".to_string(),
        Value::Builtin("dict_merge".to_string(), Vec::new()),
    );

    // HTML helpers
    m.insert(
        "html_escape".to_string(),
        Value::Builtin("html_escape".to_string(), Vec::new()),
    );
    m.insert(
        "html_tag".to_string(),
        Value::Builtin("html_tag".to_string(), Vec::new()),
    );
    m.insert(
        "html_attr".to_string(),
        Value::Builtin("html_attr".to_string(), Vec::new()),
    );

    // Markdown helpers
    m.insert(
        "md_heading".to_string(),
        Value::Builtin("md_heading".to_string(), Vec::new()),
    );
    m.insert(
        "md_link".to_string(),
        Value::Builtin("md_link".to_string(), Vec::new()),
    );
    m.insert(
        "md_code".to_string(),
        Value::Builtin("md_code".to_string(), Vec::new()),
    );
    m.insert(
        "md_list".to_string(),
        Value::Builtin("md_list".to_string(), Vec::new()),
    );
    m.insert(
        "markdown_to_html".to_string(),
        Value::Builtin("markdown_to_html".to_string(), Vec::new()),
    );

    // Data structures (dict is now literal syntax {key: value})
    // Note: dict_get, dict_set, dict_has_key were deprecated and removed
    // Use get, set, has_key instead (they work with both dicts and pairs)

    // System
    m.insert(
        "os".to_string(),
        Value::String(std::env::consts::OS.to_string()),
    );

    // Date/Time operations
    m.insert(
        "now".to_string(),
        Value::Builtin("now".to_string(), Vec::new()),
    );
    m.insert(
        "date_format".to_string(),
        Value::Builtin("date_format".to_string(), Vec::new()),
    );
    m.insert(
        "date_parse".to_string(),
        Value::Builtin("date_parse".to_string(), Vec::new()),
    );
    m.insert(
        "date_add".to_string(),
        Value::Builtin("date_add".to_string(), Vec::new()),
    );
    m.insert(
        "date_diff".to_string(),
        Value::Builtin("date_diff".to_string(), Vec::new()),
    );
    m.insert(
        "timestamp".to_string(),
        Value::Builtin("timestamp".to_string(), Vec::new()),
    );
    m.insert(
        "timezone".to_string(),
        Value::Builtin("timezone".to_string(), Vec::new()),
    );

    // Type checking and introspection
    m.insert(
        "typeof".to_string(),
        Value::Builtin("typeof".to_string(), Vec::new()),
    );
    m.insert(
        "is_string".to_string(),
        Value::Builtin("is_string".to_string(), Vec::new()),
    );
    m.insert(
        "is_number".to_string(),
        Value::Builtin("is_number".to_string(), Vec::new()),
    );
    m.insert(
        "is_int".to_string(),
        Value::Builtin("is_int".to_string(), Vec::new()),
    );
    m.insert(
        "is_float".to_string(),
        Value::Builtin("is_float".to_string(), Vec::new()),
    );
    m.insert(
        "is_list".to_string(),
        Value::Builtin("is_list".to_string(), Vec::new()),
    );
    m.insert(
        "is_bool".to_string(),
        Value::Builtin("is_bool".to_string(), Vec::new()),
    );
    m.insert(
        "is_function".to_string(),
        Value::Builtin("is_function".to_string(), Vec::new()),
    );
    m.insert(
        "is_dict".to_string(),
        Value::Builtin("is_dict".to_string(), Vec::new()),
    );
    m.insert(
        "is_none".to_string(),
        Value::Builtin("is_none".to_string(), Vec::new()),
    );
    m.insert(
        "not".to_string(),
        Value::Builtin("not".to_string(), Vec::new()),
    );

    // Assertions
    m.insert(
        "assert".to_string(),
        Value::Builtin("assert".to_string(), Vec::new()),
    );

    // Debugging and error handling
    m.insert(
        "error".to_string(),
        Value::Builtin("error".to_string(), Vec::new()),
    );
    m.insert(
        "trace".to_string(),
        Value::Builtin("trace".to_string(), Vec::new()),
    );
    m.insert(
        "debug".to_string(),
        Value::Builtin("debug".to_string(), Vec::new()),
    );

    m
}

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

            let arity = match name.as_str() {
                "concat" => 2,
                "map" => 2,
                "filter" => 2,
                "fold" => 3,
                "import" => 1,
                "readfile" => 1,
                "readlines" => 1,
                "walkdir" => 1,
                "json_parse" => 1,
                "fill_template" => 2,
                "exists" => 1,
                "basename" => 1,
                "dirname" => 1,
                "upper" => 1,
                "lower" => 1,
                "trim" => 1,
                "split" => 2,
                "join" => 2,
                "replace" => 3,
                "contains" => 2,
                "starts_with" => 2,
                "ends_with" => 2,
                "length" => 1,
                "repeat" => 2,
                "pad_left" => 3,
                "pad_right" => 3,
                "indent" => 2,
                "is_digit" => 1,
                "is_alpha" => 1,
                "is_alphanumeric" => 1,
                "is_whitespace" => 1,
                "is_uppercase" => 1,
                "is_lowercase" => 1,
                "is_empty" => 1,
                "html_escape" => 1,
                "html_tag" => 2,
                "html_attr" => 2,
                "md_heading" => 2,
                "md_link" => 2,
                "md_code" => 1,
                "md_list" => 1,
                "markdown_to_html" => 1,
                "to_string" => 1,
                "to_int" => 1,
                "to_float" => 1,
                "to_bool" => 1,
                "to_char" => 1,
                "to_list" => 1,
                "neg" => 1,
                "format_int" => 2,
                "format_float" => 2,
                "format_hex" => 1,
                "format_octal" => 1,
                "format_binary" => 1,
                "format_scientific" => 2,
                "format_bytes" => 1,
                "format_list" => 2,
                "format_table" => 2,
                "format_json" => 1,
                "format_currency" => 2,
                "format_percent" => 2,
                "format_bool" => 2,
                "truncate" => 2,
                "center" => 2,
                "flatmap" => 2,
                "flatten" => 1,
                "head" => 1,
                "tail" => 1,
                "take" => 2,
                "drop" => 2,
                "zip" => 2,
                "unzip" => 1,
                "split_at" => 2,
                "partition" => 2,
                "reverse" => 1,
                "sort" => 1,
                "sort_by" => 2,
                "unique" => 1,
                "range" => 2,
                "enumerate" => 1,
                "get" => 2,
                "set" => 3,
                "keys" => 1,
                "values" => 1,
                "has_key" => 2,
                "dict_merge" => 2,
                "typeof" => 1,
                "is_string" => 1,
                "is_number" => 1,
                "is_int" => 1,
                "is_float" => 1,
                "is_list" => 1,
                "is_bool" => 1,
                "is_function" => 1,
                "is_dict" => 1,
                "is_none" => 1,
                "not" => 1,
                "assert" => 2,
                "error" => 1,
                "trace" => 2,
                "debug" => 1,
                "os" => 0,
                "env_var" => 1,
                "env_var_or" => 2,
                // Date/Time operations
                "now" => 0,
                "date_format" => 2,
                "date_parse" => 2,
                "date_add" => 2,
                "date_diff" => 2,
                "timestamp" => 0,
                "timezone" => 0,
                _ => 1,
            };

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
fn value_to_string_auto(val: &Value, source: &str, line: usize) -> Result<String, EvalError> {
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

// Helper function to parse duration strings like "1d", "2h", "30m", etc.
fn parse_duration(s: &str) -> Result<chrono::Duration, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty duration string".to_string());
    }

    // Find where the number ends and unit begins
    let mut num_end = 0;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() || c == '-' || c == '+' || c == '.' {
            num_end = i + 1;
        } else {
            break;
        }
    }

    if num_end == 0 {
        return Err(format!("no number found in '{}'", s));
    }

    let num_str = &s[..num_end];
    let unit = s[num_end..].trim();

    let value: i64 = num_str
        .parse()
        .map_err(|_| format!("invalid number: '{}'", num_str))?;

    match unit {
        "s" | "sec" | "second" | "seconds" => Ok(chrono::Duration::seconds(value)),
        "m" | "min" | "minute" | "minutes" => Ok(chrono::Duration::minutes(value)),
        "h" | "hour" | "hours" => Ok(chrono::Duration::hours(value)),
        "d" | "day" | "days" => Ok(chrono::Duration::days(value)),
        "w" | "week" | "weeks" => Ok(chrono::Duration::weeks(value)),
        "y" | "year" | "years" => Ok(chrono::Duration::days(value * 365)),
        "" => Err("missing unit (use s/m/h/d/w/y)".to_string()),
        _ => Err(format!("unknown unit: '{}' (use s/m/h/d/w/y)", unit)),
    }
}

pub fn execute_builtin(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    match name {
        "concat" => {
            let sa = value_to_string_auto(&args[0], source, line)?;
            let sb = value_to_string_auto(&args[1], source, line)?;
            Ok(Value::String(format!("{}{}", sa, sb)))
        }
        "map" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            // Prepend map name if not already present
                            if !err.message.starts_with("map:") {
                                err.message = format!("map: {}", err.message);
                            }
                            err
                        })?;
                    out.push(res);
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "filter" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            // Prepend filter name if not already present
                            if !err.message.starts_with("filter:") {
                                err.message = format!("filter: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::Bool(true) => out.push(item.clone()),
                        Value::Bool(false) => {}
                        other => {
                            return Err(EvalError::type_mismatch(
                                "bool",
                                other.to_string(source),
                                0,
                            ))
                        }
                    }
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "fold" => {
            let func = &args[0];
            let mut acc = args[1].clone();
            let list = &args[2];
            if let Value::List(items) = list {
                for item in items {
                    let step = apply_function(func, acc, source, line).map_err(|mut err| {
                        // Prepend fold name if not already present
                        if !err.message.starts_with("fold:") {
                            err.message = format!("fold: {}", err.message);
                        }
                        err
                    })?;
                    acc =
                        apply_function(&step, item.clone(), source, line).map_err(|mut err| {
                            // Prepend fold name if not already present
                            if !err.message.starts_with("fold:") {
                                err.message = format!("fold: {}", err.message);
                            }
                            err
                        })?;
                }
                Ok(acc)
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "import" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let data = std::fs::read_to_string(&p).map_err(|e| {
                EvalError::new(format!("failed to import {}: {}", p, e), None, None, line)
            })?;
            let tokens = tokenize(data.clone())?;
            let ast = parse(tokens);
            let mut env = initial_builtins();
            let val = eval(ast.program, &mut env, &data)?;
            Ok(val)
        }
        "readfile" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let data = std::fs::read_to_string(&p).map_err(|e| {
                EvalError::new(format!("failed to read {}: {}", p, e), None, None, line)
            })?;
            Ok(Value::String(data))
        }
        "fill_template" => {
            // Args: filename (string or path), substitutions (dict or list of [key, value] pairs)
            let pathv = &args[0];
            let subsv = &args[1];

            let filename = value_to_path_string(pathv, source)?;
            // Read the template file
            let mut template = std::fs::read_to_string(&filename).map_err(|e| {
                EvalError::new(
                    "fill_template".to_string(),
                    Some("file".to_string()),
                    Some(e.to_string()),
                    line,
                )
            })?;

            // Process substitutions - accept both dict and list of pairs
            match subsv {
                Value::Dict(map) => {
                    // Modern approach: use dict directly
                    for (key, val) in map.iter() {
                        let val_str = val.to_string(source);
                        let placeholder = format!("{{{}}}", key);
                        template = template.replace(&placeholder, &val_str);
                    }
                    Ok(Value::String(template))
                }
                Value::List(pairs) => {
                    // Legacy approach: list of [key, value] pairs
                    for pair in pairs {
                        if let Value::List(kv) = pair {
                            if kv.len() != 2 {
                                return Err(EvalError::new(
                                    "fill_template",
                                    Some("[key, value] pair".to_string()),
                                    Some(format!("list of {}", kv.len())),
                                    0,
                                ));
                            }

                            let key = match &kv[0] {
                                Value::String(s) => s.clone(),
                                other => {
                                    return Err(EvalError::type_mismatch(
                                        "string",
                                        match other {
                                            Value::Number(_) => "number",
                                            Value::List(_) => "list",
                                            Value::Bool(_) => "bool",
                                            Value::Function { .. } => "function",
                                            _ => "unknown",
                                        },
                                        0,
                                    ))
                                }
                            };

                            let val = kv[1].to_string(source);
                            let placeholder = format!("{{{}}}", key);
                            template = template.replace(&placeholder, &val);
                        } else {
                            return Err(EvalError::new(
                                "fill_template",
                                Some("list".to_string()),
                                Some(pair.to_string(source)),
                                0,
                            ));
                        }
                    }
                    Ok(Value::String(template))
                }
                _ => Err(EvalError::type_mismatch(
                    "dict or list of pairs",
                    match subsv {
                        Value::String(_) => "string",
                        Value::Number(_) => "number",
                        Value::Bool(_) => "bool",
                        Value::Function { .. } => "function",
                        _ => "unknown",
                    },
                    0,
                )),
            }
        }
        "upper" => {
            let s = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::String(s.to_uppercase()))
        }
        "lower" => {
            let s = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::String(s.to_lowercase()))
        }
        "trim" => {
            let s = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::String(s.trim().to_string()))
        }
        "contains" => {
            let sa = value_to_string_auto(&args[0], source, line)?;
            let sb = value_to_string_auto(&args[1], source, line)?;
            Ok(Value::Bool(sa.contains(&sb)))
        }
        "starts_with" => {
            let sa = value_to_string_auto(&args[0], source, line)?;
            let sb = value_to_string_auto(&args[1], source, line)?;
            Ok(Value::Bool(sa.starts_with(&sb)))
        }
        "ends_with" => {
            let sa = value_to_string_auto(&args[0], source, line)?;
            let sb = value_to_string_auto(&args[1], source, line)?;
            Ok(Value::Bool(sa.ends_with(&sb)))
        }
        "split" => {
            let sa = value_to_string_auto(&args[0], source, line)?;
            let sb = value_to_string_auto(&args[1], source, line)?;
            let parts: Vec<Value> = sa
                .split(&sb)
                .map(|s| Value::String(s.to_string()))
                .collect();
            Ok(Value::List(parts))
        }
        "join" => {
            let a = &args[0];
            let sep = value_to_string_auto(&args[1], source, line)?;
            if let Value::List(list) = a {
                let parts: Vec<String> = list.iter().map(|it| it.to_string(source)).collect();
                Ok(Value::String(parts.join(&sep)))
            } else {
                Err(EvalError::type_mismatch("list", a.to_string(source), line))
            }
        }
        "replace" => {
            let sa = value_to_string_auto(&args[0], source, line)?;
            let sb = value_to_string_auto(&args[1], source, line)?;
            let sc = value_to_string_auto(&args[2], source, line)?;
            Ok(Value::String(sa.replace(&sb, &sc)))
        }
        "readlines" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let data = std::fs::read_to_string(&p).map_err(|e| {
                EvalError::new(format!("failed to read {}: {}", p, e), None, None, line)
            })?;
            let lines: Vec<Value> = data.lines().map(|s| Value::String(s.to_string())).collect();
            Ok(Value::List(lines))
        }
        "walkdir" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let mut out = Vec::new();
            let base = std::path::Path::new(&p);
            if base.exists() {
                let mut stack = vec![base.to_path_buf()];
                while let Some(cur) = stack.pop() {
                    if let Ok(md) = std::fs::read_dir(&cur) {
                        for e in md.flatten() {
                            let pth = e.path();
                            out.push(Value::String(pth.to_string_lossy().to_string()));
                            if pth.is_dir() {
                                stack.push(pth);
                            }
                        }
                    }
                }
            }
            Ok(Value::List(out))
        }
        "json_parse" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let data = std::fs::read_to_string(p).map_err(|e| {
                    EvalError::new(format!("failed to read {}: {}", p, e), None, None, line)
                })?;
                let jr: serde_json::Value = serde_json::from_str(&data).map_err(|e| {
                    EvalError::new(format!("json parse error: {}", e), None, None, line)
                })?;
                fn conv(j: &serde_json::Value) -> Value {
                    match j {
                        serde_json::Value::Null => Value::None,
                        serde_json::Value::Bool(b) => Value::Bool(*b),
                        serde_json::Value::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                Value::Number(Number::Int(i))
                            } else if let Some(f) = n.as_f64() {
                                Value::Number(Number::Float(f))
                            } else {
                                Value::None
                            }
                        }
                        serde_json::Value::String(s) => Value::String(s.clone()),
                        serde_json::Value::Array(a) => Value::List(a.iter().map(conv).collect()),
                        serde_json::Value::Object(o) => {
                            // Convert JSON object to Dict (hash map)
                            let mut map = HashMap::new();
                            for (k, v) in o.iter() {
                                map.insert(k.clone(), conv(v));
                            }
                            Value::Dict(map)
                        }
                    }
                }
                Ok(conv(&jr))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    pathv.to_string(source),
                    0,
                ))
            }
        }
        "exists" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            Ok(Value::Bool(std::path::Path::new(&p).exists()))
        }
        "basename" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let b = std::path::Path::new(&p)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            Ok(Value::String(b))
        }
        "dirname" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let d = std::path::Path::new(&p)
                .parent()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            Ok(Value::String(d))
        }
        "length" => match &args[0] {
            Value::String(s) => Ok(Value::Number(Number::Int(s.len() as i64))),
            Value::Template(_, _) => {
                let s = value_to_string_auto(&args[0], source, line)?;
                Ok(Value::Number(Number::Int(s.len() as i64)))
            }
            Value::List(items) => Ok(Value::Number(Number::Int(items.len() as i64))),
            other => Err(EvalError::type_mismatch(
                "string, template, or list",
                other.to_string(source),
                line,
            )),
        },
        "repeat" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            if let Value::Number(Number::Int(count)) = &args[1] {
                Ok(Value::String(st.repeat(*count as usize)))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    args[1].to_string(source),
                    line,
                ))
            }
        }
        "pad_left" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            let width = &args[1];
            let pad = value_to_string_auto(&args[2], source, line)?;
            if let Value::Number(Number::Int(w)) = width {
                let pad_char = pad.chars().next().unwrap_or(' ');
                let result = format!("{:>width$}", st, width = *w as usize)
                    .replace(' ', &pad_char.to_string());
                Ok(Value::String(result))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    width.to_string(source),
                    line,
                ))
            }
        }
        "pad_right" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            let width = &args[1];
            let pad = value_to_string_auto(&args[2], source, line)?;
            if let Value::Number(Number::Int(w)) = width {
                let pad_char = pad.chars().next().unwrap_or(' ');
                let result = format!("{:<width$}", st, width = *w as usize)
                    .replace(' ', &pad_char.to_string());
                Ok(Value::String(result))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    width.to_string(source),
                    line,
                ))
            }
        }
        "indent" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            if let Value::Number(Number::Int(n)) = &args[1] {
                let indent_str = " ".repeat(*n as usize);
                let lines: Vec<String> = st
                    .lines()
                    .map(|line| format!("{}{}", indent_str, line))
                    .collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    args[1].to_string(source),
                    line,
                ))
            }
        }
        "is_digit" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::Bool(
                !st.is_empty() && st.chars().all(|c| c.is_ascii_digit()),
            ))
        }
        "is_alpha" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::Bool(
                !st.is_empty() && st.chars().all(|c| c.is_alphabetic()),
            ))
        }
        "is_alphanumeric" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::Bool(
                !st.is_empty() && st.chars().all(|c| c.is_alphanumeric()),
            ))
        }
        "is_whitespace" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::Bool(
                !st.is_empty() && st.chars().all(|c| c.is_whitespace()),
            ))
        }
        "is_uppercase" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            let letters: Vec<char> = st.chars().filter(|c| c.is_alphabetic()).collect();
            Ok(Value::Bool(
                !letters.is_empty() && letters.iter().all(|c| c.is_uppercase()),
            ))
        }
        "is_lowercase" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            let letters: Vec<char> = st.chars().filter(|c| c.is_alphabetic()).collect();
            Ok(Value::Bool(
                !letters.is_empty() && letters.iter().all(|c| c.is_lowercase()),
            ))
        }
        "is_empty" => match &args[0] {
            Value::String(st) => Ok(Value::Bool(st.is_empty())),
            Value::Template(_, _) => {
                let st = value_to_string_auto(&args[0], source, line)?;
                Ok(Value::Bool(st.is_empty()))
            }
            Value::List(items) => Ok(Value::Bool(items.is_empty())),
            Value::Dict(map) => Ok(Value::Bool(map.is_empty())),
            other => Err(EvalError::type_mismatch(
                "string, template, list, or dict",
                other.to_string(source),
                line,
            )),
        },
        "html_escape" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            let escaped = st
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;");
            Ok(Value::String(escaped))
        }
        "html_tag" => {
            let t = value_to_string_auto(&args[0], source, line)?;
            let c = value_to_string_auto(&args[1], source, line)?;
            Ok(Value::String(format!("<{}>{}</{}>", t, c, t)))
        }
        "html_attr" => {
            let n = value_to_string_auto(&args[0], source, line)?;
            let v = value_to_string_auto(&args[1], source, line)?;
            let escaped = v
                .replace('&', "&amp;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;");
            Ok(Value::String(format!("{}=\"{}\"", n, escaped)))
        }
        "md_heading" => {
            if let Value::Number(Number::Int(lvl)) = &args[0] {
                let txt = value_to_string_auto(&args[1], source, line)?;
                let hashes = "#".repeat((*lvl).clamp(1, 6) as usize);
                Ok(Value::String(format!("{} {}", hashes, txt)))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    args[0].to_string(source),
                    line,
                ))
            }
        }
        "md_link" => {
            let txt = value_to_string_auto(&args[0], source, line)?;
            let u = value_to_string_auto(&args[1], source, line)?;
            Ok(Value::String(format!("[{}]({})", txt, u)))
        }
        "md_code" => {
            let c = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::String(format!("`{}`", c)))
        }
        "md_list" => {
            let items = &args[0];
            if let Value::List(list) = items {
                let lines: Vec<String> = list
                    .iter()
                    .map(|item| format!("- {}", item.to_string(source)))
                    .collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    items.to_string(source),
                    line,
                ))
            }
        }
        "markdown_to_html" => {
            let md = value_to_string_auto(&args[0], source, line)?;
            // Simple markdown to HTML converter
            let lines: Vec<&str> = md.lines().collect();
            let mut html_lines = Vec::new();
            for line in lines {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    html_lines.push("<br>".to_string());
                } else if let Some(text) = trimmed.strip_prefix("# ") {
                    html_lines.push(format!("<h1>{}</h1>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("## ") {
                    html_lines.push(format!("<h2>{}</h2>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("### ") {
                    html_lines.push(format!("<h3>{}</h3>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("#### ") {
                    html_lines.push(format!("<h4>{}</h4>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("##### ") {
                    html_lines.push(format!("<h5>{}</h5>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("###### ") {
                    html_lines.push(format!("<h6>{}</h6>", text.trim()));
                } else {
                    // Process inline formatting: **bold**, *italic*, `code`
                    // Bold: **text** -> <strong>text</strong>
                    let mut result = String::new();
                    let parts: Vec<&str> = trimmed.split("**").collect();
                    for (i, part) in parts.iter().enumerate() {
                        if i % 2 == 1 {
                            result.push_str("<strong>");
                            result.push_str(part);
                            result.push_str("</strong>");
                        } else {
                            // Process italic and code within non-bold parts
                            let italic_parts: Vec<&str> = part.split('*').collect();
                            for (j, italic_part) in italic_parts.iter().enumerate() {
                                if j > 0 && j % 2 == 1 {
                                    result.push_str("<em>");
                                }
                                // Code: `text` -> <code>text</code>
                                let code_parts: Vec<&str> = italic_part.split('`').collect();
                                for (k, code_part) in code_parts.iter().enumerate() {
                                    if k > 0 && k % 2 == 1 {
                                        result.push_str("<code>");
                                    }
                                    result.push_str(code_part);
                                    if k > 0 && k % 2 == 1 {
                                        result.push_str("</code>");
                                    }
                                }
                                if j > 0 && j % 2 == 1 {
                                    result.push_str("</em>");
                                }
                            }
                        }
                    }
                    html_lines.push(format!("<p>{}</p>", result));
                }
            }
            Ok(Value::String(html_lines.join("\n")))
        }
        "to_string" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::String(i.to_string())),
                Value::Number(Number::Float(f)) => {
                    // Format float nicely - remove unnecessary trailing zeros
                    let s = format!("{}", f);
                    Ok(Value::String(s))
                }
                Value::String(s) => Ok(Value::String(s.clone())),
                Value::Bool(b) => Ok(Value::String(b.to_string())),
                other => Ok(Value::String(other.to_string(source))),
            }
        }
        "to_int" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Int(*i))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Int(*f as i64))),
                Value::String(s) => s
                    .trim()
                    .parse::<i64>()
                    .map(|i| Value::Number(Number::Int(i)))
                    .map_err(|_| {
                        EvalError::new(format!("cannot convert '{}' to int", s), None, None, line)
                    }),
                Value::Bool(b) => Ok(Value::Number(Number::Int(if *b { 1 } else { 0 }))),
                other => Err(EvalError::new(
                    format!("cannot convert {} to int", other.to_string(source)),
                    None,
                    None,
                    0,
                )),
            }
        }
        "to_float" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Float(*i as f64))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Float(*f))),
                Value::String(s) => s
                    .trim()
                    .parse::<f64>()
                    .map(|f| Value::Number(Number::Float(f)))
                    .map_err(|_| {
                        EvalError::new(format!("cannot convert '{}' to float", s), None, None, line)
                    }),
                other => Err(EvalError::new(
                    format!("cannot convert {} to float", other.to_string(source)),
                    None,
                    None,
                    0,
                )),
            }
        }
        "to_bool" => {
            let val = &args[0];
            match val {
                Value::Bool(b) => Ok(Value::Bool(*b)),
                Value::Number(Number::Int(i)) => Ok(Value::Bool(*i != 0)),
                Value::Number(Number::Float(f)) => Ok(Value::Bool(*f != 0.0)),
                Value::String(s) => {
                    let lower = s.to_lowercase();
                    match lower.as_str() {
                        "true" | "yes" | "1" | "on" => Ok(Value::Bool(true)),
                        "false" | "no" | "0" | "off" | "" => Ok(Value::Bool(false)),
                        _ => Err(EvalError::new(
                            format!("cannot convert '{}' to bool", s),
                            None,
                            None,
                            0,
                        )),
                    }
                }
                Value::List(items) => Ok(Value::Bool(!items.is_empty())),
                _ => Ok(Value::Bool(true)), // Other values are truthy
            }
        }
        "to_char" => {
            // to_char :: Number -> String
            // Converts a Unicode codepoint (integer) to a single-character string
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => {
                    let codepoint = *i as u32;
                    match char::from_u32(codepoint) {
                        Some(c) => Ok(Value::String(c.to_string())),
                        None => Err(EvalError::new(
                            format!("to_char: {} is not a valid Unicode codepoint", i),
                            None,
                            None,
                            line,
                        )),
                    }
                }
                Value::Number(Number::Float(f)) => {
                    let codepoint = *f as u32;
                    match char::from_u32(codepoint) {
                        Some(c) => Ok(Value::String(c.to_string())),
                        None => Err(EvalError::new(
                            format!("to_char: {} is not a valid Unicode codepoint", f),
                            None,
                            None,
                            line,
                        )),
                    }
                }
                other => Err(EvalError::type_mismatch(
                    "Number (Unicode codepoint)",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "to_list" => {
            // to_list :: String -> [String]
            // Converts a string to a list of single-character strings
            let val = &args[0];
            match val {
                Value::String(s) => {
                    let chars: Vec<Value> =
                        s.chars().map(|c| Value::String(c.to_string())).collect();
                    Ok(Value::List(chars))
                }
                Value::List(items) => Ok(Value::List(items.clone())), // Already a list
                other => Err(EvalError::type_mismatch(
                    "String or List",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "neg" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Int(-i))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Float(-f))),
                other => Err(EvalError::type_mismatch(
                    "number",
                    other.to_string(source),
                    0,
                )),
            }
        }
        "format_int" => {
            let val = &args[0];
            let width = &args[1];
            if let (Value::Number(num), Value::Number(Number::Int(w))) = (val, width) {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                let formatted = if *w > 0 {
                    format!("{:0width$}", int_val, width = *w as usize)
                } else {
                    format!("{}", int_val)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, number",
                    format!("{}, {}", val.to_string(source), width.to_string(source)),
                    0,
                ))
            }
        }
        "format_float" => {
            let val = &args[0];
            let precision = &args[1];
            if let (Value::Number(num), Value::Number(Number::Int(p))) = (val, precision) {
                let float_val = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = if *p >= 0 {
                    format!("{:.prec$}", float_val, prec = *p as usize)
                } else {
                    format!("{}", float_val)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, number",
                    format!("{}, {}", val.to_string(source), precision.to_string(source)),
                    0,
                ))
            }
        }
        "format_hex" => {
            // format_hex :: Number -> String
            // Formats a number as hexadecimal (lowercase)
            let val = &args[0];
            if let Value::Number(num) = val {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                Ok(Value::String(format!("{:x}", int_val)))
            } else {
                Err(EvalError::type_mismatch("number", val.to_string(source), 0))
            }
        }
        "format_octal" => {
            // format_octal :: Number -> String
            // Formats a number as octal
            let val = &args[0];
            if let Value::Number(num) = val {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                Ok(Value::String(format!("{:o}", int_val)))
            } else {
                Err(EvalError::type_mismatch("number", val.to_string(source), 0))
            }
        }
        "format_binary" => {
            // format_binary :: Number -> String
            // Formats a number as binary
            let val = &args[0];
            if let Value::Number(num) = val {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                Ok(Value::String(format!("{:b}", int_val)))
            } else {
                Err(EvalError::type_mismatch("number", val.to_string(source), 0))
            }
        }
        "format_scientific" => {
            // format_scientific :: Number -> Int -> String
            // Formats a number in scientific notation with specified precision
            let val = &args[0];
            let precision = &args[1];
            if let (Value::Number(num), Value::Number(Number::Int(p))) = (val, precision) {
                let float_val = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = if *p >= 0 {
                    format!("{:.prec$e}", float_val, prec = *p as usize)
                } else {
                    format!("{:e}", float_val)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, number",
                    format!("{}, {}", val.to_string(source), precision.to_string(source)),
                    0,
                ))
            }
        }
        "format_bytes" => {
            // format_bytes :: Number -> String
            // Formats a number as human-readable bytes (KB, MB, GB, etc.)
            let val = &args[0];
            if let Value::Number(num) = val {
                let bytes = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = if bytes < 1024.0 {
                    format!("{} B", bytes as i64)
                } else if bytes < 1024.0 * 1024.0 {
                    format!("{:.2} KB", bytes / 1024.0)
                } else if bytes < 1024.0 * 1024.0 * 1024.0 {
                    format!("{:.2} MB", bytes / (1024.0 * 1024.0))
                } else if bytes < 1024.0 * 1024.0 * 1024.0 * 1024.0 {
                    format!("{:.2} GB", bytes / (1024.0 * 1024.0 * 1024.0))
                } else {
                    format!("{:.2} TB", bytes / (1024.0 * 1024.0 * 1024.0 * 1024.0))
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch("number", val.to_string(source), 0))
            }
        }
        "format_list" => {
            // format_list :: [a] -> String -> String
            // Formats a list with a custom separator
            let list = &args[0];
            let separator = &args[1];
            if let (Value::List(items), Value::String(sep)) = (list, separator) {
                let strings: Vec<String> = items.iter().map(|v| v.to_string(source)).collect();
                Ok(Value::String(strings.join(sep)))
            } else {
                Err(EvalError::type_mismatch(
                    "list, string",
                    format!(
                        "{}, {}",
                        list.to_string(source),
                        separator.to_string(source)
                    ),
                    0,
                ))
            }
        }
        "format_table" => {
            // format_table :: ([[a]]|Dict) -> String -> String
            // Formats a 2D list as a simple table with column separator
            // Also accepts a dict, which is converted to [keys, values] format
            let table = &args[0];
            let separator = &args[1];

            if let Value::String(sep) = separator {
                let rows: Vec<Vec<String>> = match table {
                    Value::Dict(dict) => {
                        // Convert dict to table format: [keys_row, values_row]
                        let keys_row: Vec<String> = dict.keys().cloned().collect();
                        let values_row: Vec<String> =
                            dict.values().map(|v| v.to_string(source)).collect();
                        vec![keys_row, values_row]
                    }
                    Value::List(rows) => {
                        // Existing behavior: list of lists
                        let mut result = Vec::new();
                        for row in rows {
                            if let Value::List(cols) = row {
                                let strings: Vec<String> =
                                    cols.iter().map(|v| v.to_string(source)).collect();
                                result.push(strings);
                            } else {
                                result.push(vec![row.to_string(source)]);
                            }
                        }
                        result
                    }
                    _ => {
                        return Err(EvalError::type_mismatch(
                            "list of lists or dict",
                            table.to_string(source),
                            0,
                        ));
                    }
                };

                let lines: Vec<String> = rows.iter().map(|row| row.join(sep)).collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    separator.to_string(source),
                    0,
                ))
            }
        }
        "format_json" => {
            // format_json :: a -> String
            // Formats any value as JSON (basic implementation)
            let val = &args[0];
            let json_str = match val {
                Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
                Value::Number(Number::Int(i)) => format!("{}", i),
                Value::Number(Number::Float(f)) => format!("{}", f),
                Value::Bool(b) => format!("{}", b),
                Value::List(items) => {
                    let json_items: Vec<String> = items
                        .iter()
                        .map(|v| {
                            match execute_builtin(
                                "format_json",
                                std::slice::from_ref(v),
                                source,
                                line,
                            ) {
                                Ok(Value::String(s)) => s,
                                _ => v.to_string(source),
                            }
                        })
                        .collect();
                    format!("[{}]", json_items.join(", "))
                }
                Value::None => "null".to_string(),
                other => format!(
                    "\"{}\"",
                    other
                        .to_string(source)
                        .replace('\\', "\\\\")
                        .replace('"', "\\\"")
                ),
            };
            Ok(Value::String(json_str))
        }
        "format_currency" => {
            // format_currency :: Number -> String -> String
            // Formats a number as currency with the given symbol
            let val = &args[0];
            let symbol = &args[1];
            if let (Value::Number(num), Value::String(sym)) = (val, symbol) {
                let float_val = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = format!("{}{:.2}", sym, float_val);
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, string",
                    format!("{}, {}", val.to_string(source), symbol.to_string(source)),
                    0,
                ))
            }
        }
        "format_percent" => {
            // format_percent :: Number -> Int -> String
            // Formats a number as a percentage with specified decimal places
            let val = &args[0];
            let precision = &args[1];
            if let (Value::Number(num), Value::Number(Number::Int(p))) = (val, precision) {
                let float_val = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = if *p >= 0 {
                    format!("{:.prec$}%", float_val * 100.0, prec = *p as usize)
                } else {
                    format!("{}%", float_val * 100.0)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, number",
                    format!("{}, {}", val.to_string(source), precision.to_string(source)),
                    0,
                ))
            }
        }
        "format_bool" => {
            // format_bool :: Bool -> String -> String
            // Formats a boolean with custom true/false strings (e.g., "Yes"/"No")
            let val = &args[0];
            let format_style = &args[1];
            if let (Value::Bool(b), Value::String(style)) = (val, format_style) {
                let result = match style.to_lowercase().as_str() {
                    "yesno" | "yes/no" => {
                        if *b {
                            "Yes"
                        } else {
                            "No"
                        }
                    }
                    "onoff" | "on/off" => {
                        if *b {
                            "On"
                        } else {
                            "Off"
                        }
                    }
                    "10" | "1/0" => {
                        if *b {
                            "1"
                        } else {
                            "0"
                        }
                    }
                    "enabled" => {
                        if *b {
                            "Enabled"
                        } else {
                            "Disabled"
                        }
                    }
                    "active" => {
                        if *b {
                            "Active"
                        } else {
                            "Inactive"
                        }
                    }
                    "success" => {
                        if *b {
                            "Success"
                        } else {
                            "Failure"
                        }
                    }
                    _ => {
                        if *b {
                            "true"
                        } else {
                            "false"
                        }
                    }
                };
                Ok(Value::String(result.to_string()))
            } else {
                Err(EvalError::type_mismatch(
                    "bool, string",
                    format!(
                        "{}, {}",
                        val.to_string(source),
                        format_style.to_string(source)
                    ),
                    0,
                ))
            }
        }
        "truncate" => {
            // truncate :: String -> Int -> String
            // Truncates a string to the specified length, adding "..." if truncated
            let text = &args[0];
            let max_len = &args[1];
            if let (Value::String(s), Value::Number(Number::Int(len))) = (text, max_len) {
                let max_length = (*len).max(0) as usize;
                if s.len() <= max_length {
                    Ok(Value::String(s.clone()))
                } else if max_length <= 3 {
                    Ok(Value::String(s.chars().take(max_length).collect()))
                } else {
                    let truncated: String = s.chars().take(max_length - 3).collect();
                    Ok(Value::String(format!("{}...", truncated)))
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string, number",
                    format!("{}, {}", text.to_string(source), max_len.to_string(source)),
                    0,
                ))
            }
        }
        "center" => {
            // center :: String -> Int -> String
            // Centers a string within the specified width
            let text = &args[0];
            let width = &args[1];
            if let (Value::String(s), Value::Number(Number::Int(w))) = (text, width) {
                let total_width = (*w).max(0) as usize;
                if s.len() >= total_width {
                    Ok(Value::String(s.clone()))
                } else {
                    let padding = total_width - s.len();
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    Ok(Value::String(format!(
                        "{}{}{}",
                        " ".repeat(left_pad),
                        s,
                        " ".repeat(right_pad)
                    )))
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string, number",
                    format!("{}, {}", text.to_string(source), width.to_string(source)),
                    0,
                ))
            }
        }
        "flatmap" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            // Prepend flatmap name if not already present
                            if !err.message.starts_with("flatmap:") {
                                err.message = format!("flatmap: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::List(sub_items) => out.extend(sub_items),
                        single => out.push(single),
                    }
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "flatten" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    match item {
                        Value::List(sub_items) => out.extend(sub_items.clone()),
                        single => out.push(single.clone()),
                    }
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "head" => {
            // head :: [a] -> a | None
            // Returns first element of list, or None if list is empty
            let list = &args[0];
            if let Value::List(items) = list {
                Ok(items.first().cloned().unwrap_or(Value::None))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "tail" => {
            // tail :: [a] -> [a]
            // Returns all elements except first, or empty list if already empty
            let list = &args[0];
            if let Value::List(items) = list {
                if items.is_empty() {
                    Ok(Value::List(Vec::new()))
                } else {
                    Ok(Value::List(items[1..].to_vec()))
                }
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "take" => {
            let n_val = &args[0];
            let list = &args[1];
            let n = match n_val {
                Value::Number(Number::Int(i)) => *i as usize,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        n_val.to_string(source),
                        0,
                    ))
                }
            };
            if let Value::List(items) = list {
                Ok(Value::List(items.iter().take(n).cloned().collect()))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "drop" => {
            let n_val = &args[0];
            let list = &args[1];
            let n = match n_val {
                Value::Number(Number::Int(i)) => *i as usize,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        n_val.to_string(source),
                        0,
                    ))
                }
            };
            if let Value::List(items) = list {
                Ok(Value::List(items.iter().skip(n).cloned().collect()))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "zip" => {
            let list1 = &args[0];
            let list2 = &args[1];
            if let (Value::List(items1), Value::List(items2)) = (list1, list2) {
                let mut out = Vec::new();
                let min_len = items1.len().min(items2.len());
                for i in 0..min_len {
                    out.push(Value::List(vec![items1[i].clone(), items2[i].clone()]));
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    format!("{}, {}", list1.to_string(source), list2.to_string(source)),
                    0,
                ))
            }
        }
        "unzip" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut list1 = Vec::new();
                let mut list2 = Vec::new();
                for item in items {
                    if let Value::List(pair) = item {
                        if pair.len() >= 2 {
                            list1.push(pair[0].clone());
                            list2.push(pair[1].clone());
                        }
                    }
                }
                Ok(Value::List(vec![Value::List(list1), Value::List(list2)]))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "split_at" => {
            let n_val = &args[0];
            let list = &args[1];
            let n = match n_val {
                Value::Number(Number::Int(i)) => *i as usize,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        n_val.to_string(source),
                        0,
                    ))
                }
            };
            if let Value::List(items) = list {
                let first = Value::List(items.iter().take(n).cloned().collect());
                let second = Value::List(items.iter().skip(n).cloned().collect());
                Ok(Value::List(vec![first, second]))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "partition" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut true_list = Vec::new();
                let mut false_list = Vec::new();
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("partition:") {
                                err.message = format!("partition: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::Bool(true) => true_list.push(item.clone()),
                        Value::Bool(false) => false_list.push(item.clone()),
                        other => {
                            return Err(EvalError::type_mismatch(
                                "bool",
                                other.to_string(source),
                                0,
                            ))
                        }
                    }
                }
                Ok(Value::List(vec![
                    Value::List(true_list),
                    Value::List(false_list),
                ]))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "reverse" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut reversed = items.clone();
                reversed.reverse();
                Ok(Value::List(reversed))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "sort" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut sorted = items.clone();
                // Sort by converting to strings and comparing
                sorted.sort_by(|a, b| {
                    let a_str = a.to_string(source);
                    let b_str = b.to_string(source);
                    // Try numeric comparison first
                    match (a, b) {
                        (Value::Number(Number::Int(a_int)), Value::Number(Number::Int(b_int))) => {
                            a_int.cmp(b_int)
                        }
                        (
                            Value::Number(Number::Float(a_float)),
                            Value::Number(Number::Float(b_float)),
                        ) => a_float
                            .partial_cmp(b_float)
                            .unwrap_or(std::cmp::Ordering::Equal),
                        (
                            Value::Number(Number::Int(a_int)),
                            Value::Number(Number::Float(b_float)),
                        ) => (*a_int as f64)
                            .partial_cmp(b_float)
                            .unwrap_or(std::cmp::Ordering::Equal),
                        (
                            Value::Number(Number::Float(a_float)),
                            Value::Number(Number::Int(b_int)),
                        ) => a_float
                            .partial_cmp(&(*b_int as f64))
                            .unwrap_or(std::cmp::Ordering::Equal),
                        // For non-numeric, use string comparison
                        _ => a_str.cmp(&b_str),
                    }
                });
                Ok(Value::List(sorted))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "sort_by" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                // Create pairs of (original_item, sort_key)
                let mut pairs: Vec<(Value, Value)> = Vec::new();
                for item in items {
                    let key =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("sort_by:") {
                                err.message = format!("sort_by: {}", err.message);
                            }
                            err
                        })?;
                    pairs.push((item.clone(), key));
                }

                // Sort by keys
                pairs.sort_by(|(_, a_key), (_, b_key)| {
                    let a_str = a_key.to_string(source);
                    let b_str = b_key.to_string(source);
                    match (a_key, b_key) {
                        (Value::Number(Number::Int(a_int)), Value::Number(Number::Int(b_int))) => {
                            a_int.cmp(b_int)
                        }
                        (
                            Value::Number(Number::Float(a_float)),
                            Value::Number(Number::Float(b_float)),
                        ) => a_float
                            .partial_cmp(b_float)
                            .unwrap_or(std::cmp::Ordering::Equal),
                        (
                            Value::Number(Number::Int(a_int)),
                            Value::Number(Number::Float(b_float)),
                        ) => (*a_int as f64)
                            .partial_cmp(b_float)
                            .unwrap_or(std::cmp::Ordering::Equal),
                        (
                            Value::Number(Number::Float(a_float)),
                            Value::Number(Number::Int(b_int)),
                        ) => a_float
                            .partial_cmp(&(*b_int as f64))
                            .unwrap_or(std::cmp::Ordering::Equal),
                        _ => a_str.cmp(&b_str),
                    }
                });

                // Extract sorted items
                let sorted: Vec<Value> = pairs.into_iter().map(|(item, _)| item).collect();
                Ok(Value::List(sorted))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "unique" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut seen = HashSet::new();
                let mut result = Vec::new();
                for item in items {
                    // Use string representation for uniqueness check
                    let key = item.to_string(source);
                    if seen.insert(key) {
                        result.push(item.clone());
                    }
                }
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "range" => {
            let start = &args[0];
            let end = &args[1];

            match (start, end) {
                (Value::Number(Number::Int(s)), Value::Number(Number::Int(e))) => {
                    if s <= e {
                        let result: Vec<Value> =
                            (*s..=*e).map(|i| Value::Number(Number::Int(i))).collect();
                        Ok(Value::List(result))
                    } else {
                        // Empty list for invalid range
                        Ok(Value::List(Vec::new()))
                    }
                }
                _ => Err(EvalError::type_mismatch(
                    "two integers",
                    format!("{}, {}", start.to_string(source), end.to_string(source)),
                    0,
                )),
            }
        }
        "enumerate" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let result: Vec<Value> = items
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        Value::List(vec![Value::Number(Number::Int(idx as i64)), item.clone()])
                    })
                    .collect();
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "get" => {
            // get :: (Dict|[[String, a]]) -> String -> a
            // Works with both dicts and list of pairs
            // Usage: get {name: "alice", age: 30} "name" => "alice"
            //        get [["name", "alice"], ["age", "30"]] "name" => "alice"
            // Returns None if key not found - use has_key to check first, or is_none after
            let map = &args[0];
            let key = &args[1];
            if let Value::String(k) = key {
                match map {
                    Value::Dict(dict) => Ok(dict.get(k).cloned().unwrap_or(Value::None)),
                    Value::List(pairs) => {
                        for pair in pairs {
                            if let Value::List(kv) = pair {
                                if kv.len() >= 2 {
                                    if let Value::String(pair_key) = &kv[0] {
                                        if pair_key == k {
                                            return Ok(kv[1].clone());
                                        }
                                    }
                                }
                            }
                        }
                        // Key not found - return None
                        Ok(Value::None)
                    }
                    _ => Err(EvalError::type_mismatch(
                        "dict or list of pairs",
                        map.to_string(source),
                        0,
                    )),
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string key",
                    key.to_string(source),
                    0,
                ))
            }
        }
        "set" => {
            // set :: (Dict|[[String, a]]) -> String -> a -> (Dict|[[String, a]])
            // Works with both dicts and list of pairs
            // Usage: set {name: "alice"} "age" 30 => {name: "alice", age: 30}
            //        set [["name", "alice"]] "age" 30 => [["name", "alice"], ["age", 30]]
            let map = &args[0];
            let key = &args[1];
            let value = &args[2];
            if let Value::String(k) = key {
                match map {
                    Value::Dict(dict) => {
                        let mut new_dict = dict.clone();
                        new_dict.insert(k.clone(), value.clone());
                        Ok(Value::Dict(new_dict))
                    }
                    Value::List(pairs) => {
                        let mut new_pairs = Vec::new();
                        let mut found = false;

                        // Update existing key or keep pairs
                        for pair in pairs {
                            if let Value::List(kv) = pair {
                                if kv.len() >= 2 {
                                    if let Value::String(pair_key) = &kv[0] {
                                        if pair_key == k {
                                            // Replace the value for this key
                                            new_pairs.push(Value::List(vec![
                                                Value::String(k.clone()),
                                                value.clone(),
                                            ]));
                                            found = true;
                                        } else {
                                            // Keep unchanged
                                            new_pairs.push(pair.clone());
                                        }
                                    } else {
                                        new_pairs.push(pair.clone());
                                    }
                                } else {
                                    new_pairs.push(pair.clone());
                                }
                            } else {
                                new_pairs.push(pair.clone());
                            }
                        }

                        // If key wasn't found, add it
                        if !found {
                            new_pairs
                                .push(Value::List(vec![Value::String(k.clone()), value.clone()]));
                        }

                        Ok(Value::List(new_pairs))
                    }
                    _ => Err(EvalError::type_mismatch(
                        "dict or list of pairs",
                        map.to_string(source),
                        0,
                    )),
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string key",
                    key.to_string(source),
                    0,
                ))
            }
        }
        "keys" => {
            // keys :: (Dict|[[String, a]]) -> [String]
            // Works with both dicts and list of pairs
            // Usage: keys {name: "alice", age: 30} => ["name", "age"]
            //        keys [["name", "alice"], ["age", "30"]] => ["name", "age"]
            let map = &args[0];
            match map {
                Value::Dict(dict) => {
                    let keys: Vec<Value> = dict.keys().cloned().map(Value::String).collect();
                    Ok(Value::List(keys))
                }
                Value::List(pairs) => {
                    let mut keys = Vec::new();
                    for pair in pairs {
                        if let Value::List(kv) = pair {
                            if !kv.is_empty() {
                                keys.push(kv[0].clone());
                            }
                        }
                    }
                    Ok(Value::List(keys))
                }
                _ => Err(EvalError::type_mismatch(
                    "dict or list of pairs",
                    map.to_string(source),
                    0,
                )),
            }
        }
        "values" => {
            // values :: (Dict|[[String, a]]) -> [a]
            // Works with both dicts and list of pairs
            // Usage: values {name: "alice", age: 30} => ["alice", 30]
            //        values [["name", "alice"], ["age", "30"]] => ["alice", "30"]
            let map = &args[0];
            match map {
                Value::Dict(dict) => {
                    let vals: Vec<Value> = dict.values().cloned().collect();
                    Ok(Value::List(vals))
                }
                Value::List(pairs) => {
                    let mut vals = Vec::new();
                    for pair in pairs {
                        if let Value::List(kv) = pair {
                            if kv.len() >= 2 {
                                vals.push(kv[1].clone());
                            }
                        }
                    }
                    Ok(Value::List(vals))
                }
                _ => Err(EvalError::type_mismatch(
                    "dict or list of pairs",
                    map.to_string(source),
                    0,
                )),
            }
        }
        "has_key" => {
            // has_key :: (Dict|[[String, a]]) -> String -> Bool
            // Works with both dicts and list of pairs
            // Usage: has_key {name: "alice"} "name" => true
            //        has_key [["name", "alice"]] "name" => true
            let map = &args[0];
            let key = &args[1];
            if let Value::String(k) = key {
                match map {
                    Value::Dict(dict) => Ok(Value::Bool(dict.contains_key(k))),
                    Value::List(pairs) => {
                        for pair in pairs {
                            if let Value::List(kv) = pair {
                                if !kv.is_empty() {
                                    if let Value::String(pair_key) = &kv[0] {
                                        if pair_key == k {
                                            return Ok(Value::Bool(true));
                                        }
                                    }
                                }
                            }
                        }
                        Ok(Value::Bool(false))
                    }
                    _ => Err(EvalError::type_mismatch(
                        "dict or list of pairs",
                        map.to_string(source),
                        0,
                    )),
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string key",
                    key.to_string(source),
                    0,
                ))
            }
        }
        "dict_merge" => {
            // dict_merge :: Dict -> Dict -> Dict
            // Merges two dictionaries, with second dict values overriding first
            // Usage: dict_merge {a: 1} {b: 2} => {a: 1, b: 2}
            //        dict_merge {a: 1} {a: 2} => {a: 2}
            match (&args[0], &args[1]) {
                (Value::Dict(dict1), Value::Dict(dict2)) => {
                    let mut result = dict1.clone();
                    for (k, v) in dict2.iter() {
                        result.insert(k.clone(), v.clone());
                    }
                    Ok(Value::Dict(result))
                }
                (Value::Dict(_), other) => {
                    Err(EvalError::type_mismatch("Dict", other.to_string(source), 1))
                }
                (other, _) => Err(EvalError::type_mismatch("Dict", other.to_string(source), 0)),
            }
        }

        // Type introspection
        "env_var" => {
            // env_var :: String -> String
            // Returns the value of an environment variable.
            // Errors if the variable is not set (fail-fast behavior).
            // Use env_var_or for graceful handling with a default.
            let name = &args[0];
            if let Value::String(key) = name {
                match std::env::var(key) {
                    Ok(val) => Ok(Value::String(val)),
                    Err(_) => Err(EvalError::new(
                        format!("env_var: environment variable '{}' is not set", key),
                        Some("use env_var_or for a default value".to_string()),
                        None,
                        0,
                    )),
                }
            } else {
                Err(EvalError::type_mismatch(
                    "String",
                    name.to_string(source),
                    0,
                ))
            }
        }
        "env_var_or" => {
            // env_var_or :: String -> String -> String
            // Returns the value of an environment variable or a default value if not set.
            let name = &args[0];
            let default = &args[1];
            if let Value::String(key) = name {
                if let Value::String(def_val) = default {
                    match std::env::var(key) {
                        Ok(val) => Ok(Value::String(val)),
                        Err(_) => Ok(Value::String(def_val.clone())),
                    }
                } else {
                    Err(EvalError::type_mismatch(
                        "String (default value)",
                        default.to_string(source),
                        0,
                    ))
                }
            } else {
                Err(EvalError::type_mismatch(
                    "String (variable name)",
                    name.to_string(source),
                    0,
                ))
            }
        }

        // Date/Time operations
        "now" => {
            // now :: String
            // Returns current date/time in ISO 8601 format (RFC 3339)
            let now = Local::now();
            Ok(Value::String(now.to_rfc3339()))
        }
        "date_format" => {
            // date_format :: String -> String -> String
            // Format a date string with a given format
            // First arg: ISO 8601 date string
            // Second arg: format string (strftime format)
            let date_str = value_to_string_auto(&args[0], source, line)?;
            let format_str = value_to_string_auto(&args[1], source, line)?;

            // Parse the date string as RFC 3339 (ISO 8601)
            match DateTime::parse_from_rfc3339(&date_str) {
                Ok(dt) => {
                    let formatted = dt.format(&format_str).to_string();
                    Ok(Value::String(formatted))
                }
                Err(_) => {
                    // Try parsing as RFC 2822
                    match DateTime::parse_from_rfc2822(&date_str) {
                        Ok(dt) => {
                            let formatted = dt.format(&format_str).to_string();
                            Ok(Value::String(formatted))
                        }
                        Err(_) => Err(EvalError::new(
                            format!("date_format: invalid date string: {}", date_str),
                            Some("Expected ISO 8601 (RFC 3339) or RFC 2822 format".to_string()),
                            Some("Example: \"2024-03-15T14:30:00+00:00\"".to_string()),
                            line,
                        )),
                    }
                }
            }
        }
        "date_parse" => {
            // date_parse :: String -> String -> String
            // Parse a date string with a given format and return ISO 8601
            // First arg: date string
            // Second arg: format string (strftime format)
            let date_str = value_to_string_auto(&args[0], source, line)?;
            let format_str = value_to_string_auto(&args[1], source, line)?;

            // Try parsing with the given format
            match NaiveDateTime::parse_from_str(&date_str, &format_str) {
                Ok(naive_dt) => {
                    // Convert to local timezone
                    let local_dt = Local
                        .from_local_datetime(&naive_dt)
                        .earliest()
                        .unwrap_or_else(|| {
                            Local.from_utc_datetime(&naive_dt.and_utc().naive_utc())
                        });
                    Ok(Value::String(local_dt.to_rfc3339()))
                }
                Err(_) => Err(EvalError::new(
                    format!(
                        "date_parse: could not parse '{}' with format '{}'",
                        date_str, format_str
                    ),
                    Some("Check that the format string matches the date string".to_string()),
                    Some("Example: date_parse \"2024-03-15 14:30\" \"%Y-%m-%d %H:%M\"".to_string()),
                    line,
                )),
            }
        }
        "date_add" => {
            // date_add :: String -> String -> String
            // Add duration to a date
            // First arg: ISO 8601 date string
            // Second arg: duration string (e.g., "1d", "2h", "30m", "45s", "1w", "1y")
            let date_str = value_to_string_auto(&args[0], source, line)?;
            let duration_str = value_to_string_auto(&args[1], source, line)?;

            // Parse the date
            let dt = DateTime::parse_from_rfc3339(&date_str).map_err(|_| {
                EvalError::new(
                    format!("date_add: invalid date string: {}", date_str),
                    Some("Expected ISO 8601 (RFC 3339) format".to_string()),
                    Some("Use 'now' or 'date_parse' to create valid dates".to_string()),
                    line,
                )
            })?;

            // Parse duration string
            let duration = parse_duration(&duration_str).map_err(|e| {
                EvalError::new(
                    format!("date_add: invalid duration: {}", e),
                    Some("Format: number + unit (s/m/h/d/w/y)".to_string()),
                    Some("Examples: \"1d\", \"2h\", \"30m\", \"1w\", \"1y\"".to_string()),
                    line,
                )
            })?;

            let new_dt = dt + duration;
            Ok(Value::String(new_dt.to_rfc3339()))
        }
        "date_diff" => {
            // date_diff :: String -> String -> Number
            // Calculate difference between two dates in seconds
            // First arg: ISO 8601 date string (later date)
            // Second arg: ISO 8601 date string (earlier date)
            let date1_str = value_to_string_auto(&args[0], source, line)?;
            let date2_str = value_to_string_auto(&args[1], source, line)?;

            let dt1 = DateTime::parse_from_rfc3339(&date1_str).map_err(|_| {
                EvalError::new(
                    format!("date_diff: invalid first date string: {}", date1_str),
                    Some("Expected ISO 8601 (RFC 3339) format".to_string()),
                    None,
                    line,
                )
            })?;

            let dt2 = DateTime::parse_from_rfc3339(&date2_str).map_err(|_| {
                EvalError::new(
                    format!("date_diff: invalid second date string: {}", date2_str),
                    Some("Expected ISO 8601 (RFC 3339) format".to_string()),
                    None,
                    line,
                )
            })?;

            let diff = dt1.signed_duration_since(dt2);
            Ok(Value::Number(Number::Int(diff.num_seconds())))
        }
        "timestamp" => {
            // timestamp :: Number
            // Returns current Unix timestamp (seconds since epoch)
            let now = Utc::now();
            Ok(Value::Number(Number::Int(now.timestamp())))
        }
        "timezone" => {
            // timezone :: String
            // Returns current timezone offset (e.g., "+00:00", "-05:00")
            let now = Local::now();
            Ok(Value::String(now.offset().to_string()))
        }

        "typeof" => {
            // typeof :: a -> String
            // Returns the type name of a value
            let val = &args[0];
            let type_name = match val {
                Value::String(_) => "String",
                Value::Number(_) => "Number",
                Value::Bool(_) => "Bool",
                Value::List(_) => "List",
                Value::Function { .. } => "Function",
                Value::Builtin(_, _) => "Builtin",
                Value::FileTemplate { .. } => "FileTemplate",
                Value::Template(_, _) => "Template",
                Value::Path(_, _) => "Path",
                Value::Dict(_) => "Dict",
                Value::None => "None",
            };
            Ok(Value::String(type_name.to_string()))
        }

        // Type predicates
        "is_string" => {
            // is_string :: a -> Bool
            Ok(Value::Bool(matches!(args[0], Value::String(_))))
        }
        "is_number" => {
            // is_number :: a -> Bool
            Ok(Value::Bool(matches!(args[0], Value::Number(_))))
        }
        "is_int" => {
            // is_int :: a -> Bool
            Ok(Value::Bool(matches!(
                args[0],
                Value::Number(Number::Int(_))
            )))
        }
        "is_float" => {
            // is_float :: a -> Bool
            Ok(Value::Bool(matches!(
                args[0],
                Value::Number(Number::Float(_))
            )))
        }
        "is_list" => {
            // is_list :: a -> Bool
            Ok(Value::Bool(matches!(args[0], Value::List(_))))
        }
        "is_bool" => {
            // is_bool :: a -> Bool
            Ok(Value::Bool(matches!(args[0], Value::Bool(_))))
        }
        "is_function" => {
            // is_function :: a -> Bool
            Ok(Value::Bool(matches!(
                args[0],
                Value::Function { .. } | Value::Builtin(_, _)
            )))
        }
        "is_dict" => {
            // is_dict :: a -> Bool
            Ok(Value::Bool(matches!(args[0], Value::Dict(_))))
        }
        "is_none" => {
            // is_none :: a -> Bool
            // Check if a value is None (from empty list head, missing dict key, or JSON null)
            Ok(Value::Bool(matches!(args[0], Value::None)))
        }
        "not" => {
            // not :: Bool -> Bool
            // Logical negation - returns true if false, false if true
            match &args[0] {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                other => Err(EvalError::type_mismatch("Bool", format!("{:?}", other), 0)),
            }
        }

        // Assertions
        "assert" => {
            // assert :: Bool -> a -> a
            // Returns the second argument if first is true, throws error if false
            match &args[0] {
                Value::Bool(true) => Ok(args[1].clone()),
                Value::Bool(false) => {
                    let debug_info = format!("{:#?}", args[1]);
                    let message = format!("\x1b[91massertion failed\x1b[0m\nvalue: {}", debug_info);
                    Err(EvalError::new(message, None, None, line))
                }
                other => Err(EvalError::type_mismatch("Bool", format!("{:?}", other), 0)),
            }
        }

        // Debugging and error handling
        "error" => {
            // error :: String -> a
            // Throws an error with the given message
            match &args[0] {
                Value::String(msg) => Err(EvalError::new(msg.clone(), None, None, 0)),
                other => Err(EvalError::new(
                    format!(
                        "error expects String message, got: {}",
                        other.to_string(source)
                    ),
                    None,
                    None,
                    0,
                )),
            }
        }
        "trace" => {
            // trace :: String -> a -> a
            // Prints label and value to stderr, returns the value
            let label = &args[0];
            let val = &args[1];
            match label {
                Value::String(s) => {
                    eprintln!("[TRACE] {}: {}", s, val.to_string(source));
                    Ok(val.clone())
                }
                _ => Err(EvalError::type_mismatch(
                    "String (for trace label)",
                    format!("{:?}", label),
                    0,
                )),
            }
        }
        "debug" => {
            // debug :: a -> a
            // Pretty-prints the value structure to stderr, returns the value
            let val = &args[0];
            eprintln!("[DEBUG] {:?}", val);
            Ok(val.clone())
        }

        other => Err(EvalError::new(
            format!("unimplemented builtin {}", other),
            None,
            None,
            0,
        )),
    }
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
