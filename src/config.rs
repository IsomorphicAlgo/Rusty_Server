use serde::{Deserialize, Serialize};
use std::env;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub noaa: NoaaConfig,
    pub donki: DonkiConfig,
    pub exoplanet: ExoplanetConfig,
    pub ml_service: MLServiceConfig,
    pub cache: CacheConfig,
    pub rate_limit: RateLimitConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
    pub security: SecurityConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            // Using test database by default for development
            // Change to rusty_server when ready for production
            connection_string: "mysql://user:password@localhost/rusty_server_test".to_string(),
            max_connections: 10,
        }
    }
}

/// NOAA API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoaaConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
}

impl Default for NoaaConfig {
    fn default() -> Self {
        Self {
            base_url: "https://services.swpc.noaa.gov".to_string(),
            api_key: None,
            timeout_seconds: 30,
        }
    }
}

/// DONKI API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonkiConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
}

impl Default for DonkiConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.nasa.gov/DONKI".to_string(),
            api_key: None,
            timeout_seconds: 30,
        }
    }
}

/// Exoplanet Archive TAP API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExoplanetConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
}

impl Default for ExoplanetConfig {
    fn default() -> Self {
        Self {
            base_url: "https://exoplanetarchive.ipac.caltech.edu/TAP".to_string(),
            timeout_seconds: 60, // Longer timeout for TAP queries
        }
    }
}

/// ML Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLServiceConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub enabled: bool,
}

impl Default for MLServiceConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8001".to_string(),
            timeout_seconds: 30,
            enabled: false, // Disabled by default until model is trained
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub current_conditions_ttl_seconds: u64,
    pub historical_data_ttl_seconds: u64,
    pub alerts_ttl_seconds: u64,
    pub max_size_mb: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            current_conditions_ttl_seconds: 900,  // 15 minutes
            historical_data_ttl_seconds: 3600,   // 1 hour
            alerts_ttl_seconds: 300,             // 5 minutes
            max_size_mb: 100,
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub burst_size: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            burst_size: 10,
        }
    }
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry_hours: u64,
    pub require_auth: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            token_expiry_hours: 24,
            require_auth: false,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String, // "json" or "pretty"
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// CORS allowed origins (comma-separated, or "*" for all)
    pub cors_allowed_origins: String,
    /// CORS allowed methods (comma-separated)
    pub cors_allowed_methods: String,
    /// CORS allowed headers (comma-separated)
    pub cors_allowed_headers: String,
    /// Maximum request body size in bytes
    pub max_request_size_bytes: u64,
    /// Enable HSTS (HTTP Strict Transport Security)
    pub enable_hsts: bool,
    /// HSTS max-age in seconds
    pub hsts_max_age_seconds: u64,
    /// Enable X-Content-Type-Options header
    pub enable_x_content_type_options: bool,
    /// Enable X-Frame-Options header
    pub enable_x_frame_options: bool,
    /// X-Frame-Options value (DENY, SAMEORIGIN, or ALLOW-FROM)
    pub x_frame_options_value: String,
    /// Enable X-XSS-Protection header
    pub enable_x_xss_protection: bool,
    /// Enable Referrer-Policy header
    pub enable_referrer_policy: bool,
    /// Referrer-Policy value
    pub referrer_policy_value: String,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            cors_allowed_origins: "*".to_string(),
            cors_allowed_methods: "GET,POST,PUT,DELETE,OPTIONS".to_string(),
            cors_allowed_headers: "Content-Type,Authorization,X-API-Key".to_string(),
            max_request_size_bytes: 10 * 1024 * 1024, // 10 MB
            enable_hsts: false, // Only enable in production with HTTPS
            hsts_max_age_seconds: 31536000, // 1 year
            enable_x_content_type_options: true,
            enable_x_frame_options: true,
            x_frame_options_value: "DENY".to_string(),
            enable_x_xss_protection: true,
            enable_referrer_policy: true,
            referrer_policy_value: "strict-origin-when-cross-origin".to_string(),
        }
    }
}

