//! Error handling for dadagen
//!
//! This module defines all error types used throughout the dadagen library.
//! All errors implement the [`std::error::Error`] trait via [`thiserror`].

use std::path::PathBuf;

/// The main error type for dadagen operations
#[derive(Debug, thiserror::Error)]
pub enum DadagenError {
    /// Error parsing DSL syntax
    #[error("Parse error: {message} at line {line}, column {column}")]
    ParseError {
        /// Description of the parse error
        message: String,
        /// Line number where the error occurred
        line: usize,
        /// Column number where the error occurred
        column: usize,
    },

    /// Error during data generation
    #[error("Generation error: {message}")]
    GenerationError {
        /// Description of the generation error
        message: String,
    },

    /// File operation error
    #[error("File operation error: {0}")]
    FileError(#[from] std::io::Error),

    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigError {
        /// Description of the configuration error
        message: String,
    },

    /// Circular dependency detected in field definitions
    #[error("Circular dependency detected: {field} depends on itself (cycle: {cycle})")]
    CircularDependencyError {
        /// The field that has a circular dependency
        field: String,
        /// The dependency cycle path
        cycle: String,
    },

    /// List manager error
    #[error("List manager error: {message}")]
    ListManagerError {
        /// Description of the list manager error
        message: String,
    },

    /// Context operation error
    #[error("Context error: {message}")]
    ContextError {
        /// Description of the context error
        message: String,
    },

    /// CSV parsing error
    #[error("CSV parsing error: {0}")]
    CsvError(#[from] csv::Error),

    /// YAML parsing error
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Field not found in context
    #[error("Field '{field}' not found in context")]
    FieldNotFound {
        /// The field name that was not found
        field: String,
    },

    /// Type mismatch error
    #[error("Type mismatch for field '{field}': expected {expected}, got {actual}")]
    TypeMismatch {
        /// The field name
        field: String,
        /// Expected type
        expected: String,
        /// Actual type
        actual: String,
    },

    /// Invalid generator configuration
    #[error("Invalid generator configuration for '{generator}': {reason}")]
    InvalidGeneratorConfig {
        /// Generator name
        generator: String,
        /// Reason for invalidity
        reason: String,
    },

    /// Resource not found
    #[error("Resource not found: {path}")]
    ResourceNotFound {
        /// Path to the resource
        path: PathBuf,
    },

    /// Invalid range specified
    #[error("Invalid range: {reason}")]
    InvalidRange {
        /// Reason why the range is invalid
        reason: String,
    },

    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingRequiredField {
        /// The required field that is missing
        field: String,
    },
}

/// A specialized [`Result`](std::result::Result) type for dadagen operations
pub type Result<T> = std::result::Result<T, DadagenError>;

impl DadagenError {
    /// Create a parse error
    #[must_use]
    pub fn parse_error(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::ParseError {
            message: message.into(),
            line,
            column,
        }
    }

    /// Create a generation error
    #[must_use]
    pub fn generation_error(message: impl Into<String>) -> Self {
        Self::GenerationError {
            message: message.into(),
        }
    }

    /// Create a configuration error
    #[must_use]
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }

    /// Create a circular dependency error
    #[must_use]
    pub fn circular_dependency(field: impl Into<String>, cycle: impl Into<String>) -> Self {
        Self::CircularDependencyError {
            field: field.into(),
            cycle: cycle.into(),
        }
    }

    /// Create a list manager error
    #[must_use]
    pub fn list_manager_error(message: impl Into<String>) -> Self {
        Self::ListManagerError {
            message: message.into(),
        }
    }

    /// Create a context error
    #[must_use]
    pub fn context_error(message: impl Into<String>) -> Self {
        Self::ContextError {
            message: message.into(),
        }
    }

    /// Create a field not found error
    #[must_use]
    pub fn field_not_found(field: impl Into<String>) -> Self {
        Self::FieldNotFound {
            field: field.into(),
        }
    }

    /// Create a type mismatch error
    #[must_use]
    pub fn type_mismatch(
        field: impl Into<String>,
        expected: impl Into<String>,
        actual: impl Into<String>,
    ) -> Self {
        Self::TypeMismatch {
            field: field.into(),
            expected: expected.into(),
            actual: actual.into(),
        }
    }
}
