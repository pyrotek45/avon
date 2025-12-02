use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Chunk {
    String(String),
    Expr(String, usize),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Int(i64, usize),
    Float(f64, usize),
    String(String, usize),
    Identifier(String, usize),
    Template(Vec<Chunk>, usize),
    Path(Vec<Chunk>, usize),
    Greater(usize),
    Less(usize),
    GreaterEqual(usize),
    LessEqual(usize),
    NotEqual(usize),
    Mul(usize),
    Div(usize),
    Mod(usize),
    Add(usize),
    Sub(usize),
    Dot(usize),
    Equal(usize),
    DoubleEqual(usize),
    DoubleDot(usize),
    And(usize),
    Or(usize),
    LParen(usize),
    RParen(usize),
    LBracket(usize),
    RBracket(usize),
    LBrace(usize),
    RBrace(usize),
    Comma(usize),
    Colon(usize),
    Question(usize),
    At(usize),
    BackSlash(usize),
    Pipe(usize),
}

impl Token {
    pub fn line(&self) -> usize {
        match self {
            Token::Int(_, l) => *l,
            Token::Float(_, l) => *l,
            Token::String(_, l) => *l,
            Token::Identifier(_, l) => *l,
            Token::Template(_, l) => *l,
            Token::Path(_, l) => *l,
            Token::Greater(l) => *l,
            Token::Less(l) => *l,
            Token::GreaterEqual(l) => *l,
            Token::LessEqual(l) => *l,
            Token::NotEqual(l) => *l,
            Token::Mul(l) => *l,
            Token::Div(l) => *l,
            Token::Mod(l) => *l,
            Token::Add(l) => *l,
            Token::Sub(l) => *l,
            Token::Dot(l) => *l,
            Token::Equal(l) => *l,
            Token::DoubleEqual(l) => *l,
            Token::DoubleDot(l) => *l,
            Token::And(l) => *l,
            Token::Or(l) => *l,
            Token::LParen(l) => *l,
            Token::RParen(l) => *l,
            Token::LBracket(l) => *l,
            Token::RBracket(l) => *l,
            Token::LBrace(l) => *l,
            Token::RBrace(l) => *l,
            Token::Comma(l) => *l,
            Token::Colon(l) => *l,
            Token::Question(l) => *l,
            Token::At(l) => *l,
            Token::BackSlash(l) => *l,
            Token::Pipe(l) => *l,
        }
    }

    pub fn is_term_op(&self) -> bool {
        matches!(self, Token::Add(_) | Token::Sub(_))
    }

    pub fn is_factor_op(&self) -> bool {
        matches!(self, Token::Mul(_) | Token::Div(_) | Token::Mod(_))
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
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(v), Number::Int(r)) => Number::Int(v.wrapping_add(r)),
            (Number::Int(v), Number::Float(r)) => Number::Float(v as f64 + r),
            (Number::Float(v), Number::Int(r)) => Number::Float(v + r as f64),
            (Number::Float(v), Number::Float(r)) => Number::Float(v + r),
        }
    }
}

impl std::ops::Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(v), Number::Int(r)) => Number::Int(v.wrapping_mul(r)),
            (Number::Int(v), Number::Float(r)) => Number::Float(v as f64 * r),
            (Number::Float(v), Number::Int(r)) => Number::Float(v * r as f64),
            (Number::Float(v), Number::Float(r)) => Number::Float(v * r),
        }
    }
}

impl std::ops::Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        match (self, other) {
            // Use floor division for integers
            (Number::Int(v), Number::Int(r)) => {
                if r == 0 {
                    panic!("division by zero");
                }

                // Handle overflow: MIN / -1
                if v == i64::MIN && r == -1 {
                    return Number::Int(v); // wrapping behavior: MIN / -1 = MIN
                }

                // Floor division: v / r rounded toward negative infinity
                let quotient = v / r;
                let remainder = v % r;
                // If signs differ and there's a remainder, adjust down
                let result = if remainder != 0 && (v < 0) != (r < 0) {
                    quotient - 1
                } else {
                    quotient
                };
                Number::Int(result)
            }
            (Number::Int(v), Number::Float(r)) => Number::Float(v as f64 / r),
            (Number::Float(v), Number::Int(r)) => Number::Float(v / r as f64),
            (Number::Float(v), Number::Float(r)) => Number::Float(v / r),
        }
    }
}

impl std::ops::Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(v), Number::Int(r)) => Number::Int(v.wrapping_sub(r)),
            (Number::Int(v), Number::Float(r)) => Number::Float(v as f64 - r),
            (Number::Float(v), Number::Int(r)) => Number::Float(v - r as f64),
            (Number::Float(v), Number::Float(r)) => Number::Float(v - r),
        }
    }
}

impl std::ops::Rem for Number {
    type Output = Number;

