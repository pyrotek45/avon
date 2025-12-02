//! Markdown functions: markdown_to_html, md_code, md_heading, md_link, md_list

use crate::common::{EvalError, Number, Value};
use crate::eval::value_to_string_auto;

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
pub fn execute(
    name: &str,
    args: &[Value],
    source: &str,
    line: usize,
) -> Result<Value, EvalError> {
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
            // Simple markdown to HTML converter
            let lines: Vec<&str> = md.lines().collect();
            let mut html_lines = Vec::new();
            for md_line in lines {
                let trimmed = md_line.trim();
                if trimmed.is_empty() {
                    html_lines.push("<br>".to_string());
                } else if let Some(text) = trimmed.strip_prefix("# ") {
                    html_lines.push(format!("<h1>{}</h1>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("## ") {
                    html_lines.push(format!("<h2>{}</h2>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("### ") {
                    html_lines.push(format!("<h3>{}</h3>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("#### ") {
                    html_lines.push(format!("<h4>{}</h4>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("##### ") {
                    html_lines.push(format!("<h5>{}</h5>", text.trim()));
                } else if let Some(text) = trimmed.strip_prefix("###### ") {
                    html_lines.push(format!("<h6>{}</h6>", text.trim()));
                } else {
                    // Process inline formatting: **bold**, *italic*, `code`
                    let mut result = String::new();
                    let parts: Vec<&str> = trimmed.split("**").collect();
                    for (i, part) in parts.iter().enumerate() {
                        if i % 2 == 1 {
                            result.push_str("<strong>");
                            result.push_str(part);
                            result.push_str("</strong>");
                        } else {
                            // Process italic and code within non-bold parts
                            let italic_parts: Vec<&str> = part.split('*').collect();
                            for (j, italic_part) in italic_parts.iter().enumerate() {
                                if j > 0 && j % 2 == 1 {
                                    result.push_str("<em>");
                                }
                                // Code: `text` -> <code>text</code>
                                let code_parts: Vec<&str> = italic_part.split('`').collect();
                                for (k, code_part) in code_parts.iter().enumerate() {
                                    if k > 0 && k % 2 == 1 {
                                        result.push_str("<code>");
                                    }
                                    result.push_str(code_part);
                                    if k > 0 && k % 2 == 1 {
                                        result.push_str("</code>");
                                    }
                                }
                                if j > 0 && j % 2 == 1 {
                                    result.push_str("</em>");
                                }
                            }
                        }
                    }
                    html_lines.push(format!("<p>{}</p>", result));
                }
            }
            Ok(Value::String(html_lines.join("\n")))
        }
        _ => Err(EvalError::new(
            format!("unknown markdown function: {}", name),
            None,
            None,
            line,
        )),
    }
}
