//! Template-based string generation

use crate::context::Context;
use crate::errors::{Result, DadagenError};
use crate::generators::core::Generator;
use crate::generators::number::IntegerGenerator;
use std::collections::HashMap;
use regex::Regex;

/// Template engine for complex string generation
#[derive(Debug, Clone)]
pub struct TemplateGenerator {
    name: String,
    template: String,
    dependencies: Vec<String>,
    #[allow(dead_code)]
    functions: HashMap<String, TemplateFunction>,
}

/// Built-in template functions
#[derive(Debug, Clone)]
pub enum TemplateFunction {
    Random { min: i32, max: i32 },
    Choose { options: Vec<String> },
    Format { pattern: String },
    Conditional { condition: String, true_value: String, false_value: String },
}

impl TemplateGenerator {
    pub fn new(name: String, template: String) -> Self {
        let mut generator = Self {
            name,
            template,
            dependencies: vec![],
            functions: HashMap::new(),
        };
        
        // Register built-in functions
        generator.register_builtin_functions();
        generator.extract_dependencies();
        generator
    }
    
    fn register_builtin_functions(&mut self) {
        // Functions will be parsed from template
    }
    
    fn extract_dependencies(&mut self) {
        // Extract field references from template
        let field_regex = Regex::new(r"\{\{(\w+)\}\}").unwrap();
        for cap in field_regex.captures_iter(&self.template) {
            if let Some(field) = cap.get(1) {
                let field_name = field.as_str().to_string();
                if !self.dependencies.contains(&field_name) {
                    self.dependencies.push(field_name);
                }
            }
        }
        
        // Extract function calls that might reference fields
        let func_regex = Regex::new(r"\{\{(\w+)\((.*?)\)\}\}").unwrap();
        for cap in func_regex.captures_iter(&self.template) {
            if let Some(args) = cap.get(2) {
                // Parse arguments for field references
                let arg_field_regex = Regex::new(r"\b(\w+)\b").unwrap();
                for arg_cap in arg_field_regex.captures_iter(args.as_str()) {
                    if let Some(field) = arg_cap.get(1) {
                        let field_name = field.as_str().to_string();
                        // Only add if it's not a function name or literal
                        if !["random", "choose", "format", "if"].contains(&field_name.as_str()) &&
                           !field_name.chars().all(|c| c.is_ascii_digit()) &&
                           !self.dependencies.contains(&field_name) {
                            self.dependencies.push(field_name);
                        }
                    }
                }
            }
        }
    }
    
    /// Process template with context
    /// Template syntax:
    /// - {{field_name}} - Direct field reference
    /// - {{random(1, 100)}} - Random number between 1 and 100
    /// - {{choose("a", "b", "c")}} - Choose from list
    /// - {{format("{}", field_name)}} - Format field value
    /// - {{if(condition, "true_val", "false_val")}} - Conditional
    fn process_template(&self, template: &str, context: &Context) -> Result<String> {
        let mut result = template.to_string();
        
        // Process field references first
        let field_regex = Regex::new(r"\{\{(\w+)\}\}").unwrap();
        result = field_regex.replace_all(&result, |caps: &regex::Captures| {
            let field_name = &caps[1];
            match context.get_field_state::<String>(field_name) {
                Ok(Some(value)) => format!("{}", value),
                Ok(None) => format!("{{ERROR: Field '{}' not found}}", field_name),
                Err(_) => format!("{{ERROR: Field '{}' not found}}", field_name),
            }
        }).to_string();
        
        // Process function calls
        let func_regex = Regex::new(r"\{\{(\w+)\((.*?)\)\}\}").unwrap();
        result = func_regex.replace_all(&result, |caps: &regex::Captures| {
            let func_name = &caps[1];
            let args = &caps[2];
            
            match self.execute_function(func_name, args, context) {
                Ok(value) => value,
                Err(e) => format!("{{ERROR: {}}}", e),
            }
        }).to_string();
        
        // Check for any remaining errors
        if result.contains("{{ERROR:") {
            return Err(DadagenError::GenerationError { message: format!("Template processing failed: {}", result) });
        }
        
        Ok(result)
    }
    
