use crate::models::*;

/// Validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidSolarFlareClass(String),
    InvalidGeomagneticLevel(String),
    InvalidRadiationAlertLevel(String),
    InvalidKpIndex(f64),
    InvalidKpLevel(String),
    InvalidDateRange,
    InvalidLimit(u32),
    InvalidOffset(u32),
    NegativeValue(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidSolarFlareClass(class) => {
                write!(f, "Invalid solar flare class: {}. Must be X, M, C, B, or A", class)
            }
            ValidationError::InvalidGeomagneticLevel(level) => {
                write!(f, "Invalid geomagnetic level: {}. Must be G5, G4, G3, G2, G1, or None", level)
            }
            ValidationError::InvalidRadiationAlertLevel(level) => {
                write!(f, "Invalid radiation alert level: {}. Must be S1-S5 or None", level)
            }
            ValidationError::InvalidKpIndex(value) => {
                write!(f, "Invalid KP index: {}. Must be between 0 and 9", value)
            }
            ValidationError::InvalidKpLevel(level) => {
                write!(f, "Invalid KP level: {}", level)
            }
            ValidationError::InvalidDateRange => {
                write!(f, "Invalid date range: start_date must be before end_date")
            }
            ValidationError::InvalidLimit(limit) => {
                write!(f, "Invalid limit: {}. Must be between 1 and 1000", limit)
            }
            ValidationError::InvalidOffset(offset) => {
                write!(f, "Invalid offset: {}. Must be non-negative", offset)
            }
            ValidationError::NegativeValue(field) => {
                write!(f, "Invalid {}: value cannot be negative", field)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validation result type
pub type ValidationResult = Result<(), ValidationError>;

/// Validate solar flare class
pub fn validate_solar_flare_class(class: &str) -> ValidationResult {
    let class_upper = class.to_uppercase();
    if class_upper.starts_with('X') || class_upper.starts_with('M') 
        || class_upper.starts_with('C') || class_upper.starts_with('B') 
        || class_upper.starts_with('A') {
        Ok(())
    } else {
        Err(ValidationError::InvalidSolarFlareClass(class.to_string()))
    }
}

/// Validate geomagnetic storm level
pub fn validate_geomagnetic_level(level: &str) -> ValidationResult {
    let level_upper = level.to_uppercase();
    if level_upper == "G5" || level_upper == "G4" || level_upper == "G3" 
        || level_upper == "G2" || level_upper == "G1" || level_upper == "NONE" {
        Ok(())
    } else {
        Err(ValidationError::InvalidGeomagneticLevel(level.to_string()))
    }
}

/// Validate radiation alert level
pub fn validate_radiation_alert_level(level: &str) -> ValidationResult {
    let level_upper = level.to_uppercase();
    if level_upper == "S5" || level_upper == "S4" || level_upper == "S3" 
        || level_upper == "S2" || level_upper == "S1" || level_upper == "NONE" {
        Ok(())
    } else {
        Err(ValidationError::InvalidRadiationAlertLevel(level.to_string()))
    }
}

/// Validate KP index value
pub fn validate_kp_index(value: f64) -> ValidationResult {
    if value >= 0.0 && value <= 9.0 {
        Ok(())
    } else {
        Err(ValidationError::InvalidKpIndex(value))
    }
}

/// Validate KP level string
pub fn validate_kp_level(level: &str) -> ValidationResult {
    let valid_levels = [
        "Quiet", "Unsettled", "Active", "Minor", 
        "Moderate", "Strong", "Severe", "Extreme"
    ];
    if valid_levels.contains(&level) {
        Ok(())
    } else {
        Err(ValidationError::InvalidKpLevel(level.to_string()))
    }
}

/// Validate solar flare data
impl SolarFlare {
    pub fn validate(&self) -> ValidationResult {
        validate_solar_flare_class(&self.class)?;
        
        if let Some(begin) = self.begin_time {
            if let Some(end) = self.end_time {
                if begin > end {
                    return Err(ValidationError::InvalidDateRange);
                }
            }
            if begin > self.peak_time {
                return Err(ValidationError::InvalidDateRange);
            }
        }
        
        if let Some(end) = self.end_time {
            if end < self.peak_time {
                return Err(ValidationError::InvalidDateRange);
            }
        }
        
        Ok(())
    }
}

/// Validate geomagnetic storm data
impl GeomagneticStorm {
    pub fn validate(&self) -> ValidationResult {
        validate_geomagnetic_level(&self.level)?;
        validate_kp_index(self.kp_index)?;
        
        if let Some(start) = self.start_time {
            if let Some(end) = self.end_time {
                if start > end {
                    return Err(ValidationError::InvalidDateRange);
                }
            }
        }
        
        Ok(())
    }
}

/// Validate radiation levels
impl RadiationLevels {
    pub fn validate(&self) -> ValidationResult {
        validate_radiation_alert_level(&self.alert_level)?;
        
        if let Some(flux) = self.proton_flux {
            if flux < 0.0 {
                return Err(ValidationError::NegativeValue("proton_flux".to_string()));
            }
        }
        
        if let Some(flux) = self.electron_flux {
            if flux < 0.0 {
                return Err(ValidationError::NegativeValue("electron_flux".to_string()));
            }
        }
        
        Ok(())
    }
}

/// Validate solar wind data
impl SolarWind {
    pub fn validate(&self) -> ValidationResult {
        if self.speed < 0.0 {
            return Err(ValidationError::NegativeValue("speed".to_string()));
        }
        if self.density < 0.0 {
            return Err(ValidationError::NegativeValue("density".to_string()));
        }
        if self.temperature < 0.0 {
            return Err(ValidationError::NegativeValue("temperature".to_string()));
        }
        
        Ok(())
    }
}

/// Validate KP index
impl KpIndex {
    pub fn validate(&self) -> ValidationResult {
        validate_kp_index(self.value)?;
        validate_kp_level(&self.level)?;
        Ok(())
    }
}

/// Validate historical query parameters
impl HistoricalQuery {
    pub fn validate(&self) -> ValidationResult {
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 1000 {
                return Err(ValidationError::InvalidLimit(limit));
            }
        }
        
        // Offset validation: type ensures it's non-negative (u32)
        // No additional validation needed
        let _ = self.offset;
        
        // Validate date range if both are provided
        if let (Some(start_str), Some(end_str)) = (&self.start_date, &self.end_date) {
            if let (Ok(start), Ok(end)) = (
                chrono::DateTime::parse_from_rfc3339(start_str),
                chrono::DateTime::parse_from_rfc3339(end_str)
            ) {
                if start > end {
                    return Err(ValidationError::InvalidDateRange);
                }
            }
        }
        
        Ok(())
    }
}

/// Validate alert query parameters
impl AlertQuery {
    pub fn validate(&self) -> ValidationResult {
        if let Some(severity) = &self.severity {
            let valid_severities = ["minor", "moderate", "strong", "severe", "extreme"];
            if !valid_severities.contains(&severity.to_lowercase().as_str()) {
                return Err(ValidationError::InvalidGeomagneticLevel(severity.clone()));
            }
        }
        
        if let Some(alert_type) = &self.alert_type {
            let valid_types = ["solar_flare", "geomagnetic_storm", "radiation"];
            if !valid_types.contains(&alert_type.to_lowercase().as_str()) {
                return Err(ValidationError::InvalidGeomagneticLevel(alert_type.clone()));
            }
        }
        
        Ok(())
    }
}

/// Validate radiation query parameters
impl RadiationQuery {
    pub fn validate(&self) -> ValidationResult {
        if let Some(threshold) = self.threshold {
            if threshold < 0.0 {
                return Err(ValidationError::NegativeValue("threshold".to_string()));
            }
        }
        
        if let Some(alert_level) = &self.alert_level {
            validate_radiation_alert_level(alert_level)?;
        }
        
        Ok(())
    }
}

