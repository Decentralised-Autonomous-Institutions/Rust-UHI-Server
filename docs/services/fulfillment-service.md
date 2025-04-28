# FulfillmentService Technical Design

## Overview

The FulfillmentService is a critical component responsible for managing healthcare service delivery in the UHI Protocol implementation. It handles scheduling, availability management, and all aspects of service fulfillment. This service bridges the gap between healthcare providers and patients, ensuring that services are delivered according to agreed terms and conditions.

## Responsibilities

- Manage healthcare service fulfillment schedules and time slots
- Track availability of healthcare professionals and resources
- Handle appointment booking, rescheduling, and cancellation
- Process fulfillment state transitions (scheduled, in-progress, completed)
- Coordinate between patients and healthcare providers during service delivery
- Manage fulfillment types (teleconsultation, in-person, home visit, etc.)
- Ensure fulfillment compliance with order terms
- Track fulfillment history and provide audit trails

## Interfaces

### Public Methods

```rust
pub struct FulfillmentService {
    storage: Arc<dyn Storage>,
    provider_service: ProviderService,
}

impl FulfillmentService {
    /// Create a new fulfillment service with injected dependencies
    pub fn new(storage: Arc<dyn Storage>) -> Self;
    
    /// Create a new fulfillment
    /// 
    /// # Parameters
    /// * `fulfillment` - The fulfillment to create
    /// 
    /// # Returns
    /// * `Result<Fulfillment, ServiceError>` - Created fulfillment or error
    pub async fn create_fulfillment(&self, fulfillment: Fulfillment) 
        -> Result<Fulfillment, ServiceError>;
    
    /// Get a fulfillment by ID
    /// 
    /// # Parameters
    /// * `id` - The fulfillment ID
    /// 
    /// # Returns
    /// * `Result<Fulfillment, ServiceError>` - Fulfillment or error if not found
    pub async fn get_fulfillment(&self, id: &str) -> Result<Fulfillment, ServiceError>;
    
    /// Update an existing fulfillment
    /// 
    /// # Parameters
    /// * `fulfillment` - The updated fulfillment
    /// 
    /// # Returns
    /// * `Result<Fulfillment, ServiceError>` - Updated fulfillment or error
    pub async fn update_fulfillment(&self, fulfillment: Fulfillment) 
        -> Result<Fulfillment, ServiceError>;
    
    /// List fulfillments by provider
    /// 
    /// # Parameters
    /// * `provider_id` - The provider's ID
    /// 
    /// # Returns
    /// * `Result<Vec<Fulfillment>, ServiceError>` - List of fulfillments or error
    pub async fn list_fulfillments_by_provider(&self, provider_id: &str) 
        -> Result<Vec<Fulfillment>, ServiceError>;
    
    /// Check availability for a time slot
    /// 
    /// # Parameters
    /// * `provider_id` - The ID of the provider to check availability for
    /// * `requested_time` - The requested start time for the appointment
    /// * `duration_seconds` - The duration of the appointment in seconds
    /// 
    /// # Returns
    /// * `Result<bool, ServiceError>` - Whether the time slot is available
    pub async fn check_availability(
        &self, 
        provider_id: &str, 
        requested_time: &DateTime<Utc>,
        duration_seconds: i64
    ) -> Result<bool, ServiceError>;
    
    /// Find available time slots
    /// 
    /// # Parameters
    /// * `provider_id` - The provider ID
    /// * `start_date` - The start date to search from
    /// * `end_date` - The end date to search until
    /// * `duration_seconds` - The required duration in seconds
    /// 
    /// # Returns
    /// * `Result<Vec<TimeSlot>, ServiceError>` - List of available time slots
    pub async fn find_available_slots(
        &self,
        provider_id: &str,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
        duration_seconds: i64
    ) -> Result<Vec<TimeSlot>, ServiceError>;
    
    /// Update fulfillment state
    /// 
    /// # Parameters
    /// * `fulfillment_id` - The fulfillment ID
    /// * `state` - The new state
    /// * `context` - Optional state change context information
    /// 
    /// # Returns
    /// * `Result<Fulfillment, ServiceError>` - Updated fulfillment or error
    pub async fn update_state(
        &self,
        fulfillment_id: &str,
        state: &str,
        context: Option<HashMap<String, String>>
    ) -> Result<Fulfillment, ServiceError>;
}
```

