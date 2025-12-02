//! Math functions: neg (and future math functions)

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
