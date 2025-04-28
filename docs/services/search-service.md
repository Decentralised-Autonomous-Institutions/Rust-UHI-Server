# SearchService Technical Design

## Overview

The SearchService is a core component of the UHI Protocol implementation that facilitates healthcare service discovery. It serves as an intermediary between the user-facing handlers and the underlying storage layer, implementing business logic for processing search requests and handling responses from healthcare service providers.

## Responsibilities

- Process search requests from End User Applications (EUAs)
- Forward search requests to appropriate Health Service Provider Applications (HSPAs) through the Gateway
- Aggregate and filter search results from multiple HSPAs
- Handle search criteria matching and relevance sorting
- Manage stateful search sessions across the request lifecycle

## Interfaces

### Public Methods

```rust
pub struct SearchService {
    storage: Arc<dyn Storage>,
}

impl SearchService {
    /// Create a new search service with injected storage
    pub fn new(storage: Arc<dyn Storage>) -> Self;
    
    /// Process a search request from an EUA
    /// 
    /// # Parameters
    /// * `request` - The search request containing intent criteria
    /// 
    /// # Returns
    /// * `Result<SearchResponse, ServiceError>` - Success or failure with detailed error
    pub async fn search(&self, request: SearchRequest) -> Result<SearchResponse, ServiceError>;
    
    /// Process search results from HSPAs
    /// 
    /// # Parameters
    /// * `response` - The search response from an HSPA
    /// 
    /// # Returns
    /// * `Result<(), ServiceError>` - Success or failure with detailed error
    pub async fn on_search(&self, response: SearchResponse) -> Result<(), ServiceError>;
    
    /// Track search transactions to maintain session state
    /// 
    /// # Parameters
    /// * `transaction_id` - The identifier for the search transaction
    /// * `search_data` - The data associated with the search
    /// 
    /// # Returns
    /// * `Result<(), ServiceError>` - Success or failure with detailed error
    pub async fn track_search_transaction(
        &self, 
        transaction_id: &str, 
        search_data: SearchMetadata
    ) -> Result<(), ServiceError>;
}
```

### Dependencies

- **Storage**: Persistent storage layer for tracking search transactions and provider information
- **ProviderService**: (Optional) For filtering provider information during search

## Data Models

### Search Request

```rust
pub struct SearchRequest {
    /// Query parameters for searching
    pub query: HashMap<String, Vec<String>>,
    
    /// Item characteristics to filter results
    pub item: Option<Item>,
    
    /// Fulfillment criteria to filter results
    pub fulfillment: Option<Fulfillment>,
    
    /// Payment criteria to filter results
    pub payment: Option<Payment>,
    
    /// Location criteria to filter results
    pub location: Option<Location>,
}
```

### Search Response

```rust
pub struct SearchResponse {
    /// Catalog with matching items
    pub catalog: Catalog,
}
```

### Search Metadata

```rust
pub struct SearchMetadata {
    /// Transaction ID for the search session
    pub transaction_id: String,
    
    /// Timestamp when search was initiated
    pub timestamp: DateTime<Utc>,
    
    /// Original search request
    pub request: SearchRequest,
    
    /// List of providers to which the search was forwarded
    pub forwarded_to: Vec<String>,
    
    /// Responses received from providers
    pub responses: HashMap<String, SearchResponse>,
}
```

## Implementation Details

### Search Process Flow

1. **Request Validation**
   - Validate incoming search request against schema
   - Check for required fields based on search type (by name, provider, category, etc.)

2. **Provider Discovery**
   - Identify relevant HSPAs based on search criteria (domain, location, specialty)
   - Retrieve provider endpoints from storage or network registry

3. **Request Forwarding**
   - Create stateful search session with transaction ID
   - Forward search request to identified HSPAs
   - Track forwarded requests in storage

4. **Response Handling**
   - Receive and validate responses from HSPAs via `on_search`
   - Associate responses with original transaction
   - Apply filtering and relevance sorting

5. **Result Aggregation**
   - Merge catalogs from multiple providers
   - Remove duplicates and apply ranking
   - Format final response according to UHI Protocol specification

### Search Criteria Handling

The SearchService implements specialized handlers for different search criteria types:

- **Keyword Search**: Matching service names, descriptions across providers
- **Provider Search**: Filtering by healthcare provider attributes
- **Specialty Search**: Finding providers by medical specialty
- **Availability Search**: Filtering by appointment time slots
- **Location-Based Search**: Finding nearby providers within radius

### Error Handling

- **Validation Errors**: Return detailed field-specific validation errors
- **Provider Connection Errors**: Handle timeouts and connection issues gracefully
- **Partial Results**: Return partial results if some providers fail to respond
- **Rate Limiting**: Implement rate limiting for search requests

## Configuration

The SearchService is configurable through the following parameters:

- `search_timeout`: Maximum time to wait for provider responses (default: 30s)
- `max_providers_per_search`: Maximum number of providers to forward a search to (default: 10)
- `min_providers_for_results`: Minimum providers that must respond for valid results (default: 1)
- `concurrent_search_limit`: Maximum number of concurrent searches (default: 100)

## Usage Examples

### Basic Search Request

```rust
let storage = Arc::new(MemoryStorage::new());
let search_service = SearchService::new(storage);

let search_request = SearchRequest {
    query: HashMap::from([
        ("specialty".to_string(), vec!["Cardiology".to_string()]),
    ]),
    location: Some(Location {
        gps: "12.9716,77.5946".to_string(),
        radius: Some(5.0),
    }),
    ..Default::default()
};

let response = search_service.search(search_request).await?;
```

### Processing an On_Search Response

```rust
let on_search_response = SearchResponse {
    catalog: Catalog {
        descriptor: Descriptor {
            name: "Hospital ABC".to_string(),
            ..Default::default()
        },
        providers: vec![/* provider details */],
        ..Default::default()
    },
};

search_service.on_search(on_search_response).await?;
```

## Testing Strategy

1. **Unit Tests**
   - Test individual search criteria handling
   - Test response aggregation and filtering
   - Test error handling scenarios

2. **Integration Tests**
   - Test with mock HSPAs
   - Verify correct transaction handling
   - Test timeout and retry mechanisms

3. **Performance Tests**
   - Benchmark with large result sets
   - Test concurrent search handling