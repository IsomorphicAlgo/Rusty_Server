# Rusty Server - Iterative Development Plan

## Project Overview

**Rusty_Server** is a Rust-based REST API service for fetching, caching, and serving space weather data. This project complements the CLI_Astro_Calc project by providing real-time and historical space weather information for satellite operations.

### Project Goals
- Build a production-ready REST API service for space weather data
- Integrate with NOAA Space Weather API and similar sources
- Provide local caching and historical data storage
- Implement security features (rate limiting, authentication)
- Deploy on personal server rack hardware
- Showcase for portfolio (GitHub/LinkedIn)

### Hardware Context
- **CPU**: 2x 8-core 8-thread Xeon processors
- **Memory**: 32GB DDR4 ECC
- **Storage**: SAS3 12-drive backplane
- **Network**: 4x 10G RJ45 ports
- **Management**: IPMI
- **Power**: Redundant 800W PSUs

---

## Development & Deployment Workflow

### Development Environment (Windows Laptop)
- **Primary Development**: Write and test code on your Windows laptop
- **Local Testing**: Run the API server locally (localhost) for development
- **Database Options**:
  - Option A: Install MySQL locally on Windows for development
  - Option B: Connect to MySQL on your server (if network accessible)
  - Option C: Use Docker with MySQL container (consistent environment)
- **Testing**: Run all unit tests, integration tests locally
- **Version Control**: Use git to manage code

### Deployment Environment (Linux Server Rack)
- **Target Platform**: Linux (distribution TBD)
- **Build Process**:
  - Option A: Cross-compile from Windows to Linux (using `cargo build --target x86_64-unknown-linux-gnu`)
  - Option B: Build directly on Linux server (SSH in and build)
  - Option C: Use Docker for consistent builds
- **Database**: Production MySQL instance on server
- **Service Management**: systemd service or similar
- **Network**: Accessible via your 10G network ports

### Recommended Workflow
1. **Develop** on Windows laptop
2. **Test** locally with local or remote MySQL
3. **Commit** code to git repository
4. **Deploy** to Linux server (build and run)
5. **Monitor** production service

**Note**: We'll set up configuration management to easily switch between dev and prod environments.

---

## Development Philosophy

- **Iterative**: Build step-by-step, ask before proceeding
- **Test-Driven**: Comprehensive testing at each step
- **Security-First**: Consider security implications throughout
- **Documentation**: Update README and create/update OVERVIEW.md continuously
- **Simple First**: Core functionality before advanced features

---

## Phase 1: Project Foundation & Setup

### Step 1.1: Project Structure & Dependencies ✅ (Ready to Start)

**Objective**: Set up the Rust project structure and initial dependencies.

**Tasks**:
- [ ] Initialize Cargo project with proper metadata
- [ ] Set up project structure (src/, tests/, docs/)
- [ ] Add initial dependencies:
  - Web framework: `axum` or `actix-web` (recommend `axum` for modern async)
  - HTTP client: `reqwest` (for NOAA API calls)
  - Database: `sqlx` with MySQL support (you have MySQL installed)
  - Serialization: `serde` with `serde_json`
  - Logging: `tracing` + `tracing-subscriber` (modern Rust logging)
  - Error handling: `anyhow` + `thiserror`
  - Configuration: `config` or `serde` with env vars
  - Time handling: `chrono` (consistent with CLI_Astro_Calc)
  - Rate limiting: `governor` or `dashmap` + custom
  - Authentication: `jsonwebtoken` + `argon2` (password hashing)
- [ ] Create basic module structure:
  ```
  src/
  ├── main.rs          # Application entry point
  ├── lib.rs           # Library root, public API
  ├── api/             # REST API handlers
  ├── services/        # Business logic
  ├── models/          # Data models
  ├── database/        # Database operations
  ├── cache/           # Caching layer
  ├── config/          # Configuration management
  ├── auth/            # Authentication & authorization
  └── errors/          # Error types
  ```
- [ ] Set up `.gitignore` (if using git)
- [ ] Create initial `README.md` structure
- [ ] Create `OVERVIEW.md` template

**Deliverables**:
- Working Cargo project that compiles
- Basic module structure in place
- Dependencies configured

