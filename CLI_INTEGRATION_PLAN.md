# CLI_Astro_Calc Integration Plan

## Overview

This document outlines the integration plan between **Rusty_Server** (REST API) and **CLI_Astro_Calc** (command-line tool). The goal is to enable the CLI tool to query space weather data from Rusty_Server instead of (or in addition to) directly calling the NOAA API.

## Integration Architecture

```
┌─────────────────────┐
│  CLI_Astro_Calc     │
│  (Command Line)     │
└──────────┬──────────┘
           │
           │ HTTP/REST API
           │ (reqwest client)
           ▼
┌─────────────────────┐
│   Rusty_Server      │
│   (REST API)        │
│                     │
│  ┌───────────────┐  │
│  │  Cache Layer  │  │
│  └───────┬───────┘  │
│          │          │
│  ┌───────▼───────┐  │
│  │   Database    │  │
│  └───────┬───────┘  │
│          │          │
└──────────┼──────────┘
           │
           ▼
┌─────────────────────┐
│    NOAA API         │
│   (External)        │
└─────────────────────┘
```

## Integration Points

### 1. CLI Tool → Rusty_Server API

The CLI tool will query Rusty_Server API endpoints to get space weather data:

- **Current Conditions**: `GET /api/v1/space-weather/current`
- **Historical Data**: `GET /api/v1/space-weather/historical`
- **Alerts**: `GET /api/v1/space-weather/alerts`
- **Radiation**: `GET /api/v1/space-weather/radiation`

### 2. Benefits of Integration

- **Caching**: CLI benefits from Rusty_Server's caching layer
- **Historical Data**: Access to stored historical data without hitting NOAA API
- **Consistency**: Both CLI and web clients use the same data source
- **Performance**: Faster responses due to caching
- **Reduced API Calls**: Fewer direct calls to NOAA API

## CLI Command Design

### Proposed Commands

```bash
# Get current space weather conditions
astro-calc space-weather current

# Get historical data
astro-calc space-weather historical --start 2024-01-01 --end 2024-01-31

# Get active alerts
astro-calc space-weather alerts

# Get radiation levels
astro-calc space-weather radiation

# Configure API server URL
astro-calc config set-api-url http://localhost:3000
```

### Command Options

```bash
# Current conditions with formatting options
astro-calc space-weather current --format json
astro-calc space-weather current --format table
astro-calc space-weather current --format detailed

# Historical data with filters
astro-calc space-weather historical \
  --start 2024-01-01 \
  --end 2024-01-31 \
  --type solar-flare \
  --limit 10

# Alerts with filtering
astro-calc space-weather alerts --severity high --active-only

# Radiation with thresholds
astro-calc space-weather radiation --threshold 100
```

## Configuration

### CLI Configuration File

The CLI tool should support configuration for:

1. **API Server URL**: Default to `http://localhost:3000`
2. **API Key**: Optional, if authentication is enabled
3. **Timeout**: Request timeout (default: 30 seconds)
4. **Format**: Default output format (json, table, detailed)

**Example config file** (`~/.astro-calc/config.toml` or similar):
```toml
[api]
url = "http://localhost:3000"
api_key = "rs_optional_api_key_here"
timeout_seconds = 30

[output]
format = "table"  # json, table, detailed
```

### Environment Variables

Also support environment variables:
```bash
ASTRO_CALC_API_URL=http://localhost:3000
ASTRO_CALC_API_KEY=rs_optional_key
ASTRO_CALC_TIMEOUT=30
```

## Implementation Details

### HTTP Client

The CLI tool should use `reqwest` (or similar HTTP client) to make requests:

```rust
// Example structure
pub struct RustyServerClient {
    base_url: String,
    api_key: Option<String>,
    client: reqwest::Client,
    timeout: Duration,
}

impl RustyServerClient {
    pub async fn get_current_conditions(&self) -> Result<SpaceWeatherResponse> {
        // Implementation
    }
    
    pub async fn get_historical_data(&self, query: HistoricalQuery) -> Result<HistoricalResponse> {
        // Implementation
    }
    
    // ... other methods
}
```

### Error Handling

Handle common errors:
- **Connection errors**: Server not running, network issues
- **Authentication errors**: Invalid API key (if auth enabled)
- **Rate limiting**: 429 Too Many Requests
- **Server errors**: 500 Internal Server Error

### Fallback Strategy

