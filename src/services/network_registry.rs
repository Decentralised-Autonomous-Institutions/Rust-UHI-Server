use std::sync::Arc;
use crate::storage::Storage;
use crate::models::network_registry::{Subscriber, NetworkRegistryLookup};
use super::error::ServiceError;

/// Network registry service for managing network participants
pub struct NetworkRegistryService {
    /// Storage implementation injected via constructor
    storage: Arc<dyn Storage>,
}

impl NetworkRegistryService {
    /// Create a new network registry service with storage dependency
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
    
    /// Register a new subscriber
    pub async fn register_subscriber(&self, subscriber: Subscriber) -> Result<Subscriber, ServiceError> {
        // Validate subscriber data
        self.validate_subscriber(&subscriber)?;
        
        // Register in storage
        let registered = self.storage.register_subscriber(subscriber).await?;
        Ok(registered)
    }
    
    /// Get a subscriber by ID
    pub async fn get_subscriber(&self, id: &str) -> Result<Subscriber, ServiceError> {
        let subscriber = self.storage.get_subscriber(id).await?;
        Ok(subscriber)
    }
    
    /// Lookup a subscriber based on criteria
    pub async fn lookup_subscriber(&self, lookup: NetworkRegistryLookup) -> Result<Subscriber, ServiceError> {
        let subscriber = self.storage.lookup_subscriber(lookup).await?;
        Ok(subscriber)
    }
    
    /// List all subscribers
    pub async fn list_subscribers(&self) -> Result<Vec<Subscriber>, ServiceError> {
        let subscribers = self.storage.list_subscribers().await?;
        Ok(subscribers)
    }
    
    /// Validate subscriber data
    fn validate_subscriber(&self, subscriber: &Subscriber) -> Result<(), ServiceError> {
        // Check that required fields are present
        if subscriber.id.is_empty() {
            return Err(ServiceError::Validation("Subscriber ID is required".to_string()));
        }
        
        if subscriber.type_field.is_empty() {
            return Err(ServiceError::Validation("Subscriber type is required".to_string()));
        }
        
        if subscriber.domain.is_empty() {
            return Err(ServiceError::Validation("Subscriber domain is required".to_string()));
        }
        
        if subscriber.url.is_empty() {
            return Err(ServiceError::Validation("Subscriber URL is required".to_string()));
        }
        
        if subscriber.public_key.is_empty() {
            return Err(ServiceError::Validation("Subscriber public key is required".to_string()));
        }
        
        // All validations passed
        Ok(())
    }
} 