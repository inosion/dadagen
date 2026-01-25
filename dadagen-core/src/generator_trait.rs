//! Redesigned Generator Trait System
//!
//! This module provides a type-safe, composable generator trait system that
//! integrates with the AST and supports constraint-based generation.

use crate::ast::*;
use crate::context::Context;
use crate::errors::{DadagenError, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Core trait for all data generators
/// 
/// Generators produce values based on configurations from the AST and runtime context.
/// They are designed to be composable, testable, and type-safe.
pub trait DataGenerator: Debug + Send + Sync {
    /// Generate a value as a string (unified output format)
    fn generate(&self, context: &Context) -> Result<String>;
    
    /// Get the list of field dependencies for this generator
    /// Returns field names that must be generated before this one
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
    
    /// Get the generator type identifier
    fn generator_type(&self) -> GeneratorType;
    
    /// Validate generator configuration
    /// Called during AST construction to catch errors early
    fn validate(&self) -> Result<()> {
        Ok(())
    }
    
    /// Clone the generator as a trait object
    fn clone_box(&self) -> Box<dyn DataGenerator>;
}

/// Generator type identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneratorType {
    String,
    Number,
    Boolean,
    DateTime,
    Date,
    Time,
    Choice,
    List,
    Template,
    Regex,
    Counter,
    Gender,
    Name,
    Address,
}

impl std::fmt::Display for GeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorType::String => write!(f, "string"),
            GeneratorType::Number => write!(f, "number"),
            GeneratorType::Boolean => write!(f, "boolean"),
            GeneratorType::DateTime => write!(f, "datetime"),
            GeneratorType::Date => write!(f, "date"),
            GeneratorType::Time => write!(f, "time"),
            GeneratorType::Choice => write!(f, "choice"),
            GeneratorType::List => write!(f, "list"),
            GeneratorType::Template => write!(f, "template"),
            GeneratorType::Regex => write!(f, "regex"),
            GeneratorType::Counter => write!(f, "counter"),
            GeneratorType::Gender => write!(f, "gender"),
            GeneratorType::Name => write!(f, "name"),
            GeneratorType::Address => write!(f, "address"),
        }
    }
}

// ============================================================================
// Basic Type Generators
// ============================================================================

/// String generator with constraint support
#[derive(Debug, Clone)]
pub struct StringDataGenerator {
    config: StringGenerator,
}

impl StringDataGenerator {
    pub fn new(config: StringGenerator) -> Self {
        Self { config }
    }
    
    pub fn from_ast(ast: &StringGenerator) -> Result<Self> {
        let gen = Self::new(ast.clone());
        gen.validate()?;
        Ok(gen)
    }
}

