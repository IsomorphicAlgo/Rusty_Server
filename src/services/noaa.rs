use crate::models::*;
use crate::Result;
use crate::services::parsing::*;
use chrono::Utc;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn};

/// NOAA Space Weather API client
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
    pub async fn get_current_conditions(&self) -> Result<SpaceWeatherResponse> {
        info!("Fetching current space weather conditions from NOAA");

        // Fetch multiple data sources in parallel
        let (kp_index, solar_wind, _alerts) = tokio::try_join!(
            self.fetch_kp_index(),
            self.fetch_solar_wind(),
            self.fetch_alerts()
        )?;

        let response = SpaceWeatherResponse {
            data: SpaceWeatherData {
                solar_flare: None, // Will be implemented when we find the endpoint
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
                source: "noaa".to_string(),
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
        
        // Fetch magnetic field data (plasma endpoint doesn't exist, so we'll use what we have)
        match self.fetch_with_retry(&mag_url).await {
            Ok(mag_json) => {
                // Parse with empty plasma data since that endpoint doesn't exist
                if let Some(wind) = parse_solar_wind(&mag_json, &serde_json::Value::Null)? {
                    if wind.speed == 0.0 {
                        warn!("Solar wind speed/density/temperature not available from NOAA API");
                    }
                    info!("Fetched solar wind data: speed={} km/s, bz={:?}", wind.speed, wind.bz);
                    Ok(Some(wind))
                } else {
                    warn!("No solar wind data available");
                    Ok(None)
                }
            }
            Err(e) => {
                warn!("Failed to fetch solar wind: {}", e);
                Ok(None)
            }
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


