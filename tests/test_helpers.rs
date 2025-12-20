use rusty_server::{AppState, services::NoaaClient, database::DatabasePool, SpaceWeatherCache, config::{CacheConfig, RateLimitConfig, AuthConfig, Config, SecurityConfig}};
use rusty_server::api::create_rate_limiter;
use rusty_server::auth::ApiKeyStore;
use std::env;
use std::fs;
use std::path::Path;

/// Parse credentials from credentials.txt file
fn parse_credentials_file() -> Option<(String, String, String, String, String)> {
    let creds_path = Path::new("credentials.txt");
    if !creds_path.exists() {
        return None;
    }
    
    let content = fs::read_to_string(creds_path).ok()?;
    let mut user = None;
    let mut password = None;
    let mut host = None;
    let mut port = None;
    let mut db_name = None;
    
    // Look for test database credentials (second occurrence)
    let mut found_first_db = false;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "DB_USER" => {
                    if found_first_db {
                        user = Some(value.to_string());
                    } else {
                        found_first_db = true;
                    }
                }
                "DB_PASSWORD" => {
                    if found_first_db {
                        password = Some(value.to_string());
                    }
                }
                "DB_HOST" => {
                    if found_first_db {
                        host = Some(value.to_string());
                    }
                }
                "DB_PORT" => {
                    if found_first_db {
                        port = Some(value.to_string());
                    }
                }
                "DB_NAME" => {
                    if found_first_db {
                        db_name = Some(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }
    
    if let (Some(u), Some(p), Some(h), Some(prt), Some(db)) = (user, password, host, port, db_name) {
        Some((u, p, h, prt, db))
    } else {
        None
    }
}

/// Build test database connection string from environment variables, credentials.txt, or defaults
pub fn get_test_db_connection_string() -> String {
    // Try environment variables first
    if let (Ok(user), Ok(password), Ok(host), Ok(port), Ok(db_name)) = (
        env::var("DB_USER"),
        env::var("DB_PASSWORD"),
        env::var("DB_HOST"),
        env::var("DB_PORT"),
        env::var("DB_NAME"),
    ) {
        return format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, db_name);
    }
    
    // Try to read from credentials.txt
    if let Some((user, password, host, port, db_name)) = parse_credentials_file() {
        return format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, db_name);
    }
    
    // Fallback to defaults
    format!("mysql://rusty_user:password@localhost:3306/rusty_server_test")
}

/// Create test application state with test database
pub async fn create_test_state() -> AppState {
    let noaa_client = NoaaClient::new(
        "https://services.swpc.noaa.gov".to_string(),
        None,
        30,
    );
    
    let connection_string = get_test_db_connection_string();
    let db_pool = DatabasePool::new(&connection_string)
        .await
        .expect("Failed to connect to test database. Make sure MySQL is running and test database exists.");
    
    let cache_config = CacheConfig::default();
    let cache = SpaceWeatherCache::new(&cache_config);
    
    let rate_limit_config = RateLimitConfig::default();
    let rate_limiter = create_rate_limiter(&rate_limit_config);
    
    let api_key_store = ApiKeyStore::new();
    
    AppState::new(noaa_client, db_pool, cache, rate_limiter, api_key_store)
}

/// Create test configuration (authentication disabled by default for tests)
pub fn create_test_config() -> Config {
    Config {
        server: rusty_server::config::ServerConfig::default(),
        database: rusty_server::config::DatabaseConfig::default(),
        noaa: rusty_server::config::NoaaConfig::default(),
        cache: CacheConfig::default(),
        rate_limit: RateLimitConfig::default(),
        auth: AuthConfig {
            jwt_secret: "test-secret".to_string(),
            token_expiry_hours: 24,
            require_auth: false, // Disable auth for tests by default
        },
        logging: rusty_server::config::LoggingConfig::default(),
        security: SecurityConfig::default(),
    }
}

