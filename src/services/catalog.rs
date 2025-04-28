use super::error::ServiceError;
use crate::models::catalog::{Catalog, Item, Quotation};
use crate::storage::Storage;
use std::sync::Arc;

/// Catalog service for managing healthcare service catalogs and selections
pub struct CatalogService {
    /// Storage implementation injected via constructor
    storage: Arc<dyn Storage>,
}

impl CatalogService {
    /// Create a new catalog service with storage dependency
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }

    /// Create a catalog for a provider
    pub async fn create_catalog(
        &self,
        provider_id: &str,
        catalog: Catalog,
    ) -> Result<Catalog, ServiceError> {
        let created = self.storage.create_catalog(provider_id, catalog).await?;
        Ok(created)
    }

    /// Get catalog for a provider
    pub async fn get_catalog(&self, provider_id: &str) -> Result<Catalog, ServiceError> {
        let catalog = self.storage.get_catalog(provider_id).await?;
        Ok(catalog)
    }

    /// Process item selection
    pub async fn select(
        &self,
        provider_id: &str,
        items: Vec<String>,
    ) -> Result<Vec<Item>, ServiceError> {
        // Business logic for item selection
        // For now, just get the catalog and filter the selected items
        let catalog = self.storage.get_catalog(provider_id).await?;

        // Find the selected items
        let selected_items: Vec<Item> = catalog
            .items
            .into_iter()
            .filter(|item| items.contains(&item.id))
            .collect();

        if selected_items.is_empty() {
            return Err(ServiceError::NotFound(
                "No matching items found".to_string(),
            ));
        }

        Ok(selected_items)
    }

    /// Process price quotation
    pub async fn on_select(
        &self,
        _provider_id: &str,
        items: Vec<Item>,
    ) -> Result<Quotation, ServiceError> {
        // Simplified implementation - in reality, would include pricing rules and discounts
        // This is placeholder logic to calculate a basic quote
        let total_price = items.iter().fold(0.0, |acc, item| {
            // Parse the price value from string to f64, or default to 0 if parsing fails
            let value = item.price.value.parse::<f64>().unwrap_or(0.0);
            acc + value
        });

        // Create a basic quotation
        let quotation = Quotation {
            price: crate::models::catalog::Price {
                currency: "INR".to_string(),
                value: total_price.to_string(),
                maximum_value: None,
            },
            breakup: vec![], // In a real implementation, would contain item breakdowns
            ttl: "PT1H".to_string(), // 1 hour validity
        };

        Ok(quotation)
    }
}
