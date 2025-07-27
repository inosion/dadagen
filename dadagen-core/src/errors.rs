//! Error handling for dadagen

use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum DadagenError {
    #[error("Parse error: {message} at line {line}, column {column}")]
    ParseError { 
        message: String, 
        line: usize, 
        column: usize 
    },
    
    #[error("Generation error: {message}")]
    GenerationError { 
        message: String 
    },
    
    #[error("File operation error: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("Configuration error: {message}")]
    ConfigError { 
        message: String 
    },
    
    #[error("Dependency resolution error: circular dependency detected in {field}")]
    CircularDependencyError { 
        field: String 
    },

    #[error("List manager error: {message}")]
    ListManagerError {
        message: String
    },

    #[error("Context error: {message}")]
    ContextError {
        message: String
    },

    #[error("CSV parsing error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, DadagenError>;
