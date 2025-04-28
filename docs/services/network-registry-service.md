# NetworkRegistryService Technical Design

## Overview

The NetworkRegistryService is responsible for managing the registry of participants in the UHI network. It handles subscriber registration, discovery, and lookup operations, ensuring secure and authenticated communication between End User Applications (EUAs), Health Service Provider Applications (HSPAs), and the Gateway. This service is crucial for maintaining the integrity and security of the UHI ecosystem.

## Responsibilities

- Manage participant registration and verification in the UHI network
- Handle subscriber lookup by various criteria (ID, domain, type)
- Validate subscriber credentials and digital signatures
- Maintain subscriber metadata and status
- Support network participant discovery
- Ensure secure communication between network entities
- Track participant certificates and public keys
- Manage participant subscription lifecycle

## Interfaces

### Public Methods

```rust
pub struct NetworkRegistryService {
    storage: Arc<dyn Storage>,
}

impl NetworkRegistryService {
    /// Create a new network registry service with injected storage
    pub fn new(storage: Arc<dyn Storage>) -> Self;
    
    /// Register a new subscriber
    /// 
    /// # Parameters
    /// * `subscriber` - The subscriber information to register
    /// 
    /// # Returns
    /// * `Result<Subscriber, ServiceError>` - Registered subscriber or error
    pub async fn register_subscriber(&self, subscriber: Subscriber) 
        -> Result<Subscriber, ServiceError>;
    
    /// Get a subscriber by ID
    /// 
    /// # Parameters
    /// * `id` - The subscriber ID
    /// 
    /// # Returns
    /// * `Result<Subscriber, ServiceError>` - Subscriber or error if not found
    pub async fn get_subscriber(&self, id: &str) -> Result<Subscriber, ServiceError>;
    
    /// Lookup a subscriber based on criteria
    /// 
    /// # Parameters
    /// * `lookup` - The lookup criteria
    /// 
    /// # Returns
    /// * `Result<Subscriber, ServiceError>` - Matching subscriber or error
    pub async fn lookup_subscriber(&self, lookup: NetworkRegistryLookup) 
        -> Result<Subscriber, ServiceError>;
    
    /// List all subscribers
    /// 
    /// # Returns
    /// * `Result<Vec<Subscriber>, ServiceError>` - List of subscribers or error
    pub async fn list_subscribers(&self) -> Result<Vec<Subscriber>, ServiceError>;
    
    /// Validate subscriber credentials
    /// 
    /// # Parameters
    /// * `subscriber_id` - The subscriber ID
    /// * `signature` - The digital signature to validate
    /// * `data` - The data that was signed
    /// 
    /// # Returns
    /// * `Result<bool, ServiceError>` - Whether the signature is valid
    pub async fn validate_signature(
        &self,
        subscriber_id: &str,
        signature: &str,
        data: &[u8]
    ) -> Result<bool, ServiceError>;
    
    /// Update subscriber status
    /// 
    /// # Parameters
    /// * `subscriber_id` - The subscriber ID
    /// * `status` - The new status
    /// 
    /// # Returns
    /// * `Result<Subscriber, ServiceError>` - Updated subscriber or error
    pub async fn update_subscriber_status(
        &self,
        subscriber_id: &str,
        status: &str
    ) -> Result<Subscriber, ServiceError>;
    
    /// Find subscribers by type
    /// 
    /// # Parameters
    /// * `type_field` - The type to filter by (EUA, HSP, GATEWAY)
    /// 
    /// # Returns
    /// * `Result<Vec<Subscriber>, ServiceError>` - Matching subscribers or error
    pub async fn find_subscribers_by_type(
        &self,
        type_field: &str
    ) -> Result<Vec<Subscriber>, ServiceError>;
    
    /// Find subscribers by domain
    /// 
    /// # Parameters
    /// * `domain` - The domain to filter by
    /// 
    /// # Returns
    /// * `Result<Vec<Subscriber>, ServiceError>` - Matching subscribers or error
    pub async fn find_subscribers_by_domain(
        &self,
        domain: &str
    ) -> Result<Vec<Subscriber>, ServiceError>;
    
    /// Verify subscriber domain and certificate
    /// 
    /// # Parameters
    /// * `subscriber_id` - The subscriber ID
    /// 
    /// # Returns
    /// * `Result<VerificationResult, ServiceError>` - Verification result or error
    pub async fn verify_subscriber(
        &self,
        subscriber_id: &str
    ) -> Result<VerificationResult, ServiceError>;
}
```

### Dependencies

- **Storage**: Persistent storage layer for subscriber data

## Data Models

### Subscriber

```rust
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
```

### NetworkRegistryLookup

```rust
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
```

### Participant

```rust
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
```

### VerificationResult

```rust
pub struct VerificationResult {
    /// Whether the verification was successful
    pub success: bool,
    
    /// Detailed verification results by check
    pub checks: HashMap<String, CheckResult>,
    
    /// Overall status message
    pub message: String,
}

pub struct CheckResult {
    /// Whether this specific check passed
    pub passed: bool,
    
    /// Details about the check
    pub details: String,
}
```

