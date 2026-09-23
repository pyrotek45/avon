#!/usr/bin/env bash
# Standalone: bash testing/integration/test_release_examples.sh
# Uses the current release binary; never rebuilds or needs network access.
set -euo pipefail
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
AVON="$PROJECT_ROOT/target/release/avon"
[[ -x "$AVON" ]] || { echo "FAIL: release binary is not executable: $AVON" >&2; exit 1; }
command -v jq >/dev/null || { echo 'FAIL: jq is required' >&2; exit 1; }
TEST_DIR="$(mktemp -d)"
trap 'rm -rf -- "$TEST_DIR"' EXIT
trap 'echo "FAIL at line $LINENO" >&2' ERR
cd "$TEST_DIR"
RELEASE="$PROJECT_ROOT/examples/release_matrix.av"
STRESS="$PROJECT_ROOT/examples/parallel_stress.av"

summary=$("$AVON" eval "$RELEASE" -output summary)
jq -e '. == {ok:true, example:"release_matrix", environments:3, files:9}' <<< "$summary" >/dev/null
echo "PASS release summary: $summary"

# Test a nonexistent nested root with spaces, including its absent parent.
root="$TEST_DIR/planned output/nested"
plan=$("$AVON" deploy "$RELEASE" --root "$root" --dry-run)
[[ ! -e "$TEST_DIR/planned output" ]]
[[ $(grep -c 'CREATE ' <<< "$plan") -eq 9 ]]
for environment in dev staging prod; do
    for extension in json yaml toml; do
        grep -Fq "CREATE $root/$environment/config.$extension" <<< "$plan"
    done
done
echo 'PASS deploy --dry-run: nine CREATE actions; no root or parent created'

deployment=$("$AVON" deploy "$RELEASE" --root "$root")
[[ $(grep -c '^Wrote ' <<< "$deployment") -eq 9 ]]
[[ $(find "$root" -type f | wc -l) -eq 9 ]]
for environment in dev staging prod; do
    for extension in json yaml toml; do
        [[ -s "$root/$environment/config.$extension" ]]
    done
    # Exact structural comparison also checks types, defaults and overrides.
    jq -e --arg env "$environment" '
      . == {environment:$env, service:"atlas", version:"1.2.3",
            region:(if $env == "staging" then "eu-west" else "local" end),
            port:(if $env == "prod" then 9090 else 8080 end),
            replicas:(if $env == "prod" then 4 elif $env == "staging" then 2 else 1 end),
            debug:($env == "dev"), timeout:30}
    ' "$root/$environment/config.json" >/dev/null
done
echo 'PASS deploy: nine nonempty files; all three JSON configurations match'

check_stress() {
    local threads="$1" size="$2" result
    shift 2
    result=$(RAYON_NUM_THREADS="$threads" "$AVON" eval "$STRESS" "$@")
    jq -e --argjson n "$size" '
      . == {ok:true, example:"parallel_stress", size:$n, mapped:$n,
            filtered:($n / 3 | floor), sum:($n * ($n + 1) / 2),
            mapped_sum:($n * ($n + 1) * (2 * $n + 1) / 6 + 3 * $n * ($n + 1) / 2 + 7 * $n)}
    ' <<< "$result" >/dev/null
    echo "PASS threads=$threads: $result"
}

check_stress 2 2000
for threads in 1 2 4; do
    # 2049 exercises uneven partitions; 0 and 1 exercise empty/tiny inputs.
    for size in 2049 0 1; do
        check_stress "$threads" "$size" -size "$size"
    done
done
check_stress 4 10000 -size 10000

for invalid in -1 10001 99999 100000 999999999999999999999999 abc 1.5 1e3 '' ' 2' '2 '; do
    if result=$(RAYON_NUM_THREADS=2 "$AVON" eval "$STRESS" -size "$invalid" 2>&1); then
        echo "FAIL: accepted invalid size '$invalid': $result" >&2
        exit 1
    fi
    [[ "$result" == *'parallel_stress: size must be an integer from 0 to 10000'* ]]
done
echo 'PASS invalid sizes: negative, over-limit, overflow, noninteger, empty and whitespace rejected'
echo 'PASS: release examples integration tests'