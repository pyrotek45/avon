//! List operations: choice, chunks, combinations, drop, enumerate, filter, find, find_index, flatmap, flatten, fold, group_by, head, intersperse, last, map, nth, partition, permutations, pfilter, pfold, pmap, range, reverse, sample, shuffle, sort, sort_by, split_at, tail, take, transpose, unique, unzip, windows, zip, zip_with

use crate::common::{EvalError, Number, Value};
use crate::eval::apply_function;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

/// Names of list builtins
pub const NAMES: &[&str] = &[
    "choice",
    "chunks",
    "combinations",
    "drop",
    "enumerate",
    "filter",
    "find",
    "find_index",
    "flatmap",
    "flatten",
    "fold",
    "group_by",
    "head",
    "intersperse",
    "last",
    "map",
    "nth",
    "partition",
    "permutations",
    "pfilter",
    "pfold",
    "pmap",
    "range",
    "reverse",
    "sample",
    "shuffle",
    "sort",
    "sort_by",
    "split_at",
    "tail",
    "take",
    "transpose",
    "unique",
    "unzip",
    "windows",
    "zip",
    "zip_with",
];

/// Get arity for list functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "choice" | "enumerate" | "flatten" | "head" | "last" | "reverse" | "shuffle" | "sort"
        | "tail" | "transpose" | "unique" | "unzip" => Some(1),
        "chunks" | "combinations" | "drop" | "filter" | "find" | "find_index" | "flatmap"
        | "group_by" | "intersperse" | "map" | "nth" | "partition" | "permutations" | "pfilter"
        | "pmap" | "range" | "sample" | "sort_by" | "split_at" | "take" | "windows" | "zip" => {
            Some(2)
        }
        "fold" | "pfold" | "zip_with" => Some(3),
        _ => None,
    }
}

