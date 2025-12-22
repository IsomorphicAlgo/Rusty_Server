# Rusty Server

A Rust-based REST API service for fetching, caching, and serving space weather data critical for satellite operations. This project complements the CLI_Astro_Calc project by providing real-time and historical space weather information.

## Project Status

**Current Phase**: Phase 8 - Testing & Quality Assurance ✅ COMPLETE  
**Completed Steps**: 
- ✅ 1.1 (Project Structure & Dependencies)
- ✅ 1.2 (Configuration System)
- ✅ 1.3 (Logging & Error Handling)
- ✅ 2.1 (Basic HTTP Server)
- ✅ 2.2 (REST API Structure)
- ✅ 2.3 (Data Models)
- ✅ 3.1 (NOAA API Integration)
- ✅ 3.2 (Data Parsing & Transformation)
- ✅ 4.1 (Database Schema & Setup)
- ✅ 4.2 (Database Operations)
- ✅ 4.3 (Caching Layer)
- ✅ 5.1 (Current Conditions Endpoint)
- ✅ 5.2 (Historical Data Endpoint)
- ✅ 5.3 (Alerts & Radiation Endpoints)
- ✅ 6.1 (Rate Limiting)
- ✅ 6.2 (Authentication & Authorization)
- ✅ 6.3 (Security Hardening)
- ✅ 7.1 (CLI Integration Planning)
- ✅ 10.1 (NASA DONKI Integration - Solar Flares)
- ✅ 10.2 (Exoplanet Archive Integration)
- ✅ 10.3 (ML Model Integration - CPU-based)

**Next Step**: Phase 9 (Deployment & Operations) ✅ **Web UI Complete**

### What's Been Completed

- ✅ **Project Structure**: Cargo project initialized with proper module structure
- ✅ **Dependencies**: All required dependencies configured (axum, sqlx, reqwest, etc.)
- ✅ **Configuration System**: Complete configuration management with environment variable support
- ✅ **Error Handling**: Custom error types with status codes, logging, and Result extension traits
- ✅ **Logging System**: Structured logging with tracing (pretty and JSON formats)
- ✅ **HTTP Middleware**: Request/response logging middleware ready for HTTP server
- ✅ **Module Structure**: All core modules created and organized
- ✅ **Documentation**: README, OVERVIEW.md, and ITERATIVE_PLAN.md created
- ✅ **Tests**: Comprehensive tests for configuration, logging, and error handling
- ✅ **NOAA API Integration**: HTTP client with retry logic, error handling, and data fetching
- ✅ **Service Layer**: NoaaClient service for fetching space weather data from NOAA
- ✅ **Application State**: State management for sharing services across handlers
- ✅ **Data Parsing Module**: Dedicated parsing module with validation and error handling
- ✅ **Data Transformation**: Robust parsing of NOAA JSON responses to internal models
- ✅ **Data Validation**: Comprehensive validation for parsed space weather data
- ✅ **Database Schema**: Complete MySQL schema designed for 10+ years of historical data
- ✅ **Database Connection Pool**: Connection pooling with health checks and migration system
- ✅ **Database Operations**: Complete CRUD operations with transactions, batch operations, and query optimization
- ✅ **API-Database Integration**: Handlers now store and retrieve data from database
- ✅ **Data Persistence**: Observations automatically stored when fetched from NOAA API
- ✅ **Historical Data Queries**: Full support for date range and type-based queries
- ✅ **Caching Layer**: High-performance in-memory caching with moka (TTL-based expiration, metrics tracking)
- ✅ **Cache Integration**: All API handlers use cache to reduce API calls and improve response times
- ✅ **Current Conditions Endpoint**: Fully implemented with cache → API → database → mock fallback chain
- ✅ **Historical Data Endpoint**: Fully implemented with query parameters, pagination, date validation, and caching
- ✅ **Alerts Endpoint**: Fully implemented with filtering (severity, type, active_only) and caching
- ✅ **Radiation Endpoint**: Fully implemented with threshold and alert level filtering
- ✅ **Enhanced Logging**: Comprehensive logging for all endpoint operations with proper log levels
- ✅ **Rate Limiting**: Per-IP rate limiting using token bucket algorithm (governor crate)
- ✅ **DONKI Integration**: NASA DONKI API client for solar flare data (FLR endpoint)
- ✅ **Solar Flare Data**: Real solar flare data from DONKI integrated into current conditions endpoint
- ✅ **Exoplanet Archive Integration**: NASA Exoplanet Archive TAP client with ADQL query support
- ✅ **Exoplanet Data Models**: Complete data models for planetary systems and composite parameters
- ✅ **Exoplanet Database Schema**: Database tables for storing exoplanet data and discovery notifications
- ✅ **Exoplanet API Endpoints**: REST endpoints for querying exoplanets with filtering and pagination
- ✅ **ML Service Integration**: Python microservice for CPU-based solar flare prediction
- ✅ **XGBoost Model**: CPU-optimized model for solar flare prediction
- ✅ **Prediction Endpoints**: API endpoints for solar flare predictions with accuracy tracking
- ✅ **Model Training Pipeline**: Scripts for training models on historical data
- ✅ **Web Dashboard**: Interactive web page displaying project summary and latest data (weather, solar, exoplanet)
- ✅ **Data Refresh Endpoint**: API endpoint to force fresh data fetch and database storage
- ✅ **Latest Exoplanet Endpoint**: API endpoint to retrieve the most recently synced exoplanet
- ⏳ **CLI_Astro_Calc Server Integration**: Host CLI_Astro_Calc as server-based service (planned)
- ⏳ **Surya Model Integration**: Host and integrate NASA/IBM Surya foundation model for advanced solar flare prediction (future)
- ⏳ **Satellite Tracking**: TLE data integration and orbital mechanics calculations (future)
- ⏳ **ML-Based Deorbit Prediction**: Machine learning algorithms for satellite re-entry prediction (future)

