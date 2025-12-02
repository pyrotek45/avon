//! Math functions: neg (and future math functions)

use crate::common::{EvalError, Number, Value};

/// Names of math builtins
pub const NAMES: &[&str] = &["neg"];

/// Get arity for math functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "neg" => Some(1),
        _ => None,
    }
}

/// Check if name is a math builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a math builtin function
pub fn execute(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    match name {
        "neg" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Int(-i))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Float(-f))),
                other => Err(EvalError::type_mismatch(
                    "number",
                    other.to_string(source),
                    line,
                )),
            }
        }
        _ => Err(EvalError::new(
            format!("unknown math function: {}", name),
            None,
            None,
            line,
        )),
    }
}
