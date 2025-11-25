use crate::common::{Chunk, EvalError, Token};
use std::iter::Peekable;
use std::str::Chars;

pub fn identifier(next: char, stream: &mut Peekable<Chars<'_>>) -> Token {
    let mut ident = String::new();
    ident.push(next);
    loop {
        let Some(peek) = stream.peek().clone() else {
            break;
        };
        if peek.is_whitespace() || (!peek.is_alphanumeric() && *peek != '_') {
            break;
        }
        // Safe: we just peeked and confirmed there's a character
        ident.push(stream.next().expect("character exists after peek"));
    }
    Token::Identifier(ident)
}

pub fn string(stream: &mut Peekable<Chars<'_>>) -> Result<Token, EvalError> {
    let mut string = String::new();
    loop {
        let next_opt = stream.next();
        match next_opt {
            None => return Err(EvalError::new("unterminated string", None, None, 0)),
            Some('\\') => {
                // Handle escape sequences
                match stream.next() {
                    Some('n') => string.push('\n'),
                    Some('t') => string.push('\t'),
                    Some('r') => string.push('\r'),
                    Some('\\') => string.push('\\'),
                    Some('"') => string.push('"'),
                    Some(c) => {
                        // Unknown escape, keep as-is
                        string.push('\\');
                        string.push(c);
                    }
                    None => {
                        return Err(EvalError::new(
                            "unterminated string (backslash at end)",
                            None,
                            None,
                            0,
                        ))
                    }
                }
            }
            Some('"') => break,
            Some(next) => string.push(next),
        }
    }
    Ok(Token::String(string))
}

pub fn chunk(stream: &mut Peekable<Chars<'_>>) -> Result<Token, EvalError> {
    let mut open_count = 1;
    while matches!(stream.peek(), Some('{')) {
        stream.next();
        open_count += 1;
    }

    while matches!(stream.peek(), Some(c) if c.is_whitespace()) {
        stream.next();
    }

    match stream.next() {
        Some('"') => {}
        _ => {
            return Err(EvalError::new(
                "expected '\"' after opening braces",
                None,
                None,
                0,
            ))
        }
    }

    let mut chunks = Vec::<Chunk>::new();
    let mut cur = String::new();

    while let Some(ch) = stream.next() {
        if ch == '"' {
            // Check if this quote is followed by exactly open_count closing braces
            let mut matched_braces = 0;
            for _ in 0..open_count {
                if let Some('}') = stream.peek().cloned() {
                    stream.next();
                    matched_braces += 1;
                } else {
                    break;
                }
            }

            // Only close the template if we found all the required braces
            if matched_braces == open_count {
                if !cur.is_empty() {
                    chunks.push(Chunk::String(cur));
                }
                return Ok(Token::Template(chunks));
            } else {
                // Not a closing sequence, treat as literal quote and put back the braces
                cur.push('"');
                for _ in 0..matched_braces {
                    cur.push('}');
                }
            }
            continue;
        }

        if ch == '{' {
            // Count consecutive opening braces
            let mut brace_count = 1;
            while let Some('{') = stream.peek().cloned() {
                stream.next();
                brace_count += 1;
            }

            if brace_count == open_count {
                // Start interpolation
                if !cur.is_empty() {
                    chunks.push(Chunk::String(cur));
                    cur = String::new();
                }

                let mut expr = String::new();
                loop {
                    match stream.next() {
                        Some(c2) => {
                            if c2 == '}' {
                                // Count closing braces to match interpolation terminator
                                let mut got = 1;
                                for _ in 0..(open_count - 1) {
                                    if let Some('}') = stream.peek().cloned() {
                                        stream.next();
                                        got += 1;
                                    } else {
                                        break;
                                    }
                                }
                                if got == open_count {
                                    break; // end interpolation
                                } else {
                                    // Not enough to terminate; treat collected as literal '}' chars inside expr
                                    expr.push_str(&"}".repeat(got));
                                    continue;
                                }
                            } else {
                                expr.push(c2);
                            }
                        }
                        None => {
                            return Err(EvalError::new(
                                "unexpected EOF inside template interpolation",
                                None,
                                None,
                                0,
                            ))
                        }
                    }
                }
                chunks.push(Chunk::Expr(expr));
                continue;
            } else if brace_count == open_count + 1 {
                // Escape hatch: one literal '{'
                cur.push('{');
                continue;
            } else if brace_count > open_count + 1 {
                // For K > open_count+1 output (K - open_count) literal '{' characters.
                for _ in 0..(brace_count - open_count) {
                    cur.push('{');
                }
                continue;
            } else {
                // brace_count < open_count (should not normally happen) -> treat all as literal
                for _ in 0..brace_count {
                    cur.push('{');
                }
                continue;
            }
        }

        if ch == '}' {
            // Count consecutive closing braces for escape hatch in literals.
            let mut brace_count = 1;
            while let Some('}') = stream.peek().cloned() {
                stream.next();
                brace_count += 1;
            }
            if brace_count == open_count + 1 {
                cur.push('}');
                continue;
            } else if brace_count > open_count + 1 {
                for _ in 0..(brace_count - open_count) {
                    cur.push('}');
                }
                continue;
            } else if open_count == 1 && brace_count == 2 {
                // Legacy special-case (covered above, but retained for clarity)
                cur.push('}');
                continue;
            } else {
                for _ in 0..brace_count {
                    cur.push('}');
                }
                continue;
            }
        }

        cur.push(ch);
    }

    Err(EvalError::new(
        "unexpected EOF inside template",
        None,
        None,
        0,
    ))
}

