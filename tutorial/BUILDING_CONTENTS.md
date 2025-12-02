# Building Up File Contents

Avon provides several ways to build up file contents incrementally. This guide shows you the different patterns, when to use each, and why. Pick the right tool for the job—you wouldn't use a hammer to make coffee.

## Quick Decision Guide

**Use templates (`{"..."}`) when:**
- Building multi-line content
- Merging multiple sections together
- You need interpolation with variables
- You want to avoid escaping issues

**Use strings (`"..."`) when:**
- Simple single-line values
- No interpolation needed
- Building from data (lists, maps)

**Use `+` operator when:**
- Merging strings or templates (preferred over `concat`)
- Combining multiple sections
- Building content incrementally

**Use `join` when:**
- Combining lists of strings
- Adding separators between items
- Building from data structures

## Method 1: Build Content in Memory (Recommended)

**Best for:** Building complete files from multiple parts in a single run.

**Why:** Most efficient, atomic (all-or-nothing), and easiest to reason about. All content is built before any file is written.

### Using Templates with `+` Operator (Preferred)

Templates solve many string issues (escaping, newlines, interpolation) and should be preferred when merging content:

```avon
# Build up a configuration file from multiple sections using templates
let header = {"# Configuration File
version = 1.0
"} in
let database_section = {"
[database]
host = localhost
port = 5432
"} in
let app_section = {"
[app]
name = myapp
debug = true
"} in
let footer = {"
# End of configuration
"} in

# Combine all sections using + operator (preferred over concat)
let full_config = header + database_section + app_section + footer in

@config.ini {"{full_config}"}
```

Why templates here:** Multi-line content, easy to read, no escaping needed, and the `+` operator works seamlessly with templates.

### Using a Single Template (Simplest)

For simpler cases, use one template with interpolation:

```avon
let app_name = "myapp" in
let full_config = {"# Configuration File
version = 1.0

[database]
host = localhost
port = 5432

[app]
name = {app_name}
debug = true

# End of configuration
"} in

@config.ini {"{full_config}"}
```

Why this approach:** Simplest when you don't need to build sections separately. All content in one place, easy to maintain.

### Using `join` for Lists

When building from data structures, `join` is cleaner:

```avon
let sections = [
  "# Configuration File",
  "version = 1.0",
  "",
  "[database]",
  "host = localhost",
  "port = 5432",
  "",
  "[app]",
  "name = myapp",
  "debug = true",
  "",
  "# End of configuration"
] in

let full_config = join sections "\n" in

@config.ini {"{full_config}"}
# Output (in config.ini):
# # Configuration File
# version = 1.0
#
# [database]
# host = localhost
# port = 5432
#
# [app]
# name = myapp
# debug = true
#
# # End of configuration
```

Why `join`:** Perfect for lists of strings. Cleaner than chaining `+` or `concat`. Use strings here since each line is simple.

### Using the Pipe Operator

For functional style transformations:

```avon
let sections = [
  "# Configuration File",
  "version = 1.0",
  "",
  "[database]",
  "host = localhost",
  "port = 5432",
  "",
  "[app]",
  "name = myapp",
  "debug = true"
] in

sections
  -> join "\n"
  -> (\content @config.ini {"{content}"})
```

Why pipe operator:** Makes data transformations readable and composable. Good for complex processing pipelines. Also makes you feel like a functional programming wizard, which is nice for the ego.

## Method 2: Using `--append` Flag (Multiple Runs)

**Best for:** Logs, accumulating data over time, or when you need to add to existing files.

**Why:** Sometimes you genuinely need to accumulate content over multiple runs (e.g., logs, reports that build up over time).

The `--append` flag lets you add content to existing files:

```avon
# First run: Create initial file
@log.txt {"
2024-01-01 10:00:00 - Application started
"}
```

```bash
avon deploy log.av --root ./logs
```

```avon
# Second run: Append to the file
@log.txt {"
2024-01-01 11:00:00 - User logged in
2024-01-01 11:05:00 - Data processed
"}
```

```bash
avon deploy log.av --root ./logs --append
```

**Result:** The file now contains both entries.

**Important Notes:**
- `--append` only works if the file already exists
- If the file doesn't exist, it creates it (same as normal write)
- Each deployment adds content to the end of the file
- Useful for logs, accumulating reports, or building files over multiple runs

