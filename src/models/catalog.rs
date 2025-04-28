use crate::models::provider::{Category, Descriptor, Location};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Price represents the monetary value of an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    /// Currency code (e.g., "INR")
    pub currency: String,

    /// Value in the specified currency
    pub value: String,

    /// Maximum value in case of a price range
    pub maximum_value: Option<String>,
}

/// Item in a service catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Unique ID for the item
    pub id: String,

    /// Parent ID for hierarchical items
    pub parent_item_id: Option<String>,

    /// Descriptive information about the item
    pub descriptor: Descriptor,

    /// Price of the item
    pub price: Price,

    /// Category ID this item belongs to
    pub category_id: String,

    /// Fulfillment ID associated with this item
    pub fulfillment_id: String,

    /// Location ID where this item is available
    pub location_id: Option<String>,

    /// Time when this item was last updated
    pub time: Option<DateTime<Utc>>,

    /// Whether this item is recommended
    pub recommended: Option<bool>,

    /// Tags associated with this item
    pub tags: Option<HashMap<String, String>>,
}

/// Catalog representing a collection of items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    /// Descriptive information about the catalog
    pub descriptor: Descriptor,

    /// Categories in the catalog
    pub categories: Vec<Category>,

    /// Fulfillments offered in the catalog
    pub fulfillments: Vec<String>,

    /// Payments accepted for catalog items
    pub payments: Vec<String>,

    /// Locations where catalog items are available
    pub locations: Vec<Location>,

    /// Items in the catalog
    pub items: Vec<Item>,
}

/// Search request for finding catalog items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    /// Query parameters for searching
    pub query: HashMap<String, Vec<String>>,

    /// Item characteristics to filter results
    pub item: Option<Item>,

    /// Fulfillment criteria to filter results
    pub fulfillment: Option<String>,

    /// Payment criteria to filter results
    pub payment: Option<String>,

    /// Location criteria to filter results
    pub location: Option<Location>,
}

/// Search response with catalog items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// Catalog with matching items
    pub catalog: Catalog,
}

/// Item response for selected items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResponse {
    /// Items selected
    pub items: Vec<Item>,
}

/// Quotation for pricing selected items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    /// Price details
    pub price: Price,

    /// Breakdown of price components
    pub breakup: Vec<QuotationBreakup>,

    /// Time when quotation was generated
    pub ttl: String,
}

/// Breakdown component of a quotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotationBreakup {
    /// Title of the breakup component
    pub title: String,

    /// Price for this component
    pub price: Price,
}
