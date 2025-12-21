"""
Training Script for Solar Flare Prediction Model
Trains XGBoost model on historical data from MySQL database
"""

import os
import sys
import pandas as pd
import numpy as np
from datetime import datetime, timedelta
from pathlib import Path
import logging
from dotenv import load_dotenv
import pymysql
from sqlalchemy import create_engine

from models.solar_flare_predictor import SolarFlarePredictor
from models.model_loader import ModelLoader

# Load environment variables
load_dotenv()

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


def get_database_connection():
    """Get database connection string from environment"""
    # Try to get from environment (same format as Rust config)
    connection_string = os.getenv("RUSTY_SERVER__DATABASE__CONNECTION_STRING")
    
    if not connection_string:
        # Fallback to individual components
        user = os.getenv("DB_USER", "rusty_user")
        password = os.getenv("DB_PASSWORD", "")
        host = os.getenv("DB_HOST", "localhost")
        port = os.getenv("DB_PORT", "3306")
        database = os.getenv("DB_NAME", "rusty_server_test")
        
        # Convert MySQL URL to SQLAlchemy format
        connection_string = f"mysql+pymysql://{user}:{password}@{host}:{port}/{database}"
    
    else:
        # Convert Rust MySQL URL format to SQLAlchemy format
        # mysql://user:pass@host/db -> mysql+pymysql://user:pass@host/db
        connection_string = connection_string.replace("mysql://", "mysql+pymysql://", 1)
    
    return connection_string


def load_training_data(connection_string: str, days_back: int = 1825) -> pd.DataFrame:
    """
    Load historical data from MySQL database
    
    Args:
        connection_string: Database connection string
        days_back: Number of days of historical data to load
        
    Returns:
        DataFrame with features and target
    """
    logger.info(f"Loading training data from database (last {days_back} days)...")
    
    try:
        engine = create_engine(connection_string)
        
        # Query to get historical observations with solar flares
        query = f"""
        SELECT 
            kp_index_value,
            solar_wind_speed,
            solar_wind_density,
            solar_wind_temperature,
            solar_wind_bz,
            radiation_proton_flux,
            radiation_electron_flux,
            solar_flare_class,
            timestamp
        FROM space_weather_observations
        WHERE timestamp >= DATE_SUB(NOW(), INTERVAL {days_back} DAY)
        ORDER BY timestamp
        """
        
        df = pd.read_sql(query, engine)
        logger.info(f"Loaded {len(df)} records from database")
        
        return df
    
    except Exception as e:
        logger.error(f"Failed to load training data: {e}")
        raise


def engineer_features(df: pd.DataFrame) -> pd.DataFrame:
    """
    Engineer features for training
    
    Args:
        df: Raw data DataFrame
        
    Returns:
        DataFrame with engineered features
    """
    logger.info("Engineering features...")
    
    # Sort by timestamp
    df = df.sort_values('timestamp').reset_index(drop=True)
    
    # Create feature DataFrame
    features_df = pd.DataFrame()
    
    # Basic features
    features_df['kp_index'] = df['kp_index_value'].fillna(2.0)
    features_df['solar_wind_speed'] = df['solar_wind_speed'].fillna(400.0)
    features_df['solar_wind_density'] = df['solar_wind_density'].fillna(5.0)
    features_df['solar_wind_temperature'] = df['solar_wind_temperature'].fillna(50000.0)
    features_df['solar_wind_bz'] = df['solar_wind_bz'].fillna(0.0)
    features_df['radiation_proton_flux'] = df['radiation_proton_flux'].fillna(1.0)
    features_df['radiation_electron_flux'] = df['radiation_electron_flux'].fillna(50.0)
    
    # Time-based features
    # Days since last flare
    flare_mask = df['solar_flare_class'].notna()
    last_flare_indices = []
    for i in range(len(df)):
        # Look back for last flare
        for j in range(i-1, max(-1, i-30), -1):
            if flare_mask.iloc[j]:
                days_since = (df.iloc[i]['timestamp'] - df.iloc[j]['timestamp']).days
                last_flare_indices.append(days_since)
                break
        else:
            last_flare_indices.append(30.0)  # Default if no recent flare
    
    features_df['days_since_last_flare'] = last_flare_indices
    
    # Flare counts in rolling windows
    features_df['flare_count_last_7_days'] = df['solar_flare_class'].notna().rolling(window=7, min_periods=1).sum().fillna(0).astype(int)
    features_df['flare_count_last_30_days'] = df['solar_flare_class'].notna().rolling(window=30, min_periods=1).sum().fillna(0).astype(int)
    
    # Target: flare class (or "None" if no flare)
    features_df['target'] = df['solar_flare_class'].fillna('None')
    
    # Filter out rows with all NaN features (if any)
    features_df = features_df.dropna(subset=['kp_index', 'solar_wind_speed'])
    
    logger.info(f"Feature engineering complete. {len(features_df)} samples ready for training")
    
    return features_df


def train_model():
    """Main training function"""
    logger.info("Starting model training...")
    
    # Get database connection
    try:
        connection_string = get_database_connection()
        logger.info("Database connection configured")
    except Exception as e:
        logger.error(f"Failed to configure database: {e}")
        logger.error("Set RUSTY_SERVER__DATABASE__CONNECTION_STRING environment variable")
        sys.exit(1)
    
    # Load training data
    try:
        df = load_training_data(connection_string, days_back=1825)  # 5 years
    except Exception as e:
        logger.error(f"Failed to load training data: {e}")
        sys.exit(1)
    
    if len(df) == 0:
        logger.error("No training data found. Run collect_training_data.rs first!")
        sys.exit(1)
    
    # Engineer features
    features_df = engineer_features(df)
    
    # Separate features and target
    feature_cols = [
        'kp_index', 'solar_wind_speed', 'solar_wind_density',
        'solar_wind_temperature', 'solar_wind_bz',
        'radiation_proton_flux', 'radiation_electron_flux',
        'days_since_last_flare', 'flare_count_last_7_days', 'flare_count_last_30_days'
    ]
    
    X = features_df[feature_cols]
    y = features_df['target']
    
    logger.info(f"Training on {len(X)} samples")
    logger.info(f"Class distribution:\n{y.value_counts()}")
    
    # Create and train model
    predictor = SolarFlarePredictor(model_version=f"v1.0.0_{datetime.now().strftime('%Y%m%d_%H%M%S')}")
    predictor.train(X, y)
    
    # Evaluate (simple accuracy on training set)
    from sklearn.metrics import accuracy_score, classification_report
    y_pred = predictor.model.predict(X)
    y_pred_decoded = predictor.label_encoder.inverse_transform(y_pred)
    accuracy = accuracy_score(y, y_pred_decoded)
    
    logger.info(f"Training accuracy: {accuracy:.4f}")
    logger.info("\nClassification Report:")
    logger.info(classification_report(y, y_pred_decoded))
    
    # Save feature importance
    importance = predictor.get_feature_importance()
    logger.info("\nFeature Importance:")
    for feature, score in sorted(importance.items(), key=lambda x: x[1], reverse=True):
        logger.info(f"  {feature}: {score:.4f}")
    
    # Store performance metrics
    predictor.performance_metrics = {
        "training_accuracy": float(accuracy),
        "training_samples": len(X),
        "class_distribution": y.value_counts().to_dict()
    }
    
    # Save model
    model_loader = ModelLoader()
    model_loader.save_model(predictor)
    
    logger.info("Model training and saving complete!")
    logger.info("You can now start the ML service: python -m uvicorn app:app --port 8001")


if __name__ == "__main__":
    train_model()

