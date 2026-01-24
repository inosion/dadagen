//! Generator derive macro implementation
//!
//! This module implements the `#[derive(DataGenerator)]` macro which analyzes
//! struct definitions and generates corresponding data generators at compile time.

use syn::{
    DeriveInput, Data, Fields, Field, Meta, Lit, Type, Ident, Expr, ExprLit,
    MetaList, punctuated::Punctuated, Token
};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use crate::utils;

mod codegen;
mod validation;
pub use codegen::{generate_generator_struct, generate_generator_impl, generate_datagen_trait_impl};
use validation::validate_generator_config;

/// Configuration for a generated field extracted from attributes
#[derive(Debug, Clone)]
pub struct FieldConfig {
    pub field_name: String,
    pub field_type: Type,
    pub generator_type: GeneratorSpec,
    pub is_optional: bool,
    pub is_vec: bool,
    pub is_nested_generator: bool, // True if field type implements DataGenerator
}

/// Specification for generator type and parameters
#[derive(Debug, Clone)]
pub enum GeneratorSpec {
    /// String generator with optional parameters
    String {
        length: Option<usize>,
        min_length: Option<usize>,
        max_length: Option<usize>,
        charset: Option<String>,
        case: Option<String>,
    },
    /// Number generator with range and distribution
    Number {
        min: Option<f64>,
        max: Option<f64>,
        decimal_places: Option<usize>,
    },
    /// Boolean generator with probability
    Boolean {
        true_probability: Option<f64>,
    },
    /// Choice from static list
    Choice {
        options: Vec<String>,
    },
    /// List from named data file
    List {
        name: String,
    },
    /// Template with field references
    Template {
        pattern: String,
    },
    /// Regex pattern generator
    Regex {
        pattern: String,
    },
    /// Counter with start and step
    Counter {
        start: Option<i64>,
        step: Option<i64>,
    },
    /// Name generator (given, surname, full)
    Name {
        name_type: Option<String>, // "given", "surname", "full"
    },
    /// Gender generator
    Gender,
    /// Address generator with optional components
    Address {
        components: Option<Vec<String>>, // ["street", "city", "state", "zip"]
    },
    /// Inferred from Rust type
    Inferred,
}

/// Expand the DataGenerator derive macro
pub fn expand_generator_derive(input: &DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    // Validate that generic parameters have appropriate bounds if used with generators
    validate_generic_parameters(&generics)?;
    
    match &input.data {
        Data::Struct(data_struct) => {
            let generator_impl = generate_struct_impl(
                struct_name,
                &data_struct.fields,
                impl_generics,
                ty_generics,
                where_clause,
                generics,
            )?;
            
            Ok(generator_impl)
        },
        Data::Enum(_) => {
            Err(syn::Error::new_spanned(
                struct_name, 
                "DataGenerator derive is not supported for enums yet"
            ))
        },
        Data::Union(_) => {
            Err(syn::Error::new_spanned(
                struct_name, 
                "DataGenerator derive is not supported for unions"
            ))
        },
    }
}


/// Generate implementation for struct types
fn generate_struct_impl(
    struct_name: &Ident,
    fields: &Fields,
    impl_generics: syn::ImplGenerics,
    ty_generics: syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    generics: &syn::Generics,
) -> syn::Result<TokenStream> {
    match fields {
        Fields::Named(fields_named) => {
            // Analyze all fields and extract configurations
            let field_configs = analyze_fields(&fields_named.named)?;
            
            // TODO: Re-enable validation after debugging
            // Validate all generator configurations at compile time
            // for config in &field_configs {
            //     validate_generator_config(config)?;
            // }
            
            // Generate the generator struct and implementation
            let generator_struct = generate_generator_struct(struct_name, &field_configs, generics)?;
            let generator_impl = generate_generator_impl(
                struct_name,
                &field_configs,
                &impl_generics,
                &ty_generics,
                where_clause,
                generics,
            )?;
            let datagen_impl = generate_datagen_trait_impl(
                struct_name,
                &field_configs,
                &impl_generics,
                &ty_generics,
                where_clause,
            )?;
            
            Ok(quote! {
                #generator_struct
                #generator_impl
                #datagen_impl
            })
        },
        Fields::Unnamed(_) => {
            Err(syn::Error::new_spanned(
                fields,
                "DataGenerator derive for tuple structs is not implemented yet"
            ))
        },
        Fields::Unit => {
            // Unit structs don't need generators
            let generator_name = format_ident!("{}Generator", struct_name);
            Ok(quote! {
                /// Auto-generated generator for unit struct #struct_name
                #[derive(Debug, Clone)]
                pub struct #generator_name;
                
                impl #generator_name {
                    pub fn new() -> Self {
                        Self
                    }
                    
                    pub fn generate(&self, _context: &dadagen_core::context::Context) 
                        -> dadagen_core::errors::Result<#struct_name> 
                    {
                        Ok(#struct_name)
                    }
                }
            })
        }
    }
}

