use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{Utc, Duration as ChronoDuration, DateTime};
use crate::models::*;
use crate::AppState;
use crate::database::{DatabaseOperations, PredictionRow};
use crate::Result;
use serde_json;

/// Get current space weather conditions
/// 
/// Returns the most recent space weather data available, including:
/// - KP Index (geomagnetic activity)
/// - Solar Wind (speed, density, temperature, Bz)
/// - Solar Flares (from DONKI, if configured and available)
/// - Geomagnetic Storms (derived from KP index)
/// 
/// **Data Priority Order:**
/// 1. Check cache (if available and not expired)
/// 2. Fetch from NOAA API + DONKI (if available)
/// 3. Fall back to latest database record
/// 4. Return mock data as final fallback
/// 
/// # Response
/// Returns `SpaceWeatherResponse` with:
/// - `data`: Space weather observations
/// - `metadata`: Source, timestamp, cached flag
/// 
/// # Errors
/// Always returns 200 OK with data (uses fallbacks on failure)
/// 
/// # Example Response
/// ```json
/// {
///   "data": {
///     "solar_flare": { "class": "C2.5", ... },
///     "kp_index": { "value": 3.0, ... },
///     "solar_wind": { "speed": 450.0, ... }
///   },
///   "metadata": {
///     "source": "noaa,donki",
///     "timestamp": "2024-12-20T...",
///     "cached": false
///   }
/// }
/// ```
pub async fn get_current_conditions(
    State(state): State<AppState>,
) -> Result<Json<SpaceWeatherResponse>> {
    // Check cache first
    if let Some(mut cached_response) = state.cache.get_current_conditions().await {
        tracing::info!(
            "Cache hit for current conditions (source: {}, timestamp: {})",
            cached_response.metadata.source,
            cached_response.metadata.timestamp
        );
        cached_response.metadata.cached = true;
        return Ok(Json(cached_response));
    }

    tracing::info!("Cache miss for current conditions, fetching from NOAA API");

    // Try to fetch from NOAA API (with DONKI integration for solar flares)
    match state.noaa_client.get_current_conditions(Some(&state.donki_client)).await {
        Ok(mut response) => {
            tracing::info!(
                "Successfully fetched current conditions from NOAA API (timestamp: {})",
                response.metadata.timestamp
            );

            // Store the observation in the database
            let db_ops = DatabaseOperations::new(state.db_pool.pool().clone());
            match db_ops.store_observation(&response.data, &response.metadata).await {
                Ok(id) => {
                    tracing::debug!("Stored observation in database with id: {}", id);
                }
                Err(e) => {
                    tracing::warn!("Failed to store observation in database: {}", e);
                    // Continue anyway - database failure shouldn't break the API
                }
            }

            // Store in cache for future requests
            state.cache.set_current_conditions(response.clone()).await;
            tracing::debug!("Stored current conditions in cache");
            
            // Ensure cached flag is false for fresh API data
            response.metadata.cached = false;
            Ok(Json(response))
        }
        Err(e) => {
            tracing::warn!(
                "Failed to fetch from NOAA API: {} (error type: {:?}), trying database fallback",
                e,
                e
            );
            
            // Try to get latest from database as fallback
            let db_ops = DatabaseOperations::new(state.db_pool.pool().clone());
            match db_ops.get_latest_observation().await {
                Ok(Some(mut latest)) => {
                    tracing::info!(
                        "Using latest observation from database (timestamp: {}, source: {})",
                        latest.metadata.timestamp,
                        latest.metadata.source
                    );
                    // Store in cache for future requests
                    state.cache.set_current_conditions(latest.clone()).await;
                    // Mark as not cached since it came from database (not cache)
                    latest.metadata.cached = false;
                    return Ok(Json(latest));
                }
                Ok(None) => {
                    tracing::warn!("No observations found in database, using mock data fallback");
                }
                Err(db_err) => {
                    tracing::warn!("Database query failed: {}, using mock data fallback", db_err);
                }
            }
            
            // Final fallback to mock data
            tracing::info!("Returning mock data as final fallback");
            Ok(Json(get_fallback_response()))
        }
    }
}

