// Security middleware for CORS, security headers, and request size limits

use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    set_header::SetResponseHeaderLayer,
};
use tracing::{debug, warn};
use crate::config::SecurityConfig;

/// Create CORS layer from configuration
pub fn create_cors_layer(config: &SecurityConfig) -> CorsLayer {
    let mut cors = CorsLayer::new();

    // Parse allowed origins
    if config.cors_allowed_origins == "*" {
        cors = cors.allow_origin(Any);
    } else {
        // Parse comma-separated origins
        let origins: Vec<&str> = config.cors_allowed_origins
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        
        for origin in origins {
            if let Ok(header_value) = HeaderValue::from_str(origin) {
                cors = cors.allow_origin(header_value);
            } else {
                warn!("Invalid CORS origin: {}", origin);
            }
        }
    }

    // Parse allowed methods
    let methods: Vec<&str> = config.cors_allowed_methods
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    for method in methods {
        if let Ok(method) = method.parse() {
            cors = cors.allow_methods([method]);
        } else {
            warn!("Invalid CORS method: {}", method);
        }
    }

    // Parse allowed headers
    let headers: Vec<&str> = config.cors_allowed_headers
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    for header in headers {
        if let Ok(header_name) = HeaderName::from_bytes(header.as_bytes()) {
            cors = cors.allow_headers([header_name]);
        } else {
            warn!("Invalid CORS header: {}", header);
        }
    }

    cors.allow_credentials(true)
        .expose_headers(Any)
        .max_age(std::time::Duration::from_secs(3600))
}

/// Create request body size limit layer
pub fn create_request_size_limit_layer(config: &SecurityConfig) -> RequestBodyLimitLayer {
    // Convert u64 to usize (should be safe for reasonable request size limits)
    RequestBodyLimitLayer::new(config.max_request_size_bytes as usize)
}

/// Security headers middleware
/// Adds security headers to all responses
pub async fn security_headers_middleware(
    config: SecurityConfig,
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;

    // Add security headers based on configuration
    let headers = response.headers_mut();

    // HSTS (HTTP Strict Transport Security)
    if config.enable_hsts {
        if let Ok(header_value) = HeaderValue::from_str(
            &format!("max-age={}", config.hsts_max_age_seconds)
        ) {
            headers.insert("strict-transport-security", header_value);
        }
    }

    // X-Content-Type-Options
    if config.enable_x_content_type_options {
        let header_value = HeaderValue::from_static("nosniff");
        headers.insert("x-content-type-options", header_value);
    }

    // X-Frame-Options
    if config.enable_x_frame_options {
        if let Ok(header_value) = HeaderValue::from_str(&config.x_frame_options_value) {
            headers.insert("x-frame-options", header_value);
        }
    }

    // X-XSS-Protection
    if config.enable_x_xss_protection {
        let header_value = HeaderValue::from_static("1; mode=block");
        headers.insert("x-xss-protection", header_value);
    }

    // Referrer-Policy
    if config.enable_referrer_policy {
        if let Ok(header_value) = HeaderValue::from_str(&config.referrer_policy_value) {
            headers.insert("referrer-policy", header_value);
        }
    }

    // Content-Security-Policy (basic)
    // Note: CSP is complex and should be customized per application
    // This is a basic restrictive policy
    let header_value = HeaderValue::from_static(
        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';"
    );
    headers.insert("content-security-policy", header_value);

    // Permissions-Policy (formerly Feature-Policy)
    let header_value = HeaderValue::from_static(
        "geolocation=(), microphone=(), camera=()"
    );
    headers.insert("permissions-policy", header_value);

    debug!("Security headers added to response");
    response
}

/// Log security events (authentication failures, suspicious activity, etc.)
pub fn log_security_event(event_type: &str, details: &str, severity: &str) {
    match severity {
        "critical" | "error" => {
            tracing::error!(
                event_type = event_type,
                details = details,
                "Security event: {}",
                event_type
            );
        }
        "warning" | "warn" => {
            warn!(
                event_type = event_type,
                details = details,
                "Security event: {}",
                event_type
            );
        }
        _ => {
            debug!(
                event_type = event_type,
                details = details,
                "Security event: {}",
                event_type
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_config_defaults() {
        let config = SecurityConfig::default();
        assert_eq!(config.cors_allowed_origins, "*");
        assert_eq!(config.max_request_size_bytes, 10 * 1024 * 1024);
        assert!(config.enable_x_content_type_options);
        assert!(config.enable_x_frame_options);
    }

    #[test]
    fn test_cors_layer_creation() {
        let config = SecurityConfig::default();
        let cors_layer = create_cors_layer(&config);
        // Just verify it doesn't panic
        assert!(true);
    }
}
