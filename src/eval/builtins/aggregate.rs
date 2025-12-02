//! Aggregate functions: sum, product, min, max, all, any, count

use crate::common::{EvalError, Number, Value};
use crate::eval::apply_function;

/// Names of aggregate builtins
pub const NAMES: &[&str] = &["all", "any", "count", "max", "min", "product", "sum"];

/// Get arity for aggregate functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "max" | "min" | "product" | "sum" => Some(1),
        "all" | "any" | "count" => Some(2),
        _ => None,
    }
}

/// Check if name is an aggregate builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute an aggregate builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "sum" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut total = 0i64;
                let mut has_float = false;
                let mut float_total = 0.0;
                for item in items {
                    match item {
                        Value::Number(Number::Int(i)) => {
                            if has_float {
                                float_total += *i as f64;
                            } else {
                                total += i;
                            }
                        }
                        Value::Number(Number::Float(f)) => {
                            if !has_float {
                                has_float = true;
                                float_total = total as f64;
                            }
                            float_total += f;
                        }
                        _ => {
                            return Err(EvalError::type_mismatch(
                                "number",
                                item.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                if has_float {
                    Ok(Value::Number(Number::Float(float_total)))
                } else {
                    Ok(Value::Number(Number::Int(total)))
                }
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "product" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut total = 1i64;
                let mut has_float = false;
                let mut float_total = 1.0;
                for item in items {
                    match item {
                        Value::Number(Number::Int(i)) => {
                            if has_float {
                                float_total *= *i as f64;
                            } else {
                                total *= i;
                            }
                        }
                        Value::Number(Number::Float(f)) => {
                            if !has_float {
                                has_float = true;
                                float_total = total as f64;
                            }
                            float_total *= f;
                        }
                        _ => {
                            return Err(EvalError::type_mismatch(
                                "number",
                                item.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                if has_float {
                    Ok(Value::Number(Number::Float(float_total)))
                } else {
                    Ok(Value::Number(Number::Int(total)))
                }
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "min" => {
            let list = &args[0];
            if let Value::List(items) = list {
                if items.is_empty() {
                    return Ok(Value::None);
                }
                let mut min_val = items[0].clone();
                for item in &items[1..] {
                    match (&min_val, item) {
                        (Value::Number(Number::Int(a)), Value::Number(Number::Int(b))) => {
                            if b < a {
                                min_val = item.clone();
                            }
                        }
                        (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => {
                            if b < a {
                                min_val = item.clone();
                            }
                        }
                        (Value::Number(Number::Int(a)), Value::Number(Number::Float(b))) => {
                            if *b < (*a as f64) {
                                min_val = item.clone();
                            }
                        }
                        (Value::Number(Number::Float(a)), Value::Number(Number::Int(b))) => {
                            if (*b as f64) < *a {
                                min_val = item.clone();
                            }
                        }
                        (Value::String(a), Value::String(b)) => {
                            if b < a {
                                min_val = item.clone();
                            }
                        }
                        _ => {
                            return Err(EvalError::type_mismatch(
                                "comparable values",
                                item.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                Ok(min_val)
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "max" => {
            let list = &args[0];
            if let Value::List(items) = list {
                if items.is_empty() {
                    return Ok(Value::None);
                }
                let mut max_val = items[0].clone();
                for item in &items[1..] {
                    match (&max_val, item) {
                        (Value::Number(Number::Int(a)), Value::Number(Number::Int(b))) => {
                            if b > a {
                                max_val = item.clone();
                            }
                        }
                        (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => {
                            if b > a {
                                max_val = item.clone();
                            }
                        }
                        (Value::Number(Number::Int(a)), Value::Number(Number::Float(b))) => {
                            if *b > (*a as f64) {
                                max_val = item.clone();
                            }
                        }
                        (Value::Number(Number::Float(a)), Value::Number(Number::Int(b))) => {
                            if (*b as f64) > *a {
                                max_val = item.clone();
                            }
                        }
                        (Value::String(a), Value::String(b)) => {
                            if b > a {
                                max_val = item.clone();
                            }
                        }
                        _ => {
                            return Err(EvalError::type_mismatch(
                                "comparable values",
                                item.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                Ok(max_val)
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "all" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("all:") {
                                err.message = format!("all: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::Bool(false) => return Ok(Value::Bool(false)),
                        Value::Bool(true) => {}
                        other => {
                            return Err(EvalError::type_mismatch(
                                "bool",
                                other.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                Ok(Value::Bool(true))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "any" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("any:") {
                                err.message = format!("any: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::Bool(true) => return Ok(Value::Bool(true)),
                        Value::Bool(false) => {}
                        other => {
                            return Err(EvalError::type_mismatch(
                                "bool",
                                other.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                Ok(Value::Bool(false))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "count" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut count = 0i64;
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("count:") {
                                err.message = format!("count: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::Bool(true) => count += 1,
                        Value::Bool(false) => {}
                        other => {
                            return Err(EvalError::type_mismatch(
                                "bool",
                                other.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                Ok(Value::Number(Number::Int(count)))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        _ => Err(EvalError::new(
            format!("unknown aggregate function: {}", name),
            None,
            None,
            line,
        )),
    }
}
