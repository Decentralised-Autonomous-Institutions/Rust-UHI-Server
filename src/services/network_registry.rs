use super::error::ServiceError;
use crate::models::network_registry::{NetworkRegistryLookup, Subscriber, LookupRequest, LookupResponse, Participant};
use crate::storage::Storage;
use std::sync::Arc;
use chrono::Utc;
use std::collections::HashMap;
use ring::signature::{self, UnparsedPublicKey, KeyPair, Ed25519KeyPair, ECDSA_P256_SHA256_ASN1};
use ring::rand::SystemRandom;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::time::Duration;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use url::Url;
use std::str::FromStr;

/// Network registry service for managing network participants
pub struct NetworkRegistryService {
    /// Storage implementation injected via constructor
    storage: Arc<dyn Storage>,
    /// HTTP client for domain verification
    http_client: Client,
}

impl NetworkRegistryService {
    /// Create a new network registry service with storage dependency
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        // Create HTTP client with reasonable timeout
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { 
            storage,
            http_client: client,
        }
    }

    /// Register a new subscriber
    pub async fn register_subscriber(
        &self,
        subscriber: Subscriber,
    ) -> Result<Subscriber, ServiceError> {
        // Validate subscriber data
        self.validate_subscriber(&subscriber)?;

        // Optionally verify domain if URL is provided
        if !subscriber.url.is_empty() {
            self.verify_domain_ownership(&subscriber).await?;
        }

        // Register in storage
        let registered = self.storage.register_subscriber(subscriber).await?;
        Ok(registered)
    }

    /// Get a subscriber by ID
    pub async fn get_subscriber(&self, id: &str) -> Result<Subscriber, ServiceError> {
        let subscriber = self.storage.get_subscriber(id).await?;
        Ok(subscriber)
    }

    /// Lookup a subscriber based on criteria
    pub async fn lookup_subscriber(
        &self,
        lookup: NetworkRegistryLookup,
    ) -> Result<Subscriber, ServiceError> {
        // Basic input validation
        if lookup.type_field.is_empty() && lookup.domain.is_empty() {
            return Err(ServiceError::Validation(
                "At least one lookup criteria must be provided".to_string(),
            ));
        }

        let subscriber = self.storage.lookup_subscriber(lookup).await?;
        Ok(subscriber)
    }

    /// Enhanced lookup for participants with multiple criteria
    pub async fn lookup_participants(
        &self,
        request: LookupRequest,
    ) -> Result<LookupResponse, ServiceError> {
        // Validate request
        if request.subscriber_id.is_none() && request.domain.is_none() && request.participant_type.is_none() {
            return Err(ServiceError::Validation(
                "At least one lookup criteria must be provided".to_string(),
            ));
        }

        // Get all subscribers
        let subscribers = self.storage.list_subscribers().await?;
        
        // Filter based on criteria
        let mut participants = Vec::new();
        
        for subscriber in subscribers {
            let mut matches = true;
            
            // Filter by subscriber_id if provided
            if let Some(ref id) = request.subscriber_id {
                if subscriber.id != *id {
                    matches = false;
                }
            }
            
            // Filter by domain if provided
            if let Some(ref domain) = request.domain {
                if !subscriber.domain.contains(domain) {
                    matches = false;
                }
            }
            
            // Filter by type if provided
            if let Some(ref participant_type) = request.participant_type {
                if subscriber.type_field != *participant_type {
                    matches = false;
                }
            }
            
            if matches {
                // Convert to Participant
                let participant = Participant {
                    subscriber_id: subscriber.id,
                    participant_type: subscriber.type_field,
                    domains: vec![subscriber.domain],
                    url: subscriber.url,
                    status: subscriber.status,
                    public_key: subscriber.public_key,
                    created_at: subscriber.created_at,
                    updated_at: subscriber.updated_at,
                    certificate: None, // Not implemented in basic version
                    metadata: None,    // Not implemented in basic version
                };
                
                participants.push(participant);
            }
        }
        
        Ok(LookupResponse { participants })
    }

    /// List all subscribers
    pub async fn list_subscribers(&self) -> Result<Vec<Subscriber>, ServiceError> {
        let subscribers = self.storage.list_subscribers().await?;
        Ok(subscribers)
    }

    /// Validate a signature using the subscriber's public key
    pub async fn validate_signature(
        &self,
        subscriber_id: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool, ServiceError> {
        // Get the subscriber to retrieve their public key
        let subscriber = self.get_subscriber(subscriber_id).await?;
        
        // Decode the signature from base64
        let signature_bytes = match BASE64.decode(signature) {
            Ok(bytes) => bytes,
            Err(_) => return Err(ServiceError::Validation("Invalid signature format".to_string())),
        };
        
        // Decode the public key from base64
        let public_key_bytes = match BASE64.decode(&subscriber.public_key) {
            Ok(bytes) => bytes,
            Err(_) => return Err(ServiceError::Validation("Invalid public key format".to_string())),
        };
        
        // Verify the signature (assuming Ed25519 algorithm, you might need to support others)
        let public_key = UnparsedPublicKey::new(&signature::ED25519, &public_key_bytes);
        
        match public_key.verify(message, &signature_bytes) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Create a test key pair for development and testing
    pub fn generate_test_keypair() -> Result<(String, String), ServiceError> {
        // Generate a new Ed25519 key pair
        let rng = SystemRandom::new();
        let pkcs8_bytes = match Ed25519KeyPair::generate_pkcs8(&rng) {
            Ok(bytes) => bytes,
            Err(_) => return Err(ServiceError::Internal("Failed to generate key pair".to_string())),
        };
        
        let key_pair = match Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()) {
            Ok(kp) => kp,
            Err(_) => return Err(ServiceError::Internal("Failed to parse key pair".to_string())),
        };
        
        // Get public key in proper format
        let public_key_bytes = key_pair.public_key().as_ref();
        let public_key = BASE64.encode(public_key_bytes);
        
        // Encode private key (in real system, this would be more secure)
        let private_key = BASE64.encode(pkcs8_bytes);
        
        Ok((public_key, private_key))
    }

    /// Verify domain ownership using DNS verification or web verification
    async fn verify_domain_ownership(&self, subscriber: &Subscriber) -> Result<(), ServiceError> {
        // Parse URL to extract domain
        let url = match Url::parse(&subscriber.url) {
            Ok(u) => u,
            Err(_) => return Err(ServiceError::Validation(format!(
                "Invalid URL: {}", subscriber.url
            ))),
        };
        
        let domain = match url.host_str() {
            Some(host) => host,
            None => return Err(ServiceError::Validation(
                "URL has no host component".to_string()
            )),
        };
        
        // Check if the domain matches the claimed domain in the subscriber record
        if domain != subscriber.domain {
            return Err(ServiceError::Validation(format!(
                "URL domain {} does not match claimed domain {}",
                domain, subscriber.domain
            )));
        }
        
        // In a real implementation, we would perform actual domain verification such as:
        // 1. DNS TXT record verification
        // 2. Serving a specific file at a well-known URL
        // 3. HTTPS certificate validation
        
        // For now, we'll just do a simple HTTP GET to verify the domain is reachable
        // In production, this should be replaced with proper verification
        
        // Skip actual HTTP verification for localhost/development
        if domain.contains("localhost") || domain.contains("127.0.0.1") {
            return Ok(());
        }
        
        // Make a HEAD request to the URL to check if it's reachable
        match self.http_client.head(&subscriber.url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(ServiceError::Validation(format!(
                        "Domain verification failed: HTTP status {}", response.status()
                    )))
                }
            },
            Err(e) => Err(ServiceError::Validation(format!(
                "Domain verification failed: {}", e
            ))),
        }
    }
    
    /// Generate a verification token for a domain
    pub fn generate_verification_token(&self, subscriber_id: &str) -> String {
        // In a real implementation, this would generate a secure random token
        // and store it associated with the domain for later verification
        
        // For simplicity, we're just creating a deterministic token here
        format!("uhi-verify-{}-{}", subscriber_id, Utc::now().timestamp())
    }

    /// Validate subscriber data
    fn validate_subscriber(&self, subscriber: &Subscriber) -> Result<(), ServiceError> {
        // Check that required fields are present
        if subscriber.id.is_empty() {
            return Err(ServiceError::Validation(
                "Subscriber ID is required".to_string(),
            ));
        }

        if subscriber.type_field.is_empty() {
            return Err(ServiceError::Validation(
                "Subscriber type is required".to_string(),
            ));
        }

        if subscriber.domain.is_empty() {
            return Err(ServiceError::Validation(
                "Subscriber domain is required".to_string(),
            ));
        }

        if subscriber.url.is_empty() {
            return Err(ServiceError::Validation(
                "Subscriber URL is required".to_string(),
            ));
        }

        if subscriber.public_key.is_empty() {
            return Err(ServiceError::Validation(
                "Subscriber public key is required".to_string(),
            ));
        }
        
        // Validate URL format
        if let Err(e) = Url::from_str(&subscriber.url) {
            return Err(ServiceError::Validation(format!(
                "Invalid URL format: {}", e
            )));
        }

        // All validations passed
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::memory::MemoryStorage;
    use chrono::Utc;
    
    fn create_test_subscriber() -> Subscriber {
        Subscriber {
            id: "test-subscriber-1".to_string(),
            type_field: "HSP".to_string(),
            domain: "example.com".to_string(),
            city: Some("Test City".to_string()),
            country: Some("Test Country".to_string()),
            url: "https://example.com/api".to_string(),
            status: "ACTIVE".to_string(),
            public_key: "dGVzdC1wdWJsaWMta2V5".to_string(), // base64 for "test-public-key"
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    #[tokio::test]
    async fn test_register_subscriber() {
        let storage = MemoryStorage::empty();
        let service = NetworkRegistryService::new(storage);
        
        let subscriber = create_test_subscriber();
        let result = service.register_subscriber(subscriber.clone()).await;
        
        assert!(result.is_ok());
        let registered = result.unwrap();
        assert_eq!(registered.id, subscriber.id);
    }
    
    #[tokio::test]
    async fn test_lookup_subscriber() {
        let storage = MemoryStorage::empty();
        let service = NetworkRegistryService::new(storage);
        
        let subscriber = create_test_subscriber();
        let _ = service.register_subscriber(subscriber.clone()).await;
        
        let lookup = NetworkRegistryLookup {
            type_field: "HSP".to_string(),
            domain: "example.com".to_string(),
            city: None,
            country: None,
        };
        
        let result = service.lookup_subscriber(lookup).await;
        assert!(result.is_ok());
        let found = result.unwrap();
        assert_eq!(found.id, subscriber.id);
    }
    
    #[tokio::test]
    async fn test_generate_keypair() {
        let result = NetworkRegistryService::generate_test_keypair();
        assert!(result.is_ok());
        
        let (public_key, private_key) = result.unwrap();
        assert!(!public_key.is_empty());
        assert!(!private_key.is_empty());
    }
    
    // Additional tests would be added for signature validation, domain verification, etc.
}
