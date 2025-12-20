use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use rusty_server::api::create_router;
use tower::util::ServiceExt;

mod test_helpers;
use test_helpers::create_test_state;

#[tokio::test]
async fn test_health_check() {
    let app = create_router(create_test_state().await, create_test_config());

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

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "healthy");
    assert!(json["timestamp"].is_number());
    assert_eq!(json["service"], "rusty-server");
}

#[tokio::test]
async fn test_health_check_api_v1() {
    let app = create_router(create_test_state().await, create_test_config());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "healthy");
}

