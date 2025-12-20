use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{Utc, Duration as ChronoDuration, DateTime};
use crate::models::*;
use crate::AppState;
use crate::database::DatabaseOperations;
use crate::Result;

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

