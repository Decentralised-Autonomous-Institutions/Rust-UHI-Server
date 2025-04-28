# UHI Protocol - Technical Architecture

## Overview

This document outlines the technical architecture for the Unified Health Interface (UHI) protocol implementation built using Rust, Actix-web, and SQLx. The UHI is an open protocol for various digital health services, enabling a wide variety of interactions between patients and health service providers (HSPs) including appointment booking, teleconsultation, service discovery, and others.

## Technology Stack

* Backend Framework: Actix-web 4.x
* Database Access: SQLx (PostgreSQL)
* Language: Rust 2021 Edition
* Logging: tracing, tracing-subscriber, tracing-actix-web
* Serialization: serde, serde_json
* ID Generation: uuid
* Async Runtime: tokio
* Error Handling: anyhow, thiserror
* Metrics: prometheus-client

## Core Architecture

The system is designed as a layered architecture with the following components:

### 1. HTTP Layer (`src/routes.rs`, `src/handlers/`)

The HTTP layer is implemented using Actix-web and handles all incoming HTTP requests. Key components:

* **Server Setup**: Configured in `src/main.rs` using `HttpServer` and `App`
* **Route Configuration**: Defined in `src/routes.rs` with endpoints for UHI Protocol operations
* **Middleware**: Includes:
  * Authentication middleware (X-Gateway-Authorization)
  * Logging middleware
  * Error handling middleware
  * Request tracing

UHI Gateway API endpoints include:
- `/api/v1/search` & `/api/v1/on_search` - Discovery of healthcare services
- `/api/v1/select` & `/api/v1/on_select` - Selection of specific services
- `/api/v1/init` & `/api/v1/on_init` - Initialization of service booking
- `/api/v1/confirm` & `/api/v1/on_confirm` - Confirmation of service booking
- `/api/v1/status` & `/api/v1/on_status` - Status checking of booked services
- `/api/v1/networkregistry/lookup` - Network registry for provider discovery

### 2. Handler Layer (`src/handlers/`)

The handler layer processes HTTP requests and delegates to the service layer. Handlers never access storage directly:

* **Search/On_Search Handlers**: Handle healthcare service discovery
* **Select/On_Select Handlers**: Handle service selection and price quotation
* **Init/On_Init Handlers**: Handle order initialization
* **Confirm/On_Confirm Handlers**: Handle order confirmation
* **Status/On_Status Handlers**: Handle order status checks
* **Network Registry Handler**: Handle network participant lookups

Each handler:
- Receives appropriate service(s) via dependency injection (web::Data)
- Validates input parameters
- Extracts request data
- Performs context validation
- Calls appropriate service methods
- Formats and returns responses

Example handler implementation:

```rust
pub async fn search(
    payload: web::Json<SearchRequest>,
    service: web::Data<SearchService>,
) -> Result<HttpResponse, Error> {
    // Input validation
    // Call to service layer
    let response = service.search(payload.into_inner()).await?;
    // Response formatting
    Ok(HttpResponse::Ok().json(response))
}
```

### 3. Service Layer (`src/services/`)

The service layer contains the business logic and orchestrates data operations. Services act as intermediaries between handlers and storage, providing a clean separation of concerns. Each service is designed for a specific domain responsibility:

#### SearchService

Handles the discovery of healthcare services through search functionality:
- Processes search requests from End User Applications (EUAs)
- Forwards search requests to Health Service Provider Applications (HSPAs)
- Aggregates and filters search results
- Manages stateful search sessions

```rust
pub struct SearchService {
    storage: Arc<dyn Storage>,
}

impl SearchService {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
    
    pub async fn search(&self, request: SearchRequest) -> Result<SearchResponse, ServiceError> {
        // Search logic implementation
    }
    
    pub async fn on_search(&self, response: SearchResponse) -> Result<(), ServiceError> {
        // On_search callback handling
    }
}
```

#### CatalogService

Manages healthcare service catalogs and selection:
- Creates and updates provider catalogs
- Processes selection of services by patients
- Generates price quotations
- Validates service selection against availability

```rust
pub struct CatalogService {
    storage: Arc<dyn Storage>,
}

impl CatalogService {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
    
    pub async fn create_catalog(&self, provider_id: &str, catalog: Catalog) 
        -> Result<Catalog, ServiceError> {
        // Catalog creation logic
    }
    
    pub async fn select(&self, provider_id: &str, items: Vec<String>) 
        -> Result<Vec<Item>, ServiceError> {
        // Item selection logic
    }
    
    pub async fn on_select(&self, provider_id: &str, items: Vec<Item>) 
        -> Result<Quotation, ServiceError> {
        // Quotation generation logic
    }
}
```

