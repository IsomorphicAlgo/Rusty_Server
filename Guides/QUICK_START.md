# Quick Start Guide

## Starting the Server

```bash
cargo run
```

The server will start on `http://localhost:3000` (or your configured host/port).

## Stopping the Server

### Method 1: Graceful Shutdown (Recommended)

Press **CTRL+C** in the terminal where the server is running.

The server will:
- Stop accepting new requests
- Wait for in-flight requests to complete (up to 2 seconds)
- Log "Shutting down gracefully..."
- Exit cleanly

### Method 2: Close Terminal

Simply close the terminal window or PowerShell window. The server process will be terminated.

### Method 3: Task Manager (If Stuck)

If the server doesn't respond to CTRL+C:

1. Open Task Manager (Ctrl+Shift+Esc)
2. Find `rusty-server.exe` or `cargo.exe` process
3. Right-click → End Task

## Testing the Server

### Local Testing

```powershell
# Health check
Invoke-WebRequest -Uri http://localhost:3000/health -UseBasicParsing

# Current conditions
Invoke-WebRequest -Uri http://localhost:3000/api/v1/space-weather/current -UseBasicParsing
```

### Testing from Another Device

1. Find your laptop's IP: `ipconfig` (look for IPv4 Address)
2. Configure Windows Firewall to allow port 3000
3. From another device: `http://YOUR_IP:3000/health`

See [TESTING_GUIDE.md](TESTING_GUIDE.md) for detailed instructions.

## Common Commands

```bash
# Build
cargo build

# Run
cargo run

# Test
cargo test

# Check for errors
cargo check

# Clean build artifacts
cargo clean
```

