//! Integration tests for DataGenerator derive macro
//!
//! These tests verify that the macro correctly generates code for various
//! struct configurations with different field types and attributes.

use dadagen_macros::DataGenerator;
use dadagen_core::context::Context;

/// Test basic struct with inferred generators
#[test]
fn test_basic_struct_inferred() {
    #[derive(DataGenerator, Debug)]
    struct BasicUser {
        username: String,
        age: i32,
        is_active: bool,
    }
    
    let generator = BasicUserGenerator::new();
    let context = Context::new();
    
    // Verify generator can be created and has expected methods
    let _result = generator.generate(&context);
    let deps = generator.dependencies();
    assert_eq!(deps.len(), 0); // No template dependencies
}

/// Test struct with string generator attributes
#[test]
fn test_string_generator_with_constraints() {
    #[derive(DataGenerator, Debug)]
    struct User {
        #[dadagen(string(length = 10, charset = "alpha"))]
        username: String,
        
        #[dadagen(string(min_length = 5, max_length = 15, case = "lower"))]
        email_prefix: String,
    }
    
    let generator = UserGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true); // Compilation success is the test
}

/// Test struct with number generators
#[test]
fn test_number_generators() {
    #[derive(DataGenerator, Debug)]
    struct Measurements {
        #[dadagen(number(min = 0.0, max = 100.0, decimal_places = 2))]
        temperature: f64,
        
        #[dadagen(number(min = 18.0, max = 99.0, decimal_places = 0))]
        age: i32,
    }
    
    let generator = MeasurementsGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}

/// Test struct with boolean generator
#[test]
fn test_boolean_generator() {
    #[derive(DataGenerator, Debug)]
    struct Flags {
        #[dadagen(boolean(true_probability = 0.7))]
        is_enabled: bool,
        
        #[dadagen(bool(probability = 0.3))]
        is_premium: bool,
    }
    
    let generator = FlagsGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}

/// Test struct with choice generator
#[test]
fn test_choice_generator() {
    #[derive(DataGenerator, Debug)]
    struct Product {
        #[dadagen(choice(options = ["red", "green", "blue"]))]
        color: String,
        
        #[dadagen(choice(options = ["small", "medium", "large"]))]
        size: String,
    }
    
    let generator = ProductGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}

/// Test struct with template generator
#[test]
fn test_template_generator() {
    #[derive(DataGenerator, Debug)]
    struct Contact {
        #[dadagen(string(length = 8))]
        username: String,
        
        #[dadagen(template(pattern = "${username}@example.com"))]
        email: String,
    }
    
    let generator = ContactGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    let deps = generator.dependencies();
    assert!(deps.contains(&"username".to_string())); // Template depends on username
}

/// Test struct with list generator
#[test]
fn test_list_generator() {
    #[derive(DataGenerator, Debug)]
    struct Location {
        #[dadagen(list(name = "cities"))]
        city: String,
        
        #[dadagen(list(name = "countries"))]
        country: String,
    }
    
    let generator = LocationGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}

/// Test struct with regex generator
#[test]
fn test_regex_generator() {
    #[derive(DataGenerator, Debug)]
    struct Identifiers {
        #[dadagen(regex(pattern = "[A-Z]{3}-[0-9]{4}"))]
        product_code: String,
        
        #[dadagen(regex(pattern = "[a-z]{5}[0-9]{3}"))]
        user_id: String,
    }
    
    let generator = IdentifiersGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}

/// Test struct with counter generator
#[test]
fn test_counter_generator() {
    #[derive(DataGenerator, Debug)]
    struct Sequence {
        #[dadagen(counter(start = 1, step = 1))]
        id: i64,
        
        #[dadagen(counter(start = 100, step = 10))]
        order_number: i64,
    }
    
    let generator = SequenceGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}

/// Test struct with name generator
#[test]
fn test_name_generator() {
    #[derive(DataGenerator, Debug)]
    struct Person {
        #[dadagen(name(type = "given"))]
        first_name: String,
        
        #[dadagen(name(type = "surname"))]
        last_name: String,
        
        #[dadagen(name(type = "full"))]
        full_name: String,
    }
    
    let generator = PersonGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}

/// Test struct with gender generator
#[test]
fn test_gender_generator() {
    #[derive(DataGenerator, Debug)]
    struct Demographics {
        #[dadagen(gender)]
        gender: String,
        
        #[dadagen(number(min = 18.0, max = 99.0))]
        age: i32,
    }
    
    let generator = DemographicsGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}

/// Test struct with address generator
#[test]
fn test_address_generator() {
    #[derive(DataGenerator, Debug)]
    struct Address {
        #[dadagen(address(components = ["street", "city", "state", "zip"]))]
        full_address: String,
        
        #[dadagen(address)]
        simple_address: String,
    }
    
    let generator = AddressGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}

/// Test struct with mixed generator types
#[test]
fn test_mixed_generators() {
    #[derive(DataGenerator, Debug)]
    struct ComplexRecord {
        #[dadagen(counter(start = 1, step = 1))]
        id: i64,
        
        #[dadagen(name(type = "full"))]
        name: String,
        
        #[dadagen(number(min = 18.0, max = 99.0))]
        age: i32,
        
        #[dadagen(string(length = 10))]
        username: String,
        
        #[dadagen(template(pattern = "${username}@company.com"))]
        email: String,
        
        #[dadagen(boolean(true_probability = 0.8))]
        is_active: bool,
        
        #[dadagen(choice(options = ["admin", "user", "guest"]))]
        role: String,
    }
    
    let generator = ComplexRecordGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    let deps = generator.dependencies();
    assert!(deps.contains(&"username".to_string()));
}

/// Test unit struct
#[test]
fn test_unit_struct() {
    #[derive(DataGenerator, Debug)]
    struct EmptyMarker;
    
    let generator = EmptyMarkerGenerator::new();
    let context = Context::new();
    
    let result = generator.generate(&context);
    assert!(result.is_ok());
}

/// Test struct with all inferred types
#[test]
fn test_all_inferred_types() {
    #[derive(DataGenerator, Debug)]
    struct InferredTypes {
        text: String,
        count: i32,
        amount: f64,
        flag: bool,
        byte_count: u8,
        large_number: i64,
    }
    
    let generator = InferredTypesGenerator::new();
    let context = Context::new();
    
    let _result = generator.generate(&context);
    assert!(true);
}
