# Built-in Functions Reference

Avon comes with a comprehensive standard library of built-in functions, plus constants like `os`. All functions are curried, meaning they can be partially applied.

**Scope:** This categorized reference lists all 231 unique names in the current `NAMES` arrays under [src/eval/builtins](../src/eval/builtins). Name completeness is checked separately from behavior: this is **not** a guarantee that every entry or edge case has been regression-tested. For detailed tutorials, see [TUTORIAL.md](./TUTORIAL.md), particularly the "Collections" and "Builtin Functions" sections.

`now`, `timestamp`, `timezone`, `env_vars`, `hostname`, `whoami`, and `uuid` are values when referenced, not functions to call with `()`. Their time/environment/random results can vary between evaluations; bind a value with `let` when it must be reused. `Number` includes integers and floating-point numbers. Example expressions are independent unless joined by `let ... in`.

Focused checks for the corrections below are in [test_builtin_reference.sh](../testing/integration/test_builtin_reference.sh). Run it with `bash testing/integration/test_builtin_reference.sh`; it requires Bash, standard Unix tools, `jq`, Perl, and `rustc`. Behavior checks use the existing release binary; a temporary standalone Rust probe checks the edited CLI help without rebuilding Avon. The script checks registry/reference name completeness, uses temporary fixtures with no network, and is not wired into the test runner. Dictionary/JSON object key order in displayed examples may vary.

