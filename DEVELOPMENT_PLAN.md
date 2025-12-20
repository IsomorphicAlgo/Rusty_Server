# Rusty Server - Development Plan

## Overview

This development plan incorporates all planning information for Rusty_Server, a Rust-based REST API service for fetching, caching, and serving space weather data. The plan maintains an iterative development approach while expanding into a comprehensive astronomical and space weather monitoring platform with satellite tracking, Mars weather forecasting, and advanced machine learning integration.

---

## 1. Project Overview

### Project Goals

- Build a production-ready REST API service for space weather data
- Integrate with NOAA Space Weather API and NASA DONKI
- Provide local caching and historical data storage
- Implement security features (rate limiting, authentication)
- Deploy on personal server rack hardware
- Expand into comprehensive astronomical monitoring platform
- Showcase for portfolio (GitHub/LinkedIn)

### Hardware Context

- **CPU**: 2x 8-core 8-thread Xeon processors
- **Memory**: 32GB DDR4 ECC
- **Storage**: SAS3 12-drive backplane (12 TB available, can acquire more)
- **Network**: 4x 10G RJ45 ports
- **Management**: IPMI
- **Power**: Redundant 800W PSUs
- **GPU**: NVIDIA GTX 960, RTX 2070 available (can acquire more 2070s/2080s)

### Development & Deployment Workflow

**Development Environment (Windows Laptop):**
- Primary development and testing on Windows laptop
- Local MySQL instance or connect to server's MySQL
- Test API endpoints on localhost
- Run all tests and development tools

**Deployment Environment (Linux Server Rack):**
- Target Platform: Linux
- Build Process: Cross-compile from Windows or build on server
- Production MySQL instance on server
- Service management: systemd service
- Network: Accessible via 10G network ports

---

## 2. Current Status

### Completed Phases

**✅ Phase 1: Project Foundation & Setup**
- Project structure and dependencies configured
- Configuration system with environment variable support
- Logging and error handling systems
- Module structure organized

**✅ Phase 2: Core API Infrastructure**
- HTTP server with axum framework
- REST API structure with versioning
- Health check endpoints
- Data models for space weather data
- Request/response models with validation

**✅ Phase 3: Data Fetching & Integration**
- NOAA API integration with retry logic
- Data parsing and transformation
- Service layer for fetching space weather data
- Error handling for API failures

**✅ Phase 4: Data Storage & Caching**
- MySQL database schema (designed for 10+ years of data)
- Database connection pool with health checks
- Complete CRUD operations with transactions
- High-performance in-memory caching (moka)
- Cache TTL management and metrics

**✅ Phase 5: API Implementation**
- Current conditions endpoint (cache → API → database → mock fallback)
- Historical data endpoint with query parameters and pagination
- Alerts endpoint with filtering
- Radiation endpoint with threshold filtering
- Comprehensive error handling and logging

**✅ Phase 6: Security & Production Features**
- Rate limiting (per-IP token bucket algorithm)
- API key authentication with configurable requirement
- API key management (generate, list, revoke)
- Security hardening (CORS, security headers, request size limits)
- Security logging

**✅ Phase 7.1: CLI Integration Planning**
- Integration plan document created
- Architecture designed for CLI_Astro_Calc integration

### Next Steps

- Phase 8: Testing & Quality Assurance
- Phase 9: Deployment & Operations
- Phase 10: Advanced Data Sources & ML Integration
- Phase 11: Satellite Tracking & Orbital Decay
- Phase 12: Mars Weather Forecasting

---

## 3. Current Priorities

Based on project goals and requirements, the following priorities have been established:

### Priority A: Solar Flare Data Acquisition
**Status**: In Progress
- Acquire solar flare data and other space weather data
- Implement NASA DONKI integration for solar flares (FLR endpoint)
- Add radiation data endpoints
- Complete space weather data coverage

