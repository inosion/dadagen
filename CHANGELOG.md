# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Cargo workspace configuration with shared dependencies
- CI/CD pipeline with GitHub Actions
  - Format checking
  - Linting with clippy
  - Cross-platform testing (Linux, Windows, macOS)
  - Security auditing
  - Code coverage reporting
- Build orchestration with Just
- Security configuration with cargo-deny
- Comprehensive development documentation
- Contributing guidelines
- Release automation workflow

### Changed
- Enhanced workspace metadata in Cargo.toml
- Standardized formatting with rustfmt configuration
- Stricter linting rules with clippy configuration

### Fixed
- N/A

## [0.1.0] - YYYY-MM-DD

### Added
- Initial project structure
- Core library (dadagen-core)
- Procedural macros (dadagen-macros)
- CLI application (dadagen-cli)
- GUI application skeleton (dadagen-gui)
- Web application skeleton (dadagen-web)
- Python bindings skeleton (dadagen-python)

[Unreleased]: https://github.com/inosion/dadagen/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/inosion/dadagen/releases/tag/v0.1.0
