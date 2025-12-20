#!/bin/bash
# Database Backup Script for Rusty Server
# Run this script via cron for automated backups

# Configuration
BACKUP_DIR="/opt/rusty-server/backups"
DATE=$(date +%Y%m%d_%H%M%S)
DB_NAME="rusty_server"
DB_USER="rusty_user"
# DB_PASSWORD should be set via environment variable or .my.cnf
RETENTION_DAYS=7

# Create backup directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

# Backup filename
BACKUP_FILE="$BACKUP_DIR/rusty_server_$DATE.sql"

# Perform backup
echo "Starting database backup: $BACKUP_FILE"
mysqldump -u "$DB_USER" -p"$DB_PASSWORD" "$DB_NAME" > "$BACKUP_FILE" 2>&1

# Check if backup was successful
if [ $? -eq 0 ]; then
    echo "Backup successful: $BACKUP_FILE"
    
    # Compress backup
    gzip "$BACKUP_FILE"
    echo "Backup compressed: ${BACKUP_FILE}.gz"
    
    # Remove old backups (keep last N days)
    find "$BACKUP_DIR" -name "rusty_server_*.sql.gz" -mtime +$RETENTION_DAYS -delete
    echo "Old backups cleaned (keeping last $RETENTION_DAYS days)"
else
    echo "ERROR: Backup failed!"
    exit 1
fi