**📋 See [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) for the complete development plan.**  
**📚 See [OVERVIEW.md](OVERVIEW.md) for architecture and technical details.**  
**📖 See [Guides/](Guides/) for setup guides, API documentation, and detailed instructions.**  
**🔧 See [Troubleshooting/](Troubleshooting/) for troubleshooting guides.**  
**🖥️ See [Guides/IPMI_SETUP_GUIDE.md](Guides/IPMI_SETUP_GUIDE.md) for remote server management (no monitor needed!).**

---

## Introduction

**Rusty Server** is a comprehensive astronomical data platform that serves as a centralized server infrastructure for space weather monitoring, exoplanet discovery tracking, and astronomical calculations. The project hosts multiple services and databases on a powerful home server, providing real-time data, historical archives, and predictive capabilities.

### Core Objectives

1. **Host CLI_Astro_Calc**: Serve the CLI_Astro_Calc tool as a server-based service, making astronomical calculations accessible via API
2. **Space Weather & Solar Flare Databases**: Maintain comprehensive databases for space weather data and solar flare events from NOAA and NASA DONKI
3. **Exoplanet Discovery Database**: Track and store exoplanet data from NASA's Exoplanet Archive
4. **Machine Learning Predictions**: Implement and host ML models for solar flare prediction, starting with CPU-optimized models and progressing to the Surya foundation model
5. **Satellite Deorbit Prediction** (Future): Calculate satellite orbital decay and predict re-entry times using machine learning algorithms

This project is a continuation of the CLI_Astro_Calc Project, expanding it into a full server-based platform.

## Use Cases

- **Satellite Operators**: Monitor space weather conditions and receive alerts for solar flares and geomagnetic storms
- **Mission Planning**: Access historical space weather patterns for mission planning
- **Astronomical Calculations**: Perform complex astronomical calculations via server-hosted CLI_Astro_Calc
- **Exoplanet Research**: Query and analyze exoplanet discovery data
- **Space Weather Prediction**: Access ML-powered solar flare predictions with confidence scores
- **Satellite Tracking** (Future): Track satellite positions and predict orbital decay and re-entry times

## Features

### Planned Features

