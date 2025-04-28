# OrderService Technical Design

## Overview

The OrderService is a core component responsible for managing the lifecycle of healthcare service bookings within the UHI Protocol implementation. It handles order initialization, confirmation, and status tracking, serving as the central service for all order-related operations. This service coordinates between patients, healthcare providers, and payment systems to ensure smooth fulfillment of healthcare services.

## Responsibilities

- Process order initialization requests from End User Applications (EUAs)
- Handle order confirmation and payment integration
- Track order status throughout its lifecycle
- Manage order updates and cancellations
- Coordinate with FulfillmentService for appointment scheduling
- Process order-related callbacks from providers
- Maintain order history and audit trails
- Generate notifications for order state changes

## Interfaces

### Public Methods

```rust
pub struct OrderService {
    storage: Arc<dyn Storage>,
}

impl OrderService {
    /// Create a new order service with injected storage
    pub fn new(storage: Arc<dyn Storage>) -> Self;
    
    /// Create a new order
    /// 
    /// # Parameters
    /// * `order` - The order to create
    /// 
    /// # Returns
    /// * `Result<Order, ServiceError>` - Created order or error
    pub async fn create_order(&self, order: Order) -> Result<Order, ServiceError>;
    
    /// Get an order by ID
    /// 
    /// # Parameters
    /// * `id` - The order ID
    /// 
    /// # Returns
    /// * `Result<Order, ServiceError>` - Order or error if not found
    pub async fn get_order(&self, id: &str) -> Result<Order, ServiceError>;
    
    /// Update an existing order
    /// 
    /// # Parameters
    /// * `order` - The updated order
    /// 
    /// # Returns
    /// * `Result<Order, ServiceError>` - Updated order or error
    pub async fn update_order(&self, order: Order) -> Result<Order, ServiceError>;
    
    /// Initialize an order (init)
    /// 
    /// # Parameters
    /// * `order` - The order initialization data
    /// 
    /// # Returns
    /// * `Result<Order, ServiceError>` - Initialized order or error
    pub async fn init(&self, order: Order) -> Result<Order, ServiceError>;
    
    /// Handle provider's response to order initialization (on_init)
    /// 
    /// # Parameters
    /// * `order_id` - The ID of the order being initialized
    /// * `provider_order` - The provider's order response
    /// 
    /// # Returns
    /// * `Result<Order, ServiceError>` - Updated order or error
    pub async fn on_init(&self, order_id: &str, provider_order: Order) -> Result<Order, ServiceError>;
    
    /// Confirm an order
    /// 
    /// # Parameters
    /// * `order_id` - The ID of the order to confirm
    /// 
    /// # Returns
    /// * `Result<Order, ServiceError>` - Confirmed order or error
    pub async fn confirm(&self, order_id: &str) -> Result<Order, ServiceError>;
    
    /// Handle provider's confirmation response
    /// 
    /// # Parameters
    /// * `order_id` - The ID of the order being confirmed
    /// * `provider_order` - The provider's confirmed order
    /// 
    /// # Returns
    /// * `Result<Order, ServiceError>` - Updated order with confirmation details or error
    pub async fn on_confirm(&self, order_id: &str, provider_order: Order) -> Result<Order, ServiceError>;
    
    /// Get order status
    /// 
    /// # Parameters
    /// * `order_id` - The ID of the order
    /// 
    /// # Returns
    /// * `Result<OrderStatus, ServiceError>` - Current order status or error
    pub async fn status(&self, order_id: &str) -> Result<OrderStatus, ServiceError>;
    
    /// Handle provider's status response
    /// 
    /// # Parameters
    /// * `order_id` - The ID of the order
    /// * `status` - The updated status from provider
    /// 
    /// # Returns
    /// * `Result<Order, ServiceError>` - Updated order with status details or error
    pub async fn on_status(&self, order_id: &str, status: OrderStatus) -> Result<Order, ServiceError>;
    
    /// List orders by provider
    /// 
    /// # Parameters
    /// * `provider_id` - The provider's ID
    /// 
    /// # Returns
    /// * `Result<Vec<Order>, ServiceError>` - List of orders for provider or error
    pub async fn list_orders_by_provider(&self, provider_id: &str) -> Result<Vec<Order>, ServiceError>;
    
    /// List orders by customer
    /// 
    /// # Parameters
    /// * `customer_id` - The customer's ID
    /// 
    /// # Returns
    /// * `Result<Vec<Order>, ServiceError>` - List of orders for customer or error
    pub async fn list_orders_by_customer(&self, customer_id: &str) -> Result<Vec<Order>, ServiceError>;
}
```

### Dependencies

- **Storage**: Persistent storage layer for order data
- **FulfillmentService**: For appointment scheduling and availability checks
- **CatalogService**: For item validation and pricing information
- **ProviderService**: For provider information

## Data Models

### Order

```rust
pub struct Order {
    /// Unique ID for the order
    pub id: String,
    
    /// Provider summary
    pub provider: ProviderSummary,
    
    /// Current state of the order
    pub state: String,
    
    /// List of ordered items
    pub items: Vec<OrderItem>,
    
    /// Billing information
    pub billing: Billing,
    
    /// Fulfillment details
    pub fulfillment: Fulfillment,
    
    /// Price quotation
    pub quote: Option<Quotation>,
    
    /// Payment details
    pub payment: Option<Payment>,
    
    /// Time when the order was created
    pub created_at: DateTime<Utc>,
    
    /// Time when the order was last updated
    pub updated_at: DateTime<Utc>,
}
```

