#!/usr/bin/env bash
set -euo pipefail
root=$(mktemp -d)
echo "Using temp root: $root"

echo "Building..."
cargo build --quiet

fail=0

echo "Test: deploy examples/test.av"
if cargo run --quiet -- examples/test.av --deploy alice --root "$root"; then
  if [ -f "$root/home/alice/file" ]; then
    grep -q "hello alice" "$root/home/alice/file" || { echo "FAIL: content mismatch"; fail=1; }
  else
    echo "FAIL: file not created"; fail=1
  fi
else
  echo "FAIL: deploy failed"; fail=1
fi

echo "Test: deploy list_insert.av"
if cargo run --quiet -- examples/list_insert.av --deploy test --root "$root"; then
  # path without leading slash: $root/test/file
  if [ -f "$root/test/file" ]; then
    # Check that list items expanded without brackets
    if grep -q "line 1" "$root/test/file" && ! grep -q "\[" "$root/test/file"; then
      echo "OK"
    else
      echo "FAIL: list interpolation not correct"; fail=1
    fi
  else
    echo "FAIL: list file not created"; fail=1
  fi
else
  echo "FAIL: list deploy failed"; fail=1
fi

echo "Test: eval import_example.av"
out=$(cargo run --quiet -- eval examples/import_example.av)
if [[ "$out" == "[one, two]" ]]; then
  echo "OK"
else
  echo "FAIL: import eval output unexpected: $out"; fail=1
fi

echo "Test: eval map_example.av"
out=$(cargo run --quiet -- eval examples/map_example.av)
if [[ "$out" == "[a-, b-, c-]" ]]; then
  echo "OK"
else
  echo "FAIL: map output unexpected: $out"; fail=1
fi

echo "Test: eval fold_example.av"
out=$(cargo run --quiet -- eval examples/fold_example.av)
if [[ "$out" == "abc" || "$out" == "ab c" || "$out" == "ab c" ]]; then
  echo "OK"
else
  echo "fold output: $out"; fail=1
fi

echo "Test: deploy site_generator.av"
if cargo run --quiet -- examples/site_generator.av --deploy --root "$root"; then
  ok=1
  for p in index about contact; do
    if [ ! -f "$root/${p}.html" ]; then
      echo "FAIL: $p.html not created"; ok=0; fail=1
    fi
  done
  if [ $ok -eq 1 ]; then echo "OK"; fi
else
  echo "FAIL: site_generator deploy failed"; fail=1
fi

echo "Test: deploy multi_file_deploy.av"
if cargo run --quiet -- examples/multi_file_deploy.av --deploy --root "$root"; then
  if [ -f "$root/home/alice/welcome.txt" ] && [ -f "$root/home/bob/welcome.txt" ]; then
    echo "OK"
  else
    echo "FAIL: user welcome files missing"; fail=1
  fi
else
  echo "FAIL: multi_file_deploy failed"; fail=1
fi

echo "Test: deploy complex_template.av"
if cargo run --quiet -- examples/complex_template.av --deploy --root "$root"; then
  if [ -f "$root/tmp/complex_sections.txt" ]; then
    if grep -q "Page Sections" "$root/tmp/complex_sections.txt" && grep -q -- "- Header" "$root/tmp/complex_sections.txt"; then
      echo "OK"
    else
      echo "FAIL: complex_template content mismatch"; fail=1
    fi
  else
    echo "FAIL: complex_template file missing"; fail=1
  fi
else
  echo "FAIL: complex_template deploy failed"; fail=1
fi

echo "Test: deploy large_program.av"
if cargo run --quiet -- examples/large_program.av --deploy --root "$root"; then
  # expect three files under $root/files
  if [ -f "$root/files/A_B_x.txt" ] && [ -f "$root/files/A_B_y.txt" ] && [ -f "$root/files/A_B_z.txt" ]; then
    echo "OK"
  else
    echo "FAIL: large_program files missing"; fail=1
  fi
else
  echo "FAIL: large_program deploy failed"; fail=1
fi

echo "Test: eval features_demo.av"
out=$(cargo run --quiet -- eval examples/features_demo.av)
if [[ -n "$out" ]]; then
  echo "OK"
else
  echo "FAIL: features_demo produced no output"; fail=1
fi

