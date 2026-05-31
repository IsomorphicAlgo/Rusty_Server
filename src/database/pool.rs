use crate::Result;
use sqlx::{MySql, Pool};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use std::time::Duration;
use tracing::{info, error};

/// Database connection pool manager
#[derive(Clone)]
pub struct DatabasePool {
    pool: Pool<MySql>,
}

fn default_pool_options() -> MySqlPoolOptions {
    MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
}

impl DatabasePool {
    /// Create a new database connection pool from a URL string (e.g. `mysql://user:pass@host:3306/db`).
    pub async fn new(connection_string: &str) -> Result<Self> {
        info!("Initializing database connection pool...");
        let options: MySqlConnectOptions = connection_string
            .parse()
            .map_err(|e| {
                error!("Invalid MySQL connection string: {}", e);
                crate::AppError::Internal(format!("Invalid MySQL connection string: {e}"))
            })?;
        Self::connect_with(options).await
    }

    /// Create a new pool from structured options (avoids URL-encoding issues with special characters in passwords).
    pub async fn connect_with(options: MySqlConnectOptions) -> Result<Self> {
        info!("Initializing database connection pool...");

        let pool = default_pool_options()
            .connect_with(options)
            .await
            .map_err(|e| {
                error!("Failed to create database connection pool: {}", e);
                crate::AppError::Database(e)
            })?;

        info!("Database connection pool created successfully");

        Ok(Self { pool })
    }

    /// Create a pool that does not connect to MySQL until the first query.
    /// Use for HTTP integration tests on routes that never touch the database (e.g. ephemeris).
    pub fn connect_lazy(options: MySqlConnectOptions) -> Result<Self> {
        info!("Creating lazy MySQL pool (handshake deferred until first use)...");

        let pool = MySqlPoolOptions::new()
            .max_connections(2)
            .min_connections(0)
            .connect_lazy_with(options);

        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &Pool<MySql> {
        &self.pool
    }

    /// Check if the database connection is healthy
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Database health check failed: {}", e);
                crate::AppError::Database(e)
            })?;
        
        Ok(())
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations...");
        
        // Use sqlx's runtime migrations
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| {
                error!("Database migration failed: {}", e);
                // Convert MigrateError to Internal error
                crate::AppError::Internal(format!("Database migration failed: {}", e))
            })?;
        
        info!("Database migrations completed successfully");
        Ok(())
    }
}

