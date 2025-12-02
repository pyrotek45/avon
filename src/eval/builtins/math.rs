//! Math functions: abs, gcd, lcm, neg

use crate::common::{EvalError, Number, Value};

/// Names of math builtins
pub const NAMES: &[&str] = &["abs", "gcd", "lcm", "neg"];

/// Get arity for math functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "abs" | "neg" => Some(1),
        "gcd" | "lcm" => Some(2),
        _ => None,
    }
}

/// Check if name is a math builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b).abs() / gcd(a, b)
    }
}

/// Execute a math builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "abs" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Int(i.abs()))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Float(f.abs()))),
                other => Err(EvalError::type_mismatch(
                    "number",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "gcd" => {
            let a_val = &args[0];
            let b_val = &args[1];
            match (a_val, b_val) {
                (Value::Number(Number::Int(a)), Value::Number(Number::Int(b))) => {
                    Ok(Value::Number(Number::Int(gcd(*a, *b))))
                }
                _ => Err(EvalError::type_mismatch(
                    "two integers",
                    format!("{}, {}", a_val.to_string(source), b_val.to_string(source)),
                    line,
                )),
            }
        }
        "lcm" => {
            let a_val = &args[0];
            let b_val = &args[1];
            match (a_val, b_val) {
                (Value::Number(Number::Int(a)), Value::Number(Number::Int(b))) => {
                    Ok(Value::Number(Number::Int(lcm(*a, *b))))
                }
                _ => Err(EvalError::type_mismatch(
                    "two integers",
                    format!("{}, {}", a_val.to_string(source), b_val.to_string(source)),
                    line,
                )),
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
