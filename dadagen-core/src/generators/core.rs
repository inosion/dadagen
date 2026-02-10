//! Core generator trait and base functionality

use crate::context::Context;
use crate::errors::Result;
use std::fmt::Debug;

/// Core trait for all data generators
pub trait Generator<T>: Debug + Send + Sync
where
    T: Clone + Send + Sync + 'static,
{
    /// Generate a value using the provided context
    fn generate(&self, context: &Context) -> Result<T>;

    /// Get the list of field dependencies for this generator
    fn dependencies(&self) -> Vec<String>;

    /// Get the name/identifier for this generator
    fn name(&self) -> &str;

    /// Get metadata about this generator
    fn metadata(&self) -> GeneratorMetadata {
        GeneratorMetadata {
            name: self.name().to_string(),
            generator_type: std::any::type_name::<Self>().to_string(),
            dependencies: self.dependencies(),
        }
    }
}

/// Metadata about a generator
#[derive(Debug, Clone)]
pub struct GeneratorMetadata {
    pub name: String,
    pub generator_type: String,
    pub dependencies: Vec<String>,
}

/// Trait for generators that can be configured with constraints
pub trait ConfigurableGenerator<T, C>: Generator<T>
where
    T: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Apply configuration/constraints to the generator
    fn with_config(self, config: C) -> Self;

    /// Get the current configuration
    fn config(&self) -> &C;
}

/// Base generator for simple value generation
#[derive(Debug, Clone)]
pub struct BaseGenerator<T> {
    name: String,
    generate_fn: fn(&Context) -> Result<T>,
    dependencies: Vec<String>,
}

impl<T> BaseGenerator<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(
        name: String,
        generate_fn: fn(&Context) -> Result<T>,
        dependencies: Vec<String>,
    ) -> Self {
        Self {
            name,
            generate_fn,
            dependencies,
        }
    }
}

impl<T> Generator<T> for BaseGenerator<T>
where
    T: Clone + Send + Sync + 'static + std::fmt::Debug,
{
    fn generate(&self, context: &Context) -> Result<T> {
        (self.generate_fn)(context)
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    fn name(&self) -> &str {
        &self.name
    }
}
