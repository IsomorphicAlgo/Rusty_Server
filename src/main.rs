use rusty_server::config::Config;
use rusty_server::{Result, ResultExt};
use rusty_server::logging;
use rusty_server::api::create_router;
use rusty_server::start_server;

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

    // Create the API router
    let router = create_router();

    // TODO: Initialize database connection pool
    // TODO: Initialize cache
    // TODO: Add middleware (logging, error handling, etc.)

    tracing::info!("Rusty Server initialized successfully");

    // Start the HTTP server
    start_server(router, &config.server.host, config.server.port)
        .await
        .map_err(|e| rusty_server::AppError::Internal(format!("Server error: {}", e)))?;

    Ok(())
}
