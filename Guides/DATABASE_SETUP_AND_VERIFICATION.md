# Database Setup and Verification Guide

## Why You Don't See Tables

The tables are created automatically when you **run the server** for the first time. The migration system runs on startup. However, you need to:

1. **Create the databases** (they must exist before migrations can run)
2. **Run the server** (migrations execute automatically)
3. **Verify the tables were created**

---

## Step 1: Create the Databases

You need **TWO databases**:
- `rusty_server` - Production database
- `rusty_server_test` - Test database

### Option A: Using MySQL Command Line

```bash
# Connect to MySQL as root
mysql -u root -p

# Then run these SQL commands:
CREATE DATABASE IF NOT EXISTS rusty_server CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE DATABASE IF NOT EXISTS rusty_server_test CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

# Verify they were created
SHOW DATABASES;

# Exit MySQL
EXIT;
```

### Option B: Using MySQL Workbench

1. Open MySQL Workbench
2. Connect to your MySQL server
3. In the SQL Editor, run:

```sql
CREATE DATABASE IF NOT EXISTS rusty_server CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE DATABASE IF NOT EXISTS rusty_server_test CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

4. Click the refresh button (⚡) in the SCHEMAS panel
5. You should see both databases listed

---

## Step 2: Set Up Database User and Permissions

Make sure your database user has permissions on **both** databases:

```sql
-- Connect as root
mysql -u root -p

-- Grant permissions on both databases
GRANT ALL PRIVILEGES ON rusty_server.* TO 'rusty_user'@'localhost';
GRANT ALL PRIVILEGES ON rusty_server_test.* TO 'rusty_user'@'localhost';

-- Apply changes
FLUSH PRIVILEGES;

-- Verify permissions
SHOW GRANTS FOR 'rusty_user'@'localhost';
```

---

## Step 3: Run the Server (This Creates Tables)

The migrations run automatically when the server starts. Here's how:

### Set Database Connection String

**PowerShell:**
```powershell
# URL encode special characters: ? = %3F, & = %26
$env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server"
```

**Command Prompt:**
```cmd
set RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server
```

### Start the Server

```bash
cargo run
```

**Look for these log messages:**
```
Running database migrations...
Database migrations completed successfully
Database connection verified
```

If you see these messages, the tables were created successfully!

---

## Step 4: Verify Tables Were Created

### Option A: Using MySQL Command Line

```bash
# Connect to the database
mysql -u rusty_user -p rusty_server

# Show all tables
SHOW TABLES;

# You should see:
# - space_weather_observations
# - space_weather_alerts
# - cache_metadata
# - api_request_logs

# Check table structure
DESCRIBE space_weather_observations;

# Exit
EXIT;
```

### Option B: Using MySQL Workbench

1. Open MySQL Workbench
2. Connect to your server
3. Expand `rusty_server` database in the SCHEMAS panel
4. Expand "Tables"
5. You should see 4 tables:
   - `space_weather_observations`
   - `space_weather_alerts`
   - `cache_metadata`
   - `api_request_logs`

### Option C: Using SQL Query

```sql
USE rusty_server;

-- List all tables
SHOW TABLES;

-- Count rows in each table (should be 0 initially)
SELECT COUNT(*) FROM space_weather_observations;
SELECT COUNT(*) FROM space_weather_alerts;
SELECT COUNT(*) FROM cache_metadata;
SELECT COUNT(*) FROM api_request_logs;
```

---

## Step 5: Verify Test Database

For the test database, you can either:

### Option A: Run Tests (Creates Tables Automatically)

```bash
# Set test database connection
$env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server_test"

