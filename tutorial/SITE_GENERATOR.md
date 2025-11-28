# Static Site Generator Guide

This guide demonstrates how to use Avon as a static site generator, similar to tools like Jekyll, Hugo, or Rusto. Avon can generate HTML pages from markdown content using template placeholders like `<!-- expand-title -->` and `<!-- expand-body -->`. No npm install required. No 847 dependencies. Just Avon.

> Tip: Share Your Site Generators:** Use the `--git` flag to share your site generator templates! Put your `.av` file in GitHub, and others can deploy it with custom content. Example: `avon deploy --git user/repo/site_generator.av --root ./site -title "My Blog"`. This makes it easy to share and collaborate on site generator templates.

## Quick Start: Minimal Example

The simplest example shows the core pattern in just 26 lines:

```bash
avon deploy examples/site_generator_minimal.av --root ./site --force
```

**The Pattern:**
1. Define markdown content as a template
2. Create HTML template with comment placeholders
3. Convert markdown to HTML using `markdown_to_html`
4. Replace placeholders with `replace`
5. Generate the file

```avon
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

# Replace placeholders (templates auto-convert to strings in string functions)
let html = replace (replace html_template "<!-- expand-title -->" title) "<!-- expand-body -->" html_body in

@index.html {"{html}"}
```

**Key Features:**
- **Templates for content**: Both HTML and markdown use Avon's template syntax `{"..."}` which allows interpolation
- **Built-in markdown conversion**: The `markdown_to_html` function handles headings, bold, italic, inline code, paragraphs, and line breaks
- **Template auto-conversion**: When you use `replace` (or any string function), templates automatically convert to strings. No need to call `to_string` first!
- **Comment placeholders**: HTML comments act as placeholders that get replaced with actual content

## Simple Example

The `site_generator_simple.av` file shows a basic single-page example:

```bash
avon deploy examples/site_generator_simple.av --root ./site
```

This generates:
- `./site/index.html` - A single HTML page with markdown content converted to HTML

### How it works:

1. **Markdown content** is defined as a template string
2. **HTML template** contains placeholders like `<!-- expand-body -->`
3. **Markdown converter** (`markdown_to_html`) transforms markdown syntax to HTML
4. **Template replacement** fills in the placeholders using `replace`
5. **File generation** outputs the final HTML

## Advanced Example: Multiple Pages

The `site_generator_advanced.av` file shows a more complete example with multiple pages:

```bash
avon deploy examples/site_generator_advanced.av --root ./site
```

This generates:
- `./site/posts/getting-started.html`
- `./site/posts/static-sites.html`

### Features:

- **Multiple pages** from a list of posts
- **Template system** with multiple placeholders:
  - `<!-- expand-title -->` - Post title
  - `<!-- expand-author -->` - Author name
  - `<!-- expand-date -->` - Publication date
  - `<!-- expand-body -->` - Markdown content converted to HTML
- **Dynamic paths** using post slugs

### Example Structure:

```avon
let posts = [
    {
        title: "Getting Started",
        author: "Alice",
        date: "2024-01-01",
        slug: "getting-started",
        content: {"# Getting Started\nWelcome to my blog!"}
    },
    {
        title: "Static Sites",
        author: "Bob",
        date: "2024-01-15",
        slug: "static-sites",
        content: {"# Static Sites\nStatic sites are great!"}
    }
    # Add more posts here. Or don't. We're not your manager.
] in

let generate_post = \post
    let html_body = markdown_to_html post.content in
    let html = replace (replace (replace html_template "<!-- expand-title -->" post.title) "<!-- expand-body -->" html_body) "<!-- expand-date -->" post.date in
    @posts/{post.slug}.html {"{html}"}
in

map generate_post posts
```

## Index Page Example

The `site_generator_index.av` file generates a homepage listing all posts:

```bash
avon deploy examples/site_generator_index.av --root ./site
```

This generates:
- `./site/index.html` - Homepage with post listings

The index page can list all posts with links, dates, and excerpts.

## Complete Workflow

To generate a full site with multiple pages:

```bash
# Generate all posts
avon deploy examples/site_generator_advanced.av --root ./site

# Generate index page
avon deploy examples/site_generator_index.av --root ./site
```

Or combine them in one file that returns a list of FileTemplates:

```avon
[
    @index.html (generate_index config posts),
    ...(map (\post @posts/{post.slug}.html (generate_post config post)) posts)
]
```

