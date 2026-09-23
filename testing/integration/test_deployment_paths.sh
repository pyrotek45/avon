#!/bin/bash
# Verify the CLI behavior and security guarantees documented in the tutorial.
# Run with: bash testing/integration/test_deployment_paths.sh
set -eo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common.sh"
AVON="$(realpath "$AVON")"
TEST_DIR="$(mktemp -d)"
trap 'rm -rf "$TEST_DIR"' EXIT
trap 'echo "FAIL at line $LINENO" >&2' ERR
mkdir -p "$TEST_DIR/source" "$TEST_DIR/work"
cd "$TEST_DIR/work"

cp "$PROJECT_ROOT/examples/deployment_paths.av" ../source/paths.av
preview=$'--- config/app.txt ---\nHello, deployment!'

[[ "$("$AVON" eval ../source/paths.av)" == "$preview" ]]
[[ "$("$AVON" ../source/paths.av)" == "$preview" ]]
[[ "$("$AVON" run '@config/app.txt {"Hello, deployment!"}')" == "$preview" ]]
[[ "$("$AVON" eval ../source/paths.av --root ./unused)" == "$preview" ]]
[[ ! -e config && ! -e unused && ! -e ../source/config ]]
echo 'PASS: eval, bare file, and run preview template paths; eval ignores --root'

[[ "$("$AVON" run 'let subdir = "config" in @{subdir}/app.txt {"Hello, deployment!"}')" == "$preview" ]]
[[ "$("$AVON" run '[@one.txt {"one"}, @two.txt {"two"}]')" == $'--- one.txt ---\none\n--- two.txt ---\ntwo' ]]
echo 'PASS: preview renders interpolated paths and prints each FileTemplate separately'

[[ "$("$AVON" deploy ../source/paths.av)" == 'Wrote config/app.txt' ]]
[[ "$(cat config/app.txt)" == 'Hello, deployment!' && ! -e ../source/config ]]
echo 'PASS: no --root writes relative to working directory, not source directory'

[[ "$("$AVON" deploy ../source/paths.av --root ./output)" == "Wrote $TEST_DIR/work/output/config/app.txt" ]]
[[ "$(cat output/config/app.txt)" == 'Hello, deployment!' ]]
[[ "$("$AVON" deploy ../source/paths.av --root "$TEST_DIR/absolute output")" == "Wrote $TEST_DIR/absolute output/config/app.txt" ]]
[[ "$(cat "$TEST_DIR/absolute output/config/app.txt")" == 'Hello, deployment!' ]]
echo 'PASS: relative and absolute roots create directories and log absolute destinations'

[[ "$("$AVON" deploy ../source/paths.av --root . --force)" == "Overwrote $TEST_DIR/work/config/app.txt" ]]
[[ "$("$AVON" ../source/paths.av --deploy --root ./bare-deploy)" == "Wrote $TEST_DIR/work/bare-deploy/config/app.txt" ]]
echo 'PASS: explicit dot root logs absolute paths; bare-file --deploy writes'

mkdir -p "$TEST_DIR/real-root"
ln -s "$TEST_DIR/real-root" root-alias
[[ "$("$AVON" deploy ../source/paths.av --root ./root-alias)" == "Wrote $TEST_DIR/real-root/config/app.txt" ]]
echo 'PASS: explicit root is canonicalized (including root symlinks)'

printf '%s' 'Keep me' > config/app.txt
output=$("$AVON" deploy ../source/paths.av 2>&1)
[[ "$output" == *'WARNING: File config/app.txt exists.'* && "$(cat config/app.txt)" == 'Keep me' ]]
[[ "$("$AVON" deploy ../source/paths.av --if-not-exists)" == 'Skipped config/app.txt (exists)' ]]
"$AVON" deploy ../source/paths.av --backup
[[ "$(cat config/app.txt.bak)" == 'Keep me' && "$(cat config/app.txt)" == 'Hello, deployment!' ]]
"$AVON" deploy ../source/paths.av --append
[[ "$(cat config/app.txt)" == 'Hello, deployment!Hello, deployment!' ]]
"$AVON" deploy ../source/paths.av --force
[[ "$(cat config/app.txt)" == 'Hello, deployment!' ]]
echo 'PASS: default skip, explicit skip, backup, append, and overwrite behavior'

