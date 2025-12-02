//! File I/O functions: basename, dirname, exists, fill_template, import, json_parse, readfile, readlines, walkdir

use crate::common::{EvalError, Number, Value};
use crate::eval::{eval, initial_builtins, value_to_path_string};
use crate::lexer::tokenize;
use crate::parser::parse;
use std::collections::HashMap;

/// Names of file I/O builtins
pub const NAMES: &[&str] = &[
    "basename",
    "dirname",
    "exists",
    "fill_template",
    "import",
    "json_parse",
    "readfile",
    "readlines",
    "walkdir",
];

/// Get arity for file I/O functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "basename" | "dirname" | "exists" | "import" | "json_parse" | "readfile" | "readlines"
        | "walkdir" => Some(1),
        "fill_template" => Some(2),
        _ => None,
    }
}

/// Check if name is a file I/O builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a file I/O builtin function
pub fn execute(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    match name {
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
        "readlines" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let data = std::fs::read_to_string(&p).map_err(|e| {
                EvalError::new(format!("failed to read {}: {}", p, e), None, None, line)
            })?;
            let lines: Vec<Value> = data.lines().map(|s| Value::String(s.to_string())).collect();
            Ok(Value::List(lines))
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
                                    line,
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
                                        line,
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
                                line,
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
                    line,
                )),
            }
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
                    line,
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
        _ => Err(EvalError::new(
            format!("unknown file_io function: {}", name),
            None,
            None,
            line,
        )),
    }
}
