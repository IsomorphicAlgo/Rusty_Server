# Database Setup Script for Rusty Server
# This script helps set up both production and test databases

Write-Host "=== Rusty Server Database Setup ===" -ForegroundColor Cyan
Write-Host ""

# Check if MySQL is accessible
Write-Host "Checking MySQL connection..." -ForegroundColor Yellow
$mysqlCheck = Get-Command mysql -ErrorAction SilentlyContinue
if (-not $mysqlCheck) {
    Write-Host "ERROR: MySQL command not found. Make sure MySQL is installed and in PATH." -ForegroundColor Red
    exit 1
}

# Get database credentials
Write-Host "Enter MySQL root password:" -ForegroundColor Yellow
$rootPassword = Read-Host -AsSecureString
$rootPasswordPlain = [Runtime.InteropServices.Marshal]::PtrToStringAuto(
    [Runtime.InteropServices.Marshal]::SecureStringToBSTR($rootPassword)
)

# Or use environment variables if set
$dbUser = if ($env:DB_USER) { $env:DB_USER } else { "rusty_user" }
$dbPassword = if ($env:DB_PASSWORD) { $env:DB_PASSWORD } else { Read-Host "Enter database user password" -AsSecureString }
$dbHost = if ($env:DB_HOST) { $env:DB_HOST } else { "localhost" }
$dbPort = if ($env:DB_PORT) { $env:DB_PORT } else { "3306" }

Write-Host ""
Write-Host "Creating databases..." -ForegroundColor Yellow

# Create SQL script
$sqlScript = @"
CREATE DATABASE IF NOT EXISTS rusty_server CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE DATABASE IF NOT EXISTS rusty_server_test CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE USER IF NOT EXISTS '$dbUser'@'$dbHost' IDENTIFIED BY '$dbPassword';
GRANT ALL PRIVILEGES ON rusty_server.* TO '$dbUser'@'$dbHost';
GRANT ALL PRIVILEGES ON rusty_server_test.* TO '$dbUser'@'$dbHost';
FLUSH PRIVILEGES;
SHOW DATABASES LIKE 'rusty_server%';
"@

# Execute SQL
$sqlScript | mysql -u root -p"$rootPasswordPlain" 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✓ Databases created successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "1. Set connection string environment variable:"
    Write-Host "   `$env:RUSTY_SERVER__DATABASE__CONNECTION_STRING=`"mysql://$dbUser`:$dbPassword@$dbHost`:$dbPort/rusty_server`""
    Write-Host ""
    Write-Host "2. Start the server to run migrations:"
    Write-Host "   cargo run"
    Write-Host ""
    Write-Host "3. Verify tables were created using verify_databases.sql"
} else {
    Write-Host ""
    Write-Host "ERROR: Failed to create databases. Check MySQL connection and permissions." -ForegroundColor Red
    exit 1
}
