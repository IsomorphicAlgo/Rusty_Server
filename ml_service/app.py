"""
ML Service for Solar Flare Prediction
FastAPI application providing ML inference endpoints
"""

from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Optional, List
from datetime import datetime
import logging

from models.solar_flare_predictor import SolarFlarePredictor
from models.model_loader import ModelLoader

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Create FastAPI app
app = FastAPI(
    title="Rusty Server ML Service",
    description="ML service for solar flare prediction",
    version="0.1.0"
)

# CORS middleware (allow Rust API to call this service)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # In production, restrict to Rust API URL
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Global model loader
model_loader = ModelLoader()
predictor: Optional[SolarFlarePredictor] = None


@app.on_event("startup")
async def startup_event():
    """Load model on startup"""
    global predictor
    try:
        logger.info("Loading solar flare prediction model...")
        predictor = model_loader.load_latest_model()
        if predictor is None:
            logger.warning("No model found. Train a model first using train_model.py")
        else:
            logger.info(f"Model loaded: {predictor.model_version}")
    except Exception as e:
        logger.error(f"Failed to load model: {e}")
        predictor = None


# Request/Response Models
class SpaceWeatherFeatures(BaseModel):
    """Space weather features for prediction"""
    kp_index: Optional[float] = None
    solar_wind_speed: Optional[float] = None
    solar_wind_density: Optional[float] = None
    solar_wind_temperature: Optional[float] = None
    solar_wind_bz: Optional[float] = None
    radiation_proton_flux: Optional[float] = None
    radiation_electron_flux: Optional[float] = None
    days_since_last_flare: Optional[float] = None
    flare_count_last_7_days: Optional[int] = None
    flare_count_last_30_days: Optional[int] = None


class PredictionRequest(BaseModel):
    """Request for solar flare prediction"""
    features: SpaceWeatherFeatures
    timestamp: Optional[datetime] = None


class PredictionResponse(BaseModel):
    """Response with prediction results"""
    predicted_flare_class: Optional[str]  # A, B, C, M, X, or None
    predicted_peak_time: Optional[datetime]
    confidence_score: float  # 0.0 to 1.0
    model_version: str
    prediction_timestamp: datetime
    features_used: List[str]


class HealthResponse(BaseModel):
    """Health check response"""
    status: str
    model_loaded: bool
    model_version: Optional[str] = None
    service_version: str = "0.1.0"


# Endpoints
@app.get("/health", response_model=HealthResponse)
async def health_check():
    """Health check endpoint"""
    return HealthResponse(
        status="healthy",
        model_loaded=predictor is not None,
        model_version=predictor.model_version if predictor else None
    )


@app.post("/predict", response_model=PredictionResponse)
async def predict_solar_flare(request: PredictionRequest):
    """
    Predict solar flare based on current space weather conditions
    
    Returns prediction with confidence score and model version
    """
    if predictor is None:
        raise HTTPException(
            status_code=503,
            detail="Model not loaded. Train a model first using train_model.py"
        )
    
    try:
        # Extract features
        features_dict = request.features.dict(exclude_none=True)
        
        # Make prediction
        prediction = predictor.predict(features_dict)
        
        return PredictionResponse(
            predicted_flare_class=prediction.get("flare_class"),
            predicted_peak_time=prediction.get("peak_time"),
            confidence_score=prediction.get("confidence", 0.0),
            model_version=predictor.model_version,
            prediction_timestamp=request.timestamp or datetime.utcnow(),
            features_used=list(features_dict.keys())
        )
    except Exception as e:
        logger.error(f"Prediction error: {e}")
        raise HTTPException(status_code=500, detail=f"Prediction failed: {str(e)}")


@app.get("/models")
async def list_models():
    """List available models"""
    try:
        models = model_loader.list_available_models()
        return {"models": models, "current": predictor.model_version if predictor else None}
    except Exception as e:
        logger.error(f"Error listing models: {e}")
        raise HTTPException(status_code=500, detail=str(e))


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8001)

