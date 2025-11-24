use std::collections::HashMap;
use crate::common::{Expr, Value, Token, Chunk, Number, EvalError};
use crate::lexer::tokenize;
use crate::parser::parse;

pub fn initial_builtins() -> HashMap<String, Value> {
    let mut m = HashMap::new();
    // Core operations
    m.insert("concat".to_string(), Value::Builtin("concat".to_string(), Vec::new()));
    m.insert("map".to_string(), Value::Builtin("map".to_string(), Vec::new()));
    m.insert("filter".to_string(), Value::Builtin("filter".to_string(), Vec::new()));
    m.insert("fold".to_string(), Value::Builtin("fold".to_string(), Vec::new()));
    m.insert("import".to_string(), Value::Builtin("import".to_string(), Vec::new()));
    
    // File operations
    m.insert("readfile".to_string(), Value::Builtin("readfile".to_string(), Vec::new()));
    m.insert("exists".to_string(), Value::Builtin("exists".to_string(), Vec::new()));
    m.insert("basename".to_string(), Value::Builtin("basename".to_string(), Vec::new()));
    m.insert("dirname".to_string(), Value::Builtin("dirname".to_string(), Vec::new()));
    m.insert("readlines".to_string(), Value::Builtin("readlines".to_string(), Vec::new()));
    m.insert("walkdir".to_string(), Value::Builtin("walkdir".to_string(), Vec::new()));
    m.insert("json_parse".to_string(), Value::Builtin("json_parse".to_string(), Vec::new()));
    
    // String operations
    m.insert("upper".to_string(), Value::Builtin("upper".to_string(), Vec::new()));
    m.insert("lower".to_string(), Value::Builtin("lower".to_string(), Vec::new()));
    m.insert("trim".to_string(), Value::Builtin("trim".to_string(), Vec::new()));
    m.insert("split".to_string(), Value::Builtin("split".to_string(), Vec::new()));
    m.insert("join".to_string(), Value::Builtin("join".to_string(), Vec::new()));
    m.insert("replace".to_string(), Value::Builtin("replace".to_string(), Vec::new()));
    m.insert("contains".to_string(), Value::Builtin("contains".to_string(), Vec::new()));
    m.insert("starts_with".to_string(), Value::Builtin("starts_with".to_string(), Vec::new()));
    m.insert("ends_with".to_string(), Value::Builtin("ends_with".to_string(), Vec::new()));
    m.insert("length".to_string(), Value::Builtin("length".to_string(), Vec::new()));
    m.insert("repeat".to_string(), Value::Builtin("repeat".to_string(), Vec::new()));
    m.insert("pad_left".to_string(), Value::Builtin("pad_left".to_string(), Vec::new()));
    m.insert("pad_right".to_string(), Value::Builtin("pad_right".to_string(), Vec::new()));
    m.insert("indent".to_string(), Value::Builtin("indent".to_string(), Vec::new()));
    
    // String predicates
    m.insert("is_digit".to_string(), Value::Builtin("is_digit".to_string(), Vec::new()));
    m.insert("is_alpha".to_string(), Value::Builtin("is_alpha".to_string(), Vec::new()));
    m.insert("is_alphanumeric".to_string(), Value::Builtin("is_alphanumeric".to_string(), Vec::new()));
    m.insert("is_whitespace".to_string(), Value::Builtin("is_whitespace".to_string(), Vec::new()));
    m.insert("is_uppercase".to_string(), Value::Builtin("is_uppercase".to_string(), Vec::new()));
    m.insert("is_lowercase".to_string(), Value::Builtin("is_lowercase".to_string(), Vec::new()));
    m.insert("is_empty".to_string(), Value::Builtin("is_empty".to_string(), Vec::new()));
    
    // Type conversion/casting
    m.insert("to_string".to_string(), Value::Builtin("to_string".to_string(), Vec::new()));
    m.insert("to_int".to_string(), Value::Builtin("to_int".to_string(), Vec::new()));
    m.insert("to_float".to_string(), Value::Builtin("to_float".to_string(), Vec::new()));
    m.insert("to_bool".to_string(), Value::Builtin("to_bool".to_string(), Vec::new()));
    m.insert("format_int".to_string(), Value::Builtin("format_int".to_string(), Vec::new()));
    m.insert("format_float".to_string(), Value::Builtin("format_float".to_string(), Vec::new()));
    
    // List operations (advanced)
    m.insert("flatmap".to_string(), Value::Builtin("flatmap".to_string(), Vec::new()));
    m.insert("flatten".to_string(), Value::Builtin("flatten".to_string(), Vec::new()));
    
    // HTML helpers
    m.insert("html_escape".to_string(), Value::Builtin("html_escape".to_string(), Vec::new()));
    m.insert("html_tag".to_string(), Value::Builtin("html_tag".to_string(), Vec::new()));
    m.insert("html_attr".to_string(), Value::Builtin("html_attr".to_string(), Vec::new()));
    
    // Markdown helpers
    m.insert("md_heading".to_string(), Value::Builtin("md_heading".to_string(), Vec::new()));
    m.insert("md_link".to_string(), Value::Builtin("md_link".to_string(), Vec::new()));
    m.insert("md_code".to_string(), Value::Builtin("md_code".to_string(), Vec::new()));
    m.insert("md_list".to_string(), Value::Builtin("md_list".to_string(), Vec::new()));
    
    // System
    m.insert("os".to_string(), Value::String(std::env::consts::OS.to_string()));
    m
}