### Dependencies

- **Storage**: Persistent storage layer for fulfillment data
- **ProviderService**: For provider availability and working hours information

## Data Models

### Fulfillment

```rust
pub struct Fulfillment {
    /// Unique ID for the fulfillment
    pub id: String,
    
    /// Type of fulfillment (e.g., "Teleconsultation", "Physical")
    pub fulfillment_type: String,
    
    /// ID of the provider delivering the service
    pub provider_id: String,
    
    /// Agent delivering the service
    pub agent: Option<Agent>,
    
    /// Start time slot
    pub start: TimeSlot,
    
    /// End time slot
    pub end: TimeSlot,
    
    /// Customer receiving the service
    pub customer: Option<Customer>,
    
    /// Current state of the fulfillment
    pub state: Option<State>,
    
    /// Additional metadata about the fulfillment
    pub tags: HashMap<String, String>,
}
```

### Agent

```rust
pub struct Agent {
    /// ID of the agent
    pub id: String,
    
    /// Name of the agent
    pub name: String,
    
    /// Gender of the agent
    pub gender: Option<String>,
    
    /// Image URL of the agent
    pub image: Option<String>,
    
    /// Additional details about the agent
    pub tags: HashMap<String, String>,
}
```

### TimeSlot

```rust
pub struct TimeSlot {
    /// Time information
    pub time: Time,
    
    /// Duration of the slot in seconds
    pub duration: Option<i64>,
}

pub struct Time {
    /// Timestamp in ISO format
    pub timestamp: DateTime<Utc>,
    
    /// Label for the time (e.g., "start", "end")
    pub label: Option<String>,
}
```

### State

```rust
pub struct State {
    /// Current state of the fulfillment
    pub descriptor: String,
    
    /// Updated time for this state
    pub updated_at: DateTime<Utc>,
}
```

## Implementation Details

### Fulfillment State Machine

The FulfillmentService implements a state machine to manage fulfillment states:

1. **Fulfillment States**:
   - `SCHEDULED`: Appointment has been booked but not started
   - `WAITING`: Patient is in waiting room/queue
   - `IN_PROGRESS`: Service delivery has started
   - `COMPLETED`: Service has been successfully delivered
   - `CANCELLED`: Appointment was cancelled
   - `NO_SHOW`: Patient didn't show up for appointment
   - `RESCHEDULED`: Appointment was rescheduled to new time

2. **State Transitions**:
   - Each state transition is validated against allowed paths
   - Required fields and conditions for each state are enforced
   - State change timestamps are recorded for audit
   - Appropriate notifications are generated for state changes

### Availability Management

The FulfillmentService implements sophisticated availability checking:

1. **Provider Working Hours**:
   - Integrates with ProviderService to check basic working hours
   - Handles different schedules for weekdays, weekends, and holidays
   - Respects provider break times and blocked periods

2. **Conflict Detection**:
   - Checks for overlapping appointments with existing fulfillments
   - Handles buffer times between appointments
   - Considers travel time for home visits or multi-location providers

3. **Resource Allocation**:
   - Tracks resource requirements (rooms, equipment) for in-person appointments
   - Manages concurrent appointment limits based on provider capacity
   - Handles specialized resource constraints (e.g., operating rooms)

### Slot Management Algorithms

The FulfillmentService implements efficient algorithms for:

1. **Slot Search**:
   - Finds available slots within a date range
   - Optimizes search performance with indexes and caching
   - Supports filtering by duration, provider specialties, etc.

2. **Slot Suggestion**:
   - Recommends optimal slots based on provider and patient preferences
   - Implements "next available" functionality
   - Supports recurring appointment scheduling

3. **Buffer Management**:
   - Handles preparation and cleanup time between appointments
   - Adjusts buffers based on appointment type and provider preferences
   - Manages transition time between virtual and physical appointments

### Error Handling

- **Validation Errors**: Detailed errors for invalid fulfillment structures
- **Availability Errors**: Clear messaging for unavailable time slots
- **State Transition Errors**: Specific errors for invalid state changes
- **Resource Conflicts**: Details on conflicting resources or appointments
- **Provider Unavailability**: Handling unexpected provider unavailability

