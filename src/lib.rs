// Core modules
pub mod api;
pub mod services;
pub mod models;
pub mod database;
pub mod cache;
pub mod config;
pub mod auth;
pub mod errors;
pub mod logging;

// Re-export commonly used types
pub use errors::{AppError, Result, ResultExt};
pub use config::Config;

