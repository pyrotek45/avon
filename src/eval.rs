use crate::common::{Chunk, EvalError, Expr, Number, Token, Value};
use crate::lexer::tokenize;
use crate::parser::parse;
use std::collections::HashMap;

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

    // Data structures (dict is now literal syntax {key: value})
    // Dict operations
    m.insert(
        "dict_get".to_string(),
        Value::Builtin("dict_get".to_string(), Vec::new()),
    );
    m.insert(
        "dict_set".to_string(),
        Value::Builtin("dict_set".to_string(), Vec::new()),
    );
    m.insert(
        "dict_has_key".to_string(),
        Value::Builtin("dict_has_key".to_string(), Vec::new()),
    );

    // System
    m.insert(
        "os".to_string(),
        Value::String(std::env::consts::OS.to_string()),
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
        match self {
            Value::None => "None".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(Number::Int(v)) => v.to_string(),
            Value::Number(Number::Float(v)) => v.to_string(),
            Value::String(s) => s.clone(),
            Value::Template(chunks, symbols) => {
                let raw = render_chunks_to_string(chunks, symbols, source)
                    .unwrap_or_else(|e| format!("<eval error: {}>", e));
                dedent(&raw)
            }
            Value::Path(chunks, symbols) => {
                let raw = render_chunks_to_string(chunks, symbols, source)
                    .unwrap_or_else(|e| format!("<eval error: {}>", e));
                dedent(&raw)
            }
            Value::FileTemplate {
                path: _p,
                template: t,
            } => {
                let raw = render_chunks_to_string(&t.0, &t.1, source)
                    .unwrap_or_else(|e| format!("<eval error: {}>", e));
                dedent(&raw)
            }
            Value::List(items) => {
                let inner: Vec<String> = items.iter().map(|v| v.to_string(source)).collect();
                format!("[{}]", inner.join(", "))
            }
            Value::Function { .. } => "<function>".to_string(),
            Value::Builtin(name, _collected) => format!("<builtin:{}>", name),
            Value::Dict(map) => {
                let mut entries: Vec<String> = Vec::new();
                for (k, v) in map.iter() {
                    let val_str = match v {
                        Value::String(s) => format!("\"{}\"", s),
                        _ => v.to_string(source),
                    };
                    entries.push(format!("{}: {}", k, val_str));
                }
                format!("{{{}}}", entries.join(", "))
            }
        }
    }
}

// Security: Validate path to prevent directory traversal attacks
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

    // Block absolute paths that try to access /etc or other system paths
    // Allow relative paths and absolute paths that aren't trying to escape
    if path.starts_with('/') && (path.contains("etc/") || path.contains("etc\\")) {
        return Err(EvalError::new(
            format!("Access to system paths not allowed: {}", path),
            None,
            None,
            0,
        ));
    }

    Ok(())
}

// Helper function to extract a file path from either a String or Path value
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

