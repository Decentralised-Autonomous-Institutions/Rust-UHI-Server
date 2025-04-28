use super::error::ServiceError;
use crate::models::order::{Order, OrderStatus};
use crate::storage::Storage;
use std::sync::Arc;

/// Order service for managing healthcare service bookings
pub struct OrderService {
    /// Storage implementation injected via constructor
    storage: Arc<dyn Storage>,
}

impl OrderService {
    /// Create a new order service with storage dependency
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
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

        // Return status information
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
        order.state = status.state;

        let updated = self.storage.update_order(order).await?;
        Ok(updated)
    }
}
