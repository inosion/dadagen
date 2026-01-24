//! Generator registry and factory for dynamic generator creation

use crate::generators::{Generator, GeneratorMetadata};
use crate::context::Context;
use crate::errors::{Result, DadagenError};
use std::collections::HashMap;
use std::sync::Arc;
use std::fmt::Debug;

/// Generated value types that can be type-erased
#[derive(Debug, Clone, PartialEq)]
pub enum GeneratedValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<GeneratedValue>),
}

impl GeneratedValue {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            GeneratedValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            GeneratedValue::Integer(i) => Some(*i),
            _ => None,
        }
    }
    
    pub fn as_float(&self) -> Option<f64> {
        match self {
            GeneratedValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            GeneratedValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_list(&self) -> Option<&Vec<GeneratedValue>> {
        match self {
            GeneratedValue::List(l) => Some(l),
            _ => None,
        }
    }
}

/// Type alias for boxed generator trait objects
pub type BoxedGenerator = Box<dyn ErasedGenerator>;

/// Type-erased generator trait for dynamic dispatch
pub trait ErasedGenerator: Debug + Send + Sync {
    fn generate_value(&self, context: &Context) -> Result<GeneratedValue>;
    fn dependencies(&self) -> Vec<String>;
    fn name(&self) -> &str;
    fn metadata(&self) -> GeneratorMetadata;
}

/// Registry for all available generator types
pub struct GeneratorRegistry {
    registry: HashMap<String, Arc<dyn GeneratorFactory + Send + Sync>>,
}

impl GeneratorRegistry {
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }
    
    /// Register a generator factory by name
    pub fn register_factory(&mut self, name: &str, factory: Arc<dyn GeneratorFactory + Send + Sync>) {
        self.registry.insert(name.to_string(), factory);
    }
    
    /// Create a generator from DSL type name and config
    pub fn create_generator(&self, type_name: &str, config: &str) -> Result<BoxedGenerator> {
        if let Some(factory) = self.registry.get(type_name) {
            factory.create(config)
        } else {
            Err(DadagenError::GenerationError { message: format!("Unknown generator type: {}", type_name) })
        }
    }
    
    /// List all registered generator types
    pub fn list_types(&self) -> Vec<String> {
        self.registry.keys().cloned().collect()
    }
    
    /// Get generator metadata for all types
    pub fn metadata(&self) -> Vec<GeneratorMetadata> {
        self.registry.values().map(|f| f.metadata()).collect()
    }
}

/// Factory trait for generator creation
pub trait GeneratorFactory: Send + Sync {
    fn create(&self, config: &str) -> Result<BoxedGenerator>;
    fn metadata(&self) -> GeneratorMetadata;
}

/// Wrapper for concrete generators to implement ErasedGenerator
pub struct GeneratorWrapper<T, G> 
where 
    T: Clone + Send + Sync + 'static,
    G: Generator<T>
{
    inner: G,
    _marker: std::marker::PhantomData<T>,
}

