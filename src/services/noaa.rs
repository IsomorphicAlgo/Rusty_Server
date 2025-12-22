use crate::models::*;
use crate::Result;
use crate::services::parsing::*;
use crate::services::DonkiClient;
use chrono::Utc;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn};

/// NOAA Space Weather API client
/// 
/// Fetches space weather data from NOAA Space Weather Prediction Center.
/// Integrates with DONKI client to include solar flare data.
/// 
/// # Data Sources
/// - KP Index (geomagnetic activity)
/// - Solar Wind (speed, density, temperature, Bz)
/// - Solar Flares (via DONKI integration)
/// 
/// # Example
/// ```no_run
/// use rusty_server::services::NoaaClient;
/// 
/// let client = NoaaClient::new(
///     "https://services.swpc.noaa.gov".to_string(),
///     None,
///     30,
/// );
/// 
/// let response = client.get_current_conditions(Some(&donki_client)).await?;
/// ```
#[derive(Clone)]
pub struct NoaaClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
    timeout: Duration,
}

impl NoaaClient {
    /// Create a new NOAA API client
    pub fn new(base_url: String, api_key: Option<String>, timeout_seconds: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            api_key,
            timeout: Duration::from_secs(timeout_seconds),
        }
    }

    /// Fetch current space weather conditions
    /// 
    /// Fetches current space weather data from NOAA and optionally integrates
    /// solar flare data from DONKI.
    /// 
    /// # Arguments
    /// * `donki_client` - Optional DONKI client for fetching solar flare data.
    ///   If provided, will fetch recent flares (last 7 days) and include the most recent one.
    ///   If None, solar flare data will be None.
    /// 
    /// # Returns
    /// `SpaceWeatherResponse` containing:
    /// - KP Index (if available)
    /// - Solar Wind data (if available)
    /// - Solar Flare (if DONKI client provided and flares exist)
    /// - Geomagnetic Storm (derived from KP index)
    /// 
    /// # Errors
    /// Returns error if all data sources fail. Individual data source failures
    /// are logged as warnings but don't fail the entire request.
    /// 
    /// # Example
    /// ```no_run
    /// let response = noaa_client.get_current_conditions(Some(&donki_client)).await?;
    /// // Response will include solar flare if DONKI has recent flare data
    /// ```
    pub async fn get_current_conditions(
        &self,
        donki_client: Option<&DonkiClient>,
    ) -> Result<SpaceWeatherResponse> {
        info!("Fetching current space weather conditions from NOAA");

        // Fetch multiple data sources in parallel
        let (kp_index, solar_wind, _alerts) = tokio::try_join!(
            self.fetch_kp_index(),
            self.fetch_solar_wind(),
            self.fetch_alerts()
        )?;

        // Fetch solar flares from DONKI if client is available
        let solar_flare = if let Some(donki) = donki_client {
            // Fetch recent flares (last 7 days) and get the most recent one
            match donki.fetch_recent_solar_flares(7).await {
                Ok(mut flares) => {
                    // Sort by peak_time descending (most recent first) and get the first one
                    flares.sort_by(|a, b| b.peak_time.cmp(&a.peak_time));
                    flares.into_iter().next()
                }
                Err(e) => {
                    warn!("Failed to fetch solar flares from DONKI: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let source = if solar_flare.is_some() { "noaa,donki" } else { "noaa" }.to_string();
        
        let response = SpaceWeatherResponse {
            data: SpaceWeatherData {
                solar_flare,
                geomagnetic_storm: kp_index.as_ref().map(|kp| GeomagneticStorm {
                    level: kp_to_geomagnetic_level(kp.value),
                    start_time: None, // NOAA doesn't provide this directly
                    end_time: None,
                    kp_index: kp.value,
                }),
                radiation: None, // Will be implemented when we find the endpoint
                solar_wind,
                kp_index,
            },
            metadata: ResponseMetadata {
                timestamp: Utc::now(),
                source,
                cached: false,
            },
        };

        // Validate the parsed data
        if let Err(e) = validate_space_weather_data(&response.data) {
            warn!("Validation failed for space weather data: {}", e);
            // Continue anyway - validation is a warning, not a hard error
        }

        Ok(response)
    }

    /// Fetch KP index (geomagnetic activity)
    async fn fetch_kp_index(&self) -> Result<Option<KpIndex>> {
        let url = format!("{}/json/planetary_k_index_1m.json", self.base_url);
        
        match self.fetch_with_retry(&url).await {
            Ok(json) => {
                // Use the parsing module to parse KP index
                if let Some(latest) = parse_kp_index(&json)? {
                    info!("Fetched KP index: {}", latest.value);
                    Ok(Some(latest))
                } else {
                    warn!("No KP index data available");
                    Ok(None)
                }
            }
            Err(e) => {
                warn!("Failed to fetch KP index: {}", e);
                Ok(None) // Return None instead of error for graceful degradation
            }
        }
    }

    /// Fetch solar wind data
    async fn fetch_solar_wind(&self) -> Result<Option<SolarWind>> {
        let mag_url = format!("{}/json/rtsw/rtsw_mag_1m.json", self.base_url);
        let plasma_url = format!("{}/json/rtsw/rtsw_plasma_1m.json", self.base_url);
        
        // Fetch both magnetic field data and plasma data in parallel
        let (mag_result, plasma_result) = tokio::join!(
            self.fetch_with_retry(&mag_url),
            self.fetch_with_retry(&plasma_url)
        );
        
        let mag_json = match mag_result {
            Ok(json) => json,
            Err(e) => {
                warn!("Failed to fetch solar wind magnetic data: {}", e);
                return Ok(None);
            }
        };
        
        // Plasma data is optional - if it fails, we can still return Bz data
        let plasma_json = match plasma_result {
            Ok(json) => json,
            Err(e) => {
                warn!("Failed to fetch solar wind plasma data: {}, continuing with magnetic data only", e);
                serde_json::Value::Null
            }
        };
        
        if let Some(wind) = parse_solar_wind(&mag_json, &plasma_json)? {
            if wind.speed == 0.0 && wind.density == 0.0 && wind.temperature == 0.0 {
                warn!("Solar wind speed/density/temperature not available from NOAA API (plasma data may be unavailable)");
            }
            info!("Fetched solar wind data: speed={} km/s, density={} cm^-3, temp={} K, bz={:?}", 
                  wind.speed, wind.density, wind.temperature, wind.bz);
            Ok(Some(wind))
        } else {
            warn!("No solar wind data available");
            Ok(None)
        }
    }

    /// Fetch active alerts
    async fn fetch_alerts(&self) -> Result<Vec<SpaceWeatherResponse>> {
        // NOAA alert endpoint - currently not available, return empty
        // TODO: Find correct alerts endpoint or implement alternative alert source
        Ok(Vec::new())
    }

    /// Fetch data with retry logic
    async fn fetch_with_retry(&self, url: &str) -> Result<serde_json::Value> {
        let max_retries = 3;
        let mut last_error = None;

        for attempt in 1..=max_retries {
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let json: serde_json::Value = response.json().await?;
                        return Ok(json);
                    } else {
                        let status = response.status();
                        last_error = Some(format!("HTTP {}: {}", status, response.status().canonical_reason().unwrap_or("Unknown")));
                        
                        if status.is_client_error() {
                            // Don't retry on client errors (4xx)
                            break;
                        }
                    }
                }
                Err(e) => {
                    last_error = Some(format!("Network error: {}", e));
                }
            }

            if attempt < max_retries {
                let delay = Duration::from_millis(100 * (1 << (attempt - 1))); // Exponential backoff
                warn!("Request failed (attempt {}/{}), retrying in {:?}...", attempt, max_retries, delay);
                tokio::time::sleep(delay).await;
            }
        }

        Err(crate::AppError::Internal(
            format!("Failed to fetch {} after {} attempts: {:?}", url, max_retries, last_error.unwrap_or_else(|| "Unknown error".to_string()))
        ))
    }

}


