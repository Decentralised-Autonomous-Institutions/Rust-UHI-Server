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

### 1. HTTP Layer (`src/main.rs`, `src/routes.rs`)

The HTTP layer is implemented using Actix-web and handles all incoming HTTP requests. Key components:

* **Server Setup**: Configured in `src/main.rs` using `HttpServer` and `App`
* **Route Configuration**: Defined in `src/routes.rs` with endpoints for UHI Protocol operations
* **Middleware**: Includes:
  * Authentication middleware (X-Gateway-Authorization)
  * Logging middleware
  * Error handling middleware
  * Request tracing

UHI Gateway API endpoints include:
- `/api/v1/search` - Search for healthcare services
- `/api/v1/on_search` - Receive healthcare service catalog
- `/api/v1/select` - Select healthcare services
- `/api/v1/on_select` - Receive price quotation
- `/api/v1/init` - Initialize order
- `/api/v1/on_init` - Receive order with payment details
- `/api/v1/confirm` - Confirm order
- `/api/v1/on_confirm` - Receive confirmed order
- `/api/v1/status` - Check order status
- `/api/v1/on_status` - Receive order status
- `/api/v1/networkregistry/lookup` - Lookup network participants

### 2. Service Layer (`src/services/`)

The service layer contains business logic and orchestrates data operations. Services act as intermediaries between handlers and storage, providing a clean separation of concerns:

* **Search Service**: Orchestrates healthcare service discovery
* **Catalog Service**: Manages healthcare service catalogs
* **Order Service**: Manages healthcare service bookings
* **Fulfillment Service**: Manages service delivery
* **Provider Service**: Manages healthcare service providers
* **Network Registry Service**: Manages network participant registry

Each service:
- Receives a storage implementation via constructor (dependency injection)
- Implements domain-specific business logic
- Communicates with the storage layer through traits
- Handles proper error propagation
- Ensures data consistency

Example service implementation:

```rust
pub struct SearchService {
    storage: Arc<dyn Storage>,
}

impl SearchService {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
    
    pub async fn search(&self, request: SearchRequest) -> Result<SearchResponse, ServiceError> {
        // Business logic
        // Storage access via self.storage
    }
}
```

### 3. Handler Layer (`src/handlers/`)

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

### 4. Storage Layer (`src/storage/`)

The storage layer handles data persistence through a trait-based interface:

* **Storage Trait**: Defines abstract interface for data access
* **PostgreSQL Implementation**: Implements storage trait for PostgreSQL
* **In-Memory Implementation**: Implements storage trait for testing

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

### 6. Error Handling (`src/errors/`)

Comprehensive error handling system:

* **Error Types**: Custom error types for different failure scenarios
* **HTTP Integration**: Maps domain errors to appropriate HTTP status codes
* **Error Propagation**: Structured error propagation across layers
* **Contextual Information**: Enriches errors with contextual information

### 7. Configuration (`src/config.rs`)

* Environment-based configuration
* Database connection settings
* Gateway settings
* Logging configuration
* Security configuration (keys, auth)

### 8. Logging (`src/logging.rs`)

* Structured logging using tracing
* Request/response logging
* Transaction tracing across components
* Error logging with context
* Performance metrics

## Key Architectural Patterns

1. **Trait-based Design**: Storage operations defined through traits for flexibility
2. **Dependency Injection**: Services receive storage implementations via constructor
3. **Service Layer Pattern**: Handlers interact with services, never directly with storage
4. **Middleware Pipeline**: Requests processed through a series of middleware
5. **Async/Await**: Leverages Rust's async capabilities for non-blocking I/O
6. **Error Propagation**: Structured error handling across architectural layers

## Data Flow

### Search Flow Example:
1. EUA sends search request to Gateway
2. Gateway routes to SearchHandler
3. SearchHandler validates request and calls SearchService
4. SearchService processes request and accesses storage
5. SearchService forwards request to appropriate HSPAs via adapter
6. HSPAs respond with on_search to Gateway
7. Gateway routes on_search responses to EUA
8. EUA displays search results to user

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

3. **Error Handling**:
   * Use custom error types for domain-specific errors
   * Implement proper error propagation
   * Log errors with context
   * Provide meaningful error messages

4. **Testing**:
   * Unit tests for business logic in services
   * Integration tests for API endpoints
   * Mock tests for external dependencies
   * Error handling tests

5. **Performance**:
   * Use async/await for I/O operations
   * Implement connection pooling
   * Cache frequently accessed data
   * Monitor and optimize database queries

6. **Security**:
   * Validate all inputs
   * Use proper authentication (X-Gateway-Authorization)
   * Secure database connections
   * Implement rate limiting