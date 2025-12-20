use crate::Result;
use crate::models::*;
use chrono::{DateTime, Utc, Duration as ChronoDuration};
use sqlx::{MySql, Pool, Row};
use tracing::{warn, info};

/// Database operations for space weather data
pub struct DatabaseOperations {
    pool: Pool<MySql>,
}

impl DatabaseOperations {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    /// Store a space weather observation
    pub async fn store_observation(&self, data: &SpaceWeatherData, metadata: &ResponseMetadata) -> Result<u64> {
        let timestamp = metadata.timestamp;
        let source = &metadata.source;
        let cached = metadata.cached;

        // Extract values from the data structure
        let kp_index_value = data.kp_index.as_ref().map(|kp| kp.value);
        let kp_index_level = data.kp_index.as_ref().map(|kp| kp.level.as_str());

        let geomagnetic_storm_level = data.geomagnetic_storm.as_ref().map(|gs| gs.level.as_str());
        let geomagnetic_storm_kp_index = data.geomagnetic_storm.as_ref().map(|gs| gs.kp_index);
        let geomagnetic_storm_start_time = data.geomagnetic_storm.as_ref().and_then(|gs| gs.start_time);
        let geomagnetic_storm_end_time = data.geomagnetic_storm.as_ref().and_then(|gs| gs.end_time);

        let solar_wind_speed = data.solar_wind.as_ref().map(|sw| sw.speed);
        let solar_wind_density = data.solar_wind.as_ref().map(|sw| sw.density);
        let solar_wind_temperature = data.solar_wind.as_ref().map(|sw| sw.temperature);
        let solar_wind_bz = data.solar_wind.as_ref().and_then(|sw| sw.bz);

        let solar_flare_class = data.solar_flare.as_ref().map(|sf| sf.class.as_str());
        let solar_flare_peak_time = data.solar_flare.as_ref().map(|sf| sf.peak_time);
        let solar_flare_begin_time = data.solar_flare.as_ref().and_then(|sf| sf.begin_time);
        let solar_flare_end_time = data.solar_flare.as_ref().and_then(|sf| sf.end_time);
        let solar_flare_source_location = data.solar_flare.as_ref().and_then(|sf| sf.source_location.as_deref());

        let radiation_proton_flux = data.radiation.as_ref().and_then(|r| r.proton_flux);
        let radiation_electron_flux = data.radiation.as_ref().and_then(|r| r.electron_flux);
        let radiation_alert_level = data.radiation.as_ref().map(|r| r.alert_level.as_str());

        let result = sqlx::query(
            r#"
            INSERT INTO space_weather_observations (
                timestamp, source, cached,
                kp_index_value, kp_index_level,
                geomagnetic_storm_level, geomagnetic_storm_kp_index,
                geomagnetic_storm_start_time, geomagnetic_storm_end_time,
                solar_wind_speed, solar_wind_density, solar_wind_temperature, solar_wind_bz,
                solar_flare_class, solar_flare_peak_time, solar_flare_begin_time, solar_flare_end_time, solar_flare_source_location,
                radiation_proton_flux, radiation_electron_flux, radiation_alert_level
            ) VALUES (
                ?, ?, ?,
                ?, ?,
                ?, ?,
                ?, ?,
                ?, ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?
            )
            "#,
        )
        .bind(timestamp)
        .bind(source)
        .bind(cached)
        .bind(kp_index_value)
        .bind(kp_index_level)
        .bind(geomagnetic_storm_level)
        .bind(geomagnetic_storm_kp_index)
        .bind(geomagnetic_storm_start_time)
        .bind(geomagnetic_storm_end_time)
        .bind(solar_wind_speed)
        .bind(solar_wind_density)
        .bind(solar_wind_temperature)
        .bind(solar_wind_bz)
        .bind(solar_flare_class)
        .bind(solar_flare_peak_time)
        .bind(solar_flare_begin_time)
        .bind(solar_flare_end_time)
        .bind(solar_flare_source_location)
        .bind(radiation_proton_flux)
        .bind(radiation_electron_flux)
        .bind(radiation_alert_level)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to store observation: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(result.last_insert_id())
    }