pub fn path(stream: &mut Peekable<Chars<'_>>) -> Result<Token, EvalError> {
    let mut cur = String::new();
    let mut chunks = Vec::<Chunk>::new();

    loop {
        let c_opt = stream.next();
        match c_opt {
            None => {
                if !cur.is_empty() {
                    chunks.push(Chunk::String(cur));
                }
                return Ok(Token::Path(chunks));
            }
            Some(c) => {
                if c.is_whitespace() {
                    if !cur.is_empty() {
                        chunks.push(Chunk::String(cur));
                    }
                    return Ok(Token::Path(chunks));
                }

                if c == '{' {
                    if !cur.is_empty() {
                        chunks.push(Chunk::String(cur));
                        cur = String::new();
                    }

                    let mut expr = String::new();
                    loop {
                        match stream.next() {
                            None => {
                                return Err(EvalError::new(
                                    "EOF in path interpolation",
                                    None,
                                    None,
                                    0,
                                ))
                            }
                            Some(ch2) => {
                                if ch2 == '}' {
                                    break;
                                } else {
                                    expr.push(ch2);
                                }
                            }
                        }
                    }

                    chunks.push(Chunk::Expr(expr));
                    continue;
                }

                cur.push(c);
            }
        }
    }
}

pub fn number(next: char, stream: &mut Peekable<Chars<'_>>) -> Result<Token, EvalError> {
    let mut number = String::new();
    number.push(next);
    loop {
        let Some(peek) = stream.peek().clone() else {
            break;
        };
        if peek.is_whitespace() || !peek.is_numeric() {
            break;
        }
        // Safe: we just peeked and confirmed there's a character
        number.push(stream.next().expect("character exists after peek"));
    }

    let Some(peek) = stream.peek() else {
        let number: i64 = number.parse().unwrap_or_default();
        return Ok(Token::Int(number));
    };

    if peek == &'.' {
        // Safe: we just checked peek == '.'
        number.push(stream.next().expect("'.' character exists after peek"));
        loop {
            let Some(peek) = stream.peek().clone() else {
                break;
            };
            if peek.is_whitespace() || !peek.is_numeric() {
                break;
            }
            // Safe: we just peeked and confirmed there's a character
            number.push(stream.next().expect("character exists after peek"));
        }
        let number: f64 = number.parse().unwrap_or_default();
        return Ok(Token::Float(number));
    }

    let number: i64 = number.parse().unwrap_or_default();
    Ok(Token::Int(number))
}

