//! File I/O functions: basename, dirname, exists, fill_template, import, import_git, json_parse, readfile, readlines, walkdir, glob, relpath, abspath, yaml_parse, toml_parse

use crate::common::{EvalError, Number, Value};
use crate::eval::{eval, initial_builtins, value_to_path_string};
use crate::lexer::tokenize;
use crate::parser::parse;
use glob::glob;
use pathdiff::diff_paths;
use std::collections::HashMap;

/// Names of file I/O builtins
pub const NAMES: &[&str] = &[
    "abspath",
    "basename",
    "csv_parse",
    "dirname",
    "exists",
    "fill_template",
    "glob",
    "import",
    "import_git",
    "json_parse",
    "readfile",
    "readlines",
    "relpath",
    "toml_parse",
    "walkdir",
    "yaml_parse",
];

/// Get arity for file I/O functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "abspath" | "basename" | "csv_parse" | "dirname" | "exists" | "glob" | "import"
        | "json_parse" | "readfile" | "readlines" | "toml_parse" | "walkdir" | "yaml_parse" => {
            Some(1)
        }
        "fill_template" | "import_git" | "relpath" => Some(2),
        _ => None,
    }
}

/// Check if name is a file I/O builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a file I/O builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "import" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let data = std::fs::read_to_string(&p).map_err(|e| {
                EvalError::new(format!("failed to import {}: {}", p, e), None, None, line)
            })?;
            let tokens = tokenize(data.clone())?;
            let ast = parse(tokens);
            let mut env = initial_builtins();
            let val = eval(ast.program, &mut env, &data)?;
            Ok(val)
        }
        "import_git" => {
            // Arguments: repo_path (owner/repo/path/to/file.av), commit_hash
            if args.len() != 2 {
                return Err(EvalError::new(
                    "import_git expects 2 arguments: repo_path and commit_hash".to_string(),
                    None,
                    None,
                    line,
                ));
            }

            let git_path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(EvalError::new(
                    "import_git: first argument (repo_path) must be a string (e.g. 'owner/repo/file.av')".to_string(),
                    None,
                    None,
                    line,
                )),
            };

            let commit_hash = match &args[1] {
                Value::String(s) => s.clone(),
                _ => {
                    return Err(EvalError::new(
                        "import_git: second argument (commit_hash) must be a string".to_string(),
                        None,
                        None,
                        line,
                    ))
                }
            };

            // Parse the git path: owner/repo/path/to/file
            let parts: Vec<&str> = git_path.split('/').collect();
            if parts.len() < 3 {
                return Err(EvalError::new(
                    format!(
                        "import_git path must be 'owner/repo/path/to/file.av', got '{}'",
                        git_path
                    ),
                    None,
                    None,
                    line,
                ));
            }

            let owner = parts[0];
            let repo = parts[1];
            let file_path = parts[2..].join("/");

            // Construct GitHub raw content URL using the specific commit hash
            let url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, commit_hash, file_path
            );

            // Download the file from GitHub with error handling
            let response = ureq::get(&url).call().map_err(|e| {
                let error_str = format!("{}", e);
                let error_msg = if error_str.contains("404") || error_str.contains("not found") {
                    format!(
                        "import_git: file not found (404) at {}. Verify:\n  - owner: '{}'\n  - repo: '{}'\n  - file path: '{}'\n  - commit hash: '{}'",
                        url, owner, repo, file_path, commit_hash
                    )
                } else if error_str.contains("timeout") || error_str.contains("time") {
                    format!(
                        "import_git: request timed out downloading from {}. Check network connection.",
                        url
                    )
                } else {
                    format!(
                        "import_git: failed to download from {}: {}",
                        url, e
                    )
                };
                EvalError::new(error_msg, None, None, line)
            })?;

            let status = response.status();
            if !status.is_success() {
                let error_msg = if status == 404 {
                    format!(
                        "import_git: file not found (404) at {}. Check:\n  - owner: '{}'\n  - repo: '{}'\n  - file path: '{}'\n  - commit hash: '{}'",
                        url, owner, repo, file_path, commit_hash
                    )
                } else {
                    format!(
                        "import_git: failed to fetch {} (HTTP {}). File may not exist at this commit.",
                        url, status
                    )
                };
                return Err(EvalError::new(error_msg, None, None, line));
            }

            let content = response.into_body().read_to_string().map_err(|e| {
                EvalError::new(
                    format!("import_git: failed to read response from {}: {}", url, e),
                    None,
                    None,
                    line,
                )
            })?;

            // Ensure we got actual content
            if content.is_empty() {
                return Err(EvalError::new(
                    format!(
                        "import_git: downloaded file is empty from {}. File may be corrupted or deleted.",
                        url
                    ),
                    None,
                    None,
                    line,
                ));
            }

            // Parse and evaluate the downloaded Avon code
            let tokens = match tokenize(content.clone()) {
                Ok(t) => t,
                Err(e) => {
                    return Err(EvalError::new(
                        format!(
                            "import_git: failed to parse downloaded file from {}. Syntax error: {}",
                            url, e
                        ),
                        None,
                        None,
                        line,
                    ))
                }
            };

            let ast = parse(tokens);
            let mut env = initial_builtins();
            match eval(ast.program, &mut env, &content) {
                Ok(val) => Ok(val),
                Err(e) => Err(EvalError::new(
                    format!(
                        "import_git: failed to evaluate downloaded file from {}. Error: {}",
                        url, e.message
                    ),
                    None,
                    None,
                    line,
                )),
            }
        }
        "readfile" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let data = std::fs::read_to_string(&p).map_err(|e| {
                EvalError::new(format!("failed to read {}: {}", p, e), None, None, line)
            })?;
            Ok(Value::String(data))
        }
        "readlines" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let data = std::fs::read_to_string(&p).map_err(|e| {
                EvalError::new(format!("failed to read {}: {}", p, e), None, None, line)
            })?;
            let lines: Vec<Value> = data.lines().map(|s| Value::String(s.to_string())).collect();
            Ok(Value::List(lines))
        }
        "fill_template" => {
            // Args: filename (string or path), substitutions (dict or list of [key, value] pairs)
            let pathv = &args[0];
            let subsv = &args[1];

            let filename = value_to_path_string(pathv, source)?;
            // Read the template file
            let mut template = std::fs::read_to_string(&filename).map_err(|e| {
                EvalError::new(
                    "fill_template".to_string(),
                    Some("file".to_string()),
                    Some(e.to_string()),
                    line,
                )
            })?;

            // Process substitutions - accept both dict and list of pairs
            match subsv {
                Value::Dict(map) => {
                    // Modern approach: use dict directly
                    for (key, val) in map.iter() {
                        let val_str = val.to_string(source);
                        let placeholder = format!("{{{}}}", key);
                        template = template.replace(&placeholder, &val_str);
                    }
                    Ok(Value::String(template))
                }
                Value::List(pairs) => {
                    // Legacy approach: list of [key, value] pairs
                    for pair in pairs {
                        if let Value::List(kv) = pair {
                            if kv.len() != 2 {
                                return Err(EvalError::new(
                                    "fill_template",
                                    Some("[key, value] pair".to_string()),
                                    Some(format!("list of {}", kv.len())),
                                    line,
                                ));
                            }

                            let key = match &kv[0] {
                                Value::String(s) => s.clone(),
                                other => {
                                    return Err(EvalError::type_mismatch(
                                        "string",
                                        match other {
                                            Value::Number(_) => "number",
                                            Value::List(_) => "list",
                                            Value::Bool(_) => "bool",
                                            Value::Function { .. } => "function",
                                            _ => "unknown",
                                        },
                                        line,
                                    ))
                                }
                            };

                            let val = kv[1].to_string(source);
                            let placeholder = format!("{{{}}}", key);
                            template = template.replace(&placeholder, &val);
                        } else {
                            return Err(EvalError::new(
                                "fill_template",
                                Some("list".to_string()),
                                Some(pair.to_string(source)),
                                line,
                            ));
                        }
                    }
                    Ok(Value::String(template))
                }
                _ => Err(EvalError::type_mismatch(
                    "dict or list of pairs",
                    match subsv {
                        Value::String(_) => "string",
                        Value::Number(_) => "number",
                        Value::Bool(_) => "bool",
                        Value::Function { .. } => "function",
                        _ => "unknown",
                    },
                    line,
                )),
            }
        }
        "walkdir" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let mut out = Vec::new();
            let base = std::path::Path::new(&p);
            if base.exists() {
                let mut stack = vec![base.to_path_buf()];
                while let Some(cur) = stack.pop() {
                    if let Ok(md) = std::fs::read_dir(&cur) {
                        for e in md.flatten() {
                            let pth = e.path();
                            out.push(Value::String(pth.to_string_lossy().to_string()));
                            if pth.is_dir() {
                                stack.push(pth);
                            }
                        }
                    }
                }
            }
            Ok(Value::List(out))
        }
        "json_parse" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let data = std::fs::read_to_string(p).map_err(|e| {
                    EvalError::new(format!("failed to read {}: {}", p, e), None, None, line)
                })?;
                let jr: serde_json::Value = serde_json::from_str(&data).map_err(|e| {
                    EvalError::new(format!("json parse error: {}", e), None, None, line)
                })?;
                fn conv(j: &serde_json::Value) -> Value {
                    match j {
                        serde_json::Value::Null => Value::None,
                        serde_json::Value::Bool(b) => Value::Bool(*b),
                        serde_json::Value::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                Value::Number(Number::Int(i))
                            } else if let Some(f) = n.as_f64() {
                                Value::Number(Number::Float(f))
                            } else {
                                Value::None
                            }
                        }
                        serde_json::Value::String(s) => Value::String(s.clone()),
                        serde_json::Value::Array(a) => Value::List(a.iter().map(conv).collect()),
                        serde_json::Value::Object(o) => {
                            // Convert JSON object to Dict (hash map)
                            let mut map = HashMap::new();
                            for (k, v) in o.iter() {
                                map.insert(k.clone(), conv(v));
                            }
                            Value::Dict(map)
                        }
                    }
                }
                Ok(conv(&jr))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    pathv.to_string(source),
                    line,
                ))
            }
        }
        "exists" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            Ok(Value::Bool(std::path::Path::new(&p).exists()))
        }
        "basename" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let b = std::path::Path::new(&p)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            Ok(Value::String(b))
        }
        "dirname" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let d = std::path::Path::new(&p)
                .parent()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            Ok(Value::String(d))
        }
        "glob" => {
            let pattern_val = &args[0];
            if let Value::String(pattern) = pattern_val {
                let mut matches = Vec::new();
                let paths = glob(pattern)
                    .map_err(|e| EvalError::new(format!("glob error: {}", e), None, None, line))?;
                for entry in paths {
                    match entry {
                        Ok(path) => {
                            matches.push(Value::String(path.to_string_lossy().to_string()));
                        }
                        Err(e) => {
                            return Err(EvalError::new(
                                format!("glob entry error: {}", e),
                                None,
                                None,
                                line,
                            ));
                        }
                    }
                }
                Ok(Value::List(matches))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    pattern_val.to_string(source),
                    line,
                ))
            }
        }
        "abspath" => {
            let pathv = &args[0];
            let p = value_to_path_string(pathv, source)?;
            let abs = std::fs::canonicalize(&p).map_err(|e| {
                EvalError::new(
                    format!("failed to resolve path {}: {}", p, e),
                    None,
                    None,
                    line,
                )
            })?;
            Ok(Value::String(abs.to_string_lossy().to_string()))
        }
        "relpath" => {
            let base_val = &args[0];
            let target_val = &args[1];
            let base = value_to_path_string(base_val, source)?;
            let target = value_to_path_string(target_val, source)?;

            let diff = diff_paths(&target, &base).ok_or_else(|| {
                EvalError::new(
                    format!(
                        "could not calculate relative path from {} to {}",
                        base, target
                    ),
                    None,
                    None,
                    line,
                )
            })?;
            Ok(Value::String(diff.to_string_lossy().to_string()))
        }
        "yaml_parse" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let data = std::fs::read_to_string(p).map_err(|e| {
                    EvalError::new(format!("failed to read {}: {}", p, e), None, None, line)
                })?;
                let yr: serde_yaml::Value = serde_yaml::from_str(&data).map_err(|e| {
                    EvalError::new(format!("yaml parse error: {}", e), None, None, line)
                })?;
                fn conv(y: &serde_yaml::Value) -> Value {
                    match y {
                        serde_yaml::Value::Null => Value::None,
                        serde_yaml::Value::Bool(b) => Value::Bool(*b),
                        serde_yaml::Value::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                Value::Number(Number::Int(i))
                            } else if let Some(f) = n.as_f64() {
                                Value::Number(Number::Float(f))
                            } else {
                                Value::None
                            }
                        }
                        serde_yaml::Value::String(s) => Value::String(s.clone()),
                        serde_yaml::Value::Sequence(a) => Value::List(a.iter().map(conv).collect()),
                        serde_yaml::Value::Mapping(m) => {
                            let mut map = HashMap::new();
                            for (k, v) in m {
                                if let serde_yaml::Value::String(ks) = k {
                                    map.insert(ks.clone(), conv(v));
                                } else {
                                    let ks = format!("{:?}", k);
                                    map.insert(ks, conv(v));
                                }
                            }
                            Value::Dict(map)
                        }
                        serde_yaml::Value::Tagged(t) => conv(&t.value),
                    }
                }
                Ok(conv(&yr))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    pathv.to_string(source),
                    line,
                ))
            }
        }
        "toml_parse" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let data = std::fs::read_to_string(p).map_err(|e| {
                    EvalError::new(format!("failed to read {}: {}", p, e), None, None, line)
                })?;
                let tr: toml::Value = toml::from_str(&data).map_err(|e| {
                    EvalError::new(format!("toml parse error: {}", e), None, None, line)
                })?;
                fn conv(t: &toml::Value) -> Value {
                    match t {
                        toml::Value::String(s) => Value::String(s.clone()),
                        toml::Value::Integer(i) => Value::Number(Number::Int(*i)),
                        toml::Value::Float(f) => Value::Number(Number::Float(*f)),
                        toml::Value::Boolean(b) => Value::Bool(*b),
                        toml::Value::Datetime(d) => Value::String(d.to_string()),
                        toml::Value::Array(a) => Value::List(a.iter().map(conv).collect()),
                        toml::Value::Table(t) => {
                            let mut map = HashMap::new();
                            for (k, v) in t {
                                map.insert(k.clone(), conv(v));
                            }
                            Value::Dict(map)
                        }
                    }
                }
                Ok(conv(&tr))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    pathv.to_string(source),
                    line,
                ))
            }
        }
        "csv_parse" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let mut reader = csv::Reader::from_path(p).map_err(|e| {
                    EvalError::new(format!("failed to read csv {}: {}", p, e), None, None, line)
                })?;

                let mut rows = Vec::new();

                // Try to read headers
                let headers = reader.headers().cloned().unwrap_or_default();
                let has_headers = !headers.is_empty();

                for result in reader.records() {
                    let record = result.map_err(|e| {
                        EvalError::new(format!("csv record error: {}", e), None, None, line)
                    })?;

                    if has_headers {
                        let mut row_dict = HashMap::new();
                        for (i, field) in record.iter().enumerate() {
                            if let Some(header) = headers.get(i) {
                                row_dict
                                    .insert(header.to_string(), Value::String(field.to_string()));
                            }
                        }
                        rows.push(Value::Dict(row_dict));
                    } else {
                        let row_list: Vec<Value> = record
                            .iter()
                            .map(|s| Value::String(s.to_string()))
                            .collect();
                        rows.push(Value::List(row_list));
                    }
                }
                Ok(Value::List(rows))
            } else {
                Err(EvalError::type_mismatch(
                    "string",
                    pathv.to_string(source),
                    line,
                ))
            }
        }
        _ => Err(EvalError::new(
            format!("unknown file_io function: {}", name),
            None,
            None,
            line,
        )),
    }
}