/// Analyze struct fields and extract generator configurations
fn analyze_fields(
    fields: &Punctuated<Field, Token![,]>
) -> syn::Result<Vec<FieldConfig>> {
    let mut configs = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "Field must have a name"))?
            .to_string();
        
        // Skip phantom data fields (used for generics)
        if field_name.starts_with("_phantom") {
            continue;
        }
        
        let field_type = field.ty.clone();
        
        // Check if field is Option<T> or Vec<T>
        let is_optional = utils::is_option_type(&field_type);
        let is_vec = utils::is_vec_type(&field_type);
        
        // Check if field type might be a nested DataGenerator
        let is_nested_generator = is_nested_generator_type(&field_type);
        
        // Parse dadagen attributes to determine generator type
        let generator_type = parse_field_attributes(field, &field_type)?;
        
        configs.push(FieldConfig {
            field_name,
            field_type,
            generator_type,
            is_optional,
            is_vec,
            is_nested_generator,
        });
    }
    
    Ok(configs)
}

/// Parse field attributes to determine generator configuration
fn parse_field_attributes(field: &Field, field_type: &Type) -> syn::Result<GeneratorSpec> {
    // Look for #[dadagen(...)] attribute
    for attr in &field.attrs {
        if attr.path().is_ident("dadagen") {
            return parse_dadagen_attribute(attr, field_type);
        }
    }
    
    // If no attribute, infer from type
    Ok(GeneratorSpec::Inferred)
}

/// Parse the #[dadagen(...)] attribute structure
fn parse_dadagen_attribute(attr: &syn::Attribute, _field_type: &Type) -> syn::Result<GeneratorSpec> {
    match &attr.meta {
        // #[dadagen(string(length = 10, ...))]
        Meta::List(meta_list) => {
            parse_meta_list(meta_list)
        },
        // #[dadagen = "template"]
        Meta::NameValue(name_value) => {
            if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = &name_value.value {
                Ok(GeneratorSpec::Template {
                    pattern: lit_str.value(),
                })
            } else {
                Err(syn::Error::new_spanned(
                    attr,
                    "Expected string literal for dadagen attribute"
                ))
            }
        },
        Meta::Path(_) => {
            Err(syn::Error::new_spanned(
                attr,
                "dadagen attribute requires parameters: #[dadagen(...)]"
            ))
        }
    }
}

/// Parse nested meta list (e.g., string(length = 10, charset = "alpha"))
fn parse_meta_list(meta_list: &MetaList) -> syn::Result<GeneratorSpec> {
    let tokens = &meta_list.tokens;
    let tokens_str = tokens.to_string();
    
    // Parse the generator type from the first identifier
    if tokens_str.starts_with("string") {
        parse_string_config(&tokens_str)
    } else if tokens_str.starts_with("number") {
        parse_number_config(&tokens_str)
    } else if tokens_str.starts_with("boolean") || tokens_str.starts_with("bool") {
        parse_boolean_config(&tokens_str)
    } else if tokens_str.starts_with("choice") {
        parse_choice_config(&tokens_str)
    } else if tokens_str.starts_with("list") {
        parse_list_config(&tokens_str)
    } else if tokens_str.starts_with("template") {
        parse_template_config(&tokens_str)
    } else if tokens_str.starts_with("regex") {
        parse_regex_config(&tokens_str)
    } else if tokens_str.starts_with("counter") {
        parse_counter_config(&tokens_str)
    } else if tokens_str.starts_with("name") {
        parse_name_config(&tokens_str)
    } else if tokens_str.starts_with("gender") {
        Ok(GeneratorSpec::Gender)
    } else if tokens_str.starts_with("address") {
        parse_address_config(&tokens_str)
    } else {
        Err(syn::Error::new_spanned(
            meta_list,
            format!("Unknown generator type: {}", tokens_str)
        ))
    }
}


