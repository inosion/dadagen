# Development Setup Guide

This guide covers setting up the Dadagen development environment.

## Prerequisites

- Rust 1.75 or later
- Just command runner (optional but recommended)
- Git

## Quick Start

### Using Just (Recommended)

```bash
# Install Just if not already installed
cargo install just

# Set up development environment
just setup

# Build the project
just build

# Run tests
just test

# See all available commands
just
```

### Using Cargo Directly

```bash
# Install dependencies
cargo fetch

# Build workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --all-targets --all-features
```

## Workspace Structure

```
dadagen/
├── dadagen-core/          # Core data generation library
├── dadagen-macros/        # Procedural macros for derive support
├── dadagen-cli/           # Command-line interface
├── dadagen-gui/           # Cross-platform GUI (Tauri)
├── dadagen-web/           # WASM web application
├── dadagen-python/        # Python bindings (PyO3)
├── config/                # Configuration files and examples
├── docs/                  # Documentation
└── .github/workflows/     # CI/CD pipelines
```

## Development Workflow

### Making Changes

1. Create a feature branch:
   ```bash
   git checkout -b feature/my-feature
   ```

2. Make your changes and test them:
   ```bash
   just check  # Format, lint, and test
   ```

3. Commit your changes:
   ```bash
   git add .
   git commit -m "feat: add my feature"
   ```

4. Push and create a pull request:
   ```bash
   git push origin feature/my-feature
   ```

### Code Quality

The project enforces strict code quality standards:

- **Formatting**: All code must be formatted with `rustfmt`
  ```bash
  just format
  ```

- **Linting**: Code must pass `clippy` with no warnings
  ```bash
  just lint
  ```

- **Testing**: All tests must pass
  ```bash
  just test
  ```

- **Security**: Dependencies are audited for vulnerabilities
  ```bash
  just security
  ```

### Pre-commit Checks

Before committing, run:
```bash
just pre-commit
```

This will run formatting checks, linting, and tests.

## Common Tasks

### Building Individual Crates

```bash
# Build core library
cargo build --package dadagen-core

# Build CLI
cargo build --package dadagen-cli

# Build macros
cargo build --package dadagen-macros
```

### Running Tests for Specific Crates

```bash
# Test core library
just test-crate dadagen-core

# Test macros
just test-crate dadagen-macros
```

### Generating Documentation

```bash
# Generate and open documentation
just docs

# Or with cargo
cargo doc --open --workspace
```

### Code Coverage

```bash
# Generate coverage report
just test-coverage

# View lcov.info with your preferred tool
```

## IDE Setup

### VS Code

Recommended extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Even Better TOML
- crates

The workspace includes VS Code settings in `.vscode/settings.json`.

### IntelliJ IDEA / CLion

Use the Rust plugin for full IDE support.

## Troubleshooting

### Build Issues

If you encounter build issues:

```bash
# Clean everything and rebuild
just clean-all
cargo fetch
just build
```

### Dependency Issues

```bash
# Update dependencies
just update

# Check for outdated dependencies
just outdated
```

### Test Failures

```bash
# Run tests with verbose output
cargo test --workspace --all-features -- --nocapture

# Run a specific test
cargo test --package dadagen-core test_name
```

## CI/CD

The project uses GitHub Actions for continuous integration:

- **Format Check**: Ensures code is properly formatted
- **Lint**: Runs clippy on all code
- **Test Suite**: Runs tests on Linux, Windows, and macOS
- **Security Audit**: Checks dependencies for vulnerabilities
- **Coverage**: Generates code coverage reports
- **Build**: Builds all targets

All checks must pass before merging pull requests.

## Release Process

1. Update version numbers in all `Cargo.toml` files
2. Update CHANGELOG.md
3. Run release checks:
   ```bash
   just release-check
   ```
4. Build release artifacts:
   ```bash
   just release-build
   ```
5. Create and push a git tag:
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

## Additional Resources

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Just Manual](https://github.com/casey/just)
- [GitHub Copilot Instructions](.github/instructions/github-copilot-development.instructions.md)
