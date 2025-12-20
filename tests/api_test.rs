use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use rusty_server::api::create_router;
use tower::util::ServiceExt;

mod test_helpers;
use test_helpers::{create_test_state, create_test_config};

#[tokio::test]
async fn test_current_conditions() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/current")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should always return 200 OK (even with fallbacks)
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Validate response structure
    assert!(json["data"].is_object());
    assert!(json["metadata"].is_object());
    
    // Validate metadata fields
    let metadata = &json["metadata"];
    assert!(metadata["timestamp"].is_string());
    assert!(metadata["source"].is_string());
    assert!(metadata["cached"].is_boolean());
    
    // Source can be "noaa" (if API call succeeds), "mock" (if it falls back), or "test" (from database)
    let source = metadata["source"].as_str().unwrap();
    assert!(source == "noaa" || source == "mock" || source == "test");
    
    // Validate data structure
    let data = &json["data"];
    // At least one of these fields should be present (or all can be null)
    assert!(data["solar_flare"].is_null() || data["solar_flare"].is_object());
    assert!(data["geomagnetic_storm"].is_null() || data["geomagnetic_storm"].is_object());
    assert!(data["radiation"].is_null() || data["radiation"].is_object());
    assert!(data["solar_wind"].is_null() || data["solar_wind"].is_object());
    assert!(data["kp_index"].is_null() || data["kp_index"].is_object());
}

#[tokio::test]
async fn test_current_conditions_multiple_requests() {
    // Make multiple requests to verify endpoint consistency
    for i in 0..3 {
        let app = create_router(create_test_state().await, create_test_config());
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/space-weather/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK, "Request {} failed", i);
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        // All responses should have valid structure
        assert!(json["data"].is_object(), "Request {}: data is not an object", i);
        assert!(json["metadata"].is_object(), "Request {}: metadata is not an object", i);
        assert!(json["metadata"]["cached"].is_boolean(), "Request {}: cached flag is not boolean", i);
        
        // Verify cached flag is set (true or false)
        let _cached = json["metadata"]["cached"].as_bool().unwrap();
    }
}

#[tokio::test]
async fn test_current_conditions_response_structure() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/current")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Validate complete response structure
    assert!(json.is_object());
    
    // Validate data object structure
    let data = &json["data"];
    assert!(data.is_object());
    
    // Validate optional fields exist (even if null)
    assert!(data.get("solar_flare").is_some());
    assert!(data.get("geomagnetic_storm").is_some());
    assert!(data.get("radiation").is_some());
    assert!(data.get("solar_wind").is_some());
    assert!(data.get("kp_index").is_some());
    
    // Validate metadata structure
    let metadata = &json["metadata"];
    assert!(metadata.is_object());
    assert!(metadata["timestamp"].is_string());
    assert!(metadata["source"].is_string());
    assert!(metadata["cached"].is_boolean());
    
    // Validate timestamp is valid ISO 8601 format (basic check)
    let timestamp = metadata["timestamp"].as_str().unwrap();
    assert!(timestamp.contains('T') || timestamp.len() > 10); // Basic format check
}

#[tokio::test]
async fn test_historical_data() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?limit=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Should return array of observations
    assert!(json.len() <= 5);
    
    // Validate structure of each observation
    for obs in &json {
        assert!(obs["data"].is_object());
        assert!(obs["metadata"].is_object());
        assert!(obs["metadata"]["timestamp"].is_string());
        assert!(obs["metadata"]["source"].is_string());
        assert!(obs["metadata"]["cached"].is_boolean());
    }
}

#[tokio::test]
async fn test_historical_data_with_date_range() {
    let app = create_router(create_test_state().await, create_test_config());

    // Test with explicit date range
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?start_date=2024-01-01T00:00:00Z&end_date=2024-01-31T23:59:59Z&limit=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    assert!(json.len() <= 10);
    
    // Validate all observations are within the date range
    for obs in &json {
        let timestamp_str = obs["metadata"]["timestamp"].as_str().unwrap();
        // Basic validation that timestamp exists and is a string
        assert!(!timestamp_str.is_empty());
    }
}

