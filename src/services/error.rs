use crate::storage::StorageError;
use std::fmt;

/// Error type for service operations
#[derive(Debug)]
pub enum ServiceError {
    /// Error from the storage layer
    Storage(StorageError),
    
    /// Resource not found
    NotFound(String),
    
    /// Validation error
    Validation(String),
    
    /// Business logic error
    BusinessLogic(String),
    
    /// External service error
    ExternalService(String),
    
    /// Generic error
    Internal(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::Storage(err) => write!(f, "Storage error: {}", err),
            ServiceError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ServiceError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ServiceError::BusinessLogic(msg) => write!(f, "Business logic error: {}", msg),
            ServiceError::ExternalService(msg) => write!(f, "External service error: {}", msg),
            ServiceError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<StorageError> for ServiceError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::NotFound(msg) => ServiceError::NotFound(msg),
            _ => ServiceError::Storage(err),
        }
    }
}

/// Convert error from String
impl From<String> for ServiceError {
    fn from(err: String) -> Self {
        ServiceError::Internal(err)
    }
}

/// Convert from standard error
impl From<Box<dyn std::error::Error>> for ServiceError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ServiceError::Internal(err.to_string())
    }
} 