//! Enhanced Context and State Management
//!
//! This module provides thread-safe context management for data generation
//! with support for field state tracking and dependency resolution.
//!
//! # Thread Safety
//!
//! The `Context` type is designed for concurrent access using `Arc<RwLock<T>>`.
//! Multiple threads can safely read and write field state without data races.
//!
//! # Type-Erased Storage
//!
//! The `AnyClone` trait enables storing different types in the same HashMap
//! while maintaining the ability to clone and downcast values.

use crate::errors::{DadagenError, Result};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use tracing::{debug, instrument, trace};

/// Trait for type-erased cloneable values
///
/// This trait allows storing heterogeneous types in a collection while
/// preserving the ability to clone them. It combines `Any` for downcasting
/// with a custom cloning mechanism.
pub trait AnyClone: Any + Send + Sync {
    /// Clone this value into a new Box
    fn clone_box(&self) -> Box<dyn AnyClone>;

    /// Get a reference to this value as `Any`
    fn as_any(&self) -> &dyn Any;

    /// Get a debug representation of this value
    fn debug_value(&self) -> String;
}

impl<T> AnyClone for T
where
    T: Clone + Any + Send + Sync + fmt::Debug,
{
    fn clone_box(&self) -> Box<dyn AnyClone> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn debug_value(&self) -> String {
        format!("{:?}", self)
    }
}

impl Clone for Box<dyn AnyClone> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Metadata about the generation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationMetadata {
    /// When generation started (skipped in serialization as SystemTime doesn't have Default)
    #[serde(skip, default = "std::time::SystemTime::now")]
    pub start_time: std::time::SystemTime,

    /// Total number of fields defined
    pub total_fields: usize,

    /// Number of records generated so far
    pub generated_records: usize,

    /// Field access frequency (for optimization)
    #[serde(default)]
    pub field_access_count: HashMap<String, usize>,

    /// Average generation time per record (in microseconds)
    #[serde(default)]
    pub avg_generation_time_us: Option<u64>,
}

impl Default for GenerationMetadata {
    fn default() -> Self {
        Self {
            start_time: std::time::SystemTime::now(),
            total_fields: 0,
            generated_records: 0,
            field_access_count: HashMap::new(),
            avg_generation_time_us: None,
        }
    }
}

/// Thread-safe context for data generation
///
/// The context maintains all state during data generation including:
/// - Current iteration number
/// - Field values from previous generations
/// - Metadata about the generation process
/// - Optional random seed for reproducibility
///
/// # Thread Safety
///
/// All internal state is protected by `RwLock` for concurrent access.
/// Multiple readers can access state simultaneously, while writers get
/// exclusive access.
///
/// # Examples
///
/// ```rust
/// use dadagen_core::Context;
///
/// let ctx = Context::new();
/// ctx.insert_field_state("name".to_string(), "Alice".to_string()).unwrap();
/// let name: Option<String> = ctx.get_field_state("name").unwrap();
/// assert_eq!(name, Some("Alice".to_string()));
/// ```
pub struct Context {
    current_iter: Arc<RwLock<u64>>,
    data_field_state: Arc<RwLock<HashMap<String, Box<dyn AnyClone>>>>,
    random_seed: Option<u64>,
    generation_metadata: Arc<RwLock<GenerationMetadata>>,
    /// Field dependency tracking
    field_dependencies: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.current_iter.read().ok().map(|i| *i);
        let field_count = self.data_field_state.read().ok().map(|s| s.len());
        let metadata = self
            .generation_metadata
            .read()
            .ok()
            .and_then(|m| Some(m.clone()));

