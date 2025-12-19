# MySQL Setup Guide

## Understanding MySQL Setup

You need to create **TWO things** in MySQL:

1. **A MySQL User** (username + password) - This is the account that will connect to MySQL
2. **A Database** (named `rusty_server`) - This is where your application will store data

Think of it like this:
- **User** = The person with a key (username/password)
- **Database** = The house (where data lives)

## Step-by-Step Setup

### Option A: Using MySQL Workbench (Recommended - Visual)

#### 1. Create a MySQL User

1. Open **MySQL Workbench**
2. Connect to your MySQL server (usually `localhost` with root user)
3. In the SQL Editor, run these commands:

```sql
-- Create a new user for Rusty Server
CREATE USER 'rusty_user'@'localhost' IDENTIFIED BY 'your_secure_password_here';

-- Grant all privileges on the rusty_server database to this user
GRANT ALL PRIVILEGES ON rusty_server.* TO 'rusty_user'@'localhost';

-- Apply the changes
FLUSH PRIVILEGES;
```

**Replace:**
- `rusty_user` = Your chosen username
- `your_secure_password_here` = Your chosen password (make it strong!)

#### 2. Create the Database

In the same SQL Editor, run:

```sql
-- Create the database
CREATE DATABASE rusty_server CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Verify it was created
SHOW DATABASES;
```

You should see `rusty_server` in the list.

#### 3. Test the Connection

In MySQL Workbench:
1. Click "Manage Server Connections" (or File → Manage Connections)
2. Click "New" to create a new connection
3. Fill in:
   - Connection Name: `Rusty Server`
   - Username: `rusty_user` (or whatever you chose)
   - Password: Click "Store in Vault" and enter your password
   - Default Schema: `rusty_server`
4. Click "Test Connection" - should say "Successfully made the MySQL connection"

### Option B: Using MySQL Command Line

1. Open Command Prompt or PowerShell
2. Connect to MySQL:
   ```bash
   mysql -u root -p
   ```
   (Enter your root password when prompted)

3. Run the same SQL commands as above:
   ```sql
   CREATE USER 'rusty_user'@'localhost' IDENTIFIED BY 'your_secure_password_here';
   CREATE DATABASE rusty_server CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
   GRANT ALL PRIVILEGES ON rusty_server.* TO 'rusty_user'@'localhost';
   FLUSH PRIVILEGES;
   ```

## Filling Out credentials.txt

Once you've created the user and database, fill out your `credentials.txt` file:

```txt
# Database Credentials
DB_USER=rusty_user                    # The username you created
DB_PASSWORD=your_secure_password_here # The password you set
DB_HOST=localhost                      # Usually localhost for local dev
DB_PORT=3306                           # Default MySQL port
DB_NAME=rusty_server                   # The database name you created
```

## Development vs Production

### Current Setup (Development - Your Windows Laptop)

- **MySQL runs on**: Your Windows laptop
- **Database location**: Local (`localhost`)
- **Credentials stored in**: `credentials.txt` (gitignored, stays on your laptop)
- **Purpose**: Development and testing

### Future Setup (Production - Your Linux Server)

When we get to deployment (much later), we'll:

1. **Set up MySQL on your Linux server**
   - Install MySQL on the server
   - Create a new user and database on the server
   - Use different credentials (never use dev credentials in production!)

2. **Deploy credentials securely**
   - **Option 1**: Environment variables set on the server
   - **Option 2**: Secure config file on the server (not in git)
   - **Option 3**: Secrets management service

3. **Connection**
   - The Rusty Server application (running on Linux server) will connect to MySQL (also on Linux server)
   - Both will be on the same server, so connection will be `localhost` or `127.0.0.1`

## Security Best Practices

### For Development (Now)

✅ **DO:**
- Use a dedicated MySQL user (not root)
- Use a strong password
- Keep `credentials.txt` gitignored (already done)
- Use different credentials than production

❌ **DON'T:**
- Use the root MySQL user for the application
- Commit credentials to git (already protected)
- Share credentials.txt

### For Production (Later)

✅ **DO:**
- Create a completely separate MySQL user
- Use a different, strong password
- Use environment variables or secure config
- Never commit production credentials

❌ **DON'T:**
- Use development credentials in production
- Store production credentials in git
- Use weak passwords

## Testing Your Setup

After creating the user and database, test the connection:

### Using MySQL Workbench
1. Create a new connection with your credentials
2. Test the connection
3. You should see the `rusty_server` database

### Using Command Line
```bash
mysql -u rusty_user -p rusty_server
```
(Enter your password when prompted)

If you can connect, you're all set!

## Troubleshooting

### "Access Denied" Error
- Check username and password are correct
- Verify the user has privileges: `SHOW GRANTS FOR 'rusty_user'@'localhost';`
- Make sure you're connecting to the right host

### "Database doesn't exist" Error
- Verify database was created: `SHOW DATABASES;`
- Check the database name matches in credentials.txt

### "Can't connect to MySQL server"
- Make sure MySQL is running
- Check the port (default is 3306)
- Verify host is correct (localhost for local dev)

## Next Steps

Once your credentials.txt is filled out:

1. ✅ MySQL user created
2. ✅ Database created
3. ✅ credentials.txt filled out
4. ⏳ We'll update the application to use these credentials (in a future step)
5. ⏳ Test the database connection from the Rust application

---

**Note**: We won't actually connect to the database from the Rust app until we implement the database layer (Phase 4). For now, just having MySQL set up and credentials ready is perfect!

