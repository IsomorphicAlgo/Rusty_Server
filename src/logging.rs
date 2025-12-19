use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_subscriber::fmt;
use std::io;

/// Initialize the logging system based on configuration
pub fn init_logging(level: &str, format: &str) {
    // Create environment filter from log level
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::try_from(level).unwrap_or_else(|_| EnvFilter::new("info"))
        });

    // Create formatter based on format preference
    if format == "json" {
        // JSON format for structured logging (production)
        Registry::default()
            .with(env_filter)
            .with(fmt::layer().json().with_writer(io::stderr))
            .init();
    } else {
        // Pretty format for development
        Registry::default()
            .with(env_filter)
            .with(
                fmt::layer()
                    .pretty()
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_writer(io::stderr),
            )
            .init();
    }

    tracing::info!("Logging initialized: level={}, format={}", level, format);
}

/// Log application startup information
pub fn log_startup(config: &crate::config::Config) {
    tracing::info!("========================================");
    tracing::info!("Rusty Server Starting");
    tracing::info!("========================================");
    tracing::info!("Server: {}:{}", config.server.host, config.server.port);
    tracing::info!("Database: {}", mask_connection_string(&config.database.connection_string));
    tracing::info!("NOAA API: {}", config.noaa.base_url);
    tracing::info!("Cache TTL - Current: {}s, Historical: {}s, Alerts: {}s",
        config.cache.current_conditions_ttl_seconds,
        config.cache.historical_data_ttl_seconds,
        config.cache.alerts_ttl_seconds);
    tracing::info!("Rate Limit: {} req/min, {} req/hour",
        config.rate_limit.requests_per_minute,
        config.rate_limit.requests_per_hour);
    tracing::info!("Authentication: {}", if config.auth.require_auth { "Required" } else { "Optional" });
    tracing::info!("========================================");
}

/// Mask sensitive information in connection strings
fn mask_connection_string(conn_str: &str) -> String {
    // Mask password in connection string
    if let Some(at_pos) = conn_str.find('@') {
        if let Some(slash_pos) = conn_str[..at_pos].rfind('/') {
            let protocol = &conn_str[..=slash_pos];
            let rest = &conn_str[at_pos..];
            format!("{}***{}", protocol, rest)
        } else {
            format!("***{}", &conn_str[at_pos..])
        }
    } else {
        "***".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_connection_string() {
        let conn = "mysql://user:password@localhost/db";
        let masked = mask_connection_string(conn);
        assert!(masked.contains("***"));
        assert!(!masked.contains("password"));
        assert!(masked.contains("localhost"));
    }

    #[test]
    fn test_mask_connection_string_no_password() {
        let conn = "mysql://localhost/db";
        let masked = mask_connection_string(conn);
        // Should still mask something
        assert!(masked.contains("***"));
    }
}