impl Value {
    pub fn to_string(&self, source: &str) -> String {
        match self {
            Value::None => "None".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(Number::Int(v)) => v.to_string(),
            Value::Number(Number::Float(v)) => v.to_string(),
            Value::String(s) => s.clone(),
            Value::Template(chunks, symbols) => {
                let raw = render_chunks_to_string(chunks, symbols, source).unwrap_or_else(|e| format!("<eval error: {}>", e));
                dedent(&raw)
            }
            Value::Path(chunks, symbols) => {
                let raw = render_chunks_to_string(chunks, symbols, source).unwrap_or_else(|e| format!("<eval error: {}>", e));
                dedent(&raw)
            }
            Value::FileTemplate { path: _p, template: t } => {
                let raw = render_chunks_to_string(&t.0, &t.1, source).unwrap_or_else(|e| format!("<eval error: {}>", e));
                dedent(&raw)
            }
            Value::List(items) => {
                let inner: Vec<String> = items.iter().map(|v| v.to_string(source)).collect();
                format!("[{}]", inner.join(", "))
            }
            Value::Function { .. } => "<function>".to_string(),
            Value::Builtin(name, _collected) => format!("<builtin:{}>", name),
        }
    }
}

pub fn render_chunks_to_string(
    chunks: &Vec<Chunk>,
    symbols: &HashMap<String, Value>,
    source: &str,
) -> Result<String, EvalError> {
    let mut out = String::new();
    for c in chunks.iter() {
        match c {
            Chunk::String(s) => out.push_str(s),
            Chunk::Expr(e) => {
                let tokens = tokenize(e.to_string())?;
                let ast = parse(tokens);
                let mut env = symbols.clone();
                let v = eval(ast.program, &mut env, source)?;
                match v {
                    Value::List(ref items) => {
                        let items_str: Vec<String> = items.iter().map(|it| it.to_string(source)).collect();
                        let indent = out.rsplit('\n').next().unwrap_or("");
                        let indent_prefix: String = indent.chars().take_while(|c| *c == ' ' || *c == '\t').collect();

                        let mut first_item = true;
                        for item_s in items_str.iter() {
                            let lines: Vec<&str> = item_s.lines().collect();
                            if !first_item {
                                out.push('\n');
                                out.push_str(&indent_prefix);
                            }
                            if !lines.is_empty() {
                                out.push_str(lines[0]);
                                for ln in &lines[1..] {
                                    out.push('\n');
                                    out.push_str(&indent_prefix);
                                    out.push_str(ln);
                                }
                            }
                            first_item = false;
                        }
                    }
                    _ => out.push_str(&v.to_string(source)),
                }
            }
        }
    }
    Ok(out)
}

pub fn dedent(s: &str) -> String {
    let mut lines: Vec<&str> = s.lines().collect();

    while !lines.is_empty() && lines.first().unwrap().trim().is_empty() {
        lines.remove(0);
    }
    while !lines.is_empty() && lines.last().unwrap().trim().is_empty() {
        lines.pop();
    }

    let min_indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().take_while(|c| *c == ' ' || *c == '\t').count())
        .min()
        .unwrap_or(0);

    let out_lines: Vec<String> = lines
        .into_iter()
        .map(|l| {
            if l.len() >= min_indent {
                l.chars().skip(min_indent).collect()
            } else {
                "".to_string()
            }
        })
        .collect();

    out_lines.join("\n")
}

pub fn find_line_for_symbol(sym: &str, source: &str) -> usize {
    if let Some(idx) = source.find(sym) {
        return source[..idx].chars().filter(|c| *c == '\n').count() + 1;
    }
    0
}