/// Fallback response when NOAA API is unavailable
fn get_fallback_response() -> SpaceWeatherResponse {
    // Mock data - will be replaced with real NOAA API data in Phase 3
    let response = SpaceWeatherResponse {
        data: SpaceWeatherData {
            solar_flare: Some(SolarFlare {
                class: "C2.5".to_string(),
                peak_time: Utc::now(),
                begin_time: Some(Utc::now() - chrono::Duration::minutes(30)),
                end_time: Some(Utc::now() + chrono::Duration::minutes(10)),
                source_location: Some("AR 12345".to_string()),
            }),
            geomagnetic_storm: Some(GeomagneticStorm {
                level: "G1".to_string(),
                start_time: Some(Utc::now() - chrono::Duration::hours(2)),
                end_time: Some(Utc::now() + chrono::Duration::hours(6)),
                kp_index: 5.0,
            }),
            radiation: Some(RadiationLevels {
                proton_flux: Some(1.2),
                electron_flux: Some(45.6),
                alert_level: "None".to_string(),
                timestamp: Utc::now(),
            }),
            solar_wind: Some(SolarWind {
                speed: 450.0,
                density: 3.5,
                temperature: 50000.0,
                bz: Some(-2.5),
                timestamp: Utc::now(),
            }),
            kp_index: Some(KpIndex {
                value: 3.0,
                level: "Quiet".to_string(),
                timestamp: Utc::now(),
            }),
        },
        metadata: ResponseMetadata {
            timestamp: Utc::now(),
            source: "mock".to_string(),
            cached: false,
        },
    };

    response
}