### Priority B: Web UI Development
**Status**: Not Started
- Create simple web page to interact with data
- Display solar data within a week
- Add query section for date ranges and data types
- Connect to existing REST API endpoints

### Priority C: ML Integration
**Status**: Waiting for Priority A completion
- Set up Python microservice for ML training/inference
- Create data pipeline from database → ML service
- Implement simple prediction model
- Store predictions in database

### Priority D: Predictions Display
**Status**: Blocked by Priorities B and C
- Extend web UI with predictions vs actual display
- Add comparison visualizations
- Display accuracy metrics

### Priority E: Mars Project
**Status**: Future
- Similar structure to solar weather project
- Mars weather data integration
- Mars weather forecasting models

---

## 4. Next Phases

### Phase 8: Testing & Quality Assurance

#### Step 8.1: Comprehensive Testing
**Objective**: Ensure comprehensive test coverage.

**Tasks**:
- [ ] Review test coverage
- [ ] Add missing unit tests
- [ ] Add integration tests for all endpoints
- [ ] Add end-to-end tests
- [ ] Add performance/load tests
- [ ] Add security tests
- [ ] Set up test coverage reporting (optional)

**Deliverables**:
- Comprehensive test suite
- Test coverage report
- All tests passing

#### Step 8.2: Documentation
**Objective**: Complete project documentation.

**Tasks**:
- [ ] Update README.md with complete information
- [ ] Complete OVERVIEW.md with architecture details
- [ ] Add inline code documentation
- [ ] Create API documentation (OpenAPI/Swagger)

**Deliverables**:
- Complete README.md
- Complete OVERVIEW.md
- Code documentation
- API documentation

---

### Phase 9: Deployment & Operations

#### Step 9.1: Deployment Preparation
**Objective**: Prepare the service for deployment.

**Tasks**:
- [ ] Create deployment configuration
- [ ] Set up environment-specific configs (dev, prod)
- [ ] Create Docker container (optional)
- [ ] Set up systemd service file (for Linux)
- [ ] Create deployment scripts
- [ ] Document deployment process
- [ ] Set up backup procedures for database

**Deliverables**:
- Deployment configuration
- Deployment scripts
- Deployment documentation

#### Step 9.2: Monitoring & Observability
**Objective**: Add monitoring and observability features.

**Tasks**:
- [ ] Add metrics collection (Prometheus format)
- [ ] Enhance health check endpoints (database, cache, external APIs)
- [ ] Implement structured logging (already done, enhance)
- [ ] Add performance monitoring
- [ ] Create monitoring dashboard (optional)
- [ ] Add alerting for critical issues (optional)
- [ ] Add cache metrics endpoint

**Deliverables**:
- Metrics collection
- Enhanced health monitoring
- Observability features

#### Step 9.3: Production Deployment
**Objective**: Deploy to production server.

