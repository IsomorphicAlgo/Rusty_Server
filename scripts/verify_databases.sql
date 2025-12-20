-- Database Verification Script
-- Run this in MySQL to verify your databases are set up correctly

-- ============================================
-- 1. Check if databases exist
-- ============================================
SHOW DATABASES LIKE 'rusty_server%';

-- ============================================
-- 2. Check production database (rusty_server)
-- ============================================
USE rusty_server;

-- List all tables
SHOW TABLES;

-- Check if migration tracking table exists
SHOW TABLES LIKE '_sqlx_migrations';

-- If migrations ran, show migration history
SELECT * FROM _sqlx_migrations;

-- Count rows in each table
SELECT 
    'space_weather_observations' AS table_name, 
    COUNT(*) AS row_count 
FROM space_weather_observations
UNION ALL
SELECT 
    'space_weather_alerts' AS table_name, 
    COUNT(*) AS row_count 
FROM space_weather_alerts
UNION ALL
SELECT 
    'cache_metadata' AS table_name, 
    COUNT(*) AS row_count 
FROM cache_metadata
UNION ALL
SELECT 
    'api_request_logs' AS table_name, 
    COUNT(*) AS row_count 
FROM api_request_logs;

-- Show table structure
DESCRIBE space_weather_observations;

-- ============================================
-- 3. Check test database (rusty_server_test)
-- ============================================
USE rusty_server_test;

-- List all tables
SHOW TABLES;

-- Check if migration tracking table exists
SHOW TABLES LIKE '_sqlx_migrations';

-- If migrations ran, show migration history
SELECT * FROM _sqlx_migrations;

-- ============================================
-- 4. Check user permissions
-- ============================================
SHOW GRANTS FOR CURRENT_USER();

-- ============================================
-- 5. Test connection (should return 1)
-- ============================================
SELECT 1 AS connection_test;
