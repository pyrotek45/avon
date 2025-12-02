//! Builtin function module - registry, arity, and dispatcher
//!
//! This module provides the central interface for builtin functions:
//! - Registry: which functions exist and their arities
//! - Execution: delegating to category-specific handlers
//!
//! Builtins are organized by category:
//! - aggregate: sum, min, max, all, any, count
//! - debug: assert, debug, error, not, trace
//! - datetime: date_add, date_diff, date_format, date_parse, now, timestamp, timezone
//! - dict: dict_merge, get, has_key, keys, set, values
//! - env: env_var, env_var_or, os
//! - file_io: basename, dirname, exists, fill_template, import, json_parse, readfile, readlines, walkdir
//! - formatting: center, format_*, truncate
//! - html: html_attr, html_escape, html_tag
//! - list: drop, enumerate, filter, flatmap, flatten, fold, head, map, partition, range, reverse, sort, sort_by, split_at, tail, take, unique, unzip, zip
//! - markdown: markdown_to_html, md_code, md_heading, md_link, md_list
//! - math: neg
//! - string: char_at, chars, concat, contains, ends_with, indent, is_alpha, is_alphanumeric, is_digit, is_empty, is_lowercase, is_uppercase, is_whitespace, join, length, lower, pad_left, pad_right, repeat, replace, slice, split, starts_with, trim, upper
//! - types: is_bool, is_dict, is_float, is_function, is_int, is_list, is_none, is_number, is_string, typeof, to_bool, to_char, to_float, to_int, to_list, to_string

// Category modules
pub mod aggregate;
pub mod datetime;
pub mod debug;
pub mod dict;
pub mod env;
pub mod file_io;
pub mod formatting;
pub mod html;
pub mod list;
pub mod markdown;
pub mod math;
pub mod registry;
pub mod string;
pub mod types;

// Re-export registry functions
pub use registry::{get_builtin_arity, initial_builtins, is_builtin_name};
