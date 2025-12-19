use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use rusty_server::api::create_router;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_current_conditions() {
    let app = create_router();

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
    assert_eq!(json["metadata"]["source"], "mock");
}

#[tokio::test]
async fn test_historical_data() {
    let app = create_router();

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
    let app = create_router();

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
    let app = create_router();

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

