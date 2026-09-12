//! Color manipulation functions for theme generation and palette management

use crate::common::{EvalError, Number, Value};
use std::collections::HashMap;

/// Names of color builtins
pub const NAMES: &[&str] = &[
    "hex_to_rgb",
    "rgb_to_hex",
    "hex_to_hsl",
    "hsl_to_hex",
    "lighten",
    "darken",
    "saturate",
    "complementary",
    "palette_monochromatic",
    "palette_analogous",
    "palette_triadic",
    "blend_colors",
    "invert_color",
    "grayscale",
    "contrast_ratio",
];

/// Get arity for color functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "hex_to_rgb" | "hex_to_hsl" | "complementary" | "invert_color" | "grayscale"
        | "palette_analogous" | "palette_triadic" => Some(1),
        "rgb_to_hex" | "hsl_to_hex" => Some(3),
        "lighten" | "darken" | "saturate" | "palette_monochromatic" | "contrast_ratio" => Some(2),
        "blend_colors" => Some(3),
        _ => None,
    }
}

/// Check if name is a color builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

// Helper: parse hex color to RGB components (0-255 each)
fn parse_hex(hex: &str, line: usize) -> Result<(u8, u8, u8), EvalError> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(EvalError::new(
            format!("Invalid hex color: {}. Expected 6 hex digits.", hex),
            None,
            None,
            line,
        ));
    }
    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| EvalError::new(format!("Invalid hex: {}", hex), None, None, line))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| EvalError::new(format!("Invalid hex: {}", hex), None, None, line))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| EvalError::new(format!("Invalid hex: {}", hex), None, None, line))?;
    Ok((r, g, b))
}

// Helper: format RGB to hex
fn rgb_to_hex_str(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

// Helper: convert RGB (0-255) to HSL (0-360, 0-100, 0-100)
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    let (h, s) = if (max - min).abs() < 0.0001 {
        (0.0, 0.0)
    } else {
        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };
        let h = if (r - max).abs() < 0.0001 {
            ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
        } else if (g - max).abs() < 0.0001 {
            ((b - r) / d + 2.0) / 6.0
        } else {
            ((r - g) / d + 4.0) / 6.0
        };
        (h * 360.0, s * 100.0)
    };

    (h, s, l * 100.0)
}

// Helper: convert HSL (0-360, 0-100, 0-100) to RGB (0-255 each)
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let h = h / 360.0;
    let s = s / 100.0;
    let l = l / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h * 6.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

