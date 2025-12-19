#[cfg(test)]
mod tests {
    use crate::models::{
        validate_solar_flare_class, validate_geomagnetic_level, validate_radiation_alert_level,
        validate_kp_index, validate_kp_level,
        SolarFlare, GeomagneticStorm, RadiationLevels, SolarWind, KpIndex,
        HistoricalQuery, AlertQuery, RadiationQuery,
        SpaceWeatherResponse, SpaceWeatherData, ResponseMetadata,
    };
    use chrono::Utc;

    #[test]
    fn test_validate_solar_flare_class() {
        assert!(validate_solar_flare_class("X1.5").is_ok());
        assert!(validate_solar_flare_class("M2.3").is_ok());
        assert!(validate_solar_flare_class("C5.0").is_ok());
        assert!(validate_solar_flare_class("B1.2").is_ok());
        assert!(validate_solar_flare_class("A0.5").is_ok());
        assert!(validate_solar_flare_class("invalid").is_err());
    }

    #[test]
    fn test_validate_geomagnetic_level() {
        assert!(validate_geomagnetic_level("G5").is_ok());
        assert!(validate_geomagnetic_level("G1").is_ok());
        assert!(validate_geomagnetic_level("None").is_ok());
        assert!(validate_geomagnetic_level("invalid").is_err());
    }

    #[test]
    fn test_validate_radiation_alert_level() {
        assert!(validate_radiation_alert_level("S5").is_ok());
        assert!(validate_radiation_alert_level("S1").is_ok());
        assert!(validate_radiation_alert_level("None").is_ok());
        assert!(validate_radiation_alert_level("invalid").is_err());
    }

    #[test]
    fn test_validate_kp_index() {
        assert!(validate_kp_index(0.0).is_ok());
        assert!(validate_kp_index(4.5).is_ok());
        assert!(validate_kp_index(9.0).is_ok());
        assert!(validate_kp_index(-1.0).is_err());
        assert!(validate_kp_index(10.0).is_err());
    }

    #[test]
    fn test_validate_kp_level() {
        assert!(validate_kp_level("Quiet").is_ok());
        assert!(validate_kp_level("Extreme").is_ok());
        assert!(validate_kp_level("invalid").is_err());
    }

    #[test]
    fn test_solar_flare_validation() {
        let now = Utc::now();
        let valid_flare = SolarFlare {
            class: "M2.5".to_string(),
            peak_time: now,
            begin_time: Some(now - chrono::Duration::minutes(30)),
            end_time: Some(now + chrono::Duration::minutes(15)),
            source_location: Some("AR 12345".to_string()),
        };
        assert!(valid_flare.validate().is_ok());

        let invalid_class = SolarFlare {
            class: "Invalid".to_string(),
            peak_time: now,
            begin_time: None,
            end_time: None,
            source_location: None,
        };
        assert!(invalid_class.validate().is_err());

        let invalid_times = SolarFlare {
            class: "M2.5".to_string(),
            peak_time: now,
            begin_time: Some(now + chrono::Duration::minutes(30)), // After peak
            end_time: None,
            source_location: None,
        };
        assert!(invalid_times.validate().is_err());
    }

    #[test]
    fn test_geomagnetic_storm_validation() {
        let now = Utc::now();
        let valid_storm = GeomagneticStorm {
            level: "G1".to_string(),
            start_time: Some(now - chrono::Duration::hours(1)),
            end_time: Some(now + chrono::Duration::hours(5)),
            kp_index: 5.0,
        };
        assert!(valid_storm.validate().is_ok());

        let invalid_level = GeomagneticStorm {
            level: "Invalid".to_string(),
            start_time: None,
            end_time: None,
            kp_index: 5.0,
        };
        assert!(invalid_level.validate().is_err());

        let invalid_kp = GeomagneticStorm {
            level: "G1".to_string(),
            start_time: None,
            end_time: None,
            kp_index: 10.0, // Invalid
        };
        assert!(invalid_kp.validate().is_err());
    }

