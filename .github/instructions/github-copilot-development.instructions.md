---
applyTo: "**"
---

# GitHub Copilot Instructions

## Section 1 - Development

### Spec-Driven Development Workflow

This document provides structured instructions for GitHub Copilot to maintain consistency, quality and best practice across the code base and developed applications.
Follow these guidelines to ensure high-quality code generation that aligns with project standards and spec-driven development workflow.


### Code Quality Standards

#### 1. Code Structure and Organization

**ALWAYS:**

- Write clean, readable, and maintainable code
- Use meaningful variable and function names that clearly describe their purpose
- Follow the single responsibility principle - one function, one purpose
- Keep functions small and focused (ideally under 50 lines)
- Use consistent indentation and formatting
- Add appropriate comments for complex logic, but prefer self-documenting code
- Prefer to not embed languages in other languages (e.g., avoid SQL in Rust code) (e.g. avoid CSS inside HTML files, import CSS files instead)

**EXAMPLE:**

```rust
// Good: Clear, descriptive naming
fn calculate_quantum_circuit_fidelity(
    ideal_state: &QuantumState,
    measured_state: &QuantumState,
) -> Result<f64, QuantumError> {
    validate_state_compatibility(ideal_state, measured_state)?;
    let fidelity = compute_state_overlap(ideal_state, measured_state);
    Ok(fidelity.abs().powi(2))
}

// Bad: Unclear naming and purpose
fn calc(a: &QS, b: &QS) -> Result<f64, QE> {
    let x = a.dot(b);
    Ok(x.abs().powi(2))
}
```

#### 2. Error Handling

**ALWAYS:**

- Use proper error handling with Result types in Rust, exceptions in other languages
- Provide meaningful error messages that help with debugging
- Handle errors at the appropriate level - don't ignore them
- Use custom error types for domain-specific errors

**EXAMPLE:**

```rust
#[derive(Debug, thiserror::Error)]
pub enum QuantumDeviceError {
    #[error("Device {device_id} is not available: {reason}")]
    DeviceUnavailable { device_id: String, reason: String },

    #[error("Invalid quantum circuit: {details}")]
    InvalidCircuit { details: String },

    #[error("Calibration failed for device {device_id}: {error}")]
    CalibrationFailed { device_id: String, error: String },
}
```

#### 3. Testing

**ALWAYS:**

- Write unit tests for all public functions
- Include both positive and negative test cases
- Test edge cases and error conditions
- Use descriptive test names that explain what is being tested
- Mock external dependencies in tests

#### 4. Documentation

**ALWAYS:**

- Write comprehensive documentation for public APIs
- Include examples in documentation
- Document complex algorithms and business logic
- Keep documentation up to date with code changes

### Spec-Driven Development Workflow

#### Overview

**ALWAYS follow the spec workflow for complex features:**

1. **Requirements Phase**: Create detailed requirements using EARS format
2. **Design Phase**: Develop comprehensive design documents with architecture
3. **Tasks Phase**: Break down implementation into discrete coding tasks
4. **Execution Phase**: Execute tasks one at a time with user approval

**Spec File Structure:**

```
.specs/{feature-name}/
├── requirements.md
├── design.md
└── tasks.md
└── tasks-prototype.md
```

#### Requirements Management

**ALWAYS when creating requirements:**

- Use EARS format (Easy Approach to Requirements Syntax)
- Include user stories with clear roles, features, and benefits
- Define acceptance criteria that are testable
- Consider edge cases and error scenarios
- Reference related requirements and dependencies

**TEMPLATE:**

```markdown
# Requirements Document

### Introduction

[Brief description of the feature/component and its purpose]

### Requirements

#### Requirement 1: [Clear, concise title]

**User Story:** As a [role], I want [feature], so that [benefit]

###### Acceptance Criteria

1. WHEN [event/condition] THEN [system] SHALL [response/behavior]
2. IF [precondition] THEN [system] SHALL [response/behavior]
3. THE [system] SHALL [requirement] WITHIN [time constraint]

###### Dependencies

- [List any dependencies on other requirements or systems]

###### Edge Cases

- [Describe edge cases and how they should be handled]
```

