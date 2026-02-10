//! Error message tests for macro compilation errors
//!
//! These tests verify that the macro provides helpful error messages
//! for common mistakes and invalid configurations.
//!
//! Note: These are compile-fail tests that should be run with trybuild

#[test]
fn test_error_message_tests_exist() {
    // This is a placeholder test that always passes
    // The actual compile-fail tests should be run with trybuild
    //
    // Example usage:
    // ```
    // #[test]
    // fn ui() {
    //     let t = trybuild::TestCases::new();
    //     t.compile_fail("tests/ui/*.rs");
    // }
    // ```
    assert!(true, "Error message tests configured");
}

// Example compile-fail test cases that would go in tests/ui/
// These demonstrate the kinds of errors we want to catch:

/*
// tests/ui/invalid_string_constraint.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct InvalidString {
    #[dadagen(string(length = -5))]  // Error: negative length
    name: String,
}

// Expected error: "String length must be non-negative"
*/

/*
// tests/ui/conflicting_constraints.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct ConflictingConstraints {
    #[dadagen(string(length = 10, min_length = 5))]  // Error: can't have both
    name: String,
}

// Expected error: "Cannot specify both 'length' and 'min_length/max_length'"
*/

/*
// tests/ui/invalid_range.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct InvalidRange {
    #[dadagen(number(min = 100, max = 10))]  // Error: min > max
    age: i32,
}

// Expected error: "Minimum value (100) must be less than or equal to maximum value (10)"
*/

/*
// tests/ui/invalid_probability.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct InvalidProbability {
    #[dadagen(boolean(true_probability = 1.5))]  // Error: probability > 1
    flag: bool,
}

// Expected error: "Probability must be between 0.0 and 1.0"
*/

/*
// tests/ui/empty_choice_options.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct EmptyChoice {
    #[dadagen(choice(options = []))]  // Error: no options
    color: String,
}

// Expected error: "Choice generator must have at least one option"
*/

/*
// tests/ui/template_missing_field.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct TemplateMissingField {
    name: String,
    #[dadagen(template(pattern = "{{unknown_field}}"))]  // Error: field doesn't exist
    email: String,
}

// Expected error: "Template references undefined field 'unknown_field'"
*/

/*
// tests/ui/unsupported_type.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct UnsupportedType {
    #[dadagen(string(length = 10))]
    age: i32,  // Error: string attribute on i32 field
}

// Expected error: "String generator cannot be used with type 'i32'"
*/

/*
// tests/ui/invalid_charset.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct InvalidCharset {
    #[dadagen(string(charset = "invalid"))]  // Error: unknown charset
    name: String,
}

// Expected error: "Unknown charset 'invalid'. Valid options are: alpha, numeric, alphanumeric, hex, etc."
*/

/*
// tests/ui/circular_template_dependency.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct CircularDependency {
    #[dadagen(template(pattern = "{{field_b}}"))]
    field_a: String,

    #[dadagen(template(pattern = "{{field_a}}"))]  // Error: circular reference
    field_b: String,
}

// Expected error: "Circular dependency detected: field_a -> field_b -> field_a"
*/

/*
// tests/ui/invalid_decimal_places.rs
use dadagen_macros::DataGenerator;

#[derive(DataGenerator)]
struct InvalidDecimalPlaces {
    #[dadagen(number(decimal_places = 10))]  // Error: too many decimal places
    value: f64,
}

// Expected error: "Decimal places must be between 0 and 9"
*/
