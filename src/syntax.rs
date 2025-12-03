//! Syntax highlighting for the Avon REPL
//!
//! This module provides syntax highlighting for the interactive REPL,
//! using the same highlighting rules as the VS Code extension.

use std::borrow::Cow;

/// ANSI color codes for terminal highlighting
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const KEYWORD: &str = "\x1b[35m"; // Magenta for keywords (let, in, if, then, else)
    pub const BUILTIN: &str = "\x1b[94m"; // Light blue for builtin functions
    pub const STRING: &str = "\x1b[32m"; // Green for strings
    pub const NUMBER: &str = "\x1b[33m"; // Yellow for numbers
    pub const BOOLEAN: &str = "\x1b[33m"; // Yellow for booleans
    pub const COMMENT: &str = "\x1b[90m"; // Gray for comments
    pub const OPERATOR: &str = "\x1b[91m"; // Light red for operators
    pub const PATH: &str = "\x1b[36m"; // Cyan for path literals
    pub const TEMPLATE: &str = "\x1b[32m"; // Green for templates
    pub const INTERPOLATION: &str = "\x1b[93m"; // Bright yellow for interpolation braces
}

/// List of Avon keywords
const KEYWORDS: &[&str] = &["let", "in", "if", "then", "else", "fn", "match"];

/// List of Avon builtin functions (matching VS Code extension)
const BUILTINS: &[&str] = &[
    "abs",
    "abspath",
    "all",
    "any",
    "assert",
    "basename",
    "ceil",
    "center",
    "char_at",
    "chars",
    "chunks",
    "combinations",
    "concat",
    "contains",
    "count",
    "csv_parse",
    "date_add",
    "date_diff",
    "date_format",
    "date_parse",
    "debug",
    "dict_merge",
    "dirname",
    "drop",
    "ends_with",
    "enumerate",
    "env_var",
    "env_var_or",
    "error",
    "exists",
    "fill_template",
    "filter",
    "flatmap",
    "flatten",
    "floor",
    "fold",
    "format_binary",
    "format_bool",
    "format_bytes",
    "format_csv",
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
    "gcd",
    "get",
    "glob",
    "has_key",
    "head",
    "html_attr",
    "html_escape",
    "html_tag",
    "import",
    "indent",
    "is_alpha",
    "is_alphanumeric",
    "is_bool",
    "is_dict",
    "is_digit",
    "is_empty",
    "is_float",
    "is_function",
    "is_int",
    "is_list",
    "is_lowercase",
    "is_none",
    "is_number",
    "is_string",
    "is_uppercase",
    "is_whitespace",
    "join",
    "json_parse",
    "keys",
    "last",
    "lcm",
    "length",
    "lines",
    "log",
    "log10",
    "lower",
    "map",
    "markdown_to_html",
    "max",
    "md_code",
    "md_heading",
    "md_link",
    "md_list",
    "min",
    "neg",
    "not",
    "now",
    "nth",
    "os",
    "pad_left",
    "pad_right",
    "partition",
    "permutations",
    "pow",
    "product",
    "range",
    "readfile",
    "readlines",
    "regex_match",
    "regex_replace",
    "regex_split",
    "relpath",
    "repeat",
    "replace",
    "reverse",
    "round",
    "scan",
    "set",
    "slice",
    "sort",
    "sort_by",
    "split",
    "split_at",
    "spy",
    "sqrt",
    "starts_with",
    "sum",
    "tail",
    "take",
    "tap",
    "timestamp",
    "timezone",
    "to_bool",
    "to_char",
    "to_float",
    "to_int",
    "to_list",
    "to_string",
    "toml_parse",
    "trace",
    "transpose",
    "trim",
    "truncate",
    "typeof",
    "unique",
    "unlines",
    "unwords",
    "unzip",
    "upper",
    "values",
    "walkdir",
    "windows",
    "words",
    "yaml_parse",
    "zip",
];

/// Syntax highlighter for Avon code
pub struct AvonHighlighter {
    enabled: bool,
}

impl Default for AvonHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl AvonHighlighter {
    pub fn new() -> Self {
        // Check if terminal supports colors
        let enabled = std::env::var("NO_COLOR").is_err()
            && std::env::var("TERM").map(|t| t != "dumb").unwrap_or(true);
        Self { enabled }
    }

