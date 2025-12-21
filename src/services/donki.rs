use crate::models::*;
use crate::Result;
use chrono::{DateTime, Utc, Duration as ChronoDuration};
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn};

/// NASA DONKI (Database of Notifications, Knowledge, Information) Space Weather API client
/// 
/// DONKI provides comprehensive space weather event data including:
/// - Solar flares (FLR)
/// - Coronal mass ejections (CME)
/// - Geomagnetic storms (GST)
/// - Interplanetary shocks (IPS)
/// - High speed streams (HSS)
/// 
/// This client implements the FLR endpoint for solar flare data.
/// 
/// # Example
/// ```no_run
/// use rusty_server::services::DonkiClient;
/// use chrono::Utc;
/// 
/// let client = DonkiClient::new(
///     "https://api.nasa.gov/DONKI".to_string(),
///     Some("your-api-key".to_string()),
///     30,
/// );
/// 
/// let flares = client.fetch_recent_solar_flares(7).await?;
/// ```
#[derive(Clone)]
pub struct DonkiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
    timeout: Duration,
}

impl DonkiClient {
    /// Create a new DONKI API client
    /// 
    /// # Arguments
    /// * `base_url` - Base URL for DONKI API (typically "https://api.nasa.gov/DONKI")
    /// * `api_key` - NASA API key (required for DONKI endpoints, get free key from https://api.nasa.gov)
    /// * `timeout_seconds` - Request timeout in seconds
    /// 
    /// # Panics
    /// Panics if the HTTP client cannot be created
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

    /// Fetch solar flares (FLR endpoint)
    /// 
    /// Fetches solar flare data for the specified date range from NASA DONKI.
    /// 
    /// # Arguments
    /// * `start_date` - Start date for the query (required by DONKI API)
    /// * `end_date` - End date for the query (optional, defaults to today)
    /// 
    /// # Returns
    /// Returns a vector of `SolarFlare` objects, sorted by peak_time (most recent first).
    /// Returns empty vector if no flares occurred in the date range or if API call fails.
    /// 
    /// # Errors
    /// Returns an error if:
    /// - API key is not configured
    /// - Network request fails after retries
    /// - Response cannot be parsed
    /// 
    /// # Example
    /// ```no_run
    /// use chrono::{Utc, Duration as ChronoDuration};
    /// 
    /// let start = Utc::now() - ChronoDuration::days(7);
    /// let flares = client.fetch_solar_flares(start, None).await?;
    /// ```
    pub async fn fetch_solar_flares(
        &self,
        start_date: DateTime<Utc>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<SolarFlare>> {
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            crate::AppError::Config(config::ConfigError::Message(
                "DONKI API key is required but not configured".to_string(),
            ))
        })?;

        let start_date_str = start_date.format("%Y-%m-%d").to_string();
        let end_date_str = end_date
            .unwrap_or_else(Utc::now)
            .format("%Y-%m-%d")
            .to_string();

        let url = format!(
            "{}/FLR?startDate={}&endDate={}&api_key={}",
            self.base_url, start_date_str, end_date_str, api_key
        );

        info!("Fetching solar flares from DONKI: {} to {}", start_date_str, end_date_str);

        match self.fetch_with_retry(&url).await {
            Ok(json) => {
                // DONKI returns an array of flare events
                if let Some(flares_array) = json.as_array() {
                    let mut flares = Vec::new();
                    for flare_json in flares_array {
                        if let Some(flare) = crate::services::parsing::parse_donki_flare(flare_json)? {
                            flares.push(flare);
                        }
                    }
                    info!("Fetched {} solar flares from DONKI", flares.len());
                    Ok(flares)
                } else {
                    warn!("DONKI returned non-array response for solar flares");
                    Ok(Vec::new())
                }
            }
            Err(e) => {
                warn!("Failed to fetch solar flares from DONKI: {}", e);
                Ok(Vec::new()) // Return empty vec instead of error for graceful degradation
            }
        }
    }

    /// Fetch the most recent solar flares (last N days)
    /// 
    /// Convenience method that fetches flares from `(today - days)` to today.
    /// 
    /// # Arguments
    /// * `days` - Number of days to look back (e.g., 7 for last week)
    /// 
    /// # Returns
    /// Vector of solar flares from the specified time period, sorted by peak_time (most recent first).
    pub async fn fetch_recent_solar_flares(&self, days: i64) -> Result<Vec<SolarFlare>> {
        let end_date = Utc::now();
        let start_date = end_date - ChronoDuration::days(days);
        self.fetch_solar_flares(start_date, Some(end_date)).await
    }

    /// Fetch data with retry logic and rate limit compliance
    /// 
    /// Implements exponential backoff retry pattern with NASA API rate limit compliance.
    /// - Checks X-RateLimit-Remaining headers
    /// - Handles 429 (Too Many Requests) with proper backoff
    /// - Retries up to 3 times on network errors or server errors (5xx)
    /// - Does not retry on client errors (4xx) except 429
    /// 
    /// # Arguments
    /// * `url` - Full URL to fetch
    /// 
    /// # Returns
    /// JSON response value on success
    /// 
    /// # Errors
    /// Returns error if all retry attempts fail
    async fn fetch_with_retry(&self, url: &str) -> Result<serde_json::Value> {
        let max_retries = 3;
        let mut last_error = None;

        for attempt in 1..=max_retries {
            match self.client.get(url).send().await {
                Ok(response) => {
                    // Check rate limit headers (NASA API provides these)
                    if let Some(remaining) = response.headers().get("X-RateLimit-Remaining") {
                        if let Ok(remaining_str) = remaining.to_str() {
                            if let Ok(remaining_count) = remaining_str.parse::<u32>() {
                                if remaining_count < 10 {
                                    warn!("DONKI API rate limit low: {} requests remaining", remaining_count);
                                }
                            }
                        }
                    }

                    let status = response.status();
                    
                    if status.is_success() {
                        let json: serde_json::Value = response.json().await?;
                        return Ok(json);
                    } else if status == 429 {
                        // Rate limit exceeded - wait longer before retry
                        let retry_after = response.headers()
                            .get("Retry-After")
                            .and_then(|h| h.to_str().ok())
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(3600); // Default: wait 1 hour if not specified
                        
                        last_error = Some(format!(
                            "Rate limit exceeded (429). Retry after {} seconds",
                            retry_after
                        ));

                        if attempt < max_retries {
                            warn!(
                                "DONKI rate limit exceeded (attempt {}/{}). Waiting {} seconds before retry...",
                                attempt, max_retries, retry_after
                            );
                            tokio::time::sleep(Duration::from_secs(retry_after)).await;
                            continue;
                        }
                    } else {
                        last_error = Some(format!(
                            "HTTP {}: {}",
                            status,
                            response.status().canonical_reason().unwrap_or("Unknown")
                        ));

                        if status.is_client_error() && status != 429 {
                            // Don't retry on other client errors (4xx)
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
                warn!(
                    "DONKI request failed (attempt {}/{}), retrying in {:?}...",
                    attempt, max_retries, delay
                );
                tokio::time::sleep(delay).await;
            }
        }

        Err(crate::AppError::Internal(format!(
            "Failed to fetch {} after {} attempts: {:?}",
            url,
            max_retries,
            last_error.unwrap_or_else(|| "Unknown error".to_string())
        )))
    }
}
