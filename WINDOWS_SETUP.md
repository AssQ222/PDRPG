# Windows Development Setup for PDRPG

This document provides detailed setup instructions for Windows developers working on PDRPG.

## 🚨 Common Issues & Solutions

### Issue: `failed to get cargo metadata: program not found`

**Root Cause:** Tauri CLI cannot locate Rust's `cargo` executable in system PATH.

**Solution:**
```powershell
# Automatic fix
pnpm setup

# Manual fix
$cargoPath = "$env:USERPROFILE\.cargo\bin"
[Environment]::SetEnvironmentVariable('Path', "$cargoPath;$env:PATH", 'User')
# Restart terminal after PATH modification
```

**Related:** [tauri-apps/tauri#5223](https://github.com/tauri-apps/tauri/issues/5223)

### Issue: `could not find Cargo.toml in D:\path\to\PDRPG`

**Root Cause:** Running `cargo` commands from wrong directory.

**Solution:**
- Always run `pnpm tauri dev` from project root (`D:\path\to\PDRPG`)
- For backend only: Run `cargo run` from `src-tauri` directory

### Issue: Rust toolchain errors on Windows

**Root Cause:** Wrong Rust target or missing MSVC toolchain.

**Solution:**
```powershell
rustup default stable-msvc
rustup target add x86_64-pc-windows-msvc
```

## 🛠️ Development Environment Requirements

### Prerequisites
- **OS:** Windows 10/11 (build 1903+)
- **PowerShell:** 5.1 or later (Windows PowerShell or PowerShell Core)
- **Node.js:** 18.0+ LTS
- **Package Manager:** pnpm (recommended) or npm

### Required Tools
- **Rust:** 1.70+ with MSVC toolchain
- **Visual Studio Build Tools:** 2019 or later (for MSVC)
- **Git:** 2.30+ for version control

## 📋 Step-by-Step Setup

### 1. One-Time Automated Setup
```powershell
# Clone repository
git clone <repo-url>
cd PDRPG

# Run automated setup
pnpm setup
```

The setup script will:
- Install Rust via winget (if not present)
- Configure stable-msvc toolchain
- Add cargo to PATH (User or System)
- Install Node.js dependencies
- Verify all components

### 2. Manual Setup (if automated fails)

#### Install Rust
```powershell
# Via winget (recommended)
winget install --id Rustlang.Rustup -e

# Or download from https://rustup.rs/
```

#### Configure Rust Toolchain
```powershell
rustup default stable-msvc
rustup target add x86_64-pc-windows-msvc
rustup component add clippy rustfmt
```

#### Configure PATH
```powershell
# Add cargo to User PATH
$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$cargoPath = "$env:USERPROFILE\.cargo\bin"
[Environment]::SetEnvironmentVariable('Path', "$cargoPath;$userPath", 'User')
```

#### Install Dependencies
```powershell
pnpm install
```

### 3. Verify Setup
```powershell
# Check versions
cargo --version        # Should show 1.70+
rustc --version        # Should show stable MSVC
pnpm tauri --version   # Should show Tauri CLI version

# Test build
pnpm tauri dev
```

## 🏃‍♂️ Running the Application

### Development Mode
```powershell
# Full application (recommended)
pnpm tauri dev

# Separate processes (debugging)
# Terminal 1 - Frontend
pnpm dev

# Terminal 2 - Backend  
cd src-tauri
cargo run
```

### Production Build
```powershell
pnpm tauri build
```

## 🐛 Troubleshooting

### PATH Issues
If commands fail after setup, restart your terminal to refresh environment variables.

### Permission Errors
If setup script fails with permission errors:
1. Run PowerShell as Administrator
2. Or use manual setup instructions above

### MSVC Toolchain Issues
If you get C++ compiler errors:
```powershell
# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# Or install Visual Studio Community with C++ workload
winget install Microsoft.VisualStudio.2022.Community
```

### Node.js Version Issues
Ensure you're using Node.js 18+ LTS:
```powershell
node --version    # Should be 18.0+
pnpm --version    # Should be 8.0+
```

## 🔧 IDE Configuration

### VS Code/Cursor Setup
The project includes pre-configured:
- `launch.json` for Rust debugging
- `tasks.json` for build automation  
- `settings.json` with rust-analyzer config
- Recommended extensions list

### Required Extensions
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) (for debugging)

## 📚 Additional Resources

- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
- [Rust Installation Guide](https://forge.rust-lang.org/infra/channel-layout.html#the-channels)
- [Windows Dev Environment Setup](https://learn.microsoft.com/en-us/windows/dev-environment/)

## 🤝 Contributing

When contributing to Windows-specific fixes:
1. Test on multiple Windows versions (10, 11)
2. Verify both PowerShell 5.1 and 7+
3. Include version info in commit messages
4. Reference related issues (especially tauri-apps/tauri#5223)

---

For additional help, check the main [README.md](./README.md) or open an issue. 