impl DataGenerator for StringDataGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        use rand::seq::SliceRandom;
        
        let mut rng = StdRng::from_entropy();
        
        // Determine length
        let length = match (self.config.length, self.config.min_length, self.config.max_length) {
            (Some(len), _, _) => len,
            (None, Some(min), Some(max)) => {
                if min > max {
                    return Err(DadagenError::ValidationError {
                        message: format!("min_length ({}) cannot be greater than max_length ({})", min, max),
                    });
                }
                rng.gen_range(min..=max)
            }
            (None, Some(min), None) => rng.gen_range(min..=(min + 10)),
            (None, None, Some(max)) => rng.gen_range(1..=max),
            (None, None, None) => 10, // Default length
        };
        
        // Get character set
        let chars: Vec<char> = match &self.config.charset {
            CharacterSet::Alpha => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            CharacterSet::Numeric => "0123456789".chars().collect(),
            CharacterSet::AlphaNumeric => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect(),
            CharacterSet::Hex => "0123456789ABCDEF".chars().collect(),
            CharacterSet::Ascii => (32u8..=126u8).map(|b| b as char).collect(),
            CharacterSet::Base64 => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().collect(),
        };
        
        if chars.is_empty() {
            return Err(DadagenError::GenerationError {
                message: "Character set is empty".to_string(),
            });
        }
        
        // Generate string
        let mut result: String = (0..length)
            .map(|_| chars.choose(&mut rng).unwrap())
            .collect();
        
        // Apply case transformation
        result = match self.config.case {
            Case::Lower => result.to_lowercase(),
            Case::Upper => result.to_uppercase(),
            Case::Title => {
                let mut chars: Vec<char> = result.chars().collect();
                if let Some(first) = chars.get_mut(0) {
                    *first = first.to_uppercase().next().unwrap();
                }
                chars.into_iter().collect()
            }
            Case::Mixed => result,
        };
        
        Ok(result)
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::String
    }
    
    fn validate(&self) -> Result<()> {
        // Validate length constraints
        if let (Some(min), Some(max)) = (self.config.min_length, self.config.max_length) {
            if min > max {
                return Err(DadagenError::ValidationError {
                    message: format!("min_length ({}) cannot be greater than max_length ({})", min, max),
                });
            }
        }
        
        if let Some(_length) = self.config.length {
            if self.config.min_length.is_some() || self.config.max_length.is_some() {
                return Err(DadagenError::ValidationError {
                    message: "Cannot specify both 'length' and 'min_length/max_length'".to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(self.clone())
    }
}

/// Number generator with constraint support
#[derive(Debug, Clone)]
pub struct NumberDataGenerator {
    config: NumberGenerator,
}

impl NumberDataGenerator {
    pub fn new(config: NumberGenerator) -> Self {
        Self { config }
    }
    
    pub fn from_ast(ast: &NumberGenerator) -> Result<Self> {
        let gen = Self::new(ast.clone());
        gen.validate()?;
        Ok(gen)
    }
}

impl DataGenerator for NumberDataGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        use rand_distr::{Normal, Exp, Distribution as RandDist};
        
        let mut rng = match self.config.seed {
            Some(seed) => StdRng::seed_from_u64(seed as u64),
            None => StdRng::from_entropy(),
        };
        
        let (min, max) = (
            self.config.min.unwrap_or(0.0),
            self.config.max.unwrap_or(100.0),
        );
        
        if min >= max {
            return Err(DadagenError::ValidationError {
                message: format!("min ({}) must be less than max ({})", min, max),
            });
        }
        
        // Generate based on distribution
        let value: f64 = match &self.config.distribution {
            crate::ast::Distribution::Uniform => {
                if self.config.decimal_places.is_some() && self.config.decimal_places.unwrap() > 0 {
                    rng.gen_range(min as f64..=max as f64)
                } else {
                    rng.gen_range(min as i64..=max as i64) as f64
                }
            }
            crate::ast::Distribution::Normal { mean, std_dev } => {
                let normal = Normal::new(*mean, *std_dev).map_err(|e| DadagenError::GenerationError {
                    message: format!("Failed to create normal distribution: {}", e),
                })?;
                normal.sample(&mut rng).clamp(min, max)
            }
            crate::ast::Distribution::Exponential { lambda } => {
                let exp = Exp::new(*lambda).map_err(|e| DadagenError::GenerationError {
                    message: format!("Failed to create exponential distribution: {}", e),
                })?;
                let sample = exp.sample(&mut rng);
                (min + sample).clamp(min, max)
            }
        };
        
        // Format based on decimal places
        if let Some(places) = self.config.decimal_places {
            if places > 0 {
                return Ok(format!("{:.prec$}", value, prec = places));
            }
        }
        
        Ok((value as i64).to_string())
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::Number
    }
    
    fn validate(&self) -> Result<()> {
        if let (Some(min), Some(max)) = (self.config.min, self.config.max) {
            if min >= max {
                return Err(DadagenError::ValidationError {
                    message: format!("min ({}) must be less than max ({})", min, max),
                });
            }
        }
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(self.clone())
    }
}

/// Boolean generator with probability support
#[derive(Debug, Clone)]
pub struct BooleanDataGenerator {
    config: BooleanGenerator,
}

impl BooleanDataGenerator {
    pub fn new(config: BooleanGenerator) -> Self {
        Self { config }
    }
    
    pub fn from_ast(ast: &BooleanGenerator) -> Result<Self> {
        let gen = Self::new(ast.clone());
        gen.validate()?;
        Ok(gen)
    }
}

impl DataGenerator for BooleanDataGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        
        let mut rng = StdRng::from_entropy();
        let random_value: f64 = rng.gen();
        
        let result = random_value < self.config.true_probability;
        Ok(result.to_string())
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::Boolean
    }
    
    fn validate(&self) -> Result<()> {
        if self.config.true_probability < 0.0 || self.config.true_probability > 1.0 {
            return Err(DadagenError::ValidationError {
                message: format!(
                    "true_probability must be between 0.0 and 1.0, got {}",
                    self.config.true_probability
                ),
            });
        }
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(self.clone())
    }
}

/// Choice generator - selects from a list of options
#[derive(Debug, Clone)]
pub struct ChoiceDataGenerator {
    config: ChoiceGenerator,
}

impl ChoiceDataGenerator {
    pub fn new(config: ChoiceGenerator) -> Self {
        Self { config }
    }
    
