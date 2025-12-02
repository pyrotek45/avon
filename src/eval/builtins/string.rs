//! String operations: char_at, chars, concat, contains, ends_with, indent, is_alpha, is_alphanumeric, is_digit, is_empty, is_lowercase, is_uppercase, is_whitespace, join, length, lower, pad_left, pad_right, repeat, replace, slice, split, starts_with, trim, upper

/// Names of string builtins
pub const NAMES: &[&str] = &[
    "char_at",
    "chars",
    "concat",
    "contains",
    "ends_with",
    "indent",
    "is_alpha",
    "is_alphanumeric",
    "is_digit",
    "is_empty",
    "is_lowercase",
    "is_uppercase",
    "is_whitespace",
    "join",
    "length",
    "lower",
    "pad_left",
    "pad_right",
    "repeat",
    "replace",
    "slice",
    "split",
    "starts_with",
    "trim",
    "upper",
];

/// Get arity for string functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "chars" | "is_alpha" | "is_alphanumeric" | "is_digit" | "is_empty" | "is_lowercase"
        | "is_uppercase" | "is_whitespace" | "length" | "lower" | "trim" | "upper" => Some(1),
        "char_at" | "concat" | "contains" | "ends_with" | "indent" | "join" | "repeat"
        | "split" | "starts_with" => Some(2),
        "pad_left" | "pad_right" | "replace" | "slice" => Some(3),
        _ => None,
    }
}

/// Check if name is a string builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