**Decisions Made**:
- ✅ **Web Framework**: `axum` (modern async Rust) - following recommendation
- ✅ **Database**: MySQL (you have it installed) - we'll use `sqlx` with MySQL support
- ✅ **Development**: Windows laptop for development, Linux server for deployment

**Development Workflow Answer**:
Yes! You can absolutely develop on your Windows laptop and deploy to your Linux server rack. Here's the recommended approach:

1. **Development on Windows Laptop**:
   - Write and test code locally
   - Use local MySQL instance or connect to server's MySQL (if accessible)
   - Test API endpoints locally (localhost)
   - Run all tests and development tools

2. **Deployment to Linux Server**:
   - Build for Linux target (cross-compilation or build on server)
   - Deploy binary/service to server rack
   - Configure production MySQL on server
   - Set up as systemd service or similar

3. **Best Practices**:
   - Use environment variables for different configs (dev vs prod)
   - Test on Linux before final deployment (can use WSL2 or VM)
   - Use git to sync code between laptop and server
   - Consider Docker for consistent deployment (optional)

**Note**: Rust compiles to native binaries, so we'll build a Linux binary for your server. We can cross-compile from Windows or build directly on the Linux server.

---

### Step 1.2: Configuration System

**Objective**: Implement configuration management for the service.

**Tasks**:
- [ ] Design configuration structure:
  - Server settings (host, port)
  - Database connection string
  - NOAA API endpoints and keys
  - Cache settings (TTL, size limits)
  - Rate limiting configuration
  - Authentication settings (JWT secret, token expiry)
  - Logging configuration
- [ ] Implement config loading (env vars + config file)
- [ ] Add configuration validation
- [ ] Create example config file
- [ ] Add unit tests for configuration

**Deliverables**:
- Configuration module that loads settings from env vars and/or config file
- Example configuration file
- Tests for configuration loading

---

### Step 1.3: Logging & Error Handling ✅ (Completed)

**Objective**: Set up comprehensive logging and error handling systems.

**Tasks**:
- [x] Set up `tracing` with structured logging
- [x] Configure log levels (DEBUG, INFO, WARN, ERROR)
- [x] Add request/response logging middleware
- [x] Create custom error types (`thiserror`)
- [x] Implement error conversion and formatting
- [x] Add error logging with context
- [x] Create logging tests

**Deliverables**:
- ✅ Logging system that outputs structured logs (pretty and JSON formats)
- ✅ Error types for all major error cases with status codes
- ✅ Request/response logging middleware for HTTP requests
- ✅ Error logging utilities with context
- ✅ Result extension traits for convenient error logging
- ✅ Comprehensive tests for error handling

---

## Phase 2: Core API Infrastructure

### Step 2.1: Basic HTTP Server

**Objective**: Create a minimal HTTP server with health check endpoint.

**Tasks**:
- [ ] Set up web framework (axum/actix-web)
- [ ] Create basic server structure
- [ ] Implement health check endpoint (`GET /health`)
- [ ] Add graceful shutdown handling
- [ ] Create integration tests for health endpoint
- [ ] Add server startup logging

**Deliverables**:
- HTTP server that starts and responds to health checks
- Graceful shutdown handling
- Basic integration tests

---

### Step 2.2: REST API Structure

**Objective**: Define and implement the REST API structure.

**Tasks**:
- [ ] Design API endpoints:
  - `GET /api/v1/space-weather/current` - Current conditions
  - `GET /api/v1/space-weather/historical` - Historical data
  - `GET /api/v1/space-weather/alerts` - Active alerts
  - `GET /api/v1/space-weather/radiation` - Radiation levels
  - `GET /api/v1/health` - Health check
- [ ] Create request/response models (`serde`)
- [ ] Implement basic endpoint handlers (return mock data initially)
- [ ] Add API versioning
- [ ] Create OpenAPI/Swagger documentation (optional but recommended)
- [ ] Add endpoint tests

**Deliverables**:
- API structure with all planned endpoints
- Request/response models
- Mock data handlers (to be replaced with real data in Phase 3)

---

### Step 2.3: Data Models

**Objective**: Define data models for space weather data.

**Tasks**:
- [ ] Research NOAA Space Weather API data structure
- [ ] Create Rust structs for space weather data:
  - Solar flare data
  - Geomagnetic storm data
  - Radiation levels
  - Solar wind data
  - KP index
  - Other relevant metrics