## Implementation Details

### Subscriber Registration Flow

1. **Registration Request Validation**:
   - Validate required subscriber information
   - Check for uniqueness of subscriber ID
   - Verify domain ownership (if applicable)
   - Validate URL accessibility and SSL certificate

2. **Public Key Handling**:
   - Validate public key format and integrity
   - Store public key for future signature verification
   - Support rotation of keys (if applicable)

3. **Subscription Status Management**:
   - Initial status set to `INITIATED`
   - Transition through verification steps
   - Final status of `SUBSCRIBED` when all checks pass
   - Support for suspension and unsubscription

### Lookup and Discovery

1. **Lookup Mechanisms**:
   - Direct lookup by subscriber ID
   - Lookup by criteria (type, domain, location)
   - Filter results based on status (active subscribers only)
   - Optimize lookup performance with indexing

2. **Caching Strategy**:
   - Cache frequently accessed subscribers
   - Invalidate cache on subscriber updates
   - Use time-based cache expiration
   - Implement cache warming for critical subscribers

### Signature Verification

1. **Verification Process**:
   - Extract public key for the subscriber
   - Verify signature against provided data
   - Support multiple signature algorithms (Ed25519, RSA)
   - Handle signature verification errors gracefully

2. **Security Considerations**:
   - Protection against replay attacks
   - Timestamp validation for signatures
   - Rate limiting for verification requests
   - Audit logging for verification attempts

### Error Handling

- **Validation Errors**: Detailed errors for invalid subscriber information
- **Not Found Errors**: Clear messaging for missing subscribers
- **Signature Errors**: Specific errors for signature verification failures
- **Certificate Errors**: Errors related to SSL certificate validation
- **Status Errors**: Error handling for invalid status transitions

## Configuration

The NetworkRegistryService is configurable through the following parameters:

- `signature_ttl`: Maximum age of signatures (default: 5m)
- `subscriber_cache_ttl`: Cache time for subscriber data (default: 15m)
- `verification_timeout`: Timeout for external verification calls (default: 30s)
- `enable_domain_verification`: Toggle for domain verification (default: true)
- `max_verification_retries`: Maximum retries for verification (default: 3)

## Usage Examples

### Registering a Subscriber

```rust
let storage = Arc::new(MemoryStorage::new());
let network_registry_service = NetworkRegistryService::new(storage);

let subscriber = Subscriber {
    id: "eua-example".to_string(),
    type_field: "EUA".to_string(),
    domain: "nic2004:85111".to_string(),
    city: Some("std:080".to_string()),
    country: Some("IND".to_string()),
    url: "https://example-eua.com/api/v1".to_string(),
    status: "INITIATED".to_string(),
    public_key: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...".to_string(),
    created_at: Utc::now(),
    updated_at: Utc::now(),
};

let registered_subscriber = network_registry_service.register_subscriber(subscriber).await?;
println!("Subscriber registered with ID: {}", registered_subscriber.id);
```

### Looking Up Subscribers

```rust
// Lookup by ID
let subscriber = network_registry_service.get_subscriber("eua-example").await?;

// Lookup by criteria
let lookup = NetworkRegistryLookup {
    type_field: "HSP".to_string(),
    domain: "nic2004:85111".to_string(),
    city: Some("std:080".to_string()),
    country: Some("IND".to_string()),
};

let matching_subscriber = network_registry_service.lookup_subscriber(lookup).await?;

// Find by type
let euas = network_registry_service.find_subscribers_by_type("EUA").await?;
println!("Found {} EUAs", euas.len());
```

### Validating Signatures

```rust
let subscriber_id = "eua-example";
let data = r#"{"context":{"domain":"nic2004:85111","country":"IND","city":"std:080"}}"#.as_bytes();
let signature = "base64_encoded_signature_here";

let is_valid = network_registry_service
    .validate_signature(subscriber_id, signature, data)
    .await?;

if is_valid {
    println!("Signature is valid");
} else {
    println!("Signature is invalid");
}
```

### Verifying Subscriber

```rust
let verification_result = network_registry_service
    .verify_subscriber("eua-example")
    .await?;

if verification_result.success {
    println!("Subscriber verified successfully");
} else {
    println!("Subscriber verification failed: {}", verification_result.message);
    for (check_name, result) in verification_result.checks {
        if !result.passed {
            println!("Check '{}' failed: {}", check_name, result.details);
        }
    }
}
```

## Testing Strategy

1. **Unit Tests**
   - Test subscriber validation
   - Test signature verification algorithms
   - Test lookup filtering mechanisms
   - Test error handling scenarios

2. **Integration Tests**
   - Test full registration flow
   - Verify correct storage interaction
   - Test lookup functionality with real-world examples
   - Test signature verification with valid and invalid signatures

3. **Performance Tests**
   - Benchmark subscriber lookup operations
   - Test caching effectiveness
   - Measure signature verification performance
   - Test concurrent registry operations