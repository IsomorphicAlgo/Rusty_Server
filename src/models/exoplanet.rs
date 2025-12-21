use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Exoplanet data response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExoplanetResponse {
    pub data: Vec<Exoplanet>,
    pub metadata: ExoplanetMetadata,
}

/// Exoplanet metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExoplanetMetadata {
    pub count: usize,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

/// Exoplanet information from Planetary Systems (ps) table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exoplanet {
    /// Planet name (e.g., "Kepler-186 f")
    pub pl_name: String,
    
    /// Host star name (e.g., "Kepler-186")
    pub hostname: String,
    
    /// Discovery method (e.g., "Transit", "Radial Velocity")
    pub discoverymethod: Option<String>,
    
    /// Discovery year
    pub disc_year: Option<i32>,
    
    /// Discovery facility
    pub disc_facility: Option<String>,
    
    /// Discovery telescope
    pub disc_telescope: Option<String>,
    
    /// Orbital period in days
    pub pl_orbper: Option<f64>,
    
    /// Orbital period error (upper)
    pub pl_orbpererr1: Option<f64>,
    
    /// Orbital period error (lower)
    pub pl_orbpererr2: Option<f64>,
    
    /// Orbital period limit flag
    pub pl_orbperlim: Option<f64>,
    
    /// Planet radius in Earth radii
    pub pl_rade: Option<f64>,
    
    /// Planet radius error (upper)
    pub pl_radeerr1: Option<f64>,
    
    /// Planet radius error (lower)
    pub pl_radeerr2: Option<f64>,
    
    /// Planet mass in Earth masses
    pub pl_bmasse: Option<f64>,
    
    /// Planet mass error (upper)
    pub pl_bmasseerr1: Option<f64>,
    
    /// Planet mass error (lower)
    pub pl_bmasseerr2: Option<f64>,
    
    /// Equilibrium temperature in Kelvin
    pub pl_eqt: Option<f64>,
    
    /// Stellar effective temperature in Kelvin
    pub st_teff: Option<f64>,
    
    /// Stellar radius in solar radii
    pub st_rad: Option<f64>,
    
    /// Stellar mass in solar masses
    pub st_mass: Option<f64>,
    
    /// Distance to star in parsecs
    pub sy_dist: Option<f64>,
    
    /// Number of planets in system
    pub sy_pnum: Option<i32>,
    
    /// Last updated timestamp
    pub rowupdate: Option<String>,
    
    /// Release date
    pub releasedate: Option<String>,
}

/// Exoplanet query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExoplanetQueryParams {
    /// Limit number of results
    pub limit: Option<usize>,
    
    /// Offset for pagination
    pub offset: Option<usize>,
    
    /// Filter by discovery method
    pub discovery_method: Option<String>,
    
    /// Filter by minimum discovery year
    pub min_year: Option<i32>,
    
    /// Filter by maximum discovery year
    pub max_year: Option<i32>,
    
    /// Filter by host star name
    pub hostname: Option<String>,
    
    /// Filter by minimum planet radius (Earth radii)
    pub min_radius: Option<f64>,
    
    /// Filter by maximum planet radius (Earth radii)
    pub max_radius: Option<f64>,
    
    /// Filter by minimum planet mass (Earth masses)
    pub min_mass: Option<f64>,
    
    /// Filter by maximum planet mass (Earth masses)
    pub max_mass: Option<f64>,
    
    /// Sort field (e.g., "pl_name", "disc_year", "pl_rade")
    pub sort_by: Option<String>,
    
    /// Sort order ("asc" or "desc")
    pub sort_order: Option<String>,
}

impl Default for ExoplanetQueryParams {
    fn default() -> Self {
        Self {
            limit: Some(100),
            offset: Some(0),
            discovery_method: None,
            min_year: None,
            max_year: None,
            hostname: None,
            min_radius: None,
            max_radius: None,
            min_mass: None,
            max_mass: None,
            sort_by: Some("pl_name".to_string()),
            sort_order: Some("asc".to_string()),
        }
    }
}

/// Discovery notification tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryNotification {
    pub id: u64,
    pub pl_name: String,
    pub hostname: String,
    pub discovery_year: Option<i32>,
    pub discovery_method: Option<String>,
    pub notification_sent: bool,
    pub created_at: DateTime<Utc>,
    pub notified_at: Option<DateTime<Utc>>,
}

