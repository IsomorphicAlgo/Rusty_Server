"""
Solar Flare Predictor
CPU-optimized model for solar flare prediction using XGBoost
"""

import numpy as np
import pandas as pd
from typing import Dict, Optional, List
from datetime import datetime, timedelta
import logging
import xgboost as xgb
from sklearn.preprocessing import LabelEncoder

logger = logging.getLogger(__name__)


class SolarFlarePredictor:
    """
    Solar flare prediction model using XGBoost (CPU-optimized)
    
    Predicts solar flare class (A, B, C, M, X) based on space weather features
    """
    
    def __init__(self, model: Optional[xgb.XGBClassifier] = None, model_version: str = "v1.0.0"):
        """
        Initialize predictor
        
        Args:
            model: Trained XGBoost model (None for new model)
            model_version: Version string for this model
        """
        self.model = model
        self.model_version = model_version
        self.model_type = "XGBoost"
        self.label_encoder = LabelEncoder()
        self.feature_names = [
            "kp_index",
            "solar_wind_speed",
            "solar_wind_density",
            "solar_wind_temperature",
            "solar_wind_bz",
            "radiation_proton_flux",
            "radiation_electron_flux",
            "days_since_last_flare",
            "flare_count_last_7_days",
            "flare_count_last_30_days"
        ]
        
        # Flare class mapping (for encoding/decoding)
        self.flare_classes = ["None", "A", "B", "C", "M", "X"]
        if model is None:
            # Fit label encoder (for new models)
            self.label_encoder.fit(self.flare_classes)
    
    def predict(self, features: Dict) -> Dict:
        """
        Make a prediction based on space weather features
        
        Args:
            features: Dictionary of feature values
            
        Returns:
            Dictionary with prediction results:
            - flare_class: Predicted class (A, B, C, M, X, or None)
            - peak_time: Estimated peak time (2 hours from now)
            - confidence: Confidence score (0.0 to 1.0)
        """
        if self.model is None:
            raise ValueError("Model not trained. Train the model first.")
        
        # Prepare features as array
        feature_array = self._prepare_features(features)
        
        # Make prediction
        prediction = self.model.predict(feature_array.reshape(1, -1))[0]
        probabilities = self.model.predict_proba(feature_array.reshape(1, -1))[0]
        
        # Get confidence (max probability)
        confidence = float(np.max(probabilities))
        
        # Decode prediction
        flare_class = self.label_encoder.inverse_transform([prediction])[0]
        if flare_class == "None":
            flare_class = None
        
        # Estimate peak time (2 hours from now, as per Surya model lead time)
        peak_time = datetime.utcnow() + timedelta(hours=2)
        
        return {
            "flare_class": flare_class,
            "peak_time": peak_time,
            "confidence": confidence
        }
    
    def _prepare_features(self, features: Dict) -> np.ndarray:
        """
        Prepare feature dictionary into numpy array
        
        Args:
            features: Dictionary of feature values
            
        Returns:
            Numpy array of features in correct order
        """
        feature_array = []
        
        for feature_name in self.feature_names:
            value = features.get(feature_name)
            
            # Handle missing values (use median/default)
            if value is None:
                # Default values (can be improved with actual medians from training data)
                defaults = {
                    "kp_index": 2.0,
                    "solar_wind_speed": 400.0,
                    "solar_wind_density": 5.0,
                    "solar_wind_temperature": 50000.0,
                    "solar_wind_bz": 0.0,
                    "radiation_proton_flux": 1.0,
                    "radiation_electron_flux": 50.0,
                    "days_since_last_flare": 7.0,
                    "flare_count_last_7_days": 0,
                    "flare_count_last_30_days": 0
                }
                value = defaults.get(feature_name, 0.0)
            
            feature_array.append(float(value))
        
        return np.array(feature_array)
    
    def train(self, X: pd.DataFrame, y: pd.Series, **kwargs):
        """
        Train the XGBoost model
        
        Args:
            X: Feature DataFrame
            y: Target Series (flare classes)
            **kwargs: Additional XGBoost parameters
        """
        logger.info("Training XGBoost model for solar flare prediction...")
        
        # Encode target labels
        y_encoded = self.label_encoder.fit_transform(y)
        
        # Default XGBoost parameters (CPU-optimized)
        params = {
            "objective": "multi:softprob",
            "num_class": len(self.flare_classes),
            "max_depth": 6,
            "learning_rate": 0.1,
            "n_estimators": 100,
            "subsample": 0.8,
            "colsample_bytree": 0.8,
            "random_state": 42,
            "n_jobs": -1,  # Use all CPU cores
            "tree_method": "hist",  # CPU-optimized
            **kwargs
        }
        
        # Create and train model
        self.model = xgb.XGBClassifier(**params)
        self.model.fit(X, y_encoded)
        
        logger.info("Model training complete")
    
    def get_feature_importance(self) -> Dict[str, float]:
        """
        Get feature importance scores
        
        Returns:
            Dictionary mapping feature names to importance scores
        """
        if self.model is None:
            return {}
        
        importances = self.model.feature_importances_
        return dict(zip(self.feature_names, importances.tolist()))

