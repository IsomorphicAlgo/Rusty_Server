use thiserror::Error;
use tracing::{error, warn};

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl AppError {
    /// Log the error with appropriate level
    pub fn log(&self) {
        match self {
            AppError::Config(e) => {
                error!(error = %e, "Configuration error occurred");
            }
            AppError::Database(e) => {
                error!(error = %e, "Database error occurred");
            }
            AppError::Http(e) => {
                warn!(error = %e, "HTTP request failed");
            }
            AppError::Serialization(e) => {
                warn!(error = %e, "Serialization error occurred");
            }
            AppError::Io(e) => {
                error!(error = %e, "IO error occurred");
            }
            AppError::Auth(msg) => {
                warn!(message = %msg, "Authentication error");
            }
            AppError::Validation(msg) => {
                warn!(message = %msg, "Validation error");
            }
            AppError::NotFound(msg) => {
                warn!(message = %msg, "Resource not found");
            }
            AppError::Internal(msg) => {
                error!(message = %msg, "Internal error occurred");
            }
        }
    }

    /// Get HTTP status code for the error
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::Config(_) => 500,
            AppError::Database(_) => 500,
            AppError::Http(_) => 502,
            AppError::Serialization(_) => 400,
            AppError::Io(_) => 500,
            AppError::Auth(_) => 401,
            AppError::Validation(_) => 400,
            AppError::NotFound(_) => 404,
            AppError::Internal(_) => 500,
        }
    }

    /// Check if error should be logged as error level
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            AppError::Config(_) | AppError::Database(_) | AppError::Io(_) | AppError::Internal(_)
        )
    }
}

/// Result type alias for application errors
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        let config_err = AppError::Config(config::ConfigError::Message("test".to_string()));
        assert_eq!(config_err.status_code(), 500);

        let auth_err = AppError::Auth("unauthorized".to_string());
        assert_eq!(auth_err.status_code(), 401);

        let not_found_err = AppError::NotFound("resource".to_string());
        assert_eq!(not_found_err.status_code(), 404);

        let validation_err = AppError::Validation("invalid".to_string());
        assert_eq!(validation_err.status_code(), 400);
    }

    #[test]
    fn test_error_is_critical() {
        let config_err = AppError::Config(config::ConfigError::Message("test".to_string()));
        assert!(config_err.is_critical());

        let auth_err = AppError::Auth("unauthorized".to_string());
        assert!(!auth_err.is_critical());

        let validation_err = AppError::Validation("invalid".to_string());
        assert!(!validation_err.is_critical());
    }

    #[test]
    fn test_result_ext_log_error() {
        let result: Result<()> = Err(AppError::Validation("test".to_string()));
        // Should not panic, just log
        let _ = result.log_error();
    }

    #[test]
    fn test_result_ext_log_critical() {
        let critical_result: Result<()> = Err(AppError::Internal("test".to_string()));
        // Should log critical errors
        let _ = critical_result.log_critical();

        let non_critical_result: Result<()> = Err(AppError::Validation("test".to_string()));
        // Should not log non-critical errors
        let _ = non_critical_result.log_critical();
    }
}

/// Extension trait for Result to add logging
pub trait ResultExt<T> {
    /// Log the error if it's an Err, then return the result
    fn log_error(self) -> Self;
    
    /// Log the error if it's an Err and is critical, then return the result
    fn log_critical(self) -> Self;
}

impl<T> ResultExt<T> for Result<T> {
    fn log_error(self) -> Self {
        if let Err(ref e) = self {
            e.log();
        }
        self
    }

    fn log_critical(self) -> Self {
        if let Err(ref e) = self {
            if e.is_critical() {
                e.log();
            }
        }
        self
    }
}

