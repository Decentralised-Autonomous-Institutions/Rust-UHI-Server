# Unified Health Interface (UHI) Protocol Implementation

This repository contains a Rust implementation of the Unified Health Interface (UHI) Protocol, which aims to standardize healthcare interactions between patients, healthcare providers, and other stakeholders in the healthcare ecosystem.

## Project Structure

The project is organized as follows:

- **src/** - Source code directory
  - **config.rs** - Configuration module for loading settings
  - **errors.rs** - Error handling definitions
  - **handlers/** - HTTP request handlers
    - **search.rs** - Healthcare service discovery handlers
    - **select.rs** - Service selection handlers
    - **init.rs** - Order initialization handlers
    - **confirm.rs** - Order confirmation handlers
    - **status.rs** - Status checking handlers
    - **network_registry.rs** - Network registry handlers
  - **services/** - Business logic and domain services
    - **search_service.rs** - Healthcare service discovery
    - **catalog_service.rs** - Healthcare service catalogs
    - **order_service.rs** - Healthcare service bookings
    - **fulfillment_service.rs** - Service delivery and scheduling
    - **provider_service.rs** - Healthcare service providers
    - **network_registry_service.rs** - Network participant registry
  - **storage/** - Data persistence layer
    - **memory.rs** - In-memory storage implementation
    - **postgres.rs** - PostgreSQL storage implementation
  - **models/** - Data models and schemas
  - **logging.rs** - Logging configuration
  - **routes.rs** - API route definitions
  - **main.rs** - Application entry point

- **config/** - Configuration files
  - **default.toml** - Default configuration
  - **development.toml** - Development environment configuration

- **docs/** - Documentation
  - **technical.md** - Technical architecture overview
  - **tasks.md** - Implementation tasks
  - **status.md** - Project status
  - **services/** - Service layer documentation
    - **search-service.md** - SearchService technical design
    - **catalog-service.md** - CatalogService technical design
    - **order-service.md** - OrderService technical design
    - **fulfillment-service.md** - FulfillmentService technical design
    - **provider-service.md** - ProviderService technical design
    - **network-registry-service.md** - NetworkRegistryService technical design

## Getting Started

### Prerequisites

- Rust toolchain (latest stable)
- PostgreSQL database (optional, in-memory storage available for development)

### Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-uhi.git
   cd rust-uhi
   ```

2. Copy the example env file:
   ```
   cp env.example .env
   ```

3. Modify the `.env` file with your configuration.

4. Build the project:
   ```
   cargo build
   ```

5. Run the project:
   ```
   cargo run
   ```

## Architecture Overview

This project implements a layered architecture with a clear separation of concerns:

1. **HTTP Layer**: Handles incoming requests and routes them to appropriate handlers
2. **Handler Layer**: Processes HTTP requests and delegates to the service layer
3. **Service Layer**: Contains business logic and orchestrates data operations
4. **Storage Layer**: Manages data persistence through a trait-based interface

The service layer follows a dependency injection pattern:
- Storage instances are created in `main.rs` and wrapped in `Arc`
- Services receive storage through their constructors
- Handlers receive services via Actix's `web::Data`

This approach ensures:
- Clear separation of concerns
- Easier testing through mock implementations
- Consistent error handling across layers
- Proper resource sharing

For more details, see [Technical Architecture](docs/technical.md).

## Service Layer

The core of the application is the service layer, which implements the business logic for the UHI Protocol:

- **SearchService**: Handles discovery of healthcare services
- **CatalogService**: Manages healthcare service catalogs
- **OrderService**: Manages healthcare service bookings
- **FulfillmentService**: Handles scheduling and service delivery
- **ProviderService**: Manages healthcare provider information
- **NetworkRegistryService**: Manages network participant registry

Each service is designed with clear responsibilities and interfaces for improved modularity and testability. For detailed documentation on each service, see the [services directory](docs/services/).

## API Endpoints

The UHI Protocol defines the following core API endpoints:

- `/api/v1/search` & `/api/v1/on_search` - Discovery of healthcare services
- `/api/v1/select` & `/api/v1/on_select` - Selection of specific services
- `/api/v1/init` & `/api/v1/on_init` - Initialization of service booking
- `/api/v1/confirm` & `/api/v1/on_confirm` - Confirmation of service booking
- `/api/v1/status` & `/api/v1/on_status` - Status checking of booked services
- `/api/v1/networkregistry/lookup` - Network registry for provider discovery

Each endpoint corresponds to a specific part of the healthcare service discovery and booking flow. See the [UHI Protocol specification](schema/core.yml) for more details.

## Development Status

This project is currently in active development. The implementation status is as follows:

- **FulfillmentService**: ~80% complete
- **ProviderService**: ~60% complete
- **SearchService**: ~20% complete
- **CatalogService**: ~10% complete
- **OrderService**: ~10% complete
- **NetworkRegistryService**: ~30% complete

For more details on the current status and upcoming tasks, see:
- [Project Status](docs/status.md)
- [Implementation Tasks](docs/tasks.md)

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to improve the implementation.

Before contributing, please:
1. Check the [Project Status](docs/status.md) to see what's in progress
2. Review the [Implementation Tasks](docs/tasks.md) for guidance
3. Follow the existing code style and patterns

## Testing

To run the tests:
```
cargo test
```

We use a combination of unit tests, integration tests, and mock objects to ensure the quality of the codebase.

## License

This project is licensed under the MIT License - see the LICENSE file for details.