    #[test]
    fn test_radiation_levels_validation() {
        let valid_radiation = RadiationLevels {
            proton_flux: Some(1.5),
            electron_flux: Some(50.0),
            alert_level: "None".to_string(),
            timestamp: Utc::now(),
        };
        assert!(valid_radiation.validate().is_ok());

        let negative_flux = RadiationLevels {
            proton_flux: Some(-1.0), // Invalid
            electron_flux: Some(50.0),
            alert_level: "None".to_string(),
            timestamp: Utc::now(),
        };
        assert!(negative_flux.validate().is_err());
    }

    #[test]
    fn test_solar_wind_validation() {
        let valid_wind = SolarWind {
            speed: 450.0,
            density: 3.5,
            temperature: 50000.0,
            bz: Some(-2.5),
            timestamp: Utc::now(),
        };
        assert!(valid_wind.validate().is_ok());

        let negative_speed = SolarWind {
            speed: -100.0, // Invalid
            density: 3.5,
            temperature: 50000.0,
            bz: None,
            timestamp: Utc::now(),
        };
        assert!(negative_speed.validate().is_err());
    }

    #[test]
    fn test_kp_index_validation() {
        let valid_kp = KpIndex {
            value: 3.0,
            level: "Quiet".to_string(),
            timestamp: Utc::now(),
        };
        assert!(valid_kp.validate().is_ok());

        let invalid_value = KpIndex {
            value: 10.0, // Invalid
            level: "Quiet".to_string(),
            timestamp: Utc::now(),
        };
        assert!(invalid_value.validate().is_err());
    }

    #[test]
    fn test_historical_query_validation() {
        let valid_query = HistoricalQuery {
            start_date: None,
            end_date: None,
            data_type: None,
            limit: Some(10),
            offset: Some(0),
        };
        assert!(valid_query.validate().is_ok());

        let invalid_limit = HistoricalQuery {
            start_date: None,
            end_date: None,
            data_type: None,
            limit: Some(0), // Invalid
            offset: None,
        };
        assert!(invalid_limit.validate().is_err());

        let too_large_limit = HistoricalQuery {
            start_date: None,
            end_date: None,
            data_type: None,
            limit: Some(2000), // Too large
            offset: None,
        };
        assert!(too_large_limit.validate().is_err());
    }

    #[test]
    fn test_alert_query_validation() {
        let valid_query = AlertQuery {
            severity: Some("moderate".to_string()),
            alert_type: Some("solar_flare".to_string()),
            active_only: Some(true),
        };
        assert!(valid_query.validate().is_ok());

        let invalid_severity = AlertQuery {
            severity: Some("invalid".to_string()),
            alert_type: None,
            active_only: None,
        };
        assert!(invalid_severity.validate().is_err());
    }

    #[test]
    fn test_radiation_query_validation() {
        let valid_query = RadiationQuery {
            threshold: Some(1.5),
            alert_level: Some("S1".to_string()),
        };
        assert!(valid_query.validate().is_ok());

        let negative_threshold = RadiationQuery {
            threshold: Some(-1.0), // Invalid
            alert_level: None,
        };
        assert!(negative_threshold.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let response = SpaceWeatherResponse {
            data: SpaceWeatherData {
                solar_flare: None,
                geomagnetic_storm: None,
                radiation: Some(RadiationLevels {
                    proton_flux: Some(1.5),
                    electron_flux: Some(50.0),
                    alert_level: "None".to_string(),
                    timestamp: Utc::now(),
                }),
                solar_wind: None,
                kp_index: None,
            },
            metadata: ResponseMetadata {
                timestamp: Utc::now(),
                source: "test".to_string(),
                cached: false,
            },
        };

        // Test serialization
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("radiation"));
        assert!(json.contains("metadata"));

        // Test deserialization
        let deserialized: SpaceWeatherResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.metadata.source, "test");
    }
}

