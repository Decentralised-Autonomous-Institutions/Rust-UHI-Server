# ProviderService Technical Design

## Overview

The ProviderService is responsible for managing healthcare service providers within the UHI Protocol implementation. It serves as the core service for provider registration, profile management, and availability tracking. This service enables discovery of healthcare providers and ensures their information is accurate and up-to-date for patient access.

## Responsibilities

- Manage healthcare provider registration and profile information
- Track provider specialties, qualifications, and services offered
- Handle provider location and service area management
- Monitor provider availability and working hours
- Validate provider credentials and certifications
- Maintain provider ratings and reviews
- Support provider search and filtering
- Manage provider onboarding and offboarding processes

## Interfaces

### Public Methods

```rust
pub struct ProviderService {
    storage: Arc<dyn Storage>,
}

impl ProviderService {
    /// Create a new provider service with injected storage
    pub fn new(storage: Arc<dyn Storage>) -> Self;
    
    /// Register a new provider
    /// 
    /// # Parameters
    /// * `provider` - The provider information to register
    /// 
    /// # Returns
    /// * `Result<Provider, ServiceError>` - Registered provider or error
    pub async fn register_provider(&self, provider: Provider) -> Result<Provider, ServiceError>;
    
    /// Get a provider by ID
    /// 
    /// # Parameters
    /// * `id` - The provider ID
    /// 
    /// # Returns
    /// * `Result<Provider, ServiceError>` - Provider or error if not found
    pub async fn get_provider(&self, id: &str) -> Result<Provider, ServiceError>;
    
    /// Update a provider profile
    /// 
    /// # Parameters
    /// * `provider` - The updated provider information
    /// 
    /// # Returns
    /// * `Result<Provider, ServiceError>` - Updated provider or error
    pub async fn update_provider(&self, provider: Provider) -> Result<Provider, ServiceError>;
    
    /// Delete a provider
    /// 
    /// # Parameters
    /// * `id` - The provider ID
    /// 
    /// # Returns
    /// * `Result<(), ServiceError>` - Success or error
    pub async fn delete_provider(&self, id: &str) -> Result<(), ServiceError>;
    
    /// List all providers
    /// 
    /// # Returns
    /// * `Result<Vec<Provider>, ServiceError>` - List of providers or error
    pub async fn list_providers(&self) -> Result<Vec<Provider>, ServiceError>;
    
    /// Check if a provider is available at a specific time
    /// 
    /// # Parameters
    /// * `provider_id` - The ID of the provider to check
    /// * `requested_time` - The time to check availability for
    /// 
    /// # Returns
    /// * `Result<bool, ServiceError>` - Whether the provider is available
    pub async fn check_provider_availability(
        &self,
        provider_id: &str,
        requested_time: &DateTime<Utc>
    ) -> Result<bool, ServiceError>;
    
    /// Get provider working hours
    /// 
    /// # Parameters
    /// * `provider_id` - The ID of the provider
    /// 
    /// # Returns
    /// * `Result<WorkingHours, ServiceError>` - Provider working hours or error
    pub async fn get_working_hours(
        &self,
        provider_id: &str
    ) -> Result<WorkingHours, ServiceError>;
    
    /// Find providers by specialty
    /// 
    /// # Parameters
    /// * `specialty` - The specialty code
    /// 
    /// # Returns
    /// * `Result<Vec<Provider>, ServiceError>` - Matching providers or error
    pub async fn find_providers_by_specialty(
        &self,
        specialty: &str
    ) -> Result<Vec<Provider>, ServiceError>;
    
    /// Find providers by location
    /// 
    /// # Parameters
    /// * `location` - The location coordinates
    /// * `radius_km` - The search radius in kilometers
    /// 
    /// # Returns
    /// * `Result<Vec<Provider>, ServiceError>` - Matching providers or error
    pub async fn find_providers_by_location(
        &self,
        location: &str,
        radius_km: f64
    ) -> Result<Vec<Provider>, ServiceError>;
}
```

### Dependencies

- **Storage**: Persistent storage layer for provider data

## Data Models

### Provider

```rust
pub struct Provider {
    /// Unique ID for the provider
    pub id: String,
    
    /// Descriptive information about the provider
    pub descriptor: Descriptor,
    
    /// Categories of services offered by the provider
    pub categories: Vec<Category>,
    
    /// Time when the provider was created
    pub created_at: DateTime<Utc>,
    
    /// Time when the provider was last updated
    pub updated_at: DateTime<Utc>,
}
```

### Descriptor

```rust
pub struct Descriptor {
    /// Name of the provider
    pub name: String,
    
    /// Short description of the provider
    pub short_desc: Option<String>,
    
    /// Long description of the provider
    pub long_desc: Option<String>,
    
    /// List of image URLs representing the provider
    pub images: Option<Vec<String>>,
}
```

### Category

```rust
pub struct Category {
    /// Unique ID for the category
    pub id: String,
    
    /// Category descriptor
    pub descriptor: Descriptor,
    
    /// Time when the category was last updated
    pub time: Option<DateTime<Utc>>,
    
    /// Tags associated with the category
    pub tags: Option<HashMap<String, String>>,
}
```

### Location

```rust
pub struct Location {
    /// Unique ID for the location
    pub id: String,
    
    /// Descriptive information about the location
    pub descriptor: Descriptor,
    
    /// GPS coordinates of the location
    pub gps: String,
    
    /// Full address
    pub address: Option<String>,
    
    /// City
    pub city: Option<String>,
    
    /// State/province
    pub state: Option<String>,
    
    /// Country
    pub country: Option<String>,
    
    /// Area code or pincode
    pub area_code: Option<String>,
}
```

### WorkingHours

