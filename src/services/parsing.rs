use crate::models::*;
use crate::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use tracing::warn;

/// Parse NOAA KP index data from JSON response
pub fn parse_kp_index(json: &Value) -> Result<Option<KpIndex>> {
    // NOAA returns {"value": [...]} with array of KP index readings
    let array = json.get("value").and_then(|v| v.as_array());
    
    if let Some(array) = array {
        if array.is_empty() {
            return Ok(None);
        }
        
        // Get the most recent reading (last in array)
        if let Some(latest) = array.last() {
            // Use estimated_kp if available, otherwise kp_index
            let kp_value = latest.get("estimated_kp")
                .and_then(|v| v.as_f64())
                .or_else(|| latest.get("kp_index").and_then(|v| v.as_u64().map(|n| n as f64)))
                .unwrap_or(0.0);
            
            // Validate KP value is in valid range (0-9)
            if kp_value < 0.0 || kp_value > 9.0 {
                warn!("Invalid KP index value: {}, clamping to valid range", kp_value);
                return Ok(None);
            }
            
            let time_str = latest.get("time_tag")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let timestamp = parse_noaa_timestamp(time_str)
                .unwrap_or_else(|| Utc::now());

            // Validate timestamp is not too far in the future
            let now = Utc::now();
            if timestamp > now + chrono::Duration::hours(1) {
                warn!("KP index timestamp is in the future: {}, using current time", timestamp);
                return Ok(None);
            }

            return Ok(Some(KpIndex {
                value: kp_value,
                level: kp_value_to_level(kp_value),
                timestamp,
            }));
        }
    }
    
    Ok(None)
}

/// Parse solar wind data from NOAA JSON responses
pub fn parse_solar_wind(mag_json: &Value, plasma_json: &Value) -> Result<Option<SolarWind>> {
    // Get the most recent active reading from magnetic field data
    let (bz, timestamp) = match extract_latest_active_reading(mag_json) {
        Some(entry) => {
            let bz = entry.get("bz_gsm").and_then(|v| v.as_f64());
            let time_str = entry.get("time_tag").and_then(|v| v.as_str()).unwrap_or("");
            let timestamp = parse_noaa_timestamp(time_str).unwrap_or_else(|| Utc::now());
            (bz, timestamp)
        }
        None => return Ok(None),
    };
    
    // Get speed, density, temperature from plasma data (if available)
    let (speed, density, temperature) = if let Some(entry) = extract_latest_active_reading(plasma_json) {
        let speed = entry.get("speed").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let density = entry.get("density").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let temperature = entry.get("temperature").and_then(|v| v.as_f64()).unwrap_or(0.0);
        
        // Validate values are reasonable
        let speed = if speed < 0.0 || speed > 2000.0 {
            warn!("Invalid solar wind speed: {} km/s, using 0.0", speed);
            0.0
        } else {
            speed
        };
        
        let density = if density < 0.0 || density > 100.0 {
            warn!("Invalid solar wind density: {} cm^-3, using 0.0", density);
            0.0
        } else {
            density
        };
        
        let temperature = if temperature < 0.0 || temperature > 1_000_000.0 {
            warn!("Invalid solar wind temperature: {} K, using 0.0", temperature);
            0.0
        } else {
            temperature
        };
        
        (speed, density, temperature)
    } else {
        (0.0, 0.0, 0.0)
    };
    
    Ok(Some(SolarWind {
        speed,
        density,
        temperature,
        bz,
        timestamp,
    }))
}

/// Extract the most recent active reading from a NOAA JSON array
fn extract_latest_active_reading(json: &Value) -> Option<&Value> {
    let array = json.as_array()?;
    if array.is_empty() {
        return None;
    }
    
    // Find the most recent active reading (prefer active=true)
    let mut latest_active: Option<&Value> = None;
    let mut latest_timestamp = String::new();
    
    for entry in array.iter().rev() {
        if let Some(active) = entry.get("active").and_then(|v| v.as_bool()) {
            if active {
                if let Some(time_str) = entry.get("time_tag").and_then(|v| v.as_str()) {
                    if time_str > latest_timestamp.as_str() {
                        latest_timestamp = time_str.to_string();
                        latest_active = Some(entry);
                    }
                }
            }
        }
    }
    
    // If no active reading found, use the most recent entry
    latest_active.or_else(|| array.last())
}

/// Parse NOAA timestamp format
pub fn parse_noaa_timestamp(time_str: &str) -> Option<DateTime<Utc>> {
    if time_str.is_empty() {
        return None;
    }

    // Try ISO 8601 format first (with Z)
    if let Ok(dt) = DateTime::parse_from_rfc3339(time_str) {
        return Some(dt.with_timezone(&Utc));
    }

    // Try format without Z (assume UTC)
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S") {
        return Some(DateTime::from_naive_utc_and_offset(dt, Utc));
    }

    // Try space-separated format (assume UTC)
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
        return Some(DateTime::from_naive_utc_and_offset(dt, Utc));
    }

    None
}