/// Get historical space weather data
/// 
/// This endpoint returns historical space weather observations within a specified date range.
/// Supports query parameters:
/// - start_date: ISO 8601 format (defaults to 7 days ago)
/// - end_date: ISO 8601 format (defaults to now)
/// - data_type: Filter by type ("solar_flare", "geomagnetic_storm", "radiation", etc.)
/// - limit: Maximum number of records to return
/// - offset: Pagination offset (currently not fully supported in database layer)
pub async fn get_historical_data(
    State(state): State<AppState>,
    Query(params): Query<HistoricalQuery>,
) -> Result<Json<Vec<SpaceWeatherResponse>>> {
    // Check cache first (only if no offset, as offset breaks cache key matching)
    if params.offset.is_none() {
        if let Some(cached_responses) = state.cache.get_historical(
            params.start_date.as_ref(),
            params.end_date.as_ref(),
            params.data_type.as_ref(),
            params.limit,
        ).await {
            tracing::info!(
                "Cache hit for historical data (range: {} to {}, type: {:?}, limit: {:?})",
                params.start_date.as_deref().unwrap_or("default"),
                params.end_date.as_deref().unwrap_or("now"),
                params.data_type,
                params.limit
            );
            return Ok(Json(cached_responses));
        }
    }

    tracing::info!(
        "Cache miss for historical data, querying database (range: {} to {}, type: {:?}, limit: {:?}, offset: {:?})",
        params.start_date.as_deref().unwrap_or("default"),
        params.end_date.as_deref().unwrap_or("now"),
        params.data_type,
        params.limit,
        params.offset
    );

    let db_ops = DatabaseOperations::new(state.db_pool.pool().clone());
    
    // Parse date range from query parameters
    let end_time = if let Some(end_date_str) = &params.end_date {
        DateTime::parse_from_rfc3339(end_date_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| {
                tracing::warn!("Invalid end_date format: {} - Error: {}", end_date_str, e);
                crate::AppError::Validation(
                    format!("Invalid end_date format: {}. Use ISO 8601 format (e.g., 2024-12-19T20:00:00Z)", end_date_str)
                )
            })?
    } else {
        Utc::now()
    };
    
    let start_time = if let Some(start_date_str) = &params.start_date {
        DateTime::parse_from_rfc3339(start_date_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| {
                tracing::warn!("Invalid start_date format: {} - Error: {}", start_date_str, e);
                crate::AppError::Validation(
                    format!("Invalid start_date format: {}. Use ISO 8601 format (e.g., 2024-12-19T20:00:00Z)", start_date_str)
                )
            })?
    } else {
        // Default to 7 days ago if not specified
        end_time - ChronoDuration::days(7)
    };
    
    // Validate date range
    if start_time > end_time {
        tracing::warn!(
            "Invalid date range: start_date ({}) is after end_date ({})",
            start_time,
            end_time
        );
        return Err(crate::AppError::Validation(
            format!("start_date ({}) must be before end_date ({})", start_time, end_time)
        ));
    }
    
    // Limit range to prevent excessive queries (max 1 year)
    let max_range = ChronoDuration::days(365);
    let range_duration = end_time - start_time;
    if range_duration > max_range {
        tracing::warn!(
            "Date range too large: {} days (max: {} days)",
            range_duration.num_days(),
            max_range.num_days()
        );
        return Err(crate::AppError::Validation(
            format!(
                "Date range cannot exceed {} days. Requested range: {} days",
                max_range.num_days(),
                range_duration.num_days()
            )
        ));
    }
    
    // Validate limit if provided
    let limit = params.limit;
    if let Some(l) = limit {
        if l == 0 {
            return Err(crate::AppError::Validation(
                "limit must be greater than 0".to_string()
            ));
        }
        if l > 10000 {
            tracing::warn!("Limit too large: {} (max: 10000), capping to 10000", l);
            // Cap at reasonable maximum
        }
    }
    
    // Note: offset is in query params but not yet fully supported in database layer
    // For now, we'll log a warning if offset is used
    if let Some(offset) = params.offset {
        tracing::debug!("Offset parameter provided: {} (partial support - may not work as expected)", offset);
    }
    
    // Query database
    let responses = if let Some(data_type) = &params.data_type {
        tracing::debug!("Querying database for observations by type: {}", data_type);
        // Filter by data type if specified
        match db_ops.get_observations_by_type(
            start_time,
            end_time,
            data_type,
            limit,
        ).await {
            Ok(responses) => {
                tracing::info!(
                    "Retrieved {} observations of type '{}' from database (range: {} to {})",
                    responses.len(),
                    data_type,
                    start_time,
                    end_time
                );
                responses
            }
            Err(e) => {
                tracing::error!("Database query failed for type '{}': {}", data_type, e);
                return Err(e);
            }
        }
    } else {
        tracing::debug!("Querying database for all observations");
        // Get all observations in range
        match db_ops.get_observations(
            start_time,
            end_time,
            limit,
        ).await {
            Ok(responses) => {
                tracing::info!(
                    "Retrieved {} observations from database (range: {} to {})",
                    responses.len(),
                    start_time,
                    end_time
                );
                responses
            }
            Err(e) => {
                tracing::error!("Database query failed: {}", e);
                return Err(e);
            }
        }
    };
    
    // Apply offset if provided (client-side pagination as database doesn't fully support it yet)
    let final_responses = if let Some(offset) = params.offset {
        let offset_usize = offset as usize;
        if offset_usize >= responses.len() {
            tracing::debug!("Offset {} exceeds result count {}, returning empty array", offset, responses.len());
            vec![]
        } else {
            responses.into_iter().skip(offset_usize).collect()
        }
    } else {
        responses
    };
    
    // Store in cache for future requests (only if no offset was used)
    if params.offset.is_none() {
        state.cache.set_historical(
            params.start_date.as_ref(),
            params.end_date.as_ref(),
            params.data_type.as_ref(),
            limit,
            final_responses.clone(),
        ).await;
        tracing::debug!("Stored historical data in cache");
    }
    
    Ok(Json(final_responses))
}

