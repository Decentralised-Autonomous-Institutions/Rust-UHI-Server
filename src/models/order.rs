use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::catalog::{Item, Quotation};
use crate::models::fulfillment::Fulfillment;
use crate::models::billing::Billing;
use crate::models::payment::Payment;

/// Summary of a provider for order references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSummary {
    /// ID of the provider
    pub id: String,
    
    /// Descriptive name of the provider
    pub descriptor: String,
    
    /// Categories the provider belongs to
    pub categories: Vec<String>,
}

/// Order item with quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    /// ID of the item
    pub id: String,
    
    /// Quantity ordered
    pub quantity: i32,
    
    /// Full catalog item details
    pub item: Item,
}

/// Order status object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatus {
    /// Current state of the order
    pub state: String,
    
    /// Timestamp when status was updated
    pub updated_at: DateTime<Utc>,
}

/// Order representing a healthcare service booking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Unique ID for the order
    pub id: String,
    
    /// Provider summary
    pub provider: ProviderSummary,
    
    /// List of ordered items
    pub items: Vec<OrderItem>,
    
    /// Billing information
    pub billing: Billing,
    
    /// Fulfillment details
    pub fulfillment: Fulfillment,
    
    /// Price quotation
    pub quote: Option<Quotation>,
    
    /// Payment details
    pub payment: Option<Payment>,
    
    /// Current state of the order
    pub state: String,
    
    /// Time when the order was created
    pub created_at: DateTime<Utc>,
    
    /// Time when the order was last updated
    pub updated_at: DateTime<Utc>,
}

/// Order initialization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInitRequest {
    /// List of items to order
    pub items: Vec<OrderItem>,
    
    /// Billing information
    pub billing: Billing,
    
    /// Fulfillment details
    pub fulfillment: Fulfillment,
}

/// Order initialization response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInitResponse {
    /// Order details
    pub order: Order,
}

/// Order confirmation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderConfirmRequest {
    /// Order ID to confirm
    pub order_id: String,
    
    /// Payment details
    pub payment: Payment,
}

/// Order confirmation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderConfirmResponse {
    /// Confirmed order details
    pub order: Order,
}

/// Order status request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    /// Order ID to check
    pub order_id: String,
}

/// Order status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusResponse {
    /// Order status details
    pub order: Order,
} 