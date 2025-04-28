use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Network participant types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParticipantType {
    /// End User Application
    #[serde(rename = "EUA")]
    Eua,
    
    /// Health Service Provider
    #[serde(rename = "HSP")]
    Hsp,
    
    /// Gateway
    #[serde(rename = "GATEWAY")]
    Gateway,
}

/// Network participant status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParticipantStatus {
    /// Participant is active
    #[serde(rename = "ACTIVE")]
    Active,
    
    /// Participant is inactive
    #[serde(rename = "INACTIVE")]
    Inactive,
    
    /// Participant is suspended
    #[serde(rename = "SUSPENDED")]
    Suspended,
}

/// Subscriber in the network registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    /// Unique ID for the subscriber
    pub id: String,
    
    /// Type of subscriber (EUA, HSP, GATEWAY)
    pub type_field: String,
    
    /// Domain of operation
    pub domain: String,
    
    /// City of operation
    pub city: Option<String>,
    
    /// Country of operation
    pub country: Option<String>,
    
    /// Base URL for the subscriber
    pub url: String,
    
    /// Status of the subscriber
    pub status: String,
    
    /// Public key for signature verification
    pub public_key: String,
    
    /// Time when the subscriber was created
    pub created_at: DateTime<Utc>,
    
    /// Time when the subscriber was last updated
    pub updated_at: DateTime<Utc>,
}

/// Network registry lookup criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRegistryLookup {
    /// Type of subscriber to look up
    pub type_field: String,
    
    /// Domain to look up
    pub domain: String,
    
    /// City to filter by (optional)
    pub city: Option<String>,
    
    /// Country to filter by (optional)
    pub country: Option<String>,
}

/// Network participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// Unique subscriber ID
    pub subscriber_id: String,
    
    /// Type of participant
    pub participant_type: String,
    
    /// Domains supported by the participant
    pub domains: Vec<String>,
    
    /// Participant base URL for callbacks
    pub url: String,
    
    /// Participant status
    pub status: String,
    
    /// Public key for signature verification
    pub public_key: String,
    
    /// Time when the participant was registered
    pub created_at: DateTime<Utc>,
    
    /// Time when the participant was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Certificate details
    pub certificate: Option<String>,
    
    /// Additional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// Network registry lookup request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupRequest {
    /// Subscriber ID to look up
    pub subscriber_id: Option<String>,
    
    /// Domain to filter by
    pub domain: Option<String>,
    
    /// Participant type to filter by
    pub participant_type: Option<String>,
}

/// Network registry lookup response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupResponse {
    /// Matching participants
    pub participants: Vec<Participant>,
}

/// Registration request for new participants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    /// Participant type being registered
    pub participant_type: String,
    
    /// Domains supported by the participant
    pub domains: Vec<String>,
    
    /// Participant base URL for callbacks
    pub url: String,
    
    /// Public key for signature verification
    pub public_key: String,
    
    /// Certificate details
    pub certificate: Option<String>,
    
    /// Additional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// Registration response with assigned ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResponse {
    /// Assigned subscriber ID
    pub subscriber_id: String,
    
    /// Registration status
    pub status: String,
}

/// Subscription details for a participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Subscriber ID
    pub subscriber_id: String,
    
    /// URL for receiving notifications
    pub url: String,
    
    /// Type of events to subscribe to
    pub events: Vec<String>,
} 