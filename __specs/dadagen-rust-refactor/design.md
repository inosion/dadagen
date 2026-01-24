# Design Document: Dadagen Rust Library Refactor

### Overview

This design document outlines the architecture and implementation strategy for refactoring dadagen into a comprehensive Rust-based test data generation platform. The system will provide procedural macro support, cross-platform applications, WASM deployment, and Python bindings while maintaining high performance and usability.

### Architecture

The dadagen ecosystem will consist of multiple interconnected components built around a shared core library:

```
┌─────────────────────────────────────────────────────────────┐
│                    Dadagen Ecosystem                        │
├─────────────────────────────────────────────────────────────┤
│  GUI App (Tauri)  │  Web App (WASM)  │  Python Ext (PyO3) │
├─────────────────────────────────────────────────────────────┤
│              Procedural Macros (proc-macro)                 │
├─────────────────────────────────────────────────────────────┤
│                   Core Library (dadagen-core)               │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐│
│  │ DSL Parser  │ │ Generators  │ │    Data Structures      ││
│  │   (Pest)    │ │   Engine    │ │   (Context, Lists)      ││
│  └─────────────┘ └─────────────┘ └─────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│                  File Parsers & Utils                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐│
│  │ CSV Parser  │ │JSON Parser  │ │    Excel Parser         ││
│  └─────────────┘ └─────────────┘ └─────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### Components and Interfaces

#### 1. Core Library (dadagen-core)

**Purpose**: Central library containing all core functionality for data generation

**Key Components**:
- **Generator Trait System**: Unified interface for all data generators
- **Context Management**: Thread-safe state management for data generation
- **DSL Parser**: Pest-based parser for dadagen DSL syntax
- **List Manager**: Efficient storage and retrieval of reference data

**Public API**:
```rust
pub trait Generator<T: Clone> {
    fn generate(&self, context: &mut Context) -> T;
    fn dependencies(&self) -> Vec<String>;
}

pub struct Context {
    current_iter: u64,
    data_field_state: HashMap<String, Box<dyn AnyClone>>,
}

pub struct DslEngine {
    pub fn parse_dsl(input: &str) -> Result<Vec<FieldDefinition>, ParseError>;
    pub fn execute_generation(definitions: &[FieldDefinition], count: usize) -> Result<Vec<GeneratedRecord>, GenerationError>;
}
```

#### 2. Procedural Macro Crate (dadagen-macros)

**Purpose**: Compile-time code generation for struct annotation

**Key Features**:
- Derive macro `#[derive(TestDadagen)]`
- Field attribute support for generation constraints
- Type analysis and appropriate generator selection
- Integration with core generator trait system

**Generated Code Pattern**:
```rust
impl TestDataGenerator for MyStruct {
    fn generate() -> Self {
        Self {
            field1: StringGenerator::new("field1", 10..50).generate(&mut Context::new()),
            field2: IntegerGenerator::new("field2", 1, 100).generate(&mut Context::new()),
        }
    }
    
    fn generate_with_context(context: &mut Context) -> Self {
        // Context-aware generation for related fields
    }
}
```

#### 3. Cross-Platform GUI Application (dadagen-gui)

**Framework**: Tauri with React/TypeScript frontend

**Architecture**:
```
Frontend (React/TS)     Backend (Rust/Tauri)
┌─────────────────┐    ┌─────────────────────┐
│  File Upload    │────│  File Parser        │
│  Data Preview   │────│  Type Inference     │
│  DSL Editor     │────│  DSL Generator      │
│  Export Dialog  │────│  File Operations    │
└─────────────────┘    └─────────────────────┘
```

**Key Modules**:
- **File Analysis Engine**: Infers data types and patterns from sample files
- **DSL Generation Engine**: Converts analyzed data into dadagen DSL
- **Real-time Preview**: Shows generated data samples as user configures
- **Export Manager**: Handles DSL and data export to various formats

#### 4. WASM Web Application (dadagen-web)

**Framework**: Yew or Leptos with wasm-bindgen

