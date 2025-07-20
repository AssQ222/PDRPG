# PDRPG Setup Script for Windows
# Automatically configures Rust toolchain and development environment
# Related issue: https://github.com/tauri-apps/tauri/issues/5223

Write-Host "PDRPG Development Environment Setup" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")

if (-not $isAdmin) {
    Write-Host "Running without Administrator privileges. Will use User PATH instead of System PATH." -ForegroundColor Yellow
}

# Step 1: Install Rust if not present
Write-Host "`n1. Checking Rust installation..." -ForegroundColor Green

$rustupPath = "$env:USERPROFILE\.cargo\bin\rustup.exe"
$cargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"

if (-not (Test-Path $cargoPath)) {
    Write-Host "Installing Rust via winget..." -ForegroundColor Yellow
    try {
        winget install --id Rustlang.Rustup -e --silent
        Write-Host "Rust installed successfully" -ForegroundColor Green
    } catch {
        Write-Host "Failed to install Rust via winget. Please install manually from https://rustup.rs/" -ForegroundColor Red
        Write-Host "Error: $_" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "Rust is already installed" -ForegroundColor Green
}

# Step 2: Configure Rust toolchain for Windows MSVC
Write-Host "`n2. Configuring Rust toolchain..." -ForegroundColor Green

if (Test-Path $rustupPath) {
    try {
        & $rustupPath default stable-msvc
        & $rustupPath target add x86_64-pc-windows-msvc
        Write-Host "Rust toolchain configured for Windows MSVC" -ForegroundColor Green
    } catch {
        Write-Host "Warning: Could not configure Rust toolchain. Error: $_" -ForegroundColor Yellow
    }
} else {
    Write-Host "rustup not found. Skipping toolchain configuration." -ForegroundColor Yellow
}

# Step 3: Add Cargo to PATH
Write-Host "`n3. Configuring PATH..." -ForegroundColor Green

$cargoDir = "$env:USERPROFILE\.cargo\bin"

if ($isAdmin) {
    # System PATH (requires admin)
    try {
        $systemPath = [Environment]::GetEnvironmentVariable('Path', 'Machine')
        if ($systemPath -notlike "*$cargoDir*") {
            [Environment]::SetEnvironmentVariable('Path', "$systemPath;$cargoDir", 'Machine')
            Write-Host "Added Cargo to System PATH" -ForegroundColor Green
        } else {
            Write-Host "Cargo already in System PATH" -ForegroundColor Green
        }
    } catch {
        Write-Host "Could not modify System PATH. Using User PATH instead." -ForegroundColor Yellow
        $isAdmin = $false
    }
}

if (-not $isAdmin) {
    # User PATH (no admin required)
    try {
        $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
        if ($userPath -notlike "*$cargoDir*") {
            [Environment]::SetEnvironmentVariable('Path', "$cargoDir;$userPath", 'User')
            Write-Host "Added Cargo to User PATH" -ForegroundColor Green
        } else {
            Write-Host "Cargo already in User PATH" -ForegroundColor Green
        }
    } catch {
        Write-Host "Failed to modify PATH. Error: $_" -ForegroundColor Red
        exit 1
    }
}

# Step 4: Install Node.js dependencies
Write-Host "`n4. Installing Node.js dependencies..." -ForegroundColor Green

if (Get-Command pnpm -ErrorAction SilentlyContinue) {
    try {
        pnpm install
        Write-Host "Dependencies installed successfully" -ForegroundColor Green
    } catch {
        Write-Host "Failed to install dependencies. Error: $_" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "pnpm not found. Please install pnpm first: npm install -g pnpm" -ForegroundColor Red
    exit 1
}

# Step 5: Verify installation
Write-Host "`n5. Verifying installation..." -ForegroundColor Green

# Refresh PATH for current session
$env:PATH = "$cargoDir;$env:PATH"

$verificationPassed = $true

# Check Rust
if (Test-Path $cargoPath) {
    try {
        $cargoVersion = & $cargoPath --version
        Write-Host "Cargo: $cargoVersion" -ForegroundColor Green
    } catch {
        Write-Host "Cargo verification failed" -ForegroundColor Red
        $verificationPassed = $false
    }
} else {
    Write-Host "Cargo not found" -ForegroundColor Red
    $verificationPassed = $false
}

# Check Tauri
try {
    $tauriVersion = pnpm tauri --version
    Write-Host "Tauri CLI: $tauriVersion" -ForegroundColor Green
} catch {
    Write-Host "Tauri CLI verification failed" -ForegroundColor Red
    $verificationPassed = $false
}

# Final status
Write-Host "`nSetup Complete!" -ForegroundColor Cyan
Write-Host "=================" -ForegroundColor Cyan

if ($verificationPassed) {
    Write-Host "All components verified successfully" -ForegroundColor Green
    Write-Host "`nYou can now run:" -ForegroundColor Cyan
    Write-Host "   pnpm tauri dev  # Start development server" -ForegroundColor White
    Write-Host "   pnpm build      # Build for production" -ForegroundColor White
    Write-Host "`nNote: You may need to restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
} else {
    Write-Host "Some components failed verification. Please check the errors above." -ForegroundColor Red
    Write-Host "Try restarting your terminal and running this script again." -ForegroundColor Yellow
    exit 1
}

Write-Host "`nFor more information, see: https://tauri.app/v1/guides/getting-started/prerequisites" -ForegroundColor Blue 