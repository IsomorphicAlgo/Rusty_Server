use rusty_server::config::Config;
use rusty_server::{Result, ResultExt};
use rusty_server::logging;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize basic logging first (before config loading)
    // This allows us to log config errors
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Load configuration
    let config = Config::load()
        .map_err(|e| rusty_server::AppError::Config(e))
        .log_error()?;

    // Validate configuration
    config.validate()
        .map_err(|e| rusty_server::AppError::Validation(e))
        .log_error()?;

    // Re-initialize logging with configured settings
    logging::init_logging(&config.logging.level, &config.logging.format);

    // Log startup information
    logging::log_startup(&config);

    // TODO: Initialize database connection pool
    // TODO: Initialize cache
    // TODO: Set up HTTP server with routes
    // TODO: Start server

    tracing::info!("Rusty Server initialized successfully");

    // For now, just wait (will be replaced with server startup)
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");

    tracing::info!("Shutting down gracefully...");
    Ok(())
}
