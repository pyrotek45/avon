//! Environment functions: env_var, env_var_or, os

/// Names of environment builtins
pub const NAMES: &[&str] = &["env_var", "env_var_or", "os"];

/// Get arity for environment functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "env_var" => Some(1),
        "env_var_or" => Some(2),
        // "os" is a constant, not a function - no arity
        _ => None,
    }
}

/// Check if name is an environment builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