#### OrderService

Manages the lifecycle of healthcare service bookings:
- Processes order initialization
- Handles order confirmation
- Tracks order status
- Coordinates with FulfillmentService for scheduling

```rust
pub struct OrderService {
    storage: Arc<dyn Storage>,
}

impl OrderService {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
    
    pub async fn init(&self, order: Order) -> Result<Order, ServiceError> {
        // Order initialization logic
    }
    
    pub async fn confirm(&self, order_id: &str) -> Result<Order, ServiceError> {
        // Order confirmation logic
    }
    
    pub async fn status(&self, order_id: &str) -> Result<OrderStatus, ServiceError> {
        // Order status checking logic
    }
}
```

#### FulfillmentService

Handles scheduling and delivery of healthcare services:
- Manages provider availability and time slots
- Processes appointment booking and rescheduling
- Tracks fulfillment state transitions
- Ensures fulfillment compliance with order terms

```rust
pub struct FulfillmentService {
    storage: Arc<dyn Storage>,
}

impl FulfillmentService {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
    
    pub async fn create_fulfillment(&self, fulfillment: Fulfillment) 
        -> Result<Fulfillment, ServiceError> {
        // Fulfillment creation logic
    }
    
    pub async fn check_availability(
        &self, 
        provider_id: &str, 
        requested_time: &DateTime<Utc>,
        duration_seconds: i64
    ) -> Result<bool, ServiceError> {
        // Availability checking logic
    }
    
    pub async fn update_state(
        &self,
        fulfillment_id: &str,
        state: &str,
        context: Option<HashMap<String, String>>
    ) -> Result<Fulfillment, ServiceError> {
        // State transition logic
    }
}
```

#### ProviderService

Manages healthcare provider information:
- Handles provider registration and profiles
- Tracks provider specialties and qualifications
- Monitors provider availability and working hours
- Validates provider credentials

```rust
pub struct ProviderService {
    storage: Arc<dyn Storage>,
}

impl ProviderService {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
    
    pub async fn register_provider(&self, provider: Provider) -> Result<Provider, ServiceError> {
        // Provider registration logic
    }
    
    pub async fn check_provider_availability(
        &self,
        provider_id: &str,
        requested_time: &DateTime<Utc>
    ) -> Result<bool, ServiceError> {
        // Provider availability checking logic
    }
    
    pub async fn find_providers_by_specialty(
        &self,
        specialty: &str
    ) -> Result<Vec<Provider>, ServiceError> {
        // Provider search logic
    }
}
```

#### NetworkRegistryService

Manages the registry of participants in the UHI network:
- Handles subscriber registration and verification
- Processes subscriber lookups
- Validates subscriber credentials
- Maintains subscriber metadata

```rust
pub struct NetworkRegistryService {
    storage: Arc<dyn Storage>,
}

impl NetworkRegistryService {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
    
    pub async fn register_subscriber(&self, subscriber: Subscriber) 
        -> Result<Subscriber, ServiceError> {
        // Subscriber registration logic
    }
    
    pub async fn lookup_subscriber(&self, lookup: NetworkRegistryLookup) 
        -> Result<Subscriber, ServiceError> {
        // Subscriber lookup logic
    }
    
    pub async fn validate_signature(
        &self,
        subscriber_id: &str,
        signature: &str,
        data: &[u8]
    ) -> Result<bool, ServiceError> {
        // Signature validation logic
    }
}
```

### 4. Storage Layer (`src/storage/`)

The storage layer handles data persistence through a trait-based interface:

* **Storage Trait**: Defines abstract interface for data access
* **PostgreSQL Implementation**: Implements storage trait for PostgreSQL
* **In-Memory Implementation**: Implements storage trait for testing

The Storage trait is defined as follows:

```rust
#[async_trait]
pub trait Storage: Send + Sync {
    async fn get_provider(&self, id: &str) -> Result<Provider, StorageError>;
    async fn create_provider(&self, provider: Provider) -> Result<Provider, StorageError>;
    async fn update_provider(&self, provider: Provider) -> Result<Provider, StorageError>;
    async fn delete_provider(&self, id: &str) -> Result<(), StorageError>;
    async fn list_providers(&self) -> Result<Vec<Provider>, StorageError>;
    
    // Similar methods for other entities (Catalog, Order, Fulfillment, etc.)
}
```