// ============================================================================
// Configuration Parsers
// ============================================================================

/// Parse string generator configuration
fn parse_string_config(config: &str) -> syn::Result<GeneratorSpec> {
    Ok(GeneratorSpec::String {
        length: extract_usize_param(config, "length"),
        min_length: extract_usize_param(config, "min_length"),
        max_length: extract_usize_param(config, "max_length"),
        charset: extract_string_param(config, "charset"),
        case: extract_string_param(config, "case"),
    })
}

/// Parse number generator configuration
fn parse_number_config(config: &str) -> syn::Result<GeneratorSpec> {
    Ok(GeneratorSpec::Number {
        min: extract_f64_param(config, "min"),
        max: extract_f64_param(config, "max"),
        decimal_places: extract_usize_param(config, "decimal_places"),
    })
}

/// Parse boolean generator configuration
fn parse_boolean_config(config: &str) -> syn::Result<GeneratorSpec> {
    Ok(GeneratorSpec::Boolean {
        true_probability: extract_f64_param(config, "true_probability")
            .or_else(|| extract_f64_param(config, "probability")),
    })
}

/// Parse choice generator configuration
fn parse_choice_config(config: &str) -> syn::Result<GeneratorSpec> {
    // Extract options array: choice(options = ["a", "b", "c"])
    let options = extract_string_array_param(config, "options")?;
    Ok(GeneratorSpec::Choice { options })
}

/// Parse list generator configuration
fn parse_list_config(config: &str) -> syn::Result<GeneratorSpec> {
    if let Some(name) = extract_string_param(config, "name") {
        Ok(GeneratorSpec::List { name })
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "list generator requires 'name' parameter"
        ))
    }
}

/// Parse template generator configuration
fn parse_template_config(config: &str) -> syn::Result<GeneratorSpec> {
    if let Some(pattern) = extract_string_param(config, "pattern")
        .or_else(|| extract_string_param(config, "template")) 
    {
        Ok(GeneratorSpec::Template { pattern })
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "template generator requires 'pattern' or 'template' parameter"
        ))
    }
}

/// Parse regex generator configuration
fn parse_regex_config(config: &str) -> syn::Result<GeneratorSpec> {
    if let Some(pattern) = extract_string_param(config, "pattern") {
        Ok(GeneratorSpec::Regex { pattern })
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "regex generator requires 'pattern' parameter"
        ))
    }
}

/// Parse counter generator configuration
fn parse_counter_config(config: &str) -> syn::Result<GeneratorSpec> {
    Ok(GeneratorSpec::Counter {
        start: extract_i64_param(config, "start"),
        step: extract_i64_param(config, "step"),
    })
}

/// Parse name generator configuration
fn parse_name_config(config: &str) -> syn::Result<GeneratorSpec> {
    Ok(GeneratorSpec::Name {
        name_type: extract_string_param(config, "type")
            .or_else(|| extract_string_param(config, "name_type")),
    })
}

/// Parse address generator configuration
fn parse_address_config(config: &str) -> syn::Result<GeneratorSpec> {
    Ok(GeneratorSpec::Address {
        components: if let Ok(comps) = extract_string_array_param(config, "components") {
            Some(comps)
        } else {
            None
        },
    })
}

// ============================================================================
// Parameter Extraction Utilities
// ============================================================================

