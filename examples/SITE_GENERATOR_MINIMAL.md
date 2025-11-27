# Minimal Static Site Generator Example

This is a very concise example (26 lines) demonstrating how to use Avon as a static site generator. It shows the core pattern: HTML templates with comment placeholders, markdown content, and automatic conversion.

## The Problem

You want to:
1. Have an HTML file with comment placeholders like `<!-- expand-title -->` and `<!-- expand-body -->`
2. Have markdown content
3. Replace those placeholders with actual content

This is similar to static site generators like Jekyll, Hugo, or Rusto.

## The Solution

Avon solves this by:
- Using templates for both HTML and markdown (powerful and flexible)
- Using the built-in `markdown_to_html` function for conversion
- Using `replace` to substitute comment placeholders
- Templates automatically convert to strings in string functions (no `to_string` needed!)

## The Code

```avon
# Static Site Generator: HTML with comment placeholders + markdown
# Problem: HTML with <!-- expand-title --> and <!-- expand-body -->, markdown file, replace placeholders
# Solution: Use templates, convert markdown to HTML, replace comments

let title = "My Site" in
let markdown = {"# Hello World
This is **markdown** content.
"} in
let html_template = {"
<!DOCTYPE html>
<html>
<head><title><!-- expand-title --></title></head>
<body>
    <h1><!-- expand-title --></h1>
    <!-- expand-body -->
</body>
</html>
"} in

# Convert markdown to HTML using built-in function
let html_body = markdown_to_html markdown in

# Replace placeholders (templates auto-convert to strings in string functions like replace)
let html = replace (replace html_template "<!-- expand-title -->" title) "<!-- expand-body -->" html_body in

@index.html {"{html}"}
```

## Key Features

1. **Templates for content**: Both HTML and markdown use Avon's template syntax `{"..."}` which allows interpolation and preserves formatting.

2. **Built-in markdown conversion**: The `markdown_to_html` function handles:
   - Headings (`#` through `######`)
   - Bold (`**text**`)
   - Italic (`*text*`)
   - Inline code (`` `code` ``)
   - Paragraphs and line breaks

3. **Template auto-conversion**: When you use `replace` (or any string function), templates automatically convert to strings. No need to call `to_string` first!

4. **Comment placeholders**: The HTML template uses HTML comments as placeholders that get replaced with actual content.

## How to Run

```bash
# Deploy the example
avon deploy examples/site_generator_minimal.av --root /tmp/site --force

# Check the output
cat /tmp/site/index.html
```

## Expected Output

```html
<!DOCTYPE html>
<html>
<head><title>My Site</title></head>
<body>
    <h1>My Site</h1>
    <h1>Hello World</h1>
<p>This is <strong>markdown</strong> content.</p>
</body>
</html>
```

## Why This Works Well

- **Concise**: Only 26 lines for a complete static site generator
- **Readable**: Templates make the HTML and markdown easy to read and maintain
- **Powerful**: Template interpolation means you can use variables, functions, and expressions
- **Convenient**: No manual `to_string` calls needed—templates auto-convert in string functions

This demonstrates Avon's full capabilities as a static site generator. Avon provides everything you need to build complete websites—from simple blogs to complex multi-page sites—with a clean, powerful, and flexible approach that rivals tools like Jekyll or Hugo.
