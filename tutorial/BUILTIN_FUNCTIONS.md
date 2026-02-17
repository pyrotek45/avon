# Built-in Functions Reference

Avon comes with a comprehensive standard library of built-in functions, plus constants like `os`. All functions are curried, meaning they can be partially applied.

**Note:** This document provides a complete reference of all built-in functions with their type signatures. For detailed tutorials on how to use these functions, see [TUTORIAL.md](./TUTORIAL.md), particularly the "Collections" and "Builtin Functions" sections.

**Quick Navigation:**
- [Aggregate Functions](#aggregate-functions) — sum, max, min, all, any, count
- [Date/Time Functions](#datetime-functions) — date operations and timestamps
- [Debug Functions](#debug-functions) — trace, debug, assert
- [Dictionary Functions](#dictionary-functions) — get, set, merge, keys, values
- [Environment Functions](#environment-functions) — env_var, os
- [File I/O Functions](#file-io-functions) — readfile, glob, import, json_parse, xml_parse, html_parse, opml_parse, ini_parse, *_parse_string
- [Formatting Functions](#formatting-functions) — format_json, format_csv, format_xml, format_html, format_opml, format_ini, format_table
- [HTML Functions](#html-functions) — html_escape, html_tag, html_attr
- [List Functions](#list-functions) — map, filter, fold, sort, unique
- [Markdown Functions](#markdown-functions) — md_heading, md_link, md_list
- [Math Functions](#math-functions) — abs, pow, sqrt, ceil, floor
- [Regex Functions](#regex-functions) — regex_match, regex_replace, scan
- [String Functions](#string-functions) — concat, upper, lower, split, replace
- [Type Functions](#type-functions) — typeof, is_string, to_int, etc.
- [Data Format Conversion](#data-format-conversion) — JSON ↔ YAML ↔ TOML ↔ CSV ↔ XML ↔ HTML ↔ OPML ↔ INI

---

## Aggregate Functions

Functions for aggregating values from lists.

| Function | Signature | Description |
|----------|-----------|-------------|
| `all` | `(a -> Bool) -> [a] -> Bool` | Returns true if the predicate returns true for all items in the list. |
| `any` | `(a -> Bool) -> [a] -> Bool` | Returns true if the predicate returns true for any item in the list. |
| `count` | `(a -> Bool) -> [a] -> Number` | Returns the number of items where the predicate returns true. |
| `default` | `a -> a -> a` | Provides a default value if the second argument is `None`. Otherwise returns the second argument. |
| `max` | `[Number\|String] -> Number\|String` | Returns the maximum value in a list, or `None` if the list is empty. |
| `min` | `[Number\|String] -> Number\|String` | Returns the minimum value in a list, or `None` if the list is empty. |
| `product` | `[Number] -> Number` | Returns the product of all numbers in a list. |
| `sum` | `[Number] -> Number` | Returns the sum of all numbers in a list. |

**Examples:**
```avon
sum [1, 2, 3, 4, 5]                    # 15
product [2, 3, 4]                      # 24
max [5, 2, 8, 1, 9]                    # 9
min [5, 2, 8, 1, 9]                    # 1
all (\x x > 0) [1, 2, 3, 4]            # true
any (\x x > 3) [1, 2, 3, 4]            # true
count (\x x > 2) [1, 2, 3, 4, 5]       # 3

# Using default with data access functions
head []                        # None
head [] -> default "empty"     # "empty"

get {a: 1} "b"                # None
get {a: 1} "b" -> default 0   # 0

find (\x x > 10) [1, 2, 3]    # None
find (\x x > 10) [1, 2, 3] -> default 999  # 999
```

## Date/Time Functions

Functions for working with dates and times. Dates are represented as ISO 8601 strings (e.g., "2024-12-10T15:30:00Z").

| Function | Signature | Description |
|----------|-----------|-------------|
| `date_add` | `String -> String -> String` | Adds a duration (e.g., "1d", "2h") to an ISO 8601 date string. |
| `date_diff` | `String -> String -> Number` | Calculates the difference between two ISO 8601 dates in seconds. |
| `date_format` | `String -> String -> String` | Formats an ISO 8601 date string using a format string (e.g., "%Y-%m-%d"). |
| `date_parse` | `String -> String -> String` | Parses a date string with a given format and returns an ISO 8601 string. |
| `now` | `() -> String` | Returns the current date and time in ISO 8601 format. |
| `timestamp` | `() -> Number` | Returns the current Unix timestamp (seconds since epoch). |
| `timezone` | `() -> String` | Returns the current timezone offset (e.g., "+00:00"). |

**Examples:**
```avon
now                                    # "2024-12-10T15:30:00Z"
timestamp                              # 1733850600
timezone                               # "+00:00"
date_add "2024-12-10T15:30:00Z" "1d"   # "2024-12-11T15:30:00Z"
date_format "2024-12-10T15:30:00Z" "%Y-%m-%d"  # "2024-12-10"
```

## Debug Functions

Functions for debugging and assertions. These are essential for development and validation.

| Function | Signature | Description |
|----------|-----------|-------------|
| `assert` | `Bool -> a -> a` | Returns the second argument if the first is true, otherwise raises an error. |
| `debug` | `String -> a -> a` | Prints a label and the value's internal structure to stderr, then returns it. |
| `error` | `String -> a` | Raises a runtime error with the given message. |
| `not` | `Bool -> Bool` | Logical negation. |
| `spy` | `a -> a` | Auto-numbered debug trace. Prints "[SPY 1] value" to stderr and returns the value. |
| `tap` | `(a -> b) -> a -> a` | Run a function for side effects, then return original value. |
| `trace` | `String -> a -> a` | Prints a label and value to stderr, then returns the value. |

**Examples:**
```avon
assert (5 > 0) "value"                 # "value"
not true                               # false
trace "debug" 42                        # prints "[debug] 42" to stderr, returns 42
```

---



Functions for working with dictionaries.

| Function | Signature | Description |
|----------|-----------|-------------|
| `dict_merge` | `Dict -> Dict -> Dict` | Merges two dictionaries. Keys in the second dict override the first. |
| `get` | `Dict -> String -> a\|None` | Returns the value for a key, or `None` if not found. |
| `has_key` | `Dict -> String -> Bool` | Returns true if the dictionary contains the key. |
| `keys` | `Dict -> [String]` | Returns a list of keys in the dictionary. |
| `set` | `Dict -> String -> a -> Dict` | Returns a new dictionary with the key set to the value. |
| `values` | `Dict -> [a]` | Returns a list of values in the dictionary. |

**Examples:**
```avon
let config = {host: "localhost", port: 8080} in

# Get existing key
get config "host"                   # "localhost"

# Get missing key returns None
get config "timeout"                # None

# Use default with get
get config "timeout" -> default 30  # 30

# Check if key exists
has_key config "port"               # true
has_key config "timeout"            # false
```

## Environment Functions

Functions for accessing the system environment.

| Function | Signature | Description |
|----------|-----------|-------------|
| `env_var` | `String -> String` | Returns the value of an environment variable. Errors if not set. |
| `env_var_or` | `String -> String -> String` | Returns the value of an environment variable, or a default if not set. |
| `env_vars` | `() -> Dict` | Returns a dictionary of all environment variables. |
| `os` | *Constant* | A constant string representing the operating system (e.g., "linux", "macos"). This is not a function but a value. |

## File I/O Functions

Functions for file system operations.

| Function | Signature | Description |
|----------|-----------|-------------|
| `abspath` | `String\|Path -> String` | Returns the absolute path. |
| `basename` | `String\|Path -> String` | Returns the filename portion of a path. |
| `csv_parse` | `String -> [Dict]\|[[String]]` | Parses a CSV file into a list of dicts (if headers) or list of lists. |
| `csv_parse_string` | `String -> [Dict]\|[[String]]` | Parses a raw CSV string into a list of dicts (if headers) or list of lists. |
| `dirname` | `String\|Path -> String` | Returns the directory portion of a path. |
| `exists` | `String\|Path -> Bool` | Returns true if the file or directory exists. |
| `fill_template` | `String\|Path -> Dict\|List -> String` | Reads a file and replaces placeholders `{key}` with values. |
| `glob` | `String -> [String]` | Returns a list of files matching the glob pattern. |
| `html_parse` | `String -> Dict` | Parses an HTML file into a nested Dict with tag, attrs, children, and text. Uses a real HTML5 parser. |
| `html_parse_string` | `String -> Dict` | Parses a raw HTML string into a nested Dict. Uses a real HTML5 parser. |
| `import` | `String\|Path -> a` | Imports and evaluates another Avon file. |
| `import_git` | `String -> String -> a` | Downloads and evaluates an Avon file from GitHub by commit hash (e.g., `import_git "owner/repo/file.av" "abc123..."` for GitHub safety). |
| `ini_parse` | `String -> Dict` | Parses an INI file into a Dict of section Dicts. Global keys go under "global". |
| `ini_parse_string` | `String -> Dict` | Parses a raw INI string into a Dict of section Dicts. |
| `json_parse` | `String -> Dict\|List\|a` | Parses a JSON file into an Avon value (Dict for objects, List for arrays). |
| `json_parse_string` | `String -> Dict\|List\|a` | Parses a raw JSON string into an Avon value. |
| `opml_parse` | `String -> Dict` | Parses an OPML file into a Dict with version, head, and outlines. |
| `opml_parse_string` | `String -> Dict` | Parses a raw OPML string into a Dict with version, head, and outlines. |
| `readfile` | `String\|Path -> String` | Reads the entire content of a file. |
| `readlines` | `String\|Path -> [String]` | Reads a file line by line into a list. |
| `relpath` | `String\|Path -> String\|Path -> String` | Returns the relative path from base to target. |
| `toml_parse` | `String -> Dict\|List\|a` | Parses a TOML file into an Avon value (Dict for tables, List for arrays). |
| `toml_parse_string` | `String -> Dict\|List\|a` | Parses a raw TOML string into an Avon value. |
| `walkdir` | `String\|Path -> [String]` | Recursively lists all files in a directory. |
| `xml_parse` | `String -> Dict` | Parses an XML file into a nested Dict with tag, attrs, children, and text. |
| `xml_parse_string` | `String -> Dict` | Parses a raw XML string into a nested Dict. |
| `yaml_parse` | `String -> Dict\|List\|a` | Parses a YAML file into an Avon value (Dict for mappings, List for sequences). |
| `yaml_parse_string` | `String -> Dict\|List\|a` | Parses a raw YAML string into an Avon value. |

## Formatting Functions

Functions for formatting values.

| Function | Signature | Description |
|----------|-----------|-------------|
| `center` | `String -> Number -> String` | Centers a string within a given width. |
| `format_binary` | `Number -> String` | Formats a number as binary. |
| `format_bool` | `Bool -> String -> String` | Formats a boolean (e.g., "yes/no", "on/off"). |
| `format_bytes` | `Number -> String` | Formats a number as bytes (e.g., "1.5 MB"). |
| `format_csv` | `[Dict]\|[[String]] -> String` | Formats a list of dicts or list of lists as a CSV string. |
| `format_currency` | `Number -> String -> String` | Formats a number as currency with a symbol. |
| `format_float` | `Number -> Number -> String` | Formats a float with specific precision. |
| `format_hex` | `Number -> String` | Formats a number as hexadecimal. |
| `format_html` | `Dict -> String` | Formats a Dict (with tag/attrs/children/text) as an indented HTML string. Handles void elements (br, img, hr, etc.). |
| `format_ini` | `Dict -> String` | Formats a Dict of section Dicts as an INI config string. |
| `format_int` | `Number -> Number -> String` | Formats an integer with minimum width (padding with zeros). |
| `format_json` | `a -> String` | Serializes a value to a JSON string. |
| `format_list` | `[a] -> String -> String` | Joins list items with a separator. |
| `format_octal` | `Number -> String` | Formats a number as octal. |
| `format_opml` | `Dict -> String` | Formats a Dict as an OPML 2.0 document with XML declaration. |
| `format_percent` | `Number -> Number -> String` | Formats a number as a percentage. |
| `format_scientific` | `Number -> Number -> String` | Formats a number in scientific notation. |
| `format_table` | `[[String]]\|Dict -> String -> String` | Formats data as a table with a separator. |
| `format_toml` | `a -> String` | Serializes a value to a TOML string. |
| `format_xml` | `Dict -> String` | Formats a Dict (with tag/attrs/children/text) as an indented XML string. |
| `format_yaml` | `a -> String` | Serializes a value to a YAML string. |
| `truncate` | `String -> Number -> String` | Truncates a string to a maximum length, adding "...". |

## HTML Functions

Functions for generating HTML.

| Function | Signature | Description |
|----------|-----------|-------------|
| `html_attr` | `String -> String -> String` | Generates an HTML attribute string (e.g., `key="value"`). |
| `html_escape` | `String -> String` | Escapes special HTML characters. |
| `html_tag` | `String -> String -> String` | Generates an HTML tag with content. |

## List Functions

Functions for working with lists.

| Function | Signature | Description |
|----------|-----------|-------------|
| `choice` | `[a] -> a` | Returns a random element from a list. Errors on empty list. |
| `chunks` | `Number -> [a] -> [[a]]` | Splits the list into chunks of size n. |
| `combinations` | `Number -> [a] -> [[a]]` | Returns all combinations of length k. |
| `drop` | `Number -> [a] -> [a]` | Returns the list without the first n items. |
| `enumerate` | `[a] -> [[Number, a]]` | Returns a list of [index, item] pairs. |
| `filter` | `(a -> Bool) -> [a] -> [a]` | Returns a list of items where the predicate is true. |
| `find` | `(a -> Bool) -> [a] -> a\|None` | Returns the first item matching the predicate, or None. |
| `find_index` | `(a -> Bool) -> [a] -> Number\|None` | Returns the index of the first matching item, or None. |
| `flatmap` | `(a -> [b]\|b) -> [a] -> [b]` | Maps a function and flattens the result. |
| `flatten` | `[[a]] -> [a]` | Flattens a list of lists. |
| `fold` | `(b -> a -> b) -> b -> [a] -> b` | Reduces a list to a single value using an accumulator. |
| `group_by` | `(a -> k) -> [a] -> Dict[k, [a]]` | Groups list items by the result of a key function. |
| `head` | `[a] -> a\|None` | Returns the first item, or None if empty. |
| `intersperse` | `a -> [a] -> [a]` | Inserts a separator between each element of a list. |
| `last` | `[a] -> a\|None` | Returns the last item, or None if empty. |
| `length` | `[a] -> Number` | Returns the number of items in the list. |
| `map` | `(a -> b) -> [a] -> [b]` | Applies a function to each item in the list. |
| `nth` | `Number -> [a] -> a\|None` | Returns the item at index (0-based), or None if out of bounds. |
| `partition` | `(a -> Bool) -> [a] -> [[a], [a]]` | Splits a list into two lists: [matches, non-matches]. |
| `permutations` | `Number -> [a] -> [[a]]` | Returns all permutations of length k. |
| `pfilter` | `(a -> Bool) -> [a] -> [a]` | Parallel filter. Like `filter` but uses multiple CPU cores. |
| `pfold` | `(b -> a -> b) -> b -> [a] -> b` | Parallel fold. **Requires an associative combiner function.** |
| `pmap` | `(a -> b) -> [a] -> [b]` | Parallel map. Like `map` but uses multiple CPU cores. |
| `drop` | `Number -> [a] -> [a]` | Removes the first n items. |
| `range` | `Number -> Number -> [Number]` | Generates a list of numbers from start to end (inclusive). |
| `reverse` | `[a] -> [a]` | Returns the list in reverse order. |
| `sample` | `Number -> [a] -> [a]` | Returns n unique random elements from a list. Errors if n > length. |
| `shuffle` | `[a] -> [a]` | Returns a new list with elements in random order. |
| `sort` | `[a] -> [a]` | Sorts the list. |
| `sort_by` | `(a -> b) -> [a] -> [a]` | Sorts the list based on the result of the key function. |
| `split_at` | `Number -> [a] -> [[a], [a]]` | Splits the list at the given index. |
| `tail` | `[a] -> [a]` | Returns the list without the first item. |
| `take` | `Number -> [a] -> [a]` | Returns the first n items. |
| `transpose` | `[[a]] -> [[a]]` | Transposes a list of lists (matrix). |
| `unique` | `[a] -> [a]` | Returns the list with duplicates removed. |
| `unzip` | `[[a, b]] -> [[a], [b]]` | Splits a list of pairs into two lists. |
| `windows` | `Number -> [a] -> [[a]]` | Returns sliding windows of size n. |
| `zip` | `[a] -> [b] -> [[a, b]]` | Combines two lists into a list of pairs. |
| `zip_with` | `(a -> b -> c) -> [a] -> [b] -> [c]` | Combines two lists using a function. |

**Examples:**
```avon
# Basic list operations
map (\x x * 2) [1, 2, 3]                # [2, 4, 6]
filter (\x x > 2) [1, 2, 3, 4, 5]       # [3, 4, 5]
fold (\acc \x acc + x) 0 [1, 2, 3]     # 6

# Parallel list operations (same results, uses multiple CPU cores)
pmap (\x x * 2) [1, 2, 3]               # [2, 4, 6]
pfilter (\x x > 2) [1, 2, 3, 4, 5]      # [3, 4, 5]
pfold (\acc \x acc + x) 0 [1, 2, 3]    # 6
# NOTE: pfold requires an ASSOCIATIVE combiner (e.g., +, *, max, min)
# Non-associative operations like subtraction will give different results!

# Random selection
choice [1, 2, 3, 4, 5]                  # A random element, e.g., 3
shuffle [1, 2, 3, 4, 5]                 # A randomized list, e.g., [3, 1, 5, 2, 4]
sample 3 [1, 2, 3, 4, 5]                # 3 random unique elements, e.g., [2, 5, 1]

# Finding elements
find (\x x > 5) [1, 3, 7, 2, 9]         # 7 (first match)
find (\x x > 100) [1, 2, 3]             # None (no match)
find_index (\x x > 5) [1, 3, 7, 2, 9]   # 2 (index of 7)

# Grouping elements
group_by (\x x % 2) [1, 2, 3, 4, 5, 6]  # {0: [2, 4, 6], 1: [1, 3, 5]}
group_by length ["a", "bb", "c", "ddd"] # {1: ["a", "c"], 2: ["bb"], 3: ["ddd"]}

# Combining lists with functions
zip_with (\a \b a + b) [1, 2, 3] [10, 20, 30]   # [11, 22, 33]
zip_with (\a \b a * b) [2, 3, 4] [5, 6, 7]      # [10, 18, 28]

# Inserting separators
intersperse 0 [1, 2, 3]                 # [1, 0, 2, 0, 3]
intersperse ", " ["a", "b", "c"]        # ["a", ", ", "b", ", ", "c"]

# Other list operations
take 3 [1, 2, 3, 4, 5]                  # [1, 2, 3]
drop 2 [1, 2, 3, 4, 5]                  # [3, 4, 5]
unique [1, 2, 2, 3, 3, 3]               # [1, 2, 3]
reverse [1, 2, 3]                       # [3, 2, 1]
sort [3, 1, 4, 1, 5]                    # [1, 1, 3, 4, 5]
```

## Markdown Functions

Functions for generating Markdown.

| Function | Signature | Description |
|----------|-----------|-------------|
| `markdown_to_html` | `String -> String` | Converts simple Markdown to HTML. |
| `md_code` | `String -> String` | Formats text as inline code. |
| `md_heading` | `Number -> String -> String` | Creates a Markdown heading of the given level. |
| `md_link` | `String -> String -> String` | Creates a Markdown link `[text](url)`. |
| `md_list` | `[String] -> String` | Formats a list as a Markdown bullet list. |

## Math Functions

Mathematical functions.

**Operators:** In addition to these functions, Avon supports math operators:
- `**` — Power/exponentiation (right-associative): `2 ** 3` → `8`
- `/` — Division (always returns float): `10 / 3` → `3.333...`
- `//` — Integer division (floor toward -∞): `10 // 3` → `3`, `-7 // 3` → `-3`
- `%` — Modulo/remainder: `10 % 3` → `1`

**Arithmetic Edge Cases:** All operators handle edge cases gracefully:
- Division/modulo by zero → Runtime error (not a panic)
- Integer overflow → Wraps (e.g., `MAX + 1` → `MIN`)
- `MIN // -1` → `MIN` (wrapping behavior)
- `MIN % -1` → `0` (mathematically correct)
- `0 ** -1` → `inf`, `(-1) ** 0.5` → `NaN`, `0 ** 0` → `1`
- `sqrt (-1)` → `NaN`, `log 0` → `-inf`, `log (-1)` → `NaN`

| Function | Signature | Description |
|----------|-----------|-------------|
| `abs` | `Number -> Number` | Returns the absolute value of a number. |
| `ceil` | `Number -> Number` | Rounds up to the nearest integer (toward +∞). |
| `floor` | `Number -> Number` | Rounds down to the nearest integer (toward -∞). |
| `gcd` | `Number -> Number -> Number` | Returns the greatest common divisor of two numbers. |
| `lcm` | `Number -> Number -> Number` | Returns the least common multiple of two numbers. |
| `log` | `Number -> Number` | Returns the natural logarithm (base e). Returns `-inf` for 0, `NaN` for negative. |
| `log10` | `Number -> Number` | Returns the base-10 logarithm. Returns `-inf` for 0, `NaN` for negative. |
| `neg` | `Number -> Number` | Negates a number. |
| `pow` | `Number -> Number -> Number` | Raises a number to a power. Also available as `**` operator. |
| `random_float` | `Number -> Number -> Number` | Returns a random float in the range [min, max]. |
| `random_int` | `Number -> Number -> Number` | Returns a random integer in the range [min, max] (inclusive). |
| `round` | `Number -> Number` | Rounds to the nearest integer. |
| `sqrt` | `Number -> Number` | Returns the square root. Returns `NaN` for negative numbers. |
| `uuid` | `() -> String` | Generates a new random UUID v4 string. |

## Regex Functions

Functions for regular expressions.

| Function | Signature | Description |
|----------|-----------|-------------|
| `regex_match` | `String -> String -> Bool` | Returns true if the text matches the regex pattern. |
| `regex_replace` | `String -> String -> String -> String` | Replaces all matches of the regex pattern with the replacement string. |
| `regex_split` | `String -> String -> [String]` | Splits the text by the regex pattern. |
| `scan` | `String -> String -> [String\|[String]]` | Returns a list of all matches (or capture groups) in the text. |

## String Functions

Functions for string manipulation.

| Function | Signature | Description |
|----------|-----------|-------------|
| `char_at` | `String -> Number -> String` | Returns the character at the given index. |
| `chars` | `String -> [String]` | Returns a list of characters in the string. |
| `concat` | `String -> String -> String` | Concatenates two strings. |
| `contains` | `String -> String -> Bool` or `a -> [a] -> Bool` | Returns true if string contains substring, OR if list contains element. |
| `ends_with` | `String -> String -> Bool` | Returns true if the string ends with the suffix. |
| `indent` | `String -> Number -> String` | Indents each line of the string by n spaces. |
| `is_alpha` | `String -> Bool` | Returns true if the string contains only alphabetic characters. |
| `is_alphanumeric` | `String -> Bool` | Returns true if the string contains only alphanumeric characters. |
| `is_digit` | `String -> Bool` | Returns true if the string contains only digits. |
| `is_empty` | `String\|List\|Dict -> Bool` | Returns true if the value is empty. |
| `is_lowercase` | `String -> Bool` | Returns true if the string contains only lowercase characters. |
| `is_uppercase` | `String -> Bool` | Returns true if the string contains only uppercase characters. |
| `is_whitespace` | `String -> Bool` | Returns true if the string contains only whitespace. |
| `join` | `[String] -> String -> String` | Joins a list of strings with a separator. |
| `length` | `String\|List -> Number` | Returns the length of a string or list. |
| `lines` | `String -> [String]` | Splits a string into lines (by newlines). |
| `lower` | `String -> String` | Converts the string to lowercase. |
| `pad_left` | `String -> Number -> String -> String` | Pads the string on the left to the given width. |
| `pad_right` | `String -> Number -> String -> String` | Pads the string on the right to the given width. |
| `repeat` | `String -> Number -> String` | Repeats the string n times. |
| `replace` | `String -> String -> String -> String` | Replaces occurrences of a substring with another string. |
| `slice` | `String\|List -> Number -> Number -> String\|List` | Returns a slice of the string or list. |
| `split` | `String -> String -> [String]` | Splits the string by a separator. |
| `starts_with` | `String -> String -> Bool` | Returns true if the string starts with the prefix. |
| `trim` | `String -> String` | Removes leading and trailing whitespace. |
| `unlines` | `[String] -> String` | Joins lines with newlines. |
| `unwords` | `[String] -> String` | Joins words with a single space. |
| `upper` | `String -> String` | Converts the string to uppercase. |
| `words` | `String -> [String]` | Splits a string into words (by whitespace). |
| `base64_encode` | `String -> String` | Encodes a string to Base64. |
| `base64_decode` | `String -> String` | Decodes a Base64-encoded string. |
| `hash_md5` | `String -> String` | Returns the MD5 hash of a string as a hex string. |
| `hash_sha256` | `String -> String` | Returns the SHA-256 hash of a string as a hex string. |

**Examples:**
```avon
concat "hello" "world"                      # "helloworld"
upper "hello"                               # "HELLO"
lower "HELLO"                               # "hello"
split "a,b,c" ","                          # ["a", "b", "c"]
join ["a", "b", "c"] "-"                    # "a-b-c"
contains "hello world" "world"              # true (string contains substring)
contains 3 [1, 2, 3, 4]                     # true (list contains element)
starts_with "hello" "he"                    # true
ends_with "hello" "lo"                      # true
trim "  hello  "                            # "hello"
length "hello"                              # 5
char_at "hello" 1                           # "e"
chars "abc"                                 # ["a", "b", "c"]
repeat "ab" 3                               # "ababab"
replace "hello world" "world" "Avon"        # "hello Avon"
base64_encode "Hello, World!"               # "SGVsbG8sIFdvcmxkIQ=="
base64_decode "SGVsbG8sIFdvcmxkIQ=="        # "Hello, World!"
hash_md5 "hello"                            # "5d41402abc4b2a76b9719d911017c592"
hash_sha256 "hello"                         # "2cf24dba5fb0a30e26e83b2ac5b9e29e..."
```

## Type Functions

Functions for type checking and conversion.

| Function | Signature | Description |
|----------|-----------|-------------|
| `is_bool` | `a -> Bool` | Returns true if the value is a boolean. |
| `is_dict` | `a -> Bool` | Returns true if the value is a dictionary. |
| `is_float` | `a -> Bool` | Returns true if the value is a float. |
| `is_function` | `a -> Bool` | Returns true if the value is a function. |
| `is_int` | `a -> Bool` | Returns true if the value is an integer. |
| `is_list` | `a -> Bool` | Returns true if the value is a list. |
| `is_none` | `a -> Bool` | Returns true if the value is None. |
| `is_number` | `a -> Bool` | Returns true if the value is a number. |
| `is_string` | `a -> Bool` | Returns true if the value is a string. |
| `to_bool` | `a -> Bool` | Converts the value to a boolean. |
| `to_char` | `Number -> String` | Converts a Unicode codepoint to a character. |
| `to_float` | `a -> Number` | Converts the value to a float. |
| `to_int` | `a -> Number` | Converts the value to an integer. |
| `to_list` | `String -> [String]` | Converts a string to a list of characters. |
| `to_string` | `a -> String` | Converts the value to a string. |
| `typeof` | `a -> String` | Returns the type name of the value. |

## Data Format Conversion

Avon supports **8 structured data formats**, each with a paired parser and formatter for full round-trip conversion. You can parse any format into Avon values, transform the data, and output it in any other format.

### The Universal Value Type

This is the key idea that makes Avon's data conversion powerful: **every parser produces the same Avon value types** (Dict, List, String, Number, Bool), and **every formatter consumes those same types**. There is no "JSON object" or "YAML mapping" — there are only Avon Dicts and Lists.

```
    ┌──────────┐                           ┌──────────┐
    │   JSON   │──json_parse──┐            │   JSON   │
    │   YAML   │──yaml_parse──┤            │   YAML   │
    │   TOML   │──toml_parse──┤  ┌──────┐  │   TOML   │
    │   CSV    │──csv_parse───┼─▶│ Dict │──┼─▶format_* │
    │   XML    │──xml_parse───┤  │ List │  │   CSV    │
    │   HTML   │──html_parse──┤  └──────┘  │   XML    │
    │   OPML   │──opml_parse──┤            │   HTML   │
    │   INI    │──ini_parse───┘            │   OPML   │
    └──────────┘                           │   INI    │
         Any file in                       └──────────┘
                                             Any format out
    ┌──────────┐
    │ Raw      │──*_parse_string──▶ Same Dict/List
    │ Strings  │                   values as above
    └──────────┘
```

Once data is parsed, it's just a Dict or List — you can use **every Avon builtin** on it:

```avon
# Parse JSON → it's just a Dict now
let config = json_parse "config.json" in

typeof config                          # "Dict"
keys config                            # ["host", "port", "debug"]
get config "host"                      # "localhost"
has_key config "port"                  # true
config.host                            # "localhost" (dot notation works)

# Use map, filter, fold on parsed lists — same as any Avon list
let users = json_parse "users.json" in
map (\u u.name) users                  # extract all names
filter (\u u.age > 30) users           # keep users over 30
fold (\acc \u acc + u.age) 0 users     # sum all ages
```

This means you never need to learn format-specific APIs. Once you know `map`, `filter`, `fold`, `keys`, `values`, `get`, `set`, and dot notation, you can work with data from **any** of the 8 formats.

### Supported Formats

| Format | Parser | Formatter | Typical Use |
|--------|--------|-----------|-------------|
| **JSON** | `json_parse` | `format_json` | APIs, configs, data exchange |
| **YAML** | `yaml_parse` | `format_yaml` | Kubernetes, Docker Compose, CI/CD |
| **TOML** | `toml_parse` | `format_toml` | Cargo.toml, pyproject.toml, configs |
| **CSV** | `csv_parse` | `format_csv` | Spreadsheets, data exports, reports |
| **XML** | `xml_parse` | `format_xml` | Configs, SOAP, RSS, data interchange |
| **HTML** | `html_parse` | `format_html` | Web scraping, HTML analysis, template generation |
| **OPML** | `opml_parse` | `format_opml` | RSS subscriptions, podcast directories |
| **INI** | `ini_parse` | `format_ini` | App configs, database settings, .gitconfig |

### Round-Trip (Parse → Format)

Parse a file and re-format it in the same format:

```avon
# Parse JSON and format it back
let data = json_parse "config.json" in
format_json data

# Parse XML and format it back
let doc = xml_parse "data.xml" in
format_xml doc

# Parse OPML and format it back
let feeds = opml_parse "feeds.opml" in
format_opml feeds

# Parse HTML and format it back
let page = html_parse "index.html" in
format_html page

# Parse INI and format it back
let cfg = ini_parse "config.ini" in
format_ini cfg
```

### Cross-Format Conversion

Parse one format and output another — Avon values are the universal intermediate representation:

```avon
# JSON to YAML
json_parse "data.json" -> format_yaml

# YAML to TOML
yaml_parse "config.yml" -> format_toml

# TOML to JSON
toml_parse "Cargo.toml" -> format_json

# CSV to JSON
csv_parse "data.csv" -> format_json

# XML to JSON
xml_parse "config.xml" -> format_json

# XML to YAML
xml_parse "data.xml" -> format_yaml

# HTML to JSON
html_parse "page.html" -> format_json

# INI to JSON
ini_parse "config.ini" -> format_json

# JSON to INI
json_parse "config.json" -> format_ini
```

### With Transformation

Parse data, transform it, then output in any format:

```avon
# Read CSV, filter rows, output as JSON
let data = csv_parse "users.csv" in
let active = filter (\u u.status == "active") data in
format_json active

# Read JSON config, modify a field, output as YAML
let config = json_parse "config.json" in
let updated = set config "port" 9090 in
format_yaml updated

# Read XML, extract data, output as CSV
let doc = xml_parse "people.xml" in
let rows = map (\p {name: p.attrs.name, age: p.attrs.age}) doc.children in
format_csv rows
```
