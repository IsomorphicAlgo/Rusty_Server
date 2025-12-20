// Authentication middleware for API key validation

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse, Json},
};
use tracing::{warn, debug};
use crate::config::AuthConfig;
use super::api_key::ApiKeyStore;

/// Extract API key from request headers
/// 
/// Looks for API key in:
/// - Authorization header: "Bearer <key>" or "ApiKey <key>"
/// - X-API-Key header
fn extract_api_key(headers: &HeaderMap) -> Option<String> {
    // Try Authorization header first
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // Support "Bearer <key>" or "ApiKey <key>" format
            if let Some(key) = auth_str.strip_prefix("Bearer ") {
                return Some(key.trim().to_string());
            }
            if let Some(key) = auth_str.strip_prefix("ApiKey ") {
                return Some(key.trim().to_string());
            }
            // Also support just the key directly
            if !auth_str.contains(' ') {
                return Some(auth_str.to_string());
            }
        }
    }

    // Try X-API-Key header
    if let Some(api_key_header) = headers.get("x-api-key") {
        if let Ok(key_str) = api_key_header.to_str() {
            return Some(key_str.to_string());
        }
    }

    None
}

/// Authentication middleware
/// 
/// Validates API keys for protected endpoints.
/// If authentication is required but no valid key is provided,
/// returns 401 Unauthorized.
pub async fn auth_middleware(
    api_key_store: ApiKeyStore,
    config: AuthConfig,
    request: Request,
    next: Next,
) -> Response {
    // If authentication is not required, skip validation
    if !config.require_auth {
        return next.run(request).await;
    }

    // Extract API key from headers
    let api_key = match extract_api_key(request.headers()) {
        Some(key) => key,
        None => {
            debug!("No API key provided in request");
            // Log security event
            crate::api::security::log_security_event(
                "authentication_failure",
                "No API key provided in request",
                "warning"
            );
            return create_unauthorized_response("API key required".to_string());
        }
    };

    // Validate API key
    if !api_key_store.validate_key(&api_key).await {
        let masked_key = mask_key(&api_key);
        warn!("Invalid API key attempted: {}", masked_key);
        // Log security event
        crate::api::security::log_security_event(
            "authentication_failure",
            &format!("Invalid API key: {}", masked_key),
            "warning"
        );
        return create_unauthorized_response("Invalid API key".to_string());
    }

    debug!("API key validated successfully");
    next.run(request).await
}

/// Create 401 Unauthorized response
fn create_unauthorized_response(message: String) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({
            "error": "Unauthorized",
            "message": message
        })),
    ).into_response()
}

/// Mask API key for logging (show first 8 chars only)
fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        "****".to_string()
    } else {
        format!("{}****", &key[..8])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[test]
    fn test_extract_api_key_bearer() {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", "Bearer rs_1234567890abcdef".parse().unwrap());
        
        assert_eq!(
            extract_api_key(&headers),
            Some("rs_1234567890abcdef".to_string())
        );
    }

    #[test]
    fn test_extract_api_key_x_header() {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", "rs_1234567890abcdef".parse().unwrap());
        
        assert_eq!(
            extract_api_key(&headers),
            Some("rs_1234567890abcdef".to_string())
        );
    }

    #[test]
    fn test_extract_api_key_none() {
        let headers = HeaderMap::new();
        assert_eq!(extract_api_key(&headers), None);
    }

    #[test]
    fn test_mask_key() {
        assert_eq!(mask_key("rs_1234567890abcdef"), "rs_12345****");
        assert_eq!(mask_key("short"), "****");
    }
}
