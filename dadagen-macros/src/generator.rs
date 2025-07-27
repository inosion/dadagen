//! Generator derive macro implementation

use syn::{DeriveInput, Data, Fields, Field, Attribute, Meta, MetaList, NestedMeta, Lit, Type, Ident};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use std::collections::HashMap;

/// Expand the Generator derive macro
pub fn expand_generator_derive(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    match &input.data {
        Data::Struct(data_struct) => {
            let generator_impl = generate_struct_impl(name, &data_struct.fields)?;
            
            Ok(quote! {
                #generator_impl
            })
        },
        Data::Enum(_) => {
            Err(syn::Error::new_spanned(
                name, 
                "Generator derive is not supported for enums yet"
            ))
        },
        Data::Union(_) => {
            Err(syn::Error::new_spanned(
                name, 
                "Generator derive is not supported for unions"
            ))
        },
    }
}

/// Generate implementation for struct types
fn generate_struct_impl(name: &Ident, fields: &Fields) -> syn::Result<TokenStream> {
    match fields {
        Fields::Named(fields_named) => {
            let field_generators = generate_field_generators(&fields_named.named)?;
            let field_assignments = generate_field_assignments(&fields_named.named)?;
            let dependencies = extract_dependencies(&fields_named.named)?;
            
            let generator_name = format!("{}Generator", name);
            let generator_ident = format_ident!("{}", generator_name);
            
            Ok(quote! {
                /// Auto-generated generator for #name
                #[derive(Debug, Clone)]
                pub struct #generator_ident {
                    name: String,
                    #field_generators
                }
                
                impl #generator_ident {
                    pub fn new(name: String) -> Self {
                        Self {
                            name,
                            #(#field_assignments,)*
                        }
                    }
                }
                
                impl dadagen_core::generators::Generator<#name> for #generator_ident {
                    fn generate(&self, context: &dadagen_core::context::Context) -> dadagen_core::errors::Result<#name> {
                        Ok(#name {
                            #(#field_assignments,)*
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
        },
        Fields::Unnamed(_) => {
            Err(syn::Error::new_spanned(
                fields,
                "Generator derive for tuple structs is not implemented yet"
            ))
        },
        Fields::Unit => {
            Ok(quote! {
                impl dadagen_core::generators::Generator<#name> for () {
                    fn generate(&self, _context: &dadagen_core::context::Context) -> dadagen_core::errors::Result<#name> {
                        Ok(#name)
                    }
                    
                    fn dependencies(&self) -> Vec<String> {
                        vec![]
                    }
                    
                    fn name(&self) -> &str {
                        stringify!(#name)
                    }
                }
            })
        }
    }
}

/// Generate field generator declarations
fn generate_field_generators(fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>) -> syn::Result<TokenStream> {
    let mut generators = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let generator_name = format_ident!("{}_generator", field_name);
        let generator_type = parse_field_generator_type(field)?;
        
        generators.push(quote! {
            #generator_name: #generator_type
        });
    }
    
    Ok(quote! {
        #(#generators,)*
    })
}

/// Generate field assignment expressions
fn generate_field_assignments(fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>) -> syn::Result<Vec<TokenStream>> {
    let mut assignments = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let generator_name = format_ident!("{}_generator", field_name);
        let generator_init = parse_field_generator_init(field)?;
        
        assignments.push(quote! {
            #field_name: self.#generator_name.generate(context)?
        });
    }
    
    Ok(assignments)
}

