//! Aggregate functions: sum, min, max, all, any, count

/// Names of aggregate builtins
pub const NAMES: &[&str] = &["all", "any", "count", "max", "min", "sum"];

/// Get arity for aggregate functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "max" | "min" | "sum" => Some(1),
        "all" | "any" | "count" => Some(2),
        _ => None,
    }
}

/// Check if name is an aggregate builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
