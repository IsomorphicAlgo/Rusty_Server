# Rusty Server

A Rust-based REST API service for fetching, caching, and serving space weather data critical for satellite operations. This project complements **[Ephemerust](https://github.com/IsomorphicAlgo/Ephemerust)** (astronomy and satellite geometry library/CLI) by providing real-time and historical space weather information and hosted calculation APIs.

## Project Status

**Latest milestone**: Phases **1–6** (foundation through security), **Phase 8** (testing & QA), and **Phase 10.1–10.3** (DONKI solar flares, Exoplanet Archive, CPU ML) are **complete**. **Phase 9** (deployment & operations) is **not** started. **Web dashboard** is complete.

**Completed steps** (see [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) for full detail):
- ✅ 1.1–1.3 (Project structure, configuration, logging & errors)
- ✅ 2.1–2.3 (HTTP server, REST API structure, data models)
- ✅ 3.1–3.2 (NOAA integration, parsing & transformation)
- ✅ 4.1–4.3 (Database schema, operations, caching)
- ✅ 5.1–5.3 (Current conditions, historical data, alerts & radiation)
- ✅ 6.1–6.3 (Rate limiting, authentication, security hardening)
- ✅ 7.1 (Ephemerust integration planning)
- ✅ 8.1–8.2 (Comprehensive tests, documentation)
- ✅ 10.1 (NASA DONKI — solar flares / FLR)
- ✅ 10.2 (Exoplanet Archive / TAP)
- ✅ 10.3 (ML microservice — CPU / XGBoost)
- ✅ **Ephemerust path + MSRV** — `ephemerust` `path = "../Ephemerust"`, Rust **1.88** (`rust-toolchain.toml`, `Cargo.toml` `rust-version`; see [`EPHEMERUST_INTEGRATION_PLAN.md`](EPHEMERUST_INTEGRATION_PLAN.md) Phases 2–2.3)
- ✅ **Ephemerust doc alignment** (integration plan Phase 1 — product naming; sibling path `../Ephemerust` for Cargo `path`)

**Next (high priority)**:
1. **Phase 9 — Deployment & operations** — prod configs, systemd, backups, metrics/health, TLS/reverse proxy (see deployment section in the development plan).
2. **Phase 11 — Satellite tracking** — TLE ingestion, catalog persistence, and decay/ML layers (see development plan; propagation already available via **Ephemerust** through [`/api/v1/ephemeris/...`](Guides/API_EPHEMERIS.md)).

**Future roadmap**: Phase **11** (satellite / TLE / decay), **12** (Mars weather), **Surya** (GPU), and **Phase 13** extras are outlined in [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md).

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
- ✅ **Resilient Endpoint Handling**: Multiple fallback endpoints for plasma data (handles endpoint changes/deprecation)
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
- ✅ **Phase 8**: Broad test coverage (unit, integration, security), API and database guides
- ✅ **Ephemerust-backed ephemeris API**: `ephemerust` (path dependency) + **`POST /api/v1/ephemeris/time`**, **`/position`**, **`/satellite/track`** — see [`Guides/API_EPHEMERIS.md`](Guides/API_EPHEMERIS.md) and [`EPHEMERUST_INTEGRATION_PLAN.md`](EPHEMERUST_INTEGRATION_PLAN.md)
- ⏳ **Phase 9 (Deployment)**: Production deployment, monitoring, backups (see development plan)
- ⏳ **Surya Model Integration**: Host and integrate NASA/IBM Surya foundation model for advanced solar flare prediction (future)
- ⏳ **Satellite Tracking**: TLE data integration and orbital mechanics calculations (future)
- ⏳ **ML-Based Deorbit Prediction**: Machine learning algorithms for satellite re-entry prediction (future)

**📋 See [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) for the complete development plan.**  
**📚 See [OVERVIEW.md](OVERVIEW.md) for architecture and technical details.**  
**📖 See [Guides/](Guides/) for setup guides and instructions — including [Guides/API_EPHEMERIS.md](Guides/API_EPHEMERIS.md) (Ephemeris REST API contract + examples).**  
**🔧 See [Troubleshooting/](Troubleshooting/) for troubleshooting guides.**  
**🖥️ See [Guides/IPMI_SETUP_GUIDE.md](Guides/IPMI_SETUP_GUIDE.md) for remote server management (no monitor needed!).**

---

## Introduction

**Rusty Server** is a comprehensive astronomical data platform that serves as a centralized server infrastructure for space weather monitoring, exoplanet discovery tracking, and astronomical calculations. The project hosts multiple services and databases on a powerful home server, providing real-time data, historical archives, and predictive capabilities.

### Core Objectives

1. **Host Ephemerust-backed calculations**: Ephemerust is exposed via REST at **`/api/v1/ephemeris/...`** ([`Guides/API_EPHEMERIS.md`](Guides/API_EPHEMERIS.md), [`EPHEMERUST_INTEGRATION_PLAN.md`](EPHEMERUST_INTEGRATION_PLAN.md))
2. **Space Weather & Solar Flare Databases**: Maintain comprehensive databases for space weather data and solar flare events from NOAA and NASA DONKI
3. **Exoplanet Discovery Database**: Track and store exoplanet data from NASA's Exoplanet Archive
4. **Machine Learning Predictions**: Implement and host ML models for solar flare prediction, starting with CPU-optimized models and progressing to the Surya foundation model
5. **Satellite Deorbit Prediction** (Future): Calculate satellite orbital decay and predict re-entry times using machine learning algorithms

This project extends the **Ephemerust** ecosystem (formerly *CLI_Astro_Calc*) into a full server-based data and API platform.

## Use Cases

- **Satellite Operators**: Monitor space weather conditions and receive alerts for solar flares and geomagnetic storms
- **Mission Planning**: Access historical space weather patterns for mission planning
- **Astronomical Calculations**: Perform calculations via Ephemerust-backed HTTP APIs (see integration plan)
- **Exoplanet Research**: Query and analyze exoplanet discovery data
- **Space Weather Prediction**: Access ML-powered solar flare predictions with confidence scores
- **Satellite Tracking** (Future): Track satellite positions and predict orbital decay and re-entry times

## Features

### Implemented (summary)

The **axum** REST API, **NOAA** and **DONKI** ingestion, **Exoplanet Archive** TAP integration, **MySQL** storage, **moka** caching, **rate limiting** and **API key** auth, **web dashboard**, **CPU ML** predictions (Python microservice), and **Ephemerust-backed ephemeris** (`POST /api/v1/ephemeris/...`) are implemented. Details are in **Current Implementation** below and in [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md).

### Still planned or in progress

- **Phase 9** — deployment scripts, systemd, observability, production DB, TLS, backups
- **ML** — optional Surya / GPU path; tuning and richer prediction UI over time
- **Satellite catalog / TLE persistence / decay ML** (Phase 11 — propagation MVP already via **`/api/v1/ephemeris/satellite/track`**)
- **Mars weather** (Phase 12)
- **Phase 13** — e.g. retrograde calculator, real-time discovery streams

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

- Rust **1.88+** (required: matches **Ephemerust** and `rust-toolchain.toml` / `Cargo.toml` `rust-version`; see [`EPHEMERUST_INTEGRATION_PLAN.md`](EPHEMERUST_INTEGRATION_PLAN.md) Phase 2.3 and [Troubleshooting/BUILD_TROUBLESHOOTING.md](Troubleshooting/BUILD_TROUBLESHOOTING.md))
- **Ephemerust** as a **sibling repo** at `../Ephemerust` (same parent directory as this repo; e.g. `C:\Users\micha\Rust\Ephemerust` next to `C:\Users\micha\Rust\Rusty_Server` — required for the `ephemerust` path dependency; see integration plan Phase 0)
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

# Run a single integration test target (example: ephemeris, no MySQL required)
cargo test --test ephemeris_api_test
```

**Test Results**: 80+ tests passing (varies by target)
- 49+ unit tests (models, config, errors, logging, cache, auth, parsing, ephemeris handler unit test)
  - 7 authentication unit tests (API key generation, validation, middleware)
  - 9 DONKI parsing and client tests
- 20 API integration tests (current conditions, historical data, alerts, radiation)
- 5 rate limiting tests
- 2 health check tests
- 6 authentication integration tests (require database connection)
- 11 security tests (headers, CORS, request limits, SQL injection, XSS, path traversal, input validation)
- 10 ephemeris integration tests (`ephemeris_api_test`; Ephemerust-backed routes, lazy DB pool)

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

# Ephemeris (Ephemerust-backed): Julian date + GMST for an instant (no API key if auth.require_auth = false)
Invoke-WebRequest -Uri http://localhost:3000/api/v1/ephemeris/time -Method POST -ContentType "application/json" -Body '{"utc":"2000-01-01T12:00:00Z"}' -UseBasicParsing

# Same request with curl (bash / Git Bash on Windows)
curl -s -X POST http://localhost:3000/api/v1/ephemeris/time -H "Content-Type: application/json" -d "{\"utc\":\"2000-01-01T12:00:00Z\"}"

# Or use the test script (PowerShell)
.\scripts\test_space_weather.ps1

# Or use the test script (Linux/Mac)
./scripts/test_space_weather.sh

# Open web dashboard in browser
# Navigate to http://localhost:3000/
```

Full schemas, satellite `track` modes, limits, and errors: **[Guides/API_EPHEMERIS.md](Guides/API_EPHEMERIS.md)**.

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

- **Phase 9 — Deployment & operations**: Environment-specific configs, systemd, backups, metrics and enhanced health checks, production cutover with TLS (see [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) § Phase 9).
- **Ephemeris / Ephemerust**: **`/api/v1/ephemeris/...`** is live; see [Guides/API_EPHEMERIS.md](Guides/API_EPHEMERIS.md) and [`EPHEMERUST_INTEGRATION_PLAN.md`](EPHEMERUST_INTEGRATION_PLAN.md) Phase 5 for doc polish.
- **Enhanced ML predictions**: More training data and tuning for the CPU model.
- **Exoplanet discovery notifications**: Automatic notifications for newly synced or notable discoveries (see development plan periodic-sync notes).

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
- **Phase 9**: Deployment & operations
- **Phase 11**: Satellite tracking & orbital decay
- **Phase 12**: Mars weather forecasting
- **Surya / GPU**: [Guides/SURYA_ML_INTEGRATION_PLAN.md](Guides/SURYA_ML_INTEGRATION_PLAN.md)

## Contributing

This is a personal portfolio project. For questions or suggestions, please open an issue on GitHub.

## License

MIT License - see LICENSE file for details

## Acknowledgments

- NOAA Space Weather Prediction Center for data sources
- [Ephemerust](https://github.com/IsomorphicAlgo/Ephemerust) for inspiration and integration
- Rust community for excellent tooling and libraries

---

**Status**: Active development — Phase 10.3 + ephemeris API (Priority E MVP) complete; next: **Phase 9 (deployment)** and/or **Phase 11** (TLE catalog / persistence).  
**Last updated**: May 2026 — Ephemeris handlers live (`Guides/API_EPHEMERIS.md`); Phases 1–5 of [EPHEMERUST_INTEGRATION_PLAN.md](EPHEMERUST_INTEGRATION_PLAN.md) reflected in docs.  
**Version**: 0.1.0