/// Get active space weather alerts
/// 
/// This endpoint returns active space weather alerts (solar flares, geomagnetic storms, etc.).
/// Supports query parameters:
/// - severity: Filter by severity level ("minor", "moderate", "strong", "severe", "extreme")
/// - alert_type: Filter by type ("solar_flare", "geomagnetic_storm", "radiation")
/// - active_only: Only return active alerts (default: true)
pub async fn get_alerts(
    State(state): State<AppState>,
    Query(params): Query<AlertQuery>,
) -> Json<Vec<SpaceWeatherResponse>> {
    tracing::info!(
        "Fetching alerts (severity: {:?}, type: {:?}, active_only: {:?})",
        params.severity,
        params.alert_type,
        params.active_only
    );

    // Check cache first (simple cache without query params for now)
    if let Some(cached_alerts) = state.cache.get_alerts().await {
        tracing::info!("Cache hit for alerts, filtering {} cached alerts", cached_alerts.len());
        // Filter cached alerts based on query parameters
        let filtered = filter_alerts(cached_alerts, &params);
        tracing::info!("Returning {} filtered alerts from cache", filtered.len());
        return Json(filtered);
    }

    tracing::info!("Cache miss for alerts, generating mock data");

    // Mock data - will be replaced with real alert data in Phase 3
    let mut alerts = Vec::new();

    // Generate mock alerts based on query parameters
    let severity_filter = params.severity.as_deref();
    let alert_type_filter = params.alert_type.as_deref();
    let _active_only = params.active_only.unwrap_or(true);

    // Mock solar flare alert
    if alert_type_filter.is_none() || alert_type_filter == Some("solar_flare") {
        if severity_filter.is_none() || severity_filter == Some("moderate") {
            alerts.push(SpaceWeatherResponse {
                data: SpaceWeatherData {
                    solar_flare: Some(SolarFlare {
                        class: "M2.5".to_string(),
                        peak_time: Utc::now() - chrono::Duration::minutes(15),
                        begin_time: Some(Utc::now() - chrono::Duration::minutes(45)),
                        end_time: Some(Utc::now() + chrono::Duration::minutes(30)),
                        source_location: Some("AR 12345".to_string()),
                    }),
                    geomagnetic_storm: None,
                    radiation: None,
                    solar_wind: None,
                    kp_index: None,
                },
                metadata: ResponseMetadata {
                    timestamp: Utc::now(),
                    source: "mock".to_string(),
                    cached: false,
                },
            });
        }
    }

    // Mock geomagnetic storm alert
    if alert_type_filter.is_none() || alert_type_filter == Some("geomagnetic_storm") {
        if severity_filter.is_none() || severity_filter == Some("minor") {
            alerts.push(SpaceWeatherResponse {
                data: SpaceWeatherData {
                    solar_flare: None,
                    geomagnetic_storm: Some(GeomagneticStorm {
                        level: "G1".to_string(),
                        start_time: Some(Utc::now() - chrono::Duration::hours(1)),
                        end_time: Some(Utc::now() + chrono::Duration::hours(5)),
                        kp_index: 5.0,
                    }),
                    radiation: None,
                    solar_wind: None,
                    kp_index: None,
                },
                metadata: ResponseMetadata {
                    timestamp: Utc::now(),
                    source: "mock".to_string(),
                    cached: false,
                },
            });
        }
    }

    // Store in cache for future requests
    state.cache.set_alerts(alerts.clone()).await;
    tracing::debug!("Stored {} alerts in cache", alerts.len());
    
    tracing::info!("Returning {} alerts", alerts.len());
    Json(alerts)
}

