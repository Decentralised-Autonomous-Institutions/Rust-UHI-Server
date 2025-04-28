use super::error::ServiceError;
use super::fulfillment::FulfillmentService;
use crate::models::catalog::{Catalog, Item, Price, Quotation, QuotationBreakup};
use crate::storage::Storage;
use chrono::{DateTime, Duration, Utc};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

/// Configuration parameters for CatalogService
pub struct CatalogServiceConfig {
    /// Default time-to-live for catalogs (in hours)
    pub catalog_ttl_default: u64,
    /// Default time-to-live for quotations (in minutes)
    pub quotation_ttl_default: u64,
    /// Maximum items that can be selected in one request
    pub max_items_per_selection: usize,
    /// Toggle for dynamic pricing capabilities
    pub enable_dynamic_pricing: bool,
    /// Decimal precision for price calculations
    pub price_precision: u8,
}

impl Default for CatalogServiceConfig {
    fn default() -> Self {
        Self {
            catalog_ttl_default: 24,
            quotation_ttl_default: 15,
            max_items_per_selection: 20,
            enable_dynamic_pricing: false,
            price_precision: 2,
        }
    }
}

/// Catalog service for managing healthcare service catalogs and selections
pub struct CatalogService {
    /// Storage implementation injected via constructor
    storage: Arc<dyn Storage>,
    /// Fulfillment service for availability checking
    fulfillment_service: FulfillmentService,
    /// Configuration parameters
    config: CatalogServiceConfig,
}

