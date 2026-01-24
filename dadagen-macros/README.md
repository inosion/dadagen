# Dadagen Procedural Macros

This crate provides procedural macros for the dadagen data generation framework, enabling compile-time code generation for data generators.

## Overview

The dadagen-macros crate offers:

- **Derive macros** for automatic generator implementation
- **Attribute macros** for field-level configuration
- **Function-like macros** for schema definition
- **Compile-time validation** of generator configurations
- **Debugging utilities** for macro development

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dadagen-macros = "0.1"
dadagen-core = "0.1"
```

## Usage

### Basic Derive Macro

```rust
use dadagen_macros::DataGenerator;

#[derive(DataGenerator, Debug)]
struct User {
    username: String,
    age: u32,
    email: String,
}
```

### With Attributes

```rust
use dadagen_macros::DataGenerator;

#[derive(DataGenerator, Debug)]
struct User {
    #[dadagen(string(length = 10, charset = "alphanumeric"))]
    username: String,
    
    #[dadagen(number(min = 18, max = 99))]
    age: u32,
    
    #[dadagen(template = "${username}@example.com")]
    email: String,
}
```

### Schema Macro

```rust
use dadagen_macros::dadagen_schema;

dadagen_schema! {
    User {
        id: u64 = counter(start: 1000),
        username: String = string(length: 10),
        email: String = template("${username}@example.com"),
    }
}
```

## Attribute Reference

### String Generation

```rust
#[dadagen(string(
    length = 10,              // Fixed length
    min_length = 5,           // Or variable length range
    max_length = 20,
    charset = "alphanumeric", // alpha, numeric, alphanumeric, hex, ascii
    case = "lower",           // lower, upper, title, mixed
))]
field_name: String
```

### Number Generation

```rust
#[dadagen(number(
    min = 0,
    max = 100,
    decimal_places = 2,  // For floats
))]
field_name: i32
```

### Template Generation

```rust
#[dadagen(template = "${field1}-${field2}")]
field_name: String
```

### List Selection

```rust
#[dadagen(list = "cities")]
city: String
```

### Counter Generation

```rust
#[dadagen(counter(start = 1000, step = 1))]
id: u64
```

## Debugging

### Environment Variables

Set `DADAGEN_MACRO_DEBUG=1` to enable debug output:

```bash
DADAGEN_MACRO_DEBUG=1 cargo build
```

### Cargo Expand

Use `cargo expand` to view the fully expanded macro code:

```bash
cargo install cargo-expand
cargo expand --lib
```

### IDE Support

Most IDEs with rust-analyzer will show macro expansions inline. Use the "Expand Macro Recursively" command in VS Code.

## Architecture

### Crate Structure

```
dadagen-macros/
├── src/
│   ├── lib.rs              # Main macro definitions
│   ├── generator.rs        # Derive macro implementation
│   ├── schema.rs           # Schema macro implementation
│   ├── field_generators.rs # Field-level generator logic
│   └── utils.rs            # Debugging and utility functions
├── tests/
│   └── integration.rs      # Integration tests
└── Cargo.toml
```

### Dependencies

- **syn** (v2.0+): AST parsing with full syntax support
- **quote**: Code generation and token manipulation
- **proc-macro2**: Proc macro runtime support
- **darling**: Attribute parsing helper
- **thiserror**: Error handling

### Dev Dependencies

- **trybuild**: Compile-fail testing
- **macrotest**: Macro expansion snapshot testing
- **dadagen-core**: For integration testing

## Development

### Running Tests

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test integration

# All tests
cargo test
```

### Testing Macro Expansion

Create test cases in `tests/` using the `trybuild` pattern:

```rust
#[test]
fn test_macro_expansion() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/fail/*.rs");
}
```

### Code Quality

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Check documentation
cargo doc --no-deps --open
```

## Roadmap

### Implemented (Task 5.1)
- ✅ Macro crate setup with proc-macro configuration
- ✅ syn, quote, and proc-macro2 dependencies
- ✅ Basic derive macro skeleton
- ✅ Debugging and testing utilities

### Planned (Tasks 5.2-5.4)
- [ ] Complete struct field analysis
- [ ] Type mapping and validation
- [ ] Code generation for all generator types
- [ ] Generic struct support
- [ ] Nested struct handling
- [ ] Custom attribute parsing
- [ ] Comprehensive test suite

## Contributing

Contributions welcome! When adding new macro features:

1. Add tests in `tests/`
2. Update attribute documentation in `lib.rs`
3. Add examples to this README
4. Ensure all clippy lints pass

## License

MIT OR Apache-2.0

## Links

- [Main Repository](https://github.com/inosion/dadagen)
- [Dadagen Core](../dadagen-core)
- [Documentation](https://docs.rs/dadagen-macros)
