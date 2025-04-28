use super::error::ServiceError;
use crate::models::provider::Provider;
use crate::storage::Storage;
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use std::sync::Arc;

/// Provider service for managing healthcare service providers
pub struct ProviderService {
    /// Storage implementation injected via constructor
    storage: Arc<dyn Storage>,
}

impl ProviderService {
    /// Create a new provider service with storage dependency
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }

    /// Register a new provider
    pub async fn register_provider(&self, provider: Provider) -> Result<Provider, ServiceError> {
        let registered = self.storage.create_provider(provider).await?;
        Ok(registered)
    }

    /// Get a provider by ID
    pub async fn get_provider(&self, id: &str) -> Result<Provider, ServiceError> {
        let provider = self.storage.get_provider(id).await?;
        Ok(provider)
    }

    /// Update a provider
    pub async fn update_provider(&self, provider: Provider) -> Result<Provider, ServiceError> {
        // Verify provider exists
        let _ = self.storage.get_provider(&provider.id).await?;

        // Update in storage
        let updated = self.storage.update_provider(provider).await?;
        Ok(updated)
    }

    /// Delete a provider
    pub async fn delete_provider(&self, id: &str) -> Result<(), ServiceError> {
        let result = self.storage.delete_provider(id).await?;
        Ok(result)
    }

    /// List all providers
    pub async fn list_providers(&self) -> Result<Vec<Provider>, ServiceError> {
        let providers = self.storage.list_providers().await?;
        Ok(providers)
    }

    /// Check if a provider is available at a specific time
    ///
    /// # Parameters
    /// * `provider_id` - The ID of the provider to check
    /// * `requested_time` - The time to check availability for
    ///
    /// # Returns
    /// * `true` if the provider is available, `false` otherwise
    pub async fn check_provider_availability(
        &self,
        provider_id: &str,
        requested_time: &DateTime<Utc>,
    ) -> Result<bool, ServiceError> {
        // Get the provider to check their working hours
        let provider = self.storage.get_provider(provider_id).await?;

        // The provider must exist to check availability
        // Extract the day of the week (0 = Sunday, 6 = Saturday)
        let day_of_week = requested_time.weekday().num_days_from_sunday();

        // Extract the hour of the day (0-23)
        let hour = requested_time.hour();

        // For now, implement a simple check: providers are available
        // Monday-Friday (1-5), 9 AM to 5 PM (9-16)
        let is_weekday = day_of_week >= 1 && day_of_week <= 5;
        let is_business_hours = hour >= 9 && hour < 17;

        // TODO: In a real implementation, we would check the provider's actual working hours
        // from their profile or a separate schedule table
        // For now, just check if it's a weekday during business hours
        if is_weekday && is_business_hours {
            Ok(true)
        } else {
            // Outside of business hours
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::provider::{Category, Descriptor, Provider};
    use crate::storage::memory::MemoryStorage;
    use std::collections::HashMap;

    // Helper function to create a test provider
    fn create_test_provider(id: &str, name: &str) -> Provider {
        Provider {
            id: id.to_string(),
            descriptor: Descriptor {
                name: name.to_string(),
                short_desc: Some("A test provider".to_string()),
                long_desc: None,
                images: None,
            },
            categories: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_register_provider() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);

        // Create a test provider
        let provider = create_test_provider("test-provider-1", "Test Provider 1");

        // Register the provider
        let result = service.register_provider(provider).await;
        assert!(result.is_ok());

        // Retrieve the provider
        let retrieved = service.get_provider("test-provider-1").await;
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().descriptor.name, "Test Provider 1");
    }

    #[tokio::test]
    async fn test_update_provider() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);

        // Create and register a test provider
        let provider = create_test_provider("test-provider-2", "Test Provider 2");
        let _ = service.register_provider(provider).await.unwrap();

        // Update the provider
        let mut updated_provider = create_test_provider("test-provider-2", "Updated Provider 2");
        updated_provider.descriptor.short_desc = Some("Updated description".to_string());

        let result = service.update_provider(updated_provider).await;
        assert!(result.is_ok());

        // Retrieve the updated provider
        let retrieved = service.get_provider("test-provider-2").await;
        assert!(retrieved.is_ok());
        let retrieved_provider = retrieved.unwrap();
        assert_eq!(retrieved_provider.descriptor.name, "Updated Provider 2");
        assert_eq!(
            retrieved_provider.descriptor.short_desc,
            Some("Updated description".to_string())
        );
    }

    #[tokio::test]
    async fn test_delete_provider() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);

        // Create and register a test provider
        let provider = create_test_provider("test-provider-3", "Test Provider 3");
        let _ = service.register_provider(provider).await.unwrap();

        // Delete the provider
        let result = service.delete_provider("test-provider-3").await;
        assert!(result.is_ok());

        // Try to retrieve the deleted provider - should fail
        let retrieved = service.get_provider("test-provider-3").await;
        assert!(retrieved.is_err());
    }

    #[tokio::test]
    async fn test_list_providers() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);

        // List providers when none exist
        let empty_list = service.list_providers().await;
        assert!(empty_list.is_ok());
        assert_eq!(empty_list.unwrap().len(), 0);

        // Create and register two test providers
        let provider1 = create_test_provider("test-provider-4a", "Test Provider 4A");
        let provider2 = create_test_provider("test-provider-4b", "Test Provider 4B");
        let _ = service.register_provider(provider1).await.unwrap();
        let _ = service.register_provider(provider2).await.unwrap();

        // List providers - should return both
        let list_result = service.list_providers().await;
        assert!(list_result.is_ok());
        let providers = list_result.unwrap();
        assert_eq!(providers.len(), 2);

        // Verify the providers are in the list
        let provider_ids: Vec<String> = providers.iter().map(|p| p.id.clone()).collect();
        assert!(provider_ids.contains(&"test-provider-4a".to_string()));
        assert!(provider_ids.contains(&"test-provider-4b".to_string()));
    }

    #[tokio::test]
    async fn test_check_provider_availability_business_hours() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);

        // Create and register a test provider
        let provider = create_test_provider("test-provider-5", "Test Provider 5");
        let _ = service.register_provider(provider).await.unwrap();

        // Find the next Monday at 10 AM (should be available - business hours)
        let now = Utc::now();
        let days_to_monday = (8 - now.weekday().num_days_from_sunday()) % 7;
        let next_monday = now + Duration::days(days_to_monday as i64);
        let next_monday_10am = Utc::now()
            .with_day(next_monday.day())
            .unwrap()
            .with_month(next_monday.month())
            .unwrap()
            .with_year(next_monday.year())
            .unwrap()
            .with_hour(10)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        // Check availability for this time - should be available
        let is_available = service
            .check_provider_availability("test-provider-5", &next_monday_10am)
            .await;
        assert!(is_available.is_ok());
        assert!(is_available.unwrap());
    }

    #[tokio::test]
    async fn test_check_provider_availability_non_business_hours() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);

        // Create and register a test provider
        let provider = create_test_provider("test-provider-6", "Test Provider 6");
        let _ = service.register_provider(provider).await.unwrap();

        // Find the next Sunday at 10 AM (should not be available - weekend)
        let now = Utc::now();
        let days_to_sunday = (7 - now.weekday().num_days_from_sunday()) % 7;
        let next_sunday = now + Duration::days(days_to_sunday as i64);
        let next_sunday_10am = Utc::now()
            .with_day(next_sunday.day())
            .unwrap()
            .with_month(next_sunday.month())
            .unwrap()
            .with_year(next_sunday.year())
            .unwrap()
            .with_hour(10)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        // Check availability for this time - should not be available (weekend)
        let is_available = service
            .check_provider_availability("test-provider-6", &next_sunday_10am)
            .await;
        assert!(is_available.is_ok());
        assert!(!is_available.unwrap());

        // Find next Monday at 7 PM (should not be available - after business hours)
        let days_to_monday = (8 - now.weekday().num_days_from_sunday()) % 7;
        let next_monday = now + Duration::days(days_to_monday as i64);
        let next_monday_7pm = Utc::now()
            .with_day(next_monday.day())
            .unwrap()
            .with_month(next_monday.month())
            .unwrap()
            .with_year(next_monday.year())
            .unwrap()
            .with_hour(19)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        // Check availability for this time - should not be available (after hours)
        let is_available = service
            .check_provider_availability("test-provider-6", &next_monday_7pm)
            .await;
        assert!(is_available.is_ok());
        assert!(!is_available.unwrap());
    }
}
