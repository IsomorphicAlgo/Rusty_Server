use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use rusty_server::api::create_router;
use rusty_server::auth::ApiKeyStore;
use tower::util::ServiceExt;

mod test_helpers;
use test_helpers::{create_test_state, create_test_config};

#[tokio::test]
async fn test_api_key_generation() {
    let store = ApiKeyStore::new();
    let api_key = store.generate_key(Some("test-key".to_string()), None).await;
    
    assert!(api_key.key.starts_with("rs_"));
    assert_eq!(api_key.name, Some("test-key".to_string()));
    assert!(api_key.is_active);
}

#[tokio::test]
async fn test_api_key_validation() {
    let store = ApiKeyStore::new();
    let api_key = store.generate_key(None, None).await;
    
    assert!(store.validate_key(&api_key.key).await);
    assert!(!store.validate_key("invalid_key").await);
}

#[tokio::test]
async fn test_api_key_with_auth_required() {
    // Create config with auth required
    let mut config = create_test_config();
    config.auth.require_auth = true;
    
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    // Request without API key should fail
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/current")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "Unauthorized");
}

#[tokio::test]
async fn test_api_key_with_valid_key() {
    // Create config with auth required
    let mut config = create_test_config();
    config.auth.require_auth = true;
    
    let state = create_test_state().await;
    
    // Generate an API key
    let api_key = state.api_key_store.generate_key(Some("test".to_string()), None).await;
    
    let app = create_router(state, config);
    
    // Request with valid API key should succeed
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/current")
                .header("x-api-key", api_key.key.as_str())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should succeed (may be 200 or 500 if database is down, but not 401)
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_api_key_with_bearer_token() {
    // Create config with auth required
    let mut config = create_test_config();
    config.auth.require_auth = true;
    
    let state = create_test_state().await;
    
    // Generate an API key
    let api_key = state.api_key_store.generate_key(None, None).await;
    
    let app = create_router(state, config);
    
    // Request with Bearer token should work
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/current")
                .header("authorization", format!("Bearer {}", api_key.key))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should succeed (may be 200 or 500 if database is down, but not 401)
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_api_key_with_auth_optional() {
    // Create config with auth optional (default)
    let config = create_test_config();
    // config.auth.require_auth is false by default
    
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    // Request without API key should succeed (auth is optional)
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/space-weather/current")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should succeed (not 401)
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_generate_api_key_endpoint() {
    let config = create_test_config();
    let state = create_test_state().await;
    let app = create_router(state, config);
    
    // Generate API key via endpoint
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/keys")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name": "test-key"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json["key"].is_string());
    assert!(json["key"].as_str().unwrap().starts_with("rs_"));
    assert_eq!(json["name"], "test-key");
}

#[tokio::test]
async fn test_list_api_keys_endpoint() {
    let config = create_test_config();
    let state = create_test_state().await;
    
    // Generate a few keys
    state.api_key_store.generate_key(Some("key1".to_string()), None).await;
    state.api_key_store.generate_key(Some("key2".to_string()), None).await;
    
    let app = create_router(state, config);
    
    // List API keys
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/keys")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    
    assert!(json.len() >= 2);
}