impl<T, G> GeneratorWrapper<T, G>
where 
    T: Clone + Send + Sync + 'static,
    G: Generator<T>
{
    pub fn new(generator: G) -> Self {
        Self {
            inner: generator,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, G> std::fmt::Debug for GeneratorWrapper<T, G>
where 
    T: Clone + Send + Sync + 'static,
    G: Generator<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GeneratorWrapper({})", self.inner.name())
    }
}

impl<G> ErasedGenerator for GeneratorWrapper<String, G>
where 
    G: Generator<String>
{
    fn generate_value(&self, context: &Context) -> Result<GeneratedValue> {
        self.inner.generate(context).map(GeneratedValue::String)
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.inner.dependencies()
    }
    
    fn name(&self) -> &str {
        self.inner.name()
    }
    
    fn metadata(&self) -> GeneratorMetadata {
        self.inner.metadata()
    }
}

impl<G> ErasedGenerator for GeneratorWrapper<i64, G>
where 
    G: Generator<i64>
{
    fn generate_value(&self, context: &Context) -> Result<GeneratedValue> {
        self.inner.generate(context).map(GeneratedValue::Integer)
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.inner.dependencies()
    }
    
    fn name(&self) -> &str {
        self.inner.name()
    }
    
    fn metadata(&self) -> GeneratorMetadata {
        self.inner.metadata()
    }
}

impl<G> ErasedGenerator for GeneratorWrapper<f64, G>
where 
    G: Generator<f64>
{
    fn generate_value(&self, context: &Context) -> Result<GeneratedValue> {
        self.inner.generate(context).map(GeneratedValue::Float)
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.inner.dependencies()
    }
    
    fn name(&self) -> &str {
        self.inner.name()
    }
    
    fn metadata(&self) -> GeneratorMetadata {
        self.inner.metadata()
    }
}

impl<G> ErasedGenerator for GeneratorWrapper<bool, G>
where 
    G: Generator<bool>
{
    fn generate_value(&self, context: &Context) -> Result<GeneratedValue> {
        self.inner.generate(context).map(GeneratedValue::Boolean)
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.inner.dependencies()
    }
    
    fn name(&self) -> &str {
        self.inner.name()
    }
    
    fn metadata(&self) -> GeneratorMetadata {
        self.inner.metadata()
    }
}

/// Example: Registering a StringGenerator factory
pub struct StringGeneratorFactory;

impl GeneratorFactory for StringGeneratorFactory {
    fn create(&self, _config: &str) -> Result<BoxedGenerator> {
        // Parse config string and create generator
        // For now, just create default
        let gen = crate::generators::StringGenerator::new("string".to_string());
        Ok(Box::new(GeneratorWrapper::new(gen)))
    }
    fn metadata(&self) -> GeneratorMetadata {
        GeneratorMetadata {
            name: "string".to_string(),
            generator_type: "StringGenerator".to_string(),
            dependencies: vec![],
        }
    }
}

/// Example: Registering an IntegerGenerator factory
pub struct IntegerGeneratorFactory;

impl GeneratorFactory for IntegerGeneratorFactory {
    fn create(&self, _config: &str) -> Result<BoxedGenerator> {
        let gen = crate::generators::IntegerGenerator::new("integer".to_string());
        Ok(Box::new(GeneratorWrapper::new(gen)))
    }
    fn metadata(&self) -> GeneratorMetadata {
        GeneratorMetadata {
            name: "integer".to_string(),
            generator_type: "IntegerGenerator".to_string(),
            dependencies: vec![],
        }
    }
}

// ... Add more factories for other generator types as needed ...

/// Float generator factory
pub struct FloatGeneratorFactory;

impl GeneratorFactory for FloatGeneratorFactory {
    fn create(&self, _config: &str) -> Result<BoxedGenerator> {
        let gen = crate::generators::FloatGenerator::new("float".to_string());
        Ok(Box::new(GeneratorWrapper::new(gen)))
    }
    fn metadata(&self) -> GeneratorMetadata {
        GeneratorMetadata {
            name: "float".to_string(),
            generator_type: "FloatGenerator".to_string(),
            dependencies: vec![],
        }
    }
}

/// Boolean generator factory  
pub struct BooleanGeneratorFactory;

impl GeneratorFactory for BooleanGeneratorFactory {
    fn create(&self, _config: &str) -> Result<BoxedGenerator> {
        // Create a simple boolean generator using IntegerGenerator with range 0-1
        let gen = crate::generators::IntegerGenerator::new("boolean".to_string())
            .with_range(0, 1);
        Ok(Box::new(GeneratorWrapper::new(gen)))
    }
    fn metadata(&self) -> GeneratorMetadata {
        GeneratorMetadata {
            name: "boolean".to_string(),
            generator_type: "BooleanGenerator".to_string(),
            dependencies: vec![],
        }
    }
}

/// Template generator factory with configurable templates
pub struct TemplateGeneratorFactory;

impl GeneratorFactory for TemplateGeneratorFactory {
    fn create(&self, config: &str) -> Result<BoxedGenerator> {
        // Parse config to extract template pattern
        let template = if config.is_empty() {
            "Generated-{random}".to_string()
        } else {
            config.to_string()
        };
        
        let gen = crate::generators::TemplateGenerator::new(
            "template".to_string(),
            template,
        );
        Ok(Box::new(GeneratorWrapper::new(gen)))
    }
    fn metadata(&self) -> GeneratorMetadata {
        GeneratorMetadata {
            name: "template".to_string(),
            generator_type: "TemplateGenerator".to_string(),
            dependencies: vec!["context".to_string()],
        }
    }
}

/// Registry builder for convenient setup
pub struct RegistryBuilder {
    registry: GeneratorRegistry,
}

impl RegistryBuilder {
    pub fn new() -> Self {
        Self {
            registry: GeneratorRegistry::new(),
        }
    }
    
    /// Register all standard generator types
    pub fn with_standard_generators(mut self) -> Self {
        self.registry.register_factory("string", Arc::new(StringGeneratorFactory));
        self.registry.register_factory("integer", Arc::new(IntegerGeneratorFactory));
        self.registry.register_factory("float", Arc::new(FloatGeneratorFactory));
        self.registry.register_factory("boolean", Arc::new(BooleanGeneratorFactory));
        self.registry.register_factory("template", Arc::new(TemplateGeneratorFactory));
        self
    }
    
    /// Register a custom generator factory
    pub fn with_factory<F>(mut self, name: &str, factory: F) -> Self 
    where 
        F: GeneratorFactory + Send + Sync + 'static
    {
        self.registry.register_factory(name, Arc::new(factory));
        self
    }
    
    /// Build the final registry
    pub fn build(self) -> GeneratorRegistry {
        self.registry
    }
}

impl Default for RegistryBuilder {
    fn default() -> Self {
        Self::new().with_standard_generators()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;

    #[test]
    fn test_registry_register_and_create() {
        let mut registry = GeneratorRegistry::new();
        registry.register_factory("string", Arc::new(StringGeneratorFactory));
        registry.register_factory("integer", Arc::new(IntegerGeneratorFactory));
        
        let gen = registry.create_generator("string", "").unwrap();
        assert_eq!(gen.name(), "string");
        let gen2 = registry.create_generator("integer", "").unwrap();
        assert_eq!(gen2.name(), "integer");
        assert!(registry.list_types().contains(&"string".to_string()));
        assert!(registry.list_types().contains(&"integer".to_string()));
    }

    #[test]
    fn test_generator_value_generation() {
        let mut registry = GeneratorRegistry::new();
        registry.register_factory("string", Arc::new(StringGeneratorFactory));
        registry.register_factory("integer", Arc::new(IntegerGeneratorFactory));
        
        let context = Context::new();
        
        // Test string generator
        let string_gen = registry.create_generator("string", "").unwrap();
        let string_value = string_gen.generate_value(&context).unwrap();
        match string_value {
            GeneratedValue::String(s) => {
                assert!(!s.is_empty(), "Generated string should not be empty");
            },
            _ => panic!("Expected GeneratedValue::String"),
        }
        
        // Test integer generator
        let int_gen = registry.create_generator("integer", "").unwrap();
        let int_value = int_gen.generate_value(&context).unwrap();
        match int_value {
            GeneratedValue::Integer(i) => {
                assert!(i >= 0, "Generated integer should be non-negative");
            },
            _ => panic!("Expected GeneratedValue::Integer"),
        }
    }

    #[test]
    fn test_generator_metadata_collection() {
        let mut registry = GeneratorRegistry::new();
        registry.register_factory("string", Arc::new(StringGeneratorFactory));
        registry.register_factory("integer", Arc::new(IntegerGeneratorFactory));
        
        let metadata = registry.metadata();
        assert_eq!(metadata.len(), 2);
        
        let names: Vec<&str> = metadata.iter().map(|m| m.name.as_str()).collect();
        assert!(names.contains(&"string"));
        assert!(names.contains(&"integer"));
    }

    #[test]
    fn test_unknown_generator_type() {
        let registry = GeneratorRegistry::new();
        let result = registry.create_generator("unknown", "");
        assert!(result.is_err());
        
        if let Err(DadagenError::GenerationError { message }) = result {
            assert!(message.contains("Unknown generator type"));
        } else {
            panic!("Expected GenerationError");
        }
    }

    #[test]
    fn test_generated_value_type_accessors() {
        // Test string accessor
        let string_val = GeneratedValue::String("test".to_string());
        assert_eq!(string_val.as_string(), Some(&"test".to_string()));
        assert_eq!(string_val.as_integer(), None);
        assert_eq!(string_val.as_float(), None);
        assert_eq!(string_val.as_boolean(), None);
        assert_eq!(string_val.as_list(), None);
        
        // Test integer accessor
        let int_val = GeneratedValue::Integer(42);
        assert_eq!(int_val.as_integer(), Some(42));
        assert_eq!(int_val.as_string(), None);
        
        // Test float accessor
        let float_val = GeneratedValue::Float(3.14);
        assert_eq!(int_val.as_float(), None);
        assert_eq!(float_val.as_float(), Some(3.14));
        
        // Test boolean accessor
        let bool_val = GeneratedValue::Boolean(true);
        assert_eq!(bool_val.as_boolean(), Some(true));
        
        // Test list accessor
        let list_val = GeneratedValue::List(vec![
            GeneratedValue::String("item1".to_string()),
            GeneratedValue::Integer(123),
        ]);
        assert!(list_val.as_list().is_some());
        assert_eq!(list_val.as_list().unwrap().len(), 2);
    }

    #[test]
    fn test_registry_thread_safety() {
        use std::thread;
        use std::sync::Arc;
        
        let mut registry = GeneratorRegistry::new();
        registry.register_factory("string", Arc::new(StringGeneratorFactory));
        let registry = Arc::new(registry);
        
        let handles: Vec<_> = (0..10).map(|_| {
            let registry = Arc::clone(&registry);
            thread::spawn(move || {
                let context = Context::new();
                let gen = registry.create_generator("string", "").unwrap();
                let value = gen.generate_value(&context).unwrap();
                matches!(value, GeneratedValue::String(_))
            })
        }).collect();
        
        for handle in handles {
            assert!(handle.join().unwrap(), "Thread should successfully generate values");
        }
    }

    #[test]
    fn test_registry_builder() {
        let registry = RegistryBuilder::new()
            .with_standard_generators()
            .build();
        
        let types = registry.list_types();
        assert!(types.contains(&"string".to_string()));
        assert!(types.contains(&"integer".to_string()));
        assert!(types.contains(&"float".to_string()));
        assert!(types.contains(&"boolean".to_string()));
        assert!(types.contains(&"template".to_string()));
        
        // Test each generator type
        let context = Context::new();
        
        // String generator
        let string_gen = registry.create_generator("string", "").unwrap();
        let string_val = string_gen.generate_value(&context).unwrap();
        assert!(matches!(string_val, GeneratedValue::String(_)));
        
        // Integer generator  
        let int_gen = registry.create_generator("integer", "").unwrap();
        let int_val = int_gen.generate_value(&context).unwrap();
        assert!(matches!(int_val, GeneratedValue::Integer(_)));
        
        // Float generator
        let float_gen = registry.create_generator("float", "").unwrap();
        let float_val = float_gen.generate_value(&context).unwrap();
        assert!(matches!(float_val, GeneratedValue::Float(_)));
        
        // Boolean generator (using integer 0-1)
        let bool_gen = registry.create_generator("boolean", "").unwrap();
        let bool_val = bool_gen.generate_value(&context).unwrap();
        assert!(matches!(bool_val, GeneratedValue::Integer(i) if i == 0 || i == 1));
        
        // Template generator
        let template_gen = registry.create_generator("template", "Hello-{random}").unwrap();
        let template_val = template_gen.generate_value(&context).unwrap();
        assert!(matches!(template_val, GeneratedValue::String(_)));
    }

    #[test]
    fn test_default_registry_builder() {
        let registry = RegistryBuilder::default().build();
        
        // Should have all standard generators
        let types = registry.list_types();
        assert_eq!(types.len(), 5);
        assert!(types.contains(&"string".to_string()));
        assert!(types.contains(&"integer".to_string()));
        assert!(types.contains(&"float".to_string()));
        assert!(types.contains(&"boolean".to_string()));
        assert!(types.contains(&"template".to_string()));
    }

    #[test]
    fn test_template_generator_with_config() {
        let registry = RegistryBuilder::default().build();
        let context = Context::new();
        
        // Test default template
        let default_gen = registry.create_generator("template", "").unwrap();
        let default_val = default_gen.generate_value(&context).unwrap();
        if let GeneratedValue::String(s) = default_val {
            assert!(s.starts_with("Generated-"));
        } else {
            panic!("Expected string value from template generator");
        }
        
        // Test custom template
        let custom_gen = registry.create_generator("template", "User-{id}-Profile").unwrap();
        let custom_val = custom_gen.generate_value(&context).unwrap();
        if let GeneratedValue::String(s) = custom_val {
            assert!(s.contains("User-") && s.contains("-Profile"));
        } else {
            panic!("Expected string value from template generator");
        }
    }

    #[test]
    fn test_generator_dependencies() {
        let registry = RegistryBuilder::default().build();
        
        // Template generator with field dependencies should have dependencies
        let template_gen = registry.create_generator("template", "Hello ${user_name}, your ID is ${user_id}").unwrap();
        let deps = template_gen.dependencies();
        assert!(!deps.is_empty());
        assert!(deps.contains(&"user_name".to_string()));
        assert!(deps.contains(&"user_id".to_string()));
        
        // String generator should have no dependencies
        let string_gen = registry.create_generator("string", "").unwrap();
        let deps = string_gen.dependencies();
        assert!(deps.is_empty());
        
        // Template generator without field references should have no dependencies
        let simple_template_gen = registry.create_generator("template", "Simple static text").unwrap();
        let deps = simple_template_gen.dependencies();
        assert!(deps.is_empty());
    }
}
