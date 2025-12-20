# Build Troubleshooting Guide

## Linker Error LNK1104

If you're seeing this error:
```
LINK : fatal error LNK1104: cannot open file '...\build_script_build.exe'
```

This is typically caused by antivirus software blocking or quarantining build files.

### Solutions

#### 1. Antivirus Exclusions (Most Common Fix)

**Windows Defender:**
1. Open Windows Security
2. Go to "Virus & threat protection"
3. Click "Manage settings" under "Virus & threat protection settings"
4. Scroll down to "Exclusions"
5. Click "Add or remove exclusions"
6. Add these exclusions:
   - **Folder**: `C:\Users\micha\Rust\Rusty_Server\target`
   - **Folder**: `C:\Users\micha\.cargo` (if you want to exclude all Rust builds)
   - **Process**: `link.exe` (the linker)
   - **Process**: `rustc.exe` (Rust compiler)

**Other Antivirus Software:**
- Add the `target` folder to exclusions
- Add `link.exe` and `rustc.exe` to process exclusions
- May need to restart after adding exclusions

#### 2. Restart After Adding Exclusions

After adding antivirus exclusions:
1. **Restart your computer** (antivirus may need a full restart)
2. Clean the build: `cargo clean`
3. Try building again: `cargo check`

#### 3. Check for Quarantined Files

1. Check your antivirus quarantine
2. Restore any quarantined files from `target/` directory
3. Add exclusions before restoring

#### 4. Run as Administrator (Temporary Workaround)

Sometimes running as administrator helps:
```powershell
# Right-click PowerShell/Command Prompt
# Select "Run as Administrator"
cd C:\Users\micha\Rust\Rusty_Server
cargo clean
cargo check
```

#### 5. Check File Permissions

Ensure you have write permissions:
```powershell
# Check permissions
icacls "target"

# If needed, take ownership (run as admin)
takeown /f "target" /r /d y
icacls "target" /grant "${env:USERNAME}:(OI)(CI)F" /t
```

#### 6. Disable Real-Time Protection Temporarily

**Only for testing!** Re-enable immediately after:
1. Temporarily disable real-time protection
2. Run `cargo clean && cargo check`
3. If it works, the issue is definitely antivirus
4. Re-enable protection and add proper exclusions

#### 7. Use Alternative Linker (Advanced)

If issues persist, you can try using the GNU toolchain instead of MSVC:
```powershell
# Install GNU toolchain
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu

# Then try building
cargo clean
cargo check
```

## Other Common Issues

### Out of Disk Space
```powershell
# Check disk space
Get-PSDrive C
```

### Corrupted Build Cache
```powershell
# Clean everything
cargo clean
# Remove Cargo registry cache (if needed)
Remove-Item "$env:USERPROFILE\.cargo\registry\cache" -Recurse -Force
```

### Multiple Rust Processes
```powershell
# Kill any stuck Rust processes
Get-Process | Where-Object {$_.Name -like "*rust*" -or $_.Name -like "*cargo*"} | Stop-Process -Force
```

## Verification

After fixing, verify the build works:
```powershell
cargo clean
cargo check
cargo build
```

If `cargo check` succeeds, the issue is resolved!

