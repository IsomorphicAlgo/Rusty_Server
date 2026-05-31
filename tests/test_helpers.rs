//! Shared helpers for integration tests (`tests/*.rs`). Each test binary only uses a subset of these; suppress `dead_code` noise.
#![allow(dead_code)]

use rusty_server::{
    AppState,
    services::{DonkiClient, ExoplanetClient, NoaaClient},
    database::DatabasePool,
    SpaceWeatherCache,
    config::{
        AuthConfig, CacheConfig, Config, DonkiConfig, ExoplanetConfig, MLServiceConfig,
        RateLimitConfig, SecurityConfig,
    },
};
use rusty_server::api::create_rate_limiter;
use rusty_server::auth::ApiKeyStore;
use sqlx::mysql::MySqlConnectOptions;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use urlencoding::encode;

/// Parse the second (test) DB block from `credentials.txt` content.
fn parse_test_db_credentials(content: &str) -> Option<(String, String, String, String, String)> {
    let mut user = None;
    let mut password = None;
    let mut host = None;
    let mut port = None;
    let mut db_name = None;

    // Look for test database credentials (second occurrence of DB_USER)
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

/// Parse credentials from `credentials.txt` next to `Cargo.toml`, then cwd (integration tests sometimes differ).
fn parse_credentials_file() -> Option<(String, String, String, String, String)> {
    let candidates = [
        Path::new(env!("CARGO_MANIFEST_DIR")).join("credentials.txt"),
        PathBuf::from("credentials.txt"),
    ];
    for creds_path in candidates {
        if !creds_path.exists() {
            continue;
        }
        let Ok(content) = fs::read_to_string(&creds_path) else {
            continue;
        };
        if let Some(parsed) = parse_test_db_credentials(&content) {
            return Some(parsed);
        }
    }
    None
}

/// Normalize MySQL port for connection URLs (empty or invalid env/credential values break sqlx parsing).
fn sanitize_mysql_port(raw: &str) -> String {
    let t = raw.trim();
    if t.is_empty() {
        return "3306".to_string();
    }
    match t.parse::<u16>() {
        Ok(p) if p > 0 => p.to_string(),
        _ => "3306".to_string(),
    }
}

fn mysql_test_connect_options(
    user: &str,
    password: &str,
    host: &str,
    port_raw: &str,
    database: &str,
) -> MySqlConnectOptions {
    let port: u16 = sanitize_mysql_port(port_raw).parse().unwrap_or(3306);
    MySqlConnectOptions::new()
        .host(host)
        .port(port)
        .username(user)
        .password(password)
        .database(database)
}

fn test_db_connect_options_from_env() -> Option<MySqlConnectOptions> {
    if let (Ok(user), Ok(password), Ok(host), Ok(port), Ok(db_name)) = (
        env::var("DB_USER"),
        env::var("DB_PASSWORD"),
        env::var("DB_HOST"),
        env::var("DB_PORT"),
        env::var("DB_NAME"),
    ) {
        let user = user.trim();
        let host = host.trim();
        let db_name = db_name.trim();
        if !user.is_empty() && !host.is_empty() && !db_name.is_empty() {
            return Some(mysql_test_connect_options(
                user,
                password.trim(),
                host,
                port.trim(),
                db_name,
            ));
        }
    }
    None
}

/// Resolved MySQL options for integration tests (env → credentials.txt → defaults).
pub fn test_mysql_connect_options() -> MySqlConnectOptions {
    test_db_connect_options_from_env()
        .or_else(|| {
            parse_credentials_file().map(|(user, password, host, port, db_name)| {
                mysql_test_connect_options(&user, &password, &host, &port, &db_name)
            })
        })
        .unwrap_or_else(|| {
            mysql_test_connect_options(
                "rusty_user",
                "password",
                "localhost",
                "3306",
                "rusty_server_test",
            )
        })
}

fn mysql_test_connection_url(
    user: &str,
    password: &str,
    host: &str,
    port_raw: &str,
    db_name: &str,
) -> String {
    let port = sanitize_mysql_port(port_raw);
    format!(
        "mysql://{}:{}@{}:{}/{}",
        encode(user),
        encode(password),
        host,
        port,
        db_name
    )
}

/// Build test database connection string from environment variables, credentials.txt, or defaults
pub fn get_test_db_connection_string() -> String {
    // Try environment variables first (treat empty strings as unset — otherwise URLs like `mysql://:@:3306/` break sqlx)
    if let (Ok(user), Ok(password), Ok(host), Ok(port), Ok(db_name)) = (
        env::var("DB_USER"),
        env::var("DB_PASSWORD"),
        env::var("DB_HOST"),
        env::var("DB_PORT"),
        env::var("DB_NAME"),
    ) {
        let user = user.trim();
        let host = host.trim();
        let db_name = db_name.trim();
        if !user.is_empty() && !host.is_empty() && !db_name.is_empty() {
            return mysql_test_connection_url(user, &password, host, &port, db_name);
        }
    }
    
    // Try to read from credentials.txt
    if let Some((user, password, host, port, db_name)) = parse_credentials_file() {
        return mysql_test_connection_url(&user, &password, &host, &port, &db_name);
    }
    
    // Fallback to defaults
    mysql_test_connection_url(
        "rusty_user",
        "password",
        "localhost",
        "3306",
        "rusty_server_test",
    )
}

/// Create test application state with test database
pub async fn create_test_state() -> AppState {
    let noaa_client = NoaaClient::new(
        "https://services.swpc.noaa.gov".to_string(),
        None,
        30,
    );
    
    // Create DONKI client for tests (with optional API key from env)
    let donki_api_key = std::env::var("DONKI_API_KEY").ok();
    let donki_client = DonkiClient::new(
        "https://api.nasa.gov/DONKI".to_string(),
        donki_api_key,
        30,
    );
    
    let db_pool = DatabasePool::connect_with(test_mysql_connect_options())
        .await
        .expect("Failed to connect to test database. Make sure MySQL is running and test database exists.");
    
    let cache_config = CacheConfig::default();
    let cache = SpaceWeatherCache::new(&cache_config);
    
    let rate_limit_config = RateLimitConfig::default();
    let rate_limiter = create_rate_limiter(&rate_limit_config);
    
    let api_key_store = ApiKeyStore::new();
    
    let exoplanet_cfg = ExoplanetConfig::default();
    let exoplanet_client = ExoplanetClient::new(
        exoplanet_cfg.base_url.clone(),
        exoplanet_cfg.timeout_seconds,
    );

    AppState::new(
        noaa_client,
        donki_client,
        exoplanet_client,
        None,
        db_pool,
        cache,
        rate_limiter,
        api_key_store,
    )
}

/// [`AppState`] for HTTP tests on routes that never query MySQL (e.g. ephemeris). Uses a **lazy** pool so tests run without a live database.
pub fn create_test_state_ephemeris() -> AppState {
    let noaa_client = NoaaClient::new(
        "https://services.swpc.noaa.gov".to_string(),
        None,
        30,
    );

    let donki_api_key = std::env::var("DONKI_API_KEY").ok();
    let donki_client = DonkiClient::new(
        "https://api.nasa.gov/DONKI".to_string(),
        donki_api_key,
        30,
    );

    let db_pool = DatabasePool::connect_lazy(test_mysql_connect_options())
        .expect("lazy MySQL pool options should always be valid");

    let cache_config = CacheConfig::default();
    let cache = SpaceWeatherCache::new(&cache_config);

    let rate_limit_config = RateLimitConfig::default();
    let rate_limiter = create_rate_limiter(&rate_limit_config);

    let api_key_store = ApiKeyStore::new();

    let exoplanet_cfg = ExoplanetConfig::default();
    let exoplanet_client = ExoplanetClient::new(
        exoplanet_cfg.base_url.clone(),
        exoplanet_cfg.timeout_seconds,
    );

    AppState::new(
        noaa_client,
        donki_client,
        exoplanet_client,
        None,
        db_pool,
        cache,
        rate_limiter,
        api_key_store,
    )
}

/// Create test configuration (authentication disabled by default for tests)
pub fn create_test_config() -> Config {
    Config {
        server: rusty_server::config::ServerConfig::default(),
        database: rusty_server::config::DatabaseConfig::default(),
        noaa: rusty_server::config::NoaaConfig::default(),
        donki: DonkiConfig::default(),
        exoplanet: ExoplanetConfig::default(),
        ml_service: MLServiceConfig::default(),
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