    fn rem(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(v), Number::Int(r)) => Number::Int(v % r),
            (Number::Int(v), Number::Float(r)) => Number::Float((v as f64) % r),
            (Number::Float(v), Number::Int(r)) => Number::Float(v % (r as f64)),
            (Number::Float(v), Number::Float(r)) => Number::Float(v % r),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    None(usize),
    Bool(bool, usize),
    Ident(String, usize),
    Let {
        ident: String,
        value: Box<Expr>,
        expr: Box<Expr>,
        line: usize,
    },
    Function {
        ident: String,
        default: Option<Box<Expr>>,
        expr: Box<Expr>,
        line: usize,
    },
    Application {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        line: usize,
    },
    Number(Number, usize),
    String(String, usize),
    Template(Vec<Chunk>, usize),
    Path(Vec<Chunk>, usize),
    FileTemplate {
        path: Vec<Chunk>,
        template: Vec<Chunk>,
        line: usize,
    },
    List(Vec<Expr>, usize),
    Range {
        start: Box<Expr>,
        step: Option<Box<Expr>>,
        end: Box<Expr>,
        line: usize,
    },
    Dict(Vec<(String, Expr)>, usize),
    Builtin(String, Vec<String>, usize),
    If {
        cond: Box<Expr>,
        t: Box<Expr>,
        f: Box<Expr>,
        line: usize,
    },
    Binary {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
        line: usize,
    },
    Member {
        object: Box<Expr>,
        field: String,
        line: usize,
    },
    Pipe {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        line: usize,
    },
}

impl Expr {
    pub fn line(&self) -> usize {
        match self {
            Expr::None(l) => *l,
            Expr::Bool(_, l) => *l,
            Expr::Ident(_, l) => *l,
            Expr::Let { line, .. } => *line,
            Expr::Function { line, .. } => *line,
            Expr::Application { line, .. } => *line,
            Expr::Number(_, l) => *l,
            Expr::String(_, l) => *l,
            Expr::Template(_, l) => *l,
            Expr::Path(_, l) => *l,
            Expr::FileTemplate { line, .. } => *line,
            Expr::List(_, l) => *l,
            Expr::Range { line, .. } => *line,
            Expr::Dict(_, l) => *l,
            Expr::Builtin(_, _, l) => *l,
            Expr::If { line, .. } => *line,
            Expr::Binary { line, .. } => *line,
            Expr::Member { line, .. } => *line,
            Expr::Pipe { line, .. } => *line,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Bool(bool),
    Number(Number),
    String(String),
    #[allow(dead_code)]
    Function {
        name: Option<String>, // The name the function was bound to (e.g., "stringify")
        ident: String,        // The parameter name (e.g., "x")
        default: Option<Box<Value>>,
        expr: Box<Expr>,
        env: std::rc::Rc<HashMap<String, Value>>, // Reference counted immutable environment capture
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
/// Main error type for evaluation and parsing.
/// Contains multiple String fields for rich error reporting (message, expected, found, context, hint).
/// While larger than ideal, boxing would require changes throughout the codebase and add
/// unnecessary indirection since this error type is fundamental to the architecture.
#[allow(clippy::result_large_err)]
pub struct EvalError {
    pub message: String,
    pub expected: Option<String>,
    pub found: Option<String>,
    #[allow(dead_code)]
    pub line: usize,
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
        Self::new(format!("unknown symbol: {}", sym_str), None, None, line).with_hint(hint)
    }

    pub fn type_mismatch(
        expected: impl Into<String>,
        found: impl Into<String>,
        line: usize,
    ) -> Self {
        let expected_str = expected.into();
        let found_str = found.into();
        let hint = Self::suggest_for_type_mismatch(&expected_str, &found_str);

        Self::new("type mismatch", Some(expected_str), Some(found_str), line).with_hint(hint)
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
            ("to_string", "str"), // Alternative suggestion
            ("print", "trace or debug"),
            ("println", "trace or debug"),
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
            return "Use general 'assert' function: assert (is_number x) x, assert (x > 0) x, etc."
                .to_string();
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
            return "Use a comparison operator (==, !=, <, >, etc.) to create a boolean"
                .to_string();
        }
        if expected.contains("Int") && found.contains("Float") {
            return "Use 'to_int' to convert a float to an integer, or use assert (is_number x) x instead of assert (is_int x) x".to_string();
        }
        if expected.contains("Number") && found.contains("Template") {
            return "Templates cannot be used in arithmetic. Use template interpolation instead: {expr}".to_string();
        }

        String::new()
    }

    #[allow(dead_code)]
    pub fn pretty(&self, source: &str) -> String {
        let mut out = self.format_simple();
        if self.line > 0 {
            if let Some(line_str) = source.lines().nth(self.line - 1) {
                out.push('\n');
                out.push_str(&format!("{:4} | {}", self.line, line_str));
            }
        }
        out
    }

    pub fn pretty_with_file(&self, source: &str, filename: Option<&str>) -> String {
        let mut out = self.format_simple();
        if let Some(fname) = filename {
            if self.line > 0 {
                out.push_str(&format!(" in {}", fname));
            }
        }
        if self.line > 0 {
            if let Some(line_str) = source.lines().nth(self.line - 1) {
                out.push('\n');
                out.push_str(&format!("{:4} | {}", self.line, line_str));
            }
        }
        out
    }

    fn format_simple(&self) -> String {
        let loc = if self.line > 0 {
            format!(" on line {}", self.line)
        } else {
            String::new()
        };
        if let (Some(exp), Some(found)) = (self.expected.as_ref(), self.found.as_ref()) {
            format!("{}: expected {}, found {}{}", self.message, exp, found, loc)
        } else {
            format!("{}{}", self.message, loc)
        }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_simple())
    }
}
