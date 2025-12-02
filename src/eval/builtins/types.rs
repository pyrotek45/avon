//! Type checking and conversion functions

/// Names of type checking builtins
#[allow(dead_code)]
pub const TYPE_CHECK_NAMES: &[&str] = &[
    "is_bool",
    "is_dict",
    "is_float",
    "is_function",
    "is_int",
    "is_list",
    "is_none",
    "is_number",
    "is_string",
    "typeof",
];

/// Names of type conversion builtins
#[allow(dead_code)]
pub const TYPE_CONVERT_NAMES: &[&str] = &[
    "to_bool",
    "to_char",
    "to_float",
    "to_int",
    "to_list",
    "to_string",
];

/// All type-related builtin names
pub const NAMES: &[&str] = &[
    // Type checking
    "is_bool",
    "is_dict",
    "is_float",
    "is_function",
    "is_int",
    "is_list",
    "is_none",
    "is_number",
    "is_string",
    "typeof",
    // Type conversion
    "to_bool",
    "to_char",
    "to_float",
    "to_int",
    "to_list",
    "to_string",
];

/// Get arity for type functions (all are arity 1)
pub fn get_arity(name: &str) -> Option<usize> {
    if NAMES.contains(&name) {
        Some(1)
    } else {
        None
    }
}

/// Check if name is a type builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
