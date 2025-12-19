use rusty_server::config::Config;
use rusty_server::Result;
use rusty_server::logging;
use rusty_server::api::create_router;
use rusty_server::start_server;
use rusty_server::services::NoaaClient;
use rusty_server::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration first (before logging initialization)
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            eprintln!("Using default configuration values.");
            // Return error or use defaults - for now, return error
            return Err(rusty_server::AppError::Config(e));
        }
    };

    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("Configuration validation failed: {}", e);
        return Err(rusty_server::AppError::Validation(e));
    }

    // Initialize logging with configured settings (only once)
    logging::init_logging(&config.logging.level, &config.logging.format);

    // Log startup information
    logging::log_startup(&config);

    // Initialize NOAA API client
    let noaa_client = NoaaClient::new(
        config.noaa.base_url.clone(),
        config.noaa.api_key.clone(),
        config.noaa.timeout_seconds,
    );

    // Create application state
    let app_state = AppState::new(noaa_client);

    // Create the API router
    let router = create_router(app_state);

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
