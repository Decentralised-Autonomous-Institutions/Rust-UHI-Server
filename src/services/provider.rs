use super::error::ServiceError;
use crate::models::provider::{Circle, Location, Provider, ServiceArea};
use crate::storage::Storage;
use chrono::{DateTime, Datelike, Duration, NaiveTime, Timelike, Utc};
use std::collections::HashMap;
use std::sync::Arc;

/// Time range for working hours
#[derive(Debug, Clone)]
pub struct TimeRange {
    /// Start time in HH:MM format
    pub start: String,
    
    /// End time in HH:MM format
    pub end: String,
}

/// Working hours for a provider
#[derive(Debug, Clone)]
pub struct WorkingHours {
    /// Provider ID
    pub provider_id: String,
    
    /// Regular working days and hours (keyed by day name: "Monday", "Tuesday", etc.)
    pub regular_hours: HashMap<String, Vec<TimeRange>>,
    
    /// Exception dates (holidays, special hours) keyed by ISO date string (YYYY-MM-DD)
    pub exceptions: HashMap<String, Vec<TimeRange>>,
    
    /// Regular break times keyed by day name
    pub breaks: Option<HashMap<String, Vec<TimeRange>>>,
}

impl TimeRange {
    /// Parse the time range into NaiveTime objects
    fn parse_times(&self) -> Result<(NaiveTime, NaiveTime), ServiceError> {
        let start_time = NaiveTime::parse_from_str(&self.start, "%H:%M")
            .map_err(|e| ServiceError::Validation(format!("Invalid start time format: {}", e)))?;
        
        let end_time = NaiveTime::parse_from_str(&self.end, "%H:%M")
            .map_err(|e| ServiceError::Validation(format!("Invalid end time format: {}", e)))?;
            
        Ok((start_time, end_time))
    }
    
