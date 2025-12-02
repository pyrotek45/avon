//! Dictionary functions: dict_merge, get, has_key, keys, set, values

use crate::common::{EvalError, Value};
use std::collections::HashMap;

/// Names of dictionary builtins
pub const NAMES: &[&str] = &["dict_merge", "get", "has_key", "keys", "set", "values"];

/// Get arity for dictionary functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "keys" | "values" => Some(1),
        "dict_merge" | "get" | "has_key" => Some(2),
        "set" => Some(3),
        _ => None,
    }
}

/// Check if name is a dictionary builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a dictionary builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "get" => {
            // get :: (Dict|[[String, a]]) -> String -> a
            let map = &args[0];
            let key = &args[1];
            if let Value::String(k) = key {
                match map {
                    Value::Dict(dict) => Ok(dict.get(k).cloned().unwrap_or(Value::None)),
                    Value::List(pairs) => {
                        for item in pairs {
                            if let Value::List(pair) = item {
                                if pair.len() >= 2 {
                                    if let Value::String(pair_key) = &pair[0] {
                                        if pair_key == k {
                                            return Ok(pair[1].clone());
                                        }
                                    }
                                }
                            }
                        }
                        Ok(Value::None)
                    }
                    other => Err(EvalError::type_mismatch(
                        "dict or list of pairs",
                        other.to_string(source),
                        line,
                    )),
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    key.to_string(source),
                    line,
                ))
            }
        }
        "set" => {
            // set :: (Dict|[[String, a]]) -> String -> a -> (Dict|[[String, a]])
            let map = &args[0];
            let key = &args[1];
            let value = &args[2];
            if let Value::String(k) = key {
                match map {
                    Value::Dict(dict) => {
                        let mut new_dict = dict.clone();
                        new_dict.insert(k.clone(), value.clone());
                        Ok(Value::Dict(new_dict))
                    }
                    Value::List(pairs) => {
                        let mut new_pairs = Vec::new();
                        let mut found = false;
                        for item in pairs {
                            if let Value::List(pair) = item {
                                if pair.len() >= 2 {
                                    if let Value::String(pair_key) = &pair[0] {
                                        if pair_key == k {
                                            new_pairs.push(Value::List(vec![
                                                Value::String(k.clone()),
                                                value.clone(),
                                            ]));
                                            found = true;
                                            continue;
                                        }
                                    }
                                }
                            }
                            new_pairs.push(item.clone());
                        }
                        if !found {
                            new_pairs
                                .push(Value::List(vec![Value::String(k.clone()), value.clone()]));
                        }
                        Ok(Value::List(new_pairs))
                    }
                    other => Err(EvalError::type_mismatch(
                        "dict or list of pairs",
                        other.to_string(source),
                        line,
                    )),
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    key.to_string(source),
                    line,
                ))
            }
        }
        "keys" => {
            // keys :: (Dict|[[String, a]]) -> [String]
            let map = &args[0];
            match map {
                Value::Dict(dict) => {
                    let keys: Vec<Value> = dict.keys().map(|k| Value::String(k.clone())).collect();
                    Ok(Value::List(keys))
                }
                Value::List(pairs) => {
                    let mut keys = Vec::new();
                    for item in pairs {
                        if let Value::List(pair) = item {
                            if !pair.is_empty() {
                                keys.push(pair[0].clone());
                            }
                        }
                    }
                    Ok(Value::List(keys))
                }
                other => Err(EvalError::type_mismatch(
                    "dict or list of pairs",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "values" => {
            // values :: (Dict|[[String, a]]) -> [a]
            let map = &args[0];
            match map {
                Value::Dict(dict) => {
                    let values: Vec<Value> = dict.values().cloned().collect();
                    Ok(Value::List(values))
                }
                Value::List(pairs) => {
                    let mut values = Vec::new();
                    for item in pairs {
                        if let Value::List(pair) = item {
                            if pair.len() >= 2 {
                                values.push(pair[1].clone());
                            }
                        }
                    }
                    Ok(Value::List(values))
                }
                other => Err(EvalError::type_mismatch(
                    "dict or list of pairs",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "has_key" => {
            // has_key :: (Dict|[[String, a]]) -> String -> Bool
            let map = &args[0];
            let key = &args[1];
            if let Value::String(k) = key {
                match map {
                    Value::Dict(dict) => Ok(Value::Bool(dict.contains_key(k))),
                    Value::List(pairs) => {
                        for item in pairs {
                            if let Value::List(pair) = item {
                                if !pair.is_empty() {
                                    if let Value::String(pair_key) = &pair[0] {
                                        if pair_key == k {
                                            return Ok(Value::Bool(true));
                                        }
                                    }
                                }
                            }
                        }
                        Ok(Value::Bool(false))
                    }
                    other => Err(EvalError::type_mismatch(
                        "dict or list of pairs",
                        other.to_string(source),
                        line,
                    )),
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    key.to_string(source),
                    line,
                ))
            }
        }
        "dict_merge" => {
            // dict_merge :: Dict -> Dict -> Dict
            match (&args[0], &args[1]) {
                (Value::Dict(d1), Value::Dict(d2)) => {
                    let mut merged: HashMap<String, Value> = d1.clone();
                    for (k, v) in d2 {
                        merged.insert(k.clone(), v.clone());
                    }
                    Ok(Value::Dict(merged))
                }
                (a, b) => Err(EvalError::type_mismatch(
                    "dict and dict",
                    format!("{} and {}", a.to_string(source), b.to_string(source)),
                    line,
                )),
            }
        }
        _ => Err(EvalError::new(
            format!("unknown dict function: {}", name),
            None,
            None,
            line,
        )),
    }
}
