# Changelog

## [0.6.0] - 2026-09-22

### Added
- Added color and ricing builtin function modules, including palette, contrast, and theme-generation helpers.
- Added comprehensive built-in documentation and examples for the new function set, including the color and ricing categories.
- Added direct inline CLI evaluation so a positional argument that is not a real file is treated as Avon source and executed immediately.

### Improved
- Expanded the tutorial and getting-started material to cover the new builtins, documentation structure, and current CLI workflows.
- Improved reference examples and clarifications for template syntax, color utilities, and ricing-related usage.
- Updated the release packaging and validation metadata for the 0.6.0 line.

### Fixed
- Corrected tutorial and example syntax issues for the color and ricing functions and related docs.
- Fixed the system monitor example syntax and resolved clippy/test warnings discovered during validation.
- Fixed the missing-file fallback path so `avon '1+1'` works without breaking standard file-based execution.

