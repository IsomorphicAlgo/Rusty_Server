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

/// Database operations for exoplanet data
impl DatabaseOperations {
    /// Store or update an exoplanet
    pub async fn store_exoplanet(&self, exoplanet: &crate::models::Exoplanet) -> Result<u64> {
        let result = sqlx::query(
            r#"
            INSERT INTO exoplanets (
                pl_name, hostname, discoverymethod, disc_year, disc_facility, disc_telescope,
                pl_orbper, pl_orbpererr1, pl_orbpererr2, pl_orbperlim,
                pl_rade, pl_radeerr1, pl_radeerr2,
                pl_bmasse, pl_bmasseerr1, pl_bmasseerr2,
                pl_eqt, st_teff, st_rad, st_mass, sy_dist, sy_pnum,
                rowupdate, releasedate, last_synced_at
            ) VALUES (
                ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?, ?, ?, ?,
                ?, ?, NOW()
            )
            ON DUPLICATE KEY UPDATE
                hostname = VALUES(hostname),
                discoverymethod = VALUES(discoverymethod),
                disc_year = VALUES(disc_year),
                disc_facility = VALUES(disc_facility),
                disc_telescope = VALUES(disc_telescope),
                pl_orbper = VALUES(pl_orbper),
                pl_orbpererr1 = VALUES(pl_orbpererr1),
                pl_orbpererr2 = VALUES(pl_orbpererr2),
                pl_orbperlim = VALUES(pl_orbperlim),
                pl_rade = VALUES(pl_rade),
                pl_radeerr1 = VALUES(pl_radeerr1),
                pl_radeerr2 = VALUES(pl_radeerr2),
                pl_bmasse = VALUES(pl_bmasse),
                pl_bmasseerr1 = VALUES(pl_bmasseerr1),
                pl_bmasseerr2 = VALUES(pl_bmasseerr2),
                pl_eqt = VALUES(pl_eqt),
                st_teff = VALUES(st_teff),
                st_rad = VALUES(st_rad),
                st_mass = VALUES(st_mass),
                sy_dist = VALUES(sy_dist),
                sy_pnum = VALUES(sy_pnum),
                rowupdate = VALUES(rowupdate),
                releasedate = VALUES(releasedate),
                last_synced_at = NOW(),
                updated_at = NOW()
            "#
        )
        .bind(&exoplanet.pl_name)
        .bind(&exoplanet.hostname)
        .bind(&exoplanet.discoverymethod)
        .bind(exoplanet.disc_year)
        .bind(&exoplanet.disc_facility)
        .bind(&exoplanet.disc_telescope)
        .bind(exoplanet.pl_orbper)
        .bind(exoplanet.pl_orbpererr1)
        .bind(exoplanet.pl_orbpererr2)
        .bind(exoplanet.pl_orbperlim)
        .bind(exoplanet.pl_rade)
        .bind(exoplanet.pl_radeerr1)
        .bind(exoplanet.pl_radeerr2)
        .bind(exoplanet.pl_bmasse)
        .bind(exoplanet.pl_bmasseerr1)
        .bind(exoplanet.pl_bmasseerr2)
        .bind(exoplanet.pl_eqt)
        .bind(exoplanet.st_teff)
        .bind(exoplanet.st_rad)
        .bind(exoplanet.st_mass)
        .bind(exoplanet.sy_dist)
        .bind(exoplanet.sy_pnum)
        .bind(&exoplanet.rowupdate)
        .bind(&exoplanet.releasedate)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to store exoplanet: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(result.last_insert_id())
    }

