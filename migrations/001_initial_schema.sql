-- Rusty Server Database Schema
-- Designed to store 10+ years of space weather data
-- Created: 2024-12-19

-- Space Weather Observations Table
-- Stores aggregated space weather data points
CREATE TABLE IF NOT EXISTS space_weather_observations (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    timestamp DATETIME NOT NULL,
    source VARCHAR(50) NOT NULL DEFAULT 'noaa',
    cached BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- KP Index Data
    kp_index_value DECIMAL(3,2) NULL,
    kp_index_level VARCHAR(20) NULL,
    
    -- Geomagnetic Storm Data
    geomagnetic_storm_level VARCHAR(10) NULL,
    geomagnetic_storm_kp_index DECIMAL(3,2) NULL,
    geomagnetic_storm_start_time DATETIME NULL,
    geomagnetic_storm_end_time DATETIME NULL,
    
    -- Solar Wind Data
    solar_wind_speed DECIMAL(8,2) NULL,
    solar_wind_density DECIMAL(6,2) NULL,
    solar_wind_temperature DECIMAL(10,2) NULL,
    solar_wind_bz DECIMAL(6,2) NULL,
    
    -- Solar Flare Data
    solar_flare_class VARCHAR(10) NULL,
    solar_flare_peak_time DATETIME NULL,
    solar_flare_begin_time DATETIME NULL,
    solar_flare_end_time DATETIME NULL,
    solar_flare_source_location VARCHAR(100) NULL,
    
    -- Radiation Data
    radiation_proton_flux DECIMAL(10,4) NULL,
    radiation_electron_flux DECIMAL(10,4) NULL,
    radiation_alert_level VARCHAR(20) NULL,
    
    -- Metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    -- Indexes for efficient querying
    INDEX idx_timestamp (timestamp),
    INDEX idx_source (source),
    INDEX idx_kp_index (kp_index_value),
    INDEX idx_geomagnetic_storm (geomagnetic_storm_level),
    INDEX idx_solar_wind_speed (solar_wind_speed),
    INDEX idx_created_at (created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Space Weather Alerts Table
-- Stores active alerts and warnings
CREATE TABLE IF NOT EXISTS space_weather_alerts (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    alert_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    source VARCHAR(50) NOT NULL DEFAULT 'noaa',
    
    -- Metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    -- Indexes
    INDEX idx_alert_type (alert_type),
    INDEX idx_severity (severity),
    INDEX idx_active (active),
    INDEX idx_start_time (start_time),
    INDEX idx_end_time (end_time)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Cache Metadata Table
-- Tracks cache status and TTL for different data types
CREATE TABLE IF NOT EXISTS cache_metadata (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    cache_key VARCHAR(255) NOT NULL UNIQUE,
    cache_type VARCHAR(50) NOT NULL,
    data_type VARCHAR(50) NOT NULL,
    expires_at DATETIME NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    -- Indexes
    INDEX idx_cache_key (cache_key),
    INDEX idx_cache_type (cache_type),
    INDEX idx_expires_at (expires_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- API Request Log Table (Optional - for analytics)
CREATE TABLE IF NOT EXISTS api_request_logs (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    endpoint VARCHAR(255) NOT NULL,
    method VARCHAR(10) NOT NULL,
    status_code INT NOT NULL,
    response_time_ms INT NULL,
    client_ip VARCHAR(45) NULL,
    user_agent TEXT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    -- Indexes
    INDEX idx_endpoint (endpoint),
    INDEX idx_status_code (status_code),
    INDEX idx_timestamp (timestamp)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

