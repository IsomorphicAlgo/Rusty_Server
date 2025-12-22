/// Database Population Script
/// 
/// This script populates both test and production databases with:
/// - Exoplanet data from NASA Exoplanet Archive
/// - Historical solar flare data from NASA DONKI
/// 
/// Usage:
///   cargo run --bin populate_databases                    # Populate test database only
///   cargo run --bin populate_databases -- --all           # Populate both test and production
///   cargo run --bin populate_databases -- --exoplanets-only
///   cargo run --bin populate_databases -- --solar-only
///   cargo run --bin populate_databases -- --months 12     # Collect 12 months of solar data

use rusty_server::config::Config;
use rusty_server::Result;
use rusty_server::services::{DonkiClient, ExoplanetClient};
use rusty_server::database::DatabasePool;
use rusty_server::database::DatabaseOperations;
use rusty_server::models::*;
use chrono::{DateTime, Utc, Duration as ChronoDuration};
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
    let populate_all = args.contains(&"--all".to_string());
    let exoplanets_only = args.contains(&"--exoplanets-only".to_string());
    let solar_only = args.contains(&"--solar-only".to_string());
    let months = parse_months_arg(&args).unwrap_or(24); // Default: 24 months (2 years)

    info!("Starting database population script");
    if populate_all {
        info!("Will populate both test and production databases");
    } else {
        info!("Will populate test database only (use --all for both)");
    }

    // Load configuration
    let config = Config::load().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        rusty_server::AppError::Config(e)
    })?;

    // Initialize clients
    let exoplanet_client = ExoplanetClient::new(
        config.exoplanet.base_url.clone(),
        config.exoplanet.timeout_seconds,
    );

    let donki_client = DonkiClient::new(
        config.donki.base_url.clone(),
        config.donki.api_key.clone(),
        config.donki.timeout_seconds,
    );

    // Check if DONKI API key is configured (we'll check by trying to use it)
    let has_donki_key = config.donki.api_key.is_some();
    if !has_donki_key && !exoplanets_only {
        error!("DONKI API key not configured! Set RUSTY_SERVER__DONKI__API_KEY");
        return Err(rusty_server::AppError::Config(
            config::ConfigError::Message("DONKI API key required for solar data collection".to_string())
        ));
    }

    // Determine which databases to populate
    let databases = if populate_all {
        vec![
            ("rusty_server_test", "Test"),
            ("rusty_server", "Production"),
        ]
    } else {
        vec![("rusty_server_test", "Test")]
    };

    for (db_name, db_label) in databases {
        info!("{}", "=".repeat(60));
        info!("Populating {} database: {}", db_label, db_name);
        info!("{}", "=".repeat(60));

        // Build connection string for this database
        let connection_string = build_connection_string(&config.database.connection_string, db_name);
        
        // Initialize database
        let db_pool = DatabasePool::new(&connection_string).await?;
        db_pool.migrate().await?;
        let db_ops = DatabaseOperations::new(db_pool.pool().clone());
        info!("Connected to {} database", db_name);

        // Populate exoplanets
        if !solar_only {
            info!("");
            info!("Step 1: Populating exoplanet data...");
            populate_exoplanets(&exoplanet_client, &db_ops).await?;
        }

        // Populate solar/space weather data
        if !exoplanets_only {
            info!("");
            info!("Step 2: Populating historical solar flare data...");
            let end_date = Utc::now();
            let start_date = end_date - ChronoDuration::days((months * 30) as i64);
            populate_solar_data(&donki_client, &db_ops, start_date, end_date).await?;
        }

        info!("");
        info!("✓ {} database population complete!", db_label);
        info!("");
    }

    info!("{}", "=".repeat(60));
    info!("All databases populated successfully!");
    info!("{}", "=".repeat(60));

    Ok(())
}

async fn populate_exoplanets(
    client: &ExoplanetClient,
    db_ops: &DatabaseOperations,
) -> Result<()> {
    info!("Fetching exoplanets from NASA Exoplanet Archive...");
    info!("Note: ADQL doesn't support OFFSET well, so fetching a large batch");
    
    // Fetch in smaller batches to avoid JSON parsing issues with large responses
    // The Exoplanet Archive has ~5000+ confirmed exoplanets
    // Using smaller batches to avoid trailing comma issues in large JSON responses
    let batch_size = 2000;
    let mut total_stored = 0;
    let mut total_skipped = 0;

    let params = rusty_server::models::ExoplanetQueryParams {
        limit: Some(batch_size),
        offset: Some(0), // Will be ignored by ADQL but kept for compatibility
        sort_by: Some("pl_name".to_string()),
        sort_order: Some("asc".to_string()),
        ..Default::default()
    };

    info!("Fetching up to {} exoplanets...", batch_size);
    
    match client.query_exoplanets(&params).await {
        Ok(exoplanets) => {
            info!("Received {} exoplanets from TAP service", exoplanets.len());
            info!("Storing in database...");

            for exoplanet in &exoplanets {
                match db_ops.store_exoplanet(exoplanet).await {
                    Ok(_) => {
                        total_stored += 1;
                    }
                    Err(e) => {
                        warn!("Failed to store exoplanet {}: {}", exoplanet.pl_name, e);
                        total_skipped += 1;
                    }
                }

                // Progress updates and small delays
                if total_stored % 500 == 0 {
                    info!("Progress: {} exoplanets stored...", total_stored);
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                } else if total_stored % 100 == 0 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch exoplanets: {}", e);
            return Err(e);
        }
    }

    info!("Exoplanet population complete:");
    info!("  - Stored: {}", total_stored);
    info!("  - Skipped: {}", total_skipped);
    info!("  - Total: {}", total_stored + total_skipped);

    Ok(())
}

async fn populate_solar_data(
    donki_client: &DonkiClient,
    db_ops: &DatabaseOperations,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<()> {
    info!("Fetching solar flares from DONKI...");
    info!("Date range: {} to {}", start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d"));
    
    let days_span = (end_date - start_date).num_days();
    info!("Time span: {} days ({:.1} years)", days_span, days_span as f64 / 365.25);

    // Small delay before API call
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

    info!("Storing solar flares in database...");
    let mut flares_stored = 0;
    let mut flares_skipped = 0;

    for flare in &flares {
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
                // Show progress
                let progress_interval = if flares.len() > 500 { 50 } else { 25 };
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
        if flares_stored % 100 == 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    }

    info!("Solar data population complete:");
    info!("  - Flares collected: {}", flares.len());
    info!("  - Flares stored: {}", flares_stored);
    info!("  - Flares skipped: {}", flares_skipped);
    if flares_stored > 0 {
        let avg_per_month = (flares_stored as f64 / (days_span as f64 / 30.0)) as u32;
        info!("  - Average: ~{} flares per month", avg_per_month);
    }

    Ok(())
}

fn build_connection_string(original: &str, db_name: &str) -> String {
    // Extract the base connection string and replace the database name
    // Format: mysql://user:password@host:port/database_name
    if let Some(slash_pos) = original.rfind('/') {
        format!("{}/{}", &original[..slash_pos + 1], db_name)
    } else {
        // Fallback: just append the database name
        format!("{}/{}", original, db_name)
    }
}

fn parse_months_arg(args: &[String]) -> Option<u32> {
    for (i, arg) in args.iter().enumerate() {
        if arg == "--months" && i + 1 < args.len() {
            return args[i + 1].parse().ok();
        }
    }
    None
}

