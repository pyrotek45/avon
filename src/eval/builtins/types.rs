//! Type checking and conversion functions

use crate::common::{EvalError, Number, Value};
use crate::eval::value_to_string_auto;

/// Names of type checking builtins
#[allow(dead_code)]
pub const TYPE_CHECK_NAMES: &[&str] = &[
    "is_bool",
    "is_dict",
    "is_float",
    "is_function",
    "is_int",
    "is_list",
    "is_none",
    "is_number",
    "is_string",
    "typeof",
];

/// Names of type conversion builtins
#[allow(dead_code)]
pub const TYPE_CONVERT_NAMES: &[&str] = &[
    "to_bool",
    "to_char",
    "to_float",
    "to_int",
    "to_list",
    "to_string",
];

/// All type-related builtin names
pub const NAMES: &[&str] = &[
    // Type checking
    "is_bool",
    "is_dict",
    "is_float",
    "is_function",
    "is_int",
    "is_list",
    "is_none",
    "is_number",
    "is_string",
    "typeof",
    // Type conversion
    "to_bool",
    "to_char",
    "to_float",
    "to_int",
    "to_list",
    "to_string",
];

/// Get arity for type functions (all are arity 1)
pub fn get_arity(name: &str) -> Option<usize> {
    if NAMES.contains(&name) {
        Some(1)
    } else {
        None
    }
}

/// Check if name is a type builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a type builtin function
pub fn execute(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    match name {
        // Type checking
        "typeof" => {
            let val = &args[0];
            let type_name = match val {
                Value::None => "None",
                Value::Bool(_) => "Bool",
                Value::Number(_) => "Number",
                Value::String(_) => "String",
                Value::Template(_, _) => "Template",
                Value::Path(_, _) => "Path",
                Value::List(_) => "List",
                Value::Dict(_) => "Dict",
                Value::Function { .. } => "Function",
                Value::Builtin(_, _) => "Builtin",
                Value::FileTemplate { .. } => "FileTemplate",
            };
            Ok(Value::String(type_name.to_string()))
        }
        "is_string" => Ok(Value::Bool(matches!(args[0], Value::String(_)))),
        "is_number" => Ok(Value::Bool(matches!(args[0], Value::Number(_)))),
        "is_int" => Ok(Value::Bool(matches!(args[0], Value::Number(Number::Int(_))))),
        "is_float" => Ok(Value::Bool(matches!(args[0], Value::Number(Number::Float(_))))),
        "is_list" => Ok(Value::Bool(matches!(args[0], Value::List(_)))),
        "is_bool" => Ok(Value::Bool(matches!(args[0], Value::Bool(_)))),
        "is_function" => Ok(Value::Bool(matches!(
            args[0],
            Value::Function { .. } | Value::Builtin(_, _)
        ))),
        "is_dict" => Ok(Value::Bool(matches!(args[0], Value::Dict(_)))),
        "is_none" => Ok(Value::Bool(matches!(args[0], Value::None))),

        // Type conversion
        "to_string" => {
            let val = &args[0];
            match val {
                Value::String(s) => Ok(Value::String(s.clone())),
                Value::Template(chunks, symbols) => {
                    let s = crate::eval::render_chunks_to_string(chunks, symbols, source)?;
                    Ok(Value::String(s))
                }
                other => Ok(Value::String(other.to_string(source))),
            }
        }
        "to_int" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Int(*i))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Int(*f as i64))),
                Value::String(s) => match s.parse::<i64>() {
                    Ok(i) => Ok(Value::Number(Number::Int(i))),
                    Err(_) => Err(EvalError::new(
                        format!("cannot convert '{}' to int", s),
                        None,
                        None,
                        line,
                    )),
                },
                Value::Bool(b) => Ok(Value::Number(Number::Int(if *b { 1 } else { 0 }))),
                other => Err(EvalError::type_mismatch(
                    "number, string, or bool",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "to_float" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Float(*f))),
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Float(*i as f64))),
                Value::String(s) => match s.parse::<f64>() {
                    Ok(f) => Ok(Value::Number(Number::Float(f))),
                    Err(_) => Err(EvalError::new(
                        format!("cannot convert '{}' to float", s),
                        None,
                        None,
                        line,
                    )),
                },
                other => Err(EvalError::type_mismatch(
                    "number or string",
                    other.to_string(source),
                    line,
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
                            line,
                        )),
                    }
                }
                Value::List(items) => Ok(Value::Bool(!items.is_empty())),
                Value::None => Ok(Value::Bool(false)),
                other => Err(EvalError::type_mismatch(
                    "bool, number, string, list, or none",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "to_char" => {
            // to_char :: Number -> String
            // Converts a Unicode codepoint (integer) to a single-character string
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => {
                    if *i < 0 || *i > 0x10FFFF {
                        return Err(EvalError::new(
                            format!(
                                "codepoint {} is out of range (0-0x10FFFF)",
                                i
                            ),
                            None,
                            None,
                            line,
                        ));
                    }
                    match char::from_u32(*i as u32) {
                        Some(c) => Ok(Value::String(c.to_string())),
                        None => Err(EvalError::new(
                            format!("invalid Unicode codepoint: {}", i),
                            None,
                            None,
                            line,
                        )),
                    }
                }
                Value::Number(Number::Float(f)) => {
                    let i = *f as i64;
                    if i < 0 || i > 0x10FFFF {
                        return Err(EvalError::new(
                            format!(
                                "codepoint {} is out of range (0-0x10FFFF)",
                                i
                            ),
                            None,
                            None,
                            line,
                        ));
                    }
                    match char::from_u32(i as u32) {
                        Some(c) => Ok(Value::String(c.to_string())),
                        None => Err(EvalError::new(
                            format!("invalid Unicode codepoint: {}", i),
                            None,
                            None,
                            line,
                        )),
                    }
                }
                other => Err(EvalError::type_mismatch(
                    "number",
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
                Value::Template(_, _) => {
                    let s = value_to_string_auto(val, source, line)?;
                    let chars: Vec<Value> =
                        s.chars().map(|c| Value::String(c.to_string())).collect();
                    Ok(Value::List(chars))
                }
                Value::List(items) => Ok(Value::List(items.clone())),
                other => Err(EvalError::type_mismatch(
                    "string or list",
                    other.to_string(source),
                    line,
                )),
            }
        }
        _ => Err(EvalError::new(
            format!("unknown type function: {}", name),
            None,
            None,
            line,
        )),
    }
}