**Quick Navigation:**
- [Aggregate Functions](#aggregate-functions) — sum, max, min, all, any, count
- [Color Functions](#color-functions) — hex_to_rgb, rgb_to_hex, lighten, darken, palette_*, contrast_ratio
- [Date/Time Functions](#datetime-functions) — date operations and timestamps
- [Debug Functions](#debug-functions) — trace, debug, assert
- [Dictionary Functions](#dictionary-functions) — get, set, merge, keys, values
- [Encoding & Hashing Functions](#encoding--hashing-functions) — hash_sha256, hash_md5
- [Environment Functions](#environment-functions) — env_var, os
- [File I/O Functions](#file-io-functions) — readfile, glob, import, json_parse, xml_parse, html_parse, opml_parse, ini_parse, *_parse_string
- [Formatting Functions](#formatting-functions) — format_json, format_csv, format_xml, format_html, format_opml, format_ini, format_table
- [HTML Functions](#html-functions) — html_escape, html_tag, html_attr
- [List Functions](#list-functions) — map, filter, fold, sort, unique
- [Markdown Functions](#markdown-functions) — md_heading, md_link, md_list
- [Math Functions](#math-functions) — abs, pow, sqrt, ceil, floor
- [Regex Functions](#regex-functions) — regex_match, regex_replace, scan
- [Ricing Functions](#ricing-functions) — format_filesize, format_uptime, chmod_symbolic, progressbar, gauge, shebang
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

## Color Functions

Color manipulation functions for theme generation, terminal customization, and visual consistency across applications. These functions enable declarative color scheme management.

| Function | Signature | Description |
|----------|-----------|-------------|
| `hex_to_rgb` | `String -> Dict` | Parses hex color (e.g., "#FF5733") and returns dict with r, g, b keys (0-255). |
| `rgb_to_hex` | `Int -> Int -> Int -> String` | Converts RGB values (0-255) to hex color string. |
| `hex_to_hsl` | `String -> Dict` | Converts hex color to HSL dict with h (0-360), s (0-100), l (0-100). |
| `hsl_to_hex` | `Number -> Number -> Number -> String` | Converts HSL values to hex color string; accepts fractional components. |
| `lighten` | `String -> Number -> String` | Lightens a color by percentage (0-100). Returns hex string. |
| `darken` | `String -> Number -> String` | Darkens a color by percentage (0-100). Returns hex string. |
| `saturate` | `String -> Number -> String` | Adjusts saturation (+/- 0-100). Positive increases, negative decreases. |
| `complementary` | `String -> String` | Returns complementary color hex string (opposite on color wheel). |
| `palette_monochromatic` | `String -> Int -> [String]` | Hex color, then non-negative count. For positive count n, uses base hue/saturation at lightness 100/n, 200/n, ..., 100 percent: darker to lighter, ending in white. Base color is not necessarily included; count 0 returns `[]`. |
| `palette_analogous` | `String -> [String]` | Base color first, then +30 and -30 degree hue variants. See the verified red wraparound limitation below. |
| `palette_triadic` | `String -> [String]` | Base color first, then +120 and +240 degree hue variants, in that order. |
| `invert_color` | `String -> String` | Inverts a color (RGB inversion). |
| `grayscale` | `String -> String` | Converts color to grayscale. |
| `blend_colors` | `String -> String -> Number -> String` | Blends two colors. Third param: 0=first color, 1=second color. |
| `contrast_ratio` | `String -> String -> Number` | Calculates WCAG contrast ratio between two hex colors. Returns number. |

**Examples:**
```avon
# Parse and convert colors
hex_to_rgb "#FF5733"                   # {r: 255, g: 87, b: 51}
rgb_to_hex 255 87 51                   # "#ff5733"

# Convert between color spaces
let hsl = hex_to_hsl "#88c0d0" in
hsl_to_hex hsl.h hsl.s hsl.l            # "#88c0d0"
hsl_to_hex 190 45 59                    # "#67b6c5"

# Adjust colors
lighten "#88c0d0" 20                    # Lighter variant
darken "#88c0d0" 20                     # Darker variant
saturate "#88c0d0" 30                   # More vibrant
saturate "#88c0d0" (-20)                # More muted

# Generate color palettes
palette_monochromatic "#88c0d0" 5      # [#1d3f49, #3a7f92, #6db2c5, #b6d8e2, #ffffff]
palette_analogous "#88c0d0"            # [#88c0d0, #889cd0, #88d0bc]
palette_triadic "#88c0d0"              # [#88c0d0, #d088c0, #c0d088]
palette_monochromatic "#88c0d0" 1      # [#ffffff], not the base color
palette_monochromatic "#88c0d0" 0      # []
palette_analogous "#FF0000"            # [#ff0000, #ff8000, #ff0000]
# Current negative-hue wraparound limitation: do not assume three distinct colors near red.

# Color transformations
invert_color "#88c0d0"                  # Rough complement
grayscale "#88c0d0"                    # Desaturated version
blend_colors "#FF0000" "#0000FF" 0.5  # Mix red and blue 50/50
complementary "#88c0d0"                # Returns hex complement

# Accessibility checks
contrast_ratio "#ffffff" "#000000"     # 21 (max contrast)
```

See the [Linux Ricing and System Customization](./TUTORIAL.md#linux-ricing-and-system-customization) section for practical applications.

---

## Date/Time Functions

Functions for working with dates and times. Dates are represented as ISO 8601 strings (e.g., "2024-12-10T15:30:00Z").

| Function | Signature | Description |
|----------|-----------|-------------|
| `date_add` | `String -> String -> String` | Adds a duration (e.g., "1d", "2h") to an ISO 8601 date string. |
| `date_diff` | `String -> String -> Number` | First ISO 8601 date minus second date, in seconds. |
| `date_format` | `String -> String -> String` | Formats an ISO 8601 date string using a format string (e.g., "%Y-%m-%d"). |
| `date_parse` | `String -> String -> String` | Parses a date string with a given format and returns an ISO 8601 string. |
| `now` | `String` (value) | Current local date/time in RFC 3339 format, including an offset. |
| `timestamp` | `Number` (integer value) | Current Unix timestamp in seconds. |
| `timezone` | `String` (value) | Current local timezone offset (e.g., "+00:00"). |

**Examples:**
```avon
typeof now                             # "String" (the date/time itself varies)
is_int timestamp                       # true (the timestamp itself varies)
typeof timezone                        # "String" (the offset depends on the system)
date_add "2024-12-10T15:30:00Z" "1d"   # "2024-12-11T15:30:00+00:00"
date_format "2024-12-10T15:30:00Z" "%Y-%m-%d"  # "2024-12-10"
date_diff "2024-12-10T15:30:00Z" "2024-12-11T15:30:00Z"  # -86400
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

## Dictionary Functions

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
[get config "host", get config "timeout",
 default 30 (get config "timeout"),
 has_key config "port", has_key config "timeout"]
# [localhost, None, 30, true, false]
```

## Environment Functions

Functions for accessing the system environment.

| Function | Signature | Description |
|----------|-----------|-------------|
| `env_var` | `String -> String` | Returns the value of an environment variable. Errors if not set. |
| `env_var_or` | `String -> String -> String` | Returns the value of an environment variable, or a default if not set. |
| `env_vars` | `Dict` (value) | Dictionary of environment variables when referenced. |
| `hostname` | `String` (value) | Reads `HOSTNAME`, falling back to `COMPUTERNAME` when unset; errors if neither is set. Does not query the system hostname. Empty values are returned unchanged. |
| `os` | *Constant* | A constant string representing the operating system (e.g., "linux", "macos"). This is not a function but a value. |
| `whoami` | `String` (value) | Reads `USER`, falling back to `LOGNAME` when unset; errors if neither is set. Does not look up the effective user ID. Empty values are returned unchanged. |

Use `hostname` and `whoami` without arguments. Their values depend on the process environment, not a guaranteed OS identity lookup.

## File I/O Functions

Functions for file system operations.

| Function | Signature | Description |
|----------|-----------|-------------|
| `abspath` | `String\|Path -> String` | Returns the absolute path. |
| `basename` | `String\|Path -> String` | Returns the filename portion of a path. |
| `csv_parse` | `String -> [Dict]` | Parses a CSV file using the first record as headers; field values are strings. |
| `csv_parse_string` | `String -> [Dict]` | Parses CSV text using the first record as headers; field values are strings. |
| `dirname` | `String\|Path -> String` | Returns the directory portion of a path. |
| `exists` | `String\|Path -> Bool` | Returns true if the file or directory exists. |
| `file_is_file` | `String\|Path -> Bool` | Tests for a regular file; false for missing paths. |
| `file_is_dir` | `String\|Path -> Bool` | Tests for a directory; false for missing paths. |
| `file_mtime` | `String\|Path -> Number` | Modification time as integer Unix seconds; errors if metadata cannot be read. |
| `file_size` | `String\|Path -> Number` | Metadata size in bytes as an integer; errors if metadata cannot be read. |
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
| `lines_grep` | `String\|Path -> String -> [String]` | File path first, regex second; returns matching lines without line endings. Errors on unreadable files or invalid regexes. |
| `opml_parse` | `String -> Dict` | Parses an OPML file into a Dict with version, head, and outlines. |
| `opml_parse_string` | `String -> Dict` | Parses a raw OPML string into a Dict with version, head, and outlines. |
| `publish` | `String\|Path -> String\|Template\|Path -> FileTemplate` | Output path first, content second. Constructs a file template; does not itself write a file. |
| `readfile` | `String\|Path -> String` | Reads the entire content of a file. |
| `readlines` | `String\|Path -> [String]` | Reads a file line by line into a list. |
| `relpath` | `String\|Path -> String\|Path -> String` | **Base first, target second**; returns the relative path from base to target. |
| `toml_parse` | `String -> Dict\|List\|a` | Parses a TOML file into an Avon value (Dict for tables, List for arrays). |
| `toml_parse_string` | `String -> Dict\|List\|a` | Parses a raw TOML string into an Avon value. |
| `walkdir` | `String\|Path -> [String]` | Recursively lists all files in a directory. |
| `xml_parse` | `String -> Dict` | Parses an XML file into a nested Dict with tag, attrs, children, and text. |
| `xml_parse_string` | `String -> Dict` | Parses a raw XML string into a nested Dict. |
| `yaml_parse` | `String -> Dict\|List\|a` | Parses a YAML file into an Avon value (Dict for mappings, List for sequences). |
| `yaml_parse_string` | `String -> Dict\|List\|a` | Parses a raw YAML string into an Avon value. |

**Examples:**
```avon
relpath "/a/b" "/a/b/c"                # "c"
relpath "/a/b/c" "/a/b"                # ".."
publish "out.txt" "hello"              # FileTemplate; run/eval previews, deploy writes
```

## Formatting Functions

Functions for formatting values.

| Function | Signature | Description |
|----------|-----------|-------------|
| `center` | `String -> Number -> String` | Centers a string within a given width. |
| `format_avon` | `a -> String` | Formats a value as Avon-style text; not a general lossless serializer for every value type. |
| `format_binary` | `Number -> String` | Formats a number as binary. |
| `format_bool` | `Bool -> String -> String` | Case-insensitive style: yesno/yes/no, onoff/on/off, truefalse/true/false, 10/1/0, enabled/enabled/disabled, active/active/inactive. Also accepts a custom "true-text/false-text" pair, lowercasing it; unknown styles fall back to "true"/"false". |
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
| `format_toml` | `Dict -> String` | Serializes a top-level dictionary to TOML; top-level lists/scalars are unsupported. `None` inside a dictionary becomes the string `"null"`, not a null value. |
| `format_xml` | `Dict -> String` | Formats a Dict (with tag/attrs/children/text) as an indented XML string. |
| `format_yaml` | `a -> String` | Serializes a value to a YAML string. |
| `truncate` | `String -> Number -> String` | Truncates a string to a maximum length, adding "...". |

```avon
format_avon [1, true, "hello"]          # '[1, true, "hello"]' (a String)
format_bool true "YES/NO"               # "Yes"
format_bool false "onoff"               # "Off"
format_bool true "UP/DOWN"              # "up", not "UP"
format_bool false "UP/DOWN"             # "down"
format_bool true "unknown"              # "true"
```

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
| `pfold` | `(a -> a -> a) -> a -> [a] -> a` | Parallel reduction. Requires an associative combiner and a true identity; items, partial results, and identity must share a compatible type. |
| `pmap` | `(a -> b) -> [a] -> [b]` | Parallel map. Like `map` but uses multiple CPU cores. |
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
# pfold needs an associative combiner AND identity (0 for addition, 1 for multiplication).
# The identity is used for each chunk and again when combining partial results.
# Unlike fold, the same function must also accept partial results as its second argument.
# Subtraction or a non-identity seed is not a valid parallel reduction.

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
| `random_range` | `Number -> Number -> Int` | Random integer between minimum and maximum, inclusive. Finite fractional bounds are truncated toward zero before checking min <= max; reversed bounds error. Checks cover modest bounds, not extreme ranges. |
| `round` | `Number -> Number` | Rounds to the nearest integer. |
| `sqrt` | `Number -> Number` | Returns the square root. Returns `NaN` for negative numbers. |
| `uuid` | `String` (value) | Random UUID v4 generated when referenced; bind it with `let` to reuse one ID. |

```avon
random_range 2 2                       # 2
random_range (-2.9) (-2.1)             # -2 (bounds truncated toward zero)
random_range 5 2                       # Error: min must be <= max
```

## Regex Functions

Functions for regular expressions.

| Function | Signature | Description |
|----------|-----------|-------------|
| `regex_match` | `String -> String -> Bool` | Returns true if the text matches the regex pattern. |
| `regex_replace` | `String -> String -> String -> String` | Replaces all matches of the regex pattern with the replacement string. |
| `regex_split` | `String -> String -> [String]` | Splits the text by the regex pattern. |
| `scan` | `String -> String -> [String\|[String]]` | Returns a list of all matches (or capture groups) in the text. |

## Ricing Functions

Utilities for Linux system customization, status displays, file formatting, and script generation. These functions support theme generation, dotfile creation, and system monitoring displays.

| Function | Signature | Description |
|----------|-----------|-------------|
| `format_filesize` | `Number -> String` | Converts bytes to human-readable size (B, KiB, MiB, GiB, TiB, PiB). |
| `format_temp` | `Number -> String` | Formats temperature in Celsius (e.g., "62°C"). |
| `format_uptime` | `Number -> String -> String` | Formats uptime seconds to readable format. Second arg: "short", "medium", "long", or "hms". |
| `progressbar` | `Number -> Number -> String` | Unicode bar: fraction (0–1), then positive width. Fraction is clamped; filled cells are rounded. |
| `gauge` | `Number -> Number -> String` | Fraction (0–1), then a numeric width placeholder (currently ignored). Clamps the fraction and displays one Unicode symbol plus a rounded percentage. |
| `chmod_numeric` | `String -> Number` | Converts symbolic chmod (e.g., "rwxr-xr-x") to numeric (755). |
| `chmod_symbolic` | `Number -> String` | Converts numeric chmod (755) to symbolic ("rwxr-xr-x"). |
| `shebang` | `String -> String` | Generates shebang line for scripts. Args: interpreter name/path. |

**Examples:**
```avon
# File and data formatting
format_filesize 1048576                 # "1.0 MiB"
format_filesize 1024                    # "1.0 KiB"
format_temp 62                          # "62°C"

# Uptime formatting
format_uptime 345600 "short"            # "4d 0h"
format_uptime 345600 "medium"           # "4d 0h 0m"
format_uptime 345600 "long"             # "4d 0h 0m 0s"
format_uptime 3661 "hms"                # "01:01:01"

# Progress and gauge displays
progressbar 0.5 10                      # "█████░░░░░"
progressbar 0.7 10                      # "███████░░░"
gauge 0.5 10                            # "◐ 50%"
gauge 0.75 10                           # "◕ 75%"

# File permissions
chmod_symbolic 755                      # "rwxr-xr-x"
chmod_symbolic 644                      # "rw-r--r--"
chmod_symbolic 600                      # "rw-------"
chmod_numeric "rwxr-xr-x"               # 755
chmod_numeric "rw-r--r--"               # 644

# Script generation
shebang "bash"                          # "#!/usr/bin/env bash"
shebang "python3"                       # "#!/usr/bin/env python3"
shebang "/usr/bin/zsh"                  # "#!/usr/bin/zsh"
```

**Note:** For percentage formatting, use `format_percent` from the [Formatting Functions](#formatting-functions) section with precision control: `format_percent 0.75 2` → `"75.00%"`.

See the [Linux Ricing and System Customization](./TUTORIAL.md#linux-ricing-and-system-customization) section for complete ricing workflows and examples.

---

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
| `base64_decode` | `String -> String` | Decodes Base64 text; errors on invalid encoding or non-UTF-8 decoded bytes. |
| `hex_encode` | `String -> String` | Encodes a string as hexadecimal (2 hex digits per byte). |
| `hex_decode` | `String -> String` | Decodes hexadecimal text; errors on invalid encoding or non-UTF-8 decoded bytes. |
| `sha256` | `String -> String` | Returns the SHA-256 hash of a string as a hex string. |
| `sha512` | `String -> String` | Returns the SHA-512 hash of a string as a hex string. |

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
hex_encode "hello"                          # "68656c6c6f"
hex_decode "68656c6c6f"                     # "hello"
sha256 "hello"                              # "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
sha512 "hello"                              # "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
```

## Encoding & Hashing Functions

The encoding functions and `sha256`/`sha512` are listed in [String Functions](#string-functions). Additional hash names:

| Function | Signature | Description |
|----------|-----------|-------------|
| `hash_sha256` | `String -> String` | Alternative name for SHA-256 hashing, with the same digest as `sha256`. |
| `hash_md5` | `String -> String` | MD5 digest as lowercase hexadecimal. MD5 is not suitable for security-sensitive integrity checks. |

```avon
(hash_sha256 "hello") == (sha256 "hello")  # true
hash_md5 "hello"                       # "5d41402abc4b2a76b9719d911017c592"
```

There is no `hash_sha512` alias; use `sha512`.

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
| `to_bool` | `Bool\|Number\|String\|List\|None -> Bool` | Booleans unchanged; zero numbers, empty lists, and None become false; non-zero numbers and non-empty lists become true. Case-insensitive strings: true/yes/1/on become true; false/no/0/off and empty string become false. No whitespace trimming. Other strings and unsupported types (including Dict) error. |
| `to_char` | `Number -> String` | Converts a Unicode codepoint to a character. |
| `to_float` | `a -> Number` | Converts the value to a float. |
| `to_int` | `a -> Number` | Converts the value to an integer. |
| `to_list` | `String -> [String]` | Converts a string to a list of characters. |
| `to_string` | `a -> String` | Converts the value to a string. |
| `typeof` | `a -> String` | Returns the type name of the value. |

There are no `bool` or `string` builtin aliases in the current release: both produce `unknown symbol`. Use `to_bool` and `to_string`. These names must not be inferred from type labels or error-message strings when extracting the builtin inventory.

```avon
to_bool "YES"                          # true
to_bool "false"                        # false, despite being a non-empty string
to_bool [1]                            # true
to_bool "arbitrary"                    # Error: cannot convert to bool
to_bool {}                             # Error: unsupported type
to_string 42                           # "42"
```

## Data Format Conversion

Avon has paired parsers and formatters for the **8 structured formats** below. They share Avon value types, but **not a universal schema or lossless round trip**. Choose transformations to match the destination formatter's expected structure.

### Shared Values, Different Schemas

- JSON and YAML represent mappings, lists, and scalar values; null becomes Avon's `None` value. TOML uses tables and has no null value.
- CSV uses the first record as headers and returns a list of dictionaries with **string** fields, not inferred numbers or booleans. There is no headerless-mode argument.
- XML and HTML use element dictionaries with `tag` and optional `attrs`, `text`, or `children`. Children can include strings as well as element dictionaries. HTML parsing can insert `html`, `head`, and `body` elements.
- OPML uses `version`, optional `head`, and `outlines`; outline attributes are direct dictionary keys, with optional nested `children`.
- INI uses dictionaries of sections, with unsectioned keys under `global`; parsed values are strings.

Use collection functions on matching value types, and inspect the schema before transforming it. A generic configuration dictionary is not automatically an XML element or an OPML outline.

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

Each file parser above takes a filename string. Its `*_parse_string` counterpart takes the file contents instead.

### Parse → Format Limitations

Formatting is a new serialization, not source preservation: comments and layout can be lost. XML/HTML parsing drops comments and whitespace-only text nodes. Converting to another format can also change types or omit data:

- `format_toml` rejects a top-level list. A JSON null inside a dictionary becomes the **string** `"null"` in TOML, not a null value.
- `format_ini {port: 8080}` returns an empty string: non-dictionary section values are skipped. Use `{global: {port: 8080}}` or a named section instead. Parsing that output returns `"8080"` as a string.
- `format_xml {port: 8080}` and `format_html {port: 8080}` return display text `"{port: 8080}"`, not element markup. `format_opml {port: 8080}` produces a document with no outlines, losing `port`.
- TOML date/time values become strings when parsed; formatting them back does not preserve their original TOML date/time type.

```avon
json_parse_string "{\"port\":8080}" -> format_json  # '{"port": 8080}'
(toml_parse_string (format_toml (json_parse_string "{\"missing\":null}"))).missing  # "null" (String)
format_ini {port: 8080}                # "" (wrong section shape)
get (ini_parse_string (format_ini {global: {port: 8080}})).global "port"  # "8080"
```

### Cross-Format Conversion with Explicit Shapes

These examples are self-contained; file input can use the corresponding file parser. Output is only as meaningful as the destination schema and supported types allow.

```avon
# JSON config to YAML, after changing a field
let config = json_parse_string "{\"port\":8080}" in
format_yaml (set config "port" 9090)    # "port: 9090\n"

# CSV field values stay strings; convert age explicitly before producing JSON
let users = csv_parse_string "name,age\nAda,42\n" in
format_json (map (\u {name: u.name, age: to_int u.age}) users)
# '[{"age": 42, "name": "Ada"}]'

# JSON to INI requires a dictionary of sections
json_parse_string "{\"app\":{\"port\":8080}}" -> format_ini
# "[app]\nport=8080\n"

# Element-shaped XML data can be serialized as JSON
xml_parse_string "<person name=\"Ada\"/>" -> format_json
# '{"attrs": {"name": "Ada"}, "tag": "person"}'
```
