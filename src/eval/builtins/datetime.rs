//! Date/Time functions: date_add, date_diff, date_format, date_parse, now, timestamp, timezone

/// Names of datetime builtins
pub const NAMES: &[&str] = &[
    "date_add",
    "date_diff",
    "date_format",
    "date_parse",
    "now",
    "timestamp",
    "timezone",
];

/// Get arity for datetime functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "now" | "timestamp" | "timezone" => Some(1),
        "date_add" | "date_diff" | "date_format" | "date_parse" => Some(2),
        _ => None,
    }
}

/// Check if name is a datetime builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
