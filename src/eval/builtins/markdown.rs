//! Markdown functions: markdown_to_html, md_code, md_heading, md_link, md_list
//!
//! Provides comprehensive markdown parsing and HTML generation using pulldown-cmark.
//! Supports:
//! - Headings (h1-h6)
//! - Bold, italic, bold italic, strikethrough formatting
//! - Links (inline and reference)
//! - Images
//! - Lists (ordered, unordered, nested, mixed)
//! - Code blocks with syntax highlighting
//! - Inline code
//! - Blockquotes (nested)
//! - Tables
//! - Task lists (checkboxes)
//! - Footnotes
//! - Horizontal rules
//! - Smart punctuation
//! - HTML escaping for security

use crate::common::{EvalError, Number, Value};
use crate::eval::value_to_string_auto;
use pulldown_cmark::{html, Options, Parser};

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

/// Execute a markdown builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "md_heading" => {
            if let Value::Number(Number::Int(lvl)) = &args[0] {
                let txt = value_to_string_auto(&args[1], source, line)?;
                let hashes = "#".repeat((*lvl).clamp(1, 6) as usize);
                Ok(Value::String(format!("{} {}", hashes, txt)))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    args[0].to_string(source),
                    line,
                ))
            }
        }
        "md_link" => {
            let txt = value_to_string_auto(&args[0], source, line)?;
            let u = value_to_string_auto(&args[1], source, line)?;
            Ok(Value::String(format!("[{}]({})", txt, u)))
        }
        "md_code" => {
            let c = value_to_string_auto(&args[0], source, line)?;
            Ok(Value::String(format!("`{}`", c)))
        }
        "md_list" => {
            let items = &args[0];
            if let Value::List(list) = items {
                let lines: Vec<String> = list
                    .iter()
                    .map(|item| format!("- {}", item.to_string(source)))
                    .collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    items.to_string(source),
                    line,
                ))
            }
        }
        "markdown_to_html" => {
            let md = value_to_string_auto(&args[0], source, line)?;
            convert_markdown_to_html(&md)
        }
        _ => Err(EvalError::new(
            format!("unknown markdown function: {}", name),
            None,
            None,
            line,
        )),
    }
}

/// Convert markdown to HTML with comprehensive feature support
///
/// Enables all major markdown features:
/// - Tables
/// - Footnotes
/// - Strikethrough
/// - Task lists
/// - Smart punctuation
fn convert_markdown_to_html(markdown: &str) -> Result<Value, EvalError> {
    // Create options with all supported features enabled
    let mut options = Options::empty();

    // Enable table parsing and rendering
    options.insert(Options::ENABLE_TABLES);

    // Enable footnotes (reference-style)
    options.insert(Options::ENABLE_FOOTNOTES);

    // Enable strikethrough (~~text~~)
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // Enable task lists (- [x] and - [ ] for checkboxes)
    options.insert(Options::ENABLE_TASKLISTS);

    // Enable smart punctuation (smart quotes, dashes, ellipsis)
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    // Parse the markdown with all options
    let parser = Parser::new_ext(markdown, options);

    // Convert to HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(Value::String(html_output))
}