pub fn eval(expr: Expr, symbols: &mut HashMap<String, Value>, source: &str) -> Result<Value, EvalError> {
    match expr {
        Expr::Number(value) => Ok(Value::Number(value)),
        Expr::String(value) => Ok(Value::String(value)),
        Expr::Binary { lhs, op, rhs } => {
            let l_eval = eval(*lhs.clone(), symbols, source)?;
            let r_eval = eval(*rhs.clone(), symbols, source)?;

            match op {
                Token::Add => {
                    match (l_eval.clone(), r_eval.clone()) {
                        (Value::Number(ln), Value::Number(rn)) => Ok(Value::Number(ln.add(rn))),
                        (Value::String(ls), Value::String(rs)) => {
                            let mut out = ls.clone();
                            out.push_str(&rs);
                            Ok(Value::String(out))
                        }
                        (Value::List(mut la), Value::List(lb)) => {
                            la.extend(lb.into_iter());
                            Ok(Value::List(la))
                        }
                        (a, b) => Err(EvalError::type_mismatch("number/string/list", format!("{}, {}", a.to_string(source), b.to_string(source)), find_line_for_symbol("", source))),
                    }
                }
                Token::Mul | Token::Div | Token::Sub => {
                    let lnumber = match l_eval {
                        Value::Number(n) => n,
                        other => return Err(EvalError::type_mismatch("number", other.to_string(source), find_line_for_symbol("", source))),
                    };

                    let rnumber = match r_eval {
                        Value::Number(n) => n,
                        other => return Err(EvalError::type_mismatch("number", other.to_string(source), find_line_for_symbol("", source))),
                    };

                    let res = match op {
                        Token::Mul => Value::Number(lnumber.mul(rnumber)),
                        Token::Div => Value::Number(lnumber.div(rnumber)),
                        Token::Sub => Value::Number(lnumber.sub(rnumber)),
                        _ => unreachable!(),
                    };
                    Ok(res)
                }
                Token::DoubleEqual | Token::NotEqual | Token::Greater | Token::Less | Token::GreaterEqual | Token::LessEqual => {
                    let eq = match (&l_eval, &r_eval) {
                        (Value::Number(ln), Value::Number(rn)) => {
                            let lval = match ln { Number::Int(i) => *i as f64, Number::Float(f) => *f };
                            let rval = match rn { Number::Int(i) => *i as f64, Number::Float(f) => *f };
                            match op {
                                Token::DoubleEqual => lval == rval,
                                Token::NotEqual => lval != rval,
                                Token::Greater => lval > rval,
                                Token::Less => lval < rval,
                                Token::GreaterEqual => lval >= rval,
                                Token::LessEqual => lval <= rval,
                                _ => false,
                            }
                        }
                        (Value::String(ls), Value::String(rs)) => {
                            match op {
                                Token::DoubleEqual => ls == rs,
                                Token::NotEqual => ls != rs,
                                Token::Greater => ls > rs,
                                Token::Less => ls < rs,
                                Token::GreaterEqual => ls >= rs,
                                Token::LessEqual => ls <= rs,
                                _ => false,
                            }
                        }
                        (Value::Bool(lb), Value::Bool(rb)) => {
                            match op {
                                Token::DoubleEqual => lb == rb,
                                Token::NotEqual => lb != rb,
                                _ => return Err(EvalError::new("invalid comparison for bool", None, None, 0)),
                            }
                        }
                        (a, b) => {
                            let sa = a.to_string(source);
                            let sb = b.to_string(source);
                            match op {
                                Token::DoubleEqual => sa == sb,
                                Token::NotEqual => sa != sb,
                                Token::Greater => sa > sb,
                                Token::Less => sa < sb,
                                Token::GreaterEqual => sa >= sb,
                                Token::LessEqual => sa <= sb,
                                _ => false,
                            }
                        }
                    };
                    Ok(Value::Bool(eq))
                }
                _ => return Err(EvalError::new("Not a valid operation", None, None, 0)),
            }
        }
        Expr::Ident(ident) => {
            if let Some(value) = symbols.get(&ident) {
                Ok(value.clone())
            } else {
                Err(EvalError::unknown_symbol(ident.clone(), find_line_for_symbol(&ident, source)))
            }
        }
        Expr::Let { ident, value, expr } => {
            let evalue = eval(*value, symbols, source)?;
            symbols.insert(ident, evalue.clone());
            eval(*expr, symbols, source)
        }
        Expr::Function { ident, default, expr } => {
            let default_val = if let Some(def_expr_box) = default {
                Some(Box::new(eval(*def_expr_box, symbols, source)?))
            } else {
                None
            };
            Ok(Value::Function { ident, default: default_val, expr, env: symbols.clone() })
        }
        Expr::Application { lhs, rhs } => {
            let lhs_eval = eval(*lhs, symbols, source)?;
            let arg_val = eval(*rhs, symbols, source)?;
            match lhs_eval {
                Value::Function { ident, expr, env, .. } => {
                    let mut new_env = env.clone();
                    new_env.insert(ident, arg_val);
                    eval(*expr, &mut new_env, source)
                }
                builtin @ Value::Builtin(_, _) => apply_function(&builtin, arg_val, source),
                other => Err(EvalError::new(format!("Tried to call a Non-Function {:?}", other), None, None, 0)),
            }
        }
        Expr::None => Ok(Value::None),
        Expr::Template(chunks) => Ok(Value::Template(chunks, symbols.clone())),
        Expr::Builtin(function, args) => match function.as_str() {
            "concat" => {
                let arg1 = symbols
                    .get(&args[0])
                    .cloned()
                    .ok_or_else(|| EvalError::unknown_symbol(args[0].clone(), find_line_for_symbol(&args[0], source)))?;

                let arg2 = symbols
                    .get(&args[1])
                    .cloned()
                    .ok_or_else(|| EvalError::unknown_symbol(args[1].clone(), find_line_for_symbol(&args[1], source)))?;

                match (arg1, arg2) {
                    (Value::String(mut lhs), Value::String(rhs)) => {
                        lhs.push_str(rhs.as_str());
                        Ok(Value::String(lhs))
                    }
                    (a, b) => Err(EvalError::type_mismatch(
                        "string",
                        format!("{}, {}", a.to_string(source), b.to_string(source)),
                        find_line_for_symbol("", source),
                    )),
                }
            }
            _ => Err(EvalError::new("unimplemented builtin", None, None, 0)),
        },
        Expr::Bool(value) => Ok(Value::Bool(value)),
        Expr::If { cond, t, f } => {
            let cond_eval = eval(*cond, symbols, source)?;
            if let Value::Bool(cond_value) = cond_eval {
                if cond_value {
                    eval(*t, symbols, source)
                } else {
                    eval(*f, symbols, source)
                }
            } else {
                Err(EvalError::type_mismatch(
                    "bool",
                    cond_eval.to_string(source),
                    find_line_for_symbol("", source),
                ))
            }
        }
        Expr::Path(chunks) => Ok(Value::Path(chunks, symbols.clone())),
        Expr::List(items) => {
            let mut evaluated = Vec::new();
            for item in items {
                evaluated.push(eval(item, symbols, source)?);
            }
            Ok(Value::List(evaluated))
        }
        Expr::FileTemplate { path, template } => Ok(Value::FileTemplate {
            path: (path, symbols.clone()),
            template: (template, symbols.clone()),
        }),
    }
}