    fn execute_function(&self, func_name: &str, args: &str, context: &Context) -> Result<String> {
        match func_name {
            "random" => {
                let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                if parts.len() != 2 {
                    return Err(DadagenError::GenerationError { message: "random() requires exactly 2 arguments: min, max".to_string() });
                }
                
                let min: i64 = parts[0].parse().map_err(|_| 
                    DadagenError::GenerationError { message: "Invalid min value for random()".to_string() }
                )?;
                let max: i64 = parts[1].parse().map_err(|_| 
                    DadagenError::GenerationError { message: "Invalid max value for random()".to_string() }
                )?;
                
                let generator = IntegerGenerator::new("temp_random".to_string())
                    .with_range(min, max);
                let value = generator.generate(context)?;
                Ok(value.to_string())
            },
            "choose" => {
                // Parse quoted strings
                let options = self.parse_quoted_args(args)?;
                if options.is_empty() {
                    return Err(DadagenError::GenerationError { message: "choose() requires at least one option".to_string() });
                }
                
                let generator = crate::generators::string::ListGenerator::new(
                    "temp_choose".to_string(),
                    options
                );
                generator.generate(context)
            },
            "format" => {
                let parts: Vec<&str> = args.splitn(2, ',').map(|s| s.trim()).collect();
                if parts.len() != 2 {
                    return Err(DadagenError::GenerationError { message: "format() requires exactly 2 arguments: pattern, value".to_string() });
                }
                
                let pattern = parts[0].trim_matches('"');
                let value_name = parts[1];
                
                match context.get_field_state::<String>(value_name) {
                    Ok(Some(value)) => Ok(pattern.replace("{}", &format!("{}", value))),
                    Ok(None) => Err(DadagenError::GenerationError { message: format!("Field '{}' not found for format()", value_name) }),
                    Err(e) => Err(e),
                }
            },
            "if" => {
                let parts: Vec<&str> = args.splitn(3, ',').map(|s| s.trim()).collect();
                if parts.len() != 3 {
                    return Err(DadagenError::GenerationError { message: "if() requires exactly 3 arguments: condition, true_value, false_value".to_string() });
                }
                
                let condition = parts[0];
                let true_value = parts[1].trim_matches('"');
                let false_value = parts[2].trim_matches('"');
                
                // Simple condition evaluation (can be extended)
                let result = if self.evaluate_condition(condition, context)? {
                    true_value.to_string()
                } else {
                    false_value.to_string()
                };
                
                Ok(result)
            },
            _ => Err(DadagenError::GenerationError { message: format!("Unknown template function: {}", func_name) }),
        }
    }
    
    fn parse_quoted_args(&self, args: &str) -> Result<Vec<String>> {
        let mut options = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = args.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    if !in_quotes && !current.is_empty() {
                        options.push(current.clone());
                        current.clear();
                    }
                },
                ',' if !in_quotes => {
                    if !current.is_empty() {
                        options.push(current.trim().to_string());
                        current.clear();
                    }
                },
                ch if in_quotes => current.push(ch),
                ch if !ch.is_whitespace() => current.push(ch),
                _ => {}, // Skip whitespace outside quotes
            }
        }
        
        if !current.is_empty() {
            options.push(current.trim().to_string());
        }
        
        Ok(options)
    }
    
    fn evaluate_condition(&self, condition: &str, context: &Context) -> Result<bool> {
        // Simple condition evaluation - can be extended for complex expressions
        if let Ok(Some(value)) = context.get_field_state::<String>(condition) {
            // Convert to boolean based on value
            match value.to_string().to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Ok(true),
                "false" | "0" | "no" | "off" | "" => Ok(false),
                _ => {
                    // For numbers, non-zero is true
                    if let Ok(num) = value.to_string().parse::<f64>() {
                        Ok(num != 0.0)
                    } else {
                        // For strings, non-empty is true
                        Ok(!value.to_string().is_empty())
                    }
                }
            }
        } else {
            Ok(false) // Field not found evaluates to false
        }
    }
}

impl Generator<String> for TemplateGenerator {
    fn generate(&self, context: &Context) -> Result<String> {
        self.process_template(&self.template, context)
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Composite generator that combines multiple generators
#[derive(Debug)]
pub struct CompositeGenerator {
    name: String,
    generators: Vec<(String, Box<dyn Generator<String>>)>,
    template: String,
    dependencies: Vec<String>,
}

impl CompositeGenerator {
    pub fn new(name: String, template: String) -> Self {
        Self {
            name,
            generators: vec![],
            template,
            dependencies: vec![],
        }
    }
    
    pub fn add_generator<G>(mut self, name: String, generator: G) -> Self 
    where 
        G: Generator<String> + 'static
    {
        self.dependencies.extend(generator.dependencies());
        self.generators.push((name, Box::new(generator)));
        self
    }
}

// Note: This implementation is simplified for the current trait system
// In a more advanced version, we might need trait objects or an enum approach
impl Generator<String> for CompositeGenerator {
    fn generate(&self, context: &Context) -> Result<String> {
        // Create a local context with generated values
        let local_context = context.clone();
        
        // Generate values from sub-generators
        for (name, generator) in &self.generators {
            let value = generator.generate(context)?;
            let _ = local_context.insert_field_state(name.clone(), value);
        }
        
        // Process the template with the enhanced context
        let template_gen = TemplateGenerator::new(
            format!("{}_template", self.name),
            self.template.clone()
        );
        
        template_gen.generate(&local_context)
    }
    
    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}
