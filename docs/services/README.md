# UHI Protocol Service Layer: Comprehensive Overview

## Contents

Here are the technical design documents for each core service:

- [CatalogService Technical Design](catalog-service.md)
- [FulfillmentService Technical Design](fulfillment-service.md)
- [NetworkRegistryService Technical Design](network-registry-service.md)
- [OrderService Technical Design](order-service.md)
- [ProviderService Technical Design](provider-service.md)
- [SearchService Technical Design](search-service.md)


## Introduction

The service layer is the heart of the UHI (Universal Health Interface) Protocol implementation, providing the core business logic and domain functionality that drives the application. It serves as an intermediate layer between the API handlers that process HTTP requests and the storage layer that persists data. This document provides a high-level overview of the service layer architecture, design principles, and the responsibilities of each service.

## Architectural Principles

The service layer follows these key architectural principles:

1. **Separation of Concerns**: Each service has a well-defined responsibility within the domain.
2. **Dependency Injection**: Services receive their dependencies via constructor injection.
3. **Interface-Based Design**: Services define clear interfaces for testing and replaceable implementations.
4. **Stateless Operation**: Services do not maintain state between invocations.
5. **Error Handling**: Consistent error handling and propagation throughout the service layer.
6. **Domain-Driven Design**: Services are organized around domain concepts rather than technical concerns.

## Dependency Flow

The service layer implements a clean dependency injection pattern:

1. **Storage Layer Dependency**: Services depend on the storage layer via the `Storage` trait.
2. **Storage Injection**: Storage implementations are injected into services via constructor.
3. **Inter-Service Dependencies**: Services may depend on other services for specialized functionality.
4. **Handler Layer Consumption**: Handlers consume services via Actix's `web::Data`.

```
Handlers → Services → Storage
```

## Core Services

The UHI Protocol implementation includes the following core services:

### 1. SearchService

Handles the discovery of healthcare services and providers through search functionality. It facilitates the search process between patients and healthcare providers, allowing patients to find appropriate healthcare services based on various criteria such as specialty, location, and availability.

[Detailed Technical Design](search-service-design.md)

### 2. CatalogService

Manages healthcare service catalogs and the selection of services by patients. It handles price quotation generation and ensures accurate representation of available healthcare services.

[Detailed Technical Design](catalog-service-design.md)

### 3. OrderService

Manages the lifecycle of healthcare service bookings from initialization through confirmation and status updates. It coordinates between patients, providers, and payment systems to ensure smooth fulfillment of healthcare services.

[Detailed Technical Design](order-service-design.md)

### 4. FulfillmentService

Handles the scheduling and delivery aspects of healthcare services, including availability management, appointment booking, and service delivery tracking.

[Detailed Technical Design](fulfillment-service-design.md)

### 5. ProviderService

Manages healthcare provider information, credentials, specialties, and availability. It ensures that provider data is accurate and accessible for service discovery.

[Detailed Technical Design](provider-service-design.md)

### 6. NetworkRegistryService

Manages the registry of participants in the UHI network, including authentication, authorization, and discovery of network entities such as EUAs (End User Applications) and HSPAs (Health Service Provider Applications).

[Detailed Technical Design](network-registry-service-design.md)

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

## Testing Strategy

The service layer's testing approach includes:

1. **Unit Testing**:
   - Mock storage for isolated service testing
   - Test business logic in isolation
   - Test error handling paths

2. **Integration Testing**:
   - Test service with in-memory storage
   - Verify correct interaction between services
   - Test complete workflows

3. **Performance Testing**:
   - Benchmark core service operations
   - Test with representative data volumes
   - Identify bottlenecks in service processing

## Implementation Guidelines

When implementing or extending the service layer, follow these guidelines:

1. **Service Responsibilities**: Keep services focused on specific domain responsibilities.
2. **Constructor Injection**: All dependencies should be injected via the constructor.
3. **Error Handling**: Use the `ServiceError` enum for all error cases.
4. **Validation**: Validate all inputs at the service boundary.
5. **Logging**: Include appropriate logging for significant operations and errors.
6. **Transaction Management**: Consider transaction boundaries for multi-step operations.
7. **Performance**: Be mindful of performance implications, especially for data-intensive operations.

## Configuration

Services are configured through application-level configuration parameters:

1. **Service-Specific Settings**: Each service has its own configuration section.
2. **Default Values**: All configuration parameters have sensible defaults.
3. **Environment Overrides**: Configuration can be overridden via environment variables.
4. **Dynamic Configuration**: Some settings can be adjusted at runtime.

## Conclusion

The service layer in the UHI Protocol implementation provides a robust, domain-oriented framework for healthcare service discovery, booking, and fulfillment. By following clean architectural principles and clear separation of concerns, it ensures maintainability, testability, and extensibility while delivering the complex business functionality required by the UHI Protocol.