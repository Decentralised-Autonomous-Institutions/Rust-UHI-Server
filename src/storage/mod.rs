pub mod memory;

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{
    provider::Provider,
    catalog::{Item, Catalog, SearchRequest, SearchResponse},
    order::Order,
    fulfillment::Fulfillment,
    network_registry::{Subscriber, NetworkRegistryLookup},
};

/// Error type for storage operations
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Item not found: {0}")]
    NotFound(String),
    
    #[error("Duplicate item: {0}")]
    Duplicate(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Storage error: {0}")]
    Internal(String),
}

/// Result type for storage operations
pub type StorageResult<T> = Result<T, StorageError>;

/// Storage interface for persistence operations
#[async_trait]
pub trait Storage: Send + Sync + 'static {
    // Provider operations
    async fn create_provider(&self, provider: Provider) -> StorageResult<Provider>;
    async fn get_provider(&self, id: &str) -> StorageResult<Provider>;
    async fn update_provider(&self, provider: Provider) -> StorageResult<Provider>;
    async fn delete_provider(&self, id: &str) -> StorageResult<()>;
    async fn list_providers(&self) -> StorageResult<Vec<Provider>>;
    
    // Catalog operations
    async fn create_catalog(&self, provider_id: &str, catalog: Catalog) -> StorageResult<Catalog>;
    async fn get_catalog(&self, provider_id: &str) -> StorageResult<Catalog>;
    async fn update_catalog(&self, provider_id: &str, catalog: Catalog) -> StorageResult<Catalog>;
    async fn search_catalog(&self, request: SearchRequest) -> StorageResult<SearchResponse>;
    
    // Order operations
    async fn create_order(&self, order: Order) -> StorageResult<Order>;
    async fn get_order(&self, id: &str) -> StorageResult<Order>;
    async fn update_order(&self, order: Order) -> StorageResult<Order>;
    async fn list_orders_by_provider(&self, provider_id: &str) -> StorageResult<Vec<Order>>;
    async fn list_orders_by_customer(&self, customer_id: &str) -> StorageResult<Vec<Order>>;
    
    // Fulfillment operations
    async fn create_fulfillment(&self, fulfillment: Fulfillment) -> StorageResult<Fulfillment>;
    async fn get_fulfillment(&self, id: &str) -> StorageResult<Fulfillment>;
    async fn update_fulfillment(&self, fulfillment: Fulfillment) -> StorageResult<Fulfillment>;
    async fn list_fulfillments_by_provider(&self, provider_id: &str) -> StorageResult<Vec<Fulfillment>>;
    
    // Network registry operations
    async fn register_subscriber(&self, subscriber: Subscriber) -> StorageResult<Subscriber>;
    async fn get_subscriber(&self, id: &str) -> StorageResult<Subscriber>;
    async fn lookup_subscriber(&self, lookup: NetworkRegistryLookup) -> StorageResult<Subscriber>;
    async fn list_subscribers(&self) -> StorageResult<Vec<Subscriber>>;
    
    // Transaction tracking
    async fn record_transaction(&self, transaction_id: &str, data: serde_json::Value) -> StorageResult<()>;
    async fn get_transaction(&self, transaction_id: &str) -> StorageResult<serde_json::Value>;
} 