use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provider descriptor with basic identifying information
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Category for classifying health services
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Provider model representing a healthcare service provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    /// Unique ID for the provider
    pub id: String,

    /// Provider descriptor with name and other metadata
    pub descriptor: Descriptor,

    /// Categories of services offered by the provider
    pub categories: Vec<Category>,

    /// Time when the provider was created
    pub created_at: DateTime<Utc>,

    /// Time when the provider was last updated
    pub updated_at: DateTime<Utc>,
}

/// Location information for a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Unique ID for the location
    pub id: String,

    /// Descriptive information about the location
    pub descriptor: Descriptor,

    /// GPS coordinates of the location
    pub gps: String,

    /// Full address as text
    pub address: Option<String>,

    /// City where the location is
    pub city: Option<String>,

    /// State/province where the location is
    pub state: Option<String>,

    /// Country where the location is
    pub country: Option<String>,

    /// Area code or pincode
    pub area_code: Option<String>,
}

/// Circle representing a service area
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    /// GPS coordinates for the center of the circle
    pub gps: String,

    /// Radius of the circle in meters
    pub radius: Option<f64>,
}

/// Provider service area
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceArea {
    /// Type of service area (e.g., "circle", "polygon")
    pub service_area_type: String,

    /// Circle defining the service area (if type is "circle")
    pub circle: Option<Circle>,

    /// Polygon points defining the service area (if type is "polygon")
    pub polygon: Option<String>,

    /// 3-letter country code for the service area
    pub country: Option<String>,
}
