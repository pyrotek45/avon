//! Formatting functions: center, format_*, truncate

use crate::common::{EvalError, Number, Value};

/// Names of formatting builtins
pub const NAMES: &[&str] = &[
    "center",
    "format_binary",
    "format_bool",
    "format_bytes",
    "format_csv",
    "format_currency",
    "format_float",
    "format_hex",
    "format_html",
    "format_ini",
    "format_int",
    "format_json",
    "format_list",
    "format_octal",
    "format_opml",
    "format_percent",
    "format_scientific",
    "format_table",
    "format_toml",
    "format_xml",
    "format_yaml",
    "truncate",
];

/// Get arity for formatting functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "format_binary" | "format_bytes" | "format_csv" | "format_hex" | "format_html"
        | "format_ini" | "format_json" | "format_octal" | "format_opml" | "format_toml"
        | "format_xml" | "format_yaml" => Some(1),
        "center" | "format_bool" | "format_currency" | "format_float" | "format_int"
        | "format_list" | "format_percent" | "format_scientific" | "format_table" | "truncate" => {
            Some(2)
        }
        _ => None,
    }
}

/// Check if name is a formatting builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a formatting builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "format_int" => {
            let val = &args[0];
            let width = &args[1];
            if let (Value::Number(num), Value::Number(Number::Int(w))) = (val, width) {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                let formatted = if *w > 0 {
                    // Limit width to reasonable value to prevent hang/OOM
                    if *w > 100000 {
                        return Err(EvalError::new(
                            format!("format_int width {} is too large (max 100,000)", w),
                            None,
                            None,
                            line,
                        ));
                    }
                    format!("{:0width$}", int_val, width = *w as usize)
                } else {
                    format!("{}", int_val)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, number",
                    format!("{}, {}", val.to_string(source), width.to_string(source)),
                    line,
                ))
            }
        }
        "format_float" => {
            let val = &args[0];
            let precision = &args[1];
            if let (Value::Number(num), Value::Number(Number::Int(p))) = (val, precision) {
                let float_val = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = if *p >= 0 {
                    // Limit precision to reasonable value to prevent hang/OOM
                    if *p > 100 {
                        return Err(EvalError::new(
                            format!("format_float precision {} is too large (max 100)", p),
                            None,
                            None,
                            line,
                        ));
                    }
                    format!("{:.prec$}", float_val, prec = *p as usize)
                } else {
                    format!("{}", float_val)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, number",
                    format!("{}, {}", val.to_string(source), precision.to_string(source)),
                    line,
                ))
            }
        }
        "format_hex" => {
            let val = &args[0];
            if let Value::Number(num) = val {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                Ok(Value::String(format!("{:x}", int_val)))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    val.to_string(source),
                    line,
                ))
            }
        }
        "format_octal" => {
            let val = &args[0];
            if let Value::Number(num) = val {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                Ok(Value::String(format!("{:o}", int_val)))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    val.to_string(source),
                    line,
                ))
            }
        }
        "format_binary" => {
            let val = &args[0];
            if let Value::Number(num) = val {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                Ok(Value::String(format!("{:b}", int_val)))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    val.to_string(source),
                    line,
                ))
            }
        }
        "format_scientific" => {
            let val = &args[0];
            let precision = &args[1];
            if let (Value::Number(num), Value::Number(Number::Int(p))) = (val, precision) {
                let float_val = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = if *p >= 0 {
                    // Limit precision to reasonable value to prevent hang/OOM
                    if *p > 100 {
                        return Err(EvalError::new(
                            format!("format_scientific precision {} is too large (max 100)", p),
                            None,
                            None,
                            line,
                        ));
                    }
                    format!("{:.prec$e}", float_val, prec = *p as usize)
                } else {
                    format!("{:e}", float_val)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, number",
                    format!("{}, {}", val.to_string(source), precision.to_string(source)),
                    line,
                ))
            }
        }
        "format_bytes" => {
            let val = &args[0];
            if let Value::Number(num) = val {
                let bytes = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = if bytes < 1024.0 {
                    format!("{} B", bytes as i64)
                } else if bytes < 1024.0 * 1024.0 {
                    format!("{:.2} KB", bytes / 1024.0)
                } else if bytes < 1024.0 * 1024.0 * 1024.0 {
                    format!("{:.2} MB", bytes / (1024.0 * 1024.0))
                } else if bytes < 1024.0 * 1024.0 * 1024.0 * 1024.0 {
                    format!("{:.2} GB", bytes / (1024.0 * 1024.0 * 1024.0))
                } else {
                    format!("{:.2} TB", bytes / (1024.0 * 1024.0 * 1024.0 * 1024.0))
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number",
                    val.to_string(source),
                    line,
                ))
            }
        }
        "format_list" => {
            let list = &args[0];
            let separator = &args[1];
            if let (Value::List(items), Value::String(sep)) = (list, separator) {
                let strings: Vec<String> = items.iter().map(|v| v.to_string(source)).collect();
                Ok(Value::String(strings.join(sep)))
            } else {
                Err(EvalError::type_mismatch(
                    "list, string",
                    format!(
                        "{}, {}",
                        list.to_string(source),
                        separator.to_string(source)
                    ),
                    line,
                ))
            }
        }
        "format_table" => {
            let table = &args[0];
            let separator = &args[1];

            if let Value::String(sep) = separator {
                let rows: Vec<Vec<String>> = match table {
                    Value::Dict(dict) => {
                        let keys_row: Vec<String> = dict.keys().cloned().collect();
                        let values_row: Vec<String> =
                            dict.values().map(|v| v.to_string(source)).collect();
                        vec![keys_row, values_row]
                    }
                    Value::List(rows) => {
                        let mut result = Vec::new();
                        for row in rows {
                            if let Value::List(cols) = row {
                                let strings: Vec<String> =
                                    cols.iter().map(|v| v.to_string(source)).collect();
                                result.push(strings);
                            } else {
                                result.push(vec![row.to_string(source)]);
                            }
                        }
                        result
                    }
                    _ => {
                        return Err(EvalError::type_mismatch(
                            "list of lists or dict",
                            table.to_string(source),
                            line,
                        ));
                    }
                };

                let lines: Vec<String> = rows.iter().map(|row| row.join(sep)).collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    separator.to_string(source),
                    line,
                ))
            }
        }
        "format_json" => {
            let val = &args[0];
            let json_str = format_json_value(val, source, line)?;
            Ok(Value::String(json_str))
        }
        "format_yaml" => {
            let val = &args[0];
            let yaml_val = value_to_yaml(val, source);
            let yaml_str = serde_yaml::to_string(&yaml_val).map_err(|e| {
                EvalError::new(format!("YAML serialization error: {}", e), None, None, line)
            })?;
            Ok(Value::String(yaml_str))
        }
        "format_toml" => {
            let val = &args[0];
            let toml_val = value_to_toml(val, source);
            let toml_str = toml::to_string_pretty(&toml_val).map_err(|e| {
                EvalError::new(format!("TOML serialization error: {}", e), None, None, line)
            })?;
            Ok(Value::String(toml_str))
        }
        "format_ini" => {
            let val = &args[0];
            let ini_str = value_to_ini(val, source, line)?;
            Ok(Value::String(ini_str))
        }
        "format_html" => {
            let val = &args[0];
            let html_str = value_to_html(val, source, 0);
            Ok(Value::String(html_str))
        }
        "format_xml" => {
            let val = &args[0];
            let xml_str = value_to_xml(val, source, 0);
            Ok(Value::String(xml_str))
        }
        "format_opml" => {
            let val = &args[0];
            let opml_str = value_to_opml(val, source, line)?;
            Ok(Value::String(opml_str))
        }
        "format_csv" => {
            let val = &args[0];
            let mut wtr = csv::Writer::from_writer(vec![]);

            match val {
                Value::List(items) => {
                    if items.is_empty() {
                        return Ok(Value::String(String::new()));
                    }

                    // Check if it's a list of dicts or list of lists
                    if let Value::Dict(first) = &items[0] {
                        // List of Dicts: Use keys as headers
                        let headers: Vec<String> = first.keys().cloned().collect();
                        wtr.write_record(&headers).map_err(|e| {
                            EvalError::new(format!("csv write error: {}", e), None, None, line)
                        })?;

                        for item in items {
                            if let Value::Dict(d) = item {
                                let row: Vec<String> = headers
                                    .iter()
                                    .map(|h| {
                                        d.get(h).map(|v| v.to_string(source)).unwrap_or_default()
                                    })
                                    .collect();
                                wtr.write_record(&row).map_err(|e| {
                                    EvalError::new(
                                        format!("csv write error: {}", e),
                                        None,
                                        None,
                                        line,
                                    )
                                })?;
                            }
                        }
                    } else if let Value::List(_) = &items[0] {
                        // List of Lists: No headers, just data
                        for item in items {
                            if let Value::List(row) = item {
                                let row_strs: Vec<String> =
                                    row.iter().map(|v| v.to_string(source)).collect();
                                wtr.write_record(&row_strs).map_err(|e| {
                                    EvalError::new(
                                        format!("csv write error: {}", e),
                                        None,
                                        None,
                                        line,
                                    )
                                })?;
                            }
                        }
                    } else {
                        return Err(EvalError::type_mismatch(
                            "list of dicts or list of lists",
                            val.to_string(source),
                            line,
                        ));
                    }
                }
                _ => {
                    return Err(EvalError::type_mismatch(
                        "list",
                        val.to_string(source),
                        line,
                    ));
                }
            }

            let data = String::from_utf8(wtr.into_inner().map_err(|e| {
                EvalError::new(format!("csv flush error: {}", e), None, None, line)
            })?)
            .map_err(|e| EvalError::new(format!("csv utf8 error: {}", e), None, None, line))?;

            Ok(Value::String(data))
        }
        "format_currency" => {
            let val = &args[0];
            let symbol = &args[1];
            if let (Value::Number(num), Value::String(sym)) = (val, symbol) {
                let float_val = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = format!("{}{:.2}", sym, float_val);
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, string",
                    format!("{}, {}", val.to_string(source), symbol.to_string(source)),
                    line,
                ))
            }
        }
        "format_percent" => {
            let val = &args[0];
            let precision = &args[1];
            if let (Value::Number(num), Value::Number(Number::Int(p))) = (val, precision) {
                let float_val = match num {
                    Number::Int(i) => *i as f64,
                    Number::Float(f) => *f,
                };
                let formatted = if *p >= 0 {
                    // Limit precision to reasonable value to prevent hang/OOM
                    if *p > 100 {
                        return Err(EvalError::new(
                            format!("format_percent precision {} is too large (max 100)", p),
                            None,
                            None,
                            line,
                        ));
                    }
                    format!("{:.prec$}%", float_val * 100.0, prec = *p as usize)
                } else {
                    format!("{}%", float_val * 100.0)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch(
                    "number, number",
                    format!("{}, {}", val.to_string(source), precision.to_string(source)),
                    line,
                ))
            }
        }
        "format_bool" => {
            let val = &args[0];
            let format_style = &args[1];
            if let (Value::Bool(b), Value::String(style)) = (val, format_style) {
                let lower = style.to_lowercase();
                let result = match lower.as_str() {
                    "yesno" | "yes/no" => {
                        if *b {
                            "Yes"
                        } else {
                            "No"
                        }
                    }
                    "onoff" | "on/off" => {
                        if *b {
                            "On"
                        } else {
                            "Off"
                        }
                    }
                    "truefalse" | "true/false" => {
                        if *b {
                            "True"
                        } else {
                            "False"
                        }
                    }
                    "10" | "1/0" => {
                        if *b {
                            "1"
                        } else {
                            "0"
                        }
                    }
                    "enabled" | "enabled/disabled" => {
                        if *b {
                            "Enabled"
                        } else {
                            "Disabled"
                        }
                    }
                    "active" | "active/inactive" => {
                        if *b {
                            "Active"
                        } else {
                            "Inactive"
                        }
                    }
                    custom => {
                        let parts: Vec<&str> = custom.split('/').collect();
                        if parts.len() == 2 {
                            if *b {
                                parts[0]
                            } else {
                                parts[1]
                            }
                        } else if *b {
                            "true"
                        } else {
                            "false"
                        }
                    }
                };
                Ok(Value::String(result.to_string()))
            } else {
                Err(EvalError::type_mismatch(
                    "bool, string",
                    format!(
                        "{}, {}",
                        val.to_string(source),
                        format_style.to_string(source)
                    ),
                    line,
                ))
            }
        }
        "truncate" => {
            let text = &args[0];
            let max_len = &args[1];
            if let (Value::String(s), Value::Number(Number::Int(len))) = (text, max_len) {
                let max = (*len).max(0) as usize;
                if s.len() <= max {
                    Ok(Value::String(s.clone()))
                } else if max <= 3 {
                    Ok(Value::String(s.chars().take(max).collect()))
                } else {
                    let truncated: String = s.chars().take(max - 3).collect();
                    Ok(Value::String(format!("{}...", truncated)))
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string, number",
                    format!("{}, {}", text.to_string(source), max_len.to_string(source)),
                    line,
                ))
            }
        }
        "center" => {
            let text = &args[0];
            let width = &args[1];
            if let (Value::String(s), Value::Number(Number::Int(w))) = (text, width) {
                const MAX_WIDTH: i64 = 100_000;
                if *w > MAX_WIDTH {
                    return Err(EvalError::new(
                        format!("center width {} is too large (max {})", w, MAX_WIDTH),
                        None,
                        None,
                        line,
                    ));
                }
                let target_width = (*w).max(0) as usize;
                if s.len() >= target_width {
                    Ok(Value::String(s.clone()))
                } else {
                    let padding = target_width - s.len();
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    Ok(Value::String(format!(
                        "{}{}{}",
                        " ".repeat(left_pad),
                        s,
                        " ".repeat(right_pad)
                    )))
                }
            } else {
                Err(EvalError::type_mismatch(
                    "string, number",
                    format!("{}, {}", text.to_string(source), width.to_string(source)),
                    line,
                ))
            }
        }
        _ => Err(EvalError::new(
            format!("unknown formatting function: {}", name),
            None,
            None,
            line,
        )),
    }
}

