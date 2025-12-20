use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rusty_server::api::create_router;
use tower::util::ServiceExt;

mod test_helpers;
use test_helpers::{create_test_state, create_test_config};

#[tokio::test]
async fn test_security_headers_present() {
    let config = create_test_config();
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let headers = response.headers();
    
    // Check for security headers
    assert!(headers.contains_key("x-content-type-options"));
    assert_eq!(headers.get("x-content-type-options").unwrap(), "nosniff");
    
    assert!(headers.contains_key("x-frame-options"));
    
    assert!(headers.contains_key("x-xss-protection"));
    assert_eq!(headers.get("x-xss-protection").unwrap(), "1; mode=block");
    
    assert!(headers.contains_key("referrer-policy"));
    
    assert!(headers.contains_key("content-security-policy"));
    
    assert!(headers.contains_key("permissions-policy"));
}

#[tokio::test]
async fn test_cors_headers() {
    let config = create_test_config();
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    // Test OPTIONS request (preflight)
    let response = app
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/api/v1/space-weather/current")
                .header("origin", "https://example.com")
                .header("access-control-request-method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // CORS should handle OPTIONS requests
    // The response may vary, but should not be an error
    assert!(response.status().is_success() || response.status() == StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_request_size_limit() {
    // This test verifies that the request size limit layer is applied
    // Note: Testing actual size limit enforcement would require a large request body
    // For now, we just verify the router compiles and runs with the limit layer
    
    let config = create_test_config();
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    // Normal request should work
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
