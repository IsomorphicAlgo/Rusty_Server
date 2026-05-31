//! Integration tests for `/api/v1/ephemeris/*` (Ephemerust-backed).

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use rusty_server::api::create_router;
use std::sync::OnceLock;
use tower::util::ServiceExt;

mod test_helpers;
use test_helpers::{create_test_config, create_test_state_ephemeris};

static EPHEMERIS_STATE: OnceLock<rusty_server::AppState> = OnceLock::new();

fn ephemeris_state() -> rusty_server::AppState {
    EPHEMERIS_STATE
        .get_or_init(create_test_state_ephemeris)
        .clone()
}

fn ephemeris_app() -> axum::Router {
    create_router(ephemeris_state(), create_test_config())
}

/// ISS (ZARYA) sample TLE — same element set as Ephemerust `Tle::parse` doc test.
const SAMPLE_ISS_TLE: &str = r#"ISS (ZARYA)
1 25544U 98067A   20194.88612269 -.00002218  00000-0 -31515-4 0  9992
2 25544  51.6461 221.2784 0001413  89.1723 280.4612 15.49507896236008"#;

fn json_post(uri: &str, body: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
async fn ephemeris_time_j2000() {
    let app = ephemeris_app();
    let body = r#"{"utc":"2000-01-01T12:00:00Z"}"#;
    let res = app
        .oneshot(json_post("/api/v1/ephemeris/time", body))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!((v["julian_date"].as_f64().unwrap() - 2451545.0).abs() < 1e-4);
    assert!(v["gmst_hours"].is_f64());
    assert!(v["lst_hours"].is_null());
}

#[tokio::test]
async fn ephemeris_time_invalid_utc() {
    let app = ephemeris_app();
    let res = app
        .oneshot(json_post(
            "/api/v1/ephemeris/time",
            r#"{"utc":"not-a-date"}"#,
        ))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn ephemeris_position_jupiter() {
    let app = ephemeris_app();
    let body = r#"{"object":"jupiter","utc":"2000-01-01T12:00:00Z"}"#;
    let res = app
        .oneshot(json_post("/api/v1/ephemeris/position", body))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["object"], "jupiter");
    assert!(v["ra_hours"].is_f64());
    assert!(v["dec_deg"].is_f64());
}

#[tokio::test]
async fn ephemeris_position_unknown_object() {
    let app = ephemeris_app();
    let res = app
        .oneshot(json_post(
            "/api/v1/ephemeris/position",
            r#"{"object":"pluto","utc":"2000-01-01T12:00:00Z"}"#,
        ))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn ephemeris_track_state() {
    let app = ephemeris_app();
    let payload = format!(
        r#"{{"tle":{},"utc":"2020-07-12T12:00:00Z","mode":"state"}}"#,
        serde_json::to_string(SAMPLE_ISS_TLE).unwrap()
    );
    let res = app
        .oneshot(json_post("/api/v1/ephemeris/satellite/track", &payload))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK, "track state failed");
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["mode"], "state");
    assert_eq!(v["catalog_number"], 25544);
    assert!(v["teme"]["position_km"].is_array());
}

#[tokio::test]
async fn ephemeris_track_subpoint() {
    let app = ephemeris_app();
    let payload = format!(
        r#"{{"tle":{},"utc":"2020-07-12T12:00:00Z","mode":"subpoint"}}"#,
        serde_json::to_string(SAMPLE_ISS_TLE).unwrap()
    );
    let res = app
        .oneshot(json_post("/api/v1/ephemeris/satellite/track", &payload))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["mode"], "subpoint");
    assert!(v["geodetic"]["latitude_deg"].is_f64());
}

#[tokio::test]
async fn ephemeris_track_look() {
    let app = ephemeris_app();
    let payload = format!(
        r#"{{"tle":{},"utc":"2020-07-12T12:00:00Z","mode":"look",
            "observer":{{"latitude_deg":47.9088,"longitude_deg":-122.2503,"elevation_m":0}}}}"#,
        serde_json::to_string(SAMPLE_ISS_TLE).unwrap()
    );
    let res = app
        .oneshot(json_post("/api/v1/ephemeris/satellite/track", &payload))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["mode"], "look");
    assert!(v["look"]["elevation_deg"].is_f64());
}

#[tokio::test]
async fn ephemeris_track_passes() {
    let app = ephemeris_app();
    let payload = format!(
        r#"{{"tle":{},"utc":"2020-07-12T12:00:00Z","mode":"passes",
            "predict_passes_hours":6,
            "observer":{{"latitude_deg":47.9088,"longitude_deg":-122.2503,"elevation_m":0}}}}"#,
        serde_json::to_string(SAMPLE_ISS_TLE).unwrap()
    );
    let res = app
        .oneshot(json_post("/api/v1/ephemeris/satellite/track", &payload))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["mode"], "passes");
    assert!(v["passes"].is_array());
}

#[tokio::test]
async fn ephemeris_track_ground() {
    let app = ephemeris_app();
    let payload = format!(
        r#"{{"tle":{},"utc":"2020-07-12T12:00:00Z","mode":"ground","ground_track_hours":2}}"#,
        serde_json::to_string(SAMPLE_ISS_TLE).unwrap()
    );
    let res = app
        .oneshot(json_post("/api/v1/ephemeris/satellite/track", &payload))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["mode"], "ground");
    let samples = v["samples"].as_array().unwrap();
    assert!(!samples.is_empty());
}

#[tokio::test]
async fn ephemeris_track_missing_observer_for_look() {
    let app = ephemeris_app();
    let payload = format!(
        r#"{{"tle":{},"utc":"2020-07-12T12:00:00Z","mode":"look"}}"#,
        serde_json::to_string(SAMPLE_ISS_TLE).unwrap()
    );
    let res = app
        .oneshot(json_post("/api/v1/ephemeris/satellite/track", &payload))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
