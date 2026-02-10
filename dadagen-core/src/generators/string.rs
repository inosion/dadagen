//! String generation utilities

use crate::context::Context;
use crate::errors::{DadagenError, Result};
use crate::generators::core::{ConfigurableGenerator, Generator};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

/// Configuration for string generators
#[derive(Debug, Clone)]
pub struct StringConfig {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub charset: CharacterSet,
    pub patterns: Vec<String>,
    pub seed: Option<u64>,
    pub case: Case,
}

impl Default for StringConfig {
    fn default() -> Self {
        Self {
            min_length: Some(5),
            max_length: Some(10),
            charset: CharacterSet::AlphaNumeric,
            patterns: vec![],
            seed: None,
            case: Case::Mixed,
        }
    }
}

/// Character set options for string generation
#[derive(Debug, Clone)]
pub enum CharacterSet {
    Alpha,
    Numeric,
    AlphaNumeric,
    Alphanumeric,
    ASCII,
    Custom(String),
    Hex,
    Base64,
}

impl CharacterSet {
    fn to_chars(&self) -> Vec<char> {
        match self {
            CharacterSet::Alpha => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .collect(),
            CharacterSet::Numeric => "0123456789".chars().collect(),
            CharacterSet::AlphaNumeric | CharacterSet::Alphanumeric => {
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                    .chars()
                    .collect()
            }
            CharacterSet::ASCII => (32u8..=126u8).map(|b| b as char).collect(),
            CharacterSet::Custom(chars) => chars.chars().collect(),
            CharacterSet::Hex => "0123456789ABCDEF".chars().collect(),
            CharacterSet::Base64 => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                    .chars()
                    .collect()
            }
        }
    }
}

/// Case transformation options
#[derive(Debug, Clone)]
pub enum Case {
    Lower,
    Upper,
    Title,
    Mixed,
}

/// Random string generator
#[derive(Debug, Clone)]
pub struct StringGenerator {
    name: String,
    config: StringConfig,
    dependencies: Vec<String>,
}

impl StringGenerator {
    pub fn new(name: String) -> Self {
        Self {
            name,
            config: StringConfig::default(),
            dependencies: vec![],
        }
    }

    pub fn with_length_range(mut self, min: usize, max: usize) -> Self {
        self.config.min_length = Some(min);
        self.config.max_length = Some(max);
        self
    }

    pub fn with_fixed_length(mut self, length: usize) -> Self {
        self.config.min_length = Some(length);
        self.config.max_length = Some(length);
        self
    }

    pub fn with_charset(mut self, charset: CharacterSet) -> Self {
        self.config.charset = charset;
        self
    }

    pub fn with_case(mut self, case: Case) -> Self {
        self.config.case = case;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }
}