**Architecture**:
```rust
#[wasm_bindgen]
pub struct DadagenWasm {
    engine: DslEngine,
    context: Context,
}

#[wasm_bindgen]
impl DadagenWasm {
    pub fn new() -> Self;
    pub fn parse_file(&mut self, file_data: &[u8], file_type: &str) -> Result<String, JsValue>;
    pub fn generate_dsl(&self, analysis: &str) -> Result<String, JsValue>;
    pub fn generate_data(&mut self, dsl: &str, count: usize) -> Result<String, JsValue>;
}
```

**Key Features**:
- Client-side file processing with FileReader API
- Progressive Web App (PWA) capabilities
- Responsive design for mobile devices
- Offline functionality with service workers

#### 5. Python Extension (dadagen-python)

**Framework**: PyO3 with maturin for packaging

**Python API Design**:
```python
from dadagen import Generator, Context, parse_dsl

# High-level API
generator = Generator.from_dsl("field { name string between 5 and 20 }")
data = generator.generate(count=1000)

# Low-level API for performance
context = Context()
for i in range(1000):
    record = generator.generate_with_context(context)
    
# Integration with existing tools
import pytest
from dadagen.pytest import dadagen_fixture

@dadagen_fixture("user_data.dadagen")
def test_user_processing(generated_users):
    assert len(generated_users) == 100
```

### Data Models

#### Core Data Structures

```rust
#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub generator_type: GeneratorType,
    pub constraints: HashMap<String, ConstraintValue>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum GeneratorType {
    String(StringConfig),
    Number(NumberConfig),
    Template(TemplateConfig),
    List(ListConfig),
    Custom(CustomConfig),
}

#[derive(Debug, Clone)]
pub struct GeneratedRecord {
    pub fields: HashMap<String, GeneratedValue>,
    pub metadata: RecordMetadata,
}

pub trait AnyClone {
    fn clone_box(&self) -> Box<dyn AnyClone>;
}

impl<T: Clone + 'static> AnyClone for T {
    fn clone_box(&self) -> Box<dyn AnyClone> {
        Box::new(self.clone())
    }
}
```

#### Context and State Management

```rust
pub struct Context {
    current_iter: u64,
    data_field_state: HashMap<String, Box<dyn AnyClone>>,
    random_seed: Option<u64>,
    generation_metadata: GenerationMetadata,
}

impl Context {
    pub fn new() -> Self;
    pub fn with_seed(seed: u64) -> Self;
    pub fn insert_field_state<T: Clone + 'static>(&mut self, key: String, value: T);
    pub fn get_field_state<T: Clone + 'static>(&self, key: &str) -> Option<T>;
    pub fn increment_iteration(&mut self);
}
```

### Error Handling

#### Error Type Hierarchy

```rust
#[derive(Debug, thiserror::Error)]
pub enum DadagenError {
    #[error("Parse error: {message} at line {line}, column {column}")]
    ParseError { message: String, line: usize, column: usize },
    
    #[error("Generation error: {message}")]
    GenerationError { message: String },
    
    #[error("File operation error: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
    
    #[error("Dependency resolution error: circular dependency detected in {field}")]
    CircularDependencyError { field: String },
}

pub type Result<T> = std::result::Result<T, DadagenError>;
```

### Testing Strategy

#### Unit Testing Framework

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    // Property-based testing for generators
    proptest! {
        #[test]
        fn string_generator_respects_length_bounds(
            min in 1usize..100,
            max in 100usize..1000
        ) {
            let generator = StringGenerator::new("test", min..max);
            let result = generator.generate(&mut Context::new());
            prop_assert!(result.len() >= min && result.len() <= max);
        }
    }
    
    // Integration testing
    #[test]
    fn dsl_roundtrip_parsing() {
        let dsl = r#"field { "name" string between 5 and 20 }"#;
        let parsed = DslEngine::parse_dsl(dsl).unwrap();
        let regenerated = DslEngine::to_dsl(&parsed);
        assert_eq!(normalize_dsl(dsl), normalize_dsl(&regenerated));
    }
}
```

#### Performance Testing

```rust
#[cfg(test)]
mod benchmarks {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn benchmark_generation_speed(c: &mut Criterion) {
        c.bench_function("generate_1000_records", |b| {
            let engine = DslEngine::new();
            let dsl = include_str!("../test_data/complex_schema.dadagen");
            let definitions = engine.parse_dsl(dsl).unwrap();
            
            b.iter(|| {
                black_box(engine.execute_generation(&definitions, 1000))
            });
        });
    }
    
