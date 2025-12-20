# DONKI API Key Setup

## Quick Setup

Your DONKI API key is in `credentials.txt` as `DONKI_KEY`. To use it with Rusty_Server, set it as an environment variable:

### Windows PowerShell:
```powershell
$env:RUSTY_SERVER__DONKI__API_KEY="zyiOSTMbLl7p3jGbM8gIPsETuZs4ZvPbt41NhBbg"
```

### Windows Command Prompt:
```cmd
set RUSTY_SERVER__DONKI__API_KEY=zyiOSTMbLl7p3jGbM8gIPsETuZs4ZvPbt41NhBbg
```

### Or add to config.toml:
```toml
[donki]
api_key = "zyiOSTMbLl7p3jGbM8gIPsETuZs4ZvPbt41NhBbg"
```

## Verification

After setting the API key, run the server:
```bash
cargo run
```

You should see:
```
DONKI API client initialized with API key
```

If you see a warning instead, the API key is not being loaded correctly.

## Testing

Once the server is running, test the current conditions endpoint:
```bash
curl http://localhost:3000/api/v1/space-weather/current
```

The response should now include `solar_flare` data from DONKI if any flares occurred in the last 7 days.
