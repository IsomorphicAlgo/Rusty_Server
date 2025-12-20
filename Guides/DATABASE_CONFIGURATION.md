# Database Configuration Guide

## Current Setup: Test Database for Development

**We are currently using the TEST database (`rusty_server_test`) for development.**

This is the recommended approach until you're ready to move to production. The test database allows you to:
- Experiment without affecting production data
- Run tests safely
- Reset data easily if needed

---

## Quick Setup

### Option 1: Environment Variable (Recommended)

**PowerShell:**
```powershell
# URL encode special characters: ? = %3F, & = %26
$env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server_test"
```

**Command Prompt:**
```cmd
set RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server_test
```

### Option 2: Config File

Create `config.toml` in the project root:

```toml
[database]
# Using test database for development
connection_string = "mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server_test"
max_connections = 10
```

Then set:
```powershell
$env:CONFIG_FILE="config.toml"
```

---

## Database Names

- **`rusty_server_test`** - Test/Development database (currently in use)
- **`rusty_server`** - Production database (use when ready for production)

---

## Switching to Production Database

When you're ready to use the production database:

1. **Update environment variable:**
   ```powershell
   # Change from rusty_server_test to rusty_server
   $env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server"
   ```

2. **Or update config.toml:**
   ```toml
   [database]
   connection_string = "mysql://rusty_user:CXRTV8_7%3F4sPQ%26f@localhost:3306/rusty_server"
   ```

3. **Verify production database exists:**
   ```sql
   SHOW DATABASES LIKE 'rusty_server';
   ```

4. **Start the server** - Migrations will run automatically

---

## Verifying Current Database

To check which database you're currently using:

1. **Check environment variable:**
   ```powershell
   $env:RUSTY_SERVER__DATABASE__CONNECTION_STRING
   ```

2. **Check server logs** when starting:
   ```
   Initializing database connection pool...
   Database connection pool created successfully
   ```

3. **Query the database:**
   ```sql
   SELECT DATABASE();
   ```

---

## Important Notes

- **Test Database**: Safe to reset, experiment with, and use for development
- **Production Database**: Contains real data - be careful!
- **Both databases** should exist and have the same schema
- **Migrations** run automatically on server start for whichever database is configured

---

## Troubleshooting

### "Unknown database 'rusty_server_test'"

**Solution:** Create the test database:
```sql
CREATE DATABASE IF NOT EXISTS rusty_server_test CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

### "Access denied"

**Solution:** Grant permissions:
```sql
GRANT ALL PRIVILEGES ON rusty_server_test.* TO 'rusty_user'@'localhost';
FLUSH PRIVILEGES;
```

### Want to use production database?

Simply change the connection string from `rusty_server_test` to `rusty_server` in your environment variable or config file.

---

## Summary

✅ **Current Configuration**: Using `rusty_server_test` (test database)  
✅ **Safe for Development**: Yes - test database is isolated  
✅ **Ready for Production**: Change connection string to `rusty_server` when ready