## Configuration

The FulfillmentService is configurable through the following parameters:

- `default_appointment_duration`: Default duration for appointments (default: 15m)
- `minimum_reschedule_notice`: Minimum notice required for rescheduling (default: 6h)
- `cancellation_window`: Time window when cancellation is allowed (default: 24h)
- `buffer_between_appointments`: Default buffer time between appointments (default: 5m)
- `availability_search_limit`: Maximum days to search for availability (default: 30 days)

## Usage Examples

### Creating a Fulfillment

```rust
let storage = Arc::new(MemoryStorage::new());
let fulfillment_service = FulfillmentService::new(storage);

let now = Utc::now();
let start_time = now + Duration::hours(24); // Tomorrow
let end_time = start_time + Duration::minutes(30);

let fulfillment = Fulfillment {
    id: Uuid::new_v4().to_string(),
    fulfillment_type: "Teleconsultation".to_string(),
    provider_id: "provider-123".to_string(),
    agent: Some(Agent {
        id: "doctor-456".to_string(),
        name: "Dr. Jane Smith".to_string(),
        gender: Some("female".to_string()),
        image: Some("https://example.com/doctors/jane-smith.jpg".to_string()),
        tags: HashMap::from([
            ("speciality".to_string(), "Cardiology".to_string()),
            ("experience".to_string(), "10".to_string()),
        ]),
    }),
    start: TimeSlot {
        time: Time {
            timestamp: start_time,
            label: Some("start".to_string()),
        },
        duration: Some(1800), // 30 minutes
    },
    end: TimeSlot {
        time: Time {
            timestamp: end_time,
            label: Some("end".to_string()),
        },
        duration: None,
    },
    customer: Some(Customer {
        person: Person {
            name: "John Doe".to_string(),
            image: None,
            gender: Some("male".to_string()),
            cred: None,
            tags: None,
        },
        contact: HashMap::from([
            ("phone".to_string(), "1234567890".to_string()),
            ("email".to_string(), "john@example.com".to_string()),
        ]),
    }),
    state: Some(State {
        descriptor: "SCHEDULED".to_string(),
        updated_at: Utc::now(),
    }),
    tags: HashMap::new(),
};

let created_fulfillment = fulfillment_service.create_fulfillment(fulfillment).await?;
```

### Checking Availability

```rust
let provider_id = "provider-123";
let requested_time = Utc::now() + Duration::days(1); // Tomorrow
let duration = 1800; // 30 minutes

let is_available = fulfillment_service.check_availability(
    provider_id,
    &requested_time,
    duration
).await?;

if is_available {
    println!("The requested time slot is available");
} else {
    println!("The requested time slot is not available");
    
    // Find alternative slots
    let start_date = Utc::now() + Duration::days(1);
    let end_date = Utc::now() + Duration::days(7);
    
    let available_slots = fulfillment_service.find_available_slots(
        provider_id,
        &start_date,
        &end_date,
        duration
    ).await?;
    
    println!("Found {} alternative slots", available_slots.len());
    for slot in available_slots {
        println!("Available at: {}", slot.time.timestamp);
    }
}
```

### Updating Fulfillment State

```rust
// Update state to IN_PROGRESS
let updated_fulfillment = fulfillment_service.update_state(
    "fulfillment-123",
    "IN_PROGRESS",
    Some(HashMap::from([
        ("started_by".to_string(), "provider".to_string()),
        ("meeting_url".to_string(), "https://meeting.example.com/123".to_string()),
    ])),
).await?;

println!("Fulfillment now in state: {}", updated_fulfillment.state.unwrap().descriptor);
```

## Testing Strategy

1. **Unit Tests**
   - Test time slot availability checking
   - Test state transition validation
   - Test buffer and conflict detection
   - Test error handling scenarios

2. **Integration Tests**
   - Test with mock provider schedules
   - Verify correct interaction with ProviderService
   - Test full lifecycle from scheduling to completion

3. **Performance Tests**
   - Benchmark availability checking algorithms
   - Test concurrent booking performance
   - Measure slot search performance with large datasets