#### Design Documentation

**ALWAYS when creating design documents:**

- Start with a clear overview and architecture diagram
- Define components and their interfaces
- Specify data models and their relationships
- Include error handling strategies
- Define testing approaches
- Consider scalability and performance

**TEMPLATE:**

```markdown
# Design Document: [Feature Name]

### Overview

[High-level description of what this design addresses]

### Architecture

[Include diagrams showing system components and their relationships]

### Components and Interfaces

[Define all components and their interfaces]

### Data Models

[Define all data structures and their relationships]

### Error Handling

[Describe error handling strategy and error types]

### Testing Strategy

[Outline testing approach including unit, integration, and performance tests]
```

#### Task Management

**ALWAYS when creating task lists:**

- Break down work into discrete, manageable tasks
- Use hierarchical numbering for organization
- Include clear acceptance criteria for each task
- Reference specific requirements
- Focus on coding tasks that can be executed by developers
- Estimate complexity and dependencies

**TEMPLATE:**

```markdown
# Implementation Plan: [Feature Name]

### Task Breakdown

- [ ] 1. [Epic/Major Component Name]
  - [ ] 1.1 [Specific implementation task]
    - Create [specific deliverable]
    - Implement [specific functionality]
    - Write unit tests for [specific component]
    - _Requirements: [Reference to requirements]_
    - _Estimated effort: [S/M/L/XL]_
    - _Dependencies: [List dependencies]_
```

#### Task Execution Guidelines

**ALWAYS when executing spec tasks:**

- Read requirements.md, design.md, and tasks.md before starting
- Execute only ONE task at a time
- Stop after completing each task for user review
- Update task status using appropriate tools
- Reference specific requirements in implementation
- Never proceed to next task without explicit user approval

**EXAMPLE Task Implementation:**

```rust
// Task 2.1: Implement User model with validation
// Requirements: 1.2, 3.3
// This task focuses ONLY on the User model, not other components

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String) -> Result<Self, ValidationError> {
        let email = Email::new(email)?;
        Ok(Self {
            id: UserId::new(),
            email,
            created_at: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_with_valid_email() {
        let user = User::new("test@example.com".to_string());
        assert!(user.is_ok());
    }

    #[test]
    fn test_user_creation_with_invalid_email() {
        let user = User::new("invalid-email".to_string());
        assert!(user.is_err());
    }
}
```

### Language-Specific Guidelines

#### Rust

**ALWAYS:**

- Use `Result<T, E>` for error handling
- Prefer `&str` over `String` for function parameters when possible
- Use `#[derive(Debug)]` on structs and enums
- Implement `Display` and `Error` traits for custom error types
- Use `cargo clippy` recommendations
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)

#### Python

**ALWAYS:**

- Use type hints for all function parameters and return values
- Follow PEP 8 style guidelines
- Use dataclasses or Pydantic models for structured data
- Implement proper exception handling with custom exception classes
- Use context managers for resource management
- Write docstrings in Google or NumPy format

#### TypeScript/JavaScript

**ALWAYS:**

- Use TypeScript with strict mode enabled
- Define interfaces for all data structures
- Use async/await for asynchronous operations
- Implement proper error handling with custom error classes
- Use ESLint and Prettier for code formatting
- Follow functional programming principles where appropriate

### Architecture Patterns and Design Principles

#### SOLID Principles

**ALWAYS apply SOLID principles:**

- **Single Responsibility**: Each class/module should have one reason to change
- **Open/Closed**: Open for extension, closed for modification
- **Liskov Substitution**: Subtypes must be substitutable for their base types
- **Interface Segregation**: Clients shouldn't depend on interfaces they don't use
- **Dependency Inversion**: Depend on abstractions, not concretions

#### Domain-Driven Design (DDD)