// Helper for recursive JSON formatting
fn format_json_value(val: &Value, source: &str, _line: usize) -> Result<String, EvalError> {
    Ok(match val {
        Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        Value::Number(Number::Int(i)) => format!("{}", i),
        Value::Number(Number::Float(f)) => format!("{}", f),
        Value::Bool(b) => format!("{}", b),
        Value::List(items) => {
            let json_items: Vec<String> = items
                .iter()
                .map(|v| format_json_value(v, source, 0).unwrap_or_else(|_| v.to_string(source)))
                .collect();
            format!("[{}]", json_items.join(", "))
        }
        Value::Dict(dict) => {
            let json_pairs: Vec<String> = dict
                .iter()
                .map(|(k, v)| {
                    let json_val =
                        format_json_value(v, source, 0).unwrap_or_else(|_| v.to_string(source));
                    format!("\"{}\": {}", k, json_val)
                })
                .collect();
            format!("{{{}}}", json_pairs.join(", "))
        }
        Value::None => "null".to_string(),
        other => format!(
            "\"{}\"",
            other
                .to_string(source)
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
        ),
    })
}

// Helper to convert Value to serde_yaml::Value
fn value_to_yaml(val: &Value, source: &str) -> serde_yaml::Value {
    match val {
        Value::String(s) => serde_yaml::Value::String(s.clone()),
        Value::Number(Number::Int(i)) => serde_yaml::Value::Number(serde_yaml::Number::from(*i)),
        Value::Number(Number::Float(f)) => {
            // Handle special float cases
            if f.is_nan() || f.is_infinite() {
                serde_yaml::Value::String(f.to_string())
            } else {
                serde_yaml::Value::Number(serde_yaml::Number::from(*f))
            }
        }
        Value::Bool(b) => serde_yaml::Value::Bool(*b),
        Value::List(items) => {
            let yaml_items: Vec<serde_yaml::Value> =
                items.iter().map(|v| value_to_yaml(v, source)).collect();
            serde_yaml::Value::Sequence(yaml_items)
        }
        Value::Dict(dict) => {
            let mut map = serde_yaml::Mapping::new();
            for (k, v) in dict.iter() {
                map.insert(
                    serde_yaml::Value::String(k.clone()),
                    value_to_yaml(v, source),
                );
            }
            serde_yaml::Value::Mapping(map)
        }
        Value::None => serde_yaml::Value::Null,
        other => serde_yaml::Value::String(other.to_string(source)),
    }
}

