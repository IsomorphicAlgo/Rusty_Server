# Rusty Server - Overview

## Project Overview

**Rusty_Server** is a Rust-based REST API service for fetching, caching, and serving space weather data. This project complements the CLI_Astro_Calc project by providing real-time and historical space weather information for satellite operations.

## Architecture

### High-Level Architecture

```
┌─────────────┐
│   Client    │
│  (Browser/  │
│   CLI App)  │
└──────┬──────┘
       │ HTTP/REST
       ▼
┌─────────────────────────────────┐
│      Rusty Server (axum)        │
│  ┌───────────────────────────┐  │
│  │   API Layer (handlers)    │  │
│  └───────────┬──────────────┘  │
│              │                  │
│  ┌───────────▼──────────────┐  │
│  │  Service Layer (logic)   │  │
│  └───────────┬──────────────┘  │
│              │                  │
│  ┌───────────▼──────────────┐  │
│  │   Cache Layer (memory)   │  │
│  └───────────┬──────────────┘  │
│              │                  │
│  ┌───────────▼──────────────┐  │
│  │  Database Layer (MySQL)  │  │
│  └──────────────────────────┘  │
└─────────────────────────────────┘
       │
       ▼
┌─────────────┐
│ NOAA API    │
│ (External)  │
└─────────────┘
```

### Component Overview

1. **API Layer** (`src/api/`): REST API endpoint handlers
2. **Service Layer** (`src/services/`): Business logic for space weather data
3. **Models** (`src/models/`): Data structures for space weather data
4. **Database Layer** (`src/database/`): MySQL database operations
5. **Cache Layer** (`src/cache/`): In-memory caching for performance
6. **Config** (`src/config/`): Configuration management
7. **Auth** (`src/auth/`): Authentication and authorization
8. **Errors** (`src/errors/`): Error handling types

## Space Weather Data

### What is Space Weather?

Space weather refers to the environmental conditions in space as influenced by solar activity. It affects:
- Satellite operations
- GPS accuracy
- Radio communications
- Power grids
- Astronaut safety

### Key Space Weather Metrics

1. **Solar Flares**: Sudden bursts of radiation from the Sun
2. **Geomagnetic Storms**: Disturbances in Earth's magnetic field
3. **Radiation Levels**: High-energy particle radiation
4. **Solar Wind**: Stream of charged particles from the Sun
5. **KP Index**: Measure of geomagnetic activity (0-9 scale)

### NOAA Space Weather Data Sources

The service integrates with NOAA (National Oceanic and Atmospheric Administration) Space Weather Prediction Center:
- Real-time space weather conditions
- Historical data archives
- Alert systems for significant events
- Multiple data formats (JSON, XML)

## Configuration System

### Configuration Sources (Priority Order)

1. **Environment Variables**: Highest priority, overrides all
2. **Config File**: `config.toml` or path specified by `CONFIG_FILE`
3. **Defaults**: Built-in sensible defaults

### Configuration Structure

- **Server**: Host, port settings
- **Database**: MySQL connection string, pool settings
- **NOAA**: API endpoints, keys, timeouts
- **Cache**: TTL values, size limits
- **Rate Limiting**: Request limits per minute/hour
- **Authentication**: JWT secrets, token expiry
- **Logging**: Log level, format (pretty/json)

### Environment Variable Format

Use double underscore (`__`) to represent nested config:
```
RUSTY_SERVER__SERVER__PORT=3000
RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://...
```

## Security Considerations

### Authentication & Authorization

- JWT-based authentication (optional, configurable)
- API key support for programmatic access
- Password hashing using Argon2

### Rate Limiting

- Per-IP rate limiting
- Per-API-key rate limiting
- Configurable limits (requests per minute/hour)

### Security Best Practices

- Never commit secrets or API keys
- Use environment variables for sensitive data
- Parameterized database queries (SQL injection prevention)
- Input validation on all endpoints
- Proper error handling (don't leak sensitive info)
- Security headers (CORS, HSTS)

## Development Workflow

### Local Development (Windows Laptop)

1. Install Rust toolchain
2. Install MySQL (or connect to remote)
3. Copy `config.example.toml` to `config.toml`
4. Set up `.env` file with local settings
5. Run `cargo run` to start development server
6. Test endpoints locally

### Deployment (Linux Server)

1. Build Linux binary (cross-compile or build on server)
2. Set up production MySQL database
3. Configure production environment variables
4. Set up systemd service
5. Deploy and monitor

## API Design

### RESTful Endpoints

- `GET /api/v1/space-weather/current` - Current conditions
- `GET /api/v1/space-weather/historical` - Historical data
- `GET /api/v1/space-weather/alerts` - Active alerts
- `GET /api/v1/space-weather/radiation` - Radiation levels
- `GET /api/v1/health` - Health check

### Response Format

All responses use JSON format with consistent structure:
```json
{
  "data": { ... },
  "metadata": {
    "timestamp": "2024-01-01T00:00:00Z",
    "source": "noaa"
  }
}
```

## Database Schema

### Planned Tables

- `space_weather_data`: Historical space weather records
- `alerts`: Space weather alerts
- `api_keys`: API key management (if using API keys)
- `users`: User accounts (if using JWT auth)

### Indexing Strategy

- Timestamp indexes for time-range queries
- Type indexes for filtering by data type
- Composite indexes for common query patterns

## Caching Strategy

### Cache Types

1. **Current Conditions**: Short TTL (15 minutes)
2. **Historical Data**: Longer TTL (1 hour)
3. **Alerts**: Very short TTL (5 minutes)

### Cache Invalidation

- Time-based expiration (TTL)
- Manual invalidation on data updates
- Size-based eviction (LRU)

## Testing Strategy

### Unit Tests

- Test individual functions and modules
- Mock external dependencies
- Test error conditions

### Integration Tests

- Test complete API workflows
- Test database operations
- Test external API integration (with mocks)

### End-to-End Tests

- Test complete user workflows
- Test deployment scenarios

## Performance Considerations

### Optimization Strategies

- In-memory caching to reduce API calls
- Database connection pooling
- Async/await for non-blocking I/O
- Efficient serialization/deserialization
- Query optimization with proper indexes

### Monitoring

- Request/response logging
- Performance metrics
- Cache hit/miss rates
- Database query performance

## Future Enhancements

- WebSocket support for real-time updates
- GraphQL API alternative
- Machine learning for predictions
- Additional data source integrations
- Advanced alerting system
- Web dashboard/UI
- Mobile app API

---

*This document will be continuously updated as the project evolves.*

