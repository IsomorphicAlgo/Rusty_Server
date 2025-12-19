# Troubleshooting Guide

## Port Already in Use Error

**Error**: `Only one usage of each socket address (protocol/network address/port) is normally permitted. (os error 10048)`

This means port 3000 is already in use by another process (likely a previous server instance).

### Solution 1: Find and Kill the Process

**Step 1: Find the process using port 3000**
```powershell
netstat -ano | findstr :3000
```

This will show something like:
```
TCP    0.0.0.0:3000           0.0.0.0:0              LISTENING       24452
```

The last number (24452) is the Process ID (PID).

**Step 2: Kill the process**
```powershell
Stop-Process -Id 24452 -Force
```

Replace `24452` with the actual PID from step 1.

**Step 3: Verify port is free**
```powershell
netstat -ano | findstr :3000
```

If nothing is returned, the port is free.

**Step 4: Start the server again**
```powershell
cargo run
```

### Solution 2: Use a Different Port

If you want to keep the other process running, change the server port:

**Option A: Environment Variable**
```powershell
$env:RUSTY_SERVER__SERVER__PORT=3001
cargo run
```

**Option B: Config File**
Edit `config.toml` (or create it from `config.example.toml`):
```toml
[server]
port = 3001
```

### Solution 3: Find All Rust/Cargo Processes

```powershell
Get-Process | Where-Object {$_.ProcessName -like "*rusty*" -or $_.ProcessName -like "*cargo*"}
```

Then kill them:
```powershell
Stop-Process -Name "rusty-server" -Force
# or
Stop-Process -Name "cargo" -Force
```

## Other Common Issues

### Server Won't Start

1. **Check if port is in use** (see above)
2. **Check configuration errors**: Look for error messages about missing config
3. **Check MySQL connection**: If database connection fails, server may not start

### Can't Connect from Another Device

1. **Check Windows Firewall**: Port 3000 must be allowed
2. **Verify IP address**: Use `ipconfig` to get correct IP
3. **Check network**: Both devices must be on same WiFi
4. **Verify server is running**: Check server logs

### Build Errors

1. **Clean and rebuild**:
   ```bash
   cargo clean
   cargo build
   ```

2. **Check for file locks**: Close any IDEs or editors with files open
3. **Antivirus issues**: See BUILD_TROUBLESHOOTING.md

### Configuration Errors

1. **Missing fields**: Check that all required config fields have defaults or values
2. **Invalid values**: Check config validation error messages
3. **Environment variables**: Verify format is correct (`RUSTY_SERVER__FIELD__SUBFIELD`)

## Getting Help

If you encounter issues not covered here:

1. Check the error message carefully
2. Look at server logs (they show file and line numbers)
3. Check ITERATIVE_PLAN.md for current phase status
4. Review relevant documentation files

