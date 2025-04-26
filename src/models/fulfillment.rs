use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Person info with contact details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Name of the person
    pub name: String,
    
    /// Image URL of the person
    pub image: Option<String>,
    
    /// Gender of the person
    pub gender: Option<String>,
    
    /// Creds/qualifications of the person
    pub creds: Option<String>,
    
    /// Tags associated with the person
    pub tags: Option<HashMap<String, String>>,
}

/// Agent providing the service (e.g., doctor, technician)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// ID of the agent
    pub id: String,
    
    /// Name of the agent
    pub name: String,
    
    /// Gender of the agent
    pub gender: Option<String>,
    
    /// Image URL of the agent
    pub image: Option<String>,
    
    /// Additional details about the agent
    pub tags: HashMap<String, String>,
}

/// Time information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    /// Timestamp in ISO format
    pub timestamp: DateTime<Utc>,
    
    /// Label for the time (e.g., "start", "end")
    pub label: Option<String>,
}

/// Time slot with a defined duration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    /// Start time of the slot
    pub time: Time,
    
    /// Duration of the slot in seconds
    pub duration: Option<i64>,
}

/// State of the fulfillment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    /// Current state of the fulfillment
    pub descriptor: String,
    
    /// Updated time for this state
    pub updated_at: DateTime<Utc>,
}

/// Customer for a fulfillment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// Person details for the customer
    pub person: Person,
    
    /// Contact information
    pub contact: HashMap<String, String>,
}

/// Fulfillment representing service delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fulfillment {
    /// Unique ID for the fulfillment
    pub id: String,
    
    /// Type of fulfillment (e.g., "home-delivery", "teleconsultation")
    pub fulfillment_type: String,
    
    /// ID of the provider delivering the service
    pub provider_id: String,
    
    /// Agent delivering the service
    pub agent: Option<Agent>,
    
    /// Start time slot
    pub start: TimeSlot,
    
    /// End time slot
    pub end: TimeSlot,
    
    /// Customer receiving the service
    pub customer: Option<Customer>,
    
    /// Current state of the fulfillment
    pub state: Option<State>,
    
    /// Additional metadata about the fulfillment
    pub tags: HashMap<String, String>,
} 