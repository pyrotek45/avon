# Changelog

## [0.6.0] - 2026-09-22

### Added
- Inline expression mode in the CLI: when a positional argument is not an actual file, Avon now treats it as source code and evaluates it directly.
- Expanded built-in documentation and examples for the newer language functions and helpers.
- Improved release packaging and project validation for the 0.6.0 line.

### Improved
- Updated tutorial and getting-started content to cover fileless expression execution and current CLI workflows.
- Refined deployment, import, and file I/O behavior for more predictable project automation.
- Continued documentation polish for the built-in function reference and example-driven usage.

### Fixed
- Fixed the missing-file fallback path so `avon '1+1'` works as expected without breaking normal file-based execution.
- Resolved formatting and lint regression issues discovered during the shell validation pass.

