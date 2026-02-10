//! Utility functions for macro development and debugging

use proc_macro2::TokenStream;
use syn::spanned::Spanned;

/// Pretty-print a TokenStream for debugging
#[allow(dead_code)]
pub fn debug_tokens(tokens: &TokenStream) -> String {
    format!("{}", tokens)
}

/// Format error message with span information
pub fn span_error<T: Spanned>(item: &T, message: impl std::fmt::Display) -> syn::Error {
    syn::Error::new(item.span(), message)
}

/// Create a compile error with helpful context
pub fn compile_error(message: impl std::fmt::Display) -> TokenStream {
    syn::Error::new(proc_macro2::Span::call_site(), message).to_compile_error()
}

/// Check if a type is Option<T>
pub fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Extract inner type from Option<T>
pub fn extract_option_inner(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}

/// Check if a type is Vec<T>
pub fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

/// Get the base type name as a string
pub fn type_name(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return Some(segment.ident.to_string());
        }
    }
    None
}

/// Map Rust primitive types to dadagen generator types
pub fn map_type_to_generator(ty: &syn::Type) -> Option<&'static str> {
    type_name(ty).as_deref().and_then(|name| match name {
        "String" | "str" => Some("string"),
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => Some("integer"),
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => Some("integer"),
        "f32" | "f64" => Some("float"),
        "bool" => Some("boolean"),
        _ => None,
    })
}

/// Validate attribute structure
pub fn validate_attribute_structure(attr: &syn::Attribute) -> syn::Result<()> {
    // Basic validation - actual parsing happens elsewhere
    if attr.path().is_ident("dadagen") {
        Ok(())
    } else {
        Err(syn::Error::new_spanned(
            attr,
            "Expected #[dadagen(...)] attribute",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_is_option_type() {
        let ty: syn::Type = parse_quote!(Option<String>);
        assert!(is_option_type(&ty));

        let ty: syn::Type = parse_quote!(String);
        assert!(!is_option_type(&ty));
    }

    #[test]
    fn test_is_vec_type() {
        let ty: syn::Type = parse_quote!(Vec<i32>);
        assert!(is_vec_type(&ty));

        let ty: syn::Type = parse_quote!(i32);
        assert!(!is_vec_type(&ty));
    }

    #[test]
    fn test_type_name() {
        let ty: syn::Type = parse_quote!(String);
        assert_eq!(type_name(&ty), Some("String".to_string()));

        let ty: syn::Type = parse_quote!(Option<i32>);
        assert_eq!(type_name(&ty), Some("Option".to_string()));
    }

    #[test]
    fn test_map_type_to_generator() {
        let ty: syn::Type = parse_quote!(String);
        assert_eq!(map_type_to_generator(&ty), Some("string"));

        let ty: syn::Type = parse_quote!(i32);
        assert_eq!(map_type_to_generator(&ty), Some("integer"));

        let ty: syn::Type = parse_quote!(f64);
        assert_eq!(map_type_to_generator(&ty), Some("float"));

        let ty: syn::Type = parse_quote!(bool);
        assert_eq!(map_type_to_generator(&ty), Some("boolean"));
    }

    #[test]
    fn test_extract_option_inner() {
        let ty: syn::Type = parse_quote!(Option<String>);
        let inner = extract_option_inner(&ty);
        assert!(inner.is_some());

        let ty: syn::Type = parse_quote!(String);
        let inner = extract_option_inner(&ty);
        assert!(inner.is_none());
    }
}