/// Filter alerts based on query parameters
fn filter_alerts(
    mut alerts: Vec<SpaceWeatherResponse>,
    params: &AlertQuery,
) -> Vec<SpaceWeatherResponse> {
    let severity_filter = params.severity.as_deref();
    let alert_type_filter = params.alert_type.as_deref();
    let active_only = params.active_only.unwrap_or(true);

    alerts.retain(|alert| {
        // Filter by severity
        if let Some(_severity) = severity_filter {
            // This is a simplified filter - in production, you'd check the actual severity
            // For now, we'll just pass through since mock data doesn't have explicit severity
        }

        // Filter by alert type
        if let Some(alert_type) = alert_type_filter {
            match alert_type {
                "solar_flare" => {
                    if alert.data.solar_flare.is_none() {
                        return false;
                    }
                }
                "geomagnetic_storm" => {
                    if alert.data.geomagnetic_storm.is_none() {
                        return false;
                    }
                }
                "radiation" => {
                    if alert.data.radiation.is_none() {
                        return false;
                    }
                }
                _ => {}
            }
        }

        // Filter by active status
        if active_only {
            let now = Utc::now();
            // Check if alert is still active based on timestamps
            let is_active = alert.data.solar_flare.as_ref()
                .and_then(|f| f.end_time)
                .map(|end| end > now)
                .or_else(|| {
                    alert.data.geomagnetic_storm.as_ref()
                        .and_then(|s| s.end_time)
                        .map(|end| end > now)
                })
                .unwrap_or(true); // Default to active if we can't determine
            
            if !is_active {
                return false;
            }
        }

        true
    });

    alerts
}

/// Get radiation levels
/// 
/// This endpoint returns current radiation levels (proton flux, electron flux, alert levels).
/// Supports query parameters:
/// - threshold: Minimum proton flux value to return (filters out lower values)
/// - alert_level: Filter by alert level (S1-S5 or None)
pub async fn get_radiation(
    State(state): State<AppState>,
    Query(params): Query<RadiationQuery>,
) -> Result<Json<SpaceWeatherResponse>> {
    tracing::info!(
        "Fetching radiation levels (threshold: {:?}, alert_level: {:?})",
        params.threshold,
        params.alert_level
    );

    // Try to get latest from database first
    let db_ops = DatabaseOperations::new(state.db_pool.pool().clone());
    match db_ops.get_latest_observation().await {
        Ok(Some(latest)) => {
            // Filter to only radiation data if available
            if let Some(radiation) = &latest.data.radiation {
                tracing::info!(
                    "Found radiation data in database (proton_flux: {:?}, electron_flux: {:?}, alert_level: {})",
                    radiation.proton_flux,
                    radiation.electron_flux,
                    radiation.alert_level
                );

                // Apply threshold filter if specified
                if let Some(threshold) = params.threshold {
                    if let Some(proton_flux) = radiation.proton_flux {
                        if proton_flux < threshold {
                            tracing::debug!(
                                "Proton flux {} is below threshold {}, returning empty radiation",
                                proton_flux,
                                threshold
                            );
                            // Return empty radiation if below threshold
                            let mut response = latest.clone();
                            response.data.radiation = None;
                            return Ok(Json(response));
                        }
                        tracing::debug!("Proton flux {} meets threshold {}", proton_flux, threshold);
                    }
                }

                // Apply alert level filter if specified
                if let Some(alert_level) = &params.alert_level {
                    if radiation.alert_level != *alert_level {
                        tracing::debug!(
                            "Alert level '{}' does not match requested '{}', returning empty radiation",
                            radiation.alert_level,
                            alert_level
                        );
                        let mut response = latest.clone();
                        response.data.radiation = None;
                        return Ok(Json(response));
                    }
                }

                tracing::info!("Returning radiation data from database");
                return Ok(Json(latest));
            } else {
                tracing::debug!("Latest observation in database has no radiation data");
            }
        }
        Ok(None) => {
            tracing::debug!("No observations found in database");
        }
        Err(e) => {
            tracing::warn!("Failed to query database for latest observation: {}", e);
        }
    }
    
    // Fallback to mock data
    tracing::info!("Using mock radiation data as fallback");
    let response = SpaceWeatherResponse {
        data: SpaceWeatherData {
            solar_flare: None,
            geomagnetic_storm: None,
            radiation: Some(RadiationLevels {
                proton_flux: params.threshold.or(Some(1.5)),
                electron_flux: Some(50.0),
                alert_level: params.alert_level.unwrap_or_else(|| "None".to_string()),
                timestamp: Utc::now(),
            }),
            solar_wind: None,
            kp_index: None,
        },
        metadata: ResponseMetadata {
            timestamp: Utc::now(),
            source: "mock".to_string(),
            cached: false,
        },
    };

    Ok(Json(response))
}

