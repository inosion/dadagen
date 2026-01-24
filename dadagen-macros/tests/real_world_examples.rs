//! Real-world integration tests demonstrating practical usage patterns
//!
//! These tests show how the DataGenerator macro would be used in actual
//! applications to generate realistic test data.

use dadagen_macros::DataGenerator;
use dadagen_core::context::Context;

/// E-commerce user with comprehensive attributes
#[test]
fn test_ecommerce_user_generator() {
    #[derive(DataGenerator, Debug)]
    struct EcommerceUser {
        #[dadagen(string(min_length = 3, max_length = 20, charset = "alphanumeric"))]
        username: String,
        
        #[dadagen(string(min_length = 8, max_length = 30))]
        display_name: String,
        
        #[dadagen(template(pattern = "${username}@example.com"))]
        email: String,
        
        #[dadagen(number(min = 18, max = 99))]
        age: i32,
        
        #[dadagen(boolean(true_probability = 0.85))]
        is_verified: bool,
        
        #[dadagen(boolean(true_probability = 0.25))]
        is_premium: bool,
        
        #[dadagen(choice(options = ["active", "suspended", "pending", "inactive"]))]
        status: String,
    }
    
    let generator = EcommerceUserGenerator::new();
    let context = Context::new();
    
    // Generate multiple users to verify consistency
    for _ in 0..10 {
        let user = generator.generate(&context);
        assert!(user.is_ok(), "Should successfully generate user");
    }
}

/// Financial transaction record with precise numeric constraints
#[test]
fn test_financial_transaction_generator() {
    #[derive(DataGenerator, Debug)]
    struct Transaction {
        #[dadagen(string(length = 16, charset = "alphanumeric"))]
        transaction_id: String,
        
        #[dadagen(number(min = 0.01, max = 10000.0, decimal_places = 2))]
        amount: f64,
        
        #[dadagen(choice(options = ["USD", "EUR", "GBP", "JPY"]))]
        currency: String,
        
        #[dadagen(choice(options = ["pending", "completed", "failed", "refunded"]))]
        status: String,
        
        #[dadagen(choice(options = ["credit_card", "debit_card", "paypal", "wire_transfer"]))]
        payment_method: String,
        
        #[dadagen(boolean(true_probability = 0.95))]
        is_successful: bool,
    }
    
    let generator = TransactionGenerator::new();
    let context = Context::new();
    
    for _ in 0..5 {
        let tx = generator.generate(&context);
        assert!(tx.is_ok(), "Should successfully generate transaction");
    }
}

/// Product catalog entry with various field types
#[test]
fn test_product_catalog_generator() {
    #[derive(DataGenerator, Debug)]
    struct Product {
        #[dadagen(string(length = 10, charset = "alphanumeric"))]
        sku: String,
        
        #[dadagen(string(min_length = 10, max_length = 100))]
        name: String,
        
        #[dadagen(string(min_length = 50, max_length = 500))]
        description: String,
        
        #[dadagen(number(min = 0.01, max = 9999.99, decimal_places = 2))]
        price: f64,
        
        #[dadagen(number(min = 0, max = 1000))]
        stock_quantity: i32,
        
        #[dadagen(choice(options = ["electronics", "clothing", "food", "books", "toys"]))]
        category: String,
        
        #[dadagen(boolean(true_probability = 0.9))]
        is_available: bool,
        
        #[dadagen(boolean(true_probability = 0.2))]
        is_featured: bool,
    }
    
    let generator = ProductGenerator::new();
    let context = Context::new();
    
    let product = generator.generate(&context);
    assert!(product.is_ok(), "Should successfully generate product");
}

/// User profile with template dependencies
#[test]
fn test_user_profile_with_templates() {
    #[derive(DataGenerator, Debug)]
    struct UserProfile {
        #[dadagen(string(min_length = 5, max_length = 15, charset = "alpha"))]
        first_name: String,
        
        #[dadagen(string(min_length = 5, max_length = 20, charset = "alpha"))]
        last_name: String,
        
        #[dadagen(template(pattern = "${first_name}.${last_name}"))]
        username: String,
        
        #[dadagen(template(pattern = "${username}@company.com"))]
        email: String,
        
        #[dadagen(template(pattern = "+1-555-${phone_suffix}"))]
        phone: String,
        
        #[dadagen(number(min = 1000, max = 9999))]
        phone_suffix: i32,
    }
    
    let generator = UserProfileGenerator::new();
    let context = Context::new();
    
    // Verify dependencies are tracked correctly
    let deps = generator.dependencies();
    assert!(!deps.is_empty(), "Should have template dependencies");
    
    let profile = generator.generate(&context);
    assert!(profile.is_ok(), "Should successfully generate profile");
}

/// IoT sensor data with realistic numeric ranges
#[test]
fn test_sensor_data_generator() {
    #[derive(DataGenerator, Debug)]
    struct SensorReading {
        #[dadagen(string(length = 12, charset = "alphanumeric"))]
        sensor_id: String,
        
        #[dadagen(number(min = -40.0, max = 85.0, decimal_places = 2))]
        temperature_celsius: f64,
        
        #[dadagen(number(min = 0.0, max = 100.0, decimal_places = 1))]
        humidity_percent: f64,
        
        #[dadagen(number(min = 900.0, max = 1100.0, decimal_places = 2))]
        pressure_hpa: f64,
        
        #[dadagen(boolean(true_probability = 0.98))]
        is_calibrated: bool,
        
        #[dadagen(choice(options = ["online", "offline", "error", "maintenance"]))]
        status: String,
    }
    
    let generator = SensorReadingGenerator::new();
    let context = Context::new();
    
    for _ in 0..20 {
        let reading = generator.generate(&context);
        assert!(reading.is_ok(), "Should successfully generate sensor reading");
    }
}

