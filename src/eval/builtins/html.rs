//! HTML functions: html_attr, html_escape, html_tag

/// Names of HTML builtins
pub const NAMES: &[&str] = &["html_attr", "html_escape", "html_tag"];

/// Get arity for HTML functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "html_escape" => Some(1),
        "html_attr" | "html_tag" => Some(2),
        _ => None,
    }
}

/// Check if name is an HTML builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