pub fn apply_function(func: &Value, arg: Value, source: &str) -> Result<Value, EvalError> {
    match func {
        Value::Function { ident, expr, env, .. } => {
            let mut new_env = env.clone();
            new_env.insert(ident.clone(), arg);
            eval(*expr.clone(), &mut new_env, source)
        }
        Value::Builtin(name, collected) => {
            let mut new_collected = collected.clone();
            new_collected.push(arg);

            let arity = match name.as_str() {
                "concat" => 2,
                "map" => 2,
                "filter" => 2,
                "fold" => 3,
                "import" => 1,
                "readfile" => 1,
                "readlines" => 1,
                "walkdir" => 1,
                "json_parse" => 1,
                "exists" => 1,
                "basename" => 1,
                "dirname" => 1,
                "upper" => 1,
                "lower" => 1,
                "trim" => 1,
                "split" => 2,
                "join" => 2,
                "replace" => 3,
                "contains" => 2,
                "starts_with" => 2,
                "ends_with" => 2,
                "length" => 1,
                "repeat" => 2,
                "pad_left" => 3,
                "pad_right" => 3,
                "indent" => 2,
                "is_digit" => 1,
                "is_alpha" => 1,
                "is_alphanumeric" => 1,
                "is_whitespace" => 1,
                "is_uppercase" => 1,
                "is_lowercase" => 1,
                "is_empty" => 1,
                "html_escape" => 1,
                "html_tag" => 2,
                "html_attr" => 2,
                "md_heading" => 2,
                "md_link" => 2,
                "md_code" => 1,
                "md_list" => 1,
                "to_string" => 1,
                "to_int" => 1,
                "to_float" => 1,
                "to_bool" => 1,
                "format_int" => 2,
                "format_float" => 2,
                "flatmap" => 2,
                "flatten" => 1,
                _ => 1,
            };

            if new_collected.len() < arity {
                return Ok(Value::Builtin(name.clone(), new_collected));
            }

            // execute builtin (implementation continues in cli module or separate file)
            execute_builtin(name, &new_collected, source)
        }
        other => Err(EvalError::new(format!("Tried to call a Non-Function {:?}", other), None, None, 0)),
    }
}

