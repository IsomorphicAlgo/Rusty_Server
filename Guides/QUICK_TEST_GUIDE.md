# Quick Test Guide - Solar Flare Testing

## 🚀 Starting the Server

### 1. Set Database Connection String

**PowerShell:**
```powershell
# From credentials.txt: DB_USER=rusty_user, DB_PASSWORD=CXRTV8_7?4sPQ&f, DB_NAME=rusty_server_test
# Note: Using TEST database (rusty_server_test) for development
# Note: Special characters in password need URL encoding (? = %3F, & = %26)
$env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server_test"
```

**Command Prompt:**
```cmd
set RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server_test
```

**Or create `config.toml` in project root:**
```toml
[database]
# Using test database for development - change to rusty_server for production
connection_string = "mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server_test"
```

### 2. Set DONKI API Key (if not in config.toml)

**PowerShell:**
```powershell
$env:RUSTY_SERVER__DONKI__API_KEY="zyiOSTMbLl7p3jGbM8gIPsETuZs4ZvPbt41NhBbg"
```

**Command Prompt:**
```cmd
set RUSTY_SERVER__DONKI__API_KEY=zyiOSTMbLl7p3jGbM8gIPsETuZs4ZvPbt41NhBbg
```

### 3. Start the Server

```bash
cargo run
```

**Expected Output:**
```
DONKI API client initialized with API key
Rusty Server initialized successfully
Server listening on http://0.0.0.0:3000
```

---

## 📡 Testing Solar Flare Endpoint

### Test Current Conditions (includes solar flares)

**PowerShell:**
```powershell
Invoke-WebRequest -Uri http://localhost:3000/api/v1/space-weather/current -UseBasicParsing | Select-Object -ExpandProperty Content | ConvertFrom-Json | ConvertTo-Json -Depth 10
```

**Command Prompt (with curl):**
```bash
curl http://localhost:3000/api/v1/space-weather/current
```

**Browser:**
```
http://localhost:3000/api/v1/space-weather/current
```

### What to Look For

The response should include a `solar_flare` object if any flares occurred in the last 7 days:

```json
{
  "data": {
    "solar_flare": {
      "class": "C2.5",
      "peak_time": "2024-12-19T12:05:00Z",
      "begin_time": "2024-12-19T12:00:00Z",
      "end_time": "2024-12-19T12:10:00Z",
      "source_location": "N10W10 AR 12345"
    },
    "kp_index": { ... },
    "solar_wind": { ... },
    ...
  },
  "metadata": {
    "source": "noaa,donki",  // ← Shows DONKI is working!
    "timestamp": "...",
    "cached": false
  }
}
```

**If no flares in last 7 days:**
- `solar_flare` will be `null`
- `metadata.source` will be `"noaa"` (not `"noaa,donki"`)

---

## 🧪 Other Test Endpoints

### Health Check
```powershell
Invoke-WebRequest -Uri http://localhost:3000/health -UseBasicParsing
```

### Historical Data (last 7 days)
```powershell
$startDate = (Get-Date).AddDays(-7).ToString("yyyy-MM-dd")
$endDate = (Get-Date).ToString("yyyy-MM-dd")
Invoke-WebRequest -Uri "http://localhost:3000/api/v1/space-weather/historical?start_date=$startDate&end_date=$endDate" -UseBasicParsing
```

### Historical Data - Solar Flares Only
```powershell
$startDate = (Get-Date).AddDays(-7).ToString("yyyy-MM-dd")
$endDate = (Get-Date).ToString("yyyy-MM-dd")
Invoke-WebRequest -Uri "http://localhost:3000/api/v1/space-weather/historical?start_date=$startDate&end_date=$endDate&data_type=solar_flare" -UseBasicParsing
```

---

## 🛑 Safely Shutting Down

### Method 1: Keyboard Interrupt (Recommended)
Press `Ctrl+C` in the terminal where the server is running.

**Expected Output:**
```
^C
Shutting down gracefully...
Server shutdown complete
```

### Method 2: Close Terminal
Simply close the terminal window (Windows will send SIGTERM).

---

## 🔍 Troubleshooting

### No Solar Flare Data?

1. **Check API Key:**
   - Look for: `DONKI API client initialized with API key`
   - If you see a warning instead, the API key isn't loaded

2. **Check Logs:**
   - Look for: `Fetched X solar flares from DONKI`
   - If you see: `Failed to fetch solar flares from DONKI`, check your internet connection

3. **No Flares in Last 7 Days:**
   - Solar flares are rare - it's normal to have `null` if none occurred
   - Try a longer date range in historical endpoint

### Server Won't Start?

1. **Database Connection Error (Access Denied):**
   - The config system doesn't automatically load `credentials.txt`
   - **Solution:** Set the connection string as environment variable:
     ```powershell
     $env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server"
     ```
   - **Note:** Special characters in password must be URL-encoded:
     - `?` becomes `%3F`
     - `&` becomes `%26`
     - `/` becomes `%2F`
     - `@` becomes `%40`

2. **Check MySQL is running:**
   ```powershell
   # Check if MySQL service is running (Windows)
   Get-Service -Name MySQL*
   ```

3. **Check database connection manually:**
   ```powershell
   # Test MySQL connection
   mysql -u rusty_user -p -h localhost rusty_server
   # Enter password: CXRTV8_7?4sPQ&f
   ```

4. **Check port 3000 is free:**
   ```powershell
   netstat -ano | findstr :3000
   ```

---

## 📝 Quick Reference

| Action | Command |
|--------|---------|
| **Set DB Connection** | `$env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server"` |
| **Set DONKI Key** | `$env:RUSTY_SERVER__DONKI__API_KEY="zyiOSTMbLl7p3jGbM8gIPsETuZs4ZvPbt41NhBbg"` |
| **Start Server** | `cargo run` |
| **Test Current** | `Invoke-WebRequest http://localhost:3000/api/v1/space-weather/current` |
| **Test Health** | `Invoke-WebRequest http://localhost:3000/health` |
| **Stop Server** | `Ctrl+C` |

---

## 🎯 Testing Solar Flares Specifically

To see if DONKI is working, check the response metadata:

✅ **Working:** `"source": "noaa,donki"`  
❌ **Not Working:** `"source": "noaa"` (and `solar_flare` is `null`)

If you see `"noaa,donki"` but `solar_flare` is still `null`, it means:
- DONKI is working ✅
- No flares occurred in the last 7 days (normal!)

To test with actual flare data, try the historical endpoint with a longer date range.
