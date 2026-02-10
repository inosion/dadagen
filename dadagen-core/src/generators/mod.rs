//! Generator trait system and implementations
//!
//! This module provides a unified interface for all data generators
//! and concrete implementations for basic types.

pub mod core;
pub mod number;
pub mod registry;
pub mod string;
pub mod template;

// Re-export commonly used types
pub use core::{BaseGenerator, ConfigurableGenerator, Generator, GeneratorMetadata};
pub use number::{
    FloatGenerator, IntegerGenerator, NumberConfig, NumberDistribution, RangeGenerator,
};
pub use string::{
    Case, CharacterSet, ListGenerator, PatternGenerator, StringConfig, StringGenerator,
};
pub use template::{CompositeGenerator, TemplateGenerator};

pub use registry::{BoxedGenerator, GeneratorFactory, GeneratorRegistry};
