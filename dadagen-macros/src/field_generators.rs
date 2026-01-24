//! Field generator macro implementations

use syn::{parse::Parse, Token, Ident, Expr};
use quote::quote;
use proc_macro2::TokenStream;

/// Generator call structure for dadagen_generate! macro
pub struct GeneratorCall {
    pub generator_type: Ident,
    pub params: Vec<GeneratorParam>,
}

/// Generator parameter
pub struct GeneratorParam {
    pub name: Ident,
    pub value: Expr,
}

impl Parse for GeneratorCall {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let generator_type: Ident = input.parse()?;
        
        let mut params = Vec::new();
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            
            while !content.is_empty() {
                let name: Ident = content.parse()?;
                content.parse::<Token![=]>()?;
                let value: Expr = content.parse()?;
                
                params.push(GeneratorParam { name, value });
                
                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                }
            }
        }
        
        Ok(GeneratorCall {
            generator_type,
            params,
        })
    }
}

/// Parse and expand dadagen_generate! macro
pub fn parse_generate_call(input: TokenStream) -> syn::Result<TokenStream> {
    let call: GeneratorCall = syn::parse2(input)?;
    expand_generate_call(&call)
}

/// Expand generator call into inline generation code
fn expand_generate_call(call: &GeneratorCall) -> syn::Result<TokenStream> {
    let generator_type = &call.generator_type;
    
    match generator_type.to_string().as_str() {
        "string" => expand_string_generator(call),
        "integer" => expand_integer_generator(call),
        "float" => expand_float_generator(call),
        "pattern" => expand_pattern_generator(call),
        "list" => expand_list_generator(call),
        _ => Err(syn::Error::new_spanned(
            generator_type,
            format!("Unknown generator type: {}", generator_type)
        )),
    }
}

/// Expand string generator call
fn expand_string_generator(call: &GeneratorCall) -> syn::Result<TokenStream> {
    let mut generator = quote! {
        dadagen_core::generators::StringGenerator::new("inline".to_string())
    };
    
    // Apply parameters
    for param in &call.params {
        match param.name.to_string().as_str() {
            "min_length" | "max_length" => {
                // Find both min and max length parameters
                if let (Some(min), Some(max)) = (
                    find_param_value(call, "min_length"),
                    find_param_value(call, "max_length")
                ) {
                    generator = quote! {
                        #generator.with_length_range(#min, #max)
                    };
                }
            },
            "length" => {
                let value = &param.value;
                generator = quote! {
                    #generator.with_fixed_length(#value)
                };
            },
            "charset" => {
                let charset = expand_charset_value(&param.value)?;
                generator = quote! {
                    #generator.with_charset(#charset)
                };
            },
            "case" => {
                let case = expand_case_value(&param.value)?;
                generator = quote! {
                    #generator.with_case(#case)
                };
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    &param.name,
                    format!("Unknown parameter for string generator: {}", param.name)
                ));
            }
        }
    }
    
    Ok(quote! {
        {
            let context = dadagen_core::context::Context::new();
            #generator.generate(&context).unwrap()
        }
    })
}

/// Expand integer generator call
fn expand_integer_generator(call: &GeneratorCall) -> syn::Result<TokenStream> {
    let mut generator = quote! {
        dadagen_core::generators::IntegerGenerator::new("inline".to_string())
    };
    
    // Apply parameters
    for param in &call.params {
        match param.name.to_string().as_str() {
            "min" | "max" => {
                if let (Some(min), Some(max)) = (
                    find_param_value(call, "min"),
                    find_param_value(call, "max")
                ) {
                    generator = quote! {
                        #generator.with_range(#min, #max)
                    };
                }
            },
            "seed" => {
                let value = &param.value;
                generator = quote! {
                    #generator.with_seed(#value)
                };
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    &param.name,
                    format!("Unknown parameter for integer generator: {}", param.name)
                ));
            }
        }
    }
    
    Ok(quote! {
        {
            let context = dadagen_core::context::Context::new();
            #generator.generate(&context).unwrap()
        }
    })
}

/// Expand float generator call
fn expand_float_generator(call: &GeneratorCall) -> syn::Result<TokenStream> {
    let mut generator = quote! {
        dadagen_core::generators::FloatGenerator::new("inline".to_string())
    };
    
    // Apply parameters
    for param in &call.params {
        match param.name.to_string().as_str() {
            "min" | "max" => {
                if let (Some(min), Some(max)) = (
                    find_param_value(call, "min"),
                    find_param_value(call, "max")
                ) {
                    generator = quote! {
                        #generator.with_range(#min as f64, #max as f64)
                    };
                }
            },
            "decimal_places" => {
                let value = &param.value;
                generator = quote! {
                    #generator.with_decimal_places(#value)
                };
            },
            "seed" => {
                let value = &param.value;
                generator = quote! {
                    #generator.with_seed(#value)
                };
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    &param.name,
                    format!("Unknown parameter for float generator: {}", param.name)
                ));
            }
        }
    }
    
    Ok(quote! {
        {
            let context = dadagen_core::context::Context::new();
            #generator.generate(&context).unwrap()
        }
    })
}

