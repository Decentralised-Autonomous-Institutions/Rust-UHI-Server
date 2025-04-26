# UHI Protocol - Technical Architecture

## Overview

This document outlines the technical architecture for the Universal Health Interface (UHI) protocol implementation built using Rust, Actix-web, and SQLx. The UHI is an open protocol for various digital health services, enabling a wide variety of interactions between patients and health service providers (HSPs) including appointment booking, teleconsultation, service discovery, and others.

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

### 2. Handler Layer (`src/handlers/`)

The handler layer processes HTTP requests and delegates to the service layer:

* **Search/On_Search Handlers**: Handle healthcare service discovery
* **Select/On_Select Handlers**: Handle service selection and price quotation
* **Init/On_Init Handlers**: Handle order initialization
* **Confirm/On_Confirm Handlers**: Handle order confirmation
* **Status/On_Status Handlers**: Handle order status checks
* **Network Registry Handler**: Handle network participant lookups

Each handler:
- Validates input parameters
- Extracts request data
- Performs context validation (domain, country, city, timestamp, etc.)
- Calls appropriate service methods
- Formats and returns responses

### 3. Service Layer (`src/services/`)

The service layer contains business logic and orchestrates data operations:

* **Search Service**: Orchestrates healthcare service discovery
* **Catalog Service**: Manages healthcare service catalogs
* **Order Service**: Manages healthcare service bookings
* **Fulfillment Service**: Manages service delivery
* **Provider Service**: Manages healthcare service providers
* **Network Registry Service**: Manages network participant registry

Each service:
- Implements domain-specific business logic
- Communicates with the storage layer through traits
- Handles proper error propagation
- Ensures data consistency

### 4. Storage Layer (`src/storage/`)

The storage layer handles data persistence:

* **Storage Trait**: Defines abstract interface for data access
* **PostgreSQL Implementation**: Implements storage trait for PostgreSQL
* **In-Memory Implementation**: Implements storage trait for testing

Interface methods include:
- Provider-related operations (create, read, update, delete)
- Catalog-related operations (search, retrieve)
- Order-related operations (create, update, retrieve)
- Fulfillment-related operations (create, update, retrieve)
- Network registry operations (register, lookup)

### 5. Error Handling (`src/errors/`)

Comprehensive error handling system:

* **Error Types**: Custom error types for different failure scenarios
* **HTTP Integration**: Maps domain errors to appropriate HTTP status codes
* **Error Propagation**: Structured error propagation across layers
* **Contextual Information**: Enriches errors with contextual information

### 6. Configuration (`src/config.rs`)

* Environment-based configuration
* Database connection settings
* Gateway settings
* Logging configuration
* Security configuration (keys, auth)

### 7. Logging (`src/logging.rs`)

* Structured logging using tracing
* Request/response logging
* Transaction tracing across components
* Error logging with context
* Performance metrics

## Key Architectural Patterns

1. **Trait-based Design**: Storage operations defined through traits for flexibility
2. **Dependency Injection**: Services receive storage implementations via constructor
3. **Middleware Pipeline**: Requests processed through a series of middleware
4. **Async/Await**: Leverages Rust's async capabilities for non-blocking I/O
5. **Error Propagation**: Structured error handling across architectural layers

## Data Flow

### Search Flow Example:
1. EUA sends search request to Gateway
2. Gateway authenticates request and routes to SearchHandler
3. SearchHandler validates request and calls SearchService
4. SearchService processes request and forwards to appropriate HSPAs
5. HSPAs respond with on_search to Gateway
6. Gateway routes on_search responses to EUA
7. EUA displays search results to user

## Data Models

### Context Model (`src/models/context.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub domain: String,
    pub country: String,
    pub city: String,
    pub action: String,
    pub core_version: String,
    pub consumer_id: String,
    pub consumer_uri: String,
    pub provider_id: Option<String>,
    pub provider_uri: Option<String>,
    pub transaction_id: String,
    pub message_id: String,
    pub timestamp: DateTime<Utc>,
}
```

### Provider Model (`src/models/provider.rs`)

```rust
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub descriptor: Descriptor,
    pub categories: Vec<Category>,
    pub fulfillments: Vec<Fulfillment>,
    pub items: Vec<Item>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub name: String,
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
    pub images: Option<Vec<String>>,
}
```

### Fulfillment Model (`src/models/fulfillment.rs`)

```rust
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Fulfillment {
    pub id: String,
    pub fulfillment_type: String,
    pub provider_id: String,
    pub agent: Option<Agent>,
    pub start: TimeSlot,
    pub end: TimeSlot,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub gender: Option<String>,
    pub image: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub time: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    pub timestamp: DateTime<Utc>,
}
```

### Order Model (`src/models/order.rs`)

```rust
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub provider: ProviderSummary,
    pub items: Vec<OrderItem>,
    pub billing: Billing,
    pub fulfillment: Fulfillment,
    pub quote: Option<Quotation>,
    pub payment: Option<Payment>,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Billing {
    pub name: String,
    pub address: Address,
    pub email: Option<String>,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub uri: String,
    pub tl_method: Option<String>,
    pub params: Option<HashMap<String, String>>,
    pub payment_type: String,
    pub status: String,
}
```

## Development Guidelines

1. **Error Handling**:
   * Use custom error types for domain-specific errors
   * Implement proper error propagation
   * Log errors with context
   * Provide meaningful error messages

2. **Logging**:
   * Use structured logging for all operations
   * Include transaction IDs in logs
   * Log errors with full context
   * Use appropriate log levels

3. **Testing**:
   * Unit tests for business logic
   * Integration tests for API endpoints
   * Mock tests for external dependencies
   * Error handling tests

4. **Performance**:
   * Use async/await for I/O operations
   * Implement connection pooling
   * Cache frequently accessed data
   * Monitor and optimize database queries

5. **Security**:
   * Validate all inputs
   * Use proper authentication (X-Gateway-Authorization)
   * Secure database connections
   * Implement rate limiting