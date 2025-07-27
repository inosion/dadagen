//! Number generation utilities

use crate::context::Context;
use crate::errors::{Result, DadagenError};
use crate::generators::core::{Generator, ConfigurableGenerator};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::ops::Range;

/// Configuration for number generators
#[derive(Debug, Clone)]
pub struct NumberConfig {
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub decimal_places: Option<usize>,
    pub distribution: NumberDistribution,
    pub seed: Option<u64>,
}

impl Default for NumberConfig {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
            decimal_places: None,
            distribution: NumberDistribution::Uniform,
            seed: None,
        }
    }
}

/// Number distribution types
#[derive(Debug, Clone)]
pub enum NumberDistribution {
    Uniform,
    Normal { mean: f64, std_dev: f64 },
    Exponential { lambda: f64 },
}

/// Integer generator
#[derive(Debug, Clone)]
pub struct IntegerGenerator {
    name: String,
    config: NumberConfig,
    dependencies: Vec<String>,
}

impl IntegerGenerator {
    pub fn new(name: String) -> Self {
        Self {
            name,
            config: NumberConfig::default(),
            dependencies: vec![],
        }
    }
    
    pub fn with_range(mut self, min: i64, max: i64) -> Self {
        self.config.min = Some(min);
        self.config.max = Some(max);
        self
    }
    
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }
}

impl Generator<i64> for IntegerGenerator {
    fn generate(&self, _context: &Context) -> Result<i64> {
        let mut rng = match self.config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        
        let (min, max) = match (self.config.min, self.config.max) {
            (Some(min), Some(max)) => (min, max),
            (Some(min), None) => (min, i64::MAX),
            (None, Some(max)) => (i64::MIN, max),
            (None, None) => (0, 100), // Default range
        };
        
        if min >= max {
            return Err(DadagenError::GenerationError { message: format!("Invalid range: min ({}) must be less than max ({})", min, max) });
        }
        
        Ok(rng.gen_range(min..=max))
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl ConfigurableGenerator<i64, NumberConfig> for IntegerGenerator {
    fn with_config(mut self, config: NumberConfig) -> Self {
        self.config = config;
        self
    }
    
    fn config(&self) -> &NumberConfig {
        &self.config
    }
}

/// Float generator
#[derive(Debug, Clone)]
pub struct FloatGenerator {
    name: String,
    config: NumberConfig,
    dependencies: Vec<String>,
}

impl FloatGenerator {
    pub fn new(name: String) -> Self {
        Self {
            name,
            config: NumberConfig::default(),
            dependencies: vec![],
        }
    }
    
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.config.min = Some(min as i64);
        self.config.max = Some(max as i64);
        self
    }
    
    pub fn with_decimal_places(mut self, places: usize) -> Self {
        self.config.decimal_places = Some(places);
        self
    }
    
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }
}

impl Generator<f64> for FloatGenerator {
    fn generate(&self, _context: &Context) -> Result<f64> {
        let mut rng = match self.config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        
        let (min, max) = match (self.config.min, self.config.max) {
            (Some(min), Some(max)) => (min as f64, max as f64),
            (Some(min), None) => (min as f64, f64::MAX),
            (None, Some(max)) => (f64::MIN, max as f64),
            (None, None) => (0.0, 1.0), // Default range
        };
        
        if min >= max {
            return Err(DadagenError::GenerationError { message: format!("Invalid range: min ({}) must be less than max ({})", min, max) });
        }
        
        let value = rng.gen_range(min..=max);
        
        // Apply decimal places if specified
        let result = if let Some(places) = self.config.decimal_places {
            let multiplier = 10_f64.powi(places as i32);
            (value * multiplier).round() / multiplier
        } else {
            value
        };
        
        Ok(result)
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl ConfigurableGenerator<f64, NumberConfig> for FloatGenerator {
    fn with_config(mut self, config: NumberConfig) -> Self {
        self.config = config;
        self
    }
    
    fn config(&self) -> &NumberConfig {
        &self.config
    }
}

/// Range generator for creating number ranges
#[derive(Debug, Clone)]
pub struct RangeGenerator {
    name: String,
    config: NumberConfig,
    dependencies: Vec<String>,
}

impl RangeGenerator {
    pub fn new(name: String) -> Self {
        Self {
            name,
            config: NumberConfig::default(),
            dependencies: vec![],
        }
    }
    
    pub fn with_bounds(mut self, min: i64, max: i64) -> Self {
        self.config.min = Some(min);
        self.config.max = Some(max);
        self
    }
}

impl Generator<Range<i64>> for RangeGenerator {
    fn generate(&self, _context: &Context) -> Result<Range<i64>> {
        let min = self.config.min.unwrap_or(0);
        let max = self.config.max.unwrap_or(100);
        
        if min >= max {
            return Err(DadagenError::GenerationError { message: format!("Invalid range bounds: min ({}) must be less than max ({})", min, max) });
        }
        
        Ok(min..max)
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl ConfigurableGenerator<Range<i64>, NumberConfig> for RangeGenerator {
    fn with_config(mut self, config: NumberConfig) -> Self {
        self.config = config;
        self
    }
    
    fn config(&self) -> &NumberConfig {
        &self.config
    }
}