    pub fn from_ast(ast: &ChoiceGenerator) -> Result<Self> {
        let gen = Self::new(ast.clone());
        gen.validate()?;
        Ok(gen)
    }
}

impl DataGenerator for ChoiceDataGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        use rand::{SeedableRng, seq::SliceRandom};
        use rand::rngs::StdRng;
        
        if self.config.options.is_empty() {
            return Err(DadagenError::GenerationError {
                message: "Choice generator has no options".to_string(),
            });
        }
        
        let mut rng = StdRng::from_entropy();
        let choice = self.config.options
            .choose(&mut rng)
            .ok_or_else(|| DadagenError::GenerationError {
                message: "Failed to choose from options".to_string(),
            })?;
        
        Ok(choice.clone())
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::Choice
    }
    
    fn validate(&self) -> Result<()> {
        if self.config.options.is_empty() {
            return Err(DadagenError::ValidationError {
                message: "Choice generator must have at least one option".to_string(),
            });
        }
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(self.clone())
    }
}

/// Counter generator - produces sequential numbers
#[derive(Debug, Clone)]
pub struct CounterDataGenerator {
    config: CounterGenerator,
    current: std::sync::Arc<std::sync::Mutex<i64>>,
}

impl CounterDataGenerator {
    pub fn new(config: CounterGenerator) -> Self {
        let start = config.start.unwrap_or(0);
        Self {
            config,
            current: std::sync::Arc::new(std::sync::Mutex::new(start)),
        }
    }
    
    pub fn from_ast(ast: &CounterGenerator) -> Result<Self> {
        Ok(Self::new(ast.clone()))
    }
}

impl DataGenerator for CounterDataGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        let step = self.config.step.unwrap_or(1);
        let mut current = self.current.lock().unwrap();
        let value = *current;
        *current += step;
        Ok(value.to_string())
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::Counter
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(Self {
            config: self.config.clone(),
            current: std::sync::Arc::clone(&self.current),
        })
    }
}

// ============================================================================
// Specialized Generators
// ============================================================================

/// Template generator with variable substitution
#[derive(Debug, Clone)]
pub struct TemplateDataGenerator {
    config: TemplateGenerator,
}

impl TemplateDataGenerator {
    pub fn new(config: TemplateGenerator) -> Self {
        Self { config }
    }
    
    pub fn from_ast(ast: &TemplateGenerator) -> Result<Self> {
        let gen = Self::new(ast.clone());
        gen.validate()?;
        Ok(gen)
    }
}

impl DataGenerator for TemplateDataGenerator {
    fn generate(&self, context: &Context) -> Result<String> {
        use regex::Regex;
        
        let mut result = self.config.template.clone();
        
        // Replace {{field_name}} with values from context
            let field_regex = Regex::new(r"\{\{([^}]+)\}\}").map_err(|e| DadagenError::GenerationError {
            message: format!("Invalid regex pattern: {}", e),
        })?;
        
        result = field_regex.replace_all(&result, |caps: &regex::Captures| {
            let field_name = &caps[1];
            
            // Check if it's a generator expression like gen:string
            if field_name.starts_with("gen:") {
                // For now, return placeholder - full gen: support in future
                format!("[{}]", field_name)
            } else {
                // Try to get field value from context
                match context.get_field_state::<String>(field_name) {
                    Ok(Some(value)) => value,
                    Ok(None) => format!("[missing:{}]", field_name),
                    Err(_) => format!("[error:{}]", field_name),
                }
            }
        }).to_string();
        
        Ok(result)
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.config.variables.iter().map(|v| v.name.clone()).collect()
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::Template
    }
    
    fn validate(&self) -> Result<()> {
        if self.config.template.is_empty() {
            return Err(DadagenError::ValidationError {
                message: "Template cannot be empty".to_string(),
            });
        }

        // Ensure there are no empty template placeholders like {{}}
        let placeholder_re = regex::Regex::new(r"\{\{([^}]*)\}\}").map_err(|e| DadagenError::ValidationError {
            message: format!("Invalid template placeholder regex: {}", e),
        })?;

        for caps in placeholder_re.captures_iter(&self.config.template) {
            let name = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            if name.is_empty() {
                return Err(DadagenError::ValidationError {
                    message: "Template variable name cannot be empty".to_string(),
                });
            }
        }

        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(self.clone())
    }
}

/// List generator - picks from a data list with discriminator support
#[derive(Debug, Clone)]
pub struct ListDataGenerator {
    config: ListGenerator,
    // Cache for loaded list data
    list_data: std::sync::Arc<std::sync::RwLock<Option<Vec<String>>>>,
}