/// Expand pattern generator call
fn expand_pattern_generator(call: &GeneratorCall) -> syn::Result<TokenStream> {
    let pattern_param = call.params.iter()
        .find(|p| p.name.to_string() == "pattern")
        .ok_or_else(|| syn::Error::new_spanned(
            &call.generator_type,
            "pattern generator requires a 'pattern' parameter"
        ))?;
    
    let pattern = &pattern_param.value;
    
    let mut generator = quote! {
        dadagen_core::generators::PatternGenerator::new("inline".to_string(), #pattern.to_string())
    };
    
    // Apply other parameters
    for param in &call.params {
        match param.name.to_string().as_str() {
            "pattern" => {}, // Already handled
            "seed" => {
                let value = &param.value;
                generator = quote! {
                    #generator.with_seed(#value)
                };
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    &param.name,
                    format!("Unknown parameter for pattern generator: {}", param.name)
                ));
            }
        }
    }
    
    Ok(quote! {
        {
            let context = dadagen_core::context::Context::new();
            #generator.generate(&context).unwrap()
        }
    })
}

/// Expand list generator call
fn expand_list_generator(call: &GeneratorCall) -> syn::Result<TokenStream> {
    let options_param = call.params.iter()
        .find(|p| p.name.to_string() == "options")
        .ok_or_else(|| syn::Error::new_spanned(
            &call.generator_type,
            "list generator requires an 'options' parameter"
        ))?;
    
    let options = &options_param.value;
    
    let mut generator = quote! {
        dadagen_core::generators::ListGenerator::new("inline".to_string(), #options)
    };
    
    // Apply other parameters
    for param in &call.params {
        match param.name.to_string().as_str() {
            "options" => {}, // Already handled
            "seed" => {
                let value = &param.value;
                generator = quote! {
                    #generator.with_seed(#value)
                };
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    &param.name,
                    format!("Unknown parameter for list generator: {}", param.name)
                ));
            }
        }
    }
    
    Ok(quote! {
        {
            let context = dadagen_core::context::Context::new();
            #generator.generate(&context).unwrap()
        }
    })
}

/// Find parameter value by name
fn find_param_value<'a>(call: &'a GeneratorCall, name: &str) -> Option<&'a Expr> {
    call.params.iter()
        .find(|p| p.name.to_string() == name)
        .map(|p| &p.value)
}

/// Expand charset value expression
fn expand_charset_value(expr: &Expr) -> syn::Result<TokenStream> {
    match expr {
        Expr::Lit(expr_lit) => {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let charset = match lit_str.value().as_str() {
                    "alpha" => quote! { dadagen_core::generators::CharacterSet::Alpha },
                    "numeric" => quote! { dadagen_core::generators::CharacterSet::Numeric },
                    "alphanumeric" => quote! { dadagen_core::generators::CharacterSet::AlphaNumeric },
                    "ascii" => quote! { dadagen_core::generators::CharacterSet::ASCII },
                    "hex" => quote! { dadagen_core::generators::CharacterSet::Hex },
                    "base64" => quote! { dadagen_core::generators::CharacterSet::Base64 },
                    custom => quote! { dadagen_core::generators::CharacterSet::Custom(#custom.to_string()) },
                };
                Ok(charset)
            } else {
                Err(syn::Error::new_spanned(expr, "charset must be a string literal"))
            }
        },
        _ => Err(syn::Error::new_spanned(expr, "charset must be a string literal")),
    }
}

/// Expand case value expression
fn expand_case_value(expr: &Expr) -> syn::Result<TokenStream> {
    match expr {
        Expr::Lit(expr_lit) => {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let case = match lit_str.value().as_str() {
                    "lower" => quote! { dadagen_core::generators::Case::Lower },
                    "upper" => quote! { dadagen_core::generators::Case::Upper },
                    "title" => quote! { dadagen_core::generators::Case::Title },
                    "mixed" => quote! { dadagen_core::generators::Case::Mixed },
                    _ => return Err(syn::Error::new_spanned(expr, "invalid case value")),
                };
                Ok(case)
            } else {
                Err(syn::Error::new_spanned(expr, "case must be a string literal"))
            }
        },
        _ => Err(syn::Error::new_spanned(expr, "case must be a string literal")),
    }
}

/// Expand field attribute macro
pub fn expand_field_attribute(_args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    // For now, just return the input unchanged
    // This could be extended to add metadata or modify the field
    Ok(input)
}
