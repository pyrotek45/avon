// Helper functions for CLI

use avon::syntax::AvonHighlighter;

/// Check if a file path is an Avon file (has .av extension)
pub fn is_avon_file(path: &str) -> bool {
    path.ends_with(".av")
}

/// Print text, with syntax highlighting only if it's Avon code
pub fn print_file_content(path: &str, text: &str) {
    if is_avon_file(path) {
        let highlighter = AvonHighlighter::new();
        println!("{}", highlighter.highlight(text));
    } else {
        println!("{}", text);
    }
}

/// Helper function to check if an expression appears complete
#[cfg(test)]
pub fn is_expression_complete(input: &str) -> bool {
    is_expression_complete_impl(input)
}

/// Check if an expression for :let is complete
/// For :let, we need balanced brackets, complete if/then/else, and matched let/in pairs
pub fn is_let_expr_complete(input: &str) -> bool {
    let mut paren_depth = 0;
    let mut bracket_depth = 0;
    let mut brace_depth = 0;
    let mut in_string = false;
    let mut in_template = false;
    let mut string_escape_next = false;
    let mut if_count = 0;
    let mut then_count = 0;
    let mut else_count = 0;
    let mut let_count = 0;
    let mut in_count = 0;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Handle escape sequences only inside strings
        if in_string {
            if string_escape_next {
                string_escape_next = false;
                i += 1;
                continue;
            }
            if c == '\\' {
                string_escape_next = true;
                i += 1;
                continue;
            }
            if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if in_template {
            if c == '}' {
                let mut j = i;
                let mut brace_count = 1;
                while j > 0 && chars[j - 1] == '{' {
                    j -= 1;
                    brace_count += 1;
                }
                if brace_count >= 2 {
                    in_template = false;
                }
            }
            i += 1;
            continue;
        }

        match c {
            '"' => in_string = true,
            '{' => {
                if i + 2 < chars.len() && chars[i + 1] == '{' && chars[i + 2] == '"' {
                    in_template = true;
                    i += 3;
                    continue;
                } else {
                    brace_depth += 1;
                }
            }
            '}' => {
                if !in_template {
                    brace_depth -= 1;
                }
            }
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            _ => {}
        }

        // Check for keywords
        if !in_string && !in_template {
            let remaining: String = chars[i..].iter().collect();

            // Check for "let" keyword
            if i + 3 <= chars.len()
                && remaining.starts_with("let")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 3 >= chars.len() || !chars[i + 3].is_alphanumeric())
            {
                let_count += 1;
                i += 3;
                continue;
            }

            // Check for "in" keyword
            if i + 2 <= chars.len()
                && remaining.starts_with("in")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 2 >= chars.len() || !chars[i + 2].is_alphanumeric())
            {
                in_count += 1;
                i += 2;
                continue;
            }

            if i + 2 <= chars.len()
                && remaining.starts_with("if")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 2 >= chars.len() || !chars[i + 2].is_alphanumeric())
            {
                if_count += 1;
                i += 2;
                continue;
            }

            if i + 4 <= chars.len()
                && remaining.starts_with("then")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 4 >= chars.len() || !chars[i + 4].is_alphanumeric())
            {
                then_count += 1;
                i += 4;
                continue;
            }

            if i + 4 <= chars.len()
                && remaining.starts_with("else")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 4 >= chars.len() || !chars[i + 4].is_alphanumeric())
            {
                else_count += 1;
                i += 4;
                continue;
            }
        }

        i += 1;
    }

    // Check for incomplete lambda at end
    let trimmed = input.trim_end();
    let has_incomplete_lambda = ends_with_lambda_param(trimmed);
    
    // Check for incomplete pipeline (ends with ->)
    let ends_with_pipe = trimmed.ends_with("->");

    // For :let expressions, we need:
    // - Balanced brackets/parens/braces
    // - Every let has a matching in (in_count >= let_count)
    //   Note: in_count can be > let_count if "in" is used as a variable name
    // - Complete if/then/else
    // - Not in string or template
    // - No incomplete lambda
    // - Not ending with a pipe operator
    in_count >= let_count
        && if_count == then_count
        && if_count == else_count
        && paren_depth == 0
        && bracket_depth == 0
        && brace_depth == 0
        && !in_string
        && !in_template
        && !has_incomplete_lambda
        && !ends_with_pipe
}