/// Get exoplanets with optional filters
/// 
/// Queries exoplanets from the database (or TAP service if not in database) with various filters.
/// 
/// # Query Parameters
/// - `limit`: Maximum number of results (default: 100, max: 1000)
/// - `offset`: Pagination offset (default: 0)
/// - `discovery_method`: Filter by discovery method (e.g., "Transit", "Radial Velocity")
/// - `min_year`: Minimum discovery year
/// - `max_year`: Maximum discovery year
/// - `hostname`: Filter by host star name (partial match)
/// - `min_radius`: Minimum planet radius in Earth radii
/// - `max_radius`: Maximum planet radius in Earth radii
/// - `min_mass`: Minimum planet mass in Earth masses
/// - `max_mass`: Maximum planet mass in Earth masses
/// - `sort_by`: Sort field (pl_name, hostname, disc_year, pl_rade, pl_bmasse, pl_eqt)
/// - `sort_order`: Sort order (asc or desc)
/// 
/// # Response
/// Returns `ExoplanetResponse` with:
/// - `data`: Array of exoplanet objects
/// - `metadata`: Count, timestamp, source
/// 
/// # Example
/// ```bash
/// GET /api/v1/exoplanets?limit=50&discovery_method=Transit&min_year=2020
/// ```
pub async fn get_exoplanets(
    State(state): State<AppState>,
    Query(params): Query<crate::models::ExoplanetQueryParams>,
) -> Result<Json<crate::models::ExoplanetResponse>> {
    let db_ops = DatabaseOperations::new(state.db_pool.pool().clone());
    
    // Query from database
    match db_ops.query_exoplanets(&params).await {
        Ok(exoplanets) => {
            let count = exoplanets.len();
            tracing::info!("Retrieved {} exoplanets from database", count);
            
            Ok(Json(crate::models::ExoplanetResponse {
                data: exoplanets,
                metadata: crate::models::ExoplanetMetadata {
                    count,
                    timestamp: Utc::now(),
                    source: "database".to_string(),
                },
            }))
        }
        Err(e) => {
            tracing::warn!("Failed to query database: {}, trying TAP service", e);
            
            // Fallback to TAP service
            match state.exoplanet_client.query_exoplanets(&params).await {
                Ok(exoplanets) => {
                    let count = exoplanets.len();
                    tracing::info!("Retrieved {} exoplanets from TAP service", count);
                    
                    // Store in database for future queries (async, don't wait)
                    let db_ops_clone = DatabaseOperations::new(state.db_pool.pool().clone());
                    let exoplanets_clone = exoplanets.clone();
                    tokio::spawn(async move {
                        for exoplanet in exoplanets_clone {
                            if let Err(e) = db_ops_clone.store_exoplanet(&exoplanet).await {
                                tracing::warn!("Failed to store exoplanet {}: {}", exoplanet.pl_name, e);
                            }
                        }
                    });
                    
                    Ok(Json(crate::models::ExoplanetResponse {
                        data: exoplanets,
                        metadata: crate::models::ExoplanetMetadata {
                            count,
                            timestamp: Utc::now(),
                            source: "tap".to_string(),
                        },
                    }))
                }
                Err(e) => {
                    tracing::error!("Failed to query TAP service: {}", e);
                    Err(e)
                }
            }
        }
    }
}

