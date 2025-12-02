//! Debug/Assertion functions: assert, debug, error, not, trace

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