# A deployment dry-run must show absolute destinations, without creating roots.
output=$("$AVON" deploy ../source/paths.av --root ./planned/deep --dry-run)
[[ "$output" == *"Root: $TEST_DIR/work/planned/deep"* ]]
[[ "$output" == *"CREATE $TEST_DIR/work/planned/deep/config/app.txt"* ]]
[[ "$output" != *'Hello, deployment!'* && ! -e planned ]]
output=$("$AVON" deploy ../source/paths.av --dry-run)
[[ "$output" == *"Root: $TEST_DIR/work (current working directory)"* ]]
[[ "$output" == *"SKIP (exists) $TEST_DIR/work/config/app.txt"* ]]
output=$("$AVON" deploy ../source/paths.av --backup --append --dry-run)
[[ "$output" == *"BACKUP $TEST_DIR/work/config/app.txt -> $TEST_DIR/work/config/app.txt.bak"* ]]
[[ "$output" == *"APPEND $TEST_DIR/work/config/app.txt"* ]]
[[ "$(cat config/app.txt)" == 'Hello, deployment!' && "$(cat config/app.txt.bak)" == 'Keep me' ]]
output=$("$AVON" deploy ../source/paths.av --force --dry-run)
[[ "$output" == *"OVERWRITE $TEST_DIR/work/config/app.txt"* ]]
output=$("$AVON" ../source/paths.av --deploy --root ./bare-plan --dry-run)
[[ "$output" == *"CREATE $TEST_DIR/work/bare-plan/config/app.txt"* && ! -e bare-plan ]]
printf '%s\n' '{build: "touch task-ran"}' > ../source/tasks.av
"$AVON" do --dry-run build ../source/tasks.av
[[ ! -e task-ran ]]
echo 'PASS: deploy plans all actions without writes or contents; do still previews tasks'

reject_dry_run() {
    local result
    if result=$("$AVON" "$@" 2>&1); then
        echo "FAIL: unsupported dry-run accepted: $*" >&2
        exit 1
    fi
    [[ "$result" == *'--dry-run is only supported for do and deploy operations'* ]]
}
reject_dry_run eval ../source/paths.av --dry-run
reject_dry_run eval --git invalid/no/network.av --dry-run
reject_dry_run run 'error "must not evaluate"' --dry-run
reject_dry_run ../source/paths.av --dry-run
reject_dry_run --eval-input '42' --dry-run
reject_dry_run --git-eval invalid/no/network.av --dry-run
for command in doc docs --doc help --help -h version --version -v repl; do
    reject_dry_run "$command" --dry-run
done
[[ "$("$AVON" run '"--dry-run"')" == '--dry-run' ]]
output=$(printf '%s\n' ':preview ../source/paths.av --dry-run' ':quit' | "$AVON" repl 2>&1)
[[ "$output" == *'--dry-run is only supported for do and deploy operations'* ]]
echo 'PASS: dry-run rejects non-deployment/non-task modes before source evaluation'

printf '%s\n' '@../escape.txt {"blocked"}' > ../source/traversal.av
[[ "$("$AVON" eval ../source/traversal.av)" == $'--- ../escape.txt ---\nblocked' ]]
if "$AVON" deploy ../source/traversal.av; then
    echo 'FAIL: traversal without root was accepted' >&2
    exit 1
fi
if "$AVON" deploy ../source/traversal.av --root ./output; then
    echo 'FAIL: traversal was accepted' >&2
    exit 1
fi
[[ ! -e escape.txt && ! -e ../escape.txt ]]
if "$AVON" run '@/absolute.txt {"blocked"}'; then
    echo 'FAIL: absolute path literal was accepted' >&2
    exit 1
fi
echo 'PASS: parent traversal and absolute @path literals are rejected'

# String-based publish can preview an absolute path, unlike an @/ literal.
printf '%s\n' 'publish "/config/app.txt" "Hello, deployment!"' > ../source/absolute.av
[[ "$("$AVON" eval ../source/absolute.av)" == $'--- /config/app.txt ---\nHello, deployment!' ]]
if "$AVON" deploy ../source/absolute.av; then
    echo 'FAIL: absolute publish path without root was accepted' >&2
    exit 1
fi
if "$AVON" deploy ../source/absolute.av --root ./rebased; then
    echo 'FAIL: absolute publish path with root was accepted' >&2
    exit 1