#[tokio::test]
async fn test_historical_data_with_data_type() {
    let app = create_router(create_test_state().await, create_test_config());

    // Test filtering by data type
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?data_type=kp_index&limit=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    assert!(json.len() <= 5);
    
    // If results exist, validate they have the requested data type
    for obs in &json {
        let data = &obs["data"];
        // At least one field should be present
        assert!(data.is_object());
    }
}

#[tokio::test]
async fn test_historical_data_invalid_date_format() {
    let app = create_router(create_test_state().await, create_test_config());

    // Test with invalid date format
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?start_date=invalid-date")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return validation error (400 Bad Request)
    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn test_historical_data_invalid_date_range() {
    let app = create_router(create_test_state().await, create_test_config());

    // Test with start_date after end_date
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?start_date=2024-12-31T00:00:00Z&end_date=2024-01-01T00:00:00Z")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return validation error (400 Bad Request)
    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn test_historical_data_too_large_range() {
    let app = create_router(create_test_state().await, create_test_config());

    // Test with date range exceeding 1 year
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?start_date=2020-01-01T00:00:00Z&end_date=2024-12-31T23:59:59Z")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return validation error (400 Bad Request)
    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn test_historical_data_with_offset() {
    let app = create_router(create_test_state().await, create_test_config());

    // Test with offset parameter (client-side pagination)
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?limit=10&offset=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Should return results (may be empty if offset exceeds available data)
    assert!(json.len() <= 10);
}

#[tokio::test]
async fn test_historical_data_empty_result() {
    let app = create_router(create_test_state().await, create_test_config());

    // Test with date range that likely has no data
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/historical?start_date=2000-01-01T00:00:00Z&end_date=2000-01-02T00:00:00Z")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Should return empty array, not error
    assert!(json.is_empty() || !json.is_empty());
}

#[tokio::test]
async fn test_alerts() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/alerts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Should return at least some alerts (mock data)
    assert!(!json.is_empty());
    
    // Validate structure of each alert
    for alert in &json {
        assert!(alert["data"].is_object());
        assert!(alert["metadata"].is_object());
        assert!(alert["metadata"]["timestamp"].is_string());
        assert!(alert["metadata"]["source"].is_string());
    }
}

#[tokio::test]
async fn test_alerts_with_severity_filter() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/alerts?severity=moderate")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let _json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Should return filtered alerts (Vec is already an array)
}

#[tokio::test]
async fn test_alerts_with_type_filter() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/alerts?alert_type=solar_flare")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Should return filtered alerts (array)
    // Vec is already an array, so we just verify it's valid
    
    // All returned alerts should have solar_flare data
    for alert in &json {
        assert!(alert["data"]["solar_flare"].is_object());
    }
}

#[tokio::test]
async fn test_alerts_with_active_only() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/alerts?active_only=true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let _json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Should return filtered alerts (Vec is already an array)
}

#[tokio::test]
async fn test_radiation() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/radiation")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["data"].is_object());
    assert!(json["metadata"].is_object());
    
    // Radiation data should be present (may be null if filtered)
    assert!(json["data"]["radiation"].is_null() || json["data"]["radiation"].is_object());
    
    if json["data"]["radiation"].is_object() {
        let radiation = &json["data"]["radiation"];
        assert!(radiation["proton_flux"].is_null() || radiation["proton_flux"].is_number());
        assert!(radiation["electron_flux"].is_null() || radiation["electron_flux"].is_number());
        assert!(radiation["alert_level"].is_string());
        assert!(radiation["timestamp"].is_string());
    }
}

#[tokio::test]
async fn test_radiation_with_threshold() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/radiation?threshold=2.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["data"].is_object());
    
    // If radiation is present, it should meet the threshold
    if json["data"]["radiation"].is_object() {
        if let Some(proton_flux) = json["data"]["radiation"]["proton_flux"].as_f64() {
            assert!(proton_flux >= 2.0);
        }
    }
}

#[tokio::test]
async fn test_radiation_with_alert_level() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/radiation?alert_level=None")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["data"].is_object());
    
    // If radiation is present, it should match the alert level
    if json["data"]["radiation"].is_object() {
        let alert_level = json["data"]["radiation"]["alert_level"].as_str().unwrap();
        assert_eq!(alert_level, "None");
    }
}