- [ ] Add serialization/deserialization (`serde`)
- [ ] Add validation for data models
- [ ] Create unit tests for models

**Deliverables**:
- Complete data models matching NOAA API structure
- Serialization/deserialization working
- Model validation tests

---

## Phase 3: Data Fetching & Integration

### Step 3.1: NOAA API Integration

**Objective**: Integrate with NOAA Space Weather API.

**Tasks**:
- [ ] Research NOAA Space Weather API endpoints:
  - Current conditions endpoints
  - Historical data endpoints
  - Alert endpoints
  - Documentation review
- [ ] Implement HTTP client for NOAA API
- [ ] Add API key management (if required)
- [ ] Create service layer for fetching data
- [ ] Implement error handling for API failures
- [ ] Add retry logic with exponential backoff
- [ ] Create integration tests (with mock HTTP server)
- [ ] Add rate limiting for NOAA API calls

**Deliverables**:
- Working NOAA API integration
- Service layer that fetches real space weather data
- Error handling and retry logic
- Tests (unit + integration with mocks)

**Questions to Ask**:
- Do you have a NOAA API key, or is the data publicly available?
- What specific NOAA endpoints should we prioritize?

---

### Step 3.2: Data Parsing & Transformation

**Objective**: Parse and transform NOAA API responses into our data models.

**Tasks**:
- [ ] Implement parsers for NOAA API responses
- [ ] Handle different data formats (JSON, XML if applicable)
- [ ] Transform NOAA data to our internal models
- [ ] Add data validation
- [ ] Handle missing or malformed data gracefully
- [ ] Add parsing tests with real API responses (saved samples)

**Deliverables**:
- Parser that converts NOAA responses to our models
- Data transformation layer
- Comprehensive parsing tests

---

## Phase 4: Data Storage & Caching

### Step 4.1: Database Schema & Setup

**Objective**: Design and implement database schema for historical data.

**Tasks**:
- [ ] Design database schema:
  - Space weather data table(s)
  - Timestamps and indexing
  - Data relationships
- [ ] Set up MySQL database (using `sqlx` with MySQL support)
- [ ] Set up database connection pool
- [ ] Create migration system (sqlx migrations or custom)
- [ ] Implement database initialization
- [ ] Add database health checks
- [ ] Create schema documentation

**Deliverables**:
- Database schema designed and implemented
- Migration system in place
- Database connection pool configured

**Decisions Made**:
- ✅ **Database**: MySQL (using `sqlx` with MySQL support)

**Questions to Ask**:
- How much historical data do you want to store? (affects schema design)
- Will MySQL run on your laptop for development, or only on the server?

---

### Step 4.2: Database Operations

**Objective**: Implement database operations for storing and retrieving data.

**Tasks**:
- [ ] Implement data insertion operations
- [ ] Implement data retrieval operations (by date range, by type)
- [ ] Add database query optimization (indexes)
- [ ] Implement data cleanup/archival (optional)
- [ ] Add transaction handling
- [ ] Create database operation tests
- [ ] Add database error handling

**Deliverables**:
- Complete database operations layer
- CRUD operations for space weather data
- Database tests

---

### Step 4.3: Caching Layer

**Objective**: Implement in-memory caching to reduce API calls and improve response times.

**Tasks**:
- [ ] Choose caching strategy (in-memory HashMap, `dashmap`, or `moka`)
- [ ] Implement cache structure:
  - Current conditions cache (short TTL, e.g., 5-15 minutes)
  - Historical data cache (longer TTL, e.g., 1 hour)
  - Alert cache (short TTL, e.g., 1-5 minutes)
- [ ] Add cache TTL management
- [ ] Implement cache invalidation
- [ ] Add cache metrics (hit/miss rates)
- [ ] Create cache tests
- [ ] Add cache size limits

**Deliverables**:
- Working caching layer
- TTL management
- Cache metrics and monitoring

---

## Phase 5: API Implementation

### Step 5.1: Current Conditions Endpoint

**Objective**: Implement `GET /api/v1/space-weather/current`.

**Tasks**:
- [ ] Check cache for current data
- [ ] If cache miss, fetch from NOAA API
- [ ] Store in cache and database
- [ ] Return formatted response
- [ ] Add error handling
- [ ] Add response logging
- [ ] Create endpoint tests

