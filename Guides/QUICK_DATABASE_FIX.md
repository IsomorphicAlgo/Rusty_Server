# Quick Database Fix Guide

## Why You Don't See Tables

**The tables are created automatically when you run the server**, but only if:
1. ✅ The database exists
2. ✅ The server starts successfully
3. ✅ Migrations run without errors

---

## Quick Fix (3 Steps)

### Step 1: Create the Databases

**MySQL Command Line:**
```bash
mysql -u root -p
```

Then run:
```sql
CREATE DATABASE IF NOT EXISTS rusty_server CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE DATABASE IF NOT EXISTS rusty_server_test CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
EXIT;
```

**Or use MySQL Workbench:**
- Right-click in SCHEMAS panel → "Create Schema"
- Name: `rusty_server`, Collation: `utf8mb4_unicode_ci`
- Repeat for `rusty_server_test`

### Step 2: Set Connection String

**PowerShell:**
```powershell
# URL encode special characters: ? = %3F, & = %26
# NOTE: Using test database (rusty_server_test) for development
$env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server_test"
```

### Step 3: Run the Server

```bash
cargo run
```

**Look for these messages in the output:**
```
Running database migrations...
Database migrations completed successfully
Database connection verified
```

If you see these, tables were created! ✅

---

## Verify Tables Were Created

**MySQL Command Line:**
```bash
mysql -u rusty_user -p rusty_server
SHOW TABLES;
```

**You should see:**
- `space_weather_observations`
- `space_weather_alerts`
- `cache_metadata`
- `api_request_logs`
- `_sqlx_migrations` (migration tracking)

**Or use the verification script:**
```bash
mysql -u root -p < verify_databases.sql
```

---

## Common Issues

### Issue: "Database does not exist"
**Fix:** Create the database (Step 1 above)

### Issue: "Access denied"
**Fix:** Grant permissions:
```sql
GRANT ALL PRIVILEGES ON rusty_server.* TO 'rusty_user'@'localhost';
GRANT ALL PRIVILEGES ON rusty_server_test.* TO 'rusty_user'@'localhost';
FLUSH PRIVILEGES;
```

### Issue: "Tables still not showing"
**Check:**
1. Did server start successfully? (Check logs)
2. Did you see "Database migrations completed successfully"?
3. Are you looking at the right database? (`rusty_server`, not `rusty_server_test`)
4. Try running the server again - migrations are idempotent (safe to run multiple times)

### Issue: "Connection string error"
**Fix:** Make sure password is URL-encoded:
- `?` becomes `%3F`
- `&` becomes `%26`
- `/` becomes `%2F`
- `@` becomes `%40`

---

## Test Database Setup

For tests, the test database (`rusty_server_test`) needs the same setup:

1. Create the database (same as Step 1)
2. Grant permissions (same as above)
3. Tests will run migrations automatically when you run `cargo test`

---

## Full Documentation

See `DATABASE_SETUP_AND_VERIFICATION.md` for complete details.
