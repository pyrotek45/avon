#![allow(clippy::result_large_err)] // EvalError is large but fundamental to the architecture

use crate::common::{Chunk, EvalError, Token};
use std::iter::Peekable;
use std::str::Chars;

pub fn identifier(next: char, stream: &mut Peekable<Chars<'_>>, line: usize) -> Token {
    let mut ident = String::new();
    ident.push(next);
    loop {
        let Some(peek) = stream.peek() else {
            break;
        };
        if peek.is_whitespace() || (!peek.is_alphanumeric() && *peek != '_') {
            break;
        }
        // Safe: we just peeked and confirmed there's a character
        ident.push(stream.next().expect("character exists after peek"));
    }
    Token::Identifier(ident, line)
}

pub fn string(stream: &mut Peekable<Chars<'_>>, line: &mut usize) -> Result<Token, EvalError> {
    let start_line = *line;
    let mut string = String::new();
    loop {
        let next_opt = stream.next();
        match next_opt {
            None => {
                return Err(EvalError::new(
                    "unterminated string",
                    None,
                    None,
                    start_line,
                ))
            }
            Some('\n') => {
                *line += 1;
                string.push('\n');
            }
            Some('\\') => {
                // Handle escape sequences
                match stream.next() {
                    Some('n') => string.push('\n'),
                    Some('t') => string.push('\t'),
                    Some('r') => string.push('\r'),
                    Some('\\') => string.push('\\'),
                    Some('"') => string.push('"'),
                    Some(c) => {
                        if c == '\n' {
                            *line += 1;
                        }
                        // Unknown escape, keep as-is
                        string.push('\\');
                        string.push(c);
                    }
                    None => {
                        return Err(EvalError::new(
                            "unterminated string (backslash at end)",
                            None,
                            None,
                            start_line,
                        ))
                    }
                }
            }
            Some('"') => break,
            Some(next) => string.push(next),
        }
    }
    Ok(Token::String(string, start_line))
}

pub fn chunk(stream: &mut Peekable<Chars<'_>>, line: &mut usize) -> Result<Token, EvalError> {
    let start_line = *line;
    let mut open_count = 1;
    while matches!(stream.peek(), Some('{')) {
        stream.next();
        open_count += 1;
    }

    while matches!(stream.peek(), Some(c) if c.is_whitespace()) {
        let c = stream.next().unwrap();
        if c == '\n' {
            *line += 1;
        }
    }

    match stream.next() {
        Some('"') => {}
        _ => {
            return Err(EvalError::new(
                "expected '\"' after opening braces",
                None,
                None,
                start_line,
            ))
        }
    }

    let mut chunks = Vec::<Chunk>::new();
    let mut cur = String::new();

    while let Some(ch) = stream.next() {
        if ch == '\n' {
            *line += 1;
        }

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
                return Ok(Token::Template(chunks, start_line));
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

                // Capture the line where the expression content actually starts
                let expr_line = *line;
                let mut expr = String::new();

                // Use a stack to track nested template structures
                // Each entry is (brace_count, is_in_interpolation)
                let mut template_stack: Vec<(usize, bool)> = Vec::new();

                loop {
                    match stream.next() {
                        Some(c2) => {
                            if c2 == '\n' {
                                *line += 1;
                            }

                            // Check for quote which might start/end a nested template
                            if c2 == '"' {
                                // If we're tracking a template on the stack, check if this closes it
                                if let Some((nested_open_count, in_interp)) =
                                    template_stack.last_mut()
                                {
                                    if !*in_interp {
                                        // We're looking at the closing quote of a nested template
                                        // Check if the required closing braces follow
                                        let mut matched_braces = 0;
                                        for _ in 0..*nested_open_count {
                                            if let Some('}') = stream.peek().cloned() {
                                                stream.next();
                                                matched_braces += 1;
                                            } else {
                                                break;
                                            }
                                        }

                                        expr.push('"');
                                        for _ in 0..matched_braces {
                                            expr.push('}');
                                        }

                                        if matched_braces == *nested_open_count {
                                            template_stack.pop();
                                        }
                                        continue;
                                    }
                                }
                            }

                            // Check for opening braces - could be nested template or interpolation
                            if c2 == '{' {
                                let mut brace_count_here = 1;
                                while let Some('{') = stream.peek().cloned() {
                                    stream.next();
                                    expr.push('{');
                                    brace_count_here += 1;
                                }
                                expr.push('{');

                                // Skip whitespace
                                let mut temp_stream = stream.clone();
                                while let Some(&wsc) = temp_stream.peek() {
                                    if wsc.is_whitespace() {
                                        temp_stream.next();
                                    } else {
                                        break;
                                    }
                                }

                                // Check what follows the braces
                                match temp_stream.peek() {
                                    Some('"') => {
                                        // This is the start of a nested template
                                        template_stack.push((brace_count_here, false));
                                    }
                                    _ => {
                                        // This is just braces in an expression, not a template start
                                        // Continue processing normally
                                    }
                                }
                                continue;
                            }

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

                                // Check if this closes the current interpolation
                                if got == open_count && template_stack.is_empty() {
                                    break; // end interpolation
                                } else {
                                    // Either not enough braces, or we're inside a nested template
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
                                start_line,
                            ))
                        }
                    }
                }
                chunks.push(Chunk::Expr(expr, expr_line));
                continue;
            } else {
                // brace_count != open_count: treat all collected braces as literal
                for _ in 0..brace_count {
                    cur.push('{');
                }
                continue;
            }
        }

        if ch == '}' {
            // Count consecutive closing braces.
            let mut brace_count = 1;
            while let Some('}') = stream.peek().cloned() {
                stream.next();
                brace_count += 1;
            }
            // All closing braces are literal (no escape hatch)
            for _ in 0..brace_count {
                cur.push('}');
            }
            continue;
        }

        cur.push(ch);
    }

    Err(EvalError::new(
        "unexpected EOF inside template",
        None,
        None,
        start_line,
    ))
}

