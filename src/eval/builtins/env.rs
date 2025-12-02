//! Environment functions: env_var, env_var_or, os

use crate::common::{EvalError, Value};

/// Names of environment builtins
pub const NAMES: &[&str] = &["env_var", "env_var_or", "os"];

/// Get arity for environment functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "env_var" => Some(1),
        "env_var_or" => Some(2),
        // "os" is a constant, not a function - no arity
        _ => None,
    }
}

/// Check if name is an environment builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute an environment builtin function
pub fn execute(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
    match name {
        "env_var" => {
            // env_var :: String -> String
            // Returns the value of an environment variable.
            // Errors if the variable is not set (fail-fast behavior).
            let name = &args[0];
            if let Value::String(key) = name {
                match std::env::var(key) {
                    Ok(val) => Ok(Value::String(val)),
                    Err(_) => Err(EvalError::new(
                        format!(
                            "environment variable '{}' is not set. Use env_var_or for a default value.",
                            key
                        ),
                        None,
                        None,
                        line,
                    )),
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    name.to_string(source),
                    line,
                ))
            }
        }
        "env_var_or" => {
            // env_var_or :: String -> String -> String
            // Returns the value of an environment variable or a default value if not set.
            let name = &args[0];
            let default = &args[1];
            if let Value::String(key) = name {
                match std::env::var(key) {
                    Ok(val) => Ok(Value::String(val)),
                    Err(_) => match default {
                        Value::String(def) => Ok(Value::String(def.clone())),
                        other => Err(EvalError::type_mismatch(
                            "string",
                            other.to_string(source),
                            line,
                        )),
                    },
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    name.to_string(source),
                    line,
                ))
            }
        }
        _ => Err(EvalError::new(
            format!("unknown env function: {}", name),
            None,
            None,
            line,
        )),
    }
}
