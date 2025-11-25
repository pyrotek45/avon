use crate::common::{Ast, Expr, Token};
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse_atom(stream: &mut Peekable<Iter<Token>>) -> Expr {
    match stream.next() {
        Some(atom) => match atom {
            Token::Int(value) => Expr::Number(crate::common::Number::from(*value)),
            Token::Float(value) => Expr::Number(crate::common::Number::from_f64(*value)),
            Token::String(value) => Expr::String(value.clone()),
            Token::Template(chunks) => Expr::Template(chunks.clone()),
            Token::LBracket => {
                let mut items = Vec::new();
                loop {
                    match stream.peek() {
                        Some(Token::RBracket) => {
                            stream.next();
                            break;
                        }
                        Some(_) => {
                            let expr = parse_expr(stream);
                            items.push(expr);
                            match stream.peek() {
                                Some(Token::Comma) => {
                                    stream.next();
                                    continue;
                                }
                                Some(Token::RBracket) => {
                                    stream.next();
                                    break;
                                }
                                a => panic!("expected ',' or ']' in list, got {a:?}"),
                            }
                        }
                        _ => panic!("unexpected EOF in list literal"),
                    }
                }
                Expr::List(items)
            }
            Token::Identifier(value) if value != "in" && value != "let" => {
                Expr::Ident(value.clone())
            }
            Token::LParen => {
                let expr = parse_expr(stream);
                stream.next();
                expr
            }
            _ => Expr::None,
        },
        _ => Expr::None,
    }
}

pub fn parse_factor(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_atom(stream);

    while let Some(peek) = stream.peek() {
        if !peek.is_factor_op() {
            break;
        }
        let op = stream.next().unwrap().clone();
        let rhs = parse_atom(stream);
        lhs = Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        };
    }

    lhs
}

pub fn parse_term(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_factor(stream);

    while let Some(peek) = stream.peek() {
        if !peek.is_term_op() {
            break;
        }
        let op = stream.next().unwrap().clone();
        let rhs = parse_factor(stream);
        lhs = Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        };
    }

    lhs
}

pub fn parse_cmp(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_term(stream);

    loop {
        match stream.peek() {
            Some(Token::DoubleEqual)
            | Some(Token::NotEqual)
            | Some(Token::Greater)
            | Some(Token::Less)
            | Some(Token::GreaterEqual)
            | Some(Token::LessEqual) => {
                let op = stream.next().unwrap().clone();
                let rhs = parse_term(stream);
                lhs = Expr::Binary {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                };
                continue;
            }
            _ => break,
        }
    }

    lhs
}

fn eat(stream: &mut Peekable<Iter<Token>>, token: Token) {
    let next = stream.next();
    if next.is_some_and(|t| t != &token) {
        panic!("failed to eat a {token:?}, got {next:?}")
    }
}

pub fn parse_expr(stream: &mut Peekable<Iter<Token>>) -> Expr {
    match stream.peek() {
        Some(token) => match token {
            Token::Identifier(ident) if ident == "let" => {
                eat(stream, Token::Identifier(String::from("let")));
                let ident = stream
                    .next()
                    .and_then(|t| match t {
                        Token::Identifier(ident) => Some(ident.clone()),
                        a => panic!("expected an ident, got {a:?}"),
                    })
                    .unwrap();
                eat(stream, Token::Equal);
                let value = Box::new(parse_expr(stream));
                eat(stream, Token::Identifier(String::from("in")));
                let expr = Box::new(parse_expr(stream));
                return Expr::Let { ident, value, expr };
            }
            Token::BackSlash => {
                eat(stream, Token::BackSlash);
                let ident = stream
                    .next()
                    .and_then(|t| match t {
                        Token::Identifier(ident) => Some(ident.clone()),
                        _ => None,
                    })
                    .expect("expected an ident");

                let mut default: Option<Expr> = None;
                if let Some(Token::Question) = stream.peek() {
                    stream.next();
                    let def_expr = parse_term(stream);
                    default = Some(def_expr);
                }

                let expr = parse_expr(stream);

                return Expr::Function {
                    ident,
                    default: default.map(|d| Box::new(d)),
                    expr: Box::new(expr),
                };
            }
            Token::Path(chunks) => {
                let path_chunks = chunks.clone();
                stream.next();
                match stream.peek() {
                    Some(Token::Template(template_chunks)) => {
                        stream.next();
                        return Expr::FileTemplate {
                            path: path_chunks.clone(),
                            template: template_chunks.clone(),
                        };
                    }
                    Some(Token::At) => {
                        stream.next();
                        match stream.next() {
                            Some(Token::Template(template_chunks)) => {
                                return Expr::FileTemplate {
                                    path: path_chunks.clone(),
                                    template: template_chunks.clone(),
                                };
                            }
                            a => panic!("expected a template after @, got {a:?}"),
                        }
                    }
                    _ => {
                        return Expr::Path(path_chunks);
                    }
                }
            }
            Token::Identifier(ident) if ident == "if" => {
                eat(stream, Token::Identifier(String::from("if")));
                let cond = Box::new(parse_expr(stream));
                eat(stream, Token::Identifier(String::from("then")));
                let t = Box::new(parse_expr(stream));
                eat(stream, Token::Identifier(String::from("else")));
                let f = Box::new(parse_expr(stream));
                return Expr::If { cond, t, f };
            }
            Token::Identifier(ident) if ident == "true" || ident == "false" => {
                let value = ident == "true";
                stream.next();
                return Expr::Bool(value);
            }
            _ => {}
        },
        _ => {}
    }

    let lhs = parse_cmp(stream);

    let mut lhs = lhs;
    loop {
        match stream.peek() {
            Some(Token::Int(_))
            | Some(Token::Float(_))
            | Some(Token::String(_))
            | Some(Token::Template(_))
            | Some(Token::LParen)
            | Some(Token::LBracket)
            | Some(Token::Path(_))
            | Some(Token::BackSlash) => {
                let rhs = parse_term(stream);
                lhs = Expr::Application {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            }
            Some(Token::Identifier(ident)) => {
                if ident != "in" && ident != "let" && ident != "then" && ident != "else" {
                    let rhs = parse_term(stream);
                    lhs = Expr::Application {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                    continue;
                }
                break;
            }
            _ => break,
        }
    }

    lhs
}

pub fn parse(input: Vec<Token>) -> Ast {
    let mut stream = input.iter().peekable();
    Ast {
        program: parse_expr(&mut stream),
    }
}