When to use:** Only when you truly need to accumulate over multiple runs. For most cases, prefer building everything in memory (Method 1). Your disk I/O will thank you.

## Method 3: Read, Modify, Write

**Best for:** Extending existing files that were created outside Avon or in a previous run.

**Why:** Sometimes files exist that you didn't create, or you need to modify files from previous deployments without losing existing content.

```avon
# Step 1: Read existing file
let existing_content = readfile "base.txt" in

# Step 2: Add new content using template (preferred for multi-line)
let new_section = {"
# New Section
value = 42
"} in
let updated_content = existing_content + new_section in

# Step 3: Write the combined content
@base.txt {"{updated_content}"}
```

```bash
# Deploy with --force to overwrite
avon deploy update.av --root . --force
```

Why templates here:** When adding multi-line sections, templates are cleaner than string concatenation with `\n`.

## Method 4: Building from Lists with `map` and `join`

**Best for:** Generating files from data structures.

**Why:** When you have structured data (lists, dicts), use `map` to transform it and `join` to combine it. This is more maintainable than manual loops.

```avon
let users = [
  {name: "Alice", role: "admin", email: "alice@example.com"},
  {name: "Bob", role: "user", email: "bob@example.com"},
  {name: "Charlie", role: "user", email: "charlie@example.com"}
] in

let generate_user_line = \user {"{user.name} ({user.role}): {user.email}"} in
let user_lines = map generate_user_line users in
let header = {"# User List
"} in
let content = header + (join user_lines "\n") in

@users.txt {"{content}"}
```

Why this pattern:** 
- `map` transforms each item using a template (clean interpolation)
- `join` combines the list with separators
- Header uses template for consistency, then `+` to merge with joined lines

### Using `fold` (Advanced)

For more complex accumulation, `fold` can be useful:

```avon
let items = ["item1", "item2", "item3"] in
let build_list = fold (\acc \item acc + {"- {item}
"}) {""} items in

@list.txt {"{build_list}"}
```

Why `fold`:** When you need custom accumulation logic. However, `map` + `join` is usually simpler and preferred:

```avon
let items = ["item1", "item2", "item3"] in
let list_lines = map (\item {"- {item}"}) items in
let content = join list_lines "\n" in

@list.txt {"{content}"}
# Output (in list.txt):
# - item1
# - item2
# - item3
```

Why prefer `map` + `join`:** More readable, easier to understand, and follows functional programming best practices.

## Method 5: Conditional Content Building

**Best for:** Building files with optional sections based on conditions.

**Why:** Some configs have optional features. Building conditionally is cleaner than maintaining separate files. And unlike languages with 47 different ways to check for null, we just use `if then else`.

```avon
let include_debug = "true" in
let include_metrics = "true" in

let base_config = {"
[app]
name = myapp
"} in
let debug_section = if include_debug == "true" then {"
[debug]
level = verbose
"} else {""} in
let metrics_section = if include_metrics == "true" then {"
[metrics]
enabled = true
"} else {""} in

let full_config = base_config + debug_section + metrics_section in

@config.ini {"{full_config}"}
```

Why templates + `+`:** 
- Templates handle multi-line content cleanly
- `+` operator merges sections easily
- Empty template `{""}` works as a no-op when condition is false

## Real-World Examples

### Example 1: Building a Log File

```avon
let log_time = "2024-01-01 12:00:00" in
let events = [
  "Application started",
  "Database connected",
  "Cache initialized",
  "Server listening on port 8080"
] in

let log_lines = map (\event {"{log_time} - {event}"}) events in
let log_content = join log_lines "\n" in

@app.log {"{log_content}"}
# Output (in app.log):
# 2024-01-01 12:00:00 - Application started
# 2024-01-01 12:00:00 - Database connected
# 2024-01-01 12:00:00 - Cache initialized
# 2024-01-01 12:00:00 - Server listening on port 8080
```

Why this approach:** 
- Simple strings in the list (no interpolation needed per item)
- Template in `map` for interpolation (`{timestamp} - {event}`)
- `join` to combine lines

### Example 2: Building a Multi-Section Config