    pub fn with_enabled(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Highlight an interpolation expression inside a string/template
    fn highlight_interpolation(&self, expr: &str) -> String {
        let mut result = String::new();
        result.push_str(colors::INTERPOLATION);
        result.push('{');
        result.push_str(colors::RESET);

        // Highlight the expression inside the braces
        let inner = &expr[1..expr.len() - 1]; // Remove { and }
        let highlighted_inner = self.highlight_expression(inner);
        result.push_str(&highlighted_inner);

        result.push_str(colors::INTERPOLATION);
        result.push('}');
        result.push_str(colors::RESET);
        result
    }

    /// Highlight an expression (used for interpolations)
    fn highlight_expression(&self, expr: &str) -> String {
        let chars: Vec<char> = expr.chars().collect();
        let mut result = String::new();
        let mut i = 0;

        while i < chars.len() {
            // Skip whitespace
            if chars[i].is_whitespace() {
                result.push(chars[i]);
                i += 1;
                continue;
            }

            // Check for identifier (keywords, builtins, variables)
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                if KEYWORDS.contains(&word.as_str()) {
                    result.push_str(colors::KEYWORD);
                    result.push_str(&word);
                    result.push_str(colors::RESET);
                } else if word == "true" || word == "false" || word == "none" {
                    result.push_str(colors::BOOLEAN);
                    result.push_str(&word);
                    result.push_str(colors::RESET);
                } else if BUILTINS.contains(&word.as_str()) {
                    result.push_str(colors::BUILTIN);
                    result.push_str(&word);
                    result.push_str(colors::RESET);
                } else {
                    // Variable reference - use default color
                    result.push_str(&word);
                }
                continue;
            }

            // Check for number
            if chars[i].is_ascii_digit() {
                result.push_str(colors::NUMBER);
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    result.push(chars[i]);
                    i += 1;
                }
                result.push_str(colors::RESET);
                continue;
            }

            // Check for operators
            if matches!(
                chars[i],
                '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '.'
            ) {
                result.push_str(colors::OPERATOR);
                result.push(chars[i]);
                i += 1;
                result.push_str(colors::RESET);
                continue;
            }

            // Default: just copy the character
            result.push(chars[i]);
            i += 1;
        }