    criterion_group!(benches, benchmark_generation_speed);
    criterion_main!(benches);
}
```

### Concurrency and Performance

#### Thread Safety Design

```rust
use std::sync::{Arc, RwLock, Mutex};
use rayon::prelude::*;

pub struct ThreadSafeGenerator {
    inner: Arc<dyn Generator<String> + Send + Sync>,
    context_pool: Arc<Mutex<Vec<Context>>>,
}

impl ThreadSafeGenerator {
    pub fn generate_parallel(&self, count: usize) -> Vec<String> {
        (0..count).into_par_iter()
            .map(|_| {
                let mut context = self.acquire_context();
                let result = self.inner.generate(&mut context);
                self.return_context(context);
                result
            })
            .collect()
    }
}
```

#### Memory Management

```rust
pub struct StreamingGenerator<T> {
    generator: Box<dyn Generator<T>>,
    batch_size: usize,
}

impl<T> StreamingGenerator<T> {
    pub fn generate_stream(&mut self, total_count: usize) -> impl Iterator<Item = T> + '_ {
        (0..total_count)
            .step_by(self.batch_size)
            .flat_map(move |batch_start| {
                let batch_end = (batch_start + self.batch_size).min(total_count);
                let batch_size = batch_end - batch_start;
                
                (0..batch_size).map(move |_| {
                    self.generator.generate(&mut Context::new())
                })
            })
    }
}
```

### Deployment and Distribution

#### Build Configuration

```toml
# Cargo.toml workspace configuration
[workspace]
members = [
    "dadagen-core",
    "dadagen-macros", 
    "dadagen-gui",
    "dadagen-web",
    "dadagen-python",
    "dadagen-cli"
]

[workspace.dependencies]
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

#### CI/CD Pipeline Structure

```yaml
# .github/workflows/release.yml
name: Release Pipeline
on:
  push:
    tags: ['v*']

jobs:
  build-rust:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - name: Build core library
      - name: Run tests
      - name: Build GUI application
      - name: Create platform installer
      
  build-wasm:
    steps:
      - name: Build WASM package
      - name: Optimize bundle size
      - name: Deploy to static hosting
      
  build-python:
    strategy:
      matrix:
        python-version: ['3.8', '3.9', '3.10', '3.11', '3.12']
    steps:
      - name: Build Python wheels
      - name: Test Python integration
      - name: Publish to PyPI
```

### Security Considerations

#### Input Validation

```rust
pub struct DslValidator {
    max_field_count: usize,
    max_nesting_depth: usize,
    allowed_generators: HashSet<String>,
}

impl DslValidator {
    pub fn validate_dsl(&self, dsl: &str) -> Result<(), ValidationError> {
        let parsed = self.parse_with_limits(dsl)?;
        self.check_security_constraints(&parsed)?;
        self.validate_dependencies(&parsed)?;
        Ok(())
    }
    
    fn check_security_constraints(&self, ast: &DslAst) -> Result<(), ValidationError> {
        // Prevent resource exhaustion attacks
        // Validate generator parameters
        // Check for malicious patterns
    }
}
```

#### Sandboxing for Web Application

```rust
#[wasm_bindgen]
pub struct SecureWasmInterface {
    validator: DslValidator,
    resource_limiter: ResourceLimiter,
}

impl SecureWasmInterface {
    pub fn safe_generate(&mut self, dsl: &str, count: usize) -> Result<String, JsValue> {
        // Validate input size and complexity
        self.resource_limiter.check_limits(dsl, count)?;
        
        // Validate DSL for security
        self.validator.validate_dsl(dsl)?;
        
        // Execute with resource monitoring
        self.resource_limiter.with_monitoring(|| {
            self.generate_internal(dsl, count)
        })
    }
}
```

This design document provides a comprehensive architectural foundation for implementing the dadagen Rust library refactor. The modular design ensures maintainability while the shared core library provides consistency across all interfaces.