impl ListDataGenerator {
    pub fn new(config: ListGenerator) -> Self {
        Self {
            config,
            list_data: std::sync::Arc::new(std::sync::RwLock::new(None)),
        }
    }
    
    pub fn from_ast(ast: &ListGenerator) -> Result<Self> {
        let gen = Self::new(ast.clone());
        gen.validate()?;
        Ok(gen)
    }
    
    fn load_list_data(&self) -> Result<Vec<String>> {
        // Check cache first
        {
            let cache = self.list_data.read().unwrap();
            if let Some(data) = &*cache {
                return Ok(data.clone());
            }
        }
        
        // Load from list manager (simplified for now - would use actual list manager)
        // For now, return dummy data
        let data = vec![
            format!("item1_{}", self.config.name),
            format!("item2_{}", self.config.name),
            format!("item3_{}", self.config.name),
        ];
        
        // Cache the data
        {
            let mut cache = self.list_data.write().unwrap();
            *cache = Some(data.clone());
        }
        
        Ok(data)
    }
}

impl DataGenerator for ListDataGenerator {
    fn generate(&self, context: &Context) -> Result<String> {
        use rand::{SeedableRng, seq::SliceRandom};
        use rand::rngs::StdRng;
        
        // If there's a discriminator, get its value first
        let discriminator_value = if let Some(discriminator_field) = &self.config.discriminator {
            match context.get_field_state::<String>(discriminator_field) {
                Ok(Some(value)) => Some(value),
                Ok(None) => {
                    return Err(DadagenError::GenerationError {
                        message: format!("Discriminator field '{}' not found in context", discriminator_field),
                    });
                }
                Err(e) => {
                    return Err(DadagenError::GenerationError {
                        message: format!("Error reading discriminator field '{}': {}", discriminator_field, e),
                    });
                }
            }
        } else {
            None
        };
        
        // Load list data
        let items = self.load_list_data()?;
        
        if items.is_empty() {
            return Err(DadagenError::GenerationError {
                message: format!("List '{}' is empty", self.config.name),
            });
        }
        
        // Filter by discriminator if provided
        let filtered_items: Vec<&String> = if let Some(disc_val) = discriminator_value {
            items.iter().filter(|item| item.contains(&disc_val)).collect()
        } else {
            items.iter().collect()
        };
        
        if filtered_items.is_empty() {
            return Err(DadagenError::GenerationError {
                message: format!("No items match discriminator in list '{}'", self.config.name),
            });
        }
        
        // Select item (weighted or uniform)
        let mut rng = StdRng::from_entropy();
        let selected = if self.config.weighted {
            // For weighted selection, would parse weights from data
            // For now, use uniform distribution
            filtered_items.choose(&mut rng)
        } else {
            filtered_items.choose(&mut rng)
        };
        
        Ok(selected
            .ok_or_else(|| DadagenError::GenerationError {
                message: "Failed to select item from list".to_string(),
            })?
            .to_string())
    }
    
    fn dependencies(&self) -> Vec<String> {
        if let Some(discriminator) = &self.config.discriminator {
            vec![discriminator.clone()]
        } else {
            Vec::new()
        }
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::List
    }
    
    fn validate(&self) -> Result<()> {
        if self.config.name.is_empty() {
            return Err(DadagenError::ValidationError {
                message: "List name cannot be empty".to_string(),
            });
        }
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(Self {
            config: self.config.clone(),
            list_data: std::sync::Arc::clone(&self.list_data),
        })
    }
}

/// Regex-based string generator
#[derive(Debug, Clone)]
pub struct RegexDataGenerator {
    config: RegexGenerator,
}

impl RegexDataGenerator {
    pub fn new(config: RegexGenerator) -> Self {
        Self { config }
    }
    
    pub fn from_ast(ast: &RegexGenerator) -> Result<Self> {
        let gen = Self::new(ast.clone());
        gen.validate()?;
        Ok(gen)
    }
}

impl DataGenerator for RegexDataGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        use rand_regex::Regex;
        
        let pattern = &self.config.pattern;
        let mut rng = StdRng::from_entropy();
        
        // Use rand_regex crate for full regex support
        // Set max_repeat to 100 to limit potentially infinite patterns
        let generator = Regex::compile(pattern, 100).map_err(|e| {
            DadagenError::GenerationError {
                message: format!("Invalid regex pattern '{}': {}", pattern, e),
            }
        })?;
        
        // Sample a random string matching the pattern
        Ok(rng.sample(&generator))
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::Regex
    }
    
    fn validate(&self) -> Result<()> {
        if self.config.pattern.is_empty() {
            return Err(DadagenError::ValidationError {
                message: "Regex pattern cannot be empty".to_string(),
            });
        }
        
        // Validate regex syntax by attempting to compile
        use rand_regex::Regex;
        Regex::compile(&self.config.pattern, 100).map_err(|e| {
            DadagenError::ValidationError {
                message: format!("Invalid regex pattern '{}': {}", self.config.pattern, e),
            }
        })?;
        
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(self.clone())
    }
}

