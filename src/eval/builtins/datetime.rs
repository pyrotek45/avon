//! Date/Time functions: date_add, date_diff, date_format, date_parse, now, timestamp, timezone

use crate::common::{EvalError, Number, Value};
use crate::eval::value_to_string_auto;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

/// Names of datetime builtins
pub const NAMES: &[&str] = &[
    "date_add",
    "date_diff",
    "date_format",
    "date_parse",
    "now",
    "timestamp",
    "timezone",
];

/// Get arity for datetime functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "now" | "timestamp" | "timezone" => Some(1),
        "date_add" | "date_diff" | "date_format" | "date_parse" => Some(2),
        _ => None,
    }
}

/// Check if name is a datetime builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

// Helper function to parse duration strings like "1d", "2h", "30m", etc.
fn parse_duration(s: &str) -> Result<chrono::Duration, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty duration string".to_string());
    }

    // Find where the number ends and unit begins
    let mut num_end = 0;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() || c == '-' || c == '+' || c == '.' {
            num_end = i + 1;
        } else {
            break;
        }
    }

    if num_end == 0 {
        return Err(format!("no number found in '{}'", s));
    }

    let num_str = &s[..num_end];
    let unit = s[num_end..].trim();

    let value: i64 = num_str
        .parse()
        .map_err(|_| format!("invalid number: '{}'", num_str))?;

    match unit {
        "s" | "sec" | "second" | "seconds" => Ok(chrono::Duration::seconds(value)),
        "m" | "min" | "minute" | "minutes" => Ok(chrono::Duration::minutes(value)),
        "h" | "hour" | "hours" => Ok(chrono::Duration::hours(value)),
        "d" | "day" | "days" => Ok(chrono::Duration::days(value)),
        "w" | "week" | "weeks" => Ok(chrono::Duration::weeks(value)),
        "y" | "year" | "years" => Ok(chrono::Duration::days(value * 365)),
        "" => Err("missing unit (use s/m/h/d/w/y)".to_string()),
        _ => Err(format!("unknown unit: '{}' (use s/m/h/d/w/y)", unit)),
    }
}