echo "Test: deploy escape_hatch.av"
if cargo run --quiet -- examples/escape_hatch.av --deploy --root "$root"; then
  if [ -f "$root/single_demo.txt" ] && [ -f "$root/double_demo.txt" ]; then
    # Verify single-brace template output: {{ should produce {
    if grep -q "One brace escape: {" "$root/single_demo.txt" && grep -q "Three literal opens: {{{" "$root/single_demo.txt"; then
      # Verify double-brace template output: {{ expr }} should work
      if grep -q "Computed in double braces: 30" "$root/double_demo.txt"; then
        echo "OK"
      else
        echo "FAIL: double-brace interpolation failed"; fail=1
      fi
    else
      echo "FAIL: single-brace escape hatch output mismatch"; fail=1
    fi
  else
    echo "FAIL: escape_hatch files not created"; fail=1
  fi
else
  echo "FAIL: escape_hatch deploy failed"; fail=1
fi
if cargo run --quiet -- examples/complex_usage_1.av --deploy --root "$root"; then
  if [ -f "$root/tmp/complex_usage_1.txt" ]; then
    echo "OK"
  else
    echo "FAIL: complex_usage_1 not created"; fail=1
  fi
else
  echo "FAIL: complex_usage_1 deploy failed"; fail=1
fi

echo "Test: deploy complex_usage_2.av"
if cargo run --quiet -- examples/complex_usage_2.av --deploy --root "$root"; then
  if [ -f "$root/files/A_report.txt" ] && [ -f "$root/files/B_report.txt" ] && [ -f "$root/files/C_report.txt" ]; then
    echo "OK"
  else
    echo "FAIL: complex_usage_2 files missing"; fail=1
  fi
else
  echo "FAIL: complex_usage_2 deploy failed"; fail=1
fi

echo "Test: deploy complex_usage_3.av"
if cargo run --quiet -- examples/complex_usage_3.av --deploy --root "$root"; then
  if [ -f "$root/reports/anonymous_Summary.txt" ] && [ -f "$root/reports/bob_Detailed.txt" ]; then
    echo "OK"
  else
    echo "FAIL: complex_usage_3 expected reports missing"; fail=1
  fi
else
  echo "FAIL: complex_usage_3 deploy failed"; fail=1
fi

echo "Test: deploy complex_usage_4.av"
if cargo run --quiet -- examples/complex_usage_4.av --deploy --root "$root"; then
  if [ -f "$root/tmp/complex_usage_4.txt" ]; then
    echo "OK"
  else
    echo "FAIL: complex_usage_4 not created"; fail=1
  fi
else
  echo "FAIL: complex_usage_4 deploy failed"; fail=1
fi

echo "Test: deploy complex_usage_5.av"
if cargo run --quiet -- examples/complex_usage_5.av --deploy --root "$root"; then
  if [ -f "$root/tmp/complex_usage_5.txt" ]; then
    echo "OK"
  else
    echo "FAIL: complex_usage_5 missing"; fail=1
  fi
else
  echo "FAIL: complex_usage_5 deploy failed"; fail=1
fi

echo "Test: eval plus_strings.av"
out=$(cargo run --quiet -- eval examples/plus_strings.av)
if [[ -n "$out" ]]; then
  echo "OK"
else
  echo "FAIL: plus_strings produced no output"; fail=1
fi

echo "Test: deploy plus_lists.av"
if cargo run --quiet -- examples/plus_lists.av --deploy --root "$root"; then
  if [ -f "$root/tmp/plus_lists.txt" ]; then
    echo "OK"
  else
    echo "FAIL: plus_lists missing"; fail=1
  fi
else
  echo "FAIL: plus_lists deploy failed"; fail=1
fi

echo "Test: deploy operators_demo.av"
if cargo run --quiet -- examples/operators_demo.av --deploy --root "$root"; then
  if [ -f "$root/tmp/operators_demo.txt" ]; then
    echo "OK"
  else
    echo "FAIL: operators_demo missing"; fail=1
  fi
else
  echo "FAIL: operators_demo deploy failed"; fail=1
fi

# test overwrite protection
echo "Test: overwrite protection"
cargo run --quiet -- examples/test.av --deploy alice --root "$root" 2> /tmp/stderr.txt || true
if grep -q "WARNING:.*exists.*Use --force" /tmp/stderr.txt || grep -q "already exists" /tmp/stderr.txt; then
  echo "OK (refused overwrite)"