/// Address component generator
#[derive(Debug, Clone)]
pub struct AddressDataGenerator {
    config: AddressGenerator,
}

impl AddressDataGenerator {
    pub fn new(config: AddressGenerator) -> Self {
        Self { config }
    }
    
    pub fn from_ast(ast: &AddressGenerator) -> Result<Self> {
        Ok(Self::new(ast.clone()))
    }
}

impl DataGenerator for AddressDataGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        use rand::{SeedableRng, seq::SliceRandom};
        use rand::rngs::StdRng;
        
        let mut rng = StdRng::from_entropy();
        
        // Generate based on address component type
        let result = match self.config.component {
            AddressComponent::CityTown => {
                let cities = vec!["Sydney", "Melbourne", "Brisbane", "Perth", "Adelaide", "Canberra"];
                cities.choose(&mut rng).unwrap().to_string()
            }
            AddressComponent::Suburb => {
                let suburbs = vec!["Parramatta", "Chatswood", "Bondi", "Richmond", "Carlton"];
                suburbs.choose(&mut rng).unwrap().to_string()
            }
            AddressComponent::PostZipCode => {
                format!("{:04}", rng.gen_range(1000..9999))
            }
            AddressComponent::Street => {
                let streets = vec!["Main", "High", "Park", "Church", "Station"];
                let types = vec!["Street", "Road", "Avenue", "Lane", "Drive"];
                format!("{} {}", 
                    streets.choose(&mut rng).unwrap(),
                    types.choose(&mut rng).unwrap())
            }
            AddressComponent::Property => {
                format!("{}", rng.gen_range(1..999))
            }
            AddressComponent::Country => {
                let countries = vec!["Australia", "United States", "United Kingdom", "Canada", "New Zealand"];
                countries.choose(&mut rng).unwrap().to_string()
            }
            AddressComponent::StateCounty => {
                let states = vec!["NSW", "VIC", "QLD", "WA", "SA", "TAS", "NT", "ACT"];
                states.choose(&mut rng).unwrap().to_string()
            }
        };
        
        Ok(result)
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::Address
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(self.clone())
    }
}

/// Name generator
#[derive(Debug, Clone)]
pub struct NameDataGenerator {
    config: NameGenerator,
}

impl NameDataGenerator {
    pub fn new(config: NameGenerator) -> Self {
        Self { config }
    }
    
    pub fn from_ast(ast: &NameGenerator) -> Result<Self> {
        Ok(Self::new(ast.clone()))
    }
}

impl DataGenerator for NameDataGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        use rand::{SeedableRng, seq::SliceRandom};
        use rand::rngs::StdRng;
        
        let mut rng = StdRng::from_entropy();
        
        let first_names = vec!["James", "Mary", "John", "Patricia", "Robert", "Jennifer", "Michael", "Linda"];
        let last_names = vec!["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis"];
        
        let result = match self.config.name_type {
            NameType::GivenName => {
                first_names.choose(&mut rng).unwrap().to_string()
            }
            NameType::Surname => {
                last_names.choose(&mut rng).unwrap().to_string()
            }
            NameType::Full => {
                format!("{} {}", 
                    first_names.choose(&mut rng).unwrap(),
                    last_names.choose(&mut rng).unwrap())
            }
        };
        
        Ok(result)
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::Name
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(self.clone())
    }
}

/// Gender generator
#[derive(Debug, Clone)]
pub struct GenderDataGenerator {
    config: GenderGenerator,
}

impl GenderDataGenerator {
    pub fn new(config: GenderGenerator) -> Self {
        Self { config }
    }
    
    pub fn from_ast(ast: &GenderGenerator) -> Result<Self> {
        Ok(Self::new(ast.clone()))
    }
}

impl DataGenerator for GenderDataGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        use rand::{SeedableRng, seq::SliceRandom};
        use rand::rngs::StdRng;
        
        let mut rng = StdRng::from_entropy();
        let genders = vec!["Male", "Female", "Other", "Prefer not to say"];
        
        Ok(genders.choose(&mut rng).unwrap().to_string())
    }
    
    fn generator_type(&self) -> GeneratorType {
        GeneratorType::Gender
    }
    
    fn clone_box(&self) -> Box<dyn DataGenerator> {
        Box::new(self.clone())
    }
}