pub fn execute_builtin(name: &str, args: &[Value], source: &str) -> Result<Value, EvalError> {
    match name {
        "concat" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                let mut out = sa.clone();
                out.push_str(sb);
                Ok(Value::String(out))
            } else {
                Err(EvalError::type_mismatch("string", format!("{}, {}", a.to_string(source), b.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "map" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    let res = apply_function(func, item.clone(), source)?;
                    out.push(res);
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "filter" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    let res = apply_function(func, item.clone(), source)?;
                    match res {
                        Value::Bool(true) => out.push(item.clone()),
                        Value::Bool(false) => {}
                        other => return Err(EvalError::type_mismatch("bool", other.to_string(source), find_line_for_symbol("", source))),
                    }
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "fold" => {
            let func = &args[0];
            let mut acc = args[1].clone();
            let list = &args[2];
            if let Value::List(items) = list {
                for item in items {
                    let step = apply_function(func, acc, source)?;
                    acc = apply_function(&step, item.clone(), source)?;
                }
                Ok(acc)
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "import" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let data = std::fs::read_to_string(p).map_err(|e| EvalError::new(format!("failed to import {}: {}", p, e), None, None, 0))?;
                let tokens = tokenize(data.clone())?;
                let ast = parse(tokens);
                let mut env = initial_builtins();
                let val = eval(ast.program, &mut env, &data)?;
                Ok(val)
            } else {
                Err(EvalError::type_mismatch("string", pathv.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "readfile" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let data = std::fs::read_to_string(p).map_err(|e| EvalError::new(format!("failed to read {}: {}", p, e), None, None, 0))?;
                Ok(Value::String(data))
            } else {
                Err(EvalError::type_mismatch("string", pathv.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "upper" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::String(st.to_uppercase()))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "lower" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::String(st.to_lowercase()))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "trim" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::String(st.trim().to_string()))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "contains" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                Ok(Value::Bool(sa.contains(sb)))
            } else {
                Err(EvalError::type_mismatch("string", format!("{}, {}", a.to_string(source), b.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "starts_with" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                Ok(Value::Bool(sa.starts_with(sb)))
            } else {
                Err(EvalError::type_mismatch("string", format!("{}, {}", a.to_string(source), b.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "ends_with" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                Ok(Value::Bool(sa.ends_with(sb)))
            } else {
                Err(EvalError::type_mismatch("string", format!("{}, {}", a.to_string(source), b.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "split" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::String(sa), Value::String(sb)) = (a, b) {
                let parts: Vec<Value> = sa.split(sb).map(|s| Value::String(s.to_string())).collect();
                Ok(Value::List(parts))
            } else {
                Err(EvalError::type_mismatch("string", format!("{}, {}", a.to_string(source), b.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "join" => {
            let a = &args[0];
            let b = &args[1];
            if let (Value::List(list), Value::String(sep)) = (a, b) {
                let parts: Vec<String> = list.iter().map(|it| it.to_string(source)).collect();
                Ok(Value::String(parts.join(sep)))
            } else {
                Err(EvalError::type_mismatch("list/string", format!("{}, {}", a.to_string(source), b.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "replace" => {
            let a = &args[0];
            let b = &args[1];
            let c = &args[2];
            if let (Value::String(sa), Value::String(sb), Value::String(sc)) = (a, b, c) {
                Ok(Value::String(sa.replace(sb, sc)))
            } else {
                Err(EvalError::type_mismatch("string", format!("{}, {}, {}", a.to_string(source), b.to_string(source), c.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "readlines" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let data = std::fs::read_to_string(p).map_err(|e| EvalError::new(format!("failed to read {}: {}", p, e), None, None, 0))?;
                let lines: Vec<Value> = data.lines().map(|s| Value::String(s.to_string())).collect();
                Ok(Value::List(lines))
            } else {
                Err(EvalError::type_mismatch("string", pathv.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "walkdir" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let mut out = Vec::new();
                let base = std::path::Path::new(p);
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
            } else {
                Err(EvalError::type_mismatch("string", pathv.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "json_parse" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let data = std::fs::read_to_string(p).map_err(|e| EvalError::new(format!("failed to read {}: {}", p, e), None, None, 0))?;
                let jr: serde_json::Value = serde_json::from_str(&data).map_err(|e| EvalError::new(format!("json parse error: {}", e), None, None, 0))?;
                fn conv(j: &serde_json::Value) -> Value {
                    match j {
                        serde_json::Value::Null => Value::None,
                        serde_json::Value::Bool(b) => Value::Bool(*b),
                        serde_json::Value::Number(n) => {
                            if let Some(i) = n.as_i64() { Value::Number(Number::Int(i)) } else if let Some(f) = n.as_f64() { Value::Number(Number::Float(f)) } else { Value::None }
                        }
                        serde_json::Value::String(s) => Value::String(s.clone()),
                        serde_json::Value::Array(a) => Value::List(a.iter().map(|it| conv(it)).collect()),
                        serde_json::Value::Object(o) => {
                            let mut m = std::collections::HashMap::new();
                            for (k,v) in o.iter() { m.insert(k.clone(), conv(v)); }
                            Value::String(serde_json::to_string(o).unwrap_or_default())
                        }
                    }
                }
                Ok(conv(&jr))
            } else {
                Err(EvalError::type_mismatch("string", pathv.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "exists" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                Ok(Value::Bool(std::path::Path::new(p).exists()))
            } else {
                Err(EvalError::type_mismatch("string", pathv.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "basename" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let b = std::path::Path::new(p).file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
                Ok(Value::String(b))
            } else {
                Err(EvalError::type_mismatch("string", pathv.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "dirname" => {
            let pathv = &args[0];
            if let Value::String(p) = pathv {
                let d = std::path::Path::new(p).parent().and_then(|s| s.to_str()).unwrap_or("").to_string();
                Ok(Value::String(d))
            } else {
                Err(EvalError::type_mismatch("string", pathv.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "length" => {
            let val = &args[0];
            match val {
                Value::String(s) => Ok(Value::Number(Number::Int(s.len() as i64))),
                Value::List(items) => Ok(Value::Number(Number::Int(items.len() as i64))),
                other => Err(EvalError::type_mismatch("string or list", other.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "repeat" => {
            let s = &args[0];
            let n = &args[1];
            if let (Value::String(st), Value::Number(Number::Int(count))) = (s, n) {
                Ok(Value::String(st.repeat(*count as usize)))
            } else {
                Err(EvalError::type_mismatch("string, number", format!("{}, {}", s.to_string(source), n.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "pad_left" => {
            let s = &args[0];
            let width = &args[1];
            let pad = &args[2];
            if let (Value::String(st), Value::Number(Number::Int(w)), Value::String(pc)) = (s, width, pad) {
                let pad_char = pc.chars().next().unwrap_or(' ');
                let result = format!("{:>width$}", st, width = *w as usize).replace(' ', &pad_char.to_string());
                Ok(Value::String(result))
            } else {
                Err(EvalError::type_mismatch("string, number, string", format!("{}, {}, {}", s.to_string(source), width.to_string(source), pad.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "pad_right" => {
            let s = &args[0];
            let width = &args[1];
            let pad = &args[2];
            if let (Value::String(st), Value::Number(Number::Int(w)), Value::String(pc)) = (s, width, pad) {
                let pad_char = pc.chars().next().unwrap_or(' ');
                let result = format!("{:<width$}", st, width = *w as usize).replace(' ', &pad_char.to_string());
                Ok(Value::String(result))
            } else {
                Err(EvalError::type_mismatch("string, number, string", format!("{}, {}, {}", s.to_string(source), width.to_string(source), pad.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "indent" => {
            let s = &args[0];
            let spaces = &args[1];
            if let (Value::String(st), Value::Number(Number::Int(n))) = (s, spaces) {
                let indent_str = " ".repeat(*n as usize);
                let lines: Vec<String> = st.lines().map(|line| format!("{}{}", indent_str, line)).collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch("string, number", format!("{}, {}", s.to_string(source), spaces.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "is_digit" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::Bool(!st.is_empty() && st.chars().all(|c| c.is_ascii_digit())))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "is_alpha" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::Bool(!st.is_empty() && st.chars().all(|c| c.is_alphabetic())))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "is_alphanumeric" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::Bool(!st.is_empty() && st.chars().all(|c| c.is_alphanumeric())))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "is_whitespace" => {
            let s = &args[0];
            if let Value::String(st) = s {
                Ok(Value::Bool(!st.is_empty() && st.chars().all(|c| c.is_whitespace())))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "is_uppercase" => {
            let s = &args[0];
            if let Value::String(st) = s {
                let letters: Vec<char> = st.chars().filter(|c| c.is_alphabetic()).collect();
                Ok(Value::Bool(!letters.is_empty() && letters.iter().all(|c| c.is_uppercase())))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "is_lowercase" => {
            let s = &args[0];
            if let Value::String(st) = s {
                let letters: Vec<char> = st.chars().filter(|c| c.is_alphabetic()).collect();
                Ok(Value::Bool(!letters.is_empty() && letters.iter().all(|c| c.is_lowercase())))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "is_empty" => {
            let s = &args[0];
            match s {
                Value::String(st) => Ok(Value::Bool(st.is_empty())),
                Value::List(items) => Ok(Value::Bool(items.is_empty())),
                other => Err(EvalError::type_mismatch("string or list", other.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "html_escape" => {
            let s = &args[0];
            if let Value::String(st) = s {
                let escaped = st
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;")
                    .replace('\'', "&#x27;");
                Ok(Value::String(escaped))
            } else {
                Err(EvalError::type_mismatch("string", s.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "html_tag" => {
            let tag = &args[0];
            let content = &args[1];
            if let (Value::String(t), Value::String(c)) = (tag, content) {
                Ok(Value::String(format!("<{}>{}</{}>", t, c, t)))
            } else {
                Err(EvalError::type_mismatch("string", format!("{}, {}", tag.to_string(source), content.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "html_attr" => {
            let name = &args[0];
            let value = &args[1];
            if let (Value::String(n), Value::String(v)) = (name, value) {
                let escaped = v
                    .replace('&', "&amp;")
                    .replace('"', "&quot;")
                    .replace('\'', "&#x27;");
                Ok(Value::String(format!("{}=\"{}\"", n, escaped)))
            } else {
                Err(EvalError::type_mismatch("string", format!("{}, {}", name.to_string(source), value.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "md_heading" => {
            let level = &args[0];
            let text = &args[1];
            if let (Value::Number(Number::Int(lvl)), Value::String(txt)) = (level, text) {
                let hashes = "#".repeat((*lvl).max(1).min(6) as usize);
                Ok(Value::String(format!("{} {}", hashes, txt)))
            } else {
                Err(EvalError::type_mismatch("number, string", format!("{}, {}", level.to_string(source), text.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "md_link" => {
            let text = &args[0];
            let url = &args[1];
            if let (Value::String(txt), Value::String(u)) = (text, url) {
                Ok(Value::String(format!("[{}]({})", txt, u)))
            } else {
                Err(EvalError::type_mismatch("string", format!("{}, {}", text.to_string(source), url.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "md_code" => {
            let code = &args[0];
            if let Value::String(c) = code {
                Ok(Value::String(format!("`{}`", c)))
            } else {
                Err(EvalError::type_mismatch("string", code.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "md_list" => {
            let items = &args[0];
            if let Value::List(list) = items {
                let lines: Vec<String> = list.iter().map(|item| format!("- {}", item.to_string(source))).collect();
                Ok(Value::String(lines.join("\n")))
            } else {
                Err(EvalError::type_mismatch("list", items.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "to_string" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::String(i.to_string())),
                Value::Number(Number::Float(f)) => {
                    // Format float nicely - remove unnecessary trailing zeros
                    let s = format!("{}", f);
                    Ok(Value::String(s))
                }
                Value::String(s) => Ok(Value::String(s.clone())),
                Value::Bool(b) => Ok(Value::String(b.to_string())),
                other => Ok(Value::String(other.to_string(source)))
            }
        }
        "to_int" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Int(*i))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Int(*f as i64))),
                Value::String(s) => {
                    s.trim().parse::<i64>()
                        .map(|i| Value::Number(Number::Int(i)))
                        .map_err(|_| EvalError::new(format!("cannot convert '{}' to int", s), None, None, 0))
                }
                Value::Bool(b) => Ok(Value::Number(Number::Int(if *b { 1 } else { 0 }))),
                other => Err(EvalError::new(format!("cannot convert {} to int", other.to_string(source)), None, None, 0))
            }
        }
        "to_float" => {
            let val = &args[0];
            match val {
                Value::Number(Number::Int(i)) => Ok(Value::Number(Number::Float(*i as f64))),
                Value::Number(Number::Float(f)) => Ok(Value::Number(Number::Float(*f))),
                Value::String(s) => {
                    s.trim().parse::<f64>()
                        .map(|f| Value::Number(Number::Float(f)))
                        .map_err(|_| EvalError::new(format!("cannot convert '{}' to float", s), None, None, 0))
                }
                other => Err(EvalError::new(format!("cannot convert {} to float", other.to_string(source)), None, None, 0))
            }
        }
        "to_bool" => {
            let val = &args[0];
            match val {
                Value::Bool(b) => Ok(Value::Bool(*b)),
                Value::Number(Number::Int(i)) => Ok(Value::Bool(*i != 0)),
                Value::Number(Number::Float(f)) => Ok(Value::Bool(*f != 0.0)),
                Value::String(s) => {
                    let lower = s.to_lowercase();
                    match lower.as_str() {
                        "true" | "yes" | "1" | "on" => Ok(Value::Bool(true)),
                        "false" | "no" | "0" | "off" | "" => Ok(Value::Bool(false)),
                        _ => Err(EvalError::new(format!("cannot convert '{}' to bool", s), None, None, 0))
                    }
                }
                Value::List(items) => Ok(Value::Bool(!items.is_empty())),
                _ => Ok(Value::Bool(true)) // Other values are truthy
            }
        }
        "format_int" => {
            let val = &args[0];
            let width = &args[1];
            if let (Value::Number(num), Value::Number(Number::Int(w))) = (val, width) {
                let int_val = match num {
                    Number::Int(i) => *i,
                    Number::Float(f) => *f as i64,
                };
                let formatted = if *w > 0 {
                    format!("{:0width$}", int_val, width = *w as usize)
                } else {
                    format!("{}", int_val)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch("number, number", format!("{}, {}", val.to_string(source), width.to_string(source)), find_line_for_symbol("", source)))
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
                    format!("{:.prec$}", float_val, prec = *p as usize)
                } else {
                    format!("{}", float_val)
                };
                Ok(Value::String(formatted))
            } else {
                Err(EvalError::type_mismatch("number, number", format!("{}, {}", val.to_string(source), precision.to_string(source)), find_line_for_symbol("", source)))
            }
        }
        "flatmap" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    let res = apply_function(func, item.clone(), source)?;
                    match res {
                        Value::List(sub_items) => out.extend(sub_items),
                        single => out.push(single),
                    }
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), find_line_for_symbol("", source)))
            }
        }
        "flatten" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    match item {
                        Value::List(sub_items) => out.extend(sub_items.clone()),
                        single => out.push(single.clone()),
                    }
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch("list", list.to_string(source), find_line_for_symbol("", source)))
            }
        }
        other => Err(EvalError::new(format!("unimplemented builtin {}", other), None, None, 0)),
    }
}

pub fn collect_file_templates(v: &Value, source: &str) -> Result<Vec<(String, String)>, EvalError> {
    match v {
        Value::FileTemplate { path: (pchunks, penv), template: (tchunks, tenv) } => {
            let path = render_chunks_to_string(pchunks, penv, source)?;
            let raw = render_chunks_to_string(tchunks, tenv, source)?;
            let content = dedent(&raw);
            Ok(vec![(path, content)])
        }
        Value::List(items) => {
            let mut out = Vec::new();
            for item in items {
                let mut res = collect_file_templates(item, source)?;
                out.append(&mut res);
            }
            Ok(out)
        }
        _ => Err(EvalError::new("expected filetemplate or list of filetemplates", None, None, 0)),
    }
}

pub fn fetch_git_raw(spec: &str) -> Result<String, EvalError> {
    let parts: Vec<&str> = spec.split('/').collect();
    if parts.len() < 3 {
        return Err(EvalError::new("invalid git spec (expected owner/repo/path)", None, None, 0));
    }
    let owner = parts[0];
    let repo = parts[1];
    let path = parts[2..].join("/");
    let url = format!("https://raw.githubusercontent.com/{}/{}/main/{}", owner, repo, path);
    let resp = ureq::get(&url).call().map_err(|e| EvalError::new(format!("failed to fetch {}: {}", url, e), None, None, 0))?;
    let status = resp.status();
    if status >= 400 {
        return Err(EvalError::new(format!("failed to fetch {}: status {}", url, status), None, None, 0));
    }
    let text = resp.into_string().map_err(|e| EvalError::new(format!("failed to read response: {}", e), None, None, 0))?;
    Ok(text)
}
