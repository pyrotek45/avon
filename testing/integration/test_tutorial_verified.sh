#!/usr/bin/env bash
# Focused documentation examples, not an exhaustive tutorial or security audit.
# Run with: bash testing/integration/test_tutorial_verified.sh
set -euo pipefail
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
AVON="$(realpath "${AVON:-$PROJECT_ROOT/target/release/avon}")"
[[ -x "$AVON" ]] || { echo "Release binary required" >&2; exit 1; }
TEST_DIR="$(mktemp -d)"
trap 'rm -rf "$TEST_DIR"' EXIT
trap 'echo "FAIL at line $LINENO" >&2' ERR
cd "$TEST_DIR"

expect() {
    local expression="$1" expected="$2" actual
    actual=$("$AVON" run "$expression")
    [[ "$actual" == "$expected" ]] || {
        printf 'FAIL: %s\nExpected: %s\nActual: %s\n' "$expression" "$expected" "$actual" >&2
        exit 1
    }
    printf 'PASS: %s => %s\n' "$expression" "$actual"
}
reject() {
    local fragment="$1" output
    shift
    if output=$("$AVON" "$@" 2>&1); then
        printf 'FAIL: unexpectedly accepted %s\n' "$*" >&2
        exit 1
    fi
    [[ "$output" == *"$fragment"* ]]
    printf 'PASS: rejected %s (%s)\n' "$*" "$fragment"
}

expect 'let f = \a \b [a,b] in 1 -> f 2' '[2, 1]'
expect '"hello world" -> regex_match "^hello"' 'true'
expect 'regex_match "^hello" "hello world"' 'true'
expect 'join ["a","b"] ","' 'a,b'
reject 'type mismatch' run '["a","b"] -> join ","'
expect '["a","b"] -> (\xs join xs ",")' 'a,b'
expect 'replace "aaa bbb" "aaa" "XXX"' 'XXX bbb'
expect '"aaa bbb" -> replace "aaa" "XXX"' 'aaa'
expect '"aaa bbb" -> replace "aaa" "XXX" -> replace "bbb" "YYY"' 'bbb'
expect 'replace (replace "aaa bbb" "aaa" "XXX") "bbb" "YYY"' 'XXX YYY'
expect '"aaa bbb" -> (\s replace s "aaa" "XXX") -> (\s replace s "bbb" "YYY")' 'XXX YYY'
expect '[1,2,3,4,5] -> map (\x x * 2) -> filter (\x x > 2) -> length' '4'
expect $'map (\\s\n s -> upper -> length) ["ab","c"]' '[2, 1]'
expect $'map (\\s\n (s -> upper -> length)) ["ab","c"]' '[2, 1]'
reject 'unknown symbol: dict_set' run 'dict_set {} "x" 1'
expect 'set {} "x" 1' '{x: 1}'
reject 'unknown symbol: not_an_api' run 'let unused = not_an_api 1 in 1'
reject 'failed to read' run 'let unused = readfile "missing.txt" in 1'
reject 'failed to read' run 'json_parse "[1,2]"'

mkdir -p source data dir.with.dot
printf '%s' hello > config.yml
printf '%s' 'cwd data' > input.txt
printf '%s' 'source data' > source/input.txt
printf '%s' 'readfile "input.txt"' > source/module.av
printf '%s' '42' > source/value.av
printf '%s' '7' > value.av
printf '%s' 'import "value.av"' > source/nested.av
printf '%s' '[1,2]' > data/a.json
expect 'readfile @config.yml' 'hello'
expect 'readfile "config.yml"' 'hello'
expect 'import @source/value.av' '42'
expect 'import "source/value.av"' '42'
expect 'import "source/module.av"' 'cwd data'
expect 'import "source/nested.av"' '7'
[[ "$("$AVON" eval source/module.av --root unused-root)" == 'cwd data' && ! -e unused-root ]]
expect 'contains "dir.with.dot" (glob "**/*.*")' 'true'
expect $'map (\\f\n json_parse f\n -> map (\\n n * 2)\n -> format_json\n -> publish (f + ".out")\n) ["data/a.json"]' $'--- data/a.json.out ---\n[2, 4]'
expect $'map (\\f\n (json_parse f\n -> map (\\n n * 2)\n -> format_json\n -> publish (f + ".out"))\n) ["data/a.json"]' $'--- data/a.json.out ---\n[2, 4]'
expect 'let commands = ["start", "stop", "restart"] in let input = "invalid" in let cmd = if contains input commands then input else "start" in publish "config.json" (format_json {command: cmd})' $'--- config.json ---\n{"command": "start"}'
expect 'let value = "safe_\"quoted" in let safe_value = if starts_with value "safe_" then value else "" in publish "config.json" (format_json {value: safe_value})' $'--- config.json ---\n{"value": "safe_\\"quoted"}'
expect 'let value = "\"quoted" in @raw.txt {"{value}"}' $'--- raw.txt ---\n"quoted'
expect '@config.json {"setting: value"}' $'--- config.json ---\nsetting: value'
expect '@app/config/settings.json {"debug: true"}' $'--- app/config/settings.json ---\ndebug: true'
expect '@subdir/file.txt {"content: data"}' $'--- subdir/file.txt ---\ncontent: data'

