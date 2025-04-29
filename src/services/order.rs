use super::error::ServiceError;
use super::fulfillment::FulfillmentService;
use crate::models::order::{Order, OrderStatus};
use crate::storage::Storage;
use std::collections::HashMap;
use std::sync::Arc;

/// Order service for managing healthcare service bookings
pub struct OrderService {
    /// Storage implementation injected via constructor
    storage: Arc<dyn Storage>,
    /// Fulfillment service for managing fulfillment details
    fulfillment_service: FulfillmentService,
}

impl OrderService {
    /// Create a new order service with storage dependency
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self {
            fulfillment_service: FulfillmentService::new(storage.clone()),
            storage,
        }
    }

    /// Create a new order
    pub async fn create_order(&self, order: Order) -> Result<Order, ServiceError> {
        let created = self.storage.create_order(order).await?;
        Ok(created)
    }

    /// Get an order by ID
    pub async fn get_order(&self, id: &str) -> Result<Order, ServiceError> {
        let order = self.storage.get_order(id).await?;
        Ok(order)
    }

    /// Update an order
    pub async fn update_order(&self, order: Order) -> Result<Order, ServiceError> {
        let updated = self.storage.update_order(order).await?;
        Ok(updated)
    }

    /// Initialize an order (init)
    pub async fn init(&self, order: Order) -> Result<Order, ServiceError> {
        // Business logic for order initialization
        // For simplified implementation, just create the order in storage
        let mut order_with_state = order;
        order_with_state.state = "INITIALIZED".to_string();

        let created = self.storage.create_order(order_with_state).await?;
        Ok(created)
    }

    /// Handle provider's response to order initialization (on_init)
    pub async fn on_init(
        &self,
        order_id: &str,
        provider_order: Order,
    ) -> Result<Order, ServiceError> {
        // Get the existing order
        let existing_order = self.storage.get_order(order_id).await?;

        // Validate provider ID matches
        if existing_order.provider.id != provider_order.provider.id {
            return Err(ServiceError::Validation("Provider ID mismatch".to_string()));
        }

        // Update with provider's order information
        let mut updated_order = existing_order;
        updated_order.quote = provider_order.quote;
        updated_order.payment = provider_order.payment;
        updated_order.state = "QUOTED".to_string();

        let updated = self.storage.update_order(updated_order).await?;
        Ok(updated)
    }

    /// Confirm an order
    pub async fn confirm(&self, order_id: &str) -> Result<Order, ServiceError> {
        // Get the existing order
        let mut order = self.storage.get_order(order_id).await?;

        // Update state to CONFIRMED
        order.state = "CONFIRMED".to_string();

        let updated = self.storage.update_order(order).await?;
        Ok(updated)
    }

    /// Handle provider's confirmation response
    pub async fn on_confirm(
        &self,
        order_id: &str,
        provider_order: Order,
    ) -> Result<Order, ServiceError> {
        // Get the existing order
        let existing_order = self.storage.get_order(order_id).await?;

        // Validate provider ID matches
        if existing_order.provider.id != provider_order.provider.id {
            return Err(ServiceError::Validation("Provider ID mismatch".to_string()));
        }

        // Update with provider's confirmation information
        let mut updated_order = existing_order;
        updated_order.state = provider_order.state;

        let updated = self.storage.update_order(updated_order).await?;
        Ok(updated)
    }

    /// Get order status
    pub async fn status(&self, order_id: &str) -> Result<OrderStatus, ServiceError> {
        // Get the order
        let order = self.storage.get_order(order_id).await?;

        // If the order has a fulfillment ID, check the fulfillment status
        if !order.fulfillment.id.is_empty() {
            let fulfillment_id = &order.fulfillment.id;
            // Get fulfillment status from FulfillmentService
            match self.fulfillment_service.get_fulfillment(fulfillment_id).await {
                Ok(fulfillment) => {
                    // If fulfillment has state, use it to update order status
                    if let Some(state) = &fulfillment.state {
                        // Map fulfillment state to order state
                        let order_state = match state.descriptor.as_str() {
                            "SCHEDULED" => "CONFIRMED",
                            "WAITING" => "FULFILLMENT_PENDING",
                            "IN_PROGRESS" => "IN_PROGRESS",
                            "COMPLETED" => "COMPLETED",
                            "CANCELLED" => "CANCELLED",
                            "NO_SHOW" => "NO_SHOW",
                            "RESCHEDULED" => "RESCHEDULED",
                            _ => &order.state, // Keep existing state if unknown
                        };

                        // If state doesn't match the order's current state, update the order
                        if order_state != order.state {
                            let mut updated_order = order.clone();
                            updated_order.state = order_state.to_string();
                            // Update the order in storage
                            let _ = self.storage.update_order(updated_order).await?;
                        }

                        // Return the mapped status
                        return Ok(OrderStatus {
                            state: order_state.to_string(),
                            updated_at: state.updated_at,
                        });
                    }
                }
                Err(_) => {
                    // If we can't get the fulfillment, just use the order's state
                    // This could happen if the fulfillment was deleted or is invalid
                }
            }
        }

        // Return status information from the order if no fulfillment or fulfillment not found
        let status = OrderStatus {
            state: order.state,
            updated_at: order.updated_at,
        };

        Ok(status)
    }

    /// Handle provider's status response
    pub async fn on_status(
        &self,
        order_id: &str,
        status: OrderStatus,
    ) -> Result<Order, ServiceError> {
        // Get the existing order
        let mut order = self.storage.get_order(order_id).await?;

        // Update state with provider's status
        order.state = status.state.clone();

        // If there's a fulfillment ID associated with this order, update its state too
        if !order.fulfillment.id.is_empty() {
            let fulfillment_id = &order.fulfillment.id;
            // Map order state to fulfillment state
            let fulfillment_state = match status.state.as_str() {
                "CONFIRMED" => "SCHEDULED",
                "IN_PROGRESS" => "IN_PROGRESS",
                "COMPLETED" => "COMPLETED",
                "CANCELLED" => "CANCELLED",
                "NO_SHOW" => "NO_SHOW",
                "RESCHEDULED" => "RESCHEDULED",
                _ => return Err(ServiceError::Validation(format!(
                    "Unsupported order state for fulfillment mapping: {}",
                    status.state
                ))),
            };

            // Update the fulfillment state
            let context = HashMap::from([
                ("source".to_string(), "order_status_update".to_string()),
                ("order_id".to_string(), order_id.to_string()),
            ]);

            let _ = self.fulfillment_service
                .update_state(fulfillment_id, fulfillment_state, Some(context))
                .await;
            // We don't propagate errors here, as we want to continue updating the order
            // even if the fulfillment update fails
        }

        let updated = self.storage.update_order(order).await?;
        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::billing::{Address, Billing};
    use crate::models::fulfillment::{Agent, Customer, Fulfillment, Person, State, Time, TimeSlot};
    use crate::models::order::ProviderSummary;
    use crate::storage::memory::MemoryStorage;
    use chrono::Utc;
    use std::collections::HashMap;
    use uuid::Uuid;

    // Helper function to create a test order
    fn create_test_order(id: &str, provider_id: &str, fulfillment_id: &str) -> Order {
        Order {
            id: id.to_string(),
            provider: ProviderSummary {
                id: provider_id.to_string(),
                descriptor: "Test Provider".to_string(),
                categories: vec!["Healthcare".to_string()],
            },
            items: Vec::new(),
            billing: Billing {
                name: "John Doe".to_string(),
                organization: None,
                address: Address {
                    door: Some("101".to_string()),
                    building: Some("Test Building".to_string()),
                    street: Some("Test Street".to_string()),
                    locality: Some("Test Locality".to_string()),
                    city: "Test City".to_string(),
                    state: "Test State".to_string(),
                    country: "Test Country".to_string(),
                    area_code: "12345".to_string(),
                },
                email: Some("john@example.com".to_string()),
                phone: "1234567890".to_string(),
                tax_number: None,
            },
            fulfillment: Fulfillment {
                id: fulfillment_id.to_string(),
                fulfillment_type: "Teleconsultation".to_string(),
                provider_id: provider_id.to_string(),
                agent: None,
                start: TimeSlot {
                    time: Time {
                        timestamp: Utc::now(),
                        label: None,
                    },
                    duration: Some(3600),
                },
                end: TimeSlot {
                    time: Time {
                        timestamp: Utc::now() + chrono::Duration::hours(1),
                        label: None,
                    },
                    duration: None,
                },
                customer: None,
                state: None,
                tags: HashMap::new(),
            },
            quote: None,
            payment: None,
            state: "INITIALIZED".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // Helper function to create a test fulfillment
    fn create_test_fulfillment(id: &str, provider_id: &str, state: &str) -> Fulfillment {
        Fulfillment {
            id: id.to_string(),
            fulfillment_type: "Teleconsultation".to_string(),
            provider_id: provider_id.to_string(),
            agent: Some(Agent {
                id: "agent-1".to_string(),
                name: "Dr. Smith".to_string(),
                gender: Some("male".to_string()),
                image: None,
                tags: HashMap::new(),
            }),
            start: TimeSlot {
                time: Time {
                    timestamp: Utc::now(),
                    label: None,
                },
                duration: Some(3600),
            },
            end: TimeSlot {
                time: Time {
                    timestamp: Utc::now() + chrono::Duration::hours(1),
                    label: None,
                },
                duration: None,
            },
            customer: Some(Customer {
                person: Person {
                    name: "John Doe".to_string(),
                    image: None,
                    gender: Some("male".to_string()),
                    creds: None,
                    tags: None,
                },
                contact: HashMap::from([
                    ("phone".to_string(), "1234567890".to_string()),
                    ("email".to_string(), "john@example.com".to_string()),
                ]),
            }),
            state: Some(State {
                descriptor: state.to_string(),
                updated_at: Utc::now(),
            }),
            tags: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_status_checking_flow() {
        // Create memory storage
        let storage = Arc::new(MemoryStorage::new());
        
        // Create a provider for testing
        let provider_id = "provider-1";
        let provider = crate::models::provider::Provider {
            id: provider_id.to_string(),
            descriptor: crate::models::provider::Descriptor {
                name: "Test Provider".to_string(),
                short_desc: None,
                long_desc: None,
                images: None,
            },
            categories: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let _ = storage.create_provider(provider).await.unwrap();
        
        // Create OrderService and FulfillmentService
        let order_service = OrderService::new(storage.clone());
        let fulfillment_service = FulfillmentService::new(storage.clone());
        
        // Create a fulfillment
        let fulfillment_id = "fulfillment-1";
        let fulfillment = create_test_fulfillment(fulfillment_id, provider_id, "SCHEDULED");
        let created_fulfillment = fulfillment_service.create_fulfillment(fulfillment).await.unwrap();
        
        // Create an order that references the fulfillment
        let order_id = "order-1";
        let order = create_test_order(order_id, provider_id, fulfillment_id);
        let created_order = order_service.create_order(order).await.unwrap();
        
        // Check initial order status
        let initial_status = order_service.status(order_id).await.unwrap();
        assert_eq!(initial_status.state, "CONFIRMED"); // Should map from SCHEDULED to CONFIRMED
        
        // Update fulfillment state to IN_PROGRESS
        let updated_fulfillment = fulfillment_service
            .update_state(fulfillment_id, "IN_PROGRESS", None)
            .await
            .unwrap();
        assert_eq!(updated_fulfillment.state.unwrap().descriptor, "IN_PROGRESS");
        
        // Check that order status reflects the fulfillment status
        let updated_status = order_service.status(order_id).await.unwrap();
        assert_eq!(updated_status.state, "IN_PROGRESS");
        
        // Test on_status handler with a status update
        let completed_status = OrderStatus {
            state: "COMPLETED".to_string(),
            updated_at: Utc::now(),
        };
        
        let updated_order = order_service.on_status(order_id, completed_status).await.unwrap();
        assert_eq!(updated_order.state, "COMPLETED");
        
        // Verify that fulfillment was also updated
        let final_fulfillment = fulfillment_service.get_fulfillment(fulfillment_id).await.unwrap();
        assert_eq!(final_fulfillment.state.unwrap().descriptor, "COMPLETED");
    }
}
