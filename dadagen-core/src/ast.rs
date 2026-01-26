//! Abstract Syntax Tree (AST) structures for the dadagen DSL
//!
//! This module defines the complete AST representation of the dadagen DSL,
//! including all generator types, constraints, and metadata for error reporting.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Location information for error reporting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }
}

/// Root AST node representing the complete DSL document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DslDocument {
    pub fields: Vec<FieldDefinition>,
    pub span: Option<Span>,
}

/// A single field definition in the DSL
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub generator: Generator,
    pub span: Option<Span>,
}

/// Top-level generator enum covering all generator types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Generator {
    String(StringGenerator),
    Number(NumberGenerator),
    Boolean(BooleanGenerator),
    DateTime(DateTimeGenerator),
    Date(DateGenerator),
    Time(TimeGenerator),
    Choice(ChoiceGenerator),
    List(ListGenerator),
    Template(TemplateGenerator),
    Regex(RegexGenerator),
    Counter(CounterGenerator),
    Gender(GenderGenerator),
    Name(NameGenerator),
    Address(AddressGenerator),
}

/// String generator with constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringGenerator {
    pub length: Option<usize>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub charset: CharacterSet,
    pub case: Case,
    pub pattern: Option<String>,
    pub span: Option<Span>,
}

impl Default for StringGenerator {
    fn default() -> Self {
        Self {
            length: None,
            min_length: None,
            max_length: None,
            charset: CharacterSet::AlphaNumeric,
            case: Case::Mixed,
            pattern: None,
            span: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CharacterSet {
    Alpha,
    Numeric,
    AlphaNumeric,
    Ascii,
    Hex,
    Base64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Case {
    Lower,
    Upper,
    Title,
    Mixed,
}

/// Number generator supporting both integer and floating-point
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberGenerator {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub decimal_places: Option<usize>,
    pub distribution: Distribution,
    pub seed: Option<u64>,
    pub span: Option<Span>,
}

impl Default for NumberGenerator {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
            decimal_places: None,
            distribution: Distribution::Uniform,
            seed: None,
            span: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Distribution {
    Uniform,
    Normal { mean: f64, std_dev: f64 },
    Exponential { lambda: f64 },
}

/// Boolean generator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BooleanGenerator {
    pub true_probability: f64,
    pub span: Option<Span>,
}

impl Default for BooleanGenerator {
    fn default() -> Self {
        Self {
            true_probability: 0.5,
            span: None,
        }
    }
}

/// DateTime generator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateTimeGenerator {
    pub start: Option<String>,
    pub end: Option<String>,
    pub format: Option<String>,
    pub span: Option<Span>,
}

impl Default for DateTimeGenerator {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            format: None,
            span: None,
        }
    }
}

/// Date generator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateGenerator {
    pub start: Option<String>,
    pub end: Option<String>,
    pub format: Option<String>,
    pub span: Option<Span>,
}

impl Default for DateGenerator {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            format: None,
            span: None,
        }
    }
}

/// Time generator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeGenerator {
    pub start: Option<String>,
    pub end: Option<String>,
    pub format: Option<String>,
    pub span: Option<Span>,
}

impl Default for TimeGenerator {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            format: None,
            span: None,
        }
    }
}

/// Choice/Enum generator - picks from a list of options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceGenerator {
    pub options: Vec<String>,
    pub span: Option<Span>,
}

/// List generator - picks from a named list with optional discriminator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListGenerator {
    pub name: String,
    pub discriminator: Option<String>,
    pub weighted: bool,
    pub mode: ListMode,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListMode {
    Random,
    Sequential,
}

/// Template generator with variable substitution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateGenerator {
    pub template: String,
    pub variables: Vec<TemplateVariable>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub generator: Option<Box<Generator>>,
}

/// Regex-based string generator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegexGenerator {
    pub pattern: String,
    pub span: Option<Span>,
}

/// Counter/iteration generator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CounterGenerator {
    pub start: Option<i64>,
    pub step: Option<i64>,
    pub span: Option<Span>,
}

impl Default for CounterGenerator {
    fn default() -> Self {
        Self {
            start: Some(0),
            step: Some(1),
            span: None,
        }
    }
}

/// Gender generator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenderGenerator {
    pub span: Option<Span>,
}

