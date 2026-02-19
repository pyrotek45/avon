#!/usr/bin/env bash
# Round-trip / idempotent tests for all 8 data formats
# Verifies: parse → format → parse → format produces identical output
set -euo pipefail

AVON="${AVON:-./target/debug/avon}"
PASS=0
FAIL=0

assert_eq() {
    local name="$1" actual="$2" expected="$3"
    if [ "$actual" = "$expected" ]; then
        PASS=$((PASS + 1))
    else
        FAIL=$((FAIL + 1))
        echo "FAIL: $name"
        echo "  expected: $expected"
        echo "  actual:   $actual"
    fi
}

echo "=== Round-trip / Idempotent Tests ==="
echo ""

# ── JSON ──────────────────────────────────────────────
echo "── JSON round-trip ──"

JSON_NAME=$($AVON run 'let d = {name: "avon", version: "1.0"} in let rt = json_parse_string (format_json d) in get rt "name"')
assert_eq "JSON round-trip preserves name" "$JSON_NAME" "avon"

JSON_VER=$($AVON run 'let d = {name: "avon", version: "1.0"} in let rt = json_parse_string (format_json d) in get rt "version"')
assert_eq "JSON round-trip preserves version" "$JSON_VER" "1.0"

JSON_FEAT=$($AVON run 'let d = {features: ["templates", "tasks", "repl"]} in let rt = json_parse_string (format_json d) in length (get rt "features")')
assert_eq "JSON round-trip preserves list length" "$JSON_FEAT" "3"

JSON_DOUBLE=$($AVON run 'let d = {x: "hello"} in let a = json_parse_string (format_json d) in let b = json_parse_string (format_json a) in get b "x"')
assert_eq "JSON double round-trip preserves value" "$JSON_DOUBLE" "hello"

# ── YAML ──────────────────────────────────────────────
echo "── YAML round-trip ──"

YAML_PORT=$($AVON run 'let d = {host: "localhost", port: 8080, debug: true} in let rt = yaml_parse_string (format_yaml d) in get rt "port"')
assert_eq "YAML round-trip preserves port" "$YAML_PORT" "8080"

YAML_VAL=$($AVON run 'let d = {host: "localhost", port: 8080} in let rt = yaml_parse_string (format_yaml d) in get rt "host"')
assert_eq "YAML round-trip preserves host" "$YAML_VAL" "localhost"

# ── TOML ──────────────────────────────────────────────
echo "── TOML round-trip ──"

TOML_RT=$($AVON run 'let d = {title: "My App", version: "2.0"} in let a = format_toml d in let b = toml_parse_string a in let c = format_toml b in c == a')
assert_eq "TOML round-trip stable" "$TOML_RT" "true"

TOML_VAL=$($AVON run 'let d = {title: "Config", owner: "alice"} in let rt = toml_parse_string (format_toml d) in get rt "owner"')
assert_eq "TOML round-trip preserves owner" "$TOML_VAL" "alice"

# ── CSV ───────────────────────────────────────────────
echo "── CSV round-trip ──"

CSV_RT=$($AVON run 'let a = csv_parse_string "name,age,city\nalice,30,NYC\nbob,25,LA" in let b = format_csv a in let c = csv_parse_string b in length c')
assert_eq "CSV round-trip preserves row count" "$CSV_RT" "2"

CSV_NAME=$($AVON run 'let a = csv_parse_string "name,age,city\nalice,30,NYC\nbob,25,LA" in let b = format_csv a in let c = csv_parse_string b in get (nth 0 c) "name"')
assert_eq "CSV round-trip preserves first name" "$CSV_NAME" "alice"

CSV_CITY=$($AVON run 'let a = csv_parse_string "name,age,city\nalice,30,NYC\nbob,25,LA" in let b = format_csv a in let c = csv_parse_string b in get (nth 1 c) "city"')
assert_eq "CSV round-trip preserves second city" "$CSV_CITY" "LA"

# ── XML ───────────────────────────────────────────────
echo "── XML round-trip ──"

XML_TYPE=$($AVON run 'let x = xml_parse_string "<root><item key=\"a\">hello</item><item key=\"b\">world</item></root>" in let y = format_xml x in typeof (xml_parse_string y)')
assert_eq "XML round-trip type preserved" "$XML_TYPE" "Dict"

XML_TAG=$($AVON run 'let x = xml_parse_string "<root><item>hello</item></root>" in let y = format_xml x in let z = xml_parse_string y in get z "tag"')
assert_eq "XML round-trip preserves tag" "$XML_TAG" "root"

# ── INI ───────────────────────────────────────────────
echo "── INI round-trip ──"

INI_RT=$($AVON run 'let d = {database: {host: "localhost", port: "5432"}, app: {name: "myapp"}} in let a = format_ini d in let b = ini_parse_string a in let c = format_ini b in c == a')
assert_eq "INI round-trip stable" "$INI_RT" "true"

INI_VAL=$($AVON run 'let d = {server: {host: "0.0.0.0", port: "80"}} in let rt = ini_parse_string (format_ini d) in get (get rt "server") "host"')
assert_eq "INI round-trip preserves host" "$INI_VAL" "0.0.0.0"

# ── OPML ──────────────────────────────────────────────
echo "── OPML round-trip ──"

OPML_TYPE=$($AVON run 'let a = opml_parse_string "<?xml version=\"1.0\"?><opml version=\"2.0\"><head><title>Feeds</title></head><body><outline text=\"Tech\" type=\"rss\"><outline text=\"HN\" type=\"rss\" xmlUrl=\"https://hn.com/rss\"/></outline></body></opml>" in let b = format_opml a in typeof (opml_parse_string b)')
assert_eq "OPML round-trip type preserved" "$OPML_TYPE" "Dict"

OPML_VER=$($AVON run 'let a = opml_parse_string "<?xml version=\"1.0\"?><opml version=\"2.0\"><head><title>Feeds</title></head><body><outline text=\"Tech\" type=\"rss\"/></body></opml>" in let b = format_opml a in let c = opml_parse_string b in get c "version"')
assert_eq "OPML round-trip preserves version" "$OPML_VER" "2.0"

OPML_TITLE=$($AVON run 'let a = opml_parse_string "<?xml version=\"1.0\"?><opml version=\"2.0\"><head><title>Feeds</title></head><body></body></opml>" in let b = format_opml a in let c = opml_parse_string b in get (get c "head") "title"')
assert_eq "OPML round-trip preserves title" "$OPML_TITLE" "Feeds"

# ── HTML ──────────────────────────────────────────────
echo "── HTML round-trip ──"

HTML_TYPE=$($AVON run 'let h = html_parse_string "<div class=\"main\"><p>Hello</p><p>World</p></div>" in let f = format_html h in typeof (html_parse_string f)')
assert_eq "HTML round-trip type preserved" "$HTML_TYPE" "Dict"

HTML_TAG=$($AVON run 'let h = html_parse_string "<div><p>Hello</p></div>" in let f = format_html h in let h2 = html_parse_string f in get h2 "tag"')
assert_eq "HTML round-trip preserves wrapper" "$HTML_TAG" "html"

HTML_BODY=$($AVON run 'let h = html_parse_string "<div><p>Hello</p></div>" in let f = format_html h in let h2 = html_parse_string f in let body = nth 1 (get h2 "children") in let div = nth 0 (get body "children") in get div "tag"')
assert_eq "HTML round-trip preserves inner div" "$HTML_BODY" "div"

# ── Summary ───────────────────────────────────────────
echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
