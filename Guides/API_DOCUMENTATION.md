# Rusty Server API Documentation

## Base URL

```
http://localhost:3000
```

Production: `https://your-domain.com` (when deployed)

---

## Authentication

Most endpoints support optional API key authentication. When `require_auth` is enabled in configuration, API keys are required.

### API Key Header

```http
X-API-Key: rs_your_api_key_here
```

### Bearer Token

```http
Authorization: Bearer rs_your_api_key_here
```

### Getting an API Key

**POST** `/api/v1/auth/keys`

```json
{
  "name": "My API Key",
  "expires_in_hours": 720
}
```

**Response:**
```json
{
  "key": "rs_abc123...",
  "name": "My API Key",
  "created_at": "2024-12-20T12:00:00Z",
  "expires_at": "2025-01-20T12:00:00Z",
  "is_active": true
}
```

---

## Ephemeris (Ephemerust-backed)

Astronomical time, planet positions, and satellite propagation (TLE / SGP4 geometry) are documented in **[Guides/API_EPHEMERIS.md](API_EPHEMERIS.md)** (`POST` JSON only).

---

## Endpoints

### Health Check

**GET** `/health`

Returns server health status.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": 1703078400,
  "service": "rusty-server",
  "version": "0.1.0"
}
```

**Status Codes:**
- `200 OK` - Server is healthy

---

### Current Space Weather Conditions

**GET** `/api/v1/space-weather/current`

Returns the most recent space weather data available.

**Response:**
```json
{
  "data": {
    "solar_flare": {
      "class": "C2.5",
      "peak_time": "2024-12-20T12:05:00Z",
      "begin_time": "2024-12-20T12:00:00Z",
      "end_time": "2024-12-20T12:10:00Z",
      "source_location": "N10W10 AR 12345"
    },
    "geomagnetic_storm": {
      "level": "G1",
      "start_time": "2024-12-20T10:00:00Z",
      "end_time": "2024-12-20T18:00:00Z",
      "kp_index": 5.0
    },
    "solar_wind": {
      "speed": 450.0,
      "density": 3.5,
      "temperature": 50000.0,
      "bz": -2.5,
      "timestamp": "2024-12-20T12:00:00Z"
    },
    "kp_index": {
      "value": 3.0,
      "level": "Quiet",
      "timestamp": "2024-12-20T12:00:00Z"
    },
    "radiation": {
      "proton_flux": 1.2,
      "electron_flux": 45.6,
      "alert_level": "None",
      "timestamp": "2024-12-20T12:00:00Z"
    }
  },
  "metadata": {
    "timestamp": "2024-12-20T12:00:00Z",
    "source": "noaa,donki",
    "cached": false
  }
}
```

**Status Codes:**
- `200 OK` - Success
- `401 Unauthorized` - API key required (if auth enabled)
- `429 Too Many Requests` - Rate limit exceeded

**Notes:**
- Data is cached for performance (see `metadata.cached`)
- Falls back to database if API calls fail
- Solar flare data comes from NASA DONKI (if configured)
- Solar wind data (speed, density, temperature) is fetched from NOAA's plasma endpoint (`rtsw_plasma_1m.json`)
- If plasma data is unavailable, only magnetic field data (Bz) will be returned, with speed/density/temperature set to 0.0

---

### Historical Space Weather Data

**GET** `/api/v1/space-weather/historical`

Returns historical space weather observations within a date range.

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `start_date` | string | No | ISO 8601 format (e.g., `2024-12-19T00:00:00Z`). Defaults to 7 days ago. |
| `end_date` | string | No | ISO 8601 format (e.g., `2024-12-20T23:59:59Z`). Defaults to now. |
| `data_type` | string | No | Filter by type: `solar_flare`, `geomagnetic_storm`, `radiation`, `solar_wind`, `kp_index` |
| `limit` | integer | No | Maximum number of records (default: 100, max: 10000) |
| `offset` | integer | No | Pagination offset (partial support) |

**Example Request:**
```
GET /api/v1/space-weather/historical?start_date=2024-12-19T00:00:00Z&end_date=2024-12-20T23:59:59Z&data_type=solar_flare&limit=50
```

**Response:**
```json
[
  {
    "data": {
      "solar_flare": { ... },
      "kp_index": { ... },
      ...
    },
    "metadata": {
      "timestamp": "2024-12-20T12:00:00Z",
      "source": "noaa,donki",
      "cached": false
    }
  },
  ...
]
```

**Status Codes:**
- `200 OK` - Success
- `400 Bad Request` - Invalid date format or range
- `401 Unauthorized` - API key required (if auth enabled)
- `429 Too Many Requests` - Rate limit exceeded

**Validation Rules:**
- Date range cannot exceed 365 days
- `start_date` must be before `end_date`
- `limit` must be > 0 and <= 10000
- Dates must be ISO 8601 format

---

### Space Weather Alerts

**GET** `/api/v1/space-weather/alerts`

Returns active space weather alerts and warnings.

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `severity` | string | No | Filter by severity: `minor`, `moderate`, `severe`, `extreme` |
| `alert_type` | string | No | Filter by type: `solar_flare`, `geomagnetic_storm`, `radiation` |
| `active_only` | boolean | No | Return only active alerts (default: `false`) |

**Example Request:**
```
GET /api/v1/space-weather/alerts?severity=moderate&active_only=true
```

**Response:**
```json
[
  {
    "data": {
      "solar_flare": { ... },
      "geomagnetic_storm": { ... },
      ...
    },
    "metadata": {
      "timestamp": "2024-12-20T12:00:00Z",
      "source": "noaa",
      "cached": false
    }
  },
  ...
]
```

**Status Codes:**
- `200 OK` - Success
- `401 Unauthorized` - API key required (if auth enabled)
- `429 Too Many Requests` - Rate limit exceeded

---

### Radiation Levels

**GET** `/api/v1/space-weather/radiation`

Returns current radiation levels.

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `threshold` | float | No | Minimum proton flux threshold |
| `alert_level` | string | No | Filter by alert level: `None`, `S1`, `S2`, `S3`, `S4`, `S5` |

**Example Request:**
```
GET /api/v1/space-weather/radiation?threshold=2.0&alert_level=S1
```

**Response:**
```json
{
  "data": {
    "radiation": {
      "proton_flux": 1.2,
      "electron_flux": 45.6,
      "alert_level": "None",
      "timestamp": "2024-12-20T12:00:00Z"
    }
  },
  "metadata": {
    "timestamp": "2024-12-20T12:00:00Z",
    "source": "noaa",
    "cached": false
  }
}
```

**Status Codes:**
- `200 OK` - Success
- `401 Unauthorized` - API key required (if auth enabled)
- `429 Too Many Requests` - Rate limit exceeded

---

## API Key Management

### Generate API Key

**POST** `/api/v1/auth/keys`

**Request Body:**
```json
{
  "name": "My API Key",
  "expires_in_hours": 720
}
```

**Response:**
```json
{
  "key": "rs_abc123def456...",
  "name": "My API Key",
  "created_at": "2024-12-20T12:00:00Z",
  "expires_at": "2025-01-20T12:00:00Z",
  "is_active": true
}
```

**Status Codes:**
- `201 Created` - API key generated
- `400 Bad Request` - Invalid request

---

### List API Keys

**GET** `/api/v1/auth/keys`

**Response:**
```json
[
  {
    "key": "rs_abc123...",
    "name": "My API Key",
    "created_at": "2024-12-20T12:00:00Z",
    "expires_at": "2025-01-20T12:00:00Z",
    "is_active": true
  },
  ...
]
```

**Note:** Keys are partially masked for security.

**Status Codes:**
- `200 OK` - Success

---

### Revoke API Key

**DELETE** `/api/v1/auth/keys/:key`

**Status Codes:**
- `204 No Content` - Key revoked
- `404 Not Found` - Key not found

---

## Error Responses

All errors follow this format:

```json
{
  "error": "Error message",
  "details": "Additional error details (optional)"
}
```

**Common Status Codes:**
- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Authentication required
- `404 Not Found` - Resource not found
- `429 Too Many Requests` - Rate limit exceeded
- `500 Internal Server Error` - Server error

---

## Rate Limiting

Rate limiting is applied per IP address using a token bucket algorithm.

**Default Limits:**
- 60 requests per minute
- Burst size: 10 requests

**Rate Limit Headers:**
```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 1703078460
```

**Status Code:**
- `429 Too Many Requests` - Rate limit exceeded

---

## Data Sources

- **NOAA Space Weather Prediction Center**
  - KP index: `planetary_k_index_1m.json`
  - Solar wind magnetic field (Bz): `rtsw/rtsw_mag_1m.json`
  - Solar wind plasma (speed, density, temperature): `rtsw/rtsw_plasma_1m.json`
- **NASA DONKI** - Solar flare data (FLR endpoint)

---

## Examples

### PowerShell

```powershell
# Get current conditions
Invoke-WebRequest -Uri http://localhost:3000/api/v1/space-weather/current -UseBasicParsing | Select-Object -ExpandProperty Content | ConvertFrom-Json

