// REPL state management

use crate::common::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Stores pending :let command data for multi-line input
#[derive(Debug, Clone)]
pub struct PendingLet {
    pub var_name: String,
    pub expr_buffer: String,
}

pub struct ReplState {
    pub symbols: HashMap<String, Value>,
    pub symbols_rc: Rc<RefCell<HashMap<String, Value>>>,
    pub input_buffer: String,
    pub watched_vars: HashMap<String, Value>,
    pub pending_let: Option<PendingLet>,
    pub consecutive_empty_lines: u32,
}

impl ReplState {
    pub fn new(
        symbols: HashMap<String, Value>,
        symbols_rc: Rc<RefCell<HashMap<String, Value>>>,
    ) -> Self {
        Self {
            symbols,
            symbols_rc,
            input_buffer: String::new(),
            watched_vars: HashMap::new(),
            pending_let: None,
            consecutive_empty_lines: 0,
        }
    }

    pub fn sync_symbols(&self) {
        *self.symbols_rc.borrow_mut() = self.symbols.clone();
    }
}
