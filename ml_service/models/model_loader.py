"""
Model Loader
Handles loading and versioning of ML models
"""

import os
import pickle
import json
from pathlib import Path
from typing import Optional, List, Dict
from datetime import datetime
import logging

from .solar_flare_predictor import SolarFlarePredictor

logger = logging.getLogger(__name__)


class ModelLoader:
    """Loads and manages ML model versions"""
    
    def __init__(self, model_dir: str = "models/saved"):
        """
        Initialize model loader
        
        Args:
            model_dir: Directory where models are stored
        """
        self.model_dir = Path(model_dir)
        self.model_dir.mkdir(parents=True, exist_ok=True)
    
    def load_latest_model(self) -> Optional[SolarFlarePredictor]:
        """
        Load the most recent model
        
        Returns:
            SolarFlarePredictor instance or None if no model found
        """
        models = self.list_available_models()
        if not models:
            logger.warning("No models found in model directory")
            return None
        
        # Get latest model (sorted by version/timestamp)
        latest = sorted(models, key=lambda x: x.get("timestamp", ""), reverse=True)[0]
        model_path = self.model_dir / latest["filename"]
        
        try:
            logger.info(f"Loading model: {latest['version']} from {model_path}")
            return self._load_model(model_path)
        except Exception as e:
            logger.error(f"Failed to load model {latest['version']}: {e}")
            return None
    
    def load_model_by_version(self, version: str) -> Optional[SolarFlarePredictor]:
        """
        Load a specific model version
        
        Args:
            version: Model version string
            
        Returns:
            SolarFlarePredictor instance or None if not found
        """
        model_path = self.model_dir / f"model_{version}.pkl"
        if not model_path.exists():
            logger.error(f"Model version {version} not found")
            return None
        
        try:
            return self._load_model(model_path)
        except Exception as e:
            logger.error(f"Failed to load model {version}: {e}")
            return None
    
    def _load_model(self, model_path: Path) -> SolarFlarePredictor:
        """Internal method to load model from file"""
        with open(model_path, 'rb') as f:
            predictor = pickle.load(f)
        
        if not isinstance(predictor, SolarFlarePredictor):
            raise ValueError("Loaded object is not a SolarFlarePredictor")
        
        return predictor
    
    def list_available_models(self) -> List[Dict]:
        """
        List all available model versions
        
        Returns:
            List of model metadata dictionaries
        """
        models = []
        
        for model_file in self.model_dir.glob("model_*.pkl"):
            # Extract version from filename: model_v1.0.0_20240101.pkl
            parts = model_file.stem.split("_")
            if len(parts) >= 2:
                version = parts[1]
                timestamp = parts[2] if len(parts) > 2 else ""
                
                # Try to load metadata if available
                metadata_file = model_file.with_suffix(".json")
                metadata = {}
                if metadata_file.exists():
                    try:
                        with open(metadata_file, 'r') as f:
                            metadata = json.load(f)
                    except Exception as e:
                        logger.warning(f"Failed to load metadata for {model_file}: {e}")
                
                models.append({
                    "version": version,
                    "filename": model_file.name,
                    "timestamp": timestamp,
                    "path": str(model_file),
                    "metadata": metadata
                })
        
        return models
    
    def save_model(self, predictor: SolarFlarePredictor, version: Optional[str] = None):
        """
        Save a trained model
        
        Args:
            predictor: Trained SolarFlarePredictor instance
            version: Optional version string (auto-generated if not provided)
        """
        if version is None:
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            version = f"v1.0.0_{timestamp}"
        
        model_path = self.model_dir / f"model_{version}.pkl"
        metadata_path = self.model_dir / f"model_{version}.json"
        
        # Save model
        with open(model_path, 'wb') as f:
            pickle.dump(predictor, f)
        
        # Save metadata
        metadata = {
            "version": version,
            "model_type": predictor.model_type,
            "trained_at": datetime.now().isoformat(),
            "features": predictor.feature_names,
            "performance": getattr(predictor, 'performance_metrics', {})
        }
        
        with open(metadata_path, 'w') as f:
            json.dump(metadata, f, indent=2)
        
        logger.info(f"Model saved: {version} at {model_path}")

