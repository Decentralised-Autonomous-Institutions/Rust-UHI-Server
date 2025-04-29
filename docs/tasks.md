# UHI Project Tasks

This document outlines the detailed tasks for implementing the Unified Health Interface (UHI) project. Tasks are organized by component and broken down into subtasks for easier tracking and implementation.

## 1. Core UHI Gateway Server

### 1.1 Project Setup and Configuration
- [x] Initialize project structure
- [x] Set up dependency management
- [x] Create configuration module
- [x] Implement environment-based configuration
- [x] Set up logging configuration
- [x] Configure server parameters (host, port, workers)

### 1.2 Data Models
- [x] Define context model
- [x] Define provider model
- [x] Define catalog model
- [x] Define fulfillment model
- [x] Define order model
- [x] Define billing model
- [x] Define payment model
- [x] Define network registry models
- [x] Implement serialization/deserialization

### 1.3 Storage Layer
- [x] Define storage trait interface
- [ ] Implement in-memory storage for testing and development
  - [x] Define in-memory data structures
  - [x] Implement in-memory provider operations
  - [x] Implement in-memory catalog operations
  - [x] Implement in-memory fulfillment operations
  - [x] Implement in-memory order operations
  - [x] Implement in-memory network registry operations
  - [x] Implement in-memory transaction tracking
- [ ] Create mock data for testing with in-memory storage
- [ ] Write unit tests for in-memory storage
- [ ] **(Later Phase)** Create database schema
- [ ] **(Later Phase)** Implement database migrations
- [ ] **(Later Phase)** Implement PostgreSQL storage
  - [ ] Provider operations
  - [ ] Catalog operations
  - [ ] Fulfillment operations
  - [ ] Order operations
  - [ ] Network registry operations
  - [ ] Transaction tracking

### 1.4 Service Layer
- [x] Design service layer with dependency injection pattern
  - [x] Define service interfaces
  - [x] Implement constructor-based storage injection
  - [ ] Configure service registration in main.rs
- [ ] Implement search service
  - [x] Define service interface
  - [x] Implement search functionality
  - [x] Implement transaction tracking
  - [x] Implement provider lookup
  - [x] Implement result aggregation and merging
  - [ ] Implement advanced search criteria matching
  - [ ] Implement result filtering and sorting
- [ ] Implement catalog service
  - [x] Define service interface
  - [x] Implement catalog management
  - [x] Implement select functionality
  - [x] Implement on_select functionality
  - [x] Implement pricing and quotation
  - [x] Implement item availability checking
  - [ ] Implement advanced business rules for discounts
- [ ] Implement order service
  - [ ] Define service interface
  - [ ] Implement order creation
  - [ ] Implement init functionality
  - [ ] Implement on_init functionality
  - [ ] Implement confirm functionality
  - [ ] Implement on_confirm functionality
  - [ ] Implement status functionality
  - [ ] Implement on_status functionality
  - [ ] Implement order state transitions
- [ ] Implement fulfillment service
  - [ ] Define service interface
  - [ ] Implement basic CRUD operations
  - [ ] Implement provider availability checking
  - [ ] Implement time slot validation
  - [ ] Implement state transitions
  - [ ] Implement buffer management
  - [ ] Implement recurring appointment handling
- [ ] Implement provider service
  - [x] Define service interface
  - [x] Implement basic CRUD operations
  - [x] Implement provider availability checking
  - [x] Implement working hours validation
  - [x] Implement specialty-based search
  - [x] Implement location-based search
  - [ ] Implement credential validation
- [ ] Implement network registry service
  - [ ] Define service interface
  - [ ] Implement subscriber registration
  - [ ] Implement subscriber lookup
  - [ ] Implement signature validation
  - [ ] Implement domain verification
  - [ ] Implement certificate management

### 1.5 Service Integration
- [ ] Implement service interaction patterns
  - [ ] Define service dependencies
  - [ ] Implement FulfillmentService integration with ProviderService
  - [ ] Implement CatalogService integration with FulfillmentService
  - [ ] Implement OrderService integration with CatalogService
  - [ ] Implement OrderService integration with FulfillmentService
  - [ ] Implement SearchService integration with ProviderService
