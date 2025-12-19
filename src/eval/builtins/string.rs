//! String operations: base64_decode, base64_encode, char_at, chars, concat, contains, ends_with, hash_md5, hash_sha256, indent, is_alpha, is_alphanumeric, is_digit, is_empty, is_lowercase, is_uppercase, is_whitespace, join, length, lines, lower, pad_left, pad_right, repeat, replace, slice, split, starts_with, trim, unlines, unwords, upper, words

use crate::common::{EvalError, Number, Value};
use crate::eval::value_to_string_auto;
use base64::{engine::general_purpose::STANDARD, Engine};
use md5::Md5;
use sha2::{Digest, Sha256};

/// Names of string builtins
pub const NAMES: &[&str] = &[
    "base64_decode",
    "base64_encode",
    "char_at",
    "chars",
    "concat",
    "contains",
    "ends_with",
    "hash_md5",
    "hash_sha256",
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
    "lines",
    "lower",
    "pad_left",
    "pad_right",
    "repeat",
    "replace",
    "slice",
    "split",
    "starts_with",
    "trim",
    "unlines",
    "unwords",
    "upper",
    "words",
];

/// Get arity for string functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "base64_decode" | "base64_encode" | "chars" | "hash_md5" | "hash_sha256" | "is_alpha"
        | "is_alphanumeric" | "is_digit" | "is_empty" | "is_lowercase" | "is_uppercase"
        | "is_whitespace" | "length" | "lines" | "lower" | "trim" | "upper" | "words" => Some(1),
        "char_at" | "concat" | "contains" | "ends_with" | "indent" | "join" | "repeat"
        | "split" | "starts_with" => Some(2),
        "pad_left" | "pad_right" | "replace" | "slice" => Some(3),
        "unlines" | "unwords" => Some(1),
        _ => None,
    }
}

/// Check if name is a string builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a string builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "base64_encode" => {
            let s = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::String(STANDARD.encode(s.as_bytes())))
        }
        "base64_decode" => {
            let s = value_to_string_auto(&args[0], source, line)?;
            match STANDARD.decode(s.as_bytes()) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(decoded) => Ok(Value::String(decoded)),
                    Err(e) => Err(EvalError::new(
                        format!("base64_decode: decoded bytes are not valid UTF-8: {}", e),
                        None,
                        None,
                        line,
                    )),
                },
                Err(e) => Err(EvalError::new(
                    format!("base64_decode: invalid base64 input: {}", e),
                    None,
                    None,
                    line,
                )),
            }
        }
        "hash_sha256" => {
            let s = value_to_string_auto(&args[0], source, line)?;
            let mut hasher = Sha256::new();
            hasher.update(s.as_bytes());
            let result = hasher.finalize();
            Ok(Value::String(format!("{:x}", result)))
        }
        "hash_md5" => {
            let s = value_to_string_auto(&args[0], source, line)?;
            let mut hasher = Md5::new();
            hasher.update(s.as_bytes());
            let result = hasher.finalize();
            Ok(Value::String(format!("{:x}", result)))
        }
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
            // Overloaded: works for both strings and lists
            // For strings: contains "haystack" "needle" => true if "haystack" contains "needle"
            // For lists: contains elem list => true if list contains elem
            match &args[1] {
                Value::List(items) => {
                    // List membership check - compare using string representation
                    let needle = &args[0];
                    let needle_str = needle.to_string(source);
                    let found = items
                        .iter()
                        .any(|item| item.to_string(source) == needle_str);
                    Ok(Value::Bool(found))
                }
                _ => {
                    // String contains check - original semantics: contains haystack needle
                    let sa = value_to_string_auto(&args[0], source, line)?;
                    let sb = value_to_string_auto(&args[1], source, line)?;
                    Ok(Value::Bool(sa.contains(&sb)))
                }
            }
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
                if *count < 0 {
                    return Err(EvalError::new(
                        format!("repeat count must be non-negative, got {}", count),
                        None,
                        None,
                        line,
                    ));
                }
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
                if *w < 0 {
                    return Err(EvalError::new(
                        format!("pad_left width must be non-negative, got {}", w),
                        None,
                        None,
                        line,
                    ));
                }
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
                if *w < 0 {
                    return Err(EvalError::new(
                        format!("pad_right width must be non-negative, got {}", w),
                        None,
                        None,
                        line,
                    ));
                }
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
                if *n < 0 {
                    return Err(EvalError::new(
                        format!("indent spaces must be non-negative, got {}", n),
                        None,
                        None,
                        line,
                    ));
                }
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
                )),
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
                )),
            }
        }
        "chars" => {
            let str_val = &args[0];
            match str_val {
                Value::String(s) => {
                    let char_list: Vec<Value> =
                        s.chars().map(|c| Value::String(c.to_string())).collect();
                    Ok(Value::List(char_list))
                }
                _ => Err(EvalError::type_mismatch(
                    "string",
                    str_val.to_string(source),
                    line,
                )),
            }
        }
        "words" => {
            let s = value_to_string_auto(&args[0], source, line)?;
            let word_list: Vec<Value> = s
                .split_whitespace()
                .map(|w| Value::String(w.to_string()))
                .collect();
            Ok(Value::List(word_list))
        }
        "unwords" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let words: Vec<String> = items.iter().map(|v| v.to_string(source)).collect();
                Ok(Value::String(words.join(" ")))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "lines" => {
            let s = value_to_string_auto(&args[0], source, line)?;
            // Handle both \n and \r\n line endings
            let line_list: Vec<Value> = s.lines().map(|l| Value::String(l.to_string())).collect();
            Ok(Value::List(line_list))
        }
        "unlines" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let lines: Vec<String> = items.iter().map(|v| v.to_string(source)).collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
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
