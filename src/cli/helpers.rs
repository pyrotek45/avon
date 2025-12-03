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
            // Note: backslash outside strings is lambda syntax, not escape
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

    // Expression is complete if:
    // - All let statements have matching in
    // - All if statements have matching then and else
    // - All brackets/parens/braces are balanced
    // - Not in the middle of a string or template
    let_count == in_count
        && if_count == then_count
        && if_count == else_count
        && paren_depth == 0
        && bracket_depth == 0
        && brace_depth == 0
        && !in_string
        && !in_template
}
