//! Code generation for DataGenerator derive macro
//!
//! This module contains the code generation logic for producing generator
//! structs and trait implementations from analyzed field configurations.

use super::{FieldConfig, GeneratorSpec};
use syn::Ident;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

/// Generate the generator struct definition
pub fn generate_generator_struct(
    struct_name: &Ident,
    field_configs: &[FieldConfig],
) -> syn::Result<TokenStream> {
    let generator_name = format_ident!("{}Generator", struct_name);
    
    // Generate field declarations for generator struct
    let generator_fields: Vec<TokenStream> = field_configs
        .iter()
        .map(|config| {
            let field_name = format_ident!("{}", config.field_name);
            let _generator_type = get_generator_ast_type(&config.generator_type);
            quote! {
                #field_name: Box<dyn dadagen_core::generator_trait::DataGenerator>
            }
        })
        .collect();
    
    Ok(quote! {
        /// Auto-generated data generator for #struct_name
        ///
        /// This generator was automatically created by the DataGenerator derive macro.
        /// It contains individual generators for each field and implements the
        /// DataGenerator trait to produce #struct_name instances.
        #[allow(dead_code)]
        pub struct #generator_name {
            #(#generator_fields,)*
        }
    })
}

/// Generate the implementation block for the generator struct
pub fn generate_generator_impl(
    struct_name: &Ident,
    field_configs: &[FieldConfig],
    _impl_generics: &syn::ImplGenerics,
    _ty_generics: &syn::TypeGenerics,
    _where_clause: Option<&syn::WhereClause>,
) -> syn::Result<TokenStream> {
    let generator_name = format_ident!("{}Generator", struct_name);
    
    // Generate field initializations
    let field_inits: Vec<TokenStream> = field_configs
        .iter()
        .map(|config| {
            let field_name = format_ident!("{}", config.field_name);
            let init_expr = generate_generator_init(&config)?;
            Ok(quote! {
                #field_name: Box::new(#init_expr)
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    
    // Generate field generation code
    let field_generations: Vec<TokenStream> = field_configs
        .iter()
        .map(|config| {
            let field_name = format_ident!("{}", config.field_name);
            generate_field_generation(config, &field_name)
        })
        .collect::<syn::Result<Vec<_>>>()?;
    
    // Extract dependencies from template fields for the dependencies() method
    let template_deps: Vec<String> = field_configs
        .iter()
        .filter_map(|config| {
            if let super::GeneratorSpec::Template { pattern } = &config.generator_type {
                Some(extract_template_dependencies(pattern))
            } else {
                None
            }
        })
        .flatten()
        .collect();
    
    let deps_literal = if template_deps.is_empty() {
        quote! { vec![] }
    } else {
        quote! { vec![#(#template_deps.to_string()),*] }
    };
    
    Ok(quote! {
        impl #generator_name {
            /// Create a new generator for #struct_name
            pub fn new() -> Self {
                Self {
                    #(#field_inits,)*
                }
            }
            
            /// Generate a new instance of #struct_name
            pub fn generate(&self, context: &dadagen_core::context::Context) 
                -> dadagen_core::errors::Result<#struct_name> 
            {
                Ok(#struct_name {
                    #(#field_generations,)*
                })
            }
            
            /// Get list of field dependencies
            pub fn dependencies(&self) -> Vec<String> {
                #deps_literal
            }
        }
        
        impl Default for #generator_name {
            fn default() -> Self {
                Self::new()
            }
        }
    })
}

/// Generate DataGenerator trait implementation
pub fn generate_datagen_trait_impl(
    struct_name: &Ident,
    field_configs: &[FieldConfig],
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
) -> syn::Result<TokenStream> {
    let generator_name = format_ident!("{}Generator", struct_name);
    
    // Extract dependencies from template fields
    let template_deps: Vec<String> = field_configs
        .iter()
        .filter_map(|config| {
            if let GeneratorSpec::Template { pattern } = &config.generator_type {
                Some(extract_template_dependencies(pattern))
            } else {
                None
            }
        })
        .flatten()
        .collect();
    
    let deps_literal = if template_deps.is_empty() {
        quote! { vec![] }
    } else {
        quote! { vec![#(#template_deps.to_string()),*] }
    };
    
    Ok(quote! {
        impl #impl_generics dadagen_core::generator_trait::DataGenerator for #generator_name #ty_generics #where_clause {
            fn generate(&self, context: &dadagen_core::context::Context) 
                -> dadagen_core::errors::Result<String> 
            {
                let instance = self.generate(context)?;
                // Serialize to JSON for string representation
                Ok(format!("{:?}", instance))
            }
            
            fn dependencies(&self) -> Vec<String> {
                #deps_literal
            }
            
            fn generator_type(&self) -> dadagen_core::generator_trait::GeneratorType {
                dadagen_core::generator_trait::GeneratorType::String
            }
            
            fn clone_box(&self) -> Box<dyn dadagen_core::generator_trait::DataGenerator> {
                Box::new(Self::new())
            }
        }
        
        impl #impl_generics std::fmt::Debug for #generator_name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#generator_name)).finish()
            }
        }
        
        impl #impl_generics Clone for #generator_name #ty_generics #where_clause {
            fn clone(&self) -> Self {
                Self::new()
            }
        }
    })
}

/// Generate initialization expression for a field's generator
fn generate_generator_init(config: &FieldConfig) -> syn::Result<TokenStream> {
    use crate::generator::GeneratorSpec::*;
    
    let _field_name = &config.field_name;
    
    match &config.generator_type {
        String { length, min_length, max_length, charset, case } => {
            let length_init = length.map(|l| quote! { length: Some(#l), });
            let min_init = min_length.map(|m| quote! { min_length: Some(#m), });
            let max_init = max_length.map(|m| quote! { max_length: Some(#m), });
            let charset_init = charset.as_ref().map(|c| {
                let cs = match c.as_str() {
                    "alpha" => quote! { dadagen_core::ast::CharacterSet::Alpha },
                    "numeric" => quote! { dadagen_core::ast::CharacterSet::Numeric },
                    "alphanumeric" => quote! { dadagen_core::ast::CharacterSet::AlphaNumeric },
                    "ascii" => quote! { dadagen_core::ast::CharacterSet::Ascii },
                    "hex" => quote! { dadagen_core::ast::CharacterSet::Hex },
                    _ => quote! { dadagen_core::ast::CharacterSet::AlphaNumeric },
                };
                quote! { charset: #cs, }
            });
            let case_init = case.as_ref().map(|c| {
                let cs = match c.as_str() {
                    "lower" => quote! { dadagen_core::ast::Case::Lower },
                    "upper" => quote! { dadagen_core::ast::Case::Upper },
                    "title" => quote! { dadagen_core::ast::Case::Title },
                    _ => quote! { dadagen_core::ast::Case::Mixed },
                };
                quote! { case: #cs, }
            });
            
            Ok(quote! {
                dadagen_core::generator_trait::StringDataGenerator::new(
                    dadagen_core::ast::StringGenerator {
                        #length_init
                        #min_init
                        #max_init
                        #charset_init
                        #case_init
                        pattern: None,
                        span: None,
                        ..Default::default()
                    }
                )
            })
        },
        
        Number { min, max, decimal_places } => {
            let min_init = min.map(|m| quote! { min: Some(#m), });
            let max_init = max.map(|m| quote! { max: Some(#m), });
            let dec_init = decimal_places.map(|d| quote! { decimal_places: Some(#d), });
            
            Ok(quote! {
                dadagen_core::generator_trait::NumberDataGenerator::new(
                    dadagen_core::ast::NumberGenerator {
                        #min_init
                        #max_init
                        #dec_init
                        ..Default::default()
                    }
                )
            })
        },
        
        Boolean { true_probability } => {
            let prob = true_probability.unwrap_or(0.5);
            Ok(quote! {
                dadagen_core::generator_trait::BooleanDataGenerator::new(
                    dadagen_core::ast::BooleanGenerator {
                        true_probability: #prob,
                        span: None,
                    }
                )
            })
        },
        
        Choice { options } => {
            let opts = options.iter().map(|o| quote! { #o.to_string() });
            Ok(quote! {
                dadagen_core::generator_trait::ChoiceDataGenerator::new(
                    dadagen_core::ast::ChoiceGenerator {
                        options: vec![#(#opts),*],
                        span: None,
                    }
                )
            })
        },
        
        List { name } => {
            Ok(quote! {
                dadagen_core::generator_trait::ListDataGenerator::new(
                    dadagen_core::ast::ListGenerator {
                        name: #name.to_string(),
                        discriminator: None,
                        weighted: false,
                        span: None,
                    }
                )
            })
        },
        
        Template { pattern } => {
            Ok(quote! {
                dadagen_core::generator_trait::TemplateDataGenerator::new(
                    dadagen_core::ast::TemplateGenerator {
                        template: #pattern.to_string(),
                        variables: vec![],
                        span: None,
                    }
                )
            })
        },
        
        Regex { pattern } => {
            Ok(quote! {
                dadagen_core::generator_trait::RegexDataGenerator::new(
                    dadagen_core::ast::RegexGenerator {
                        pattern: #pattern.to_string(),
                        span: None,
                    }
                )
            })
        },
        
        Counter { start, step } => {
            let start_opt = start.map(|s| quote! { Some(#s) }).unwrap_or_else(|| quote! { Some(0) });
            let step_opt = step.map(|s| quote! { Some(#s) }).unwrap_or_else(|| quote! { Some(1) });
            Ok(quote! {
                dadagen_core::generator_trait::CounterDataGenerator::new(
                    dadagen_core::ast::CounterGenerator {
                        start: #start_opt,
                        step: #step_opt,
                        span: None,
                    }
                )
            })
        },
        
        Name { name_type } => {
            let nt = name_type.as_ref().map(|s| s.as_str()).unwrap_or("full");
            let name_type_enum = match nt {
                "given" => quote! { dadagen_core::ast::NameType::GivenName },
                "surname" => quote! { dadagen_core::ast::NameType::Surname },
                _ => quote! { dadagen_core::ast::NameType::Full },
            };
            Ok(quote! {
                dadagen_core::generator_trait::NameDataGenerator::new(
                    dadagen_core::ast::NameGenerator {
                        name_type: #name_type_enum,
                        span: None,
                    }
                )
            })
        },
        
        Gender => {
            Ok(quote! {
                dadagen_core::generator_trait::GenderDataGenerator::new(
                    dadagen_core::ast::GenderGenerator {
                        span: None,
                    }
                )
            })
        },
        
        Address { components } => {
            // AddressGenerator uses component field (singular AddressComponent enum)
            // For now, default to CityTown if no specific component requested
            let component = if let Some(cs) = components {
                if cs.contains(&"city".to_string()) {
                    quote! { dadagen_core::ast::AddressComponent::CityTown }
                } else if cs.contains(&"street".to_string()) {
                    quote! { dadagen_core::ast::AddressComponent::Street }
                } else if cs.contains(&"zip".to_string()) {
                    quote! { dadagen_core::ast::AddressComponent::PostZipCode }
                } else {
                    quote! { dadagen_core::ast::AddressComponent::CityTown }
                }
            } else {
                quote! { dadagen_core::ast::AddressComponent::CityTown }
            };
            
            Ok(quote! {
                dadagen_core::generator_trait::AddressDataGenerator::new(
                    dadagen_core::ast::AddressGenerator {
                        component: #component,
                        span: None,
                    }
                )
            })
        },
        
        Inferred => {
            // Infer based on Rust type
            let type_name = crate::utils::type_name(&config.field_type);
            match type_name.unwrap_or_default().as_str() {
                "String" => Ok(quote! {
                    dadagen_core::generator_trait::StringDataGenerator::new(
                        dadagen_core::ast::StringGenerator::default()
                    )
                }),
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => Ok(quote! {
                    dadagen_core::generator_trait::NumberDataGenerator::new(
                        dadagen_core::ast::NumberGenerator {
                            decimal_places: Some(0),
                            ..Default::default()
                        }
                    )
                }),
                "f32" | "f64" => Ok(quote! {
                    dadagen_core::generator_trait::NumberDataGenerator::new(
                        dadagen_core::ast::NumberGenerator::default()
                    )
                }),
                "bool" => Ok(quote! {
                    dadagen_core::generator_trait::BooleanDataGenerator::new(
                        dadagen_core::ast::BooleanGenerator::default()
                    )
                }),
                _ => Ok(quote! {
                    dadagen_core::generator_trait::StringDataGenerator::new(
                        dadagen_core::ast::StringGenerator::default()
                    )
                }),
            }
        },
    }
}

/// Generate field generation code
fn generate_field_generation(
    config: &FieldConfig,
    field_ident: &Ident,
) -> syn::Result<TokenStream> {
    let gen_call = quote! {
        self.#field_ident.generate(context)?
    };
    
    if config.is_optional {
        // For Option<T>, wrap in Some()
        Ok(quote! {
            #field_ident: Some({
                let value_str = #gen_call;
                // Parse the generated string into the target type
                value_str.parse().ok()?
            })
        })
    } else if config.is_vec {
        // For Vec<T>, generate a single element for now (can be enhanced)
        Ok(quote! {
            #field_ident: vec![{
                let value_str = #gen_call;
                value_str.parse().map_err(|e| {
                    dadagen_core::errors::DadagenError::GenerationError {
                        message: format!("Failed to parse generated value for field '{}': {:?}", 
                            stringify!(#field_ident), e),
                    }
                })?
            }]
        })
    } else {
        // For regular types, parse the generated string
        Ok(quote! {
            #field_ident: {
                let value_str = #gen_call;
                value_str.parse().map_err(|e| {
                    dadagen_core::errors::DadagenError::GenerationError {
                        message: format!("Failed to parse generated value for field '{}': {:?}", 
                            stringify!(#field_ident), e),
                    }
                })?
            }
        })
    }
}

/// Get the AST type for a generator spec
fn get_generator_ast_type(spec: &GeneratorSpec) -> TokenStream {
    use crate::generator::GeneratorSpec::*;
    
    match spec {
        String { .. } => quote! { dadagen_core::ast::StringGenerator },
        Number { .. } => quote! { dadagen_core::ast::NumberGenerator },
        Boolean { .. } => quote! { dadagen_core::ast::BooleanGenerator },
        Choice { .. } => quote! { dadagen_core::ast::ChoiceGenerator },
        List { .. } => quote! { dadagen_core::ast::ListGenerator },
        Template { .. } => quote! { dadagen_core::ast::TemplateGenerator },
        Regex { .. } => quote! { dadagen_core::ast::RegexGenerator },
        Counter { .. } => quote! { dadagen_core::ast::CounterGenerator },
        Name { .. } => quote! { dadagen_core::ast::NameGenerator },
        Gender => quote! { dadagen_core::ast::GenderGenerator },
        Address { .. } => quote! { dadagen_core::ast::AddressGenerator },
        Inferred => quote! { dadagen_core::ast::StringGenerator },
    }
}

/// Extract field dependencies from template pattern
fn extract_template_dependencies(pattern: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let mut chars = pattern.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '$' && chars.peek() == Some(&'{') {
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
