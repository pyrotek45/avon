//! Builtin function module - registry, arity, and dispatcher
//!
//! This module provides the central interface for builtin functions:
//! - Registry: which functions exist and their arities
//! - Execution: delegating to category-specific handlers

mod registry;

// Re-export registry functions
pub use registry::{get_builtin_arity, initial_builtins, is_builtin_name};
