# Static Site Generator Examples

These examples demonstrate how to use Avon as a static site generator, similar to tools like Jekyll, Hugo, or Rusto. Avon can generate HTML pages from markdown content using template placeholders like `<!-- expand-title -->` and `<!-- expand-body -->`.

## Simple Example

The `site_generator_simple.av` file shows a basic example:

```bash
avon deploy examples/site_generator_simple.av --root ./site
```

This generates:
- `./site/index.html` - A single HTML page with markdown content converted to HTML

### How it works:

1. **Markdown content** is defined as a string
2. **HTML template** contains placeholders like `<!-- expand-body -->`
3. **Markdown converter** transforms markdown syntax to HTML
4. **Template replacement** fills in the placeholders
5. **File generation** outputs the final HTML

## Advanced Example

The `site_generator_advanced.av` file shows a more complete example:

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

## Index Page Example

The `site_generator_index.av` file generates a homepage listing all posts:

```bash
avon deploy examples/site_generator_index.av --root ./site
```

This generates:
- `./site/index.html` - Homepage with post listings

## Complete Workflow

To generate a full site:

```bash
# Generate all posts
avon deploy examples/site_generator_advanced.av --root ./site

# Generate index page
avon deploy examples/site_generator_index.av --root ./site

# Or combine them in one file
```

## Extending the Examples

### Add More Markdown Features

You can enhance the markdown converter to support:
- Links: `[text](url)` → `<a href="url">text</a>` (use `replace` and `split`)
- Images: `![alt](src)` → `<img src="src" alt="alt">`
- Code blocks with syntax highlighting (detect language from ` ```lang`)
- Tables (parse pipe-separated lines)
- Blockquotes (lines starting with `>`)
- Inline code (backticks)

### Add More Pages

Create templates for:
- About page
- Contact page
- Archive page
- Tag/category pages

### Use External Markdown Files

Instead of embedding markdown in the Avon file, you can read from files:

```avon
let post_content = readfile "posts/getting-started.md" in
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
"}
```

## Comparison with Other Tools

| Feature | Avon | Jekyll | Hugo |
|---------|------|--------|------|
| Template System | ✅ | ✅ | ✅ |
| Markdown Support | ✅ (custom) | ✅ | ✅ |
| Variables | ✅ | ✅ | ✅ |
| Functions | ✅ | ❌ | ❌ |
| Multi-file Output | ✅ | ✅ | ✅ |
| No Dependencies | ✅ | ❌ (Ruby) | ❌ (Go) |

## Advantages of Using Avon

1. **No runtime dependencies** - Just the Avon binary
2. **Functional programming** - Use functions, maps, filters
3. **Type safety** - Runtime type checking
4. **Flexible** - Works with any text format
5. **Fast** - Single command generates everything

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
    let lines = readlines content in
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