/// Name generator (given name or surname)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NameGenerator {
    pub name_type: NameType,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NameType {
    GivenName,
    Surname,
    Full,
}

/// Address component generator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressGenerator {
    pub component: AddressComponent,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressComponent {
    CityTown,
    Suburb,
    Street,
    Property,
    PostZipCode,
    StateCounty,
    Country,
}

/// Validation errors that can occur during AST construction or semantic analysis
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum AstError {
    #[error("Invalid constraint: {message} at {span:?}")]
    InvalidConstraint { message: String, span: Option<Span> },

    #[error("Type mismatch: expected {expected}, found {found} at {span:?}")]
    TypeMismatch {
        expected: String,
        found: String,
        span: Option<Span>,
    },

    #[error("Undefined reference: {name} at {span:?}")]
    UndefinedReference { name: String, span: Option<Span> },

    #[error("Duplicate field definition: {name} at {span:?}")]
    DuplicateField { name: String, span: Option<Span> },

    #[error("Invalid value: {message} at {span:?}")]
    InvalidValue { message: String, span: Option<Span> },
}

pub type AstResult<T> = Result<T, AstError>;

/// Trait for AST nodes to support validation
pub trait Validate {
    fn validate(&self) -> AstResult<()>;
}

impl Validate for DslDocument {
    fn validate(&self) -> AstResult<()> {
        let mut field_names = HashMap::new();
        
        for field in &self.fields {
            // Check for duplicate field names
            if let Some(_prev_span) = field_names.insert(&field.name, &field.span) {
                return Err(AstError::DuplicateField {
                    name: field.name.clone(),
                    span: field.span.clone(),
                });
            }
            
            // Validate individual field
            field.validate()?;
        }
        
        Ok(())
    }
}

impl Validate for FieldDefinition {
    fn validate(&self) -> AstResult<()> {
        // Validate field name is not empty
        if self.name.is_empty() {
            return Err(AstError::InvalidValue {
                message: "Field name cannot be empty".to_string(),
                span: self.span.clone(),
            });
        }
        
        // Validate generator
        self.generator.validate()
    }
}

impl Validate for Generator {
    fn validate(&self) -> AstResult<()> {
        match self {
            Generator::String(g) => g.validate(),
            Generator::Number(g) => g.validate(),
            Generator::Boolean(g) => g.validate(),
            Generator::DateTime(g) => g.validate(),
            Generator::Date(g) => g.validate(),
            Generator::Time(g) => g.validate(),
            Generator::Choice(g) => g.validate(),
            Generator::List(g) => g.validate(),
            Generator::Template(g) => g.validate(),
            Generator::Regex(g) => g.validate(),
            Generator::Counter(g) => g.validate(),
            Generator::Gender(g) => g.validate(),
            Generator::Name(g) => g.validate(),
            Generator::Address(g) => g.validate(),
        }
    }
}

impl Validate for StringGenerator {
    fn validate(&self) -> AstResult<()> {
        // Validate length constraints are consistent
        if let (Some(min), Some(max)) = (self.min_length, self.max_length) {
            if min > max {
                return Err(AstError::InvalidConstraint {
                    message: format!("min_length ({}) cannot be greater than max_length ({})", min, max),
                    span: self.span.clone(),
                });
            }
        }
        
        if let Some(_length) = self.length {
            if self.min_length.is_some() || self.max_length.is_some() {
                return Err(AstError::InvalidConstraint {
                    message: "Cannot specify both 'length' and 'min_length'/'max_length'".to_string(),
                    span: self.span.clone(),
                });
            }
        }
        
        Ok(())
    }
}

impl Validate for NumberGenerator {
    fn validate(&self) -> AstResult<()> {
        // Validate min < max
        if let (Some(min), Some(max)) = (self.min, self.max) {
            if min > max {
                return Err(AstError::InvalidConstraint {
                    message: format!("min ({}) cannot be greater than max ({})", min, max),
                    span: self.span.clone(),
                });
            }
        }
        
        Ok(())
    }
}

impl Validate for BooleanGenerator {
    fn validate(&self) -> AstResult<()> {
        // Validate probability is in [0, 1]
        if !(0.0..=1.0).contains(&self.true_probability) {
            return Err(AstError::InvalidConstraint {
                message: format!("true_probability ({}) must be between 0.0 and 1.0", self.true_probability),
                span: self.span.clone(),
            });
        }
        Ok(())
    }
}

