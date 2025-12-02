//! Debug/Assertion functions: assert, debug, error, not, trace

use crate::common::{EvalError, Value};

/// Names of debug builtins
pub const NAMES: &[&str] = &["assert", "debug", "error", "not", "trace"];

/// Get arity for debug functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "debug" | "error" | "not" => Some(1),
        "assert" | "trace" => Some(2),
        _ => None,
    }
}

/// Check if name is a debug builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a debug/assertion builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "not" => {
            // not :: Bool -> Bool
            // Logical negation - returns true if false, false if true
            match &args[0] {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                other => Err(EvalError::type_mismatch(
                    "bool",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "assert" => {
            // assert :: Bool -> a -> a
            // Returns the second argument if first is true, throws error if false
            match &args[0] {
                Value::Bool(true) => Ok(args[1].clone()),
                Value::Bool(false) => Err(EvalError::new(
                    format!("assertion failed: {}", args[1].to_string(source)),
                    None,
                    None,
                    line,
                )),
                other => Err(EvalError::type_mismatch(
                    "bool",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "error" => {
            // error :: String -> a
            // Throws an error with the given message
            match &args[0] {
                Value::String(msg) => Err(EvalError::new(msg.clone(), None, None, line)),
                Value::Template(chunks, symbols) => {
                    let msg = crate::eval::render_chunks_to_string(chunks, symbols, source)
                        .unwrap_or_else(|_| "<template error>".to_string());
                    Err(EvalError::new(msg, None, None, line))
                }
                other => Err(EvalError::type_mismatch(
                    "string",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "trace" => {
            // trace :: String -> a -> a
            // Prints label and value to stderr, returns the value
            let label = &args[0];
            let val = &args[1];
            match label {
                Value::String(lbl) => {
                    eprintln!("[TRACE] {}: {}", lbl, val.to_string(source));
                    Ok(val.clone())
                }
                other => Err(EvalError::type_mismatch(
                    "string",
                    other.to_string(source),
                    line,
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
        _ => Err(EvalError::new(
            format!("unknown debug function: {}", name),
            None,
            None,
            line,
        )),
    }
}
