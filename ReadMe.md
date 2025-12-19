# Rusty Server

A Rust-based REST API service for fetching, caching, and serving space weather data critical for satellite operations. This project complements the CLI_Astro_Calc project by providing real-time and historical space weather information.

## Project Status

**Current Phase**: Phase 1 - Project Foundation & Setup  
**Completed Steps**: 
- ✅ 1.1 (Project Structure & Dependencies)
- ✅ 1.2 (Configuration System)
- ✅ 1.3 (Logging & Error Handling)

**Next Step**: 2.1 (Basic HTTP Server)

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

**📋 See [ITERATIVE_PLAN.md](ITERATIVE_PLAN.md) for the complete development plan.**  
**📚 See [OVERVIEW.md](OVERVIEW.md) for architecture and technical details.**

---

## Introduction

The purpose of this project is to explore creating a Rust-based REST API service to control and serve space weather data from a home server unit. This project is a continuation of the CLI_Astro_Calc Project.

Rusty_Server will host a REST API service for fetching, caching, and serving space weather data critical for satellite operations. This service will complement the CLI tool by providing real-time and historical space weather information.

## Use Cases

- Satellite operators monitoring space weather conditions
- Mission planning based on historical space weather patterns
- Real-time alerts for solar flares and geomagnetic storms
- Radiation level monitoring for space missions

## Features

### Planned Features

- **Rust-Based REST API**: Modern async web service using axum
- **Data Fetching**: Integration with NOAA Space Weather API and similar sources
- **Local Caching**: Reduce API calls and improve response times
- **REST Endpoints**: Clean API for querying current conditions and historical data
- **Data Storage**: Historical data storage in MySQL
- **Production Features**: Rate limiting and authentication
- **Deployment**: Self-hosted on personal server rack

### Current Implementation

- ✅ Configuration management (environment variables + config files)
- ✅ Error handling system with status codes and logging
- ✅ Structured logging (pretty and JSON formats)
- ✅ Request/response logging middleware
- ✅ Module structure for all components
- ⏳ HTTP server (next step)
- ⏳ Database integration (planned)
- ⏳ NOAA API integration (planned)
- ⏳ Caching layer (planned)

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
- MySQL (for database)
- Git (for version control)

### Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Rusty_Server.git
   cd Rusty_Server
   ```

2. **Configure the application**:
   ```bash
   # Copy example config
   cp config.example.toml config.toml
   
   # Edit config.toml with your settings
   # Or use environment variables (see Configuration section)
   ```

3. **Set up environment variables** (optional):
   ```bash
   # Create .env file (not committed to git)
   # See config.example.toml for available options
   ```

4. **Build and run**:
   ```bash
   cargo build
   cargo run
   ```

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
└── ITERATIVE_PLAN.md    # Development plan
```

## Development Workflow

### Local Development (Windows Laptop)

1. Develop and test code locally
2. Use local MySQL instance or connect to server's MySQL
3. Test API endpoints on localhost
4. Run tests: `cargo test`

### Deployment (Linux Server)

1. Build Linux binary (cross-compile or build on server)
2. Deploy to server rack
3. Configure production MySQL
4. Set up as systemd service

## Testing

Run tests with:
```bash
cargo test
```

Test coverage includes:
- Unit tests for individual modules
- Integration tests for API workflows
- Configuration validation tests

## Security Considerations

- ✅ **Credential Management**: All sensitive files are gitignored
- ✅ **Password Masking**: Passwords are automatically masked in logs
- ✅ **Environment Variables**: Secure credential storage via environment variables
- ✅ **Never commit secrets**: All credential files are gitignored
- ✅ Parameterized database queries (SQL injection prevention)
- ✅ Input validation on all endpoints
- ✅ Proper error handling (don't leak sensitive info)
- ⏳ Rate limiting (planned)
- ⏳ Authentication/authorization (planned)
- ⏳ Security headers (planned)

**📋 See [SECURITY.md](SECURITY.md) for detailed security guidelines and credential management.**

## Troubleshooting

### Build Errors

If you encounter linker errors (LNK1104), this is often caused by antivirus software. See [BUILD_TROUBLESHOOTING.md](BUILD_TROUBLESHOOTING.md) for solutions.

**Quick fix:**
1. Add `target/` folder to antivirus exclusions
2. Restart your computer
3. Run `cargo clean && cargo check`

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