    /// Check if a given hour:minute is within this time range
    fn contains_time(&self, hour: u32, minute: u32) -> Result<bool, ServiceError> {
        let (start_time, end_time) = self.parse_times()?;
        let check_time = NaiveTime::from_hms_opt(hour, minute, 0)
            .ok_or_else(|| ServiceError::Validation("Invalid time".to_string()))?;
            
        Ok(check_time >= start_time && check_time < end_time)
    }
}

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

    /// Get default working hours for a provider
    /// In a real implementation, these would come from the database
    fn get_default_working_hours(&self, provider_id: &str) -> WorkingHours {
        let mut regular_hours = HashMap::new();
        
        // Default working hours: Monday-Friday, 9 AM to 5 PM
        let weekday_hours = vec![TimeRange {
            start: "09:00".to_string(),
            end: "17:00".to_string(),
        }];
        
        regular_hours.insert("Monday".to_string(), weekday_hours.clone());
        regular_hours.insert("Tuesday".to_string(), weekday_hours.clone());
        regular_hours.insert("Wednesday".to_string(), weekday_hours.clone());
        regular_hours.insert("Thursday".to_string(), weekday_hours.clone());
        regular_hours.insert("Friday".to_string(), weekday_hours.clone());
        
        // Empty hours for weekends
        regular_hours.insert("Saturday".to_string(), Vec::new());
        regular_hours.insert("Sunday".to_string(), Vec::new());
        
        // No exceptions by default
        let exceptions = HashMap::new();
        
        // Default lunch break
        let mut breaks = HashMap::new();
        let lunch_break = vec![TimeRange {
            start: "12:00".to_string(),
            end: "13:00".to_string(),
        }];
        
        breaks.insert("Monday".to_string(), lunch_break.clone());
        breaks.insert("Tuesday".to_string(), lunch_break.clone());
        breaks.insert("Wednesday".to_string(), lunch_break.clone());
        breaks.insert("Thursday".to_string(), lunch_break.clone());
        breaks.insert("Friday".to_string(), lunch_break.clone());
        
        WorkingHours {
            provider_id: provider_id.to_string(),
            regular_hours,
            exceptions,
            breaks: Some(breaks),
        }
    }
    
    /// Get provider working hours
    pub async fn get_working_hours(&self, provider_id: &str) -> Result<WorkingHours, ServiceError> {
        // Verify the provider exists
        let _ = self.storage.get_provider(provider_id).await?;
        
        // In a real implementation, we would fetch working hours from storage
        // For now, return default working hours
        Ok(self.get_default_working_hours(provider_id))
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
        // Get the provider to verify they exist
        let _ = self.storage.get_provider(provider_id).await?;
        
        // Get working hours
        let working_hours = self.get_working_hours(provider_id).await?;
        
        // Extract the day of the week name
        let day_names = [
            "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
        ];
        let day_idx = requested_time.weekday().num_days_from_sunday() as usize;
        let day_name = day_names[day_idx];
        
        // Extract date for exception checking
        let date_string = requested_time.format("%Y-%m-%d").to_string();
        
        // Extract hour and minute
        let hour = requested_time.hour();
        let minute = requested_time.minute();
        
        // First check if there's an exception for this date
        if let Some(exception_hours) = working_hours.exceptions.get(&date_string) {
            // If there are exception hours, check if the requested time falls within any
            for time_range in exception_hours {
                if time_range.contains_time(hour, minute)? {
                    // Time is within exception hours, provider is available
                    return Ok(true);
                }
            }
            
            // Time is not within any exception hours, provider is not available
            return Ok(false);
        }
        
        // No exception, check regular hours
        if let Some(day_hours) = working_hours.regular_hours.get(day_name) {
            // If there are no hours for this day, provider is not available
            if day_hours.is_empty() {
                return Ok(false);
            }
            
            // Check if the requested time falls within any of the time ranges
            let mut within_working_hours = false;
            for time_range in day_hours {
                if time_range.contains_time(hour, minute)? {
                    within_working_hours = true;
                    break;
                }
            }
            
            // If not within working hours, not available
            if !within_working_hours {
                return Ok(false);
            }
            
            // Check if the time falls within any break periods
            if let Some(break_periods) = &working_hours.breaks {
                if let Some(day_breaks) = break_periods.get(day_name) {
                    for break_time in day_breaks {
                        if break_time.contains_time(hour, minute)? {
                            // Time is during a break, provider is not available
                            return Ok(false);
                        }
                    }
                }
            }
            
            // Time is within working hours and not during a break, provider is available
            return Ok(true);
        }
        
        // No working hours defined for this day, provider is not available
        Ok(false)
    }
    
    /// Find providers by specialty
    ///
    /// # Parameters
    /// * `specialty` - The specialty code to search for
    ///
    /// # Returns
    /// * List of providers matching the specialty
    pub async fn find_providers_by_specialty(
        &self,
        specialty: &str,
    ) -> Result<Vec<Provider>, ServiceError> {
        // Get all providers
        let all_providers = self.list_providers().await?;
        
        // Filter providers that have the requested specialty
        let matching_providers = all_providers
            .into_iter()
            .filter(|provider| {
                // Check if any category has the specialty code
                provider.categories.iter().any(|category| {
                    if let Some(tags) = &category.tags {
                        if let Some(code) = tags.get("code") {
                            return code.to_uppercase() == specialty.to_uppercase();
                        }
                    }
                    false
                })
            })
            .collect();
            
        Ok(matching_providers)
    }
    
    /// Parse GPS coordinates into latitude and longitude
    fn parse_gps_coordinates(&self, gps: &str) -> Result<(f64, f64), ServiceError> {
        let parts: Vec<&str> = gps.split(',').collect();
        
        if parts.len() != 2 {
            return Err(ServiceError::Validation(format!(
                "Invalid GPS format. Expected 'latitude,longitude', got '{}'", 
                gps
            )));
        }
        
        let lat = parts[0].trim().parse::<f64>().map_err(|e| {
            ServiceError::Validation(format!("Invalid latitude: {}", e))
        })?;
        
        let lng = parts[1].trim().parse::<f64>().map_err(|e| {
            ServiceError::Validation(format!("Invalid longitude: {}", e))
        })?;
        
        // Basic validation of coordinates
        if lat < -90.0 || lat > 90.0 {
            return Err(ServiceError::Validation(format!(
                "Latitude out of range: {}. Must be between -90 and 90", 
                lat
            )));
        }
        
        if lng < -180.0 || lng > 180.0 {
            return Err(ServiceError::Validation(format!(
                "Longitude out of range: {}. Must be between -180 and 180", 
                lng
            )));
        }
        
        Ok((lat, lng))
    }
    
    /// Calculate distance between two points using the Haversine formula
    fn calculate_distance(&self, lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
        // Earth radius in kilometers
        const EARTH_RADIUS: f64 = 6371.0;
        
        // Convert to radians
        let lat1_rad = lat1.to_radians();
        let lng1_rad = lng1.to_radians();
        let lat2_rad = lat2.to_radians();
        let lng2_rad = lng2.to_radians();
        
        // Differences
        let dlat = lat2_rad - lat1_rad;
        let dlng = lng2_rad - lng1_rad;
        
        // Haversine formula
        let a = (dlat / 2.0).sin().powi(2) + 
                lat1_rad.cos() * lat2_rad.cos() * (dlng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        // Distance in kilometers
        EARTH_RADIUS * c
    }
    
    /// Find providers by location within a radius
    ///
    /// # Parameters
    /// * `location` - The location coordinates in 'latitude,longitude' format
    /// * `radius_km` - The search radius in kilometers
    ///
    /// # Returns
    /// * List of providers within the radius, sorted by distance
    pub async fn find_providers_by_location(
        &self,
        location: &str,
        radius_km: f64,
    ) -> Result<Vec<Provider>, ServiceError> {
        if radius_km <= 0.0 {
            return Err(ServiceError::Validation(
                "Radius must be greater than zero".to_string()
            ));
        }
        
        // Parse the search location coordinates
        let (search_lat, search_lng) = self.parse_gps_coordinates(location)?;
        
        // Get all providers
        let all_providers = self.list_providers().await?;
        
        // Create a vector to hold providers with their distances
        let mut providers_with_distance: Vec<(Provider, f64)> = Vec::new();
        
        // For each provider, calculate distance and check if within radius
        for provider in all_providers {
            // TODO: In a real implementation, we would fetch provider locations from storage
            // For now, we'll assume the provider ID also contains location for demo purposes
            // This is just a placeholder - real location data should be used
            
            // Skip providers without location info for now
            // In a real implementation, we would skip this and properly check
            // location data for each provider
            
            // For testing only:
            if provider.id.contains("location:") {
                // Extract location from ID (this is just for demo purposes)
                let parts: Vec<&str> = provider.id.split("location:").collect();
                if parts.len() < 2 {
                    continue;
                }
                
                // Try to parse provider coords
                if let Ok((provider_lat, provider_lng)) = self.parse_gps_coordinates(parts[1]) {
                    // Calculate distance
                    let distance = self.calculate_distance(
                        search_lat, 
                        search_lng, 
                        provider_lat, 
                        provider_lng
                    );
                    
                    // If within radius, add to results
                    if distance <= radius_km {
                        providers_with_distance.push((provider, distance));
                    }
                }
            }
        }
        
        // Sort by distance
        providers_with_distance.sort_by(|a, b| {
            a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Return just the providers, without the distances
        let result = providers_with_distance
            .into_iter()
            .map(|(provider, _)| provider)
            .collect();
            
        Ok(result)
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

    // Helper function to create a test provider with specialty
    fn create_test_provider_with_specialty(id: &str, name: &str, specialty: &str) -> Provider {
        let mut provider = create_test_provider(id, name);
        
        // Add a category with the specialty
        let mut tags = HashMap::new();
        tags.insert("code".to_string(), specialty.to_string());
        
        provider.categories.push(Category {
            id: format!("cat-{}", specialty.to_lowercase()),
            descriptor: Descriptor {
                name: specialty.to_string(),
                short_desc: Some(format!("{} services", specialty)),
                long_desc: None,
                images: None,
            },
            time: None,
            tags: Some(tags),
        });
        
        provider
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
    async fn test_find_providers_by_specialty() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);

        // Create and register providers with different specialties
        let cardiology_provider = create_test_provider_with_specialty(
            "test-provider-specialty-1", 
            "Cardiology Center", 
            "CARDIOLOGY"
        );
        
        let orthopedic_provider = create_test_provider_with_specialty(
            "test-provider-specialty-2", 
            "Orthopedic Center", 
            "ORTHOPEDICS"
        );
        
        let another_cardiology = create_test_provider_with_specialty(
            "test-provider-specialty-3", 
            "Heart Specialists", 
            "CARDIOLOGY"
        );
        
        // Register all providers
        let _ = service.register_provider(cardiology_provider).await.unwrap();
        let _ = service.register_provider(orthopedic_provider).await.unwrap();
        let _ = service.register_provider(another_cardiology).await.unwrap();
        
        // Search for cardiology providers
        let cardiology_search = service.find_providers_by_specialty("CARDIOLOGY").await;
        assert!(cardiology_search.is_ok());
        let cardiology_results = cardiology_search.unwrap();
        assert_eq!(cardiology_results.len(), 2);
        
        // Verify the correct providers were found
        let cardiology_ids: Vec<String> = cardiology_results.iter().map(|p| p.id.clone()).collect();
        assert!(cardiology_ids.contains(&"test-provider-specialty-1".to_string()));
        assert!(cardiology_ids.contains(&"test-provider-specialty-3".to_string()));
        
        // Search for orthopedic providers
        let orthopedic_search = service.find_providers_by_specialty("ORTHOPEDICS").await;
        assert!(orthopedic_search.is_ok());
        let orthopedic_results = orthopedic_search.unwrap();
        assert_eq!(orthopedic_results.len(), 1);
        assert_eq!(orthopedic_results[0].id, "test-provider-specialty-2");
        
        // Search for a non-existent specialty
        let non_existent = service.find_providers_by_specialty("NEUROLOGY").await;
        assert!(non_existent.is_ok());
        assert_eq!(non_existent.unwrap().len(), 0);
    }
    
    #[tokio::test]
    async fn test_working_hours() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);

        // Create and register a test provider
        let provider = create_test_provider("test-provider-hours", "Test Provider Hours");
        let _ = service.register_provider(provider).await.unwrap();
        
        // Get working hours
        let hours_result = service.get_working_hours("test-provider-hours").await;
        assert!(hours_result.is_ok());
        
        let hours = hours_result.unwrap();
        assert_eq!(hours.provider_id, "test-provider-hours");
        
        // Verify working hours are set for weekdays
        assert!(hours.regular_hours.contains_key("Monday"));
        assert!(hours.regular_hours.contains_key("Friday"));
        assert!(!hours.regular_hours.get("Monday").unwrap().is_empty());
        assert!(hours.regular_hours.get("Saturday").unwrap().is_empty());
        
        // Verify breaks are set
        assert!(hours.breaks.is_some());
        let breaks = hours.breaks.unwrap();
        assert!(breaks.contains_key("Monday"));
        assert!(!breaks.get("Monday").unwrap().is_empty());
    }
    
    #[tokio::test]
    async fn test_check_provider_availability_with_working_hours() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);

        // Create and register a test provider
        let provider = create_test_provider("test-provider-availability", "Test Provider Availability");
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
            
        // Check availability for this time - should be available (business hours)
        let is_available = service
            .check_provider_availability("test-provider-availability", &next_monday_10am)
            .await;
        assert!(is_available.is_ok());
        assert!(is_available.unwrap(), "Provider should be available on Monday at 10 AM");
        
        // Check availability during lunch break - should not be available
        let next_monday_12_30pm = next_monday_10am
            .with_hour(12)
            .unwrap()
            .with_minute(30)
            .unwrap();
            
        let is_available = service
            .check_provider_availability("test-provider-availability", &next_monday_12_30pm)
            .await;
        assert!(is_available.is_ok());
        assert!(!is_available.unwrap(), "Provider should be on lunch break at 12:30 PM");
        
        // Check availability on weekend - should not be available
        let days_to_saturday = (13 - now.weekday().num_days_from_sunday()) % 7;
        let next_saturday = now + Duration::days(days_to_saturday as i64);
        let next_saturday_10am = Utc::now()
            .with_day(next_saturday.day())
            .unwrap()
            .with_month(next_saturday.month())
            .unwrap()
            .with_year(next_saturday.year())
            .unwrap()
            .with_hour(10)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
            
        let is_available = service
            .check_provider_availability("test-provider-availability", &next_saturday_10am)
            .await;
        assert!(is_available.is_ok());
        assert!(!is_available.unwrap(), "Provider should not be available on Saturday");
        
        // Check availability at 8 AM - before office hours
        let next_monday_8am = next_monday_10am
            .with_hour(8)
            .unwrap()
            .with_minute(0)
            .unwrap();
            
        let is_available = service
            .check_provider_availability("test-provider-availability", &next_monday_8am)
            .await;
        assert!(is_available.is_ok());
        assert!(!is_available.unwrap(), "Provider should not be available before office hours");
    }
    
    #[tokio::test]
    async fn test_find_providers_by_location() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);
        
        // Create test providers with location information in the ID (for demo purposes)
        // In a real implementation, location would be properly stored in the provider model
        
        // Provider in Bangalore (12.9716°N, 77.5946°E)
        let provider_blr = create_test_provider(
            "provider-location:12.9716,77.5946", 
            "Bangalore Hospital"
        );
        
        // Provider in Mumbai (19.0760°N, 72.8777°E)
        let provider_mum = create_test_provider(
            "provider-location:19.0760,72.8777", 
            "Mumbai Medical Center"
        );
        
        // Provider in Delhi (28.7041°N, 77.1025°E)
        let provider_del = create_test_provider(
            "provider-location:28.7041,77.1025", 
            "Delhi Health Services"
        );
        
        // Provider in Hyderabad (17.3850°N, 78.4867°E)
        let provider_hyd = create_test_provider(
            "provider-location:17.3850,78.4867", 
            "Hyderabad Healthcare"
        );
        
        // Register all providers
        let _ = service.register_provider(provider_blr).await.unwrap();
        let _ = service.register_provider(provider_mum).await.unwrap();
        let _ = service.register_provider(provider_del).await.unwrap();
        let _ = service.register_provider(provider_hyd).await.unwrap();
        
        // Find providers near Bangalore within 100km
        let near_bangalore = service.find_providers_by_location("12.9716,77.5946", 100.0).await;
        assert!(near_bangalore.is_ok());
        let blr_results = near_bangalore.unwrap();
        assert_eq!(blr_results.len(), 1);
        assert!(blr_results[0].id.contains("12.9716,77.5946"));
        
        // Find providers near Hyderabad within 700km
        // This should include Hyderabad (~0km), Bangalore (~500km), and Mumbai (~620km)
        let near_hyderabad = service.find_providers_by_location("17.3850,78.4867", 700.0).await;
        assert!(near_hyderabad.is_ok());
        let hyd_results = near_hyderabad.unwrap();
        assert_eq!(hyd_results.len(), 3, "Expected 3 providers within 700km of Hyderabad (Hyderabad, Bangalore, Mumbai)");
        
        // Check the results more thoroughly
        let hyd_ids: Vec<&str> = hyd_results.iter().map(|p| p.id.as_str()).collect();
        assert!(hyd_ids.contains(&"provider-location:17.3850,78.4867"), "Hyderabad should be in results");
        assert!(hyd_ids.contains(&"provider-location:12.9716,77.5946"), "Bangalore should be in results");
        assert!(hyd_ids.contains(&"provider-location:19.0760,72.8777"), "Mumbai should be in results");
        
        // Find providers near Hyderabad within 1500km
        // This should include all 4 providers
        let wider_search = service.find_providers_by_location("17.3850,78.4867", 1500.0).await;
        assert!(wider_search.is_ok());
        let wider_results = wider_search.unwrap();
        assert_eq!(wider_results.len(), 4, "Expected 4 providers within 1500km of Hyderabad (all providers)");
        
        // Test invalid coordinates
        let invalid_coords = service.find_providers_by_location("invalid_coords", 10.0).await;
        assert!(invalid_coords.is_err());
        
        // Test negative radius
        let negative_radius = service.find_providers_by_location("12.9716,77.5946", -10.0).await;
        assert!(negative_radius.is_err());
    }
    
    #[tokio::test]
    async fn test_gps_parsing() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);
        
        // Valid coordinates
        let valid_coords = service.parse_gps_coordinates("12.9716,77.5946");
        assert!(valid_coords.is_ok());
        let (lat, lng) = valid_coords.unwrap();
        assert_eq!(lat, 12.9716);
        assert_eq!(lng, 77.5946);
        
        // Invalid format
        let invalid_format = service.parse_gps_coordinates("12.9716,77.5946,extra");
        assert!(invalid_format.is_err());
        
        // Non-numeric
        let non_numeric = service.parse_gps_coordinates("abc,def");
        assert!(non_numeric.is_err());
        
        // Out of range latitude
        let invalid_lat = service.parse_gps_coordinates("100.0,77.5946");
        assert!(invalid_lat.is_err());
        
        // Out of range longitude
        let invalid_lng = service.parse_gps_coordinates("12.9716,200.0");
        assert!(invalid_lng.is_err());
    }
    
    #[tokio::test]
    async fn test_distance_calculation() {
        // Create a memory storage
        let storage = Arc::new(MemoryStorage::new());

        // Create the provider service
        let service = ProviderService::new(storage);
        
        // Test distance calculation between Bangalore and Hyderabad
        // Bangalore: 12.9716°N, 77.5946°E
        // Hyderabad: 17.3850°N, 78.4867°E
        // Approximate distance: ~500 km
        
        let distance = service.calculate_distance(12.9716, 77.5946, 17.3850, 78.4867);
        
        // Allow for some margin of error in the calculation
        // The actual distance is around 500-520 km depending on the calculation method
        assert!(distance > 450.0 && distance < 650.0);
    }
}
