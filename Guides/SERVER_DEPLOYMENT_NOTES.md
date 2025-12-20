# Server Deployment Guide

Complete step-by-step guide for deploying Rusty Server to your Linux server rack.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Initial Server Setup](#initial-server-setup)
3. [Installing Dependencies](#installing-dependencies)
4. [Building the Application](#building-the-application)
5. [Database Setup](#database-setup)
6. [Configuration](#configuration)
7. [Running the Server](#running-the-server)
8. [Setting Up as a Service](#setting-up-as-a-service)
9. [Network Configuration](#network-configuration)
10. [Testing the Deployment](#testing-the-deployment)
11. [Monitoring & Maintenance](#monitoring--maintenance)
12. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Hardware Requirements
- ✅ Your server rack hardware (already have)
- ✅ Network connection
- ✅ Power supply

### Software Requirements
- Linux operating system (Ubuntu 22.04 LTS recommended, or Debian 11+)
- Rust toolchain (latest stable)
- MySQL 8.0+ or MariaDB 10.6+
- Git (for cloning repository)
- Systemd (for service management)

---

## Initial Server Setup

### Step 1: Power On and Initial Boot

1. **Power on the server**
   - Assemble screen and get proper power cable
   - Connect power cables
   - Press power button
   - Wait for initial boot

2. **Access BIOS/UEFI**
   - Press appropriate key during boot (usually F2, F12, or Del)
   - Configure boot order if needed
   - Save and exit

3. **IPMI Setup (Optional but Recommended)**
   - Access IPMI interface (usually via dedicated network port or shared)
   - Set IPMI IP address
   - Configure user accounts
   - This allows remote management even when server is off

### Step 2: Install Linux Operating System

**Recommended: Ubuntu 22.04 LTS Server**

1. **Create bootable USB** (on your Windows laptop):
   - Download Ubuntu 22.04 LTS Server ISO
   - Use Rufus or similar tool to create bootable USB

2. **Install Ubuntu**:
   - Boot from USB
   - Follow installation wizard
   - Set up:
     - Hostname: `rusty-server` (or your preference)
     - User account (create non-root user)
     - Network configuration
     - Disk partitioning (use default or custom)
     - SSH server (enable for remote access)

3. **Initial System Update**:
   ```bash
   sudo apt update
   sudo apt upgrade -y
   sudo reboot
   ```

4. **Verify Installation**:
   ```bash
   uname -a
   hostname
   ip addr show
   ```

---

## Installing Dependencies

### Step 1: Install Rust Toolchain

```bash
# Install Rust using rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, then:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version

# Install build dependencies
sudo apt install -y build-essential pkg-config libssl-dev
```

### Step 2: Install MySQL

```bash
# Update package list
sudo apt update

# Install MySQL server
sudo apt install -y mysql-server

# Secure MySQL installation
sudo mysql_secure_installation

# Start and enable MySQL
sudo systemctl start mysql
sudo systemctl enable mysql

# Verify MySQL is running
sudo systemctl status mysql
```

### Step 3: Install Additional Tools

```bash
# Install useful tools
sudo apt install -y git curl wget vim htop

# Install development tools (if needed)
sudo apt install -y gcc g++ make
```

---

## Building the Application

### Step 1: Clone Repository

```bash
# Create application directory
sudo mkdir -p /opt/rusty-server
sudo chown $USER:$USER /opt/rusty-server

# Clone repository (or transfer files from your laptop)
cd /opt/rusty-server
git clone <your-repo-url> .

# Or if transferring from laptop:
# Use scp, rsync, or copy files manually
```

### Step 2: Build the Application

```bash
cd /opt/rusty-server

# Build in release mode (optimized)
cargo build --release

# This will create the binary at:
# target/release/rusty-server

# Verify binary was created
ls -lh target/release/rusty-server

# Test the binary
./target/release/rusty-server --help  # (if help flag exists)
```

### Step 3: Create Application Directory Structure

```bash
# Create directories for runtime files
sudo mkdir -p /opt/rusty-server/{config,logs,data}
sudo chown -R $USER:$USER /opt/rusty-server
```

---

## Database Setup

### Step 1: Create Production Database

```bash
# Connect to MySQL as root
sudo mysql -u root -p
```

Then run these SQL commands:

```sql
-- Create production database
CREATE DATABASE rusty_server CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Create database user (replace with your credentials)
CREATE USER 'rusty_user'@'localhost' IDENTIFIED BY 'your_secure_password_here';

-- Grant privileges
GRANT ALL PRIVILEGES ON rusty_server.* TO 'rusty_user'@'localhost';

-- Apply changes
FLUSH PRIVILEGES;

-- Verify
SHOW DATABASES;
SELECT user, host FROM mysql.user WHERE user = 'rusty_user';

-- Exit
EXIT;
```

### Step 2: Test Database Connection

```bash
# Test connection
mysql -u rusty_user -p rusty_server

# If successful, you'll see MySQL prompt
# Type EXIT to leave
```

---

## Configuration

### Step 1: Create Production Configuration

```bash
cd /opt/rusty-server

# Copy example config
cp config.example.toml config.toml

# Edit configuration
nano config.toml
```

**Update `config.toml` with production values:**

```toml
[server]
host = "0.0.0.0"  # Listen on all interfaces
port = 3000

[database]
# URL encode special characters in password: ? = %3F, & = %26
connection_string = "mysql://rusty_user:your_password@localhost:3306/rusty_server"
max_connections = 10

[noaa]
base_url = "https://services.swpc.noaa.gov"
api_key = ""  # Optional
timeout_seconds = 30

[donki]
base_url = "https://api.nasa.gov/DONKI"
api_key = "your_donki_api_key_here"  # Required for solar flares
timeout_seconds = 30

[auth]
jwt_secret = "generate_strong_random_secret_here"  # MUST CHANGE!
token_expiry_hours = 24
require_auth = false  # Set to true in production for security

[logging]
level = "info"  # Use "warn" or "error" in production
format = "json"  # JSON format for production logging

[security]
cors_allowed_origins = "*"  # Restrict in production
enable_hsts = true  # Enable for HTTPS
```

### Step 2: Generate Secure JWT Secret

```bash
# Generate a strong random secret
openssl rand -base64 32

# Copy the output and paste it into config.toml as jwt_secret
```

### Step 3: Set Environment Variables (Alternative to config.toml)

You can also use environment variables instead of config.toml:

```bash
# Create environment file
sudo nano /opt/rusty-server/.env
```

Add:
```bash
RUSTY_SERVER__SERVER__HOST=0.0.0.0
RUSTY_SERVER__SERVER__PORT=3000
RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://rusty_user:password@localhost:3306/rusty_server
RUSTY_SERVER__DONKI__API_KEY=your_donki_key_here
RUSTY_SERVER__AUTH__JWT_SECRET=your_generated_secret_here
RUSTY_SERVER__AUTH__REQUIRE_AUTH=false
RUSTY_SERVER__LOGGING__LEVEL=info
RUSTY_SERVER__LOGGING__FORMAT=json
```

**Note**: Environment variables take precedence over config.toml.

---

## Running the Server

### Method 1: Manual Start (Testing)

```bash
cd /opt/rusty-server

# Run the server
./target/release/rusty-server

# Or with environment variables
RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://..." ./target/release/rusty-server
```

**Expected Output:**
```
Running database migrations...
Database migrations completed successfully
Database connection verified
DONKI API client initialized with API key
Cache initialized with TTLs: current=900s, historical=3600s, alerts=300s
Rate limiter initialized: 60 req/min, burst: 10
Rusty Server initialized successfully
Starting HTTP server on 0.0.0.0:3000
Server listening on http://0.0.0.0:3000
```

**To stop**: Press `Ctrl+C`

### Method 2: Run in Background (Temporary)

```bash
cd /opt/rusty-server

# Run in background
nohup ./target/release/rusty-server > logs/server.log 2>&1 &

# Check if running
ps aux | grep rusty-server

# View logs
tail -f logs/server.log

# Stop the process
pkill rusty-server
```

### Method 3: Systemd Service (Recommended for Production)

Create systemd service file:

```bash
sudo nano /etc/systemd/system/rusty-server.service
```

Add this content:

```ini
[Unit]
Description=Rusty Server - Space Weather API
After=network.target mysql.service
Requires=mysql.service

[Service]
Type=simple
User=rusty-server
Group=rusty-server
WorkingDirectory=/opt/rusty-server
ExecStart=/opt/rusty-server/target/release/rusty-server
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=rusty-server

# Environment variables (optional - can use config.toml instead)
# Environment="RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://..."
# Environment="RUSTY_SERVER__DONKI__API_KEY=..."

# Security settings
NoNewPrivileges=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

**Create service user:**

```bash
# Create dedicated user for the service
sudo useradd -r -s /bin/false -d /opt/rusty-server rusty-server

# Set ownership
sudo chown -R rusty-server:rusty-server /opt/rusty-server
```

**Enable and start the service:**

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable service (start on boot)
sudo systemctl enable rusty-server

# Start the service
sudo systemctl start rusty-server

# Check status
sudo systemctl status rusty-server

# View logs
sudo journalctl -u rusty-server -f
```

**Service Management Commands:**

```bash
# Start service
sudo systemctl start rusty-server

# Stop service
sudo systemctl stop rusty-server

# Restart service
sudo systemctl restart rusty-server

# Check status
sudo systemctl status rusty-server

# View logs
sudo journalctl -u rusty-server -n 50
sudo journalctl -u rusty-server -f  # Follow logs

# Disable auto-start
sudo systemctl disable rusty-server
```

---

## Network Configuration

### Step 1: Configure Firewall

**Ubuntu/Debian (ufw):**

```bash
# Install ufw if not installed
sudo apt install -y ufw

# Allow SSH (important - do this first!)
sudo ufw allow 22/tcp

# Allow Rusty Server port
sudo ufw allow 3000/tcp

# Enable firewall
sudo ufw enable

# Check status
sudo ufw status
```

**Or using iptables directly:**

```bash
# Allow port 3000
sudo iptables -A INPUT -p tcp --dport 3000 -j ACCEPT

# Save rules (Ubuntu/Debian)
sudo netfilter-persistent save
```

### Step 2: Test Network Access

**From server:**
```bash
# Test locally
curl http://localhost:3000/health
```

**From another machine on your network:**
```bash
# Replace with your server's IP address
curl http://192.168.1.100:3000/health
```

### Step 3: Set Up Reverse Proxy (Optional but Recommended)

**Install nginx:**

```bash
sudo apt install -y nginx
```

**Create nginx configuration:**

```bash
sudo nano /etc/nginx/sites-available/rusty-server
```

Add:

```nginx
server {
    listen 80;
    server_name your-domain.com;  # Or your server IP

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
```

**Enable site:**

```bash
# Create symlink
sudo ln -s /etc/nginx/sites-available/rusty-server /etc/nginx/sites-enabled/

# Test configuration
sudo nginx -t

# Reload nginx
sudo systemctl reload nginx
```

### Step 4: Set Up SSL/TLS (Optional but Recommended)

**Using Let's Encrypt (Certbot):**

```bash
# Install certbot
sudo apt install -y certbot python3-certbot-nginx

# Get certificate (requires domain name)
sudo certbot --nginx -d your-domain.com

# Certbot will automatically configure nginx for HTTPS
# Certificates auto-renew via cron job
```

---

## Testing the Deployment

### Step 1: Health Check

```bash
# From server
curl http://localhost:3000/health

# Expected response:
# {"status":"healthy","timestamp":...,"service":"rusty-server","version":"0.1.0"}
```

### Step 2: Test API Endpoints

```bash
# Current conditions
curl http://localhost:3000/api/v1/space-weather/current

# Historical data
curl "http://localhost:3000/api/v1/space-weather/historical?limit=5"

# Alerts
curl http://localhost:3000/api/v1/space-weather/alerts
```

### Step 3: Test from External Machine

```bash
# From your laptop or another machine
curl http://your-server-ip:3000/health

# Or if using nginx reverse proxy
curl http://your-domain.com/health
```

### Step 4: Verify Database

```bash
# Connect to database
mysql -u rusty_user -p rusty_server

# Check tables
SHOW TABLES;

# Check for data
SELECT COUNT(*) FROM space_weather_observations;
SELECT * FROM space_weather_observations ORDER BY timestamp DESC LIMIT 5;

EXIT;
```

---

## Monitoring & Maintenance

### Step 1: Log Management

**View Logs:**

```bash
# If using systemd
sudo journalctl -u rusty-server -f

# If using nohup
tail -f /opt/rusty-server/logs/server.log

# Log rotation (create logrotate config)
sudo nano /etc/logrotate.d/rusty-server
```

Add:
```
/opt/rusty-server/logs/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 0640 rusty-server rusty-server
}
```

### Step 2: Monitor Service Status

```bash
# Check service status
sudo systemctl status rusty-server

# Check if process is running
ps aux | grep rusty-server

# Check port is listening
sudo netstat -tlnp | grep 3000
# Or
sudo ss -tlnp | grep 3000
```

### Step 3: Database Backups

**Create backup script:**

```bash
sudo nano /opt/rusty-server/scripts/backup-database.sh
```

Add:
```bash
#!/bin/bash
BACKUP_DIR="/opt/rusty-server/backups"
DATE=$(date +%Y%m%d_%H%M%S)
DB_NAME="rusty_server"
DB_USER="rusty_user"

mkdir -p $BACKUP_DIR

mysqldump -u $DB_USER -p$DB_PASSWORD $DB_NAME > $BACKUP_DIR/rusty_server_$DATE.sql

# Keep only last 7 days
find $BACKUP_DIR -name "rusty_server_*.sql" -mtime +7 -delete
```

**Make executable:**

```bash
chmod +x /opt/rusty-server/scripts/backup-database.sh
```

**Set up cron job:**

```bash
crontab -e
```

Add:
```
# Daily backup at 2 AM
0 2 * * * /opt/rusty-server/scripts/backup-database.sh
```

### Step 4: Update Procedure

```bash
# Stop service
sudo systemctl stop rusty-server

# Backup current version
cp target/release/rusty-server target/release/rusty-server.backup

# Pull latest code (or transfer new files)
git pull
# Or: scp new files from laptop

# Rebuild
cargo build --release

# Run migrations (if any)
# Migrations run automatically on startup

# Start service
sudo systemctl start rusty-server

# Verify
sudo systemctl status rusty-server
curl http://localhost:3000/health
```

---

## Troubleshooting

### Server Won't Start

**Check logs:**
```bash
sudo journalctl -u rusty-server -n 50
```

**Common issues:**

1. **Database connection error:**
   - Verify MySQL is running: `sudo systemctl status mysql`
   - Check connection string in config
   - Test connection: `mysql -u rusty_user -p rusty_server`

2. **Port already in use:**
   ```bash
   # Check what's using port 3000
   sudo lsof -i :3000
   # Or
   sudo netstat -tlnp | grep 3000
   ```

3. **Permission errors:**
   ```bash
   # Check file permissions
   ls -la /opt/rusty-server/target/release/rusty-server
   # Make executable if needed
   chmod +x /opt/rusty-server/target/release/rusty-server
   ```

### Database Issues

**Migrations failed:**
```bash
# Check migration files exist
ls -la /opt/rusty-server/migrations/

# Manually run migrations (if needed)
mysql -u rusty_user -p rusty_server < migrations/001_initial_schema.sql
```

**Connection refused:**
```bash
# Check MySQL is running
sudo systemctl status mysql

# Check MySQL is listening
sudo netstat -tlnp | grep 3306
```

### Service Issues

**Service won't start:**
```bash
# Check service file syntax
sudo systemctl daemon-reload

# Check for errors
sudo journalctl -u rusty-server -n 100

# Test binary manually
/opt/rusty-server/target/release/rusty-server
```

**Service keeps restarting:**
```bash
# Check logs for crash reason
sudo journalctl -u rusty-server -n 100

# Check system resources
free -h
df -h
```

### Network Issues

**Can't access from other machines:**
```bash
# Check firewall
sudo ufw status

# Check server is listening
sudo ss -tlnp | grep 3000

# Test from server itself
curl http://localhost:3000/health
```

**Connection timeout:**
- Check firewall rules
- Verify server IP address
- Check network connectivity

---

## Quick Reference

### Essential Commands

```bash
# Service management
sudo systemctl start rusty-server
sudo systemctl stop rusty-server
sudo systemctl restart rusty-server
sudo systemctl status rusty-server

# View logs
sudo journalctl -u rusty-server -f

# Test endpoints
curl http://localhost:3000/health
curl http://localhost:3000/api/v1/space-weather/current

# Database access
mysql -u rusty_user -p rusty_server

# Check processes
ps aux | grep rusty-server

# Check ports
sudo ss -tlnp | grep 3000
```

### File Locations

- **Binary**: `/opt/rusty-server/target/release/rusty-server`
- **Config**: `/opt/rusty-server/config.toml`
- **Logs**: `/var/log/journal/` (systemd) or `/opt/rusty-server/logs/`
- **Service file**: `/etc/systemd/system/rusty-server.service`
- **Database**: MySQL `rusty_server` database

---

## Security Checklist

Before going live:

- [ ] Change default JWT secret to strong random value
- [ ] Set `require_auth = true` in production
- [ ] Restrict CORS origins (don't use `*`)
- [ ] Enable HSTS if using HTTPS
- [ ] Configure firewall properly
- [ ] Use strong database passwords
- [ ] Set up SSL/TLS certificates
- [ ] Configure log rotation
- [ ] Set up automated backups
- [ ] Review and restrict file permissions
- [ ] Keep system and dependencies updated

---

## Next Steps

After deployment:

1. **Monitor** the service for the first few days
2. **Set up** automated backups
3. **Configure** monitoring/alerting (optional)
4. **Document** your specific deployment details
5. **Test** all endpoints thoroughly
6. **Plan** for updates and maintenance

---

## Getting Help

If you encounter issues:

1. Check logs: `sudo journalctl -u rusty-server -n 100`
2. Verify configuration: `cat /opt/rusty-server/config.toml`
3. Test database connection manually
4. Check system resources: `free -h`, `df -h`
5. Review this guide's troubleshooting section

---

**Last Updated**: 2024-12-20  
**Status**: Ready for deployment