**Deliverables**:
- Working current conditions endpoint
- Caching and database storage
- Comprehensive tests

---

### Step 5.2: Historical Data Endpoint

**Objective**: Implement `GET /api/v1/space-weather/historical`.

**Tasks**:
- [ ] Design query parameters (date range, data type, etc.)
- [ ] Implement database query for historical data
- [ ] Add pagination if needed
- [ ] Handle date range validation
- [ ] Add cache for recent historical queries
- [ ] Return formatted response
- [ ] Create endpoint tests

**Deliverables**:
- Working historical data endpoint
- Query parameter handling
- Pagination (if needed)
- Tests

---

### Step 5.3: Alerts & Radiation Endpoints

**Objective**: Implement alerts and radiation monitoring endpoints.

**Tasks**:
- [ ] Implement `GET /api/v1/space-weather/alerts`
- [ ] Implement `GET /api/v1/space-weather/radiation`
- [ ] Add alert filtering (by severity, type)
- [ ] Add radiation level thresholds
- [ ] Create endpoint tests

**Deliverables**:
- Working alerts endpoint
- Working radiation endpoint
- Filtering and query capabilities
- Tests

---

## Phase 6: Security & Production Features

### Step 6.1: Rate Limiting

**Objective**: Implement rate limiting to prevent API abuse.

**Tasks**:
- [ ] Choose rate limiting strategy (token bucket, sliding window)
- [ ] Implement rate limiting middleware
- [ ] Add per-IP rate limiting
- [ ] Add per-API-key rate limiting (if using API keys)
- [ ] Configure rate limits (requests per minute/hour)
- [ ] Add rate limit headers to responses
- [ ] Create rate limiting tests

**Deliverables**:
- Rate limiting middleware
- Per-IP and per-key rate limiting
- Rate limit headers
- Tests

---

### Step 6.2: Authentication & Authorization

**Objective**: Implement authentication for protected endpoints.

**Tasks**:
- [ ] Design authentication strategy (JWT tokens)
- [ ] Implement user registration/login (if needed)
- [ ] Implement API key generation (alternative to JWT)
- [ ] Add password hashing (`argon2`)
- [ ] Create JWT token generation and validation
- [ ] Add authentication middleware
- [ ] Protect sensitive endpoints
- [ ] Add token refresh mechanism
- [ ] Create authentication tests

**Deliverables**:
- Authentication system (JWT or API keys)
- Protected endpoints
- Token management
- Security tests

**Questions to Ask**:
- Do you want user accounts, or just API keys?
- Should all endpoints require authentication, or only some?

---

### Step 6.3: Security Hardening

**Objective**: Implement additional security measures.

**Tasks**:
- [ ] Add CORS configuration
- [ ] Implement request size limits
- [ ] Add input validation and sanitization
- [ ] Implement security headers (HSTS, CSP if applicable)
- [ ] Add SQL injection prevention (parameterized queries)
- [ ] Review and secure configuration (secrets management)
- [ ] Add security logging
- [ ] Create security documentation

**Deliverables**:
- Security hardening measures
- Security documentation
- Security tests

---

## Phase 7: Integration with CLI_Astro_Calc

### Step 7.1: CLI Integration Planning

**Objective**: Plan integration between Rusty_Server and CLI_Astro_Calc.

**Tasks**:
- [ ] Review CLI_Astro_Calc codebase
- [ ] Identify integration points:
  - Can CLI tool query Rusty_Server API?
  - Can Rusty_Server use CLI_Astro_Calc library functions?
- [ ] Design integration architecture
- [ ] Document integration plan

**Deliverables**:
- Integration plan document
- Architecture diagram (optional)

---

### Step 7.2: CLI Tool Enhancement

**Objective**: Add space weather querying to CLI_Astro_Calc.

**Tasks**:
- [ ] Add HTTP client to CLI_Astro_Calc
- [ ] Add new CLI command: `space-weather` or similar
- [ ] Implement API calls to Rusty_Server
- [ ] Format and display space weather data
- [ ] Add CLI tests

**Deliverables**:
- Enhanced CLI tool with space weather commands
- Integration with Rusty_Server API
- Tests

---

## Phase 8: Testing & Quality Assurance

### Step 8.1: Comprehensive Testing

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

---

### Step 8.2: Documentation

**Objective**: Complete project documentation.