    /// Query exoplanets with filters
    pub async fn query_exoplanets(
        &self,
        params: &crate::models::ExoplanetQueryParams,
    ) -> Result<Vec<crate::models::Exoplanet>> {
        // Build base query with safe field names for ORDER BY
        let sort_field = params.sort_by.as_deref().unwrap_or("pl_name");
        // Validate sort field to prevent SQL injection
        let valid_sort_fields = ["pl_name", "hostname", "disc_year", "pl_rade", "pl_bmasse", "pl_eqt"];
        let sort_field = if valid_sort_fields.contains(&sort_field) {
            sort_field
        } else {
            "pl_name"
        };
        
        let sort_order = params.sort_order.as_deref().unwrap_or("asc");
        let sort_order = if sort_order.to_uppercase() == "DESC" { "DESC" } else { "ASC" };
        
        let limit = params.limit.unwrap_or(100).min(1000); // Cap at 1000
        let offset = params.offset.unwrap_or(0);

        // Build query with proper parameterization
        let mut query = format!(
            "SELECT pl_name, hostname, discoverymethod, disc_year, disc_facility, disc_telescope, \
             pl_orbper, pl_orbpererr1, pl_orbpererr2, pl_orbperlim, \
             pl_rade, pl_radeerr1, pl_radeerr2, \
             pl_bmasse, pl_bmasseerr1, pl_bmasseerr2, \
             pl_eqt, st_teff, st_rad, st_mass, sy_dist, sy_pnum, rowupdate, releasedate \
             FROM exoplanets WHERE 1=1"
        );

        // Build query string with conditions (safe because we're using parameterized queries)
        if params.discovery_method.is_some() {
            query.push_str(" AND discoverymethod = ?");
        }
        if params.min_year.is_some() {
            query.push_str(" AND disc_year >= ?");
        }
        if params.max_year.is_some() {
            query.push_str(" AND disc_year <= ?");
        }
        if params.hostname.is_some() {
            query.push_str(" AND hostname LIKE ?");
        }
        if params.min_radius.is_some() {
            query.push_str(" AND pl_rade >= ?");
        }
        if params.max_radius.is_some() {
            query.push_str(" AND pl_rade <= ?");
        }
        if params.min_mass.is_some() {
            query.push_str(" AND pl_bmasse >= ?");
        }
        if params.max_mass.is_some() {
            query.push_str(" AND pl_bmasse <= ?");
        }

        query.push_str(&format!(" ORDER BY {} {}", sort_field, sort_order));
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        // Build query with bindings
        let mut sqlx_query = sqlx::query_as::<_, ExoplanetRow>(&query);
        
        if let Some(ref method) = params.discovery_method {
            sqlx_query = sqlx_query.bind(method);
        }
        if let Some(year) = params.min_year {
            sqlx_query = sqlx_query.bind(year);
        }
        if let Some(year) = params.max_year {
            sqlx_query = sqlx_query.bind(year);
        }
        if let Some(ref host) = params.hostname {
            sqlx_query = sqlx_query.bind(format!("%{}%", host));
        }
        if let Some(radius) = params.min_radius {
            sqlx_query = sqlx_query.bind(radius);
        }
        if let Some(radius) = params.max_radius {
            sqlx_query = sqlx_query.bind(radius);
        }
        if let Some(mass) = params.min_mass {
            sqlx_query = sqlx_query.bind(mass);
        }
        if let Some(mass) = params.max_mass {
            sqlx_query = sqlx_query.bind(mass);
        }

        let rows = sqlx_query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                warn!("Failed to query exoplanets: {}", e);
                crate::AppError::Database(e)
            })?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    /// Get exoplanet by name
    pub async fn get_exoplanet_by_name(&self, pl_name: &str) -> Result<Option<crate::models::Exoplanet>> {
        let row = sqlx::query_as::<_, ExoplanetRow>(
            "SELECT pl_name, hostname, discoverymethod, disc_year, disc_facility, disc_telescope, \
             pl_orbper, pl_orbpererr1, pl_orbpererr2, pl_orbperlim, \
             pl_rade, pl_radeerr1, pl_radeerr2, \
             pl_bmasse, pl_bmasseerr1, pl_bmasseerr2, \
             pl_eqt, st_teff, st_rad, st_mass, sy_dist, sy_pnum, rowupdate, releasedate \
             FROM exoplanets WHERE pl_name = ?"
        )
        .bind(pl_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to get exoplanet by name: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(row.map(|r| r.into()))
    }

    /// Get the most recently synced exoplanet
    pub async fn get_latest_exoplanet(&self) -> Result<Option<crate::models::Exoplanet>> {
        let row = sqlx::query_as::<_, ExoplanetRow>(
            "SELECT pl_name, hostname, discoverymethod, disc_year, disc_facility, disc_telescope, \
             pl_orbper, pl_orbpererr1, pl_orbpererr2, pl_orbperlim, \
             pl_rade, pl_radeerr1, pl_radeerr2, \
             pl_bmasse, pl_bmasseerr1, pl_bmasseerr2, \
             pl_eqt, st_teff, st_rad, st_mass, sy_dist, sy_pnum, rowupdate, releasedate \
             FROM exoplanets ORDER BY last_synced_at DESC, disc_year DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to get latest exoplanet: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(row.map(|r| r.into()))
    }

    /// Store discovery notification
    pub async fn store_discovery_notification(
        &self,
        pl_name: &str,
        hostname: &str,
        discovery_year: Option<i32>,
        discovery_method: Option<String>,
    ) -> Result<u64> {
        let result = sqlx::query(
            r#"
            INSERT INTO discovery_notifications (pl_name, hostname, discovery_year, discovery_method)
            VALUES (?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                hostname = VALUES(hostname),
                discovery_year = VALUES(discovery_year),
                discovery_method = VALUES(discovery_method)
            "#
        )
        .bind(pl_name)
        .bind(hostname)
        .bind(discovery_year)
        .bind(discovery_method)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to store discovery notification: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(result.last_insert_id())
    }

    /// Get unprocessed discovery notifications
    pub async fn get_unprocessed_discoveries(&self) -> Result<Vec<crate::models::DiscoveryNotification>> {
        let rows = sqlx::query_as::<_, DiscoveryNotificationRow>(
            "SELECT id, pl_name, hostname, discovery_year, discovery_method, notification_sent, created_at, notified_at
             FROM discovery_notifications
             WHERE notification_sent = FALSE
             ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to get unprocessed discoveries: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    /// Mark discovery notification as sent
    pub async fn mark_notification_sent(&self, id: u64) -> Result<()> {
        sqlx::query(
            "UPDATE discovery_notifications SET notification_sent = TRUE, notified_at = NOW() WHERE id = ?"
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to mark notification as sent: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(())
    }

    /// Store solar flare prediction
    pub async fn store_prediction(
        &self,
        prediction: &crate::services::PredictionResponse,
        input_features: &serde_json::Value,
        model_type: &str,
    ) -> Result<u64> {
        let result = sqlx::query(
            r#"
            INSERT INTO solar_flare_predictions (
                prediction_time, predicted_flare_class, predicted_peak_time,
                confidence_score, model_version, model_type, input_features
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(prediction.prediction_timestamp)
        .bind(&prediction.predicted_flare_class)
        .bind(prediction.predicted_peak_time)
        .bind(prediction.confidence_score)
        .bind(&prediction.model_version)
        .bind(model_type)
        .bind(input_features)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to store prediction: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(result.last_insert_id())
    }

    /// Update prediction with actual result
    pub async fn update_prediction_with_actual(
        &self,
        prediction_id: u64,
        actual_flare_class: Option<&str>,
        actual_peak_time: Option<DateTime<Utc>>,
    ) -> Result<()> {
        // Calculate if prediction was correct
        let prediction_correct = if let Some(actual) = actual_flare_class {
            // Get the original prediction
            let row = sqlx::query(
                "SELECT predicted_flare_class FROM solar_flare_predictions WHERE id = ?"
            )
            .bind(prediction_id)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(row) = row {
                let predicted: Option<String> = row.try_get("predicted_flare_class")?;
                Some(predicted.as_deref() == Some(actual))
            } else {
                None
            }
        } else {
            // No flare occurred - prediction of None would be correct
            None
        };

        sqlx::query(
            r#"
            UPDATE solar_flare_predictions
            SET actual_flare_class = ?,
                actual_peak_time = ?,
                prediction_correct = ?,
                updated_at = NOW()
            WHERE id = ?
            "#
        )
        .bind(actual_flare_class)
        .bind(actual_peak_time)
        .bind(prediction_correct)
        .bind(prediction_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to update prediction with actual result: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(())
    }

    /// Get recent predictions
    pub async fn get_recent_predictions(&self, limit: usize) -> Result<Vec<PredictionRow>> {
        let rows = sqlx::query_as::<_, PredictionRowInternal>(
            "SELECT id, prediction_time, predicted_flare_class, predicted_peak_time, \
             confidence_score, model_version, model_type, actual_flare_class, \
             prediction_correct, created_at \
             FROM solar_flare_predictions \
             ORDER BY prediction_time DESC \
             LIMIT ?"
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to get recent predictions: {}", e);
            crate::AppError::Database(e)
        })?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Get prediction accuracy statistics
    pub async fn get_prediction_accuracy(&self) -> Result<PredictionAccuracy> {
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total_predictions,
                SUM(CASE WHEN prediction_correct = TRUE THEN 1 ELSE 0 END) as correct_predictions,
                SUM(CASE WHEN prediction_correct = FALSE THEN 1 ELSE 0 END) as incorrect_predictions,
                AVG(confidence_score) as avg_confidence
            FROM solar_flare_predictions
            WHERE prediction_correct IS NOT NULL
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to get prediction accuracy: {}", e);
            crate::AppError::Database(e)
        })?;

        let total: i64 = row.try_get("total_predictions")?;
        let correct: i64 = row.try_get("correct_predictions")?;
        let incorrect: i64 = row.try_get("incorrect_predictions")?;
        let avg_confidence: Option<f64> = row.try_get("avg_confidence")?;

        let accuracy = if total > 0 {
            Some(correct as f64 / total as f64)
        } else {
            None
        };

        Ok(PredictionAccuracy {
            total_predictions: total as u64,
            correct_predictions: correct as u64,
            incorrect_predictions: incorrect as u64,
            accuracy,
            avg_confidence,
        })
    }
}

/// Public prediction row structure (for API responses)
#[derive(Debug, Clone)]
pub struct PredictionRow {
    pub id: u64,
    pub prediction_time: DateTime<Utc>,
    pub predicted_flare_class: Option<String>,
    pub predicted_peak_time: Option<DateTime<Utc>>,
    pub confidence_score: f64,
    pub model_version: String,
    pub model_type: String,
    pub actual_flare_class: Option<String>,
    pub prediction_correct: Option<bool>,
    pub created_at: DateTime<Utc>,
}

/// Database row structure for predictions (internal)
#[derive(sqlx::FromRow)]
struct PredictionRowInternal {
    id: u64,
    prediction_time: DateTime<Utc>,
    predicted_flare_class: Option<String>,
    predicted_peak_time: Option<DateTime<Utc>>,
    confidence_score: f64,
    model_version: String,
    model_type: String,
    actual_flare_class: Option<String>,
    prediction_correct: Option<bool>,
    created_at: DateTime<Utc>,
}

impl From<PredictionRowInternal> for PredictionRow {
    fn from(row: PredictionRowInternal) -> Self {
        Self {
            id: row.id,
            prediction_time: row.prediction_time,
            predicted_flare_class: row.predicted_flare_class,
            predicted_peak_time: row.predicted_peak_time,
            confidence_score: row.confidence_score,
            model_version: row.model_version,
            model_type: row.model_type,
            actual_flare_class: row.actual_flare_class,
            prediction_correct: row.prediction_correct,
            created_at: row.created_at,
        }
    }
}

/// Prediction accuracy statistics
#[derive(Debug)]
pub struct PredictionAccuracy {
    pub total_predictions: u64,
    pub correct_predictions: u64,
    pub incorrect_predictions: u64,
    pub accuracy: Option<f64>,
    pub avg_confidence: Option<f64>,
}

/// Database row structure for exoplanets
#[derive(sqlx::FromRow)]
struct ExoplanetRow {
    pl_name: String,
    hostname: String,
    discoverymethod: Option<String>,
    disc_year: Option<i32>,
    disc_facility: Option<String>,
    disc_telescope: Option<String>,
    pl_orbper: Option<f64>,
    pl_orbpererr1: Option<f64>,
    pl_orbpererr2: Option<f64>,
    pl_orbperlim: Option<f64>,
    pl_rade: Option<f64>,
    pl_radeerr1: Option<f64>,
    pl_radeerr2: Option<f64>,
    pl_bmasse: Option<f64>,
    pl_bmasseerr1: Option<f64>,
    pl_bmasseerr2: Option<f64>,
    pl_eqt: Option<f64>,
    st_teff: Option<f64>,
    st_rad: Option<f64>,
    st_mass: Option<f64>,
    sy_dist: Option<f64>,
    sy_pnum: Option<i32>,
    rowupdate: Option<String>,
    releasedate: Option<String>,
}

impl From<ExoplanetRow> for crate::models::Exoplanet {
    fn from(row: ExoplanetRow) -> Self {
        Self {
            pl_name: row.pl_name,
            hostname: row.hostname,
            discoverymethod: row.discoverymethod,
            disc_year: row.disc_year,
            disc_facility: row.disc_facility,
            disc_telescope: row.disc_telescope,
            pl_orbper: row.pl_orbper,
            pl_orbpererr1: row.pl_orbpererr1,
            pl_orbpererr2: row.pl_orbpererr2,
            pl_orbperlim: row.pl_orbperlim,
            pl_rade: row.pl_rade,
            pl_radeerr1: row.pl_radeerr1,
            pl_radeerr2: row.pl_radeerr2,
            pl_bmasse: row.pl_bmasse,
            pl_bmasseerr1: row.pl_bmasseerr1,
            pl_bmasseerr2: row.pl_bmasseerr2,
            pl_eqt: row.pl_eqt,
            st_teff: row.st_teff,
            st_rad: row.st_rad,
            st_mass: row.st_mass,
            sy_dist: row.sy_dist,
            sy_pnum: row.sy_pnum,
            rowupdate: row.rowupdate,
            releasedate: row.releasedate,
        }
    }
}

/// Database row structure for discovery notifications
#[derive(sqlx::FromRow)]
struct DiscoveryNotificationRow {
    id: u64,
    pl_name: String,
    hostname: String,
    discovery_year: Option<i32>,
    discovery_method: Option<String>,
    notification_sent: bool,
    created_at: DateTime<Utc>,
    notified_at: Option<DateTime<Utc>>,
}

impl From<DiscoveryNotificationRow> for crate::models::DiscoveryNotification {
    fn from(row: DiscoveryNotificationRow) -> Self {
        Self {
            id: row.id,
            pl_name: row.pl_name,
            hostname: row.hostname,
            discovery_year: row.discovery_year,
            discovery_method: row.discovery_method,
            notification_sent: row.notification_sent,
            created_at: row.created_at,
            notified_at: row.notified_at,
        }
    }
}

