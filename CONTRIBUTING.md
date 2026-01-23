# Contributing to Dadagen

Thank you for your interest in contributing to Dadagen! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Be respectful, inclusive, and constructive in all interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, etc.)
- Any relevant error messages or logs

### Suggesting Enhancements

Enhancement suggestions are welcome! Please provide:

- A clear and descriptive title
- Detailed description of the proposed functionality
- Rationale for why this enhancement would be useful
- Any relevant examples or mockups

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Follow the spec-driven development workflow** for complex features:
   - Create requirements document in `.specs/{feature-name}/requirements.md`
   - Create design document in `.specs/{feature-name}/design.md`
   - Create task breakdown in `.specs/{feature-name}/tasks.md`
   - Implement tasks one at a time
3. **Write tests** for your changes
4. **Update documentation** as needed
5. **Run quality checks**:
   ```bash
   just pre-commit
   ```
6. **Commit with clear messages** following conventional commits format:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation changes
   - `refactor:` for code refactoring
   - `test:` for adding tests
   - `chore:` for maintenance tasks

## Development Setup

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed setup instructions.

Quick start:
```bash
# Clone the repository
git clone https://github.com/inosion/dadagen.git
cd dadagen

# Set up development environment
just setup

# Run tests
just test
```

## Coding Standards

### Rust Code Style

- Follow the Rust API Guidelines
- Use `rustfmt` for formatting (configuration in `rustfmt.toml`)
- Pass all `clippy` lints (configuration in `.clippy.toml`)
- Write comprehensive documentation comments
- Include examples in documentation
- Write unit tests for all public APIs

### Code Quality Requirements

All code must:
- Pass `cargo fmt --check`
- Pass `cargo clippy -- -D warnings`
- Pass `cargo test --workspace`
- Pass `cargo audit` (no known vulnerabilities)
- Have appropriate test coverage

Run all checks with:
```bash
just pre-commit
```

### Documentation

- Document all public APIs with doc comments
- Include usage examples in documentation
- Update relevant documentation files (README, guides, etc.)
- Keep the changelog updated

### Testing

- Write unit tests for all new functionality
- Add integration tests for complex features
- Ensure tests are deterministic and don't depend on external services
- Test error cases and edge conditions
- Run tests with:
  ```bash
  just test
  ```

## Spec-Driven Development

For complex features, follow the spec-driven development workflow:

1. **Requirements Phase**: Create `requirements.md` with EARS format
2. **Design Phase**: Create `design.md` with architecture details
3. **Tasks Phase**: Create `tasks.md` breaking down implementation
4. **Execution Phase**: Implement tasks one at a time with reviews

See `.github/instructions/github-copilot-development.instructions.md` for detailed guidance.

## Project Structure

```
dadagen/
├── dadagen-core/          # Core library
├── dadagen-macros/        # Procedural macros
├── dadagen-cli/           # Command-line interface
├── dadagen-gui/           # GUI application
├── dadagen-web/           # WASM web app
├── dadagen-python/        # Python bindings
├── .github/workflows/     # CI/CD pipelines
├── .specs/                # Feature specifications
└── docs/                  # Documentation
```

## Commit Message Guidelines

Follow conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Example:
```
feat(core): add support for weighted random selection

Implement weighted random selection for list generators.
This allows users to specify probabilities for each item.

Closes #123
```

## Review Process

1. All pull requests require review before merging
2. CI checks must pass
3. Code must follow project standards
4. Documentation must be updated
5. Tests must be included and passing

## Release Process

Releases are managed by maintainers:

1. Update version numbers in all `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Run release checks: `just release-check`
4. Create and push tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
5. CI will build and publish release artifacts

## Questions?

If you have questions, feel free to:
- Open a discussion on GitHub
- Ask in pull request comments
- Contact the maintainers

Thank you for contributing to Dadagen!
