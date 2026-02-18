# Real-World Task Runner Examples

This file demonstrates how to use the Avon Task Runner for real projects.

## Example 1: Rust Project Build Pipeline

**File: `Avonfile.av`**
```avon
{
  # Format and lint
  fmt: "cargo fmt --check",
  lint: "cargo clippy -- -D warnings",
  
  # Compilation and testing
  build: {
    cmd: "cargo build",
    deps: ["fmt", "lint"]
  },
  test: {
    cmd: "cargo test",
    deps: ["build"]
  },
  
  # Documentation
  doc: {
    cmd: "cargo doc --no-deps --open",
    deps: ["build"]
  },
  
  # Release builds
  release: {
    cmd: "cargo build --release",
    deps: ["test"]
  },
  
  # Full CI pipeline
  ci: {
    cmd: "echo 'CI pipeline complete!'",
    deps: ["release", "doc"]
  },
  
  # Cleanup
  clean: "cargo clean"
}
```

**Usage:**
```bash
# Just check formatting
avon do fmt

# Full CI pipeline (fmt → lint → build → test → release → doc → ci)
avon do ci

# Release builds only (test → release)
avon do release

# Clean workspace
avon do clean
```

---

## Example 2: Web Development

**File: `tasks.av`**
```avon
{
  # Frontend
  npm_install: "npm install",
  npm_build: {
    cmd: "npm run build",
    deps: ["npm_install"]
  },
  npm_test: {
    cmd: "npm test",
    deps: ["npm_install"]
  },
  
  # Backend (Python)
  pip_install: "pip install -r requirements.txt",
  pytest: {
    cmd: "pytest",
    deps: ["pip_install"]
  },
  
  # Docker
  docker_build: "docker build -t myapp .",
  docker_test: {
    cmd: "docker run myapp pytest",
    deps: ["docker_build"]
  },
  
  # Full deployment pipeline
  deploy: {
    cmd: "docker push myapp:latest",
    deps: ["npm_test", "pytest", "docker_test"]
  }
}
```

**Usage:**
```bash
# Setup environment
avon do npm_install tasks.av
avon do pip_install tasks.av

# Test everything
avon do npm_test tasks.av
avon do pytest tasks.av

# Full deployment (tests frontend → backend → docker → deploy)
avon do deploy tasks.av
```

---

## Example 3: Documentation Site

**File: `docs.av`**
```avon
{
  clean: "rm -rf build/",
  
  setup: {
    cmd: "pip install -r docs/requirements.txt",
    deps: ["clean"]
  },
  
  build: {
    cmd: "sphinx-build -b html docs/ build/",
    deps: ["setup"]
  },
  
  spell_check: {
    cmd: "hunspell -d en_US -l -H docs/**/*.rst",
    deps: ["setup"]
  },
  
  link_check: {
    cmd: "linkchecker build/",
    deps: ["build"]
  },
  
  publish: {
    cmd: "rsync -avz build/ user@example.com:/var/www/docs",
    deps: ["build", "spell_check", "link_check"]
  }
}
```

**Usage:**
```bash
# Build docs
avon do build docs.av

# Full quality check and publish
avon do publish docs.av  # Runs: setup → clean → build → spell_check → link_check → publish
```

---

## Example 4: Data Pipeline

**File: `pipeline.av`**
```avon
{
  # Data ingestion
  fetch_raw: "python scripts/fetch_data.py",
  
  # Data cleaning
  clean: {
    cmd: "python scripts/clean.py",
    deps: ["fetch_raw"]
  },
  
  # Data validation
  validate: {
    cmd: "python scripts/validate.py",
    deps: ["clean"]
  },
  
  # Analysis
  analyze: {
    cmd: "python scripts/analyze.py",
    deps: ["validate"]
  },
  
  # Report generation
  report: {
    cmd: "python scripts/generate_report.py",
    deps: ["analyze"]
  },
  
  # Database updates
  db_backup: "pg_dump dbname > backup.sql",
  db_update: {
    cmd: "python scripts/update_db.py",
    deps: ["validate", "db_backup"]
  },
  
  # Full pipeline
  run_all: {
    cmd: "echo 'Pipeline complete'",
    deps: ["report", "db_update"]
  }
}
```

**Execution Flow:**
```
fetch_raw
    ↓
  clean
    ↓
 validate
   ↙   ↘
analyze  db_backup
  ↓         ↓
report   db_update
  ↘       ↙
  run_all
```

---

## Example 5: Multi-Environment Builds

**File: `build.av`**
```avon
{
  # Shared steps
  lint: "eslint src/",
  test: "jest",
  
  # Development
  dev: {
    cmd: "webpack --mode development",
    deps: ["lint"]
  },
  
  # Staging
  staging: {
    cmd: "webpack --mode production && npm run build:staging",
    deps: ["lint", "test"]
  },
  
  # Production
  prod: {
    cmd: "webpack --mode production && npm run optimize",
    deps: ["lint", "test"]
  },
  
  # Environments
  build_all: {
    cmd: "echo 'All builds complete'",
    deps: ["dev", "staging", "prod"]
  }
}
```

---

## Best Practices from Examples

1. **Logical Grouping** - Group related tasks (fmt, lint, build, test)
2. **Clear Dependencies** - Make task relationships explicit
3. **Reusable Base Tasks** - Create small, focused tasks that can be combined
4. **Meaningful Names** - Use clear, searchable task names
5. **Diamond Patterns** - Let tasks converge on final steps (like `publish`)
6. **Idempotent Operations** - Tasks should be safe to run multiple times

---

## Running These Examples

```bash
# Example 1: Rust project
avon do ci Avonfile.av

# Example 2: Web project
avon do deploy tasks.av

# Example 3: Documentation
avon do publish docs.av

# Example 4: Data pipeline
avon do run_all pipeline.av

# Example 5: Multi-environment
avon do build_all build.av
```

All examples demonstrate the power of dependency-based task execution!