- **Rust-Based REST API**: Modern async web service using axum
- **CLI_Astro_Calc Server**: Host astronomical calculation tool as server-based service
- **Data Fetching**: Integration with NOAA Space Weather API and NASA DONKI API
- **Exoplanet Database**: Integration with NASA Exoplanet Archive via TAP protocol
- **Local Caching**: Reduce API calls and improve response times
- **REST Endpoints**: Clean API for querying current conditions, historical data, and exoplanets
- **Data Storage**: Historical data storage in MySQL (space weather, solar flares, exoplanets)
- **Machine Learning**: CPU-optimized solar flare prediction models (XGBoost), future Surya integration
- **Satellite Tracking** (Future): TLE data integration, orbital mechanics calculations, ML-based deorbit prediction
- **Production Features**: Rate limiting and authentication
- **Deployment**: Self-hosted on personal server rack

### Current Implementation

- ✅ Configuration management (environment variables + config files)
- ✅ Error handling system with status codes and logging
- ✅ Structured logging (pretty and JSON formats)
- ✅ Request/response logging middleware
- ✅ HTTP server with health check endpoints
- ✅ Graceful shutdown handling
- ✅ REST API endpoints (current, historical, alerts, radiation)
- ✅ Request/response models for space weather data
- ✅ Data validation system (14 validation tests passing)
- ✅ Mock data handlers (ready for real data integration)
- ✅ Module structure for all components
- ✅ Database integration with MySQL
- ✅ NOAA API integration with retry logic
- ✅ Caching layer with TTL management and metrics
- ✅ Rate limiting with per-IP token bucket algorithm
- ✅ API key authentication with configurable requirement
- ✅ API key management endpoints
- ✅ Security hardening (CORS, security headers, request size limits)
- ✅ Security logging for authentication failures
- ✅ NASA DONKI API integration for solar flare data
- ✅ Solar flare data automatically included in current conditions endpoint
- ✅ NASA Exoplanet Archive TAP integration for exoplanet data
- ✅ Exoplanet query endpoints with filtering, sorting, and pagination
- ✅ Python ML microservice for solar flare prediction (CPU-optimized)
- ✅ Solar flare prediction API with confidence scores
- ✅ Prediction accuracy tracking and monitoring
- ✅ Web dashboard with real-time data display
- ✅ Interactive refresh button to fetch fresh data from APIs

## Technology Stack

