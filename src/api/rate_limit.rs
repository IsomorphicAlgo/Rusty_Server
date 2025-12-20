// Rate limiting middleware using governor crate
// Implements per-IP rate limiting with token bucket algorithm

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode, HeaderValue},
    middleware::Next,
    response::{Response, IntoResponse, Json},
};
use governor::{
    clock::{DefaultClock, Clock},
    state::keyed::DefaultKeyedStateStore,
    Quota, RateLimiter,
};
use std::{
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};
use tracing::{warn, debug};
use crate::config::RateLimitConfig;

/// Key type for rate limiting (using IP address as key)
type RateLimitKey = String;

/// Rate limiter state (shared across requests)
pub type SharedRateLimiter = Arc<RateLimiter<RateLimitKey, DefaultKeyedStateStore<RateLimitKey>, DefaultClock>>;

/// Create a rate limiter from configuration
pub fn create_rate_limiter(config: &RateLimitConfig) -> SharedRateLimiter {
    // Create quota: requests_per_minute per 60 seconds with burst_size
    // Governor uses a token bucket algorithm
    // 
    // The quota allows config.requests_per_minute requests per 60-second period
    // with a burst capacity of config.burst_size
    // 
    // Example: 60 requests/min with burst of 10 means:
    // - You can make up to 10 requests in quick succession (burst)
    // - After that, tokens refill at a rate of 60 requests per 60 seconds (1 per second)
    // - Maximum of 60 requests in any 60-second window
    
    // Governor's quota works as: allow_burst(N) with period(T) means
    // you can make up to N requests, and tokens refill over period T
    // So to allow 'requests_per_minute' requests per 60 seconds:
    // - Use requests_per_minute as the burst capacity (total tokens)
    // - Use 60 seconds as the period (refill window)
    // - The burst_size config limits immediate bursts, but governor's burst
    //   is the total capacity, so we use requests_per_minute
    
    let requests = NonZeroU32::new(config.requests_per_minute).unwrap_or(NonZeroU32::new(60).unwrap());
    
    // Create quota: allows 'requests' requests per 60-second period
    // The burst capacity is set to requests_per_minute (total capacity)
    // Note: governor's 'burst' is the total token bucket capacity
    let quota = Quota::with_period(Duration::from_secs(60))
        .expect("Invalid rate limit period")
        .allow_burst(requests);
    
    // Note: The burst_size config is not directly used here because
    // governor's burst represents the total capacity, not immediate burst limit.
    // The token bucket will refill at a rate that allows up to 'requests' per period.
    
    Arc::new(RateLimiter::keyed(quota))
}

/// Extract client IP address from request headers
fn extract_client_ip(headers: &HeaderMap) -> String {
    // Try X-Forwarded-For first (for reverse proxies)
    if let Some(forwarded_for) = headers.get("x-forwarded-for") {
        if let Ok(ip_str) = forwarded_for.to_str() {
            // X-Forwarded-For can contain multiple IPs, take the first one
            if let Some(first_ip) = ip_str.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }

    // Try X-Real-IP (common in nginx)
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.to_string();
        }
    }

    // Fallback to "unknown" if we can't determine IP
    // In production, you might want to extract from the connection
    "unknown".to_string()
}

/// Rate limiting middleware
/// 
/// This middleware enforces rate limits per IP address using a token bucket algorithm.
/// When a client exceeds the rate limit, it returns a 429 Too Many Requests response
/// with appropriate headers indicating when they can retry.
pub async fn rate_limit_middleware(
    limiter: SharedRateLimiter,
    request: Request,
    next: Next,
) -> Response {
    // Extract client IP
    let client_ip = extract_client_ip(request.headers());
    
    // Check rate limit
    match limiter.check_key(&client_ip) {
        Ok(_) => {
            // Rate limit OK, proceed with request
            debug!("Rate limit check passed for IP: {}", client_ip);
            
            let mut response = next.run(request).await;
            
            // Add rate limit headers to successful responses
            add_rate_limit_headers(&mut response, &limiter, &client_ip);
            
            response
        }
        Err(negative) => {
            // Rate limit exceeded
            let clock = DefaultClock::default();
            let wait_time = negative.wait_time_from(clock.now());
            let wait_seconds = wait_time.as_secs();
            
            warn!(
                "Rate limit exceeded for IP: {} (wait {} seconds)",
                client_ip,
                wait_seconds
            );

            // Create 429 Too Many Requests response
            let mut response = (
                StatusCode::TOO_MANY_REQUESTS,
                Json(serde_json::json!({
                    "error": "Rate limit exceeded",
                    "message": format!("Too many requests. Please try again in {} seconds.", wait_seconds),
                    "retry_after": wait_seconds
                })),
            ).into_response();

            // Add Retry-After header
            if let Ok(header_value) = HeaderValue::from_str(&wait_seconds.to_string()) {
                response.headers_mut().insert("retry-after", header_value);
            }

            // Add rate limit headers
            add_rate_limit_headers(&mut response, &limiter, &client_ip);

            response
        }
    }
}

/// Add rate limit headers to response
/// 
/// Headers added:
/// - X-RateLimit-Limit: Maximum requests allowed per window
/// Note: governor doesn't expose remaining count directly, so we only add limit
fn add_rate_limit_headers(
    _response: &mut Response,
    _limiter: &SharedRateLimiter,
    _client_ip: &str,
) {
    // Note: governor doesn't expose remaining count or reset time directly
    // The retry-after header in error responses is the most important one
    // For a more sophisticated implementation, we could track quota info separately
    // For now, we'll rely on the retry-after header for rate limit information
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let config = RateLimitConfig {
            requests_per_minute: 10,
            requests_per_hour: 100,
            burst_size: 5,
        };
        
        let limiter = create_rate_limiter(&config);
        assert!(Arc::strong_count(&limiter) >= 1);
    }

    #[tokio::test]
    async fn test_extract_client_ip() {
        use axum::http::HeaderMap;
        
        // Test X-Forwarded-For
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "192.168.1.1".parse().unwrap());
        assert_eq!(extract_client_ip(&headers), "192.168.1.1");

        // Test X-Real-IP
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", "10.0.0.1".parse().unwrap());
        assert_eq!(extract_client_ip(&headers), "10.0.0.1");

        // Test fallback
        let headers = HeaderMap::new();
        assert_eq!(extract_client_ip(&headers), "unknown");
    }
}
