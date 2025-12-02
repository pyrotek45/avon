//! Formatting functions: center, format_*, truncate

use crate::common::{EvalError, Number, Value};

/// Names of formatting builtins
pub const NAMES: &[&str] = &[
    "center",
    "format_binary",
    "format_bool",
    "format_bytes",
    "format_currency",
    "format_float",
    "format_hex",
    "format_int",
    "format_json",
    "format_list",
    "format_octal",
    "format_percent",
    "format_scientific",
    "format_table",
    "truncate",
];

/// Get arity for formatting functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "format_binary" | "format_bytes" | "format_hex" | "format_json" | "format_octal" => {
            Some(1)
        }
        "center" | "format_bool" | "format_currency" | "format_float" | "format_int"
        | "format_list" | "format_percent" | "format_scientific" | "format_table" | "truncate" => {
            Some(2)
        }
        _ => None,
    }
}

/// Check if name is a formatting builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a formatting builtin function
pub fn execute(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    match name {
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
                    line,
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
                    line,
                ))
            }
        }
        "format_hex" => {
            let val = &args[0];
            if let Value::Number(num) = val {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                Ok(Value::String(format!("{:x}", int_val)))
            } else {
                Err(EvalError::type_mismatch("number", val.to_string(source), line))
            }
        }
        "format_octal" => {
            let val = &args[0];
            if let Value::Number(num) = val {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                Ok(Value::String(format!("{:o}", int_val)))
            } else {
                Err(EvalError::type_mismatch("number", val.to_string(source), line))
            }
        }
        "format_binary" => {
            let val = &args[0];
            if let Value::Number(num) = val {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                Ok(Value::String(format!("{:b}", int_val)))
            } else {
                Err(EvalError::type_mismatch("number", val.to_string(source), line))
            }
        }
        "format_scientific" => {
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
                    line,
                ))
            }
        }
        "format_bytes" => {
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
                Err(EvalError::type_mismatch("number", val.to_string(source), line))
            }
        }
        "format_list" => {
            let list = &args[0];
            let separator = &args[1];
            if let (Value::List(items), Value::String(sep)) = (list, separator) {
                let strings: Vec<String> = items.iter().map(|v| v.to_string(source)).collect();
                Ok(Value::String(strings.join(sep)))
            } else {
                Err(EvalError::type_mismatch(
                    "list, string",
                    format!("{}, {}", list.to_string(source), separator.to_string(source)),
                    line,
                ))
            }
        }
        "format_table" => {
            let table = &args[0];
            let separator = &args[1];

            if let Value::String(sep) = separator {
                let rows: Vec<Vec<String>> = match table {
                    Value::Dict(dict) => {
                        let keys_row: Vec<String> = dict.keys().cloned().collect();
                        let values_row: Vec<String> =
                            dict.values().map(|v| v.to_string(source)).collect();
                        vec![keys_row, values_row]
                    }
                    Value::List(rows) => {
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
                            line,
                        ));
                    }
                };

                let lines: Vec<String> = rows.iter().map(|row| row.join(sep)).collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    separator.to_string(source),
                    line,
                ))
            }
        }
        "format_json" => {
            let val = &args[0];
            let json_str = format_json_value(val, source, line)?;
            Ok(Value::String(json_str))
        }
        "format_currency" => {
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
                    line,
                ))
            }
        }
        "format_percent" => {
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
                    line,
                ))
            }
        }
        "format_bool" => {
            let val = &args[0];
            let format_style = &args[1];
            if let (Value::Bool(b), Value::String(style)) = (val, format_style) {
                let lower = style.to_lowercase();
                let result = match lower.as_str() {
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
                    "truefalse" | "true/false" => {
                        if *b {
                            "True"
                        } else {
                            "False"
                        }
                    }
                    "10" | "1/0" => {
                        if *b {
                            "1"
                        } else {
                            "0"
                        }
                    }
                    "enabled" | "enabled/disabled" => {
                        if *b {
                            "Enabled"
                        } else {
                            "Disabled"
                        }
                    }
                    "active" | "active/inactive" => {
                        if *b {
                            "Active"
                        } else {
                            "Inactive"
                        }
                    }
                    custom => {
                        let parts: Vec<&str> = custom.split('/').collect();
                        if parts.len() == 2 {
                            if *b {
                                parts[0]
                            } else {
                                parts[1]
                            }
                        } else if *b {
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
                    format!("{}, {}", val.to_string(source), format_style.to_string(source)),
                    line,
                ))
            }
        }
        "truncate" => {
            let text = &args[0];
            let max_len = &args[1];
            if let (Value::String(s), Value::Number(Number::Int(len))) = (text, max_len) {
                let max = (*len).max(0) as usize;
                if s.len() <= max {
                    Ok(Value::String(s.clone()))
                } else if max <= 3 {
                    Ok(Value::String(s.chars().take(max).collect()))
                } else {
                    let truncated: String = s.chars().take(max - 3).collect();
                    Ok(Value::String(format!("{}...", truncated)))
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string, number",
                    format!("{}, {}", text.to_string(source), max_len.to_string(source)),
                    line,
                ))
            }
        }
        "center" => {
            let text = &args[0];
            let width = &args[1];
            if let (Value::String(s), Value::Number(Number::Int(w))) = (text, width) {
                let target_width = (*w).max(0) as usize;
                if s.len() >= target_width {
                    Ok(Value::String(s.clone()))
                } else {
                    let padding = target_width - s.len();
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
                    line,
                ))
            }
        }
        _ => Err(EvalError::new(
            format!("unknown formatting function: {}", name),
            None,
            None,
            line,
        )),
    }
}

// Helper for recursive JSON formatting
fn format_json_value(val: &Value, source: &str, _line: usize) -> Result<String, EvalError> {
    Ok(match val {
        Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        Value::Number(Number::Int(i)) => format!("{}", i),
        Value::Number(Number::Float(f)) => format!("{}", f),
        Value::Bool(b) => format!("{}", b),
        Value::List(items) => {
            let json_items: Vec<String> = items
                .iter()
                .map(|v| format_json_value(v, source, 0).unwrap_or_else(|_| v.to_string(source)))
                .collect();
            format!("[{}]", json_items.join(", "))
        }
        Value::Dict(dict) => {
            let json_pairs: Vec<String> = dict
                .iter()
                .map(|(k, v)| {
                    let json_val = format_json_value(v, source, 0).unwrap_or_else(|_| v.to_string(source));
                    format!("\"{}\": {}", k, json_val)
                })
                .collect();
            format!("{{{}}}", json_pairs.join(", "))
        }
        Value::None => "null".to_string(),
        other => format!(
            "\"{}\"",
            other
                .to_string(source)
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
        ),
    })
}
