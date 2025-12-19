use axum::{
    routing::get,
    Router,
    Json,
};
use serde_json::{json, Value};
use std::time::SystemTime;

/// Create the main application router
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/health", get(health_check))
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

