//! Generator Registry and Factory System
//!
//! This module provides a dynamic generator registration and creation system
//! that integrates with the AST-based generator trait system. It allows for:
//! - Registration of generator factories
//! - Creation of generators from AST structures
//! - Metadata and documentation for all generator types
//! - Integration testing across all generator types

use crate::ast::*;
use crate::context::Context;
use crate::errors::{DadagenError, Result};
use crate::generator_trait::{DataGenerator, GeneratorType, create_generator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Metadata describing a generator type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorTypeMetadata {
    /// The generator type identifier
    pub generator_type: GeneratorType,

    /// Human-readable name
    pub display_name: String,

    /// Detailed description of what this generator does
    pub description: String,

    /// List of configuration parameters
    pub parameters: Vec<ParameterMetadata>,

    /// Example configurations
    pub examples: Vec<String>,

    /// Whether this generator can have dependencies on other fields
    pub can_have_dependencies: bool,
}

/// Metadata for a single generator parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterMetadata {
    /// Parameter name
    pub name: String,

    /// Parameter data type
    pub param_type: String,

    /// Whether this parameter is required
    pub required: bool,

    /// Default value if not specified
    pub default_value: Option<String>,

    /// Parameter description
    pub description: String,
}

/// Dynamic generator registry for creating and managing generators
pub struct GeneratorRegistry {
    /// Metadata for all registered generator types
    metadata: HashMap<GeneratorType, GeneratorTypeMetadata>,

    /// Cache of created generators by field name
    generator_cache: Arc<RwLock<HashMap<String, Box<dyn DataGenerator>>>>,
}

impl GeneratorRegistry {
    /// Create a new registry with all standard generator types registered
    pub fn new() -> Self {
        let mut registry = Self {
            metadata: HashMap::new(),
            generator_cache: Arc::new(RwLock::new(HashMap::new())),
        };

        registry.register_standard_generators();
        registry
    }

