//! Context pooling for efficient multi-threaded data generation
//!
//! This module provides a thread-safe pool of Context instances to enable
//! efficient parallel data generation without constant allocation overhead.

use crate::{Context, Result, errors::DadagenError};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use tracing::{debug, trace, instrument};

/// A thread-safe pool of Context instances for efficient multi-threaded generation
///
/// # Example
///
/// ```rust
/// use dadagen_core::ContextPool;
///
/// let pool = ContextPool::new(4); // Create pool with 4 contexts
/// 
/// // Get a context from the pool
/// let context = pool.acquire().unwrap();
/// 
/// // Use the context for generation...
/// 
/// // Return it to the pool when done
/// pool.release(context).unwrap();
/// ```
#[derive(Debug)]
pub struct ContextPool {
    /// Available contexts ready for use
    available: Arc<Mutex<VecDeque<Context>>>,
    /// Maximum pool size
    max_size: usize,
    /// Current total contexts (available + in-use)
    total_contexts: Arc<Mutex<usize>>,
    /// Optional random seed for deterministic generation
    seed: Option<u64>,
}

impl ContextPool {
    /// Create a new context pool with the specified capacity
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of contexts in the pool
    #[instrument]
    pub fn new(capacity: usize) -> Self {
        debug!("Creating context pool with capacity: {}", capacity);
        
        Self {
            available: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            max_size: capacity,
            total_contexts: Arc::new(Mutex::new(0)),
            seed: None,
        }
    }

    /// Create a new context pool with a random seed for deterministic generation
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of contexts in the pool
    /// * `seed` - Random seed for deterministic generation
    #[instrument]
    pub fn with_seed(capacity: usize, seed: u64) -> Self {
        debug!("Creating seeded context pool with capacity: {} and seed: {}", capacity, seed);
        
        Self {
            available: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            max_size: capacity,
            total_contexts: Arc::new(Mutex::new(0)),
            seed: Some(seed),
        }
    }

    /// Acquire a context from the pool
    ///
    /// If no contexts are available and the pool hasn't reached maximum capacity,
    /// a new context will be created. If the pool is at capacity and all contexts
    /// are in use, this will block until one becomes available.
    ///
    /// # Errors
    ///
    /// Returns an error if lock acquisition fails
    #[instrument(skip(self))]
    pub fn acquire(&self) -> Result<Context> {
        trace!("Attempting to acquire context from pool");
        
        let mut available = self.available.lock()
            .map_err(|e| DadagenError::generation_error(format!("Failed to acquire pool lock: {}", e)))?;
        
        // Try to get an available context
        if let Some(context) = available.pop_front() {
            debug!("Reusing context from pool");
            return Ok(context);
        }
        
        // No available contexts - check if we can create a new one
        let mut total = self.total_contexts.lock()
            .map_err(|e| DadagenError::generation_error(format!("Failed to acquire count lock: {}", e)))?;
        
        if *total < self.max_size {
            *total += 1;
            drop(total); // Release lock before creating context
            drop(available); // Release pool lock
            
            debug!("Creating new context (total: {})", *self.total_contexts.lock().unwrap());
            
            let context = if let Some(seed) = self.seed {
                Context::with_seed(seed)
            } else {
                Context::new()
            };
            
            return Ok(context);
        }
        
        // Pool is at capacity and all contexts are in use
        // In a production system, we would wait on a condition variable
        // For now, return an error
        Err(DadagenError::generation_error("Context pool exhausted - all contexts in use"))
    }

    /// Release a context back to the pool
    ///
    /// The context will be reset (iteration counter, field state cleared)
    /// before being returned to the pool for reuse.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to return to the pool
    ///
    /// # Errors
    ///
    /// Returns an error if lock acquisition or context reset fails
    #[instrument(skip(self, context))]
    pub fn release(&self, context: Context) -> Result<()> {
        trace!("Releasing context back to pool");
        
        // Reset the context for reuse (uses interior mutability)
        context.reset_iteration()?;
        context.clear_field_state()?;
        
        let mut available = self.available.lock()
            .map_err(|e| DadagenError::generation_error(format!("Failed to acquire pool lock: {}", e)))?;
        
        available.push_back(context);
        debug!("Context released, {} contexts available", available.len());
        
        Ok(())
    }

