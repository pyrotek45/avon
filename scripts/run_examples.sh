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
if grep -q "refusing to overwrite" /tmp/stderr.txt; then
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

if [ $fail -ne 0 ]; then
  echo "Some tests failed"
  exit 1
fi

echo "All tests passed"
exit 0
