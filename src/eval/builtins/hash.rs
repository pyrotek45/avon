//! Hashing functions: sha256, sha512, base64_encode, base64_decode, hex_encode, hex_decode

use crate::common::{EvalError, Value};

/// Names of hashing builtins
pub const NAMES: &[&str] = &[
    "base64_decode",
    "base64_encode",
    "hex_decode",
    "hex_encode",
    "sha256",
    "sha512",
];

/// Get arity for hashing functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "base64_decode" | "base64_encode" | "hex_decode" | "hex_encode" | "sha256" | "sha512" => {
            Some(1)
        }
        _ => None,
    }
}

/// Check if name is a hashing builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a hashing builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "sha256" => {
            let input = &args[0];
            let s = match input {
                Value::String(s) => s.clone(),
                _ => {
                    return Err(EvalError::type_mismatch(
                        "string",
                        input.to_string(source),
                        line,
                    ))
                }
            };

            use sha2::Digest;
            let mut hasher = sha2::Sha256::new();
            hasher.update(s.as_bytes());
            let result = hasher.finalize();
            Ok(Value::String(format!("{:x}", result)))
        }
        "sha512" => {
            let input = &args[0];
            let s = match input {
                Value::String(s) => s.clone(),
                _ => {
                    return Err(EvalError::type_mismatch(
                        "string",
                        input.to_string(source),
                        line,
                    ))
                }
            };

            use sha2::Digest;
            let mut hasher = sha2::Sha512::new();
            hasher.update(s.as_bytes());
            let result = hasher.finalize();
            Ok(Value::String(format!("{:x}", result)))
        }
        "base64_encode" => {
            let input = &args[0];
            let s = match input {
                Value::String(s) => s.clone(),
                _ => {
                    return Err(EvalError::type_mismatch(
                        "string",
                        input.to_string(source),
                        line,
                    ))
                }
            };

            use base64::Engine;
            let encoded = base64::engine::general_purpose::STANDARD.encode(s.as_bytes());
            Ok(Value::String(encoded))
        }
        "base64_decode" => {
            let input = &args[0];
            let s = match input {
                Value::String(s) => s.clone(),
                _ => {
                    return Err(EvalError::type_mismatch(
                        "string",
                        input.to_string(source),
                        line,
                    ))
                }
            };

            use base64::Engine;
            match base64::engine::general_purpose::STANDARD.decode(&s) {
                Ok(decoded) => match String::from_utf8(decoded) {
                    Ok(string) => Ok(Value::String(string)),
                    Err(_) => Err(EvalError::new(
                        "base64_decode: decoded bytes are not valid UTF-8".to_string(),
                        None,
                        None,
                        line,
                    )),
                },
                Err(e) => Err(EvalError::new(
                    format!("base64_decode: invalid base64: {}", e),
                    None,
                    None,
                    line,
                )),
            }
        }
        "hex_encode" => {
            let input = &args[0];
            let s = match input {
                Value::String(s) => s.clone(),
                _ => {
                    return Err(EvalError::type_mismatch(
                        "string",
                        input.to_string(source),
                        line,
                    ))
                }
            };

            let hex = s
                .as_bytes()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            Ok(Value::String(hex))
        }
        "hex_decode" => {
            let input = &args[0];
            let s = match input {
                Value::String(s) => s.clone(),
                _ => {
                    return Err(EvalError::type_mismatch(
                        "string",
                        input.to_string(source),
                        line,
                    ))
                }
            };

            // Check if the string length is even
            if s.len() % 2 != 0 {
                return Err(EvalError::new(
                    "hex_decode: hex string must have even length".to_string(),
                    None,
                    None,
                    line,
                ));
            }

            // Decode hex pairs
            let mut bytes = Vec::new();
            for i in (0..s.len()).step_by(2) {
                match u8::from_str_radix(&s[i..i + 2], 16) {
                    Ok(b) => bytes.push(b),
                    Err(_) => {
                        return Err(EvalError::new(
                            format!("hex_decode: invalid hex sequence: {}", &s[i..i + 2]),
                            None,
                            None,
                            line,
                        ))
                    }
                }
            }

            match String::from_utf8(bytes) {
                Ok(string) => Ok(Value::String(string)),
                Err(_) => Err(EvalError::new(
                    "hex_decode: decoded bytes are not valid UTF-8".to_string(),
                    None,
                    None,
                    line,
                )),
            }
        }
        _ => Err(EvalError::new(
            format!("unknown hash function: {}", name),
            None,
            None,
            line,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_prog(prog: &str) -> Value {
        use crate::eval::{eval, initial_builtins};
        use crate::lexer::tokenize;
        use crate::parser::parse;

        let tokens = tokenize(prog.to_string()).expect("tokenize");
        let ast = parse(tokens);
        let mut symbols = initial_builtins();
        eval(ast.program, &mut symbols, prog).expect("eval")
    }

    #[test]
    fn test_sha256() {
        match eval_prog("sha256 \"hello\"") {
            Value::String(s) => {
                assert_eq!(
                    s,
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                );
            }
            v => panic!("expected string, got {:?}", v),
        }
    }

    #[test]
    fn test_base64_encode_decode() {
        // Test encode
        match eval_prog("base64_encode \"hello world\"") {
            Value::String(s) => {
                assert_eq!(s, "aGVsbG8gd29ybGQ=");
            }
            v => panic!("expected string, got {:?}", v),
        }

        // Test decode
        match eval_prog("base64_decode \"aGVsbG8gd29ybGQ=\"") {
            Value::String(s) => {
                assert_eq!(s, "hello world");
            }
            v => panic!("expected string, got {:?}", v),
        }
    }

    #[test]
    fn test_hex_encode_decode() {
        // Test encode
        match eval_prog("hex_encode \"hello\"") {
            Value::String(s) => {
                assert_eq!(s, "68656c6c6f");
            }
            v => panic!("expected string, got {:?}", v),
        }

        // Test decode
        match eval_prog("hex_decode \"68656c6c6f\"") {
            Value::String(s) => {
                assert_eq!(s, "hello");
            }
            v => panic!("expected string, got {:?}", v),
        }
    }
}