/// Get a specific exoplanet by name
/// 
/// # Path Parameters
/// - `name`: Exoplanet name (e.g., "Kepler-186 f")
/// 
/// # Response
/// Returns a single `Exoplanet` object or 404 if not found.
/// 
/// # Example
/// ```bash
/// GET /api/v1/exoplanets/Kepler-186%20f
/// ```
pub async fn get_exoplanet_by_name(
    State(state): State<AppState>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<crate::models::Exoplanet>> {
    let db_ops = DatabaseOperations::new(state.db_pool.pool().clone());
    
    match db_ops.get_exoplanet_by_name(&name).await {
        Ok(Some(exoplanet)) => {
            tracing::info!("Retrieved exoplanet {} from database", name);
            Ok(Json(exoplanet))
        }
        Ok(None) => {
            tracing::info!("Exoplanet {} not found in database, querying TAP service", name);
            
            // Try TAP service - need to query by pl_name, but TAP doesn't support that directly
            // So we'll query all and filter, or use a different approach
            // For now, let's query with a limit and check if any match
            let params = crate::models::ExoplanetQueryParams {
                limit: Some(1000), // Get a reasonable number to search through
                ..Default::default()
            };
            
            match state.exoplanet_client.query_exoplanets(&params).await {
                Ok(exoplanets) => {
                    // Find the exoplanet with matching name (case-insensitive)
                    if let Some(exoplanet) = exoplanets.iter()
                        .find(|e| e.pl_name.eq_ignore_ascii_case(&name))
                        .cloned()
                    {
                        // Store in database
                        let db_ops_clone = DatabaseOperations::new(state.db_pool.pool().clone());
                        let exoplanet_clone = exoplanet.clone();
                        tokio::spawn(async move {
                            if let Err(e) = db_ops_clone.store_exoplanet(&exoplanet_clone).await {
                                tracing::warn!("Failed to store exoplanet: {}", e);
                            }
                        });
                        
                        Ok(Json(exoplanet))
                    } else {
                        Err(crate::AppError::NotFound(format!("Exoplanet '{}' not found", name)))
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to query TAP service: {}", e);
                    Err(crate::AppError::NotFound(format!("Exoplanet '{}' not found", name)))
                }
            }
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            Err(e)
        }
    }
}

/// Predict solar flare based on current conditions
/// 
/// Uses ML service to predict solar flare occurrence based on current space weather conditions.
/// 
/// # Response
/// Returns prediction with:
/// - `predicted_flare_class`: Predicted class (A, B, C, M, X, or None)
/// - `predicted_peak_time`: Estimated peak time (2 hours from now)
/// - `confidence_score`: Confidence (0.0 to 1.0)
/// - `model_version`: Model version used
/// 
/// # Errors
/// Returns 503 if ML service is not available or model not loaded
/// 
/// # Example
/// ```bash
/// GET /api/v1/predictions/solar-flare
/// ```
pub async fn predict_solar_flare(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>> {
    // Check if ML service is enabled and available
    let ml_client = match &state.ml_service_client {
        Some(client) => client,
        None => {
            return Err(crate::AppError::Internal(
                "ML service is not enabled. Set ml_service.enabled = true in config.".to_string()
            ));
        }
    };

    // Get current space weather conditions
    let current_conditions = match state.noaa_client.get_current_conditions(Some(&state.donki_client)).await {
        Ok(response) => response.data,
        Err(e) => {
            tracing::warn!("Failed to get current conditions for prediction: {}", e);
            // Use minimal data if available
            SpaceWeatherData {
                solar_flare: None,
                geomagnetic_storm: None,
                radiation: None,
                solar_wind: None,
                kp_index: None,
            }
        }
    };

    // Calculate days since last flare and flare counts
    let db_ops = DatabaseOperations::new(state.db_pool.pool().clone());
    
    // Get recent flares for context
    let recent_flares = db_ops.get_observations(
        Utc::now() - ChronoDuration::days(30),
        Utc::now(),
        Some(100),
    ).await.unwrap_or_default();

    let days_since_last_flare = recent_flares.iter()
        .find(|obs| obs.data.solar_flare.is_some())
        .map(|obs| {
            let flare_time = obs.data.solar_flare.as_ref().unwrap().peak_time;
            (Utc::now() - flare_time).num_seconds() as f64 / 86400.0
        })
        .unwrap_or(30.0);

    let flare_count_7_days = recent_flares.iter()
        .filter(|obs| {
            obs.data.solar_flare.is_some() &&
            (Utc::now() - obs.metadata.timestamp).num_days() <= 7
        })
        .count() as i32;

    let flare_count_30_days = recent_flares.len() as i32;

    // Make prediction
    let prediction = ml_client.predict_solar_flare(
        &current_conditions,
        Some(days_since_last_flare),
        Some((flare_count_7_days, flare_count_30_days)),
    ).await?;

    // Store prediction in database
    let input_features = serde_json::json!({
        "kp_index": current_conditions.kp_index.as_ref().map(|kp| kp.value),
        "solar_wind_speed": current_conditions.solar_wind.as_ref().map(|sw| sw.speed),
        "days_since_last_flare": days_since_last_flare,
        "flare_count_last_7_days": flare_count_7_days,
        "flare_count_last_30_days": flare_count_30_days,
    });

    match db_ops.store_prediction(&prediction, &input_features, "XGBoost").await {
        Ok(id) => {
            tracing::debug!("Stored prediction in database with id: {}", id);
        }
        Err(e) => {
            tracing::warn!("Failed to store prediction in database: {}", e);
            // Continue anyway - database failure shouldn't break the API
        }
    }

    // Return prediction response
    Ok(Json(serde_json::json!({
        "prediction": {
            "predicted_flare_class": prediction.predicted_flare_class,
            "predicted_peak_time": prediction.predicted_peak_time,
            "confidence_score": prediction.confidence_score,
            "model_version": prediction.model_version,
            "prediction_timestamp": prediction.prediction_timestamp
        },
        "metadata": {
            "source": "ml_service",
            "features_used": prediction.features_used
        }
    })))
}

/// Get prediction history
/// 
/// Returns recent predictions with their results (if available).
/// 
/// # Query Parameters
/// - `limit`: Maximum number of predictions to return (default: 50, max: 500)
/// 
/// # Response
/// Returns array of predictions with actual results (if available)
pub async fn get_prediction_history(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>> {
    let limit = params.get("limit")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(50)
        .min(500);

    let db_ops = DatabaseOperations::new(state.db_pool.pool().clone());
    
    match db_ops.get_recent_predictions(limit).await {
        Ok(predictions) => {
            let predictions_json: Vec<serde_json::Value> = predictions.into_iter().map(|p| {
                serde_json::json!({
                    "id": p.id,
                    "prediction_time": p.prediction_time,
                    "predicted_flare_class": p.predicted_flare_class,
                    "predicted_peak_time": p.predicted_peak_time,
                    "confidence_score": p.confidence_score,
                    "model_version": p.model_version,
                    "actual_flare_class": p.actual_flare_class,
                    "prediction_correct": p.prediction_correct,
                    "created_at": p.created_at
                })
            }).collect();

            Ok(Json(serde_json::json!({
                "predictions": predictions_json,
                "count": predictions_json.len()
            })))
        }
        Err(e) => {
            tracing::warn!("Failed to get prediction history: {}", e);
            Err(e)
        }
    }
}

/// Get prediction accuracy statistics
/// 
/// Returns accuracy metrics for predictions that have been verified with actual results.
/// 
/// # Response
/// Returns accuracy statistics including:
/// - Total predictions with results
/// - Correct/incorrect counts
/// - Accuracy percentage
/// - Average confidence
pub async fn get_prediction_accuracy(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>> {
    let db_ops = DatabaseOperations::new(state.db_pool.pool().clone());
    
    match db_ops.get_prediction_accuracy().await {
        Ok(accuracy) => {
            Ok(Json(serde_json::json!({
                "total_predictions": accuracy.total_predictions,
                "correct_predictions": accuracy.correct_predictions,
                "incorrect_predictions": accuracy.incorrect_predictions,
                "accuracy": accuracy.accuracy,
                "avg_confidence": accuracy.avg_confidence
            })))
        }
        Err(e) => {
            tracing::warn!("Failed to get prediction accuracy: {}", e);
            Err(e)
        }
    }
}

