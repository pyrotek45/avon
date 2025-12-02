//! File I/O functions: basename, dirname, exists, fill_template, import, json_parse, readfile, readlines, walkdir

/// Names of file I/O builtins
pub const NAMES: &[&str] = &[
    "basename",
    "dirname",
    "exists",
    "fill_template",
    "import",
    "json_parse",
    "readfile",
    "readlines",
    "walkdir",
];

/// Get arity for file I/O functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "basename" | "dirname" | "exists" | "import" | "json_parse" | "readfile" | "readlines"
        | "walkdir" => Some(1),
        "fill_template" => Some(2),
        _ => None,
    }
}

/// Check if name is a file I/O builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