**Tasks**:
- [ ] Set up production environment
- [ ] Configure production database
- [ ] Set up reverse proxy (nginx) with SSL/TLS
- [ ] Configure SSL/TLS certificates (Let's Encrypt)
- [ ] Deploy application
- [ ] Verify deployment
- [ ] Monitor initial operation
- [ ] Set up automated backups

**Deliverables**:
- Production deployment
- Running service with HTTPS
- Monitoring in place
- Backup system operational

---

### Phase 10: Advanced Data Sources & ML Integration

#### Step 10.1: NASA DONKI Integration
**Objective**: Integrate NASA Space Weather Database (DONKI) for comprehensive space weather alerts.

**Tasks**:
- [ ] Research DONKI API endpoints (CMEs, Solar Flares, Geomagnetic Storms, IPS, HSS)
- [ ] Implement DONKI API client
- [ ] Add data models for DONKI events
- [ ] Create database schema for DONKI events (already compatible)
- [ ] Implement event parsing and storage
- [ ] Add endpoints for DONKI data queries
- [ ] Add real-time alert processing
- [ ] Integrate with existing space weather endpoints

**Deliverables**:
- DONKI API integration
- Database schema for space weather events
- API endpoints for DONKI data
- Real-time alert processing

**Data Sources**:
- CME (Coronal Mass Ejections)
- FLR (Solar Flares) - **Priority A**
- GST (Geomagnetic Storms)
- IPS (Interplanetary Shocks)
- HSS (High Speed Streams)
- WSAEnlilSimulations

**Implementation Notes**:
- See `DONKI_IMPLEMENTATION_ANALYSIS.md` for detailed implementation guide
- Free NASA API key required (register at https://api.nasa.gov)
- Rate limits: 1,000 requests/hour with registered key
- Database schema already supports solar flares

#### Step 10.2: Exoplanet Archive Integration
**Objective**: Integrate NASA Exoplanet Archive using TAP protocol for exoplanet data.

**Tasks**:
- [ ] Research TAP (Table Access Protocol) for Exoplanet Archive
- [ ] Implement TAP client with ADQL query support
- [ ] Add data models for exoplanet data (ps, pscomppars tables)
- [ ] Create database schema for exoplanet data
- [ ] Implement exoplanet data ingestion
- [ ] Add endpoints for exoplanet queries
- [ ] Add discovery notification tracking
- [ ] Implement periodic data updates

**Deliverables**:
- TAP protocol integration
- Exoplanet database schema
- API endpoints for exoplanet queries
- Discovery tracking system

**Key Tables**:
- `ps` (Planetary Systems)
- `pscomppars` (Composite Parameters)
- `cumulative` (KOI Cumulative List)

#### Step 10.3: Surya ML Model Integration (Optional - Advanced)
**Objective**: Integrate Surya foundation model for solar flare prediction.

**Tasks**:
- [ ] Research Surya model requirements and deployment
- [ ] Evaluate hardware requirements (GPU/CUDA) - RTX 2070 available
- [ ] Set up model inference service (Python microservice - **preferred approach**)
- [ ] Integrate SDO image data fetching
- [ ] Implement model inference pipeline
- [ ] Add solar flare prediction endpoints
- [ ] Create prediction storage and tracking
- [ ] Add prediction accuracy monitoring

**Deliverables**:
- Surya model integration (if hardware allows)
- Solar flare prediction API
- Prediction tracking system

**Note**: This is optional and depends on GPU availability. Can be deferred or implemented as separate service.

---

### Phase 11: Satellite Tracking & Orbital Decay

#### Step 11.1: TLE Data Integration
**Objective**: Integrate Two-Line Element (TLE) data for satellite tracking.

**Tasks**:
- [ ] Research TLE data sources (Space-Track, CelesTrak)
- [ ] Implement TLE data fetching and parsing
- [ ] Create database schema for satellite TLE data
- [ ] Implement TLE data storage and versioning
- [ ] Add satellite catalog endpoints
- [ ] Implement TLE update scheduling

**Deliverables**:
- TLE data integration
- Satellite catalog database
- API endpoints for satellite data

#### Step 11.2: Orbital Mechanics Calculations
**Objective**: Implement orbital mechanics calculations for satellite position and decay.

**Tasks**:
- [ ] Research orbital mechanics libraries (SGP4, Orekit bindings, or custom implementation)
- [ ] Implement satellite position calculation from TLE
- [ ] Implement orbital decay calculation
- [ ] Add atmospheric drag modeling (considering solar activity)
- [ ] Create calculation service module
- [ ] Add endpoints for satellite position queries
- [ ] Add endpoints for decay predictions

**Deliverables**:
- Orbital mechanics calculation engine
- Satellite position API
- Decay prediction API

#### Step 11.3: ML-Based Decay Prediction
**Objective**: Implement machine learning model for orbital decay prediction.

**Tasks**:
- [ ] Research physics-guided neural networks for orbital decay
- [ ] Collect historical TLE data for training
- [ ] Design model architecture (or use existing research)
- [ ] Implement model training pipeline (Python service - **preferred approach**)
- [ ] Integrate model with orbital mechanics calculations
- [ ] Add re-entry prediction endpoints
- [ ] Implement prediction accuracy tracking
- [ ] Add alerts for predicted re-entries

**Deliverables**:
- ML model for decay prediction
- Re-entry prediction API
- Alert system for re-entries

**Key Features**:
- Historical TLE data analysis
- Physics-guided neural network
- Solar activity correlation
- Re-entry time prediction

---

### Phase 12: Mars Weather Forecasting

#### Step 12.1: Mars Weather Data Integration
**Objective**: Integrate Mars weather data from various sources.

**Tasks**:
- [ ] Research Mars weather data sources (MEDA, REMS, Mars Climate Database)
- [ ] Implement PDS (Planetary Data System) data access
- [ ] Add data models for Mars weather (temperature, pressure, wind, dust)
- [ ] Create database schema for Mars weather data
- [ ] Implement data ingestion from multiple sources
- [ ] Add endpoints for Mars weather queries
- [ ] Implement data aggregation and averaging

**Deliverables**:
- Mars weather data integration
- Database schema for Mars weather
- API endpoints for Mars weather

**Data Sources**:
- MEDA (Perseverance rover)
- REMS (Curiosity rover)
- Mars Climate Database (MCD)
- OpenMARS dataset

#### Step 12.2: Mars Weather Forecasting Models
**Objective**: Implement machine learning models for Mars weather forecasting.

**Tasks**:
- [ ] Research TCN and TiDE architectures for Mars weather
- [ ] Obtain or generate OpenMARS dataset
- [ ] Implement data preprocessing pipeline
- [ ] Design model architecture (TCN for temperature/pressure)
- [ ] Implement model training (Python service - **preferred approach**)
- [ ] Add forecasting endpoints
- [ ] Implement forecast storage and tracking
- [ ] Add forecast accuracy evaluation

**Deliverables**:
- Mars weather forecasting models
- Forecast API endpoints
- Forecast tracking system

**Model Types**:
- TCN (Temporal Convolutional Network) for temperature/pressure
- Transformer/LSTM for dust optical depth
- CNN for wind stress patterns

#### Step 12.3: Dust Storm Prediction
**Objective**: Implement dust storm prediction and alerting.

**Tasks**:
- [ ] Research dust storm initiation patterns
- [ ] Implement dust devil tracking (if orbital imagery available)
- [ ] Add dust storm detection algorithms
- [ ] Integrate dust storm predictions with forecasting models
- [ ] Add dust storm alert endpoints
- [ ] Implement alert notification system
- [ ] Add historical dust storm analysis

**Deliverables**:
- Dust storm prediction system
- Alert endpoints
- Notification system

**Note**: Dust storm initiation prediction is still a research challenge, but we can implement detection and tracking of developing storms.

---

### Phase 13: Additional Integrations (Future Enhancements)

#### Step 13.1: Retrograde Motion Calculator
**Objective**: Add retrograde motion calculation capabilities.

**Tasks**:
- [ ] Research ephemeris libraries (Skyfield, Astropy bindings)
- [ ] Implement retrograde motion detection algorithm
- [ ] Add endpoints for retrograde calculations
- [ ] Add planetary position tracking

**Deliverables**:
- Retrograde motion calculator
- Planetary position API

#### Step 13.2: Real-Time Discovery Pipeline
**Objective**: Integrate real-time discovery alert streams.

**Tasks**:
- [ ] Research GCN (General Coordinates Network) integration
- [ ] Research Lasair broker for LSST alerts
- [ ] Implement alert stream processing
- [ ] Add watchlist functionality
- [ ] Implement cross-matching with catalogs

**Deliverables**:
- Real-time alert processing
- Watchlist system
- Discovery notification API

---

## 5. Technical Research & Reference

### NASA DONKI API

**Overview**: NASA Space Weather Database of Notifications, Knowledge, and Information provides comprehensive API for tracking space weather events.

**Key Endpoints**:
- **FLR (Solar Flares)**: `https://api.nasa.gov/DONKI/FLR`
  - Required: `startDate` (YYYY-MM-DD), `api_key`
  - Optional: `endDate` (defaults to today)
  - Returns: Array of solar flare events with classType, peakTime, beginTime, endTime, sourceLocation

- **CME (Coronal Mass Ejections)**: Tracks CME events and their properties
- **GST (Geomagnetic Storms)**: Geomagnetic storm data
- **IPS (Interplanetary Shocks)**: Shock front data
- **HSS (High Speed Streams)**: Solar wind speed data

**Rate Limits**:
- Registered API Key: 1,000 requests/hour
- DEMO_KEY: 30 requests/hour
- Headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`

**Authentication**: Free API key from https://api.nasa.gov

### Exoplanet Archive TAP Protocol

**Overview**: Table Access Protocol (TAP) service for programmatic access to exoplanet data using ADQL (Astronomical Data Query Language).

**Key Tables**:
- `ps` (Planetary Systems): Primary record of confirmed exoplanets
- `pscomppars` (Composite Parameters): Best-estimate data from multiple sources
- `cumulative`: KOI Cumulative List

**Implementation**: Requires TAP client with ADQL query support for server-side filtering.

### Mars Weather Data Sources

**MEDA (Perseverance Rover)**:
- Air temperature, ground temperature, humidity, wind velocity, atmospheric pressure
- Data archived in Planetary Data System (PDS)
- Derived data products include dust opacity and water vapor columns

**REMS (Curiosity Rover)**:
- Similar meteorological data to MEDA
- Long-term operational dataset

**Mars Climate Database (MCD)**:
- Three-dimensional spatial grid of Martian atmosphere
- Global Circulation Model (GCM) output
- Provides "weather map" for entire planet

**OpenMARS Dataset**:
- Reanalysis product merging spacecraft observations with GCM
- Used for training time-series forecasting models

### Machine Learning Approaches

**Solar Flare Prediction**:
- Surya foundation model (NASA/IBM collaboration)
- 366-million-parameter transformer
- Trained on 9 years of SDO imagery
- Provides 2-hour lead time predictions
- Hardware: CUDA-capable GPUs (RTX 2070 available)

**Mars Weather Forecasting**:
- TCN (Temporal Convolutional Network) for temperature/pressure
- TiDE architecture for short-term diurnal cycle prediction
- Transformer/LSTM for dust optical depth
- CNN for wind stress patterns

**Orbital Decay Prediction**:
- Physics-guided neural networks
- Historical TLE data analysis
- Solar activity correlation

---

## 6. Implementation Priorities

### High Priority (Core Functionality)
1. **Phase 10.1: NASA DONKI Integration** (Priority A - Solar Flare Data)
   - Immediate focus on FLR endpoint
   - Complete space weather data acquisition
   - See `DONKI_IMPLEMENTATION_ANALYSIS.md` for details

2. **Priority B: Web UI Development**
   - Simple HTML/JavaScript frontend
   - Display last 7 days of data
   - Query interface

3. **Phase 8: Testing & Quality Assurance**
   - Comprehensive test coverage
   - Documentation completion

4. **Phase 9: Deployment & Operations**
   - Production deployment preparation
   - Monitoring and observability

### Medium Priority (Enhanced Features)
5. **Priority C: ML Integration**
   - Python microservice setup
   - Simple prediction model
   - Data pipeline implementation

6. **Priority D: Predictions Display**
   - Web UI enhancements
   - Comparison visualizations

7. **Phase 10.2: Exoplanet Archive Integration**
8. **Phase 11.1-11.2: Satellite Tracking** (TLE + calculations)
9. **Phase 12.1: Mars Weather Data Integration**

### Lower Priority (Advanced ML)
10. **Phase 11.3: ML-Based Decay Prediction**
11. **Phase 12.2-12.3: Mars Weather Forecasting**
12. **Phase 10.3: Surya ML Model** (if hardware available)

### Future Enhancements
13. **Phase 13: Additional Integrations**

---

## 7. Technical Considerations

### Data Storage Strategy
- **Hot Tier**: Real-time JSON streams (Redis/cache)
- **Warm Tier**: Frequently queried data (MySQL/PostgreSQL)
- **Cold Tier**: Raw sensor data, images (file storage or object storage)
- **Storage Capacity**: 12 TB available, can acquire more

### API Rate Limits
- Register for NASA API keys (beyond DEMO_KEY)
- Implement rate limiting for external API calls
- Use caching to minimize API calls
- Monitor rate limit headers

### Machine Learning Deployment
- **Approach**: Python microservices (preferred for training/tuning practice)
- **Communication**: HTTP/gRPC between Rust API and Python service
- **Hardware**: RTX 2070 available for training
- **Storage**: 12 TB available for datasets

### Hardware Requirements
- **Current**: Standard server (CPU, RAM) - ✅ Available
- **For Surya**: GPU with CUDA support - ✅ RTX 2070 available
- **For ML Training**: GPU recommended - ✅ RTX 2070 available, can acquire more

---

## 8. Questions Resolved

### Hardware
**Q**: Do you have GPU access for ML models, or should we focus on CPU-based solutions?
**A**: Yes, I have an old NVIDIA GTX 960 and an NVIDIA RTX 2070 on my other machine. I can also acquire other 2070s or 2080s.

### ML Approach
**Q**: Python microservices vs Rust ML bindings?
**A**: Python please. I want to be able to train my models and get practice. That's a huge important step for me. I want to be able to tune.

### Data Volume
**Q**: How much historical data should we store?
**A**: I have 12 TB of HDD. I can acquire more.

### Priorities
**Q**: Which features are most important for your use case?
**A**: The first thing I want to prioritize is:
- A. Acquiring solar flare data and other space weather data.
- B. Creating a simple web page to interact with this data. I want to see solar data within a week and have a section to make queries.
- C. Creating the ML to start predictions
- D. Displaying predictions vs Actual on the web page
- E. Commencing on the Mars project, similar structure.

### Timeline
**Q**: What's the target timeline for these features?
**A**: See above. No deadlines, this is a personal project.

---

## 9. Next Steps

### Immediate Actions
1. ✅ **DONKI Account Setup** - User handling NASA API key registration
2. ⏳ **Implement DONKI Integration** - Priority A (solar flare data)
3. ⏳ **Build Web UI** - Priority B (simple HTML/JS frontend)
4. ⏳ **Set Up ML Infrastructure** - Priority C (after data complete)

### Short-term Goals (1-3 months)
- Complete Priority A (solar flare data acquisition)
- Complete Priority B (web UI)
- Begin Priority C (ML integration)
- Complete Phase 8 (testing)
- Complete Phase 9 (deployment)

### Long-term Vision
- Comprehensive space weather monitoring platform
- ML-powered predictions for solar flares and space weather
- Mars weather forecasting system
- Satellite tracking and orbital decay prediction
- Real-time discovery pipeline
- Full astronomical monitoring platform

---

## 10. Development Philosophy

- **Iterative**: Build step-by-step, ask before proceeding
- **Test-Driven**: Comprehensive testing at each step
- **Security-First**: Consider security implications throughout
- **Documentation**: Update README and OVERVIEW.md continuously
- **Simple First**: Core functionality before advanced features
- **Learning-Focused**: This is a personal project for learning and practice

---

**Note**: This plan is ambitious and can be implemented incrementally. Each phase can be completed independently, allowing for flexible development based on priorities and resources.

**Last Updated**: 2024
**Status**: Active Development
