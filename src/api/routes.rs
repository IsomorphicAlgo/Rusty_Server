use axum::{
    routing::get,
    Router,
    Json,
};
use serde_json::{json, Value};
use std::time::SystemTime;

use super::handlers;

/// Create the main application router
pub fn create_router() -> Router {
    Router::new()
        // Health check endpoints
        .route("/health", get(health_check))
        .route("/api/v1/health", get(health_check))
        // Space weather API endpoints
        .route("/api/v1/space-weather/current", get(handlers::get_current_conditions))
        .route("/api/v1/space-weather/historical", get(handlers::get_historical_data))
        .route("/api/v1/space-weather/alerts", get(handlers::get_alerts))
        .route("/api/v1/space-weather/radiation", get(handlers::get_radiation))
}

/// Health check endpoint
/// Returns server status and basic information
async fn health_check() -> Json<Value> {
    let timestamp = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Json(json!({
        "status": "healthy",
        "timestamp": timestamp,
        "service": "rusty-server",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