        f.debug_struct("Context")
            .field("current_iter", &iter)
            .field("field_count", &field_count)
            .field("random_seed", &self.random_seed)
            .field("metadata", &metadata)
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
            field_dependencies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new context with a specific random seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            current_iter: Arc::new(RwLock::new(0)),
            data_field_state: Arc::new(RwLock::new(HashMap::new())),
            random_seed: Some(seed),
            generation_metadata: Arc::new(RwLock::new(GenerationMetadata::default())),
            field_dependencies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Insert a field state value with type-erased storage
    ///
    /// # Type Safety
    ///
    /// Values must implement `Clone + Send + Sync + Debug + 'static` to be stored.
    ///
    /// # Instrumentation
    ///
    /// This method tracks field access for optimization purposes.
    #[instrument(skip(self, value), fields(key = %key))]
    pub fn insert_field_state<T>(&self, key: String, value: T) -> Result<()>
    where
        T: Clone + Send + Sync + fmt::Debug + 'static,
    {
        trace!("Inserting field state for key: {}", key);

        let mut state = self.data_field_state.write().map_err(|e| {
            DadagenError::context_error(format!(
                "Failed to acquire write lock for field state: {}",
                e
            ))
        })?;

        state.insert(key.clone(), Box::new(value));

        // Track field access
        self.update_metadata(|meta| {
            *meta.field_access_count.entry(key).or_insert(0) += 1;
        })?;

        debug!("Field state inserted successfully");
        Ok(())
    }

    /// Get a field state value with proper type casting
    ///
    /// # Returns
    ///
    /// - `Ok(Some(value))` if the field exists and type matches
    /// - `Ok(None)` if the field doesn't exist or type doesn't match
    /// - `Err(_)` if there's a lock acquisition error
    #[instrument(skip(self), fields(key = %key))]
    pub fn get_field_state<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: Clone + Sync + Send + 'static,
    {
        trace!("Getting field state for key: {}", key);

        let result = {
            let state = self.data_field_state.read().map_err(|e| {
                DadagenError::context_error(format!(
                    "Failed to acquire read lock for field state: {}",
                    e
                ))
            })?;

            if let Some(boxed_value) = state.get(key) {
                // Downcast to the requested type
                if let Some(value) = boxed_value.as_any().downcast_ref::<T>() {
                    debug!("Field state retrieved successfully");
                    Some(value.clone())
                } else {
                    debug!("Type mismatch for field: {}", key);
                    None
                }
            } else {
                trace!("Field not found: {}", key);
                None
            }
        }; // state lock is dropped here

        // Track field access after releasing the read lock
        if result.is_some() {
            self.update_metadata(|meta| {
                *meta.field_access_count.entry(key.to_string()).or_insert(0) += 1;
            })?;
        }

        Ok(result)
    }

    /// Get a field value or return an error if not found
    ///
    /// This is a convenience method that returns an error instead of `None`
    /// for required fields.
    pub fn get_field_required<T>(&self, key: &str) -> Result<T>
    where
        T: Clone + Send + Sync + 'static,
    {
        self.get_field_state(key)?
            .ok_or_else(|| DadagenError::field_not_found(key))
    }

    /// Check if a field exists in the context
    pub fn has_field(&self, key: &str) -> Result<bool> {
        let state = self.data_field_state.read().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(state.contains_key(key))
    }

    /// Get all field names currently stored
    pub fn field_names(&self) -> Result<Vec<String>> {
        let state = self.data_field_state.read().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(state.keys().cloned().collect())
    }