// Helper to convert Value to toml::Value
fn value_to_toml(val: &Value, source: &str) -> toml::Value {
    match val {
        Value::String(s) => toml::Value::String(s.clone()),
        Value::Number(Number::Int(i)) => toml::Value::Integer(*i),
        Value::Number(Number::Float(f)) => {
            // Handle special float cases - TOML doesn't support NaN/Infinity
            if f.is_nan() || f.is_infinite() {
                toml::Value::String(f.to_string())
            } else {
                toml::Value::Float(*f)
            }
        }
        Value::Bool(b) => toml::Value::Boolean(*b),
        Value::List(items) => {
            let toml_items: Vec<toml::Value> =
                items.iter().map(|v| value_to_toml(v, source)).collect();
            toml::Value::Array(toml_items)
        }
        Value::Dict(dict) => {
            let mut map = toml::map::Map::new();
            for (k, v) in dict.iter() {
                map.insert(k.clone(), value_to_toml(v, source));
            }
            toml::Value::Table(map)
        }
        Value::None => toml::Value::String("null".to_string()),
        other => toml::Value::String(other.to_string(source)),
    }
}

/// Convert an Avon Dict to INI format string.
///
/// Expected structure: Dict of sections, where each section is a Dict of key=value pairs.
///   {section1: {key1: "val1", key2: "val2"}, section2: {key3: "val3"}}
///
/// A "global" section's keys are written without a section header.
fn value_to_ini(val: &Value, source: &str, line: usize) -> Result<String, EvalError> {
    match val {
        Value::Dict(sections) => {
            let mut out = String::new();
            let mut first = true;
            // Write "global" section first (no header)
            if let Some(Value::Dict(globals)) = sections.get("global") {
                let mut keys: Vec<&String> = globals.keys().collect();
                keys.sort();
                for k in keys {
                    if let Some(v) = globals.get(k) {
                        out.push_str(&format!("{}={}\n", k, v.to_string(source)));
                    }
                }
                first = false;
            }
            // Write named sections
            let mut section_names: Vec<&String> =
                sections.keys().filter(|k| k.as_str() != "global").collect();
            section_names.sort();
            for name in section_names {
                if let Some(Value::Dict(props)) = sections.get(name) {
                    if !first {
                        out.push('\n');
                    }
                    out.push_str(&format!("[{}]\n", name));
                    let mut keys: Vec<&String> = props.keys().collect();
                    keys.sort();
                    for k in keys {
                        if let Some(v) = props.get(k) {
                            out.push_str(&format!("{}={}\n", k, v.to_string(source)));
                        }
                    }
                    first = false;
                }
            }
            Ok(out)
        }
        _ => Err(EvalError::new(
            "format_ini: expected a Dict of sections".to_string(),
            None,
            None,
            line,
        )),
    }
}