/// Execute a datetime builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "now" => {
            // now :: String
            // Returns current date/time in ISO 8601 format (RFC 3339)
            let now = Local::now();
            Ok(Value::String(now.to_rfc3339()))
        }
        "date_format" => {
            // date_format :: String -> String -> String
            // Format a date string with a given format
            let date_str = value_to_string_auto(&args[0], source, line)?;
            let format_str = value_to_string_auto(&args[1], source, line)?;

            match DateTime::parse_from_rfc3339(&date_str) {
                Ok(dt) => {
                    // Try formatting - catch panics from invalid format strings
                    let formatted = std::panic::catch_unwind(|| dt.format(&format_str).to_string());
                    match formatted {
                        Ok(s) => Ok(Value::String(s)),
                        Err(_) => Err(EvalError::new(
                            format!("invalid date format string: '{}'", format_str),
                            None,
                            None,
                            line,
                        )),
                    }
                }
                Err(_) => {
                    // Try parsing as naive datetime
                    match NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S") {
                        Ok(naive) => {
                            let local = match Local.from_local_datetime(&naive).earliest() {
                                Some(dt) => dt,
                                None => {
                                    return Err(EvalError::new(
                                        format!("ambiguous or invalid local time: '{}'", date_str),
                                        None,
                                        None,
                                        line,
                                    ));
                                }
                            };
                            // Try formatting - catch panics from invalid format strings
                            let formatted =
                                std::panic::catch_unwind(|| local.format(&format_str).to_string());
                            match formatted {
                                Ok(s) => Ok(Value::String(s)),
                                Err(_) => Err(EvalError::new(
                                    format!("invalid date format string: '{}'", format_str),
                                    None,
                                    None,
                                    line,
                                )),
                            }
                        }
                        Err(_) => Err(EvalError::new(
                            format!(
                                "could not parse date '{}' - expected ISO 8601 format",
                                date_str
                            ),
                            None,
                            None,
                            line,
                        )),
                    }
                }
            }
        }
        "date_parse" => {
            // date_parse :: String -> String -> String
            // Parse a date string with a given format and return ISO 8601
            let date_str = value_to_string_auto(&args[0], source, line)?;
            let format_str = value_to_string_auto(&args[1], source, line)?;

            match NaiveDateTime::parse_from_str(&date_str, &format_str) {
                Ok(naive) => {
                    let local = match Local.from_local_datetime(&naive).earliest() {
                        Some(dt) => dt,
                        None => {
                            return Err(EvalError::new(
                                format!("ambiguous or invalid local time: '{}'", date_str),
                                None,
                                None,
                                line,
                            ));
                        }
                    };
                    Ok(Value::String(local.to_rfc3339()))
                }
                Err(_) => {
                    // Try parsing just date
                    match chrono::NaiveDate::parse_from_str(&date_str, &format_str) {
                        Ok(date) => {
                            let naive = date
                                .and_hms_opt(0, 0, 0)
                                .unwrap_or(NaiveDateTime::default());
                            let local = match Local.from_local_datetime(&naive).earliest() {
                                Some(dt) => dt,
                                None => {
                                    return Err(EvalError::new(
                                        format!("ambiguous or invalid local time: '{}'", date_str),
                                        None,
                                        None,
                                        line,
                                    ));
                                }
                            };
                            Ok(Value::String(local.to_rfc3339()))
                        }
                        Err(_) => Err(EvalError::new(
                            format!(
                                "could not parse date '{}' with format '{}'",
                                date_str, format_str
                            ),
                            None,
                            None,
                            line,
                        )),
                    }
                }
            }
        }
        "date_add" => {
            // date_add :: String -> String -> String
            // Add duration to a date
            let date_str = value_to_string_auto(&args[0], source, line)?;
            let duration_str = value_to_string_auto(&args[1], source, line)?;

            let dt = DateTime::parse_from_rfc3339(&date_str).map_err(|_| {
                EvalError::new(
                    format!(
                        "could not parse date '{}' - expected ISO 8601 format",
                        date_str
                    ),
                    None,
                    None,
                    line,
                )
            })?;

            let duration = parse_duration(&duration_str).map_err(|e| {
                EvalError::new(
                    format!("invalid duration '{}': {}", duration_str, e),
                    None,
                    None,
                    line,
                )
            })?;

            let new_dt = dt + duration;
            Ok(Value::String(new_dt.to_rfc3339()))
        }
        "date_diff" => {
            // date_diff :: String -> String -> Number
            // Calculate difference between two dates in seconds
            let date1_str = value_to_string_auto(&args[0], source, line)?;
            let date2_str = value_to_string_auto(&args[1], source, line)?;

            let dt1 = DateTime::parse_from_rfc3339(&date1_str).map_err(|_| {
                EvalError::new(
                    format!(
                        "could not parse date '{}' - expected ISO 8601 format",
                        date1_str
                    ),
                    None,
                    None,
                    line,
                )
            })?;

            let dt2 = DateTime::parse_from_rfc3339(&date2_str).map_err(|_| {
                EvalError::new(
                    format!(
                        "could not parse date '{}' - expected ISO 8601 format",
                        date2_str
                    ),
                    None,
                    None,
                    line,
                )
            })?;

            let diff = dt1.signed_duration_since(dt2);
            Ok(Value::Number(Number::Int(diff.num_seconds())))
        }
        "timestamp" => {
            // timestamp :: Number
            // Returns current Unix timestamp (seconds since epoch)
            let now = Utc::now();
            Ok(Value::Number(Number::Int(now.timestamp())))
        }
        "timezone" => {
            // timezone :: String
            // Returns current timezone offset (e.g., "+00:00", "-05:00")
            let now = Local::now();
            Ok(Value::String(now.offset().to_string()))
        }
        _ => Err(EvalError::new(
            format!("unknown datetime function: {}", name),
            None,
            None,
            line,
        )),
    }
}
