//! List operations: drop, enumerate, filter, flatmap, flatten, fold, head, map, partition, range, reverse, sort, sort_by, split_at, tail, take, unique, unzip, zip

/// Names of list builtins
pub const NAMES: &[&str] = &[
    "drop",
    "enumerate",
    "filter",
    "flatmap",
    "flatten",
    "fold",
    "head",
    "map",
    "partition",
    "range",
    "reverse",
    "sort",
    "sort_by",
    "split_at",
    "tail",
    "take",
    "unique",
    "unzip",
    "zip",
];

/// Get arity for list functions
pub fn get_arity(name: &str) -> Option<usize> {
    match name {
        "enumerate" | "flatten" | "head" | "reverse" | "sort" | "tail" | "unique" | "unzip" => {
            Some(1)
        }
        "drop" | "filter" | "flatmap" | "map" | "partition" | "range" | "sort_by" | "split_at"
        | "take" | "zip" => Some(2),
        "fold" => Some(3),
        _ => None,
    }
}

/// Check if name is a list builtin
pub fn is_builtin(name: &str) -> bool {
    NAMES.contains(&name)
}
