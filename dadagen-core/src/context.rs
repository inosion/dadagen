//! Enhanced Context and State Management
//! 
//! This module provides thread-safe context management for data generation
//! with support for field state tracking and dependency resolution.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::any::Any;
use crate::errors::{DadagenError, Result};

/// Trait for type-erased cloneable storage
pub trait AnyClone: Send + Sync {
    fn clone_box(&self) -> Box<dyn AnyClone>;
    fn as_any(&self) -> &dyn Any;
}

impl<T: Clone + Send + Sync + 'static> AnyClone for T {
    fn clone_box(&self) -> Box<dyn AnyClone> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for Box<dyn AnyClone> {
    fn clone(&self) -> Box<dyn AnyClone> {
        self.clone_box()
    }
}

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
    data_field_state: Arc<RwLock<HashMap<String, Box<dyn AnyClone>>>>,
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
        
        state.insert(key, Box::new(value));
        Ok(())
    }

    /// Get a field state value
    pub fn get_field_state<T: Clone + Send + Sync + 'static>(&self, key: &str) -> Result<Option<T>> {
        let state = self.data_field_state.read()
            .map_err(|_| DadagenError::ContextError { 
                message: "Failed to acquire read lock for field state".to_string() 
            })?;
        
        if let Some(boxed_value) = state.get(key) {
            if let Some(value) = boxed_value.as_any().downcast_ref::<T>() {
                Ok(Some(value.clone()))
            } else {
                Err(DadagenError::ContextError {
                    message: format!("Type mismatch for field '{}'", key)
                })
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
        // Test the most basic case first
        let test_value = "hello".to_string();
        let boxed: Box<dyn AnyClone> = Box::new(test_value);
        
        // What type is it?
        println!("Boxed value type: {:?}", boxed.as_any().type_id());
        println!("String type: {:?}", std::any::TypeId::of::<String>());
        
        // Can we downcast?
        if let Some(s) = boxed.as_any().downcast_ref::<String>() {
            println!("Successfully downcast to String: '{}'", s);
        } else {
            println!("Failed to downcast to String");
            
            // Let's see what types it could be
            println!("Trying &str...");
            if let Some(s) = boxed.as_any().downcast_ref::<&str>() {
                println!("It's &str: '{}'", s);
            } else {
                println!("Not &str either");
            }
        }
        
        // Skip the context test for now to focus on the basic issue
        panic!("Debug test - stopping here");
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