**ALWAYS:**

- Model the business domain accurately
- Use ubiquitous language consistently
- Separate domain logic from infrastructure
- Define clear bounded contexts
- Use value objects for domain concepts

### Configuration Management

#### Environment-Based Configuration

**ALWAYS:**

- Use environment variables for configuration
- Provide sensible defaults
- Validate configuration on startup
- Use configuration structs with validation

**EXAMPLE:**

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub max_concurrent_tasks: u32,
    pub task_timeout_seconds: u64,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = Self {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL"))?,
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            max_concurrent_tasks: env::var("MAX_CONCURRENT_TASKS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidValue("MAX_CONCURRENT_TASKS"))?,
            task_timeout_seconds: env::var("TASK_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidValue("TASK_TIMEOUT_SECONDS"))?,
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        };

        config.validate()?;
        Ok(config)
    }
}
```

### Database and Data Management

#### Migration Strategy

**ALWAYS:**

- Use database migrations for schema changes
- Make migrations reversible when possible
- Test migrations on production-like data
- Version migrations clearly

#### Data Access Patterns

**ALWAYS:**

- Use repository pattern for data access
- Implement proper connection pooling
- Handle database transactions appropriately
- Use prepared statements to prevent SQL injection

### API Design Guidelines

#### RESTful API Design

**ALWAYS:**

- Follow REST conventions consistently
- Use appropriate HTTP methods and status codes
- Implement proper pagination for list endpoints
- Version your APIs explicitly
- Use consistent error response format

**EXAMPLE:**

```rust
#[derive(Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<TaskResults>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    pub request_id: String,
}
```

### Concurrency and Async Programming

#### Async Best Practices

**ALWAYS:**

- Use async/await for I/O operations
- Avoid blocking operations in async contexts
- Use appropriate synchronization primitives
- Handle cancellation gracefully
- Use timeouts for external calls

**EXAMPLE:**

```rust
use tokio::time::{timeout, Duration};
use tokio::sync::{Semaphore, RwLock};
use std::sync::Arc;

pub struct TaskExecutor {
    device_client: Arc<dyn DeviceClient + Send + Sync>,
    semaphore: Arc<Semaphore>, // Limit concurrent executions
    active_tasks: Arc<RwLock<HashMap<String, TaskHandle>>>,
}

impl TaskExecutor {
    pub async fn execute_task(&self, task: QuantumTask) -> Result<TaskResult, ExecutionError> {
        // Acquire semaphore to limit concurrency
        let _permit = self.semaphore.acquire().await
            .map_err(|_| ExecutionError::ResourceExhausted)?;

        // Execute with timeout and cancellation support
        let execution_future = self.execute_with_device(&task);
        let timeout_future = timeout(Duration::from_secs(300), execution_future);

        match timeout_future.await {
            Ok(Ok(task_result)) => Ok(task_result),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(ExecutionError::Timeout),
        }
    }
}
```

### Observability and Debugging

#### Structured Logging

**ALWAYS:**

- Use structured logging with key-value pairs
- Include correlation IDs for request tracing
- Log at appropriate levels
- Never log sensitive information
- Use consistent log formats

**EXAMPLE:**

```rust
use tracing::{info, warn, error, debug, instrument, Span};