### OrderItem

```rust
pub struct OrderItem {
    /// ID of the item
    pub id: String,
    
    /// Quantity ordered
    pub quantity: i32,
    
    /// Full catalog item details
    pub item: Item,
}
```

### OrderStatus

```rust
pub struct OrderStatus {
    /// Current state of the order
    pub state: String,
    
    /// Timestamp when status was updated
    pub updated_at: DateTime<Utc>,
}
```

### ProviderSummary

```rust
pub struct ProviderSummary {
    /// ID of the provider
    pub id: String,
    
    /// Descriptive name of the provider
    pub descriptor: String,
    
    /// Categories the provider belongs to
    pub categories: Vec<String>,
}
```

## Implementation Details

### Order Lifecycle Management

The OrderService implements a state machine to manage the order lifecycle:

1. **Order States**:
   - `INITIATED`: Order has been initialized but not yet accepted by provider
   - `QUOTED`: Provider has provided pricing details
   - `PROVISIONALLY_BOOKED`: Order slot has been temporarily reserved
   - `CONFIRMED`: Order has been confirmed but service not yet delivered
   - `IN_PROGRESS`: Service delivery has started
   - `COMPLETED`: Service has been delivered successfully
   - `CANCELLED`: Order has been cancelled
   - `FAILED`: Order processing has failed

2. **State Transitions**:
   - Each state transition is validated for correctness
   - Required fields for each state are enforced
   - Appropriate timestamps for state changes are recorded
   - Notifications are generated for relevant state changes

### Order Processing Workflow

1. **Initialization (`init`)**
   - Validate order structure and required fields
   - Verify item availability with CatalogService
   - Generate order ID
   - Set initial state to INITIATED
   - Store order in database
   - Return created order object

2. **Provider Response (`on_init`)**
   - Match response to original order
   - Update order with provider details (quote, payment options)
   - Set state to QUOTED
   - Store updated order

3. **Confirmation (`confirm`)**
   - Validate order is in QUOTED state
   - Update order with payment details
   - Check slot availability again with FulfillmentService
   - Set state to PROVISIONALLY_BOOKED
   - Store updated order

4. **Provider Confirmation Response (`on_confirm`)**
   - Validate payment status
   - Update order with provider confirmation details
   - Set state to CONFIRMED
   - Store finalized order

5. **Status Updates**
   - Process status updates from providers
   - Update order state according to provider status
   - Handle transitions through IN_PROGRESS to COMPLETED
   - Store status history for audit trails

### Error Handling

- **Validation Errors**: Return detailed validation errors for each field
- **State Transition Errors**: Clear messaging when invalid state transitions are attempted
- **Availability Errors**: Handle cases when slot becomes unavailable during booking
- **Payment Errors**: Graceful handling of payment failures
- **Provider Communication Errors**: Retry mechanisms for provider callbacks

## Configuration

The OrderService is configurable through the following parameters:

- `provisional_booking_ttl`: How long a provisional booking is held (default: 15m)
- `payment_timeout`: Maximum time allowed for payment completion (default: 30m)
- `order_history_retention`: How long to keep completed orders (default: 3 years)
- `max_concurrent_orders_per_user`: Throttling limit (default: 5)
- `cancellation_window`: Time window when cancellation is allowed (default: 24h)

## Usage Examples

### Initializing an Order

```rust
let storage = Arc::new(MemoryStorage::new());
let order_service = OrderService::new(storage);

let order = Order {
    id: "".to_string(), // Will be generated
    provider: ProviderSummary {
        id: "provider-123".to_string(),
        descriptor: "Hospital ABC".to_string(),
        categories: vec!["Cardiology".to_string()],
    },
    state: "".to_string(), // Will be set to INITIATED
    items: vec![
        OrderItem {
            id: "item-1".to_string(),
            quantity: 1,
            item: Item {
                id: "item-1".to_string(),
                descriptor: Descriptor {
                    name: "Cardiac Consultation".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            },
        },
    ],
    billing: Billing {
        name: "John Doe".to_string(),
        address: Address {
            door: Some("42".to_string()),
            building: Some("Apartment Complex".to_string()),
            street: Some("Main Street".to_string()),
            locality: Some("Downtown".to_string()),
            city: "Metropolis".to_string(),
            state: "State".to_string(),
            country: "Country".to_string(),
            area_code: "12345".to_string(),
        },
        email: Some("john.doe@example.com".to_string()),
        phone: "+1234567890".to_string(),
        ..Default::default()
    },
    fulfillment: Fulfillment {
        id: "fulfillment-1".to_string(),
        type_field: "Teleconsultation".to_string(),
        provider_id: "provider-123".to_string(),
        ..Default::default()
    },
    ..Default::default()
};

let initialized_order = order_service.init(order).await?;
println!("Order initialized with ID: {}", initialized_order.id);
```

### Confirming an Order

```rust
let confirmed_order = order_service.confirm("order-456").await?;
println!("Order confirmed with state: {}", confirmed_order.state);
```

### Checking Order Status

```rust
let status = order_service.status("order-456").await?;
println!("Current order state: {}", status.state);
println!("Last updated: {}", status.updated_at);
```

## Testing Strategy

1. **Unit Tests**
   - Test order state transitions
   - Test validation logic
   - Test error handling scenarios

2. **Integration Tests**
   - Test with mock providers
   - Test full order lifecycle from init to completion
   - Test order listing and filtering

3. **Performance Tests**
   - Benchmark order creation
   - Test concurrent order processing
   - Measure order retrieval performance with large datasets