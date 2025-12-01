// Avon Language Library
// Exposes lexer, parser, and type system for use in other binaries and tools

pub mod common;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod syntax;

pub use common::{Ast, Chunk, Expr, Token};
pub use lexer::tokenize;
pub use parser::parse;
pub use syntax::AvonHighlighter;
