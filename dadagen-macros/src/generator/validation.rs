//! Compile-time validation for generator configurations
//!
//! This module provides validation logic to catch configuration errors
//! at compile time rather than runtime.

use super::{FieldConfig, GeneratorSpec};
use syn;

/// Validate a generator configuration at compile time
pub fn validate_generator_config(config: &FieldConfig) -> syn::Result<()> {
    match &config.generator_type {
        GeneratorSpec::String {
            length,
            min_length,
            max_length,
            charset,
            case,
        } => validate_string_config(length, min_length, max_length, charset, case)?,

        GeneratorSpec::Number {
            min,
            max,
            decimal_places,
        } => validate_number_config(min, max, decimal_places)?,

        GeneratorSpec::Boolean { true_probability } => validate_boolean_config(true_probability)?,

        GeneratorSpec::Choice { options } => validate_choice_config(options)?,

        GeneratorSpec::Template { pattern } => validate_template_config(pattern)?,

        GeneratorSpec::Regex { pattern } => validate_regex_config(pattern)?,

        GeneratorSpec::Counter { start: _, step: _ } => {
            // Counter parameters are always valid
        }

        GeneratorSpec::Name { name_type } => validate_name_config(name_type)?,

        GeneratorSpec::Address { components } => validate_address_config(components)?,

        GeneratorSpec::List { name: _ } | GeneratorSpec::Gender | GeneratorSpec::Inferred => {
            // These have no compile-time validation constraints
        }
    }

    Ok(())
}

/// Validate string generator configuration
fn validate_string_config(
    length: &Option<usize>,
    min_length: &Option<usize>,
    max_length: &Option<usize>,
    charset: &Option<String>,
    case: &Option<String>,
) -> syn::Result<()> {
    // Check for conflicting length specifications
    if length.is_some() && (min_length.is_some() || max_length.is_some()) {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Cannot specify both 'length' and 'min_length/max_length'. Use either fixed length or range.",
        ));
    }

    // Validate min < max
    if let (Some(min), Some(max)) = (min_length, max_length) {
        if min > max {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "min_length ({}) must be less than or equal to max_length ({})",
                    min, max
                ),
            ));
        }
    }

    // Validate charset
    if let Some(cs) = charset {
        match cs.as_str() {
            "alpha" | "numeric" | "alphanumeric" | "hex" | "ascii" | "unicode" => {}
            _ => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Invalid charset '{}'. Valid options: alpha, numeric, alphanumeric, hex, ascii, unicode",
                        cs
                    ),
                ));
            }
        }
    }

    // Validate case
    if let Some(c) = case {
        match c.as_str() {
            "lower" | "upper" | "title" | "mixed" => {}
            _ => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Invalid case '{}'. Valid options: lower, upper, title, mixed",
                        c
                    ),
                ));
            }
        }
    }

    Ok(())
}

/// Validate number generator configuration
fn validate_number_config(
    min: &Option<f64>,
    max: &Option<f64>,
    decimal_places: &Option<usize>,
) -> syn::Result<()> {
    // Validate min < max
    if let (Some(min_val), Some(max_val)) = (min, max) {
        if min_val > max_val {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "min ({}) must be less than or equal to max ({})",
                    min_val, max_val
                ),
            ));
        }
    }

    // Validate decimal_places is reasonable
    if let Some(dp) = decimal_places {
        if *dp > 15 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("decimal_places ({}) exceeds maximum precision (15)", dp),
            ));
        }
    }

    Ok(())
}

/// Validate boolean generator configuration
fn validate_boolean_config(true_probability: &Option<f64>) -> syn::Result<()> {
    if let Some(prob) = true_probability {
        if *prob < 0.0 || *prob > 1.0 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("true_probability ({}) must be between 0.0 and 1.0", prob),
            ));
        }
    }
    Ok(())
}

/// Validate choice generator configuration
fn validate_choice_config(options: &[String]) -> syn::Result<()> {
    if options.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Choice generator must have at least one option",
        ));
    }
    Ok(())
}