- **Web Framework**: [axum](https://github.com/tokio-rs/axum) (modern async Rust)
- **Database**: MySQL with [sqlx](https://github.com/launchbadge/sqlx)
- **HTTP Client**: [reqwest](https://github.com/seanmonstar/reqwest)
- **Logging**: [tracing](https://github.com/tokio-rs/tracing)
- **Serialization**: [serde](https://serde.rs/)
- **Configuration**: [config](https://github.com/mehcode/config-rs)

## Development Environment

### Prerequisites

- Rust toolchain (latest stable)
- MySQL (for database) - **See [Guides/MYSQL_SETUP_GUIDE.md](Guides/MYSQL_SETUP_GUIDE.md)**
- Git (for version control)

### Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/MY_USERNAME/Rusty_Server.git
   cd Rusty_Server
   ```

2. **Set up MySQL** (for local development):
   - **📋 See [Guides/MYSQL_SETUP_GUIDE.md](Guides/MYSQL_SETUP_GUIDE.md) for detailed instructions**
   - Create a MySQL user and database
   - Fill out `credentials.txt` with my MySQL credentials

3. **Configure the application**:
   ```bash
   # Copy example config
   cp config.example.toml config.toml
   
   # Edit config.toml with my settings
   # Or use environment variables (see Configuration section)
   ```

4. **Set up credentials**:
   ```bash
   # Copy credentials template
   cp credentials.example.txt credentials.txt
   
   # Edit credentials.txt with my MySQL username, password, etc.
   # This file is gitignored - my secrets are safe!
   ```

5. **Build and run**:
   ```bash
   cargo build
   cargo run
   ```

**📋 For server deployment (much later), see [Guides/SERVER_DEPLOYMENT_NOTES.md](Guides/SERVER_DEPLOYMENT_NOTES.md)**

## Configuration

Configuration can be provided via:
1. **Environment variables** (highest priority)
2. **Config file** (`config.toml` or path in `CONFIG_FILE`)
3. **Defaults** (built-in sensible defaults)

### Environment Variable Format

Use double underscore (`__`) for nested configuration:
```bash
RUSTY_SERVER__SERVER__PORT=3000
RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://user:password@localhost/rusty_server
RUSTY_SERVER__NOAA__BASE_URL=https://services.swpc.noaa.gov
```

### Configuration Options

- **Server**: Host, port
- **Database**: Connection string, max connections
- **NOAA API**: Base URL, API key (optional), timeout
- **DONKI API**: Base URL, API key (required), timeout
- **Exoplanet Archive**: Base URL, timeout (TAP service, no API key required)
- **ML Service**: Base URL, timeout, enabled flag (Python microservice)
- **ML Service**: Base URL, timeout, enabled flag (Python microservice)
- **Cache**: TTL values, size limits
- **Rate Limiting**: Requests per minute/hour, burst size
- **Authentication**: JWT secret, token expiry, require auth
- **Logging**: Level (trace/debug/info/warn/error), format (pretty/json)

See `config.example.toml` for complete configuration options.

## Project Structure

```
Rusty_Server/
├── src/
│   ├── main.rs          # Application entry point
│   ├── lib.rs           # Library root, public API
│   ├── api/             # REST API handlers
│   ├── services/        # Business logic
│   ├── models/          # Data models
│   ├── database/        # Database operations
│   ├── cache/           # Caching layer
│   ├── config/          # Configuration management
│   ├── auth/            # Authentication & authorization
│   └── errors/          # Error types
├── config.example.toml  # Example configuration file
├── Cargo.toml           # Rust project manifest
├── README.md            # This file
├── OVERVIEW.md          # Architecture and technical overview
└── DEVELOPMENT_PLAN.md  # Development plan
```

## Development Workflow

### Local Development (Windows Laptop)

1. Develop and test code locally
2. Use local MySQL instance or connect to server's MySQL
3. Test API endpoints on localhost
4. Run tests: `cargo test`

### Deployment (Linux Server)

**📋 See [Guides/SERVER_DEPLOYMENT_NOTES.md](Guides/SERVER_DEPLOYMENT_NOTES.md) for complete step-by-step deployment instructions.**

Quick overview:
1. Install Linux OS (Ubuntu 22.04 LTS recommended)
2. Install Rust toolchain and MySQL
3. Build Linux binary (`cargo build --release`)
4. Set up production database
5. Configure application (config.toml or environment variables)
6. Set up as systemd service (see `scripts/rusty-server.service`)
7. Configure firewall and network access
8. Optional: Set up nginx reverse proxy and SSL/TLS

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test
```

**Test Results**: 70+ tests passing
- 48 unit tests (models, config, errors, logging, cache, auth, parsing)
  - 7 authentication unit tests (API key generation, validation, middleware)
  - 9 DONKI parsing and client tests
- 20 API integration tests (current conditions, historical data, alerts, radiation)
- 5 rate limiting tests
- 2 health check tests
- 6 authentication integration tests (require database connection)
- 11 security tests (headers, CORS, request limits, SQL injection, XSS, path traversal, input validation)

### Testing the API

**📋 See [Guides/QUICK_TEST_GUIDE.md](Guides/QUICK_TEST_GUIDE.md) for complete testing instructions**, including:
- Local testing
- Testing from another device on my WiFi
- Windows Firewall configuration
- Troubleshooting tips

**Quick Start:**
```bash
# Start the server
cargo run

# Test health endpoint (in another terminal)
Invoke-WebRequest -Uri http://localhost:3000/health -UseBasicParsing

# Open web dashboard in browser
# Navigate to http://localhost:3000/
```

## Web Dashboard

The server includes a web dashboard accessible at the root path (`/`). The dashboard displays:

- **Project Summary**: Overview of the Rusty Server project
- **Space Weather Data**: Latest KP index, geomagnetic storms, solar wind, and radiation levels
- **Solar Activity**: Recent solar flares, solar wind Bz component
- **Exoplanet Data**: Most recently synced exoplanet information

### Features

- **Auto-refresh on load**: Data is automatically loaded when you open the page
- **Manual refresh**: Click the "Refresh Data" button to fetch fresh data from APIs
- **Database integration**: All refreshed data is automatically stored in the database
- **Real-time updates**: Data timestamps show when each dataset was last updated

### API Endpoints Used

The web dashboard uses the following API endpoints:
- `GET /api/v1/refresh` - Fetches fresh data from all APIs and stores in database
- The refresh endpoint returns both space weather and exoplanet data in a single response

## Security Considerations

- ✅ **Credential Management**: All sensitive files are gitignored
- ✅ **Password Masking**: Passwords are automatically masked in logs
- ✅ **Environment Variables**: Secure credential storage via environment variables
- ✅ **Never commit secrets**: All credential files are gitignored
- ✅ Parameterized database queries (SQL injection prevention)
- ✅ Input validation on all endpoints
- ✅ Proper error handling (don't leak sensitive info)
- ✅ Rate limiting (per-IP token bucket algorithm)
- ✅ Authentication/authorization (API key-based authentication)
- ✅ API key management (generate, list, revoke)
- ✅ Configurable authentication requirement
- ✅ Security headers (HSTS, CSP, X-Frame-Options, X-Content-Type-Options, etc.)
- ✅ CORS configuration
- ✅ Request size limits
- ✅ Security logging

**📋 See [SECURITY.md](SECURITY.md) for detailed security guidelines and credential management.**  
**📋 See [Troubleshooting/BUILD_TROUBLESHOOTING.md](Troubleshooting/BUILD_TROUBLESHOOTING.md) for build issues.**

## Troubleshooting

### Build Errors

If I encounter linker errors (LNK1104), this is often caused by antivirus software. See [Troubleshooting/BUILD_TROUBLESHOOTING.md](Troubleshooting/BUILD_TROUBLESHOOTING.md) for solutions.

**Quick fix:**
1. Add `target/` folder to antivirus exclusions
2. Restart my computer
3. Run `cargo clean && cargo check`

## Future Development Goals

### Short-Term Goals

- **CLI_Astro_Calc Server Integration**: Host the CLI_Astro_Calc tool as a server-based service, making astronomical calculations accessible via REST API
- **Enhanced ML Predictions**: Improve solar flare prediction accuracy with additional training data and model tuning
- **Exoplanet Discovery Notifications**: Implement automatic notifications for newly discovered exoplanets

### Long-Term Goals

- **Surya Foundation Model Integration**: Host and integrate the NASA/IBM Surya foundation model (366-million-parameter transformer) for advanced solar flare prediction. This will require GPU hardware for optimal performance.
- **Satellite Tracking System**: 
  - Integrate Two-Line Element (TLE) data from Space-Track or CelesTrak
  - Implement orbital mechanics calculations (SGP4) for satellite position tracking
  - Calculate orbital decay rates based on atmospheric drag and solar activity
- **ML-Based Deorbit Prediction**: 
  - Develop machine learning algorithms to predict satellite re-entry times
  - Use physics-guided neural networks trained on historical TLE data
  - Account for solar activity effects on atmospheric density
  - Provide uncertainty estimates for predictions
- **Mars Weather Forecasting**: Expand platform to include Mars weather data and forecasting models

### Technical Roadmap

See [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) for detailed implementation plans, including:
- Phase 11: Satellite Tracking & Orbital Decay
- Phase 12: Mars Weather Forecasting
- Surya ML Integration details in `Guides/SURYA_ML_INTEGRATION_PLAN.md`

## Contributing

This is a personal portfolio project. For questions or suggestions, please open an issue on GitHub.

## License

MIT License - see LICENSE file for details

## Acknowledgments

- NOAA Space Weather Prediction Center for data sources
- CLI_Astro_Calc project for inspiration and integration
- Rust community for excellent tooling and libraries

---

**Status**: Active Development  
**Last Updated**: 2024  
**Version**: 0.1.0
