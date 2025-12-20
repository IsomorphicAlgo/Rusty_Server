use rusty_server::services::{DonkiClient, parsing::parse_donki_flare};
use serde_json::json;
use chrono::{Utc, Duration as ChronoDuration};

#[tokio::test]
async fn test_donki_client_creation() {
    let client = DonkiClient::new(
        "https://api.nasa.gov/DONKI".to_string(),
        Some("test-key".to_string()),
        30,
    );
    
    // Just verify it doesn't panic
    assert!(true);
}

#[tokio::test]
async fn test_donki_client_no_api_key() {
    let client = DonkiClient::new(
        "https://api.nasa.gov/DONKI".to_string(),
        None,
        30,
    );
    
    // Should fail when trying to fetch without API key
    let start_date = Utc::now() - ChronoDuration::days(7);
    let result = client.fetch_solar_flares(start_date, None).await;
    
    // Should return error about missing API key
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(format!("{}", error).contains("API key") || format!("{}", error).contains("required"));
}

#[test]
fn test_parse_donki_flare_complete() {
    let json = json!({
        "flrID": "2024-12-19T12:00:00-FLR-001",
        "beginTime": "2024-12-19T12:00:00Z",
        "peakTime": "2024-12-19T12:05:00Z",
        "endTime": "2024-12-19T12:10:00Z",
        "classType": "M2.5",
        "sourceLocation": "N15W20",
        "activeRegionNum": "12345"
    });
    
    let result = parse_donki_flare(&json).unwrap();
    assert!(result.is_some());
    let flare = result.unwrap();
    
    assert_eq!(flare.class, "M2.5");
    assert_eq!(flare.source_location, Some("N15W20 AR 12345".to_string()));
    assert!(flare.begin_time.is_some());
    assert!(flare.end_time.is_some());
    assert!(flare.peak_time.to_string().contains("2024-12-19"));
}

#[test]
fn test_parse_donki_flare_minimal() {
    let json = json!({
        "flrID": "2024-12-19T12:00:00-FLR-001",
        "peakTime": "2024-12-19T12:05:00Z",
        "classType": "C1.0"
    });
    
    let result = parse_donki_flare(&json).unwrap();
    assert!(result.is_some());
    let flare = result.unwrap();
    
    assert_eq!(flare.class, "C1.0");
    assert_eq!(flare.source_location, None);
    assert_eq!(flare.begin_time, None);
    assert_eq!(flare.end_time, None);
}

#[test]
fn test_parse_donki_flare_missing_class() {
    let json = json!({
        "flrID": "2024-12-19T12:00:00-FLR-001",
        "peakTime": "2024-12-19T12:05:00Z"
    });
    
    let result = parse_donki_flare(&json).unwrap();
    assert!(result.is_none()); // Should return None when required fields are missing
}

#[test]
fn test_parse_donki_flare_missing_peak_time() {
    let json = json!({
        "flrID": "2024-12-19T12:00:00-FLR-001",
        "classType": "C1.0"
    });
    
    let result = parse_donki_flare(&json).unwrap();
    assert!(result.is_none()); // Should return None when peakTime is missing
}

#[test]
fn test_parse_donki_flare_source_location_only() {
    let json = json!({
        "flrID": "2024-12-19T12:00:00-FLR-001",
        "peakTime": "2024-12-19T12:05:00Z",
        "classType": "C1.0",
        "sourceLocation": "N10W10"
    });
    
    let result = parse_donki_flare(&json).unwrap();
    assert!(result.is_some());
    let flare = result.unwrap();
    assert_eq!(flare.source_location, Some("N10W10".to_string()));
}

#[test]
fn test_parse_donki_flare_active_region_only() {
    let json = json!({
        "flrID": "2024-12-19T12:00:00-FLR-001",
        "peakTime": "2024-12-19T12:05:00Z",
        "classType": "C1.0",
        "activeRegionNum": "67890"
    });
    
    let result = parse_donki_flare(&json).unwrap();
    assert!(result.is_some());
    let flare = result.unwrap();
    assert_eq!(flare.source_location, Some("AR 67890".to_string()));
}

#[test]
fn test_parse_donki_flare_x_class() {
    let json = json!({
        "flrID": "2024-12-19T12:00:00-FLR-001",
        "peakTime": "2024-12-19T12:05:00Z",
        "classType": "X5.2",
        "sourceLocation": "N20E30",
        "activeRegionNum": "99999"
    });
    
    let result = parse_donki_flare(&json).unwrap();
    assert!(result.is_some());
    let flare = result.unwrap();
    assert_eq!(flare.class, "X5.2");
    assert_eq!(flare.source_location, Some("N20E30 AR 99999".to_string()));
}