    /// Retrieve observations within a date range
    pub async fn get_observations(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        limit: Option<u32>,
    ) -> Result<Vec<SpaceWeatherResponse>> {
        let limit = limit.unwrap_or(100).min(1000); // Cap at 1000

        let rows = sqlx::query_as::<_, ObservationRow>(
            r#"
            SELECT 
                id, timestamp, source, cached,
                kp_index_value, kp_index_level,
                geomagnetic_storm_level, geomagnetic_storm_kp_index,
                geomagnetic_storm_start_time, geomagnetic_storm_end_time,
                solar_wind_speed, solar_wind_density, solar_wind_temperature, solar_wind_bz,
                solar_flare_class, solar_flare_peak_time, solar_flare_begin_time, solar_flare_end_time, solar_flare_source_location,
                radiation_proton_flux, radiation_electron_flux, radiation_alert_level
            FROM space_weather_observations
            WHERE timestamp >= ? AND timestamp <= ?
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(start_time)
        .bind(end_time)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to retrieve observations: {}", e);
            crate::AppError::Database(e)
        })?;

        let mut responses = Vec::new();
        for row in rows {
            responses.push(self.row_to_response(row)?);
        }

        Ok(responses)
    }

    /// Get the latest observation
    pub async fn get_latest_observation(&self) -> Result<Option<SpaceWeatherResponse>> {
        let row = sqlx::query_as::<_, ObservationRow>(
            r#"
            SELECT 
                id, timestamp, source, cached,
                kp_index_value, kp_index_level,
                geomagnetic_storm_level, geomagnetic_storm_kp_index,
                geomagnetic_storm_start_time, geomagnetic_storm_end_time,
                solar_wind_speed, solar_wind_density, solar_wind_temperature, solar_wind_bz,
                solar_flare_class, solar_flare_peak_time, solar_flare_begin_time, solar_flare_end_time, solar_flare_source_location,
                radiation_proton_flux, radiation_electron_flux, radiation_alert_level
            FROM space_weather_observations
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to retrieve latest observation: {}", e);
            crate::AppError::Database(e)
        })?;

        if let Some(row) = row {
            Ok(Some(self.row_to_response(row)?))
        } else {
            Ok(None)
        }
    }

    /// Get observations by data type (e.g., only KP index, only solar wind)
    pub async fn get_observations_by_type(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        data_type: &str,
        limit: Option<u32>,
    ) -> Result<Vec<SpaceWeatherResponse>> {
        let limit = limit.unwrap_or(100).min(1000);

        // Build query based on data type
        let where_clause = match data_type {
            "kp_index" => "kp_index_value IS NOT NULL",
            "solar_wind" => "solar_wind_speed IS NOT NULL OR solar_wind_bz IS NOT NULL",
            "solar_flare" => "solar_flare_class IS NOT NULL",
            "geomagnetic_storm" => "geomagnetic_storm_level IS NOT NULL",
            "radiation" => "radiation_proton_flux IS NOT NULL OR radiation_electron_flux IS NOT NULL",
            _ => return Err(crate::AppError::Validation(format!("Invalid data type: {}", data_type))),
        };

        let query = format!(
            r#"
            SELECT 
                id, timestamp, source, cached,
                kp_index_value, kp_index_level,
                geomagnetic_storm_level, geomagnetic_storm_kp_index,
                geomagnetic_storm_start_time, geomagnetic_storm_end_time,
                solar_wind_speed, solar_wind_density, solar_wind_temperature, solar_wind_bz,
                solar_flare_class, solar_flare_peak_time, solar_flare_begin_time, solar_flare_end_time, solar_flare_source_location,
                radiation_proton_flux, radiation_electron_flux, radiation_alert_level
            FROM space_weather_observations
            WHERE timestamp >= ? AND timestamp <= ? AND {}
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
            where_clause
        );

        let rows = sqlx::query_as::<_, ObservationRow>(&query)
            .bind(start_time)
            .bind(end_time)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                warn!("Failed to retrieve observations by type: {}", e);
                crate::AppError::Database(e)
            })?;

        let mut responses = Vec::new();
        for row in rows {
            responses.push(self.row_to_response(row)?);
        }

        Ok(responses)
    }

    /// Store multiple observations in a transaction
    pub async fn store_observations_batch(
        &self,
        observations: &[(SpaceWeatherData, ResponseMetadata)],
    ) -> Result<Vec<u64>> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            warn!("Failed to begin transaction: {}", e);
            crate::AppError::Database(e)
        })?;

        let mut ids = Vec::new();

        for (data, metadata) in observations {
            let timestamp = metadata.timestamp;
            let source = &metadata.source;
            let cached = metadata.cached;

            let kp_index_value = data.kp_index.as_ref().map(|kp| kp.value);
            let kp_index_level = data.kp_index.as_ref().map(|kp| kp.level.as_str());
            let geomagnetic_storm_level = data.geomagnetic_storm.as_ref().map(|gs| gs.level.as_str());
            let geomagnetic_storm_kp_index = data.geomagnetic_storm.as_ref().map(|gs| gs.kp_index);
            let geomagnetic_storm_start_time = data.geomagnetic_storm.as_ref().and_then(|gs| gs.start_time);
            let geomagnetic_storm_end_time = data.geomagnetic_storm.as_ref().and_then(|gs| gs.end_time);
            let solar_wind_speed = data.solar_wind.as_ref().map(|sw| sw.speed);
            let solar_wind_density = data.solar_wind.as_ref().map(|sw| sw.density);
            let solar_wind_temperature = data.solar_wind.as_ref().map(|sw| sw.temperature);
            let solar_wind_bz = data.solar_wind.as_ref().and_then(|sw| sw.bz);
            let solar_flare_class = data.solar_flare.as_ref().map(|sf| sf.class.as_str());
            let solar_flare_peak_time = data.solar_flare.as_ref().map(|sf| sf.peak_time);
            let solar_flare_begin_time = data.solar_flare.as_ref().and_then(|sf| sf.begin_time);
            let solar_flare_end_time = data.solar_flare.as_ref().and_then(|sf| sf.end_time);
            let solar_flare_source_location = data.solar_flare.as_ref().and_then(|sf| sf.source_location.as_deref());
            let radiation_proton_flux = data.radiation.as_ref().and_then(|r| r.proton_flux);
            let radiation_electron_flux = data.radiation.as_ref().and_then(|r| r.electron_flux);
            let radiation_alert_level = data.radiation.as_ref().map(|r| r.alert_level.as_str());

            let result = sqlx::query(
                r#"
                INSERT INTO space_weather_observations (
                    timestamp, source, cached,
                    kp_index_value, kp_index_level,
                    geomagnetic_storm_level, geomagnetic_storm_kp_index,
                    geomagnetic_storm_start_time, geomagnetic_storm_end_time,
                    solar_wind_speed, solar_wind_density, solar_wind_temperature, solar_wind_bz,
                    solar_flare_class, solar_flare_peak_time, solar_flare_begin_time, solar_flare_end_time, solar_flare_source_location,
                    radiation_proton_flux, radiation_electron_flux, radiation_alert_level
                ) VALUES (
                    ?, ?, ?,
                    ?, ?,
                    ?, ?,
                    ?, ?,
                    ?, ?, ?, ?,
                    ?, ?, ?, ?, ?,
                    ?, ?, ?
                )
                "#,
            )
            .bind(timestamp)
            .bind(source)
            .bind(cached)
            .bind(kp_index_value)
            .bind(kp_index_level)
            .bind(geomagnetic_storm_level)
            .bind(geomagnetic_storm_kp_index)
            .bind(geomagnetic_storm_start_time)
            .bind(geomagnetic_storm_end_time)
            .bind(solar_wind_speed)
            .bind(solar_wind_density)
            .bind(solar_wind_temperature)
            .bind(solar_wind_bz)
            .bind(solar_flare_class)
            .bind(solar_flare_peak_time)
            .bind(solar_flare_begin_time)
            .bind(solar_flare_end_time)
            .bind(solar_flare_source_location)
            .bind(radiation_proton_flux)
            .bind(radiation_electron_flux)
            .bind(radiation_alert_level)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                warn!("Failed to store observation in batch: {}", e);
                crate::AppError::Database(e)
            })?;

            ids.push(result.last_insert_id());
        }

        tx.commit().await.map_err(|e| {
            warn!("Failed to commit transaction: {}", e);
            crate::AppError::Database(e)
        })?;

        info!("Stored {} observations in batch", ids.len());
        Ok(ids)
    }

    /// Clean up old observations (archival)
    /// Removes observations older than the specified duration
    pub async fn cleanup_old_observations(&self, older_than_days: u32) -> Result<u64> {
        let cutoff_date = Utc::now() - ChronoDuration::days(older_than_days as i64);

        let result = sqlx::query(
            r#"
            DELETE FROM space_weather_observations
            WHERE timestamp < ?
            "#,
        )
        .bind(cutoff_date)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to cleanup old observations: {}", e);
            crate::AppError::Database(e)
        })?;

        let deleted = result.rows_affected();
        if deleted > 0 {
            info!("Cleaned up {} old observations (older than {} days)", deleted, older_than_days);
        }

        Ok(deleted)
    }

    /// Get observation count in a date range
    pub async fn get_observation_count(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<u64> {
        let row = if let (Some(start), Some(end)) = (start_time, end_time) {
            sqlx::query("SELECT COUNT(*) as count FROM space_weather_observations WHERE timestamp >= ? AND timestamp <= ?")
                .bind(start)
                .bind(end)
                .fetch_one(&self.pool)
                .await
        } else if let Some(start) = start_time {
            sqlx::query("SELECT COUNT(*) as count FROM space_weather_observations WHERE timestamp >= ?")
                .bind(start)
                .fetch_one(&self.pool)
                .await
        } else if let Some(end) = end_time {
            sqlx::query("SELECT COUNT(*) as count FROM space_weather_observations WHERE timestamp <= ?")
                .bind(end)
                .fetch_one(&self.pool)
                .await
        } else {
            sqlx::query("SELECT COUNT(*) as count FROM space_weather_observations")
                .fetch_one(&self.pool)
                .await
        }
        .map_err(|e| {
            warn!("Failed to get observation count: {}", e);
            crate::AppError::Database(e)
        })?;

        let count: i64 = row.try_get("count").map_err(|e| {
            warn!("Failed to get count from row: {}", e);
            crate::AppError::Database(sqlx::Error::ColumnNotFound("count".to_string()))
        })?;
        Ok(count as u64)
    }

    /// Convert database row to SpaceWeatherResponse
    fn row_to_response(&self, row: ObservationRow) -> Result<SpaceWeatherResponse> {
        let data = SpaceWeatherData {
            kp_index: row.kp_index_value.map(|value| KpIndex {
                value,
                level: row.kp_index_level.unwrap_or_else(|| "Unknown".to_string()),
                timestamp: row.timestamp,
            }),
            geomagnetic_storm: if row.geomagnetic_storm_level.is_some() {
                Some(GeomagneticStorm {
                    level: row.geomagnetic_storm_level.unwrap_or_else(|| "None".to_string()),
                    start_time: row.geomagnetic_storm_start_time,
                    end_time: row.geomagnetic_storm_end_time,
                    kp_index: row.geomagnetic_storm_kp_index.unwrap_or(0.0),
                })
            } else {
                None
            },
            solar_wind: if row.solar_wind_speed.is_some() || row.solar_wind_bz.is_some() {
                Some(SolarWind {
                    speed: row.solar_wind_speed.unwrap_or(0.0),
                    density: row.solar_wind_density.unwrap_or(0.0),
                    temperature: row.solar_wind_temperature.unwrap_or(0.0),
                    bz: row.solar_wind_bz,
                    timestamp: row.timestamp,
                })
            } else {
                None
            },
            solar_flare: if row.solar_flare_class.is_some() {
                Some(SolarFlare {
                    class: row.solar_flare_class.unwrap_or_else(|| "Unknown".to_string()),
                    peak_time: row.solar_flare_peak_time.unwrap_or(row.timestamp),
                    begin_time: row.solar_flare_begin_time,
                    end_time: row.solar_flare_end_time,
                    source_location: row.solar_flare_source_location,
                })
            } else {
                None
            },
            radiation: if row.radiation_proton_flux.is_some() || row.radiation_electron_flux.is_some() {
                Some(RadiationLevels {
                    proton_flux: row.radiation_proton_flux,
                    electron_flux: row.radiation_electron_flux,
                    alert_level: row.radiation_alert_level.unwrap_or_else(|| "None".to_string()),
                    timestamp: row.timestamp,
                })
            } else {
                None
            },
        };

        Ok(SpaceWeatherResponse {
            data,
            metadata: ResponseMetadata {
                timestamp: row.timestamp,
                source: row.source,
                cached: row.cached,
            },
        })
    }
}

/// Database row structure for observations
#[derive(sqlx::FromRow)]
struct ObservationRow {
    id: u64,
    timestamp: DateTime<Utc>,
    source: String,
    cached: bool,
    kp_index_value: Option<f64>,
    kp_index_level: Option<String>,
    geomagnetic_storm_level: Option<String>,
    geomagnetic_storm_kp_index: Option<f64>,
    geomagnetic_storm_start_time: Option<DateTime<Utc>>,
    geomagnetic_storm_end_time: Option<DateTime<Utc>>,
    solar_wind_speed: Option<f64>,
    solar_wind_density: Option<f64>,
    solar_wind_temperature: Option<f64>,
    solar_wind_bz: Option<f64>,
    solar_flare_class: Option<String>,
    solar_flare_peak_time: Option<DateTime<Utc>>,
    solar_flare_begin_time: Option<DateTime<Utc>>,
    solar_flare_end_time: Option<DateTime<Utc>>,
    solar_flare_source_location: Option<String>,
    radiation_proton_flux: Option<f64>,
    radiation_electron_flux: Option<f64>,
    radiation_alert_level: Option<String>,
}

