use crate::common::{Ast, EvalError, Expr, Token};
use std::iter::Peekable;
use std::slice::Iter;

type ParseResult<T> = Result<T, EvalError>;

pub fn parse_postfix(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut expr = parse_atom(stream);

    // Handle member access (dot notation)
    while let Some(Token::Dot) = stream.peek() {
        stream.next(); // consume the dot
        if let Some(Token::Identifier(field)) = stream.peek() {
            let field_name = field.clone();
            stream.next(); // consume the identifier
            expr = Expr::Member {
                object: Box::new(expr),
                field: field_name,
            };
        } else {
            eprintln!("Parse error: expected identifier after '.'");
            return Expr::None;
        }
    }

    expr
}

pub fn parse_atom(stream: &mut Peekable<Iter<Token>>) -> Expr {
    match stream.next() {
        Some(atom) => match atom {
            Token::Int(value) => Expr::Number(crate::common::Number::from(*value)),
            Token::Float(value) => Expr::Number(crate::common::Number::from_f64(*value)),
            Token::String(value) => Expr::String(value.clone()),
            Token::Template(chunks) => Expr::Template(chunks.clone()),
            Token::Path(chunks) => Expr::Path(chunks.clone()),
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
                                a => {
                                    eprintln!(
                                        "Parse error: expected ',' or ']' in list, got {:?}",
                                        a
                                    );
                                    return Expr::None;
                                }
                            }
                        }
                        None => {
                            eprintln!("Parse error: unexpected end of input in list literal");
                            return Expr::None;
                        }
                    }
                }
                Expr::List(items)
            }
            Token::LBrace => {
                // Parse dict literal: {key:value, key:value, ...}
                let mut pairs = Vec::new();
                loop {
                    match stream.peek() {
                        Some(Token::RBrace) => {
                            stream.next();
                            break;
                        }
                        Some(Token::Identifier(key)) => {
                            let key_name = key.clone();
                            stream.next();

                            // Expect colon
                            match stream.next() {
                                Some(Token::Colon) => {}
                                other => {
                                    eprintln!(
                                        "Parse error: expected ':' after dict key, got {:?}",
                                        other
                                    );
                                    return Expr::None;
                                }
                            }

                            // Parse value expression
                            // We need to handle lambdas, so use try_parse_expr but don't error on failure
                            let value = match try_parse_expr(stream) {
                                Ok(expr) => expr,
                                Err(_) => {
                                    // Fall back to parse_logic if try_parse_expr fails
                                    parse_logic(stream)
                                }
                            };
                            pairs.push((key_name, value));

                            // Check for comma or closing brace
                            match stream.peek() {
                                Some(Token::Comma) => {
                                    stream.next();
                                    // Check if there's another entry or if this is a trailing comma
                                    match stream.peek() {
                                        Some(Token::RBrace) => {
                                            stream.next();
                                            break;
                                        }
                                        Some(_) => continue,
                                        None => {
                                            eprintln!(
                                                "Parse error: unexpected EOF in dict literal"
                                            );
                                            return Expr::None;
                                        }
                                    }
                                }
                                Some(Token::RBrace) => {
                                    stream.next();
                                    break;
                                }
                                other => {
                                    eprintln!(
                                        "Parse error: expected ',' or '}}' in dict, got {:?}",
                                        other
                                    );
                                    return Expr::None;
                                }
                            }
                        }
                        None => {
                            eprintln!("Parse error: unexpected end of input in dict literal");
                            return Expr::None;
                        }
                        other => {
                            eprintln!(
                                "Parse error: expected identifier as dict key, got {:?}",
                                other
                            );
                            return Expr::None;
                        }
                    }
                }
                Expr::Dict(pairs)
            }
            Token::Identifier(value) if value != "in" && value != "let" => match value.as_str() {
                "true" => Expr::Bool(true),
                "false" => Expr::Bool(false),
                _ => Expr::Ident(value.clone()),
            },
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
    let mut lhs = parse_postfix(stream);

    while let Some(peek) = stream.peek() {
        if !peek.is_factor_op() {
            break;
        }
        // Safe: we just peeked and confirmed there's a token
        let op = stream.next().expect("token exists after peek").clone();
        let rhs = parse_postfix(stream);
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
        // Safe: we just peeked and confirmed there's a token
        let op = stream.next().expect("token exists after peek").clone();
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
                // Safe: we just peeked and confirmed there's a token
                let op = stream.next().expect("token exists after peek").clone();
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


pub fn parse_logic(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_cmp(stream);

    loop {
        match stream.peek() {
            Some(Token::And) | Some(Token::Or) => {
                // Safe: we just peeked and confirmed there's a token
                let op = stream.next().expect("token exists after peek").clone();
                let rhs = parse_cmp(stream);
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
fn eat(stream: &mut Peekable<Iter<Token>>, token: Token) -> ParseResult<()> {
    let next = stream.next();
    if let Some(t) = next {
        if t != &token {
            return Err(EvalError::new(
                format!("parse error: expected {:?}, found {:?}", token, t),
                Some(format!("{:?}", token)),
                Some(format!("{:?}", t)),
                1,
            ));
        }
    } else {
        return Err(EvalError::new(
            format!("parse error: expected {:?}, found end of input", token),
            Some(format!("{:?}", token)),
            Some("end of input".to_string()),
            1,
        ));
    }
    Ok(())
}

pub fn parse_expr(stream: &mut Peekable<Iter<Token>>) -> Expr {
    match try_parse_expr(stream) {
        Ok(expr) => expr,
        Err(e) => {
            eprintln!("Parse error: {}", e.message);
            std::process::exit(1);
        }
    }
}

fn try_parse_expr(stream: &mut Peekable<Iter<Token>>) -> ParseResult<Expr> {
    match stream.peek() {
        Some(token) => match token {
            Token::Identifier(ident) if ident == "let" => {
                eat(stream, Token::Identifier(String::from("let")))?;
                let ident = stream
                    .next()
                    .and_then(|t| match t {
                        Token::Identifier(ident) => Some(ident.clone()),
                        a => {
                            eprintln!("Parse error: expected identifier after 'let', got {:?}", a);
                            None
                        }
                    })
                    .ok_or_else(|| {
                        EvalError::new(
                            "expected identifier after 'let'",
                            Some("identifier".to_string()),
                            Some("something else".to_string()),
                            1,
                        )
                    })?;
                eat(stream, Token::Equal)?;
                let value = Box::new(try_parse_expr(stream)?);
                eat(stream, Token::Identifier(String::from("in")))?;
                let expr = Box::new(try_parse_expr(stream)?);
                return Ok(Expr::Let { ident, value, expr });
            }
            Token::BackSlash => {
                eat(stream, Token::BackSlash)?;
                let ident = stream
                    .next()
                    .and_then(|t| match t {
                        Token::Identifier(ident) => Some(ident.clone()),
                        _ => None,
                    })
                    .ok_or_else(|| {
                        EvalError::new(
                            "expected identifier after '\\'",
                            Some("identifier".to_string()),
                            None,
                            1,
                        )
                    })?;

                let mut default: Option<Expr> = None;
                if let Some(Token::Question) = stream.peek() {
                    stream.next();
                    let def_expr = parse_term(stream);
                    default = Some(def_expr);
                }

                let expr = try_parse_expr(stream)?;

                return Ok(Expr::Function {
                    ident,
                    default: default.map(|d| Box::new(d)),
                    expr: Box::new(expr),
                });
            }
            Token::Path(chunks) => {
                let path_chunks = chunks.clone();
                stream.next();
                match stream.peek() {
                    Some(Token::Template(template_chunks)) => {
                        stream.next();
                        return Ok(Expr::FileTemplate {
                            path: path_chunks.clone(),
                            template: template_chunks.clone(),
                        });
                    }
                    Some(Token::At) => {
                        stream.next();
                        match stream.next() {
                            Some(Token::Template(template_chunks)) => {
                                return Ok(Expr::FileTemplate {
                                    path: path_chunks.clone(),
                                    template: template_chunks.clone(),
                                });
                            }
                            a => {
                                return Err(EvalError::new(
                                    format!("expected template after '@', got {:?}", a),
                                    Some("template string".to_string()),
                                    Some(format!("{:?}", a)),
                                    1,
                                ))
                            }
                        }
                    }
                    _ => {
                        return Ok(Expr::Path(path_chunks));
                    }
                }
            }
            Token::Identifier(ident) if ident == "if" => {
                eat(stream, Token::Identifier(String::from("if")))?;
                let cond = Box::new(try_parse_expr(stream)?);
                eat(stream, Token::Identifier(String::from("then")))?;
                let t = Box::new(try_parse_expr(stream)?);
                eat(stream, Token::Identifier(String::from("else")))?;
                let f = Box::new(try_parse_expr(stream)?);
                return Ok(Expr::If { cond, t, f });
            }
            Token::Identifier(ident) if ident == "true" || ident == "false" => {
                let value = ident == "true";
                stream.next();
                return Ok(Expr::Bool(value));
            }
            _ => {}
        },
        _ => {}
    }

    let lhs = parse_pipe(stream);
    Ok(lhs)
}

fn parse_pipe(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_app(stream);

    loop {
        match stream.peek() {
            Some(Token::Pipe) => {
                // Handle pipe operator: lhs -> rhs
                stream.next(); // consume the pipe token
                let rhs = parse_app(stream);
                lhs = Expr::Pipe {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            }
            _ => break,
        }
    }

    lhs
}

fn parse_app(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_logic(stream);

    loop {
        match stream.peek() {
            Some(Token::Identifier(id)) if id != "in" && id != "then" && id != "else" => {
                let rhs = parse_term(stream);
                lhs = Expr::Application {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            }
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
