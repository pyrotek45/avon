//! HTML functions: html_attr, html_escape, html_tag

use crate::common::{EvalError, Value};
use crate::eval::value_to_string_auto;

/// Names of HTML builtins
pub const NAMES: &[&str] = &["html_attr", "html_escape", "html_tag"];

/// Get arity for HTML functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "html_escape" => Some(1),
        "html_attr" | "html_tag" => Some(2),
        _ => None,
    }
}

/// Check if name is an HTML builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute an HTML builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "html_escape" => {
            let st = value_to_string_auto(&args[0], source, line)?;
            let escaped = st
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;");
            Ok(Value::String(escaped))
        }
        "html_tag" => {
            let t = value_to_string_auto(&args[0], source, line)?;
            let c = value_to_string_auto(&args[1], source, line)?;
            Ok(Value::String(format!("<{}>{}</{}>", t, c, t)))
        }
        "html_attr" => {
            let n = value_to_string_auto(&args[0], source, line)?;
            let v = value_to_string_auto(&args[1], source, line)?;
            let escaped = v
                .replace('&', "&amp;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;");
            Ok(Value::String(format!("{}=\"{}\"", n, escaped)))
        }
        _ => Err(EvalError::new(
            format!("unknown html function: {}", name),
            None,
            None,
            line,
        )),
    }
}
