-- Exoplanet Archive Database Schema
-- Stores exoplanet data from NASA Exoplanet Archive
-- Created: 2024-12-20

-- Exoplanets Table
-- Stores exoplanet data from the Planetary Systems (ps) table
CREATE TABLE IF NOT EXISTS exoplanets (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    
    -- Basic identification
    pl_name VARCHAR(255) NOT NULL UNIQUE,
    hostname VARCHAR(255) NOT NULL,
    
    -- Discovery information
    discoverymethod VARCHAR(100) NULL,
    disc_year INT NULL,
    disc_facility VARCHAR(255) NULL,
    disc_telescope VARCHAR(255) NULL,
    
    -- Orbital parameters
    pl_orbper DECIMAL(15,5) NULL,  -- Orbital period in days
    pl_orbpererr1 DECIMAL(15,5) NULL,  -- Orbital period error (upper)
    pl_orbpererr2 DECIMAL(15,5) NULL,  -- Orbital period error (lower)
    pl_orbperlim DECIMAL(15,5) NULL,  -- Orbital period limit flag
    
    -- Physical parameters
    pl_rade DECIMAL(10,4) NULL,  -- Planet radius in Earth radii
    pl_radeerr1 DECIMAL(10,4) NULL,  -- Planet radius error (upper)
    pl_radeerr2 DECIMAL(10,4) NULL,  -- Planet radius error (lower)
    
    pl_bmasse DECIMAL(10,4) NULL,  -- Planet mass in Earth masses
    pl_bmasseerr1 DECIMAL(10,4) NULL,  -- Planet mass error (upper)
    pl_bmasseerr2 DECIMAL(10,4) NULL,  -- Planet mass error (lower)
    
    pl_eqt DECIMAL(8,2) NULL,  -- Equilibrium temperature in Kelvin
    
    -- Stellar parameters
    st_teff DECIMAL(6,2) NULL,  -- Stellar effective temperature in Kelvin
    st_rad DECIMAL(8,4) NULL,  -- Stellar radius in solar radii
    st_mass DECIMAL(8,4) NULL,  -- Stellar mass in solar masses
    
    -- System parameters
    sy_dist DECIMAL(10,4) NULL,  -- Distance to star in parsecs
    sy_pnum INT NULL,  -- Number of planets in system
    
    -- Metadata from archive
    rowupdate VARCHAR(50) NULL,  -- Last update timestamp from archive
    releasedate VARCHAR(50) NULL,  -- Release date
    
    -- Local metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    last_synced_at TIMESTAMP NULL,  -- Last time data was synced from archive
    
    -- Indexes for efficient querying
    INDEX idx_pl_name (pl_name),
    INDEX idx_hostname (hostname),
    INDEX idx_discoverymethod (discoverymethod),
    INDEX idx_disc_year (disc_year),
    INDEX idx_pl_rade (pl_rade),
    INDEX idx_pl_bmasse (pl_bmasse),
    INDEX idx_created_at (created_at),
    INDEX idx_last_synced_at (last_synced_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Discovery Notifications Table
-- Tracks exoplanet discoveries for notification purposes
CREATE TABLE IF NOT EXISTS discovery_notifications (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    pl_name VARCHAR(255) NOT NULL,
    hostname VARCHAR(255) NOT NULL,
    discovery_year INT NULL,
    discovery_method VARCHAR(100) NULL,
    notification_sent BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    notified_at TIMESTAMP NULL,
    
    -- Indexes
    INDEX idx_pl_name (pl_name),
    INDEX idx_notification_sent (notification_sent),
    INDEX idx_created_at (created_at),
    
    -- Foreign key constraint (optional, can be removed if needed)
    FOREIGN KEY (pl_name) REFERENCES exoplanets(pl_name) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

