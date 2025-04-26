use serde::{Deserialize, Serialize};

/// Address with components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Door/unit number
    pub door: Option<String>,
    
    /// Name of building/complex
    pub building: Option<String>,
    
    /// Street name
    pub street: Option<String>,
    
    /// Locality/area name
    pub locality: Option<String>,
    
    /// City/town name
    pub city: String,
    
    /// State/province name
    pub state: String,
    
    /// Country name
    pub country: String,
    
    /// Area code (PIN code, ZIP code)
    pub area_code: String,
}

/// Billing information for orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Billing {
    /// Name of the person/entity being billed
    pub name: String,
    
    /// Organization name (optional)
    pub organization: Option<String>,
    
    /// Billing address
    pub address: Address,
    
    /// Email for billing communications
    pub email: Option<String>,
    
    /// Phone for billing communications
    pub phone: String,
    
    /// Tax information (e.g., GST/VAT number)
    pub tax_number: Option<String>,
}

/// Invoice for a completed order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    /// Invoice number/ID
    pub id: String,
    
    /// Billing details
    pub billing: Billing,
    
    /// Order ID associated with this invoice
    pub order_id: String,
    
    /// Tax details (tax rate, tax value)
    pub taxes: Option<Vec<Tax>>,
}

/// Tax component in an invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tax {
    /// Name of the tax (e.g., "GST", "VAT")
    pub name: String,
    
    /// Percentage rate of the tax
    pub rate: f64,
    
    /// Value of the tax amount
    pub value: String,
} 