    /// Clear all field state
    pub fn clear_field_state(&self) -> Result<()> {
        let mut state = self.data_field_state.write().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire write lock: {}", e))
        })?;
        state.clear();
        debug!("Field state cleared");
        Ok(())
    }

    /// Increment the iteration counter and return the new value
    ///
    /// This is used to track which record is being generated in batch operations.
    #[instrument(skip(self))]
    pub fn increment_iteration(&self) -> Result<u64> {
        let mut iter = self.current_iter.write().map_err(|e| {
            DadagenError::context_error(format!(
                "Failed to acquire write lock for iteration counter: {}",
                e
            ))
        })?;

        *iter += 1;
        trace!("Iteration incremented to: {}", *iter);
        Ok(*iter)
    }

    /// Get the current iteration number
    pub fn current_iteration(&self) -> Result<u64> {
        let iter = self.current_iter.read().map_err(|e| {
            DadagenError::context_error(format!(
                "Failed to acquire read lock for iteration counter: {}",
                e
            ))
        })?;

        Ok(*iter)
    }

    /// Reset the iteration counter to zero
    pub fn reset_iteration(&self) -> Result<()> {
        let mut iter = self.current_iter.write().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire write lock: {}", e))
        })?;
        *iter = 0;
        debug!("Iteration counter reset");
        Ok(())
    }

    /// Get the random seed if set
    pub fn random_seed(&self) -> Option<u64> {
        self.random_seed
    }

    /// Register field dependencies for resolution ordering
    ///
    /// This allows the generator to determine the correct order to generate
    /// fields when they depend on each other.
    pub fn register_field_dependency(&self, field: String, depends_on: Vec<String>) -> Result<()> {
        let mut deps = self.field_dependencies.write().map_err(|e| {
            DadagenError::context_error(format!(
                "Failed to acquire write lock for dependencies: {}",
                e
            ))
        })?;

        deps.insert(field, depends_on);
        Ok(())
    }

    /// Get the dependencies for a specific field
    pub fn get_field_dependencies(&self, field: &str) -> Result<Vec<String>> {
        let deps = self.field_dependencies.read().map_err(|e| {
            DadagenError::context_error(format!(
                "Failed to acquire read lock for dependencies: {}",
                e
            ))
        })?;

        Ok(deps.get(field).cloned().unwrap_or_default())
    }

    /// Get all field dependencies as a map
    pub fn all_dependencies(&self) -> Result<HashMap<String, Vec<String>>> {
        let deps = self.field_dependencies.read().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(deps.clone())
    }

    /// Update generation metadata with a closure
    ///
    /// This provides safe concurrent access to metadata updates.
    pub fn update_metadata<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut GenerationMetadata),
    {
        let mut metadata = self.generation_metadata.write().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire write lock for metadata: {}", e))
        })?;

        updater(&mut *metadata);
        Ok(())
    }

    /// Get a copy of the current metadata
    pub fn get_metadata(&self) -> Result<GenerationMetadata> {
        let metadata = self.generation_metadata.read().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire read lock for metadata: {}", e))
        })?;

        Ok(metadata.clone())
    }

    /// Create a snapshot of the current context for debugging
    ///
    /// Returns a serializable snapshot that can be saved for analysis.
    pub fn create_snapshot(&self) -> Result<ContextSnapshot> {
        let iter = self.current_iteration()?;
        let metadata = self.get_metadata()?;
        let field_names = self.field_names()?;
        let dependencies = self.all_dependencies()?;

        // Collect field values with debug representations
        let state = self.data_field_state.read().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire read lock: {}", e))
        })?;

        let mut field_values = HashMap::new();
        for (key, value) in state.iter() {
            field_values.insert(key.clone(), value.debug_value());
        }

        Ok(ContextSnapshot {
            current_iter: iter,
            random_seed: self.random_seed,
            metadata,
            field_names,
            field_values,
            dependencies,
        })
    }

    /// Deep clone the context creating independent copies of all state
    ///
    /// This is useful for parallel generation where each thread needs
    /// its own context.
    pub fn deep_clone(&self) -> Result<Self> {
        let iter = self.current_iteration()?;
        let metadata = self.get_metadata()?;
        let deps = self.all_dependencies()?;

        let state = self.data_field_state.read().map_err(|e| {
            DadagenError::context_error(format!("Failed to acquire read lock: {}", e))
        })?;

        let mut new_state = HashMap::new();
        for (key, value) in state.iter() {
            new_state.insert(key.clone(), value.clone());
        }

        Ok(Self {
            current_iter: Arc::new(RwLock::new(iter)),
            data_field_state: Arc::new(RwLock::new(new_state)),
            random_seed: self.random_seed,
            generation_metadata: Arc::new(RwLock::new(metadata)),
            field_dependencies: Arc::new(RwLock::new(deps)),
        })
    }
}

/// A serializable snapshot of context state for debugging
///
/// This structure can be serialized to JSON or other formats for
/// analysis, debugging, or state persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSnapshot {
    /// Current iteration number
    pub current_iter: u64,

    /// Random seed if set
    pub random_seed: Option<u64>,

    /// Generation metadata
    pub metadata: GenerationMetadata,

    /// List of all field names
    pub field_names: Vec<String>,

    /// Field values as debug strings (not type-safe but serializable)
    pub field_values: HashMap<String, String>,

    /// Field dependencies
    pub dependencies: HashMap<String, Vec<String>>,
}

