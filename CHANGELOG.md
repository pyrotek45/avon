# Changelog

All notable changes to Avon are documented in this file.

## [0.6.0] - 2026-09-22

### Added
- Direct inline evaluation from the terminal: `avon '1+1'` now works when the argument is not a real file, falling back to evaluating the string as Avon code.
- Better one-off CLI ergonomics for quick expressions without creating a temporary `.av` file.
- Expanded quick-start and usage docs so new users can discover the direct-expression workflow immediately.

### Improved
- CLI file loading now distinguishes between a genuinely missing file and a normal I/O error, so real problems still surface correctly instead of being silently misinterpreted.
- Release metadata is now aligned with the v0.6.0 release version across the package and CLI output.

### Documentation
- Added a dedicated release log for the 0.6.0 milestone.
- Updated the getting-started docs to show both `avon run 'expr'` and the direct `avon 'expr'` shorthand.

### Notes
- This release focuses on a smoother terminal workflow for fast testing and experimentation while preserving the existing file-based deployment and evaluation model.
