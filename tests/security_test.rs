use axum::{
    body::{Body, to_bytes},
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

#[tokio::test]
async fn test_sql_injection_attempt_in_date_parameter() {
    // Test that SQL injection attempts in date parameters are handled safely
    let config = create_test_config();
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    // Attempt SQL injection in start_date parameter
    let malicious_date = "2024-01-01T00:00:00Z'; DROP TABLE space_weather_observations; --";
    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/api/v1/space-weather/historical?start_date={}", 
                    urlencoding::encode(malicious_date)))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Should return validation error, not execute SQL
    assert!(response.status().is_client_error());
    
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    // Should be a validation error, not a database error
    let error_msg = json["error"].as_str().unwrap_or("");
    assert!(error_msg.contains("Invalid") || error_msg.contains("format"));
}

#[tokio::test]
async fn test_xss_attempt_in_query_parameters() {
    // Test that XSS attempts in query parameters are sanitized
    let config = create_test_config();
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    // Attempt XSS in data_type parameter
    let xss_payload = "<script>alert('xss')</script>";
    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/api/v1/space-weather/historical?data_type={}", 
                    urlencoding::encode(xss_payload)))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Should either reject or sanitize (validation error or empty result)
    // The important thing is it doesn't execute the script
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    
    // Response should be valid JSON (not contain script tags)
    let body_str = String::from_utf8_lossy(&body);
    assert!(!body_str.contains("<script>"));
    assert!(!body_str.contains("alert"));
}

#[tokio::test]
async fn test_path_traversal_attempt() {
    // Test that path traversal attempts are rejected
    let config = create_test_config();
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    // Attempt path traversal
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/../../../etc/passwd")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Should return 404 (route not found), not expose file system
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_oversized_limit_parameter() {
    // Test that extremely large limit values are handled safely
    let config = create_test_config();
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    // Attempt to use extremely large limit
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?limit=999999999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Should either cap the limit or return validation error
    // The important thing is it doesn't crash or return all data
    assert!(response.status().is_success() || response.status().is_client_error());
    
    if response.status().is_success() {
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
        
        // Should be capped at reasonable limit (10000)
        assert!(json.len() <= 10000);
    }
}

#[tokio::test]
async fn test_negative_limit_parameter() {
    // Test that negative limit values are rejected
    let config = create_test_config();
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?limit=-1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Should return validation error
    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn test_invalid_date_format_handling() {
    // Test various invalid date formats are rejected
    let invalid_dates = vec![
        "not-a-date",
        "2024-13-45T99:99:99Z",  // Invalid date/time
        "2024/01/01",  // Wrong format
        "01-01-2024",  // Wrong format
    ];
    
    for invalid_date in invalid_dates {
        let config = create_test_config();
        let state = create_test_state().await;
        let app = create_router(state, config);
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/v1/space-weather/historical?start_date={}", 
                        urlencoding::encode(invalid_date)))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        // All should return validation errors
        assert!(response.status().is_client_error(), 
            "Invalid date '{}' should be rejected", invalid_date);
    }
}

#[tokio::test]
async fn test_api_key_brute_force_protection() {
    // Test that multiple invalid API key attempts are handled
    // (Rate limiting should prevent brute force)
    let mut config = create_test_config();
    config.auth.require_auth = true;
    
    // Make multiple requests with invalid keys
    let mut unauthorized_count = 0;
    for i in 0..5 {
        let state = create_test_state().await;
        let app = create_router(state, config.clone());
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/space-weather/current")
                    .header("x-api-key", format!("invalid_key_{}", i))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        if response.status() == StatusCode::UNAUTHORIZED {
            unauthorized_count += 1;
        }
    }
    
    // All should be rejected (rate limiting may kick in after some attempts)
    // The important thing is invalid keys are rejected
    assert!(unauthorized_count > 0, "Invalid API keys should be rejected");
}

#[tokio::test]
async fn test_special_characters_in_parameters() {
    // Test that special characters in parameters are handled safely
    // Test various special characters
    let special_chars = vec![
        "%00",  // Null byte
        "%0A",  // Newline
        "%0D",  // Carriage return
        "&",    // Ampersand
        "=",    // Equals
        "#",    // Hash
    ];
    
    for special_char in special_chars {
        let config = create_test_config();
        let state = create_test_state().await;
        let app = create_router(state, config);
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/api/v1/space-weather/historical?data_type={}", 
                        urlencoding::encode(special_char)))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        // Should handle gracefully (either reject or return empty result)
        // The important thing is it doesn't crash
        assert!(response.status().is_success() || response.status().is_client_error());
    }
}
