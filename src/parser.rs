use crate::common::{Ast, EvalError, Expr, Token};
use std::iter::Peekable;
use std::slice::Iter;

type ParseResult<T> = Result<T, EvalError>;

pub fn parse_postfix(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut expr = parse_atom(stream);

    // Handle member access (dot notation)
    while let Some(Token::Dot(line)) = stream.peek() {
        let line = *line;
        stream.next(); // consume the dot
        if let Some(Token::Identifier(field, _)) = stream.peek() {
            let field_name = field.clone();
            stream.next(); // consume the identifier
            expr = Expr::Member {
                object: Box::new(expr),
                field: field_name,
                line,
            };
        } else {
            eprintln!("Parse error: expected identifier after '.'");
            return Expr::None(line);
        }
    }

    expr
}

pub fn parse_atom(stream: &mut Peekable<Iter<Token>>) -> Expr {
    match stream.next() {
        Some(atom) => {
            match atom {
                Token::Int(value, line) => Expr::Number(crate::common::Number::from(*value), *line),
                Token::Float(value, line) => {
                    Expr::Number(crate::common::Number::from_f64(*value), *line)
                }
                Token::String(value, line) => Expr::String(value.clone(), *line),
                Token::Template(chunks, line) => Expr::Template(chunks.clone(), *line),
                Token::Path(chunks, line) => Expr::Path(chunks.clone(), *line),
                Token::LBracket(line) => {
                    let line = *line;
                    // Check for empty list
                    if let Some(Token::RBracket(_)) = stream.peek() {
                        stream.next();
                        return Expr::List(Vec::new(), line);
                    }

                    // Parse first expression
                    let first_expr = parse_expr(stream);

                    // Check for range syntax: [start..end] or [start, step..end]
                    match stream.peek() {
                        Some(Token::DoubleDot(_)) => {
                            // Simple range: [start..end]
                            stream.next(); // consume ..
                            let end_expr = parse_expr(stream);
                            match stream.peek() {
                                Some(Token::RBracket(_)) => {
                                    stream.next(); // consume ]
                                    Expr::Range {
                                        start: Box::new(first_expr),
                                        step: None,
                                        end: Box::new(end_expr),
                                        line,
                                    }
                                }
                                other => {
                                    eprintln!(
                                        "Parse error: expected ']' after range end, got {:?}",
                                        other
                                    );
                                    Expr::None(line)
                                }
                            }
                        }
                        Some(Token::Comma(_)) => {
                            // Could be [start, step..end] or regular list [start, second, ...]
                            stream.next(); // consume comma

                            // Check for trailing comma after first element: [first,]
                            if let Some(Token::RBracket(_)) = stream.peek() {
                                stream.next();
                                return Expr::List(vec![first_expr], line);
                            }

                            // Parse the expression after comma (could be step or second element)
                            let second_expr = parse_expr(stream);
                            // Check if next is DoubleDot (range with step) or something else (regular list)
                            match stream.peek() {
                                Some(Token::DoubleDot(_)) => {
                                    // Range with step: [start, step..end]
                                    stream.next(); // consume ..
                                    let end_expr = parse_expr(stream);
                                    match stream.peek() {
                                        Some(Token::RBracket(_)) => {
                                            stream.next(); // consume ]
                                            Expr::Range {
                                                start: Box::new(first_expr),
                                                step: Some(Box::new(second_expr)),
                                                end: Box::new(end_expr),
                                                line,
                                            }
                                        }
                                        other => {
                                            eprintln!("Parse error: expected ']' after range end, got {:?}", other);
                                            Expr::None(line)
                                        }
                                    }
                                }
                                _ => {
                                    // Regular list: [first, second, ...]
                                    let mut items = vec![first_expr, second_expr];

                                    // Continue parsing rest of list
                                    loop {
                                        match stream.peek() {
                                            Some(Token::Comma(_)) => {
                                                stream.next();
                                                // Check for trailing comma
                                                if let Some(Token::RBracket(_)) = stream.peek() {
                                                    stream.next();
                                                    break;
                                                }
                                                let next_expr = parse_expr(stream);
                                                items.push(next_expr);
                                                continue;
                                            }
                                            Some(Token::RBracket(_)) => {
                                                stream.next();
                                                break;
                                            }
                                            a => {
                                                eprintln!(
                                                "Parse error: expected ',' or ']' in list, got {:?}",
                                                a
                                            );
                                                return Expr::None(line);
                                            }
                                        }
                                    }
                                    Expr::List(items, line)
                                }
                            }
                        }
                        Some(Token::RBracket(_)) => {
                            // Single element list: [expr]
                            stream.next();
                            Expr::List(vec![first_expr], line)
                        }
                        a => {
                            eprintln!(
                            "Parse error: expected ',', '..', or ']' after list element, got {:?}",
                            a
                        );
                            Expr::None(line)
                        }
                    }
                }
                Token::LBrace(line) => {
                    let line = *line;
                    // Parse dict literal: {key:value, key:value, ...}
                    let mut pairs = Vec::new();
                    loop {
                        match stream.peek() {
                            Some(Token::RBrace(_)) => {
                                stream.next();
                                break;
                            }
                            Some(Token::Identifier(key, _)) => {
                                let key_name = key.clone();
                                stream.next();

                                // Expect colon
                                match stream.next() {
                                    Some(Token::Colon(_)) => {}
                                    other => {
                                        eprintln!(
                                            "Parse error: expected ':' after dict key, got {:?}",
                                            other
                                        );
                                        return Expr::None(line);
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
                                    Some(Token::Comma(_)) => {
                                        stream.next();
                                        // Check if there's another entry or if this is a trailing comma
                                        match stream.peek() {
                                            Some(Token::RBrace(_)) => {
                                                stream.next();
                                                break;
                                            }
                                            Some(_) => continue,
                                            None => {
                                                eprintln!(
                                                    "Parse error: unexpected EOF in dict literal"
                                                );
                                                return Expr::None(line);
                                            }
                                        }
                                    }
                                    Some(Token::RBrace(_)) => {
                                        stream.next();
                                        break;
                                    }
                                    other => {
                                        eprintln!(
                                            "Parse error: expected ',' or '}}' in dict, got {:?}",
                                            other
                                        );
                                        return Expr::None(line);
                                    }
                                }
                            }
                            None => {
                                eprintln!("Parse error: unexpected end of input in dict literal");
                                return Expr::None(line);
                            }
                            other => {
                                eprintln!(
                                    "Parse error: expected identifier as dict key, got {:?}",
                                    other
                                );
                                return Expr::None(line);
                            }
                        }
                    }
                    Expr::Dict(pairs, line)
                }
                Token::Identifier(value, line) if value != "in" && value != "let" => {
                    match value.as_str() {
                        "true" => Expr::Bool(true, *line),
                        "false" => Expr::Bool(false, *line),
                        "none" => Expr::None(*line),
                        _ => Expr::Ident(value.clone(), *line),
                    }
                }
                Token::LParen(_) => {
                    let expr = parse_expr(stream);
                    stream.next();
                    expr
                }
                Token::RParen(line) => {
                    eprintln!(
                        "Parse error: unexpected closing parenthesis on line {}",
                        line
                    );
                    Expr::None(*line)
                }
                Token::RBracket(line) => {
                    eprintln!("Parse error: unexpected closing bracket on line {}", line);
                    Expr::None(*line)
                }
                Token::RBrace(line) => {
                    eprintln!("Parse error: unexpected closing brace on line {}", line);
                    Expr::None(*line)
                }
                t => Expr::None(t.line()),
            }
        }
        None => Expr::None(0),
    }
}

pub fn parse_factor(stream: &mut Peekable<Iter<Token>>) -> Expr {
    // Handle unary minus for negative numbers
    if let Some(Token::Sub(line)) = stream.peek() {
        let line = *line;
        stream.next(); // consume the minus
        let expr = parse_postfix(stream);
        let mut lhs = Expr::Binary {
            lhs: Box::new(Expr::Number(crate::common::Number::from(0), line)),
            op: Token::Sub(line),
            rhs: Box::new(expr),
            line,
        };

        // Continue with factor operations
        while let Some(peek) = stream.peek() {
            if !peek.is_factor_op() {
                break;
            }
            // Safe: we just peeked and confirmed there's a token
            let op = stream.next().expect("token exists after peek").clone();
            let line = op.line();
            let rhs = parse_postfix(stream);
            lhs = Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
                line,
            };
        }

        return lhs;
    }

    let mut lhs = parse_postfix(stream);

    while let Some(peek) = stream.peek() {
        if !peek.is_factor_op() {
            break;
        }
        // Safe: we just peeked and confirmed there's a token
        let op = stream.next().expect("token exists after peek").clone();
        let line = op.line();
        let rhs = parse_postfix(stream);
        lhs = Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
            line,
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
        let line = op.line();
        let rhs = parse_factor(stream);
        lhs = Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
            line,
        };
    }

    lhs
}

