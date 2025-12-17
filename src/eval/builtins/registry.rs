//! Builtin function registry - tracks which functions exist and their arities
//!
//! This module provides:
//! - `is_builtin_name()` - Check if a name is a builtin function
//! - `initial_builtins()` - Get initial symbol table with all builtins
//! - `get_builtin_arity()` - Get the number of arguments for a builtin
//!
//! All data is derived from category modules (aggregate, debug, datetime, etc.)

use crate::common::Value;
use std::collections::HashMap;

use super::{
    aggregate, datetime, debug, dict, env, file_io, formatting, html, list, markdown, math, regex,
    string, types,
};

/// Type alias for category module definition
type CategoryModule = (&'static [&'static str], fn(&str) -> Option<usize>);

/// All category modules for iteration
const CATEGORY_MODULES: &[CategoryModule] = &[
    (aggregate::NAMES, aggregate::get_arity),
    (debug::NAMES, debug::get_arity),
    (datetime::NAMES, datetime::get_arity),
    (dict::NAMES, dict::get_arity),
    (env::NAMES, env::get_arity),
    (file_io::NAMES, file_io::get_arity),
    (formatting::NAMES, formatting::get_arity),
    (html::NAMES, html::get_arity),
    (list::NAMES, list::get_arity),
    (markdown::NAMES, markdown::get_arity),
    (math::NAMES, math::get_arity),
    (regex::NAMES, regex::get_arity),
    (string::NAMES, string::get_arity),
    (types::NAMES, types::get_arity),
];

/// Check if a name is a builtin function name
/// This uses the NAMES arrays from each category module
pub fn is_builtin_name(name: &str) -> bool {
    aggregate::is_builtin(name)
        || debug::is_builtin(name)
        || datetime::is_builtin(name)
        || dict::is_builtin(name)
        || env::is_builtin(name)
        || file_io::is_builtin(name)
        || formatting::is_builtin(name)
        || html::is_builtin(name)
        || list::is_builtin(name)
        || markdown::is_builtin(name)
        || math::is_builtin(name)
        || regex::is_builtin(name)
        || string::is_builtin(name)
        || types::is_builtin(name)
}

/// Get the arity (number of arguments) for a builtin function
/// Delegates to category-specific get_arity functions
pub fn get_builtin_arity(name: &str) -> Option<usize> {
    aggregate::get_arity(name)
        .or_else(|| debug::get_arity(name))
        .or_else(|| datetime::get_arity(name))
        .or_else(|| dict::get_arity(name))
        .or_else(|| env::get_arity(name))
        .or_else(|| file_io::get_arity(name))
        .or_else(|| formatting::get_arity(name))
        .or_else(|| html::get_arity(name))
        .or_else(|| list::get_arity(name))
        .or_else(|| markdown::get_arity(name))
        .or_else(|| math::get_arity(name))
        .or_else(|| regex::get_arity(name))
        .or_else(|| string::get_arity(name))
        .or_else(|| types::get_arity(name))
}

/// Create initial symbol table with all builtin functions
/// Collects builtins from all category modules
pub fn initial_builtins() -> HashMap<String, Value> {
    let mut m = HashMap::new();

    // Add all builtins from each category using CATEGORY_MODULES
    for (names, _) in CATEGORY_MODULES {
        for name in *names {
            // Skip "os" - it's a constant, not a function
            if *name == "os" {
                continue;
            }
            m.insert(
                name.to_string(),
                Value::Builtin(name.to_string(), Vec::new()),
            );
        }
    }

    // Add special constants
    m.insert(
        "os".to_string(),
        Value::String(std::env::consts::OS.to_string()),
    );

    // Add args as an empty list by default (CLI will override with actual args)
    m.insert("args".to_string(), Value::List(vec![]));

    m
}

/// Get total count of all builtins (for documentation/debugging)
#[allow(dead_code)]
pub fn builtin_count() -> usize {
    CATEGORY_MODULES.iter().map(|(names, _)| names.len()).sum()
}

/// Get all builtin names grouped by category
#[allow(dead_code)]
pub fn all_builtin_names() -> Vec<(&'static str, &'static [&'static str])> {
    vec![
        ("aggregate", aggregate::NAMES),
        ("debug", debug::NAMES),
        ("datetime", datetime::NAMES),
        ("dict", dict::NAMES),
        ("env", env::NAMES),
        ("file_io", file_io::NAMES),
        ("formatting", formatting::NAMES),
        ("html", html::NAMES),
        ("list", list::NAMES),
        ("markdown", markdown::NAMES),
        ("math", math::NAMES),
        ("regex", regex::NAMES),
        ("string", string::NAMES),
        ("types", types::NAMES),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_builtins_have_arity() {
        let builtins = initial_builtins();
        for (name, _) in builtins.iter() {
            if name == "os" || name == "args" {
                continue; // os and args are constants, not functions
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
            if name == "os" || name == "args" {
                continue; // os and args are constants, not functions
            }
            assert!(
                is_builtin_name(name),
                "Builtin '{}' not in is_builtin_name()",
                name
            );
        }
    }

    #[test]
    fn test_builtin_count() {
        // Should have around 120 builtins
        let count = builtin_count();
        assert!(
            count >= 100,
            "Expected at least 100 builtins, got {}",
            count
        );
    }

    #[test]
    fn test_category_coverage() {
        // Test that each category has at least one builtin
        let categories = all_builtin_names();
        for (category, names) in categories {
            assert!(!names.is_empty(), "Category '{}' has no builtins", category);
        }
    }
}