/// API request log entry
#[test]
fn test_api_log_entry_generator() {
    #[derive(DataGenerator, Debug)]
    struct ApiLogEntry {
        #[dadagen(string(length = 32, charset = "hex"))]
        request_id: String,
        
        #[dadagen(choice(options = ["GET", "POST", "PUT", "DELETE", "PATCH"]))]
        method: String,
        
        #[dadagen(choice(options = ["/api/users", "/api/products", "/api/orders", "/api/health"]))]
        endpoint: String,
        
        #[dadagen(choice(options = ["200", "201", "400", "401", "403", "404", "500"]))]
        status_code: String,
        
        #[dadagen(number(min = 10, max = 5000))]
        response_time_ms: i32,
        
        #[dadagen(boolean(true_probability = 0.95))]
        is_successful: bool,
    }
    
    let generator = ApiLogEntryGenerator::new();
    let context = Context::new();
    
    let log_entry = generator.generate(&context);
    assert!(log_entry.is_ok(), "Should successfully generate log entry");
}

/// Customer support ticket
#[test]
fn test_support_ticket_generator() {
    #[derive(DataGenerator, Debug)]
    struct SupportTicket {
        #[dadagen(string(length = 8, charset = "alphanumeric"))]
        ticket_id: String,
        
        #[dadagen(string(min_length = 20, max_length = 200))]
        subject: String,
        
        #[dadagen(string(min_length = 50, max_length = 1000))]
        description: String,
        
        #[dadagen(choice(options = ["low", "medium", "high", "critical"]))]
        priority: String,
        
        #[dadagen(choice(options = ["open", "in_progress", "resolved", "closed"]))]
        status: String,
        
        #[dadagen(choice(options = ["technical", "billing", "feature_request", "bug"]))]
        category: String,
        
        #[dadagen(boolean(true_probability = 0.7))]
        is_assigned: bool,
    }
    
    let generator = SupportTicketGenerator::new();
    let context = Context::new();
    
    let ticket = generator.generate(&context);
    assert!(ticket.is_ok(), "Should successfully generate support ticket");
}

/// Nested struct with complex relationships
#[test]
fn test_nested_struct_generator() {
    #[derive(DataGenerator, Debug)]
    struct Address {
        #[dadagen(string(min_length = 5, max_length = 50))]
        street: String,
        
        #[dadagen(string(min_length = 3, max_length = 30))]
        city: String,
        
        #[dadagen(choice(options = ["CA", "NY", "TX", "FL"]))]
        state: String,
        
        #[dadagen(string(length = 5, charset = "numeric"))]
        zip_code: String,
    }
    
    #[derive(DataGenerator, Debug)]
    struct Customer {
        #[dadagen(string(min_length = 2, max_length = 30))]
        name: String,
        
        #[dadagen(template(pattern = "${name}@email.com"))]
        email: String,
        
        #[dadagen(number(min = 18, max = 99))]
        age: i32,
        
        // Note: Nested struct generation would require additional implementation
        // For now, we test that the macro doesn't break with nested types
        billing_address: String,
        shipping_address: String,
    }
    
    let generator = CustomerGenerator::new();
    let context = Context::new();
    
    let customer = generator.generate(&context);
    assert!(customer.is_ok(), "Should successfully generate customer");
}

/// Game character with stats
#[test]
fn test_game_character_generator() {
    #[derive(DataGenerator, Debug)]
    struct GameCharacter {
        #[dadagen(string(min_length = 3, max_length = 20, charset = "alpha"))]
        name: String,
        
        #[dadagen(choice(options = ["warrior", "mage", "rogue", "priest"]))]
        class: String,
        
        #[dadagen(number(min = 1, max = 100))]
        level: i32,
        
        #[dadagen(number(min = 100, max = 10000))]
        health: i32,
        
        #[dadagen(number(min = 50, max = 1000))]
        mana: i32,
        
        #[dadagen(number(min = 10.0, max = 100.0, decimal_places = 1))]
        strength: f64,
        
        #[dadagen(number(min = 10.0, max = 100.0, decimal_places = 1))]
        intelligence: f64,
        
        #[dadagen(number(min = 10.0, max = 100.0, decimal_places = 1))]
        agility: f64,
        
        #[dadagen(boolean(true_probability = 0.1))]
        is_legendary: bool,
    }
    
    let generator = GameCharacterGenerator::new();
    let context = Context::new();
    
    let character = generator.generate(&context);
    assert!(character.is_ok(), "Should successfully generate game character");
}

/// Blog post with realistic content
#[test]
fn test_blog_post_generator() {
    #[derive(DataGenerator, Debug)]
    struct BlogPost {
        #[dadagen(string(length = 24, charset = "alphanumeric"))]
        post_id: String,
        
        #[dadagen(string(min_length = 10, max_length = 100))]
        title: String,
        
        #[dadagen(string(min_length = 20, max_length = 100))]
        author: String,
        
        #[dadagen(string(min_length = 100, max_length = 5000))]
        content: String,
        
        #[dadagen(choice(options = ["technology", "lifestyle", "business", "science", "entertainment"]))]
        category: String,
        
        #[dadagen(boolean(true_probability = 0.8))]
        is_published: bool,
        
        #[dadagen(number(min = 0, max = 10000))]
        view_count: i32,
        
        #[dadagen(number(min = 0, max = 500))]
        like_count: i32,
    }
    
    let generator = BlogPostGenerator::new();
    let context = Context::new();
    
    let post = generator.generate(&context);
    assert!(post.is_ok(), "Should successfully generate blog post");
}