fi
[[ ! -e rebased ]]
echo 'PASS: absolute publish paths preview verbatim but deployment always rejects them'

# All "outside" destinations are still inside this test's private temp directory.
mkdir -p "$TEST_DIR/outside" ./links ./guarded
ln -s "$TEST_DIR/outside" ./links/link
ln -s "$TEST_DIR/outside" ./guarded/link
printf '%s\n' '@link/direct.txt {"symlink probe"}' > ../source/link.av
cd links
if "$AVON" deploy ../../source/link.av; then
    echo 'FAIL: implicit root followed a symlink' >&2
    exit 1
fi
[[ ! -e "$TEST_DIR/outside/direct.txt" ]]
cd ..
if "$AVON" deploy ../source/link.av --root ./guarded --force; then
    echo 'FAIL: explicit root accepted an existing target outside root' >&2
    exit 1
fi
printf '%s\n' '@link/new.txt {"symlink probe"}' > ../source/link.av
if "$AVON" deploy ../source/link.av --root ./guarded; then
    echo 'FAIL: explicit root accepted an existing parent outside root' >&2
    exit 1
fi
[[ ! -e "$TEST_DIR/outside/new.txt" ]]
echo 'PASS: implicit and explicit roots reject symlink output paths'

# Regression: missing parents beneath an existing symlink must not bypass checks.
printf '%s\n' '@link/newdir/nested.txt {"symlink probe"}' > ../source/link.av
for mode in deploy dry-run; do
    flags=()
    if [[ "$mode" == dry-run ]]; then flags+=(--dry-run); fi
    if "$AVON" deploy ../source/link.av --root ./guarded "${flags[@]}"; then
        echo 'FAIL: accepted symlink followed by missing subdirectory' >&2
        exit 1
    fi
done
[[ ! -e "$TEST_DIR/outside/newdir" ]]
echo 'PASS: symlink + missing subdirectory escape is blocked, including dry-run'

# --root changes output routing, not evaluation's working directory or read access.
printf '%s' 'input from working directory' > input.txt
printf '%s\n' '@copy.txt {"{readfile "input.txt"}"}' > ../source/read.av
"$AVON" deploy ../source/read.av --root ./read-output
[[ "$(cat read-output/copy.txt)" == 'input from working directory' ]]
echo 'PASS: relative readfile input is based on working directory, not output root'

# Preflight validation failures must not create or truncate backup files.
mkdir -p preflight/blocked-directory
printf '%s' 'original' > preflight/existing.txt
printf '%s\n' '[@existing.txt {"replacement"}, @blocked-directory {"cannot write a directory"}]' > ../source/preflight.av
if "$AVON" deploy ../source/preflight.av --root ./preflight --backup; then
    echo 'FAIL: writing to a directory was accepted' >&2
    exit 1
fi
[[ "$(cat preflight/existing.txt)" == 'original' && ! -e preflight/existing.txt.bak ]]
echo 'PASS: validation failure leaves original and backup locations untouched'

# Protect unrelated files from the former predictable write-access probe.
printf '%s' 'keep probe' > output/.avon_write_test
"$AVON" deploy ../source/paths.av --root ./output --force
[[ "$(cat output/.avon_write_test)" == 'keep probe' ]]
echo 'PASS: deployment never uses a predictable write-access probe'

# Exercise the REPL's formerly separate writer through the real binary.
output=$(printf '%s\n' ':deploy-expr @nested/repl.txt {"REPL secret"} --root ./repl-plan --dry-run' ':quit' | "$AVON" repl 2>&1)
[[ "$output" == *"CREATE $TEST_DIR/work/repl-plan/nested/repl.txt"* && ! -e repl-plan ]]
output=$(printf '%s\n' ':deploy ../source/paths.av --root ./repl-file-plan --dry-run' ':quit' | "$AVON" repl 2>&1)
[[ "$output" == *"CREATE $TEST_DIR/work/repl-file-plan/config/app.txt"* && ! -e repl-file-plan ]]
output=$(printf '%s\n' ':deploy-expr @link/missing/repl.txt {"blocked"} --root ./guarded' ':quit' | "$AVON" repl 2>&1)
[[ "$output" == *'Deployment aborted'* && ! -e "$TEST_DIR/outside/missing" ]]
echo 'PASS: REPL deployment shares dry-run and containment validation'
echo 'All deployment path checks passed.'