- [ ] Implement end-to-end flows
  - [ ] Implement search flow
  - [ ] Implement selection flow
  - [ ] Implement order initialization flow
  - [ ] Implement order confirmation flow
  - [x] Implement status checking flow

### 1.6 HTTP Layer
- [x] Define API routes
- [x] Set up middleware pipeline
- [ ] Implement authentication middleware
  - [ ] Parse X-Gateway-Authorization header
  - [ ] Verify signatures
  - [ ] Validate subscriber information
- [x] Implement error handling middleware
- [x] Implement request logging middleware
- [ ] Implement request tracing middleware

### 1.7 Handler Layer
- [x] Define handler interfaces
- [ ] Update handlers to use service dependency injection
  - [ ] Remove direct storage access from handlers
  - [ ] Inject services via web::Data
  - [ ] Implement proper error propagation
- [ ] Implement search handlers
  - [x] Define handler interface
  - [ ] Implement search handler
  - [ ] Implement on_search handler
  - [ ] Integrate with SearchService
- [ ] Implement select handlers
  - [x] Define handler interface
  - [ ] Implement select handler
  - [ ] Implement on_select handler
  - [ ] Integrate with CatalogService
- [ ] Implement init handlers
  - [x] Define handler interface
  - [ ] Implement init handler
  - [ ] Implement on_init handler
  - [ ] Integrate with OrderService
- [ ] Implement confirm handlers
  - [x] Define handler interface
  - [ ] Implement confirm handler
  - [ ] Implement on_confirm handler
  - [ ] Integrate with OrderService
- [ ] Implement status handlers
  - [x] Define handler interface
  - [ ] Implement status handler
  - [ ] Implement on_status handler
  - [ ] Integrate with OrderService
- [ ] Implement network registry handlers
  - [x] Define handler interface
  - [ ] Implement lookup handler
  - [ ] Integrate with NetworkRegistryService

### 1.8 Application Initialization
- [ ] Implement dependency injection in main.rs
  - [ ] Initialize storage with configuration
  - [ ] Create services with injected storage
  - [ ] Register services with Actix app
  - [ ] Configure routes with services

### 1.9 Testing
- [ ] Create test fixtures and helpers
- [x] Unit tests for models
- [ ] Unit tests for service layer with mock storage
  - [x] Unit tests for fulfillment service
  - [x] Unit tests for provider service
  - [ ] Unit tests for search service
  - [ ] Unit tests for catalog service
  - [ ] Unit tests for order service
  - [ ] Unit tests for network registry service
- [ ] Unit tests for handlers with mocked services
- [ ] Integration tests for API endpoints using in-memory storage
- [ ] **(Later Phase)** Integration tests with PostgreSQL
- [ ] **(Later Phase)** Load tests
  - [ ] Performance testing
  - [ ] Scalability testing

### 1.10 Documentation
- [ ] API documentation
- [ ] OpenAPI specification
- [x] Code documentation
- [ ] Example requests and responses
- [x] Architecture documentation with dependency flow
- [ ] Service layer documentation
  - [x] FulfillmentService documentation
  - [x] ProviderService documentation
  - [x] SearchService documentation
  - [x] CatalogService documentation
  - [x] OrderService documentation
  - [x] NetworkRegistryService documentation

## 2. End User Application (EUA) (Future Phase)

### 2.1 Frontend Framework Setup
- [ ] Initialize frontend project
- [ ] Set up React/Next.js framework
- [ ] Configure routing
- [ ] Set up state management
- [ ] Configure API client

### 2.2 Authentication and User Management
- [ ] Implement user registration
- [ ] Implement user login
- [ ] Implement JWT handling
- [ ] Implement user profile management

### 2.3 Healthcare Service Discovery
- [ ] Implement search interface
- [ ] Implement search result display
- [ ] Implement filtering and sorting
- [ ] Implement provider details view

