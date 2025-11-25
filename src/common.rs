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
        }
    }

    pub fn unknown_symbol(sym: impl Into<String>, line: usize) -> Self {
        Self::new(format!("unknown symbol {}", sym.into()), None, None, line)
    }

    pub fn type_mismatch(
        expected: impl Into<String>,
        found: impl Into<String>,
        line: usize,
    ) -> Self {
        Self::new(
            "type mismatch",
            Some(expected.into()),
            Some(found.into()),
            line,
        )
    }

    pub fn pretty(&self, source: &str) -> String {
        if self.line == 0 {
            return format!("{}", self);
        }

        let lines: Vec<&str> = source.lines().collect();
        let idx = if self.line > 0 { self.line - 1 } else { 0 } as usize;
        if idx >= lines.len() {
            return format!("{}", self);
        }

        let line_str = lines[idx];
        let caret = "^";
        format!("{}\n{}\n{}", self, line_str, caret)
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(exp), Some(found)) = (self.expected.as_ref(), self.found.as_ref()) {
            write!(
                f,
                "{}: expected {}, found {} (line {})",
                self.message, exp, found, self.line
            )
        } else {
            write!(f, "{} (line {})", self.message, self.line)
        }
    }
}