/// Convert KP value to level string
pub fn kp_value_to_level(value: f64) -> String {
    match value {
        v if v < 2.0 => "Quiet",
        v if v < 3.0 => "Unsettled",
        v if v < 4.0 => "Active",
        v if v < 5.0 => "Minor",
        v if v < 6.0 => "Moderate",
        v if v < 7.0 => "Strong",
        v if v < 8.0 => "Severe",
        _ => "Extreme",
    }
    .to_string()
}

/// Convert KP value to geomagnetic storm level
pub fn kp_to_geomagnetic_level(kp: f64) -> String {
    match kp {
        v if v >= 9.0 => "G5",
        v if v >= 8.0 => "G4",
        v if v >= 7.0 => "G3",
        v if v >= 6.0 => "G2",
        v if v >= 5.0 => "G1",
        _ => "None",
    }
    .to_string()
}

/// Validate parsed space weather data
pub fn validate_space_weather_data(data: &SpaceWeatherData) -> Result<()> {
    // Validate KP index if present
    if let Some(ref kp) = data.kp_index {
        if kp.value < 0.0 || kp.value > 9.0 {
            return Err(crate::AppError::Validation(
                format!("Invalid KP index value: {}", kp.value)
            ));
        }
    }
    
    // Validate solar wind if present
    if let Some(ref wind) = data.solar_wind {
        if wind.speed < 0.0 || wind.speed > 2000.0 {
            return Err(crate::AppError::Validation(
                format!("Invalid solar wind speed: {} km/s", wind.speed)
            ));
        }
        
        if wind.density < 0.0 || wind.density > 100.0 {
            return Err(crate::AppError::Validation(
                format!("Invalid solar wind density: {} cm^-3", wind.density)
            ));
        }
    }
    
    // Validate geomagnetic storm if present
    if let Some(ref storm) = data.geomagnetic_storm {
        if storm.kp_index < 0.0 || storm.kp_index > 9.0 {
            return Err(crate::AppError::Validation(
                format!("Invalid geomagnetic storm KP index: {}", storm.kp_index)
            ));
        }
    }
    
    Ok(())
}

/// Parse DONKI solar flare data from JSON response
pub fn parse_donki_flare(json: &Value) -> Result<Option<SolarFlare>> {
    // DONKI FLR response format:
    // {
    //   "flrID": "2025-12-15T00:00:00-FLR-001",
    //   "beginTime": "2025-12-15T00:00Z",
    //   "peakTime": "2025-12-15T00:10Z",
    //   "endTime": "2025-12-15T00:20Z",
    //   "classType": "C1.0",
    //   "sourceLocation": "N10W10",
    //   "activeRegionNum": "12345"
    // }

    let class_type = json.get("classType").and_then(|v| v.as_str());
    let peak_time_str = json.get("peakTime").and_then(|v| v.as_str());
    let begin_time_str = json.get("beginTime").and_then(|v| v.as_str());
    let end_time_str = json.get("endTime").and_then(|v| v.as_str());
    let source_location = json.get("sourceLocation").and_then(|v| v.as_str());
    let active_region = json.get("activeRegionNum").and_then(|v| v.as_str());

    // classType and peakTime are required
    let class = match class_type {
        Some(c) => c.to_string(),
        None => {
            warn!("DONKI flare missing classType field");
            return Ok(None);
        }
    };

    let peak_time = if let Some(time_str) = peak_time_str {
        parse_donki_timestamp(time_str).unwrap_or_else(|| Utc::now())
    } else {
        warn!("DONKI flare missing peakTime field");
        return Ok(None);
    };

    let begin_time = begin_time_str.and_then(|s| parse_donki_timestamp(s));
    let end_time = end_time_str.and_then(|s| parse_donki_timestamp(s));

    // Combine sourceLocation and activeRegionNum if both exist
    let source_location_str = match (source_location, active_region) {
        (Some(loc), Some(region)) => Some(format!("{} AR {}", loc, region)),
        (Some(loc), None) => Some(loc.to_string()),
        (None, Some(region)) => Some(format!("AR {}", region)),
        (None, None) => None,
    };

    Ok(Some(SolarFlare {
        class,
        peak_time,
        begin_time,
        end_time,
        source_location: source_location_str,
    }))
}