# Extra deployment cases complement test_deployment_paths.sh.
for path in 'C:/file' 'C:file' '//host/share' 'a\\b' 'file:stream' 'NUL' 'aux.txt' 'COM1' 'trailing.' 'trailing ' 'a?b' 'dir/' 'a\nlog'; do
    printf 'publish "%s" "content"\n' "$path" > invalid.av
    reject 'Deployment aborted' deploy invalid.av --root missing-root
    [[ ! -e missing-root ]]
done
for expression in '[@a {"one"}, @./a {"two"}]' '[@A {"one"}, @a {"two"}]' '[@a {"one"}, @a/b {"two"}]'; do
    printf '%s' "$expression" > conflict.av
    reject 'Conflicting output or backup paths' deploy conflict.av --root missing-root
    [[ ! -e missing-root ]]
done
mkdir -p root/inside
ln -s inside root/link
ln -s absent root/dangling
for path in link/new/file dangling/file; do
    printf 'publish "%s" "content"\n' "$path" > invalid.av
    reject 'symlinks' deploy invalid.av --root root
done
printf '%s' 'original' > root/a
printf '%s' 'old backup' > root/a.bak
printf '%s' 'probe' > root/.avon_write_test
printf '%s' '[@a {"new"}, @a.bak {"collision"}]' > conflict.av
reject 'Conflicting output or backup paths' deploy conflict.av --root root --backup
[[ "$(cat root/a)" == original && "$(cat root/a.bak)" == 'old backup' ]]
[[ "$(cat root/.avon_write_test)" == probe ]]

# Entry replacement must not mutate an outside hard-link alias, even for backups.
printf '%s' '@a {"new"}' > write.av
for flag in --force --append --backup; do
    rm -f root/a root/a.bak outside outside-backup
    printf '%s' 'original' > outside
    printf '%s' 'old backup' > outside-backup
    ln outside root/a
    ln outside-backup root/a.bak
    chmod 6751 root/a
    "$AVON" deploy write.av --root root "$flag"
    [[ "$(cat outside)" == original && "$(cat outside-backup)" == 'old backup' ]]
    [[ "$(stat -c %a root/a)" == 751 ]]
    if [[ "$flag" == --append ]]; then
        [[ "$(cat root/a)" == originalnew ]]
    else
        [[ "$(cat root/a)" == new ]]
    fi
    if [[ "$flag" == --backup ]]; then
        [[ "$(cat root/a.bak)" == original && "$(stat -c %a root/a.bak)" == 751 ]]
    fi
done
echo 'PASS: entry replacement isolates hard links; backup replaces old .bak; Unix rwx retained, setuid/setgid stripped'

# A missing parent defers this OS filename-length failure until actual writing.
printf -v long_name '%0256d' 0
printf '[@a {"later"}, publish "newdir/%s" "fail"]' "$long_name" > partial.av
reject 'No rollback was performed' deploy partial.av --root root --backup
[[ "$(cat root/a)" == later && "$(cat root/a.bak)" == new && -d root/newdir ]]
echo 'PASS: writer failure leaves completed output, backup and created directory'

# Explicit REPL destinations are ambient, but final symlinks are rejected.
mkdir repl
pushd repl >/dev/null
printf '%s\n' ':write ../explicit.txt "text"' ':let answer = 42' ':save-session ../session.avon' ':deploy-expr @planned.txt {"secret"} --dry-run' ':quit' | "$AVON" repl > ../repl.log 2>&1
[[ "$(cat ../explicit.txt)" == text && -s ../session.avon && ! -e planned.txt ]]
grep -Fq "CREATE $TEST_DIR/repl/planned.txt" ../repl.log
ln -s ../outside leaf
printf '%s\n' ':write leaf "changed"' ':save-session leaf' ':quit' | "$AVON" repl > ../repl-reject.log 2>&1
[[ -L leaf && "$(cat ../outside)" == original ]]
grep -qi symlink ../repl-reject.log
popd >/dev/null
echo 'PASS: REPL no-root dry-run and ambient explicit-path leaf protection'
echo 'All focused tutorial checks passed (Linux shell fixtures; no network).'