#[instrument(
    skip(self, task),
    fields(
        task_id = %task.id,
        device_id = %task.device_id,
        user_id = %task.user_id,
        correlation_id = %Uuid::new_v4()
    )
)]
pub async fn process_task(&self, task: QuantumTask) -> Result<TaskResult, ProcessingError> {
    info!(
        task_type = %task.task_type,
        circuit_depth = task.circuit.depth(),
        "Starting task processing"
    );

    match self.execute_task(&task).await {
        Ok(result) => {
            info!("Task processing completed successfully");
            Ok(result)
        }
        Err(e) => {
            error!(error = %e, "Task processing failed");
            Err(e)
        }
    }
}
```

#### Metrics and Monitoring

**ALWAYS:**

- Implement business metrics, not just technical metrics
- Use appropriate metric types (counters, gauges, histograms)
- Add alerting for critical metrics
- Monitor error rates and latencies

### Security Considerations

**ALWAYS:**

- Validate all input data
- Use parameterized queries to prevent SQL injection
- Implement proper authentication and authorization
- Log security-relevant events
- Never log sensitive information (passwords, tokens, etc.)
- Use HTTPS for all network communications
- Follow principle of least privilege

### Performance Guidelines

**ALWAYS:**

- Consider algorithmic complexity (Big O notation)
- Use appropriate data structures for the use case
- Implement caching where beneficial
- Avoid premature optimization, but design for scalability
- Profile code to identify bottlenecks
- Use connection pooling for database operations
- Implement proper resource cleanup

### VSCode Integration

#### Project Configuration

**ALWAYS consider project configuration:**

- Check `.vscode/settings.json` for workspace-specific settings and guidelines
- Follow team standards and conventions defined in project documentation
- Reference external specifications and documentation files as needed
- Apply project-specific coding standards and formatting rules

#### VSCode Feature Integration

**ALWAYS leverage VSCode capabilities:**

- Use workspace file explorer for project navigation and file analysis
- Reference Problems panel for error and warning diagnostics
- Utilize integrated terminal for command execution and debugging
- Leverage IntelliSense and code completion for context-aware suggestions
- Use built-in Git integration for version control operations
- Consider VSCode extensions for automated workflows and enhanced functionality

#### VSCode MCP Integration

**WHEN working with Model Context Protocol in VSCode:**

- Configure MCP servers through VSCode settings or extensions
- Use `.vscode/settings.json` for workspace-specific MCP configurations
- Leverage MCP tools for enhanced context and capabilities
- Test MCP tool functionality with sample calls before relying on them
- Consider MCP servers for specialized domains (databases, APIs, documentation)
- Use appropriate MCP clients and extensions available in VSCode marketplace
- Remember MCP servers may need to be installed and configured separately

### Response Style Guidelines

**ALWAYS maintain professional AI assistant communication:**

- Talk like a human, not a bot
- Be knowledgeable but not instructive
- Stay decisive, precise, and clear
- Be supportive, not authoritative
- Use positive, optimistic language
- Keep responses concise and actionable
- Avoid repetition and unnecessary verbosity
- Focus on solutions, not problems

### Development Workflow

#### Pre-commit Hooks

**ALWAYS set up pre-commit hooks:**

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.4.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-added-large-files

  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --all --
        language: system
        types: [rust]

      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --all-targets --all-features -- -D warnings
        language: system
        types: [rust]
        pass_filenames: false
```

#### Code Review Guidelines

**ALWAYS ensure code meets these criteria:**

- **Functionality**: Code works correctly and handles edge cases
- **Readability**: Code is easy to understand and well-documented
- **Maintainability**: Code follows established patterns and is easy to modify
- **Performance**: Code is reasonably efficient and scalable
- **Security**: Code follows security best practices
- **Testing**: Code includes appropriate tests
- **Error Handling**: Errors are handled gracefully with meaningful messages
- **Logging**: Important events are logged appropriately
- **Standards**: Code follows project coding standards and conventions

#### Developer Principles

- Prefer composition over inheritance
- Write code that tells a story - in should be readable like prose
- Consider the next developer who will maintain your code
- When in doubt, choose simplicity over cleverness
- Optimise for readability first, performance second
- Always think about error cases and how to handle them gracefully
- Document the "why", not just he "what"
- Design for failure - assume things will go wrong
- Measure everything - you can't improve what you don't measure
- Automate repetitive tasks - reduce human error
- Plan for change - requirements will evolve
- Embrace feedback loops - continuous improvement is key

This comprehensive instruction set ensures GitHub Copilot provides consistent, high quality code generation that follows established software engineering best practices and maintains professional developement workflow patterns.

