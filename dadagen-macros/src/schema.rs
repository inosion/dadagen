//! Schema macro implementation

use syn::{parse::Parse, Token, Ident, Type, punctuated::Punctuated};
use quote::quote;
use proc_macro2::TokenStream;

/// Schema definition structure
pub struct SchemaDefinition {
    pub name: Ident,
    pub fields: Punctuated<SchemaField, Token![,]>,
}

/// Individual schema field
pub struct SchemaField {
    pub name: Ident,
    pub field_type: Type,
    pub generator_config: TokenStream,
}

impl Parse for SchemaDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        syn::braced!(content in input);
        let fields = content.parse_terminated(SchemaField::parse, Token![,])?;
        
        Ok(SchemaDefinition { name, fields })
    }
}

impl Parse for SchemaField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let field_type: Type = input.parse()?;
        input.parse::<Token![=]>()?;
        
        // Parse the generator configuration
        let generator_config = input.parse::<TokenStream>()?;
        
        Ok(SchemaField {
            name,
            field_type,
            generator_config,
        })
    }
}

/// Parse and expand schema macro
pub fn parse_schema(input: TokenStream) -> syn::Result<TokenStream> {
    let schema: SchemaDefinition = syn::parse2(input)?;
    expand_schema_definition(&schema)
}

/// Expand schema definition into struct and generator
fn expand_schema_definition(schema: &SchemaDefinition) -> syn::Result<TokenStream> {
    let name = &schema.name;
    let generator_name = quote::format_ident!("{}Generator", name);
    
    // Generate struct fields
    let struct_fields: Vec<_> = schema.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_type = &field.field_type;
        quote! {
            pub #field_name: #field_type
        }
    }).collect();
    
    // Generate generator fields
    let generator_fields: Vec<_> = schema.fields.iter().map(|field| {
        let field_name = &field.name;
        let generator_field_name = quote::format_ident!("{}_generator", field_name);
        let generator_type = determine_generator_type(&field.generator_config)?;
        Ok(quote! {
            #generator_field_name: #generator_type
        })
    }).collect::<syn::Result<Vec<_>>>()?;
    
    // Generate field initializations
    let field_inits: Vec<_> = schema.fields.iter().map(|field| {
        let field_name = &field.name;
        let generator_field_name = quote::format_ident!("{}_generator", field_name);
        let generator_init = expand_generator_config(&field.generator_config, &field_name.to_string())?;
        Ok(quote! {
            #generator_field_name: #generator_init
        })
    }).collect::<syn::Result<Vec<_>>>()?;
    
    // Generate field generations
    let field_generations: Vec<_> = schema.fields.iter().map(|field| {
        let field_name = &field.name;
        let generator_field_name = quote::format_ident!("{}_generator", field_name);
        quote! {
            #field_name: self.#generator_field_name.generate(context)?
        }
    }).collect();
    
    // Extract dependencies
    let dependencies = extract_schema_dependencies(schema)?;
    
    Ok(quote! {
        /// Auto-generated struct from schema
        #[derive(Debug, Clone)]
        pub struct #name {
            #(#struct_fields,)*
        }
        
        /// Auto-generated generator from schema
        #[derive(Debug, Clone)]
        pub struct #generator_name {
            name: String,
            #(#generator_fields,)*
        }
        
        impl #generator_name {
            pub fn new(name: String) -> Self {
                Self {
                    name,
                    #(#field_inits,)*
                }
            }
        }
        
        impl dadagen_core::generators::Generator<#name> for #generator_name {
            fn generate(&self, context: &dadagen_core::context::Context) -> dadagen_core::errors::Result<#name> {
                Ok(#name {
                    #(#field_generations,)*
                })
            }
            
            fn dependencies(&self) -> Vec<String> {
                vec![#(#dependencies),*]
            }
            
            fn name(&self) -> &str {
                &self.name
            }
        }
    })
}

/// Determine generator type from configuration
fn determine_generator_type(config: &TokenStream) -> syn::Result<TokenStream> {
    let config_str = config.to_string();
    
    if config_str.contains("string") {
        Ok(quote! { dadagen_core::generators::StringGenerator })
    } else if config_str.contains("integer") {
        Ok(quote! { dadagen_core::generators::IntegerGenerator })
    } else if config_str.contains("float") {
        Ok(quote! { dadagen_core::generators::FloatGenerator })
    } else if config_str.contains("pattern") {
        Ok(quote! { dadagen_core::generators::PatternGenerator })
    } else if config_str.contains("template") {
        Ok(quote! { dadagen_core::generators::TemplateGenerator })
    } else if config_str.contains("list") {
        Ok(quote! { dadagen_core::generators::ListGenerator })
    } else {
        Ok(quote! { dadagen_core::generators::StringGenerator })
    }
}

