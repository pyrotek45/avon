# Changelog

## 0.6.2 — Unreleased

### Deployment and safety

- Add read-only deployment planning with `deploy --dry-run`: absolute root,
  destination and action reporting, without output files, directories, backups,
  or permission-test probes. Task dry-run remains supported.
- Reject `--dry-run` in non-task/non-deployment modes before source evaluation.
- Share deployment planning and writing between CLI and REPL deployment commands.
- Apply portable relative-path checks with both explicit and implicit roots;
  reject output symlinks, traversal, absolute paths, reserved device names,
  case-folded target conflicts, and output/backup collisions.
- Use capability-relative directory operations and staged entry replacement to
  avoid following output symlinks and mutating other hard links. Preflight no
  longer creates or truncates backup placeholders.
- Preserve ordinary Unix rwx permissions on replacement while stripping special
  mode bits; keep platform-specific code conditional.

### Documentation and examples

- Explain working-directory defaults, content preview versus deployment planning,
  log privacy, root authority, and nontransactional write failures.
- Correct builtin signatures, date units, formatting outputs, decoder errors,
  dictionary ordering, shell quoting, and literal template newlines.
- Add executable reference/guide checks and bounded release-matrix and parallel
  stress examples. Stress checks compare correctness, not speed.
- Derive the CLI version from Cargo metadata to prevent release version drift.

### Compatibility notes

- Absolute generated `publish` paths are rejected even with `--root`; place the
  absolute base in `--root` and use relative paths in templates.
- Output symlinks (including inside-root links) and portable-path conflicts are
  rejected. Choose a trusted output directory; root aliases are intentional.
- `eval --dry-run` and `run --dry-run` now fail rather than silently ignoring the
  flag. Use `eval` for contents and `deploy --dry-run` for destinations.
- Deployment is not a transaction or an evaluator sandbox. Concurrent hostile
  directory renames, cross-process serialization, and rollback are not promised.
  Review the tutorial's security limits before deploying untrusted programs.
- Build with current stable Rust. The old Rust 1.70/1.56 documentation was not a
  tested minimum for this dependency set. LSP and editor-extension versions remain
  independent of the Avon CLI version.

Release tagging/publishing is intentionally a separate step after platform CI.