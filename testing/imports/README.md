# Import and File I/O Tests

This directory contains tests for Avon's file importing and I/O functionality.

## Test Files

- `test_01_json_folder.av` — Load all JSON files from a folder into a dict
- `test_02_import_modules.av` — Import multiple Avon modules
- `test_03_filter_extension.av` — Filter files by extension
- `test_04_module_registry.av` — Build a module registry dynamically
- `test_05_group_by_dir.av` — Group files by directory
- `test_06_config_override.av` — Merge configs with overrides
- `test_07_load_folder_dict.av` — Load folder as nested dictionary
- `test_08_import_modules.av` — Import modules with validation
- `test_09_combine_data.av` — Combine multiple data files
- `test_10_filter_extension.av` — Filter by file extension pattern
- `test_11_module_registry.av` — Create module registry

## Test Data

- `config/` — JSON configuration files for testing
  - `app.json` — Application settings
  - `cache.json` — Cache configuration
  - `database.json` — Database settings

- `lib/` — Avon module files for testing
  - `math.av` — Math functions
  - `lists.av` — List operations
  - `strings.av` — String utilities

- `data/` — Data files for testing
  - `people.json` — Sample people data
  - `products.json` — Sample products data

## Running Tests

Run all import tests:
```bash
./run-import-tests.sh
```

Run from project root:
```bash
bash testing/imports/run-import-tests.sh
```

Or individually:
```bash
avon eval testing/imports/test_01_json_folder.av
```

## What Gets Tested

✅ `glob` — File pattern matching
✅ `json_parse` — JSON file parsing
✅ `import` — Module loading
✅ `readfile` — Raw file reading
✅ `basename`/`dirname` — Path manipulation
✅ `fold` — Accumulating file operations
✅ `map`/`filter` — Transforming file lists
✅ `dict_merge` — Configuration merging

## Expected Results

All tests should pass without errors. Each test validates:
- Files are found correctly
- Content is parsed/imported properly
- Functions work as documented
- Edge cases are handled

If any test fails, check:
1. Test data exists in all subdirectories (config/, lib/, data/)
2. Working directory is the project root when running tests
3. Avon binary is in PATH or accessible
