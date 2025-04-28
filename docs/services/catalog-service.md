# CatalogService Technical Design

## Overview

The CatalogService is responsible for managing healthcare service catalogs in the UHI Protocol implementation. It facilitates the selection and quotation of healthcare services, handling both provider catalogs and consumer selection requests. This service acts as a bridge between the selection handlers and the storage layer, implementing business logic for service pricing, availability, and selection validation.

## Responsibilities

- Manage healthcare service catalogs for providers
- Process service selection requests from End User Applications (EUAs)
- Generate price quotations for selected services
- Validate service selection against availability
- Handle service options, variants, and bundling
- Maintain catalog freshness and versioning
- Apply business rules for service pricing and discounts

## Interfaces

### Public Methods

```rust
pub struct CatalogService {
    storage: Arc<dyn Storage>,
}

impl CatalogService {
    /// Create a new catalog service with injected storage
    pub fn new(storage: Arc<dyn Storage>) -> Self;

    /// Create or update a provider's catalog
    /// 
    /// # Parameters
    /// * `provider_id` - The unique identifier of the provider
    /// * `catalog` - The catalog to be stored
    /// 
    /// # Returns
    /// * `Result<Catalog, ServiceError>` - Success or failure with detailed error
    pub async fn create_catalog(&self, provider_id: &str, catalog: Catalog) 
        -> Result<Catalog, ServiceError>;
    
    /// Retrieve a provider's catalog
    /// 
    /// # Parameters
    /// * `provider_id` - The unique identifier of the provider
    /// 
    /// # Returns
    /// * `Result<Catalog, ServiceError>` - Success or failure with detailed error
    pub async fn get_catalog(&self, provider_id: &str) 
        -> Result<Catalog, ServiceError>;
    
    /// Process item selection by a consumer
    /// 
    /// # Parameters
    /// * `provider_id` - The provider whose items are being selected
    /// * `items` - The list of item IDs being selected
    /// 
    /// # Returns
    /// * `Result<Vec<Item>, ServiceError>` - Selected items or failure with detailed error
    pub async fn select(&self, provider_id: &str, items: Vec<String>) 
        -> Result<Vec<Item>, ServiceError>;
    
    /// Generate a price quotation for selected items
    /// 
    /// # Parameters
    /// * `provider_id` - The provider of the selected items
    /// * `items` - The selected items for quotation
    /// 
    /// # Returns
    /// * `Result<Quotation, ServiceError>` - Price quotation or failure with detailed error
    pub async fn on_select(&self, provider_id: &str, items: Vec<Item>) 
        -> Result<Quotation, ServiceError>;
    
    /// Check if items in a catalog are available
    /// 
    /// # Parameters
    /// * `provider_id` - The provider's ID
    /// * `item_ids` - The item IDs to check availability for
    /// * `fulfillment_id` - Optional fulfillment ID for specific slot checking
    /// 
    /// # Returns
    /// * `Result<HashMap<String, bool>, ServiceError>` - Map of item IDs to availability
    pub async fn check_availability(
        &self, 
        provider_id: &str, 
        item_ids: Vec<String>,
        fulfillment_id: Option<String>
    ) -> Result<HashMap<String, bool>, ServiceError>;
}
```

### Dependencies

- **Storage**: Persistent storage layer for catalog data
- **FulfillmentService**: For checking appointment slot availability
- **ProviderService**: For accessing provider information and specialties

## Data Models

### Catalog

```rust
pub struct Catalog {
    /// Descriptive information about the catalog
    pub descriptor: Descriptor,
    
    /// Categories in the catalog
    pub categories: Vec<Category>,
    
    /// Fulfillments offered in the catalog
    pub fulfillments: Vec<Fulfillment>,
    
    /// Payments accepted for catalog items
    pub payments: Vec<Payment>,
    
    /// Locations where catalog items are available
    pub locations: Vec<Location>,
    
    /// Items in the catalog
    pub items: Vec<Item>,
    
    /// Expiration time for catalog freshness
    pub exp: Option<DateTime<Utc>>,
}
```

### Item

```rust
pub struct Item {
    /// Unique ID for the item
    pub id: String,
    
    /// Parent ID for hierarchical items
    pub parent_item_id: Option<String>,
    
    /// Descriptive information about the item
    pub descriptor: Descriptor,
    
    /// Price of the item
    pub price: Price,
    
    /// Category ID this item belongs to
    pub category_id: String,
    
    /// Fulfillment ID associated with this item
    pub fulfillment_id: String,
    
    /// Location ID where this item is available
    pub location_id: Option<String>,
    
    /// Time constraints for this item
    pub time: Option<DateTime<Utc>>,
    
    /// Flags for matching algorithms
    pub matched: Option<bool>,
    pub related: Option<bool>,
    pub recommended: Option<bool>,
    
    /// Additional metadata
    pub tags: Option<HashMap<String, String>>,
}
```