# Run tests (they will create tables if needed)
cargo test --test api_test
```

### Option B: Manually Run Migrations on Test DB

You can create a simple script to run migrations manually. See "Manual Migration Script" below.

---

## Troubleshooting

### Problem: "Database does not exist"

**Error:**
```
ERROR 1049 (42000): Unknown database 'rusty_server'
```

**Solution:**
1. Create the database (see Step 1)
2. Make sure the connection string uses the correct database name

### Problem: "Access denied"

**Error:**
```
ERROR 1045 (28000): Access denied for user 'rusty_user'@'localhost'
```

**Solution:**
1. Check username and password in connection string
2. Verify user has permissions:
   ```sql
   GRANT ALL PRIVILEGES ON rusty_server.* TO 'rusty_user'@'localhost';
   FLUSH PRIVILEGES;
   ```

### Problem: "Migrations failed"

**Error:**
```
Database migration failed: ...
```

**Solution:**
1. Check that the `migrations/` directory exists
2. Verify `001_initial_schema.sql` is present
3. Check database user has CREATE TABLE permissions
4. Look at full error message for specific SQL error

### Problem: "Tables still not showing"

**Possible Causes:**
1. **Server didn't start successfully** - Check logs for errors
2. **Connected to wrong database** - Verify you're looking at `rusty_server`, not `rusty_server_test`
3. **Migrations didn't run** - Check server logs for "Database migrations completed successfully"
4. **Permissions issue** - User might not have CREATE TABLE permission

**Solution:**
```sql
-- Check if migrations table exists (sqlx creates this)
USE rusty_server;
SHOW TABLES LIKE '_sqlx_migrations';

-- If it exists, check migration status
SELECT * FROM _sqlx_migrations;
```

---

## Manual Migration Script

If you want to run migrations manually without starting the server, you can create a simple Rust script:

**Create `scripts/run_migrations.rs`:**

```rust
use rusty_server::database::DatabasePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load from environment or use default
    let connection_string = std::env::var("RUSTY_SERVER__DATABASE__CONNECTION_STRING")
        .unwrap_or_else(|_| "mysql://rusty_user:password@localhost:3306/rusty_server".to_string());
    
    println!("Connecting to database...");
    let db_pool = DatabasePool::new(&connection_string).await?;
    
    println!("Running migrations...");
    db_pool.migrate().await?;
    
    println!("Migrations completed successfully!");
    Ok(())
}
```

Then run:
```bash
cargo run --bin run_migrations
```

---

## Quick Verification Checklist

- [ ] Both databases (`rusty_server` and `rusty_server_test`) exist
- [ ] Database user has permissions on both databases
- [ ] Connection string is set correctly (with URL-encoded password)
- [ ] Server starts without database errors
- [ ] Log shows "Database migrations completed successfully"
- [ ] Tables are visible in MySQL Workbench or command line
- [ ] Can query tables (even if empty)

---

## Expected Tables

After successful migration, you should have:

1. **space_weather_observations** - Main data table
   - Stores KP index, solar wind, solar flares, radiation data
   - Has indexes on timestamp, source, kp_index, etc.

2. **space_weather_alerts** - Alerts table
   - Stores active alerts and warnings
   - Has indexes on alert_type, severity, active status

3. **cache_metadata** - Cache tracking
   - Tracks cache status and TTL
   - Has indexes on cache_key, expires_at

4. **api_request_logs** - Request logging (optional)
   - Logs API requests for analytics
   - Has indexes on endpoint, status_code, timestamp

5. **_sqlx_migrations** - Migration tracking (created by sqlx)
   - Tracks which migrations have been run
   - Don't modify this table manually

---

## Next Steps

Once tables are created:

1. **Start the server** - It will automatically store data when you make API calls
2. **Make API calls** - Data will be stored in `space_weather_observations`
3. **Check data** - Query the tables to see stored observations

```sql
-- View recent observations
SELECT * FROM space_weather_observations 
ORDER BY timestamp DESC 
LIMIT 10;

-- Count total observations
SELECT COUNT(*) FROM space_weather_observations;
```

---

## Need Help?

If tables still aren't showing:

1. Check server logs for migration errors
2. Verify database connection string is correct
3. Ensure MySQL user has CREATE TABLE permissions
4. Try manually running the SQL from `migrations/001_initial_schema.sql`
