# Proof of Concept: Static Site Generator

This is a minimal, working example that demonstrates how Avon can be used as a static site generator, similar to tools like Jekyll, Hugo, or Rusto.

## The Example

See `site_generator_poc.av` for a complete, working example that:

1. **Defines markdown content** as a string
2. **Uses HTML comment placeholders** like `<!-- expand-body -->`
3. **Converts markdown to HTML** using Avon's powerful built-in `markdown_to_html` function
4. **Replaces placeholders** with the converted content
5. **Generates the final HTML file**

## Try It

```bash
# Generate the site
avon deploy examples/site_generator_poc.av --root ./output

# View the result
cat ./output/index.html
```

## How It Works

The example demonstrates the core concept:

```avon
# 1. Define your content
let markdown = "
# Welcome
This is **markdown** content.
" in

# 2. Create HTML template with placeholders
let template = "
<html>
<body>
    <!-- expand-body -->
</body>
</html>
" in

# 3. Convert markdown to HTML
let html_content = convert_markdown markdown in

# 4. Replace placeholder
let final_html = replace template "<!-- expand-body -->" html_content in

# 5. Generate file
@index.html {"{final_html}"}
```

## Key Features Demonstrated

- ✅ **Template placeholders** - Use HTML comments as placeholders
- ✅ **Markdown conversion** - Simple markdown to HTML transformation
- ✅ **String manipulation** - Use Avon's built-in functions (`replace`, `split`, `join`)
- ✅ **File generation** - Output HTML files with `@path {"content"}`

## Extending This

You can extend this proof of concept to:

- Read markdown from files using `readfile`
- Generate multiple pages using `map`
- Add more placeholders (`<!-- expand-title -->`, `<!-- expand-author -->`)
- Support more markdown features (links, images, code blocks)
- Generate navigation, RSS feeds, and more

See `site_generator_simple.av` and `site_generator_advanced.av` for more complete examples.

## Comparison

| Feature | Avon | Jekyll | Hugo |
|---------|------|--------|------|
| Template System | ✅ | ✅ | ✅ |
| Markdown Support | ✅ (custom) | ✅ | ✅ |
| Functions | ✅ | ❌ | ❌ |
| No Dependencies | ✅ | ❌ (Ruby) | ❌ (Go) |

Avon gives you the power of a full programming language while keeping the simplicity of a static site generator.