/// Extract usize parameter from configuration string
fn extract_usize_param(config: &str, param_name: &str) -> Option<usize> {
    let pattern = format!("{} =", param_name);
    if let Some(start) = config.find(&pattern) {
        let after = &config[start + pattern.len()..];
        let num_str = after.trim()
            .split(|c: char| c == ',' || c == ')' || c.is_whitespace())
            .next()?
            .trim();
        num_str.parse().ok()
    } else {
        None
    }
}

/// Extract f64 parameter from configuration string
fn extract_f64_param(config: &str, param_name: &str) -> Option<f64> {
    let pattern = format!("{} =", param_name);
    if let Some(start) = config.find(&pattern) {
        let after = &config[start + pattern.len()..];
        let num_str = after.trim()
            .split(|c: char| c == ',' || c == ')' || c.is_whitespace())
            .next()?
            .trim();
        num_str.parse().ok()
    } else {
        None
    }
}

/// Extract i64 parameter from configuration string
fn extract_i64_param(config: &str, param_name: &str) -> Option<i64> {
    let pattern = format!("{} =", param_name);
    if let Some(start) = config.find(&pattern) {
        let after = &config[start + pattern.len()..];
        let num_str = after.trim()
            .split(|c: char| c == ',' || c == ')' || c.is_whitespace())
            .next()?
            .trim();
        num_str.parse().ok()
    } else {
        None
    }
}

/// Extract string parameter from configuration string
fn extract_string_param(config: &str, param_name: &str) -> Option<String> {
    let pattern = format!("{} =", param_name);
    if let Some(start) = config.find(&pattern) {
        let after = &config[start + pattern.len()..].trim();
        if after.starts_with('"') {
            let end = after[1..].find('"')?;
            Some(after[1..=end].to_string())
        } else {
            None
        }
    } else {
        None
    }
}

/// Extract string array parameter from configuration string
fn extract_string_array_param(config: &str, param_name: &str) -> syn::Result<Vec<String>> {
    let pattern = format!("{} =", param_name);
    if let Some(start) = config.find(&pattern) {
        let after = &config[start + pattern.len()..].trim();
        if after.starts_with('[') {
            let end = after.find(']').ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Unclosed array for parameter {}", param_name)
                )
            })?;
            let array_str = &after[1..end];
            
            let mut items = Vec::new();
            for part in array_str.split(',') {
                let trimmed = part.trim();
                if trimmed.starts_with('"') && trimmed.ends_with('"') {
                    items.push(trimmed[1..trimmed.len()-1].to_string());
                }
            }
            Ok(items)
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Expected array for parameter {}", param_name)
            ))
        }
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Parameter {} not found", param_name)
        ))
    }
}

/// Validate generic parameters for DataGenerator compatibility
fn validate_generic_parameters(generics: &syn::Generics) -> syn::Result<()> {
    // For now, we accept any generic parameters
    // In the future, we might require Clone + Debug bounds
    for param in &generics.params {
        if let syn::GenericParam::Type(_type_param) = param {
            // Type parameters are allowed - they'll be passed through to generated code
            // We don't enforce bounds here; let the compiler do that
        }
    }
    Ok(())
}

/// Check if a type might be a DataGenerator-implementing type
fn is_nested_generator_type(ty: &Type) -> bool {
    // Check if the type looks like a custom struct (not a primitive)
    match ty {
        Type::Path(type_path) => {
            // Get the last segment of the path
            if let Some(segment) = type_path.path.segments.last() {
                let ident = &segment.ident;
                let ident_str = ident.to_string();
                
                // Skip known primitive/standard types
                !matches!(ident_str.as_str(), 
                    "String" | "str" | "i8" | "i16" | "i32" | "i64" | "i128" |
                    "u8" | "u16" | "u32" | "u64" | "u128" | "f32" | "f64" |
                    "bool" | "char" | "Vec" | "Option" | "Result" | "Box" |
                    "Arc" | "Rc" | "HashMap" | "HashSet" | "BTreeMap" | "BTreeSet"
                )
            } else {
                false
            }
        },
        _ => false,
    }
}
