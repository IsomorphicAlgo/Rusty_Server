use crate::Result;
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;
use tracing::{info, error};

/// Database connection pool manager
#[derive(Clone)]
pub struct DatabasePool {
    pool: Pool<MySql>,
}

impl DatabasePool {
    /// Create a new database connection pool
    pub async fn new(connection_string: &str) -> Result<Self> {
        info!("Initializing database connection pool...");
        
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .min_connections(2)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(connection_string)
            .await
            .map_err(|e| {
                error!("Failed to create database connection pool: {}", e);
                crate::AppError::Database(e)
            })?;

        info!("Database connection pool created successfully");
        
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

