use crate::models::exoplanet::{Exoplanet, ExoplanetQueryParams};
use crate::Result;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn, error};
use urlencoding::encode;

/// NASA Exoplanet Archive TAP (Table Access Protocol) client
/// 
/// The Exoplanet Archive provides programmatic access via TAP using ADQL (Astronomical Data Query Language).
/// This client queries the Planetary Systems (ps) table for confirmed exoplanets.
/// 
/// # Example
/// ```no_run
/// use rusty_server::services::ExoplanetClient;
/// use rusty_server::models::ExoplanetQueryParams;
/// 
/// let client = ExoplanetClient::new(
///     "https://exoplanetarchive.ipac.caltech.edu/TAP".to_string(),
///     60,
/// );
/// 
/// let params = ExoplanetQueryParams {
///     limit: Some(10),
///     discovery_method: Some("Transit".to_string()),
///     ..Default::default()
/// };
/// 
/// let exoplanets = client.query_exoplanets(&params).await?;
/// ```
#[derive(Clone)]
pub struct ExoplanetClient {
    client: Client,
    base_url: String,
    timeout: Duration,
}

impl ExoplanetClient {
    /// Create a new Exoplanet Archive TAP client
    /// 
    /// # Arguments
    /// * `base_url` - Base URL for TAP service (typically "https://exoplanetarchive.ipac.caltech.edu/TAP")
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

    /// Query exoplanets from the Planetary Systems (ps) table
    /// 
    /// Builds an ADQL query based on the provided parameters and executes it via TAP.
    /// 
    /// # Arguments
    /// * `params` - Query parameters for filtering and sorting
    /// 
    /// # Returns
    /// Returns a vector of `Exoplanet` objects matching the query criteria.
    /// 
    /// # Errors
    /// Returns an error if:
    /// - Network request fails after retries
    /// - TAP service returns an error
    /// - Response cannot be parsed
    /// 
    /// # Example
    /// ```no_run
    /// let params = ExoplanetQueryParams {
    ///     limit: Some(50),
    ///     min_year: Some(2020),
    ///     discovery_method: Some("Transit".to_string()),
    ///     ..Default::default()
    /// };
    /// 
    /// let exoplanets = client.query_exoplanets(&params).await?;
    /// ```
    pub async fn query_exoplanets(&self, params: &ExoplanetQueryParams) -> Result<Vec<Exoplanet>> {
        let adql_query = self.build_adql_query(params);
        info!("Executing TAP query: {}", adql_query);
        
        let url = format!("{}/sync?query={}&format=json", 
            self.base_url, 
            encode(&adql_query)
        );

        let response = self.fetch_with_retry(&url).await?;
        
        // TAP returns data in a specific format
        // The response is typically an array of objects, or wrapped in a "data" field
        let exoplanets: Vec<Exoplanet> = if let Some(data_array) = response.as_array() {
            serde_json::from_value(serde_json::Value::Array(data_array.clone()))
                .map_err(|e| {
                    error!("Failed to parse TAP response: {}", e);
                    crate::AppError::Internal(format!("Failed to parse TAP response: {}", e))
                })?
        } else if let Some(data_field) = response.get("data") {
            serde_json::from_value(data_field.clone())
                .map_err(|e| {
                    error!("Failed to parse TAP response data field: {}", e);
                    crate::AppError::Internal(format!("Failed to parse TAP response: {}", e))
                })?
        } else {
            // Try to parse the whole response as an array
            serde_json::from_value(response.clone())
                .map_err(|e| {
                    error!("Failed to parse TAP response: {}", e);
                    crate::AppError::Internal(format!("Failed to parse TAP response: {}", e))
                })?
        };

        info!("Retrieved {} exoplanets from TAP service", exoplanets.len());
        Ok(exoplanets)
    }