    /// Register metadata for all standard generator types
    fn register_standard_generators(&mut self) {
        // String Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::String,
            display_name: "String Generator".to_string(),
            description:
                "Generates random strings with configurable length, character sets, and case"
                    .to_string(),
            parameters: vec![
                ParameterMetadata {
                    name: "length".to_string(),
                    param_type: "integer".to_string(),
                    required: false,
                    default_value: Some("10".to_string()),
                    description: "Fixed length of generated string".to_string(),
                },
                ParameterMetadata {
                    name: "min_length".to_string(),
                    param_type: "integer".to_string(),
                    required: false,
                    default_value: None,
                    description: "Minimum length for variable-length strings".to_string(),
                },
                ParameterMetadata {
                    name: "max_length".to_string(),
                    param_type: "integer".to_string(),
                    required: false,
                    default_value: None,
                    description: "Maximum length for variable-length strings".to_string(),
                },
                ParameterMetadata {
                    name: "charset".to_string(),
                    param_type: "CharacterSet".to_string(),
                    required: false,
                    default_value: Some("AlphaNumeric".to_string()),
                    description: "Character set: Alpha, Numeric, AlphaNumeric, Hex, Ascii, Base64"
                        .to_string(),
                },
                ParameterMetadata {
                    name: "case".to_string(),
                    param_type: "Case".to_string(),
                    required: false,
                    default_value: Some("Mixed".to_string()),
                    description: "Case transformation: Lower, Upper, Title, Mixed".to_string(),
                },
            ],
            examples: vec![
                "string { length: 8, charset: Alpha, case: Lower }".to_string(),
                "string { min_length: 5, max_length: 15 }".to_string(),
            ],
            can_have_dependencies: false,
        });

        // Number Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::Number,
            display_name: "Number Generator".to_string(),
            description:
                "Generates random numbers (integer or decimal) with optional distributions"
                    .to_string(),
            parameters: vec![
                ParameterMetadata {
                    name: "min".to_string(),
                    param_type: "float".to_string(),
                    required: false,
                    default_value: Some("0.0".to_string()),
                    description: "Minimum value (inclusive)".to_string(),
                },
                ParameterMetadata {
                    name: "max".to_string(),
                    param_type: "float".to_string(),
                    required: false,
                    default_value: Some("100.0".to_string()),
                    description: "Maximum value (inclusive)".to_string(),
                },
                ParameterMetadata {
                    name: "decimal_places".to_string(),
                    param_type: "integer".to_string(),
                    required: false,
                    default_value: Some("0".to_string()),
                    description: "Number of decimal places (0 for integers)".to_string(),
                },
                ParameterMetadata {
                    name: "distribution".to_string(),
                    param_type: "Distribution".to_string(),
                    required: false,
                    default_value: Some("Uniform".to_string()),
                    description: "Distribution: Uniform, Normal, Exponential".to_string(),
                },
            ],
            examples: vec![
                "number { min: 1, max: 100 }".to_string(),
                "number { min: 0.0, max: 1.0, decimal_places: 2 }".to_string(),
                "number { distribution: Normal { mean: 50.0, std_dev: 10.0 } }".to_string(),
            ],
            can_have_dependencies: false,
        });

        // Boolean Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::Boolean,
            display_name: "Boolean Generator".to_string(),
            description: "Generates random boolean values with configurable probability"
                .to_string(),
            parameters: vec![ParameterMetadata {
                name: "true_probability".to_string(),
                param_type: "float".to_string(),
                required: false,
                default_value: Some("0.5".to_string()),
                description: "Probability of generating true (0.0 to 1.0)".to_string(),
            }],
            examples: vec![
                "boolean".to_string(),
                "boolean { true_probability: 0.7 }".to_string(),
            ],
            can_have_dependencies: false,
        });

        // Choice Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::Choice,
            display_name: "Choice Generator".to_string(),
            description: "Randomly selects from a predefined list of options".to_string(),
            parameters: vec![ParameterMetadata {
                name: "options".to_string(),
                param_type: "Vec<String>".to_string(),
                required: true,
                default_value: None,
                description: "List of options to choose from".to_string(),
            }],
            examples: vec![
                "choice { options: [\"red\", \"green\", \"blue\"] }".to_string(),
                "choice { options: [\"small\", \"medium\", \"large\", \"xl\"] }".to_string(),
            ],
            can_have_dependencies: false,
        });

        // Counter Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::Counter,
            display_name: "Counter Generator".to_string(),
            description: "Generates sequential numbers with configurable start and step"
                .to_string(),
            parameters: vec![
                ParameterMetadata {
                    name: "start".to_string(),
                    param_type: "integer".to_string(),
                    required: false,
                    default_value: Some("0".to_string()),
                    description: "Starting value".to_string(),
                },
                ParameterMetadata {
                    name: "step".to_string(),
                    param_type: "integer".to_string(),
                    required: false,
                    default_value: Some("1".to_string()),
                    description: "Increment value".to_string(),
                },
            ],
            examples: vec![
                "counter".to_string(),
                "counter { start: 1000, step: 10 }".to_string(),
            ],
            can_have_dependencies: false,
        });

        // Template Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::Template,
            display_name: "Template Generator".to_string(),
            description:
                "Generates strings by substituting variables in a template using {{field}} syntax"
                    .to_string(),
            parameters: vec![ParameterMetadata {
                name: "template".to_string(),
                param_type: "String".to_string(),
                required: true,
                default_value: None,
                description: "Template string with {{field}} placeholders".to_string(),
            }],
            examples: vec![
                "template { template: \"User-{{user_id}}-{{username}}\" }".to_string(),
                "template { template: \"{{first_name}} {{last_name}}\" }".to_string(),
            ],
            can_have_dependencies: true,
        });

        // List Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::List,
            display_name: "List Generator".to_string(),
            description: "Selects values from data lists with optional discriminator filtering"
                .to_string(),
            parameters: vec![
                ParameterMetadata {
                    name: "name".to_string(),
                    param_type: "String".to_string(),
                    required: true,
                    default_value: None,
                    description: "Name of the data list to use".to_string(),
                },
                ParameterMetadata {
                    name: "discriminator".to_string(),
                    param_type: "String".to_string(),
                    required: false,
                    default_value: None,
                    description: "Field to use for contextual filtering".to_string(),
                },
                ParameterMetadata {
                    name: "weighted".to_string(),
                    param_type: "boolean".to_string(),
                    required: false,
                    default_value: Some("false".to_string()),
                    description: "Whether to use weighted selection".to_string(),
                },
            ],
            examples: vec![
                "list { name: \"cities\" }".to_string(),
                "list { name: \"suburbs\", discriminator: \"city\" }".to_string(),
            ],
            can_have_dependencies: true,
        });

        // Regex Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::Regex,
            display_name: "Regex Generator".to_string(),
            description: "Generates strings matching a regular expression pattern".to_string(),
            parameters: vec![ParameterMetadata {
                name: "pattern".to_string(),
                param_type: "String".to_string(),
                required: true,
                default_value: None,
                description: "Regular expression pattern".to_string(),
            }],
            examples: vec![
                "regex { pattern: \"[A-Z][0-9]{3}\" }".to_string(),
                "regex { pattern: \"[a-z]{5}-[0-9]{4}\" }".to_string(),
            ],
            can_have_dependencies: false,
        });

        // Address Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::Address,
            display_name: "Address Generator".to_string(),
            description: "Generates realistic address components".to_string(),
            parameters: vec![
                ParameterMetadata {
                    name: "component".to_string(),
                    param_type: "AddressComponent".to_string(),
                    required: true,
                    default_value: None,
                    description: "Address component: CityTown, Suburb, Street, Property, PostZipCode, StateCounty, Country".to_string(),
                },
            ],
            examples: vec![
                "address { component: CityTown }".to_string(),
                "address { component: Street }".to_string(),
            ],
            can_have_dependencies: false,
        });

        // Name Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::Name,
            display_name: "Name Generator".to_string(),
            description: "Generates realistic person names".to_string(),
            parameters: vec![ParameterMetadata {
                name: "name_type".to_string(),
                param_type: "NameType".to_string(),
                required: true,
                default_value: None,
                description: "Name type: GivenName, Surname, Full".to_string(),
            }],
            examples: vec![
                "name { name_type: GivenName }".to_string(),
                "name { name_type: Full }".to_string(),
            ],
            can_have_dependencies: false,
        });

        // Gender Generator
        self.register_metadata(GeneratorTypeMetadata {
            generator_type: GeneratorType::Gender,
            display_name: "Gender Generator".to_string(),
            description: "Generates gender values".to_string(),
            parameters: vec![],
            examples: vec!["gender".to_string()],
            can_have_dependencies: false,
        });
    }

    /// Register metadata for a generator type
    pub fn register_metadata(&mut self, metadata: GeneratorTypeMetadata) {
        self.metadata
            .insert(metadata.generator_type.clone(), metadata);
    }

    /// Create a generator from an AST Generator enum
    pub fn create_from_ast(&self, ast_gen: &Generator) -> Result<Box<dyn DataGenerator>> {
        create_generator(ast_gen)
    }

    /// Create a generator for a field from a DslDocument
    pub fn create_for_field(
        &self,
        document: &DslDocument,
        field_name: &str,
    ) -> Result<Box<dyn DataGenerator>> {
        // Find the field in the document
        let field = document
            .fields
            .iter()
            .find(|f| f.name == field_name)
            .ok_or_else(|| DadagenError::GenerationError {
                message: format!("Field '{}' not found in document", field_name),
            })?;

        self.create_from_ast(&field.generator)
    }

    /// Get metadata for a generator type
    pub fn get_metadata(&self, generator_type: &GeneratorType) -> Option<&GeneratorTypeMetadata> {
        self.metadata.get(generator_type)
    }

    /// Get metadata for all registered generator types
    pub fn all_metadata(&self) -> Vec<&GeneratorTypeMetadata> {
        self.metadata.values().collect()
    }

    /// List all available generator types
    pub fn available_types(&self) -> Vec<GeneratorType> {
        self.metadata.keys().cloned().collect()
    }

    /// Check if a generator type is registered
    pub fn is_registered(&self, generator_type: &GeneratorType) -> bool {
        self.metadata.contains_key(generator_type)
    }

    /// Get generators that can have dependencies
    pub fn dependency_capable_types(&self) -> Vec<GeneratorType> {
        self.metadata
            .iter()
            .filter(|(_, meta)| meta.can_have_dependencies)
            .map(|(t, _)| t.clone())
            .collect()
    }

    /// Generate documentation for all generator types
    pub fn generate_documentation(&self) -> String {
        let mut doc = String::from("# Dadagen Generator Types\n\n");
        doc.push_str(
            "This document describes all available generator types and their parameters.\n\n",
        );

        let mut types: Vec<_> = self.metadata.values().collect();
        types.sort_by(|a, b| a.display_name.cmp(&b.display_name));

        for metadata in types {
            doc.push_str(&format!("## {}\n\n", metadata.display_name));
            doc.push_str(&format!("**Type:** `{:?}`\n\n", metadata.generator_type));
            doc.push_str(&format!("{}\n\n", metadata.description));

            if !metadata.parameters.is_empty() {
                doc.push_str("### Parameters\n\n");
                for param in &metadata.parameters {
                    let required = if param.required { " (required)" } else { "" };
                    let default = if let Some(d) = &param.default_value {
                        format!(" - Default: `{}`", d)
                    } else {
                        String::new()
                    };

                    doc.push_str(&format!(
                        "- **{}**{}: `{}` - {}{}\n",
                        param.name, required, param.param_type, param.description, default
                    ));
                }
                doc.push_str("\n");
            }

            if !metadata.examples.is_empty() {
                doc.push_str("### Examples\n\n");
                for example in &metadata.examples {
                    doc.push_str(&format!("```\n{}\n```\n\n", example));
                }
            }

            doc.push_str(&format!(
                "**Can have dependencies:** {}\n\n",
                if metadata.can_have_dependencies {
                    "Yes"
                } else {
                    "No"
                }
            ));

            doc.push_str("---\n\n");
        }

        doc
    }

    /// Clear the generator cache
    pub fn clear_cache(&self) -> Result<()> {
        let mut cache = self.generator_cache.write().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire write lock: {}", e))
        })?;
        cache.clear();
        Ok(())
    }
}