### 2.4 Booking and Appointment Management
- [ ] Implement service selection
- [ ] Implement scheduling interface
- [ ] Implement booking confirmation
- [ ] Implement payment integration
- [ ] Implement appointment status tracking

### 2.5 Health Records Management
- [ ] Implement health record upload
- [ ] Implement health record viewing
- [ ] Implement health record sharing
- [ ] Implement blockchain integration for record verification

### 2.6 Testing and Optimization
- [ ] Unit tests
- [ ] Integration tests
- [ ] E2E tests
- [ ] Accessibility testing
- [ ] Performance optimization

## 3. Health Service Provider Application (HSPA) (Future Phase)

### 3.1 Provider Dashboard
- [ ] Initialize provider dashboard project
- [ ] Set up authentication
- [ ] Implement service management
- [ ] Implement scheduling interface
- [ ] Implement provider profile management

### 3.2 Service Catalog Management
- [ ] Implement service creation
- [ ] Implement service editing
- [ ] Implement pricing configuration
- [ ] Implement category management

### 3.3 Appointment Management
- [ ] Implement appointment list
- [ ] Implement appointment details
- [ ] Implement appointment status updates
- [ ] Implement appointment rescheduling

### 3.4 Provider API Integration
- [ ] Implement API client
- [ ] Handle search requests
- [ ] Handle booking requests
- [ ] Handle status updates

### 3.5 Testing and Optimization
- [ ] Unit tests
- [ ] Integration tests
- [ ] E2E tests
- [ ] Performance optimization

## 4. Connectors (Future Phase)

### 4.1 Electronic Health Record (EHR) Connector
- [ ] Define integration interface
- [ ] Implement data transformation
- [ ] Implement authentication with EHR systems
- [ ] Create adapters for popular EHR systems

### 4.2 Hospital Information System (HIS) Connector
- [ ] Define integration interface
- [ ] Implement data transformation
- [ ] Implement authentication with HIS systems
- [ ] Create adapters for HIS systems

### 4.3 Laboratory Information System (LIS) Connector
- [ ] Define integration interface
- [ ] Implement data transformation
- [ ] Implement result retrieval
- [ ] Implement order placement

### 4.4 Insurance System Connector
- [ ] Define integration interface
- [ ] Implement eligibility verification
- [ ] Implement claim submission
- [ ] Implement payment processing

## 5. Blockchain Certification Layer (Future Phase)

### 5.1 Blockchain Infrastructure
- [ ] Select appropriate blockchain platform
- [ ] Set up blockchain nodes
- [ ] Configure consensus mechanism
- [ ] Implement smart contracts

### 5.2 Record Certification
- [ ] Implement hashing of health records
- [ ] Implement blockchain storage of hashes
- [ ] Implement certificate generation
- [ ] Implement certificate verification

## 6. Deployment and DevOps (Future Phase)

### 6.1 Infrastructure Setup
- [ ] Set up development environment
- [ ] Set up staging environment
- [ ] Set up production environment
- [ ] Configure load balancing

### 6.2 CI/CD Pipeline
- [ ] Set up continuous integration
- [ ] Set up continuous deployment
- [ ] Configure automated testing
- [ ] Set up monitoring and alerting

### 6.3 Containerization
- [ ] Create Docker containers
- [ ] Configure Docker Compose
- [ ] Set up Kubernetes deployment
- [ ] Configure auto-scaling

## Priority Tasks for Next Sprint

1. **Complete Service Layer Implementation**
   - Finish SearchService implementation
   - Advance CatalogService implementation
   - Progress on OrderService implementation
   - Complete NetworkRegistryService implementation

2. **Handler Integration**
   - Update all handlers to use service layer
   - Remove direct storage access from handlers
   - Implement proper error handling in handlers

3. **Service Integration**
   - Implement service interactions for key flows
   - Complete FulfillmentService integration with ProviderService
   - Integrate CatalogService with FulfillmentService
   - Integrate OrderService with other services

4. **Testing**
   - Add unit tests for all services
   - Implement integration tests for key flows
   - Create mock data for testing scenarios

5. **Documentation**
   - Update API documentation
   - Create example requests and responses
   - Document service interactions