/// Convert an Avon Value (Dict with tag/attrs/children/text) to an XML string.
///
/// Expected Dict structure (matches xml_parse output):
///   {tag: "div", attrs: {class: "main"}, children: [...]}
///   {tag: "p", text: "hello"}
///
/// Non-dict values are rendered as text content.
fn value_to_xml(val: &Value, source: &str, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    match val {
        Value::Dict(dict) => {
            let tag = match dict.get("tag") {
                Some(Value::String(s)) => s.clone(),
                _ => return format!("{}{}", indent, val.to_string(source)),
            };

            // Build opening tag with attributes
            let mut open = format!("<{}", tag);
            if let Some(Value::Dict(attrs)) = dict.get("attrs") {
                let mut attr_keys: Vec<&String> = attrs.keys().collect();
                attr_keys.sort();
                for k in attr_keys {
                    if let Some(v) = attrs.get(k) {
                        let v_str = match v {
                            Value::String(s) => s.clone(),
                            other => other.to_string(source),
                        };
                        open.push_str(&format!(" {}=\"{}\"", k, xml_escape(&v_str)));
                    }
                }
            }

            // Check for text content
            if let Some(Value::String(text)) = dict.get("text") {
                return format!("{}{}>{}</{}>", indent, open, xml_escape(text), tag);
            }

            // Check for children
            if let Some(Value::List(children)) = dict.get("children") {
                if children.is_empty() {
                    return format!("{}{} />", indent, open);
                }
                let mut out = format!("{}{}>\n", indent, open);
                for child in children {
                    out.push_str(&value_to_xml(child, source, depth + 1));
                    out.push('\n');
                }
                out.push_str(&format!("{}</{}>", indent, tag));
                return out;
            }

            // Self-closing tag
            format!("{}{} />", indent, open)
        }
        Value::String(s) => format!("{}{}", indent, xml_escape(s)),
        other => format!("{}{}", indent, xml_escape(&other.to_string(source))),
    }
}

