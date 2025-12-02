# Avon Builtin Functions Reference

Complete reference of all 114+ builtin functions in Avon, organized by category.

## Table of Contents
- [String Functions](#string-functions)
- [List Functions](#list-functions)
- [Math Functions](#math-functions)
- [Type Functions](#type-functions)
- [Comparison Functions](#comparison-functions)
- [I/O Functions](#io-functions)
- [Formatting Functions](#formatting-functions)
- [Date/Time Functions](#datetime-functions)
- [HTML/Markdown Functions](#htmlmarkdown-functions)
- [Path Functions](#path-functions)
- [Dictionary Functions](#dictionary-functions)

---

## String Functions

### Basic Operations

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `concat` | `(s1: str, s2: str) -> str` | Concatenate two strings | `concat "hello" " world"` → `"hello world"` |
| `upper` | `(s: str) -> str` | Convert to uppercase | `upper "hello"` → `"HELLO"` |
| `lower` | `(s: str) -> str` | Convert to lowercase | `lower "HELLO"` → `"hello"` |
| `trim` | `(s: str) -> str` | Remove leading/trailing whitespace | `trim "  hello  "` → `"hello"` |
| `length` | `(s: str) -> int` | Get string length | `length "hello"` → `5` |
| `repeat` | `(s: str, n: int) -> str` | Repeat string n times | `repeat "ab" 3` → `"ababab"` |

### Substring/Pattern Operations

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `split` | `(s: str, sep: str) -> [str]` | Split string by separator | `split "a,b,c" ","` → `["a", "b", "c"]` |
| `join` | `([str], sep: str) -> str` | Join list with separator | `join ["a", "b"] ","` → `"a,b"` |
| `replace` | `(s: str, old: str, new: str) -> str` | Replace substring | `replace "hello" "l" "L"` → `"heLLo"` |
| `slice` | `(s: str, start: int, end: int) -> str` | Extract substring [start..end) | `slice "hello" 1 4` → `"ell"` |
| `char_at` | `(s: str, index: int) -> str\|None` | Get character at index | `char_at "hello" 2` → `"l"` |
| `chars` | `(s: str) -> [str]` | Convert string to list of chars | `chars "hi"` → `["h", "i"]` |
| `contains` | `(s: str, sub: str) -> bool` | Check if substring exists | `contains "hello" "ell"` → `true` |
| `starts_with` | `(s: str, prefix: str) -> bool` | Check if string starts with prefix | `starts_with "hello" "hel"` → `true` |
| `ends_with` | `(s: str, suffix: str) -> bool` | Check if string ends with suffix | `ends_with "hello" "lo"` → `true` |

### Padding & Formatting

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `pad_left` | `(s: str, len: int, pad: str) -> str` | Pad string on left | `pad_left "hi" 5 " "` → `"   hi"` |
| `pad_right` | `(s: str, len: int, pad: str) -> str` | Pad string on right | `pad_right "hi" 5 " "` → `"hi   "` |
| `indent` | `(s: str, spaces: int) -> str` | Indent each line | `indent "a\nb" 2` → `"  a\n  b"` |

### String Predicates

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `is_digit` | `(s: str) -> bool` | Check if all characters are digits | `is_digit "123"` → `true` |
| `is_alpha` | `(s: str) -> bool` | Check if all characters are alphabetic | `is_alpha "hello"` → `true` |
| `is_alphanumeric` | `(s: str) -> bool` | Check if all characters are alphanumeric | `is_alphanumeric "hello123"` → `true` |
| `is_whitespace` | `(s: str) -> bool` | Check if all characters are whitespace | `is_whitespace "   "` → `true` |
| `is_uppercase` | `(s: str) -> bool` | Check if all letters are uppercase | `is_uppercase "HELLO"` → `true` |
| `is_lowercase` | `(s: str) -> bool` | Check if all letters are lowercase | `is_lowercase "hello"` → `true` |

---

## List Functions

### Basic Operations

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `head` | `([a]) -> a` | Get first element | `head [1, 2, 3]` → `1` |
| `tail` | `([a]) -> [a]` | Get all but first element | `tail [1, 2, 3]` → `[2, 3]` |
| `length` | `([a]) -> int` | Get list length | `length [1, 2, 3]` → `3` |
| `reverse` | `([a]) -> [a]` | Reverse list | `reverse [1, 2, 3]` → `[3, 2, 1]` |
| `sort` | `([a]) -> [a]` | Sort list (numbers ascending, strings lexicographically) | `sort [3, 1, 2]` → `[1, 2, 3]` |
| `unique` | `([a]) -> [a]` | Remove duplicates (preserves first occurrence order) | `unique [1, 2, 1, 3]` → `[1, 2, 3]` |

### Higher-Order Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `map` | `((a -> b), [a]) -> [b]` | Apply function to each element | `map (\x x*2) [1, 2, 3]` → `[2, 4, 6]` |
| `filter` | `((a -> bool), [a]) -> [a]` | Keep elements where predicate is true | `filter (\x x > 1) [1, 2, 3]` → `[2, 3]` |
| `fold` | `((acc, a -> acc), acc, [a]) -> acc` | Reduce list to single value | `fold (\a \x a+x) 0 [1,2,3]` → `6` |
| `flatmap` | `((a -> [b]), [a]) -> [b]` | Map then flatten | `flatmap (\x [x, x]) [1,2]` → `[1, 1, 2, 2]` |
| `flatten` | `([[a]]) -> [a]` | Flatten one level | `flatten [[1,2], [3,4]]` → `[1, 2, 3, 4]` |
| `sort_by` | `((a -> comparable), [a]) -> [a]` | Sort by key function | `sort_by length ["aaa", "b"]` → `["b", "aaa"]` |

### List Manipulation

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `take` | `([a], n: int) -> [a]` | Take first n elements | `take [1, 2, 3, 4] 2` → `[1, 2]` |
| `drop` | `([a], n: int) -> [a]` | Drop first n elements | `drop [1, 2, 3, 4] 2` → `[3, 4]` |
| `slice` | `([a], start: int, end: int) -> [a]` | Extract sublist [start..end) | `slice [1,2,3,4,5] 1 4` → `[2, 3, 4]` |
| `split_at` | `([a], n: int) -> [[a], [a]]` | Split list at position | `split_at [1,2,3,4] 2` → `[[1,2], [3,4]]` |
| `zip` | `([a], [b]) -> [[a, b]]` | Zip two lists | `zip [1,2] ["a","b"]` → `[[1,"a"], [2,"b"]]` |
| `unzip` | `([[a, b]]) -> [[a], [b]]` | Unzip list of pairs | `unzip [[1,"a"], [2,"b"]]` → `[[1,2], ["a","b"]]` |
| `enumerate` | `([a]) -> [[int, a]]` | Add indices | `enumerate ["a","b"]` → `[[0,"a"], [1,"b"]]` |
| `range` | `(start: int, end: int) -> [int]` | Generate range (inclusive both ends) | `range 1 4` → `[1, 2, 3, 4]` |
| `range` | `(start: int, end: int, step: int) -> [int]` | Generate range with step | `range 0 10 2` → `[0, 2, 4, 6, 8, 10]` |
| `partition` | `((a -> bool), [a]) -> [[a], [a]]` | Split list by predicate | `partition (\x x > 2) [1,2,3]` → `[[3], [1, 2]]` |

---

## Math Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `neg` | `(n: number) -> number` | Negate number | `neg 5` → `-5` |
| `abs` | `(n: number) -> number` | Absolute value | `abs -5` → `5` |

---

## Type Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `is_string` | `(x: any) -> bool` | Check if value is string | `is_string "hello"` → `true` |
| `is_number` | `(x: any) -> bool` | Check if value is number (int or float) | `is_number 42` → `true` |
| `is_int` | `(x: any) -> bool` | Check if value is integer | `is_int 42` → `true` |
| `is_float` | `(x: any) -> bool` | Check if value is float | `is_float 3.14` → `true` |
| `is_bool` | `(x: any) -> bool` | Check if value is boolean | `is_bool true` → `true` |
| `is_list` | `(x: any) -> bool` | Check if value is list | `is_list [1, 2]` → `true` |
| `is_dict` | `(x: any) -> bool` | Check if value is dict | `is_dict {a: 1}` → `true` |
| `is_function` | `(x: any) -> bool` | Check if value is function | `is_function (\x x+1)` → `true` |
| `is_none` | `(x: any) -> bool` | Check if value is none | `is_none none` → `true` |
| `is_empty` | `(x: any) -> bool` | Check if value is empty (for strings, lists, dicts) | `is_empty ""` → `true` |

### Type Conversion

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `to_string` | `(x: any) -> str` | Convert to string | `to_string 42` → `"42"` |
| `to_int` | `(x: any) -> int` | Convert to integer (truncates floats) | `to_int "42"` → `42` |
| `to_float` | `(x: any) -> float` | Convert to float | `to_float "3.14"` → `3.14` |
| `to_bool` | `(x: any) -> bool` | Convert to boolean | `to_bool 1` → `true` |
| `to_char` | `(n: int) -> str` | Convert ASCII code to character | `to_char 65` → `"A"` |
| `to_list` | `(s: str) -> [number]` | Convert string to list of char codes | `to_list "AB"` → `[65, 66]` |
| `typeof` | `(x: any) -> str` | Get type name as string | `typeof [1,2]` → `"list"` |

---

## Comparison Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `max` | `(a: comparable, b: comparable) -> comparable` | Get maximum | `max 5 3` → `5` |
| `min` | `(a: comparable, b: comparable) -> comparable` | Get minimum | `min 5 3` → `3` |

---

## I/O Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `readfile` | `(path: str) -> str` | Read entire file as string | `readfile "data.txt"` |
| `readlines` | `(path: str) -> [str]` | Read file as list of lines | `readlines "data.txt"` |
| `writefile` | `(path: str, content: str) -> none` | Write string to file | `writefile "out.txt" "hello"` |
| `exists` | `(path: str) -> bool` | Check if file exists | `exists "data.txt"` → `true` |
| `walkdir` | `(path: str) -> [str]` | List all files recursively | `walkdir "src"` |
| `json_parse` | `(path: str) -> any` | Parse JSON file | `json_parse "data.json"` |
| `import` | `(path: str) -> any` | Import and evaluate Avon file | `import "lib.av"` |
| `fill_template` | `(path: str, values: [[str, str]]) -> str` | Fill template with values | `fill_template "tmpl.txt" [["name", "Bob"]]` |
| `trace` | `(label: str, value: any) -> any` | Print value with label and return it | `trace "result" 42` |
| `debug` | `(value: any) -> any` | Print debug info and return value | `debug [1, 2, 3]` |
| `error` | `(message: str) -> none` | Raise error | `error "invalid input"` |
| `assert` | `(condition: bool, message: str) -> bool` | Assert condition true | `assert (x > 0) "x must be positive"` |

---

## Formatting Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `format_hex` | `(n: int) -> str` | Format as hexadecimal | `format_hex 255` → `"ff"` |
| `format_octal` | `(n: int) -> str` | Format as octal | `format_octal 64` → `"100"` |
| `format_binary` | `(n: int) -> str` | Format as binary | `format_binary 15` → `"1111"` |
| `format_scientific` | `(n: float, precision: int) -> str` | Scientific notation | `format_scientific 12345.6 2` → `"1.23e+04"` |
| `format_bytes` | `(n: int) -> str` | Format bytes as human-readable | `format_bytes 1024` → `"1.00 KB"` |
| `format_percent` | `(n: float, precision: int) -> str` | Format as percentage | `format_percent 0.75 1` → `"75.0%"` |
| `format_currency` | `(n: float, symbol: str) -> str` | Format as currency | `format_currency 19.99 "$"` → `"$19.99"` |
| `format_bool` | `(b: bool, style: str) -> str` | Format boolean | `format_bool true "yes/no"` → `"Yes"` |
| `format_list` | `([a], sep: str) -> str` | Format list with separator | `format_list ["a","b"] ", "` → `"a, b"` |
| `format_table` | `(data: any, sep: str) -> str` | Format as table | `format_table [["A","B"],[1,2]] " \| "` |
| `format_json` | `(x: any) -> str` | Format as JSON | `format_json {a: 1}` → `"{a: 1}"` |

---

## Date/Time Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `now` | `() -> int` | Current Unix timestamp | `now` → `1701475200` |
| `timestamp` | `() -> int` | Alias for now | `timestamp` → `1701475200` |
| `date_format` | `(ts: int, fmt: str) -> str` | Format timestamp | `date_format (now) "%Y-%m-%d"` |
| `date_parse` | `(s: str, fmt: str) -> int` | Parse date string | `date_parse "2024-12-01" "%Y-%m-%d"` |
| `date_add` | `(ts: int, duration: str) -> int` | Add duration to timestamp | `date_add (now) "1d"` |
| `date_diff` | `(ts1: int, ts2: int, unit: str) -> int` | Difference between timestamps | `date_diff t1 t2 "days"` |
| `timezone` | `() -> str` | Get current timezone | `timezone` → `"UTC"` |

---

## HTML/Markdown Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `html_escape` | `(s: str) -> str` | Escape HTML special characters | `html_escape "<div>"` → `"&lt;div&gt;"` |
| `html_tag` | `(tag: str, content: str) -> str` | Wrap in HTML tag | `html_tag "div" "hello"` → `"<div>hello</div>"` |
| `html_attr` | `(name: str, value: str) -> str` | Create HTML attribute | `html_attr "class" "btn"` → `"class=\"btn\""` |
| `markdown_to_html` | `(md: str) -> str` | Convert Markdown to HTML | `markdown_to_html "# Header"` → `"<h1>Header</h1>"` |

---

## Path Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `basename` | `(path: str) -> str` | Get filename from path | `basename "/path/to/file.txt"` → `"file.txt"` |
| `dirname` | `(path: str) -> str` | Get directory from path | `dirname "/path/to/file.txt"` → `"/path/to"` |

---

## Dictionary Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `get` | `(dict: {}, key: str) -> any` | Get value by key (returns None if missing) | `get {a: 1} "a"` → `1` |
| `set` | `(dict: {}, key: str, value: any) -> {}` | Set or update key | `set {a: 1} "b" 2` → `{a: 1, b: 2}` |
| `keys` | `(dict: {}) -> [str]` | Get all keys | `keys {a: 1, b: 2}` → `["a", "b"]` |
| `values` | `(dict: {}) -> [any]` | Get all values | `values {a: 1, b: 2}` → `[1, 2]` |
| `has_key` | `(dict: {}, key: str) -> bool` | Check if key exists | `has_key {a: 1} "a"` → `true` |

---

## Utility Functions

| Function | Signature | Description | Example |
|----------|-----------|-------------|---------|
| `center` | `(s: str, width: int, pad: str) -> str` | Center string | `center "hi" 7 " "` → `"  hi   "` |
| `truncate` | `(s: str, length: int, suffix: str) -> str` | Truncate string | `truncate "hello" 3 "..."` → `"he..."` |
| `env_var` | `(name: str) -> str` | Get environment variable | `env_var "PATH"` |
| `env_var_or` | `(name: str, default: str) -> str` | Get env var or default | `env_var_or "MISSING" "default"` → `"default"` |
| `os` | `() -> str` | Get OS name | `os` → `"linux"` or `"macos"` or `"windows"` |

---

## Notes on Currying

All builtins support **currying** - you can partially apply them:

```avon
# Partial application
let add5 = (\x x + 5) in
let add5 = map (\x x + 5) in  # equivalent

let concat_hello = concat "Hello, " in
concat_hello "World"  # → "Hello, World"

let split_by_comma = split (_, ",") in  # using underscore for placeholder
split_by_comma "a,b,c"  # → ["a", "b", "c"]
```

---

## Future Builtins (Planned)

These builtins are planned but not yet implemented:

- **`slice(s, start, end)`** - Extract substring/sublist ⭐ HIGH PRIORITY
- **`char_at(s, index)`** - Get character at index
- **`chars(s)`** - Convert string to character list
- **`find_index(list, element)`** - Find element index
- **`find(list, predicate)`** - Find first matching element
- **`any(list, predicate)`** - Check if any element matches
- **`all(list, predicate)`** - Check if all elements match
- **`group_by(list, key_fn)`** - Group list by key function

