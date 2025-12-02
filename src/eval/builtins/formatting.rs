//! Formatting functions: center, format_*, truncate

/// Names of formatting builtins
pub const NAMES: &[&str] = &[
    "center",
    "format_binary",
    "format_bool",
    "format_bytes",
    "format_currency",
    "format_float",
    "format_hex",
    "format_int",
    "format_json",
    "format_list",
    "format_octal",
    "format_percent",
    "format_scientific",
    "format_table",
    "truncate",
];

/// Get arity for formatting functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "format_binary" | "format_bytes" | "format_hex" | "format_json" | "format_octal" => {
            Some(1)
        }
        "center" | "format_bool" | "format_currency" | "format_float" | "format_int"
        | "format_list" | "format_percent" | "format_scientific" | "format_table" | "truncate" => {
            Some(2)
        }
        _ => None,
    }
}

/// Check if name is a formatting builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