impl Generator<String> for StringGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        let mut rng = match self.config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        let chars = self.config.charset.to_chars();
        if chars.is_empty() {
            return Err(DadagenError::GenerationError {
                message: "Character set is empty".to_string(),
            });
        }

        let min_len = self.config.min_length.unwrap_or(5);
        let max_len = self.config.max_length.unwrap_or(10);

        if min_len > max_len {
            return Err(DadagenError::GenerationError {
                message: format!(
                    "Invalid length range: min ({}) must be <= max ({})",
                    min_len, max_len
                ),
            });
        }

        let length = if min_len == max_len {
            min_len
        } else {
            rng.gen_range(min_len..=max_len)
        };

        let mut result: String = (0..length)
            .map(|_| *chars.choose(&mut rng).unwrap())
            .collect();

        // Apply case transformation
        result = match self.config.case {
            Case::Lower => result.to_lowercase(),
            Case::Upper => result.to_uppercase(),
            Case::Title => {
                let mut chars: Vec<char> = result.chars().collect();
                if let Some(first) = chars.first_mut() {
                    *first = first.to_uppercase().next().unwrap_or(*first);
                }
                chars.into_iter().collect()
            }
            Case::Mixed => result, // Keep as generated
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

impl ConfigurableGenerator<String, StringConfig> for StringGenerator {
    fn with_config(mut self, config: StringConfig) -> Self {
        self.config = config;
        self
    }

    fn config(&self) -> &StringConfig {
        &self.config
    }
}

/// Pattern-based string generator
#[derive(Debug, Clone)]
pub struct PatternGenerator {
    name: String,
    pattern: String,
    dependencies: Vec<String>,
    seed: Option<u64>,
}

impl PatternGenerator {
    pub fn new(name: String, pattern: String) -> Self {
        Self {
            name,
            pattern,
            dependencies: vec![],
            seed: None,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self {
        self.dependencies = deps;
        self
    }

    /// Parse pattern and generate string
    /// Pattern format:
    /// - #{field_name} - Reference to context field
    /// - {a-z} - Random lowercase letter
    /// - {A-Z} - Random uppercase letter  
    /// - {0-9} - Random digit
    /// - {5} - Random string of length 5
    /// - Literal text is kept as-is
    fn generate_from_pattern(
        &self,
        pattern: &str,
        context: &Context,
        rng: &mut StdRng,
    ) -> Result<String> {
        let mut result = String::new();
        let mut chars = pattern.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '#' if chars.peek() == Some(&'{') => {
                    // Field reference #{field_name}
                    chars.next(); // consume '{'
                    let mut field_name = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == '}' {
                            break;
                        }
                        field_name.push(ch);
                    }

                    if let Ok(Some(value)) = context.get_field_state::<String>(&field_name) {
                        result.push_str(&value);
                    } else {
                        return Err(DadagenError::GenerationError {
                            message: format!("Field '{}' not found in context", field_name),
                        });
                    }
                }
                '{' => {
                    // Pattern specifier
                    let mut spec = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == '}' {
                            break;
                        }
                        spec.push(ch);
                    }

                    let generated = match spec.as_str() {
                        "a-z" => ((rng.gen_range(b'a'..=b'z')) as char).to_string(),
                        "A-Z" => ((rng.gen_range(b'A'..=b'Z')) as char).to_string(),
                        "0-9" => rng.gen_range(0..=9).to_string(),
                        spec if spec.chars().all(|c| c.is_ascii_digit()) => {
                            let len: usize =
                                spec.parse().map_err(|_| DadagenError::GenerationError {
                                    message: format!("Invalid length specifier: {}", spec),
                                })?;

                            let charset = CharacterSet::AlphaNumeric.to_chars();
                            (0..len).map(|_| *charset.choose(rng).unwrap()).collect()
                        }
                        _ => {
                            return Err(DadagenError::GenerationError {
                                message: format!("Unknown pattern specifier: {}", spec),
                            });
                        }
                    };

                    result.push_str(&generated);
                }
                ch => result.push(ch),
            }
        }

        Ok(result)
    }
}

impl Generator<String> for PatternGenerator {
    fn generate(&self, context: &Context) -> Result<String> {
        let mut rng = match self.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        self.generate_from_pattern(&self.pattern, context, &mut rng)
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// List-based string generator (selects from predefined options)
#[derive(Debug, Clone)]
pub struct ListGenerator {
    name: String,
    options: Vec<String>,
    dependencies: Vec<String>,
    seed: Option<u64>,
}

impl ListGenerator {
    pub fn new(name: String, options: Vec<String>) -> Self {
        Self {
            name,
            options,
            dependencies: vec![],
            seed: None,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
}

impl Generator<String> for ListGenerator {
    fn generate(&self, _context: &Context) -> Result<String> {
        if self.options.is_empty() {
            return Err(DadagenError::GenerationError {
                message: "No options available for list generator".to_string(),
            });
        }

        let mut rng = match self.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        Ok(self.options.choose(&mut rng).unwrap().clone())
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    fn name(&self) -> &str {
        &self.name
    }
}