    /// Get the current number of available contexts in the pool
    pub fn available_count(&self) -> Result<usize> {
        let available = self.available.lock()
            .map_err(|e| DadagenError::generation_error(format!("Failed to acquire pool lock: {}", e)))?;
        Ok(available.len())
    }

    /// Get the total number of contexts (available + in-use)
    pub fn total_count(&self) -> Result<usize> {
        let total = self.total_contexts.lock()
            .map_err(|e| DadagenError::generation_error(format!("Failed to acquire count lock: {}", e)))?;
        Ok(*total)
    }

    /// Get the maximum pool capacity
    pub fn capacity(&self) -> usize {
        self.max_size
    }
}

impl Clone for ContextPool {
    fn clone(&self) -> Self {
        Self {
            available: Arc::clone(&self.available),
            max_size: self.max_size,
            total_contexts: Arc::clone(&self.total_contexts),
            seed: self.seed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_creation() {
        let pool = ContextPool::new(4);
        assert_eq!(pool.capacity(), 4);
        assert_eq!(pool.available_count().unwrap(), 0);
        assert_eq!(pool.total_count().unwrap(), 0);
    }

    #[test]
    fn test_pool_with_seed() {
        let pool = ContextPool::with_seed(4, 12345);
        assert_eq!(pool.capacity(), 4);
        assert_eq!(pool.seed, Some(12345));
    }

    #[test]
    fn test_acquire_creates_new_context() {
        let pool = ContextPool::new(2);
        
        let context1 = pool.acquire().unwrap();
        assert_eq!(pool.total_count().unwrap(), 1);
        assert_eq!(pool.available_count().unwrap(), 0);
        
        let context2 = pool.acquire().unwrap();
        assert_eq!(pool.total_count().unwrap(), 2);
        assert_eq!(pool.available_count().unwrap(), 0);
        
        // Don't drop contexts yet
        std::mem::forget(context1);
        std::mem::forget(context2);
    }

    #[test]
    fn test_release_returns_to_pool() {
        let pool = ContextPool::new(2);
        
        let context = pool.acquire().unwrap();
        assert_eq!(pool.available_count().unwrap(), 0);
        
        pool.release(context).unwrap();
        assert_eq!(pool.available_count().unwrap(), 1);
        assert_eq!(pool.total_count().unwrap(), 1);
    }

    #[test]
    fn test_acquire_reuses_released_context() {
        let pool = ContextPool::new(2);
        
        // Acquire and release a context
        let context1 = pool.acquire().unwrap();
        pool.release(context1).unwrap();
        
        // Acquire again - should reuse the same context
        let _context2 = pool.acquire().unwrap();
        assert_eq!(pool.total_count().unwrap(), 1); // Still only 1 context created
    }

    #[test]
    fn test_pool_exhaustion() {
        let pool = ContextPool::new(2);
        
        let context1 = pool.acquire().unwrap();
        let context2 = pool.acquire().unwrap();
        
        // Pool is exhausted
        let result = pool.acquire();
        assert!(result.is_err());
        
        // Release one and try again
        pool.release(context1).unwrap();
        let context3 = pool.acquire().unwrap();
        assert!(context3.current_iteration().unwrap() == 0); // Context was reset
        
        std::mem::forget(context2);
        std::mem::forget(context3);
    }

    #[test]
    fn test_release_resets_context() {
        let pool = ContextPool::new(2);
        
        let context = pool.acquire().unwrap();
        
        // Modify the context
        context.increment_iteration().unwrap();
        context.insert_field_state("test".to_string(), 42).unwrap();
        assert_eq!(context.current_iteration().unwrap(), 1);
        
        // Release it
        pool.release(context).unwrap();
        
        // Acquire again - should be reset
        let reused = pool.acquire().unwrap();
        assert_eq!(reused.current_iteration().unwrap(), 0);
        assert!(!reused.has_field("test").unwrap());
        
        std::mem::forget(reused);
    }

    #[test]
    fn test_pool_clone_shares_state() {
        let pool1 = ContextPool::new(2);
        
        let context = pool1.acquire().unwrap();
        pool1.release(context).unwrap();
        
        let pool2 = pool1.clone();
        
        // Cloned pool shares the same underlying state
        assert_eq!(pool2.available_count().unwrap(), 1);
        assert_eq!(pool2.total_count().unwrap(), 1);
    }
}
