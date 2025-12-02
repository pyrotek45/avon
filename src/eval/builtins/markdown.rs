//! Markdown functions: markdown_to_html, md_code, md_heading, md_link, md_list

/// Names of markdown builtins
pub const NAMES: &[&str] = &[
    "markdown_to_html",
    "md_code",
    "md_heading",
    "md_link",
    "md_list",
];

/// Get arity for markdown functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "markdown_to_html" | "md_code" | "md_list" => Some(1),
        "md_heading" | "md_link" => Some(2),
        _ => None,
    }
}

/// Check if name is a markdown builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