/// Extract dependencies from field attributes
fn extract_dependencies(fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>) -> syn::Result<Vec<String> > {
    let mut all_deps = Vec::new();
    
    for field in fields {
        let deps = parse_field_dependencies(field)?;
        all_deps.extend(deps);
    }
    
    Ok(all_deps.into_iter().map(|s| quote! { #s.to_string() }).collect())
}

/// Parse field generator type from attributes
fn parse_field_generator_type(field: &Field) -> syn::Result<TokenStream> {
    if let Some(config) = parse_dadagen_attribute(field)? {
        match config.as_str() {
            config if config.starts_with("string") => {
                Ok(quote! { dadagen_core::generators::StringGenerator })
            },
            config if config.starts_with("integer") => {
                Ok(quote! { dadagen_core::generators::IntegerGenerator })
            },
            config if config.starts_with("float") => {
                Ok(quote! { dadagen_core::generators::FloatGenerator })
            },
            config if config.starts_with("pattern") => {
                Ok(quote! { dadagen_core::generators::PatternGenerator })
            },
            config if config.starts_with("template") => {
                Ok(quote! { dadagen_core::generators::TemplateGenerator })
            },
            config if config.starts_with("list") => {
                Ok(quote! { dadagen_core::generators::ListGenerator })
            },
            _ => {
                // Default based on type
                match &field.ty {
                    Type::Path(type_path) => {
                        let type_name = type_path.path.segments.last().unwrap().ident.to_string();
                        match type_name.as_str() {
                            "String" => Ok(quote! { dadagen_core::generators::StringGenerator }),
                            "i32" | "i64" | "isize" => Ok(quote! { dadagen_core::generators::IntegerGenerator }),
                            "f32" | "f64" => Ok(quote! { dadagen_core::generators::FloatGenerator }),
                            _ => Ok(quote! { dadagen_core::generators::StringGenerator }),
                        }
                    },
                    _ => Ok(quote! { dadagen_core::generators::StringGenerator }),
                }
            }
        }
    } else {
        // Default generator based on type
        match &field.ty {
            Type::Path(type_path) => {
                let type_name = type_path.path.segments.last().unwrap().ident.to_string();
                match type_name.as_str() {
                    "String" => Ok(quote! { dadagen_core::generators::StringGenerator }),
                    "i32" | "i64" | "isize" => Ok(quote! { dadagen_core::generators::IntegerGenerator }),
                    "f32" | "f64" => Ok(quote! { dadagen_core::generators::FloatGenerator }),
                    _ => Ok(quote! { dadagen_core::generators::StringGenerator }),
                }
            },
            _ => Ok(quote! { dadagen_core::generators::StringGenerator }),
        }
    }
}

/// Parse field generator initialization from attributes
fn parse_field_generator_init(field: &Field) -> syn::Result<TokenStream> {
    let field_name = field.ident.as_ref().unwrap().to_string();
    
    if let Some(config) = parse_dadagen_attribute(field)? {
        Ok(parse_generator_config(&config, &field_name)?)
    } else {
        // Default initialization
        Ok(quote! {
            dadagen_core::generators::StringGenerator::new(#field_name.to_string())
        })
    }
}

/// Parse dependencies from field attributes
fn parse_field_dependencies(field: &Field) -> syn::Result<Vec<String>> {
    if let Some(config) = parse_dadagen_attribute(field)? {
        // Extract dependencies from pattern/template generators
        if config.contains("pattern") || config.contains("template") {
            // Parse pattern for field references like #{field_name}
            let deps = extract_pattern_dependencies(&config);
            Ok(deps)
        } else {
            Ok(vec![])
        }
    } else {
        Ok(vec![])
    }
}

/// Parse the dadagen attribute from a field
fn parse_dadagen_attribute(field: &Field) -> syn::Result<Option<String>> {
    for attr in &field.attrs {
        if attr.path.is_ident("dadagen") {
            match attr.parse_meta()? {
                Meta::List(meta_list) => {
                    // Extract the configuration string
                    let config = format!("{:?}", meta_list.nested);
                    return Ok(Some(config));
                },
                Meta::NameValue(meta_name_value) => {
                    if let Lit::Str(lit_str) = &meta_name_value.lit {
                        return Ok(Some(lit_str.value()));
                    }
                },
                _ => {}
            }
        }
    }
    Ok(None)
}

/// Parse generator configuration string into TokenStream
fn parse_generator_config(config: &str, field_name: &str) -> syn::Result<TokenStream> {
    // This is a simplified parser - in practice, you'd want a more robust solution
    if config.contains("string") {
        Ok(quote! {
            dadagen_core::generators::StringGenerator::new(#field_name.to_string())
        })
    } else if config.contains("integer") {
        Ok(quote! {
            dadagen_core::generators::IntegerGenerator::new(#field_name.to_string())
        })
    } else if config.contains("float") {
        Ok(quote! {
            dadagen_core::generators::FloatGenerator::new(#field_name.to_string())
        })
    } else if config.contains("pattern") {
        // Extract pattern from config
        let pattern = extract_pattern_from_config(config);
        Ok(quote! {
            dadagen_core::generators::PatternGenerator::new(#field_name.to_string(), #pattern.to_string())
        })
    } else {
        Ok(quote! {
            dadagen_core::generators::StringGenerator::new(#field_name.to_string())
        })
    }
}

/// Extract pattern string from configuration
fn extract_pattern_from_config(config: &str) -> &str {
    // Simplified pattern extraction
    if let Some(start) = config.find('"') {
        if let Some(end) = config[start+1..].find('"') {
            return &config[start+1..start+1+end];
        }
    }
    "default_pattern"
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