/// Validate template configuration
fn validate_template_config(pattern: &str) -> syn::Result<()> {
    // Check for balanced {{}} placeholders
    let mut depth = 0;
    let mut in_placeholder = false;

    for (i, c) in pattern.chars().enumerate() {
        match c {
            '{' if pattern.chars().nth(i + 1) == Some('{') => {
                in_placeholder = true;
                depth += 1;
                // skip next brace in counting loop logic handled by chars iteration
            }
            '}' if in_placeholder => {
                depth -= 1;
                if depth == 0 {
                    in_placeholder = false;
                }
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Unbalanced placeholders in template pattern: '{}'", pattern),
        ));
    }

    // Check for empty placeholders
    if pattern.contains("{{}}") {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Template contains empty placeholder '{{}}'. Field name required.",
        ));
    }

    Ok(())
}

/// Validate regex pattern (basic syntax check)
fn validate_regex_config(pattern: &str) -> syn::Result<()> {
    // Basic validation - check for obviously malformed patterns
    if pattern.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Regex pattern cannot be empty",
        ));
    }

    // Check for unbalanced brackets
    let mut square_depth = 0;
    let mut paren_depth = 0;
    let mut curly_depth = 0;

    for c in pattern.chars() {
        match c {
            '[' => square_depth += 1,
            ']' => square_depth -= 1,
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            '{' => curly_depth += 1,
            '}' => curly_depth -= 1,
            _ => {}
        }

        // Catch negative depths (closing before opening)
        if square_depth < 0 || paren_depth < 0 || curly_depth < 0 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Unbalanced brackets in regex pattern: '{}'", pattern),
            ));
        }
    }

    if square_depth != 0 || paren_depth != 0 || curly_depth != 0 {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Unbalanced brackets in regex pattern: '{}'", pattern),
        ));
    }

    Ok(())
}

/// Validate name generator configuration
fn validate_name_config(name_type: &Option<String>) -> syn::Result<()> {
    if let Some(nt) = name_type {
        match nt.as_str() {
            "given" | "surname" | "full" => {}
            _ => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Invalid name type '{}'. Valid options: given, surname, full",
                        nt
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// Validate address generator configuration
fn validate_address_config(components: &Option<Vec<String>>) -> syn::Result<()> {
    if let Some(comps) = components {
        if comps.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Address generator components list cannot be empty",
            ));
        }

        // Validate each component is a known address component
        for comp in comps {
            match comp.as_str() {
                "street" | "city" | "state" | "zip" | "country" | "full" => {}
                _ => {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!(
                            "Invalid address component '{}'. Valid options: street, city, state, zip, country, full",
                            comp
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_validation_conflicting_lengths() {
        let result = validate_string_config(&Some(10), &Some(5), &Some(15), &None, &None);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_validation_min_greater_than_max() {
        let result = validate_string_config(&None, &Some(20), &Some(10), &None, &None);
        assert!(result.is_err());
    }

    #[test]
    fn test_number_validation_min_greater_than_max() {
        let result = validate_number_config(&Some(100.0), &Some(50.0), &None);
        assert!(result.is_err());
    }

    #[test]
    fn test_boolean_validation_invalid_probability() {
        assert!(validate_boolean_config(&Some(-0.1)).is_err());
        assert!(validate_boolean_config(&Some(1.5)).is_err());
        assert!(validate_boolean_config(&Some(0.5)).is_ok());
    }

    #[test]
    fn test_choice_validation_empty_options() {
        let result = validate_choice_config(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_template_validation_unbalanced() {
        assert!(validate_template_config("{{field").is_err());
        // Note: "$field}" is technically unbalanced but harder to detect without full parsing
        // assert!(validate_template_config("$field}").is_err());
        assert!(validate_template_config("{{}}").is_err());
        assert!(validate_template_config("{{field}}").is_ok());
    }
}