## Extending the Examples

### Add More Markdown Features

You can enhance the markdown converter to support additional features:

- **Links**: `[text](url)` → `<a href="url">text</a>` (use `replace` and `split`)
- **Images**: `![alt](src)` → `<img src="src" alt="alt">`
- **Code blocks** with syntax highlighting (detect language from ` ```lang`)
- **Tables** (parse pipe-separated lines)
- **Blockquotes** (lines starting with `>`)
- **Inline code** (backticks) - already supported by `markdown_to_html`

### Add More Pages

Create templates for:
- About page
- Contact page
- Archive page
- Tag/category pages
- RSS feed page

### Use External Markdown Files

Instead of embedding markdown in the Avon file, you can read from files:

```avon
let post_content = readfile "posts/getting-started.md" in
let html_body = markdown_to_html post_content in
# ... process and generate HTML
```

### Add RSS Feed

Generate an RSS feed from your posts:

```avon
let rss_template = {"
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<rss version=\"2.0\">
    <channel>
        <title>{site_title}</title>
        <!-- expand-items -->
    </channel>
</rss>
"} in

let items = map (\post {"
    <item>
        <title>{post.title}</title>
        <link>{base_url}/posts/{post.slug}.html</link>
        <pubDate>{post.date}</pubDate>
    </item>
"}) posts in

let rss_content = replace rss_template "<!-- expand-items -->" (join items "\n") in
@rss.xml {"{rss_content}"}
```

## Real-World Example

Here's how you might structure a real blog:

```avon
# blog.av
let config = {
    title: "My Blog",
    author: "Alice",
    base_url: "https://example.com"
} in

# Read markdown files from directory
let post_files = filter (\f ends_with f ".md") (walkdir "posts") in

# Parse each markdown file
let parse_post = \file_path
    let content = readfile file_path in
    let lines = readlines file_path in
    # Extract frontmatter (first few lines with ---)
    # Process markdown content
    {
        title: "Post Title",  # Extract from frontmatter
        slug: basename file_path,
        date: "2024-01-01",
        content: content
    }
in

let posts = map parse_post post_files in

# Generate all pages
[
    @index.html (generate_index config posts),
    @about.html (generate_about config),
    ...(map (\post @posts/{post.slug}.html (generate_post config post)) posts)
]
```

This approach gives you:
- **Flexibility** - Use Avon's functions to process data
- **Control** - Full control over the generation process
- **Simplicity** - No complex configuration files
- **Power** - Leverage 80+ built-in functions
- **Template System** - Use HTML comments as placeholders like `<!-- expand-title -->`

## Comparison with Other Tools

| Feature | Avon | Jekyll | Hugo |
|---------|------|--------|------|
| Template System | ✅ | ✅ | ✅ |
| Markdown Support | ✅ (built-in `markdown_to_html`) | ✅ | ✅ |
| Variables | ✅ | ✅ | ✅ |
| Functions | ✅ | ❌ | ❌ |
| Multi-file Output | ✅ | ✅ | ✅ |
| No Dependencies | ✅ | ❌ (Ruby) | ❌ (Go) |
| Functional Programming | ✅ | ❌ | ❌ |
| Type Safety | ✅ | ❌ | ❌ |

## Advantages of Using Avon

1. **No runtime dependencies** - Just the Avon binary
2. **Functional programming** - Use functions, maps, filters, and folds
3. **Type safety** - Runtime type checking prevents errors
4. **Flexible** - Works with any text format, not just HTML
5. **Fast** - Single command generates everything
6. **Powerful** - 80+ built-in functions for string manipulation, list operations, and more
7. **Template auto-conversion** - Templates automatically convert to strings in string functions
8. **Job security** - Nobody else will understand your build system (just kidding, the docs are great)

## Examples in the Repository

- **`examples/site_generator_minimal.av`** - Minimal 26-line example showing core pattern
- **`examples/site_generator_poc.av`** - Proof of concept demonstrating basic functionality
- **`examples/site_generator_simple.av`** - Simple single-page example
- **`examples/site_generator_advanced.av`** - Advanced multi-page example with posts
- **`examples/site_generator_index.av`** - Index page generator

Try them out:

```bash
avon deploy examples/site_generator_minimal.av --root ./site --force
cat ./site/index.html
```
