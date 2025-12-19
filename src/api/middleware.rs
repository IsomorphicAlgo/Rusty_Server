use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tracing::{info, warn, error, Span};
use std::time::Instant;

/// Middleware for logging HTTP requests and responses
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();
    
    // Extract client IP if available
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .or_else(|| request.headers().get("x-real-ip"))
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Start timing
    let start = Instant::now();

    // Create a span for this request
    let span = tracing::span!(
        tracing::Level::INFO,
        "http_request",
        method = %method,
        uri = %uri,
        version = ?version,
        client_ip = %client_ip
    );
    
    let _enter = span.enter();
    info!("Request started");

    // Process request
    let response = next.run(request).await;

    // Calculate duration
    let duration = start.elapsed();
    let status = response.status();

    // Log response
    let status_code = status.as_u16();
    let log_level = if status_code >= 500 {
        tracing::Level::ERROR
    } else if status_code >= 400 {
        tracing::Level::WARN
    } else {
        tracing::Level::INFO
    };

    match log_level {
        tracing::Level::ERROR => {
            error!(
                method = %method,
                uri = %uri,
                status = %status_code,
                duration_ms = duration.as_millis(),
                "Request completed with error"
            );
        }
        tracing::Level::WARN => {
            warn!(
                method = %method,
                uri = %uri,
                status = %status_code,
                duration_ms = duration.as_millis(),
                "Request completed with client error"
            );
        }
        _ => {
            info!(
                method = %method,
                uri = %uri,
                status = %status_code,
                duration_ms = duration.as_millis(),
                "Request completed"
            );
        }
    }

    response
}

/// Middleware for error handling and logging
pub async fn error_handling_middleware(
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    
    // Log errors based on status code
    let status = response.status();
    if status.is_server_error() {
        error!(
            status = %status.as_u16(),
            uri = %request.uri(),
            "Server error occurred"
        );
    }

    response
}