/// Escape special XML characters
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// HTML void elements that must not have a closing tag
const HTML_VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// Escape special HTML characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Convert an Avon Value (Dict with tag/attrs/children/text) to an HTML string.
fn value_to_html(val: &Value, source: &str, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    match val {
        Value::Dict(dict) => {
            let tag = match dict.get("tag") {
                Some(Value::String(s)) => s.clone(),
                _ => return format!("{}{}", indent, val.to_string(source)),
            };

            let is_void = HTML_VOID_ELEMENTS.contains(&tag.as_str());

            // Build opening tag with attributes
            let mut open = format!("<{}", tag);
            if let Some(Value::Dict(attrs)) = dict.get("attrs") {
                let mut attr_keys: Vec<&String> = attrs.keys().collect();
                attr_keys.sort();
                for k in attr_keys {
                    if let Some(v) = attrs.get(k) {
                        let v_str = match v {
                            Value::String(s) => s.clone(),
                            other => other.to_string(source),
                        };
                        open.push_str(&format!(" {}=\"{}\"", k, html_escape(&v_str)));
                    }
                }
            }

            // Void elements: self-closing, no children or text
            if is_void {
                return format!("{}{}>", indent, open);
            }

            // Check for text content (non-empty text takes priority)
            let has_text = matches!(dict.get("text"), Some(Value::String(s)) if !s.is_empty());

            if has_text {
                if let Some(Value::String(text)) = dict.get("text") {
                    return format!("{}{}>{}</{}>", indent, open, html_escape(text), tag);
                }
            }

            // Check for children
            if let Some(Value::List(children)) = dict.get("children") {
                if children.is_empty() {
                    return format!("{}{}></{}>", indent, open, tag);
                }
                let mut out = format!("{}{}>\n", indent, open);
                for child in children {
                    out.push_str(&value_to_html(child, source, depth + 1));
                    out.push('\n');
                }
                out.push_str(&format!("{}</{}>", indent, tag));
                return out;
            }

            // Empty element (non-void)
            format!("{}{}></{}>", indent, open, tag)
        }
        Value::String(s) => format!("{}{}", indent, html_escape(s)),
        other => format!("{}{}", indent, html_escape(&other.to_string(source))),
    }
}
/// Convert an Avon Value (Dict with version/head/outlines) to an OPML string.
///
/// Expected Dict structure (matches opml_parse output):
///   {version: "2.0", head: {title: "My Feeds"}, outlines: [...]}
///
/// Each outline: {text: "News", type: "rss", xmlUrl: "...", children: [...]}
fn value_to_opml(val: &Value, source: &str, line: usize) -> Result<String, EvalError> {
    match val {
        Value::Dict(dict) => {
            let version = match dict.get("version") {
                Some(Value::String(s)) => s.clone(),
                _ => "2.0".to_string(),
            };

            let mut out = String::new();
            out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
            out.push_str(&format!("<opml version=\"{}\">\n", version));

            // Head section
            if let Some(Value::Dict(head)) = dict.get("head") {
                out.push_str("  <head>\n");
                let mut head_keys: Vec<&String> = head.keys().collect();
                head_keys.sort();
                for k in head_keys {
                    if let Some(v) = head.get(k) {
                        let text = match v {
                            Value::String(s) => s.clone(),
                            other => other.to_string(source),
                        };
                        out.push_str(&format!("    <{}>{}</{}>\n", k, xml_escape(&text), k));
                    }
                }
                out.push_str("  </head>\n");
            }

            // Body section
            out.push_str("  <body>\n");
            if let Some(Value::List(outlines)) = dict.get("outlines") {
                for outline in outlines {
                    write_opml_outline(&mut out, outline, source, 2);
                }
            }
            out.push_str("  </body>\n");
            out.push_str("</opml>");

            Ok(Value::String(out)).map(|v| match v {
                Value::String(s) => s,
                _ => unreachable!(),
            })
        }
        _ => Err(EvalError::type_mismatch(
            "dict with {version, head, outlines}",
            val.to_string(source),
            line,
        )),
    }
}

/// Write a single OPML outline element (recursive)
fn write_opml_outline(out: &mut String, val: &Value, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    if let Value::Dict(dict) = val {
        out.push_str(&format!("{}<outline", indent));

        // Write attributes (everything except "children")
        let mut attr_keys: Vec<&String> = dict.keys().filter(|k| *k != "children").collect();
        attr_keys.sort();
        // Put "text" first if present (OPML convention)
        if let Some(pos) = attr_keys.iter().position(|k| *k == "text") {
            let text_key = attr_keys.remove(pos);
            attr_keys.insert(0, text_key);
        }
        for k in &attr_keys {
            if let Some(v) = dict.get(*k) {
                let text = match v {
                    Value::String(s) => s.clone(),
                    other => other.to_string(source),
                };
                out.push_str(&format!(" {}=\"{}\"", k, xml_escape(&text)));
            }
        }

        // Children
        if let Some(Value::List(children)) = dict.get("children") {
            if children.is_empty() {
                out.push_str(" />\n");
            } else {
                out.push_str(">\n");
                for child in children {
                    write_opml_outline(out, child, source, depth + 1);
                }
                out.push_str(&format!("{}</outline>\n", indent));
            }
        } else {
            out.push_str(" />\n");
        }
    }
}