## Section 2 - Non Functionals

### User Accessibility
- Ensure compliance with WCAG (Web Content Accessibility Guidelines).
- Provide keyboard navigation and screen reader support.
- Use semantic HTML and ARIA roles where applicable.
- Test accessibility using automated tools and manual audits.

### Compliance and Regulatory Requirements
- Adhere to GDPR, CCPA, or other relevant data protection regulations.
- Ensure encryption for sensitive data in transit and at rest.
- Maintain audit logs for critical operations.
- Regularly review and update compliance documentation.

### Licensing and Intellectual Property (IP)
- Use only approved open-source libraries with compatible licenses.
- Document third-party dependencies and their licenses.
- Avoid including proprietary or confidential code in public repositories.
- Ensure proper attribution for external code or assets.

### Energy Efficiency / Sustainability
- Optimize code to reduce computational overhead.
- Minimize unnecessary API calls and database queries.
- Use efficient algorithms and data structures.
- Consider energy-efficient hosting and deployment options.

### Dependency Management
- Use dependency management tools (e.g., npm, pip, cargo, Maven) to track libraries.
- Regularly update dependencies to patch vulnerabilities.
- Avoid unnecessary dependencies to reduce attack surface.
- Lock dependency versions to ensure build consistency.

### Infrastructure and Deployment Standards
- Use Infrastructure as Code (IaC) tools (e.g., Pulumi, Ansible).
- Ensure environments are reproducible and consistent.
- Automate deployment pipelines with CI/CD tools.
- Monitor infrastructure for performance and availability.

### Service Level Objectives (SLOs) and SLAs
- Define measurable SLOs for uptime, latency, and error rates.
- Monitor adherence to SLAs using observability tools.
- Establish escalation procedures for SLA breaches.
- Regularly review and update SLOs based on user needs.

## Section 3 - DevSecOps Tooling

### Build Orchestration with Just

#### Overview

**ALWAYS use Just as the primary build orchestrator** for poly-language repositories or complex build workflows. Just provides a modern, cross-platform alternative to Make with better syntax and more reliable execution across different environments.

**Key Principles:**
- Use Just for coordinating disparate tools (cargo, npm, python, etc.)
- Avoid npm scripts for complex workflows - use Just instead
- Define clear, composable tasks that can be run independently
- Use Just for file copying, environment setup, and cross-language builds
- Keep justfiles readable and well-documented

#### Justfile Structure

**ALWAYS structure justfiles with clear sections and documentation:**