/// Parse DONKI timestamp format (ISO 8601 with Z)
fn parse_donki_timestamp(time_str: &str) -> Option<DateTime<Utc>> {
    if time_str.is_empty() {
        return None;
    }

    // Try ISO 8601 format with Z
    if let Ok(dt) = DateTime::parse_from_rfc3339(time_str) {
        return Some(dt.with_timezone(&Utc));
    }

    // Try format without Z (assume UTC)
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S") {
        return Some(DateTime::from_naive_utc_and_offset(dt, Utc));
    }

    warn!("Failed to parse DONKI timestamp: {}", time_str);
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_kp_index_valid() {
        let json = json!({
            "value": [
                {
                    "time_tag": "2024-12-19T20:00:00Z",
                    "kp_index": 2,
                    "estimated_kp": 1.67,
                    "kp": "2M"
                }
            ]
        });
        
        let result = parse_kp_index(&json).unwrap();
        assert!(result.is_some());
        let kp = result.unwrap();
        assert_eq!(kp.value, 1.67);
        assert_eq!(kp.level, "Quiet"); // 1.67 < 2.0, so it's "Quiet"
    }

    #[test]
    fn test_parse_kp_index_empty() {
        let json = json!({
            "value": []
        });
        
        let result = parse_kp_index(&json).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_kp_index_invalid_value() {
        let json = json!({
            "value": [
                {
                    "time_tag": "2024-12-19T20:00:00Z",
                    "estimated_kp": 15.0,
                    "kp": "15Z"
                }
            ]
        });
        
        let result = parse_kp_index(&json).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_noaa_timestamp() {
        // Test ISO 8601 format
        let dt1 = parse_noaa_timestamp("2024-12-19T20:00:00Z");
        assert!(dt1.is_some());

        // Test format without Z
        let dt2 = parse_noaa_timestamp("2024-12-19T20:00:00");
        assert!(dt2.is_some());

        // Test empty string
        let dt3 = parse_noaa_timestamp("");
        assert!(dt3.is_none());
    }

    #[test]
    fn test_kp_value_to_level() {
        assert_eq!(kp_value_to_level(1.0), "Quiet");
        assert_eq!(kp_value_to_level(4.5), "Minor");
        assert_eq!(kp_value_to_level(7.5), "Severe");
        assert_eq!(kp_value_to_level(9.0), "Extreme");
    }

    #[test]
    fn test_kp_to_geomagnetic_level() {
        assert_eq!(kp_to_geomagnetic_level(4.0), "None");
        assert_eq!(kp_to_geomagnetic_level(5.0), "G1");
        assert_eq!(kp_to_geomagnetic_level(7.0), "G3");
        assert_eq!(kp_to_geomagnetic_level(9.0), "G5");
    }

    #[test]
    fn test_parse_solar_wind_magnetic_only() {
        let mag_json = json!([
            {
                "time_tag": "2024-12-19T20:00:00Z",
                "active": true,
                "source": "ACE",
                "bz_gsm": -2.5
            }
        ]);
        
        let plasma_json = json!(null);
        
        let result = parse_solar_wind(&mag_json, &plasma_json).unwrap();
        assert!(result.is_some());
        let wind = result.unwrap();
        assert_eq!(wind.bz, Some(-2.5));
        assert_eq!(wind.speed, 0.0); // No plasma data
    }

    #[test]
    fn test_validate_space_weather_data() {
        let valid_data = SpaceWeatherData {
            kp_index: Some(KpIndex {
                value: 3.0,
                level: "Active".to_string(),
                timestamp: Utc::now(),
            }),
            solar_wind: Some(SolarWind {
                speed: 450.0,
                density: 3.5,
                temperature: 50000.0,
                bz: Some(-2.5),
                timestamp: Utc::now(),
            }),
            geomagnetic_storm: None,
            solar_flare: None,
            radiation: None,
        };
        
        assert!(validate_space_weather_data(&valid_data).is_ok());
    }

    #[test]
    fn test_validate_space_weather_data_invalid_kp() {
        let invalid_data = SpaceWeatherData {
            kp_index: Some(KpIndex {
                value: 15.0, // Invalid
                level: "Extreme".to_string(),
                timestamp: Utc::now(),
            }),
            solar_wind: None,
            geomagnetic_storm: None,
            solar_flare: None,
            radiation: None,
        };
        
        assert!(validate_space_weather_data(&invalid_data).is_err());
    }

    #[test]
    fn test_parse_donki_flare_valid() {
        let json = json!({
            "flrID": "2024-12-19T12:00:00-FLR-001",
            "beginTime": "2024-12-19T12:00:00Z",
            "peakTime": "2024-12-19T12:05:00Z",
            "endTime": "2024-12-19T12:10:00Z",
            "classType": "C2.5",
            "sourceLocation": "N10W10",
            "activeRegionNum": "12345"
        });
        
        let result = parse_donki_flare(&json).unwrap();
        assert!(result.is_some());
        let flare = result.unwrap();
        assert_eq!(flare.class, "C2.5");
        assert_eq!(flare.source_location, Some("N10W10 AR 12345".to_string()));
    }
}

