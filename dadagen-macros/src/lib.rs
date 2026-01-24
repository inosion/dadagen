//! # Dadagen Procedural Macros
//! 
//! This crate provides procedural macros for compile-time code generation
//! with the dadagen data generation framework.
//!
//! ## Features
//!
//! - **#[derive(DataGenerator)]** - Automatically implement generator traits
//! - **#[dadagen(...)]** - Configure field-level generation constraints
//! - **dadagen_schema!** - Define data schemas at compile time
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use dadagen_macros::DataGenerator;
//! 
//! #[derive(DataGenerator, Debug)]
//! struct User {
//!     #[dadagen(string(length = 10, charset = "alpha"))]
//!     username: String,
//!     
//!     #[dadagen(number(min = 18, max = 99))]
//!     age: u32,
//!     
//!     #[dadagen(template = "${username}@example.com")]
//!     email: String,
//! }
//! ```
//!
//! ## Attribute Reference
//!
//! ### Field Attributes
//!
//! - `#[dadagen(string(...))]` - String generation configuration
//!   - `length: usize` - Fixed string length
//!   - `min_length: usize, max_length: usize` - Variable length range
//!   - `charset: &str` - Character set: "alpha", "numeric", "alphanumeric", etc.
//!   - `case: &str` - Case transformation: "lower", "upper", "title", "mixed"
//!
//! - `#[dadagen(number(...))]` - Number generation configuration
//!   - `min: T, max: T` - Range constraints
//!   - `decimal_places: usize` - For floating point precision
//!
//! - `#[dadagen(template = "...")]` - Template-based generation
//!   - Use `${field}` to reference other fields
//!
//! - `#[dadagen(list = "...")]` - Select from predefined lists
//!   - `name: &str` - List name from list manager
//!
//! ## Debugging
//!
//! To debug macro expansion, set the `DADAGEN_MACRO_DEBUG` environment variable:
//!
//! ```bash
//! DADAGEN_MACRO_DEBUG=1 cargo build
//! ```
//!
//! Or use `cargo expand` to see the full expanded code:
//!
//! ```bash
//! cargo install cargo-expand
//! cargo expand
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod generator;
mod schema;
mod field_generators;
mod utils;

/// Derive macro to automatically implement data generator traits
/// 
/// This macro analyzes your struct definition and generates appropriate
/// data generators for each field based on type inference and attributes.
///
/// # Basic Usage
///
/// ```rust,ignore
/// use dadagen_macros::DataGenerator;
///
/// #[derive(DataGenerator)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// ```
///
/// # With Attributes
///
/// ```rust,ignore
/// #[derive(DataGenerator)]
/// struct Person {
///     #[dadagen(string(min_length = 2, max_length = 50))]
///     name: String,
///     
///     #[dadagen(number(min = 0, max = 120))]
///     age: u32,
///     
///     #[dadagen(template = "${name}@example.com")]
///     email: String,
/// }
/// ```
///
/// # Field Dependencies
///
/// Fields can reference other fields in templates:
///
/// ```rust,ignore
/// #[derive(DataGenerator)]
/// struct User {
///     user_id: u64,
///     
///     #[dadagen(template = "user_${user_id}")]
///     username: String,
/// }
/// ```
///
/// # Constraints and Validation
///
/// The macro performs compile-time validation of:
/// - Attribute syntax
/// - Type compatibility with generators
/// - Field dependency cycles
/// - Template variable references
///
/// # Generated Code
///
/// For each struct, the macro generates:
/// 1. A `{StructName}Generator` implementation
/// 2. Field-specific generator instances
/// 3. Dependency tracking
/// 4. Context-aware generation logic
///
/// # Errors
///
/// The macro will emit compilation errors for:
/// - Invalid attribute syntax
/// - Unsupported types
/// - Circular dependencies
/// - Missing field references in templates
/// - Type mismatches between generators and fields
#[proc_macro_derive(DataGenerator, attributes(dadagen))]
pub fn derive_data_generator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Check for debug mode
    if std::env::var("DADAGEN_MACRO_DEBUG").is_ok() {
        eprintln!("=== Dadagen Macro Debug ===");
        eprintln!("Input: {:#?}", input);
    }
    
    match generator::expand_generator_derive(&input) {
        Ok(tokens) => {
            if std::env::var("DADAGEN_MACRO_DEBUG").is_ok() {
                eprintln!("Generated tokens: {}", tokens);
            }
            tokens.into()
        },
        Err(err) => {
            let error = err.to_compile_error();
            if std::env::var("DADAGEN_MACRO_DEBUG").is_ok() {
                eprintln!("Error: {}", err);
            }
            error.into()
        }
    }
}

/// Function-like macro for defining data schemas at compile time
/// 
/// This macro provides a DSL-like syntax for defining complete data schemas
/// including relationships, constraints, and generation strategies.
///
/// # Usage
///
/// ```rust,ignore
/// use dadagen_macros::dadagen_schema;
///
/// dadagen_schema! {
///     User {
///         id: u64 = counter(start: 1000),
///         username: String = string(length: 10, charset: "alphanumeric"),
///         email: String = template("${username}@example.com"),
///         age: u32 = number(min: 18, max: 99),
///     }
/// }
/// ```
///
/// # Features
///
/// - Declare multiple related types
/// - Specify generation strategies inline
/// - Define cross-field dependencies
/// - Configure list-based generation
///
/// # Generated Output
///
/// The macro generates:
/// 1. Struct definitions
/// 2. Generator implementations
/// 3. Factory functions for easy instantiation
/// 4. Documentation from inline comments
#[proc_macro]
pub fn dadagen_schema(input: TokenStream) -> TokenStream {
    match schema::parse_schema(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Attribute macro for configuring struct-level generation behavior
///
/// This macro can be applied to structs to configure global generation
/// settings that apply to all fields.
///
/// # Usage
///
/// ```rust,ignore
/// use dadagen_macros::dadagen_config;
///
/// #[dadagen_config(seed = 12345, batch_size = 100)]
/// #[derive(DataGenerator)]
/// struct TestData {
///     field1: String,
///     field2: i32,
/// }
/// ```
///
/// # Parameters
///
/// - `seed: u64` - Fixed random seed for reproducible generation
/// - `batch_size: usize` - Optimize for batch generation
/// - `validate: bool` - Enable runtime validation (default: true)
#[proc_macro_attribute]
pub fn dadagen_config(attr: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just pass through - will be implemented in 5.3
    let _ = attr;
    item
}

#[cfg(test)]
mod tests {
    // Note: Proc macro tests are limited. Most testing happens in integration tests.
    // See tests/ directory for comprehensive macro expansion tests.
}