        result
    }

    /// Highlight string content, handling interpolations
    fn highlight_string_content(&self, content: &str, is_template: bool) -> String {
        let mut result = String::new();
        let chars: Vec<char> = content.chars().collect();
        let mut i = 0;
        let string_color = if is_template {
            colors::TEMPLATE
        } else {
            colors::STRING
        };

        while i < chars.len() {
            // Only check for interpolation in templates, not regular strings
            if is_template && chars[i] == '{' {
                // Find matching closing brace, handling nested braces
                let start = i;
                let mut depth = 1;
                i += 1;
                while i < chars.len() && depth > 0 {
                    if chars[i] == '{' {
                        depth += 1;
                    } else if chars[i] == '}' {
                        depth -= 1;
                    }
                    i += 1;
                }

                // Extract and highlight the interpolation
                let interpolation: String = chars[start..i].iter().collect();
                result.push_str(&self.highlight_interpolation(&interpolation));

                // Re-apply string color for remaining content
                result.push_str(string_color);
                continue;
            }

            // Check for escape sequences (in both strings and templates)
            if chars[i] == '\\' && i + 1 < chars.len() {
                result.push(chars[i]);
                i += 1;
                result.push(chars[i]);
                i += 1;
                continue;
            }

            // Regular character
            result.push(chars[i]);
            i += 1;
        }

        result
    }

    /// Highlight a line of Avon code
    pub fn highlight<'l>(&self, line: &'l str) -> Cow<'l, str> {
        if !self.enabled || line.is_empty() {
            return Cow::Borrowed(line);
        }

        // Handle REPL commands specially
        if line.trim_start().starts_with(':') {
            return Cow::Owned(format!("{}{}{}", colors::KEYWORD, line, colors::RESET));
        }

        let mut result = String::with_capacity(line.len() * 2);
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Check for line comment (#)
            if chars[i] == '#' {
                result.push_str(colors::COMMENT);
                while i < chars.len() {
                    result.push(chars[i]);
                    if chars[i] == '\n' {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                result.push_str(colors::RESET);
                continue;
            }

            // Check for line comment (//)
            if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                result.push_str(colors::COMMENT);
                while i < chars.len() {
                    result.push(chars[i]);
                    if chars[i] == '\n' {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                result.push_str(colors::RESET);
                continue;
            }

            // Check for template string with variable braces: {..."...}
            if chars[i] == '{' {
                let mut brace_count = 0;
                let mut j = i;
                while j < chars.len() && chars[j] == '{' {
                    brace_count += 1;
                    j += 1;
                }

                if brace_count > 0 && j < chars.len() && chars[j] == '"' {
                    // Found start of template: {..."
                    result.push_str(colors::TEMPLATE);
                    for _ in 0..brace_count {
                        result.push('{');
                    }
                    result.push('"');
                    i = j + 1;

                    // Collect template content until closing "...}
                    let mut content = String::new();
                    while i < chars.len() {
                        // Check for closing sequence: "...}
                        if chars[i] == '"' {
                            let mut k = i + 1;
                            let mut closing_braces = 0;
                            while k < chars.len() && chars[k] == '}' {
                                closing_braces += 1;
                                k += 1;
                            }

                            if closing_braces == brace_count {
                                // Found matching closing sequence
                                break;
                            }
                        }

                        // Handle escape sequences
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            content.push(chars[i]);
                            i += 1;
                            content.push(chars[i]);
                            i += 1;
                            continue;
                        }
                        content.push(chars[i]);
                        i += 1;
                    }

                    // Highlight the content with interpolations
                    result.push_str(&self.highlight_string_content(&content, true));

                    // Close the template
                    result.push_str(colors::TEMPLATE);
                    if i < chars.len() {
                        result.push('"');
                        for _ in 0..brace_count {
                            result.push('}');
                        }
                        i += 1 + brace_count;
                    }
                    result.push_str(colors::RESET);
                    continue;
                }
            }

            // Check for regular string
            if chars[i] == '"' {
                result.push_str(colors::STRING);
                result.push(chars[i]);
                i += 1;

                // Collect string content
                let mut content = String::new();
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        content.push(chars[i]);
                        i += 1;
                        content.push(chars[i]);
                        i += 1;
                        continue;
                    }
                    content.push(chars[i]);
                    i += 1;
                }

                // Highlight the content with interpolations
                result.push_str(&self.highlight_string_content(&content, false));

                // Close the string
                result.push_str(colors::STRING);
                if i < chars.len() {
                    result.push(chars[i]); // closing "
                    i += 1;
                }
                result.push_str(colors::RESET);
                continue;
            }

            // Check for path literal (@path)
            if chars[i] == '@' {
                result.push_str(colors::PATH);
                result.push(chars[i]);
                i += 1;
                // Read path characters, handling interpolations
                while i < chars.len()
                    && !chars[i].is_whitespace()
                    && chars[i] != '{'
                    && chars[i] != '}'
                    && chars[i] != ')'
                    && chars[i] != ']'
                {
                    result.push(chars[i]);
                    i += 1;
                }
                result.push_str(colors::RESET);
                continue;
            }

            // Check for number
            if chars[i].is_ascii_digit()
                || (chars[i] == '-'
                    && i + 1 < chars.len()
                    && chars[i + 1].is_ascii_digit()
                    && (i == 0 || !chars[i - 1].is_alphanumeric()))
            {
                result.push_str(colors::NUMBER);
                if chars[i] == '-' {
                    result.push(chars[i]);
                    i += 1;
                }
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    result.push(chars[i]);
                    i += 1;
                }
                result.push_str(colors::RESET);
                continue;
            }

            // Check for identifier (keywords, builtins, variables)
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                if KEYWORDS.contains(&word.as_str()) {
                    result.push_str(colors::KEYWORD);
                    result.push_str(&word);
                    result.push_str(colors::RESET);
                } else if word == "true" || word == "false" || word == "none" {
                    result.push_str(colors::BOOLEAN);
                    result.push_str(&word);
                    result.push_str(colors::RESET);
                } else if BUILTINS.contains(&word.as_str()) {
                    result.push_str(colors::BUILTIN);
                    result.push_str(&word);
                    result.push_str(colors::RESET);
                } else {
                    result.push_str(&word);
                }
                continue;
            }

            // Check for pipe operator
            if chars[i] == '-' && i + 1 < chars.len() && chars[i + 1] == '>' {
                result.push_str(colors::OPERATOR);
                result.push_str("->");
                result.push_str(colors::RESET);
                i += 2;
                continue;
            }

            // Check for operators
            if matches!(
                chars[i],
                '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|'
            ) {
                result.push_str(colors::OPERATOR);
                result.push(chars[i]);
                // Handle multi-character operators
                if i + 1 < chars.len() {
                    let next = chars[i + 1];
                    let is_two_char_op = matches!(
                        (chars[i], next),
                        ('=', '=') | ('!', '=') | ('<', '=') | ('>', '=') | ('&', '&') | ('|', '|')
                    );
                    if is_two_char_op {
                        i += 1;
                        result.push(chars[i]);
                    }
                }
                result.push_str(colors::RESET);
                i += 1;
                continue;
            }

            // Check for range operator
            if chars[i] == '.' && i + 1 < chars.len() && chars[i + 1] == '.' {
                result.push_str(colors::OPERATOR);
                result.push_str("..");
                result.push_str(colors::RESET);
                i += 2;
                continue;
            }

            // Check for lambda backslash
            if chars[i] == '\\' {
                result.push_str(colors::KEYWORD);
                result.push(chars[i]);
                result.push_str(colors::RESET);
                i += 1;
                continue;
            }

            // Default: just copy the character
            result.push(chars[i]);
            i += 1;
        }

        Cow::Owned(result)
    }

    /// Highlight a prompt (for rustyline)
    pub fn highlight_prompt<'p>(&self, prompt: &'p str, _default: bool) -> Cow<'p, str> {
        if !self.enabled {
            return Cow::Borrowed(prompt);
        }
        // Make prompt green and bold
        Cow::Owned(format!("\x1b[1;32m{}\x1b[0m", prompt))
    }

    /// Check if a character needs highlighting (for rustyline optimization)
    pub fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        // We always want to re-highlight on changes
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords_highlighted() {
        let highlighter = AvonHighlighter::with_enabled(true);
        let result = highlighter.highlight("let x = 1 in x");
        assert!(result.contains(colors::KEYWORD));
        assert!(result.contains("let"));
        assert!(result.contains("in"));
    }

    #[test]
    fn test_numbers_highlighted() {
        let highlighter = AvonHighlighter::with_enabled(true);
        let result = highlighter.highlight("42");
        assert!(result.contains(colors::NUMBER));
    }

    #[test]
    fn test_strings_highlighted() {
        let highlighter = AvonHighlighter::with_enabled(true);
        let result = highlighter.highlight("\"hello\"");
        assert!(result.contains(colors::STRING));
    }

    #[test]
    fn test_string_interpolation() {
        let highlighter = AvonHighlighter::with_enabled(true);
        let result = highlighter.highlight("\"hello {name}\"");
        // Regular strings should NOT have interpolation - they're just strings with literal braces
        assert!(result.contains(colors::STRING));
        // Should NOT contain interpolation color since strings don't support interpolation
        assert!(!result.contains(colors::INTERPOLATION));
    }

    #[test]
    fn test_template_interpolation() {
        let highlighter = AvonHighlighter::with_enabled(true);
        let result = highlighter.highlight("{\"hello {name}\"}");
        // Should contain template color and interpolation color
        assert!(result.contains(colors::TEMPLATE));
        assert!(result.contains(colors::INTERPOLATION));
    }

    #[test]
    fn test_comments_highlighted() {
        let highlighter = AvonHighlighter::with_enabled(true);
        let result = highlighter.highlight("# comment");
        assert!(result.contains(colors::COMMENT));
    }

    #[test]
    fn test_builtins_highlighted() {
        let highlighter = AvonHighlighter::with_enabled(true);
        let result = highlighter.highlight("map filter fold");
        assert!(result.contains(colors::BUILTIN));
    }

    #[test]
    fn test_disabled_returns_original() {
        let highlighter = AvonHighlighter::with_enabled(false);
        let input = "let x = 1";
        let result = highlighter.highlight(input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_pipe_operator() {
        let highlighter = AvonHighlighter::with_enabled(true);
        let result = highlighter.highlight("[1,2,3] -> map");
        assert!(result.contains(colors::OPERATOR));
        assert!(result.contains("->"));
    }

    #[test]
    fn test_path_literal() {
        let highlighter = AvonHighlighter::with_enabled(true);
        let result = highlighter.highlight("@config.yml");
        assert!(result.contains(colors::PATH));
    }
}