pub fn render_chunks_to_string(
    chunks: &[Chunk],
    symbols: &HashMap<String, Value>,
    source: &str,
) -> Result<String, EvalError> {
    let mut out = String::new();
    for c in chunks.iter() {
        match c {
            Chunk::String(s) => out.push_str(s),
            Chunk::Expr(e) => {
                let tokens = tokenize(e.to_string())?;
                let ast = parse(tokens);
                let mut env = symbols.clone();
                let v = eval(ast.program, &mut env, source)?;
                match v {
                    Value::List(ref items) => {
                        let items_str: Vec<String> =
                            items.iter().map(|it| it.to_string(source)).collect();
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
                    _ => out.push_str(&v.to_string(source)),
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

pub fn eval(
    expr: Expr,
    symbols: &mut HashMap<String, Value>,
    source: &str,
) -> Result<Value, EvalError> {
    let _line = expr.line();
    match expr {
        Expr::Number(value, _) => Ok(Value::Number(value)),
        Expr::String(value, _) => Ok(Value::String(value)),
        Expr::Binary { lhs, op, rhs, line } => {
            let l_eval = eval(*lhs.clone(), symbols, source)?;
            let r_eval = eval(*rhs.clone(), symbols, source)?;

            match op {
                // handle logical operators
                Token::And(_) => match (l_eval.clone(), r_eval.clone()) {
                    (Value::Bool(lb), Value::Bool(rb)) => Ok(Value::Bool(lb && rb)),
                    (a, b) => {
                        let l_type = match a {
                            Value::Number(_) => "Number",
                            Value::String(_) => "String",
                            Value::List(_) => "List",
                            Value::Bool(_) => "Bool",
                            Value::Function { .. } => "Function",
                            _ => "unknown type",
                        };
                        let r_type = match b {
                            Value::Number(_) => "Number",
                            Value::String(_) => "String",
                            Value::List(_) => "List",
                            Value::Bool(_) => "Bool",
                            Value::Function { .. } => "Function",
                            _ => "unknown type",
                        };

                        Err(EvalError::new(
                            "and",
                            Some(l_type.to_string()),
                            Some(r_type.to_string()),
                            line,
                        ))
                    }
                },
                Token::Or(_) => match (l_eval.clone(), r_eval.clone()) {
                    (Value::Bool(lb), Value::Bool(rb)) => Ok(Value::Bool(lb || rb)),
                    (a, b) => {
                        let l_type = match a {
                            Value::Number(_) => "Number",
                            Value::String(_) => "String",
                            Value::List(_) => "List",
                            Value::Bool(_) => "Bool",
                            Value::Function { .. } => "Function",
                            _ => "unknown type",
                        };
                        let r_type = match b {
                            Value::Number(_) => "Number",
                            Value::String(_) => "String",
                            Value::List(_) => "List",
                            Value::Bool(_) => "Bool",
                            Value::Function { .. } => "Function",
                            _ => "unknown type",
                        };

                        Err(EvalError::new(
                            "or",
                            Some(l_type.to_string()),
                            Some(r_type.to_string()),
                            line,
                        ))
                    }
                },
                Token::Add(_) => match (l_eval.clone(), r_eval.clone()) {
                    (Value::Number(ln), Value::Number(rn)) => Ok(Value::Number(ln.add(rn))),
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
                        // Concatenate template chunks
                        let mut combined_chunks = lchunks.clone();
                        combined_chunks.extend(rchunks.clone());
                        // Merge symbol tables
                        let mut combined_symbols = lsyms.clone();
                        combined_symbols.extend(rsyms.clone());
                        Ok(Value::Template(combined_chunks, combined_symbols))
                    }
                    (Value::Path(lchunks, lsyms), Value::Path(rchunks, rsyms)) => {
                        // Concatenate path chunks with a "/" separator
                        let mut combined_chunks = lchunks.clone();
                        // Add a "/" as a string chunk between the two paths
                        combined_chunks.push(Chunk::String("/".to_string()));
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
                            "+",
                            Some(l_type.to_string()),
                            Some(r_type.to_string()),
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
                        Token::Mul(_) => Value::Number(lnumber.mul(rnumber)),
                        Token::Div(_) => Value::Number(lnumber.div(rnumber)),
                        Token::Sub(_) => Value::Number(lnumber.sub(rnumber)),
                        Token::Mod(_) => Value::Number(lnumber.rem(rnumber)),
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
                        (a, b) => {
                            let sa = a.to_string(source);
                            let sb = b.to_string(source);
                            match op {
                                Token::DoubleEqual(_) => sa == sb,
                                Token::NotEqual(_) => sa != sb,
                                Token::Greater(_) => sa > sb,
                                Token::Less(_) => sa < sb,
                                Token::GreaterEqual(_) => sa >= sb,
                                Token::LessEqual(_) => sa <= sb,
                                _ => false,
                            }
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
            if let Some(value) = symbols.get(&ident) {
                Ok(value.clone())
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
            let mut evalue = eval(*value, symbols, source)?;
            if let Value::Function { ref mut name, .. } = evalue {
                *name = Some(ident.clone());
            }

            // Add binding to current scope, evaluate expression, then remove (stack-based scoping)
            symbols.insert(ident.clone(), evalue);
            let result = eval(*expr, symbols, source);
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
                Some(Box::new(eval(*def_expr_box, symbols, source)?))
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
            let lhs_eval = eval(*lhs, symbols, source)?;
            let arg_val = eval(*rhs, symbols, source)?;
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
        Expr::Template(chunks, _) => Ok(Value::Template(chunks, symbols.clone())),
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
            let cond_eval = eval(*cond, symbols, source)?;
            if let Value::Bool(cond_value) = cond_eval {
                if cond_value {
                    eval(*t, symbols, source)
                } else {
                    eval(*f, symbols, source)
                }
            } else {
                Err(EvalError::type_mismatch(
                    "bool",
                    cond_eval.to_string(source),
                    line,
                ))
            }
        }
        Expr::Path(chunks, _) => Ok(Value::Path(chunks, symbols.clone())),
        Expr::Range {
            start,
            step,
            end,
            line,
        } => {
            let start_val = eval(*start, symbols, source)?;
            let end_val = eval(*end, symbols, source)?;
            let step_val = if let Some(step_expr) = step {
                Some(eval(*step_expr, symbols, source)?)
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
                evaluated.push(eval(item, symbols, source)?);
            }
            Ok(Value::List(evaluated))
        }
        Expr::Dict(pairs, _) => {
            let mut map = HashMap::new();
            for (key, value_expr) in pairs {
                let value = eval(value_expr, symbols, source)?;
                map.insert(key.clone(), value);
            }
            Ok(Value::Dict(map))
        }
        Expr::Member {
            object,
            field,
            line,
        } => {
            let obj_val = eval(*object, symbols, source)?;
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
            path,
            template,
            line: _,
        } => Ok(Value::FileTemplate {
            path: (path, symbols.clone()),
            template: (template, symbols.clone()),
        }),
        Expr::Pipe { lhs, rhs, line } => {
            let lhs_val = eval(*lhs, symbols, source)?;
            let rhs_fn = eval(*rhs, symbols, source)?;
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
            eval(*expr.clone(), &mut new_env, source).map_err(|mut err| {
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
                "dict_get" => 2,
                "dict_set" => 3,
                "dict_has_key" => 2,
                "to_string" => 1,
                "to_int" => 1,
                "to_float" => 1,
                "to_bool" => 1,
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
                "get" => 2,
                "set" => 3,
                "keys" => 1,
                "values" => 1,
                "has_key" => 2,
                "typeof" => 1,
                "is_string" => 1,
                "is_number" => 1,
                "is_int" => 1,
                "is_float" => 1,
                "is_list" => 1,
                "is_bool" => 1,
                "is_function" => 1,
                "is_dict" => 1,
                "assert" => 2,
                "error" => 1,
                "trace" => 2,
                "debug" => 1,
                "os" => 0,
                "env_var" => 1,
                "env_var_or" => 2,
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

pub fn execute_builtin(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    match name {
        "concat" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                let mut out = sa.clone();
                out.push_str(sb);
                Ok(Value::String(out))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    format!("{}, {}", a.to_string(source), b.to_string(source)),
                    0,
                ))
            }
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
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::String(st.to_uppercase()))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "lower" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::String(st.to_lowercase()))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "trim" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::String(st.trim().to_string()))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "contains" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                Ok(Value::Bool(sa.contains(sb)))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    format!("{}, {}", a.to_string(source), b.to_string(source)),
                    0,
                ))
            }
        }
        "starts_with" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                Ok(Value::Bool(sa.starts_with(sb)))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    format!("{}, {}", a.to_string(source), b.to_string(source)),
                    0,
                ))
            }
        }
        "ends_with" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                Ok(Value::Bool(sa.ends_with(sb)))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    format!("{}, {}", a.to_string(source), b.to_string(source)),
                    0,
                ))
            }
        }
        "split" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                let parts: Vec<Value> =
                    sa.split(sb).map(|s| Value::String(s.to_string())).collect();
                Ok(Value::List(parts))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    format!("{}, {}", a.to_string(source), b.to_string(source)),
                    0,
                ))
            }
        }
        "join" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::List(list), Value::String(sep)) = (a, b) {
                let parts: Vec<String> = list.iter().map(|it| it.to_string(source)).collect();
                Ok(Value::String(parts.join(sep)))
            } else {
                Err(EvalError::type_mismatch(
                    "list/string",
                    format!("{}, {}", a.to_string(source), b.to_string(source)),
                    0,
                ))
            }
        }
        "replace" => {
            let a = &args[0];
            let b = &args[1];
            let c = &args[2];
            if let (Value::String(sa), Value::String(sb), Value::String(sc)) = (a, b, c) {
                Ok(Value::String(sa.replace(sb, sc)))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    format!(
                        "{}, {}, {}",
                        a.to_string(source),
                        b.to_string(source),
                        c.to_string(source)
                    ),
                    0,
                ))
            }
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
        "length" => {
            let val = &args[0];
            match val {
                Value::String(s) => Ok(Value::Number(Number::Int(s.len() as i64))),
                Value::List(items) => Ok(Value::Number(Number::Int(items.len() as i64))),
                other => Err(EvalError::type_mismatch(
                    "string or list",
                    other.to_string(source),
                    0,
                )),
            }
        }
        "repeat" => {
            let s = &args[0];
            let n = &args[1];
            if let (Value::String(st), Value::Number(Number::Int(count))) = (s, n) {
                Ok(Value::String(st.repeat(*count as usize)))
            } else {
                Err(EvalError::type_mismatch(
                    "string, number",
                    format!("{}, {}", s.to_string(source), n.to_string(source)),
                    0,
                ))
            }
        }
        "pad_left" => {
            let s = &args[0];
            let width = &args[1];
            let pad = &args[2];
            if let (Value::String(st), Value::Number(Number::Int(w)), Value::String(pc)) =
                (s, width, pad)
            {
                let pad_char = pc.chars().next().unwrap_or(' ');
                let result = format!("{:>width$}", st, width = *w as usize)
                    .replace(' ', &pad_char.to_string());
                Ok(Value::String(result))
            } else {
                Err(EvalError::type_mismatch(
                    "string, number, string",
                    format!(
                        "{}, {}, {}",
                        s.to_string(source),
                        width.to_string(source),
                        pad.to_string(source)
                    ),
                    0,
                ))
            }
        }
        "pad_right" => {
            let s = &args[0];
            let width = &args[1];
            let pad = &args[2];
            if let (Value::String(st), Value::Number(Number::Int(w)), Value::String(pc)) =
                (s, width, pad)
            {
                let pad_char = pc.chars().next().unwrap_or(' ');
                let result = format!("{:<width$}", st, width = *w as usize)
                    .replace(' ', &pad_char.to_string());
                Ok(Value::String(result))
            } else {
                Err(EvalError::type_mismatch(
                    "string, number, string",
                    format!(
                        "{}, {}, {}",
                        s.to_string(source),
                        width.to_string(source),
                        pad.to_string(source)
                    ),
                    0,
                ))
            }
        }
        "indent" => {
            let s = &args[0];
            let spaces = &args[1];
            if let (Value::String(st), Value::Number(Number::Int(n))) = (s, spaces) {
                let indent_str = " ".repeat(*n as usize);
                let lines: Vec<String> = st
                    .lines()
                    .map(|line| format!("{}{}", indent_str, line))
                    .collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch(
                    "string, number",
                    format!("{}, {}", s.to_string(source), spaces.to_string(source)),
                    0,
                ))
            }
        }
        "is_digit" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::Bool(
                    !st.is_empty() && st.chars().all(|c| c.is_ascii_digit()),
                ))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "is_alpha" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::Bool(
                    !st.is_empty() && st.chars().all(|c| c.is_alphabetic()),
                ))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "is_alphanumeric" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::Bool(
                    !st.is_empty() && st.chars().all(|c| c.is_alphanumeric()),
                ))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "is_whitespace" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::Bool(
                    !st.is_empty() && st.chars().all(|c| c.is_whitespace()),
                ))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "is_uppercase" => {
            let s = &args[0];
            if let Value::String(st) = s {
                let letters: Vec<char> = st.chars().filter(|c| c.is_alphabetic()).collect();
                Ok(Value::Bool(
                    !letters.is_empty() && letters.iter().all(|c| c.is_uppercase()),
                ))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "is_lowercase" => {
            let s = &args[0];
            if let Value::String(st) = s {
                let letters: Vec<char> = st.chars().filter(|c| c.is_alphabetic()).collect();
                Ok(Value::Bool(
                    !letters.is_empty() && letters.iter().all(|c| c.is_lowercase()),
                ))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "is_empty" => {
            let s = &args[0];
            match s {
                Value::String(st) => Ok(Value::Bool(st.is_empty())),
                Value::List(items) => Ok(Value::Bool(items.is_empty())),
                other => Err(EvalError::type_mismatch(
                    "string or list",
                    other.to_string(source),
                    0,
                )),
            }
        }
        "html_escape" => {
            let s = &args[0];
            if let Value::String(st) = s {
                let escaped = st
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;")
                    .replace('\'', "&#x27;");
                Ok(Value::String(escaped))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), 0))
            }
        }
        "html_tag" => {
            let tag = &args[0];
            let content = &args[1];
            if let (Value::String(t), Value::String(c)) = (tag, content) {
                Ok(Value::String(format!("<{}>{}</{}>", t, c, t)))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    format!("{}, {}", tag.to_string(source), content.to_string(source)),
                    0,
                ))
            }
        }
        "html_attr" => {
            let name = &args[0];
            let value = &args[1];
            if let (Value::String(n), Value::String(v)) = (name, value) {
                let escaped = v
                    .replace('&', "&amp;")
                    .replace('"', "&quot;")
                    .replace('\'', "&#x27;");
                Ok(Value::String(format!("{}=\"{}\"", n, escaped)))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    format!("{}, {}", name.to_string(source), value.to_string(source)),
                    0,
                ))
            }
        }
        "md_heading" => {
            let level = &args[0];
            let text = &args[1];
            if let (Value::Number(Number::Int(lvl)), Value::String(txt)) = (level, text) {
                let hashes = "#".repeat((*lvl).clamp(1, 6) as usize);
                Ok(Value::String(format!("{} {}", hashes, txt)))
            } else {
                Err(EvalError::type_mismatch(
                    "number, string",
                    format!("{}, {}", level.to_string(source), text.to_string(source)),
                    0,
                ))
            }
        }
        "md_link" => {
            let text = &args[0];
            let url = &args[1];
            if let (Value::String(txt), Value::String(u)) = (text, url) {
                Ok(Value::String(format!("[{}]({})", txt, u)))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    format!("{}, {}", text.to_string(source), url.to_string(source)),
                    0,
                ))
            }
        }
        "md_code" => {
            let code = &args[0];
            if let Value::String(c) = code {
                Ok(Value::String(format!("`{}`", c)))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    code.to_string(source),
                    0,
                ))
            }
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
                Err(EvalError::type_mismatch("list", items.to_string(source), 0))
            }
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
            let list = &args[0];
            if let Value::List(items) = list {
                Ok(items.first().cloned().unwrap_or(Value::None))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), 0))
            }
        }
        "tail" => {
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
        "dict_get" => {
            let dict = &args[0];
            let key = &args[1];
            if let (Value::Dict(map), Value::String(k)) = (dict, key) {
                Ok(map.get(k).cloned().unwrap_or(Value::None))
            } else {
                Err(EvalError::type_mismatch(
                    "Dict and String key",
                    format!("{}, {}", dict.to_string(source), key.to_string(source)),
                    0,
                ))
            }
        }
        "dict_set" => {
            let dict = &args[0];
            let key = &args[1];
            let value = &args[2];
            if let (Value::Dict(map), Value::String(k)) = (dict, key) {
                let mut new_map = map.clone();
                new_map.insert(k.clone(), value.clone());
                Ok(Value::Dict(new_map))
            } else {
                Err(EvalError::type_mismatch(
                    "Dict, String key, and value",
                    format!(
                        "{}, {}, {}",
                        dict.to_string(source),
                        key.to_string(source),
                        value.to_string(source)
                    ),
                    0,
                ))
            }
        }
        "dict_has_key" => {
            let dict = &args[0];
            let key = &args[1];
            if let (Value::Dict(map), Value::String(k)) = (dict, key) {
                Ok(Value::Bool(map.contains_key(k)))
            } else {
                Err(EvalError::type_mismatch(
                    "Dict and String key",
                    format!("{}, {}", dict.to_string(source), key.to_string(source)),
                    0,
                ))
            }
        }
        "get" => {
            // get :: (Dict|[[String, a]]) -> String -> a | None
            // Works with both dicts and list of pairs
            // Usage: get {name: "alice", age: 30} "name" => "alice"
            //        get [["name", "alice"], ["age", "30"]] "name" => "alice"
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

        // Type introspection
        "env_var" => {
            // env_var :: String -> String
            // Returns the value of an environment variable.
            // Errors if the variable is not set (fail-safe by default).
            let name = &args[0];
            if let Value::String(key) = name {
                match std::env::var(key) {
                    Ok(val) => Ok(Value::String(val)),
                    Err(_) => Err(EvalError::new(
                        format!("Missing environment variable: {}", key),
                        None,
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
                let mut res = collect_file_templates(item, source)?;
                out.append(&mut res);
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