impl Clone for Context {
    /// Shallow clone that shares the same underlying state
    ///
    /// This is efficient for passing context around within the same
    /// generation process. For independent contexts, use `deep_clone()`.
    fn clone(&self) -> Self {
        Self {
            current_iter: Arc::clone(&self.current_iter),
            data_field_state: Arc::clone(&self.data_field_state),
            random_seed: self.random_seed,
            generation_metadata: Arc::clone(&self.generation_metadata),
            field_dependencies: Arc::clone(&self.field_dependencies),
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
        context
            .insert_field_state(field_name.to_string(), test_value.clone())
            .unwrap();

        // Retrieve and verify
        if let Some(retrieved) = context.get_field_state::<String>(field_name).unwrap() {
            assert_eq!(retrieved, test_value);
        } else {
            panic!("Failed to retrieve stored string value");
        }

        // Test storing and retrieving an integer value
        let int_field = "int_field";
        let int_value = 42i64;

        context
            .insert_field_state(int_field.to_string(), int_value)
            .unwrap();

        if let Some(retrieved_int) = context.get_field_state::<i64>(int_field).unwrap() {
            assert_eq!(retrieved_int, int_value);
        } else {
            panic!("Failed to retrieve stored integer value");
        }

        // Test that wrong type returns None (graceful type mismatch handling)
        assert!(
            context
                .get_field_state::<f64>(field_name)
                .unwrap()
                .is_none()
        );
        assert!(
            context
                .get_field_state::<String>(int_field)
                .unwrap()
                .is_none()
        );

        // Test non-existent field
        assert!(
            context
                .get_field_state::<String>("non_existent")
                .unwrap()
                .is_none()
        );

        // Test storing and retrieving a boolean value
        let bool_field = "bool_field";
        let bool_value = true;

        context
            .insert_field_state(bool_field.to_string(), bool_value)
            .unwrap();

        if let Some(retrieved_bool) = context.get_field_state::<bool>(bool_field).unwrap() {
            assert_eq!(retrieved_bool, bool_value);
        } else {
            panic!("Failed to retrieve stored boolean value");
        }
    }

    #[test]
    fn test_field_required() {
        let context = Context::new();

        // Test that get_field_required returns error for missing field
        let result = context.get_field_required::<String>("missing");
        assert!(result.is_err());

        // Test that it returns value for existing field
        context
            .insert_field_state("exists".to_string(), "value".to_string())
            .unwrap();
        let value: String = context.get_field_required("exists").unwrap();
        assert_eq!(value, "value");
    }

    #[test]
    fn test_has_field() {
        let context = Context::new();

        assert!(!context.has_field("test").unwrap());

        context.insert_field_state("test".to_string(), 42).unwrap();
        assert!(context.has_field("test").unwrap());
    }

    #[test]
    fn test_field_names() {
        let context = Context::new();

        context.insert_field_state("field1".to_string(), 1).unwrap();
        context.insert_field_state("field2".to_string(), 2).unwrap();
        context.insert_field_state("field3".to_string(), 3).unwrap();

        let names = context.field_names().unwrap();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"field1".to_string()));
        assert!(names.contains(&"field2".to_string()));
        assert!(names.contains(&"field3".to_string()));
    }

    #[test]
    fn test_clear_field_state() {
        let context = Context::new();

        context.insert_field_state("test".to_string(), 42).unwrap();
        assert!(context.has_field("test").unwrap());

        context.clear_field_state().unwrap();
        assert!(!context.has_field("test").unwrap());
    }

    #[test]
    fn test_iteration_counter() {
        let context = Context::new();

        assert_eq!(context.current_iteration().unwrap(), 0);
        assert_eq!(context.increment_iteration().unwrap(), 1);
        assert_eq!(context.increment_iteration().unwrap(), 2);
        assert_eq!(context.current_iteration().unwrap(), 2);

        context.reset_iteration().unwrap();
        assert_eq!(context.current_iteration().unwrap(), 0);
    }

    #[test]
    fn test_metadata_management() {
        let context = Context::new();

        context
            .update_metadata(|meta| {
                meta.total_fields = 5;
                meta.generated_records = 100;
            })
            .unwrap();

        let metadata = context.get_metadata().unwrap();
        assert_eq!(metadata.total_fields, 5);
        assert_eq!(metadata.generated_records, 100);
    }

    #[test]
    fn test_field_access_tracking() {
        let context = Context::new();

        context
            .insert_field_state("test".to_string(), "value".to_string())
            .unwrap();

        // Access the field multiple times
        for _ in 0..3 {
            let _: Option<String> = context.get_field_state("test").unwrap();
        }

        let metadata = context.get_metadata().unwrap();
        // 1 insert + 3 gets = 4 accesses
        assert_eq!(metadata.field_access_count.get("test"), Some(&4));
    }

    #[test]
    fn test_field_dependencies() {
        let context = Context::new();

        context
            .register_field_dependency(
                "full_name".to_string(),
                vec!["first_name".to_string(), "last_name".to_string()],
            )
            .unwrap();

        let deps = context.get_field_dependencies("full_name").unwrap();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&"first_name".to_string()));
        assert!(deps.contains(&"last_name".to_string()));

        let all_deps = context.all_dependencies().unwrap();
        assert_eq!(all_deps.len(), 1);
    }

    #[test]
    fn test_context_snapshot() {
        let context = Context::new();

        context
            .insert_field_state("name".to_string(), "Alice".to_string())
            .unwrap();
        context
            .insert_field_state("age".to_string(), 30i32)
            .unwrap();
        context
            .register_field_dependency("email".to_string(), vec!["name".to_string()])
            .unwrap();
        context.increment_iteration().unwrap();

        let snapshot = context.create_snapshot().unwrap();

        assert_eq!(snapshot.current_iter, 1);
        assert_eq!(snapshot.field_names.len(), 2);
        assert_eq!(snapshot.field_values.len(), 2);
        assert_eq!(snapshot.dependencies.len(), 1);
    }

    #[test]
    fn test_deep_clone() {
        let context = Context::new();

        context.insert_field_state("test".to_string(), 42).unwrap();
        context.increment_iteration().unwrap();

        let cloned = context.deep_clone().unwrap();

        // Verify cloned context has same state
        assert_eq!(cloned.current_iteration().unwrap(), 1);
        assert_eq!(cloned.get_field_state::<i32>("test").unwrap(), Some(42));

        // Verify they are independent
        context
            .insert_field_state("new_field".to_string(), "value".to_string())
            .unwrap();
        assert!(
            cloned
                .get_field_state::<String>("new_field")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn test_shallow_clone() {
        let context = Context::new();

        context.insert_field_state("test".to_string(), 42).unwrap();

        // Shallow clone shares state
        let cloned = context.clone();

        // Modifications in one are visible in the other
        context
            .insert_field_state("shared".to_string(), "value".to_string())
            .unwrap();
        assert_eq!(
            cloned.get_field_state::<String>("shared").unwrap(),
            Some("value".to_string())
        );
    }

    #[test]
    fn test_anyclone_trait() {
        // Test that the AnyClone trait works with various types
        let string_val: Box<dyn AnyClone> = Box::new("test".to_string());
        let int_val: Box<dyn AnyClone> = Box::new(42i64);
        let bool_val: Box<dyn AnyClone> = Box::new(true);

        // Test cloning
        let cloned_string = string_val.clone();
        assert_eq!(
            cloned_string.as_any().downcast_ref::<String>().unwrap(),
            "test"
        );

        let cloned_int = int_val.clone();
        assert_eq!(*cloned_int.as_any().downcast_ref::<i64>().unwrap(), 42);

        let cloned_bool = bool_val.clone();
        assert_eq!(*cloned_bool.as_any().downcast_ref::<bool>().unwrap(), true);

        // Test debug_value
        assert!(string_val.debug_value().contains("test"));
        assert!(int_val.debug_value().contains("42"));
    }
}
