# Dadagen Macro Examples

Comprehensive examples demonstrating all features of the dadagen procedural macros.

## Table of Contents

- [Basic Usage](#basic-usage)
- [String Generators](#string-generators)
- [Number Generators](#number-generators)
- [Boolean Generators](#boolean-generators)
- [Choice Generators](#choice-generators)
- [Template Generators](#template-generators)
- [List Generators](#list-generators)
- [Complex Examples](#complex-examples)
- [Best Practices](#best-practices)

## Basic Usage

### Simple Struct with Type Inference

The macro automatically infers appropriate generators based on field types:

```rust
use dadagen_macros::DataGenerator;

#[derive(DataGenerator, Debug)]
struct User {
    username: String,
    age: i32,
    is_active: bool,
}

fn main() {
    let generator = UserGenerator::new();
    let context = Context::new();
    let user = generator.generate(&context).unwrap();
    println!("{:?}", user);
}
```

## String Generators

### Fixed Length Strings

```rust
#[derive(DataGenerator)]
struct Product {
    #[dadagen(string(length = 10))]
    sku: String,
}
```

### Variable Length Strings

```rust
#[derive(DataGenerator)]
struct Article {
    #[dadagen(string(min_length = 100, max_length = 5000))]
    content: String,
}
```

### Charset Constraints

```rust
#[derive(DataGenerator)]
struct Identifiers {
    #[dadagen(string(length = 8, charset = "alphanumeric"))]
    id: String,
    
    #[dadagen(string(length = 32, charset = "hex"))]
    api_key: String,
    
    #[dadagen(string(length = 10, charset = "alpha"))]
    code: String,
    
    #[dadagen(string(length = 6, charset = "numeric"))]
    pin: String,
}
```

### Case Transformations

```rust
#[derive(DataGenerator)]
struct Names {
    #[dadagen(string(min_length = 5, max_length = 20, case = "upper"))]
    company_code: String,
    
    #[dadagen(string(min_length = 5, max_length = 20, case = "lower"))]
    username: String,
    
    #[dadagen(string(min_length = 5, max_length = 20, case = "title"))]
    display_name: String,
}
```

## Number Generators

### Integer Ranges

```rust
#[derive(DataGenerator)]
struct Demographics {
    #[dadagen(number(min = 18, max = 99))]
    age: i32,
    
    #[dadagen(number(min = 0, max = 200))]
    height_cm: i32,
}
```

### Floating Point with Precision

```rust
#[derive(DataGenerator)]
struct Measurements {
    #[dadagen(number(min = 0.0, max = 100.0, decimal_places = 2))]
    percentage: f64,
    
    #[dadagen(number(min = -273.15, max = 1000.0, decimal_places = 1))]
    temperature_celsius: f64,
    
    #[dadagen(number(min = 0.01, max = 999999.99, decimal_places = 2))]
    price: f64,
}
```

### Different Integer Types

```rust
#[derive(DataGenerator)]
struct Counters {
    #[dadagen(number(min = 0, max = 255))]
    small_count: u8,
    
    #[dadagen(number(min = 0, max = 65535))]
    medium_count: u16,
    
    #[dadagen(number(min = 0, max = 4294967295))]
    large_count: u32,
}
```

## Boolean Generators

### Weighted Probabilities

```rust
#[derive(DataGenerator)]
struct Flags {
    #[dadagen(boolean(true_probability = 0.8))]
    is_verified: bool,
    
    #[dadagen(bool(probability = 0.1))]
    is_premium: bool,
    
    #[dadagen(boolean(true_probability = 0.5))]
    is_active: bool,
}
```

## Choice Generators

### Simple Choice Lists

```rust
#[derive(DataGenerator)]
struct Product {
    #[dadagen(choice(options = ["red", "green", "blue", "yellow"]))]
    color: String,
    
    #[dadagen(choice(options = ["small", "medium", "large", "xlarge"]))]
    size: String,
}
```

### Status Enums

```rust
#[derive(DataGenerator)]
struct Order {
    #[dadagen(choice(options = ["pending", "processing", "shipped", "delivered", "cancelled"]))]
    status: String,
    
    #[dadagen(choice(options = ["credit_card", "debit_card", "paypal", "crypto"]))]
    payment_method: String,
}
```

## Template Generators

### Simple Templates

```rust
#[derive(DataGenerator)]
struct UserProfile {
    #[dadagen(string(min_length = 5, max_length = 15))]
    username: String,
    
    #[dadagen(template(pattern = "{{username}}@example.com"))]
    email: String,
}
```

### Multi-Field Templates

```rust
#[derive(DataGenerator)]
struct Person {
    #[dadagen(string(min_length = 3, max_length = 20))]
    first_name: String,
    
    #[dadagen(string(min_length = 3, max_length = 20))]
    last_name: String,
    
    #[dadagen(template(pattern = "{{first_name}}.{{last_name}}"))]
    username: String,
    
    #[dadagen(template(pattern = "{{username}}@company.com"))]
    email: String,
    
    #[dadagen(template(pattern = "{{first_name}} {{last_name}}"))]
    full_name: String,
}
```

### Templates with Numbers

```rust
#[derive(DataGenerator)]
struct Contact {
    #[dadagen(number(min = 100, max = 999))]
    area_code: i32,
    
    #[dadagen(number(min = 1000000, max = 9999999))]
    phone_number: i32,
    
    #[dadagen(template(pattern = "({{area_code}}) {{phone_number}}"))]
    formatted_phone: String,
}
```

## List Generators

### Predefined Lists

```rust
#[derive(DataGenerator)]
struct Person {
    #[dadagen(list(name = "firstnames"))]
    first_name: String,
    
    #[dadagen(list(name = "surnames"))]
    last_name: String,
    
    #[dadagen(list(name = "cities"))]
    city: String,
}
```

### List with Discriminator

```rust
#[derive(DataGenerator)]
struct Address {
    #[dadagen(list(name = "countries"))]
    country: String,
    
    #[dadagen(list(name = "cities", discriminator = "country"))]
    city: String,
}
```

## Complex Examples

### E-commerce Order

```rust
#[derive(DataGenerator, Debug)]
struct Order {
    #[dadagen(string(length = 16, charset = "alphanumeric"))]
    order_id: String,
    
    #[dadagen(string(min_length = 5, max_length = 30))]
    customer_name: String,
    
    #[dadagen(template(pattern = "{{customer_name}}@example.com"))]
    customer_email: String,
    
    #[dadagen(number(min = 1, max = 100))]
    item_count: i32,
    
    #[dadagen(number(min = 0.01, max = 10000.0, decimal_places = 2))]
    total_amount: f64,
    
    #[dadagen(choice(options = ["USD", "EUR", "GBP"]))]
    currency: String,
    
    #[dadagen(choice(options = ["pending", "processing", "shipped", "delivered"]))]
    status: String,
    
    #[dadagen(boolean(true_probability = 0.95))]
    is_paid: bool,
    
    #[dadagen(boolean(true_probability = 0.2))]
    is_express_shipping: bool,
}
```

### API Log Entry

```rust
#[derive(DataGenerator, Debug)]
struct ApiLogEntry {
    #[dadagen(string(length = 32, charset = "hex"))]
    request_id: String,
    
    #[dadagen(choice(options = ["GET", "POST", "PUT", "DELETE", "PATCH"]))]
    http_method: String,
    
    #[dadagen(choice(options = ["/api/users", "/api/products", "/api/orders"]))]
    endpoint: String,
    
    #[dadagen(number(min = 100, max = 599))]
    status_code: i32,
    
    #[dadagen(number(min = 1, max = 5000))]
    response_time_ms: i32,
    
    #[dadagen(number(min = 0, max = 1000000))]
    response_size_bytes: i32,
    
    #[dadagen(boolean(true_probability = 0.95))]
    is_successful: bool,
    
    #[dadagen(string(length = 15))]
    client_ip: String,
}
```

### Game Character

```rust
#[derive(DataGenerator, Debug)]
struct GameCharacter {
    #[dadagen(string(min_length = 3, max_length = 20, charset = "alpha"))]
    name: String,
    
    #[dadagen(choice(options = ["warrior", "mage", "rogue", "priest", "paladin"]))]
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
    
    #[dadagen(number(min = 10.0, max = 100.0, decimal_places = 1))]
    stamina: f64,
    
    #[dadagen(boolean(true_probability = 0.05))]
    is_legendary: bool,
    
    #[dadagen(boolean(true_probability = 0.3))]
    is_elite: bool,
}
```

### Financial Transaction

```rust
#[derive(DataGenerator, Debug)]
struct Transaction {
    #[dadagen(string(length = 20, charset = "alphanumeric"))]
    transaction_id: String,
    
    #[dadagen(string(min_length = 10, max_length = 50))]
    sender: String,
    
    #[dadagen(string(min_length = 10, max_length = 50))]
    receiver: String,
    
    #[dadagen(number(min = 0.01, max = 1000000.0, decimal_places = 2))]
    amount: f64,
    
    #[dadagen(choice(options = ["USD", "EUR", "GBP", "JPY", "CHF"]))]
    currency: String,
    
    #[dadagen(choice(options = ["wire", "card", "ach", "swift"]))]
    transfer_type: String,
    
    #[dadagen(choice(options = ["pending", "processing", "completed", "failed", "cancelled"]))]
    status: String,
    
    #[dadagen(boolean(true_probability = 0.98))]
    is_verified: bool,
    
    #[dadagen(boolean(true_probability = 0.02))]
    is_flagged: bool,
    
    #[dadagen(template(pattern = "TXN-{{transaction_id}}"))]
    reference_number: String,
}
```

## Best Practices

### 1. Use Meaningful Field Names

Field names are used in template substitution, so choose names carefully:

```rust
// Good
#[derive(DataGenerator)]
struct User {
    first_name: String,
    #[dadagen(template(pattern = "{{first_name}}@example.com"))]
    email: String,
}

// Avoid
#[derive(DataGenerator)]
struct User {
    fn: String,  // Unclear abbreviation
    #[dadagen(template(pattern = "{{fn}}@example.com"))]
    e: String,  // Unclear abbreviation
}
```

### 2. Set Realistic Constraints

Use constraints that match your domain requirements:

```rust
#[derive(DataGenerator)]
struct User {
    // Realistic username length
    #[dadagen(string(min_length = 3, max_length = 20, charset = "alphanumeric"))]
    username: String,
    
    // Realistic age range
    #[dadagen(number(min = 18, max = 120))]
    age: i32,
    
    // Most users are regular members
    #[dadagen(boolean(true_probability = 0.05))]
    is_admin: bool,
}
```

### 3. Use Templates for Derived Fields

When fields depend on other fields, use templates:

```rust
#[derive(DataGenerator)]
struct Employee {
    employee_id: i32,
    first_name: String,
    last_name: String,
    
    #[dadagen(template(pattern = "{{first_name}}.{{last_name}}@company.com"))]
    email: String,
    
    #[dadagen(template(pattern = "EMP-{{employee_id}}"))]
    badge_number: String,
}
```

### 4. Consider Probability in Boolean Fields

Set probabilities based on real-world distributions:

```rust
#[derive(DataGenerator)]
struct Account {
    #[dadagen(boolean(true_probability = 0.95))]  // Most accounts are active
    is_active: bool,
    
    #[dadagen(boolean(true_probability = 0.1))]   // Few are premium
    is_premium: bool,
    
    #[dadagen(boolean(true_probability = 0.01))]  // Very few are flagged
    is_flagged: bool,
}
```

### 5. Use Choice for Enums and Status Fields

Explicitly list all valid options:

```rust
#[derive(DataGenerator)]
struct Task {
    #[dadagen(choice(options = ["todo", "in_progress", "review", "done"]))]
    status: String,
    
    #[dadagen(choice(options = ["low", "medium", "high", "critical"]))]
    priority: String,
}
```

### 6. Match Number Precision to Use Case

```rust
#[derive(DataGenerator)]
struct Measurements {
    // Currency: 2 decimal places
    #[dadagen(number(min = 0.01, max = 9999.99, decimal_places = 2))]
    price: f64,
    
    // Scientific: 4 decimal places
    #[dadagen(number(min = 0.0, max = 100.0, decimal_places = 4))]
    concentration: f64,
    
    // Temperature: 1 decimal place
    #[dadagen(number(min = -40.0, max = 50.0, decimal_places = 1))]
    temperature: f64,
}
```

### 7. Use Lists for Realistic Data

When available, use predefined lists for more realistic data:

```rust
#[derive(DataGenerator)]
struct Person {
    #[dadagen(list(name = "firstnames"))]
    first_name: String,
    
    #[dadagen(list(name = "surnames"))]
    last_name: String,
    
    #[dadagen(list(name = "cities"))]
    city: String,
}
```

## Debugging

### Enable Debug Output

Set the environment variable to see macro expansion:

```bash
DADAGEN_MACRO_DEBUG=1 cargo build
```

### Use cargo-expand

Install and use cargo-expand to see the generated code:

```bash
cargo install cargo-expand
cargo expand --lib
```

### Test Individual Structs

Create small test cases to verify behavior:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use dadagen_core::context::Context;
    
    #[test]
    fn test_user_generation() {
        let generator = UserGenerator::new();
        let context = Context::new();
        
        for _ in 0..100 {
            let user = generator.generate(&context).unwrap();
            // Add assertions to verify constraints
            assert!(!user.username.is_empty());
            assert!(user.age >= 18 && user.age <= 99);
        }
    }
}
```
