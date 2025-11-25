use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Chunk {
    String(String),
    Expr(String),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Template(Vec<Chunk>),
    Path(Vec<Chunk>),
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    NotEqual,
    Mul,
    Div,
    Add,
    Sub,
    Dot,
    Equal,
    DoubleEqual,
    DoubleDot,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Question,
    At,
    BackSlash,
}

impl Token {
    pub fn is_term_op(&self) -> bool {
        match self {
            Token::Add => true,
            Token::Sub => true,
            _ => false,
        }
    }

    pub fn is_factor_op(&self) -> bool {
        match self {
            Token::Mul => true,
            Token::Div => true,
            _ => false,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Number {
    pub fn from(v: i64) -> Number {
        Number::Int(v)
    }

    pub fn from_f64(v: f64) -> Number {
        Number::Float(v)
    }

    pub fn add(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(v), Number::Int(r)) => Number::Int(v + r),
            (Number::Int(v), Number::Float(r)) => Number::Float(v as f64 + r),
            (Number::Float(v), Number::Int(r)) => Number::Float(v + r as f64),
            (Number::Float(v), Number::Float(r)) => Number::Float(v + r),
        }
    }

    pub fn mul(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(v), Number::Int(r)) => Number::Int(v * r),
            (Number::Int(v), Number::Float(r)) => Number::Float(v as f64 * r),
            (Number::Float(v), Number::Int(r)) => Number::Float(v * r as f64),
            (Number::Float(v), Number::Float(r)) => Number::Float(v * r),
        }
    }

    pub fn div(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(v), Number::Int(r)) => Number::Int(v / r),
            (Number::Int(v), Number::Float(r)) => Number::Float(v as f64 / r),
            (Number::Float(v), Number::Int(r)) => Number::Float(v / r as f64),
            (Number::Float(v), Number::Float(r)) => Number::Float(v / r),
        }
    }

    pub fn sub(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(v), Number::Int(r)) => Number::Int(v - r),
            (Number::Int(v), Number::Float(r)) => Number::Float(v as f64 - r),
            (Number::Float(v), Number::Int(r)) => Number::Float(v - r as f64),
            (Number::Float(v), Number::Float(r)) => Number::Float(v - r),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    None,
    Bool(bool),
    Ident(String),
    Let {
        ident: String,
        value: Box<Expr>,
        expr: Box<Expr>,
    },
    Function {
        ident: String,
        default: Option<Box<Expr>>,
        expr: Box<Expr>,
    },
    Application {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Number(Number),
    String(String),
    Template(Vec<Chunk>),
    Path(Vec<Chunk>),
    FileTemplate {
        path: Vec<Chunk>,
        template: Vec<Chunk>,
    },
    List(Vec<Expr>),
    Builtin(String, Vec<String>),
    If {
        cond: Box<Expr>,
        t: Box<Expr>,
        f: Box<Expr>,
    },
    Binary {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    Member {
        object: Box<Expr>,
        field: String,
    },
}

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Bool(bool),
    Number(Number),
    String(String),
    #[allow(dead_code)]
    Function {
        ident: String,
        default: Option<Box<Value>>,
        expr: Box<Expr>,
        env: HashMap<String, Value>,
    },
    Builtin(String, Vec<Value>),
    Template(Vec<Chunk>, HashMap<String, Value>),
    Path(Vec<Chunk>, HashMap<String, Value>),
    FileTemplate {
        path: (Vec<Chunk>, HashMap<String, Value>),
        template: (Vec<Chunk>, HashMap<String, Value>),
    },
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub program: Expr,
}

#[derive(Debug, Clone)]
pub struct EvalError {
    pub message: String,
    pub expected: Option<String>,
    pub found: Option<String>,
    pub line: usize,
    pub column: Option<usize>,
    pub context: Option<String>,
    pub hint: Option<String>,
}

impl EvalError {
    pub fn new(
        message: impl Into<String>,
        expected: Option<String>,
        found: Option<String>,
        line: usize,
    ) -> Self {
        Self {
            message: message.into(),
            expected,
            found,
            line,
            column: None,
            context: None,
            hint: None,
        }
    }

    pub fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn unknown_symbol(sym: impl Into<String>, line: usize) -> Self {
        let sym_str = sym.into();
        let hint = Self::suggest_for_unknown_symbol(&sym_str);
        Self::new(format!("unknown symbol: {}", sym_str), None, None, line)
            .with_hint(hint)
    }

    pub fn type_mismatch(
        expected: impl Into<String>,
        found: impl Into<String>,
        line: usize,
    ) -> Self {
        let expected_str = expected.into();
        let found_str = found.into();
        let hint = Self::suggest_for_type_mismatch(&expected_str, &found_str);
        
        Self::new(
            "type mismatch",
            Some(expected_str),
            Some(found_str),
            line,
        )
        .with_hint(hint)
    }

