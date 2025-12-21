# ML Service for Solar Flare Prediction

Python microservice for CPU-based solar flare prediction using XGBoost.

## Setup

### 1. Install Dependencies

```bash
cd ml_service
pip install -r requirements.txt
```

### 2. Configure Database Connection

Set environment variable (same as Rust server):
```bash
# Windows PowerShell
$env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:password@localhost/rusty_server_test"

# Linux/Mac
export RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:password@localhost/rusty_server_test"
```

Or create `.env` file:
```
RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://rusty_user:password@localhost/rusty_server_test
```

### 3. Collect Training Data

First, collect historical data using the Rust script:
```bash
# From Rusty_Server root
cargo run --bin collect_training_data
```

### 4. Train Model

```bash
python train_model.py
```

This will:
- Load historical data from MySQL
- Engineer features
- Train XGBoost model
- Save model to `models/saved/`

### 5. Start ML Service

```bash
# Development
python app.py

# Or with uvicorn
uvicorn app:app --host 0.0.0.0 --port 8001
```

## API Endpoints

### Health Check
```
GET /health
```

### Predict Solar Flare
```
POST /predict
Content-Type: application/json

{
  "features": {
    "kp_index": 3.0,
    "solar_wind_speed": 450.0,
    "solar_wind_density": 5.0,
    "solar_wind_temperature": 50000.0,
    "solar_wind_bz": -2.5,
    "radiation_proton_flux": 1.5,
    "radiation_electron_flux": 50.0,
    "days_since_last_flare": 2.0,
    "flare_count_last_7_days": 3,
    "flare_count_last_30_days": 12
  }
}
```

### List Models
```
GET /models
```

## Model Information

- **Type**: XGBoost (CPU-optimized)
- **Features**: 10 space weather features
- **Output**: Flare class (A, B, C, M, X, or None) with confidence
- **Lead Time**: 2 hours (estimated peak time)

## Directory Structure

```
ml_service/
├── app.py                 # FastAPI application
├── train_model.py         # Training script
├── requirements.txt       # Python dependencies
├── models/
│   ├── __init__.py
│   ├── solar_flare_predictor.py  # XGBoost predictor
│   └── model_loader.py    # Model loading/versioning
└── models/saved/          # Saved model files (created on first train)
```

## Notes

- Models are saved with versioning (e.g., `model_v1.0.0_20241220_120000.pkl`)
- Model metadata is saved as JSON alongside model files
- Service runs on port 8001 by default
- CPU-optimized for 16-core Xeon processors

