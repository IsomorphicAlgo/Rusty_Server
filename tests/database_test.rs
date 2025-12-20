use rusty_server::database::{DatabasePool, DatabaseOperations};
use rusty_server::models::*;
use chrono::{Utc, Duration as ChronoDuration};

mod test_helpers;
use test_helpers::get_test_db_connection_string;

#[tokio::test]
async fn test_store_and_retrieve_observation() {
    let connection_string = get_test_db_connection_string();
    let db_pool = DatabasePool::new(&connection_string).await.expect("Failed to connect to test database");
    let db_ops = DatabaseOperations::new(db_pool.pool().clone());
    
    // Create test data
    let test_data = SpaceWeatherData {
        kp_index: Some(KpIndex {
            value: 3.5,
            level: "Active".to_string(),
            timestamp: Utc::now(),
        }),
        geomagnetic_storm: None,
        solar_wind: Some(SolarWind {
            speed: 450.0,
            density: 3.5,
            temperature: 50000.0,
            bz: Some(-2.5),
            timestamp: Utc::now(),
        }),
        solar_flare: None,
        radiation: None,
    };
    
    let metadata = ResponseMetadata {
        timestamp: Utc::now(),
        source: "test".to_string(),
        cached: false,
    };
    
    // Store observation
    let id = db_ops.store_observation(&test_data, &metadata).await.unwrap();
    assert!(id > 0);
    
    // Retrieve latest observation
    let latest = db_ops.get_latest_observation().await.unwrap();
    assert!(latest.is_some());
    let response = latest.unwrap();
    assert_eq!(response.data.kp_index.as_ref().unwrap().value, 3.5);
}

#[tokio::test]
async fn test_get_observations_by_date_range() {
    let connection_string = get_test_db_connection_string();
    let db_pool = DatabasePool::new(&connection_string).await.expect("Failed to connect to test database");
    let db_ops = DatabaseOperations::new(db_pool.pool().clone());
    
    let end_time = Utc::now();
    let start_time = end_time - ChronoDuration::days(7);
    
    let observations = db_ops.get_observations(start_time, end_time, Some(10)).await.unwrap();
    
    // Should return observations (if any exist)
    assert!(observations.len() <= 10);
}

#[tokio::test]
async fn test_get_observations_by_type() {
    let connection_string = get_test_db_connection_string();
    let db_pool = DatabasePool::new(&connection_string).await.expect("Failed to connect to test database");
    let db_ops = DatabaseOperations::new(db_pool.pool().clone());
    
    let end_time = Utc::now();
    let start_time = end_time - ChronoDuration::days(7);
    
    let observations = db_ops.get_observations_by_type(
        start_time,
        end_time,
        "kp_index",
        Some(10),
    ).await.unwrap();
    
    // All returned observations should have KP index data
    for obs in &observations {
        assert!(obs.data.kp_index.is_some());
    }
}

#[tokio::test]
async fn test_store_observations_batch() {
    let connection_string = get_test_db_connection_string();
    let db_pool = DatabasePool::new(&connection_string).await.expect("Failed to connect to test database");
    let db_ops = DatabaseOperations::new(db_pool.pool().clone());
    
    // Create multiple test observations
    let mut batch = Vec::new();
    for i in 0..5 {
        let data = SpaceWeatherData {
            kp_index: Some(KpIndex {
                value: 2.0 + (i as f64 * 0.1),
                level: "Quiet".to_string(),
                timestamp: Utc::now() - ChronoDuration::hours(i as i64),
            }),
            geomagnetic_storm: None,
            solar_wind: None,
            solar_flare: None,
            radiation: None,
        };
        
        let metadata = ResponseMetadata {
            timestamp: Utc::now() - ChronoDuration::hours(i as i64),
            source: "test".to_string(),
            cached: false,
        };
        
        batch.push((data, metadata));
    }
    
    let ids = db_ops.store_observations_batch(&batch).await.unwrap();
    assert_eq!(ids.len(), 5);
}

#[tokio::test]
async fn test_get_observation_count() {
    let connection_string = get_test_db_connection_string();
    let db_pool = DatabasePool::new(&connection_string).await.expect("Failed to connect to test database");
    let db_ops = DatabaseOperations::new(db_pool.pool().clone());
    
    let count = db_ops.get_observation_count(None, None).await.unwrap();
    assert!(count >= 0);
    
    let end_time = Utc::now();
    let start_time = end_time - ChronoDuration::days(7);
    let range_count = db_ops.get_observation_count(Some(start_time), Some(end_time)).await.unwrap();
    assert!(range_count <= count);
}

