# Server Deployment Notes

## Current Status

⚠️ **IMPORTANT**: Your server rack is not yet set up. This document will be updated with detailed, step-by-step instructions when we reach the deployment phase.

## What We Know About Your Server

- **Hardware**: Server rack with:
  - 2x 8-core 8-thread Xeon processors
  - 32GB DDR4 ECC memory
  - SAS3 12-drive backplane
  - 4x 10G RJ45 ports
  - IPMI management
  - Redundant 800W PSUs

- **Status**: 
  - ❌ Not plugged in
  - ❌ Never turned on
  - ❌ No OS installed yet

## Deployment Plan (Future)

When we reach Phase 9 (Deployment & Operations), we will provide:

### Step-by-Step Instructions For:

1. **Initial Server Setup**
   - Power on and initial boot
   - BIOS/UEFI configuration
   - IPMI setup and access
   - Network configuration

2. **Operating System Installation**
   - Linux distribution selection (we'll help you choose)
   - Installation process
   - Initial system configuration
   - User account setup

3. **MySQL Installation on Server**
   - Installing MySQL on Linux
   - Creating production database and user
   - Security configuration
   - Testing connection

4. **Rusty Server Deployment**
   - Building the application for Linux
   - Transferring files to server
   - Setting up environment variables
   - Creating systemd service
   - Starting the service

5. **Network Configuration**
   - Firewall rules
   - Port configuration
   - Reverse proxy setup (if needed)
   - SSL/TLS certificates (if needed)

6. **Monitoring & Maintenance**
   - Service monitoring
   - Log file locations
   - Backup procedures
   - Update procedures

## Development vs Production

### Current (Development)
- **Location**: Your Windows laptop
- **MySQL**: Running on Windows laptop
- **Rusty Server**: Will run on Windows laptop (localhost)
- **Purpose**: Development and testing

### Future (Production)
- **Location**: Linux server rack
- **MySQL**: Running on Linux server
- **Rusty Server**: Running on Linux server
- **Purpose**: Production service

## Credential Management

### Development Credentials
- Stored in: `credentials.txt` (on your laptop, gitignored)
- Used for: Local development
- **Never** use these in production!

### Production Credentials
- Will be created: On the Linux server
- Stored in: Environment variables or secure config on server
- Different from: Development credentials
- **Never** committed to git

## What We'll Do When Ready

1. **Ask you questions** about:
   - Which Linux distribution you prefer (or we'll recommend one)
   - Network setup preferences
   - Security requirements
   - Backup preferences

2. **Provide detailed instructions** for:
   - Each step of server setup
   - Each command to run
   - What to expect at each stage
   - How to verify each step worked

3. **Help troubleshoot** any issues that come up

4. **Test everything** before going live

## Timeline

We're currently in **Phase 1** (Project Foundation). Deployment is **Phase 9**, which is many steps away. 

**Current focus**: Building the application locally
**Future focus**: Deploying to your server (with detailed instructions)

## Questions to Answer Later

When we get closer to deployment, we'll ask:

1. Do you have a preferred Linux distribution? (Ubuntu, Debian, CentOS, etc.)
2. Will the server be on your local network or accessible from internet?
3. Do you want to use Docker or run directly on the server?
4. Do you have a domain name, or will you use IP address?
5. Do you want HTTPS/SSL certificates?
6. What's your backup strategy preference?

---

**Don't worry about server setup yet!** We'll handle it step-by-step when the time comes. For now, focus on getting MySQL set up locally for development.