impl Config {
    /// Load configuration from environment variables and config file
    pub fn load() -> Result<Self, config::ConfigError> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        let mut builder = config::Config::builder()
            // Set defaults
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .set_default("database.max_connections", 10)?
            .set_default("database.connection_string", "mysql://user:password@localhost/rusty_server")?
            .set_default("noaa.base_url", "https://services.swpc.noaa.gov")?
            .set_default("noaa.timeout_seconds", 30)?
            .set_default("noaa.api_key", "")?
            .set_default("donki.base_url", "https://api.nasa.gov/DONKI")?
            .set_default("donki.timeout_seconds", 30)?
            .set_default("donki.api_key", "")?
            .set_default("exoplanet.base_url", "https://exoplanetarchive.ipac.caltech.edu/TAP")?
            .set_default("exoplanet.timeout_seconds", 60)?
            .set_default("ml_service.base_url", "http://localhost:8001")?
            .set_default("ml_service.timeout_seconds", 30)?
            .set_default("ml_service.enabled", false)?
            .set_default("cache.current_conditions_ttl_seconds", 900)?
            .set_default("cache.historical_data_ttl_seconds", 3600)?
            .set_default("cache.alerts_ttl_seconds", 300)?
            .set_default("cache.max_size_mb", 100)?
            .set_default("rate_limit.requests_per_minute", 60)?
            .set_default("rate_limit.requests_per_hour", 1000)?
            .set_default("rate_limit.burst_size", 10)?
            .set_default("auth.jwt_secret", "change-me-in-production")?
            .set_default("auth.token_expiry_hours", 24)?
            .set_default("auth.require_auth", false)?
            .set_default("logging.level", "info")?
            .set_default("logging.format", "pretty")?
            .set_default("security.cors_allowed_origins", "*")?
            .set_default("security.cors_allowed_methods", "GET,POST,PUT,DELETE,OPTIONS")?
            .set_default("security.cors_allowed_headers", "Content-Type,Authorization,X-API-Key")?
            .set_default("security.max_request_size_bytes", 10 * 1024 * 1024)?
            .set_default("security.enable_hsts", false)?
            .set_default("security.hsts_max_age_seconds", 31536000)?
            .set_default("security.enable_x_content_type_options", true)?
            .set_default("security.enable_x_frame_options", true)?
            .set_default("security.x_frame_options_value", "DENY")?
            .set_default("security.enable_x_xss_protection", true)?
            .set_default("security.enable_referrer_policy", true)?
            .set_default("security.referrer_policy_value", "strict-origin-when-cross-origin")?;

        // Load from config file if it exists
        if let Ok(config_file) = env::var("CONFIG_FILE") {
            builder = builder.add_source(config::File::with_name(&config_file));
        } else {
            // Try default config file locations
            for path in &["config.toml", "config/local.toml"] {
                if std::path::Path::new(path).exists() {
                    builder = builder.add_source(config::File::with_name(path));
                    break;
                }
            }
        }

        // Override with environment variables
        builder = builder.add_source(
            config::Environment::with_prefix("RUSTY_SERVER")
                .separator("__")
                .try_parsing(true),
        );

        // Build and deserialize
        let config = builder.build()?;
        config.try_deserialize()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate server config
        if self.server.port == 0 {
            return Err("Server port cannot be 0".to_string());
        }

        // Validate database config
        if self.database.connection_string.is_empty() {
            return Err("Database connection string cannot be empty".to_string());
        }

        // Validate auth config
        if self.auth.jwt_secret == "change-me-in-production" && self.auth.require_auth {
            return Err("JWT secret must be changed from default in production".to_string());
        }

        // Validate logging level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.logging.level.as_str()) {
            return Err(format!(
                "Invalid logging level: {}. Must be one of: {:?}",
                self.logging.level, valid_levels
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            noaa: NoaaConfig::default(),
            donki: DonkiConfig::default(),
            exoplanet: ExoplanetConfig::default(),
            ml_service: MLServiceConfig::default(),
            cache: CacheConfig::default(),
            rate_limit: RateLimitConfig::default(),
            auth: AuthConfig::default(),
            logging: LoggingConfig::default(),
            security: SecurityConfig::default(),
        };

        assert_eq!(config.server.port, 3000);
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.cache.current_conditions_ttl_seconds, 900);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 0, // Invalid
            },
            database: DatabaseConfig::default(),
            noaa: NoaaConfig::default(),
            donki: DonkiConfig::default(),
            exoplanet: ExoplanetConfig::default(),
            ml_service: MLServiceConfig::default(),
            cache: CacheConfig::default(),
            rate_limit: RateLimitConfig::default(),
            auth: AuthConfig::default(),
            logging: LoggingConfig::default(),
            security: SecurityConfig::default(),
        };

        assert!(config.validate().is_err());

        config.server.port = 3000;
        assert!(config.validate().is_ok());
    }
}