**Tasks**:
- [ ] Update README.md with:
  - Project overview
  - Installation instructions
  - Configuration guide
  - API documentation
  - Deployment guide
  - Usage examples
- [ ] Complete OVERVIEW.md with:
  - Architecture overview
  - Space weather science explanations
  - API design decisions
  - Security considerations
- [ ] Add inline code documentation
- [ ] Create API documentation (OpenAPI/Swagger)

**Deliverables**:
- Complete README.md
- Complete OVERVIEW.md
- Code documentation
- API documentation

---

## Phase 9: Deployment & Operations

### Step 9.1: Deployment Preparation

**Objective**: Prepare the service for deployment.

**Tasks**:
- [ ] Create deployment configuration
- [ ] Set up environment-specific configs (dev, prod)
- [ ] Create Docker container (optional)
- [ ] Set up systemd service file (for Linux)
- [ ] Create deployment scripts
- [ ] Document deployment process

**Deliverables**:
- Deployment configuration
- Deployment scripts
- Deployment documentation

**Questions to Ask**:
- What OS will run on the server? (Linux distribution?)
- Do you want Docker containerization?
- How do you want to manage secrets in production?

---

### Step 9.2: Monitoring & Observability

**Objective**: Add monitoring and observability features.

**Tasks**:
- [ ] Add metrics collection (Prometheus format or similar)
- [ ] Add health check endpoints
- [ ] Implement structured logging
- [ ] Add performance monitoring
- [ ] Create monitoring dashboard (optional)
- [ ] Add alerting for critical issues (optional)

**Deliverables**:
- Metrics collection
- Health monitoring
- Observability features

---

### Step 9.3: Production Deployment

**Objective**: Deploy to production server.

**Tasks**:
- [ ] Set up production environment
- [ ] Configure production database
- [ ] Set up reverse proxy (nginx or similar) if needed
- [ ] Configure SSL/TLS certificates
- [ ] Deploy application
- [ ] Verify deployment
- [ ] Monitor initial operation

**Deliverables**:
- Production deployment
- Running service
- Monitoring in place

---

## Phase 10: Future Enhancements (Post-MVP)

### Potential Future Features
- [ ] WebSocket support for real-time updates
- [ ] GraphQL API (alternative to REST)
- [ ] Machine learning for space weather prediction
- [ ] Integration with additional data sources
- [ ] Advanced alerting system
- [ ] Dashboard/UI (frontend)
- [ ] Mobile app API
- [ ] Data export features
- [ ] Advanced analytics

---

## Testing Strategy

### Unit Tests
- Each module should have comprehensive unit tests
- Test edge cases and error conditions
- Aim for >80% code coverage

### Integration Tests
- Test complete API workflows
- Test database operations
- Test external API integration (with mocks)

### End-to-End Tests
- Test complete user workflows
- Test deployment scenarios

### Performance Tests
- Load testing for API endpoints
- Database query performance
- Cache performance

---

## Security Considerations

### Throughout Development
- [ ] Never commit secrets or API keys
- [ ] Use environment variables for sensitive config
- [ ] Validate all user inputs
- [ ] Use parameterized database queries
- [ ] Implement proper error handling (don't leak sensitive info)
- [ ] Regular security reviews
- [ ] Keep dependencies updated

---

## Questions for User

Before starting, please answer:

1. ✅ **Web Framework**: `axum` (decided - modern async Rust)
2. ✅ **Database**: MySQL (decided - you have it installed)
3. **Operating System**: What Linux distribution will run on the server?
4. **NOAA API**: Do you have an API key, or is the data publicly available?
5. **Authentication**: User accounts with JWT, or just API keys?
6. **Deployment**: Docker containerization desired?
7. **Frontend**: Will there be a web frontend, or API-only?
8. **Data Retention**: How much historical data should be stored?
9. **Rate Limits**: What rate limits should be enforced?
10. **Hardware**: Any specific considerations for your server hardware?

---

## Next Steps

Once you've reviewed this plan and answered the questions:

1. **Start with Phase 1, Step 1.1**: Project structure and dependencies
2. **Ask before proceeding**: We'll complete each step before moving to the next
3. **Update documentation**: Keep README and OVERVIEW.md current
4. **Test continuously**: Write tests as we build features

Let's begin! 🚀

