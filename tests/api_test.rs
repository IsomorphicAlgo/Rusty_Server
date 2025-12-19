use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use rusty_server::api::create_router;
use rusty_server::{AppState, services::NoaaClient};
use tower::util::ServiceExt;

fn create_test_state() -> AppState {
    let noaa_client = NoaaClient::new(
        "https://services.swpc.noaa.gov".to_string(),
        None,
        30,
    );
    AppState::new(noaa_client)
}

#[tokio::test]
async fn test_current_conditions() {
    let app = create_router(create_test_state());

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

    assert!(json["data"].is_object());
    // Source can be "noaa" (if API call succeeds) or "mock" (if it falls back)
    let source = json["metadata"]["source"].as_str().unwrap();
    assert!(source == "noaa" || source == "mock");
}

#[tokio::test]
async fn test_historical_data() {
    let app = create_router(create_test_state());

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

    assert!(json.len() <= 5);
    assert!(!json.is_empty());
}

#[tokio::test]
async fn test_alerts() {
    let app = create_router(create_test_state());

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
}

#[tokio::test]
async fn test_radiation() {
    let app = create_router(create_test_state());

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

    assert!(json["data"]["radiation"].is_object());
    assert!(json["data"]["radiation"]["proton_flux"].is_number());
}