#[allow(dead_code, unreachable_code, unused_mut)]
pub fn tokenize(input: String) -> Result<Vec<Token>, EvalError> {
    let mut output = vec![];
    let mut stream = input.chars().peekable();
    loop {
        let Some(next) = stream.next() else {
            break;
        };

        macro_rules! checkfor {
            ($e:expr,$t:ident) => {
                if let Some($e) = stream.peek() {
                    stream.next();
                    output.push(Token::$t);
                    continue;
                }
            };
        }

        match next {
            'A'..='Z' | 'a'..='z' | '_' => {
                let ident = identifier(next, &mut stream);
                output.push(ident);
            }
            '0'..='9' => {
                let number = number(next, &mut stream)?;
                output.push(number);
            }
            '\"' => {
                let string = string(&mut stream)?;
                output.push(string);
            }
            '{' => {
                // Need to distinguish between template and dict syntax
                // Dict: {identifier:value, identifier:value, ...} or {}
                // Template: {expr}" or {{expr}} etc - must have " after braces

                // Clone the stream to peek ahead without consuming
                let mut temp_stream = stream.clone();
                let mut is_dict = false;

                // Skip any additional opening braces (for templates like {{...}})
                while let Some(&'{') = temp_stream.peek() {
                    temp_stream.next();
                }

                // Skip whitespace
                while let Some(&c) = temp_stream.peek() {
                    if c.is_whitespace() {
                        temp_stream.next();
                    } else {
                        break;
                    }
                }

                // Check if next is closing brace (empty dict {})
                if let Some(&'}') = temp_stream.peek() {
                    is_dict = true;
                } else if let Some(&c) = temp_stream.peek() {
                    // Check if it's an identifier followed by colon (dict)
                    if c.is_alphabetic() || c == '_' {
                        // Skip the identifier
                        while let Some(&ch) = temp_stream.peek() {
                            if ch.is_alphanumeric() || ch == '_' {
                                temp_stream.next();
                            } else {
                                break;
                            }
                        }

                        // Skip whitespace after identifier
                        while let Some(&ch) = temp_stream.peek() {
                            if ch.is_whitespace() {
                                temp_stream.next();
                            } else {
                                break;
                            }
                        }

                        // Check if next is colon (dict syntax)
                        if let Some(&':') = temp_stream.peek() {
                            is_dict = true;
                        }
                    }
                }

                if is_dict {
                    // Parse as dict - just output a single LBrace
                    output.push(Token::LBrace);
                } else {
                    // Parse as template - chunk() will handle all the brace counting
                    let chunk = chunk(&mut stream)?;
                    output.push(chunk);
                }
            }
            '[' => output.push(Token::LBracket),
            ']' => output.push(Token::RBracket),
            '}' => output.push(Token::RBrace),
            ',' => output.push(Token::Comma),
            ':' => output.push(Token::Colon),
            '.' => {
                checkfor!('.', DoubleDot);
                output.push(Token::Dot)
            }
            '=' => {
                checkfor!('=', DoubleEqual);
                output.push(Token::Equal)
            }
            '@' => {
                let chunk = path(&mut stream)?;
                output.push(chunk);
            }
            '#' => {
                while let Some(&c) = stream.peek() {
                    stream.next();
                    if c == '\n' {
                        break;
                    }
                }
                continue;
            }
            '!' => {
                if let Some('=') = stream.peek() {
                    stream.next();
                    output.push(Token::NotEqual);
                    continue;
                }
            }
            '>' => {
                if let Some('=') = stream.peek() {
                    stream.next();
                    output.push(Token::GreaterEqual);
                    continue;
                }
                output.push(Token::Greater);
            }
            '<' => {
                if let Some('=') = stream.peek() {
                    stream.next();
                    output.push(Token::LessEqual);
                    continue;
                }
                output.push(Token::Less);
            }
            '+' => output.push(Token::Add),
            // not sure what to do with single '&' and '|', so only handle double versions
            '&' => {
                if let Some('&') = stream.peek() {
                    stream.next();
                    output.push(Token::And);
                    continue;
                }
            }
            '|' => {
                if let Some('|') = stream.peek() {
                    stream.next();
                    output.push(Token::Or);
                    continue;
                }
            }
            '-' => {
                // Check for pipe operator ->
                if let Some('>') = stream.peek() {
                    stream.next();
                    output.push(Token::Pipe);
                    continue;
                }
                output.push(Token::Sub)
            }
            '/' => output.push(Token::Div),
            '*' => output.push(Token::Mul),
            '(' => output.push(Token::LParen),
            ')' => output.push(Token::RParen),
            '\\' => output.push(Token::BackSlash),
            '?' => output.push(Token::Question),
            _ => {}
        }
    }
    Ok(output)
}
