use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Context model containing information about the request context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// Domain of the transaction (e.g., "nic2004:85111")
    pub domain: String,
    
    /// Country where the transaction is taking place
    pub country: String,
    
    /// City where the transaction is taking place
    pub city: String,
    
    /// The action being performed (e.g., "search", "on_search")
    pub action: String,
    
    /// Core version of the UHI protocol
    pub core_version: String,
    
    /// ID of the consumer application
    pub consumer_id: String,
    
    /// URI of the consumer application for callbacks
    pub consumer_uri: String,
    
    /// ID of the provider application (optional for initial requests)
    pub provider_id: Option<String>,
    
    /// URI of the provider application for callbacks (optional for initial requests)
    pub provider_uri: Option<String>,
    
    /// Unique ID for the entire transaction flow
    pub transaction_id: String,
    
    /// Unique ID for this specific message
    pub message_id: String,
    
    /// Timestamp when the message was created
    pub timestamp: DateTime<Utc>,
}

impl Context {
    /// Create a new context with mandatory fields
    pub fn new(
        domain: String,
        country: String,
        city: String,
        action: String,
        core_version: String,
        consumer_id: String,
        consumer_uri: String,
    ) -> Self {
        Self {
            domain,
            country,
            city,
            action,
            core_version,
            consumer_id,
            consumer_uri,
            provider_id: None,
            provider_uri: None,
            transaction_id: Uuid::new_v4().to_string(),
            message_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
        }
    }
    
    /// Create a response context from a request context
    pub fn create_response_context(&self, action: String, provider_id: String, provider_uri: String) -> Self {
        Self {
            domain: self.domain.clone(),
            country: self.country.clone(),
            city: self.city.clone(),
            action,
            core_version: self.core_version.clone(),
            consumer_id: self.consumer_id.clone(),
            consumer_uri: self.consumer_uri.clone(),
            provider_id: Some(provider_id),
            provider_uri: Some(provider_uri),
            transaction_id: self.transaction_id.clone(),
            message_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
        }
    }
}

/// Trait for types that can contain a context
pub trait WithContext {
    fn context(&self) -> &Context;
} 