```avon
let app_name = "myapp" in
let env = "production" in

let header = {"# {app_name} Configuration
# Environment: {env}
"} in

let database_config = {"
[database]
host = db.example.com
port = 5432
name = {app_name}
"} in

let app_config = {"
[app]
name = {app_name}
environment = {env}
debug = {if env == "production" then "false" else "true"}
"} in

let full_config = header + database_config + app_config in

@config.ini {"{full_config}"}
```

Why templates + `+`:** 
- Each section is multi-line with interpolation
- Templates handle this cleanly
- `+` operator merges sections intuitively

### Example 3: Building HTML from Components

```avon
let title = "My Page" in
let items = ["Home", "About", "Contact"] in

let nav_items = map (\item {"    <li><a href=\"/{lower item}\">{item}</a></li>"}) items in
let nav = {"  <nav>
    <ul>
{join nav_items "\n"}
    </ul>
  </nav>
"} in

let header = {"<!DOCTYPE html>
<html>
<head>
  <title>{title}</title>
</head>
<body>
  <h1>{title}</h1>
"} in

let footer = {"  <footer>© 2024</footer>
</body>
</html>
"} in

let html = header + nav + footer in

@index.html {"{html}"}
```

Why this approach:**
- Templates for multi-line HTML sections
- Template interpolation in `nav` for the `join` result
- `+` operator chains sections together
- `map` with template for generating list items

## Comparison of Methods

| Method | Use Case | Single Run? | Why Use It |
|--------|----------|-------------|------------|
| **Build in Memory** | Complete files from parts | ✅ Yes | Most efficient, atomic, easiest to reason about |
| **`--append` Flag** | Adding to existing files | ❌ No (multiple runs) | Only when you truly need to accumulate over time |
| **Read + Modify** | Extending existing files | ✅ Yes | When files exist outside Avon or from previous runs |
| **List Operations** | Generating from data | ✅ Yes | When you have structured data (lists/dicts) |
| **Conditional Building** | Optional sections | ✅ Yes | When configs have optional features |

## Best Practices

1. **Prefer building in memory**: It's more efficient and atomic (all-or-nothing). All content is ready before any file is written.

2. **Use templates for multi-line content**: Templates solve many string issues (escaping, newlines, interpolation) and should be preferred when merging strings or building multi-line content.

3. **Use strings for simple values**: When you have simple single-line values or building from lists, strings are fine. No need to overcomplicate.

4. **Use `+` operator over `concat`**: The `+` operator works with strings, templates, and lists - it's cleaner and more intuitive. Use `concat` only when you need the function form.

5. **Use `join` for lists**: Cleaner than chaining `+` or `concat`. Perfect for combining lists of strings with separators.

6. **Use pipe operator for transformations**: Makes data transformations readable and composable. Good for complex processing pipelines.

7. **Use `--append` sparingly**: Only when you truly need to accumulate over multiple runs. Most cases should build everything in memory. Appending is like duct tape—useful in emergencies, but you probably shouldn't build your whole house with it.

8. **Read existing files when needed**: Use `readfile` to extend files created outside Avon, then merge with `+`.

<!-- TODO: Add section on handling legacy configs. Just kidding, we don't talk about legacy configs. They know what they did. -->

## When to Use Strings vs Templates

**Use strings (`"..."`) when:**
- Simple single-line values: `let name = "Alice" in`
- Building from lists: `let items = ["a", "b", "c"] in`
- No interpolation needed: `let separator = "\n" in`
- Simple data: `let port = "8080" in`

**Use templates (`{"..."}`) when:**
- Multi-line content: `let config = {"[app]\nname = test\n"} in`
- Merging multiple sections: `let full = header + section + footer in`
- Interpolation needed: `let msg = {"Hello {name}"} in`
- Complex formatting: `let html = {"<div>{content}</div>"} in`

**Key insight:** When merging strings, templates are often better because they handle newlines and escaping more naturally. But simple strings are perfectly fine for basic values.

## Related Documentation

- **[TUTORIAL.md](./TUTORIAL.md)** - Complete Avon tutorial
- **[STYLE_GUIDE.md](./STYLE_GUIDE.md)** - Code style and formatting
- **[BUILTIN_FUNCTIONS.md](./BUILTIN_FUNCTIONS.md)** - Complete reference of built-in functions
- **[examples/pattern_read_append.av](../examples/pattern_read_append.av)** - Example showing these patterns