else
  echo "WARN: second deploy did not refuse overwrite (expected message)"; fail=1
fi

# test --force
cargo run --quiet -- examples/test.av --deploy alice --root "$root" --force
if [ $? -ne 0 ]; then
  echo "FAIL: --force deploy failed"; fail=1
else
  echo "OK (force)"
fi

# Test all remaining example files for basic evaluation
echo "Test: eval all remaining examples"
remaining_files=(
  "casting_demo.av"
  "ci_pipeline.av"
  "conditionals_template.av"
  "contains_starts_ends.av"
  "curly_test_1_simple_json.av"
  "curly_test_2_nested_json.av"
  "curly_test_3_mixed.av"
  "curly_test_4_array.av"
  "curly_test_5_code.av"
  "deploy_list.av"
  "dict_to_table.av"
  "docker_compose.av"
  "docker_compose_gen.av"
  "emacs_init.av"
  "filter_example.av"
  "formatting_demo.av"
  "formatting_practical.av"
  "function_defaults.av"
  "github_actions_gen.av"
  "html_page_gen.av"
  "join_replace.av"
  "json_map_demo.av"
  "kubernetes_gen.av"
  "let_cascade.av"
  "map_filter_fold.av"
  "markdown_readme_gen.av"
  "named_args.av"
  "neovim_config_fn.av"
  "neovim_config_gen.av"
  "neovim_init.av"
  "neovim_lua_simple.av"
  "neovim_simple.av"
  "nested_let.av"
  "new_functions_demo.av"
  "nginx_config.av"
  "nginx_gen.av"
  "os_conditionals.av"
  "package_json_gen.av"
  "split_join.av"
  "string_functions.av"
  "terraform_gen.av"
  "vim_plugins.av"
  "vim_simple.av"
  "pipe_operator.av"
  "pipe_operator_demo.av"
)

eval_count=0
for file in "${remaining_files[@]}"; do
  if cargo run --quiet -- eval "examples/$file" > /dev/null 2>&1; then
    eval_count=$((eval_count + 1))
  else
    echo "FAIL: examples/$file failed to eval"; fail=1
  fi
done
echo "OK ($eval_count/${#remaining_files[@]} examples evaluated successfully)"

# Test specific examples with content validation
echo "Test: eval filter_example.av"
out=$(cargo run --quiet -- eval examples/filter_example.av)
if [[ "$out" == "[a, b, c]" ]]; then
  echo "OK"
else
  echo "FAIL: filter_example output unexpected: $out"; fail=1
fi

echo "Test: eval string_functions.av"
out=$(cargo run --quiet -- eval examples/string_functions.av)
if [[ -n "$out" ]]; then
  echo "OK"
else
  echo "FAIL: string_functions produced no output"; fail=1
fi

echo "Test: eval split_join.av"
out=$(cargo run --quiet -- eval examples/split_join.av)
if [[ -n "$out" ]]; then
  echo "OK"
else
  echo "FAIL: split_join produced no output"; fail=1
fi

echo "Test: eval join_replace.av"
out=$(cargo run --quiet -- eval examples/join_replace.av)
if [[ -n "$out" ]]; then
  echo "OK"
else
  echo "FAIL: join_replace produced no output"; fail=1
fi

echo "Test: eval contains_starts_ends.av"
out=$(cargo run --quiet -- eval examples/contains_starts_ends.av)
if [[ -n "$out" ]]; then
  echo "OK"
else
  echo "FAIL: contains_starts_ends produced no output"; fail=1
fi

echo "Test: eval map_filter_fold.av"
out=$(cargo run --quiet -- eval examples/map_filter_fold.av)
if [[ -n "$out" ]]; then
  echo "OK"
else
  echo "FAIL: map_filter_fold produced no output"; fail=1
fi

echo "Test: eval nested_let.av"
out=$(cargo run --quiet -- eval examples/nested_let.av)
if [[ -n "$out" ]]; then
  echo "OK"
else
  echo "FAIL: nested_let produced no output"; fail=1
fi

echo "Test: eval let_cascade.av"
out=$(cargo run --quiet -- eval examples/let_cascade.av)
if [[ -n "$out" ]]; then
  echo "OK"
else
  echo "FAIL: let_cascade produced no output"; fail=1
fi