### Quotation

```rust
pub struct Quotation {
    /// Price details
    pub price: Price,
    
    /// Breakdown of price components
    pub breakup: Vec<QuotationBreakup>,
    
    /// Time-to-live for the quotation
    pub ttl: String,
}

pub struct QuotationBreakup {
    /// Title of the breakup component
    pub title: String,
    
    /// Price for this component
    pub price: Price,
}
```

## Implementation Details

### Catalog Management Flow

1. **Catalog Creation/Update**
   - Validate catalog structure against schema
   - Ensure all referenced entities (categories, fulfillments) exist
   - Update or create catalog in storage
   - Handle catalog versioning for updates

2. **Item Selection Processing**
   - Validate item existence in provider's catalog
   - Check item availability with FulfillmentService
   - Handle item dependencies and prerequisites
   - Return detailed item information for selected items

3. **Quotation Generation**
   - Calculate base prices for selected items
   - Apply business rules for discounts and bundling
   - Calculate taxes and additional charges
   - Create price breakup for transparency
   - Set appropriate TTL for quotation validity

### Pricing Strategies

The CatalogService implements multiple pricing strategies:

- **Fixed Pricing**: Standard listed price for items
- **Dynamic Pricing**: Prices that vary based on demand or time factors
- **Bundle Pricing**: Special pricing for service bundles
- **Promotional Pricing**: Time-limited discounts and offers
- **Personalized Pricing**: User-specific pricing based on history or agreements

### Catalog Versioning

- **Version Tracking**: Each catalog update receives a new version
- **Change Detection**: Track which items have changed between versions
- **Backward Compatibility**: Support for in-flight transactions with older catalog versions
- **Expiration Handling**: Automatic notification for soon-to-expire catalogs

### Error Handling

- **Validation Errors**: Return detailed catalog structure validation errors
- **Not Found Errors**: Handle missing items or categories gracefully
- **Pricing Errors**: Handle edge cases in price calculation
- **Availability Errors**: Provide clear feedback on unavailable items

## Configuration

The CatalogService is configurable through the following parameters:

- `catalog_ttl_default`: Default time-to-live for catalogs (default: 24h)
- `quotation_ttl_default`: Default time-to-live for quotations (default: 15m)
- `max_items_per_selection`: Maximum items that can be selected in one request (default: 20)
- `enable_dynamic_pricing`: Toggle for dynamic pricing capabilities (default: false)
- `price_precision`: Decimal precision for price calculations (default: 2)

## Usage Examples

### Creating a Catalog

```rust
let storage = Arc::new(MemoryStorage::new());
let catalog_service = CatalogService::new(storage);

let catalog = Catalog {
    descriptor: Descriptor {
        name: "Hospital ABC Services".to_string(),
        short_desc: Some("Medical services offered by Hospital ABC".to_string()),
        ..Default::default()
    },
    categories: vec![/* Category details */],
    fulfillments: vec![/* Fulfillment details */],
    payments: vec![/* Payment details */],
    locations: vec![/* Location details */],
    items: vec![
        Item {
            id: "item-1".to_string(),
            descriptor: Descriptor {
                name: "General Consultation".to_string(),
                ..Default::default()
            },
            price: Price {
                currency: "INR".to_string(),
                value: "500.0".to_string(),
                ..Default::default()
            },
            category_id: "cat-1".to_string(),
            fulfillment_id: "fulfillment-1".to_string(),
            ..Default::default()
        },
        // More items...
    ],
    ..Default::default()
};

let result = catalog_service.create_catalog("provider-123", catalog).await?;
```

### Processing Selection and Generating Quotation

```rust
// Process selection
let selected_items = catalog_service.select(
    "provider-123", 
    vec!["item-1".to_string(), "item-3".to_string()]
).await?;

// Generate quotation
let quotation = catalog_service.on_select("provider-123", selected_items).await?;

println!("Total price: {} {}", quotation.price.value, quotation.price.currency);
for component in quotation.breakup {
    println!("- {}: {}", component.title, component.price.value);
}
```

## Testing Strategy

1. **Unit Tests**
   - Test catalog validation
   - Test item selection logic
   - Test pricing calculation
   - Test error handling

2. **Integration Tests**
   - Test with mock provider catalogs
   - Verify correct pricing in various scenarios
   - Test availability checking integration with FulfillmentService

3. **Performance Tests**
   - Benchmark with large catalogs
   - Test concurrent selection handling
   - Measure quotation generation latency