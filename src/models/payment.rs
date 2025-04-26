use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::catalog::Price;

/// Payment status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    /// Payment is due
    #[serde(rename = "DUE")]
    Due,
    
    /// Payment is pending
    #[serde(rename = "PENDING")]
    Pending,
    
    /// Payment is paid
    #[serde(rename = "PAID")]
    Paid,
    
    /// Payment has failed
    #[serde(rename = "FAILED")]
    Failed,
}

/// Payment type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentType {
    /// Payment on delivery
    #[serde(rename = "ON-DELIVERY")]
    OnDelivery,
    
    /// Payment in advance
    #[serde(rename = "ON-ORDER")]
    OnOrder,
    
    /// Payment using credit
    #[serde(rename = "CREDIT")]
    Credit,
    
    /// Payment via bank transfer
    #[serde(rename = "BANK-TRANSFER")]
    BankTransfer,
}

/// Payment method for processing payments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// URI for processing payment
    pub uri: String,
    
    /// Transaction ID for the payment
    pub tl_method: Option<String>,
    
    /// Parameters for the payment
    pub params: Option<HashMap<String, String>>,
    
    /// Type of payment
    pub payment_type: String,
    
    /// Current status of the payment
    pub status: String,
    
    /// Time in ISO format when payment was created
    pub time: Option<String>,
    
    /// Total amount for this payment
    pub amount: Option<Price>,
    
    /// Currency code for the payment
    pub currency: Option<String>,
}

/// Payment details for a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDetails {
    /// Unique payment ID
    pub id: String,
    
    /// Payment method used
    pub payment: Payment,
    
    /// Gateway used for processing (e.g., "razorpay", "paytm")
    pub gateway: Option<String>,
    
    /// Additional transaction details
    pub transaction_details: Option<HashMap<String, String>>,
}

/// Card details for card payments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// Card number (masked)
    pub card_number: String,
    
    /// Card holder name
    pub holder_name: String,
    
    /// Expiry date in MM/YY format
    pub expiry: String,
}

/// Bank details for bank transfers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    /// Account holder name
    pub holder_name: String,
    
    /// Account number (masked)
    pub account_number: String,
    
    /// IFSC code
    pub ifsc_code: String,
    
    /// Bank name
    pub bank_name: Option<String>,
    
    /// Bank branch name
    pub branch_name: Option<String>,
} 