// ============================================================================
// Generator Factory
// ============================================================================

/// Create a DataGenerator from an AST Generator enum
pub fn create_generator(ast_gen: &Generator) -> Result<Box<dyn DataGenerator>> {
    match ast_gen {
        Generator::String(config) => Ok(Box::new(StringDataGenerator::from_ast(config)?)),
        Generator::Number(config) => Ok(Box::new(NumberDataGenerator::from_ast(config)?)),
        Generator::Boolean(config) => Ok(Box::new(BooleanDataGenerator::from_ast(config)?)),
        Generator::Choice(config) => Ok(Box::new(ChoiceDataGenerator::from_ast(config)?)),
        Generator::Counter(config) => Ok(Box::new(CounterDataGenerator::from_ast(config)?)),
        Generator::Template(config) => Ok(Box::new(TemplateDataGenerator::from_ast(config)?)),
        Generator::List(config) => Ok(Box::new(ListDataGenerator::from_ast(config)?)),
        Generator::Regex(config) => Ok(Box::new(RegexDataGenerator::from_ast(config)?)),
        Generator::Address(config) => Ok(Box::new(AddressDataGenerator::from_ast(config)?)),
        Generator::Name(config) => Ok(Box::new(NameDataGenerator::from_ast(config)?)),
        Generator::Gender(config) => Ok(Box::new(GenderDataGenerator::from_ast(config)?)),
        Generator::DateTime(_) | Generator::Date(_) | Generator::Time(_) => {
            Err(DadagenError::GenerationError {
                message: "DateTime generators not yet implemented".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_context() -> Context {
        Context::new()
    }
    
    #[test]
    fn test_string_generator_basic() {
        let config = StringGenerator {
            length: Some(10),
            min_length: None,
            max_length: None,
            charset: CharacterSet::Alpha,
            case: Case::Mixed,
            pattern: None,
            span: None,
        };
        
        let gen = StringDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert_eq!(result.len(), 10);
        assert!(result.chars().all(|c| c.is_alphabetic()));
    }
    
    #[test]
    fn test_string_generator_range() {
        let config = StringGenerator {
            length: None,
            min_length: Some(5),
            max_length: Some(15),
            charset: CharacterSet::AlphaNumeric,
            case: Case::Lower,
            pattern: None,
            span: None,
        };
        
        let gen = StringDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert!(result.len() >= 5 && result.len() <= 15);
        assert_eq!(result, result.to_lowercase());
    }
    
    #[test]
    fn test_string_generator_validation() {
        let config = StringGenerator {
            length: Some(10),
            min_length: Some(5),  // Conflict!
            max_length: Some(15),
            charset: CharacterSet::Alpha,
            case: Case::Mixed,
            pattern: None,
            span: None,
        };
        
        let gen = StringDataGenerator::new(config);
        assert!(gen.validate().is_err());
    }
    
    #[test]
    fn test_number_generator_range() {
        let config = NumberGenerator {
            min: Some(1.0),
            max: Some(100.0),
            decimal_places: None,
            distribution: Distribution::Uniform,
            seed: Some(42),
            span: None,
        };
        
        let gen = NumberDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        let value: i64 = result.parse().unwrap();
        
        assert!(value >= 1 && value <= 100);
    }
    
    #[test]
    fn test_number_generator_decimal() {
        let config = NumberGenerator {
            min: Some(0.0),
            max: Some(10.0),
            decimal_places: Some(2),
            distribution: Distribution::Uniform,
            seed: Some(42),
            span: None,
        };
        
        let gen = NumberDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        // Should have decimal point
        assert!(result.contains('.'));
        
        // Should have at most 2 decimal places
        let parts: Vec<&str> = result.split('.').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts[1].len() <= 2);
    }
    
    #[test]
    fn test_boolean_generator() {
        let config = BooleanGenerator {
            true_probability: 1.0, // Always true
            span: None,
        };
        
        let gen = BooleanDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert_eq!(result, "true");
    }
    
    #[test]
    fn test_boolean_validation() {
        let config = BooleanGenerator {
            true_probability: 1.5, // Invalid!
            span: None,
        };
        
        let gen = BooleanDataGenerator::new(config);
        assert!(gen.validate().is_err());
    }
    
    #[test]
    fn test_choice_generator() {
        let config = ChoiceGenerator {
            options: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
            span: None,
        };
        
        let gen = ChoiceDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert!(["red", "green", "blue"].contains(&result.as_str()));
    }
    
    #[test]
    fn test_counter_generator() {
        let config = CounterGenerator {
            start: Some(100),
            step: Some(5),
            span: None,
        };
        
        let gen = CounterDataGenerator::new(config);
        
        assert_eq!(gen.generate(&test_context()).unwrap(), "100");
        assert_eq!(gen.generate(&test_context()).unwrap(), "105");
        assert_eq!(gen.generate(&test_context()).unwrap(), "110");
    }
    
    #[test]
    fn test_generator_factory() {
        let ast_gen = Generator::String(StringGenerator {
            length: Some(5),
            min_length: None,
            max_length: None,
            charset: CharacterSet::Numeric,
            case: Case::Mixed,
            pattern: None,
            span: None,
        });
        
        let gen = create_generator(&ast_gen).unwrap();
        let result = gen.generate(&test_context()).unwrap();
        
        assert_eq!(result.len(), 5);
        assert!(result.chars().all(|c| c.is_numeric()));
    }
    
    // ========================================================================
    // Specialized Generator Tests
    // ========================================================================
    
    #[test]
    fn test_template_generator_basic() {
        let config = TemplateGenerator {
            template: "Hello {{name}}!".to_string(),
            variables: vec![TemplateVariable {
                name: "name".to_string(),
                generator: None,
            }],
            span: None,
        };
        
        let gen = TemplateDataGenerator::new(config);
        
        // Test with context containing the field
        let ctx = test_context();
        ctx.insert_field_state("name".to_string(), "World".to_string()).unwrap();
        
        let result = gen.generate(&ctx).unwrap();
        assert_eq!(result, "Hello World!");
    }
    
    #[test]
    fn test_template_generator_missing_field() {
        let config = TemplateGenerator {
            template: "User: {{username}}".to_string(),
            variables: vec![TemplateVariable {
                name: "username".to_string(),
                generator: None,
            }],
            span: None,
        };
        
        let gen = TemplateDataGenerator::new(config);
        
        // Generate with missing field should show placeholder
        let result = gen.generate(&test_context()).unwrap();
        assert_eq!(result, "User: [missing:username]");
    }
    
    #[test]
    fn test_template_generator_multiple_variables() {
        let config = TemplateGenerator {
            template: "{{first}} {{last}} - {{email}}".to_string(),
            variables: vec![
                TemplateVariable { name: "first".to_string(), generator: None },
                TemplateVariable { name: "last".to_string(), generator: None },
                TemplateVariable { name: "email".to_string(), generator: None },
            ],
            span: None,
        };
        
        let gen = TemplateDataGenerator::new(config);
        
        let ctx = test_context();
        ctx.insert_field_state("first".to_string(), "John".to_string()).unwrap();
        ctx.insert_field_state("last".to_string(), "Doe".to_string()).unwrap();
        ctx.insert_field_state("email".to_string(), "john.doe@example.com".to_string()).unwrap();
        
        let result = gen.generate(&ctx).unwrap();
        assert_eq!(result, "John Doe - john.doe@example.com");
    }
    
    #[test]
    fn test_template_generator_dependencies() {
        let config = TemplateGenerator {
            template: "{{user_id}}-{{user_name}}".to_string(),
            variables: vec![
                TemplateVariable { name: "user_id".to_string(), generator: None },
                TemplateVariable { name: "user_name".to_string(), generator: None },
            ],
            span: None,
        };
        
        let gen = TemplateDataGenerator::new(config);
        let deps = gen.dependencies();
        
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&"user_id".to_string()));
        assert!(deps.contains(&"user_name".to_string()));
    }
    
    #[test]
    fn test_template_generator_validation() {
        let config = TemplateGenerator {
            template: "{{}}".to_string(),
            variables: vec![],
            span: None,
        };
        
        let result = TemplateDataGenerator::from_ast(&config);
        assert!(result.is_err());
        
        if let Err(DadagenError::ValidationError { message }) = result {
            assert!(message.contains("cannot be empty"));
        } else {
            panic!("Expected ValidationError");
        }
    }
    
    #[test]
    fn test_list_generator_basic() {
        let config = ListGenerator {
            name: "test_list".to_string(),
            discriminator: None,
            weighted: false,
            span: None,
        };
        
        let gen = ListDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        // Should return one of the predefined test items
        assert!(result.starts_with("item"));
    }
    
    #[test]
    fn test_list_generator_with_discriminator() {
        let config = ListGenerator {
            name: "filtered_list".to_string(),
            discriminator: Some("category".to_string()),
            weighted: false,
            span: None,
        };
        
        let gen = ListDataGenerator::new(config);
        
        // Without discriminator field, should error
        let result = gen.generate(&test_context());
        assert!(result.is_err());
    }
    
    #[test]
    fn test_list_generator_dependencies() {
        let config = ListGenerator {
            name: "test_list".to_string(),
            discriminator: Some("filter_field".to_string()),
            weighted: false,
            span: None,
        };
        
        let gen = ListDataGenerator::new(config);
        let deps = gen.dependencies();
        
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0], "filter_field");
    }
    
    #[test]
    fn test_list_generator_validation() {
        let config = ListGenerator {
            name: "".to_string(),
            discriminator: None,
            weighted: false,
            span: None,
        };
        
        let result = ListDataGenerator::from_ast(&config);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_regex_generator_character_class() {
        let config = RegexGenerator {
            pattern: "[abc]".to_string(),
            span: None,
        };
        
        let gen = RegexDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert!(["a", "b", "c"].contains(&result.as_str()));
    }
    
    #[test]
    fn test_regex_generator_range() {
        let config = RegexGenerator {
            pattern: "[0-9]".to_string(),
            span: None,
        };
        
        let gen = RegexDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert_eq!(result.len(), 1);
        assert!(result.chars().next().unwrap().is_numeric());
    }
    
    #[test]
    fn test_regex_generator_validation() {
        let config = RegexGenerator {
            pattern: "".to_string(),
            span: None,
        };
        
        let result = RegexDataGenerator::from_ast(&config);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_address_generator_city() {
        let config = AddressGenerator {
            component: AddressComponent::CityTown,
            span: None,
        };
        
        let gen = AddressDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert!(!result.is_empty());
        assert_eq!(gen.generator_type(), GeneratorType::Address);
    }
    
    #[test]
    fn test_address_generator_postcode() {
        let config = AddressGenerator {
            component: AddressComponent::PostZipCode,
            span: None,
        };
        
        let gen = AddressDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!(result.chars().all(|c| c.is_numeric()));
    }
    
    #[test]
    fn test_address_generator_street() {
        let config = AddressGenerator {
            component: AddressComponent::Street,
            span: None,
        };
        
        let gen = AddressDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        // Should contain both street name and type (e.g., "Main Street")
        assert!(result.contains(' '));
    }
    
    #[test]
    fn test_name_generator_given_name() {
        let config = NameGenerator {
            name_type: NameType::GivenName,
            span: None,
        };
        
        let gen = NameDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert!(!result.is_empty());
        assert_eq!(gen.generator_type(), GeneratorType::Name);
    }
    
    #[test]
    fn test_name_generator_full_name() {
        let config = NameGenerator {
            name_type: NameType::Full,
            span: None,
        };
        
        let gen = NameDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        // Full name should contain space between first and last
        assert!(result.contains(' '));
    }
    
    #[test]
    fn test_gender_generator() {
        let config = GenderGenerator {
            span: None,
        };
        
        let gen = GenderDataGenerator::new(config);
        let result = gen.generate(&test_context()).unwrap();
        
        assert!(!result.is_empty());
        assert_eq!(gen.generator_type(), GeneratorType::Gender);
    }
    
    #[test]
    fn test_factory_with_specialized_generators() {
        // Test template generator factory
        let template_ast = Generator::Template(TemplateGenerator {
            template: "Test {{field}}".to_string(),
            variables: vec![TemplateVariable {
                name: "field".to_string(),
                generator: None,
            }],
            span: None,
        });
        
        let gen = create_generator(&template_ast).unwrap();
        assert_eq!(gen.generator_type(), GeneratorType::Template);
        
        // Test list generator factory
        let list_ast = Generator::List(ListGenerator {
            name: "test".to_string(),
            discriminator: None,
            weighted: false,
            span: None,
        });
        
        let gen = create_generator(&list_ast).unwrap();
        assert_eq!(gen.generator_type(), GeneratorType::List);
        
        // Test regex generator factory
        let regex_ast = Generator::Regex(RegexGenerator {
            pattern: "[a-z]".to_string(),
            span: None,
        });
        
        let gen = create_generator(&regex_ast).unwrap();
        assert_eq!(gen.generator_type(), GeneratorType::Regex);
        
        // Test address generator factory
        let address_ast = Generator::Address(AddressGenerator {
            component: AddressComponent::CityTown,
            span: None,
        });
        
        let gen = create_generator(&address_ast).unwrap();
        assert_eq!(gen.generator_type(), GeneratorType::Address);
    }
}
