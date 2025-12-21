/// Data Collection Script for ML Model Training
/// 
/// This script collects historical solar flare and space weather data
/// for training ML prediction models.
/// 
/// Usage:
///   cargo run --bin collect_training_data                    # Default: 5 years (60 months)
///   cargo run --bin collect_training_data -- --months 60    # Explicitly 5 years
///   cargo run --bin collect_training_data -- --months 12    # 1 year
///   cargo run --bin collect_training_data -- --start-date 2019-01-01 --end-date 2024-01-01

use rusty_server::config::Config;
use rusty_server::Result;
use rusty_server::services::{DonkiClient, NoaaClient};
use rusty_server::database::DatabasePool;
use rusty_server::database::DatabaseOperations;
use rusty_server::models::*;
use chrono::{DateTime, Utc, Duration as ChronoDuration, NaiveDate};
use std::env;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let months = parse_months_arg(&args).unwrap_or(60); // Default: 60 months (5 years)
    let (start_date, end_date) = parse_date_args(&args).unwrap_or_else(|| {
        let end = Utc::now();
        let start = end - ChronoDuration::days((months * 30) as i64);
        (start, end)
    });

    info!("Starting data collection for ML training");
    info!("Date range: {} to {}", start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d"));
    
    let days_span = (end_date - start_date).num_days();
    let estimated_flares = (days_span as f64 * 0.5) as usize; // Rough estimate: ~0.5 flares per day
    info!("Estimated collection time: {} days, ~{} solar flares expected", days_span, estimated_flares);
    if days_span > 365 {
        info!("Large dataset - this may take 10-30 minutes. Progress will be shown every 25 flares.");
    }

    // Load configuration
    let config = Config::load().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        rusty_server::AppError::Config(e)
    })?;

    // Initialize clients
    let donki_client = DonkiClient::new(
        config.donki.base_url.clone(),
        config.donki.api_key.clone(),
        config.donki.timeout_seconds,
    );

    if config.donki.api_key.is_none() {
        error!("DONKI API key not configured! Set RUSTY_SERVER__DONKI__API_KEY");
        return Err(rusty_server::AppError::Config(
            config::ConfigError::Message("DONKI API key required for data collection".to_string())
        ));
    }

    let noaa_client = NoaaClient::new(
        config.noaa.base_url.clone(),
        config.noaa.api_key.clone(),
        config.noaa.timeout_seconds,
    );

    // Initialize database
    let db_pool = DatabasePool::new(&config.database.connection_string).await?;
    db_pool.migrate().await?;
    let db_ops = DatabaseOperations::new(db_pool.pool().clone());

    info!("Database connected and migrations complete");

    // Step 1: Collect solar flares from DONKI
    // NOTE: This makes a SINGLE API call for the entire date range, which is very efficient
    // and well within NASA's rate limits (1,000 requests/hour with registered API key)
    info!("Step 1: Collecting solar flares from DONKI...");
    info!("Making a single API call for date range (respects NASA rate limits)...");
    
    // Small delay before API call to be extra respectful
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    let flares = match donki_client.fetch_solar_flares(start_date, Some(end_date)).await {
        Ok(flares) => {
            info!("Collected {} solar flares from DONKI", flares.len());
            flares
        }
        Err(e) => {
            error!("Failed to fetch solar flares from DONKI: {}", e);
            error!("This may be due to rate limiting. Please wait and try again later.");
            return Err(e);
        }
    };

    // Step 2: Store flares with minimal space weather context
    // Note: NOAA API doesn't provide historical data easily, so we'll store flares
    // with basic context. The ML model can work with flare data + any available
    // space weather data that gets collected over time.
    info!("Step 2: Storing solar flares in database...");
    let mut flares_stored = 0;
    let mut flares_skipped = 0;

    for flare in &flares {
        // Create a minimal space weather observation focused on the flare
        let flare_time = flare.peak_time;
        
        let space_weather_data = SpaceWeatherData {
            solar_flare: Some(flare.clone()),
            geomagnetic_storm: None,
            radiation: None,
            solar_wind: None,
            kp_index: None,
        };

        let metadata = ResponseMetadata {
            timestamp: flare_time,
            source: "donki".to_string(),
            cached: false,
        };

        match db_ops.store_observation(&space_weather_data, &metadata).await {
            Ok(_) => {
                flares_stored += 1;
                // Show progress more frequently for large datasets
                let progress_interval = if flares.len() > 500 { 25 } else { 10 };
                if flares_stored % progress_interval == 0 || flares_stored == flares.len() {
                    let percent = (flares_stored as f64 / flares.len() as f64 * 100.0) as u32;
                    info!("Progress: {}/{} flares stored ({}%)", flares_stored, flares.len(), percent);
                }
            }
            Err(e) => {
                warn!("Failed to store flare at {}: {}", flare_time, e);
                flares_skipped += 1;
            }
        }

        // Small delay to avoid overwhelming the database
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    info!("Note: Historical space weather conditions are not easily available via NOAA API.");
    info!("The flares are stored and can be used for training. Space weather context");
    info!("will be added as current data is collected over time.");

    // Summary
    let days_span = (end_date - start_date).num_days();
    info!("{}", "=".repeat(60));
    info!("Data Collection Complete!");
    info!("{}", "=".repeat(60));
    info!("Date range: {} to {}", start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d"));
    info!("Time span: {} days ({:.1} years)", days_span, days_span as f64 / 365.25);
    info!("Solar flares collected: {}", flares.len());
    info!("Flares stored: {}", flares_stored);
    info!("Flares skipped: {}", flares_skipped);
    if flares_stored > 0 {
        let avg_per_month = (flares_stored as f64 / (days_span as f64 / 30.0)) as u32;
        info!("Average: ~{} flares per month", avg_per_month);
    }
    info!("{}", "=".repeat(60));
    info!("");
    info!("Next steps:");
    info!("1. Verify data in database: SELECT COUNT(*) FROM space_weather_observations;");
    info!("2. Check for solar flares: SELECT COUNT(*) FROM space_weather_observations WHERE solar_flare_class IS NOT NULL;");
    info!("3. Export database when ready: mysqldump -u rusty_user -p rusty_server_test > training_data.sql");

    Ok(())
}

fn parse_months_arg(args: &[String]) -> Option<u32> {
    for (i, arg) in args.iter().enumerate() {
        if arg == "--months" && i + 1 < args.len() {
            return args[i + 1].parse().ok();
        }
    }
    None
}

fn parse_date_args(args: &[String]) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let mut start_date = None;
    let mut end_date = None;

    for (i, arg) in args.iter().enumerate() {
        if arg == "--start-date" && i + 1 < args.len() {
            if let Ok(date) = NaiveDate::parse_from_str(&args[i + 1], "%Y-%m-%d") {
                start_date = date.and_hms_opt(0, 0, 0)
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc));
            }
        } else if arg == "--end-date" && i + 1 < args.len() {
            if let Ok(date) = NaiveDate::parse_from_str(&args[i + 1], "%Y-%m-%d") {
                end_date = date.and_hms_opt(23, 59, 59)
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc));
            }
        }
    }

    match (start_date, end_date) {
        (Some(start), Some(end)) => Some((start, end)),
        _ => None,
    }
}

