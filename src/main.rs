use rusty_server::config::Config;
use rusty_server::Result;
use rusty_server::logging;
use rusty_server::api::create_router;
use rusty_server::api::create_rate_limiter;
use rusty_server::start_server;
use rusty_server::services::{NoaaClient, DonkiClient};
use rusty_server::database::DatabasePool;
use rusty_server::auth::ApiKeyStore;
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

    // Initialize DONKI API client
    let donki_client = DonkiClient::new(
        config.donki.base_url.clone(),
        config.donki.api_key.clone(),
        config.donki.timeout_seconds,
    );
    if config.donki.api_key.is_some() {
        tracing::info!("DONKI API client initialized with API key");
    } else {
        tracing::warn!("DONKI API key not configured - solar flare data will not be available. Set RUSTY_SERVER__DONKI__API_KEY environment variable or add to config.toml");
    }

    // Initialize database connection pool
    let db_pool = DatabasePool::new(&config.database.connection_string).await?;
    
    // Run database migrations
    db_pool.migrate().await?;
    
    // Perform database health check
    db_pool.health_check().await?;
    tracing::info!("Database connection verified");

    // Initialize cache
    let cache = rusty_server::SpaceWeatherCache::new(&config.cache);
    tracing::info!("Cache initialized with TTLs: current={}s, historical={}s, alerts={}s",
        config.cache.current_conditions_ttl_seconds,
        config.cache.historical_data_ttl_seconds,
        config.cache.alerts_ttl_seconds
    );

    // Initialize rate limiter
    let rate_limiter = create_rate_limiter(&config.rate_limit);
    tracing::info!("Rate limiter initialized: {} req/min, burst: {}",
        config.rate_limit.requests_per_minute,
        config.rate_limit.burst_size
    );

    // Initialize API key store
    let api_key_store = ApiKeyStore::new();
    if config.auth.require_auth {
        tracing::info!("Authentication is required - API keys must be provided");
    } else {
        tracing::info!("Authentication is optional - API keys not required");
    }

    // Create application state
    let app_state = AppState::new(noaa_client, donki_client, db_pool, cache, rate_limiter, api_key_store);

    // Create the API router
    let router = create_router(app_state, config.clone());

    // TODO: Add middleware (logging, error handling, etc.)

    tracing::info!("Rusty Server initialized successfully");

    // Start the HTTP server
    start_server(router, &config.server.host, config.server.port)
        .await
        .map_err(|e| rusty_server::AppError::Internal(format!("Server error: {}", e)))?;

    Ok(())
}