impl Validate for DateTimeGenerator {
    fn validate(&self) -> AstResult<()> {
        // TODO: Add datetime format validation
        Ok(())
    }
}

impl Validate for DateGenerator {
    fn validate(&self) -> AstResult<()> {
        // TODO: Add date format validation
        Ok(())
    }
}

impl Validate for TimeGenerator {
    fn validate(&self) -> AstResult<()> {
        // TODO: Add time format validation
        Ok(())
    }
}

impl Validate for ChoiceGenerator {
    fn validate(&self) -> AstResult<()> {
        if self.options.is_empty() {
            return Err(AstError::InvalidConstraint {
                message: "Choice generator must have at least one option".to_string(),
                span: self.span.clone(),
            });
        }
        Ok(())
    }
}

impl Validate for ListGenerator {
    fn validate(&self) -> AstResult<()> {
        if self.name.is_empty() {
            return Err(AstError::InvalidConstraint {
                message: "List generator must specify a list name".to_string(),
                span: self.span.clone(),
            });
        }
        Ok(())
    }
}

impl Validate for TemplateGenerator {
    fn validate(&self) -> AstResult<()> {
        if self.template.is_empty() {
            return Err(AstError::InvalidConstraint {
                message: "Template cannot be empty".to_string(),
                span: self.span.clone(),
            });
        }
        Ok(())
    }
}

impl Validate for RegexGenerator {
    fn validate(&self) -> AstResult<()> {
        if self.pattern.is_empty() {
            return Err(AstError::InvalidConstraint {
                message: "Regex pattern cannot be empty".to_string(),
                span: self.span.clone(),
            });
        }
        // TODO: Add regex syntax validation
        Ok(())
    }
}

impl Validate for CounterGenerator {
    fn validate(&self) -> AstResult<()> {
        if let Some(step) = self.step {
            if step == 0 {
                return Err(AstError::InvalidConstraint {
                    message: "Counter step cannot be zero".to_string(),
                    span: self.span.clone(),
                });
            }
        }
        Ok(())
    }
}

impl Validate for GenderGenerator {
    fn validate(&self) -> AstResult<()> {
        Ok(())
    }
}

impl Validate for NameGenerator {
    fn validate(&self) -> AstResult<()> {
        Ok(())
    }
}

impl Validate for AddressGenerator {
    fn validate(&self) -> AstResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_generator_validation() {
        let mut gen = StringGenerator::default();
        gen.min_length = Some(10);
        gen.max_length = Some(5);
        
        assert!(gen.validate().is_err());
        
        gen.max_length = Some(20);
        assert!(gen.validate().is_ok());
    }

    #[test]
    fn test_number_generator_validation() {
        let mut gen = NumberGenerator::default();
        gen.min = Some(100.0);
        gen.max = Some(50.0);
        
        assert!(gen.validate().is_err());
        
        gen.max = Some(200.0);
        assert!(gen.validate().is_ok());
    }

    #[test]
    fn test_boolean_generator_validation() {
        let mut gen = BooleanGenerator::default();
        gen.true_probability = 1.5;
        
        assert!(gen.validate().is_err());
        
        gen.true_probability = 0.75;
        assert!(gen.validate().is_ok());
    }

    #[test]
    fn test_choice_generator_validation() {
        let gen = ChoiceGenerator {
            options: vec![],
            span: None,
        };
        
        assert!(gen.validate().is_err());
        
        let gen = ChoiceGenerator {
            options: vec!["option1".to_string(), "option2".to_string()],
            span: None,
        };
        
        assert!(gen.validate().is_ok());
    }

    #[test]
    fn test_document_duplicate_fields() {
        let doc = DslDocument {
            fields: vec![
                FieldDefinition {
                    name: "field1".to_string(),
                    generator: Generator::Boolean(BooleanGenerator::default()),
                    span: None,
                },
                FieldDefinition {
                    name: "field1".to_string(),
                    generator: Generator::Boolean(BooleanGenerator::default()),
                    span: None,
                },
            ],
            span: None,
        };
        
        assert!(doc.validate().is_err());
    }
}
