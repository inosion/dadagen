# Dadagen - Test Data Generation Platform
# Build orchestration with Just

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
    # Install cargo tools
    cargo install cargo-watch cargo-audit cargo-deny cargo-llvm-cov --locked
    @echo "✅ Tools installed"

# Install all project dependencies
install-deps:
    @echo "🦀 Installing Rust dependencies..."
    cargo fetch
    @echo "✅ Dependencies installed"

# === Build Commands ===

# Build all components
build: build-workspace build-cli
    @echo "✅ All components built successfully"

# Build entire workspace
build-workspace:
    @echo "🦀 Building workspace..."
    cargo build --workspace --all-features
    @echo "✅ Workspace build complete"

# Build including GUI (requires WebKitGTK system libraries)
build-with-gui:
    @echo "🖥️ Building workspace with GUI..."
    cargo build --workspace --all-features
    cd dadagen-gui && cargo build --all-features
    @echo "✅ Workspace with GUI build complete"

# Build CLI in release mode
build-cli:
    @echo "🔧 Building CLI..."
    cargo build --package dadagen-cli --release
    @echo "✅ CLI build complete"

# Build for production with optimizations
build-prod: clean
    @echo "🏗️ Building for production..."
    cargo build --workspace --all-features --release
    @echo "✅ Production build complete"

# Build documentation
build-docs:
    @echo "📚 Building documentation..."
    cargo doc --no-deps --workspace --all-features
    @echo "✅ Documentation build complete"

# === Testing Commands ===

# Run all tests
test: test-workspace test-doc
    @echo "✅ All tests passed"

# Run workspace tests
test-workspace:
    @echo "🧪 Running workspace tests..."
    cargo test --workspace --all-features
    @echo "✅ Workspace tests passed"

# Run documentation tests
test-doc:
    @echo "📖 Running documentation tests..."
    cargo test --doc --workspace
    @echo "✅ Documentation tests passed"

# Run tests with coverage
test-coverage:
    @echo "📊 Running tests with coverage..."
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
    @echo "✅ Coverage report generated: lcov.info"

# Run specific crate tests
test-crate crate:
    @echo "🧪 Running tests for {{crate}}..."
    cargo test --package {{crate}} --all-features
    @echo "✅ Tests passed for {{crate}}"

# === Code Quality ===

# Run all code quality checks
quality: format lint
    @echo "✅ Code quality checks passed"

# Format all code
format:
    @echo "🎨 Formatting code..."
    cargo fmt --all
    @echo "✅ Code formatted"

# Check formatting without modifying files
format-check:
    @echo "🔍 Checking code formatting..."
    cargo fmt --all --check
    @echo "✅ Format check complete"

# Run linting
lint:
    @echo "🔍 Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
    @echo "✅ Linting complete"

# Fix automatically fixable lint issues
fix:
    @echo "🔧 Fixing lint issues..."
    cargo clippy --all-targets --all-features --fix --allow-dirty
    @echo "✅ Fixes applied"

# === Security Scanning ===

# Run comprehensive security scans
security: audit-deps
    @echo "✅ Security scans complete"

# Audit dependencies for known vulnerabilities
audit-deps:
    @echo "🔒 Auditing dependencies..."
    cargo audit
    @echo "✅ Dependency audit complete"

# Check with cargo-deny (if configured)
deny-check:
    @echo "🛡️ Running cargo-deny checks..."
    cargo deny check
    @echo "✅ Cargo-deny checks complete"

# === Development Workflows ===

# Quick development cycle (format, lint, test)
check: format lint test-workspace
    @echo "✅ Quick check complete"

# Pre-commit hook simulation
pre-commit: format-check lint test-workspace
    @echo "✅ Pre-commit checks passed"

# Watch for changes and run tests
watch:
    @echo "👀 Watching for changes..."
    cargo watch -x "test --workspace --all-features"

# Watch and run specific command
watch-cmd cmd:
    @echo "👀 Watching and running: {{cmd}}"
    cargo watch -x "{{cmd}}"

# === Clean Commands ===

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    @echo "✅ Clean complete"

# Clean and remove Cargo.lock
clean-all: clean
    @echo "🧹 Removing Cargo.lock..."
    rm -f Cargo.lock
    @echo "✅ Deep clean complete"

# === Documentation ===

# Generate and serve documentation
docs: build-docs
    @echo "📖 Opening documentation..."
    cargo doc --open --no-deps --workspace --all-features

# === Utility Commands ===

# Update all dependencies
update:
    @echo "⬆️ Updating dependencies..."
    cargo update
    @echo "✅ Dependencies updated"

# Show project status
status:
    @echo "📊 Project Status:"
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo ""
    @echo "Git status:"
    @git status --short
    @echo ""
    @echo "Workspace structure:"
    @cargo tree --depth 1

# List all workspace members
members:
    @echo "📦 Workspace Members:"
    @cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[]'

# Run benchmarks
bench:
    @echo "⚡ Running benchmarks..."
    cargo bench --workspace
    @echo "✅ Benchmarks complete"

# Check for outdated dependencies
outdated:
    @echo "📅 Checking for outdated dependencies..."
    cargo outdated --workspace
    @echo "✅ Check complete"

# === CLI Specific Commands ===

# Run the CLI
run *ARGS:
    @echo "🚀 Running dadagen CLI..."
    cargo run --package dadagen-cli -- {{ARGS}}

# Install CLI locally
install-cli:
    @echo "📦 Installing CLI..."
    cargo install --path dadagen-cli --force
    @echo "✅ CLI installed"

# === Release Commands ===

# Prepare for release (check everything)
release-check: format-check lint test security
    @echo "✅ Release checks passed"

# Build release artifacts
release-build:
    @echo "🎉 Building release artifacts..."
    cargo build --workspace --all-features --release
    @echo "✅ Release artifacts built"
    @echo "Artifacts location: target/release/"