# Get historical data
$startDate = (Get-Date).AddDays(-7).ToString("yyyy-MM-ddTHH:mm:ssZ")
$endDate = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ")
Invoke-WebRequest -Uri "http://localhost:3000/api/v1/space-weather/historical?start_date=$startDate&end_date=$endDate" -UseBasicParsing

# With API key
$headers = @{ "X-API-Key" = "rs_your_key_here" }
Invoke-WebRequest -Uri http://localhost:3000/api/v1/space-weather/current -Headers $headers -UseBasicParsing
```

### cURL

```bash
# Get current conditions
curl http://localhost:3000/api/v1/space-weather/current

# Get historical data
curl "http://localhost:3000/api/v1/space-weather/historical?start_date=2024-12-19T00:00:00Z&end_date=2024-12-20T23:59:59Z"

# With API key
curl -H "X-API-Key: rs_your_key_here" http://localhost:3000/api/v1/space-weather/current
```

### JavaScript (Fetch API)

```javascript
// Get current conditions
fetch('http://localhost:3000/api/v1/space-weather/current')
  .then(response => response.json())
  .then(data => console.log(data));

// With API key
fetch('http://localhost:3000/api/v1/space-weather/current', {
  headers: {
    'X-API-Key': 'rs_your_key_here'
  }
})
  .then(response => response.json())
  .then(data => console.log(data));
```

---

## Changelog

### Version 0.1.0
- Initial API release
- Current conditions endpoint
- Historical data endpoint
- Alerts endpoint
- Radiation endpoint
- API key authentication
- Rate limiting
- NASA DONKI integration for solar flares

---

## Support

For issues or questions, please open an issue on GitHub.