Consider implementing a fallback:
1. Try Rusty_Server API first
2. If unavailable, fall back to direct NOAA API calls (if CLI has that capability)
3. Show appropriate error messages

## API Endpoints Reference

### Current Conditions
```
GET /api/v1/space-weather/current
```

**Response:**
```json
{
  "data": {
    "solar_flares": [...],
    "geomagnetic_storms": [...],
    "radiation_levels": {...},
    "timestamp": "2024-01-01T00:00:00Z"
  },
  "cached": true,
  "source": "cache"
}
```

### Historical Data
```
GET /api/v1/space-weather/historical?start_date=2024-01-01&end_date=2024-01-31&data_type=solar-flare&limit=10
```

**Query Parameters:**
- `start_date`: ISO 8601 date (required)
- `end_date`: ISO 8601 date (required)
- `data_type`: Optional filter (solar-flare, geomagnetic-storm, radiation)
- `limit`: Optional limit (default: 100)
- `offset`: Optional pagination offset

### Alerts
```
GET /api/v1/space-weather/alerts?severity=high&active_only=true
```

**Query Parameters:**
- `severity`: Optional filter (low, moderate, high, extreme)
- `type`: Optional filter (solar-flare, geomagnetic-storm)
- `active_only`: Boolean (default: false)

### Radiation
```
GET /api/v1/space-weather/radiation?threshold=100&alert_level=true
```

**Query Parameters:**
- `threshold`: Optional minimum radiation level
- `alert_level`: Boolean, only show alert-level radiation

## Authentication

If Rusty_Server has authentication enabled (`require_auth = true`), the CLI must:

1. Include API key in requests:
   - Header: `X-API-Key: rs_xxxxx`
   - Or: `Authorization: Bearer rs_xxxxx`

2. Handle authentication errors gracefully:
   - Show clear error message
   - Suggest checking API key configuration

## Testing Strategy

### Unit Tests
- Test HTTP client methods
- Test request building
- Test response parsing
- Test error handling

### Integration Tests
- Test against running Rusty_Server instance
- Test with authentication enabled/disabled
- Test error scenarios (server down, rate limiting, etc.)

### Manual Testing
- Test all CLI commands
- Test with different output formats
- Test configuration options

## Implementation Checklist

### For CLI_Astro_Calc Project

- [ ] Add `reqwest` dependency (if not already present)
- [ ] Create `RustyServerClient` struct
- [ ] Implement API client methods
- [ ] Add CLI command: `space-weather`
- [ ] Add subcommands: `current`, `historical`, `alerts`, `radiation`
- [ ] Implement configuration file support
- [ ] Add environment variable support
- [ ] Implement output formatting (json, table, detailed)
- [ ] Add error handling and user-friendly messages
- [ ] Add tests
- [ ] Update CLI documentation

### For Rusty_Server (Already Complete)

- ✅ REST API endpoints implemented
- ✅ Authentication support (API keys)
- ✅ Rate limiting
- ✅ CORS configuration
- ✅ Security headers
- ✅ Comprehensive error responses

## Example Usage

```bash
# Configure API server
astro-calc config set-api-url http://192.168.1.100:3000

# Get current conditions (table format)
astro-calc space-weather current

# Get current conditions (JSON format)
astro-calc space-weather current --format json

# Get historical solar flares
astro-calc space-weather historical \
  --start 2024-01-01 \
  --end 2024-01-31 \
  --type solar-flare \
  --limit 20

# Get active high-severity alerts
astro-calc space-weather alerts --severity high --active-only

# Get radiation levels above threshold
astro-calc space-weather radiation --threshold 100
```

## Next Steps

1. **Review this plan** with the CLI_Astro_Calc project
2. **Implement HTTP client** in CLI_Astro_Calc
3. **Add CLI commands** for space weather
4. **Test integration** between CLI and Rusty_Server
5. **Update documentation** for both projects

## Questions to Resolve

1. Does CLI_Astro_Calc already have HTTP client capabilities?
2. What CLI framework is used? (clap, structopt, etc.)
3. What output formatting library is used? (for tables, etc.)
4. Should CLI support both Rusty_Server API and direct NOAA API calls?
5. What is the preferred configuration file format? (TOML, JSON, YAML)

## Notes

- This integration is **one-way**: CLI → Rusty_Server
- Rusty_Server does not need to import CLI_Astro_Calc code
- The integration is via HTTP/REST API only
- Both projects remain independent and can be developed separately
