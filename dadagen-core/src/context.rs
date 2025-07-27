//! Enhanced Context and State Management
//! 
//! This module provides thread-safe context management for data generation
//! with support for field state tracking and dependency resolution.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::any::Any;
use crate::errors::{DadagenError, Result};

/// Metadata about the generation process
#[derive(Debug, Clone)]
pub struct GenerationMetadata {
    pub start_time: std::time::SystemTime,
    pub total_fields: usize,
    pub generated_records: usize,
}

impl Default for GenerationMetadata {
    fn default() -> Self {
        Self {
            start_time: std::time::SystemTime::now(),
            total_fields: 0,
            generated_records: 0,
        }
    }
}

/// Thread-safe context for data generation
pub struct Context {
    current_iter: Arc<RwLock<u64>>,
    data_field_state: Arc<RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
    random_seed: Option<u64>,
    generation_metadata: Arc<RwLock<GenerationMetadata>>,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("current_iter", &"<Arc<RwLock<u64>>>")
            .field("data_field_state", &"<Arc<RwLock<HashMap<String, Box<dyn AnyClone>>>>>")
            .field("random_seed", &self.random_seed)
            .field("generation_metadata", &"<Arc<RwLock<GenerationMetadata>>>")
            .finish()
    }
}

impl Context {
    /// Create a new context
    pub fn new() -> Self {
        Self {
            current_iter: Arc::new(RwLock::new(0)),
            data_field_state: Arc::new(RwLock::new(HashMap::new())),
            random_seed: None,
            generation_metadata: Arc::new(RwLock::new(GenerationMetadata::default())),
        }
    }

    /// Create a new context with a specific random seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            current_iter: Arc::new(RwLock::new(0)),
            data_field_state: Arc::new(RwLock::new(HashMap::new())),
            random_seed: Some(seed),
            generation_metadata: Arc::new(RwLock::new(GenerationMetadata::default())),
        }
    }

    /// Insert a field state value
    pub fn insert_field_state<T: Clone + Send + Sync + 'static>(&self, key: String, value: T) -> Result<()> {
        let mut state = self.data_field_state.write()
            .map_err(|_| DadagenError::ContextError { 
                message: "Failed to acquire write lock for field state".to_string() 
            })?;
        
        state.insert(key, Arc::new(value));
        Ok(())
    }

    /// Get a field state value
    pub fn get_field_state<T: Clone + Send + Sync + 'static>(&self, key: &str) -> Result<Option<T>> {
        let state = self.data_field_state.read()
            .map_err(|_| DadagenError::ContextError { 
                message: "Failed to acquire read lock for field state".to_string() 
            })?;
        
        if let Some(arc_value) = state.get(key) {
            // Try to downcast to the requested type
            if let Some(value) = arc_value.downcast_ref::<T>() {
                Ok(Some(value.clone()))
            } else {
                // Return None for type mismatch instead of error
                // This is more ergonomic for optional field access
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Increment the iteration counter
    pub fn increment_iteration(&self) -> Result<u64> {
        let mut iter = self.current_iter.write()
            .map_err(|_| DadagenError::ContextError { 
                message: "Failed to acquire write lock for iteration counter".to_string() 
            })?;
        
        *iter += 1;
        Ok(*iter)
    }

    /// Get the current iteration number
    pub fn current_iteration(&self) -> Result<u64> {
        let iter = self.current_iter.read()
            .map_err(|_| DadagenError::ContextError { 
                message: "Failed to acquire read lock for iteration counter".to_string() 
            })?;
        
        Ok(*iter)
    }

    /// Get the random seed if set
    pub fn random_seed(&self) -> Option<u64> {
        self.random_seed
    }

    /// Update generation metadata
    pub fn update_metadata<F>(&self, updater: F) -> Result<()> 
    where 
        F: FnOnce(&mut GenerationMetadata)
    {
        let mut metadata = self.generation_metadata.write()
            .map_err(|_| DadagenError::ContextError { 
                message: "Failed to acquire write lock for metadata".to_string() 
            })?;
        
        updater(&mut *metadata);
        Ok(())
    }

    /// Get a copy of the current metadata
    pub fn get_metadata(&self) -> Result<GenerationMetadata> {
        let metadata = self.generation_metadata.read()
            .map_err(|_| DadagenError::ContextError { 
                message: "Failed to acquire read lock for metadata".to_string() 
            })?;
        
        Ok(metadata.clone())
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            current_iter: Arc::clone(&self.current_iter),
            data_field_state: Arc::clone(&self.data_field_state),
            random_seed: self.random_seed,
            generation_metadata: Arc::clone(&self.generation_metadata),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let context = Context::new();
        assert_eq!(context.current_iteration().unwrap(), 0);
        assert!(context.random_seed().is_none());
    }

    #[test]
    fn test_context_with_seed() {
        let context = Context::with_seed(12345);
        assert_eq!(context.random_seed(), Some(12345));
    }

    #[test]
    fn test_field_state_management() {
        let context = Context::new();
        
        // Test storing and retrieving a string value
        let field_name = "test_field";
        let test_value = "hello world".to_string();
        
        // Store the value
        context.insert_field_state(field_name.to_string(), test_value.clone()).unwrap();
        
        // Retrieve and verify
        if let Some(retrieved) = context.get_field_state::<String>(field_name).unwrap() {
            assert_eq!(retrieved, test_value);
        } else {
            panic!("Failed to retrieve stored string value");
        }
        
        // Test storing and retrieving an integer value
        let int_field = "int_field";
        let int_value = 42i64;
        
        context.insert_field_state(int_field.to_string(), int_value).unwrap();
        
        if let Some(retrieved_int) = context.get_field_state::<i64>(int_field).unwrap() {
            assert_eq!(retrieved_int, int_value);
        } else {
            panic!("Failed to retrieve stored integer value");
        }
        
        // Test that wrong type returns None (graceful type mismatch handling)
        assert!(context.get_field_state::<f64>(field_name).unwrap().is_none());
        assert!(context.get_field_state::<String>(int_field).unwrap().is_none());
        
        // Test non-existent field
        assert!(context.get_field_state::<String>("non_existent").unwrap().is_none());
        
        // Test storing and retrieving a boolean value
        let bool_field = "bool_field";
        let bool_value = true;
        
        context.insert_field_state(bool_field.to_string(), bool_value).unwrap();
        
        if let Some(retrieved_bool) = context.get_field_state::<bool>(bool_field).unwrap() {
            assert_eq!(retrieved_bool, bool_value);
        } else {
            panic!("Failed to retrieve stored boolean value");
        }
    }

    #[test]
    fn test_iteration_counter() {
        let context = Context::new();
        
        assert_eq!(context.current_iteration().unwrap(), 0);
        assert_eq!(context.increment_iteration().unwrap(), 1);
        assert_eq!(context.increment_iteration().unwrap(), 2);
        assert_eq!(context.current_iteration().unwrap(), 2);
    }

    #[test]
    fn test_metadata_management() {
        let context = Context::new();
        
        context.update_metadata(|meta| {
            meta.total_fields = 5;
            meta.generated_records = 100;
        }).unwrap();
        
        let metadata = context.get_metadata().unwrap();
        assert_eq!(metadata.total_fields, 5);
        assert_eq!(metadata.generated_records, 100);
    }
}
