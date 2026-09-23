#!/usr/bin/env bash
# Focused binary/help checks plus registry/reference name coverage, not a full behavior audit.
# Run: bash testing/integration/test_builtin_reference.sh
# Does not rebuild Avon, modify the runner, or access the network. AVON can override the binary.
set -euo pipefail
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
AVON="$(realpath "${AVON:-$PROJECT_ROOT/target/release/avon}")"
[[ -x "$AVON" ]] || { echo "An existing release binary is required" >&2; exit 1; }
command -v jq >/dev/null || { echo "jq is required for order-independent JSON checks" >&2; exit 1; }
command -v perl >/dev/null || { echo "Perl is required for registry/reference inventory checks" >&2; exit 1; }
command -v rustc >/dev/null || { echo "rustc is required for the standalone source-help probe" >&2; exit 1; }
TEST_DIR="$(mktemp -d)"
trap 'rm -rf "$TEST_DIR"' EXIT
trap 'echo "FAIL at script line $LINENO" >&2' ERR
cd "$TEST_DIR"
export TZ=UTC RAYON_NUM_THREADS=2 AVON_REFERENCE_TEST_VALUE=fixture
checks=0
help_checks=0
RUN_ENV=()

# Extract only the actual NAMES array bodies, not type labels or diagnostic strings.
# Deduplicate names such as length; duplicate reference rows for overloads are allowed.
perl - "$PROJECT_ROOT" > registry.txt <<'PERL'
use strict;
use warnings;
my ($root) = @ARGV;
sub slurp {
    my ($path) = @_;
    open my $fh, '<', $path or die "Cannot read $path: $!\n";
    local $/;
    return <$fh>;
}
my %registry;
for my $path (glob "$root/src/eval/builtins/*.rs") {
    my $source = slurp($path);
    next unless $source =~ /pub\s+const\s+NAMES\s*:\s*&\[&str\]\s*=\s*&\[(.*?)\];/s;
    my $body = $1;
    $body =~ s{//[^\n]*|/\*.*?\*/}{}gs;
    $registry{$1} = 1 while $body =~ /"([a-z_][a-z_0-9]*)"/g;
}
die "No builtin NAMES arrays found\n" unless keys %registry;
my $reference = slurp("$root/tutorial/BUILTIN_FUNCTIONS.md");
my %documented = map { $_ => 1 } $reference =~ /^\| `([a-z_][a-z_0-9]*)` \|/mg;
my @missing = sort grep { !$documented{$_} } keys %registry;
my @extra = sort grep { !$registry{$_} } keys %documented;
die "Registry/reference mismatch: missing [@missing], unregistered [@extra]\n" if @missing || @extra;
printf STDERR "PASS: registry/reference coverage %d/%d unique names (name coverage, not behavior coverage)\n",
    scalar(keys %documented), scalar(keys %registry);
print "$_\n" for sort keys %registry;
PERL

# Compile only docs.rs from the edited source; the release executable stays untouched.
# This module has no external dependencies. Test both detailed and summary help.
AVON_DOC_PROJECT_ROOT="$PROJECT_ROOT" rustc --edition=2021 -A dead_code -o "$TEST_DIR/doc-probe" - <<'RUST'
mod docs {
    include!(concat!(env!("AVON_DOC_PROJECT_ROOT"), "/src/cli/docs.rs"));
}
fn main() {
    let name = std::env::args().nth(1).expect("builtin name or --all required");
    if name == "--all" {
        docs::print_builtin_docs();
    } else {
        println!("{}", docs::get_builtin_doc(&name).expect("missing builtin help"));
    }
}
RUST
documented_help=0
while IFS= read -r name; do
    "$TEST_DIR/doc-probe" "$name" > "$TEST_DIR/help-$name.txt"
    [[ -s "$TEST_DIR/help-$name.txt" ]]
    documented_help=$((documented_help + 1))
done < registry.txt
printf 'PASS: detailed CLI help coverage %d/%d registered names (presence only)\n' \
    "$documented_help" "$(wc -l < registry.txt)"
"$TEST_DIR/doc-probe" --all > "$TEST_DIR/help-summary.txt"

help_has() {
    local name="$1" fragment="$2"
    if ! grep -Fq -- "$fragment" "$TEST_DIR/help-$name.txt"; then
        printf 'FAIL: %s help missing: %s\n' "$name" "$fragment" >&2
        exit 1
    fi
    help_checks=$((help_checks + 1))
}

help_lacks() {
    local name="$1" fragment="$2"
    if grep -Fq -- "$fragment" "$TEST_DIR/help-$name.txt"; then
        printf 'FAIL: %s help still contains: %s\n' "$name" "$fragment" >&2
        exit 1
    fi
    help_checks=$((help_checks + 1))
}

for decoder in base64_decode hex_decode; do
    help_has "$decoder" "$decoder :: String -> String"
    help_has "$decoder" 'Errors on'
    help_has "$decoder" 'non-UTF-8'
    help_lacks "$decoder" 'String | None'
    help_lacks "$decoder" 'Returns None'
    summary_line=$(grep "$decoder" "$TEST_DIR/help-summary.txt")
    [[ "$summary_line" == *'errors on invalid'* && "$summary_line" != *None* ]]
    help_checks=$((help_checks + 1))
done
help_has pfold '(a -> a -> a) -> a -> [a] -> a'
help_has pfold 'associative combiner AND a true identity'
help_has pfold 'every chunk and again'
help_has pfold 'partial results as its second argument'
help_lacks pfold 'max a b'
help_has hsl_to_hex 'Number -> Number -> Number -> String'
help_has hsl_to_hex '#eb9947'
help_has palette_monochromatic 'darker to lighter'
help_has palette_monochromatic 'base color is not necessarily included'
help_has palette_monochromatic '[#003366, #0066cc, #3399ff, #99ccff, #ffffff]'
help_has palette_analogous '[#ff0000, #ff8000, #ff0000]'
help_has palette_analogous '[#0066cc, #0000cc, #00cccc]'
help_has palette_triadic '[#0066cc, #cc0066, #66cc00]'
help_has to_bool 'without trimming'
help_has to_bool 'including dictionaries'
help_lacks to_bool 'any text'
help_has format_bool 'pair is lowercased'
help_has format_bool 'Unknown styles fall back'
help_has format_avon 'Not a lossless serializer'
help_has format_toml 'None becomes the string'
help_has format_xml 'display text, not XML'
help_has format_html 'display text, not HTML'
help_has format_opml 'no outlines, losing port'
help_has format_ini 'Non-dictionary section values are skipped'
help_has hostname 'falling back to COMPUTERNAME'
help_has whoami 'falling back to LOGNAME'
help_has random_range 'truncated toward zero'

expect() {
    local expression="$1" expected="$2" actual
    actual=$(env "${RUN_ENV[@]}" "$AVON" run "$expression")
    if [[ "$actual" != "$expected" ]]; then
        printf 'FAIL: %s\nExpected: %s\nActual: %s\n' "$expression" "$expected" "$actual" >&2
        exit 1
    fi
    checks=$((checks + 1))
}

reject() {
    local expression="$1" fragment="$2" actual
    if actual=$(env "${RUN_ENV[@]}" "$AVON" run "$expression" 2>&1); then
        printf 'FAIL: unexpectedly accepted %s\n' "$expression" >&2
        exit 1
    fi
    if [[ "$actual" != *"$fragment"* ]]; then
        printf 'FAIL: %s\nMissing diagnostic: %s\nActual: %s\n' "$expression" "$fragment" "$actual" >&2
        exit 1
    fi
    checks=$((checks + 1))
}

# Do not compare dictionaries using Avon's display/order-dependent equality.
# These fixtures contain JSON-compatible values; jq compares keys structurally.
same_json() {
    local left="$1" right="$2" actual
    actual=$("$AVON" run "format_json [($left), ($right)]")
    if ! jq -e 'type == "array" and length == 2 and .[0] == .[1]' <<< "$actual" >/dev/null; then
        printf 'FAIL: %s differs from %s\n%s\n' "$left" "$right" "$actual" >&2
        exit 1
    fi
    checks=$((checks + 1))
}

# Values, not calls; inspect types/shapes rather than hard-coding time or UUIDs.
expect '[typeof now, typeof timestamp, typeof timezone, typeof env_vars, typeof uuid]' '[String, Number, String, Dict, String]'
expect 'typeof now' 'String'
expect 'is_int timestamp' 'true'
expect 'typeof timezone' 'String'
expect 'timezone' '+00:00'
expect 'regex_match "^[0-9]{4}-[0-9]{2}-[0-9]{2}T.*[+]00:00$" now' 'true'
expect 'timestamp > 0' 'true'
expect 'get env_vars "AVON_REFERENCE_TEST_VALUE"' 'fixture'
expect 'regex_match "^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$" uuid' 'true'
for value in now timestamp timezone env_vars uuid; do
    reject "$value \"ignored\"" 'expected function'
    expect "let saved = $value in saved == saved" 'true'
done
expect 'date_add "2024-12-10T15:30:00Z" "1d"' '2024-12-11T15:30:00+00:00'
expect 'date_format "2024-12-10T15:30:00Z" "%Y-%m-%d"' '2024-12-10'
expect 'date_diff "2024-12-10T15:30:00Z" "2024-12-11T15:30:00Z"' '-86400'

# Environment-backed values, not OS identity queries. Control all fallback inputs.
RUN_ENV=(HOSTNAME=primary-host COMPUTERNAME=fallback-host USER=primary-user LOGNAME=fallback-user)
expect '[hostname, whoami]' '[primary-host, primary-user]'
expect '[typeof hostname, typeof whoami]' '[String, String]'
for value in hostname whoami; do
    reject "$value \"ignored\"" 'expected function'
done
RUN_ENV=(-u HOSTNAME -u USER COMPUTERNAME=fallback-host LOGNAME=fallback-user)
expect '[hostname, whoami]' '[fallback-host, fallback-user]'
RUN_ENV=(HOSTNAME= COMPUTERNAME=fallback-host USER= LOGNAME=fallback-user)
expect '[length hostname, length whoami]' '[0, 0]'
RUN_ENV=(-u HOSTNAME -u COMPUTERNAME -u USER -u LOGNAME)
reject 'hostname' 'HOSTNAME environment variable not set'
reject 'whoami' 'could not determine current user'
RUN_ENV=()

# The apparent bool/string inventory entries were type/error labels, not builtins.
reject 'bool "false"' 'unknown symbol: bool'
reject 'string 42' 'unknown symbol: string'
expect 'to_string 42' '42'
expect 'typeof (to_string 42)' 'String'
expect 'map to_bool [true, false, 0, 0.0, 1, -1, 0.5, [], [0], (head [])]' '[true, false, false, false, true, true, true, false, true, false]'
expect 'map to_bool ["true", "TRUE", "yes", "YES", "1", "on", "ON"]' '[true, true, true, true, true, true, true]'
expect 'map to_bool ["false", "FALSE", "no", "NO", "0", "off", "OFF", ""]' '[false, false, false, false, false, false, false, false]'
expect 'to_bool "YES"' 'true'
expect 'to_bool "false"' 'false'
expect 'to_bool [1]' 'true'
reject 'to_bool "arbitrary"' 'cannot convert'
reject 'to_bool " true "' 'cannot convert'
reject 'to_bool {}' 'type mismatch'
reject 'to_bool {a: 1}' 'type mismatch'
reject 'to_bool @file.txt' 'type mismatch'
reject 'to_bool {"true"}' 'type mismatch'
reject 'to_bool upper' 'type mismatch'

# Do not assert particular random draws, or assume endpoint sampling in finite trials.
expect 'random_range 2 2' '2'
expect 'random_range (-2.9) (-2.1)' '-2'
expect 'random_range 2.9 2.1' '2'
expect 'is_function (random_range 2)' 'true'
expect 'let draws = map (\i random_range (-3) 4) (range 1 128) in all (\n (is_int n) && (n >= (-3)) && (n <= 4)) draws' 'true'
reject 'random_range 5 2' 'min must be <= max'
reject 'random_range "2" 3' 'type mismatch'
reject 'random_range 2 "3"' 'type mismatch'

# HSL accepts fractional components; every palette output is checked verbatim.
expect 'let hsl = hex_to_hsl "#88c0d0" in hsl_to_hex hsl.h hsl.s hsl.l' '#88c0d0'
expect 'hsl_to_hex 190 45 59' '#67b6c5'
expect 'hsl_to_hex 30 80 60' '#eb9947'
expect 'palette_monochromatic "#88c0d0" 5' '[#1d3f49, #3a7f92, #6db2c5, #b6d8e2, #ffffff]'
expect 'palette_monochromatic "#0066CC" 5' '[#003366, #0066cc, #3399ff, #99ccff, #ffffff]'
expect 'palette_monochromatic "#88c0d0" 1' '[#ffffff]'
expect 'palette_monochromatic "#88c0d0" 0' '[]'
expect 'palette_analogous "#88c0d0"' '[#88c0d0, #889cd0, #88d0bc]'
expect 'palette_analogous "#0066CC"' '[#0066cc, #0000cc, #00cccc]'
expect 'palette_analogous "#FF0000"' '[#ff0000, #ff8000, #ff0000]'
expect 'palette_triadic "#88c0d0"' '[#88c0d0, #d088c0, #c0d088]'
expect 'palette_triadic "#0066CC"' '[#0066cc, #cc0066, #66cc00]'
expect 'let config = {host: "localhost", port: 8080} in [get config "host", get config "timeout", default 30 (get config "timeout"), has_key config "port", has_key config "timeout"]' '[localhost, None, 30, true, false]'

# All filesystem effects are confined to this temporary working directory.
mkdir folder
printf 'alpha\nbeta\nalphabet\n' > fixture.txt
touch -t 202401020304.05 fixture.txt
for path in '"fixture.txt"' '@fixture.txt'; do
    expect "file_is_file $path" 'true'
    expect "file_is_dir $path" 'false'
    expect "file_size $path" '20'
    expect "file_mtime $path" '1704164645'
    expect "is_int (file_size $path)" 'true'
    expect "is_int (file_mtime $path)" 'true'
    expect "lines_grep $path \"^alpha\"" '[alpha, alphabet]'
done
expect '[file_is_dir "folder", file_is_file "folder", file_is_dir "missing", file_is_file "missing"]' '[true, false, false, false]'
expect 'file_is_dir @folder' 'true'
reject 'file_mtime "missing"' 'failed to read'
reject 'file_size "missing"' 'failed to read'
reject 'lines_grep "missing" "x"' 'failed to read'
reject 'lines_grep "fixture.txt" "["' 'invalid regex'
expect 'relpath "/a/b" "/a/b/c"' 'c'
expect 'relpath "/a/b/c" "/a/b"' '..'
expect 'relpath @a/b @a/b/c' 'c'
expect 'is_function (relpath "/a/b")' 'true'
expect 'typeof (publish "out.txt" "hello")' 'FileTemplate'
expect 'publish "out.txt" "hello"' $'--- out.txt ---\nhello'
expect 'publish @out.txt "hello"' $'--- out.txt ---\nhello'
expect 'publish "out.txt" {"hello"}' $'--- out.txt ---\nhello'
expect 'publish "out.txt" @hello' $'--- out.txt ---\nhello'
[[ ! -e out.txt ]]
printf '%s\n' 'publish "out.txt" "hello"' > publish.av
[[ "$("$AVON" eval publish.av)" == $'--- out.txt ---\nhello' && ! -e out.txt ]]
"$AVON" deploy publish.av > deployment.log
[[ "$(cat out.txt)" == hello ]]
checks=$((checks + 3))

# Formatters, including all named boolean styles, custom pairs, and fallback.
expect 'format_avon [1, true, "hello"]' '[1, true, "hello"]'
expect 'format_avon upper' '"<builtin:upper>"'
expect 'format_bool true "YES/NO"' 'Yes'
expect 'format_bool false "onoff"' 'Off'
expect 'format_bool true "UP/DOWN"' 'up'
expect 'format_bool false "UP/DOWN"' 'down'
expect 'format_bool true "unknown"' 'true'
for entry in 'yesno:Yes:No' 'yes/no:Yes:No' 'onoff:On:Off' 'on/off:On:Off' \
    'truefalse:True:False' 'true/false:True:False' '10:1:0' '1/0:1:0' \
    'enabled:Enabled:Disabled' 'enabled/disabled:Enabled:Disabled' \
    'active:Active:Inactive' 'active/inactive:Active:Inactive' 'unknown:true:false'; do
    IFS=: read -r style yes no <<< "$entry"
    expect "[format_bool true \"$style\", format_bool false \"$style\"]" "[$yes, $no]"
done

expect 'progressbar 0.5 10' '█████░░░░░'
expect 'progressbar 0.7 10' '███████░░░'
expect 'progressbar 0.25 10' '███░░░░░░░'
expect '[progressbar (-1) 3, progressbar 5 3]' '[░░░, ███]'
reject 'progressbar 0.5 0' 'width must be > 0'
expect 'gauge 0.5 10' '◐ 50%'
expect 'gauge 0.75 10' '◕ 75%'
expect '(gauge 0.5 1) == (gauge 0.5 100)' 'true'
expect '[gauge (-1) 10, gauge 50 100, gauge 0.256 10]' '[◯ 0%, ● 100%, ◑ 26%]'

# pfold is a same-type reduction, not fold with an arbitrary initial accumulator.
expect 'pfold (\acc \x acc + x) 0 [1, 2, 3]' '6'
expect 'pfold (\acc \x acc * x) 1 [2, 3, 4]' '24'
expect 'pfold (\acc \x acc + x) 0 []' '0'
expect 'fold (\acc \x acc + x) 10 [1, 2, 3]' '16'
expect 'pfold (\acc \x acc + x) 10 [1, 2, 3]' '46'
expect 'fold (\acc \x acc - x) 0 [1, 2, 3]' '-6'
expect 'pfold (\acc \x acc - x) 0 [1, 2, 3]' '6'
expect 'fold (\acc \x acc + (length x)) 0 ["a", "bb"]' '3'
reject 'pfold (\acc \x acc + (length x)) 0 ["a", "bb"]' 'type mismatch'
for threads in 1 4; do
    export RAYON_NUM_THREADS="$threads"
    expect 'pfold (\acc \x acc + x) 0 (range 1 100)' '5050'
    expect 'pfold (\acc \x acc * x) 1 [2, 3, 4]' '24'
    reject 'pfold (\acc \x acc + (length x)) 0 ["a", "bb"]' 'type mismatch'
done
# Demonstrate that a non-identity seed is not rejected and can depend on chunking.
export RAYON_NUM_THREADS=1
expect 'pfold (\acc \x acc + x) 10 [1, 2, 3]' '26'
export RAYON_NUM_THREADS=2

expect '(hash_sha256 "hello") == (sha256 "hello")' 'true'
expect 'hash_sha256 "hello"' '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824'
expect 'hash_md5 "hello"' '5d41402abc4b2a76b9719d911017c592'
reject 'hash_sha512 "hello"' 'unknown symbol: hash_sha512'
expect 'sha512 "hello"' '9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043'
expect 'base64_decode "SGVsbG8sIFdvcmxkIQ=="' 'Hello, World!'
expect 'hex_decode "68656c6c6f"' 'hello'
reject 'base64_decode "%%%"' 'invalid base64'
reject 'base64_decode "/w=="' 'not valid UTF-8'
reject 'hex_decode "zz"' 'invalid hex'
reject 'hex_decode "f"' 'even length'
reject 'hex_decode "ff"' 'not valid UTF-8'

# All eight file parsers agree with their string counterparts on these fixtures.
printf '%s\n' '{"port":8080}' > data.json
printf 'port: 8080\n' > data.yaml
printf 'port = 8080\n' > data.toml
printf 'name,age\nAda,42\n' > data.csv
printf '%s\n' '<person name="Ada"/>' > data.xml
printf '%s\n' '<p>Hello</p>' > data.html
printf '%s\n' '<opml version="2.0"><head><title>Feeds</title></head><body><outline text="News"><outline text="Child"/></outline></body></opml>' > data.opml
printf 'port=42\n[app]\nactive=true\n' > data.ini
for kind in json yaml toml csv xml html opml ini; do
    same_json "${kind}_parse \"data.$kind\"" "${kind}_parse_string (readfile \"data.$kind\")"
    # Semantic round trip on this fixture only, not a universal guarantee.
    same_json "${kind}_parse \"data.$kind\"" "${kind}_parse_string (format_${kind} (${kind}_parse \"data.$kind\"))"
done

expect '(json_parse_string "null") == (head [])' 'true'
expect '(yaml_parse_string "null") == (head [])' 'true'
expect '[json_parse_string "[1,true,\"x\"]", yaml_parse_string "[1, true, x]"]' '[[1, true, x], [1, true, x]]'
same_json 'csv_parse "data.csv"' '[{name: "Ada", age: "42"}]'
same_json 'csv_parse_string "Ada,42\nBob,7\n"' '[set {Ada: "Bob"} "42" "7"]'
expect 'csv_parse_string ""' '[]'
reject 'csv_parse_string "a\n1\n" false' 'expected function'
same_json 'xml_parse_string "<p>Hello<b>world</b>!</p>"' '{tag: "p", children: ["Hello", {tag: "b", text: "world"}, "!"]}'
same_json 'html_parse_string "<p>Hello<b>world</b>!</p>"' '{tag: "html", children: [{tag: "head"}, {tag: "body", children: [{tag: "p", children: ["Hello", {tag: "b", text: "world"}, "!"]}]}]}'
same_json 'opml_parse "data.opml"' '{version: "2.0", head: {title: "Feeds"}, outlines: [{text: "News", children: [{text: "Child"}]}]}'
expect 'has_key (opml_parse_string "<opml><body/></opml>") "head"' 'false'
same_json 'ini_parse "data.ini"' '{global: {port: "42"}, app: {active: "true"}}'
same_json 'xml_parse_string "<p>  <!--lost--><b>x</b> </p>"' 'xml_parse_string "<p><b>x</b></p>"'
same_json 'html_parse_string "<p>  <!--lost--><b>x</b> </p>"' 'html_parse_string "<p><b>x</b></p>"'
expect '(format_yaml (yaml_parse_string "# lost\nport: 8080\n")) == "port: 8080\n"' 'true'
expect 'format_xml {port: 8080}' '{port: 8080}'
expect 'format_html {port: 8080}' '{port: 8080}'
expect 'format_xml {tag: "root", children: ["Hello", {tag: "b", text: "world"}]}' $'<root>\n  Hello\n  <b>world</b>\n</root>'
expect 'format_html {tag: "div", children: ["Hello", {tag: "b", text: "world"}]}' $'<div>\n  Hello\n  <b>world</b>\n</div>'
expect 'format_html {tag: "br"}' '<br>'
expect 'length (opml_parse_string (format_opml {port: 8080})).outlines' '0'
expect 'contains (format_opml {port: 8080}) "port"' 'false'
expect '(opml_parse_string (format_opml {outlines: []})).version' '2.0'
expect 'starts_with (format_opml {outlines: []}) "<?xml"' 'true'
same_json 'opml_parse_string (format_opml {version: "2.0", head: {title: "Feeds"}, outlines: [{text: "News", children: [{text: "Child"}]}]})' 'opml_parse "data.opml"'
reject 'format_toml [1, 2]' 'TOML serialization error'
for scalar in '42' '1.5' 'true' '"text"' '(head [])'; do
    reject "format_toml $scalar" 'TOML serialization error'
done
expect '(toml_parse_string (format_toml (json_parse_string "{\"missing\":null}"))).missing' 'null'
expect 'format_ini {port: 8080}' ''
expect '(format_ini {global: {port: 8080}}) == "port=8080\n"' 'true'
expect '(format_ini {app: {port: 8080}}) == "[app]\nport=8080\n"' 'true'
expect 'get (ini_parse_string (format_ini {global: {port: 8080}})).global "port"' '8080'
expect 'let parsed = toml_parse_string "born = 1979-05-27T07:32:00Z\n" in [typeof parsed.born, (format_toml parsed) == "born = \"1979-05-27T07:32:00Z\"\n"]' '[String, true]'

# Corrected conversion examples. Compare multi-key objects structurally: key order varies.
expect 'json_parse_string "{\"port\":8080}" -> format_json' '{"port": 8080}'
expect 'let config = json_parse_string "{\"port\":8080}" in (format_yaml (set config "port" 9090)) == "port: 9090\n"' 'true'
same_json 'let users = csv_parse_string "name,age\nAda,42\n" in json_parse_string (format_json (map (\u {name: u.name, age: to_int u.age}) users))' '[{age: 42, name: "Ada"}]'
expect '(json_parse_string "{\"app\":{\"port\":8080}}" -> format_ini) == "[app]\nport=8080\n"' 'true'
same_json 'json_parse_string (xml_parse_string "<person name=\"Ada\"/>" -> format_json)' '{attrs: {name: "Ada"}, tag: "person"}'

printf 'PASS: %d focused runtime checks; %d source-help contract checks (%s)\n' \
    "$checks" "$help_checks" "$("$AVON" --version)"