impl Default for GeneratorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = GeneratorRegistry::new();
        let types = registry.available_types();

        // Should have all standard generator types
        assert!(types.contains(&GeneratorType::String));
        assert!(types.contains(&GeneratorType::Number));
        assert!(types.contains(&GeneratorType::Boolean));
        assert!(types.contains(&GeneratorType::Choice));
        assert!(types.contains(&GeneratorType::Counter));
        assert!(types.contains(&GeneratorType::Template));
        assert!(types.contains(&GeneratorType::List));
        assert!(types.contains(&GeneratorType::Regex));
        assert!(types.contains(&GeneratorType::Address));
        assert!(types.contains(&GeneratorType::Name));
        assert!(types.contains(&GeneratorType::Gender));
    }

    #[test]
    fn test_generator_metadata_access() {
        let registry = GeneratorRegistry::new();

        let string_meta = registry.get_metadata(&GeneratorType::String).unwrap();
        assert_eq!(string_meta.display_name, "String Generator");
        assert!(!string_meta.can_have_dependencies);
        assert!(!string_meta.parameters.is_empty());

        let template_meta = registry.get_metadata(&GeneratorType::Template).unwrap();
        assert_eq!(template_meta.display_name, "Template Generator");
        assert!(template_meta.can_have_dependencies);
    }

    #[test]
    fn test_create_generator_from_ast() {
        let registry = GeneratorRegistry::new();

        let ast_gen = Generator::String(StringGenerator {
            length: Some(10),
            min_length: None,
            max_length: None,
            charset: CharacterSet::Alpha,
            case: Case::Lower,
            pattern: None,
            span: None,
        });

        let generator = registry.create_from_ast(&ast_gen).unwrap();
        assert_eq!(generator.generator_type(), GeneratorType::String);

        let context = Context::new();
        let result = generator.generate(&context).unwrap();
        assert_eq!(result.len(), 10);
        assert!(result.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_dependency_capable_types() {
        let registry = GeneratorRegistry::new();
        let dep_types = registry.dependency_capable_types();

        assert!(dep_types.contains(&GeneratorType::Template));
        assert!(dep_types.contains(&GeneratorType::List));
        assert!(!dep_types.contains(&GeneratorType::String));
        assert!(!dep_types.contains(&GeneratorType::Number));
    }

    #[test]
    fn test_documentation_generation() {
        let registry = GeneratorRegistry::new();
        let doc = registry.generate_documentation();

        assert!(doc.contains("# Dadagen Generator Types"));
        assert!(doc.contains("## String Generator"));
        assert!(doc.contains("## Template Generator"));
        assert!(doc.contains("### Parameters"));
        assert!(doc.contains("### Examples"));
    }

    #[test]
    fn test_create_for_field() {
        let registry = GeneratorRegistry::new();

        let document = DslDocument {
            fields: vec![
                FieldDefinition {
                    name: "user_id".to_string(),
                    generator: Generator::Counter(CounterGenerator {
                        start: Some(1000),
                        step: Some(1),
                        span: None,
                    }),
                    span: None,
                },
                FieldDefinition {
                    name: "username".to_string(),
                    generator: Generator::String(StringGenerator {
                        length: Some(8),
                        min_length: None,
                        max_length: None,
                        charset: CharacterSet::AlphaNumeric,
                        case: Case::Lower,
                        pattern: None,
                        span: None,
                    }),
                    span: None,
                },
            ],
            span: None,
        };

        // Create generator for user_id field
        let user_id_gen = registry.create_for_field(&document, "user_id").unwrap();
        assert_eq!(user_id_gen.generator_type(), GeneratorType::Counter);

        // Create generator for username field
        let username_gen = registry.create_for_field(&document, "username").unwrap();
        assert_eq!(username_gen.generator_type(), GeneratorType::String);

        // Try to create generator for non-existent field
        let result = registry.create_for_field(&document, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_all_generator_types_integration() {
        let registry = GeneratorRegistry::new();
        let context = Context::new();

        // Test String generator
        let string_gen = registry
            .create_from_ast(&Generator::String(StringGenerator {
                length: Some(5),
                min_length: None,
                max_length: None,
                charset: CharacterSet::Numeric,
                case: Case::Mixed,
                pattern: None,
                span: None,
            }))
            .unwrap();
        let string_result = string_gen.generate(&context).unwrap();
        assert_eq!(string_result.len(), 5);
        assert!(string_result.chars().all(|c| c.is_numeric()));

        // Test Number generator
        let number_gen = registry
            .create_from_ast(&Generator::Number(NumberGenerator {
                min: Some(10.0),
                max: Some(20.0),
                decimal_places: Some(0),
                distribution: Distribution::Uniform,
                seed: None,
                span: None,
            }))
            .unwrap();
        let number_result = number_gen.generate(&context).unwrap();
        let num: f64 = number_result.parse().unwrap();
        assert!(num >= 10.0 && num <= 20.0);

        // Test Boolean generator
        let bool_gen = registry
            .create_from_ast(&Generator::Boolean(BooleanGenerator {
                true_probability: 1.0,
                span: None,
            }))
            .unwrap();
        let bool_result = bool_gen.generate(&context).unwrap();
        assert_eq!(bool_result, "true");

        // Test Choice generator
        let choice_gen = registry
            .create_from_ast(&Generator::Choice(ChoiceGenerator {
                options: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                span: None,
            }))
            .unwrap();
        let choice_result = choice_gen.generate(&context).unwrap();
        assert!(["a", "b", "c"].contains(&choice_result.as_str()));

        // Test Counter generator
        let counter_gen = registry
            .create_from_ast(&Generator::Counter(CounterGenerator {
                start: Some(100),
                step: Some(5),
                span: None,
            }))
            .unwrap();
        let counter_result1 = counter_gen.generate(&context).unwrap();
        let counter_result2 = counter_gen.generate(&context).unwrap();
        assert_eq!(counter_result1, "100");
        assert_eq!(counter_result2, "105");

        // Test Template generator
        let template_gen = registry
            .create_from_ast(&Generator::Template(TemplateGenerator {
                template: "Hello World".to_string(),
                variables: vec![],
                span: None,
            }))
            .unwrap();
        let template_result = template_gen.generate(&context).unwrap();
        assert_eq!(template_result, "Hello World");

        // Test Regex generator
        let regex_gen = registry
            .create_from_ast(&Generator::Regex(RegexGenerator {
                pattern: "[abc]".to_string(),
                span: None,
            }))
            .unwrap();
        let regex_result = regex_gen.generate(&context).unwrap();
        assert!(["a", "b", "c"].contains(&regex_result.as_str()));

        // Test Address generator
        let address_gen = registry
            .create_from_ast(&Generator::Address(AddressGenerator {
                component: AddressComponent::CityTown,
                span: None,
            }))
            .unwrap();
        let address_result = address_gen.generate(&context).unwrap();
        assert!(!address_result.is_empty());

        // Test Name generator
        let name_gen = registry
            .create_from_ast(&Generator::Name(NameGenerator {
                name_type: NameType::GivenName,
                span: None,
            }))
            .unwrap();
        let name_result = name_gen.generate(&context).unwrap();
        assert!(!name_result.is_empty());

        // Test Gender generator
        let gender_gen = registry
            .create_from_ast(&Generator::Gender(GenderGenerator { span: None }))
            .unwrap();
        let gender_result = gender_gen.generate(&context).unwrap();
        assert!(!gender_result.is_empty());
    }

    #[test]
    fn test_metadata_completeness() {
        let registry = GeneratorRegistry::new();
        let all_meta = registry.all_metadata();

        // Ensure all metadata has required fields
        for meta in all_meta {
            assert!(!meta.display_name.is_empty());
            assert!(!meta.description.is_empty());

            // Parameters should have proper metadata
            for param in &meta.parameters {
                assert!(!param.name.is_empty());
                assert!(!param.param_type.is_empty());
                assert!(!param.description.is_empty());
            }

            // Should have at least one example
            assert!(!meta.examples.is_empty());
        }
    }

    #[test]
    fn test_cache_operations() {
        let registry = GeneratorRegistry::new();

        // Clear cache should succeed
        assert!(registry.clear_cache().is_ok());
    }
}
