use axum::{
    routing::get,
    Router,
    Json,
    middleware,
};
use serde_json::{json, Value};
use std::time::SystemTime;

use crate::AppState;
use crate::config::Config;
use super::handlers;
use super::auth_handlers;
use super::rate_limit::rate_limit_middleware;
use super::security::{create_cors_layer, create_request_size_limit_layer, security_headers_middleware};
use crate::auth::auth_middleware;

/// Create the main application router
pub fn create_router(state: AppState, config: Config) -> Router {
    let rate_limiter = state.rate_limiter.clone();
    let api_key_store = state.api_key_store.clone();
    let auth_config = config.auth.clone();
    let security_config = config.security.clone();
    
    // Create security layers
    let cors_layer = create_cors_layer(&security_config);
    let request_size_limit_layer = create_request_size_limit_layer(&security_config);
    
    // Create API router with rate limiting and authentication
    let api_routes = Router::new()
        .route("/api/v1/space-weather/current", get(handlers::get_current_conditions))
        .route("/api/v1/space-weather/historical", get(handlers::get_historical_data))
        .route("/api/v1/space-weather/alerts", get(handlers::get_alerts))
        .route("/api/v1/space-weather/radiation", get(handlers::get_radiation))
        // API key management endpoints (these should require auth in production)
        .route("/api/v1/auth/keys", axum::routing::post(auth_handlers::generate_api_key))
        .route("/api/v1/auth/keys", axum::routing::get(auth_handlers::list_api_keys))
        .route("/api/v1/auth/keys/:key", axum::routing::delete(auth_handlers::revoke_api_key))
        // Apply authentication middleware (if required by config)
        .route_layer(middleware::from_fn(move |request, next| {
            let store = api_key_store.clone();
            let auth_cfg = auth_config.clone();
            async move {
                auth_middleware(store, auth_cfg, request, next).await
            }
        }))
        // Apply rate limiting middleware to API routes
        .route_layer(middleware::from_fn(move |request, next| {
            let limiter = rate_limiter.clone();
            async move {
                rate_limit_middleware(limiter, request, next).await
            }
        }))
        .with_state(state.clone());
    
    // Combine health check (no rate limiting or auth) with API routes
    Router::new()
        // Health check endpoints (no rate limiting or auth)
        .route("/health", get(health_check))
        .route("/api/v1/health", get(health_check))
        // Merge API routes with rate limiting and authentication
        .merge(api_routes)
        // Apply security middleware layers
        .layer(cors_layer)
        .layer(request_size_limit_layer)
        .layer(middleware::from_fn(move |request, next| {
            let sec_cfg = security_config.clone();
            async move {
                security_headers_middleware(sec_cfg, request, next).await
            }
        }))
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