pub fn path(stream: &mut Peekable<Chars<'_>>, line: &mut usize) -> Result<Token, EvalError> {
    let start_line = *line;
    let mut cur = String::new();
    let mut chunks = Vec::<Chunk>::new();
    let mut is_first_char = true;

    loop {
        // Peek first to check if we should continue
        let c_opt = stream.peek().cloned();
        match c_opt {
            None => {
                if !cur.is_empty() {
                    chunks.push(Chunk::String(cur));
                }
                return Ok(Token::Path(chunks, start_line));
            }
            Some(c) => {
                // Check if this character would end the path (whitespace or delimiter)
                if c.is_whitespace() || matches!(c, ',' | ']' | ')' | '}') {
                    if !cur.is_empty() {
                        chunks.push(Chunk::String(cur));
                    }
                    // Don't consume the delimiter - let main tokenize handle it
                    return Ok(Token::Path(chunks, start_line));
                }

                // Check for interpolation before consuming
                if c == '{' {
                    // Consume the '{'
                    stream.next();

                    if !cur.is_empty() {
                        chunks.push(Chunk::String(cur));
                        cur = String::new();
                    }

                    // After an interpolation, we're no longer at the first char
                    is_first_char = false;

                    // Capture the line where the expression content actually starts
                    let expr_line = *line;
                    let mut expr = String::new();
                    loop {
                        match stream.next() {
                            None => {
                                return Err(EvalError::new(
                                    "EOF in path interpolation",
                                    None,
                                    None,
                                    start_line,
                                ))
                            }
                            Some(ch2) => {
                                if ch2 == '\n' {
                                    *line += 1;
                                }
                                if ch2 == '}' {
                                    break;
                                } else {
                                    expr.push(ch2);
                                }
                            }
                        }
                    }

                    chunks.push(Chunk::Expr(expr, expr_line));
                    continue;
                }

                // OK to consume this character
                let c = stream.next().unwrap(); // Safe: we just peeked

                if c == '\n' {
                    *line += 1;
                }

                // Reject absolute paths (starting with /)
                if is_first_char && c == '/' {
                    return Err(EvalError::new(
                        "Absolute paths are not allowed in Avon syntax. Use relative paths and deploy with --root for absolute locations.",
                        Some("relative path (e.g., @config/file.txt)".to_string()),
                        Some("absolute path starting with /".to_string()),
                        start_line,
                    ));
                }

                is_first_char = false;
                cur.push(c);
            }
        }
    }
}

