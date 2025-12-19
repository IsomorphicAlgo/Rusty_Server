# Security Guide

## Credential Management

This project uses multiple layers of security for handling sensitive information:

### 1. Environment Variables (Recommended)

The most secure method is using environment variables. Set them before running the application:

```bash
# Windows PowerShell
$env:RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://user:password@localhost/db"
$env:RUSTY_SERVER__AUTH__JWT_SECRET="your-secret-here"

# Linux/Mac
export RUSTY_SERVER__DATABASE__CONNECTION_STRING="mysql://user:password@localhost/db"
export RUSTY_SERVER__AUTH__JWT_SECRET="your-secret-here"
```

### 2. .env File (Local Development)

For local development, you can use a `.env` file (which is gitignored):

1. Copy `credentials.example.txt` to create your credentials
2. Create a `.env` file with your values
3. The application will automatically load it

**Example .env file:**
```
RUSTY_SERVER__DATABASE__CONNECTION_STRING=mysql://username:password@localhost/rusty_server
RUSTY_SERVER__AUTH__JWT_SECRET=your-strong-random-secret-here
RUSTY_SERVER__NOAA__API_KEY=your-api-key-if-needed
```

### 3. Config File (Less Secure - Not Recommended for Secrets)

Configuration files can be used, but **NEVER commit files with secrets**:
- `config/local.toml` - gitignored, for local development
- `config/production.toml` - gitignored, for production

## What's Gitignored

The following files are automatically ignored by git:
- `.env` and all `.env.*` files
- `credentials.txt` and `*.credentials` files
- `config/local.toml` and `config/production.toml`
- `config/secrets.toml` and `config/*.secret`
- Any file with `.secret`, `.key`, or `.pem` extensions

## Security Best Practices

1. **Never commit secrets**: All credential files are gitignored
2. **Use strong secrets**: Generate strong random strings for JWT secrets
3. **Rotate secrets regularly**: Change passwords and secrets periodically
4. **Use different secrets for dev/prod**: Never use production secrets in development
5. **Review .gitignore**: Ensure sensitive files are listed
6. **Mask in logs**: Passwords are automatically masked in log output

## Generating Strong Secrets

### JWT Secret
```bash
# Using OpenSSL
openssl rand -base64 32

# Using PowerShell
[Convert]::ToBase64String((1..32 | ForEach-Object { Get-Random -Minimum 0 -Maximum 256 }))
```

### Database Password
Use a strong password generator or:
```bash
# Generate random password
openssl rand -base64 24
```

## Production Deployment

For production:
1. Use environment variables set by your deployment system
2. Use a secrets management service (AWS Secrets Manager, HashiCorp Vault, etc.)
3. Never store secrets in code or config files
4. Use different credentials for each environment
5. Enable authentication (`require_auth = true` in production)

## Checking for Committed Secrets

Before committing, check for accidentally committed secrets:
```bash
# Search for common patterns
git grep -i "password"
git grep -i "secret"
git grep -i "api_key"
```

If you find any, remove them from git history:
```bash
git filter-branch --force --index-filter \
  "git rm --cached --ignore-unmatch path/to/file" \
  --prune-empty --tag-name-filter cat -- --all
```

