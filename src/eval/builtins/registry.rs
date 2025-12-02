//! Builtin function registry - tracks which functions exist and their arities
//!
//! This module provides:
//! - `is_builtin_name()` - Check if a name is a builtin function
//! - `initial_builtins()` - Get initial symbol table with all builtins
//! - `get_builtin_arity()` - Get the number of arguments for a builtin

use crate::common::Value;
use std::collections::HashMap;

/// Check if a name is a builtin function name
/// This list must be kept in sync with execute_builtin() in the dispatcher
pub fn is_builtin_name(name: &str) -> bool {
    matches!(
        name,
        // Aggregate functions
        "all"
            | "any"
            | "count"
            | "max"
            | "min"
            | "sum"
            // Assertion/Debug
            | "assert"
            | "debug"
            | "error"
            | "not"
            | "trace"
            // Date/Time
            | "date_add"
            | "date_diff"
            | "date_format"
            | "date_parse"
            | "now"
            | "timestamp"
            | "timezone"
            // Dictionary
            | "dict_merge"
            | "get"
            | "has_key"
            | "keys"
            | "set"
            | "values"
            // Environment
            | "env_var"
            | "env_var_or"
            | "os"
            // File I/O
            | "basename"
            | "dirname"
            | "exists"
            | "fill_template"
            | "import"
            | "json_parse"
            | "readfile"
            | "readlines"
            | "walkdir"
            // Formatting
            | "center"
            | "format_binary"
            | "format_bool"
            | "format_bytes"
            | "format_currency"
            | "format_float"
            | "format_hex"
            | "format_int"
            | "format_json"
            | "format_list"
            | "format_octal"
            | "format_percent"
            | "format_scientific"
            | "format_table"
            | "truncate"
            // HTML
            | "html_attr"
            | "html_escape"
            | "html_tag"
            // List operations
            | "drop"
            | "enumerate"
            | "filter"
            | "flatmap"
            | "flatten"
            | "fold"
            | "head"
            | "map"
            | "partition"
            | "range"
            | "reverse"
            | "sort"
            | "sort_by"
            | "split_at"
            | "tail"
            | "take"
            | "unique"
            | "unzip"
            | "zip"
            // Markdown
            | "markdown_to_html"
            | "md_code"
            | "md_heading"
            | "md_link"
            | "md_list"
            // Math
            | "neg"
            // String operations
            | "char_at"
            | "chars"
            | "concat"
            | "contains"
            | "ends_with"
            | "indent"
            | "is_alpha"
            | "is_alphanumeric"
            | "is_digit"
            | "is_empty"
            | "is_lowercase"
            | "is_uppercase"
            | "is_whitespace"
            | "join"
            | "length"
            | "lower"
            | "pad_left"
            | "pad_right"
            | "repeat"
            | "replace"
            | "slice"
            | "split"
            | "starts_with"
            | "trim"
            | "upper"
            // Type checking
            | "is_bool"
            | "is_dict"
            | "is_float"
            | "is_function"
            | "is_int"
            | "is_list"
            | "is_none"
            | "is_number"
            | "is_string"
            | "typeof"
            // Type conversion
            | "to_bool"
            | "to_char"
            | "to_float"
            | "to_int"
            | "to_list"
            | "to_string"
    )
}

/// Get the arity (number of arguments) for a builtin function
pub fn get_builtin_arity(name: &str) -> Option<usize> {
    Some(match name {
        // Arity 1
        "basename" | "chars" | "debug" | "dirname" | "enumerate" | "error" | "exists"
        | "flatten" | "format_binary" | "format_bytes" | "format_hex" | "format_json"
        | "format_octal" | "head" | "html_escape" | "import" | "is_alpha" | "is_alphanumeric"
        | "is_bool" | "is_dict" | "is_digit" | "is_empty" | "is_float" | "is_function"
        | "is_int" | "is_list" | "is_lowercase" | "is_none" | "is_number" | "is_string"
        | "is_uppercase" | "is_whitespace" | "json_parse" | "keys" | "length" | "lower"
        | "markdown_to_html" | "max" | "md_code" | "md_list" | "min" | "neg" | "not" | "now"
        | "readfile" | "readlines" | "reverse" | "sort" | "sum" | "tail" | "timestamp"
        | "timezone" | "to_bool" | "to_char" | "to_float" | "to_int" | "to_list"
        | "to_string" | "trim" | "typeof" | "unique" | "unzip" | "upper" | "values"
        | "walkdir" => 1,

        // Arity 2
        "all" | "any" | "center" | "char_at" | "concat" | "contains" | "count"
        | "date_add" | "date_diff" | "date_format" | "date_parse" | "dict_merge" | "drop" | "ends_with" | "env_var_or"
        | "fill_template" | "filter" | "flatmap" | "format_currency" | "format_float"
        | "format_int" | "format_list" | "format_percent" | "format_scientific"
        | "format_table" | "format_bool" | "get" | "has_key" | "html_attr" | "html_tag"
        | "indent" | "join" | "map" | "md_heading" | "md_link" | "partition" | "range"
        | "repeat" | "sort_by" | "split" | "split_at" | "starts_with" | "take" | "trace"
        | "truncate" | "zip" | "assert" => 2,

        // Arity 3
        "fold" | "pad_left" | "pad_right" | "replace" | "set"
        | "slice" => 3,

        // Special case: 0 arity but handled specially in apply_function
        "env_var" => 1,

        _ => return None,
    })
}

