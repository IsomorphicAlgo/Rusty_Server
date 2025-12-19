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
pub mod server;
pub mod state;

// Re-export commonly used types
pub use errors::{AppError, Result, ResultExt};
pub use config::Config;
pub use server::start_server;
pub use models::*;
pub use state::AppState;

