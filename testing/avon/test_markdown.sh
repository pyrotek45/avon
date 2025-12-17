#!/bin/bash
# Markdown to HTML Conversion Tests
# Tests the markdown_to_html builtin function

set -e

# Source common utilities for AVON binary detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"
AVON_BIN="$AVON"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0

cd "$PROJECT_ROOT"

# Build if needed
if [ ! -f "$AVON_BIN" ]; then
    echo "Building avon binary..."
    cargo build 2>&1 | tail -1
fi

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║          Markdown to HTML Conversion Tests                 ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Helper function to test
run_test() {
    local name="$1"
    local code="$2"
    local pattern="$3"
    
    echo -n "  ✓ $name... "
    local temp="/tmp/test_md_$$.av"
    echo "$code" > "$temp"
    
    if timeout 5 "$AVON_BIN" eval "$temp" 2>/dev/null | grep -q "$pattern"; then
        echo -e "${GREEN}PASS${NC}"
        ((PASSED++)) || true
    else
        echo -e "${RED}FAIL${NC}"
        ((FAILED++)) || true
    fi
    rm -f "$temp"
}

echo -e "${BLUE}Basic Markdown Features:${NC}"
run_test "Heading h1" 'markdown_to_html "# Title"' "<h1>Title</h1>"
run_test "Heading h2" 'markdown_to_html "## Subtitle"' "<h2>Subtitle</h2>"
run_test "Paragraph" 'markdown_to_html "Hello world"' "<p>Hello world</p>"

echo -e "${BLUE}Text Formatting:${NC}"
run_test "Bold" 'markdown_to_html "**bold**"' "<strong>bold</strong>"
run_test "Italic" 'markdown_to_html "*italic*"' "<em>italic</em>"
run_test "Strikethrough" 'markdown_to_html "~~deleted~~"' "<del>deleted</del>"

echo -e "${BLUE}Code Blocks:${NC}"
run_test "Inline code" 'markdown_to_html "Use `code` here"' "<code>code</code>"
run_test "Code block" 'markdown_to_html "```\ncode\n```"' "<pre><code>"
run_test "Rust code block" 'markdown_to_html "```rust\nfn main() {}\n```"' "language-rust"

echo -e "${BLUE}Links:${NC}"
run_test "Link" 'markdown_to_html "[Text](https://example.com)"' 'href="https://example.com"'

echo -e "${BLUE}Lists:${NC}"
run_test "Unordered list" 'markdown_to_html "- Item 1\n- Item 2"' "<ul>"
run_test "List items" 'markdown_to_html "- A\n- B"' "<li>A</li>"
run_test "Ordered list" 'markdown_to_html "1. First\n2. Second"' "<ol>"

echo -e "${BLUE}Advanced Features:${NC}"
run_test "Blockquote" 'markdown_to_html "> Quote"' "<blockquote>"
run_test "Table" 'markdown_to_html "| A | B |\n|---|---|\n| 1 | 2 |"' "<table>"
run_test "Task list" 'markdown_to_html "- [x] Done"' "type=\"checkbox\""

echo -e "${BLUE}Security:${NC}"
run_test "HTML escaping" 'markdown_to_html "< > & chars"' "&lt;"
run_test "Script tag" 'markdown_to_html "<script>alert()</script>"' "<script>"

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                    Test Summary                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo -e "✓ Passed:  ${GREEN}$PASSED${NC}"
echo -e "✗ Failed:  ${RED}$FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All markdown tests passed!${NC}"
    exit 0
else
    echo -e "${RED}✗ Some tests failed${NC}"
    exit 1
fi
