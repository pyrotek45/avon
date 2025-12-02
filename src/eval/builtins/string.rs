//! String operations: char_at, chars, concat, contains, ends_with, indent, is_alpha, is_alphanumeric, is_digit, is_empty, is_lowercase, is_uppercase, is_whitespace, join, length, lower, pad_left, pad_right, repeat, replace, slice, split, starts_with, trim, upper

use crate::common::{EvalError, Number, Value};
use crate::eval::value_to_string_auto;

/// Names of string builtins
pub const NAMES: &[&str] = &[
    "char_at",
    "chars",
    "concat",
    "contains",
    "ends_with",
    "indent",
    "is_alpha",
    "is_alphanumeric",
    "is_digit",
    "is_empty",
    "is_lowercase",
    "is_uppercase",
    "is_whitespace",
    "join",
    "length",
    "lower",
    "pad_left",
    "pad_right",
    "repeat",
    "replace",
    "slice",
    "split",
    "starts_with",
    "trim",
    "upper",
];

/// Get arity for string functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "chars" | "is_alpha" | "is_alphanumeric" | "is_digit" | "is_empty" | "is_lowercase"
        | "is_uppercase" | "is_whitespace" | "length" | "lower" | "trim" | "upper" => Some(1),
        "char_at" | "concat" | "contains" | "ends_with" | "indent" | "join" | "repeat"
        | "split" | "starts_with" => Some(2),
        "pad_left" | "pad_right" | "replace" | "slice" => Some(3),
        _ => None,
    }
}

/// Check if name is a string builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a string builtin function
pub fn execute(
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
        "slice" => {
            let collection = &args[0];
            let start_val = &args[1];
            let end_val = &args[2];
            
            let start = match start_val {
                Value::Number(Number::Int(i)) => *i as usize,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        start_val.to_string(source),
                        line,
                    ))
                }
            };
            
            let end = match end_val {
                Value::Number(Number::Int(i)) => *i as usize,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        end_val.to_string(source),
                        line,
                    ))
                }
            };
            
            match collection {
                Value::String(s) => {
                    let chars: Vec<char> = s.chars().collect();
                    let len = chars.len();
                    let start = start.min(len);
                    let end = end.min(len);
                    if start > end {
                        Ok(Value::String(String::new()))
                    } else {
                        let sliced: String = chars[start..end].iter().collect();
                        Ok(Value::String(sliced))
                    }
                }
                Value::List(items) => {
                    let len = items.len();
                    let start = start.min(len);
                    let end = end.min(len);
                    if start > end {
                        Ok(Value::List(Vec::new()))
                    } else {
                        Ok(Value::List(items[start..end].to_vec()))
                    }
                }
                _ => Err(EvalError::type_mismatch(
                    "string or list",
                    collection.to_string(source),
                    line,
                ))
            }
        }
        "char_at" => {
            let str_val = &args[0];
            let idx_val = &args[1];
            
            let idx = match idx_val {
                Value::Number(Number::Int(i)) => *i as usize,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        idx_val.to_string(source),
                        line,
                    ))
                }
            };
            
            match str_val {
                Value::String(s) => {
                    let chars: Vec<char> = s.chars().collect();
                    if idx < chars.len() {
                        Ok(Value::String(chars[idx].to_string()))
                    } else {
                        Ok(Value::None)
                    }
                }
                _ => Err(EvalError::type_mismatch(
                    "string",
                    str_val.to_string(source),
                    line,
                ))
            }
        }
        "chars" => {
            let str_val = &args[0];
            match str_val {
                Value::String(s) => {
                    let char_list: Vec<Value> = s
                        .chars()
                        .map(|c| Value::String(c.to_string()))
                        .collect();
                    Ok(Value::List(char_list))
                }
                _ => Err(EvalError::type_mismatch(
                    "string",
                    str_val.to_string(source),
                    line,
                ))
            }
        }
        _ => Err(EvalError::new(
            format!("unknown string function: {}", name),
            None,
            None,
            line,
        )),
    }
}