pub fn parse_cmp(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_term(stream);

    while let Some(Token::DoubleEqual(_))
    | Some(Token::NotEqual(_))
    | Some(Token::Greater(_))
    | Some(Token::Less(_))
    | Some(Token::GreaterEqual(_))
    | Some(Token::LessEqual(_)) = stream.peek()
    {
        // Safe: we just peeked and confirmed there's a token
        let op = stream.next().expect("token exists after peek").clone();
        let line = op.line();
        let rhs = parse_term(stream);
        lhs = Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
            line,
        };
    }

    lhs
}

pub fn parse_and(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_app(stream);

    while let Some(Token::And(line)) = stream.peek() {
        let line = *line;
        let op = stream.next().expect("token exists after peek").clone();
        let rhs = parse_app(stream);
        lhs = Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
            line,
        };
        continue;
    }

    lhs
}

pub fn parse_or(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_and(stream);

    while let Some(Token::Or(line)) = stream.peek() {
        let line = *line;
        let op = stream.next().expect("token exists after peek").clone();
        let rhs = parse_and(stream);
        lhs = Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
            line,
        };
        continue;
    }

    lhs
}

pub fn parse_logic(stream: &mut Peekable<Iter<Token>>) -> Expr {
    parse_or(stream)
}

fn eat(stream: &mut Peekable<Iter<Token>>, token: Token) -> ParseResult<()> {
    let next = stream.next();
    if let Some(t) = next {
        // Match token variants ignoring line info
        let match_ = match (t, &token) {
            (Token::Identifier(s1, _), Token::Identifier(s2, _)) => s1 == s2,
            (Token::Equal(_), Token::Equal(_)) => true,
            (Token::BackSlash(_), Token::BackSlash(_)) => true,
            _ => false,
        };

        if !match_ {
            return Err(EvalError::new(
                format!("parse error: expected {:?}, found {:?}", token, t),
                Some(format!("{:?}", token)),
                Some(format!("{:?}", t)),
                t.line(),
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
    // First check for lambda or let or path/template literals that start with keywords or specific tokens
    // We peek first to avoid consuming if we need to delegate

    let peek_token = stream.peek().cloned();

    if let Some(token) = peek_token {
        match token {
            Token::Identifier(ident, line) if ident == "let" => {
                let line = *line;
                eat(stream, Token::Identifier(String::from("let"), 0))?;
                let ident = stream
                    .next()
                    .and_then(|t| match t {
                        Token::Identifier(ident, _) => Some(ident.clone()),
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
                            line,
                        )
                    })?;
                eat(stream, Token::Equal(0))?;
                let value = Box::new(try_parse_expr(stream)?);
                eat(stream, Token::Identifier(String::from("in"), 0))?;
                let expr = Box::new(try_parse_expr(stream)?);
                return Ok(Expr::Let {
                    ident,
                    value,
                    expr,
                    line,
                });
            }
            Token::BackSlash(line) => {
                let line = *line;
                eat(stream, Token::BackSlash(0))?;
                let ident = stream
                    .next()
                    .and_then(|t| match t {
                        Token::Identifier(ident, _) => Some(ident.clone()),
                        _ => None,
                    })
                    .ok_or_else(|| {
                        EvalError::new(
                            "expected identifier after '\\'",
                            Some("identifier".to_string()),
                            None,
                            line,
                        )
                    })?;

                let mut default: Option<Expr> = None;
                if let Some(Token::Question(_)) = stream.peek() {
                    stream.next();
                    let def_expr = parse_term(stream);
                    default = Some(def_expr);
                }

                let expr = try_parse_expr(stream)?;

                return Ok(Expr::Function {
                    ident,
                    default: default.map(Box::new),
                    expr: Box::new(expr),
                    line,
                });
            }
            Token::Path(chunks, line) => {
                let line = *line;
                let path_chunks = chunks.clone();
                stream.next();
                match stream.peek() {
                    Some(Token::Template(template_chunks, _)) => {
                        stream.next();
                        return Ok(Expr::FileTemplate {
                            path: path_chunks.clone(),
                            template: template_chunks.clone(),
                            line,
                        });
                    }
                    Some(Token::At(_)) => {
                        stream.next();
                        match stream.next() {
                            Some(Token::Template(template_chunks, _)) => {
                                return Ok(Expr::FileTemplate {
                                    path: path_chunks.clone(),
                                    template: template_chunks.clone(),
                                    line,
                                });
                            }
                            a => {
                                return Err(EvalError::new(
                                    format!("expected template after '@', got {:?}", a),
                                    Some("template string".to_string()),
                                    Some(format!("{:?}", a)),
                                    line,
                                ))
                            }
                        }
                    }
                    _ => {
                        // Check if this path is part of a pipe or other expression
                        // We've consumed the token, so we return the path expr directly
                        // but we need to continue parsing if it's part of a larger expression.
                        // The original logic returned immediately, preventing `path -> function`.
                        // However, `try_parse_expr` is expected to parse the *whole* expression if it starts with these tokens.

                        // If we return here, we miss the pipe logic at the end of the function.
                        // We should construct the LHS and then fall through to `parse_pipe` logic logic below,
                        // BUT `try_parse_expr` structure is a bit rigid.

                        // A better approach: parse the prefix part, then continue.
                        let lhs = Expr::Path(path_chunks, line);
                        return Ok(parse_pipe_suffix(stream, lhs));
                    }
                }
            }
            Token::Identifier(ident, line) if ident == "if" => {
                let line = *line;
                eat(stream, Token::Identifier(String::from("if"), 0))?;
                let cond = Box::new(try_parse_expr(stream)?);
                eat(stream, Token::Identifier(String::from("then"), 0))?;
                let t = Box::new(try_parse_expr(stream)?);
                eat(stream, Token::Identifier(String::from("else"), 0))?;
                let f = Box::new(try_parse_expr(stream)?);
                return Ok(Expr::If { cond, t, f, line });
            }
            _ => {}
        }
    }

    // If not one of the special prefix forms, parse as a pipe expression
    let lhs = parse_pipe(stream);
    Ok(lhs)
}

fn parse_pipe(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let lhs = parse_logic(stream);
    parse_pipe_suffix(stream, lhs)
}

fn parse_pipe_suffix(stream: &mut Peekable<Iter<Token>>, mut lhs: Expr) -> Expr {
    while let Some(Token::Pipe(line)) = stream.peek() {
        let line = *line;
        // Handle pipe operator: lhs -> rhs
        stream.next(); // consume the pipe token

        // Check if the RHS starts with a lambda (BackSlash)
        let rhs = if let Some(Token::BackSlash(_)) = stream.peek() {
            // Directly call try_parse_expr to handle the lambda
            // We handle errors by converting to Expr::None or propagating?
            // try_parse_expr returns Result.
            match try_parse_expr(stream) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Parse error in pipe RHS: {}", e.message);
                    std::process::exit(1);
                }
            }
        } else {
            parse_logic(stream)
        };

        lhs = Expr::Pipe {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            line,
        };
    }
    lhs
}

fn parse_app(stream: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_cmp(stream);

    loop {
        match stream.peek() {
            Some(Token::Identifier(id, _)) if id != "in" && id != "then" && id != "else" => {
                let rhs = parse_cmp(stream);
                let line = lhs.line();
                lhs = Expr::Application {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    line,
                };
            }
            Some(Token::Int(_, _))
            | Some(Token::Float(_, _))
            | Some(Token::String(_, _))
            | Some(Token::Template(_, _))
            | Some(Token::LParen(_))
            | Some(Token::LBracket(_))
            | Some(Token::LBrace(_))
            | Some(Token::Path(_, _))
            | Some(Token::BackSlash(_)) => {
                let rhs = parse_cmp(stream);
                let line = lhs.line();
                lhs = Expr::Application {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    line,
                };
            }
            Some(Token::Identifier(ident, _)) => {
                if ident != "in" && ident != "let" && ident != "then" && ident != "else" {
                    let rhs = parse_cmp(stream);
                    let line = lhs.line();
                    lhs = Expr::Application {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        line,
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