Storage instances are created in `main.rs`, wrapped in an `Arc`, and injected into services:

```rust
// In main.rs
let storage = Arc::new(PostgresStorage::new(config).await?);

let search_service = web::Data::new(SearchService::new(storage.clone()));
let catalog_service = web::Data::new(CatalogService::new(storage.clone()));
// ... other services

App::new()
    .app_data(search_service.clone())
    .app_data(catalog_service.clone())
    // ... other app data and configuration
```

### 5. Application Initialization (`src/main.rs`)

The main application entry point follows these steps:

1. Initialize configuration and logging
2. Create storage implementation (wrapped in Arc)
3. Create services with injected storage
4. Register services with the Actix app as web::Data
5. Configure routes, middleware, and other app components
6. Start the server

This approach ensures:
- Clear separation of concerns
- Dependency injection for easier testing
- Consistent error handling across layers
- Proper resource sharing with Arc

## Service Interactions

The services interact with each other to fulfill complex business requirements:

1. **Search Flow**:
   - `SearchService` → `ProviderService` for provider information
   - `SearchService` → `FulfillmentService` for availability information

2. **Selection Flow**:
   - `CatalogService` → `ProviderService` for provider details
   - `CatalogService` → `FulfillmentService` for availability checking

3. **Order Flow**:
   - `OrderService` → `CatalogService` for item details and pricing
   - `OrderService` → `FulfillmentService` for appointment scheduling
   - `OrderService` → `ProviderService` for provider information

4. **Fulfillment Flow**:
   - `FulfillmentService` → `ProviderService` for provider availability

5. **Registry Operations**:
   - `NetworkRegistryService` operates independently for registry management
   - Other services consult `NetworkRegistryService` for participant information

## Error Handling

The service layer implements a unified error handling approach:

1. **Error Types**: Each service operation returns `Result<T, ServiceError>`.
2. **Error Categories**:
   - `NotFound`: Resource not found errors
   - `Validation`: Input validation errors
   - `BusinessLogic`: Business rule violation errors
   - `Storage`: Underlying storage errors
   - `External`: External system integration errors
   - `Internal`: Unexpected internal errors

3. **Error Propagation**: Errors are propagated from the storage layer through services to handlers.
4. **Error Translation**: Storage errors are translated to service-level errors with appropriate context.

## Key Architectural Patterns

1. **Trait-based Design**: Storage operations defined through traits for flexibility
2. **Dependency Injection**: Services receive storage implementations via constructor
3. **Service Layer Pattern**: Handlers interact with services, never directly with storage
4. **Middleware Pipeline**: Requests processed through a series of middleware
5. **Async/Await**: Leverages Rust's async capabilities for non-blocking I/O
6. **Error Propagation**: Structured error handling across architectural layers

## Data Flow Example: Appointment Booking

1. Patient sends search request to UHI Gateway
2. Gateway routes to SearchHandler
3. SearchHandler calls SearchService.search()
4. SearchService processes request and forwards to appropriate HSPAs
5. HSPAs respond with on_search to Gateway
6. Gateway routes on_search responses to patient
7. Patient selects an appointment slot via select request
8. CatalogService processes selection and generates quotation
9. Patient initializes order with init request
10. OrderService creates provisional booking and prepares payment information
11. Patient confirms order with confirm request
12. OrderService finalizes booking and generates confirmation
13. FulfillmentService tracks appointment status through its lifecycle

## Development Guidelines

1. **Service Layer Design**:
   * Create stateless services that receive storage via constructor
   * Implement clear interfaces with meaningful methods
   * Handle business logic and coordination in services, not handlers
   * Test services with mock storage implementations

2. **Handler Design**:
   * Keep handlers thin - they should delegate to services quickly
   * Extract and validate inputs, then call appropriate service methods
   * Format responses consistently
   * Use dependency injection for services via web::Data

3. **Testing Strategy**:
   * Unit tests: Test business logic in services with mock storage
   * Integration tests: Test APIs with in-memory storage
   * Performance tests: Benchmark critical operations

4. **Performance Considerations**:
   * Use async/await for I/O operations
   * Implement connection pooling
   * Cache frequently accessed data
   * Monitor and optimize database queries

5. **Security**:
   * Validate all inputs
   * Use proper authentication (X-Gateway-Authorization)
   * Secure database connections
   * Implement rate limiting