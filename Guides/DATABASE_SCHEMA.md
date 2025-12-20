# Database Schema Documentation

## Overview

The Rusty Server database is designed to store 10+ years of space weather data with efficient querying capabilities. The schema uses MySQL with InnoDB engine for ACID compliance and data integrity.

## Tables

### `space_weather_observations`

Primary table for storing space weather data points.

**Columns:**
- `id` (BIGINT UNSIGNED, PRIMARY KEY): Auto-incrementing unique identifier
- `timestamp` (DATETIME, NOT NULL): When the observation was recorded
- `source` (VARCHAR(50), DEFAULT 'noaa'): Data source identifier
- `cached` (BOOLEAN, DEFAULT FALSE): Whether this data was served from cache

**KP Index Data:**
- `kp_index_value` (DECIMAL(3,2), NULL): KP index value (0-9)
- `kp_index_level` (VARCHAR(20), NULL): KP index level (Quiet, Unsettled, Active, etc.)

**Geomagnetic Storm Data:**
- `geomagnetic_storm_level` (VARCHAR(10), NULL): Storm level (G1-G5, None)
- `geomagnetic_storm_kp_index` (DECIMAL(3,2), NULL): Associated KP index
- `geomagnetic_storm_start_time` (DATETIME, NULL): Storm start time
- `geomagnetic_storm_end_time` (DATETIME, NULL): Storm end time

**Solar Wind Data:**
- `solar_wind_speed` (DECIMAL(8,2), NULL): Wind speed in km/s
- `solar_wind_density` (DECIMAL(6,2), NULL): Density in cm^-3
- `solar_wind_temperature` (DECIMAL(10,2), NULL): Temperature in Kelvin
- `solar_wind_bz` (DECIMAL(6,2), NULL): Bz component in nT

**Solar Flare Data:**
- `solar_flare_class` (VARCHAR(10), NULL): Flare class (A, B, C, M, X)
- `solar_flare_peak_time` (DATETIME, NULL): Peak time of flare
- `solar_flare_begin_time` (DATETIME, NULL): Flare start time
- `solar_flare_end_time` (DATETIME, NULL): Flare end time
- `solar_flare_source_location` (VARCHAR(100), NULL): Active region identifier

**Radiation Data:**
- `radiation_proton_flux` (DECIMAL(10,4), NULL): Proton flux value
- `radiation_electron_flux` (DECIMAL(10,4), NULL): Electron flux value
- `radiation_alert_level` (VARCHAR(20), NULL): Alert level (None, Minor, Moderate, etc.)

**Metadata:**
- `created_at` (TIMESTAMP): Record creation timestamp
- `updated_at` (TIMESTAMP): Last update timestamp

**Indexes:**
- `idx_timestamp`: For time-range queries
- `idx_source`: For filtering by data source
- `idx_kp_index`: For KP index queries
- `idx_geomagnetic_storm`: For storm queries
- `idx_solar_wind_speed`: For solar wind queries
- `idx_created_at`: For data management

### `space_weather_alerts`

Stores active alerts and warnings.

**Columns:**
- `id` (BIGINT UNSIGNED, PRIMARY KEY): Auto-incrementing unique identifier
- `alert_type` (VARCHAR(50), NOT NULL): Type of alert (solar_flare, geomagnetic_storm, etc.)
- `severity` (VARCHAR(20), NOT NULL): Alert severity level
- `title` (VARCHAR(255), NOT NULL): Alert title
- `message` (TEXT, NOT NULL): Alert message/description
- `start_time` (DATETIME, NOT NULL): Alert start time
- `end_time` (DATETIME, NULL): Alert end time (NULL if ongoing)
- `active` (BOOLEAN, DEFAULT TRUE): Whether alert is currently active
- `source` (VARCHAR(50), DEFAULT 'noaa'): Alert source
- `created_at` (TIMESTAMP): Record creation timestamp
- `updated_at` (TIMESTAMP): Last update timestamp

**Indexes:**
- `idx_alert_type`: For filtering by alert type
- `idx_severity`: For filtering by severity
- `idx_active`: For finding active alerts
- `idx_start_time`: For time-based queries
- `idx_end_time`: For time-based queries

### `cache_metadata`

Tracks cache status and TTL for different data types.

**Columns:**
- `id` (BIGINT UNSIGNED, PRIMARY KEY): Auto-incrementing unique identifier
- `cache_key` (VARCHAR(255), UNIQUE): Unique cache key
- `cache_type` (VARCHAR(50), NOT NULL): Type of cache (current, historical, alerts)
- `data_type` (VARCHAR(50), NOT NULL): Data type identifier
- `expires_at` (DATETIME, NOT NULL): Cache expiration time
- `created_at` (TIMESTAMP): Record creation timestamp

**Indexes:**
- `idx_cache_key`: For cache lookups
- `idx_cache_type`: For cache type queries
- `idx_expires_at`: For cache cleanup

### `api_request_logs`

Optional table for API analytics and monitoring.

**Columns:**
- `id` (BIGINT UNSIGNED, PRIMARY KEY): Auto-incrementing unique identifier
- `endpoint` (VARCHAR(255), NOT NULL): API endpoint path
- `method` (VARCHAR(10), NOT NULL): HTTP method
- `status_code` (INT, NOT NULL): HTTP status code
- `response_time_ms` (INT, NULL): Response time in milliseconds
- `client_ip` (VARCHAR(45), NULL): Client IP address
- `user_agent` (TEXT, NULL): User agent string
- `timestamp` (TIMESTAMP): Request timestamp

**Indexes:**
- `idx_endpoint`: For endpoint analytics
- `idx_status_code`: For error monitoring
- `idx_timestamp`: For time-based queries

## Data Retention

The schema is designed to store **10+ years** of historical data. With proper indexing, queries remain efficient even with large datasets.

## Migration System

Migrations are managed using `sqlx` migrations. Migration files are stored in the `migrations/` directory and are automatically applied on server startup.

## Connection Pooling

The database uses connection pooling with the following settings:
- **Max connections**: 10
- **Min connections**: 2
- **Acquire timeout**: 30 seconds
- **Idle timeout**: 600 seconds (10 minutes)
- **Max lifetime**: 1800 seconds (30 minutes)

## Performance Considerations

1. **Indexes**: All frequently queried columns are indexed
2. **Partitioning**: Consider partitioning by year for very large datasets (future enhancement)
3. **Archival**: Old data can be archived to separate tables or databases
4. **Query Optimization**: Use EXPLAIN to analyze query performance

## Backup Strategy

Regular backups should be performed:
- **Full backups**: Daily
- **Incremental backups**: Every 6 hours
- **Retention**: 30 days of backups

## Future Enhancements

- Partitioning by year for better performance
- Data archival system for old data
- Read replicas for scaling
- Compression for historical data

