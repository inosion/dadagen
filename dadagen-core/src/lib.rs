//! Dadagen Core Library
//! 
//! This crate provides the core functionality for the dadagen test data generation platform.

pub mod common;
pub mod config;
pub mod list_manager;
pub mod address_generation;
pub mod number_generation;
pub mod string_generation;
pub mod util;
pub mod dsl;
pub mod errors;
pub mod context;
pub mod generators;

// Re-export commonly used types
pub use common::{Generator, Context};
pub use errors::{DadagenError, Result};
pub use dsl::{DslParser, Rule};
pub use list_manager::{ListManager, GLOBAL_LIST_MANAGER};
