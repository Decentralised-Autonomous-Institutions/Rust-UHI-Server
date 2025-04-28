use super::*;
use crate::models::provider::{Provider, Descriptor, Category};
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

fn create_test_provider() -> Provider {
    Provider {
        id: Uuid::new_v4().to_string(),
        descriptor: Descriptor {
            name: "Test Healthcare Provider".to_string(),
            short_desc: Some("Short description".to_string()),
            long_desc: Some("Long description of the provider".to_string()),
            images: Some(vec!["http://example.com/image.jpg".to_string()]),
        },
        categories: vec![
            Category {
                id: "cat1".to_string(),
                descriptor: Descriptor {
                    name: "Cardiology".to_string(),
                    short_desc: Some("Heart related services".to_string()),
                    long_desc: None,
                    images: None,
                },
                time: Some(Utc::now()),
                tags: Some(HashMap::new()),
            }
        ],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[tokio::test]
async fn test_create_provider() {
    let storage = MemoryStorage::new();
    let provider = create_test_provider();
    let provider_id = provider.id.clone();
    let saved = storage.create_provider(provider.clone()).await.unwrap();
    assert_eq!(saved.id, provider_id);
}

#[tokio::test]
async fn test_get_provider() {
    let storage = MemoryStorage::new();
    let provider = create_test_provider();
    let provider_id = provider.id.clone();
    storage.create_provider(provider.clone()).await.unwrap();
    let retrieved = storage.get_provider(&provider_id).await.unwrap();
    assert_eq!(retrieved.id, provider_id);
}

#[tokio::test]
async fn test_list_providers() {
    let storage = MemoryStorage::new();
    let provider = create_test_provider();
    storage.create_provider(provider).await.unwrap();
    let providers = storage.list_providers().await.unwrap();
    assert_eq!(providers.len(), 1);
}

#[tokio::test]
async fn test_update_provider() {
    let storage = MemoryStorage::new();
    let mut provider = create_test_provider();
    let provider_id = provider.id.clone();
    storage.create_provider(provider.clone()).await.unwrap();
    provider.descriptor.name = "Updated Provider Name".to_string();
    let updated = storage.update_provider(provider).await.unwrap();
    assert_eq!(updated.id, provider_id);
    assert_eq!(updated.descriptor.name, "Updated Provider Name");
}

#[tokio::test]
async fn test_delete_provider() {
    let storage = MemoryStorage::new();
    let provider = create_test_provider();
    let provider_id = provider.id.clone();
    storage.create_provider(provider).await.unwrap();
    storage.delete_provider(&provider_id).await.unwrap();
}

#[tokio::test]
async fn test_provider_not_found_after_deletion() {
    let storage = MemoryStorage::new();
    let provider = create_test_provider();
    let provider_id = provider.id.clone();
    storage.create_provider(provider.clone()).await.unwrap();
    storage.delete_provider(&provider_id).await.unwrap();
    let result = storage.get_provider(&provider_id).await;
    assert!(result.is_err());
}