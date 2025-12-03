//! Math functions: abs, gcd, lcm, neg, sqrt, pow, floor, ceil, round, log, log10

use crate::common::{EvalError, Number, Value};

/// Names of math builtins
pub const NAMES: &[&str] = &["abs", "ceil", "floor", "gcd", "lcm", "log", "log10", "neg", "pow", "round", "sqrt"];

/// Get arity for math functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "abs" | "ceil" | "floor" | "log" | "log10" | "neg" | "round" | "sqrt" => Some(1),
        "gcd" | "lcm" | "pow" => Some(2),
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
        "sqrt" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => {
                    let result = (*i as f64).sqrt();
                    Ok(Value::Number(Number::Float(result)))
                }
                Value::Number(Number::Float(f)) => {
                    let result = f.sqrt();
                    Ok(Value::Number(Number::Float(result)))
                }
                other => Err(EvalError::type_mismatch(
                    "number",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "pow" => {
            let base = &args[0];
            let exp = &args[1];
            match (base, exp) {
                (Value::Number(Number::Int(b)), Value::Number(Number::Int(e))) => {
                    if *e >= 0 {
                        Ok(Value::Number(Number::Int(b.pow(*e as u32))))
                    } else {
                        // Negative exponent requires float result
                        Ok(Value::Number(Number::Float((*b as f64).powi(*e as i32))))
                    }
                }
                (Value::Number(Number::Int(b)), Value::Number(Number::Float(e))) => {
                    Ok(Value::Number(Number::Float((*b as f64).powf(*e))))
                }
                (Value::Number(Number::Float(b)), Value::Number(Number::Int(e))) => {
                    Ok(Value::Number(Number::Float(b.powi(*e as i32))))
                }
                (Value::Number(Number::Float(b)), Value::Number(Number::Float(e))) => {
                    Ok(Value::Number(Number::Float(b.powf(*e))))
                }
                _ => Err(EvalError::type_mismatch(
                    "two numbers",
                    format!("{}, {}", base.to_string(source), exp.to_string(source)),
                    line,
                )),
            }
        }
        "floor" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Int(*i))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Int(f.floor() as i64))),
                other => Err(EvalError::type_mismatch(
                    "number",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "ceil" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Int(*i))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Int(f.ceil() as i64))),
                other => Err(EvalError::type_mismatch(
                    "number",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "round" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Int(*i))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Int(f.round() as i64))),
                other => Err(EvalError::type_mismatch(
                    "number",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "log" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => {
                    let result = (*i as f64).ln();
                    Ok(Value::Number(Number::Float(result)))
                }
                Value::Number(Number::Float(f)) => {
                    let result = f.ln();
                    Ok(Value::Number(Number::Float(result)))
                }
                other => Err(EvalError::type_mismatch(
                    "number",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "log10" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => {
                    let result = (*i as f64).log10();
                    Ok(Value::Number(Number::Float(result)))
                }
                Value::Number(Number::Float(f)) => {
                    let result = f.log10();
                    Ok(Value::Number(Number::Float(result)))
                }
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