// Helper: calculate relative luminance for contrast ratio
fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    let r = if r <= 0.03928 {
        r / 12.92
    } else {
        ((r + 0.055) / 1.055).powf(2.4)
    };
    let g = if g <= 0.03928 {
        g / 12.92
    } else {
        ((g + 0.055) / 1.055).powf(2.4)
    };
    let b = if b <= 0.03928 {
        b / 12.92
    } else {
        ((b + 0.055) / 1.055).powf(2.4)
    };

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Execute a color builtin function
pub fn execute(name: &str, args: &[Value], _source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "hex_to_rgb" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "hex_to_rgb requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "hex_to_rgb requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            let mut dict = HashMap::new();
            dict.insert("r".to_string(), Value::Number(Number::Int(r as i64)));
            dict.insert("g".to_string(), Value::Number(Number::Int(g as i64)));
            dict.insert("b".to_string(), Value::Number(Number::Int(b as i64)));
            Ok(Value::Dict(dict))
        }

        "rgb_to_hex" => {
            if args.len() < 3 {
                return Err(EvalError::new(
                    "rgb_to_hex requires 3 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let r = match &args[0] {
                Value::Number(Number::Int(n)) => *n as u8,
                Value::Number(Number::Float(f)) => *f as u8,
                _ => {
                    return Err(EvalError::new(
                        "rgb_to_hex requires numeric arguments",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let g = match &args[1] {
                Value::Number(Number::Int(n)) => *n as u8,
                Value::Number(Number::Float(f)) => *f as u8,
                _ => {
                    return Err(EvalError::new(
                        "rgb_to_hex requires numeric arguments",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let b = match &args[2] {
                Value::Number(Number::Int(n)) => *n as u8,
                Value::Number(Number::Float(f)) => *f as u8,
                _ => {
                    return Err(EvalError::new(
                        "rgb_to_hex requires numeric arguments",
                        None,
                        None,
                        line,
                    ))
                }
            };
            Ok(Value::String(rgb_to_hex_str(r, g, b)))
        }

        "hex_to_hsl" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "hex_to_hsl requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "hex_to_hsl requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            let (h, s, l) = rgb_to_hsl(r, g, b);
            let mut dict = HashMap::new();
            dict.insert("h".to_string(), Value::Number(Number::Float(h)));
            dict.insert("s".to_string(), Value::Number(Number::Float(s)));
            dict.insert("l".to_string(), Value::Number(Number::Float(l)));
            Ok(Value::Dict(dict))
        }

        "hsl_to_hex" => {
            if args.len() < 3 {
                return Err(EvalError::new(
                    "hsl_to_hex requires 3 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let h = match &args[0] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => {
                    return Err(EvalError::new(
                        "hsl_to_hex requires numeric arguments",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let s = match &args[1] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => {
                    return Err(EvalError::new(
                        "hsl_to_hex requires numeric arguments",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let l = match &args[2] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => {
                    return Err(EvalError::new(
                        "hsl_to_hex requires numeric arguments",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = hsl_to_rgb(h, s, l);
            Ok(Value::String(rgb_to_hex_str(r, g, b)))
        }

        "lighten" => {
            if args.len() < 2 {
                return Err(EvalError::new(
                    "lighten requires 2 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "lighten requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let amount = match &args[1] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => {
                    return Err(EvalError::new(
                        "lighten requires a numeric amount",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            let (h, s, mut l) = rgb_to_hsl(r, g, b);
            l = (l + amount).clamp(0.0, 100.0);
            let (r, g, b) = hsl_to_rgb(h, s, l);
            Ok(Value::String(rgb_to_hex_str(r, g, b)))
        }

        "darken" => {
            if args.len() < 2 {
                return Err(EvalError::new(
                    "darken requires 2 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "darken requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let amount = match &args[1] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => {
                    return Err(EvalError::new(
                        "darken requires a numeric amount",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            let (h, s, mut l) = rgb_to_hsl(r, g, b);
            l = (l - amount).clamp(0.0, 100.0);
            let (r, g, b) = hsl_to_rgb(h, s, l);
            Ok(Value::String(rgb_to_hex_str(r, g, b)))
        }

        "saturate" => {
            if args.len() < 2 {
                return Err(EvalError::new(
                    "saturate requires 2 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "saturate requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let amount = match &args[1] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => {
                    return Err(EvalError::new(
                        "saturate requires a numeric amount",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            let (h, mut s, l) = rgb_to_hsl(r, g, b);
            s = (s + amount).clamp(0.0, 100.0);
            let (r, g, b) = hsl_to_rgb(h, s, l);
            Ok(Value::String(rgb_to_hex_str(r, g, b)))
        }

        "complementary" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "complementary requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "complementary requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            let (mut h, s, l) = rgb_to_hsl(r, g, b);
            h = (h + 180.0) % 360.0;
            let (r, g, b) = hsl_to_rgb(h, s, l);
            Ok(Value::String(rgb_to_hex_str(r, g, b)))
        }

        "palette_monochromatic" => {
            if args.len() < 2 {
                return Err(EvalError::new(
                    "palette_monochromatic requires 2 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "palette_monochromatic requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let count = match &args[1] {
                Value::Number(Number::Int(n)) => *n as usize,
                Value::Number(Number::Float(f)) => *f as usize,
                _ => {
                    return Err(EvalError::new(
                        "palette_monochromatic requires a numeric count",
                        None,
                        None,
                        line,
                    ))
                }
            };

            let (r, g, b) = parse_hex(hex, line)?;
            let (h, s, _l) = rgb_to_hsl(r, g, b);

            let mut colors = Vec::new();
            for i in 0..count {
                let lightness = (100.0 / count as f64) * (i as f64 + 1.0);
                let (r, g, b) = hsl_to_rgb(h, s, lightness);
                colors.push(Value::String(rgb_to_hex_str(r, g, b)));
            }
            Ok(Value::List(colors))
        }

        "palette_analogous" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "palette_analogous requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "palette_analogous requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            let (h, s, l) = rgb_to_hsl(r, g, b);

            let mut colors = vec![Value::String(rgb_to_hex_str(r, g, b))];
            for offset in [30.0, -30.0] {
                let h_new = (h + offset) % 360.0;
                let (r, g, b) = hsl_to_rgb(h_new, s, l);
                colors.push(Value::String(rgb_to_hex_str(r, g, b)));
            }
            Ok(Value::List(colors))
        }

        "palette_triadic" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "palette_triadic requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "palette_triadic requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            let (h, s, l) = rgb_to_hsl(r, g, b);

            let mut colors = vec![Value::String(rgb_to_hex_str(r, g, b))];
            for offset in [120.0, 240.0] {
                let h_new = (h + offset) % 360.0;
                let (r, g, b) = hsl_to_rgb(h_new, s, l);
                colors.push(Value::String(rgb_to_hex_str(r, g, b)));
            }
            Ok(Value::List(colors))
        }

        "blend_colors" => {
            if args.len() < 3 {
                return Err(EvalError::new(
                    "blend_colors requires 3 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let hex1 = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "blend_colors requires hex strings",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let hex2 = match &args[1] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "blend_colors requires hex strings",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let ratio = match &args[2] {
                Value::Number(Number::Int(n)) => *n as f64,
                Value::Number(Number::Float(f)) => *f,
                _ => {
                    return Err(EvalError::new(
                        "blend_colors requires a numeric ratio",
                        None,
                        None,
                        line,
                    ))
                }
            };

            let (r1, g1, b1) = parse_hex(hex1, line)?;
            let (r2, g2, b2) = parse_hex(hex2, line)?;

            let ratio = ratio.clamp(0.0, 1.0);
            let r = ((r1 as f64) * (1.0 - ratio) + (r2 as f64) * ratio).round() as u8;
            let g = ((g1 as f64) * (1.0 - ratio) + (g2 as f64) * ratio).round() as u8;
            let b = ((b1 as f64) * (1.0 - ratio) + (b2 as f64) * ratio).round() as u8;

            Ok(Value::String(rgb_to_hex_str(r, g, b)))
        }

        "invert_color" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "invert_color requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "invert_color requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            Ok(Value::String(rgb_to_hex_str(255 - r, 255 - g, 255 - b)))
        }

        "grayscale" => {
            if args.is_empty() {
                return Err(EvalError::new(
                    "grayscale requires 1 argument",
                    None,
                    None,
                    line,
                ));
            }
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "grayscale requires a hex string",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let (r, g, b) = parse_hex(hex, line)?;
            let (_h, _s, l) = rgb_to_hsl(r, g, b);
            let (r, g, b) = hsl_to_rgb(0.0, 0.0, l);
            Ok(Value::String(rgb_to_hex_str(r, g, b)))
        }

        "contrast_ratio" => {
            if args.len() < 2 {
                return Err(EvalError::new(
                    "contrast_ratio requires 2 arguments",
                    None,
                    None,
                    line,
                ));
            }
            let hex1 = match &args[0] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "contrast_ratio requires hex strings",
                        None,
                        None,
                        line,
                    ))
                }
            };
            let hex2 = match &args[1] {
                Value::String(s) => s,
                _ => {
                    return Err(EvalError::new(
                        "contrast_ratio requires hex strings",
                        None,
                        None,
                        line,
                    ))
                }
            };

            let (r1, g1, b1) = parse_hex(hex1, line)?;
            let (r2, g2, b2) = parse_hex(hex2, line)?;

            let l1 = relative_luminance(r1, g1, b1);
            let l2 = relative_luminance(r2, g2, b2);

            let lighter = l1.max(l2);
            let darker = l1.min(l2);
            let ratio = (lighter + 0.05) / (darker + 0.05);

            Ok(Value::Number(Number::Float(ratio)))
        }

        _ => Err(EvalError::new(
            format!("Unknown color builtin: {}", name),
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
    fn test_hex_to_rgb() {
        let hex = Value::String("#FF5733".to_string());
        let result = execute("hex_to_rgb", &[hex], "", 0).unwrap();
        match result {
            Value::Dict(d) => {
                match d.get("r") {
                    Some(Value::Number(Number::Int(255))) => (),
                    _ => panic!("Expected r=255"),
                }
                match d.get("g") {
                    Some(Value::Number(Number::Int(87))) => (),
                    _ => panic!("Expected g=87"),
                }
                match d.get("b") {
                    Some(Value::Number(Number::Int(51))) => (),
                    _ => panic!("Expected b=51"),
                }
            }
            _ => panic!("Expected dict"),
        }
    }

    #[test]
    fn test_rgb_to_hex() {
        let result = execute(
            "rgb_to_hex",
            &[
                Value::Number(Number::Int(255)),
                Value::Number(Number::Int(87)),
                Value::Number(Number::Int(51)),
            ],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "#ff5733".to_string()),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_lighten() {
        let hex = Value::String("#808080".to_string());
        let result = execute(
            "lighten",
            &[hex.clone(), Value::Number(Number::Int(10))],
            "",
            0,
        )
        .unwrap();
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_complementary() {
        let hex = Value::String("#FF0000".to_string());
        let result = execute("complementary", &[hex], "", 0).unwrap();
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_palette_monochromatic() {
        let hex = Value::String("#FF5733".to_string());
        let result = execute(
            "palette_monochromatic",
            &[hex, Value::Number(Number::Int(5))],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::List(colors) => {
                assert_eq!(colors.len(), 5);
                for color in colors {
                    assert!(matches!(color, Value::String(_)));
                }
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_contrast_ratio() {
        let result = execute(
            "contrast_ratio",
            &[
                Value::String("#FFFFFF".to_string()),
                Value::String("#000000".to_string()),
            ],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::Number(Number::Float(ratio)) => {
                assert!(ratio > 20.0);
            }
            _ => panic!("Expected float"),
        }
    }

    #[test]
    fn test_hex_to_hsl() {
        let hex = Value::String("#FF0000".to_string());
        let result = execute("hex_to_hsl", &[hex], "", 0).unwrap();
        match result {
            Value::Dict(d) => {
                // Red should have hue around 0
                match d.get("h") {
                    Some(Value::Number(Number::Float(h))) if *h < 10.0 || *h > 350.0 => (),
                    _ => panic!("Expected hue near 0 for red"),
                }
            }
            _ => panic!("Expected dict"),
        }
    }

    #[test]
    fn test_hsl_to_hex() {
        let result = execute(
            "hsl_to_hex",
            &[
                Value::Number(Number::Int(0)),
                Value::Number(Number::Int(100)),
                Value::Number(Number::Int(50)),
            ],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => {
                // Should be a hex color
                assert!(s.starts_with('#'));
                assert_eq!(s.len(), 7);
            }
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_darken() {
        let hex = Value::String("#808080".to_string());
        let result = execute(
            "darken",
            &[hex.clone(), Value::Number(Number::Int(10))],
            "",
            0,
        )
        .unwrap();
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_saturate() {
        let hex = Value::String("#808080".to_string());
        let result = execute(
            "saturate",
            &[hex.clone(), Value::Number(Number::Int(20))],
            "",
            0,
        )
        .unwrap();
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_invert_color() {
        let hex = Value::String("#FF0000".to_string());
        let result = execute("invert_color", &[hex], "", 0).unwrap();
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_grayscale() {
        let hex = Value::String("#FF0000".to_string());
        let result = execute("grayscale", &[hex], "", 0).unwrap();
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_palette_analogous() {
        let hex = Value::String("#FF5733".to_string());
        let result = execute("palette_analogous", &[hex], "", 0).unwrap();
        match result {
            Value::List(colors) => {
                assert!(colors.len() >= 2);
                for color in colors {
                    assert!(matches!(color, Value::String(_)));
                }
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_palette_triadic() {
        let hex = Value::String("#FF5733".to_string());
        let result = execute("palette_triadic", &[hex], "", 0).unwrap();
        match result {
            Value::List(colors) => {
                assert_eq!(colors.len(), 3);
                for color in colors {
                    assert!(matches!(color, Value::String(_)));
                }
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_blend_colors() {
        let result = execute(
            "blend_colors",
            &[
                Value::String("#FF0000".to_string()),
                Value::String("#0000FF".to_string()),
                Value::Number(Number::Float(0.5)),
            ],
            "",
            0,
        )
        .unwrap();
        match result {
            Value::String(s) => {
                assert!(s.starts_with('#'));
                assert_eq!(s.len(), 7);
            }
            _ => panic!("Expected string"),
        }
    }
}
