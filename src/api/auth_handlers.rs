// Authentication-related API handlers
// Endpoints for API key management

use axum::{extract::State, Json};
use serde_json::json;
use crate::AppState;
use crate::auth::ApiKey;
use crate::Result;

/// Generate a new API key
/// 
/// POST /api/v1/auth/keys
/// Body: { "name": "optional key name", "expires_in_days": 30 }
pub async fn generate_api_key(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiKey>> {
    let name = payload.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let expires_in_days = payload.get("expires_in_days").and_then(|v| v.as_u64());

    let api_key = state.api_key_store.generate_key(name, expires_in_days).await;
    
    tracing::info!("Generated new API key: {}", mask_key(&api_key.key));
    
    Ok(Json(api_key))
}

/// List all API keys (admin function)
/// 
/// GET /api/v1/auth/keys
pub async fn list_api_keys(
    State(state): State<AppState>,
) -> Json<Vec<ApiKey>> {
    let keys = state.api_key_store.list_keys().await;
    tracing::debug!("Listed {} API keys", keys.len());
    Json(keys)
}

/// Revoke an API key
/// 
/// DELETE /api/v1/auth/keys/{key}
pub async fn revoke_api_key(
    State(state): State<AppState>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>> {
    if state.api_key_store.revoke_key(&key).await {
        tracing::info!("Revoked API key: {}", mask_key(&key));
        Ok(Json(json!({
            "success": true,
            "message": "API key revoked"
        })))
    } else {
        Err(crate::AppError::NotFound(format!("API key not found")))
    }
}

/// Mask API key for logging
fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        "****".to_string()
    } else {
        format!("{}****", &key[..8])
    }
}