echo "Test: deploy package_json_gen.av"
if cargo run --quiet -- examples/package_json_gen.av --deploy --root "$root"; then
  if [ -f "$root/package.json" ]; then
    if grep -q '"name": "awesome-app"' "$root/package.json" && grep -q '"scripts"' "$root/package.json"; then
      echo "OK"
    else
      echo "FAIL: package_json_gen content mismatch"; fail=1
    fi
  else
    echo "FAIL: package.json not created"; fail=1
  fi
else
  echo "FAIL: package_json_gen deploy failed"; fail=1
fi

echo "Test: deploy docker_compose_gen.av"
if cargo run --quiet -- examples/docker_compose_gen.av --deploy --root "$root"; then
  if [ -f "$root/docker-compose.yml" ]; then
    if grep -q "version:" "$root/docker-compose.yml" && grep -q "services:" "$root/docker-compose.yml"; then
      echo "OK"
    else
      echo "FAIL: docker_compose_gen content mismatch"; fail=1
    fi
  else
    echo "FAIL: docker-compose.yml not created"; fail=1
  fi
else
  echo "FAIL: docker_compose_gen deploy failed"; fail=1
fi

echo "Test: deploy vim_simple.av"
if cargo run --quiet -- examples/vim_simple.av --deploy --root "$root"; then
  if [ -f "$root/home/developer/.vimrc" ]; then
    if grep -q "set number" "$root/home/developer/.vimrc" && grep -q '" Simple Vim Configuration' "$root/home/developer/.vimrc"; then
      echo "OK"
    else
      echo "FAIL: vim_simple content mismatch"; fail=1
    fi
  else
    echo "FAIL: .vimrc not created"; fail=1
  fi
else
  echo "FAIL: vim_simple deploy failed"; fail=1
fi

echo "Test: deploy neovim_simple.av"
if cargo run --quiet -- examples/neovim_simple.av --deploy --root "$root"; then
  if [ -f "$root/home/developer/.config/nvim/init.vim" ]; then
    if grep -q "set number" "$root/home/developer/.config/nvim/init.vim"; then
      echo "OK"
    else
      echo "FAIL: neovim_simple content mismatch"; fail=1
    fi
  else
    echo "FAIL: neovim init.vim not created"; fail=1
  fi
else
  echo "FAIL: neovim_simple deploy failed"; fail=1
fi

echo "Test: deploy emacs_init.av"
if cargo run --quiet -- examples/emacs_init.av --deploy --root "$root"; then
  if [ -f "$root/home/emacsUser/.emacs.d/init.el" ]; then
    if grep -q "require 'package" "$root/home/emacsUser/.emacs.d/init.el"; then
      echo "OK"
    else
      echo "FAIL: emacs_init content mismatch"; fail=1
    fi
  else
    echo "FAIL: emacs init.el not created"; fail=1
  fi
else
  echo "FAIL: emacs_init deploy failed"; fail=1
fi

echo "Test: deploy github_actions_gen.av"
if cargo run --quiet -- examples/github_actions_gen.av --deploy --root "$root"; then
  if [ -f "$root/.github/workflows/ci.yml" ] && [ -f "$root/.github/workflows/release.yml" ]; then
    if grep -q 'name: CI' "$root/.github/workflows/ci.yml" && grep -q '${{ github.repository }}' "$root/.github/workflows/ci.yml"; then
      if grep -q 'createComment({' "$root/.github/workflows/ci.yml"; then
        echo "OK"
      else
        echo "FAIL: github_actions ci.yml missing JavaScript object syntax"; fail=1
      fi
    else
      echo "FAIL: github_actions ci.yml content mismatch"; fail=1
    fi
  else
    echo "FAIL: github workflows not created"; fail=1
  fi
else
  echo "FAIL: github_actions_gen deploy failed"; fail=1
fi

echo "Test: curly brace regression tests"
curly_ok=0
for i in 1 2 3 4 5; do
  if cargo run --quiet -- eval "examples/curly_test_${i}_"*.av > /dev/null 2>&1; then
    curly_ok=$((curly_ok + 1))
  else
    echo "FAIL: curly_test_${i} failed"; fail=1
  fi
done
if [ $curly_ok -eq 5 ]; then
  echo "OK (all 5 curly brace tests passed)"
else
  echo "FAIL: only $curly_ok/5 curly tests passed"; fail=1
fi

if [ $fail -ne 0 ]; then
  echo "Some tests failed"
  exit 1
fi

echo "All tests passed"
exit 0
