-- ML Predictions Database Schema
-- Stores solar flare predictions and model tracking
-- Created: 2024-12-20

-- Solar Flare Predictions Table
-- Stores predictions made by ML models
CREATE TABLE IF NOT EXISTS solar_flare_predictions (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    
    -- Prediction details
    prediction_time DATETIME NOT NULL,
    predicted_flare_class VARCHAR(10) NULL,  -- A, B, C, M, X, or NULL
    predicted_peak_time DATETIME NULL,
    confidence_score DECIMAL(5,4) NOT NULL,  -- 0.0000 to 1.0000
    
    -- Model information
    model_version VARCHAR(50) NOT NULL,
    model_type VARCHAR(50) NOT NULL DEFAULT 'XGBoost',
    
    -- Input features used (stored as JSON for flexibility)
    input_features JSON NULL,
    
    -- Actual result (filled in later when flare occurs or doesn't)
    actual_flare_class VARCHAR(10) NULL,
    actual_peak_time DATETIME NULL,
    
    -- Accuracy tracking
    prediction_correct BOOLEAN NULL,  -- NULL until actual result is known
    accuracy_score DECIMAL(5,4) NULL,  -- Calculated accuracy metric
    
    -- Metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    -- Indexes for efficient querying
    INDEX idx_prediction_time (prediction_time),
    INDEX idx_predicted_peak_time (predicted_peak_time),
    INDEX idx_model_version (model_version),
    INDEX idx_prediction_correct (prediction_correct),
    INDEX idx_created_at (created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Model Versions Table
-- Tracks ML model versions and their performance
CREATE TABLE IF NOT EXISTS model_versions (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    version VARCHAR(50) NOT NULL UNIQUE,
    model_type VARCHAR(50) NOT NULL,
    model_path VARCHAR(255) NULL,
    
    -- Training information
    trained_at DATETIME NOT NULL,
    training_samples INT NULL,
    training_accuracy DECIMAL(5,4) NULL,
    
    -- Performance metrics (stored as JSON)
    performance_metrics JSON NULL,
    
    -- Feature information
    features_used JSON NULL,  -- List of feature names
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT FALSE,
    is_production BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    notes TEXT NULL,
    
    -- Indexes
    INDEX idx_version (version),
    INDEX idx_is_active (is_active),
    INDEX idx_is_production (is_production),
    INDEX idx_trained_at (trained_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