```rust
pub struct WorkingHours {
    /// Provider ID
    pub provider_id: String,
    
    /// Regular working days and hours
    pub regular_hours: HashMap<String, Vec<TimeRange>>,
    
    /// Exception dates (holidays, special hours)
    pub exceptions: HashMap<String, Vec<TimeRange>>,
    
    /// Regular break times
    pub breaks: Option<HashMap<String, Vec<TimeRange>>>,
}

pub struct TimeRange {
    /// Start time in HH:MM format
    pub start: String,
    
    /// End time in HH:MM format
    pub end: String,
}
```

## Implementation Details

### Provider Management

1. **Provider Registration**:
   - Validate required provider information
   - Generate provider ID if not provided
   - Validate credentials and certifications (if applicable)
   - Store provider profile in database
   - Initialize default working hours

2. **Profile Management**:
   - Support partial updates to provider information
   - Track update history for audit purposes
   - Validate changes against business rules
   - Handle service category updates

3. **Provider Discovery**:
   - Implement search indexing for efficient provider lookup
   - Support filtering by specialty, location, rating, etc.
   - Implement geo-spatial search for location-based queries
   - Support provider sorting by relevance, distance, rating

### Availability Management

1. **Working Hours**:
   - Support complex working hour schedules (different for each day)
   - Handle recurring breaks and exceptions (holidays, special hours)
   - Support timezone handling for international providers
   - Handle DST transitions correctly

2. **Availability Checking**:
   - Check if requested time falls within working hours
   - Consider day of week and exceptions (holidays)
   - Handle buffer times and breaks
   - Support availability checking for specific services

3. **Provider Capacity**:
   - Track maximum concurrent appointments
   - Consider provider capacity for different service types
   - Support dynamic availability based on booked appointments

### Error Handling

- **Validation Errors**: Detailed errors for invalid provider information
- **Not Found Errors**: Clear messaging for missing providers
- **Availability Errors**: Specific errors for availability-related issues
- **Credential Errors**: Errors related to certification verification
- **Location Errors**: Errors for invalid geographic coordinates or addresses

## Configuration

The ProviderService is configurable through the following parameters:

- `geo_search_default_radius`: Default radius for location searches (default: 10km)
- `provider_cache_ttl`: Cache time for provider profiles (default: 15m)
- `working_hours_default`: Default working hours template (default: Mon-Fri 9am-5pm)
- `credential_validation_enabled`: Toggle for credential validation (default: true)
- `max_services_per_provider`: Maximum number of services per provider (default: 100)

## Usage Examples

### Registering a Provider

```rust
let storage = Arc::new(MemoryStorage::new());
let provider_service = ProviderService::new(storage);

let provider = Provider {
    id: "".to_string(), // Will be generated
    descriptor: Descriptor {
        name: "City General Hospital".to_string(),
        short_desc: Some("Multi-specialty healthcare provider".to_string()),
        long_desc: Some("City General Hospital is a leading multi-specialty healthcare provider offering comprehensive medical services across all major specialties.".to_string()),
        images: Some(vec!["https://example.com/images/city-general.jpg".to_string()]),
    },
    categories: vec![
        Category {
            id: "cat-1".to_string(),
            descriptor: Descriptor {
                name: "Cardiology".to_string(),
                short_desc: Some("Heart care services".to_string()),
                long_desc: None,
                images: None,
            },
            time: None,
            tags: Some(HashMap::from([
                ("code".to_string(), "CARDIOLOGY".to_string()),
                ("specialization".to_string(), "adult".to_string()),
            ])),
        },
        Category {
            id: "cat-2".to_string(),
            descriptor: Descriptor {
                name: "Orthopedics".to_string(),
                short_desc: Some("Bone and joint care".to_string()),
                long_desc: None,
                images: None,
            },
            time: None,
            tags: Some(HashMap::from([
                ("code".to_string(), "ORTHOPEDICS".to_string()),
            ])),
        },
    ],
    created_at: Utc::now(),
    updated_at: Utc::now(),
};

let registered_provider = provider_service.register_provider(provider).await?;
println!("Provider registered with ID: {}", registered_provider.id);
```

### Checking Provider Availability

```rust
let provider_id = "provider-123";
let requested_time = Utc::now() + Duration::days(1); // Tomorrow

let is_available = provider_service.check_provider_availability(
    provider_id,
    &requested_time
).await?;

if is_available {
    println!("Provider is available at the requested time");
} else {
    println!("Provider is not available at the requested time");
    
    // Get provider's working hours for context
    let working_hours = provider_service.get_working_hours(provider_id).await?;
    
    println!("Provider's working hours:");
    for (day, hours) in &working_hours.regular_hours {
        for range in hours {
            println!("{}: {} - {}", day, range.start, range.end);
        }
    }
}
```

### Finding Providers by Specialty and Location

```rust
// Find cardiologists
let cardiologists = provider_service.find_providers_by_specialty("CARDIOLOGY").await?;
println!("Found {} cardiologists", cardiologists.len());

// Find providers near a location
let location = "12.9716,77.5946"; // Bangalore coordinates
let radius = 5.0; // 5 km radius

let nearby_providers = provider_service.find_providers_by_location(location, radius).await?;
println!("Found {} providers within {}km", nearby_providers.len(), radius);
```

## Testing Strategy

1. **Unit Tests**
   - Test provider validation
   - Test working hours and availability logic
   - Test geo-location search algorithms
   - Test error handling scenarios

2. **Integration Tests**
   - Test provider lifecycle from creation to deletion
   - Verify correct storage interaction
   - Test complex availability patterns

3. **Performance Tests**
   - Benchmark provider search operations
   - Test provider caching effectiveness
   - Measure geo-spatial query performance