use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Space weather data response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceWeatherResponse {
    pub data: SpaceWeatherData,
    pub metadata: ResponseMetadata,
}

/// Space weather data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceWeatherData {
    pub solar_flare: Option<SolarFlare>,
    pub geomagnetic_storm: Option<GeomagneticStorm>,
    pub radiation: Option<RadiationLevels>,
    pub solar_wind: Option<SolarWind>,
    pub kp_index: Option<KpIndex>,
}

/// Solar flare information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarFlare {
    pub class: String, // X, M, C, B, A
    pub peak_time: DateTime<Utc>,
    pub begin_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub source_location: Option<String>,
}

/// Geomagnetic storm information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomagneticStorm {
    pub level: String, // G5, G4, G3, G2, G1, or None
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub kp_index: f64,
}

/// Radiation levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationLevels {
    pub proton_flux: Option<f64>, // particles/cm²/s
    pub electron_flux: Option<f64>, // particles/cm²/s
    pub alert_level: String, // S1-S5 or None
    pub timestamp: DateTime<Utc>,
}

/// Solar wind data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarWind {
    pub speed: f64, // km/s
    pub density: f64, // particles/cm³
    pub temperature: f64, // K
    pub bz: Option<f64>, // nT (magnetic field z-component)
    pub timestamp: DateTime<Utc>,
}

/// KP index (geomagnetic activity)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KpIndex {
    pub value: f64, // 0-9 scale
    pub level: String, // Quiet, Unsettled, Active, Minor, Moderate, Strong, Severe, Extreme
    pub timestamp: DateTime<Utc>,
}

/// Response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub timestamp: DateTime<Utc>,
    pub source: String, // "noaa", "mock", etc.
    pub cached: bool,
}

/// Historical data query parameters
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalQuery {
    pub start_date: Option<String>, // ISO 8601 format
    pub end_date: Option<String>, // ISO 8601 format
    pub data_type: Option<String>, // "solar_flare", "geomagnetic", "radiation", etc.
    pub limit: Option<u32>, // Max number of records
    pub offset: Option<u32>, // Pagination offset
}

/// Alert query parameters
#[derive(Debug, Clone, Deserialize)]
pub struct AlertQuery {
    pub severity: Option<String>, // "minor", "moderate", "strong", "severe", "extreme"
    pub alert_type: Option<String>, // "solar_flare", "geomagnetic_storm", "radiation"
    pub active_only: Option<bool>, // Only return active alerts
}

/// Radiation query parameters
#[derive(Debug, Clone, Deserialize)]
pub struct RadiationQuery {
    pub threshold: Option<f64>, // Minimum flux value
    pub alert_level: Option<String>, // S1-S5
}

