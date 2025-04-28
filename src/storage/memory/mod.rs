use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::models::{
    provider::Provider,
    catalog::{Catalog, Item, SearchRequest, SearchResponse},
    order::Order,
    fulfillment::Fulfillment,
    network_registry::{Subscriber, NetworkRegistryLookup},
};

use crate::storage::{Storage, StorageResult, StorageError};

#[cfg(test)]
mod tests;

/// In-memory storage implementation for testing and development
pub struct MemoryStorage {
    providers: RwLock<HashMap<String, Provider>>,
    catalogs: RwLock<HashMap<String, Catalog>>,
    orders: RwLock<HashMap<String, Order>>,
    fulfillments: RwLock<HashMap<String, Fulfillment>>,
    subscribers: RwLock<HashMap<String, Subscriber>>,
    transactions: RwLock<HashMap<String, serde_json::Value>>,
}

impl MemoryStorage {
    /// Create a new in-memory storage instance
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            catalogs: RwLock::new(HashMap::new()),
            orders: RwLock::new(HashMap::new()),
            fulfillments: RwLock::new(HashMap::new()),
            subscribers: RwLock::new(HashMap::new()),
            transactions: RwLock::new(HashMap::new()),
        }
    }
    
    /// Create an empty database for testing
    pub fn empty() -> Arc<Self> {
        Arc::new(Self::new())
    }
    
    /// Create a database with mock data for testing
    pub fn with_mock_data() -> Arc<Self> {
        let storage = Self::new();
        // TODO: Add mock data initialization
        Arc::new(storage)
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    // Provider operations
    async fn create_provider(&self, provider: Provider) -> StorageResult<Provider> {
        let mut providers = self.providers.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if providers.contains_key(&provider.id) {
            return Err(StorageError::Duplicate(format!("Provider with ID {} already exists", provider.id)));
        }
        
        let provider_clone = provider.clone();
        providers.insert(provider.id.clone(), provider);
        Ok(provider_clone)
    }
    
    async fn get_provider(&self, id: &str) -> StorageResult<Provider> {
        let providers = self.providers.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        providers.get(id)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(format!("Provider with ID {} not found", id)))
    }
    
    async fn update_provider(&self, provider: Provider) -> StorageResult<Provider> {
        let mut providers = self.providers.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if !providers.contains_key(&provider.id) {
            return Err(StorageError::NotFound(format!("Provider with ID {} not found", provider.id)));
        }
        
        let provider_clone = provider.clone();
        providers.insert(provider.id.clone(), provider);
        Ok(provider_clone)
    }
    
    async fn delete_provider(&self, id: &str) -> StorageResult<()> {
        let mut providers = self.providers.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if !providers.contains_key(id) {
            return Err(StorageError::NotFound(format!("Provider with ID {} not found", id)));
        }
        
        providers.remove(id);
        Ok(())
    }
    
    async fn list_providers(&self) -> StorageResult<Vec<Provider>> {
        let providers = self.providers.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        Ok(providers.values().cloned().collect())
    }
    
    // Catalog operations
    async fn create_catalog(&self, provider_id: &str, catalog: Catalog) -> StorageResult<Catalog> {
        // Verify provider exists
        let providers = self.providers.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if !providers.contains_key(provider_id) {
            return Err(StorageError::NotFound(format!("Provider with ID {} not found", provider_id)));
        }
        
        let mut catalogs = self.catalogs.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if catalogs.contains_key(provider_id) {
            return Err(StorageError::Duplicate(format!("Catalog for provider ID {} already exists", provider_id)));
        }
        
        let catalog_clone = catalog.clone();
        catalogs.insert(provider_id.to_string(), catalog);
        Ok(catalog_clone)
    }
    
    async fn get_catalog(&self, provider_id: &str) -> StorageResult<Catalog> {
        let catalogs = self.catalogs.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        catalogs.get(provider_id)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(format!("Catalog for provider ID {} not found", provider_id)))
    }
    
    async fn update_catalog(&self, provider_id: &str, catalog: Catalog) -> StorageResult<Catalog> {
        let mut catalogs = self.catalogs.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if !catalogs.contains_key(provider_id) {
            return Err(StorageError::NotFound(format!("Catalog for provider ID {} not found", provider_id)));
        }
        
        let catalog_clone = catalog.clone();
        catalogs.insert(provider_id.to_string(), catalog);
        Ok(catalog_clone)
    }
    
    async fn search_catalog(&self, request: SearchRequest) -> StorageResult<SearchResponse> {
        let catalogs = self.catalogs.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        // This is a simplified search implementation for in-memory storage
        // In a real implementation, we would apply filters based on the search request
        
        // For now, just return the first catalog that matches any criteria
        // or an empty response if no catalogs are found
        if catalogs.is_empty() {
            return Err(StorageError::NotFound("No catalogs found".to_string()));
        }
        
        // For simplicity, just return the first catalog found
        // In a real implementation, this would be more sophisticated
        let first_catalog = catalogs.values().next().cloned().unwrap();
        
        Ok(SearchResponse {
            catalog: first_catalog,
        })
    }
    
    // Order operations
    async fn create_order(&self, order: Order) -> StorageResult<Order> {
        let mut orders = self.orders.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if orders.contains_key(&order.id) {
            return Err(StorageError::Duplicate(format!("Order with ID {} already exists", order.id)));
        }
        
        let order_clone = order.clone();
        orders.insert(order.id.clone(), order);
        Ok(order_clone)
    }
    
    async fn get_order(&self, id: &str) -> StorageResult<Order> {
        let orders = self.orders.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        orders.get(id)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(format!("Order with ID {} not found", id)))
    }
    
    async fn update_order(&self, order: Order) -> StorageResult<Order> {
        let mut orders = self.orders.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if !orders.contains_key(&order.id) {
            return Err(StorageError::NotFound(format!("Order with ID {} not found", order.id)));
        }
        
        let order_clone = order.clone();
        orders.insert(order.id.clone(), order);
        Ok(order_clone)
    }
    
    async fn list_orders_by_provider(&self, provider_id: &str) -> StorageResult<Vec<Order>> {
        let orders = self.orders.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        Ok(orders.values()
            .filter(|order| order.provider.id == provider_id)
            .cloned()
            .collect())
    }
    
    async fn list_orders_by_customer(&self, customer_id: &str) -> StorageResult<Vec<Order>> {
        let orders = self.orders.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        // In a real implementation, we would filter by customer ID in the billing info
        // This is a simplified version that assumes customer ID is in the billing name field
        Ok(orders.values()
            .filter(|order| order.billing.name == customer_id)
            .cloned()
            .collect())
    }
    
    // Fulfillment operations
    async fn create_fulfillment(&self, fulfillment: Fulfillment) -> StorageResult<Fulfillment> {
        let mut fulfillments = self.fulfillments.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if fulfillments.contains_key(&fulfillment.id) {
            return Err(StorageError::Duplicate(format!("Fulfillment with ID {} already exists", fulfillment.id)));
        }
        
        let fulfillment_clone = fulfillment.clone();
        fulfillments.insert(fulfillment.id.clone(), fulfillment);
        Ok(fulfillment_clone)
    }
    
    async fn get_fulfillment(&self, id: &str) -> StorageResult<Fulfillment> {
        let fulfillments = self.fulfillments.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        fulfillments.get(id)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(format!("Fulfillment with ID {} not found", id)))
    }
    
    async fn update_fulfillment(&self, fulfillment: Fulfillment) -> StorageResult<Fulfillment> {
        let mut fulfillments = self.fulfillments.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if !fulfillments.contains_key(&fulfillment.id) {
            return Err(StorageError::NotFound(format!("Fulfillment with ID {} not found", fulfillment.id)));
        }
        
        let fulfillment_clone = fulfillment.clone();
        fulfillments.insert(fulfillment.id.clone(), fulfillment);
        Ok(fulfillment_clone)
    }
    
    async fn list_fulfillments_by_provider(&self, provider_id: &str) -> StorageResult<Vec<Fulfillment>> {
        let fulfillments = self.fulfillments.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        Ok(fulfillments.values()
            .filter(|fulfillment| fulfillment.provider_id == provider_id)
            .cloned()
            .collect())
    }
    
    // Network registry operations
    async fn register_subscriber(&self, subscriber: Subscriber) -> StorageResult<Subscriber> {
        let mut subscribers = self.subscribers.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        if subscribers.contains_key(&subscriber.id) {
            return Err(StorageError::Duplicate(format!("Subscriber with ID {} already exists", subscriber.id)));
        }
        
        let subscriber_clone = subscriber.clone();
        subscribers.insert(subscriber.id.clone(), subscriber);
        Ok(subscriber_clone)
    }
    
    async fn get_subscriber(&self, id: &str) -> StorageResult<Subscriber> {
        let subscribers = self.subscribers.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        subscribers.get(id)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(format!("Subscriber with ID {} not found", id)))
    }
    
    async fn lookup_subscriber(&self, lookup: NetworkRegistryLookup) -> StorageResult<Subscriber> {
        let subscribers = self.subscribers.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        // Simplified lookup that just checks the subscriber type and domain
        for subscriber in subscribers.values() {
            if subscriber.type_field == lookup.type_field && 
               subscriber.domain == lookup.domain {
                return Ok(subscriber.clone());
            }
        }
        
        Err(StorageError::NotFound(format!("No matching subscriber found for {:?}", lookup)))
    }
    
    async fn list_subscribers(&self) -> StorageResult<Vec<Subscriber>> {
        let subscribers = self.subscribers.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        Ok(subscribers.values().cloned().collect())
    }
    
    // Transaction tracking
    async fn record_transaction(&self, transaction_id: &str, data: serde_json::Value) -> StorageResult<()> {
        let mut transactions = self.transactions.write().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        transactions.insert(transaction_id.to_string(), data);
        Ok(())
    }
    
    async fn get_transaction(&self, transaction_id: &str) -> StorageResult<serde_json::Value> {
        let transactions = self.transactions.read().map_err(|e| 
            StorageError::Internal(format!("Lock error: {}", e)))?;
            
        transactions.get(transaction_id)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(format!("Transaction with ID {} not found", transaction_id)))
    }
} 