impl CatalogService {
    /// Create a new catalog service with storage dependency
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        let fulfillment_service = FulfillmentService::new(storage.clone());
        Self {
            storage,
            fulfillment_service,
            config: CatalogServiceConfig::default(),
        }
    }

    /// Create a new catalog service with custom configuration
    pub fn with_config(storage: Arc<dyn Storage>, config: CatalogServiceConfig) -> Self {
        let fulfillment_service = FulfillmentService::new(storage.clone());
        Self {
            storage,
            fulfillment_service,
            config,
        }
    }

    /// Create a catalog for a provider
    pub async fn create_catalog(
        &self,
        provider_id: &str,
        catalog: Catalog,
    ) -> Result<Catalog, ServiceError> {
        // Validate the catalog structure
        self.validate_catalog(&catalog)?;

        // Set expiration time if not provided
        let mut catalog_to_save = catalog.clone();
        if catalog_to_save.exp.is_none() {
            catalog_to_save.exp = Some(
                Utc::now() + Duration::hours(self.config.catalog_ttl_default as i64),
            );
        }

        // Create in storage
        let created = self.storage.create_catalog(provider_id, catalog_to_save).await?;
        Ok(created)
    }

    /// Get catalog for a provider
    pub async fn get_catalog(&self, provider_id: &str) -> Result<Catalog, ServiceError> {
        let catalog = self.storage.get_catalog(provider_id).await?;
        
        // Check if catalog has expired
        if let Some(exp) = catalog.exp {
            if exp < Utc::now() {
                return Err(ServiceError::BusinessLogic(format!(
                    "Catalog for provider {} has expired", 
                    provider_id
                )));
            }
        }
        
        Ok(catalog)
    }

    /// Update a provider's catalog
    pub async fn update_catalog(
        &self,
        provider_id: &str,
        catalog: Catalog,
    ) -> Result<Catalog, ServiceError> {
        // Validate the catalog
        self.validate_catalog(&catalog)?;

        // Ensure the catalog exists first
        let _ = self.storage.get_catalog(provider_id).await?;

        // Set expiration time if not provided
        let mut catalog_to_save = catalog.clone();
        if catalog_to_save.exp.is_none() {
            catalog_to_save.exp = Some(
                Utc::now() + Duration::hours(self.config.catalog_ttl_default as i64),
            );
        }

        // Update in storage
        let updated = self.storage.update_catalog(provider_id, catalog_to_save).await?;
        Ok(updated)
    }

    /// Process item selection
    pub async fn select(
        &self,
        provider_id: &str,
        items: Vec<String>,
    ) -> Result<Vec<Item>, ServiceError> {
        // Validate number of items
        if items.is_empty() {
            return Err(ServiceError::Validation("No items selected".to_string()));
        }

        if items.len() > self.config.max_items_per_selection {
            return Err(ServiceError::Validation(format!(
                "Cannot select more than {} items at once",
                self.config.max_items_per_selection
            )));
        }

        // Get the catalog
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

        // Track this selection in a transaction for later reference
        let transaction_id = uuid::Uuid::new_v4().to_string();
        let transaction_data = json!({
            "provider_id": provider_id,
            "selected_items": selected_items,
            "timestamp": Utc::now(),
            "status": "SELECTED"
        });
        self.storage.record_transaction(&transaction_id, transaction_data).await?;

        Ok(selected_items)
    }

    /// Process price quotation
    pub async fn on_select(
        &self,
        provider_id: &str,
        items: Vec<Item>,
    ) -> Result<Quotation, ServiceError> {
        // Validate provider exists
        let _ = self.storage.get_provider(provider_id).await?;

        // Check availability for items that require specific fulfillment slots
        let availability = self.check_item_availability(provider_id, &items).await?;
        
        // If any item is unavailable, return an error
        for (item_id, is_available) in &availability {
            if !is_available {
                return Err(ServiceError::BusinessLogic(format!(
                    "Item {} is currently unavailable", 
                    item_id
                )));
            }
        }

        // Calculate the quotation with proper price breakdown
        let mut total = 0.0;
        let mut breakup = Vec::new();

        for item in &items {
            let price_value = item.price.value.parse::<f64>().unwrap_or(0.0);
            total += price_value;

            // Add a breakdown entry for each item
            breakup.push(QuotationBreakup {
                title: item.descriptor.name.clone(),
                price: Price {
                    currency: item.price.currency.clone(),
                    value: item.price.value.clone(),
                    maximum_value: item.price.maximum_value.clone(),
                },
            });
        }

        // Round to the configured precision
        total = (total * 10.0_f64.powi(self.config.price_precision as i32)).round() 
                / 10.0_f64.powi(self.config.price_precision as i32);

        // Create the complete quotation
        let quotation = Quotation {
            price: Price {
                currency: if !items.is_empty() {
                    items[0].price.currency.clone()
                } else {
                    "INR".to_string()
                },
                value: total.to_string(),
                maximum_value: None,
            },
            breakup,
            ttl: format!("PT{}M", self.config.quotation_ttl_default),
        };

        // Track this quotation in a transaction
        let transaction_id = uuid::Uuid::new_v4().to_string();
        let transaction_data = json!({
            "provider_id": provider_id,
            "quoted_items": items,
            "quotation": quotation,
            "timestamp": Utc::now(),
            "status": "QUOTED",
            "valid_until": (Utc::now() + Duration::minutes(self.config.quotation_ttl_default as i64))
        });
        self.storage.record_transaction(&transaction_id, transaction_data).await?;

        Ok(quotation)
    }

    /// Check availability for specific items
    pub async fn check_availability(
        &self,
        provider_id: &str,
        item_ids: Vec<String>,
        _fulfillment_id: Option<String>,
    ) -> Result<HashMap<String, bool>, ServiceError> {
        // Get the catalog to retrieve item details
        let catalog = self.storage.get_catalog(provider_id).await?;

        // Filter for requested items
        let items: Vec<Item> = catalog
            .items
            .into_iter()
            .filter(|item| item_ids.contains(&item.id))
            .collect();

        if items.is_empty() {
            return Err(ServiceError::NotFound(
                "No matching items found".to_string(),
            ));
        }

        // Check availability for each item
        self.check_item_availability(provider_id, &items).await
    }

    /// Internal method to check availability for a list of items
    async fn check_item_availability(
        &self,
        provider_id: &str,
        items: &[Item],
    ) -> Result<HashMap<String, bool>, ServiceError> {
        let mut availability = HashMap::new();

        // For now, we'll use a simplified approach:
        // - Items without time constraints are always available
        // - Items with time constraints need fulfillment service validation
        for item in items {
            let mut is_available = true;

            // If there's a time constraint, check with fulfillment service
            if let Some(time) = item.time {
                // Default duration of 1 hour if not specified
                let duration = 3600;
                is_available = self
                    .fulfillment_service
                    .check_availability(provider_id, &time, duration)
                    .await?;
            }

            availability.insert(item.id.clone(), is_available);
        }

        Ok(availability)
    }

    /// Validate catalog structure and content
    fn validate_catalog(&self, catalog: &Catalog) -> Result<(), ServiceError> {
        // Check for required fields
        if catalog.descriptor.name.is_empty() {
            return Err(ServiceError::Validation(
                "Catalog name is required".to_string(),
            ));
        }

        // Validate catalog items
        for item in &catalog.items {
            // Check required item fields
            if item.id.is_empty() {
                return Err(ServiceError::Validation(
                    "Item ID is required".to_string(),
                ));
            }

            if item.descriptor.name.is_empty() {
                return Err(ServiceError::Validation(format!(
                    "Item name is required for item ID: {}", 
                    item.id
                )));
            }

            if item.price.value.is_empty() {
                return Err(ServiceError::Validation(format!(
                    "Item price is required for item ID: {}", 
                    item.id
                )));
            }

            // Validate price as a valid number
            if let Err(_) = item.price.value.parse::<f64>() {
                return Err(ServiceError::Validation(format!(
                    "Invalid price value for item ID: {}", 
                    item.id
                )));
            }

            // Check category exists
            if !catalog
                .categories
                .iter()
                .any(|cat| cat.id == item.category_id)
            {
                return Err(ServiceError::Validation(format!(
                    "Category {} referenced by item {} does not exist in catalog",
                    item.category_id, item.id
                )));
            }

            // Check if location exists (if specified)
            if let Some(loc_id) = &item.location_id {
                if !catalog.locations.iter().any(|loc| loc.id == *loc_id) {
                    return Err(ServiceError::Validation(format!(
                        "Location {} referenced by item {} does not exist in catalog",
                        loc_id, item.id
                    )));
                }
            }
        }

        // All validations passed
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::provider::{Category, Descriptor, Location, Provider};
    use crate::storage::memory::MemoryStorage;

    // Helper function to create a basic catalog for testing
    fn create_test_catalog() -> Catalog {
        Catalog {
            descriptor: Descriptor {
                name: "Test Catalog".to_string(),
                short_desc: Some("A test catalog".to_string()),
                long_desc: None,
                images: None,
            },
            categories: vec![Category {
                id: "cat-1".to_string(),
                descriptor: Descriptor {
                    name: "Test Category".to_string(),
                    short_desc: None,
                    long_desc: None,
                    images: None,
                },
                time: None,
                tags: None,
            }],
            fulfillments: vec!["fulfillment-1".to_string()],
            payments: vec!["payment-1".to_string()],
            locations: vec![Location {
                id: "loc-1".to_string(),
                descriptor: Descriptor {
                    name: "Test Location".to_string(),
                    short_desc: None,
                    long_desc: None,
                    images: None,
                },
                gps: "12.9716,77.5946".to_string(),
                address: None,
                city: None,
                state: None,
                country: None,
                area_code: None,
            }],
            items: vec![Item {
                id: "item-1".to_string(),
                parent_item_id: None,
                descriptor: Descriptor {
                    name: "Test Item".to_string(),
                    short_desc: Some("A test item".to_string()),
                    long_desc: None,
                    images: None,
                },
                price: Price {
                    currency: "INR".to_string(),
                    value: "100.0".to_string(),
                    maximum_value: None,
                },
                category_id: "cat-1".to_string(),
                fulfillment_id: "fulfillment-1".to_string(),
                location_id: Some("loc-1".to_string()),
                time: None,
                recommended: None,
                tags: None,
            }],
            exp: None,
        }
    }

    #[tokio::test]
    async fn test_create_catalog() {
        let storage = Arc::new(MemoryStorage::new());
        
        // Create a provider first
        let provider = Provider {
            id: "provider-1".to_string(),
            descriptor: Descriptor {
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
        
        let service = CatalogService::new(storage);
        let catalog = create_test_catalog();
        
        let result = service.create_catalog("provider-1", catalog).await;
        assert!(result.is_ok());
        
        let created = result.unwrap();
        assert_eq!(created.descriptor.name, "Test Catalog");
        assert_eq!(created.items.len(), 1);
        assert!(created.exp.is_some()); // Should have set expiration
    }
    
    #[tokio::test]
    async fn test_select_items() {
        let storage = Arc::new(MemoryStorage::new());
        
        // Create a provider
        let provider = Provider {
            id: "provider-2".to_string(),
            descriptor: Descriptor {
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
        
        // Create a catalog
        let service = CatalogService::new(storage);
        let catalog = create_test_catalog();
        let _ = service.create_catalog("provider-2", catalog).await.unwrap();
        
        // Select an item
        let result = service.select("provider-2", vec!["item-1".to_string()]).await;
        assert!(result.is_ok());
        
        let selected = result.unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].id, "item-1");
    }
    
    #[tokio::test]
    async fn test_on_select_quotation() {
        let storage = Arc::new(MemoryStorage::new());
        
        // Create a provider
        let provider = Provider {
            id: "provider-3".to_string(),
            descriptor: Descriptor {
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
        
        // Create a catalog
        let service = CatalogService::new(storage);
        let catalog = create_test_catalog();
        let _ = service.create_catalog("provider-3", catalog.clone()).await.unwrap();
        
        // Get quotation for an item
        let result = service.on_select("provider-3", vec![catalog.items[0].clone()]).await;
        assert!(result.is_ok());
        
        let quotation = result.unwrap();
        assert_eq!(quotation.price.value, "100");
        assert_eq!(quotation.breakup.len(), 1);
        assert_eq!(quotation.breakup[0].title, "Test Item");
        assert_eq!(quotation.breakup[0].price.value, "100.0");
    }
    
    #[tokio::test]
    async fn test_validate_catalog_invalid_price() {
        let storage = Arc::new(MemoryStorage::new());
        let service = CatalogService::new(storage);
        
        // Create a catalog with invalid price
        let mut catalog = create_test_catalog();
        catalog.items[0].price.value = "not-a-number".to_string();
        
        let result = service.validate_catalog(&catalog);
        assert!(result.is_err());
        
        if let Err(ServiceError::Validation(msg)) = result {
            assert!(msg.contains("Invalid price value"));
        } else {
            panic!("Expected ValidationError");
        }
    }
    
    #[tokio::test]
    async fn test_validate_catalog_invalid_category() {
        let storage = Arc::new(MemoryStorage::new());
        let service = CatalogService::new(storage);
        
        // Create a catalog with invalid category reference
        let mut catalog = create_test_catalog();
        catalog.items[0].category_id = "non-existent-category".to_string();
        
        let result = service.validate_catalog(&catalog);
        assert!(result.is_err());
        
        if let Err(ServiceError::Validation(msg)) = result {
            assert!(msg.contains("Category"));
            assert!(msg.contains("does not exist"));
        } else {
            panic!("Expected ValidationError");
        }
    }
}
