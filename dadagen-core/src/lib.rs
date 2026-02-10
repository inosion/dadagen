//! Dadagen Core Library
//!
//! This crate provides the core functionality for the dadagen test data generation platform.
//!
//! # Overview
//!
//! Dadagen is a powerful test data generation library that allows you to:
//! - Define data schemas using a custom DSL
//! - Generate realistic test data based on templates and patterns
//! - Support complex field dependencies and relationships
//! - Integrate with various data sources and formats
//!
//! # Example
//!
//! ```rust,no_run
//! use dadagen_core::{DslGrammarParser, Context, Result};
//!
//! fn generate_data() -> Result<()> {
//!     let dsl = r#"
//!         "name": firstname + " " + lastname,
//!         "email": lowercase(firstname) + "@example.com"
//!     "#;
//!     
//!     // Parse DSL and generate data
//!     // ... implementation
//!     Ok(())
//! }
//! ```
//!
//! # Module Organization
//!
//! - `errors`: Error types and result aliases
//! - `context`: Generation context and state management
//! - `generators`: Data generator implementations
//! - `parsers`: DSL parsing and AST construction
//! - `config`: Configuration structures
//! - `utils`: Utility functions and helpers

#![allow(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

// Core modules
pub mod ast;
pub mod config;
pub mod context;
pub mod context_pool;
pub mod dependency;
pub mod errors;
pub mod generator_registry;
pub mod generator_trait;
pub mod parser;

// Generator modules
pub mod generators;

// Parser modules
pub mod parsers {
    //! DSL parsing and AST construction
    pub mod dsl;
}

// Utility modules
pub mod utils {
    //! Utility functions and helpers
    pub mod list_manager;
}

// Internal DSL module (re-exported through parsers)
mod dsl;

#[cfg(test)]
mod dsl_syntax_tests;

// Legacy modules (to be refactored)
mod address_generation;
mod common;
mod list_manager;
mod number_generation;
mod string_generation;
mod util;

// Re-export commonly used types
pub use context::{Context, ContextSnapshot, GenerationMetadata};
pub use context_pool::ContextPool;
pub use errors::{DadagenError, Result};

// Re-export parser types
pub use parsers::dsl::{DslGrammarParser, Rule};

// Re-export utility types
pub use utils::list_manager::{GLOBAL_LIST_MANAGER, ListManager};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging for the library
///
/// This should be called once at the start of your application.
/// It sets up the tracing subscriber with appropriate formatting.
///
/// # Example
///
/// ```rust,no_run
/// use dadagen_core::init_logging;
///
/// fn main() {
///     init_logging();
///     // Your application code
/// }
/// ```
pub fn init_logging() {
    use tracing_subscriber::{EnvFilter, fmt};

    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("dadagen_core=info"))
        .unwrap();

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("Dadagen Core v{} initialized", VERSION);
}