/// Check if name is a list builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Execute a list builtin function
pub fn execute(name: &str, args: &[Value], source: &str, line: usize) -> Result<Value, EvalError> {
    match name {
        "map" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("map:") {
                                err.message = format!("map: {}", err.message);
                            }
                            err
                        })?;
                    out.push(res);
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "filter" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("filter:") {
                                err.message = format!("filter: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::Bool(true) => out.push(item.clone()),
                        Value::Bool(false) => {}
                        other => {
                            return Err(EvalError::type_mismatch(
                                "bool",
                                other.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "fold" => {
            let func = &args[0];
            let mut acc = args[1].clone();
            let list = &args[2];
            if let Value::List(items) = list {
                for item in items {
                    let step = apply_function(func, acc, source, line).map_err(|mut err| {
                        if !err.message.starts_with("fold:") {
                            err.message = format!("fold: {}", err.message);
                        }
                        err
                    })?;
                    acc =
                        apply_function(&step, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("fold:") {
                                err.message = format!("fold: {}", err.message);
                            }
                            err
                        })?;
                }
                Ok(acc)
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "flatmap" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut out = Vec::new();
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("flatmap:") {
                                err.message = format!("flatmap: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::List(sub_items) => out.extend(sub_items),
                        single => out.push(single),
                    }
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
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
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "head" => {
            let list = &args[0];
            if let Value::List(items) = list {
                Ok(items.first().cloned().unwrap_or(Value::None))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "last" => {
            // last :: [a] -> a | None
            // Get the last element of a list, or None if empty
            let list = &args[0];
            if let Value::List(items) = list {
                Ok(items.last().cloned().unwrap_or(Value::None))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "nth" => {
            let n_val = &args[0];
            let list = &args[1];
            let n = match n_val {
                Value::Number(Number::Int(i)) => *i,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "integer",
                        n_val.to_string(source),
                        line,
                    ))
                }
            };
            if let Value::List(items) = list {
                if n < 0 {
                    Ok(Value::None)
                } else if (n as usize) < items.len() {
                    Ok(items[n as usize].clone())
                } else {
                    Ok(Value::None)
                }
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "tail" => {
            let list = &args[0];
            if let Value::List(items) = list {
                if items.is_empty() {
                    Ok(Value::List(Vec::new()))
                } else {
                    Ok(Value::List(items[1..].to_vec()))
                }
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "take" => {
            let n_val = &args[0];
            let list = &args[1];
            let n = match n_val {
                Value::Number(Number::Int(i)) => *i as usize,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        n_val.to_string(source),
                        line,
                    ))
                }
            };
            if let Value::List(items) = list {
                Ok(Value::List(items.iter().take(n).cloned().collect()))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "drop" => {
            let n_val = &args[0];
            let list = &args[1];
            let n = match n_val {
                Value::Number(Number::Int(i)) => *i as usize,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        n_val.to_string(source),
                        line,
                    ))
                }
            };
            if let Value::List(items) = list {
                Ok(Value::List(items.iter().skip(n).cloned().collect()))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "zip" => {
            let list1 = &args[0];
            let list2 = &args[1];
            if let (Value::List(items1), Value::List(items2)) = (list1, list2) {
                let mut out = Vec::new();
                let min_len = items1.len().min(items2.len());
                for i in 0..min_len {
                    out.push(Value::List(vec![items1[i].clone(), items2[i].clone()]));
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    format!("{}, {}", list1.to_string(source), list2.to_string(source)),
                    line,
                ))
            }
        }
        "unzip" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut list1 = Vec::new();
                let mut list2 = Vec::new();
                for item in items {
                    if let Value::List(pair) = item {
                        if pair.len() >= 2 {
                            list1.push(pair[0].clone());
                            list2.push(pair[1].clone());
                        }
                    }
                }
                Ok(Value::List(vec![Value::List(list1), Value::List(list2)]))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "split_at" => {
            let n_val = &args[0];
            let list = &args[1];
            let n = match n_val {
                Value::Number(Number::Int(i)) => *i as usize,
                _ => {
                    return Err(EvalError::type_mismatch(
                        "number",
                        n_val.to_string(source),
                        line,
                    ))
                }
            };
            if let Value::List(items) = list {
                let first = Value::List(items.iter().take(n).cloned().collect());
                let second = Value::List(items.iter().skip(n).cloned().collect());
                Ok(Value::List(vec![first, second]))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "partition" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut true_list = Vec::new();
                let mut false_list = Vec::new();
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("partition:") {
                                err.message = format!("partition: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::Bool(true) => true_list.push(item.clone()),
                        Value::Bool(false) => false_list.push(item.clone()),
                        other => {
                            return Err(EvalError::type_mismatch(
                                "bool",
                                other.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                Ok(Value::List(vec![
                    Value::List(true_list),
                    Value::List(false_list),
                ]))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "reverse" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut reversed = items.clone();
                reversed.reverse();
                Ok(Value::List(reversed))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "sort" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut sorted = items.clone();
                sorted.sort_by(|a, b| {
                    let a_str = a.to_string(source);
                    let b_str = b.to_string(source);
                    match (a, b) {
                        (Value::Number(Number::Int(a_int)), Value::Number(Number::Int(b_int))) => {
                            a_int.cmp(b_int)
                        }
                        (
                            Value::Number(Number::Float(a_float)),
                            Value::Number(Number::Float(b_float)),
                        ) => a_float
                            .partial_cmp(b_float)
                            .unwrap_or(std::cmp::Ordering::Equal),
                        (
                            Value::Number(Number::Int(a_int)),
                            Value::Number(Number::Float(b_float)),
                        ) => (*a_int as f64)
                            .partial_cmp(b_float)
                            .unwrap_or(std::cmp::Ordering::Equal),
                        (
                            Value::Number(Number::Float(a_float)),
                            Value::Number(Number::Int(b_int)),
                        ) => a_float
                            .partial_cmp(&(*b_int as f64))
                            .unwrap_or(std::cmp::Ordering::Equal),
                        _ => a_str.cmp(&b_str),
                    }
                });
                Ok(Value::List(sorted))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "sort_by" => {
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut pairs: Vec<(Value, Value)> = Vec::new();
                for item in items {
                    let key =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("sort_by:") {
                                err.message = format!("sort_by: {}", err.message);
                            }
                            err
                        })?;
                    pairs.push((item.clone(), key));
                }

                pairs.sort_by(|(_, a_key), (_, b_key)| {
                    let a_str = a_key.to_string(source);
                    let b_str = b_key.to_string(source);
                    match (a_key, b_key) {
                        (Value::Number(Number::Int(a_int)), Value::Number(Number::Int(b_int))) => {
                            a_int.cmp(b_int)
                        }
                        (
                            Value::Number(Number::Float(a_float)),
                            Value::Number(Number::Float(b_float)),
                        ) => a_float
                            .partial_cmp(b_float)
                            .unwrap_or(std::cmp::Ordering::Equal),
                        (
                            Value::Number(Number::Int(a_int)),
                            Value::Number(Number::Float(b_float)),
                        ) => (*a_int as f64)
                            .partial_cmp(b_float)
                            .unwrap_or(std::cmp::Ordering::Equal),
                        (
                            Value::Number(Number::Float(a_float)),
                            Value::Number(Number::Int(b_int)),
                        ) => a_float
                            .partial_cmp(&(*b_int as f64))
                            .unwrap_or(std::cmp::Ordering::Equal),
                        _ => a_str.cmp(&b_str),
                    }
                });

                let sorted: Vec<Value> = pairs.into_iter().map(|(item, _)| item).collect();
                Ok(Value::List(sorted))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "unique" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let mut seen = HashSet::new();
                let mut result = Vec::new();
                for item in items {
                    let key = item.to_string(source);
                    if seen.insert(key) {
                        result.push(item.clone());
                    }
                }
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "range" => {
            let start = &args[0];
            let end = &args[1];

            match (start, end) {
                (Value::Number(Number::Int(s)), Value::Number(Number::Int(e))) => {
                    if s <= e {
                        // Check for excessively large ranges to prevent OOM
                        let range_size = e.saturating_sub(*s).saturating_add(1);
                        if range_size > 10_000_000 {
                            return Err(EvalError::new(
                                format!(
                                    "range {} to {} is too large ({} items, max 10 million)",
                                    s, e, range_size
                                ),
                                None,
                                None,
                                line,
                            ));
                        }
                        let result: Vec<Value> =
                            (*s..=*e).map(|i| Value::Number(Number::Int(i))).collect();
                        Ok(Value::List(result))
                    } else {
                        Ok(Value::List(Vec::new()))
                    }
                }
                _ => Err(EvalError::type_mismatch(
                    "two integers",
                    format!("{}, {}", start.to_string(source), end.to_string(source)),
                    line,
                )),
            }
        }
        "enumerate" => {
            let list = &args[0];
            if let Value::List(items) = list {
                let result: Vec<Value> = items
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        Value::List(vec![Value::Number(Number::Int(idx as i64)), item.clone()])
                    })
                    .collect();
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "windows" => {
            let size_val = &args[0];
            let list = &args[1];
            let size = match size_val {
                Value::Number(Number::Int(i)) => {
                    if *i <= 0 {
                        return Err(EvalError::new(
                            format!("windows size must be positive, got {}", i),
                            None,
                            None,
                            line,
                        ));
                    }
                    *i as usize
                }
                _ => {
                    return Err(EvalError::type_mismatch(
                        "integer",
                        size_val.to_string(source),
                        line,
                    ))
                }
            };
            if let Value::List(items) = list {
                let result: Vec<Value> = items
                    .windows(size)
                    .map(|w| Value::List(w.to_vec()))
                    .collect();
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "chunks" => {
            let size_val = &args[0];
            let list = &args[1];
            let size = match size_val {
                Value::Number(Number::Int(i)) => {
                    if *i <= 0 {
                        return Err(EvalError::new(
                            format!("chunk size must be positive, got {}", i),
                            None,
                            None,
                            line,
                        ));
                    }
                    *i as usize
                }
                _ => {
                    return Err(EvalError::type_mismatch(
                        "integer",
                        size_val.to_string(source),
                        line,
                    ))
                }
            };
            if let Value::List(items) = list {
                let result: Vec<Value> = items
                    .chunks(size)
                    .map(|c| Value::List(c.to_vec()))
                    .collect();
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "transpose" => {
            let list = &args[0];
            if let Value::List(items) = list {
                if items.is_empty() {
                    return Ok(Value::List(Vec::new()));
                }
                let mut rows = Vec::new();
                for item in items {
                    if let Value::List(row) = item {
                        rows.push(row);
                    } else {
                        return Err(EvalError::type_mismatch(
                            "list of lists",
                            item.to_string(source),
                            line,
                        ));
                    }
                }

                if rows.is_empty() {
                    return Ok(Value::List(Vec::new()));
                }

                let width = rows[0].len();
                for row in &rows {
                    if row.len() != width {
                        return Err(EvalError::new(
                            "transpose requires rectangular matrix".to_string(),
                            None,
                            None,
                            line,
                        ));
                    }
                }

                let mut out = Vec::new();
                for i in 0..width {
                    let mut new_row = Vec::new();
                    for row in &rows {
                        new_row.push(row[i].clone());
                    }
                    out.push(Value::List(new_row));
                }
                Ok(Value::List(out))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "permutations" => {
            let k_val = &args[0];
            let list = &args[1];
            let k = match k_val {
                Value::Number(Number::Int(i)) => {
                    if *i < 0 {
                        return Err(EvalError::new(
                            format!("permutations k must be non-negative, got {}", i),
                            None,
                            None,
                            line,
                        ));
                    }
                    *i as usize
                }
                _ => {
                    return Err(EvalError::type_mismatch(
                        "integer",
                        k_val.to_string(source),
                        line,
                    ))
                }
            };
            if let Value::List(items) = list {
                let result: Vec<Value> = items
                    .iter()
                    .permutations(k)
                    .map(|p| Value::List(p.into_iter().cloned().collect()))
                    .collect();
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "combinations" => {
            let k_val = &args[0];
            let list = &args[1];
            let k = match k_val {
                Value::Number(Number::Int(i)) => {
                    if *i < 0 {
                        return Err(EvalError::new(
                            format!("combinations k must be non-negative, got {}", i),
                            None,
                            None,
                            line,
                        ));
                    }
                    *i as usize
                }
                _ => {
                    return Err(EvalError::type_mismatch(
                        "integer",
                        k_val.to_string(source),
                        line,
                    ))
                }
            };
            if let Value::List(items) = list {
                let result: Vec<Value> = items
                    .iter()
                    .combinations(k)
                    .map(|c| Value::List(c.into_iter().cloned().collect()))
                    .collect();
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "choice" => {
            // choice :: [a] -> a
            // Pick a random element from a list
            let list = &args[0];
            if let Value::List(items) = list {
                if items.is_empty() {
                    return Err(EvalError::new(
                        "choice: cannot choose from empty list".to_string(),
                        None,
                        None,
                        line,
                    ));
                }
                let mut rng = rand::thread_rng();
                let idx = rng.gen_range(0..items.len());
                Ok(items[idx].clone())
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "shuffle" => {
            // shuffle :: [a] -> [a]
            // Return a randomly shuffled copy of the list
            let list = &args[0];
            if let Value::List(items) = list {
                let mut result = items.clone();
                let mut rng = rand::thread_rng();
                result.shuffle(&mut rng);
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "sample" => {
            // sample :: Number -> [a] -> [a]
            // Pick n random elements without replacement
            let n_val = &args[0];
            let list = &args[1];
            let n = match n_val {
                Value::Number(Number::Int(i)) => {
                    if *i < 0 {
                        return Err(EvalError::new(
                            format!("sample: count must be non-negative, got {}", i),
                            None,
                            None,
                            line,
                        ));
                    }
                    *i as usize
                }
                _ => {
                    return Err(EvalError::type_mismatch(
                        "integer",
                        n_val.to_string(source),
                        line,
                    ))
                }
            };
            if let Value::List(items) = list {
                if n > items.len() {
                    return Err(EvalError::new(
                        format!(
                            "sample: cannot sample {} items from list of length {}",
                            n,
                            items.len()
                        ),
                        None,
                        None,
                        line,
                    ));
                }
                let mut rng = rand::thread_rng();
                let result: Vec<Value> = items.choose_multiple(&mut rng, n).cloned().collect();
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "find" => {
            // find :: (a -> Bool) -> [a] -> a | None
            // Find first element matching predicate
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                for item in items {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("find:") {
                                err.message = format!("find: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::Bool(true) => return Ok(item.clone()),
                        Value::Bool(false) => {}
                        other => {
                            return Err(EvalError::type_mismatch(
                                "bool",
                                other.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                Ok(Value::None)
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "find_index" => {
            // find_index :: (a -> Bool) -> [a] -> Number | None
            // Find index of first element matching predicate
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                for (idx, item) in items.iter().enumerate() {
                    let res =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("find_index:") {
                                err.message = format!("find_index: {}", err.message);
                            }
                            err
                        })?;
                    match res {
                        Value::Bool(true) => return Ok(Value::Number(Number::Int(idx as i64))),
                        Value::Bool(false) => {}
                        other => {
                            return Err(EvalError::type_mismatch(
                                "bool",
                                other.to_string(source),
                                line,
                            ))
                        }
                    }
                }
                Ok(Value::None)
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "group_by" => {
            // group_by :: (a -> b) -> [a] -> Dict
            // Group list elements by a key function
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let mut groups: HashMap<String, Vec<Value>> = HashMap::new();
                for item in items {
                    let key_val =
                        apply_function(func, item.clone(), source, line).map_err(|mut err| {
                            if !err.message.starts_with("group_by:") {
                                err.message = format!("group_by: {}", err.message);
                            }
                            err
                        })?;
                    // Convert key to string for Dict key
                    let key = key_val.to_string(source);
                    groups.entry(key).or_default().push(item.clone());
                }
                // Convert to Dict
                let dict: HashMap<String, Value> = groups
                    .into_iter()
                    .map(|(k, v)| (k, Value::List(v)))
                    .collect();
                Ok(Value::Dict(dict))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "zip_with" => {
            // zip_with :: (a -> b -> c) -> [a] -> [b] -> [c]
            // Combine two lists with a function
            let func = &args[0];
            let list1 = &args[1];
            let list2 = &args[2];
            match (list1, list2) {
                (Value::List(items1), Value::List(items2)) => {
                    let mut result = Vec::new();
                    let len = std::cmp::min(items1.len(), items2.len());
                    for i in 0..len {
                        // Apply function to first arg, then to second arg (curried)
                        let partial = apply_function(func, items1[i].clone(), source, line)
                            .map_err(|mut err| {
                                if !err.message.starts_with("zip_with:") {
                                    err.message = format!("zip_with: {}", err.message);
                                }
                                err
                            })?;
                        let res = apply_function(&partial, items2[i].clone(), source, line)
                            .map_err(|mut err| {
                                if !err.message.starts_with("zip_with:") {
                                    err.message = format!("zip_with: {}", err.message);
                                }
                                err
                            })?;
                        result.push(res);
                    }
                    Ok(Value::List(result))
                }
                (Value::List(_), other) => Err(EvalError::type_mismatch(
                    "list",
                    other.to_string(source),
                    line,
                )),
                (other, _) => Err(EvalError::type_mismatch(
                    "list",
                    other.to_string(source),
                    line,
                )),
            }
        }
        "intersperse" => {
            // intersperse :: a -> [a] -> [a]
            // Insert element between all list items
            let sep = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                if items.is_empty() {
                    return Ok(Value::List(vec![]));
                }
                let mut result = Vec::with_capacity(items.len() * 2 - 1);
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        result.push(sep.clone());
                    }
                    result.push(item.clone());
                }
                Ok(Value::List(result))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "pmap" => {
            // pmap :: (a -> b) -> [a] -> [b]
            // Parallel version of map using rayon
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let func = func.clone();
                let source = source.to_string();

                let results: Result<Vec<Value>, EvalError> = items
                    .par_iter()
                    .map(|item| {
                        apply_function(&func, item.clone(), &source, line).map_err(|mut err| {
                            if !err.message.starts_with("pmap:") {
                                err.message = format!("pmap: {}", err.message);
                            }
                            err
                        })
                    })
                    .collect();

                Ok(Value::List(results?))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "pfilter" => {
            // pfilter :: (a -> Bool) -> [a] -> [a]
            // Parallel version of filter using rayon
            let func = &args[0];
            let list = &args[1];
            if let Value::List(items) = list {
                let func = func.clone();
                let source = source.to_string();

                // Evaluate predicates in parallel
                let predicate_results: Result<Vec<(Value, bool)>, EvalError> = items
                    .par_iter()
                    .map(|item| {
                        let res = apply_function(&func, item.clone(), &source, line).map_err(
                            |mut err| {
                                if !err.message.starts_with("pfilter:") {
                                    err.message = format!("pfilter: {}", err.message);
                                }
                                err
                            },
                        )?;
                        match res {
                            Value::Bool(b) => Ok((item.clone(), b)),
                            other => Err(EvalError::type_mismatch("bool", other.to_string(&source), line)),
                        }
                    })
                    .collect();

                // Filter based on results (maintaining order)
                let filtered: Vec<Value> = predicate_results?
                    .into_iter()
                    .filter(|(_, keep)| *keep)
                    .map(|(item, _)| item)
                    .collect();

                Ok(Value::List(filtered))
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        "pfold" => {
            // pfold :: (a -> a -> a) -> a -> [a] -> a
            // Parallel fold using rayon
            // Note: The combining function must be associative for correct results
            // Strategy: Split list into chunks, fold each chunk in parallel, then combine results
            let func = &args[0];
            let identity = args[1].clone();
            let list = &args[2];
            if let Value::List(items) = list {
                if items.is_empty() {
                    return Ok(identity);
                }

                let func = func.clone();
                let source = source.to_string();

                // Calculate chunk size based on number of CPUs
                let num_threads = rayon::current_num_threads();
                let chunk_size = (items.len() / num_threads).max(1);

                // Process chunks in parallel
                let chunk_results: Result<Vec<Value>, EvalError> = items
                    .chunks(chunk_size)
                    .collect::<Vec<_>>()
                    .par_iter()
                    .map(|chunk| {
                        // Sequential fold within each chunk
                        let mut acc = identity.clone();
                        for item in *chunk {
                            let step =
                                apply_function(&func, acc, &source, line).map_err(|mut err| {
                                    if !err.message.starts_with("pfold:") {
                                        err.message = format!("pfold: {}", err.message);
                                    }
                                    err
                                })?;
                            acc = apply_function(&step, item.clone(), &source, line).map_err(
                                |mut err| {
                                    if !err.message.starts_with("pfold:") {
                                        err.message = format!("pfold: {}", err.message);
                                    }
                                    err
                                },
                            )?;
                        }
                        Ok(acc)
                    })
                    .collect();

                // Combine chunk results sequentially
                let partial_results = chunk_results?;
                let mut final_acc = identity;
                for partial in partial_results {
                    let step =
                        apply_function(&func, final_acc, &source, line).map_err(|mut err| {
                            if !err.message.starts_with("pfold:") {
                                err.message = format!("pfold: {}", err.message);
                            }
                            err
                        })?;
                    final_acc =
                        apply_function(&step, partial, &source, line).map_err(|mut err| {
                            if !err.message.starts_with("pfold:") {
                                err.message = format!("pfold: {}", err.message);
                            }
                            err
                        })?;
                }

                Ok(final_acc)
            } else {
                Err(EvalError::type_mismatch(
                    "list",
                    list.to_string(source),
                    line,
                ))
            }
        }
        _ => Err(EvalError::new(
            format!("unknown list function: {}", name),
            None,
            None,
            line,
        )),
    }
}
