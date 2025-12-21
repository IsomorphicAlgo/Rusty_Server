# Server Deployment Guide

Complete step-by-step guide for deploying Rusty Server to your Linux server rack.

---

######### New User Notes##########
rusty_server is now connected via SSH and windows powershell
rusty_user@rustyserver:~$ sudo dmidecode -t system | grep -E "Manufacturer|Product"
        Manufacturer: Supermicro
        Product Name: PIO-628U-TR4T+-ST031
password for rusty_user is in credentials

ip addr show
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host
       valid_lft forever preferred_lft forever
2: enp1s0f0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 0c:c4:7a:a3:74:8c brd ff:ff:ff:ff:ff:ff
3: enp1s0f1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP group default qlen 1000
    link/ether 0c:c4:7a:a3:74:8d brd ff:ff:ff:ff:ff:ff
    inet 10.0.0.220/24 metric 100 brd 10.0.0.255 scope global dynamic enp1s0f1
       valid_lft 171636sec preferred_lft 171636sec
    inet6 2601:601:8600:6a60::8358/128 scope global dynamic noprefixroute
       valid_lft 214123sec preferred_lft 214123sec
    inet6 2601:601:8600:6a60:ec4:7aff:fea3:748d/64 scope global dynamic mngtmpaddr noprefixroute
       valid_lft 215285sec preferred_lft 215285sec
    inet6 fe80::ec4:7aff:fea3:748d/64 scope link
       valid_lft forever preferred_lft forever
4: enp3s0f0: <BROADCAST,MULTICAST> mtu 1500 qdisc noop state DOWN group default qlen 1000
    link/ether 0c:c4:7a:a3:74:8e brd ff:ff:ff:ff:ff:ff
5: enp3s0f1: <BROADCAST,MULTICAST> mtu 1500 qdisc noop state DOWN group default qlen 1000
    link/ether 0c:c4:7a:a3:74:8f brd ff:ff:ff:ff:ff:ff
rusty_user@rustyserver:~$ rustc --version
rustc 1.92.0 (ded5c06cf 2025-12-08)


## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Initial Server Setup](#initial-server-setup)
3. [Installing Dependencies](#installing-dependencies)
4. [Building the Application](#building-the-application)
5. [Database Setup](#database-setup)
6. [Configuration](#configuration)
7. [ML Service Setup](#ml-service-setup)
8. [Running the Server](#running-the-server)
9. [Setting Up as a Service](#setting-up-as-a-service)
10. [Network Configuration](#network-configuration)
11. [Testing the Deployment](#testing-the-deployment)
12. [Monitoring & Maintenance](#monitoring--maintenance)
13. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Hardware Requirements
- ✅ Your server rack hardware (already have)
- ✅ Network connection
- ✅ Power supply
- ✅ Bootable Ubuntu USB (you have this! ✅)
- ✅ IPMI/BMC network port (for remote management - no monitor needed!)

### Software Requirements
- Linux operating system (Ubuntu 22.04 LTS recommended, or Debian 11+)
- Rust toolchain (latest stable) - ✅ Installing now
- MySQL 8.0+ or MariaDB 10.6+
- **Python 3.10+** (required for ML service microservice)
- **pip and Python packages** (for ML service)
- Git (for cloning repository)
- Systemd (for service management)

### Server Information (Your Setup)
- **Manufacturer**: Supermicro
- **Model**: PIO-628U-TR4T+-ST031
- **SSH Access**: ✅ Connected via Windows PowerShell
- **Server IP**: 10.0.0.220 (enp1s0f1 interface)
- **User**: rusty_user
- **Rust Version**: 1.92.0 (installed)

### Remote Management Setup
- **📋 See [IPMI_SETUP_GUIDE.md](IPMI_SETUP_GUIDE.md) for detailed IPMI setup instructions**
- **You DON'T need a monitor!** IPMI remote console gives you a virtual monitor on your laptop

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

### Step 3: Install Python and ML Service Dependencies

**Yes, you need Python on the server!** The ML models run as a separate Python microservice.

```bash
# Install Python 3.10+ and pip
sudo apt install -y python3 python3-pip python3-venv

# Verify installation
python3 --version
pip3 --version

# Install Python build dependencies (for some ML packages)
sudo apt install -y python3-dev build-essential

# Optional: Install virtual environment tools
sudo apt install -y python3-venv
```

**Why Python?**
- ML models are implemented as a **Python microservice** (separate service)
- Rust API communicates with Python ML service via HTTP REST API
- This allows you to train/tune models easily (as you wanted!)
- Python service runs on port 8001 (configurable)
- Rust API calls Python service for predictions

**Architecture:**
```
Rust API (Port 3000)  ←→  Python ML Service (Port 8001)
     ↓                           ↓
  MySQL Database          XGBoost Model
```

### Step 4: Install Additional Tools

```bash
# Install useful tools
sudo apt install -y git curl wget vim htop

# Install development tools (if needed)
sudo apt install -y gcc g++ make
```

---

## Building the Application

### Step 1: Transfer Files to Server

Since you're connected via SSH, you can transfer files from your Windows laptop:

**Option A: Using SCP (from Windows PowerShell):**
```powershell
# From your laptop (PowerShell)
# Navigate to Rusty_Server directory
cd C:\Users\micha\Rust\Rusty_Server

# Transfer entire project (excluding target/ and other build artifacts)
scp -r -o StrictHostKeyChecking=no \
    --exclude='target/' \
    --exclude='.git/' \
    --exclude='credentials.txt' \
    * rusty_user@10.0.0.220:/opt/rusty-server/
```

**Option B: Using Git (Recommended):**
```bash
# On server
sudo mkdir -p /opt/rusty-server
sudo chown $USER:$USER /opt/rusty-server

cd /opt/rusty-server
git clone <https://github.com/IsomorphicAlgo/Rusty_Server> .

# Or if using SSH key:
git clone git@github.com:YOUR_USERNAME/Rusty_Server.git .
```

**Option C: Using rsync (if available on Windows):**
```powershell
# From Windows (if you have WSL or rsync installed)
rsync -avz --exclude 'target/' --exclude '.git/' \
    C:\Users\micha\Rust\Rusty_Server\ \
    rusty_user@10.0.0.220:/opt/rusty-server/
```

**Option D: Manual Transfer:**
- Use WinSCP, FileZilla, or similar SFTP client
- Connect to `10.0.0.220` as `rusty_user`
- Transfer project files

### Step 2: Build the Application

```bash
cd /opt/rusty-server

###Just finished the line above###
# Build in release mode (optimized)
cargo build --release

# This will create the binary at:
# target/release/rusty-server

# Verify binary was created
ls -lh target/release/rusty-server

# Test the binary
./target/release/rusty-server --help  # (if help flag exists)
```

### Step 3: Set Up ML Service

**The ML service is a separate Python microservice that runs alongside the Rust API.**

```bash
cd /opt/rusty-server

# Check if ml_service directory exists
ls -la ml_service/

# If ml_service directory exists, set it up:
cd ml_service

# Create virtual environment (recommended)
python3 -m venv venv

# Activate virtual environment
source venv/bin/activate

# Install Python dependencies
pip install -r requirements.txt

# Verify installation
python3 --version
pip list

# Deactivate virtual environment (for now)
deactivate
```

**Note**: The ML service will run as a separate service. We'll set it up as a systemd service later.

### Step 4: Create Application Directory Structure

```bash
# Create directories for runtime files
sudo mkdir -p /opt/rusty-server/{config,logs,data,backups}
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

[exoplanet]
base_url = "https://exoplanetarchive.ipac.caltech.edu/TAP"
timeout_seconds = 60

[ml_service]
base_url = "http://localhost:8001"  # Python ML service
timeout_seconds = 30
enabled = false  # Set to true after ML service is running

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

## ML Service Setup

**📋 See [ML_SERVICE_DEPLOYMENT.md](ML_SERVICE_DEPLOYMENT.md) for complete ML service setup instructions.**

**Quick Setup:**

1. **Install Python dependencies:**
   ```bash
   cd /opt/rusty-server/ml_service
   python3 -m venv venv
   source venv/bin/activate
   pip install -r requirements.txt
   ```

2. **Train initial model:**
   ```bash
   # Collect training data
   cargo run --bin collect_training_data
   
   # Train model
   python train_model.py
   ```

3. **Test ML service:**
   ```bash
   python app.py
   # Should start on http://localhost:8001
   ```

4. **Set up as systemd service** (see Step 8 below)

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

**You'll need TWO systemd services:**
1. **rusty-server.service** - Rust API (port 3000)
2. **rusty-ml-service.service** - Python ML service (port 8001)

#### Create Rust API Service

```bash
sudo nano /etc/systemd/system/rusty-server.service
```

**Or copy from scripts folder:**
```bash
sudo cp /opt/rusty-server/scripts/rusty-server.service /etc/systemd/system/
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

#### Create Python ML Service

```bash
sudo nano /etc/systemd/system/rusty-ml-service.service
```

Add this content:

```ini
[Unit]
Description=Rusty Server ML Service - Solar Flare Prediction
After=network.target mysql.service
Requires=mysql.service

[Service]
Type=simple
User=rusty-server
Group=rusty-server
WorkingDirectory=/opt/rusty-server/ml_service
Environment="PATH=/opt/rusty-server/ml_service/venv/bin"
ExecStart=/opt/rusty-server/ml_service/venv/bin/python3 /opt/rusty-server/ml_service/app.py
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=rusty-ml-service

# Environment variables
Environment="RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://rusty_user:password@localhost:3306/rusty_server"

# Security settings
NoNewPrivileges=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

**Enable and start both services:**

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable services (start on boot)
sudo systemctl enable rusty-server
sudo systemctl enable rusty-ml-service

# Start services
sudo systemctl start rusty-server
sudo systemctl start rusty-ml-service

# Check status
sudo systemctl status rusty-server
sudo systemctl status rusty-ml-service

# View logs
sudo journalctl -u rusty-server -f
sudo journalctl -u rusty-ml-service -f
```

**Service Management Commands:**

```bash
# Start services
sudo systemctl start rusty-server
sudo systemctl start rusty-ml-service

# Stop services
sudo systemctl stop rusty-server
sudo systemctl stop rusty-ml-service

# Restart services
sudo systemctl restart rusty-server
sudo systemctl restart rusty-ml-service

# Check status
sudo systemctl status rusty-server
sudo systemctl status rusty-ml-service

# View logs
sudo journalctl -u rusty-server -n 50
sudo journalctl -u rusty-ml-service -n 50
sudo journalctl -u rusty-server -f  # Follow Rust API logs
sudo journalctl -u rusty-ml-service -f  # Follow ML service logs

# Disable auto-start
sudo systemctl disable rusty-server
sudo systemctl disable rusty-ml-service
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

**Test Rust API:**
```bash
# From server
curl http://localhost:3000/health

# Expected response:
# {"status":"healthy","timestamp":...,"service":"rusty-server","version":"0.1.0"}
```

**Test ML Service:**
```bash
# From server
curl http://localhost:8001/health

# Expected response:
# {"status":"healthy","model_loaded":true,"model_version":"v1"}
```

### Step 2: Test API Endpoints

**Rust API Endpoints:**
```bash
# Current conditions
curl http://localhost:3000/api/v1/space-weather/current

# Historical data
curl "http://localhost:3000/api/v1/space-weather/historical?limit=5"

# Alerts
curl http://localhost:3000/api/v1/space-weather/alerts

# Solar flare prediction (if ML service enabled)
curl http://localhost:3000/api/v1/space-weather/predict
```

**ML Service Endpoints:**
```bash
# Health check
curl http://localhost:8001/health

# List models
curl http://localhost:8001/models

# Direct prediction (if endpoint exists)
curl -X POST http://localhost:8001/predict \
  -H "Content-Type: application/json" \
  -d '{"features": {...}}'
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

**Updating Rust API:**

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

**Updating ML Service:**

```bash
# Stop ML service
sudo systemctl stop rusty-ml-service

# Navigate to ML service directory
cd /opt/rusty-server/ml_service

# Activate virtual environment
source venv/bin/activate

# Pull latest code or transfer new files
git pull
# Or: scp new files from laptop

# Update Python dependencies (if requirements.txt changed)
pip install -r requirements.txt --upgrade

# Retrain model if needed (optional)
# python train_model.py

# Deactivate virtual environment
deactivate

# Start service
sudo systemctl start rusty-ml-service

# Verify
sudo systemctl status rusty-ml-service
curl http://localhost:8001/health
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
# Rust API service management
sudo systemctl start rusty-server
sudo systemctl stop rusty-server
sudo systemctl restart rusty-server
sudo systemctl status rusty-server

# ML Service management
sudo systemctl start rusty-ml-service
sudo systemctl stop rusty-ml-service
sudo systemctl restart rusty-ml-service
sudo systemctl status rusty-ml-service

# View logs
sudo journalctl -u rusty-server -f
sudo journalctl -u rusty-ml-service -f

# Test endpoints
curl http://localhost:3000/health  # Rust API
curl http://localhost:8001/health  # ML Service
curl http://localhost:3000/api/v1/space-weather/current
curl http://localhost:3000/api/v1/space-weather/predict

# Database access
mysql -u rusty_user -p rusty_server

# Check processes
ps aux | grep rusty-server
ps aux | grep python3 | grep ml_service

# Check ports
sudo ss -tlnp | grep 3000  # Rust API
sudo ss -tlnp | grep 8001  # ML Service
```

### File Locations

- **Rust API Binary**: `/opt/rusty-server/target/release/rusty-server`
- **ML Service**: `/opt/rusty-server/ml_service/`
- **Config**: `/opt/rusty-server/config.toml`
- **Logs**: `/var/log/journal/` (systemd) or `/opt/rusty-server/logs/`
- **Service files**: 
  - `/etc/systemd/system/rusty-server.service`
  - `/etc/systemd/system/rusty-ml-service.service`
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
