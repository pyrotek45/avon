//! Linux ricing utilities for theme and status display generation

use crate::common::{EvalError, Number, Value};

/// Names of ricing builtins
pub const NAMES: &[&str] = &[
    "format_filesize",
    "format_temp",
    "format_uptime",
    "progressbar",
    "gauge",
    "chmod_numeric",
    "chmod_symbolic",
    "shebang",
];

/// Get arity for ricing functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "format_filesize" | "format_temp" | "chmod_numeric" | "chmod_symbolic" | "shebang" => {
            Some(1)
        }
        "format_uptime" | "progressbar" | "gauge" => Some(2),
        _ => None,
    }
}

/// Check if name is a ricing builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a ricing builtin function
pub fn execute(name: &str, args: &[Value], _source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "format_filesize" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "format_filesize requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let bytes = match &args[0] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => {
                    return Err(EvalError::new(
                        "format_filesize requires a number",
                        None,
                        None,
                        line,
                    ))
                }
            };

            let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
            let mut size = bytes;
            let mut unit_idx = 0;

            while size >= 1024.0 && unit_idx < units.len() - 1 {
                size /= 1024.0;
                unit_idx += 1;
            }

            let formatted = if size < 10.0 {
                format!("{:.1} {}", size, units[unit_idx])
            } else {
                format!("{:.0} {}", size, units[unit_idx])
            };

            Ok(Value::String(formatted))
        }

        "format_temp" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "format_temp requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let temp = match &args[0] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => {
                    return Err(EvalError::new(
                        "format_temp requires a number",
                        None,
                        None,
                        line,
                    ))
                }
            };

            Ok(Value::String(format!("{}°C", temp as i64)))
        }

        "format_uptime" => {
            if args.len() < 2 {
                return Err(EvalError::new(
                    "format_uptime requires 2 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let seconds = match &args[0] {
                Value::Number(Number::Int(n)) => *n,
                Value::Number(Number::Float(f)) => *f as i64,
                _ => {
                    return Err(EvalError::new(
                        "format_uptime requires numbers",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let format_type = match &args[1] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "format_uptime requires a format string",
                        None,
                        None,
                        line,
                    ))
                }
            };

            let days = seconds / 86400;
            let hours = (seconds % 86400) / 3600;
            let minutes = (seconds % 3600) / 60;
            let secs = seconds % 60;

            let result = match format_type.as_str() {
                "short" => format!("{}d {}h", days, hours),
                "medium" => format!("{}d {}h {}m", days, hours, minutes),
                "long" => format!("{}d {}h {}m {}s", days, hours, minutes, secs),
                "hms" => format!("{:02}:{:02}:{:02}", hours, minutes, secs),
                _ => format!("{}d {}h {}m", days, hours, minutes),
            };

            Ok(Value::String(result))
        }

        "progressbar" => {
            if args.len() < 2 {
                return Err(EvalError::new(
                    "progressbar requires 2 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let filled = match &args[0] {
                Value::Number(Number::Int(n)) => *n,
                Value::Number(Number::Float(f)) => *f as i64,
                _ => {
                    return Err(EvalError::new(
                        "progressbar requires numbers",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let total = match &args[1] {
                Value::Number(Number::Int(n)) => *n,
                Value::Number(Number::Float(f)) => *f as i64,
                _ => {
                    return Err(EvalError::new(
                        "progressbar requires numbers",
                        None,
                        None,
                        line,
                    ))
                }
            };

            if total <= 0 {
                return Err(EvalError::new(
                    "progressbar total must be > 0",
                    None,
                    None,
                    line,
                ));
            }

            let filled = filled.min(total).max(0);
            let ratio = filled as f64 / total as f64;
            let bar_width = 10;
            let filled_count = ((ratio * bar_width as f64).round()) as usize;

            let mut bar = String::new();
            for _ in 0..filled_count {
                bar.push('█');
            }
            for _ in filled_count..bar_width {
                bar.push('░');
            }

            Ok(Value::String(bar))
        }

        "gauge" => {
            if args.len() < 2 {
                return Err(EvalError::new(
                    "gauge requires 2 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let current = match &args[0] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => return Err(EvalError::new("gauge requires numbers", None, None, line)),
            };
            let max = match &args[1] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => return Err(EvalError::new("gauge requires numbers", None, None, line)),
            };

            if max <= 0.0 {
                return Err(EvalError::new("gauge max must be > 0", None, None, line));
            }

            let ratio = (current / max).clamp(0.0, 1.0);
            let percent = (ratio * 100.0).round() as i64;
            let filled = ((ratio * 10.0).round()) as usize;

            let mut bar = String::from("▐");
            for _ in 0..filled {
                bar.push('▌');
            }
            for _ in filled..10 {
                bar.push(' ');
            }
            bar.push('▌');

            Ok(Value::String(format!("{} {}%", bar, percent)))
        }

        "chmod_numeric" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "chmod_numeric requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let symbolic = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "chmod_numeric requires a string",
                        None,
                        None,
                        line,
                    ))
                }
            };

            // Parse symbolic chmod like "rwxr-xr-x"
            if symbolic.len() < 9 {
                return Err(EvalError::new(
                    "chmod_numeric requires 9-char symbolic format (e.g., rwxr-xr-x)",
                    None,
                    None,
                    line,
                ));
            }

            let mut numeric = 0;
            let chars: Vec<char> = symbolic.chars().collect();

            // Owner
            if chars[0] == 'r' {
                numeric += 400;
            }
            if chars[1] == 'w' {
                numeric += 200;
            }
            if chars[2] == 'x' {
                numeric += 100;
            }

            // Group
            if chars[3] == 'r' {
                numeric += 40;
            }
            if chars[4] == 'w' {
                numeric += 20;
            }
            if chars[5] == 'x' {
                numeric += 10;
            }

            // Others
            if chars[6] == 'r' {
                numeric += 4;
            }
            if chars[7] == 'w' {
                numeric += 2;
            }
            if chars[8] == 'x' {
                numeric += 1;
            }

            Ok(Value::Number(Number::Int(numeric)))
        }

        "chmod_symbolic" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "chmod_symbolic requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let numeric = match &args[0] {
                Value::Number(Number::Int(n)) => *n,
                Value::Number(Number::Float(f)) => *f as i64,
                _ => {
                    return Err(EvalError::new(
                        "chmod_symbolic requires a number",
                        None,
                        None,
                        line,
                    ))
                }
            };

            if !(0..=777).contains(&numeric) {
                return Err(EvalError::new(
                    "chmod_symbolic requires a number between 0 and 777",
                    None,
                    None,
                    line,
                ));
            }

            let mut symbolic = String::new();

            // Extract octal digits: owner (hundreds), group (tens), others (ones)
            let owner = (numeric / 100) % 10;
            let group = (numeric / 10) % 10;
            let others = numeric % 10;

            // Owner
            symbolic.push(if owner & 4 != 0 { 'r' } else { '-' });
            symbolic.push(if owner & 2 != 0 { 'w' } else { '-' });
            symbolic.push(if owner & 1 != 0 { 'x' } else { '-' });

            // Group
            symbolic.push(if group & 4 != 0 { 'r' } else { '-' });
            symbolic.push(if group & 2 != 0 { 'w' } else { '-' });
            symbolic.push(if group & 1 != 0 { 'x' } else { '-' });

            // Others
            symbolic.push(if others & 4 != 0 { 'r' } else { '-' });
            symbolic.push(if others & 2 != 0 { 'w' } else { '-' });
            symbolic.push(if others & 1 != 0 { 'x' } else { '-' });

            Ok(Value::String(symbolic))
        }

        "shebang" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "shebang requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let interpreter = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "shebang requires a string",
                        None,
                        None,
                        line,
                    ))
                }
            };

            let shebang = if interpreter.starts_with('/') {
                format!("#!{}", interpreter)
            } else {
                format!("#!/usr/bin/env {}", interpreter)
            };

            Ok(Value::String(shebang))
        }

        _ => Err(EvalError::new(
            format!("Unknown ricing builtin: {}", name),
            None,
            None,
            line,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_filesize() {
        let result = execute(
            "format_filesize",
            &[Value::Number(Number::Int(1024))],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "1.0 KiB".to_string()),
            _ => panic!("Expected string"),
        }

        let result = execute(
            "format_filesize",
            &[Value::Number(Number::Int(1048576))],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "1.0 MiB".to_string()),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_format_temp() {
        let result = execute("format_temp", &[Value::Number(Number::Int(65))], "", 0).unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "65°C".to_string()),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_format_uptime() {
        let result = execute(
            "format_uptime",
            &[
                Value::Number(Number::Int(90061)),
                Value::String("short".to_string()),
            ],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "1d 1h".to_string()),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_progressbar() {
        let result = execute(
            "progressbar",
            &[
                Value::Number(Number::Int(5)),
                Value::Number(Number::Int(10)),
            ],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "█████░░░░░".to_string()),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_chmod_numeric() {
        let result = execute(
            "chmod_numeric",
            &[Value::String("rwxr-xr-x".to_string())],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::Number(Number::Int(n)) => assert_eq!(n, 755),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_chmod_symbolic() {
        let result = execute("chmod_symbolic", &[Value::Number(Number::Int(755))], "", 0).unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "rwxr-xr-x".to_string()),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_shebang() {
        let result = execute("shebang", &[Value::String("bash".to_string())], "", 0).unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "#!/usr/bin/env bash".to_string()),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_format_uptime_medium() {
        let result = execute(
            "format_uptime",
            &[
                Value::Number(Number::Int(345600)),
                Value::String("medium".to_string()),
            ],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "4d 0h 0m".to_string()),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_format_uptime_hms() {
        let result = execute(
            "format_uptime",
            &[
                Value::Number(Number::Int(3661)),
                Value::String("hms".to_string()),
            ],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "01:01:01".to_string()),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_gauge() {
        let result = execute(
            "gauge",
            &[
                Value::Number(Number::Int(50)),
                Value::Number(Number::Int(100)),
            ],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => {
                // Should contain percentage
                assert!(s.contains('%'));
            }
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_chmod_numeric_644() {
        let result = execute(
            "chmod_numeric",
            &[Value::String("rw-r--r--".to_string())],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::Number(Number::Int(n)) => assert_eq!(n, 644),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_shebang_python() {
        let result = execute("shebang", &[Value::String("python3".to_string())], "", 0).unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "#!/usr/bin/env python3".to_string()),
            _ => panic!("Expected string"),
        }
    }
}