```just
# Swillow SysML2 Platform - Build Orchestration
# This justfile coordinates Rust, WASM, TypeScript, and Python components

# Default recipe lists available commands
default:
    @just --list

# === Development Environment Setup ===

# Set up the complete development environment
setup: install-tools install-deps
    @echo "✅ Development environment ready"

# Install required development tools
install-tools:
    @echo "📦 Installing development tools..."
    # Install Rust toolchain components
    rustup component add rustfmt clippy
    rustup target add wasm32-unknown-unknown
    # Install WASM tools
    cargo install wasm-pack trunk
    # Install security tools
    cargo install cargo-audit cargo-deny
    # Install Python tools if needed
    pip install --upgrade pip black isort mypy
    @echo "✅ Tools installed"

# Install all project dependencies
install-deps: install-rust-deps install-wasm-deps install-python-deps
    @echo "✅ All dependencies installed"

# Install Rust dependencies
install-rust-deps:
    @echo "🦀 Installing Rust dependencies..."
    cargo fetch
    @echo "✅ Rust dependencies ready"

# Install WASM dependencies
install-wasm-deps:
    @echo "🕸️ Installing WASM dependencies..."
    cd web-client && wasm-pack build --target web
    @echo "✅ WASM dependencies ready"

# Install Python dependencies (if applicable)
install-python-deps:
    @echo "🐍 Installing Python dependencies..."
    pip install -r requirements.txt 2>/dev/null || echo "No Python requirements found"
    @echo "✅ Python dependencies ready"

# === Build Commands ===

# Build all components
build: build-rust build-wasm build-docs
    @echo "✅ All components built successfully"

# Build Rust backend services
build-rust:
    @echo "🦀 Building Rust components..."
    cargo build --release
    @echo "✅ Rust build complete"

# Build WASM frontend components
build-wasm:
    @echo "🕸️ Building WASM components..."
    cd web-client && wasm-pack build --target web --release
    cd vscode-extension && wasm-pack build --target nodejs --release
    @echo "✅ WASM build complete"

# Build documentation
build-docs:
    @echo "📚 Building documentation..."
    cargo doc --no-deps --workspace
    @echo "✅ Documentation build complete"

# === Testing Commands ===

# Run all tests
test: test-rust test-integration test-security
    @echo "✅ All tests passed"

# Run Rust unit tests
test-rust:
    @echo "🧪 Running Rust tests..."
    cargo test --workspace
    @echo "✅ Rust tests passed"

# Run integration tests
test-integration:
    @echo "🔗 Running integration tests..."
    cargo test --test integration -- --test-threads=1
    @echo "✅ Integration tests passed"

# Run security tests
test-security: audit-deps scan-vulnerabilities
    @echo "✅ Security tests passed"

# === Code Quality ===

# Run all code quality checks
quality: format lint security-scan
    @echo "✅ Code quality checks passed"

# Format all code
format:
    @echo "🎨 Formatting code..."
    cargo fmt --all
    # Format Python if present
    find . -name "*.py" -exec black {} + 2>/dev/null || true
    find . -name "*.py" -exec isort {} + 2>/dev/null || true
    @echo "✅ Code formatted"

# Run linting
lint:
    @echo "🔍 Running linters..."
    cargo clippy --all-targets --all-features -- -D warnings
    # Lint Python if present
    find . -name "*.py" -exec mypy {} + 2>/dev/null || true
    @echo "✅ Linting complete"

# === Security Scanning ===

# Run comprehensive security scans
security-scan: audit-deps scan-vulnerabilities check-licenses
    @echo "✅ Security scans complete"

# Audit dependencies for known vulnerabilities
audit-deps:
    @echo "🔒 Auditing dependencies..."
    cargo audit
    @echo "✅ Dependency audit complete"

# Scan for vulnerabilities using cargo-deny
scan-vulnerabilities:
    @echo "🛡️ Scanning for vulnerabilities..."
    cargo deny check
    @echo "✅ Vulnerability scan complete"

# Check license compatibility
check-licenses:
    @echo "📄 Checking licenses..."
    cargo deny check licenses
    @echo "✅ License check complete"

# === Development Workflows ===

# Start development environment
dev: setup build-wasm
    @echo "🚀 Starting development environment..."
    # Start backend in background
    cargo run --bin api-server &
    # Start frontend development server
    cd web-client && trunk serve --open
    @echo "✅ Development environment running"

# Quick development cycle (format, lint, test)
check: format lint test-rust
    @echo "✅ Quick check complete"

# Pre-commit hook simulation
pre-commit: format lint test-rust security-scan
    @echo "✅ Pre-commit checks passed"

# === Deployment Commands ===

# Build for production deployment
build-prod: clean build test security-scan
    @echo "🏗️ Building for production..."
    # Create deployment artifacts
    mkdir -p dist
    cp target/release/api-server dist/
    cp -r web-client/dist dist/web-client
    @echo "✅ Production build complete"

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    rm -rf web-client/dist
    rm -rf vscode-extension/dist
    rm -rf dist
    @echo "✅ Clean complete"

# === Documentation ===

# Generate and serve documentation
docs: build-docs
    @echo "📖 Serving documentation..."
    cargo doc --open --no-deps --workspace

# Generate API documentation
api-docs:
    @echo "📊 Generating API documentation..."
    # Add API doc generation here
    @echo "✅ API documentation generated"

# === Utility Commands ===

# Copy WASM artifacts to appropriate locations
copy-wasm-artifacts:
    @echo "📋 Copying WASM artifacts..."
    # Copy web client WASM to static assets
    cp web-client/pkg/* assets/wasm/ 2>/dev/null || mkdir -p assets/wasm && cp web-client/pkg/* assets/wasm/
    # Copy VSCode extension WASM to extension directory
    cp vscode-extension/pkg/* vscode-extension/out/ 2>/dev/null || mkdir -p vscode-extension/out && cp vscode-extension/pkg/* vscode-extension/out/
    @echo "✅ WASM artifacts copied"

# Update all dependencies
update-deps:
    @echo "⬆️ Updating dependencies..."
    cargo update
    # Update Python deps if present
    pip list --outdated 2>/dev/null || true
    @echo "✅ Dependencies updated"

# Show project status
status:
    @echo "📊 Project Status:"
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo "Git status:"
    @git status --short
    @echo "Dependencies status:"
    @cargo tree --depth 1
```

