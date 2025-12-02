//! Regex functions: regex_match, regex_replace, regex_split, scan

use crate::common::{EvalError, Value};
use regex::Regex;

/// Names of regex builtins
pub const NAMES: &[&str] = &["regex_match", "regex_replace", "regex_split", "scan"];

/// Get arity for regex functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "regex_match" | "regex_split" | "scan" => Some(2),
        "regex_replace" => Some(3),
        _ => None,
    }
}

/// Check if name is a regex builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a regex builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "regex_match" => {
            let pattern_val = &args[0];
            let text_val = &args[1];
            let pattern = pattern_val.to_string(source);
            let text = text_val.to_string(source);

            let re = Regex::new(&pattern)
                .map_err(|e| EvalError::new(format!("invalid regex: {}", e), None, None, line))?;

            Ok(Value::Bool(re.is_match(&text)))
        }
        "regex_replace" => {
            let pattern_val = &args[0];
            let replacement_val = &args[1];
            let text_val = &args[2];

            let pattern = pattern_val.to_string(source);
            let replacement = replacement_val.to_string(source);
            let text = text_val.to_string(source);

            let re = Regex::new(&pattern)
                .map_err(|e| EvalError::new(format!("invalid regex: {}", e), None, None, line))?;

            let result = re.replace_all(&text, replacement.as_str());
            Ok(Value::String(result.to_string()))
        }
        "regex_split" => {
            let pattern_val = &args[0];
            let text_val = &args[1];
            let pattern = pattern_val.to_string(source);
            let text = text_val.to_string(source);

            let re = Regex::new(&pattern)
                .map_err(|e| EvalError::new(format!("invalid regex: {}", e), None, None, line))?;

            let parts: Vec<Value> = re
                .split(&text)
                .map(|s| Value::String(s.to_string()))
                .collect();
            Ok(Value::List(parts))
        }
        "scan" => {
            let pattern_val = &args[0];
            let text_val = &args[1];
            let pattern = pattern_val.to_string(source);
            let text = text_val.to_string(source);

            let re = Regex::new(&pattern)
                .map_err(|e| EvalError::new(format!("invalid regex: {}", e), None, None, line))?;

            let mut matches = Vec::new();
            for cap in re.captures_iter(&text) {
                // If there are capture groups, return them as a list
                // If not, return the whole match
                if cap.len() > 1 {
                    let mut groups = Vec::new();
                    for i in 1..cap.len() {
                        groups.push(Value::String(
                            cap.get(i).map_or("", |m| m.as_str()).to_string(),
                        ));
                    }
                    matches.push(Value::List(groups));
                } else {
                    matches.push(Value::String(
                        cap.get(0).map_or("", |m| m.as_str()).to_string(),
                    ));
                }
            }
            Ok(Value::List(matches))
        }
        _ => Err(EvalError::new(
            format!("unknown regex function: {}", name),
            None,
            None,
            line,
        )),
    }
}