pub fn is_expression_complete_impl(input: &str) -> bool {
    let mut let_count = 0;
    let mut in_count = 0;
    let mut if_count = 0;
    let mut then_count = 0;
    let mut else_count = 0;
    let mut paren_depth = 0;
    let mut bracket_depth = 0;
    let mut brace_depth = 0;
    let mut in_string = false;
    let mut in_template = false;
    let mut string_escape_next = false;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Handle escape sequences only inside strings
        if in_string {
            if string_escape_next {
                string_escape_next = false;
                i += 1;
                continue;
            }
            if c == '\\' {
                string_escape_next = true;
                i += 1;
                continue;
            }
            if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if in_template {
            if c == '}' {
                // Check if it's closing a template brace
                let mut j = i;
                let mut brace_count = 1;
                while j > 0 && chars[j - 1] == '{' {
                    j -= 1;
                    brace_count += 1;
                }
                if brace_count >= 2 {
                    in_template = false;
                }
            }
            i += 1;
            continue;
        }

        match c {
            '"' => in_string = true,
            '{' => {
                // Check if it's a template start {{"
                if i + 2 < chars.len() && chars[i + 1] == '{' && chars[i + 2] == '"' {
                    in_template = true;
                    i += 3;
                    continue;
                } else {
                    brace_depth += 1;
                }
            }
            '}' => {
                if !in_template {
                    brace_depth -= 1;
                }
            }
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            _ => {}
        }

        // Check for keywords (only when not in string/template)
        // Use <= to catch keywords at end of input
        if !in_string && !in_template {
            let remaining: String = chars[i..].iter().collect();

            // Check for "let" keyword (word boundary) - 3 chars
            if i + 3 <= chars.len()
                && remaining.starts_with("let")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 3 >= chars.len() || !chars[i + 3].is_alphanumeric())
            {
                let_count += 1;
                i += 3;
                continue;
            }

            // Check for "in" keyword - 2 chars
            if i + 2 <= chars.len()
                && remaining.starts_with("in")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 2 >= chars.len() || !chars[i + 2].is_alphanumeric())
            {
                in_count += 1;
                i += 2;
                continue;
            }

            // Check for "if" keyword - 2 chars
            if i + 2 <= chars.len()
                && remaining.starts_with("if")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 2 >= chars.len() || !chars[i + 2].is_alphanumeric())
            {
                if_count += 1;
                i += 2;
                continue;
            }

            // Check for "then" keyword - 4 chars
            if i + 4 <= chars.len()
                && remaining.starts_with("then")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 4 >= chars.len() || !chars[i + 4].is_alphanumeric())
            {
                then_count += 1;
                i += 4;
                continue;
            }

            // Check for "else" keyword - 4 chars
            if i + 4 <= chars.len()
                && remaining.starts_with("else")
                && (i == 0 || !chars[i - 1].is_alphanumeric())
                && (i + 4 >= chars.len() || !chars[i + 4].is_alphanumeric())
            {
                else_count += 1;
                i += 4;
                continue;
            }
        }

        i += 1;
    }

    // Check for incomplete lambda at end of input
    // Pattern: \identifier with no body following
    let trimmed = input.trim_end();
    let has_incomplete_lambda = ends_with_lambda_param(trimmed);

    // Expression is complete if:
    // - All let statements have matching in
    // - All if statements have matching then and else
    // - All brackets/parens/braces are balanced
    // - Not in the middle of a string or template
    // - No incomplete lambda at end
    let_count == in_count
        && if_count == then_count
        && if_count == else_count
        && paren_depth == 0
        && bracket_depth == 0
        && brace_depth == 0
        && !in_string
        && !in_template
        && !has_incomplete_lambda
}

/// Check if a string ends with an incomplete lambda parameter
/// e.g., "\x", "\foo", "\x \y" (waiting for body)
fn ends_with_lambda_param(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return false;
    }

    // Walk backwards to find if we end with \identifier pattern
    let mut i = chars.len() - 1;

    // Skip trailing whitespace (already trimmed, but just in case)
    while i > 0 && chars[i].is_whitespace() {
        i -= 1;
    }

    // Must end with an identifier character
    if !chars[i].is_alphanumeric() && chars[i] != '_' {
        return false;
    }

    // Walk back through the identifier
    while i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_') {
        i -= 1;
    }

    // The character before the identifier should be a backslash
    if i > 0 && chars[i - 1] == '\\' {
        // Check that this backslash is not inside a string
        // Simple heuristic: count unescaped quotes before this position
        let prefix: String = chars[..i - 1].iter().collect();
        let quote_count = prefix.chars().filter(|&c| c == '"').count();
        // If odd number of quotes, we're inside a string
        if quote_count % 2 == 1 {
            return false;
        }
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ends_with_lambda_param() {
        // Should detect incomplete lambdas
        assert!(ends_with_lambda_param(r"\x"));
        assert!(ends_with_lambda_param(r"\foo"));
        assert!(ends_with_lambda_param(r"\x \y"));
        assert!(ends_with_lambda_param(r"map \x"));
        assert!(ends_with_lambda_param(r"\my_var"));

        // Should NOT detect these as incomplete lambdas
        assert!(!ends_with_lambda_param(r"\x x + 1"));
        assert!(!ends_with_lambda_param(r"\x \y x + y"));
        assert!(!ends_with_lambda_param("42"));
        assert!(!ends_with_lambda_param(""));
        assert!(!ends_with_lambda_param("hello"));
        assert!(!ends_with_lambda_param(r#""\x""#)); // backslash inside string
    }

    #[test]
    fn test_is_expression_complete_lambda() {
        // Incomplete lambdas
        assert!(!is_expression_complete_impl(r"\x"));
        assert!(!is_expression_complete_impl(r"\x \y"));
        assert!(!is_expression_complete_impl(r"map \x"));

        // Complete lambdas
        assert!(is_expression_complete_impl(r"\x x + 1"));
        assert!(is_expression_complete_impl(r"\x \y x + y"));
        assert!(is_expression_complete_impl(r"map (\x x * 2) [1, 2, 3]"));
    }
}