#### Integration with Language-Specific Tools

**ALWAYS coordinate language-specific tools through Just:**

```just
# Example: Rust + WASM + TypeScript coordination
build-full-stack:
    # Build Rust backend
    cargo build --release
    # Build WASM for web
    cd web-client && wasm-pack build --target web
    # Generate TypeScript bindings
    wasm-pack build --target nodejs --out-dir ../ts-bindings
    # Copy artifacts to appropriate locations
    just copy-wasm-artifacts
    @echo "✅ Full stack build complete"

# Example: Cross-platform testing
test-all-platforms:
    # Test Rust components
    cargo test --workspace
    # Test WASM in browser environment
    cd web-client && wasm-pack test --headless --firefox
    # Test TypeScript integration
    cd ts-bindings && npm test
    @echo "✅ All platform tests passed"
```

### Security Tooling

#### Dependency Security

**ALWAYS implement comprehensive dependency security:**

```toml
# deny.toml - Cargo deny configuration
[advisories]
db-path = "~/.cargo/advisory-db"
db-urls = ["https://github.com/rustsec/advisory-db"]
vulnerability = "deny"
unmaintained = "warn"
yanked = "warn"
notice = "warn"

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
]
deny = [
    "GPL-2.0",
    "GPL-3.0",
    "AGPL-1.0",
    "AGPL-3.0",
]

[bans]
multiple-versions = "warn"
wildcards = "allow"
highlight = "all"

[[bans.skip]]
name = "windows-sys"  # Common in Rust ecosystem

[sources]
unknown-registry = "warn"
unknown-git = "warn"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

#### Security Scanning Integration

**ALWAYS integrate security scanning into workflows:**

```just
# Comprehensive security workflow
security-full: audit-deps scan-code check-secrets validate-configs
    @echo "✅ Full security scan complete"

# Code security scanning
scan-code:
    @echo "🔍 Scanning code for security issues..."
    # Use semgrep for static analysis
    semgrep --config=auto src/
    # Use clippy with security-focused lints
    cargo clippy -- -W clippy::all -W clippy::pedantic
    @echo "✅ Code scan complete"

# Check for secrets in code
check-secrets:
    @echo "🔐 Checking for secrets..."
    # Use git-secrets or similar tool
    git secrets --scan
    @echo "✅ Secret scan complete"

# Validate configuration files
validate-configs:
    @echo "⚙️ Validating configurations..."
    # Validate YAML files
    find . -name "*.yml" -o -name "*.yaml" | xargs yamllint
    # Validate JSON files
    find . -name "*.json" | xargs jq empty
    @echo "✅ Configuration validation complete"
```

### Continuous Integration Integration

#### GitHub Actions with Just

**ALWAYS use Just in CI/CD pipelines:**

```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline

on: [push, pull_request]

