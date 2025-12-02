//! Dictionary functions: dict_merge, get, has_key, keys, set, values

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