/// Create initial symbol table with all builtin functions
pub fn initial_builtins() -> HashMap<String, Value> {
    let mut m = HashMap::new();

    // Helper macro to reduce boilerplate
    macro_rules! add_builtin {
        ($name:expr) => {
            m.insert($name.to_string(), Value::Builtin($name.to_string(), Vec::new()));
        };
    }

    // Aggregate functions
    add_builtin!("all");
    add_builtin!("any");
    add_builtin!("count");
    add_builtin!("max");
    add_builtin!("min");
    add_builtin!("sum");

    // Assertion/Debug
    add_builtin!("assert");
    add_builtin!("debug");
    add_builtin!("error");
    add_builtin!("not");
    add_builtin!("trace");

    // Date/Time
    add_builtin!("date_add");
    add_builtin!("date_diff");
    add_builtin!("date_format");
    add_builtin!("date_parse");
    add_builtin!("now");
    add_builtin!("timestamp");
    add_builtin!("timezone");

    // Dictionary
    add_builtin!("dict_merge");
    add_builtin!("get");
    add_builtin!("has_key");
    add_builtin!("keys");
    add_builtin!("set");
    add_builtin!("values");

    // Environment
    add_builtin!("env_var");
    add_builtin!("env_var_or");
    // os is special - it's a constant, not a function
    m.insert("os".to_string(), Value::String(std::env::consts::OS.to_string()));

    // File I/O
    add_builtin!("basename");
    add_builtin!("dirname");
    add_builtin!("exists");
    add_builtin!("fill_template");
    add_builtin!("import");
    add_builtin!("json_parse");
    add_builtin!("readfile");
    add_builtin!("readlines");
    add_builtin!("walkdir");

    // Formatting
    add_builtin!("center");
    add_builtin!("format_binary");
    add_builtin!("format_bool");
    add_builtin!("format_bytes");
    add_builtin!("format_currency");
    add_builtin!("format_float");
    add_builtin!("format_hex");
    add_builtin!("format_int");
    add_builtin!("format_json");
    add_builtin!("format_list");
    add_builtin!("format_octal");
    add_builtin!("format_percent");
    add_builtin!("format_scientific");
    add_builtin!("format_table");
    add_builtin!("truncate");

    // HTML
    add_builtin!("html_attr");
    add_builtin!("html_escape");
    add_builtin!("html_tag");

    // List operations
    add_builtin!("drop");
    add_builtin!("enumerate");
    add_builtin!("filter");
    add_builtin!("flatmap");
    add_builtin!("flatten");
    add_builtin!("fold");
    add_builtin!("head");
    add_builtin!("map");
    add_builtin!("partition");
    add_builtin!("range");
    add_builtin!("reverse");
    add_builtin!("sort");
    add_builtin!("sort_by");
    add_builtin!("split_at");
    add_builtin!("tail");
    add_builtin!("take");
    add_builtin!("unique");
    add_builtin!("unzip");
    add_builtin!("zip");

    // Markdown
    add_builtin!("markdown_to_html");
    add_builtin!("md_code");
    add_builtin!("md_heading");
    add_builtin!("md_link");
    add_builtin!("md_list");

    // Math
    add_builtin!("neg");

    // String operations
    add_builtin!("char_at");
    add_builtin!("chars");
    add_builtin!("concat");
    add_builtin!("contains");
    add_builtin!("ends_with");
    add_builtin!("indent");
    add_builtin!("is_alpha");
    add_builtin!("is_alphanumeric");
    add_builtin!("is_digit");
    add_builtin!("is_empty");
    add_builtin!("is_lowercase");
    add_builtin!("is_uppercase");
    add_builtin!("is_whitespace");
    add_builtin!("join");
    add_builtin!("length");
    add_builtin!("lower");
    add_builtin!("pad_left");
    add_builtin!("pad_right");
    add_builtin!("repeat");
    add_builtin!("replace");
    add_builtin!("slice");
    add_builtin!("split");
    add_builtin!("starts_with");
    add_builtin!("trim");
    add_builtin!("upper");

    // Type checking
    add_builtin!("is_bool");
    add_builtin!("is_dict");
    add_builtin!("is_float");
    add_builtin!("is_function");
    add_builtin!("is_int");
    add_builtin!("is_list");
    add_builtin!("is_none");
    add_builtin!("is_number");
    add_builtin!("is_string");
    add_builtin!("typeof");

    // Type conversion
    add_builtin!("to_bool");
    add_builtin!("to_char");
    add_builtin!("to_float");
    add_builtin!("to_int");
    add_builtin!("to_list");
    add_builtin!("to_string");

    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_builtins_have_arity() {
        let builtins = initial_builtins();
        for (name, _) in builtins.iter() {
            if name == "os" {
                continue; // os is a constant, not a function
            }
            assert!(
                get_builtin_arity(name).is_some(),
                "Builtin '{}' has no arity defined",
                name
            );
        }
    }

    #[test]
    fn test_all_builtins_are_registered() {
        let builtins = initial_builtins();
        for (name, _) in builtins.iter() {
            if name == "os" {
                continue; // os is a constant, not a function
            }
            assert!(
                is_builtin_name(name),
                "Builtin '{}' not in is_builtin_name()",
                name
            );
        }
    }
}
