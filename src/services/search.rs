use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use super::error::ServiceError;
use super::provider::ProviderService;
use crate::models::catalog::{SearchRequest, SearchResponse};
use crate::storage::Storage;

/// Search metadata for tracking search transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMetadata {
    /// Transaction ID for the search session
    pub transaction_id: String,

    /// Timestamp when search was initiated
    pub timestamp: DateTime<Utc>,

    /// Original search request
    pub request: SearchRequest,

    /// List of providers to which the search was forwarded
    pub forwarded_to: Vec<String>,

    /// Responses received from providers
    pub responses: HashMap<String, SearchResponse>,
}

/// Search service for handling healthcare service discovery
pub struct SearchService {
    /// Storage implementation injected via constructor
    storage: Arc<dyn Storage>,
    /// Provider service for filtering providers
    provider_service: ProviderService,
    /// Configuration parameters
    config: SearchServiceConfig,
}

/// Configuration parameters for SearchService
pub struct SearchServiceConfig {
    /// Maximum time to wait for provider responses (in seconds)
    pub search_timeout: u64,
    /// Maximum number of providers to forward a search to
    pub max_providers_per_search: usize,
    /// Minimum providers that must respond for valid results
    pub min_providers_for_results: usize,
    /// Maximum number of concurrent searches
    pub concurrent_search_limit: usize,
}

impl Default for SearchServiceConfig {
    fn default() -> Self {
        Self {
            search_timeout: 30,
            max_providers_per_search: 10,
            min_providers_for_results: 1,
            concurrent_search_limit: 100,
        }
    }
}