/// Expand generator configuration into initialization code
fn expand_generator_config(config: &TokenStream, field_name: &str) -> syn::Result<TokenStream> {
    let config_str = config.to_string();
    
    // Parse configuration parameters
    if config_str.contains("string") {
        let params = extract_parameters(&config_str);
        let mut generator = quote! {
            dadagen_core::generators::StringGenerator::new(#field_name.to_string())
        };
        
        // Add configuration methods
        if let Some(min_length) = params.get("min_length") {
            if let Some(max_length) = params.get("max_length") {
                generator = quote! {
                    #generator.with_length_range(#min_length, #max_length)
                };
            }
        }
        
        if let Some(charset) = params.get("charset") {
            let charset_enum = match charset.base10_digits() {
                "alpha" => quote! { dadagen_core::generators::CharacterSet::Alpha },
                "numeric" => quote! { dadagen_core::generators::CharacterSet::Numeric },
                "alphanumeric" => quote! { dadagen_core::generators::CharacterSet::AlphaNumeric },
                _ => quote! { dadagen_core::generators::CharacterSet::AlphaNumeric },
            };
            generator = quote! {
                #generator.with_charset(#charset_enum)
            };
        }
        
        Ok(generator)
    } else if config_str.contains("integer") {
        let params = extract_parameters(&config_str);
        let mut generator = quote! {
            dadagen_core::generators::IntegerGenerator::new(#field_name.to_string())
        };
        
        if let Some(min) = params.get("min") {
            if let Some(max) = params.get("max") {
                generator = quote! {
                    #generator.with_range(#min, #max)
                };
            }
        }
        
        Ok(generator)
    } else if config_str.contains("pattern") {
        let pattern = extract_pattern(&config_str);
        Ok(quote! {
            dadagen_core::generators::PatternGenerator::new(#field_name.to_string(), #pattern.to_string())
        })
    } else {
        Ok(quote! {
            dadagen_core::generators::StringGenerator::new(#field_name.to_string())
        })
    }
}

/// Extract parameters from configuration string
fn extract_parameters(config: &str) -> std::collections::HashMap<String, syn::LitInt> {
    let mut params = std::collections::HashMap::new();
    
    // Simple parameter extraction - in practice, use a proper parser
    if let Some(start) = config.find('(') {
        if let Some(end) = config.find(')') {
            let param_str = &config[start+1..end];
            for param in param_str.split(',') {
                let parts: Vec<&str> = param.split('=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    if let Ok(value) = parts[1].parse::<i32>() {
                        if let Ok(lit) = syn::parse_str::<syn::LitInt>(&value.to_string()) {
                            params.insert(parts[0].to_string(), lit);
                        }
                    }
                }
            }
        }
    }
    
    params
}

/// Extract pattern string from configuration
fn extract_pattern(config: &str) -> String {
    // Extract pattern from pattern("...") format
    if let Some(start) = config.find("pattern(\"") {
        let start = start + 9; // Skip 'pattern("'
        if let Some(end) = config[start..].find('"') {
            return config[start..start+end].to_string();
        }
    }
    "default_pattern".to_string()
}

/// Extract dependencies from schema
fn extract_schema_dependencies(schema: &SchemaDefinition) -> syn::Result<Vec<TokenStream>> {
    let mut all_deps = Vec::new();
    
    for field in &schema.fields {
        let config_str = field.generator_config.to_string();
        if config_str.contains("pattern") || config_str.contains("template") {
            let pattern = extract_pattern(&config_str);
            let deps = extract_pattern_dependencies(&pattern);
            for dep in deps {
                all_deps.push(quote! { #dep.to_string() });
            }
        }
    }
    
    Ok(all_deps)
}

/// Extract field dependencies from pattern strings
fn extract_pattern_dependencies(pattern: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let mut chars = pattern.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '#' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            let mut field_name = String::new();
            while let Some(ch) = chars.next() {
                if ch == '}' {
                    break;
                }
                field_name.push(ch);
            }
            if !field_name.is_empty() && !deps.contains(&field_name) {
                deps.push(field_name);
            }
        }
    }
    
    deps
}
