//! Procedural macros for dadagen data generation
//! 
//! This crate provides compile-time code generation for creating
//! data generators with minimal boilerplate.

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field};
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

mod generator;
mod schema;
mod field_generators;

pub use generator::*;
pub use schema::*;
pub use field_generators::*;

/// Derive macro to automatically implement the Generator trait
/// 
/// Usage:
/// ```
/// #[derive(Generator)]
/// struct Person {
///     #[dadagen(string(min_length = 2, max_length = 10))]
///     name: String,
///     
///     #[dadagen(integer(min = 18, max = 99))]
///     age: i32,
///     
///     #[dadagen(pattern = "#{name}.#{age}@example.com")]
///     email: String,
/// }
/// ```
#[proc_macro_derive(Generator, attributes(dadagen))]
pub fn derive_generator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    match generator::expand_generator_derive(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Macro for creating data schemas at compile time
/// 
/// Usage:
/// ```
/// dadagen_schema! {
///     Person {
///         name: String = string(min_length = 2, max_length = 10),
///         age: i32 = integer(min = 18, max = 99),
///         email: String = pattern("#{name}.#{age}@example.com"),
///     }
/// }
/// ```
#[proc_macro]
pub fn dadagen_schema(input: TokenStream) -> TokenStream {
    match schema::parse_schema(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Macro for generating random values inline
/// 
/// Usage:
/// ```
/// let value: String = dadagen_generate!(string(min_length = 5, max_length = 10));
/// let number: i32 = dadagen_generate!(integer(min = 1, max = 100));
/// ```
#[proc_macro]
pub fn dadagen_generate(input: TokenStream) -> TokenStream {
    match field_generators::parse_generate_call(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Attribute macro for field-level generator configuration
/// 
/// Usage:
/// ```
/// struct Person {
///     #[dadagen_field(string(charset = "alpha", case = "title"))]
///     name: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn dadagen_field(args: TokenStream, input: TokenStream) -> TokenStream {
    match field_generators::expand_field_attribute(args.into(), input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
