//! Generator trait system and implementations
//! 
//! This module provides a unified interface for all data generators
//! and concrete implementations for basic types.

pub mod core;
pub mod number;
pub mod string;
pub mod template;
pub mod registry;

// Re-export commonly used types
pub use core::{Generator, ConfigurableGenerator, GeneratorMetadata, BaseGenerator};
pub use number::{IntegerGenerator, FloatGenerator, RangeGenerator, NumberConfig, NumberDistribution};
pub use string::{StringGenerator, PatternGenerator, ListGenerator, StringConfig, CharacterSet, Case};
pub use template::{TemplateGenerator, CompositeGenerator};

pub use registry::{GeneratorRegistry, GeneratorFactory, BoxedGenerator};