    fn suggest_for_unknown_symbol(sym: &str) -> String {
        // Common typos and suggestions
        let suggestions = vec![
            ("lenght", "length"),
            ("concate", "concat"),
            ("uppser", "upper"),
            ("lowwer", "lower"),
            ("mapp", "map"),
            ("foldd", "fold"),
            ("filterr", "filter"),
            ("readfilee", "readfile"),
            ("to_str", "to_string"),
            ("to_string", "str"),  // Alternative suggestion
            ("print", "trace or debug"),
            ("println", "trace or debug"),
            ("assert", "assert_string, assert_number, etc."),
        ];

        for (typo, correct) in suggestions {
            if sym.to_lowercase() == typo.to_lowercase() {
                return format!("Did you mean '{}'?", correct);
            }
        }

        // Check for common patterns
        if sym.starts_with("is_") {
            return "Available type predicates: is_string, is_number, is_int, is_float, is_list, is_bool, is_function".to_string();
        }
        if sym.starts_with("assert_") {
            return "Available assertions: assert_string, assert_number, assert_int, assert_list, assert_bool".to_string();
        }
        if sym.starts_with("format_") {
            return "Available formatters: format_int, format_float, format_hex, format_bytes, format_list, etc.".to_string();
        }
        if sym.contains("file") || sym.contains("read") {
            return "File functions: readfile, readlines, exists, import, fill_template, basename, dirname, walkdir".to_string();
        }

        "Run 'avon --doc' to see all available builtin functions".to_string()
    }

    fn suggest_for_type_mismatch(expected: &str, found: &str) -> String {
        // Provide helpful conversion hints
        if expected.contains("String") && found.contains("Number") {
            return "Use 'to_string' to convert a number to a string".to_string();
        }
        if expected.contains("Number") && found.contains("String") {
            return "Use 'to_int' or 'to_float' to convert a string to a number".to_string();
        }
        if expected.contains("List") && !found.contains("List") {
            return "Wrap the value in brackets to create a list: [value]".to_string();
        }
        if expected.contains("Bool") && !found.contains("Bool") {
            return "Use a comparison operator (==, !=, <, >, etc.) to create a boolean".to_string();
        }
        if expected.contains("Int") && found.contains("Float") {
            return "Use 'to_int' to convert a float to an integer, or change assertion to assert_number".to_string();
        }
        if expected.contains("Number") && found.contains("Template") {
            return "Templates cannot be used in arithmetic. Use template interpolation instead: {expr}".to_string();
        }

        String::new()
    }

    pub fn pretty(&self, source: &str) -> String {
        self.pretty_with_file(source, None)
    }

    pub fn pretty_with_file(&self, source: &str, filename: Option<&str>) -> String {
        if self.line == 0 {
            return self.format_simple();
        }

        let lines: Vec<&str> = source.lines().collect();
        let idx = if self.line > 0 { self.line - 1 } else { 0 };
        if idx >= lines.len() {
            return self.format_simple();
        }

        let line_str = lines[idx];
        let line_num_str = format!("{:>4} | ", self.line);
        let padding = " ".repeat(line_num_str.len());
        
        // Build the error message
        let mut output = String::new();
        
        // Clickable file:line:column format at the top
        let file_location = if let Some(fname) = filename {
            if let Some(col) = self.column {
                format!("{}:{}:{}", fname, self.line, col)
            } else {
                format!("{}:{}", fname, self.line)
            }
        } else {
            if let Some(col) = self.column {
                format!("<input>:{}:{}", self.line, col)
            } else {
                format!("<input>:{}", self.line)
            }
        };
        output.push_str(&format!("\x1b[1;31mError\x1b[0m at \x1b[1m{}\x1b[0m\n", file_location));
        
        // Main error message
        if let (Some(exp), Some(found)) = (self.expected.as_ref(), self.found.as_ref()) {
            output.push_str(&format!(
                "\x1b[1m{}\x1b[0m: expected \x1b[32m{}\x1b[0m, found \x1b[31m{}\x1b[0m\n",
                self.message, exp, found
            ));
        } else {
            output.push_str(&format!("\x1b[1m{}\x1b[0m\n", self.message));
        }
        
        // Location (keeping this for readability)
        output.push_str(&format!("{}  \x1b[36m-->\x1b[0m line {}", padding, self.line));
        if let Some(col) = self.column {
            output.push_str(&format!(", column {}", col));
        }
        output.push('\n');
        
        // Empty line for spacing
        output.push_str(&format!("{}\x1b[36m|\x1b[0m\n", padding));
        
        // Source line with highlighting
        output.push_str(&format!("\x1b[36m{}\x1b[0m{}\n", line_num_str, line_str));
        
        // Caret pointer
        let caret_offset = if let Some(col) = self.column {
            col.saturating_sub(1)
        } else {
            0
        };
        output.push_str(&format!(
            "{}\x1b[36m|\x1b[0m {}\x1b[1;31m^\x1b[0m\n",
            padding,
            " ".repeat(caret_offset)
        ));
        
        // Hint if available
        if let Some(hint) = &self.hint {
            if !hint.is_empty() {
                output.push_str(&format!("{}\x1b[36m|\x1b[0m\n", padding));
                output.push_str(&format!(
                    "{}\x1b[36m= \x1b[1;36mHint:\x1b[0m {}\n",
                    padding, hint
                ));
            }
        }
        
        output
    }

    fn format_simple(&self) -> String {
        if let (Some(exp), Some(found)) = (self.expected.as_ref(), self.found.as_ref()) {
            format!(
                "\x1b[1;31mError:\x1b[0m {}: expected {}, found {} (line {})",
                self.message, exp, found, self.line
            )
        } else {
            format!("\x1b[1;31mError:\x1b[0m {} (line {})", self.message, self.line)
        }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_simple())
    }
}