impl SearchService {
    /// Create a new search service with storage dependency
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        let provider_service = ProviderService::new(storage.clone());
        Self {
            storage,
            provider_service,
            config: SearchServiceConfig::default(),
        }
    }

    /// Create a new search service with custom configuration
    pub fn with_config(storage: Arc<dyn Storage>, config: SearchServiceConfig) -> Self {
        let provider_service = ProviderService::new(storage.clone());
        Self {
            storage,
            provider_service,
            config,
        }
    }

    /// Process a search request
    pub async fn search(&self, request: SearchRequest) -> Result<SearchResponse, ServiceError> {
        // Validate search request
        self.validate_search_request(&request)?;

        // Generate a transaction ID for this search
        let transaction_id = Uuid::new_v4().to_string();

        // Initialize search metadata
        let metadata = SearchMetadata {
            transaction_id: transaction_id.clone(),
            timestamp: Utc::now(),
            request: request.clone(),
            forwarded_to: Vec::new(),
            responses: HashMap::new(),
        };

        // Track this search transaction
        self.track_search_transaction(&transaction_id, metadata.clone())
            .await?;

        // Identify relevant providers based on search criteria
        let providers = self.identify_relevant_providers(&request).await?;

        if providers.is_empty() {
            return Err(ServiceError::NotFound(
                "No matching providers found for the search criteria".to_string(),
            ));
        }

        // For now, since we don't have actual provider forwarding logic,
        // we'll just use the storage's search_catalog method
        let response = self.storage.search_catalog(request).await?;

        // In a complete implementation, we would:
        // 1. Forward the search to each provider (limited by max_providers_per_search)
        // 2. Wait for responses or until timeout
        // 3. Aggregate and filter results
        // 4. Update the transaction record with responses

        Ok(response)
    }

    /// Forward search results back to the requesting EUA
    pub async fn on_search(
        &self,
        provider_id: &str,
        _response: SearchResponse,
    ) -> Result<(), ServiceError> {
        // Validate the provider exists
        self.provider_service.get_provider(provider_id).await?;

        // In a real implementation, this would:
        // 1. Find the transaction associated with this search
        // 2. Update the transaction with this provider's response
        // 3. Check if we have all expected responses or hit the timeout
        // 4. Merge and forward final results if appropriate

        // For now, just a placeholder
        Ok(())
    }

    /// Track search transactions to maintain session state
    pub async fn track_search_transaction(
        &self,
        transaction_id: &str,
        search_data: SearchMetadata,
    ) -> Result<(), ServiceError> {
        // Convert search metadata to JSON
        let data = json!(search_data);

        // Record the transaction in storage
        self.storage
            .record_transaction(transaction_id, data)
            .await?;

        Ok(())
    }

    /// Retrieve a search transaction by ID
    pub async fn get_search_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<SearchMetadata, ServiceError> {
        // Get the transaction data from storage
        let data = self.storage.get_transaction(transaction_id).await?;

        // Convert JSON to SearchMetadata
        let metadata: SearchMetadata = serde_json::from_value(data).map_err(|e| {
            ServiceError::Internal(format!("Failed to deserialize search metadata: {}", e))
        })?;

        Ok(metadata)
    }

    /// Identify providers relevant to the search criteria
    async fn identify_relevant_providers(
        &self,
        _request: &SearchRequest,
    ) -> Result<Vec<String>, ServiceError> {
        // In a complete implementation, this would:
        // 1. Extract criteria from the request (specialty, location, etc.)
        // 2. Query provider service to find matching providers
        // 3. Apply filtering based on criteria
        // 4. Limit to max_providers_per_search

        // For now, return all providers as a simple implementation
        let providers = self.provider_service.list_providers().await?;
        let provider_ids: Vec<String> = providers
            .into_iter()
            .map(|p| p.id)
            .take(self.config.max_providers_per_search)
            .collect();

        Ok(provider_ids)
    }

    /// Validate a search request
    fn validate_search_request(&self, request: &SearchRequest) -> Result<(), ServiceError> {
        // Check that the query is not empty
        if request.query.is_empty() {
            return Err(ServiceError::Validation(
                "Search query cannot be empty".to_string(),
            ));
        }

        // Additional validation could check:
        // - Required fields based on search type
        // - Valid location format if location-based search
        // - Valid specialty codes if healthcare service search
        // - etc.

        Ok(())
    }

    /// Merge search results from multiple providers
    fn merge_search_results(
        &self,
        responses: &HashMap<String, SearchResponse>,
    ) -> Result<SearchResponse, ServiceError> {
        if responses.is_empty() {
            return Err(ServiceError::NotFound(
                "No search results found".to_string(),
            ));
        }

        // Start with the first response as a base
        let mut merged_response = responses.values().next().unwrap().clone();

        // Merge in all other responses
        for (_, response) in responses.iter().skip(1) {
            // Merge items
            merged_response
                .catalog
                .items
                .extend(response.catalog.items.clone());

            // Merge categories (avoiding duplicates)
            for category in &response.catalog.categories {
                if !merged_response
                    .catalog
                    .categories
                    .iter()
                    .any(|c| c.id == category.id)
                {
                    merged_response.catalog.categories.push(category.clone());
                }
            }

            // Merge fulfillments (avoiding duplicates)
            for fulfillment in &response.catalog.fulfillments {
                if !merged_response.catalog.fulfillments.contains(fulfillment) {
                    merged_response
                        .catalog
                        .fulfillments
                        .push(fulfillment.clone());
                }
            }

            // Merge locations (avoiding duplicates)
            for location in &response.catalog.locations {
                if !merged_response
                    .catalog
                    .locations
                    .iter()
                    .any(|l| l.id == location.id)
                {
                    merged_response.catalog.locations.push(location.clone());
                }
            }
        }

        Ok(merged_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::memory::MemoryStorage;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_search_with_empty_query() {
        let storage = Arc::new(MemoryStorage::new());
        let service = SearchService::new(storage);

        let request = SearchRequest {
            query: HashMap::new(),
            item: None,
            fulfillment: None,
            payment: None,
            location: None,
        };

        let result = service.search(request).await;
        assert!(result.is_err());

        if let Err(ServiceError::Validation(msg)) = result {
            assert_eq!(msg, "Search query cannot be empty");
        } else {
            panic!("Expected validation error");
        }
    }

    #[tokio::test]
    async fn test_track_search_transaction() {
        let storage = Arc::new(MemoryStorage::new());
        let service = SearchService::new(storage);

        // Create a test search request
        let request = SearchRequest {
            query: HashMap::from([("specialty".to_string(), vec!["Cardiology".to_string()])]),
            item: None,
            fulfillment: None,
            payment: None,
            location: None,
        };

        // Create metadata and transaction ID
        let transaction_id = "test-transaction-123".to_string();
        let metadata = SearchMetadata {
            transaction_id: transaction_id.clone(),
            timestamp: Utc::now(),
            request: request.clone(),
            forwarded_to: Vec::new(),
            responses: HashMap::new(),
        };

        // Track the transaction
        let result = service
            .track_search_transaction(&transaction_id, metadata.clone())
            .await;
        assert!(result.is_ok());

        // Retrieve the transaction and verify
        let retrieved = service.get_search_transaction(&transaction_id).await;
        assert!(retrieved.is_ok());

        let retrieved_metadata = retrieved.unwrap();
        assert_eq!(retrieved_metadata.transaction_id, transaction_id);
        assert_eq!(retrieved_metadata.forwarded_to.len(), 0);

        // Verify the request data
        let specialty = &retrieved_metadata.request.query.get("specialty").unwrap()[0];
        assert_eq!(specialty, "Cardiology");
    }
}
