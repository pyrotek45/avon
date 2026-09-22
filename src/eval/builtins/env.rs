//! Environment functions: env_var, env_var_or, env_vars, os, whoami, hostname, random_range

use crate::common::{EvalError, Number, Value};
use std::collections::HashMap;

/// Names of environment builtins
pub const NAMES: &[&str] = &[
    "env_var",
    "env_var_or",
    "env_vars",
    "hostname",
    "os",
    "random_range",
    "whoami",
];

/// Get arity for environment functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "env_var" => Some(1),
        "env_var_or" => Some(2),
        "env_vars" | "hostname" | "os" | "whoami" => Some(0),
        "random_range" => Some(2),
        // "os" is a constant, not a function - no arity
        _ => None,
    }
}

/// Check if name is an environment builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute an environment builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
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
        "env_vars" => {
            // env_vars :: () -> Dict
            // Returns a dictionary of all environment variables.
            let mut map = HashMap::new();
            for (key, val) in std::env::vars() {
                map.insert(key, Value::String(val));
            }
            Ok(Value::Dict(map))
        }
        "whoami" => {
            // whoami :: () -> String
            // Returns the current username.
            match std::env::var("USER") {
                Ok(user) => Ok(Value::String(user)),
                Err(_) => {
                    // Fallback to LOGNAME or SUDO_USER on some systems
                    match std::env::var("LOGNAME") {
                        Ok(user) => Ok(Value::String(user)),
                        Err(_) => Err(EvalError::new(
                            "whoami: could not determine current user".to_string(),
                            None,
                            None,
                            line,
                        )),
                    }
                }
            }
        }
        "hostname" => {
            // hostname :: () -> String
            // Returns the machine hostname from the HOSTNAME environment variable.
            match std::env::var("HOSTNAME") {
                Ok(host) => Ok(Value::String(host)),
                Err(_) => {
                    // Some systems may have it in different vars, try others
                    match std::env::var("COMPUTERNAME") {
                        Ok(host) => Ok(Value::String(host)),
                        Err(_) => Err(EvalError::new(
                            "hostname: HOSTNAME environment variable not set".to_string(),
                            None,
                            None,
                            line,
                        )),
                    }
                }
            }
        }
        "random_range" => {
            // random_range :: Int -> Int -> Int
            // Returns a random integer between min and max (inclusive).
            let minv = &args[0];
            let maxv = &args[1];

            let min = match minv {
                Value::Number(Number::Int(n)) => *n,
                Value::Number(Number::Float(f)) => *f as i64,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        minv.to_string(source),
                        line,
                    ))
                }
            };

            let max = match maxv {
                Value::Number(Number::Int(n)) => *n,
                Value::Number(Number::Float(f)) => *f as i64,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        maxv.to_string(source),
                        line,
                    ))
                }
            };

            if min > max {
                return Err(EvalError::new(
                    "random_range: min must be <= max".to_string(),
                    None,
                    None,
                    line,
                ));
            }

            let range = (max - min + 1) as u64;
            let rand_val = rand::random::<u64>() % range;
            Ok(Value::Number(Number::Int(min + rand_val as i64)))
        }
        _ => Err(EvalError::new(
            format!("unknown env function: {}", name),
            None,
            None,
            line,
        )),
    }
}
