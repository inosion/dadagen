use dadagen_core::dependency::DependencyGraph;
use dadagen_core::parser::parse_dsl;


fn main2() {
    // Example DSL with dependencies
    let dsl_input = r#"
        field { "first_name" string }
        field { "last_name" string }
        field { "email" template "{{first_name}}.{{last_name}}@example.com" }
        field { "country" list(name="countries") }
        field { "city" list(name="cities", discriminator="country") }
    "#;

    // Parse DSL to AST
    let doc = parse_dsl(dsl_input).expect("Failed to parse DSL");
    println!("Parsed {} fields", doc.fields.len());

    // Build dependency graph
    let graph = DependencyGraph::from_document(&doc).expect("Failed to build dependency graph");
    println!("\nDependency Graph:");
    println!("  Total fields: {}", graph.field_count());

    // Show dependencies for each field
    for field in doc.fields.iter() {
        if let Some(deps) = graph.get_dependencies(&field.name) {
            if !deps.is_empty() {
                println!("  {} depends on: {:?}", field.name, deps);
            }
        }
    }

    // Get generation order
    let order = graph.topological_sort().expect("Failed to sort");
    println!("\nGeneration Order:");
    for (i, field) in order.iter().enumerate() {
        println!("  {}. {}", i + 1, field);
    }
}


fn main() {
    // Example DSL with dependencies
    let dsl_input = r#"
        field { "first_name" string }
        field { "last_name" string }
        field { "country" string }
        field { "city" list(name="cities", discriminator="country") }
        field { "postal_code" list(name="postal_codes", discriminator="city") }
    "#;

    // Parse DSL to AST
    let doc = parse_dsl(dsl_input).expect("Failed to parse DSL");
    println!("Parsed {} fields", doc.fields.len());

    // Build dependency graph
    let graph = DependencyGraph::from_document(&doc).expect("Failed to build dependency graph");
    println!("\nDependency Graph:");
    println!("  Total fields: {}", graph.field_count());

    // Show dependencies for each field
    for field in doc.fields.iter() {
        if let Some(deps) = graph.get_dependencies(&field.name) {
            if !deps.is_empty() {
                println!("  {} depends on: {:?}", field.name, deps);
            }
        }
    }

    // Get generation order
    let order = graph.topological_sort().expect("Failed to sort");
    println!("\nGeneration Order:");
    for (i, field) in order.iter().enumerate() {
        println!("  {}. {}", i + 1, field);
    }

    main2();
}