jobs:
  security-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install Just
        uses: extractions/setup-just@v1
      - name: Install dependencies
        run: just install-tools
      - name: Security scan
        run: just security-scan

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install Just
        uses: extractions/setup-just@v1
      - name: Setup environment
        run: just setup
      - name: Run tests
        run: just test

  build:
    runs-on: ubuntu-latest
    needs: [security-scan, test]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install Just
        uses: extractions/setup-just@v1
      - name: Build for production
        run: just build-prod
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: production-build
          path: dist/
```

### Development Environment Standards

#### Local Development Setup

**ALWAYS provide comprehensive local setup:**

```just
# Complete development environment initialization
init-dev: install-tools setup-git-hooks setup-ide create-configs
    @echo "🎉 Development environment initialized"

# Set up Git hooks
setup-git-hooks:
    @echo "🪝 Setting up Git hooks..."
    # Install pre-commit hook
    echo '#!/bin/sh\nexec just pre-commit' > .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    # Install pre-push hook
    echo '#!/bin/sh\nexec just test' > .git/hooks/pre-push
    chmod +x .git/hooks/pre-push
    @echo "✅ Git hooks configured"

# Set up IDE configurations
setup-ide:
    @echo "🔧 Setting up IDE configurations..."
    # Create .vscode/settings.json if it doesn't exist
    mkdir -p .vscode
    # Copy IDE configurations
    cp templates/vscode-settings.json .vscode/settings.json 2>/dev/null || true
    @echo "✅ IDE configured"

# Create necessary configuration files
create-configs:
    @echo "⚙️ Creating configuration files..."
    # Create .env.example if it doesn't exist
    touch .env.example
    # Create logging configuration
    touch logging.toml
    @echo "✅ Configuration files created"
```

#### Environment Validation

**ALWAYS validate development environment:**

```just
# Validate development environment
validate-env:
    @echo "🔍 Validating development environment..."
    # Check Rust installation
    rustc --version || (echo "❌ Rust not installed" && exit 1)
    # Check required tools
    cargo --version || (echo "❌ Cargo not available" && exit 1)
    wasm-pack --version || (echo "❌ wasm-pack not installed" && exit 1)
    # Check environment variables
    test -f .env || echo "⚠️ .env file not found - using defaults"
    @echo "✅ Environment validation complete"

# Health check for running services
health-check:
    @echo "💊 Running health checks..."
    # Check if API server is running
    curl -f http://localhost:8080/health || echo "⚠️ API server not responding"
    # Check database connectivity
    # Add database health check here
    @echo "✅ Health checks complete"
```

### Monitoring and Observability

#### Development Monitoring

**ALWAYS include development monitoring:**

```just
# Start monitoring stack for development
monitor-dev:
    @echo "📊 Starting development monitoring..."
    # Start metrics collection (if using Prometheus)
    docker run -d --name prometheus -p 9090:9090 prom/prometheus
    # Start log aggregation (if using ELK)
    # Add monitoring setup here
    @echo "✅ Development monitoring started"

# Collect development metrics
metrics:
    @echo "📈 Collecting metrics..."
    # Build time metrics
    time just build
    # Test coverage metrics
    cargo tarpaulin --out Html
    # Performance metrics
    cargo bench
    @echo "✅ Metrics collected"
```

### Documentation and Knowledge Management

#### Documentation Generation

**ALWAYS automate documentation generation:**

```just
# Generate comprehensive documentation
docs-full: build-docs api-docs architecture-docs
    @echo "📚 Complete documentation generated"

# Generate architecture documentation
architecture-docs:
    @echo "🏗️ Generating architecture documentation..."
    # Use tools like mdbook or similar
    mdbook build docs/architecture
    @echo "✅ Architecture documentation ready"

# Generate deployment documentation
deployment-docs:
    @echo "🚀 Generating deployment documentation..."
    # Document deployment procedures
    mdbook build docs/deployment
    @echo "✅ Deployment documentation ready"
```

This comprehensive DevSecOps tooling section ensures consistent, secure, and efficient development workflows using Just as the central orchestrator for all build and development operations.


