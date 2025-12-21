use crate::models::SpaceWeatherData;
use crate::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, warn, error};

/// ML Service client for solar flare prediction
/// 
/// Communicates with Python ML microservice via HTTP REST API.
/// 
/// # Example
/// ```no_run
/// use rusty_server::services::MLServiceClient;
/// use rusty_server::models::SpaceWeatherData;
/// 
/// let client = MLServiceClient::new(
///     "http://localhost:8001".to_string(),
///     30,
/// );
/// 
/// let prediction = client.predict_solar_flare(&space_weather_data).await?;
/// ```
#[derive(Clone)]
pub struct MLServiceClient {
    client: Client,
    base_url: String,
    timeout: Duration,
}

/// Prediction request to ML service
#[derive(Debug, Serialize)]
struct PredictionRequest {
    features: PredictionFeatures,
    timestamp: Option<DateTime<Utc>>,
}

/// Features for prediction
#[derive(Debug, Serialize)]
struct PredictionFeatures {
    kp_index: Option<f64>,
    solar_wind_speed: Option<f64>,
    solar_wind_density: Option<f64>,
    solar_wind_temperature: Option<f64>,
    solar_wind_bz: Option<f64>,
    radiation_proton_flux: Option<f64>,
    radiation_electron_flux: Option<f64>,
    days_since_last_flare: Option<f64>,
    flare_count_last_7_days: Option<i32>,
    flare_count_last_30_days: Option<i32>,
}

/// Prediction response from ML service
#[derive(Debug, Deserialize)]
pub struct PredictionResponse {
    pub predicted_flare_class: Option<String>,
    pub predicted_peak_time: Option<DateTime<Utc>>,
    pub confidence_score: f64,
    pub model_version: String,
    pub prediction_timestamp: DateTime<Utc>,
    pub features_used: Vec<String>,
}

/// Health check response
#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
    model_loaded: bool,
    model_version: Option<String>,
}

impl MLServiceClient {
    /// Create a new ML service client
    /// 
    /// # Arguments
    /// * `base_url` - Base URL for ML service (typically "http://localhost:8001")
    /// * `timeout_seconds` - Request timeout in seconds
    /// 
    /// # Panics
    /// Panics if the HTTP client cannot be created
    pub fn new(base_url: String, timeout_seconds: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            timeout: Duration::from_secs(timeout_seconds),
        }
    }

    /// Check if ML service is healthy and model is loaded
    /// 
    /// # Returns
    /// Returns true if service is healthy and model is loaded, false otherwise
    pub async fn health_check(&self) -> bool {
        let url = format!("{}/health", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<HealthResponse>().await {
                        Ok(health) => {
                            if health.model_loaded {
                                info!("ML service healthy, model version: {:?}", health.model_version);
                                true
                            } else {
                                warn!("ML service healthy but no model loaded");
                                false
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse health response: {}", e);
                            false
                        }
                    }
                } else {
                    warn!("ML service health check failed: HTTP {}", response.status());
                    false
                }
            }
            Err(e) => {
                warn!("ML service health check failed: {}", e);
                false
            }
        }
    }

    /// Predict solar flare based on current space weather conditions
    /// 
    /// # Arguments
    /// * `space_weather_data` - Current space weather data
    /// * `days_since_last_flare` - Optional: days since last flare
    /// * `flare_counts` - Optional: (last_7_days, last_30_days) flare counts
    /// 
    /// # Returns
    /// Prediction response with flare class, confidence, and timing
    /// 
    /// # Errors
    /// Returns error if:
    /// - ML service is unavailable
    /// - Model is not loaded
    /// - Request fails
    pub async fn predict_solar_flare(
        &self,
        space_weather_data: &SpaceWeatherData,
        days_since_last_flare: Option<f64>,
        flare_counts: Option<(i32, i32)>,
    ) -> Result<PredictionResponse> {
        // Extract features from space weather data
        let features = PredictionFeatures {
            kp_index: space_weather_data.kp_index.as_ref().map(|kp| kp.value),
            solar_wind_speed: space_weather_data.solar_wind.as_ref().map(|sw| sw.speed),
            solar_wind_density: space_weather_data.solar_wind.as_ref().map(|sw| sw.density),
            solar_wind_temperature: space_weather_data.solar_wind.as_ref().map(|sw| sw.temperature),
            solar_wind_bz: space_weather_data.solar_wind.as_ref().and_then(|sw| sw.bz),
            radiation_proton_flux: space_weather_data.radiation.as_ref().and_then(|r| r.proton_flux),
            radiation_electron_flux: space_weather_data.radiation.as_ref().and_then(|r| r.electron_flux),
            days_since_last_flare,
            flare_count_last_7_days: flare_counts.map(|c| c.0),
            flare_count_last_30_days: flare_counts.map(|c| c.1),
        };

        let request = PredictionRequest {
            features,
            timestamp: Some(Utc::now()),
        };

        let url = format!("{}/predict", self.base_url);
        
        info!("Requesting prediction from ML service: {}", url);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to call ML service: {}", e);
                crate::AppError::Internal(format!("ML service request failed: {}", e))
            })?;

        if response.status().is_success() {
            let prediction: PredictionResponse = response.json().await.map_err(|e| {
                error!("Failed to parse ML service response: {}", e);
                crate::AppError::Internal(format!("Failed to parse ML service response: {}", e))
            })?;

            info!(
                "Prediction received: class={:?}, confidence={:.2}, model={}",
                prediction.predicted_flare_class,
                prediction.confidence_score,
                prediction.model_version
            );

            Ok(prediction)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            if status == 503 {
                warn!("ML service model not loaded: {}", error_text);
                Err(crate::AppError::Internal(
                    "ML service model not available. Train a model first.".to_string()
                ))
            } else {
                error!("ML service error: HTTP {} - {}", status, error_text);
                Err(crate::AppError::Internal(format!(
                    "ML service error: HTTP {} - {}",
                    status, error_text
                )))
            }
        }
    }
}