pub fn number(
    next: char,
    stream: &mut Peekable<Chars<'_>>,
    line: usize,
) -> Result<Token, EvalError> {
    let mut number = String::new();
    number.push(next);
    loop {
        let Some(peek) = stream.peek() else {
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
        return Ok(Token::Int(number, line));
    };

    if peek == &'.' {
        // Check if this is actually a float by looking ahead for a digit
        // after the dot. If no digit follows, it might be a range operator (..)
        // or member access (.), so leave it for the main lexer
        let mut temp_stream = stream.clone();
        temp_stream.next(); // consume the '.'

        if let Some(next_char) = temp_stream.peek() {
            if next_char.is_numeric() {
                // This is definitely a float, consume the dot and parse decimal part
                // Safe: we just checked peek == '.'
                number.push(stream.next().expect("'.' character exists after peek"));
                loop {
                    let Some(peek) = stream.peek() else {
                        break;
                    };
                    if peek.is_whitespace() || !peek.is_numeric() {
                        break;
                    }
                    // Safe: we just peeked and confirmed there's a character
                    number.push(stream.next().expect("character exists after peek"));
                }
                let number: f64 = number.parse().unwrap_or_default();
                return Ok(Token::Float(number, line));
            }
        }
    }

    let number: i64 = number.parse().unwrap_or_default();
    Ok(Token::Int(number, line))
}

#[allow(dead_code, unreachable_code, unused_mut)]
pub fn tokenize(input: String) -> Result<Vec<Token>, EvalError> {
    let mut output = vec![];
    let mut stream = input.chars().peekable();
    let mut line = 1;

    loop {
        let Some(next) = stream.next() else {
            break;
        };

        if next == '\n' {
            line += 1;
            continue;
        }

        if next.is_whitespace() {
            continue;
        }

        macro_rules! checkfor {
            ($e:expr,$t:ident) => {
                if let Some($e) = stream.peek() {
                    stream.next();
                    output.push(Token::$t(line));
                    continue;
                }
            };
        }

        match next {
            'A'..='Z' | 'a'..='z' | '_' => {
                let ident = identifier(next, &mut stream, line);
                output.push(ident);
            }
            '0'..='9' => {
                let number = number(next, &mut stream, line)?;
                output.push(number);
            }
            '\"' => {
                let string = string(&mut stream, &mut line)?;
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
                    output.push(Token::LBrace(line));
                } else {
                    // Parse as template - chunk() will handle all the brace counting
                    let chunk = chunk(&mut stream, &mut line)?;
                    output.push(chunk);
                }
            }
            '[' => output.push(Token::LBracket(line)),
            ']' => output.push(Token::RBracket(line)),
            '}' => output.push(Token::RBrace(line)),
            ',' => output.push(Token::Comma(line)),
            ':' => output.push(Token::Colon(line)),
            '.' => {
                checkfor!('.', DoubleDot);
                output.push(Token::Dot(line))
            }
            '=' => {
                checkfor!('=', DoubleEqual);
                output.push(Token::Equal(line))
            }
            '@' => {
                let chunk = path(&mut stream, &mut line)?;
                output.push(chunk);
            }
            '#' => {
                while let Some(&_c) = stream.peek() {
                    let c = stream.next().unwrap();
                    if c == '\n' {
                        line += 1;
                        break;
                    }
                }
                continue;
            }
            '!' => {
                if let Some('=') = stream.peek() {
                    stream.next();
                    output.push(Token::NotEqual(line));
                    continue;
                }
            }
            '>' => {
                if let Some('=') = stream.peek() {
                    stream.next();
                    output.push(Token::GreaterEqual(line));
                    continue;
                }
                output.push(Token::Greater(line));
            }
            '<' => {
                if let Some('=') = stream.peek() {
                    stream.next();
                    output.push(Token::LessEqual(line));
                    continue;
                }
                output.push(Token::Less(line));
            }
            '+' => output.push(Token::Add(line)),
            // not sure what to do with single '&' and '|', so only handle double versions
            '&' => {
                if let Some('&') = stream.peek() {
                    stream.next();
                    output.push(Token::And(line));
                } else {
                    // Single & is not supported - skip with warning
                    eprintln!("Warning: single '&' is not a valid token on line {}", line);
                }
            }
            '|' => {
                if let Some('|') = stream.peek() {
                    stream.next();
                    output.push(Token::Or(line));
                } else {
                    // Single | is not supported - skip with warning
                    eprintln!("Warning: single '|' is not a valid token on line {}", line);
                }
            }
            '-' => {
                // Check for pipe operator ->
                if let Some('>') = stream.peek() {
                    stream.next();
                    output.push(Token::Pipe(line));
                    continue;
                }
                output.push(Token::Sub(line))
            }
            '/' => {
                // Check for integer division operator //
                if let Some('/') = stream.peek() {
                    stream.next();
                    output.push(Token::IntDiv(line));
                    continue;
                }
                output.push(Token::Div(line))
            }
            '%' => output.push(Token::Mod(line)),
            '*' => {
                // Check for power operator **
                if let Some('*') = stream.peek() {
                    stream.next();
                    output.push(Token::Power(line));
                    continue;
                }
                output.push(Token::Mul(line))
            }
            '(' => output.push(Token::LParen(line)),
            ')' => output.push(Token::RParen(line)),
            '\\' => output.push(Token::BackSlash(line)),
            '?' => output.push(Token::Question(line)),
            _ => {}
        }
    }
    Ok(output)
}