    /// Build ADQL query from query parameters
    fn build_adql_query(&self, params: &ExoplanetQueryParams) -> String {
        let limit = params.limit.unwrap_or(100);
        let offset = params.offset.unwrap_or(0);
        
        // Build query with TOP (ADQL standard syntax)
        // TOP must come immediately after SELECT
        let mut query = format!(
            "SELECT TOP {} pl_name, hostname, discoverymethod, disc_year, disc_facility, disc_telescope, \
             pl_orbper, pl_orbpererr1, pl_orbpererr2, pl_orbperlim, \
             pl_rade, pl_radeerr1, pl_radeerr2, \
             pl_bmasse, pl_bmasseerr1, pl_bmasseerr2, \
             pl_eqt, st_teff, st_rad, st_mass, sy_dist, sy_pnum, rowupdate, releasedate \
             FROM ps WHERE 1=1",
            limit
        );

        // Add filters
        if let Some(ref method) = params.discovery_method {
            query.push_str(&format!(" AND discoverymethod = '{}'", method.replace("'", "''")));
        }

        if let Some(year) = params.min_year {
            query.push_str(&format!(" AND disc_year >= {}", year));
        }

        if let Some(year) = params.max_year {
            query.push_str(&format!(" AND disc_year <= {}", year));
        }

        if let Some(ref host) = params.hostname {
            query.push_str(&format!(" AND hostname LIKE '%{}%'", host.replace("'", "''")));
        }

        if let Some(radius) = params.min_radius {
            query.push_str(&format!(" AND pl_rade >= {}", radius));
        }

        if let Some(radius) = params.max_radius {
            query.push_str(&format!(" AND pl_rade <= {}", radius));
        }

        if let Some(mass) = params.min_mass {
            query.push_str(&format!(" AND pl_bmasse >= {}", mass));
        }

        if let Some(mass) = params.max_mass {
            query.push_str(&format!(" AND pl_bmasse <= {}", mass));
        }

        // Add sorting
        let sort_field = params.sort_by.as_deref().unwrap_or("pl_name");
        let sort_order = params.sort_order.as_deref().unwrap_or("asc");
        query.push_str(&format!(" ORDER BY {} {}", sort_field, sort_order.to_uppercase()));
        
        // Note: OFFSET is not well supported in ADQL, so we'll handle pagination
        // by adjusting the query in the calling code if needed
        if offset > 0 {
            warn!("OFFSET {} requested but ADQL doesn't support it well - using TOP only", offset);
        }
        
        query
    }

    /// Fetch data from TAP service with retry logic
    /// 
    /// Retries up to 3 times with exponential backoff on failure.
    /// Returns error if all retry attempts fail.
    async fn fetch_with_retry(&self, url: &str) -> Result<serde_json::Value> {
        let max_retries = 3;
        let mut last_error = None;

        for attempt in 1..=max_retries {
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        // Get response as text first to handle trailing commas
                        let text = response.text().await?;
                        
                        // Clean up trailing commas (common issue with TAP services)
                        let cleaned_text = text.trim_end()
                            .trim_end_matches(',')
                            .trim_end();
                        
                        // Parse as JSON
                        let json: serde_json::Value = serde_json::from_str(&cleaned_text)
                            .map_err(|e| {
                                error!("Failed to parse TAP JSON response: {}", e);
                                error!("Response preview (first 500 chars): {}", 
                                    cleaned_text.chars().take(500).collect::<String>());
                                crate::AppError::Internal(format!("Failed to parse TAP response: {}", e))
                            })?;
                        return Ok(json);
                    } else {
                        let status = response.status();
                        let status_text = response.status().canonical_reason().unwrap_or("Unknown");
                        let body_text = response.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());
                        
                        last_error = Some(format!(
                            "HTTP {}: {} - {}",
                            status, status_text, body_text
                        ));

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
                warn!(
                    "TAP request failed (attempt {}/{}), retrying in {:?}...",
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

    /// Get all confirmed exoplanets (simplified query)
    /// 
    /// Fetches all confirmed exoplanets from the ps table with a reasonable limit.
    /// 
    /// # Arguments
    /// * `limit` - Maximum number of exoplanets to return (default: 1000)
    /// 
    /// # Returns
    /// Returns a vector of `Exoplanet` objects.
    pub async fn get_all_exoplanets(&self, limit: Option<usize>) -> Result<Vec<Exoplanet>> {
        let params = ExoplanetQueryParams {
            limit: limit.or(Some(1000)),
            ..Default::default()
        };
        self.query_exoplanets(&params).await
    }

    /// Get recently discovered exoplanets
    /// 
    /// Fetches exoplanets discovered in the specified year range.
    /// 
    /// # Arguments
    /// * `min_year` - Minimum discovery year
    /// * `max_year` - Maximum discovery year
    /// * `limit` - Maximum number of results
    /// 
    /// # Returns
    /// Returns a vector of `Exoplanet` objects.
    pub async fn get_recent_discoveries(
        &self,
        min_year: i32,
        max_year: Option<i32>,
        limit: Option<usize>,
    ) -> Result<Vec<Exoplanet>> {
        let params = ExoplanetQueryParams {
            min_year: Some(min_year),
            max_year,
            limit: limit.or(Some(100)),
            sort_by: Some("disc_year".to_string()),
            sort_order: Some("desc".to_string()),
            ..Default::default()
        };
        self.query_exoplanets(&params).await
    }
}

